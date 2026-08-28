import os

import boto3

ROOT_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
ENV_PATH = os.path.join(os.path.dirname(ROOT_DIR), ".env")
APEX = "mrly.net"

# ENV

def load_env():
    if not os.path.exists(ENV_PATH): return
    with open(ENV_PATH) as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("#"): continue
            key, _, value = line.partition("=")
            os.environ.setdefault(key.strip(), value.strip())

def save_env(key, value):
    text = open(ENV_PATH).read() if os.path.exists(ENV_PATH) else ""
    if any(line.startswith(f"{key}=") for line in text.splitlines()): return
    with open(ENV_PATH, "a") as f:
        if text and not text.endswith("\n"): f.write("\n")
        f.write(f"{key}={value}\n")
    os.environ[key] = value

load_env()

# CLIENTS

def s3_client(): return boto3.client("s3")

def cf_client(): return boto3.client("cloudfront")

def r53_client(): return boto3.client("route53")

def sts_client(): return boto3.client("sts")

def acm_client(): return boto3.client("acm", region_name="us-east-1")

def budgets_client(): return boto3.client("budgets", region_name="us-east-1")

def iam_client(): return boto3.client("iam")

def ce_client(): return boto3.client("ce", region_name="us-east-1")

def cur_client(): return boto3.client("cur", region_name="us-east-1")

def lambda_client(): return boto3.client("lambda")

def scheduler_client(): return boto3.client("scheduler")
