use bevy::prelude::shape::{Capsule, CapsuleUvProfile, Torus};
use bricks::vz::bevy::prelude::*;
use bricks::vz::bevy::render::pipeline::RenderPipeline;
use bricks::vz::*;
use rand::{rngs::StdRng, Rng, SeedableRng};

fn main() {
    App::build()
        .add_plugins(BasePlugins)
        .add_startup_system(setup.system())
        .add_system(update.system())
        .run();
}

struct Marked;

fn setup(mut commands: Commands, mut mesh_assets: ResMut<Assets<Mesh>>) {
    let mut mesh = Mesh::from(Capsule::default());
    mesh.set_attribute(
        Mesh::ATTRIBUTE_COLOR,
        vec![[0.0, 0.0, 0.0, 1.0]; mesh.count_vertices()],
    );

    commands
        .spawn_bundle(MeshBundle {
            mesh: mesh_assets.add(mesh),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                NON_FILL_PIPELINE.typed(),
            )]),
            ..Default::default()
        })
        .insert(Marked);
}

fn update(mut mesh_assets: ResMut<Assets<Mesh>>, query: Query<(&Handle<Mesh>, &Marked)>) {
    let mut rng = StdRng::from_entropy();
    let (mesh_handle, _) = query.single().unwrap();
    let mesh_ref_mut = mesh_assets.get_mut(mesh_handle).unwrap();
    *mesh_ref_mut = {
        let mut mesh = if rng.gen_bool(0.5) {
            Mesh::from(Capsule::default())
        } else {
            Mesh::from(Torus::default())
        };
        mesh.set_attribute(
            Mesh::ATTRIBUTE_COLOR,
            vec![
                [
                    rng.gen_range(0.0..1.0),
                    rng.gen_range(0.0..1.0),
                    rng.gen_range(0.0..1.0),
                    1.0,
                ];
                mesh.count_vertices()
            ],
        );
        mesh
    }
}
