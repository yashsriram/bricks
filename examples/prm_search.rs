use bevy::prelude::*;
use bricks::na::{Point2, Point3};
use bricks::plan::graph::prm::PRM;
use bricks::plan::graph::search::spanning::propagations::*;
use bricks::plan::graph::search::spanning::TreeSearch;
use bricks::plan::spaces::*;
use bricks::*;

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

fn rect(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, y: f32) {
    let space = RectangleSpace {
        size: Vec2::new(12.0, 10.0),
    };
    let mut prm = PRM::with_num_samples(space, 1500, 0.5);
    let idxes = prm.add(vec![Point2::new(0.3, 0.7), Point2::new(9.5, 7.3)], 0.7);
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

    let handles = vec![vis::NON_FILL_PIPELINE.typed()];
    commands.spawn_bundle(MeshBundle {
        mesh: meshes.add(prm.state_space.into()),
        render_pipelines: RenderPipelines::from_handles(handles.iter()),
        transform: Transform::from_xyz(0.0, y, 0.0),
        ..Default::default()
    });
    commands.spawn_bundle(MeshBundle {
        mesh: meshes.add(prm.graph.into()),
        render_pipelines: RenderPipelines::from_handles(handles.iter()),
        transform: Transform::from_xyz(0.0, y, 0.0),
        ..Default::default()
    });
    for (i, mesh) in search_meshes.into_iter().enumerate() {
        commands.spawn_bundle(MeshBundle {
            mesh: meshes.add(mesh),
            render_pipelines: RenderPipelines::from_handles(handles.iter()),
            transform: Transform::from_xyz(14.0 * (i + 1) as f32, y, 0.0),
            ..Default::default()
        });
    }
}

fn cuboid(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, y: f32) {
    let space = CuboidSpace {
        size: Vec3::new(12.0, 10.0, 5.0),
    };
    let mut prm = PRM::with_num_samples(space, 2000, 1.0);
    let idxes = prm.add(
        vec![Point3::new(0.3, 0.7, 0.5), Point3::new(9.5, 7.3, 4.0)],
        1.0,
    );
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

    let handles = vec![vis::NON_FILL_PIPELINE.typed()];
    commands.spawn_bundle(MeshBundle {
        mesh: meshes.add(prm.state_space.into()),
        render_pipelines: RenderPipelines::from_handles(handles.iter()),
        transform: Transform::from_xyz(0.0, y, 0.0),
        ..Default::default()
    });
    commands.spawn_bundle(MeshBundle {
        mesh: meshes.add(prm.graph.into()),
        render_pipelines: RenderPipelines::from_handles(handles.iter()),
        transform: Transform::from_xyz(0.0, y, 0.0),
        ..Default::default()
    });
    for (i, mesh) in search_meshes.into_iter().enumerate() {
        commands.spawn_bundle(MeshBundle {
            mesh: meshes.add(mesh),
            render_pipelines: RenderPipelines::from_handles(handles.iter()),
            transform: Transform::from_xyz(14.0 * (i + 1) as f32, y, 0.0),
            ..Default::default()
        });
    }
}

fn circle(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, y: f32) {
    let space = CircleSpace { radius: 5.0 };
    let mut prm = PRM::with_num_samples(space, 2000, 0.5);
    let idxes = prm.add(vec![Point2::new(-2.3, -2.7), Point2::new(2.5, 2.3)], 1.0);
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

    let handles = vec![vis::NON_FILL_PIPELINE.typed()];
    commands.spawn_bundle(MeshBundle {
        mesh: meshes.add(prm.state_space.into()),
        render_pipelines: RenderPipelines::from_handles(handles.iter()),
        transform: Transform::from_xyz(5.0, y, 0.0),
        ..Default::default()
    });
    commands.spawn_bundle(MeshBundle {
        mesh: meshes.add(prm.graph.into()),
        render_pipelines: RenderPipelines::from_handles(handles.iter()),
        transform: Transform::from_xyz(5.0, y, 0.0),
        ..Default::default()
    });
    for (i, mesh) in search_meshes.into_iter().enumerate() {
        commands.spawn_bundle(MeshBundle {
            mesh: meshes.add(mesh),
            render_pipelines: RenderPipelines::from_handles(handles.iter()),
            transform: Transform::from_xyz(5.0 + (14.0 * (i + 1) as f32), y, 0.0),
            ..Default::default()
        });
    }
}

fn sphere(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, y: f32) {
    let space = SphereSpace { radius: 5.0 };
    let mut prm = PRM::with_num_samples(space, 5000, 0.5);
    let idxes = prm.add(
        vec![Point3::new(-2.3, -2.7, -1.0), Point3::new(2.5, 2.3, 1.0)],
        1.0,
    );
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

    let handles = vec![vis::NON_FILL_PIPELINE.typed()];
    commands.spawn_bundle(MeshBundle {
        mesh: meshes.add(prm.state_space.into()),
        render_pipelines: RenderPipelines::from_handles(handles.iter()),
        transform: Transform::from_xyz(5.0, y, 0.0),
        ..Default::default()
    });
    commands.spawn_bundle(MeshBundle {
        mesh: meshes.add(prm.graph.into()),
        render_pipelines: RenderPipelines::from_handles(handles.iter()),
        transform: Transform::from_xyz(5.0, y, 0.0),
        ..Default::default()
    });
    for (i, mesh) in search_meshes.into_iter().enumerate() {
        commands.spawn_bundle(MeshBundle {
            mesh: meshes.add(mesh),
            render_pipelines: RenderPipelines::from_handles(handles.iter()),
            transform: Transform::from_xyz(5.0 + (14.0 * (i + 1) as f32), y, 0.0),
            ..Default::default()
        });
    }
}
