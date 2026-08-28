import json
import os
import sys
import time
import uuid

from common import APEX, save_env

# DESIRED

CACHING_OPTIMIZED = "658327ea-f89d-4fab-a63d-7e88639e58f6"
CLOUDFRONT_ZONE = "Z2FDTNDATAQYW2"

CDN = f"https://cdn.{APEX}"

CSP = f"default-src 'self'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self'; img-src 'self' data: {CDN}; media-src 'self' {CDN}; font-src 'self' data: {CDN}; connect-src 'self' {CDN}; object-src 'none'; base-uri 'self'; frame-ancestors 'self'"

CORS = {
    "AccessControlAllowOrigins": {"Quantity": 1, "Items": ["*"]},
    "AccessControlAllowHeaders": {"Quantity": 1, "Items": ["*"]},
    "AccessControlAllowMethods": {"Quantity": 2, "Items": ["GET", "HEAD"]},
    "AccessControlAllowCredentials": False,
    "OriginOverride": True,
}

def security_policy(name, csp):
    return {
        "Name": name,
        "SecurityHeadersConfig": {
            "ContentSecurityPolicy": {"Override": True, "ContentSecurityPolicy": csp},
            "ContentTypeOptions": {"Override": True},
            "ReferrerPolicy": {"Override": True, "ReferrerPolicy": "strict-origin-when-cross-origin"},
        },
    }

def cors_policy(name, csp):
    return {**security_policy(name, csp), "CorsConfig": CORS}

SPA_ERRORS = {
    "Quantity": 2,
    "Items": [
        {"ErrorCode": 403, "ResponsePagePath": "/index.html", "ResponseCode": "200", "ErrorCachingMinTTL": 300},
        {"ErrorCode": 404, "ResponsePagePath": "/index.html", "ResponseCode": "200", "ErrorCachingMinTTL": 300},
    ],
}

MRLYNET_POLICY = security_policy("mrlynet-security", CSP)
MRLYCDN_POLICY = cors_policy("mrlycdn-security", CSP)

TARGETS = {
    "net": {"aliases": [APEX, f"www.{APEX}"], "comment": "mrly", "oac": "mrly", "bucket_env": "MRLYNET_BUCKET", "id_env": "MRLYNET_ID", "errors": SPA_ERRORS, "policy": MRLYNET_POLICY},
    "git": {"aliases": [f"git.{APEX}"], "comment": "mrlygit", "oac": "mrlygit", "bucket_env": "MRLYGIT_BUCKET", "id_env": "MRLYGIT_ID", "errors": SPA_ERRORS, "policy": MRLYNET_POLICY},
    "web": {"aliases": [f"web.{APEX}"], "comment": "mrlyweb", "oac": "mrlyweb", "bucket_env": "MRLYWEB_BUCKET", "id_env": "MRLYWEB_ID", "errors": SPA_ERRORS, "policy": MRLYNET_POLICY},
    "cdn": {"aliases": [f"cdn.{APEX}"], "comment": "mrlycdn", "oac": "mrlycdn", "bucket_env": "MRLYCDN_BUCKET", "id_env": "MRLYCDN_ID", "errors": {"Quantity": 0}, "policy": MRLYCDN_POLICY},
    "bot": {"aliases": [f"bot.{APEX}"], "comment": "mrlybot", "oac": "mrlybot", "bucket_env": "MRLYBOT_BUCKET", "id_env": "MRLYBOT_ID", "errors": SPA_ERRORS, "policy": MRLYNET_POLICY},
}

TARGET = None

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

def desired_config(bucket, region, oac_id):
    origin_id = TARGET["comment"]
    return {
        "CallerReference": str(uuid.uuid4()),
        "Comment": TARGET["comment"],
        "Enabled": True,
        "DefaultRootObject": "index.html",
        "HttpVersion": "http2and3",
        "IsIPV6Enabled": True,
        "PriceClass": "PriceClass_All",
        "Aliases": {"Quantity": 0},
        "Origins": {"Quantity": 1, "Items": [origin(origin_id, bucket, region, oac_id)]},
        "DefaultCacheBehavior": behavior(origin_id),
        "CacheBehaviors": {"Quantity": 0},
        "CustomErrorResponses": TARGET["errors"],
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
    dist_id = os.environ.get(TARGET["id_env"])
    if dist_id:
        try:
            cf_client.get_distribution(Id=dist_id)
            return dist_id
        except cf_client.exceptions.NoSuchDistribution:
            print(f"  {TARGET['id_env']}={dist_id} not found, searching by comment...")
    paginator = cf_client.get_paginator("list_distributions")
    for page in paginator.paginate():
        for item in page["DistributionList"].get("Items", []):
            if item["Comment"] == TARGET["comment"]:
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

def target_bucket():
    name = os.environ.get(TARGET["bucket_env"])
    if not name:
        print(f"cloudfront (add to .env: {TARGET['bucket_env']})")
    return name

def target_id():
    dist_id = os.environ.get(TARGET["id_env"])
    if not dist_id:
        print(f"cloudfront (add to .env: {TARGET['id_env']})")
    return dist_id

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
            if item["Name"] == TARGET["oac"]:
                return item["Id"]
        marker = olist.get("NextMarker")
        if not marker: break
    resp = cf_client.create_origin_access_control(OriginAccessControlConfig={
        "Name": TARGET["oac"],
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
    bucket = target_bucket()
    if not bucket: return
    dist_id = find_distribution()
    if dist_id:
        print(f"distribution: exists ({dist_id})")
    else:
        oac_id = ensure_oac()
        region = bucket_region(bucket)
        config = desired_config(bucket, region, oac_id)
        resp = cf_client.create_distribution(DistributionConfig=config)
        dist_id = resp["Distribution"]["Id"]
        print(f"distribution: created ({dist_id})")
    save_env(TARGET["id_env"], dist_id)
    ensure_bucket_policy(bucket, [dist_arn(dist_id)])
    wait_deployed(dist_id)
    _, domain, _ = dist_status(dist_id)
    print(f"deployed: https://{domain}")

def flip():
    dist_id = target_id()
    if not dist_id: return
    cert = find_cert()
    if not cert:
        print(f"no issued ACM certificate covers {APEX}")
        return
    zone_id = find_zone()
    if not zone_id:
        print(f"no hosted zone for {APEX}")
        return
    aliases = TARGET["aliases"]
    desired_aliases = {"Quantity": len(aliases), "Items": aliases}
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
    else:
        print(f"  {dist_id}: aliases still held elsewhere, giving up")
        return
    _, domain, _ = dist_status(dist_id)
    for name in aliases:
        upsert_alias(zone_id, name, domain)
    print("flipped")

def harden():
    dist_id = target_id()
    if not dist_id: return
    policy = TARGET["policy"]
    pid, pstatus = ensure_policy(policy)
    print(f"  policy {policy['Name']}: {pstatus}")
    resp = cf_client.get_distribution_config(Id=dist_id)
    config = resp["DistributionConfig"]
    if config["DefaultCacheBehavior"].get("ResponseHeadersPolicyId") == pid:
        print(f"  {dist_id}: already hardened")
        return
    config["DefaultCacheBehavior"]["ResponseHeadersPolicyId"] = pid
    cf_client.update_distribution(Id=dist_id, DistributionConfig=config, IfMatch=resp["ETag"])
    print(f"  {dist_id}: {policy['Name']} attached")

def errors():
    dist_id = target_id()
    if not dist_id: return
    resp = cf_client.get_distribution_config(Id=dist_id)
    config = resp["DistributionConfig"]
    if _canon(config["CustomErrorResponses"]) == _canon(TARGET["errors"]):
        print(f"  {dist_id}: error responses already current")
        return
    config["CustomErrorResponses"] = TARGET["errors"]
    cf_client.update_distribution(Id=dist_id, DistributionConfig=config, IfMatch=resp["ETag"])
    print(f"  {dist_id}: error responses updated")

def prune():
    dist_id = target_id()
    if not dist_id: return
    resp = cf_client.get_distribution_config(Id=dist_id)
    config = resp["DistributionConfig"]
    behaviors = config["CacheBehaviors"].get("Items", [])
    kept = [b for b in behaviors if b["PathPattern"] != "cdn/*"]
    used = {config["DefaultCacheBehavior"]["TargetOriginId"]} | {b["TargetOriginId"] for b in kept}
    origins = config["Origins"].get("Items", [])
    kept_origins = [o for o in origins if o["Id"] in used]
    if len(kept) == len(behaviors) and len(kept_origins) == len(origins):
        print(f"  {dist_id}: already pruned")
        return
    config["CacheBehaviors"] = {"Quantity": len(kept), "Items": kept} if kept else {"Quantity": 0}
    config["Origins"] = {"Quantity": len(kept_origins), "Items": kept_origins}
    cf_client.update_distribution(Id=dist_id, DistributionConfig=config, IfMatch=resp["ETag"])
    print(f"  {dist_id}: cdn origin and cdn/* behavior dropped")

def check():
    bucket = target_bucket()
    if not bucket: return
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
        ("create", "create the target distribution, set the bucket policy"),
        ("flip", "point the aliases and route53 records at the distribution"),
        ("harden", "attach the security headers policy"),
        ("errors", "sync the custom error responses"),
        ("prune", "drop the cdn origin and /cdn/* behavior from the distribution"),
    ]
    width = max(len(name) for name, _ in commands)
    print(f"cloudfront <{'|'.join(TARGETS)}> <command>")
    print()
    for name, desc in commands:
        print(f"  {name:<{width}}  {desc}")
    print()

def terminal():
    global TARGET
    args = sys.argv[1:]
    if not args or args[0] not in TARGETS:
        help()
        return
    TARGET = TARGETS[args[0]]
    args = args[1:]
    match args:
        case ["check"]: check()
        case ["create"]: create()
        case ["flip"]: flip()
        case ["harden"]: harden()
        case ["errors"]: errors()
        case ["prune"]: prune()
        case _: help()

# AWS

import common
from botocore.exceptions import ClientError

cf_client = common.cf_client()
s3_client = common.s3_client()
r53_client = common.r53_client()
sts_client = common.sts_client()
acm_client = common.acm_client()

if __name__ == "__main__":
    terminal()
