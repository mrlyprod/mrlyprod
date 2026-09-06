import json

from common import (
    ACCOUNT, DEV_BUCKET, NET_FUNCTION, PROD_BUCKET, REGION, ROLE_NAME, SITE_BUCKET,
    aws, gate, maybe, say, verb,
)

# DESIRED

POLICY_NAME = "mrlypolicy"
TRUSTED = ["lambda.amazonaws.com", "scheduler.amazonaws.com"]
WRITE_BUCKETS = [SITE_BUCKET, DEV_BUCKET]
READ_BUCKETS = [PROD_BUCKET]
LOG_ARN = f"arn:aws:logs:{REGION}:{ACCOUNT}:log-group:/aws/lambda/{NET_FUNCTION}:*"
FUNCTION_ARN = f"arn:aws:lambda:{REGION}:{ACCOUNT}:function:{NET_FUNCTION}"

TRUST = {
    "Version": "2012-10-17",
    "Statement": [{
        "Effect": "Allow",
        "Principal": {"Service": TRUSTED},
        "Action": "sts:AssumeRole",
    }],
}

def role_arn():
    return f"arn:aws:iam::{ACCOUNT}:role/{ROLE_NAME}"

def policy():
    return {
        "Version": "2012-10-17",
        "Statement": [
            {
                "Effect": "Allow",
                "Action": ["logs:CreateLogGroup", "logs:CreateLogStream", "logs:PutLogEvents"],
                "Resource": LOG_ARN,
            },
            {
                "Effect": "Allow",
                "Action": ["s3:GetObject", "s3:PutObject", "s3:DeleteObject"],
                "Resource": [f"arn:aws:s3:::{name}/*" for name in WRITE_BUCKETS],
            },
            {
                "Effect": "Allow",
                "Action": ["s3:GetObject"],
                "Resource": [f"arn:aws:s3:::{name}/*" for name in READ_BUCKETS],
            },
            {
                "Effect": "Allow",
                "Action": ["s3:ListBucket", "s3:GetBucketLocation"],
                "Resource": [f"arn:aws:s3:::{name}" for name in WRITE_BUCKETS + READ_BUCKETS],
            },
            {
                "Effect": "Allow",
                "Action": "lambda:InvokeFunction",
                "Resource": FUNCTION_ARN,
            },
        ],
    }

# VERBS

def role():
    found = maybe("iam", "get-role", "--role-name", ROLE_NAME)
    if not gate("role", [
        f"{'update' if found else 'create'} role {ROLE_NAME}",
        f"trust {', '.join(TRUSTED)}",
        f"inline {POLICY_NAME}: logs on {NET_FUNCTION}",
        f"read and write s3 on {', '.join(WRITE_BUCKETS)}",
        f"read only s3 on {', '.join(READ_BUCKETS)}",
        f"lambda:InvokeFunction on {NET_FUNCTION} so the scheduler can fire it",
    ]): return
    if found:
        aws("iam", "update-assume-role-policy", "--role-name", ROLE_NAME,
            "--policy-document", json.dumps(TRUST))
        say(f"role {ROLE_NAME} trust updated")
    else:
        aws("iam", "create-role", "--role-name", ROLE_NAME,
            "--assume-role-policy-document", json.dumps(TRUST))
        say(f"role {ROLE_NAME} created")
    aws("iam", "put-role-policy", "--role-name", ROLE_NAME,
        "--policy-name", POLICY_NAME, "--policy-document", json.dumps(policy()))
    say(f"inline {POLICY_NAME} written")
    say(role_arn())

# MAIN

VERBS = {"role": role}

if __name__ == "__main__":
    VERBS[verb(list(VERBS))]()
