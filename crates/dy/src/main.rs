use nalgebra::SVector;
use std::ops::AddAssign;
use std::ops::Mul;

trait Dynamics: AddAssign<Self::Delta> {
    type Control;
    type Derivative: Mul<Self::TimeInterval, Output = Self::Delta>;
    type TimeInterval;
    type Delta;

    fn der_given_ctrl(&self, c: Self::Control) -> Self::Derivative;

    fn eular_update(&mut self, c: Self::Control, dt: Self::TimeInterval) {
        *self += self.der_given_ctrl(c) * dt;
    }
}

#[derive(Debug, Copy, Clone)]
struct RobotState {
    a: f64,
    b: f64,
}

#[derive(Debug, Copy, Clone)]
struct RobotDerivative {
    a: f64,
    b: f64,
}

#[derive(Debug, Copy, Clone)]
struct RobotDelta {
    a: f64,
    b: f64,
}

type RobotTimeInterval = f64;

impl Mul<RobotTimeInterval> for RobotDerivative {
    type Output = RobotDelta;

    fn mul(self, dt: RobotTimeInterval) -> RobotDelta {
        RobotDelta {
            a: self.a * dt,
            b: self.b * dt,
        }
    }
}

impl AddAssign<RobotDelta> for RobotState {
    fn add_assign(&mut self, other: RobotDelta) {
        *self = Self {
            a: self.a + other.a,
            b: self.b + other.b,
        };
    }
}

#[derive(Debug, Copy, Clone)]
struct RobotControl(f64);

impl Dynamics for RobotState {
    type Control = RobotControl;
    type Derivative = RobotDerivative;
    type TimeInterval = RobotTimeInterval;
    type Delta = RobotDelta;

    fn der_given_ctrl(&self, c: RobotControl) -> RobotDerivative {
        RobotDerivative {
            a: c.0.cos(),
            b: -c.0.sin(),
        }
    }
}

fn main() {
    let mut s = RobotState { a: 0.0, b: 0.0 };
    let c = RobotControl(3.0);

    println!("{:?}", s);

    let dt = 0.1;
    s.eular_update(c, dt);

    println!("{:?}", s);
}
