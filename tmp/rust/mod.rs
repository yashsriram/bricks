use nalgebra::Point2;

pub struct CircularAgent {
    center: Point2<f32>,
    radius: f32,
    speed: f32,
    color: [f32; 4],
}

impl CircularAgent {
    pub fn new(center: Point2<f32>, radius: f32, speed: f32, color: [f32; 4]) -> Self {
        Self {
            center,
            radius,
            speed,
            color,
        }
    }
}
