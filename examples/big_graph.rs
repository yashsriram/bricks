use bevy::prelude::*;
use bricks::na::Point2;
use bricks::plan::graph::prm::PRM;
use bricks::plan::graph::search::spanning::propagations::*;
use bricks::plan::graph::search::spanning::TreeSearch;
use bricks::plan::spaces::*;
use bricks::*;
use std::time::Instant;

fn main() {
    App::build()
        .add_plugins(BasePlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let space = RectangleSpace {
        size: Vec2::new(54.0, 20.0),
    };
    let start = Instant::now();
    let mut prm = PRM::with_num_samples(space, 30000, 0.9);
    println!("{:?}", Instant::now() - start);
    let idxes = prm.add(vec![Point2::new(0.3, 0.7), Point2::new(19.5, 17.3)], 0.9);
    println!("Number of edges = ~{:?}", prm.graph.num_edges());
    let search_meshes = vec![
        Mesh::from(TreeSearch::<_, DFSLike>::try_search(
            &prm.graph, idxes[0], idxes[1],
        )),
        Mesh::from(TreeSearch::<_, BFSLike>::try_search(
            &prm.graph, idxes[0], idxes[1],
        )),
        Mesh::from(TreeSearch::<_, UCSLike>::try_search(
            &prm.graph, idxes[0], idxes[1],
        )),
        Mesh::from(TreeSearch::<_, AStarLike>::try_search(
            &prm.graph, idxes[0], idxes[1],
        )),
        Mesh::from(TreeSearch::<_, W2AStarLike>::try_search(
            &prm.graph, idxes[0], idxes[1],
        )),
    ];

    let handles = vec![vis::WIREFRAME_PIPELINE_HANDLE.typed()];
    commands.spawn_bundle(MeshBundle {
        mesh: meshes.add(prm.state_space.into()),
        render_pipelines: RenderPipelines::from_handles(handles.iter()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
    commands.spawn_bundle(MeshBundle {
        mesh: meshes.add(prm.graph.into()),
        render_pipelines: RenderPipelines::from_handles(handles.iter()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
    for (i, mesh) in search_meshes.into_iter().enumerate() {
        commands.spawn_bundle(MeshBundle {
            mesh: meshes.add(mesh),
            render_pipelines: RenderPipelines::from_handles(handles.iter()),
            transform: Transform::from_xyz(0.0, 22.0 * (i + 1) as f32, 0.0),
            ..Default::default()
        });
    }
}
