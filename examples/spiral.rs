use bricks::*;

fn algo(inp: Res<Inp>, mut output: ResMut<Outp>, keyboard: Res<Input<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Space) && inp.points.len() >= 3 {
        let start = inp
            .points
            .iter()
            .reduce(|left_most, v| if v.x < left_most.x { v } else { left_most })
            .map(|e| *e)
            .unwrap_or(Vec2::ZERO);
        let mut spiral: Vec<_> = vec![start];
        let mut rem: Vec<_> = inp.points.clone();
        loop {
            let last = *spiral.last().unwrap();
            rem = rem.into_iter().filter(|v| *v != last).collect();
            if rem.len() == 0 {
                break;
            }
            let next = rem
                .clone()
                .into_iter()
                .reduce(|all_on_right, v| {
                    let cross = ((all_on_right - last).extend(0.0)).cross((v - last).extend(0.0));
                    if cross.z > 0. {
                        v
                    } else {
                        all_on_right
                    }
                })
                .map(|e| e)
                .unwrap_or(Vec2::ZERO);
            spiral.push(next);
        }
        output.line = spiral;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (500., 400.).into(),
                canvas: Some("#interactive".into()),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .init_resource::<Inp>()
        .init_resource::<Outp>()
        .add_systems(Startup, setup)
        .add_systems(Update, (draw, add_point_on_left_click, algo))
        .run();
}

