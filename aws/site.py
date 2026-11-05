import gzip
import json
import os
import subprocess
import sys
import time
from collections import Counter
from datetime import UTC, datetime, timedelta
from urllib.parse import unquote

from common import (
    CACHE_OPTIMIZED,
    CF_ZONE,
    SITES,
    account,
    aliases,
    blob,
    bucket,
    cert_arn,
    cf_function,
    distribution_arn,
    function_arn,
    gate,
    headers_policy,
    ids,
    oac_id,
    origin,
    pick,
    records,
    say,
    site_distribution,
    site_id,
    verb,
    verdict,
    zone_id,
)
from lib import DATA_DIR, EAST, aws, maybe, need, save_env

ROUTER_MAX = 10240
EDGE_MARK = "/*MOVED*/{}"
MISSING = "/404.html"


def read(path):
    with open(path) as handle:
        return handle.read()


def router_code(site, moved):
    path = os.path.join(site["dir"], site["edge"])
    source = read(path)
    if source.count(EDGE_MARK) != 1:
        raise SystemExit(f"refuse: {path} must hold the mark {EDGE_MARK} exactly once")
    return source.replace(EDGE_MARK, json.dumps(moved, separators=(",", ":")))


def moved_paths(site):
    path = os.path.join(site["dir"], "redirects.json")
    if not os.path.exists(path):
        return {}
    with open(path) as handle:
        rows = json.load(handle)
    out = {}
    for source, row in rows.items():
        to = row["to"]
        if not source.startswith("/") or not to.startswith("/"):
            raise SystemExit(
                f"refuse: redirect {source} -> {to} must be two absolute paths"
            )
        if source == to or source.startswith("/raw/"):
            raise SystemExit(
                f"refuse: redirect {source} -> {to} loops or sits under /raw/"
            )
        if to in rows:
            raise SystemExit(
                f"refuse: redirect {source} -> {to} chains into another redirect; point it at {rows[to]['to']}"
            )
        out[source] = to
    return out


def csp(site):
    return "; ".join(
        [
            "default-src 'self'",
            "script-src 'self' 'wasm-unsafe-eval'",
            "style-src 'self' 'unsafe-inline'",
            " ".join(["img-src 'self' data:"] + site["img"]),
            "font-src 'self' data:",
            "connect-src 'self'",
            "object-src 'none'",
            "base-uri 'self'",
            "frame-src 'none'",
            "frame-ancestors 'none'",
            " ".join(["form-action 'self'"] + site["form"]),
        ]
    )


def custom_headers(site):
    items = [
        {
            "Header": "Permissions-Policy",
            "Value": "camera=(), microphone=(), geolocation=(), payment=(), usb=()",
            "Override": True,
        },
        {
            "Header": "Cross-Origin-Opener-Policy",
            "Value": "same-origin",
            "Override": True,
        },
    ]
    return {"Quantity": len(items), "Items": items}


def removed_headers():
    return {
        "Quantity": 2,
        "Items": [{"Header": "Server"}, {"Header": "x-amz-server-side-encryption"}],
    }


def check_cert(site):
    arn = cert_arn(site)
    if not arn:
        verdict(False, "cert", "not found in us-east-1, run cert")
        return
    data = maybe("acm", "describe-certificate", "--certificate-arn", arn, region=EAST)
    if not data:
        verdict(False, "cert", "arn does not resolve in us-east-1")
        return
    cert = data["Certificate"]
    names = set(cert["SubjectAlternativeNames"])
    covered = site["apex"] in names and (
        f"www.{site['apex']}" in names or f"*.{site['apex']}" in names
    )
    verdict(cert["Status"] == "ISSUED", "cert status", cert["Status"])
    verdict(covered, "cert names", ", ".join(sorted(names)))
    for option in cert["DomainValidationOptions"]:
        good = option["ValidationStatus"] == "SUCCESS"
        verdict(good, f"cert {option['DomainName']}", option["ValidationStatus"])


def check_zone(site):
    zone = zone_id(site)
    apex = site["apex"]
    if not zone:
        verdict(False, "zone", f"no public zone for {apex}, run zone")
        return
    verdict(True, "zone", f"{zone} ({site['dns']})")
    zone_ns = set()
    for record in records(zone):
        if record["Type"] == "NS" and record["Name"] == apex + ".":
            zone_ns = {
                v["Value"].rstrip(".").lower() for v in record["ResourceRecords"]
            }
    data = maybe(
        "route53domains", "get-domain-detail", "--domain-name", apex, region=EAST
    )
    if not data:
        verdict(False, f"ns {apex}", "registrar detail unavailable")
        return
    registrar = {n["Name"].rstrip(".").lower() for n in data["Nameservers"]}
    same = bool(zone_ns) and zone_ns == registrar
    verdict(same, f"ns {apex}", "zone matches registrar" if same else "drift")


def check_bucket(site):
    name = bucket(site)
    data = maybe("s3api", "get-public-access-block", "--bucket", name)
    block = (data or {}).get("PublicAccessBlockConfiguration") or {}
    flags = [
        "BlockPublicAcls",
        "IgnorePublicAcls",
        "BlockPublicPolicy",
        "RestrictPublicBuckets",
    ]
    shut = all(block.get(flag) for flag in flags)
    verdict(shut, "bucket public access", "all four blocked" if shut else "open flags")
    web = maybe("s3api", "get-bucket-website", "--bucket", name)
    verdict(
        web is None,
        "bucket website",
        "none" if web is None else "website config present",
    )
    policy = maybe("s3api", "get-bucket-policy", "--bucket", name)
    dist = site_id(site)
    if not policy:
        verdict(False, "bucket policy", "absent, run bucket after distribution")
        return
    text = policy["Policy"]
    good = (
        "cloudfront.amazonaws.com" in text
        and bool(dist)
        and distribution_arn(dist) in text
    )
    detail = (
        "grants cloudfront s3:GetObject" if good else "does not match the distribution"
    )
    verdict(good, "bucket policy", detail)
    listed = "s3:ListBucket" in text
    verdict(
        listed,
        "bucket list",
        "a missing key is a 404" if listed else "a missing key is a 403, run bucket",
    )


def check_pieces(site):
    router = function_arn(site["router"])
    verdict(bool(router), "router function", router or "absent")
    policy = headers_policy(site["headers"])
    verdict(bool(policy), "headers policy", policy or "absent")
    if policy:
        live = aws("cloudfront", "get-response-headers-policy", "--id", policy)[
            "ResponseHeadersPolicy"
        ]["ResponseHeadersPolicyConfig"]
        secure = live["SecurityHeadersConfig"]
        want = csp(site)
        have = (secure.get("ContentSecurityPolicy") or {}).get(
            "ContentSecurityPolicy", ""
        )
        verdict(
            have == want,
            "headers csp",
            "matches the tree" if have == want else have or "absent",
        )
        verdict(
            bool((secure.get("StrictTransportSecurity") or {}).get("Preload")),
            "headers hsts preload",
            "on"
            if (secure.get("StrictTransportSecurity") or {}).get("Preload")
            else "off",
        )
        sent = {
            item["Header"]: item["Value"]
            for item in ((live.get("CustomHeadersConfig") or {}).get("Items") or [])
        }
        mine = {item["Header"]: item["Value"] for item in custom_headers(site)["Items"]}
        verdict(sent == mine, "headers extras", ", ".join(sorted(sent)) or "none")
        gone = sorted(
            item["Header"]
            for item in ((live.get("RemoveHeadersConfig") or {}).get("Items") or [])
        )
        verdict(
            gone == sorted(item["Header"] for item in removed_headers()["Items"]),
            "headers removed",
            ", ".join(gone) or "none",
        )
    oac = oac_id(site["oac"])
    verdict(bool(oac), "origin access control", oac or "absent")


def check_distribution(site):
    item = site_distribution(site)
    if not item:
        verdict(False, "distribution", f"no distribution carries {site['apex']}")
        return
    names = (item.get("Aliases") or {}).get("Items") or []
    state = f"{item['Id']} {item['Status']} {item['DomainName']}"
    verdict(item["Enabled"], "distribution state", state)
    verdict(
        set(names) == set(aliases(site)),
        "distribution aliases",
        ", ".join(sorted(names)),
    )


def check_wiring(site):
    item = site_distribution(site)
    if not item:
        return
    data = aws("cloudfront", "get-distribution-config", "--id", item["Id"])
    config = data["DistributionConfig"]
    behavior = config["DefaultCacheBehavior"]
    linked = (behavior.get("FunctionAssociations") or {}).get("Items") or []
    names = [f["FunctionARN"].rsplit("/", 1)[-1] for f in linked]
    verdict(names == [site["router"]], "router wired", ", ".join(names) or "none")
    policy = behavior.get("ResponseHeadersPolicyId")
    verdict(
        policy == headers_policy(site["headers"]), "headers wired", policy or "none"
    )
    first = config["Origins"]["Items"][0]
    verdict(first["DomainName"] == origin(site), "origin", first["DomainName"])
    verdict(
        first["OriginPath"] == site["origin_path"],
        "origin path",
        first["OriginPath"] or "none",
    )
    errors = (config.get("CustomErrorResponses") or {}).get("Items") or []
    pages = {(e["ErrorCode"], e["ResponsePagePath"], e["ResponseCode"]) for e in errors}
    want = {(404, MISSING, "404")}
    verdict(pages == want, "error pages", f"{len(errors)} rules")
    cert = (config.get("ViewerCertificate") or {}).get("ACMCertificateArn")
    verdict(cert == cert_arn(site), "cert wired", cert or "none")


def check_dns(site):
    zone = zone_id(site)
    if not zone:
        return
    item = site_distribution(site)
    target = (item["DomainName"] + ".").lower() if item else ""
    found = {}
    mail = 0
    for record in records(zone):
        if record["Type"] in ["MX", "TXT"]:
            mail += 1
        alias = record.get("AliasTarget")
        if not alias:
            continue
        found[(record["Name"].lower(), record["Type"])] = alias["DNSName"].lower()
    for name in aliases(site):
        for kind in ["A", "AAAA"]:
            have = found.get((name + ".", kind))
            good = bool(target) and have == target
            verdict(good, f"dns {kind} {name}", have or "absent")
    if site["dns"] == "locked":
        verdict(mail >= 3, "dns mail records", f"{mail} MX and TXT records present")


def check():
    site = SITES[pick(SITES, "site")]
    ids(site)
    say()
    check_zone(site)
    check_cert(site)
    check_bucket(site)
    check_pieces(site)
    check_distribution(site)
    check_wiring(site)
    check_logs(site)
    check_dns(site)


def unlocked(site):
    if site["dns"] == "locked":
        raise SystemExit(
            f"refuse: the {site['apex']} zone carries live mail records and is never written"
        )


def upsert(site, zone, changes):
    unlocked(site)
    batch = {"Comment": site["comment"], "Changes": changes}
    aws(
        "route53",
        "change-resource-record-sets",
        "--hosted-zone-id",
        zone,
        "--change-batch",
        json.dumps(batch),
    )


def zone():
    key = pick(SITES, "site")
    site = SITES[key]
    apex = site["apex"]
    found = zone_id(site)
    if found:
        if not gate(
            f"zone {key}",
            [
                f"zone {found} already carries {apex}, create nothing",
                f"save {site['zone']}",
            ],
        ):
            return
        save_env(site["zone"], found)
        return
    unlocked(site)
    if not gate(
        f"zone {key}",
        [
            f"create the public hosted zone for {apex}",
            "then point the registrar at its four nameservers",
            f"save {site['zone']}",
        ],
    ):
        return
    made = aws(
        "route53",
        "create-hosted-zone",
        "--name",
        apex,
        "--caller-reference",
        f"{key}-{int(time.time())}",
    )
    new = made["HostedZone"]["Id"].rsplit("/", 1)[-1]
    say(f"zone {new} created")
    for server in made["DelegationSet"]["NameServers"]:
        say(f"  ns {server}")
    save_env(site["zone"], new)


def validation_records(arn):
    for _ in range(30):
        data = aws("acm", "describe-certificate", "--certificate-arn", arn, region=EAST)
        options = data["Certificate"].get("DomainValidationOptions") or []
        if options and all("ResourceRecord" in option for option in options):
            return options
        time.sleep(5)
    raise SystemExit("refuse: acm never published the validation records")


def cert():
    key = pick(SITES, "site")
    site = SITES[key]
    apex = site["apex"]
    found = cert_arn(site)
    if found:
        if not gate(
            f"cert {key}",
            [f"reuse the acm certificate {found}", f"save {site['cert']}"],
        ):
            return
        save_env(site["cert"], found)
        return
    zone = zone_id(site)
    if not zone:
        raise SystemExit("refuse: run zone first")
    unlocked(site)
    if not gate(
        f"cert {key}",
        [
            f"request an acm certificate in {EAST}",
            f"names {', '.join(aliases(site))}, DNS validation",
            "write the validation CNAMEs into the zone",
            "wait for ISSUED",
            f"save {site['cert']}",
        ],
    ):
        return
    made = aws(
        "acm",
        "request-certificate",
        "--domain-name",
        apex,
        "--subject-alternative-names",
        f"www.{apex}",
        "--validation-method",
        "DNS",
        "--key-algorithm",
        "RSA_2048",
        region=EAST,
    )
    found = made["CertificateArn"]
    say(f"cert requested {found}")
    changes = []
    seen = set()
    for option in validation_records(found):
        record = option["ResourceRecord"]
        if record["Name"] in seen:
            continue
        seen.add(record["Name"])
        changes.append(
            {
                "Action": "UPSERT",
                "ResourceRecordSet": {
                    "Name": record["Name"],
                    "Type": record["Type"],
                    "TTL": 300,
                    "ResourceRecords": [{"Value": record["Value"]}],
                },
            }
        )
    upsert(site, zone, changes)
    say(f"{len(changes)} validation records written")
    for _ in range(60):
        data = aws(
            "acm", "describe-certificate", "--certificate-arn", found, region=EAST
        )
        status = data["Certificate"]["Status"]
        if status == "ISSUED":
            say("cert ISSUED")
            save_env(site["cert"], found)
            return
        if status != "PENDING_VALIDATION":
            raise SystemExit(f"refuse: certificate is {status}")
        say(f"cert {status}, waiting")
        time.sleep(20)
    raise SystemExit("cert still pending, rerun cert --yes")


def bucket_verb():
    key = pick(SITES, "site")
    site = SITES[key]
    name = bucket(site)
    dist = site_id(site)
    steps = [
        f"block all public access on {name}",
        f"suspend versioning on {name} if enabled",
        f"delete any website config on {name}",
    ]
    if dist:
        steps.append(
            f"policy: s3:GetObject and s3:ListBucket to cloudfront for {distribution_arn(dist)}, so a missing key is a 404 and a 403 means access broke"
        )
    else:
        steps.append("policy: SKIPPED, no distribution yet, rerun bucket after it")
    if not gate(f"bucket {key}", steps):
        return
    aws(
        "s3api",
        "put-public-access-block",
        "--bucket",
        name,
        "--public-access-block-configuration",
        "BlockPublicAcls=true,IgnorePublicAcls=true,BlockPublicPolicy=true,RestrictPublicBuckets=true",
    )
    state = maybe("s3api", "get-bucket-versioning", "--bucket", name) or {}
    if state.get("Status") == "Enabled":
        aws(
            "s3api",
            "put-bucket-versioning",
            "--bucket",
            name,
            "--versioning-configuration",
            "Status=Suspended",
        )
    maybe("s3api", "delete-bucket-website", "--bucket", name)
    if not dist:
        say("bucket ready, policy pending the distribution")
        return
    grant = lambda sid, action, resource: {
        "Sid": sid,
        "Effect": "Allow",
        "Principal": {"Service": "cloudfront.amazonaws.com"},
        "Action": action,
        "Resource": resource,
        "Condition": {"StringEquals": {"AWS:SourceArn": distribution_arn(dist)}},
    }
    policy = {
        "Version": "2012-10-17",
        "Statement": [
            grant(
                "AllowCloudFrontServicePrincipal",
                "s3:GetObject",
                f"arn:aws:s3:::{name}/*",
            ),
            grant("AllowCloudFrontList", "s3:ListBucket", f"arn:aws:s3:::{name}"),
        ],
    }
    aws("s3api", "put-bucket-policy", "--bucket", name, "--policy", json.dumps(policy))
    say(f"bucket {name} sealed to {dist}")


def put_function(name, code, comment):
    data = maybe("cloudfront", "describe-function", "--name", name)
    config = json.dumps({"Comment": comment, "Runtime": "cloudfront-js-2.0"})
    if data:
        tag = data["ETag"]
        updated = blob(
            code,
            ".js",
            lambda ref: aws(
                "cloudfront",
                "update-function",
                "--name",
                name,
                "--if-match",
                tag,
                "--function-config",
                config,
                "--function-code",
                ref,
            ),
        )
        tag = updated["ETag"]
    else:
        created = blob(
            code,
            ".js",
            lambda ref: aws(
                "cloudfront",
                "create-function",
                "--name",
                name,
                "--function-config",
                config,
                "--function-code",
                ref,
            ),
        )
        tag = created["ETag"]
    published = aws("cloudfront", "publish-function", "--name", name, "--if-match", tag)
    arn = published["FunctionSummary"]["FunctionMetadata"]["FunctionARN"]
    say(f"{name} live {arn}")
    return arn


def function():
    key = pick(SITES, "site")
    site = SITES[key]
    apex = site["apex"]
    moved = moved_paths(site)
    code = router_code(site, moved)
    size = len(code.encode())
    if size > ROUTER_MAX:
        raise SystemExit(
            f"refuse: the router is {size} bytes, over the {ROUTER_MAX} a cloudfront function holds; move the redirects to a KeyValueStore"
        )
    if "--print" in sys.argv:
        say(code)
        say(f"{size} of {ROUTER_MAX} bytes, {len(moved)} redirects")
        return
    exists = cf_function(site["router"])
    if not gate(
        f"function {key}",
        [
            f"{'update' if exists else 'create'} cloudfront function {site['router']} from {site['edge']}",
            "runtime cloudfront-js-2.0 on viewer-request",
            f"www.{apex} 301s to https://{apex}, query kept",
            f"{len(moved)} redirects from redirects.json 301 to their target, with or without the trailing slash, query kept",
            "/raw/ passes untouched: no index.html, no extensionless 301",
            "/git/ and every path under it rewrites to /index.html",
            "a last segment with a dot is a file and passes; /x/index.html misses in S3 and gets the error page",
            "a slash-ended path rewrites to /index.html",
            "a bare last segment 301s to its slash form, query kept",
            f"publish the new version, {size} of {ROUTER_MAX} bytes",
        ],
    ):
        return
    put_function(site["router"], code, f"{apex} router")


def headers():
    key = pick(SITES, "site")
    site = SITES[key]
    name = site["headers"]
    policy = csp(site)
    config = {
        "Name": name,
        "Comment": f"{site['apex']} security headers",
        "SecurityHeadersConfig": {
            "StrictTransportSecurity": {
                "Override": True,
                "IncludeSubdomains": True,
                "Preload": True,
                "AccessControlMaxAgeSec": 31536000,
            },
            "ContentTypeOptions": {"Override": True},
            "ReferrerPolicy": {
                "Override": True,
                "ReferrerPolicy": "strict-origin-when-cross-origin",
            },
            "ContentSecurityPolicy": {
                "Override": True,
                "ContentSecurityPolicy": policy,
            },
        },
        "CustomHeadersConfig": custom_headers(site),
        "RemoveHeadersConfig": removed_headers(),
    }
    found = headers_policy(name)
    if not gate(
        f"headers {key}",
        [
            f"{'update' if found else 'create'} response headers policy {name}",
            "hsts one year, includeSubdomains, preload",
            "nosniff, referrer strict-origin-when-cross-origin",
            "permissions-policy camera, microphone, geolocation, payment and usb all off",
            "cross-origin-opener-policy same-origin",
            "drop server and x-amz-server-side-encryption, which name the origin",
            f"csp {policy}",
        ],
    ):
        return
    if found:
        current = aws("cloudfront", "get-response-headers-policy", "--id", found)
        aws(
            "cloudfront",
            "update-response-headers-policy",
            "--id",
            found,
            "--if-match",
            current["ETag"],
            "--response-headers-policy-config",
            json.dumps(config),
        )
    else:
        made = aws(
            "cloudfront",
            "create-response-headers-policy",
            "--response-headers-policy-config",
            json.dumps(config),
        )
        found = made["ResponseHeadersPolicy"]["Id"]
    say(f"{name} {found}")


def desired(site, oac, router, policy, cert, caller):
    names = aliases(site)
    return {
        "CallerReference": caller,
        "Comment": site["comment"],
        "Enabled": True,
        "Staging": False,
        "DefaultRootObject": "index.html",
        "PriceClass": "PriceClass_All",
        "HttpVersion": "http2and3",
        "IsIPV6Enabled": True,
        "WebACLId": "",
        "Aliases": {"Quantity": len(names), "Items": names},
        "Origins": {
            "Quantity": 1,
            "Items": [
                {
                    "Id": "site",
                    "DomainName": origin(site),
                    "OriginPath": site["origin_path"],
                    "CustomHeaders": {"Quantity": 0},
                    "S3OriginConfig": {"OriginAccessIdentity": ""},
                    "OriginAccessControlId": oac,
                    "ConnectionAttempts": 3,
                    "ConnectionTimeout": 10,
                }
            ],
        },
        "OriginGroups": {"Quantity": 0},
        "CacheBehaviors": {"Quantity": 0},
        "DefaultCacheBehavior": {
            "TargetOriginId": "site",
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
            "FunctionAssociations": {
                "Quantity": 1,
                "Items": [{"FunctionARN": router, "EventType": "viewer-request"}],
            },
        },
        "CustomErrorResponses": {
            "Quantity": 1,
            "Items": [
                {
                    "ErrorCode": 404,
                    "ResponsePagePath": MISSING,
                    "ResponseCode": "404",
                    "ErrorCachingMinTTL": 60,
                },
            ],
        },
        "ViewerCertificate": {
            "ACMCertificateArn": cert,
            "SSLSupportMethod": "sni-only",
            "MinimumProtocolVersion": "TLSv1.2_2021",
            "CloudFrontDefaultCertificate": False,
        },
        "Restrictions": {"GeoRestriction": {"RestrictionType": "none", "Quantity": 0}},
        "Logging": {
            "Enabled": False,
            "IncludeCookies": False,
            "Bucket": "",
            "Prefix": "",
        },
    }


def ensure_oac(site):
    found = oac_id(site["oac"])
    if found:
        return found
    config = {
        "Name": site["oac"],
        "Description": f"{site['apex']} site origin",
        "SigningProtocol": "sigv4",
        "SigningBehavior": "always",
        "OriginAccessControlOriginType": "s3",
    }
    made = aws(
        "cloudfront",
        "create-origin-access-control",
        "--origin-access-control-config",
        json.dumps(config),
    )
    return made["OriginAccessControl"]["Id"]


def distribution():
    key = pick(SITES, "site")
    site = SITES[key]
    router = function_arn(site["router"])
    policy = headers_policy(site["headers"])
    cert = cert_arn(site)
    item = site_distribution(site)
    if not gate(
        f"distribution {key}",
        [
            f"{'update' if item else 'create'} the {site['apex']} distribution",
            f"origin {origin(site)}{site['origin_path']} through oac {site['oac']}",
            f"aliases {', '.join(aliases(site))}",
            f"cert {cert or 'MISSING, run cert first'} TLSv1.2_2021 sni-only",
            f"router {router or 'MISSING, run function first'}",
            f"headers {policy or 'MISSING, run headers first'}",
            f"404 to {MISSING} with ttl 60; a 403 stays a 403, so run bucket first",
        ],
    ):
        return
    if not router or not policy or not cert:
        raise SystemExit("refuse: run cert, function and headers first")
    oac = ensure_oac(site)
    if item:
        current = aws("cloudfront", "get-distribution-config", "--id", item["Id"])
        config = desired(
            site,
            oac,
            router,
            policy,
            cert,
            current["DistributionConfig"]["CallerReference"],
        )
        aws(
            "cloudfront",
            "update-distribution",
            "--id",
            item["Id"],
            "--if-match",
            current["ETag"],
            "--distribution-config",
            json.dumps(config),
        )
        made = item
        say(f"distribution {item['Id']} {item['DomainName']} updated")
    else:
        config = desired(site, oac, router, policy, cert, f"{key}-{int(time.time())}")
        data = aws(
            "cloudfront",
            "create-distribution",
            "--distribution-config",
            json.dumps(config),
        )
        made = data["Distribution"]
        say(f"distribution {made['Id']} {made['DomainName']} created")
    save_env(site["oac_env"], oac)
    save_env(site["id"], made["Id"])


LOG_PATH = "{yyyy}/{MM}/{dd}/{HH}"


def log_spec(key):
    site = SITES[key]
    if "logs" not in site:
        raise SystemExit(f"refuse: {key} has no logs row")
    spec = site["logs"]
    return (
        need(spec["bucket"]),
        spec["prefix"],
        spec["days"],
        f"{key}-cdn",
        f"{key}-cdn-s3",
    )


def log_grant(name, prefix):
    return {
        "Sid": "AWSLogsDeliveryWrite",
        "Effect": "Allow",
        "Principal": {"Service": "delivery.logs.amazonaws.com"},
        "Action": "s3:PutObject",
        "Resource": f"arn:aws:s3:::{name}/{prefix}/*",
        "Condition": {
            "StringEquals": {
                "s3:x-amz-acl": "bucket-owner-full-control",
                "aws:SourceAccount": account(),
            },
            "ArnLike": {
                "aws:SourceArn": f"arn:aws:logs:{EAST}:{account()}:delivery-source:*"
            },
        },
    }


def log_rule(prefix, days):
    return {
        "ID": prefix.replace("/", "-"),
        "Filter": {"Prefix": f"{prefix}/"},
        "Status": "Enabled",
        "Expiration": {"Days": days},
    }


def lifecycle(name):
    found = maybe("s3api", "get-bucket-lifecycle-configuration", "--bucket", name)
    return (found or {}).get("Rules") or []


def delivery(source):
    data = aws("logs", "describe-deliveries", region=EAST)
    return next(
        (
            item
            for item in data.get("deliveries") or []
            if item["deliverySourceName"] == source
        ),
        None,
    )


def check_logs(site):
    key = next(k for k, v in SITES.items() if v is site)
    if "logs" not in site:
        return
    name, prefix, days, source, _ = log_spec(key)
    found = delivery(source)
    verdict(
        bool(found),
        "logs delivery",
        f"{source} -> s3://{name}/{prefix}/" if found else "absent, run logs",
    )
    rule = next(
        (r for r in lifecycle(name) if r.get("ID") == log_rule(prefix, days)["ID"]),
        None,
    )
    kept = (rule or {}).get("Expiration", {}).get("Days")
    verdict(kept == days, "logs expiry", f"{kept} days" if kept else "absent, run logs")


def logs_verb():
    key = pick(SITES, "site")
    site = SITES[key]
    name, prefix, days, source, target = log_spec(key)
    dist = site_id(site)
    if not dist:
        raise SystemExit("refuse: no distribution yet")
    found = delivery(source)
    if not gate(
        f"logs {key}",
        [
            f"policy on {name}: delivery.logs.amazonaws.com may put under {prefix}/, other statements kept",
            f"lifecycle on {name}: {prefix}/ expires after {days} days, other rules kept",
            f"delivery source {source}: ACCESS_LOGS of {dist}, in {EAST}",
            f"delivery destination {target}: s3://{name}/{prefix}/ as json lines",
            f"delivery {source} -> {target}, partitioned {LOG_PATH}"
            + (", exists" if found else ""),
        ],
    ):
        return
    held = maybe("s3api", "get-bucket-policy", "--bucket", name)
    policy = (
        json.loads(held["Policy"])
        if held
        else {"Version": "2012-10-17", "Statement": []}
    )
    grant = log_grant(name, prefix)
    policy["Statement"] = [
        one for one in policy["Statement"] if one.get("Sid") != grant["Sid"]
    ] + [grant]
    aws("s3api", "put-bucket-policy", "--bucket", name, "--policy", json.dumps(policy))
    rule = log_rule(prefix, days)
    rules = [one for one in lifecycle(name) if one.get("ID") != rule["ID"]] + [rule]
    aws(
        "s3api",
        "put-bucket-lifecycle-configuration",
        "--bucket",
        name,
        "--lifecycle-configuration",
        json.dumps({"Rules": rules}),
    )
    aws(
        "logs",
        "put-delivery-source",
        "--name",
        source,
        "--resource-arn",
        distribution_arn(dist),
        "--log-type",
        "ACCESS_LOGS",
        region=EAST,
    )
    dest = maybe("logs", "get-delivery-destination", "--name", target, region=EAST)
    if not dest:
        dest = aws(
            "logs",
            "put-delivery-destination",
            "--name",
            target,
            "--output-format",
            "json",
            "--delivery-destination-configuration",
            json.dumps({"destinationResourceArn": f"arn:aws:s3:::{name}/{prefix}"}),
            region=EAST,
        )
    arn = dest["deliveryDestination"]["arn"]
    if not found:
        aws(
            "logs",
            "create-delivery",
            "--delivery-source-name",
            source,
            "--delivery-destination-arn",
            arn,
            "--s3-delivery-configuration",
            json.dumps({"suffixPath": LOG_PATH, "enableHiveCompatiblePath": False}),
            region=EAST,
        )
    say(
        f"logs {source} -> s3://{name}/{prefix}/, kept {days} days; the first files land within minutes"
    )


def lines(path):
    with open(path, "rb") as handle:
        raw = handle.read()
    text = gzip.decompress(raw) if raw[:2] == b"\x1f\x8b" else raw
    for line in text.decode().splitlines():
        if line.startswith("{"):
            yield json.loads(line)


def misses():
    key = pick(SITES, "site")
    name, prefix, _, _, _ = log_spec(key)
    hours = int(sys.argv[3]) if len(sys.argv) > 3 and sys.argv[3].isdigit() else 24
    here = os.path.join(DATA_DIR, "logs", key)
    subprocess.run(
        [
            "aws",
            "s3",
            "sync",
            f"s3://{name}/{prefix}/",
            here,
            "--delete",
            "--only-show-errors",
        ],
        check=True,
    )
    since = (datetime.now(UTC) - timedelta(hours=hours)).strftime("%Y-%m-%d %H:%M:%S")
    total = 0
    bad = Counter()
    agents = Counter()
    for root, _, files in os.walk(here):
        for file in files:
            for row in lines(os.path.join(root, file)):
                if f"{row.get('date')} {row.get('time')}" < since:
                    continue
                total += 1
                status = int(row.get("sc-status") or 0)
                if status < 400:
                    continue
                bad[(status, row.get("cs-method"), row.get("cs-uri-stem"))] += 1
                agents[unquote(row.get("cs(User-Agent)") or "-")[:80]] += 1
    count = sum(bad.values())
    say(
        f"{key} last {hours} h: {total} requests, {count} at 4xx or 5xx ({100 * count / total if total else 0:.1f} %)"
    )
    for (status, method, uri), n in bad.most_common(25):
        say(f"{n:>6}  {status} {method:<7} {uri}")
    say("agents")
    for agent, n in agents.most_common(8):
        say(f"{n:>6}  {agent}")


def records_verb():
    key = pick(SITES, "site")
    site = SITES[key]
    zone = zone_id(site)
    item = site_distribution(site)
    if not zone:
        raise SystemExit("refuse: run zone first")
    if not item:
        raise SystemExit("refuse: run distribution first")
    unlocked(site)
    target = item["DomainName"]
    names = aliases(site)
    if not gate(
        f"records {key}",
        [
            f"zone {zone}",
            f"A and AAAA alias for {', '.join(names)} to {target}",
            f"alias target zone {CF_ZONE}, no health check",
        ],
    ):
        return
    changes = []
    for name in names:
        for kind in ["A", "AAAA"]:
            changes.append(
                {
                    "Action": "UPSERT",
                    "ResourceRecordSet": {
                        "Name": name,
                        "Type": kind,
                        "AliasTarget": {
                            "HostedZoneId": CF_ZONE,
                            "DNSName": target,
                            "EvaluateTargetHealth": False,
                        },
                    },
                }
            )
    upsert(site, zone, changes)
    say(f"{len(changes)} alias records point at {target}")


VERBS = {
    "check": (
        check,
        "<site>",
        "ids, zone, cert, bucket, pieces, distribution, wiring, logs, dns: GO or HOLD per line",
    ),
    "zone": (
        zone,
        "<site>",
        "find the Route53 zone, create it only if absent, save its id; a locked dns zone is never written",
    ),
    "cert": (
        cert,
        "<site>",
        "request the us-east-1 cert for apex and www, DNS validation, wait for ISSUED, save the arn",
    ),
    "bucket": (
        bucket_verb,
        "<site>",
        "block public access, suspend versioning, drop website config, seal to the distribution; rerun after distribution",
    ),
    "function": (
        function,
        "<site> [--print]",
        "publish the router from the site's edge.js with redirects.json pasted in; --print shows it and calls nothing",
    ),
    "headers": (
        headers,
        "<site>",
        "the security headers policy: hsts preload, nosniff, referrer, permissions, COOP, a static CSP; Server dropped",
    ),
    "distribution": (
        distribution,
        "<site>",
        "create or update the distribution; refuses until cert, function and headers exist",
    ),
    "records": (
        records_verb,
        "<site>",
        "apex and www A and AAAA aliases at the distribution",
    ),
    "logs": (
        logs_verb,
        "<site>",
        "CloudFront standard logs to the site's logs row in S3, with a lifecycle rule that expires them",
    ),
    "misses": (
        misses,
        "<site> [hours]",
        "read-only: sync the logs to data/logs/<site>/ and print the 4xx and 5xx of the last 24 h (or hours)",
    ),
}

ABOUT = "Sites from SITES in common.py: S3, CloudFront, ACM and Route53. A mutating verb prints a PLAN; --yes applies it. A site from nothing: zone, cert, bucket, function, headers, distribution, bucket, records, check."

if __name__ == "__main__":
    verb(VERBS, ABOUT)()
