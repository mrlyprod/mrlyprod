import json

from common import ROLES, account, gate, pick, role_arn, say, verb, verdict
from lib import REGION, aws, maybe, need

BASIC_EXECUTION = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
TRUSTED = ["lambda.amazonaws.com", "scheduler.amazonaws.com"]

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


def log_resources(spec):
    if "*" in spec["logs"]:
        return "arn:aws:logs:*:*:*"
    return [
        f"arn:aws:logs:{REGION}:{account()}:log-group:/aws/lambda/{name}:*"
        for name in spec["logs"]
    ]


def policy(spec):
    write = [need(key) for key in spec["write"]]
    read = [need(key) for key in spec["read"]]
    statements = [
        {
            "Effect": "Allow",
            "Action": [
                "logs:CreateLogGroup",
                "logs:CreateLogStream",
                "logs:PutLogEvents",
            ],
            "Resource": log_resources(spec),
        }
    ]
    if write:
        statements.append(
            {
                "Effect": "Allow",
                "Action": [
                    "s3:GetObject",
                    "s3:PutObject",
                    "s3:DeleteObject",
                    "s3:AbortMultipartUpload",
                    "s3:ListMultipartUploadParts",
                ],
                "Resource": [f"arn:aws:s3:::{name}/*" for name in write],
            }
        )
    if read:
        statements.append(
            {
                "Effect": "Allow",
                "Action": ["s3:GetObject"],
                "Resource": [f"arn:aws:s3:::{name}/*" for name in read],
            }
        )
    statements.append(
        {
            "Effect": "Allow",
            "Action": ["s3:ListBucket", "s3:GetBucketLocation"],
            "Resource": [f"arn:aws:s3:::{name}" for name in write + read],
        }
    )
    if spec.get("metrics"):
        statements.append(
            {
                "Effect": "Allow",
                "Action": [
                    "cloudwatch:GetMetricData",
                    "logs:FilterLogEvents",
                    "logs:DescribeLogGroups",
                ],
                "Resource": "*",
            }
        )
    statements.append(
        {
            "Effect": "Allow",
            "Action": "lambda:InvokeFunction",
            "Resource": [
                f"arn:aws:lambda:{REGION}:{account()}:function:{name}"
                for name in spec["invoke"]
            ],
        }
    )
    return {"Version": "2012-10-17", "Statement": statements}


def trusted_services(role):
    services = []
    for statement in role["AssumeRolePolicyDocument"].get("Statement", []):
        service = statement.get("Principal", {}).get("Service", [])
        services += [service] if isinstance(service, str) else list(service)
    return services


def check():
    name = pick(ROLES, "role")
    spec = ROLES[name]
    caller = maybe("sts", "get-caller-identity")
    verdict(bool(caller), "caller", (caller or {}).get("Arn", "no credentials"))
    found = maybe("iam", "get-role", "--role-name", name)
    if not found:
        verdict(False, "role", f"{name} absent, run role")
        return
    services = trusted_services(found["Role"])
    verdict(
        set(TRUSTED) <= set(services), "role trust", ", ".join(services) or "nothing"
    )
    inline = aws("iam", "list-role-policies", "--role-name", name)["PolicyNames"]
    verdict(spec["policy"] in inline, "role inline policy", ", ".join(inline) or "none")
    attached = aws("iam", "list-attached-role-policies", "--role-name", name)
    arns = [item["PolicyArn"] for item in attached["AttachedPolicies"]]
    verdict(
        (BASIC_EXECUTION in arns) == spec["basic"],
        "role basic execution",
        ", ".join(arns) or "none",
    )
    if spec["policy"] not in inline:
        return
    live = aws(
        "iam", "get-role-policy", "--role-name", name, "--policy-name", spec["policy"]
    )
    text = json.dumps(live["PolicyDocument"])
    for key in spec["write"] + spec["read"]:
        name_of = need(key)
        verdict(
            name_of in text,
            f"role bucket {name_of}",
            "granted" if name_of in text else "not granted",
        )
    for target in spec["invoke"]:
        verdict(
            f"function:{target}" in text,
            f"role invoke {target}",
            "granted" if f"function:{target}" in text else "not granted",
        )


def show():
    for name, spec in ROLES.items():
        found = maybe("iam", "get-role", "--role-name", name)
        say(
            f"{name:<22} {'present' if found else 'none yet'}  inline {spec['policy']}  basic {'yes' if spec['basic'] else 'no'}"
        )


def role():
    name = pick(ROLES, "role")
    spec = ROLES[name]
    found = maybe("iam", "get-role", "--role-name", name)
    write = [need(key) for key in spec["write"]]
    read = [need(key) for key in spec["read"]]
    if not gate(
        f"role {name}",
        [
            f"{'update' if found else 'create'} role {name}",
            f"trust {', '.join(TRUSTED)}",
            f"inline {spec['policy']}: logs on {', '.join(spec['logs'])}",
            f"read and write s3 on {', '.join(write) or 'nothing'}; read only on {', '.join(read) or 'nothing'}",
            f"lambda:InvokeFunction on {', '.join(spec['invoke'])}",
            f"{'attach' if spec['basic'] else 'no'} AWSLambdaBasicExecutionRole",
        ],
    ):
        return
    if found:
        aws(
            "iam",
            "update-assume-role-policy",
            "--role-name",
            name,
            "--policy-document",
            json.dumps(TRUST_POLICY),
        )
        say(f"role {name} trust updated")
    else:
        aws(
            "iam",
            "create-role",
            "--role-name",
            name,
            "--assume-role-policy-document",
            json.dumps(TRUST_POLICY),
        )
        say(f"role {name} created")
    aws(
        "iam",
        "put-role-policy",
        "--role-name",
        name,
        "--policy-name",
        spec["policy"],
        "--policy-document",
        json.dumps(policy(spec)),
    )
    say(f"inline {spec['policy']} written")
    if spec["basic"]:
        aws(
            "iam",
            "attach-role-policy",
            "--role-name",
            name,
            "--policy-arn",
            BASIC_EXECUTION,
        )
        say("AWSLambdaBasicExecutionRole attached")
    say(role_arn(name))


VERBS = {
    "list": (show, "", "each role"),
    "check": (
        check,
        "<role>",
        "caller, trust, inline policy, basic execution, bucket and invoke grants",
    ),
    "role": (role, "<role>", "create or update the role and its inline policy"),
}

ABOUT = "IAM roles from ROLES in common.py. A mutating verb prints a PLAN; --yes applies it."

if __name__ == "__main__":
    verb(VERBS, ABOUT)()
