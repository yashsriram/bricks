use bevy::prelude::*;
use bevy::render::mesh::VertexAttributeValues;
use bricks::na::{Point2, Point3, Vector2, Vector3};
use bricks::plan::graph::prm::PRM;
use bricks::plan::graph::search::spanning::propagations::*;
use bricks::plan::graph::search::spanning::TreeSearch;
use bricks::plan::spaces::*;
use bricks::vis::AsEntity;
use bricks::*;

fn main() {
    App::build()
        .add_plugins(BasePlugins)
        .add_startup_system(setup.system())
        // .add_system(foo.system())
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let space = RectangleSpace {
        size: Vector2::new(7.0, 2.0),
    };
    let mut prm = PRM::with_num_samples(space, 1500, 0.2);
    let [a, b] = prm.add([Point2::new(0.3, 0.7), Point2::new(6.5, 1.3)], 0.3);
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
            Transform::from_xyz(0.0, (i + 1) as f32 * 3.0, 0.0),
        );
    }
    prm.graph
        .spawn(&mut commands, &mut meshes, Transform::default());
}

// fn foo(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     time: Res<Time>,
//     q: Query<&Handle<Mesh>>,
// ) {
//     for handle in q.iter() {
//         let mesh = meshes.get_mut(handle).unwrap();
//         if let VertexAttributeValues::Float3(positions) =
//             mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION).unwrap()
//         {
//             positions[0] = [time.seconds_since_startup() as f32, 0.0, 0.0];
//             println!("{:?}", positions);
//         }
//     }
// }
