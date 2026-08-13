use crate::ops;
use crate::tensor::Tensor;
use crate::Result;

fn check(params: &[&mut Tensor], grads: &[Tensor]) -> Result<()> {
    if params.len() != grads.len() {
        return Err("params and grads differ in count");
    }
    for (param, grad) in params.iter().zip(grads) {
        if param.shape != grad.shape {
            return Err("param and grad shapes differ");
        }
    }
    Ok(())
}

/// Stochastic gradient descent with classical momentum.
pub struct Sgd {
    /// The learning rate.
    pub lr: f32,
    /// The momentum factor, zero for plain descent.
    pub momentum: f32,
    velocity: Vec<Vec<f32>>,
}

impl Sgd {
    /// Builds the optimiser.
    pub fn new(lr: f32, momentum: f32) -> Sgd {
        Sgd {
            lr,
            momentum,
            velocity: Vec::new(),
        }
    }

    /// Applies one update, or an error when params and grads do not line up.
    pub fn step(&mut self, params: &mut [&mut Tensor], grads: &[Tensor]) -> Result<()> {
        check(params, grads)?;
        if self.velocity.len() != params.len() {
            self.velocity = params.iter().map(|p| vec![0.0; p.size()]).collect();
        }
        for ((param, grad), vel) in params.iter_mut().zip(grads).zip(&mut self.velocity) {
            for (v, &g) in vel.iter_mut().zip(&grad.data) {
                *v = self.momentum * *v + g;
            }
            ops::axpy(-self.lr, vel, &mut param.data);
        }
        Ok(())
    }
}

/// The Adam optimiser with bias-corrected moments.
pub struct Adam {
    /// The learning rate.
    pub lr: f32,
    /// The first-moment decay.
    pub beta1: f32,
    /// The second-moment decay.
    pub beta2: f32,
    /// The numerical floor beside the root.
    pub eps: f32,
    b1_pow: f32,
    b2_pow: f32,
    first: Vec<Vec<f32>>,
    second: Vec<Vec<f32>>,
}

impl Adam {
    /// Builds the optimiser with the usual decay pair.
    pub fn new(lr: f32) -> Adam {
        Adam {
            lr,
            beta1: 0.9,
            beta2: 0.999,
            eps: 1e-8,
            b1_pow: 1.0,
            b2_pow: 1.0,
            first: Vec::new(),
            second: Vec::new(),
        }
    }

    /// Applies one update, or an error when params and grads do not line up.
    pub fn step(&mut self, params: &mut [&mut Tensor], grads: &[Tensor]) -> Result<()> {
        check(params, grads)?;
        if self.first.len() != params.len() {
            self.first = params.iter().map(|p| vec![0.0; p.size()]).collect();
            self.second = params.iter().map(|p| vec![0.0; p.size()]).collect();
        }
        self.b1_pow *= self.beta1;
        self.b2_pow *= self.beta2;
        let correct1 = 1.0 - self.b1_pow;
        let correct2 = 1.0 - self.b2_pow;
        for (((param, grad), first), second) in params
            .iter_mut()
            .zip(grads)
            .zip(&mut self.first)
            .zip(&mut self.second)
        {
            for (((p, &g), m), v) in param
                .data
                .iter_mut()
                .zip(&grad.data)
                .zip(first.iter_mut())
                .zip(second.iter_mut())
            {
                *m = self.beta1 * *m + (1.0 - self.beta1) * g;
                *v = self.beta2 * *v + (1.0 - self.beta2) * g * g;
                let lead = *m / correct1;
                let scale = (*v / correct2).sqrt() + self.eps;
                *p -= self.lr * lead / scale;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Tape;
    use crate::nn::{Act, Mlp};
    use crate::rng::Rng;

    #[test]
    fn sgd_steps_by_hand() {
        let mut p = Tensor::full(&[2], 1.0);
        let g = Tensor::full(&[2], 0.5);
        let mut sgd = Sgd::new(0.1, 0.0);
        sgd.step(&mut [&mut p], std::slice::from_ref(&g)).unwrap();
        assert_eq!(p.data, vec![0.95, 0.95]);
        let mut heavy = Sgd::new(0.1, 0.5);
        let mut q = Tensor::full(&[1], 0.0);
        heavy
            .step(&mut [&mut q], &[Tensor::full(&[1], 1.0)])
            .unwrap();
        heavy
            .step(&mut [&mut q], &[Tensor::full(&[1], 1.0)])
            .unwrap();
        assert_eq!(q.data, vec![-0.25]);
        assert_eq!(
            heavy.step(&mut [&mut q], &[Tensor::zeros(&[3])]),
            Err("param and grad shapes differ")
        );
        assert_eq!(
            heavy.step(&mut [], &[g]),
            Err("params and grads differ in count")
        );
    }

    #[test]
    fn adam_first_step_moves_by_the_rate() {
        let mut p = Tensor::full(&[3], 1.0);
        let g = Tensor::new(vec![0.5, -2.0, 0.001], vec![3]).unwrap();
        let mut adam = Adam::new(0.1);
        adam.step(&mut [&mut p], &[g]).unwrap();
        assert!((p.data[0] - 0.9).abs() < 1e-4);
        assert!((p.data[1] - 1.1).abs() < 1e-4);
        assert!((p.data[2] - 0.9).abs() < 1e-4);
    }

    fn xor_losses(kind: &str, epochs: usize) -> Vec<f32> {
        let mut rng = Rng::new(crate::seed(13, 0));
        let mut mlp = Mlp::new(&[2, 16, 1], Act::Tanh, &mut rng);
        let inputs = Tensor::from_rows(&[
            vec![0.0, 0.0],
            vec![0.0, 1.0],
            vec![1.0, 0.0],
            vec![1.0, 1.0],
        ])
        .unwrap();
        let targets = Tensor::from_rows(&[vec![0.0], vec![1.0], vec![1.0], vec![0.0]]).unwrap();
        let mut sgd = Sgd::new(0.1, if kind == "plain" { 0.0 } else { 0.9 });
        let mut adam = Adam::new(0.02);
        let mut losses = Vec::with_capacity(epochs);
        for _ in 0..epochs {
            let mut tape = Tape::new();
            let x = tape.leaf(inputs.clone());
            let (y, held) = mlp.forward(&mut tape, x).unwrap();
            let t = tape.leaf(targets.clone());
            let loss = tape.mse(y, t).unwrap();
            tape.backward(loss).unwrap();
            let grads: Vec<Tensor> = held.iter().map(|h| tape.grad(*h).clone()).collect();
            let mut params = mlp.params_mut();
            if kind == "adam" {
                adam.step(&mut params, &grads).unwrap();
            } else {
                sgd.step(&mut params, &grads).unwrap();
            }
            losses.push(tape.value(loss).data[0]);
        }
        losses
    }

    #[test]
    fn xor_learns_and_replays_to_the_pinned_loss() {
        let losses = xor_losses("sgd", 400);
        assert!(losses.iter().any(|&l| l < 0.05), "never crossed 0.05");
        assert_eq!(*losses.last().unwrap(), 5.1347815e-16);
    }

    #[test]
    fn adam_reaches_the_threshold_before_plain_sgd() {
        let threshold = 0.05;
        let plain_first = xor_losses("plain", 2000)
            .iter()
            .position(|&l| l < threshold);
        let adam_first = xor_losses("adam", 2000).iter().position(|&l| l < threshold);
        let (Some(plain_first), Some(adam_first)) = (plain_first, adam_first) else {
            panic!("one optimiser never crossed the threshold");
        };
        assert!(
            adam_first < plain_first,
            "adam {adam_first} vs plain sgd {plain_first}"
        );
    }
}
