import json
import time

from common import (
    APEX, CACHE_OPTIMIZED, CERT_ARN, EAST, HEADERS_NAME, NET_ZONE, OAC_NAME,
    ORIGIN_ID, SITE_ALIASES, SITE_BUCKET, SITE_ORIGIN, ROUTER_NAME,
    aws, blob, cf_function, distribution_arn, function_arn, gate,
    headers_policy, ids, maybe, oac_id, records, say, site_distribution, site_id,
    verb, verdict,
)

# CODE

ROUTER_CODE = """function handler(event) {
    var request = event.request;
    var uri = request.uri;
    var host = request.headers.host ? request.headers.host.value : '';
    if (host === 'www.mrly.net') {
        return redirect('https://mrly.net' + uri);
    }
    if (uri.charAt(uri.length - 1) === '/') {
        request.uri = uri + 'index.html';
        return request;
    }
    var last = uri.substring(uri.lastIndexOf('/') + 1);
    if (last.indexOf('.') === -1) {
        return redirect(uri + '/');
    }
    return request;
}

function redirect(location) {
    return {
        statusCode: 301,
        statusDescription: 'Moved Permanently',
        headers: { 'location': { value: location } }
    };
}
"""

CSP = "; ".join([
    "default-src 'self'",
    "script-src 'self' 'wasm-unsafe-eval'",
    "style-src 'self' 'unsafe-inline'",
    "img-src 'self' data:",
    "font-src 'self' data:",
    "connect-src 'self'",
    "object-src 'none'",
    "base-uri 'self'",
    "frame-ancestors 'none'",
])

# CHECK

def check_cert():
    data = maybe("acm", "describe-certificate", "--certificate-arn", CERT_ARN, region=EAST)
    if not data:
        verdict(False, "cert", "not found in us-east-1")
        return
    cert = data["Certificate"]
    names = set(cert["SubjectAlternativeNames"])
    want = {APEX, f"*.{APEX}"}
    verdict(cert["Status"] == "ISSUED", "cert status", cert["Status"])
    verdict(want <= names, "cert names", ", ".join(sorted(names)))
    for option in cert["DomainValidationOptions"]:
        good = option["ValidationStatus"] == "SUCCESS"
        verdict(good, f"cert {option['DomainName']}", option["ValidationStatus"])

def check_zone(zone, domain):
    zone_ns = set()
    for record in records(zone):
        if record["Type"] == "NS" and record["Name"] == domain + ".":
            zone_ns = {v["Value"].rstrip(".").lower() for v in record["ResourceRecords"]}
    data = maybe("route53domains", "get-domain-detail", "--domain-name", domain, region=EAST)
    if not data:
        verdict(False, f"ns {domain}", "registrar detail unavailable")
        return
    registrar = {n["Name"].rstrip(".").lower() for n in data["Nameservers"]}
    same = bool(zone_ns) and zone_ns == registrar
    verdict(same, f"ns {domain}", "zone matches registrar" if same else "drift")

def check_bucket():
    data = maybe("s3api", "get-public-access-block", "--bucket", SITE_BUCKET)
    block = (data or {}).get("PublicAccessBlockConfiguration") or {}
    flags = ["BlockPublicAcls", "IgnorePublicAcls", "BlockPublicPolicy", "RestrictPublicBuckets"]
    shut = all(block.get(flag) for flag in flags)
    verdict(shut, "bucket public access", "all four blocked" if shut else "open flags")
    site = maybe("s3api", "get-bucket-website", "--bucket", SITE_BUCKET)
    verdict(site is None, "bucket website", "none" if site is None else "website config present")
    policy = maybe("s3api", "get-bucket-policy", "--bucket", SITE_BUCKET)
    dist = site_id()
    if not policy:
        verdict(False, "bucket policy", "absent, run bucket after distribution")
        return
    text = policy["Policy"]
    good = "cloudfront.amazonaws.com" in text and (dist and distribution_arn(dist) in text)
    detail = "grants cloudfront s3:GetObject" if good else "does not match the distribution"
    verdict(bool(good), "bucket policy", detail)

def check_distribution():
    item = site_distribution()
    if not item:
        verdict(False, "distribution", "no distribution carries mrly.net")
        return
    aliases = (item.get("Aliases") or {}).get("Items") or []
    state = f"{item['Id']} {item['Status']} {item['DomainName']}"
    verdict(item["Enabled"], "distribution state", state)
    verdict(set(aliases) == set(SITE_ALIASES), "distribution aliases", ", ".join(sorted(aliases)))

def check_pieces():
    router = function_arn(ROUTER_NAME)
    verdict(bool(router), "router function", router or "absent")
    policy = headers_policy(HEADERS_NAME)
    verdict(bool(policy), "headers policy", policy or "absent")
    oac = oac_id(OAC_NAME)
    verdict(bool(oac), "origin access control", oac or "absent")

def check_dns():
    item = site_distribution()
    target = (item["DomainName"] + ".").lower() if item else ""
    found = {}
    mail = 0
    for record in records(NET_ZONE):
        if record["Type"] in ["MX", "TXT"]: mail += 1
        alias = record.get("AliasTarget")
        if not alias: continue
        found[(record["Name"].lower(), record["Type"])] = alias["DNSName"].lower()
    for name in [APEX + ".", f"www.{APEX}."]:
        for kind in ["A", "AAAA"]:
            have = found.get((name, kind))
            good = bool(target) and have == target
            verdict(good, f"dns {kind} {name}", have or "absent")
    verdict(mail >= 3, "dns mail records", f"{mail} MX and TXT records present")

def check_wiring():
    item = site_distribution()
    if not item: return
    data = aws("cloudfront", "get-distribution-config", "--id", item["Id"])
    config = data["DistributionConfig"]
    behavior = config["DefaultCacheBehavior"]
    linked = (behavior.get("FunctionAssociations") or {}).get("Items") or []
    names = [f["FunctionARN"].rsplit("/", 1)[-1] for f in linked]
    verdict(names == [ROUTER_NAME], "router wired", ", ".join(names) or "none")
    verdict(behavior.get("ResponseHeadersPolicyId") == headers_policy(HEADERS_NAME), "headers wired", behavior.get("ResponseHeadersPolicyId") or "none")
    origin = config["Origins"]["Items"][0]
    verdict(origin["DomainName"] == SITE_ORIGIN, "origin", origin["DomainName"])
    errors = (config.get("CustomErrorResponses") or {}).get("Items") or []
    pages = {(e["ErrorCode"], e["ResponsePagePath"], e["ResponseCode"]) for e in errors}
    verdict(pages == {(403, "/404.html", "404"), (404, "/404.html", "404")}, "error pages", f"{len(errors)} rules")
    cert = (config.get("ViewerCertificate") or {}).get("ACMCertificateArn")
    verdict(cert == CERT_ARN, "cert wired", cert or "none")

def check():
    ids()
    say()
    check_cert()
    check_zone(NET_ZONE, APEX)
    check_bucket()
    check_pieces()
    check_distribution()
    check_wiring()
    check_dns()

# BUCKET

def bucket():
    dist = site_id()
    steps = [
        f"block all public access on {SITE_BUCKET}",
        f"suspend versioning on {SITE_BUCKET} if enabled",
        f"delete any website config on {SITE_BUCKET}",
    ]
    if dist:
        steps.append(f"policy: s3:GetObject to cloudfront for {distribution_arn(dist)}")
    else:
        steps.append("policy: SKIPPED, no distribution yet, rerun bucket after it")
    if not gate("bucket", steps): return
    aws("s3api", "put-public-access-block", "--bucket", SITE_BUCKET,
        "--public-access-block-configuration",
        "BlockPublicAcls=true,IgnorePublicAcls=true,"
        "BlockPublicPolicy=true,RestrictPublicBuckets=true")
    state = maybe("s3api", "get-bucket-versioning", "--bucket", SITE_BUCKET) or {}
    if state.get("Status") == "Enabled":
        aws("s3api", "put-bucket-versioning", "--bucket", SITE_BUCKET,
            "--versioning-configuration", "Status=Suspended")
    maybe("s3api", "delete-bucket-website", "--bucket", SITE_BUCKET)
    if not dist:
        say("bucket ready, policy pending the distribution")
        return
    policy = {
        "Version": "2012-10-17",
        "Statement": [{
            "Sid": "AllowCloudFrontServicePrincipal",
            "Effect": "Allow",
            "Principal": {"Service": "cloudfront.amazonaws.com"},
            "Action": "s3:GetObject",
            "Resource": f"arn:aws:s3:::{SITE_BUCKET}/*",
            "Condition": {"StringEquals": {"AWS:SourceArn": distribution_arn(dist)}},
        }],
    }
    aws("s3api", "put-bucket-policy", "--bucket", SITE_BUCKET, "--policy", json.dumps(policy))
    say(f"bucket {SITE_BUCKET} sealed to {dist}")

# FUNCTION

def put_function(name, code, comment):
    data = maybe("cloudfront", "describe-function", "--name", name)
    config = json.dumps({"Comment": comment, "Runtime": "cloudfront-js-2.0"})
    if data:
        tag = data["ETag"]
        updated = blob(code, ".js", lambda ref: aws(
            "cloudfront", "update-function", "--name", name, "--if-match", tag,
            "--function-config", config, "--function-code", ref))
        tag = updated["ETag"]
    else:
        created = blob(code, ".js", lambda ref: aws(
            "cloudfront", "create-function", "--name", name,
            "--function-config", config, "--function-code", ref))
        tag = created["ETag"]
    published = aws("cloudfront", "publish-function", "--name", name, "--if-match", tag)
    arn = published["FunctionSummary"]["FunctionMetadata"]["FunctionARN"]
    say(f"{name} live {arn}")
    return arn

def function():
    exists = cf_function(ROUTER_NAME)
    if not gate("function", [
        f"{'update' if exists else 'create'} cloudfront function {ROUTER_NAME}",
        "runtime cloudfront-js-2.0 on viewer-request",
        "www.mrly.net 301s to https://mrly.net",
        "trailing slash appends index.html, extensionless path 301s to a slash",
        "publish the new version",
    ]): return
    put_function(ROUTER_NAME, ROUTER_CODE, "mrly.net router")

# HEADERS

def headers():
    config = {
        "Name": HEADERS_NAME,
        "Comment": "mrly.net security headers",
        "SecurityHeadersConfig": {
            "StrictTransportSecurity": {
                "Override": True,
                "IncludeSubdomains": True,
                "Preload": False,
                "AccessControlMaxAgeSec": 31536000,
            },
            "ContentTypeOptions": {"Override": True},
            "ReferrerPolicy": {
                "Override": True,
                "ReferrerPolicy": "strict-origin-when-cross-origin",
            },
            "ContentSecurityPolicy": {"Override": True, "ContentSecurityPolicy": CSP},
        },
    }
    found = headers_policy(HEADERS_NAME)
    if not gate("headers", [
        f"{'update' if found else 'create'} response headers policy {HEADERS_NAME}",
        "hsts one year, includeSubdomains, no preload",
        "nosniff, referrer strict-origin-when-cross-origin",
        f"csp {CSP}",
    ]): return
    if found:
        current = aws("cloudfront", "get-response-headers-policy", "--id", found)
        aws("cloudfront", "update-response-headers-policy", "--id", found,
            "--if-match", current["ETag"], "--response-headers-policy-config", json.dumps(config))
    else:
        made = aws("cloudfront", "create-response-headers-policy",
                   "--response-headers-policy-config", json.dumps(config))
        found = made["ResponseHeadersPolicy"]["Id"]
    say(f"{HEADERS_NAME} {found}")

# DISTRIBUTION

def desired(oac, router, policy, caller):
    return {
        "CallerReference": caller,
        "Comment": "mrly",
        "Enabled": True,
        "Staging": False,
        "DefaultRootObject": "index.html",
        "PriceClass": "PriceClass_All",
        "HttpVersion": "http2and3",
        "IsIPV6Enabled": True,
        "WebACLId": "",
        "Aliases": {"Quantity": len(SITE_ALIASES), "Items": list(SITE_ALIASES)},
        "Origins": {"Quantity": 1, "Items": [{
            "Id": ORIGIN_ID,
            "DomainName": SITE_ORIGIN,
            "OriginPath": "",
            "CustomHeaders": {"Quantity": 0},
            "S3OriginConfig": {"OriginAccessIdentity": ""},
            "OriginAccessControlId": oac,
            "ConnectionAttempts": 3,
            "ConnectionTimeout": 10,
        }]},
        "OriginGroups": {"Quantity": 0},
        "CacheBehaviors": {"Quantity": 0},
        "DefaultCacheBehavior": {
            "TargetOriginId": ORIGIN_ID,
            "ViewerProtocolPolicy": "redirect-to-https",
            "AllowedMethods": {
                "Quantity": 2,
                "Items": ["GET", "HEAD"],
                "CachedMethods": {"Quantity": 2, "Items": ["GET", "HEAD"]},
            },
            "Compress": True,
            "SmoothStreaming": False,
            "FieldLevelEncryptionId": "",
            "CachePolicyId": CACHE_OPTIMIZED,
            "ResponseHeadersPolicyId": policy,
            "TrustedSigners": {"Enabled": False, "Quantity": 0},
            "TrustedKeyGroups": {"Enabled": False, "Quantity": 0},
            "LambdaFunctionAssociations": {"Quantity": 0},
            "FunctionAssociations": {"Quantity": 1, "Items": [
                {"FunctionARN": router, "EventType": "viewer-request"},
            ]},
        },
        "CustomErrorResponses": {"Quantity": 2, "Items": [
            {"ErrorCode": code, "ResponsePagePath": "/404.html",
             "ResponseCode": "404", "ErrorCachingMinTTL": 60}
            for code in [403, 404]
        ]},
        "ViewerCertificate": {
            "ACMCertificateArn": CERT_ARN,
            "SSLSupportMethod": "sni-only",
            "MinimumProtocolVersion": "TLSv1.2_2021",
            "CloudFrontDefaultCertificate": False,
        },
        "Restrictions": {"GeoRestriction": {"RestrictionType": "none", "Quantity": 0}},
        "Logging": {"Enabled": False, "IncludeCookies": False, "Bucket": "", "Prefix": ""},
    }

def ensure_oac():
    found = oac_id(OAC_NAME)
    if found: return found
    config = {
        "Name": OAC_NAME,
        "Description": "mrly.net site origin",
        "SigningProtocol": "sigv4",
        "SigningBehavior": "always",
        "OriginAccessControlOriginType": "s3",
    }
    made = aws("cloudfront", "create-origin-access-control",
               "--origin-access-control-config", json.dumps(config))
    return made["OriginAccessControl"]["Id"]

def distribution():
    router = function_arn(ROUTER_NAME)
    policy = headers_policy(HEADERS_NAME)
    item = site_distribution()
    steps = [
        f"{'update' if item else 'create'} the mrly.net distribution",
        f"origin {SITE_ORIGIN} through oac {OAC_NAME}",
        f"aliases {', '.join(SITE_ALIASES)}",
        f"cert {CERT_ARN} TLSv1.2_2021 sni-only",
        f"router {router or 'MISSING, run function first'}",
        f"headers {policy or 'MISSING, run headers first'}",
        "403 and 404 to /404.html with code 404 and ttl 60",
    ]
    if not gate("distribution", steps): return
    if not router or not policy:
        raise SystemExit("refuse: run function and headers first")
    oac = ensure_oac()
    if item:
        current = aws("cloudfront", "get-distribution-config", "--id", item["Id"])
        config = desired(oac, router, policy, current["DistributionConfig"]["CallerReference"])
        aws("cloudfront", "update-distribution", "--id", item["Id"],
            "--if-match", current["ETag"], "--distribution-config", json.dumps(config))
        made = item
        say(f"distribution {item['Id']} {item['DomainName']} updated")
    else:
        config = desired(oac, router, policy, f"mrlynet-{int(time.time())}")
        data = aws("cloudfront", "create-distribution", "--distribution-config", json.dumps(config))
        made = data["Distribution"]
        say(f"distribution {made['Id']} {made['DomainName']} created")
    say(f"oac {oac}")
    say(f"MRLYNET_ID={made['Id']}")

# MAIN

VERBS = {
    "check": check,
    "bucket": bucket,
    "function": function,
    "headers": headers,
    "distribution": distribution,
}

if __name__ == "__main__":
    VERBS[verb(list(VERBS))]()
