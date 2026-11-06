import json
import os

from common import (
    FUNCTIONS,
    SCHEDULES,
    gate,
    given,
    lambda_arn,
    pick,
    role_arn,
    say,
    verb,
)
from lib import aws, maybe, tmpfile

TIMEZONE = "UTC"
RETRIES = 0
MAX_AGE = 60


def live(name):
    return maybe("scheduler", "get-schedule", "--name", name)


def target(spec):
    function = FUNCTIONS[spec["function"]]
    return {
        "Arn": lambda_arn(function["name"]),
        "RoleArn": role_arn(function["role"]),
        "Input": json.dumps(spec["input"]),
        "RetryPolicy": {
            "MaximumRetryAttempts": RETRIES,
            "MaximumEventAgeInSeconds": MAX_AGE,
        },
    }


def write(spec, state, exists):
    path = tmpfile(json.dumps(target(spec)), ".json")
    try:
        aws(
            "scheduler",
            "update-schedule" if exists else "create-schedule",
            "--name",
            spec["name"],
            "--schedule-expression",
            spec["rate"],
            "--schedule-expression-timezone",
            TIMEZONE,
            "--state",
            state,
            "--flexible-time-window",
            "Mode=OFF",
            "--target",
            f"file://{path}",
        )
    finally:
        os.remove(path)


def show():
    for key, spec in SCHEDULES.items():
        found = live(spec["name"])
        if not found:
            say(f"{key:<20} {spec['name']:<28} none yet")
            continue
        say(
            f"{key:<20} {spec['name']:<28} {found['State']} {found['ScheduleExpression']}"
        )


def sync():
    key = pick(SCHEDULES, "schedule")
    spec = SCHEDULES[key]
    function = FUNCTIONS[spec["function"]]
    found = live(spec["name"])
    state = found["State"] if found else "DISABLED"
    if not gate(
        f"schedules sync {key}",
        [
            f"{'update' if found else 'create'} schedule {spec['name']}",
            f"{spec['rate']} {TIMEZONE}, flexible window OFF",
            f"target {function['name']} as {function['role']}, input {json.dumps(spec['input'])}, retries {RETRIES}, age {MAX_AGE} s",
            f"state {state}",
        ],
    ):
        return
    write(spec, state, found is not None)
    say(f"synced {spec['name']} at {state}")


def switch(state):
    key = pick(SCHEDULES, "schedule")
    spec = SCHEDULES[key]
    found = live(spec["name"])
    if not found:
        raise SystemExit(f"refuse: {spec['name']} does not exist, sync it first")
    if not gate(
        f"schedules {state.lower()} {key}",
        [f"{spec['name']} {found['State']} -> {state}"],
    ):
        return
    write(spec, state, True)
    say(f"{spec['name']} is {state}")


def delete():
    name = given("schedule name")
    if any(spec["name"] == name for spec in SCHEDULES.values()):
        raise SystemExit(
            f"refuse: {name} is a registry row, drop it from common.py first"
        )
    found = live(name)
    if not found:
        raise SystemExit(f"refuse: {name} does not exist")
    if not gate(
        f"schedules delete {name}",
        [f"delete schedule {name}, {found['ScheduleExpression']} {found['State']}"],
    ):
        return
    aws("scheduler", "delete-schedule", "--name", name)
    say(f"deleted {name}")


def enable():
    switch("ENABLED")


def disable():
    switch("DISABLED")


VERBS = {
    "list": (show, "", "each schedule with its state and rate"),
    "sync": (
        sync,
        "<schedule>",
        "create or update; a new schedule is born DISABLED, an existing one keeps its state",
    ),
    "enable": (enable, "<schedule>", "flip the state to ENABLED, touch nothing else"),
    "disable": (
        disable,
        "<schedule>",
        "flip the state to DISABLED, touch nothing else",
    ),
    "delete": (
        delete,
        "<name>",
        "drop a schedule by literal name; refuses a name still in the registry",
    ),
}

ABOUT = "EventBridge schedules from SCHEDULES in common.py. A mutating verb prints a PLAN; --yes applies it."

if __name__ == "__main__":
    verb(VERBS, ABOUT)()
