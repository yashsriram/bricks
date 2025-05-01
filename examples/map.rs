use bricks::*;

#[derive(Resource)]
struct MAP {
    mean: f32,
    std_dev: f32,
    x_start: f32,
    x_stop: f32,
}

impl MAP {
    const X_SCALE: f32 = 1000.;
    const Y_SCALE: f32 = 40.;

    fn at(&self, x: f32) -> Option<f32> {
        if self.std_dev < 1e-6f32 {
            return None;
        }
        // Compute N(x)
        use std::f32::consts::PI;
        let x = x / Self::X_SCALE;
        let n_x = 1. / ((2. * PI).sqrt() * self.std_dev)
            * (-0.5 * ((x - self.mean) / self.std_dev).powi(2)).exp();
        let n_x = n_x * Self::Y_SCALE;
        return Some(n_x);
    }

    fn update(&mut self, msmts: &Measurements) {
        let right_term = msmts
            .0
            .iter()
            // Cap min variance to avoid div by zero
            .map(|m| (m.x / Self::X_SCALE) / (m.y.max(Self::Y_SCALE) / Self::Y_SCALE).powi(2))
            .sum::<f32>()
            + self.mean / self.std_dev.powi(2);
        let left_term = msmts
            .0
            .iter()
            // Cap min variance to avoid div by zero
            .map(|m| 1. / (m.y.max(Self::Y_SCALE) / Self::Y_SCALE).powi(2))
            .sum::<f32>()
            + 1. / self.std_dev.powi(2);
        self.mean = right_term / left_term;
        self.std_dev = (1. / left_term).sqrt();
    }
}

impl Default for MAP {
    fn default() -> Self {
        MAP {
            mean: 0.,
            std_dev: 0.,
            x_start: -500.,
            x_stop: 500.,
        }
    }
}

#[derive(Resource, Default)]
struct Measurements(Vec<Vec2>);

bricks::game_2d! {
    "maximum a-posteriori estimate",
    {
        MAP -> draw_estimate,
        Measurements -> draw_measurement,
    }
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn draw_estimate(mut gizmos: Gizmos, map: Res<MAP>) {
    // X axis
    gizmos.line_2d(
        [map.x_start, 0.].into(),
        [map.x_stop, 0.].into(),
        Color::WHITE,
    );
    // Curve
    let points = (0..4000)
        .map(|idx| idx as f32 / 4000.0 * (map.x_stop - map.x_start) + map.x_start)
        .map(|x| Vec2::new(x, map.at(x).unwrap_or_default()));
    gizmos.linestrip_2d(points, Color::srgb(0., 1., 0.));
    // Mean
    gizmos.circle_2d(
        Isometry2d::from_xy(map.mean * MAP::X_SCALE, 0.),
        10.,
        Color::srgb(1., 0., 0.),
    );
}

fn draw_measurement(mut gizmos: Gizmos, measurements: Res<Measurements>) {
    for msmt in measurements.0.iter() {
        gizmos.circle_2d(
            Isometry2d::from_xy(msmt.x, msmt.y),
            10. + msmt.y / MAP::Y_SCALE * 2.,
            Color::srgba(1., 1., 0., 0.1 + MAP::Y_SCALE / msmt.y),
        );
    }
}

fn on_spacebar_press(mut msmts: ResMut<Measurements>, mut map: ResMut<MAP>) {
    msmts.0.clear();
    *map = MAP::default();
}

fn on_mouse_click(
    In(point): In<Result<Vec2, ()>>,
    mut msmts: ResMut<Measurements>,
    mut map: ResMut<MAP>,
) {
    let Ok(point) = point else {
        return;
    };
    if point.y > 0. {
        msmts.0.push(point);
        map.update(&msmts);
    }
}
