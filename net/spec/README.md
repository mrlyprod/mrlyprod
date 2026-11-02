# SPEC

- The contract every kernel, door and app agrees on: JSON Schema documents, one per shape, language-agnostic.
- A kernel in TypeScript, Rust or C that passes `spec.test.ts` replaces the one in `../kernel/`, and nothing above it notices.
- Version `0.1` (`index.ts` `SPEC`); a document of another major is refused, never migrated.

## SHAPES

- `act.json`: one act in, `{ verb, args, who, at }`; the only way anything changes.
- `line.json`: one journal line, an act stamped with its `tick`; `n` counts repeats folded into it.
- `move.json`: one legal verb now, `{ verb, line, args }`, `args` a schema of the subset below.
- `manifest.json`: an installed app, `{ id, user, title, line, verbs, beat }`.
- `envelope.json`: the answer to every call and read, `{ spec, tick, route, state, moves, beat, last, wishes, notices, paid? }`; `paid { from, to, amount }` is there once an act moved money.
- `config.json`: what boots a world, `{ name, seed, time, apps, actors }`.
- `fault.json`: a broken request, `{ error: { code, note } }`; a refusal is an outcome in `last`, never a fault.

## NAMES

- A verb is `<user>.<app>.<verb>` for a user's app (`mrlyprod.snake.set`), `<app>.<verb>` for a system app; `nav` is the kernel's own (`nav.open`).
- An app id and a verb name are lowercase letters and digits.
- `who.kind` is `human`, `org`, `bot` or `agent`; an agent carries its human's `id`.
- Fault codes: `bad-act`, `bad-config`, `unknown-verb`, `unknown-world`, `no-iden`, `refused`, `spec-mismatch`.

## SUBSET

- Args are a JSON Schema subset, so MCP, the goose and a form read one grammar: `type` (one of integer, number, string, boolean, object, array, null), `enum`, `const`, `minimum`, `maximum`, `multipleOf`, `minLength`, `maxLength`, `pattern`, `properties`, `required`, `additionalProperties`, `items`, `minItems`, `maxItems`, plus `title`, `line` and `default` as labels.
- `spec.test.ts` proves every schema here stays inside the subset, and that what the kernel speaks passes them.

## NOT YET

- The skin, the design parts and the site config live in the shell today (`../shell/design.ts`, `../README.md`); they join here once a second implementation reads them.
