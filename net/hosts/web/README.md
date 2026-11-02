# WEB

- The web host: it builds a site from its config folder into static files and ships them to AWS.
- `scripts/`: the `bun run` verbs and the build; `scripts/README.md` has them.
- `infra/`: the code that runs in AWS or talks to it: edge, push, S3, builder, stats; `infra/README.md` has it.
- `test/`: the suites that read a built site: `content.test.ts` (the content gate `bun run check` runs: prose links, research links, the house rules, built links and redirects), `routes.test.ts` (every folder of a root is a route, a root word with a title and a line of at most 90 characters), and `dist.ts`, which builds the site once into a temp dir for every test that reads a `dist/` and removes it on exit.
- `scripts/` and `infra/` import each other; outside them only tests import them (the shell, apps, glass and fonts tests).
- `bun test net/hosts/web` from the repo root runs every test here.
