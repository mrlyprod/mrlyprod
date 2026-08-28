import datetime
import json
import os
import sys

from common import save_env

# DESIRED

POLICIES = {
    "ReadOnlyAccess": "arn:aws:iam::aws:policy/ReadOnlyAccess",
    "AWSBillingReadOnlyAccess": "arn:aws:iam::aws:policy/AWSBillingReadOnlyAccess",
}

ADMIN = "arn:aws:iam::aws:policy/AdministratorAccess"

ROOT_ONLY = [
    "Account settings -> IAM user and role access to billing information -> Activate IAM Access",
    "Billing and Cost Management -> Cost Explorer -> enable once, backfills 12 months, up to 24h",
]

# IDENTITY

def identity():
    caller = sts_client.get_caller_identity()
    arn = caller["Arn"]
    user = arn.split("/")[-1] if ":user/" in arn else None
    return caller["Account"], user, arn

def attached(user):
    arns = []
    for page in iam_client.get_paginator("list_attached_user_policies").paginate(UserName=user):
        arns += [policy["PolicyArn"] for policy in page["AttachedPolicies"]]
    return arns

# PROBES

def window():
    today = datetime.date.today()
    return str(today.replace(day=1)), str(today + datetime.timedelta(days=1))

def probe(label, call):
    try:
        call()
        print(f"  {label}: yes")
    except Exception as error:
        response = getattr(error, "response", {})
        code = response.get("Error", {}).get("Code", type(error).__name__)
        print(f"  {label}: no ({code})")

def probes(account):
    start, end = window()
    probe("cost explorer", lambda: ce_client.get_cost_and_usage(
        TimePeriod={"Start": start, "End": end},
        Granularity="MONTHLY",
        Metrics=["UnblendedCost"],
    ))
    probe("budgets", lambda: budgets_client.describe_budgets(AccountId=account))
    probe("cost and usage reports", lambda: cur_client.describe_report_definitions())

# ROLES

POLICY_NAME = "mrlypolicy"
BASIC_EXECUTION = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
TRUSTED = ["lambda.amazonaws.com", "scheduler.amazonaws.com"]
BUCKET_ENVS = ("MRLYNET_BUCKET", "MRLYGIT_BUCKET", "MRLYWEB_BUCKET", "MRLYBOT_BUCKET", "MRLYCDN_BUCKET", "MRLYGAME_BUCKET")

TRUST_POLICY = {
    "Version": "2012-10-17",
    "Statement": [
        {
            "Effect": "Allow",
            "Principal": {"Service": TRUSTED},
            "Action": "sts:AssumeRole",
        }
    ],
}

def bucket_arns():
    arns = []
    for key in BUCKET_ENVS:
        bucket = os.environ.get(key)
        if not bucket: continue
        arns += [f"arn:aws:s3:::{bucket}", f"arn:aws:s3:::{bucket}/*"]
    return list(dict.fromkeys(arns))

def execution_policy(account, arns):
    return {
        "Version": "2012-10-17",
        "Statement": [
            {
                "Effect": "Allow",
                "Action": ["logs:CreateLogGroup", "logs:CreateLogStream", "logs:PutLogEvents"],
                "Resource": "arn:aws:logs:*:*:*",
            },
            {
                "Effect": "Allow",
                "Action": ["s3:GetObject", "s3:PutObject", "s3:DeleteObject", "s3:ListBucket"],
                "Resource": arns,
            },
            {
                "Effect": "Allow",
                "Action": "lambda:InvokeFunction",
                "Resource": f"arn:aws:lambda:*:{account}:function:mrly*",
            },
        ],
    }

def role_name():
    name = os.environ.get("ROLE_NAME")
    if not name:
        print("iam (add to .env: ROLE_NAME)")
    return name

def find_role(name):
    try:
        return iam_client.get_role(RoleName=name)["Role"]
    except iam_client.exceptions.NoSuchEntityException:
        return None

def trusted_services(role):
    services = []
    for statement in role["AssumeRolePolicyDocument"].get("Statement", []):
        service = statement.get("Principal", {}).get("Service", [])
        services += [service] if isinstance(service, str) else list(service)
    return services

def role_state():
    name = os.environ.get("ROLE_NAME")
    if not name:
        print("  role: add ROLE_NAME to .env")
        return
    role = find_role(name)
    if not role:
        print(f"  role {name}: missing, run role to create it")
        return
    inline = iam_client.list_role_policies(RoleName=name).get("PolicyNames", [])
    print(f"  role {name}: trusts {', '.join(trusted_services(role)) or 'nothing'}")
    print(f"  role {name}: inline {', '.join(inline) or 'none'}")

# COMMANDS

def role():
    name = role_name()
    if not name: return
    account, _, _ = identity()
    arns = bucket_arns()
    if not arns:
        print(f"iam (add to .env: at least one of {', '.join(BUCKET_ENVS)})")
        return
    found = find_role(name)
    if found:
        arn = found["Arn"]
        print(f"{name}: exists")
    else:
        arn = iam_client.create_role(RoleName=name, AssumeRolePolicyDocument=json.dumps(TRUST_POLICY))["Role"]["Arn"]
        print(f"{name}: created")
    iam_client.update_assume_role_policy(RoleName=name, PolicyDocument=json.dumps(TRUST_POLICY))
    print(f"  trust: {', '.join(TRUSTED)}")
    iam_client.put_role_policy(RoleName=name, PolicyName=POLICY_NAME, PolicyDocument=json.dumps(execution_policy(account, arns)))
    print(f"  {POLICY_NAME}: logs, s3 on {len(arns) // 2} buckets, lambda:InvokeFunction on mrly*")
    iam_client.attach_role_policy(RoleName=name, PolicyArn=BASIC_EXECUTION)
    print("  AWSLambdaBasicExecutionRole: attached")
    save_env("ROLE_ARN", arn)
    print(f"  ROLE_ARN={arn}")

def check(target=None):
    account, caller, arn = identity()
    user = target or caller
    print(arn)
    print()
    if not user:
        print("  policies: root has no attached policies, name a user to inspect one")
    elif ADMIN in attached(user):
        print(f"  {user}: AdministratorAccess, billing actions already covered")
    else:
        have = attached(user)
        for name, policy in POLICIES.items():
            print(f"  {user} {name}: {'attached' if policy in have else 'missing'}")
    print()
    role_state()
    print()
    if user and user != caller:
        print(f"probes below answer for the caller, not {user}")
    probes(account)
    print()
    print("console only, no api reads or writes this:")
    for step in ROOT_ONLY:
        print(f"  {step}")
    print()

def grant(target=None):
    account, caller, arn = identity()
    user = target or caller
    if not user:
        print(f"{arn}: root has no attached policies, name a user to grant one")
        return
    have = attached(user)
    if ADMIN in have:
        print(f"{user}: AdministratorAccess already covers billing, nothing to attach")
        return
    for name, policy in POLICIES.items():
        if policy in have:
            print(f"{user}: {name} already attached")
            continue
        iam_client.attach_user_policy(UserName=user, PolicyArn=policy)
        print(f"{user}: attached {name}")

# TERMINAL

def help():
    commands = [
        ("check [user]", "identity, attached policies, the execution role, live billing probes"),
        ("grant [user]", "attach the read and billing policies, defaults to the caller"),
        ("role", "create or update the lambda and scheduler execution role, write ROLE_ARN"),
    ]
    width = max(len(name) for name, _ in commands)
    print("iam")
    print()
    for name, desc in commands:
        print(f"  {name:<{width}}  {desc}")
    print()

def terminal():
    match sys.argv[1:]:
        case ["check"]: check()
        case ["check", user]: check(user)
        case ["grant"]: grant()
        case ["grant", user]: grant(user)
        case ["role"]: role()
        case _: help()

# AWS

import common

iam_client = common.iam_client()
sts_client = common.sts_client()
ce_client = common.ce_client()
budgets_client = common.budgets_client()
cur_client = common.cur_client()

if __name__ == "__main__":
    terminal()
