import os
import sys

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

# RECONCILE

def signature(notification):
    return (notification["NotificationType"], notification["ComparisonOperator"], float(notification["Threshold"]))

def sync_notifications(account, email):
    desired = {signature(item["Notification"]): item for item in notifications(email)}
    current = budgets_client.describe_notifications_for_budget(AccountId=account, BudgetName=NAME).get("Notifications", [])
    have = {}
    for notification in current:
        key = signature(notification)
        if key in desired:
            have[key] = notification
            continue
        budgets_client.delete_notification(AccountId=account, BudgetName=NAME, Notification=notification)
        print(f"  notification {notification['NotificationType'].lower()} {notification['Threshold']:.0f}%: dropped")
    for key, item in desired.items():
        label = f"{item['Notification']['NotificationType'].lower()} {item['Notification']['Threshold']}%"
        if key not in have:
            budgets_client.create_notification(AccountId=account, BudgetName=NAME, Notification=item["Notification"], Subscribers=item["Subscribers"])
            print(f"  notification {label}: created, emails {email}")
            continue
        got = budgets_client.describe_subscribers_for_notification(AccountId=account, BudgetName=NAME, Notification=have[key]).get("Subscribers", [])
        for sub in got:
            if sub not in item["Subscribers"]:
                budgets_client.delete_subscriber(AccountId=account, BudgetName=NAME, Notification=have[key], Subscriber=sub)
                print(f"  subscriber {sub['Address']}: dropped from {label}")
        for sub in item["Subscribers"]:
            if sub not in got:
                budgets_client.create_subscriber(AccountId=account, BudgetName=NAME, Notification=have[key], Subscriber=sub)
                print(f"  subscriber {sub['Address']}: added to {label}")

# COMMANDS

def set_budget(limit):
    email = os.environ.get("MRLY_EMAIL")
    if not email:
        print("budget (add to .env: MRLY_EMAIL)")
        return
    account = sts_client.get_caller_identity()["Account"]
    try:
        budgets_client.describe_budget(AccountId=account, BudgetName=NAME)
    except budgets_client.exceptions.NotFoundException:
        budgets_client.create_budget(
            AccountId=account,
            Budget=desired_budget(limit),
            NotificationsWithSubscribers=notifications(email),
        )
        print(f"budget {NAME}: created at ${limit}/month")
        for kind, threshold in THRESHOLDS:
            print(f"  email {email} when {kind.lower()} spend passes {threshold}%")
        return
    budgets_client.update_budget(AccountId=account, NewBudget=desired_budget(limit))
    sync_notifications(account, email)
    print(f"budget {NAME}: updated to ${limit}/month")

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

import common

budgets_client = common.budgets_client()
sts_client = common.sts_client()

if __name__ == "__main__":
    terminal()
