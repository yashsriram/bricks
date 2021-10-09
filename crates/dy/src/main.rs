use std::fmt::Debug;

use std::ops::AddAssign;

trait State: Debug + Add<Output = Self> + AddAssign + Sized {}

#[derive(Debug)]
struct DDState {
    x: f64,
    y: f64,
    theta: f64,
}

// impl State<3> for DDState {
//     fn as_arr(&self) -> [f64; 3] {
//         [self.x, self.y, self.theta]
//     }
// }

// trait Control: Debug {}

// #[derive(Debug)]
// struct DDControl {
//     v: f64,
//     w: f64,
// }

// impl Control for DDControl {}

// trait EularianDynamics<const N: usize>
// where
//     Self: State<N> + Sized,
// {
//     fn tick(state: &self, der: Self, dt: f64) {
//         for (a, b) in state.as_arr().iter_mut().zip(der.as_arr().iter()) {
//             *a += b * dt;
//         }
//     }
// }

// impl EularianDynamics<3> for DDState {}

fn main() {
    // let mut state = DDState {
    //     x: 1.0,
    //     y: 2.0,
    //     theta: 0.1,
    // };
    // let control = DDControl { v: -1.0, w: -2.0 };
    // let der = DDState {
    //     x: control.v * state.theta.cos(),
    //     y: control.v * state.theta.sin(),
    //     theta: control.w,
    // };
    // println!("{:?}", state);
    // state.tick(&mut state, &control, 0.1);
    // println!("{:?}", state);
    // state.tick(&mut state, &control, 0.1);
    // println!("{:?}", state);
}
