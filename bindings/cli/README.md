# mrly-cli

- `mrly`: every public function of [mrly](https://crates.io/crates/mrly) from the shell, JSON in, JSON out.
- Built for scripts and agents: one call, one JSON value on stdout.
- Install: `cargo install mrly-cli`; prebuilt binaries come later.

## USE

```sh
mrly list math.two
mrly help math.two.carpet
mrly math.two.carpet '[3, 1]'
```

- `mrly list [prefix]` prints every function with its signature, one per line.
- `mrly help <name>` prints one function's signature and docs.
- `mrly <name> '<json>'` calls the function and prints its result as JSON.
- Names are the Rust paths after `mrly::`: `mrly::math::two::carpet` is `math.two.carpet`.
- A JSON array holds one value per parameter, in order; any other JSON value is the one argument.
- A lone array argument goes in its own array: `mrly core.colors.css '[[0, 255, 0, 255]]'`; piped, wrap it with `jq -c '[.]'`.
- With `-` or no argument, the JSON is read from stdin, so one call pipes into the next: `mrly math.two.carpet '[3, 1]' | mrly math.two.census`.
- `--` before the JSON passes one that starts with a dash: `mrly math.three.sponge.exact -- -0.5`.
- `-r`, `--raw` prints a string result as it is, not as a JSON string.
- `mrly --help` and `mrly --version` do what they say.

## SVG

```sh
mrly math.two.carpet '[3, 3]' | jq -c '[., 4, null, 0, "Square"]' | mrly math.two.svg --raw > carpet.svg
```

## EXIT CODES

- `0`: the call ran and its result is on stdout.
- `1`: the function returned an error; the message is on stderr.
- `2`: the command was written wrong, for example an unknown name or bad JSON; close names are suggested.

## LIBRARY

- The same crate is a library: `mrly_cli::call(name, &args)` runs one call on JSON values and returns JSON.
- `mrly_cli::list()` walks every function with its signature and docs; `mrly_cli::find(name)` looks one up.

## LINKS

- What each function does: [docs.rs/mrly](https://docs.rs/mrly).
- Source: [github.com/mrlyprod/mrlyprod](https://github.com/mrlyprod/mrlyprod).
- 0.x: any minor release may change the API. MIT licensed.
