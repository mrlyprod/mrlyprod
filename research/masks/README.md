---
title: Design masks
lead: A design as an attention mask, two ways: laid over the query-key square, nineteen masks from every family mrly draws are no better than their random twins at equal density; laid along the offsets, a digit design reaches every lag in two hops exactly when its digit sumset covers the digits, base-3 digits `{0, 1}` do it with sixteen offsets where the strided family needs seventeen, an integer program finds a sparser set than both, and on tiny Shakespeare at 600 steps the strided mask beats the designs by `0.04` nats while every window beats dense, the loss tracking the smallest offset a mask leaves out.
figure: research-masks
slug: masks
---

# Design masks

A design as an attention mask, two ways: laid over the query-key square, nineteen masks from every family mrly draws are no better than their random twins at equal density; laid along the offsets, a digit design reaches every lag in two hops exactly when its digit sumset covers the digits, base-3 digits `{0, 1}` do it with sixteen offsets where the strided family needs seventeen, an integer program finds a sparser set than both, and on tiny Shakespeare at 600 steps the strided mask beats the designs by `0.04` nats while every window beats dense, the loss tracking the smallest offset a mask leaves out.

- [notes/masks.md](notes/masks.md): Design masks
- [claims/design-masks.md](claims/design-masks.md): Design masks
- [claims/offset-masks.md](claims/offset-masks.md): Offset masks
- [lab/py/design-masks/](lab/py/design-masks/): The mask census across the five tile groups, the hex projections and two base-3 digit designs, and the paired sweep against dense, window and random masks at equal density.
- [figures/research-masks.ts](figures/research-masks.ts): research-masks
- [figures/research-masks.json](figures/research-masks.json): research-masks census

## Lost

- Two-dimensional designs laid over the query-key square: at side 81 and 300 steps, nineteen masks from every family are no better than their random twins at equal density on copy and a little worse on recall; a lag task reads only the mask's offset profile, so the route moved to offset masks.
