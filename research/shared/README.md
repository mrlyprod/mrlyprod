# Shared

The machinery every topic leans on: the sequence ledger, the claims checker, the life atlas and the figure census.

- [ledger/](ledger/): walks `research/*/notes` and `research/*/claims` and writes `sequences.md`.
- [claims/](claims/): re-runs claim witnesses.
- [atlas/](atlas/): the library behind `automata/lab/rs/life-atlas`.
- [census/](census/): one bin a figure, writing the JSON beside its figure.
- [figures/research-sequences.ts](figures/research-sequences.ts): the sequences figure, six codes by four odd sides, each checked against its fill count.
- [refs.md](refs.md): the OEIS rows the ledger cites.
- [sequences.md](sequences.md): the generated OEIS ledger every topic links.
