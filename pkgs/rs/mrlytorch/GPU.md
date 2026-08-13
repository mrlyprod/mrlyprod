# GPU

Nothing in this crate runs on a gpu today. This file fixes the boundary
so a Metal or CUDA backend can land behind it without touching `nn`,
`graph`, `optim` or any caller.

## The seam

Every heavy kernel routes through `src/ops.rs`, and only through it:

- `gemm(a, b, m, k, n, out)` - the matrix product.
- `map(op, x, out)` - elementwise transforms picked by the `Map` enum.
- `zip(op, a, b, out)` - pairwise combines picked by the `Zip` enum.
- `reduce(op, x) -> f32` - whole-buffer folds picked by the `Reduce` enum.
- `axpy(alpha, x, y)` - the scaled accumulate, y += alpha * x.
- `conv(x, xs, k, ks, pad, out)` - the direct 2d convolution.

The signatures are device shaped on purpose: contiguous f32 buffers in,
one caller-allocated buffer out, sizes as plain integers, no borrowed
iterators, no closures, no allocation inside a kernel. Op choices are
data (`Map`, `Zip`, `Reduce` enums), not code, so they cross a device
boundary as a constant. A backend swaps the six bodies for device
dispatches and keeps a resident-buffer cache behind the same calls;
the callers never change.

## What a backend provides

The six kernels above are the whole mandatory contract. Optional fused
kernels a backend may add for speed, which the frame otherwise composes
from the six: bias-row broadcast add, softmax over rows, the tanh and
relu backward products, and the optimiser updates.

## Determinism on gpu

Parallel reductions and atomics reorder float sums, so bit equality
with the cpu is not promised across devices. The frame's answer:

- Seeds, the rng, weight init and data prep stay on the cpu forever,
  so what enters a run is bit-identical on every backend.
- The tape and its walk order stay on the cpu; only kernel bodies move.
- Within one backend a run must replay exactly: kernels fix their
  reduction trees and never race accumulations through atomics.
- Across backends, comparisons drop to a per-backend tolerance instead
  of bit equality; the cpu path stays the reference.

## Metal

`gemm` maps to MPS matrix multiplication; `map`, `zip` and `axpy` are
one-line MSL compute kernels; `reduce` and `conv` are MSL kernels with
threadgroup trees of a fixed shape so a size always folds the same way.
Buffers live in shared storage, so the cpu-side tape reads results
without copies.

## CUDA

`gemm` maps to cuBLAS sgemm with the algorithm pinned, since letting
the library pick per-run breaks replay; `map`, `zip`, `axpy`, `reduce`
and `conv` are small custom kernels with fixed grid shapes.

## What stays cpu

The rng and the seed hierarchy, weight init, the grid bridge, the tape
bookkeeping and walk, index work (row gather and scatter, argmax) and
the optimiser state in this version. They are memory bound and tiny;
moving them buys nothing and spends determinism.

A Metal backend therefore lands as one file behind `ops`: six kernel
bodies over device buffers. `nn` and `graph` never learn it happened.
