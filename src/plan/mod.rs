pub mod graph;
pub mod spaces;

use std::fmt::Debug;

pub trait State: Debug {
    fn dist(&self, other: &Self) -> f32;

    fn project_to_3d(&self) -> [f32; 3];
}

pub trait StateSpace: Debug {
    type State: State;

    fn sample_batch(&self, num_samples: usize) -> Vec<Self::State>;
}
