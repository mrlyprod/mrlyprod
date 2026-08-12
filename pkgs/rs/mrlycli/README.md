# mrlycli

Mrly in a terminal: REPL, TUI and MCP behind one `mrly` binary. Every face fronts the same kernel - a verb and its JSON args go in, the envelope comes back - so a game played by hand, a script replayed from a file, and an agent calling tools over stdio are all the same conversation.

Bare `mrly` reads the room: a raw-mode TUI on a terminal, a line-by-line REPL on a pipe. The rest are one-shot subcommands for scripts and tests: pipe `{"verb":"nav.open","args":{"app":"snake"}}` into `mrly render` and the board comes back as colored blocks.

- **tui** and **repl** are the hands-on faces: arrows play, `:` and `/` command.
- **mcp** speaks tools/list, tools/call and resources/read over stdio; the tool list shifts as state does.
- **run**, **render** and **shot** replay a call script to an envelope, colored blocks, or a PNG.
- **read** and **watch** print one field of an app's state, once or on every change.
- **goose** drives an app with random legal calls from a seed.
- **drive** and **frame** play a screenplay - open, call, assert - silently or as a printed TUI screen.
- **list** and **verbs** print the kernel surface and each app's verbs.
