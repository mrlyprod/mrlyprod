use serde::{Deserialize, Serialize};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub const fn new(re: f64, im: f64) -> Complex {
        Complex { re, im }
    }
    pub fn abs(z: Complex) -> f64 {
        z.re.hypot(z.im)
    }
    pub fn arg(z: Complex) -> f64 {
        z.im.atan2(z.re)
    }
    pub fn exp(z: Complex) -> Complex {
        let r = z.re.exp();
        Complex::new(r * z.im.cos(), r * z.im.sin())
    }
    pub fn ln(z: Complex) -> Complex {
        Complex::new(Complex::abs(z).ln(), Complex::arg(z))
    }
    pub fn turn(angle: f64) -> Complex {
        Complex::new(angle.cos(), angle.sin())
    }
}

impl Add for Complex {
    type Output = Complex;
    fn add(self, other: Complex) -> Complex {
        Complex::new(self.re + other.re, self.im + other.im)
    }
}

impl Add<f64> for Complex {
    type Output = Complex;
    fn add(self, other: f64) -> Complex {
        Complex::new(self.re + other, self.im)
    }
}

impl Sub for Complex {
    type Output = Complex;
    fn sub(self, other: Complex) -> Complex {
        Complex::new(self.re - other.re, self.im - other.im)
    }
}

impl Sub<f64> for Complex {
    type Output = Complex;
    fn sub(self, other: f64) -> Complex {
        Complex::new(self.re - other, self.im)
    }
}

impl Mul for Complex {
    type Output = Complex;
    fn mul(self, other: Complex) -> Complex {
        Complex::new(
            self.re * other.re - self.im * other.im,
            self.re * other.im + self.im * other.re,
        )
    }
}

impl Mul<f64> for Complex {
    type Output = Complex;
    fn mul(self, other: f64) -> Complex {
        Complex::new(self.re * other, self.im * other)
    }
}

impl Div for Complex {
    type Output = Complex;
    fn div(self, other: Complex) -> Complex {
        let d = other.re * other.re + other.im * other.im;
        Complex::new(
            (self.re * other.re + self.im * other.im) / d,
            (self.im * other.re - self.re * other.im) / d,
        )
    }
}

impl Neg for Complex {
    type Output = Complex;
    fn neg(self) -> Complex {
        Complex::new(-self.re, -self.im)
    }
}

pub fn raise(base: f64, exponent: Complex) -> Complex {
    Complex::exp(exponent * base.ln())
}
