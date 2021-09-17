use pln::graph::prm::PRM;
use pln::graph::search::spanning::propagations::*;
use pln::graph::search::spanning::TreeSearch;
use pln::na::{Point2, Point3, Vector2, Vector3};
use pln::spaces::*;
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
    let funs = [rect, cuboid, circle, sphere];
    for (i, fun) in funs.iter().enumerate() {
        fun(&mut commands, &mut meshes, 20.0 * i as f32);
    }
}

fn rect(mut commands: &mut Commands, mut meshes: &mut ResMut<Assets<Mesh>>, y: f32) {
    let space = RectangleSpace {
        size: Vector2::new(12.0, 10.0),
    };
    let mut prm = PRM::with_num_samples(space, 1500, 0.5);
    let [a, b] = prm.add([Point2::new(0.3, 0.7), Point2::new(9.5, 7.3)], 0.7);
    let searches = [
        TreeSearch::try_search::<DFSLike>(&prm.graph, a, b),
        TreeSearch::try_search::<BFSLike>(&prm.graph, a, b),
        TreeSearch::try_search::<UCSLike>(&prm.graph, a, b),
        TreeSearch::try_search::<AStarLike>(&prm.graph, a, b),
        TreeSearch::try_search::<W2AStarLike>(&prm.graph, a, b),
    ];
    prm.state_space
        .spawn(&mut commands, &mut meshes, Transform::from_xyz(0.0, y, 0.0));
    for (i, search) in IntoIterator::into_iter(searches).enumerate() {
        search.spawn(
            &mut commands,
            &mut meshes,
            Transform::from_xyz((i + 1) as f32 * 14.0, y, 0.0),
        );
    }
    prm.graph
        .spawn(&mut commands, &mut meshes, Transform::from_xyz(0.0, y, 0.0));
}

fn cuboid(mut commands: &mut Commands, mut meshes: &mut ResMut<Assets<Mesh>>, y: f32) {
    let space = CuboidSpace {
        size: Vector3::new(12.0, 10.0, 5.0),
    };
    let mut prm = PRM::with_num_samples(space, 2000, 1.0);
    let [a, b] = prm.add(
        [Point3::new(0.3, 0.7, 0.5), Point3::new(9.5, 7.3, 4.0)],
        1.0,
    );
    let searches = [
        TreeSearch::try_search::<DFSLike>(&prm.graph, a, b),
        TreeSearch::try_search::<BFSLike>(&prm.graph, a, b),
        TreeSearch::try_search::<UCSLike>(&prm.graph, a, b),
        TreeSearch::try_search::<AStarLike>(&prm.graph, a, b),
        TreeSearch::try_search::<W2AStarLike>(&prm.graph, a, b),
    ];
    prm.state_space
        .spawn(&mut commands, &mut meshes, Transform::from_xyz(0.0, y, 0.0));
    for (i, search) in IntoIterator::into_iter(searches).enumerate() {
        search.spawn(
            &mut commands,
            &mut meshes,
            Transform::from_xyz((i + 1) as f32 * 14.0, y, 0.0),
        );
    }
    prm.graph
        .spawn(&mut commands, &mut meshes, Transform::from_xyz(0.0, y, 0.0));
}

fn circle(mut commands: &mut Commands, mut meshes: &mut ResMut<Assets<Mesh>>, y: f32) {
    let space = CircleSpace { radius: 5.0 };
    let mut prm = PRM::with_num_samples(space, 2000, 0.5);
    let [a, b] = prm.add([Point2::new(-2.3, -2.7), Point2::new(2.5, 2.3)], 1.0);
    let searches = [
        TreeSearch::try_search::<DFSLike>(&prm.graph, a, b),
        TreeSearch::try_search::<BFSLike>(&prm.graph, a, b),
        TreeSearch::try_search::<UCSLike>(&prm.graph, a, b),
        TreeSearch::try_search::<AStarLike>(&prm.graph, a, b),
        TreeSearch::try_search::<W2AStarLike>(&prm.graph, a, b),
    ];
    prm.state_space
        .spawn(&mut commands, &mut meshes, Transform::from_xyz(5.0, y, 0.0));
    for (i, search) in IntoIterator::into_iter(searches).enumerate() {
        search.spawn(
            &mut commands,
            &mut meshes,
            Transform::from_xyz((i + 1) as f32 * 14.0 + 5.0, y, 0.0),
        );
    }
    prm.graph
        .spawn(&mut commands, &mut meshes, Transform::from_xyz(5.0, y, 0.0));
}

fn sphere(mut commands: &mut Commands, mut meshes: &mut ResMut<Assets<Mesh>>, y: f32) {
    let space = SphereSpace { radius: 5.0 };
    let mut prm = PRM::with_num_samples(space, 5000, 0.5);
    let [a, b] = prm.add(
        [Point3::new(-2.3, -2.7, -1.0), Point3::new(2.5, 2.3, 1.0)],
        1.0,
    );
    let searches = [
        TreeSearch::try_search::<DFSLike>(&prm.graph, a, b),
        TreeSearch::try_search::<BFSLike>(&prm.graph, a, b),
        TreeSearch::try_search::<UCSLike>(&prm.graph, a, b),
        TreeSearch::try_search::<AStarLike>(&prm.graph, a, b),
        TreeSearch::try_search::<W2AStarLike>(&prm.graph, a, b),
    ];
    prm.state_space
        .spawn(&mut commands, &mut meshes, Transform::from_xyz(5.0, y, 0.0));
    for (i, search) in IntoIterator::into_iter(searches).enumerate() {
        search.spawn(
            &mut commands,
            &mut meshes,
            Transform::from_xyz((i + 1) as f32 * 14.0 + 5.0, y, 0.0),
        );
    }
    prm.graph
        .spawn(&mut commands, &mut meshes, Transform::from_xyz(5.0, y, 0.0));
}
