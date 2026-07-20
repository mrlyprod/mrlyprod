import sys

# BUCKETS

def buckets():
    for name in s3_client.list_buckets()["Buckets"]:
        bucket = name["Name"]
        count = 0
        size = 0
        paginator = s3_client.get_paginator("list_objects_v2")
        for page in paginator.paginate(Bucket=bucket):
            contents = page.get("Contents", [])
            count += len(contents)
            size += sum(obj["Size"] for obj in contents)
        print(f"{bucket:<20} {count:>6} objects  {size / 1_000_000:>10.1f} MB")

def keys(bucket, prefix):
    paginator = s3_client.get_paginator("list_objects_v2")
    count = 0
    for page in paginator.paginate(Bucket=bucket, Prefix=prefix):
        for obj in page.get("Contents", []):
            print(f"{obj['Size']:>12}  {obj['Key']}")
            count += 1
    print(f"{count} objects")

# DELETE

def drop(bucket, prefix):
    if not prefix:
        print("drop needs a prefix; use wipe to empty a bucket")
        return
    deleted = 0
    paginator = s3_client.get_paginator("list_objects_v2")
    for page in paginator.paginate(Bucket=bucket, Prefix=prefix):
        contents = page.get("Contents", [])
        if not contents: continue
        s3_client.delete_objects(Bucket=bucket, Delete={"Objects": [{"Key": obj["Key"]} for obj in contents]})
        deleted += len(contents)
    print(f"dropped {deleted} objects from s3://{bucket}/{prefix}")

def wipe(bucket):
    drop_all = input(f"empty s3://{bucket} entirely? type the bucket name to confirm: ")
    if drop_all != bucket:
        print("aborted")
        return
    deleted = 0
    paginator = s3_client.get_paginator("list_objects_v2")
    for page in paginator.paginate(Bucket=bucket):
        contents = page.get("Contents", [])
        if not contents: continue
        s3_client.delete_objects(Bucket=bucket, Delete={"Objects": [{"Key": obj["Key"]} for obj in contents]})
        deleted += len(contents)
    print(f"wiped {deleted} objects from s3://{bucket}")

def rmbucket(bucket):
    page = s3_client.list_objects_v2(Bucket=bucket, MaxKeys=1)
    if page.get("Contents"):
        print(f"s3://{bucket} is not empty; wipe it first")
        return
    print(f"deleting s3://{bucket} releases the name for anyone to claim")
    sure = input("type the bucket name to confirm: ")
    if sure != bucket:
        print("aborted")
        return
    s3_client.delete_bucket(Bucket=bucket)
    print(f"deleted s3://{bucket}")

# TERMINAL

def help():
    commands = [
        ("buckets", "list every bucket with object count and size"),
        ("keys <bucket> [prefix]", "list keys, optionally under a prefix"),
        ("drop <bucket> <prefix>", "delete every object under a prefix"),
        ("wipe <bucket>", "empty a bucket (confirmed)"),
        ("rmbucket <bucket>", "delete an empty bucket (confirmed; releases the name)"),
    ]
    width = max(len(name) for name, _ in commands)
    print("s3")
    print()
    for name, desc in commands:
        print(f"  {name:<{width}}  {desc}")
    print()

def terminal():
    match sys.argv[1:]:
        case ["buckets"]: buckets()
        case ["keys", bucket]: keys(bucket, "")
        case ["keys", bucket, prefix]: keys(bucket, prefix)
        case ["drop", bucket, prefix]: drop(bucket, prefix)
        case ["wipe", bucket]: wipe(bucket)
        case ["rmbucket", bucket]: rmbucket(bucket)
        case _: help()

# AWS

import boto3

s3_client = boto3.client("s3")

if __name__ == "__main__":
    terminal()
