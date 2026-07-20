import json
import os
import sys
import time
import uuid

ROOT_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

# ENV

def load_env():
    path = os.path.join(ROOT_DIR, ".env")
    if not os.path.exists(path): return
    with open(path) as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("#"): continue
            key, _, value = line.partition("=")
            os.environ.setdefault(key.strip(), value.strip())

def save_env(key, value):
    path = os.path.join(ROOT_DIR, ".env")
    with open(path) as f:
        if any(line.startswith(f"{key}=") for line in f): return
    with open(path, "a") as f:
        f.write(f"{key}={value}\n")
    os.environ[key] = value

load_env()

# DESIRED

APEX = "mrly.net"
ALIASES = [APEX, f"www.{APEX}"]
COMMENT = "mrly"
OAC_NAME = "mrly"
CACHING_OPTIMIZED = "658327ea-f89d-4fab-a63d-7e88639e58f6"
CLOUDFRONT_ZONE = "Z2FDTNDATAQYW2"

CSP = "default-src 'self'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self'; img-src 'self' data:; font-src 'self'; connect-src 'self'; object-src 'none'; base-uri 'self'; frame-ancestors 'self'"

CUSTOM_ERROR_RESPONSES = {
    "Quantity": 2,
    "Items": [
        {"ErrorCode": 403, "ResponsePagePath": "/index.html", "ResponseCode": "200", "ErrorCachingMinTTL": 300},
        {"ErrorCode": 404, "ResponsePagePath": "/index.html", "ResponseCode": "200", "ErrorCachingMinTTL": 300},
    ],
}

SECURITY_POLICY = {
    "Name": "mrlynet-security",
    "SecurityHeadersConfig": {
        "ContentSecurityPolicy": {"Override": True, "ContentSecurityPolicy": CSP},
        "ContentTypeOptions": {"Override": True},
        "ReferrerPolicy": {"Override": True, "ReferrerPolicy": "strict-origin-when-cross-origin"},
    },
}

def behavior(origin_id):
    return {
        "TargetOriginId": origin_id,
        "ViewerProtocolPolicy": "redirect-to-https",
        "AllowedMethods": {"Quantity": 2, "Items": ["GET", "HEAD"], "CachedMethods": {"Quantity": 2, "Items": ["GET", "HEAD"]}},
        "Compress": True,
        "CachePolicyId": CACHING_OPTIMIZED,
    }

def origin(origin_id, bucket, region, oac_id):
    return {
        "Id": origin_id,
        "DomainName": f"{bucket}.s3.{region}.amazonaws.com",
        "OriginPath": "",
        "CustomHeaders": {"Quantity": 0},
        "S3OriginConfig": {"OriginAccessIdentity": ""},
        "OriginAccessControlId": oac_id,
    }

def desired_config(net_bucket, cdn_bucket, region, oac_id):
    return {
        "CallerReference": str(uuid.uuid4()),
        "Comment": COMMENT,
        "Enabled": True,
        "DefaultRootObject": "index.html",
        "HttpVersion": "http2and3",
        "IsIPV6Enabled": True,
        "PriceClass": "PriceClass_All",
        "Aliases": {"Quantity": 0},
        "Origins": {"Quantity": 2, "Items": [origin("site", net_bucket, region, oac_id), origin("cdn", cdn_bucket, region, oac_id)]},
        "DefaultCacheBehavior": behavior("site"),
        "CacheBehaviors": {"Quantity": 1, "Items": [{"PathPattern": "cdn/*", **behavior("cdn")}]},
        "CustomErrorResponses": CUSTOM_ERROR_RESPONSES,
        "ViewerCertificate": {"CloudFrontDefaultCertificate": True},
    }

# COMPARE

def _canon(x):
    if isinstance(x, dict): return tuple(sorted((k, _canon(v)) for k, v in x.items()))
    if isinstance(x, list): return tuple(_canon(i) for i in x)
    return x

def matches(desired, current):
    if isinstance(desired, dict):
        keys = set(desired.keys())
        if "Quantity" in keys and keys <= {"Quantity", "Items"}:
            cur = current if isinstance(current, dict) else {}
            d = sorted(_canon(i) for i in desired.get("Items", []))
            c = sorted(_canon(i) for i in cur.get("Items", []))
            return d == c
        if not isinstance(current, dict): return False
        return all(matches(v, current.get(k)) for k, v in desired.items())
    return desired == current

# DISCOVER

def find_cert():
    paginator = acm_client.get_paginator("list_certificates")
    for page in paginator.paginate(CertificateStatuses=["ISSUED"]):
        for cert in page["CertificateSummaryList"]:
            names = [cert["DomainName"], *cert.get("SubjectAlternativeNameSummaries", [])]
            if APEX in names or f"*.{APEX}" in names:
                return cert["CertificateArn"]
    return None

def find_distribution():
    dist_id = os.environ.get("MRLY_ID")
    if dist_id: return dist_id
    paginator = cf_client.get_paginator("list_distributions")
    for page in paginator.paginate():
        for item in page["DistributionList"].get("Items", []):
            if item["Comment"] == COMMENT:
                return item["Id"]
    return None

def find_zone():
    zones = r53_client.list_hosted_zones()["HostedZones"]
    for zone in zones:
        if zone["Name"] == f"{APEX}." and not zone["Config"]["PrivateZone"]:
            return zone["Id"]
    return None

def bucket_region(bucket):
    loc = s3_client.get_bucket_location(Bucket=bucket)["LocationConstraint"]
    return loc or "us-east-1"

def dist_status(dist_id):
    d = cf_client.get_distribution(Id=dist_id)["Distribution"]
    return d["Status"], d["DomainName"], d["DistributionConfig"]

def buckets():
    net = os.environ.get("MRLYNET_BUCKET")
    cdn = os.environ.get("MRLYCDN_BUCKET")
    missing = [key for key, value in (("MRLYNET_BUCKET", net), ("MRLYCDN_BUCKET", cdn)) if not value]
    if missing:
        print(f"cloudfront (add to .env: {', '.join(missing)})")
        return None, None
    return net, cdn

# POLICIES

def find_policy(name):
    marker = None
    while True:
        kwargs = {"Type": "custom", "MaxItems": "100"}
        if marker: kwargs["Marker"] = marker
        plist = cf_client.list_response_headers_policies(**kwargs)["ResponseHeadersPolicyList"]
        for item in plist.get("Items", []):
            policy = item["ResponseHeadersPolicy"]
            if policy["ResponseHeadersPolicyConfig"].get("Name") == name:
                return policy["Id"]
        marker = plist.get("NextMarker")
        if not marker: return None

def ensure_policy(desired):
    name = desired["Name"]
    pid = find_policy(name)
    if pid is None:
        resp = cf_client.create_response_headers_policy(ResponseHeadersPolicyConfig=desired)
        return resp["ResponseHeadersPolicy"]["Id"], "created"
    got = cf_client.get_response_headers_policy(Id=pid)
    current = got["ResponseHeadersPolicy"]["ResponseHeadersPolicyConfig"]
    if matches(desired, current):
        return pid, "up to date"
    cf_client.update_response_headers_policy(Id=pid, ResponseHeadersPolicyConfig=desired, IfMatch=got["ETag"])
    return pid, "updated"

def ensure_oac():
    marker = None
    while True:
        kwargs = {"MaxItems": "100"}
        if marker: kwargs["Marker"] = marker
        olist = cf_client.list_origin_access_controls(**kwargs)["OriginAccessControlList"]
        for item in olist.get("Items", []):
            if item["Name"] == OAC_NAME:
                return item["Id"]
        marker = olist.get("NextMarker")
        if not marker: break
    resp = cf_client.create_origin_access_control(OriginAccessControlConfig={
        "Name": OAC_NAME,
        "SigningProtocol": "sigv4",
        "SigningBehavior": "always",
        "OriginAccessControlOriginType": "s3",
    })
    return resp["OriginAccessControl"]["Id"]

def ensure_bucket_policy(bucket, dist_arns):
    desired = {
        "Version": "2012-10-17",
        "Statement": [{
            "Sid": "AllowCloudFront",
            "Effect": "Allow",
            "Principal": {"Service": "cloudfront.amazonaws.com"},
            "Action": "s3:GetObject",
            "Resource": f"arn:aws:s3:::{bucket}/*",
            "Condition": {"StringEquals": {"AWS:SourceArn": sorted(dist_arns)}},
        }],
    }
    try:
        current = json.loads(s3_client.get_bucket_policy(Bucket=bucket)["Policy"])
    except ClientError as e:
        if e.response["Error"]["Code"] != "NoSuchBucketPolicy": raise
        current = None
    if current == desired:
        print(f"  bucket {bucket}: policy up to date")
        return
    s3_client.put_bucket_policy(Bucket=bucket, Policy=json.dumps(desired))
    print(f"  bucket {bucket}: policy set for {len(dist_arns)} distribution(s)")

# WAIT

def wait_deployed(dist_id):
    while True:
        status, _, _ = dist_status(dist_id)
        if status == "Deployed": return
        print(f"  {dist_id}: {status}, waiting...")
        time.sleep(20)

# DNS

def upsert_alias(zone_id, name, target):
    existing = r53_client.list_resource_record_sets(HostedZoneId=zone_id, StartRecordName=name, MaxItems="5")["ResourceRecordSets"]
    changes = []
    for record in existing:
        if record["Name"] == f"{name}." and record["Type"] == "CNAME":
            changes.append({"Action": "DELETE", "ResourceRecordSet": record})
    for rtype in ("A", "AAAA"):
        changes.append({"Action": "UPSERT", "ResourceRecordSet": {
            "Name": name,
            "Type": rtype,
            "AliasTarget": {"HostedZoneId": CLOUDFRONT_ZONE, "DNSName": target, "EvaluateTargetHealth": False},
        }})
    r53_client.change_resource_record_sets(HostedZoneId=zone_id, ChangeBatch={"Changes": changes})
    print(f"  dns {name} -> {target}")

# ARN

def dist_arn(dist_id):
    return f"arn:aws:cloudfront::{account_id()}:distribution/{dist_id}"

_account = None

def account_id():
    global _account
    if _account is None:
        _account = sts_client.get_caller_identity()["Account"]
    return _account

# COMMANDS

def create():
    net, cdn = buckets()
    if not net: return
    dist_id = find_distribution()
    if dist_id:
        print(f"distribution: exists ({dist_id})")
    else:
        oac_id = ensure_oac()
        region = bucket_region(net)
        config = desired_config(net, cdn, region, oac_id)
        resp = cf_client.create_distribution(DistributionConfig=config)
        dist_id = resp["Distribution"]["Id"]
        print(f"distribution: created ({dist_id})")
    save_env("MRLY_ID", dist_id)
    for bucket in (net, cdn):
        ensure_bucket_policy(bucket, [dist_arn(dist_id)])
    wait_deployed(dist_id)
    _, domain, _ = dist_status(dist_id)
    print(f"deployed: https://{domain}")

def flip():
    dist_id = os.environ.get("MRLY_ID")
    if not dist_id:
        print("cloudfront (add to .env: MRLY_ID)")
        return
    cert = find_cert()
    if not cert:
        print(f"no issued ACM certificate covers {APEX}")
        return
    zone_id = find_zone()
    if not zone_id:
        print(f"no hosted zone for {APEX}")
        return
    desired_aliases = {"Quantity": len(ALIASES), "Items": ALIASES}
    for attempt in range(20):
        resp = cf_client.get_distribution_config(Id=dist_id)
        config = resp["DistributionConfig"]
        if matches(desired_aliases, config["Aliases"]): break
        config["Aliases"] = desired_aliases
        config["ViewerCertificate"] = {
            "ACMCertificateArn": cert,
            "SSLSupportMethod": "sni-only",
            "MinimumProtocolVersion": "TLSv1.2_2021",
            "CloudFrontDefaultCertificate": False,
        }
        try:
            cf_client.update_distribution(Id=dist_id, DistributionConfig=config, IfMatch=resp["ETag"])
            print(f"  {dist_id}: aliases attached")
            break
        except cf_client.exceptions.CNAMEAlreadyExists:
            print("  aliases held elsewhere, retrying...")
            time.sleep(15)
    _, domain, _ = dist_status(dist_id)
    for name in ALIASES:
        upsert_alias(zone_id, name, domain)
    print("flipped")

def harden():
    dist_id = os.environ.get("MRLY_ID")
    if not dist_id:
        print("cloudfront (add to .env: MRLY_ID)")
        return
    pid, pstatus = ensure_policy(SECURITY_POLICY)
    print(f"  policy mrlynet-security: {pstatus}")
    resp = cf_client.get_distribution_config(Id=dist_id)
    config = resp["DistributionConfig"]
    if config["DefaultCacheBehavior"].get("ResponseHeadersPolicyId") == pid:
        print(f"  {dist_id}: already hardened")
        return
    config["DefaultCacheBehavior"]["ResponseHeadersPolicyId"] = pid
    cf_client.update_distribution(Id=dist_id, DistributionConfig=config, IfMatch=resp["ETag"])
    print(f"  {dist_id}: mrlynet-security attached")

def check():
    net, cdn = buckets()
    if not net: return
    dist_id = find_distribution()
    cert = find_cert()
    zone_id = find_zone()
    print(f"cert: {'found' if cert else 'MISSING'}")
    print(f"zone: {zone_id or 'MISSING'}")
    if dist_id:
        status, domain, config = dist_status(dist_id)
        aliases = config["Aliases"].get("Items", [])
        hardened = bool(config["DefaultCacheBehavior"].get("ResponseHeadersPolicyId"))
        print(f"distribution {dist_id}: {status}, aliases {aliases or 'none'}, hardened {hardened}, https://{domain}")
    else:
        print("distribution: not created")
    if zone_id:
        records = r53_client.list_resource_record_sets(HostedZoneId=zone_id)["ResourceRecordSets"]
        for record in records:
            if record["Type"] in ("A", "AAAA", "CNAME"):
                target = record.get("AliasTarget", {}).get("DNSName") or ",".join(r["Value"] for r in record.get("ResourceRecords", []))
                print(f"dns {record['Name']} {record['Type']} -> {target}")

# TERMINAL

def help():
    commands = [
        ("check", "report cert, zone, distribution, and dns state"),
        ("create", "create the mrly distribution (site + cdn/*), set bucket policies"),
        ("flip", "point the aliases and route53 records at the distribution"),
        ("harden", "attach the security headers policy"),
    ]
    width = max(len(name) for name, _ in commands)
    print("cloudfront")
    print()
    for name, desc in commands:
        print(f"  {name:<{width}}  {desc}")
    print()

def terminal():
    match sys.argv[1:]:
        case ["check"]: check()
        case ["create"]: create()
        case ["flip"]: flip()
        case ["harden"]: harden()
        case _: help()

# AWS

import boto3
from botocore.exceptions import ClientError

cf_client = boto3.client("cloudfront")
s3_client = boto3.client("s3")
r53_client = boto3.client("route53")
sts_client = boto3.client("sts")
acm_client = boto3.client("acm", region_name="us-east-1")

if __name__ == "__main__":
    terminal()
