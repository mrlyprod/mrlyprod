import json
import os
import subprocess
import sys
import tempfile

# PATHS

AWS_DIR = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.dirname(AWS_DIR)
DESK = os.path.dirname(REPO)
ENV_PATH = os.path.join(DESK, ".env")
SITE_DIR = os.path.join(REPO, "sites", "net")
DIST_DIR = os.path.join(SITE_DIR, "dist")

# ENV

def load_env():
    if not os.path.exists(ENV_PATH): return
    with open(ENV_PATH) as handle:
        for line in handle:
            line = line.strip()
            if not line or line.startswith("#"): continue
            key, _, value = line.partition("=")
            os.environ.setdefault(key.strip(), value.strip())

load_env()

def env(key, fallback):
    return os.environ.get(key) or fallback

# ACCOUNT

ACCOUNT = env("AWS_ACCOUNT_ID", "116981788437")
REGION = env("AWS_DEFAULT_REGION", "us-east-2")
EAST = "us-east-1"

# SITE

APEX = "mrly.net"
SITE_BUCKET = env("MRLYNET_BUCKET", "mrlynet")
SITE_ORIGIN = f"{SITE_BUCKET}.s3.{REGION}.amazonaws.com"
SITE_ALIASES = [APEX, f"www.{APEX}"]
CERT_DEFAULT = "arn:aws:acm:us-east-1:116981788437:certificate/9209d426-24b9-4a47-a39b-673247f6b706"
CERT_ARN = env("MRLYNET_CERT", CERT_DEFAULT)
ROUTER_NAME = "mrlynet-router"
HEADERS_NAME = "mrlynet-security"
OAC_NAME = "mrly"
ORIGIN_ID = "site"
CACHE_OPTIMIZED = "658327ea-f89d-4fab-a63d-7e88639e58f6"
CF_ZONE = "Z2FDTNDATAQYW2"

# ZONE

NET_ZONE = env("MRLYNET_ZONE", "Z00461542CJQXW1QV2Q96")

# DEAD

DEAD_DISTRIBUTIONS = ["E30KSTEYGLUN4K", "E2SLQ6B0E8H2QC", "EDJFOF4VXDD8Z", "E201GK07F7YIK9"]
DEAD_OACS = [
    "E1U6HVNDO9PS28", "EAH5AYJT9FW4Z", "E22H9YDXJLZF6",
    "ENB1SN8BYN2G7", "EV1WS5YI9NAUE", "E27ZKB0XZJDS1L",
]
DEAD_RECORDS = [
    ("web.mrly.net.", "d37gwkwtnlfmzk.cloudfront.net."),
    ("cdn.mrly.net.", "d3775rzk0rlx4d.cloudfront.net."),
    ("git.mrly.net.", "d3opj1z80crp07.cloudfront.net."),
    ("bot.mrly.net.", "da24blntn9vnu.cloudfront.net."),
]

# BUCKETS

BUCKETS = [
    "carlomitchener", "mrlybot", "mrlycdn", "mrlyconfig", "mrlydata",
    "mrlydev", "mrlygame", "mrlygit", "mrlynet", "mrlyprod",
    "mrlyshop", "mrlywear", "mrlyweb",
]

# LAMBDA

LAMBDA_NAME = env("MRLYGAME_FUNCTION", "mrlygame")
LAMBDA_LAYERS = ["mrlygame", "ffmpeg"]
STACK_NAME = "serverlessrepo-ffmpeg-lambda-layer"
ROLE_NAME = env("ROLE_NAME", "mrlyrole")
BOT_USER = "mrlybot"
KEEP_USER = "carlo"

# SHELL

def aws(*args, region=None):
    cmd = ["aws"] + [str(a) for a in args]
    if region: cmd += ["--region", region]
    cmd += ["--output", "json"]
    done = subprocess.run(cmd, capture_output=True, text=True)
    if done.returncode != 0:
        raise RuntimeError(" ".join(cmd) + "\n" + done.stderr.strip())
    text = done.stdout.strip()
    return json.loads(text) if text else None

def maybe(*args, region=None):
    try:
        return aws(*args, region=region)
    except RuntimeError:
        return None

def shell(*args, cwd=None):
    cmd = [str(a) for a in args]
    done = subprocess.run(cmd, capture_output=True, text=True, cwd=cwd)
    if done.returncode != 0:
        raise RuntimeError(" ".join(cmd) + "\n" + done.stderr.strip())
    return done.stdout

def stream(*args, cwd=None):
    done = subprocess.run([str(a) for a in args], cwd=cwd)
    if done.returncode != 0:
        raise RuntimeError(" ".join(str(a) for a in args))

def tmpfile(text, suffix):
    handle, path = tempfile.mkstemp(suffix=suffix)
    os.write(handle, text.encode())
    os.close(handle)
    return path

def blob(text, suffix, run):
    path = tmpfile(text, suffix)
    try:
        return run("fileb://" + path)
    finally:
        os.remove(path)

# SAY

def say(text=""):
    print(text)

def verdict(good, label, detail):
    say(f"{'GO' if good else 'HOLD':<4} {label:<24} {detail}")

def gate(title, steps):
    say(f"PLAN {title}")
    for step in steps: say(f"  {step}")
    if "--yes" in sys.argv: return True
    say("HOLD rerun with --yes to apply")
    return False

def verb(names):
    word = sys.argv[1] if len(sys.argv) > 1 else ""
    if word not in names:
        say("verbs: " + " ".join(names))
        raise SystemExit(1)
    return word

# RESOLVE

def distributions():
    data = aws("cloudfront", "list-distributions")
    return ((data or {}).get("DistributionList") or {}).get("Items") or []

def site_distribution():
    for item in distributions():
        aliases = (item.get("Aliases") or {}).get("Items") or []
        if APEX in aliases: return item
    return None

def site_id():
    item = site_distribution()
    return item["Id"] if item else ""

def distribution_arn(dist):
    return f"arn:aws:cloudfront::{ACCOUNT}:distribution/{dist}"

def oac_id(name):
    data = aws("cloudfront", "list-origin-access-controls")
    for item in ((data or {}).get("OriginAccessControlList") or {}).get("Items") or []:
        if item["Name"] == name: return item["Id"]
    return ""

def cf_function(name):
    data = maybe("cloudfront", "describe-function", "--name", name)
    return data["FunctionSummary"] if data else None

def function_arn(name):
    found = cf_function(name)
    return found["FunctionMetadata"]["FunctionARN"] if found else ""

def headers_policy(name):
    data = aws("cloudfront", "list-response-headers-policies", "--type", "custom")
    for item in ((data or {}).get("ResponseHeadersPolicyList") or {}).get("Items") or []:
        policy = item["ResponseHeadersPolicy"]
        if policy["ResponseHeadersPolicyConfig"]["Name"] == name: return policy["Id"]
    return ""

def records(zone):
    data = aws("route53", "list-resource-record-sets", "--hosted-zone-id", zone)
    return data["ResourceRecordSets"]

def guard_net(zone):
    if zone == NET_ZONE:
        raise SystemExit(f"refuse: {NET_ZONE} carries live mail records and is never written")

def ids():
    say(f"account       {ACCOUNT}")
    say(f"region        {REGION}")
    say(f"bucket        {SITE_BUCKET}")
    say(f"origin        {SITE_ORIGIN}")
    say(f"cert          {CERT_ARN}")
    say(f"net zone      {NET_ZONE} (read only)")
    say(f"distribution  {site_id() or 'none yet'}")
    say(f"oac           {oac_id(OAC_NAME) or 'none yet'}")
    say(f"router        {function_arn(ROUTER_NAME) or 'none yet'}")
    say(f"headers       {headers_policy(HEADERS_NAME) or 'none yet'}")
