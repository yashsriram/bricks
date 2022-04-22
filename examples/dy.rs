use bricks::vz::bevy::prelude::*;
use bricks::vz::bevy::render::mesh::{Indices, Mesh};
use bricks::vz::bevy::render::pipeline::{PrimitiveTopology, RenderPipelines};
use bricks::vz::BasePlugins;
use bricks::vz::NON_FILL_PIPELINE;

fn main() {
    App::build()
        .add_plugins(BasePlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);
    let positions = vec![
        [0.0, 0.0, 0.0],
        [5.0, 0.0, 0.0],
        [5.0, 5.0, 0.0],
        [0.0, 5.0, 0.0],
    ];
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    let indices = Indices::U32(vec![0, 1, 2, 3, 0]);
    mesh.set_indices(Some(indices));
    let colors = vec![[1.0, 1.0, 0.0, 0.1]; 4];
    mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    let mesh_id = meshes.add(mesh);
    commands.spawn_bundle(MeshBundle {
        mesh: mesh_id,
        render_pipelines: RenderPipelines::from_handles(&[NON_FILL_PIPELINE.typed()]),
        draw: Default::default(),
        visible: Default::default(),
        main_pass: Default::default(),
        transform: Default::default(),
        global_transform: Default::default(),
    });
}
