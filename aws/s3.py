import os
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

# AUDIT

BLOCK_FLAGS = ("BlockPublicAcls", "IgnorePublicAcls", "BlockPublicPolicy", "RestrictPublicBuckets")

MISSING_BLOCK = "NoSuchPublicAccessBlockConfiguration"
MISSING_ENCRYPTION = "ServerSideEncryptionConfigurationNotFoundError"

def error_code(error):
    return error.response.get("Error", {}).get("Code", "unknown")

def block_state(bucket):
    try:
        config = s3_client.get_public_access_block(Bucket=bucket)["PublicAccessBlockConfiguration"]
    except ClientError as error:
        code = error_code(error)
        return "no config" if code == MISSING_BLOCK else f"unreadable ({code})"
    off = [flag for flag in BLOCK_FLAGS if not config.get(flag)]
    return "all blocked" if not off else "off: " + ", ".join(off)

def versioning(bucket):
    return s3_client.get_bucket_versioning(Bucket=bucket).get("Status", "Disabled").lower()

def encryption(bucket):
    try:
        rules = s3_client.get_bucket_encryption(Bucket=bucket)["ServerSideEncryptionConfiguration"]["Rules"]
        return rules[0]["ApplyServerSideEncryptionByDefault"]["SSEAlgorithm"]
    except ClientError as error:
        code = error_code(error)
        return "none" if code == MISSING_ENCRYPTION else f"unreadable ({code})"
    except (KeyError, IndexError):
        return "none"

def audit():
    shapes = {}
    for item in s3_client.list_buckets()["Buckets"]:
        bucket = item["Name"]
        shapes[bucket] = (block_state(bucket), versioning(bucket), encryption(bucket))
    for bucket, (block, version, cipher) in shapes.items():
        state = "private" if block == "all blocked" else "PUBLIC"
        print(f"{state:<8} {bucket:<20} block {block}, versioning {version}, encryption {cipher}")
    print()
    if not shapes:
        print("no buckets")
        return
    counts = {}
    for shape in shapes.values():
        counts[shape] = counts.get(shape, 0) + 1
    if len(counts) == 1:
        print(f"{len(shapes)} buckets, settings identical")
        return
    common_shape = max(counts, key=counts.get)
    print(f"{len(shapes)} buckets drift from the common shape ({' | '.join(common_shape)}):")
    for bucket, shape in shapes.items():
        if shape != common_shape:
            print(f"  {bucket}: {' | '.join(shape)}")

# DELETE

BATCH = 1000

def delete_batches(bucket, keys):
    deleted = 0
    errors = []
    for start in range(0, len(keys), BATCH):
        batch = keys[start:start + BATCH]
        response = s3_client.delete_objects(Bucket=bucket, Delete={"Objects": [{"Key": key} for key in batch]})
        batch_errors = response.get("Errors", [])
        errors.extend(batch_errors)
        deleted += len(batch) - len(batch_errors)
    for error in errors:
        print(f"failed: {error['Key']} ({error.get('Code')})")
    if errors:
        print(f"delete failed for {len(errors)} of {len(keys)} objects")
        sys.exit(1)
    return deleted

def drop(bucket, prefix):
    if not prefix:
        print("drop needs a prefix; use wipe to empty a bucket")
        return
    matched = []
    paginator = s3_client.get_paginator("list_objects_v2")
    for page in paginator.paginate(Bucket=bucket, Prefix=prefix):
        matched.extend(obj["Key"] for obj in page.get("Contents", []))
    if not matched:
        print(f"nothing under s3://{bucket}/{prefix}")
        return
    for key in matched[:10]:
        print(f"  {key}")
    if len(matched) > 10:
        print(f"  ... {len(matched) - 10} more")
    sure = input(f"delete {len(matched)} objects matching s3://{bucket}/{prefix}? type the prefix to confirm: ")
    if sure != prefix:
        print("aborted")
        return
    deleted = delete_batches(bucket, matched)
    print(f"dropped {deleted} objects from s3://{bucket}/{prefix}")

def wipe(bucket):
    drop_all = input(f"empty s3://{bucket} entirely? type the bucket name to confirm: ")
    if drop_all != bucket:
        print("aborted")
        return
    matched = []
    paginator = s3_client.get_paginator("list_objects_v2")
    for page in paginator.paginate(Bucket=bucket):
        matched.extend(obj["Key"] for obj in page.get("Contents", []))
    deleted = delete_batches(bucket, matched)
    print(f"wiped {deleted} objects from s3://{bucket}")

def mkbucket(bucket):
    region = os.environ.get("AWS_DEFAULT_REGION", "us-east-1")
    kwargs = {"Bucket": bucket}
    if region != "us-east-1":
        kwargs["CreateBucketConfiguration"] = {"LocationConstraint": region}
    s3_client.create_bucket(**kwargs)
    s3_client.put_public_access_block(Bucket=bucket, PublicAccessBlockConfiguration={
        "BlockPublicAcls": True,
        "IgnorePublicAcls": True,
        "BlockPublicPolicy": True,
        "RestrictPublicBuckets": True,
    })
    print(f"created s3://{bucket} in {region}")

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
        ("audit", "public access block, versioning, encryption, and drift between buckets"),
        ("drop <bucket> <prefix>", "delete every object under a prefix (confirmed; '/' anchored)"),
        ("mkbucket <bucket>", "create a private bucket in the default region"),
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
        case ["audit"]: audit()
        case ["drop", bucket, prefix]: drop(bucket, prefix)
        case ["mkbucket", bucket]: mkbucket(bucket)
        case ["wipe", bucket]: wipe(bucket)
        case ["rmbucket", bucket]: rmbucket(bucket)
        case _: help()

# AWS

import common
from botocore.exceptions import ClientError

s3_client = common.s3_client()

if __name__ == "__main__":
    terminal()
