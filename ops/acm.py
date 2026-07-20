import sys

# CERTS

def list_certs():
    paginator = acm_client.get_paginator("list_certificates")
    for page in paginator.paginate():
        for summary in page["CertificateSummaryList"]:
            cert = acm_client.describe_certificate(CertificateArn=summary["CertificateArn"])["Certificate"]
            names = " ".join(cert.get("SubjectAlternativeNames", [cert["DomainName"]]))
            expiry = cert.get("NotAfter")
            expiry = expiry.strftime("%Y-%m-%d") if expiry else "-"
            used = "in use" if cert.get("InUseBy") else "unused"
            print(f"{cert['Status']:<20} expires {expiry}  {used:<7} {names}")
            print(f"  {cert['CertificateArn']}")

def request(domain, sans):
    kwargs = {"DomainName": domain, "ValidationMethod": "DNS"}
    if sans: kwargs["SubjectAlternativeNames"] = sans
    arn = acm_client.request_certificate(**kwargs)["CertificateArn"]
    print(f"requested {arn}")
    print("run list until the validation records appear, then add them with ops/dns.py set")

def validation(arn):
    cert = acm_client.describe_certificate(CertificateArn=arn)["Certificate"]
    print(f"{cert['Status']} {cert['DomainName']}")
    for option in cert.get("DomainValidationOptions", []):
        record = option.get("ResourceRecord")
        if record:
            print(f"  {record['Name']} {record['Type']} -> {record['Value']}")

# TERMINAL

def help():
    commands = [
        ("list", "list certificates in us-east-1 (the CloudFront region)"),
        ("request <domain> [san...]", "request a DNS-validated certificate"),
        ("validation <arn>", "show the DNS records a pending certificate needs"),
    ]
    width = max(len(name) for name, _ in commands)
    print("acm")
    print()
    for name, desc in commands:
        print(f"  {name:<{width}}  {desc}")
    print()

def terminal():
    match sys.argv[1:]:
        case ["list"]: list_certs()
        case ["request", domain, *sans]: request(domain, sans)
        case ["validation", arn]: validation(arn)
        case _: help()

# AWS

import boto3

acm_client = boto3.client("acm", region_name="us-east-1")

if __name__ == "__main__":
    terminal()
