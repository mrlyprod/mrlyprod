import argparse
import os
import sys

from lib import (
    COM,
    EAST,
    ENGINE,
    HERE,
    NET,
    ORG,
    REGION,
    UI,
    aws,
    env,
    maybe,
    need,
    tmpfile,
)

CACHE_OPTIMIZED = "658327ea-f89d-4fab-a63d-7e88639e58f6"
CF_ZONE = "Z2FDTNDATAQYW2"


def account():
    return need("AWS_ACCOUNT_ID")


SITES = {
    "mrly": {
        "apex": "mrly.net",
        "bucket": "MRLYNET_BUCKET",
        "origin_path": "",
        "zone": "MRLYNET_ZONE",
        "cert": "MRLYNET_CERT",
        "id": "MRLYNET_ID",
        "oac_env": "MRLYNET_OAC",
        "oac": "mrly",
        "router": "mrlynet-router",
        "headers": "mrlynet-security",
        "comment": "mrly",
        "img": [],
        "form": [],
        "dns": "locked",
        "root": NET,
        "dir": NET,
        "edge": os.path.join(ENGINE, "hosts", "cloud", "edge.js"),
        "logs": {"bucket": "MRLYDEV_BUCKET", "prefix": "logs/mrly", "days": 14},
    },
    "org": {
        "apex": "mrlyprod.org",
        "bucket": "MRLYORG_BUCKET",
        "origin_path": "",
        "zone": "MRLYORG_ZONE",
        "cert": "MRLYORG_CERT",
        "id": "MRLYORG_ID",
        "oac_env": "MRLYORG_OAC",
        "oac": "mrlyorg",
        "router": "mrlyorg-router",
        "headers": "mrlyorg-security",
        "comment": "mrlyorg",
        "img": [],
        "form": [],
        "dns": "locked",
        "root": ORG,
        "dir": os.path.join(ORG, "site"),
        "edge": os.path.join(ENGINE, "hosts", "cloud", "edge.js"),
    },
    "com": {
        "apex": "mrlyprod.com",
        "bucket": "MRLYCOM_BUCKET",
        "origin_path": "",
        "zone": "MRLYCOM_ZONE",
        "cert": "MRLYCOM_CERT",
        "id": "MRLYCOM_ID",
        "oac_env": "MRLYCOM_OAC",
        "oac": "mrlycom",
        "router": "mrlycom-router",
        "headers": "mrlycom-security",
        "comment": "mrlycom",
        "img": [],
        "form": [],
        "dns": "open",
        "root": COM,
        "dir": os.path.join(COM, "site"),
        "edge": os.path.join(ENGINE, "hosts", "cloud", "edge.js"),
        "logs": {"bucket": "MRLYDEV_BUCKET", "prefix": "logs/com", "days": 14},
    },
}

ROLES = {
    "mrlyrole": {
        "policy": "mrlypolicy",
        "logs": ["mrlynet-site", "mrlynet-stats"],
        "write": ["MRLYNET_BUCKET", "MRLYDEV_BUCKET"],
        "read": ["MRLYPROD_BUCKET"],
        "invoke": ["mrlynet-site", "mrlynet-stats"],
        "metrics": True,
        "basic": False,
    },
}


def stats_sources(handler):
    return [(handler, "handler.py")]


FUNCTIONS = {
    "mrlynet-site": {
        "name": "mrlynet-site",
        "role": "mrlyrole",
        "runtime": "provided.al2023",
        "arch": "arm64",
        "handler": "handler.fetch",
        "memory": 3008,
        "timeout": 900,
        "storage": 2048,
        "concurrency": 1,
        "retries": 0,
        "retention": 14,
        "layers": ["BUN_LAYER_ARN", "MRLYNET_NODE_LAYER_ARN"],
        "env": [
            "MRLYNET_BUCKET",
            "MRLYDEV_BUCKET",
            "MRLYPROD_BUCKET",
            "GITHUB_READ_TOKEN",
        ],
        "extra": {},
        "logs": "Text",
        "package": ("bun", os.path.join(HERE, "net.ts")),
        "payload": {"source": "manual"},
        "arn": "MRLYNET_SITE_FUNCTION",
    },
    "mrlynet-stats": {
        "name": "mrlynet-stats",
        "role": "mrlyrole",
        "runtime": "python3.13",
        "arch": "arm64",
        "handler": "handler.handler",
        "memory": 512,
        "timeout": 120,
        "storage": 512,
        "concurrency": 1,
        "retries": 0,
        "retention": 14,
        "layers": [],
        "env": {"STATS_BUCKET": "MRLYNET_BUCKET", "STATS_DISTRIBUTION": "MRLYNET_ID"},
        "extra": {
            "STATS_FUNCTIONS": "mrlynet-site,mrlynet-stats",
            "STATS_KEY": "stats/stats.json",
            "STATS_SIZES": "site=",
            "STATS_COUNTS": "raw=raw/",
        },
        "logs": "JSON",
        "package": (
            "zip",
            stats_sources(os.path.join(ENGINE, "hosts", "cloud", "stats.py")),
        ),
        "payload": {},
        "arn": "MRLYNET_STATS_FUNCTION",
    },
}

SCHEDULES = {
    "mrlynet-site": {
        "name": "mrlynet-site",
        "function": "mrlynet-site",
        "rate": "rate(1 hour)",
        "input": {"source": "schedule"},
    },
    "mrlynet-stats": {
        "name": "mrlynet-stats",
        "function": "mrlynet-stats",
        "rate": "rate(1 hour)",
        "input": {},
    },
}

LAYERS = {
    "bun": {
        "kind": "bun",
        "name": "bun",
        "env": "BUN_LAYER_ARN",
        "runtimes": ["provided.al2", "provided.al2023"],
    },
    "mrlynet-node": {
        "kind": "node",
        "name": "mrlynet-node",
        "env": "MRLYNET_NODE_LAYER_ARN",
        "runtimes": ["provided.al2023"],
        "site": UI,
    },
}


def blob(text, suffix, run):
    path = tmpfile(text, suffix)
    try:
        return run("fileb://" + path)
    finally:
        os.remove(path)


def say(text=""):
    print(text)


def verdict(good, label, detail):
    say(f"{'GO' if good else 'HOLD':<4} {label:<26} {detail}")


def gate(title, steps):
    say(f"PLAN {title}")
    for step in steps:
        say(f"  {step}")
    if "--yes" in sys.argv:
        return True
    say("HOLD rerun with --yes to apply")
    return False


def verb(verbs, about):
    rows = {f"{name} {spec[1]}".strip(): spec[2] for name, spec in verbs.items()}
    width = max(len(row) for row in rows) + 2
    parser = argparse.ArgumentParser(
        prog=os.path.basename(sys.argv[0]),
        usage="%(prog)s <verb> [target] [--yes]",
        description=about,
        epilog="verbs:\n"
        + "\n".join(f"  {row:<{width}}{text}" for row, text in rows.items()),
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument("verb", choices=list(verbs), help=argparse.SUPPRESS)
    found, _ = parser.parse_known_args()
    return verbs[found.verb][0]


def given(label):
    if len(sys.argv) < 3 or sys.argv[2].startswith("--"):
        raise SystemExit(f"refuse: {label} is required")
    return sys.argv[2]


def pick(registry, label):
    if len(sys.argv) < 3 or sys.argv[2].startswith("--"):
        raise SystemExit(f"refuse: {label} is required (" + " ".join(registry) + ")")
    key = sys.argv[2]
    if key not in registry:
        raise SystemExit(f"refuse: {key} is not a {label} (" + " ".join(registry) + ")")
    return key


def bucket(site):
    return need(site["bucket"])


def site_env(site):
    return {**os.environ, "MRLY_ROOT": site["dir"]}


def origin(site):
    return f"{bucket(site)}.s3.{REGION}.amazonaws.com"


def aliases(site):
    return [site["apex"], f"www.{site['apex']}"]


def zone_id(site):
    if env(site["zone"]):
        return env(site["zone"])
    data = aws("route53", "list-hosted-zones")
    for item in data["HostedZones"]:
        if (
            item["Name"].rstrip(".") == site["apex"]
            and not item["Config"]["PrivateZone"]
        ):
            return item["Id"].rsplit("/", 1)[-1]
    return ""


def records(zone):
    data = aws("route53", "list-resource-record-sets", "--hosted-zone-id", zone)
    return data["ResourceRecordSets"]


def cert_arn(site):
    if env(site["cert"]):
        return env(site["cert"])
    data = aws("acm", "list-certificates", region=EAST)
    for item in data["CertificateSummaryList"]:
        if item["DomainName"] == site["apex"]:
            return item["CertificateArn"]
    return ""


def distributions():
    data = aws("cloudfront", "list-distributions")
    return ((data or {}).get("DistributionList") or {}).get("Items") or []


def site_distribution(site):
    for item in distributions():
        names = (item.get("Aliases") or {}).get("Items") or []
        if site["apex"] in names:
            return item
    return None


def site_id(site):
    item = site_distribution(site)
    return item["Id"] if item else ""


def distribution_arn(dist):
    return f"arn:aws:cloudfront::{account()}:distribution/{dist}"


def oac_id(name):
    data = aws("cloudfront", "list-origin-access-controls")
    for item in ((data or {}).get("OriginAccessControlList") or {}).get("Items") or []:
        if item["Name"] == name:
            return item["Id"]
    return ""


def cf_function(name):
    data = maybe("cloudfront", "describe-function", "--name", name)
    return data["FunctionSummary"] if data else None


def function_arn(name):
    found = cf_function(name)
    return found["FunctionMetadata"]["FunctionARN"] if found else ""


def headers_policy(name):
    data = aws("cloudfront", "list-response-headers-policies", "--type", "custom")
    for item in ((data or {}).get("ResponseHeadersPolicyList") or {}).get(
        "Items"
    ) or []:
        policy = item["ResponseHeadersPolicy"]
        if policy["ResponseHeadersPolicyConfig"]["Name"] == name:
            return policy["Id"]
    return ""


def role_arn(name):
    return f"arn:aws:iam::{account()}:role/{name}"


def lambda_arn(name):
    return f"arn:aws:lambda:{REGION}:{account()}:function:{name}"


def ids(site):
    say(f"account       {env('AWS_ACCOUNT_ID') or 'AWS_ACCOUNT_ID missing'}")
    say(f"region        {REGION}")
    say(f"apex          {site['apex']}")
    say(f"bucket        {bucket(site)}")
    say(f"origin        {origin(site)}{site['origin_path']}")
    say(f"zone          {zone_id(site) or 'none yet'} ({site['dns']})")
    say(f"cert          {cert_arn(site) or 'none yet'}")
    say(f"distribution  {site_id(site) or 'none yet'}")
    say(f"oac           {oac_id(site['oac']) or 'none yet'}")
    say(f"router        {function_arn(site['router']) or 'none yet'}")
    say(f"headers       {headers_policy(site['headers']) or 'none yet'}")
