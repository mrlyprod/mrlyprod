# AWS

- The console for mrlyprod.org: one bucket, one distribution, two zones.
- Python 3.13 and the `aws` CLI; `common.py` holds every id, `COMMANDS.md` every verb.
- Read-only verbs run bare; anything that mutates prints its plan and waits for `--yes`.
