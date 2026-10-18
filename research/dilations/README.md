---
title: Dilated stacks
lead: The effective receptive field of a dilated convolution stack is a digit-design representation count: Stern's diatomic sequence for three taps at base 2, with Fibonacci peaks and `2L + 1` single-path lags, and a depth limit that is absolutely continuous exactly when the block polynomial vanishes at the right roots of unity, which the two common residual blocks break at every gain but at most one; the count survives the ReLU, whose gradient field at initialisation equals `s(n + 1)` at every lag within Monte Carlo error, a flat initialisation removes the residual stack's popcount bias, and learn time follows the count only against a single-path stack.
figure: research-dilations
slug: dilations
---

# Dilated stacks

The effective receptive field of a dilated convolution stack is a digit-design representation count: Stern's diatomic sequence for three taps at base 2, with Fibonacci peaks and `2L + 1` single-path lags, and a depth limit that is absolutely continuous exactly when the block polynomial vanishes at the right roots of unity, which the two common residual blocks break at every gain but at most one; the count survives the ReLU, whose gradient field at initialisation equals `s(n + 1)` at every lag within Monte Carlo error, a flat initialisation removes the residual stack's popcount bias, and learn time follows the count only against a single-path stack.

- [notes/dilations.md](notes/dilations.md): Dilated stacks
- [claims/dilated-receptive-fields.md](claims/dilated-receptive-fields.md): Dilated receptive fields
- [claims/lag-bias.md](claims/lag-bias.md): Lag bias
- [refs.md](refs.md): the references this topic cites
- [lab/py/dilated-receptive-field/](lab/py/dilated-receptive-field/): The effective receptive field of a dilated convolution stack read as a digit count, the numbers of dilations.
- [lab/py/lag-bias/](lab/py/lag-bias/): The path count against a ReLU stack's gradient field at initialisation, the popcount law of the residual stack and the flat init that removes it, and the learn time of eight lags under Adam and SGD.
- [figures/research-dilations.ts](figures/research-dilations.ts): research-dilations
- [figures/research-dilations.json](figures/research-dilations.json): research-dilations census
