import json
import os
import sys
import types
import unittest
from datetime import timedelta
from unittest import mock

os.environ.setdefault("STATS_BUCKET", "test")
os.environ.setdefault("STATS_FUNCTIONS", "one,two")
os.environ.setdefault("STATS_KEY", "stats.json")
HERE = os.path.dirname(os.path.abspath(__file__))
FIXTURE = os.path.join(HERE, "..", "src", "pages", "stats", "fixture.json")
sys.path.insert(0, HERE)

# STUBS

class Cloudwatch:
    def get_metric_data(self, **query):
        start = query["StartTime"]
        return {"MetricDataResults": [{"Timestamps": [start, start + timedelta(hours=1)], "Values": [3.0, 4.0]}]}

class Logs:
    def filter_log_events(self, **query):
        line = {"timestamp": "2026-10-19T08:37:47.067Z", "level": "ERROR", "message": "fetch failed"}
        return {"events": [{"message": json.dumps(line)}] if query["logGroupName"].endswith("-site") else []}

class Pages:
    def paginate(self, **query):
        return [{"Contents": [{"Size": 5}], "CommonPrefixes": [{"Prefix": query["Prefix"] + "a/"}]}]

class S3:
    def get_paginator(self, name):
        return Pages()

CLIENTS = {"cloudwatch": Cloudwatch, "logs": Logs, "s3": S3}
sys.modules["boto3"] = types.SimpleNamespace(client=lambda name, **options: CLIENTS[name]())

import stats

CONFIG = {
    "DISTRIBUTION": "dist",
    "FUNCTIONS": ["mrlynet-site", "mrlynet-stats"],
    "SIZES": [("site", "", "")],
    "COUNTS": [(label, f"{label}/", "") for label in ("figures", "code", "raw", "demos")],
    "FOLDERS": [(label, f"{label}/", "") for label in ("wiki", "notes", "claims", "papers", "blog")],
}

def keys(value):
    if isinstance(value, dict):
        return {key: keys(one) for key, one in value.items()}
    if isinstance(value, list):
        return [keys(value[0])] if value else []
    return None

class Fixture(unittest.TestCase):

    def test_fixture_has_the_key_tree_build_returns(self):
        with open(FIXTURE) as handle:
            fixture = json.load(handle)
        with mock.patch.multiple(stats, **CONFIG):
            built = stats.build()
        self.assertEqual(keys(built), keys(fixture))

if __name__ == "__main__":
    unittest.main()
