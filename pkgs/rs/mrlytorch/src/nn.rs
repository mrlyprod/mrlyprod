use crate::graph::{Tape, Var};
use crate::rng::Rng;
use crate::tensor::Tensor;
use crate::Result;

/// The activations a layer can apply.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Act {
    /// Passes values through unchanged.
    Ident,
    /// Clamps negatives to zero.
    Relu,
    /// Squashes values through the hyperbolic tangent.
    Tanh,
}

impl Act {
    /// Returns the display name.
    pub fn name(self) -> &'static str {
        match self {
            Act::Ident => "ident",
            Act::Relu => "relu",
            Act::Tanh => "tanh",
        }
    }

    /// Parses a display name back into its activation, or an error for an unknown name.
    pub fn parse(text: &str) -> Result<Act> {
        match text {
            "ident" => Ok(Act::Ident),
            "relu" => Ok(Act::Relu),
            "tanh" => Ok(Act::Tanh),
            _ => Err("no such activation"),
        }
    }

    /// Lists every activation.
    pub fn all() -> [Act; 3] {
        [Act::Ident, Act::Relu, Act::Tanh]
    }

    /// Records the activation on a tape.
    pub fn apply(self, tape: &mut Tape, x: Var) -> Var {
        match self {
            Act::Ident => x,
            Act::Relu => tape.relu(x),
            Act::Tanh => tape.tanh(x),
        }
    }
}

/// A fully connected layer holding a weight matrix and a bias row.
#[derive(Clone, Debug)]
pub struct Linear {
    /// The weights, inputs by outputs.
    pub w: Tensor,
    /// The bias row, one by outputs.
    pub b: Tensor,
}

impl Linear {
    /// Builds the layer with He-scaled normal weights and a zero bias.
    pub fn he(inputs: usize, outputs: usize, rng: &mut Rng) -> Linear {
        let sd = (2.0 / inputs.max(1) as f32).sqrt();
        let mut w = Tensor::zeros(&[inputs, outputs]);
        rng.fill_normal(&mut w.data, 0.0, sd);
        Linear {
            w,
            b: Tensor::zeros(&[1, outputs]),
        }
    }

    /// Builds the layer with Xavier-scaled uniform weights and a zero bias.
    pub fn xavier(inputs: usize, outputs: usize, rng: &mut Rng) -> Linear {
        let bound = (6.0 / (inputs + outputs).max(1) as f32).sqrt();
        let mut w = Tensor::zeros(&[inputs, outputs]);
        rng.fill_uniform(&mut w.data, -bound, bound);
        Linear {
            w,
            b: Tensor::zeros(&[1, outputs]),
        }
    }

    /// Records the affine map, returning the output and the weight and bias handles.
    pub fn forward(&self, tape: &mut Tape, x: Var) -> Result<(Var, [Var; 2])> {
        let w = tape.leaf(self.w.clone());
        let b = tape.leaf(self.b.clone());
        let y = tape.matmul(x, w)?;
        let out = tape.add(y, b)?;
        Ok((out, [w, b]))
    }
}

/// A sequential stack of linear layers with one hidden activation.
#[derive(Clone, Debug)]
pub struct Mlp {
    /// The layers in order.
    pub layers: Vec<Linear>,
    /// The activation between layers, never after the last.
    pub hidden: Act,
}

impl Mlp {
    /// Builds the stack from layer sizes, He behind relu and Xavier otherwise.
    pub fn new(sizes: &[usize], hidden: Act, rng: &mut Rng) -> Mlp {
        let mut layers = Vec::new();
        for pair in sizes.windows(2) {
            layers.push(match hidden {
                Act::Relu => Linear::he(pair[0], pair[1], rng),
                _ => Linear::xavier(pair[0], pair[1], rng),
            });
        }
        Mlp { layers, hidden }
    }

    /// Records the whole stack, returning the output and every parameter handle in layer order.
    pub fn forward(&self, tape: &mut Tape, x: Var) -> Result<(Var, Vec<Var>)> {
        let mut flow = x;
        let mut params = Vec::new();
        let last = self.layers.len().saturating_sub(1);
        for (i, layer) in self.layers.iter().enumerate() {
            let (y, held) = layer.forward(tape, flow)?;
            params.extend(held);
            flow = if i < last {
                self.hidden.apply(tape, y)
            } else {
                y
            };
        }
        Ok((flow, params))
    }

    /// Borrows every parameter tensor in the order forward reports handles.
    pub fn params_mut(&mut self) -> Vec<&mut Tensor> {
        let mut out = Vec::new();
        for layer in &mut self.layers {
            out.push(&mut layer.w);
            out.push(&mut layer.b);
        }
        out
    }
}

/// A lookup table mapping indices to learned rows.
#[derive(Clone, Debug)]
pub struct Embed {
    /// The table, entries by width.
    pub table: Tensor,
}

impl Embed {
    /// Builds the table with small normal entries.
    pub fn new(entries: usize, width: usize, rng: &mut Rng) -> Embed {
        let mut table = Tensor::zeros(&[entries, width]);
        rng.fill_normal(&mut table.data, 0.0, 0.02);
        Embed { table }
    }

    /// Records a row lookup, returning the gathered rows and the table handle.
    pub fn forward(&self, tape: &mut Tape, index: &[usize]) -> Result<(Var, Var)> {
        let table = tape.leaf(self.table.clone());
        let rows = tape.gather(table, index)?;
        Ok((rows, table))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn act_names_roundtrip() {
        for act in Act::all() {
            assert_eq!(Act::parse(act.name()), Ok(act));
        }
        assert_eq!(Act::parse("gelu"), Err("no such activation"));
    }

    #[test]
    fn linear_forward_matches_manual_math() {
        let mut layer = Linear::he(2, 2, &mut Rng::new(crate::seed(9, 0)));
        layer.w = Tensor::from_rows(&[vec![1.0, 2.0], vec![3.0, 4.0]]).unwrap();
        layer.b = Tensor::from_rows(&[vec![0.5, -0.5]]).unwrap();
        let mut tape = Tape::new();
        let x = tape.leaf(Tensor::from_rows(&[vec![1.0, 1.0]]).unwrap());
        let (y, held) = layer.forward(&mut tape, x).unwrap();
        assert_eq!(tape.value(y).data, vec![4.5, 5.5]);
        assert_eq!(tape.value(held[0]).data, layer.w.data);
        assert_eq!(tape.value(held[1]).data, layer.b.data);
    }

    #[test]
    fn init_replays_from_the_same_seed() {
        let mut a = Rng::new(crate::seed(10, 0));
        let mut b = Rng::new(crate::seed(10, 0));
        let one = Mlp::new(&[4, 8, 2], Act::Relu, &mut a);
        let two = Mlp::new(&[4, 8, 2], Act::Relu, &mut b);
        assert_eq!(one.layers[0].w.data, two.layers[0].w.data);
        assert_eq!(one.layers[1].w.data, two.layers[1].w.data);
        let three = Mlp::new(&[4, 8, 2], Act::Tanh, &mut Rng::new(crate::seed(10, 1)));
        assert_ne!(one.layers[0].w.data, three.layers[0].w.data);
    }

    #[test]
    fn mlp_handles_line_up_with_params() {
        let mut rng = Rng::new(crate::seed(11, 0));
        let mut mlp = Mlp::new(&[3, 5, 2], Act::Tanh, &mut rng);
        let mut tape = Tape::new();
        let x = tape.leaf(Tensor::zeros(&[1, 3]));
        let (y, held) = mlp.forward(&mut tape, x).unwrap();
        assert_eq!(tape.value(y).shape, vec![1, 2]);
        let params = mlp.params_mut();
        assert_eq!(held.len(), params.len());
        for (h, p) in held.iter().zip(&params) {
            assert_eq!(tape.value(*h).shape, p.shape);
        }
    }

    #[test]
    fn embed_gathers_learned_rows() {
        let mut rng = Rng::new(crate::seed(12, 0));
        let embed = Embed::new(6, 3, &mut rng);
        let mut tape = Tape::new();
        let (rows, table) = embed.forward(&mut tape, &[4, 0, 4]).unwrap();
        assert_eq!(tape.value(rows).shape, vec![3, 3]);
        assert_eq!(tape.value(rows).data[..3], tape.value(rows).data[6..]);
        assert_eq!(tape.value(table).data, embed.table.data);
    }
}
