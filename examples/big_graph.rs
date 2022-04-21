use bricks::{
    pl::{
        graph::{
            path::Path,
            prm::PRM,
            search::{spanning_trees::*, CostGuidedSpanningTreeSearch},
        },
        na::{Point2, Vector2},
        spaces::*,
    },
    vz::{
        bevy::prelude::*,
        plugins::{BasePlugins, NON_FILL_PIPELINE},
    },
};
use std::time::Instant;

fn main() {
    App::build()
        .add_plugins(BasePlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let space = RectangleSpace {
        size: Vector2::new(54.0, 20.0),
    };
    let start = Instant::now();
    let mut prm = PRM::with_num_samples(space, 60000, 0.9);
    println!("{:?}", Instant::now() - start);
    println!("Number of edges = ~{:?}", prm.graph.num_edges());
    let [a, b] = prm.add([Point2::new(0.3, 0.7), Point2::new(19.5, 17.3)], 0.9);
    let searches = [
        DFSLike::try_on(&prm.graph, a, b),
        BFSLike::try_on(&prm.graph, a, b),
        UCSLike::try_on(&prm.graph, a, b),
        AStarLike::try_on(&prm.graph, a, b),
        WeightedAStarLike::<11, 10>::try_on(&prm.graph, a, b),
        W2AStarLike::try_on(&prm.graph, a, b),
    ];
    // commands.spawn_bundle(MeshBundle {
    //     mesh: meshes.add(Mesh::from(&prm.state_space)),
    //     transform: Transform::from_xyz(0.0, y, 0.0),
    //     render_pipelines: RenderPipelines::from_handles(&[NON_FILL_PIPELINE.typed()]),
    //     ..Default::default()
    // });
    for (i, search) in IntoIterator::into_iter(searches).enumerate() {
        commands.spawn_bundle(MeshBundle {
            mesh: meshes.add(Mesh::from(&Path::from(&search))),
            transform: Transform::from_xyz(0.0, (i + 1) as f32 * 22.0, 0.0),
            render_pipelines: RenderPipelines::from_handles(&[NON_FILL_PIPELINE.typed()]),
            ..Default::default()
        });
        commands.spawn_bundle(MeshBundle {
            mesh: meshes.add(Mesh::from(&search)),
            transform: Transform::from_xyz(0.0, (i + 1) as f32 * 22.0, 0.0),
            render_pipelines: RenderPipelines::from_handles(&[NON_FILL_PIPELINE.typed()]),
            ..Default::default()
        });
    }
    commands.spawn_bundle(MeshBundle {
        mesh: meshes.add(Mesh::from(&prm.graph)),
        transform: Transform::default(),
        render_pipelines: RenderPipelines::from_handles(&[NON_FILL_PIPELINE.typed()]),
        ..Default::default()
    });
}
