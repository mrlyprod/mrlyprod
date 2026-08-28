import os
import sys

from fn import FUNCTIONS, deployed_name, function_arn

# DESIRED

TIMEZONE = "UTC"
RETRIES = 0
MAX_AGE = 60

RATES = {
    "game": "rate(1 day)",
}

def desired_schedule(key, rate, role, state):
    return {
        "Name": deployed_name(key),
        "ScheduleExpression": rate,
        "ScheduleExpressionTimezone": TIMEZONE,
        "State": state,
        "FlexibleTimeWindow": {"Mode": "OFF"},
        "Target": {
            "Arn": function_arn(key),
            "RoleArn": role,
            "RetryPolicy": {"MaximumRetryAttempts": RETRIES, "MaximumEventAgeInSeconds": MAX_AGE},
        },
    }

# REGISTRY

def desired_rates():
    return {deployed_name(key): rate for key, rate in RATES.items() if key in FUNCTIONS}

def schedule_name(key):
    return deployed_name(key) if key in FUNCTIONS else key

# RECONCILE

def current(name):
    try:
        return scheduler_client.get_schedule(Name=name)
    except scheduler_client.exceptions.ResourceNotFoundException:
        return None

def matches(desired, got):
    if not got: return False
    target = got.get("Target", {})
    return (
        got.get("ScheduleExpression") == desired["ScheduleExpression"]
        and got.get("ScheduleExpressionTimezone") == desired["ScheduleExpressionTimezone"]
        and got.get("FlexibleTimeWindow") == desired["FlexibleTimeWindow"]
        and target.get("Arn") == desired["Target"]["Arn"]
        and target.get("RoleArn") == desired["Target"]["RoleArn"]
        and target.get("RetryPolicy") == desired["Target"]["RetryPolicy"]
    )

def sync(key, rate, role):
    name = deployed_name(key)
    got = current(name)
    state = got["State"] if got else "DISABLED"
    desired = desired_schedule(key, rate, role, state)
    if matches(desired, got):
        print(f"  {name}: current, {rate}, {state.lower()}")
        return
    if got:
        scheduler_client.update_schedule(**desired)
        print(f"  {name}: updated, {rate}, {state.lower()}")
        return
    scheduler_client.create_schedule(**desired)
    print(f"  {name}: created, {rate}, disabled")

def restate(name, state):
    got = current(name)
    if not got:
        print(f"{name}: no schedule")
        return
    if got["State"] == state:
        print(f"{name}: already {state.lower()}")
        return
    scheduler_client.update_schedule(
        Name=name,
        ScheduleExpression=got["ScheduleExpression"],
        ScheduleExpressionTimezone=got.get("ScheduleExpressionTimezone", TIMEZONE),
        State=state,
        FlexibleTimeWindow=got["FlexibleTimeWindow"],
        Target=got["Target"],
    )
    print(f"{name}: {state.lower()}")

# COMMANDS

def role_arn():
    role = os.environ.get("ROLE_ARN")
    if not role:
        print("schedules (add to .env: ROLE_ARN)")
    return role

def check():
    live = []
    paginator = scheduler_client.get_paginator("list_schedules")
    for page in paginator.paginate():
        live += [item["Name"] for item in page["Schedules"]]
    desired = desired_rates()
    for name in live:
        got = current(name)
        if not got: continue
        target = got["Target"]["Arn"].rsplit(":", 1)[-1]
        wanted = desired.get(name)
        drift = "" if wanted in (None, got["ScheduleExpression"]) else f"  wants {wanted}"
        print(f"{name:<20} {got['State'].lower():<9} {got['ScheduleExpression']:<18} -> {target}{drift}")
    for name, rate in desired.items():
        if name not in live:
            print(f"{name:<20} {'missing':<9} {rate}")
    for key in RATES:
        if key not in FUNCTIONS:
            print(f"{key:<20} {'unknown':<9} no function in fn")
    if not live and not RATES:
        print("no schedules")

def set_schedules(key=None):
    role = role_arn()
    if not role: return
    keys = [key] if key else list(RATES)
    for target in keys:
        if target not in RATES or target not in FUNCTIONS:
            print(f"schedules (no rate for {target})")
            continue
        sync(target, RATES[target], role)

def on(key):
    restate(schedule_name(key), "ENABLED")

def off(key):
    restate(schedule_name(key), "DISABLED")

def drop(key):
    name = schedule_name(key)
    if not current(name):
        print(f"{name}: no schedule")
        return
    scheduler_client.delete_schedule(Name=name)
    print(f"dropped {name}")

# TERMINAL

def help():
    commands = [
        ("check", "every schedule with state, rate, target, and drift from the desired rate"),
        ("set [target]", "create or update the desired schedules, new ones land disabled"),
        ("on <target>", "enable a schedule, the rest of it untouched"),
        ("off <target>", "disable a schedule, the rest of it untouched"),
        ("drop <target>", "delete a schedule"),
    ]
    width = max(len(name) for name, _ in commands)
    print(f"schedules <command> <{'|'.join(RATES) or 'none'}> (or a live schedule name)")
    print()
    for name, desc in commands:
        print(f"  {name:<{width}}  {desc}")
    print()

def terminal():
    match sys.argv[1:]:
        case ["check"]: check()
        case ["set"]: set_schedules()
        case ["set", key]: set_schedules(key)
        case ["on", key]: on(key)
        case ["off", key]: off(key)
        case ["drop", key]: drop(key)
        case _: help()

# AWS

import common

scheduler_client = common.scheduler_client()

if __name__ == "__main__":
    terminal()
