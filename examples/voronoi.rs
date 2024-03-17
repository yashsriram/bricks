use bevy::{
    app::AppExit, prelude::*, render::view::screenshot::ScreenshotManager,
    sprite::MaterialMesh2dBundle, window::PrimaryWindow,
};
use rand::prelude::*;

fn main() {
    #[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
    enum AlgoState {
        #[default]
        ThreePoints,
        Triangle,
        CircumCircle,
    }
    let mut rng = rand::thread_rng();
    let a = Vec2::new(rng.gen_range(-100.0..100.0), rng.gen_range(-100.0..100.0));
    let b = Vec2::new(rng.gen_range(-100.0..100.0), rng.gen_range(-100.0..100.0));
    let c = Vec2::new(rng.gen_range(-100.0..100.0), rng.gen_range(-100.0..100.0));
    struct PointVecLine {
        p: Vec2,
        v: Vec2,
    }
    impl PointVecLine {
        fn bisect(p1: Vec2, p2: Vec2) -> Self {
            PointVecLine {
                p: (p1 + p2) / 2.0,
                v: (p1 - p2).perp(),
            }
        }
        fn xn_with(&self, other: &Self) -> Option<Vec2> {
            let existence_matrix =
                Mat2::from_cols_array(&[self.v.x, self.v.y, -1.0 * other.v.x, -1.0 * other.v.y]);
            let x_matrix = Mat2::from_cols_array(&[
                self.p.x - other.p.x,
                self.p.y - other.p.y,
                -1.0 * other.v.x,
                -1.0 * other.v.y,
            ]);
            if existence_matrix.determinant() == 0.0 {
                return None;
            } else {
                let t_1 = -1.0 * x_matrix.determinant() / existence_matrix.determinant();
                let intersection = self.p + self.v * t_1;
                Some(intersection)
            }
        }
    }
    let ab_perp = PointVecLine::bisect(a, b);
    let bc_perp = PointVecLine::bisect(b, c);
    let xn = ab_perp.xn_with(&bc_perp).unwrap();
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AlgoState>()
        .add_systems(Startup, move |mut commands: Commands| {
            commands.spawn(Camera2dBundle::default());
        })
        .add_systems(
            OnEnter(AlgoState::ThreePoints),
            move |mut next_state: ResMut<NextState<AlgoState>>,
                  mut commands: Commands,
                  mut meshes: ResMut<Assets<Mesh>>,
                  mut materials: ResMut<Assets<ColorMaterial>>,
                  mut screenshot_manager: ResMut<ScreenshotManager>,
                  main_window: Query<Entity, With<PrimaryWindow>>| {
                println!("{:?}", AlgoState::ThreePoints);
                commands.spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(10.).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::WHITE)),
                    ..default()
                });
                screenshot_manager
                    .save_screenshot_to_disk(main_window.single(), "voronoi-1.png")
                    .unwrap();
                next_state.set(AlgoState::Triangle);
            },
        )
        .add_systems(
            OnEnter(AlgoState::Triangle),
            move |mut commands: Commands,
                  mut meshes: ResMut<Assets<Mesh>>,
                  mut materials: ResMut<Assets<ColorMaterial>>,
                  mut screenshot_manager: ResMut<ScreenshotManager>,
                  main_window: Query<Entity, With<PrimaryWindow>>| {
                println!("{:?}", AlgoState::Triangle);
                commands.spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(5.).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::GREEN)),
                    ..default()
                });
                screenshot_manager
                    .save_screenshot_to_disk(main_window.single(), "voronoi-2.png")
                    .unwrap();
            },
        )
        // .add_systems(
        //     Update,
        //     move |mut counter: Local<usize>,
        //           mut screenshot_manager: ResMut<ScreenshotManager>,
        //           main_window: Query<Entity, With<PrimaryWindow>>,
        //           mut exit: EventWriter<AppExit>,
        //           mut giz: Gizmos| {
        //         match *counter {
        //             0 => {
        //                 giz.rect_2d(a, 0.0, Vec2::ONE * 5.0, Color::WHITE);
        //                 giz.rect_2d(b, 0.0, Vec2::ONE * 5.0, Color::WHITE);
        //                 giz.rect_2d(c, 0.0, Vec2::ONE * 5.0, Color::WHITE);
        //                 giz.linestrip_2d([a, b, c, a], Color::WHITE);
        //             }
        //             1 => {
        //                 giz.rect_2d(a, 0.0, Vec2::ONE * 5.0, Color::WHITE);
        //                 giz.rect_2d(b, 0.0, Vec2::ONE * 5.0, Color::WHITE);
        //                 giz.rect_2d(c, 0.0, Vec2::ONE * 5.0, Color::WHITE);
        //                 giz.linestrip_2d([a, b, c, a], Color::WHITE);
        //                 screenshot_manager
        //                     .save_screenshot_to_disk(main_window.single(), "voronoi-1.png")
        //                     .unwrap();
        //             }
        //             2 => {
        //                 giz.rect_2d(a, 0.0, Vec2::ONE * 5.0, Color::WHITE);
        //                 giz.rect_2d(b, 0.0, Vec2::ONE * 5.0, Color::WHITE);
        //                 giz.rect_2d(c, 0.0, Vec2::ONE * 5.0, Color::WHITE);
        //                 giz.linestrip_2d([a, b, c, a], Color::WHITE);
        //                 giz.rect_2d(xn, 0.0, Vec2::ONE * 10.0, Color::CYAN);
        //                 giz.line_2d(ab_perp.p, xn, Color::CYAN);
        //                 giz.line_2d(bc_perp.p, xn, Color::CYAN);
        //                 giz.line_2d(c, xn, Color::CYAN);
        //                 giz.circle_2d(xn, (xn - a).length(), Color::RED);
        //             }
        //             3 => {
        //                 giz.rect_2d(a, 0.0, Vec2::ONE * 5.0, Color::WHITE);
        //                 giz.rect_2d(b, 0.0, Vec2::ONE * 5.0, Color::WHITE);
        //                 giz.rect_2d(c, 0.0, Vec2::ONE * 5.0, Color::WHITE);
        //                 giz.linestrip_2d([a, b, c, a], Color::WHITE);
        //                 giz.rect_2d(xn, 0.0, Vec2::ONE * 10.0, Color::CYAN);
        //                 giz.line_2d(ab_perp.p, xn, Color::CYAN);
        //                 giz.line_2d(bc_perp.p, xn, Color::CYAN);
        //                 giz.line_2d(c, xn, Color::CYAN);
        //                 giz.circle_2d(xn, (xn - a).length(), Color::RED);
        //                 screenshot_manager
        //                     .save_screenshot_to_disk(main_window.single(), "voronoi-2.png")
        //                     .unwrap();
        //             }
        //             50 => {
        //                 exit.send(AppExit);
        //             }
        //             _ => {}
        //         }
        //         *counter += 1;
        //     },
        // )
        .run();
}
