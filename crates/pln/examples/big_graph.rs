use pln::graph::prm::PRM;
use pln::graph::search::spanning::propagations::*;
use pln::graph::search::spanning::TreeSearch;
use pln::na::{Point2, Vector2};
use pln::spaces::*;
use std::time::Instant;
use vz::bevy::prelude::*;
use vz::plugins::BasePlugins;
use vz::AsEntity;

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
        TreeSearch::try_search::<DFSLike>(&prm.graph, a, b),
        TreeSearch::try_search::<BFSLike>(&prm.graph, a, b),
        TreeSearch::try_search::<UCSLike>(&prm.graph, a, b),
        TreeSearch::try_search::<AStarLike>(&prm.graph, a, b),
        TreeSearch::try_search::<W2AStarLike>(&prm.graph, a, b),
    ];
    prm.state_space
        .spawn(&mut commands, &mut meshes, Transform::default());
    for (i, search) in IntoIterator::into_iter(searches).enumerate() {
        search.spawn(
            &mut commands,
            &mut meshes,
            Transform::from_xyz(0.0, (i + 1) as f32 * 22.0, 0.0),
        );
    }
    prm.graph
        .spawn(&mut commands, &mut meshes, Transform::default());
}
