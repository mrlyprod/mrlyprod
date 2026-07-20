import sys

APEX = "mrly.net"
PROTECTED = ("NS", "SOA")

# ZONES

def zone_id():
    zones = r53_client.list_hosted_zones()["HostedZones"]
    for zone in zones:
        if zone["Name"] == f"{APEX}." and not zone["Config"]["PrivateZone"]:
            return zone["Id"]
    print(f"no hosted zone for {APEX}")
    return None

def zones():
    for zone in r53_client.list_hosted_zones()["HostedZones"]:
        kind = "private" if zone["Config"]["PrivateZone"] else "public"
        print(f"{zone['Name']:<30} {kind}  {zone['Id']}  {zone['ResourceRecordSetCount']} records")

# RECORDS

def full(name):
    name = name.rstrip(".")
    if name in ("", "@"): return f"{APEX}."
    if not name.endswith(APEX): return f"{name}.{APEX}."
    return f"{name}."

def all_records():
    zid = zone_id()
    if not zid: return None
    records = []
    kwargs = {"HostedZoneId": zid}
    while True:
        page = r53_client.list_resource_record_sets(**kwargs)
        records.extend(page["ResourceRecordSets"])
        if not page["IsTruncated"]: return zid, records
        kwargs["StartRecordName"] = page["NextRecordName"]
        kwargs["StartRecordType"] = page["NextRecordType"]

def records():
    got = all_records()
    if not got: return
    _, items = got
    for record in items:
        if "AliasTarget" in record:
            print(f"{record['Name']:<30} {record['Type']:<6} alias -> {record['AliasTarget']['DNSName']}")
        else:
            values = " ".join(v["Value"] for v in record["ResourceRecords"])
            print(f"{record['Name']:<30} {record['Type']:<6} ttl {record.get('TTL', '-'):<6} {values}")

def set_record(name, rtype, values, ttl):
    zid = zone_id()
    if not zid: return
    record = {
        "Name": full(name),
        "Type": rtype.upper(),
        "TTL": int(ttl),
        "ResourceRecords": [{"Value": v} for v in values],
    }
    r53_client.change_resource_record_sets(HostedZoneId=zid, ChangeBatch={
        "Changes": [{"Action": "UPSERT", "ResourceRecordSet": record}],
    })
    print(f"set {record['Name']} {record['Type']} -> {' '.join(values)}")

def drop(name, rtype):
    got = all_records()
    if not got: return
    zid, items = got
    rtype = rtype.upper()
    if rtype in PROTECTED:
        print(f"{rtype} records are protected")
        return
    matches = [r for r in items if r["Name"] == full(name) and r["Type"] == rtype]
    if not matches:
        print(f"no {rtype} record for {full(name)}")
        return
    r53_client.change_resource_record_sets(HostedZoneId=zid, ChangeBatch={
        "Changes": [{"Action": "DELETE", "ResourceRecordSet": r} for r in matches],
    })
    print(f"dropped {full(name)} {rtype}")

# TERMINAL

def help():
    commands = [
        ("zones", "list hosted zones"),
        ("records", f"list every record in the {APEX} zone"),
        ("set <name> <type> <value...> [--ttl 300]", "upsert a record (name relative to the apex, @ for the apex)"),
        ("drop <name> <type>", "delete a record (NS and SOA protected)"),
    ]
    width = max(len(name) for name, _ in commands)
    print("dns")
    print()
    for name, desc in commands:
        print(f"  {name:<{width}}  {desc}")
    print()

def terminal():
    args = sys.argv[1:]
    ttl = 300
    if "--ttl" in args:
        i = args.index("--ttl")
        ttl = args[i + 1]
        args = args[:i] + args[i + 2:]
    match args:
        case ["zones"]: zones()
        case ["records"]: records()
        case ["set", name, rtype, *values] if values: set_record(name, rtype, values, ttl)
        case ["drop", name, rtype]: drop(name, rtype)
        case _: help()

# AWS

import boto3

r53_client = boto3.client("route53")

if __name__ == "__main__":
    terminal()
