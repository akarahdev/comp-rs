use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub struct ComplexNumber {
    real: f64,
    imag: f64
}

#[derive(Debug, Copy, Clone)]
pub struct PolarComplexNumber {
    angle: f64,
    magnitude: f64
}

impl PolarComplexNumber {
    pub fn pow(self, rhs: PolarComplexNumber) -> ComplexNumber {
        // code provided by chatgpt, i am not a mathematician
        let r_pow_s = self.magnitude.powf(rhs.magnitude);
        let new_angle = rhs.magnitude * self.angle + rhs.magnitude * self.magnitude.ln() * rhs.angle;

        ComplexNumber {
            real: r_pow_s * new_angle.cos(),
            imag: r_pow_s * new_angle.sin(),
        }
    }

    pub fn root(self, rhs: PolarComplexNumber) -> ComplexNumber {
        // code provided by chatgpt, i am not a mathematician
        let root_mag = self.magnitude.powf(1.0 / rhs.magnitude);
        let root_angle = self.angle / rhs.magnitude;

        ComplexNumber {
            real: root_mag * root_angle.cos(),
            imag: root_mag * root_angle.sin(),
        }
    }
}

impl ComplexNumber {
    pub fn conjugate(&self) -> ComplexNumber {
        ComplexNumber { real: self.real, imag: self.imag }
    }

    pub fn new(real: f64, imag: f64) -> ComplexNumber {
        ComplexNumber { real, imag }
    }

    pub fn magnitude(self) -> f64 {
        f64::sqrt((self.real * self.real) + (self.imag * self.imag))
    }

    pub fn angle(self) -> f64 {
        f64::atan(self.imag / self.real)
    }

    pub fn polar(self) -> PolarComplexNumber {
        PolarComplexNumber { angle: self.angle(), magnitude: self.magnitude() }
    }
}

impl Add for ComplexNumber {
    type Output = ComplexNumber;

    fn add(self, rhs: Self) -> Self::Output {
        ComplexNumber {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag
        }
    }
}

impl Sub for ComplexNumber {
    type Output = ComplexNumber;

    fn sub(self, rhs: Self) -> Self::Output {
        ComplexNumber {
            real: self.real - rhs.real,
            imag: self.imag - rhs.imag
        }
    }
}

impl Mul for ComplexNumber {
    type Output = ComplexNumber;

    fn mul(self, rhs: Self) -> Self::Output {
        ComplexNumber {
            real: (self.real * rhs.real) - (self.imag * rhs.imag),
            imag: (self.real * rhs.imag) - (self.imag * rhs.real)
        }
    }
}

impl Div for ComplexNumber {
    type Output = ComplexNumber;

    fn div(self, rhs: Self) -> Self::Output {
        self / rhs.conjugate()
    }
}