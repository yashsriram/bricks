use bricks::pl::graph::path::Path;
use bricks::pl::graph::prm::PRM;
use bricks::pl::graph::search::spanning_trees::*;
use bricks::pl::graph::search::CostGuidedSpanningTreeSearch;
use bricks::pl::na::{Point2, Vector2};
use bricks::pl::spaces::*;
use std::time::Instant;
use bricks::vz::bevy::prelude::*;
use bricks::vz::plugins::BasePlugins;
use bricks::vz::AsEntity;

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
    prm.state_space
        .spawn(&mut commands, &mut meshes, Transform::default());
    for (i, search) in IntoIterator::into_iter(searches).enumerate() {
        Path::from(&search).spawn(
            &mut commands,
            &mut meshes,
            Transform::from_xyz(0.0, (i + 1) as f32 * 22.0, 0.0),
        );
        search.spawn(
            &mut commands,
            &mut meshes,
            Transform::from_xyz(0.0, (i + 1) as f32 * 22.0, 0.0),
        );
    }
    prm.graph
        .spawn(&mut commands, &mut meshes, Transform::default());
}
