use bevy::prelude::*;
use lego::*;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(BasePlugins)
        .add_startup_system(setup.system())
        .add_system(update.system())
        .run();
}

struct Marked;

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let prm = plan::graph::prm::PRM::with_num_samples(
        plan::planar::RectangleSpace {
            size: Vec2::new(2.0, 3.0),
        },
        1000,
        0.2,
    );
    use bevy::render::pipeline::RenderPipeline;
    commands.spawn_bundle(MeshBundle {
        mesh: meshes.add(prm.state_space.into()),
        render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
            vis::WIREFRAME_PIPELINE_HANDLE.typed(),
        )]),
        ..Default::default()
    });
    commands
        .spawn_bundle(MeshBundle {
            mesh: meshes.add(prm.graph.into()),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                vis::WIREFRAME_PIPELINE_HANDLE.typed(),
            )]),
            ..Default::default()
        })
        .insert(Marked);
}

fn update(time: Res<Time>, mut query: Query<(&mut Transform, &Marked)>) {
    // for (mut transform, _) in query.iter_mut() {
    //     transform.translation += Vec3::new(1.0, 0.0, 0.0) * time.delta_seconds();
    // }
}
