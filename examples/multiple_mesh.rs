use bevy::prelude::*;
use bevy::render::pipeline::RenderPipeline;
use bricks::*;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(BasePlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let mut mesh = Mesh::from(shape::Box::default());
    mesh.set_attribute(
        Mesh::ATTRIBUTE_COLOR,
        vec![[1.0, 0.0, 0.0, 1.0,]; mesh.count_vertices()],
    );
    let mesh1 = meshes.add(mesh);
    let mut mesh = Mesh::from(shape::Quad::default());
    mesh.set_attribute(
        Mesh::ATTRIBUTE_COLOR,
        vec![[1.0, 0.0, 0.0, 1.0,]; mesh.count_vertices()],
    );
    let mesh2 = meshes.add(mesh);
    let mut mesh = Mesh::from(shape::Icosphere::default());
    mesh.set_attribute(
        Mesh::ATTRIBUTE_COLOR,
        vec![[1.0, 0.0, 0.0, 0.1,]; mesh.count_vertices()],
    );
    let mesh3 = meshes.add(mesh);
    let meshes = vec![mesh1, mesh2];
    commands
        .spawn()
        .insert_bundle(MeshBundle {
            mesh: mesh3,
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                vis::WIREFRAME_PIPELINE_HANDLE.typed(),
            )]),
            ..Default::default()
        })
        .insert(meshes);
}
