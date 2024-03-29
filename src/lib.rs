pub use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Inp {
    pub points: Vec<Vec2>,
}

#[derive(Resource, Default)]
pub struct Outp {
    pub line: Vec<Vec2>,
}

pub fn add_point_on_left_click(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mouse: Res<Input<MouseButton>>,
    mut inp: ResMut<Inp>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = camera_query.single();
        let Some(cursor_position) = windows.single().cursor_position() else {
            return;
        };
        let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
            return;
        };
        inp.points.push(point);
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn draw(mut gizmos: Gizmos, inp: Res<Inp>, output: Res<Outp>) {
    for point in &inp.points {
        gizmos.rect_2d(*point, 0.0, 3.0 * Vec2::ONE, Color::WHITE);
    }
    gizmos.linestrip_2d(output.line.clone(), Color::CYAN);
}

