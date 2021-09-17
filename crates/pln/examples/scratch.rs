use pln::graph::prm::PRM;
use pln::graph::search::spanning::propagations::*;
use pln::graph::search::spanning::TreeSearch;
use pln::na::{Point2, Point3, Vector2, Vector3};
use pln::spaces::*;
use vz::bevy::prelude::*;
use vz::bevy::render::mesh::VertexAttributeValues;
use vz::plugins::BasePlugins;
use vz::AsEntity;

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
    use ordered_float::OrderedFloat;
    let searches = [
        TreeSearch::try_with::<OrderedFloat<f32>, DFSLike>(&prm.graph, a, b),
        TreeSearch::try_with::<OrderedFloat<f32>, BFSLike>(&prm.graph, a, b),
        TreeSearch::try_with::<OrderedFloat<f32>, UCSLike>(&prm.graph, a, b),
        TreeSearch::try_with::<OrderedFloat<f32>, AStarLike>(&prm.graph, a, b),
        TreeSearch::try_with::<OrderedFloat<f32>, W2AStarLike<2, 2>>(&prm.graph, a, b),
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
