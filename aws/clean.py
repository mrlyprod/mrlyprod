import json

from common import (
    BOT_USER, BUCKETS, DEAD_DISTRIBUTIONS, DEAD_OACS, DEAD_RECORDS, KEEP_USER,
    LAMBDA_LAYERS, LAMBDA_NAME, NET_ZONE, REGION, ROLE_NAME, STACK_NAME,
    aws, gate, maybe, say, verb,
)

# DISTRIBUTIONS

def disable(dist):
    data = maybe("cloudfront", "get-distribution-config", "--id", dist)
    if not data:
        say(f"{dist} already gone")
        return False
    config = data["DistributionConfig"]
    if config["Enabled"]:
        config["Enabled"] = False
        aws("cloudfront", "update-distribution", "--id", dist,
            "--if-match", data["ETag"], "--distribution-config", json.dumps(config))
        say(f"{dist} disabled")
    return True

def distributions():
    if not gate("clean distributions", [
        f"disable then drop {', '.join(DEAD_DISTRIBUTIONS)}",
        "wait for each to reach Deployed first",
        f"then drop origin access controls {', '.join(DEAD_OACS)}",
    ]): return
    live = [dist for dist in DEAD_DISTRIBUTIONS if disable(dist)]
    for dist in live:
        say(f"waiting on {dist}")
        aws("cloudfront", "wait", "distribution-deployed", "--id", dist)
        data = aws("cloudfront", "get-distribution-config", "--id", dist)
        aws("cloudfront", "delete-distribution", "--id", dist, "--if-match", data["ETag"])
        say(f"{dist} gone")
    for oac in DEAD_OACS:
        data = maybe("cloudfront", "get-origin-access-control", "--id", oac)
        if not data:
            say(f"oac {oac} already gone")
            continue
        aws("cloudfront", "delete-origin-access-control", "--id", oac, "--if-match", data["ETag"])
        say(f"oac {oac} gone")

# RECORDS

def doomed(record):
    alias = record.get("AliasTarget")
    if not alias: return False
    if record["Type"] not in ["A", "AAAA"]: return False
    pair = (record["Name"].lower(), alias["DNSName"].lower())
    return pair in [(name.lower(), target.lower()) for name, target in DEAD_RECORDS]

def clean_records():
    data = aws("route53", "list-resource-record-sets", "--hosted-zone-id", NET_ZONE)
    hits = [r for r in data["ResourceRecordSets"] if doomed(r)]
    names = [f"{r['Name']} {r['Type']}" for r in hits]
    if not gate("clean records", [
        f"zone {NET_ZONE}, touch only these {len(hits)} records",
        *names,
        "every other record in this zone is refused",
    ]): return
    if not hits:
        say("nothing matches")
        return
    batch = {"Comment": "drop dead subdomain aliases",
             "Changes": [{"Action": "DELETE", "ResourceRecordSet": r} for r in hits]}
    aws("route53", "change-resource-record-sets", "--hosted-zone-id", NET_ZONE,
        "--change-batch", json.dumps(batch))
    for name in names: say(f"gone {name}")

# BUCKETS

def page(bucket):
    return aws("s3api", "list-object-versions", "--bucket", bucket, "--max-items", "500") or {}

def wipe(bucket):
    total = 0
    while True:
        data = page(bucket)
        keys = []
        for entry in (data.get("Versions") or []) + (data.get("DeleteMarkers") or []):
            keys.append({"Key": entry["Key"], "VersionId": entry["VersionId"]})
        if not keys: break
        aws("s3api", "delete-objects", "--bucket", bucket,
            "--delete", json.dumps({"Objects": keys, "Quiet": True}))
        total += len(keys)
    say(f"{bucket} emptied, {total} versions and markers gone")

def buckets():
    if not gate("clean buckets", [
        f"empty every object, version and delete marker in {len(BUCKETS)} buckets",
        ", ".join(BUCKETS),
        "every bucket itself stays",
    ]): return
    for bucket in BUCKETS: wipe(bucket)

# LAMBDA

def clean_lambda():
    listed = aws("scheduler", "list-schedules", region=REGION) or {}
    schedules = [s["Name"] for s in listed.get("Schedules", []) if LAMBDA_NAME in s["Name"]]
    if not gate("clean lambda", [
        f"disable then drop schedules {', '.join(schedules) or 'none'}",
        f"drop function {LAMBDA_NAME} in {REGION}",
        f"drop every version of layers {', '.join(LAMBDA_LAYERS)}",
        f"drop cloudformation stack {STACK_NAME}",
        f"strip and drop role {ROLE_NAME}",
    ]): return
    for name in schedules:
        found = aws("scheduler", "get-schedule", "--name", name, region=REGION)
        aws("scheduler", "update-schedule", "--name", name, "--state", "DISABLED",
            "--schedule-expression", found["ScheduleExpression"],
            "--flexible-time-window", json.dumps(found["FlexibleTimeWindow"]),
            "--target", json.dumps(found["Target"]), region=REGION)
        aws("scheduler", "delete-schedule", "--name", name, region=REGION)
        say(f"schedule {name} gone")
    if maybe("lambda", "get-function", "--function-name", LAMBDA_NAME, region=REGION):
        aws("lambda", "delete-function", "--function-name", LAMBDA_NAME, region=REGION)
        say(f"function {LAMBDA_NAME} gone")
    for layer in LAMBDA_LAYERS:
        listed = maybe("lambda", "list-layer-versions", "--layer-name", layer, region=REGION) or {}
        for version in listed.get("LayerVersions", []):
            aws("lambda", "delete-layer-version", "--layer-name", layer,
                "--version-number", version["Version"], region=REGION)
            say(f"layer {layer}:{version['Version']} gone")
    if maybe("cloudformation", "describe-stacks", "--stack-name", STACK_NAME, region=REGION):
        aws("cloudformation", "delete-stack", "--stack-name", STACK_NAME, region=REGION)
        aws("cloudformation", "wait", "stack-delete-complete",
            "--stack-name", STACK_NAME, region=REGION)
        say(f"stack {STACK_NAME} gone")
    if maybe("iam", "get-role", "--role-name", ROLE_NAME):
        attached = aws("iam", "list-attached-role-policies", "--role-name", ROLE_NAME)
        for policy in attached["AttachedPolicies"]:
            aws("iam", "detach-role-policy", "--role-name", ROLE_NAME,
                "--policy-arn", policy["PolicyArn"])
            say(f"detached {policy['PolicyName']}")
        inline = aws("iam", "list-role-policies", "--role-name", ROLE_NAME)
        for policy in inline["PolicyNames"]:
            aws("iam", "delete-role-policy", "--role-name", ROLE_NAME, "--policy-name", policy)
            say(f"dropped inline {policy}")
        aws("iam", "delete-role", "--role-name", ROLE_NAME)
        say(f"role {ROLE_NAME} gone")

# USER

def user():
    if BOT_USER == KEEP_USER:
        raise SystemExit(f"refuse: {KEEP_USER} is never touched")
    if not maybe("iam", "get-user", "--user-name", BOT_USER):
        say(f"{BOT_USER} already gone")
        return
    if not gate("clean user", [
        f"strip {BOT_USER}: access keys, signing certificates, mfa devices",
        "inline policies, attached policies, group memberships, login profile",
        f"then drop the user {BOT_USER}",
        f"{KEEP_USER} is never touched",
    ]): return
    for key in aws("iam", "list-access-keys", "--user-name", BOT_USER)["AccessKeyMetadata"]:
        aws("iam", "delete-access-key", "--user-name", BOT_USER,
            "--access-key-id", key["AccessKeyId"])
        say("access key gone")
    for cert in aws("iam", "list-signing-certificates", "--user-name", BOT_USER)["Certificates"]:
        aws("iam", "delete-signing-certificate", "--user-name", BOT_USER,
            "--certificate-id", cert["CertificateId"])
        say("signing certificate gone")
    for device in aws("iam", "list-mfa-devices", "--user-name", BOT_USER)["MFADevices"]:
        aws("iam", "deactivate-mfa-device", "--user-name", BOT_USER,
            "--serial-number", device["SerialNumber"])
        maybe("iam", "delete-virtual-mfa-device", "--serial-number", device["SerialNumber"])
        say("mfa device removed")
    for policy in aws("iam", "list-user-policies", "--user-name", BOT_USER)["PolicyNames"]:
        aws("iam", "delete-user-policy", "--user-name", BOT_USER, "--policy-name", policy)
        say(f"dropped inline {policy}")
    listed = aws("iam", "list-attached-user-policies", "--user-name", BOT_USER)
    for policy in listed["AttachedPolicies"]:
        aws("iam", "detach-user-policy", "--user-name", BOT_USER,
            "--policy-arn", policy["PolicyArn"])
        say(f"detached {policy['PolicyName']}")
    for group in aws("iam", "list-groups-for-user", "--user-name", BOT_USER)["Groups"]:
        aws("iam", "remove-user-from-group", "--user-name", BOT_USER,
            "--group-name", group["GroupName"])
        say(f"left group {group['GroupName']}")
    maybe("iam", "delete-login-profile", "--user-name", BOT_USER)
    aws("iam", "delete-user", "--user-name", BOT_USER)
    say(f"user {BOT_USER} gone")

# ALL

def clean_all():
    distributions()
    clean_records()
    buckets()
    clean_lambda()
    user()

# MAIN

VERBS = {
    "distributions": distributions,
    "records": clean_records,
    "buckets": buckets,
    "lambda": clean_lambda,
    "user": user,
    "all": clean_all,
}

if __name__ == "__main__":
    VERBS[verb(list(VERBS))]()
