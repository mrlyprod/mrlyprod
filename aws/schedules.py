import json

from common import ACCOUNT, NET_FUNCTION, REGION, ROLE_NAME, aws, gate, maybe, say, verb

# DESIRED

NAME = NET_FUNCTION
RATE = "rate(10 minutes)"
TIMEZONE = "UTC"
RETRIES = 0
MAX_AGE = 60
WINDOW = {"Mode": "OFF"}

def function_arn():
    return f"arn:aws:lambda:{REGION}:{ACCOUNT}:function:{NET_FUNCTION}"

def role_arn():
    return f"arn:aws:iam::{ACCOUNT}:role/{ROLE_NAME}"

def target():
    return {
        "Arn": function_arn(),
        "RoleArn": role_arn(),
        "Input": json.dumps({"source": "schedule"}),
        "RetryPolicy": {"MaximumRetryAttempts": RETRIES, "MaximumEventAgeInSeconds": MAX_AGE},
    }

def current():
    return maybe("scheduler", "get-schedule", "--name", NAME, region=REGION)

def put(state):
    action = "update-schedule" if current() else "create-schedule"
    aws("scheduler", action, "--name", NAME,
        "--schedule-expression", RATE,
        "--schedule-expression-timezone", TIMEZONE,
        "--state", state,
        "--flexible-time-window", json.dumps(WINDOW),
        "--target", json.dumps(target()), region=REGION)
    say(f"{NAME} {RATE} {state.lower()}")

# VERBS

def create():
    found = current()
    if not gate("schedule", [
        f"{'update' if found else 'create'} schedule {NAME} in {REGION}",
        f"{RATE} in {TIMEZONE}, flexible window off",
        f"target {function_arn()}",
        f"as {ROLE_NAME}, retries {RETRIES}, max age {MAX_AGE}s",
        "it lands enabled",
    ]): return
    put("ENABLED")

def disable():
    if not current():
        say(f"{NAME}: no schedule")
        return
    if not gate("disable", [f"set schedule {NAME} to DISABLED, nothing else changes"]): return
    put("DISABLED")

def enable():
    if not current():
        say(f"{NAME}: no schedule")
        return
    if not gate("enable", [f"set schedule {NAME} to ENABLED, {RATE}"]): return
    put("ENABLED")

def status():
    found = current()
    if not found:
        say(f"{NAME}: no schedule")
        return
    spot = found["Target"]
    say(f"{NAME} {found['State'].lower()} {found['ScheduleExpression']} {found.get('ScheduleExpressionTimezone', TIMEZONE)}")
    say(f"  target {spot['Arn']}")
    say(f"  role   {spot['RoleArn']}")
    say(f"  retry  {spot.get('RetryPolicy', {})}")

# MAIN

VERBS = {"create": create, "disable": disable, "enable": enable, "status": status}

if __name__ == "__main__":
    VERBS[verb(list(VERBS))]()
