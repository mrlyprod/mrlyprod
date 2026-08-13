use crate::math;
use crate::ops;
use crate::ops::{Map, Reduce, Zip};
use crate::tensor::Tensor;
use crate::Result;

/// A handle to one recorded value on a tape.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Var(usize);

#[derive(Clone)]
enum Op {
    Leaf,
    Matmul(usize, usize),
    Add(usize, usize),
    Mul(usize, usize),
    Relu(usize),
    Tanh(usize),
    Mean(usize),
    Mse(usize, usize),
    SoftmaxXent(usize, Vec<usize>),
    Gather(usize, Vec<usize>),
}

struct Node {
    op: Op,
    value: Tensor,
    grad: Tensor,
    aux: Option<Tensor>,
}

/// A tape of recorded ops that replays backward to fill every gradient.
pub struct Tape {
    nodes: Vec<Node>,
}

impl Tape {
    /// Builds an empty tape.
    pub fn new() -> Tape {
        Tape { nodes: Vec::new() }
    }

    fn push(&mut self, op: Op, value: Tensor, aux: Option<Tensor>) -> Var {
        let grad = Tensor::zeros(&value.shape);
        self.nodes.push(Node {
            op,
            value,
            grad,
            aux,
        });
        Var(self.nodes.len() - 1)
    }

    /// Records an input or parameter leaf.
    pub fn leaf(&mut self, value: Tensor) -> Var {
        self.push(Op::Leaf, value, None)
    }

    /// Records a matrix product, or an error when the shapes do not chain.
    pub fn matmul(&mut self, a: Var, b: Var) -> Result<Var> {
        let value = self.nodes[a.0].value.matmul(&self.nodes[b.0].value)?;
        Ok(self.push(Op::Matmul(a.0, b.0), value, None))
    }

    /// Records an elementwise sum, broadcasting a single row over rows, or an error for other mixes.
    pub fn add(&mut self, a: Var, b: Var) -> Result<Var> {
        let av = &self.nodes[a.0].value;
        let bv = &self.nodes[b.0].value;
        if av.shape == bv.shape {
            let value = av.add(bv)?;
            return Ok(self.push(Op::Add(a.0, b.0), value, None));
        }
        let broadcast = av.shape.len() == 2
            && bv.shape.len() == 2
            && bv.shape[0] == 1
            && bv.shape[1] == av.shape[1];
        if !broadcast {
            return Err("shapes do not add");
        }
        let (m, n) = (av.shape[0], av.shape[1]);
        let mut value = Tensor::zeros(&[m, n]);
        for row in 0..m {
            ops::zip(
                Zip::Add,
                &av.data[row * n..(row + 1) * n],
                &bv.data,
                &mut value.data[row * n..(row + 1) * n],
            );
        }
        Ok(self.push(Op::Add(a.0, b.0), value, None))
    }

    /// Records an elementwise product, or an error when the shapes differ.
    pub fn mul(&mut self, a: Var, b: Var) -> Result<Var> {
        let value = self.nodes[a.0].value.mul(&self.nodes[b.0].value)?;
        Ok(self.push(Op::Mul(a.0, b.0), value, None))
    }

    /// Records a rectified linear unit.
    pub fn relu(&mut self, a: Var) -> Var {
        let x = &self.nodes[a.0].value;
        let mut value = Tensor::zeros(&x.shape);
        ops::map(Map::Relu, &x.data, &mut value.data);
        self.push(Op::Relu(a.0), value, None)
    }

    /// Records a hyperbolic tangent.
    pub fn tanh(&mut self, a: Var) -> Var {
        let x = &self.nodes[a.0].value;
        let mut value = Tensor::zeros(&x.shape);
        ops::map(Map::Tanh, &x.data, &mut value.data);
        self.push(Op::Tanh(a.0), value, None)
    }

    /// Records the mean of every value as a scalar.
    pub fn mean(&mut self, a: Var) -> Var {
        let value = Tensor::full(&[1], self.nodes[a.0].value.mean());
        self.push(Op::Mean(a.0), value, None)
    }

    /// Records the mean squared error against a target, or an error when the shapes differ.
    pub fn mse(&mut self, pred: Var, target: Var) -> Result<Var> {
        let diff = self.nodes[pred.0].value.sub(&self.nodes[target.0].value)?;
        let mut sq = vec![0.0f32; diff.size()];
        ops::zip(Zip::Mul, &diff.data, &diff.data, &mut sq);
        let n = diff.size().max(1);
        let value = Tensor::full(&[1], ops::reduce(Reduce::Sum, &sq) / n as f32);
        Ok(self.push(Op::Mse(pred.0, target.0), value, Some(diff)))
    }

    /// Records fused softmax and cross entropy over logit rows, or an error for bad labels.
    pub fn softmax_xent(&mut self, logits: Var, labels: &[usize]) -> Result<Var> {
        let lv = &self.nodes[logits.0].value;
        if lv.shape.len() != 2 || lv.shape[0] == 0 {
            return Err("softmax needs a 2d tensor with rows");
        }
        let (rows, cols) = (lv.shape[0], lv.shape[1]);
        if labels.len() != rows {
            return Err("labels do not match the rows");
        }
        if labels.iter().any(|&l| l >= cols) {
            return Err("label past the columns");
        }
        let mut probs = Tensor::zeros(&[rows, cols]);
        let mut shifted = vec![0.0f32; cols];
        let mut exped = vec![0.0f32; cols];
        let mut total = 0.0f32;
        for (r, &label) in labels.iter().enumerate() {
            let row = &lv.data[r * cols..(r + 1) * cols];
            let peak = ops::reduce(Reduce::Max, row);
            ops::map(Map::Shift(-peak), row, &mut shifted);
            ops::map(Map::Exp, &shifted, &mut exped);
            let sum = ops::reduce(Reduce::Sum, &exped);
            ops::map(
                Map::Scale(1.0 / sum),
                &exped,
                &mut probs.data[r * cols..(r + 1) * cols],
            );
            total -= math::ln(probs.data[r * cols + label] as f64) as f32;
        }
        let value = Tensor::full(&[1], total / rows as f32);
        Ok(self.push(
            Op::SoftmaxXent(logits.0, labels.to_vec()),
            value,
            Some(probs),
        ))
    }

    /// Records a row gather from a 2d table, or an error for an empty or out-of-range index.
    pub fn gather(&mut self, table: Var, index: &[usize]) -> Result<Var> {
        let tv = &self.nodes[table.0].value;
        if tv.shape.len() != 2 {
            return Err("gather needs a 2d table");
        }
        if index.is_empty() {
            return Err("index is empty");
        }
        let (rows, width) = (tv.shape[0], tv.shape[1]);
        if index.iter().any(|&r| r >= rows) {
            return Err("index past the rows");
        }
        let mut value = Tensor::zeros(&[index.len(), width]);
        for (r, &row) in index.iter().enumerate() {
            value.data[r * width..(r + 1) * width]
                .copy_from_slice(&tv.data[row * width..(row + 1) * width]);
        }
        Ok(self.push(Op::Gather(table.0, index.to_vec()), value, None))
    }

    /// Reads the forward value at a handle, or panics for a handle off this tape.
    pub fn value(&self, v: Var) -> &Tensor {
        &self.nodes[v.0].value
    }

    /// Reads the gradient at a handle, zeros before backward runs, or panics off this tape.
    pub fn grad(&self, v: Var) -> &Tensor {
        &self.nodes[v.0].grad
    }

    /// Fills every gradient by walking the tape backward, or an error for a non-scalar loss.
    pub fn backward(&mut self, loss: Var) -> Result<()> {
        if self.nodes[loss.0].value.size() != 1 {
            return Err("backward needs a scalar loss");
        }
        self.nodes[loss.0].grad.data[0] += 1.0;
        for i in (0..=loss.0).rev() {
            self.flow(i)?;
        }
        Ok(())
    }

    fn spill(&mut self, target: usize, alpha: f32, contribution: &[f32]) {
        ops::axpy(alpha, contribution, &mut self.nodes[target].grad.data);
    }

    fn flow(&mut self, i: usize) -> Result<()> {
        match self.nodes[i].op.clone() {
            Op::Leaf => {}
            Op::Matmul(a, b) => {
                let g = self.nodes[i].grad.clone();
                let (m, n) = (g.shape[0], g.shape[1]);
                let k = self.nodes[a].value.shape[1];
                let bt = self.nodes[b].value.transpose()?;
                let mut da = vec![0.0f32; m * k];
                ops::gemm(&g.data, &bt.data, m, n, k, &mut da);
                self.spill(a, 1.0, &da);
                let at = self.nodes[a].value.transpose()?;
                let mut db = vec![0.0f32; k * n];
                ops::gemm(&at.data, &g.data, k, m, n, &mut db);
                self.spill(b, 1.0, &db);
            }
            Op::Add(a, b) => {
                let g = self.nodes[i].grad.clone();
                self.spill(a, 1.0, &g.data);
                if self.nodes[b].value.shape == g.shape {
                    self.spill(b, 1.0, &g.data);
                } else {
                    let (m, n) = (g.shape[0], g.shape[1]);
                    let ones = vec![1.0f32; m];
                    let mut db = vec![0.0f32; n];
                    ops::gemm(&ones, &g.data, 1, m, n, &mut db);
                    self.spill(b, 1.0, &db);
                }
            }
            Op::Mul(a, b) => {
                let g = self.nodes[i].grad.clone();
                let mut da = vec![0.0f32; g.size()];
                ops::zip(Zip::Mul, &g.data, &self.nodes[b].value.data, &mut da);
                self.spill(a, 1.0, &da);
                let mut db = vec![0.0f32; g.size()];
                ops::zip(Zip::Mul, &g.data, &self.nodes[a].value.data, &mut db);
                self.spill(b, 1.0, &db);
            }
            Op::Relu(a) => {
                let g = self.nodes[i].grad.clone();
                let mut gate = vec![0.0f32; g.size()];
                ops::map(Map::Step, &self.nodes[a].value.data, &mut gate);
                let mut da = vec![0.0f32; g.size()];
                ops::zip(Zip::Mul, &g.data, &gate, &mut da);
                self.spill(a, 1.0, &da);
            }
            Op::Tanh(a) => {
                let g = self.nodes[i].grad.clone();
                let mut slope = vec![0.0f32; g.size()];
                ops::zip(
                    Zip::Mul,
                    &self.nodes[i].value.data,
                    &self.nodes[i].value.data,
                    &mut slope,
                );
                let mut flipped = vec![0.0f32; g.size()];
                ops::map(Map::Scale(-1.0), &slope, &mut flipped);
                ops::map(Map::Shift(1.0), &flipped, &mut slope);
                let mut da = vec![0.0f32; g.size()];
                ops::zip(Zip::Mul, &g.data, &slope, &mut da);
                self.spill(a, 1.0, &da);
            }
            Op::Mean(a) => {
                let gval = self.nodes[i].grad.data[0];
                let n = self.nodes[a].value.size();
                if n > 0 {
                    let ones = vec![1.0f32; n];
                    self.spill(a, gval / n as f32, &ones);
                }
            }
            Op::Mse(p, t) => {
                let gval = self.nodes[i].grad.data[0];
                let Some(diff) = self.nodes[i].aux.clone() else {
                    return Err("tape lost its residual");
                };
                let coef = 2.0 * gval / diff.size().max(1) as f32;
                self.spill(p, coef, &diff.data);
                self.spill(t, -coef, &diff.data);
            }
            Op::SoftmaxXent(l, labels) => {
                let gval = self.nodes[i].grad.data[0];
                let Some(probs) = self.nodes[i].aux.clone() else {
                    return Err("tape lost its softmax");
                };
                let (rows, cols) = (probs.shape[0], probs.shape[1]);
                let mut delta = probs.data;
                for (r, &label) in labels.iter().enumerate() {
                    delta[r * cols + label] -= 1.0;
                }
                self.spill(l, gval / rows as f32, &delta);
            }
            Op::Gather(t, index) => {
                let g = self.nodes[i].grad.clone();
                let width = g.shape[1];
                let mut dt = vec![0.0f32; self.nodes[t].value.size()];
                for (r, &row) in index.iter().enumerate() {
                    ops::axpy(
                        1.0,
                        &g.data[r * width..(r + 1) * width],
                        &mut dt[row * width..(row + 1) * width],
                    );
                }
                self.spill(t, 1.0, &dt);
            }
        }
        Ok(())
    }
}

impl Default for Tape {
    fn default() -> Tape {
        Tape::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::Rng;

    #[test]
    fn forward_values_match_hand_math() {
        let mut tape = Tape::new();
        let x = tape.leaf(Tensor::from_rows(&[vec![1.0, -2.0]]).unwrap());
        let w = tape.leaf(Tensor::from_rows(&[vec![1.0, 0.0], vec![0.0, 1.0]]).unwrap());
        let b = tape.leaf(Tensor::from_rows(&[vec![0.5, 0.5]]).unwrap());
        let y = tape.matmul(x, w).unwrap();
        let y = tape.add(y, b).unwrap();
        assert_eq!(tape.value(y).data, vec![1.5, -1.5]);
        let r = tape.relu(y);
        assert_eq!(tape.value(r).data, vec![1.5, 0.0]);
        let m = tape.mean(r);
        assert_eq!(tape.value(m).data, vec![0.75]);
        assert_eq!(tape.backward(r), Err("backward needs a scalar loss"));
        tape.backward(m).unwrap();
        assert_eq!(tape.grad(y).data, vec![0.5, 0.0]);
    }

    #[test]
    fn broadcast_add_sums_bias_grads_over_rows() {
        let mut tape = Tape::new();
        let x = tape.leaf(Tensor::from_rows(&[vec![1.0, 2.0], vec![3.0, 4.0]]).unwrap());
        let b = tape.leaf(Tensor::from_rows(&[vec![10.0, 20.0]]).unwrap());
        let y = tape.add(x, b).unwrap();
        assert_eq!(tape.value(y).data, vec![11.0, 22.0, 13.0, 24.0]);
        let m = tape.mean(y);
        tape.backward(m).unwrap();
        assert_eq!(tape.grad(b).data, vec![0.5, 0.5]);
        let odd = tape.leaf(Tensor::zeros(&[3]));
        assert_eq!(tape.add(x, odd), Err("shapes do not add"));
    }

    #[test]
    fn softmax_probs_row_to_one_and_reject_bad_labels() {
        let mut tape = Tape::new();
        let logits =
            tape.leaf(Tensor::from_rows(&[vec![2.0, 1.0, 0.0], vec![0.0, 0.0, 0.0]]).unwrap());
        assert_eq!(
            tape.softmax_xent(logits, &[0]),
            Err("labels do not match the rows")
        );
        assert_eq!(
            tape.softmax_xent(logits, &[0, 9]),
            Err("label past the columns")
        );
        let loss = tape.softmax_xent(logits, &[0, 2]).unwrap();
        assert!(tape.value(loss).data[0] > 0.0);
        tape.backward(loss).unwrap();
        let g = tape.grad(logits);
        assert!((g.data[..3].iter().sum::<f32>()).abs() < 1e-6);
        assert!((g.data[3..].iter().sum::<f32>()).abs() < 1e-6);
    }

    #[test]
    fn gather_scatters_its_grads_back() {
        let mut tape = Tape::new();
        let table = tape.leaf(Tensor::from_rows(&[vec![1.0, 2.0], vec![3.0, 4.0]]).unwrap());
        let rows = tape.gather(table, &[1, 1, 0]).unwrap();
        assert_eq!(tape.value(rows).data, vec![3.0, 4.0, 3.0, 4.0, 1.0, 2.0]);
        let m = tape.mean(rows);
        tape.backward(m).unwrap();
        let g = tape.grad(table);
        let unit = 1.0 / 6.0;
        assert!((g.data[0] - unit).abs() < 1e-6);
        assert!((g.data[2] - 2.0 * unit).abs() < 1e-6);
        assert_eq!(tape.gather(table, &[]), Err("index is empty"));
        assert_eq!(tape.gather(table, &[5]), Err("index past the rows"));
    }

    fn xent_net_loss(params: &[Tensor], x: &Tensor, labels: &[usize]) -> f32 {
        let mut tape = Tape::new();
        let xv = tape.leaf(x.clone());
        let w1 = tape.leaf(params[0].clone());
        let b1 = tape.leaf(params[1].clone());
        let w2 = tape.leaf(params[2].clone());
        let b2 = tape.leaf(params[3].clone());
        let h = tape.matmul(xv, w1).unwrap();
        let h = tape.add(h, b1).unwrap();
        let h = tape.tanh(h);
        let o = tape.matmul(h, w2).unwrap();
        let o = tape.add(o, b2).unwrap();
        let loss = tape.softmax_xent(o, labels).unwrap();
        tape.value(loss).data[0]
    }

    fn mse_net_loss(params: &[Tensor], x: &Tensor, target: &Tensor) -> f32 {
        let mut tape = Tape::new();
        let xv = tape.leaf(x.clone());
        let w1 = tape.leaf(params[0].clone());
        let b1 = tape.leaf(params[1].clone());
        let gain = tape.leaf(params[2].clone());
        let h = tape.matmul(xv, w1).unwrap();
        let h = tape.add(h, b1).unwrap();
        let h = tape.relu(h);
        let z = tape.mul(h, gain).unwrap();
        let t = tape.leaf(target.clone());
        let loss = tape.mse(z, t).unwrap();
        tape.value(loss).data[0]
    }

    fn check_numeric(params: &mut [Tensor], grads: &[Tensor], loss_of: &dyn Fn(&[Tensor]) -> f32) {
        let eps = 1e-2f32;
        for pi in 0..params.len() {
            for j in 0..params[pi].size() {
                let keep = params[pi].data[j];
                params[pi].data[j] = keep + eps;
                let up = loss_of(params);
                params[pi].data[j] = keep - eps;
                let down = loss_of(params);
                params[pi].data[j] = keep;
                let numeric = (up - down) / (2.0 * eps);
                let analytic = grads[pi].data[j];
                assert!(
                    (numeric - analytic).abs() < 1e-3 * (1.0 + analytic.abs()),
                    "param {pi} slot {j}: numeric {numeric} vs analytic {analytic}"
                );
            }
        }
    }

    #[test]
    fn xent_gradients_match_numeric_differences() {
        let mut rng = Rng::new(crate::seed(5, 0));
        let mut params = Vec::new();
        for shape in [[2, 3], [1, 3], [3, 2], [1, 2]] {
            let mut t = Tensor::zeros(&shape);
            rng.fill_normal(&mut t.data, 0.0, 0.7);
            params.push(t);
        }
        let mut x = Tensor::zeros(&[4, 2]);
        rng.fill_uniform(&mut x.data, -1.0, 1.0);
        let labels = [0, 1, 1, 0];
        let mut tape = Tape::new();
        let xv = tape.leaf(x.clone());
        let held: Vec<Var> = params.iter().map(|p| tape.leaf(p.clone())).collect();
        let h = tape.matmul(xv, held[0]).unwrap();
        let h = tape.add(h, held[1]).unwrap();
        let h = tape.tanh(h);
        let o = tape.matmul(h, held[2]).unwrap();
        let o = tape.add(o, held[3]).unwrap();
        let loss = tape.softmax_xent(o, &labels).unwrap();
        tape.backward(loss).unwrap();
        let grads: Vec<Tensor> = held.iter().map(|v| tape.grad(*v).clone()).collect();
        check_numeric(&mut params, &grads, &|p| xent_net_loss(p, &x, &labels));
    }

    #[test]
    fn mse_gradients_match_numeric_differences() {
        let mut rng = Rng::new(crate::seed(6, 0));
        let mut params = Vec::new();
        for shape in [[3, 4], [1, 4], [5, 4]] {
            let mut t = Tensor::zeros(&shape);
            rng.fill_normal(&mut t.data, 0.0, 0.8);
            params.push(t);
        }
        let mut x = Tensor::zeros(&[5, 3]);
        rng.fill_uniform(&mut x.data, -1.0, 1.0);
        let mut target = Tensor::zeros(&[5, 4]);
        rng.fill_uniform(&mut target.data, -1.0, 1.0);
        let mut tape = Tape::new();
        let xv = tape.leaf(x.clone());
        let held: Vec<Var> = params.iter().map(|p| tape.leaf(p.clone())).collect();
        let h = tape.matmul(xv, held[0]).unwrap();
        let h = tape.add(h, held[1]).unwrap();
        let h = tape.relu(h);
        let z = tape.mul(h, held[2]).unwrap();
        let t = tape.leaf(target.clone());
        let loss = tape.mse(z, t).unwrap();
        tape.backward(loss).unwrap();
        let grads: Vec<Tensor> = held.iter().map(|v| tape.grad(*v).clone()).collect();
        check_numeric(&mut params, &grads, &|p| mse_net_loss(p, &x, &target));
    }
}
