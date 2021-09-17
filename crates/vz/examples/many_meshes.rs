use vz::bevy::prelude::*;
use vz::plugins::*;

fn main() {
    App::build()
        .add_plugins(BasePlugins)
        .add_startup_system(setup.system())
        .add_system(update.system())
        .run();
}

struct Marked;

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    use rand::{rngs::StdRng, Rng, SeedableRng};
    let mut rng = StdRng::from_entropy();
    let mut mesh = Mesh::from(shape::Capsule::default());
    mesh.set_attribute(
        Mesh::ATTRIBUTE_COLOR,
        vec![
            [
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
                0.1,
            ];
            mesh.count_vertices()
        ],
    );
    let mesh = meshes.add(mesh);
    use vz::bevy::render::pipeline::RenderPipeline;
    // 10_000 should give ~30FPS
    for _ in 0..10_000 {
        commands
            .spawn_bundle(MeshBundle {
                mesh: mesh.clone(),
                render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                    NON_FILL_PIPELINE.typed(),
                )]),
                visible: Visible {
                    is_visible: true,
                    is_transparent: false,
                },
                transform: Transform::from_xyz(
                    rng.gen_range(-50.0..50.0),
                    rng.gen_range(-50.0..50.0),
                    0.0,
                ),
                ..Default::default()
            })
            .insert(Marked);
    }
}

fn update(time: Res<Time>, mut query: Query<(&mut Transform, &Marked)>) {
    for (mut transform, _) in query.iter_mut() {
        transform.translation += Vec3::new(1.0, 0.0, 0.0) * time.delta_seconds();
    }
}
