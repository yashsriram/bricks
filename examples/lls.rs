use bricks::*;
use ndarray::prelude::*;
use ndarray_linalg::{error::LinalgError, Inverse};

#[derive(Resource)]
struct LinearLeastSquares {
    points: Vec<Vec2>,
}

impl LinearLeastSquares {
    fn fit(&self) -> Result<(f32, f32), LinalgError> {
        let A: Vec<_> = self.points.iter().map(|p| [p.x, 1.]).collect();
        let b: Vec<_> = self.points.iter().map(|p| [p.y]).collect();
        let A = arr2(&A);
        let b = arr2(&b);
        let left_term = A.t().dot(&A);
        let right_term = A.t().dot(&b);
        let x = left_term.inv()?.dot(&right_term);
        assert!(x.shape() == [2, 1]);
        Ok((x[[0, 0]], x[[1, 0]]))
    }
}

impl Default for LinearLeastSquares {
    fn default() -> Self {
        LinearLeastSquares { points: vec![] }
    }
}

#[derive(Resource, Default)]
struct MCLineForm {
    m: f32,
    c: f32,
}

impl MCLineForm {
    fn at(&self, x: f32) -> f32 {
        self.m * x + self.c
    }
}

bricks::game_2d! {
    "linear least squares",
    {
        LinearLeastSquares -> draw_lls,
        MCLineForm -> draw_line,
    }
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn draw_line(mut gizmos: Gizmos, mc_line_form: Res<MCLineForm>) {
    let x_start = -1000.;
    let x_stop = 1000.;
    gizmos.line_2d(
        [x_start, mc_line_form.at(x_start)].into(),
        [x_stop, mc_line_form.at(x_stop)].into(),
        Color::srgb(1., 1., 0.),
    );
}

fn draw_lls(mut gizmos: Gizmos, lls: Res<LinearLeastSquares>, mc: Res<MCLineForm>) {
    // Samples
    for point in lls.points.iter() {
        gizmos.circle_2d(
            Isometry2d::from_xy(point.x, point.y),
            8.,
            Color::srgba(1., 1., 1., 0.2),
        );
    }
    // Perpendiculars
    // y = m x + c
    // perpendicular has slope - 1 / m and passes through (x1, y1)
    // y1 = (-1/m) x1 + c => c = y1 + (1 / m)x1 => parametrized form (x, (x1 - x) / m + y1)
    // but at intersecion we also have y = mx + c => (x1 - x) / m + y1 = m x + c
    // x1 - x + m y1 = m * m x + c m
    // (m^2 + 1) x = x1 + m y1 - c m
    // x = (x1 + m y1 - c m) / (m^2 + 1)
    // y = m x + c again
    for p in lls.points.iter() {
        let x_perp = (p.x + mc.m * p.y - mc.c * mc.m) / (mc.m.powi(2) + 1.);
        let y_perp = mc.m * x_perp + mc.c;
        gizmos.line_2d(*p, [x_perp, y_perp].into(), Color::srgba(1., 1., 1., 1.));
    }
}

fn on_spacebar_press(mut lls: ResMut<LinearLeastSquares>, mut mc_line_form: ResMut<MCLineForm>) {
    *lls = LinearLeastSquares::default();
    *mc_line_form = MCLineForm::default();
}

fn on_mouse_click(
    In(point): In<Result<Vec2, ()>>,
    mut lls: ResMut<LinearLeastSquares>,
    mut mc_line_form: ResMut<MCLineForm>,
) {
    let Ok(point) = point else {
        return;
    };
    lls.points.push(point);
    println!("{:?}", lls.fit());
    let Ok((m, c)) = lls.fit() else {
        return;
    };
    *mc_line_form = MCLineForm { m, c };
}
