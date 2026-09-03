# COMMANDS

- Run from the `mrlyprod/` root: `uv run python aws/<script>.py <verb>`.
- Ids come from a local `.env`; the live values are the built-in defaults.
- Credentials come from the shell, never from `.env`.
- Read-only verbs run bare; a mutating verb prints its plan and needs `--yes`.

## SITE

- `site.py check` - cert, ns, bucket, distribution, wiring, dns: one GO or HOLD each.
- `site.py bucket` - block public access, drop website config, seal the bucket policy.
- `site.py function` - publish the `mrlynet-router` viewer-request function.
- `site.py headers` - publish the `mrlynet-security` response headers policy.
- `site.py distribution` - update the mrly.net distribution to the desired config.
- DNS is never written: mrly.net already points at the distribution and carries the mail records.

## DEPLOY

- `deploy.py build` - `bun run build` inside `sites/net`.
- `deploy.py plan` - dry run of both sync passes.
- `deploy.py push` - build, sync in two cache passes, fix types, invalidate `/*`.
- `deploy.py status` - the last invalidation and its state.

## CLEAN

- `clean.py distributions` - drop the four dead distributions and six dead OACs.
- `clean.py records` - drop only the web, cdn, git and bot aliases in the mrly.net zone.
- `clean.py buckets` - empty all 13 buckets; every bucket itself stays.
- `clean.py lambda` - schedules, function, layers, stack and role.
- `clean.py user` - strip and drop the `mrlybot` IAM user; `carlo` is never touched.
- `clean.py all` - the five above, in that order.

## ORDER

- `site.py check`
- `site.py bucket --yes`
- `site.py function --yes`
- `site.py headers --yes`
- `site.py distribution --yes`
- `site.py bucket --yes` again, so the policy names the distribution
- `clean.py all --yes`
- `deploy.py push --yes`

## NOTES

- The bucket policy names the distribution ARN.
- So rerun `site.py bucket --yes` once `distribution` has run.
- Nothing here writes to the mrly.net zone; `clean.py records` deletes four dead aliases in it and nothing else.
- `site.py distribution` refuses until the function and headers exist.
- A fresh distribution takes minutes to reach Deployed; `check` reports it.
