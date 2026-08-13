# mrlytorch

The minds' frame: a cpu training stack with zero dependencies. Tensors,
a tape autograd, layers, optimisers and a grid bridge, all f32, all
hand-rolled, rooted in one mixer: `seed(train, step)` names every random
stream, so the same seed unfolds into the same floats on every run.

Determinism is the law. Math runs single threaded in a fixed reduction
order, the frame carries its own exp, ln, cos and tanh so no system
library can bend a bit, and every heavy kernel routes through one small
seam - GPU.md holds the contract a Metal or CUDA backend must sign to
slot in behind it.

## Using

- `rng::Rng::new(seed(train, step))` draws uniforms and normals.
- `nn::Mlp::new(&[2, 16, 1], nn::Act::Tanh, &mut rng)` builds a net.
- `graph::Tape::new()` starts a step; `tape.leaf(tensor)` holds inputs.
- `mlp.forward(&mut tape, x)?` returns the output and parameter handles.
- `tape.mse(y, t)?` or `tape.softmax_xent(y, &labels)?` scores it.
- `tape.backward(loss)?` fills every gradient on the tape.
- `optim::Sgd` or `optim::Adam` steps `mlp.params_mut()` by the grads.
- `grid::embed` turns a small u8 grid into one-hot planes and back.
- `nn::Embed` gathers learned rows; `tensor::Tensor` holds the math.

## Dials

- `Sgd { lr, momentum }`; momentum zero is plain descent.
- `Adam::new(lr)` with the usual 0.9 / 0.999 / 1e-8 trio.
- `nn::Act::{Ident, Relu, Tanh}` picks the hidden activation.
- He init behind relu, Xavier behind the rest.
- `tensor::Pad::{Valid, Same}` picks the conv2d padding.
- `grid::{COLORS, MAX_SIDE}` bound the grid bridge at 10 and 30.
