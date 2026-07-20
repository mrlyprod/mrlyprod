import os
import sys

ROOT_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

# ENV

def load_env():
    path = os.path.join(ROOT_DIR, ".env")
    if not os.path.exists(path): return
    with open(path) as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("#"): continue
            key, _, value = line.partition("=")
            os.environ.setdefault(key.strip(), value.strip())

load_env()

# DESIRED

NAME = "mrly"
THRESHOLDS = [
    ("ACTUAL", 25),
    ("ACTUAL", 100),
    ("FORECASTED", 100),
]

def desired_budget(limit):
    return {
        "BudgetName": NAME,
        "BudgetType": "COST",
        "TimeUnit": "MONTHLY",
        "BudgetLimit": {"Amount": str(limit), "Unit": "USD"},
    }

def notifications(email):
    subscribers = [{"SubscriptionType": "EMAIL", "Address": email}]
    return [
        {
            "Notification": {
                "NotificationType": kind,
                "ComparisonOperator": "GREATER_THAN",
                "Threshold": threshold,
                "ThresholdType": "PERCENTAGE",
            },
            "Subscribers": subscribers,
        }
        for kind, threshold in THRESHOLDS
    ]

# COMMANDS

def set_budget(limit):
    email = os.environ.get("MRLY_EMAIL")
    if not email:
        print("budget (add to .env: MRLY_EMAIL)")
        return
    account = sts_client.get_caller_identity()["Account"]
    try:
        budgets_client.describe_budget(AccountId=account, BudgetName=NAME)
        budgets_client.update_budget(AccountId=account, NewBudget=desired_budget(limit))
        print(f"budget {NAME}: updated to ${limit}/month")
    except budgets_client.exceptions.NotFoundException:
        budgets_client.create_budget(
            AccountId=account,
            Budget=desired_budget(limit),
            NotificationsWithSubscribers=notifications(email),
        )
        print(f"budget {NAME}: created at ${limit}/month")
        for kind, threshold in THRESHOLDS:
            print(f"  email {email} when {kind.lower()} spend passes {threshold}%")

def check():
    account = sts_client.get_caller_identity()["Account"]
    budgets = budgets_client.describe_budgets(AccountId=account).get("Budgets", [])
    if not budgets:
        print("no budgets")
        return
    for budget in budgets:
        limit = budget["BudgetLimit"]
        spend = budget.get("CalculatedSpend", {})
        actual = spend.get("ActualSpend", {}).get("Amount", "0")
        forecast = spend.get("ForecastedSpend", {}).get("Amount", "-")
        print(f"{budget['BudgetName']}: ${float(actual):.2f} spent of ${float(limit['Amount']):.0f} {budget['TimeUnit'].lower()}, forecast ${forecast if forecast == '-' else f'{float(forecast):.2f}'}")

# TERMINAL

def help():
    commands = [
        ("set [limit]", "create or update the monthly cost budget (default $20)"),
        ("check", "show budgets with actual and forecast spend"),
    ]
    width = max(len(name) for name, _ in commands)
    print("budget")
    print()
    for name, desc in commands:
        print(f"  {name:<{width}}  {desc}")
    print()

def terminal():
    match sys.argv[1:]:
        case ["set"]: set_budget(20)
        case ["set", limit]: set_budget(int(limit))
        case ["check"]: check()
        case _: help()

# AWS

import boto3

budgets_client = boto3.client("budgets", region_name="us-east-1")
sts_client = boto3.client("sts")

if __name__ == "__main__":
    terminal()
