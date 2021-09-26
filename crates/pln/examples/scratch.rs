use pln::graph::prm::PRM;
use pln::graph::Graph;
use pln::na::Vector2;
use pln::spaces::*;
use std::time::Instant;
use vz::bevy::prelude::*;
use vz::plugins::BasePlugins;
use vz::{AsEntity, Immediate};

fn main() {
    let space = RectangleSpace {
        size: Vector2::new(54.0, 20.0),
    };
    let start = Instant::now();
    let prm = PRM::with_num_samples(space, 1000, 0.9);
    println!("{:?}", Instant::now() - start);
    println!("Number of edges = ~{:?}", prm.graph.num_edges());
    App::build()
        .insert_resource(prm.state_space)
        .insert_resource(prm.graph)
        .add_plugins(BasePlugins)
        .add_system(despawn.system().label("anchor"))
        .add_system(spawn.system().before("anchor"))
        .run();
}

fn despawn(mut commands: Commands, query: Query<Entity, With<Immediate>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    state_space: Res<RectangleSpace>,
    graph: Res<Graph<RectangleSpace>>,
) {
    state_space.spawn(&mut commands, &mut meshes, Transform::default());
    graph.spawn(&mut commands, &mut meshes, Transform::default());
}
