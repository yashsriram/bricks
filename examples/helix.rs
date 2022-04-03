use bevy::render::mesh::Indices;
use bevy::render::pipeline::PrimitiveTopology;
use bricks::vz::bevy::prelude::*;
use bricks::vz::bevy::render::mesh::VertexAttributeValues;
use bricks::vz::bevy::render::pipeline::RenderPipeline;
use bricks::vz::plugins::*;

fn main() {
    App::build()
        .add_plugins(BasePlugins)
        .add_startup_system(setup.system())
        .add_system(update.system())
        .run();
}

struct Marked;

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, vec![[1.0, 0.0, 0.0]]);
    mesh.set_attribute(
        Mesh::ATTRIBUTE_COLOR,
        vec![[1.0, 1.0, 1.0, 1.0,]; mesh.count_vertices()],
    );
    mesh.set_indices(Some(Indices::U32(vec![0])));
    let handle = meshes.add(mesh);
    commands
        .spawn_bundle(MeshBundle {
            mesh: handle.clone(),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                NON_FILL_PIPELINE.typed(),
            )]),
            ..Default::default()
        })
        .insert(Marked);
}

fn update(
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<(&Handle<Mesh>, &Marked)>,
) {
    use rand::{rngs::StdRng, Rng, SeedableRng};
    let mut rng = StdRng::from_entropy();
    for (mesh, _) in query.iter() {
        let mesh = meshes.get_mut(mesh).unwrap();
        if let Some(VertexAttributeValues::Float3(ref mut positions)) =
            mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
        {
            let time_until_now = time.seconds_since_startup() as f32;
            positions.push([time_until_now.cos(), time_until_now.sin(), time_until_now]);
        }
        if let Some(VertexAttributeValues::Float4(ref mut colors)) =
            mesh.attribute_mut(Mesh::ATTRIBUTE_COLOR)
        {
            colors.push([
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
                1.0,
            ]);
        }
        if let Some(ref mut indices) = mesh.indices_mut() {
            if let Indices::U32(ref mut indices) = indices {
                indices.push(indices.len() as u32);
                println!("{:?}", indices.len());
            }
        }
    }
}
