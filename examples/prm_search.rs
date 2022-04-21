use bricks::{
    pl::{
        graph::{
            path::Path,
            prm::PRM,
            search::{spanning_trees::*, CostGuidedSpanningTreeSearch},
        },
        na::{Point2, Point3, Vector2, Vector3},
        spaces::*,
        StateSpace,
    },
    vz::{
        bevy::prelude::*,
        plugins::{BasePlugins, NON_FILL_PIPELINE},
    },
};

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
        size: Vector2::new(12.0, 10.0),
    };
    let mut prm = PRM::with_num_samples(space, 1500, 0.5);
    let [a, b] = prm.add([Point2::new(0.3, 0.7), Point2::new(9.5, 7.3)], 0.7);
    search_and_spawn(commands, meshes, a, b, y, prm);
}

fn cuboid(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, y: f32) {
    let space = CuboidSpace {
        size: Vector3::new(12.0, 10.0, 5.0),
    };
    let mut prm = PRM::with_num_samples(space, 2000, 1.0);
    let [a, b] = prm.add(
        [Point3::new(0.3, 0.7, 0.5), Point3::new(9.5, 7.3, 4.0)],
        1.0,
    );
    search_and_spawn(commands, meshes, a, b, y, prm);
}

fn circle(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, y: f32) {
    let space = CircleSpace { radius: 5.0 };
    let mut prm = PRM::with_num_samples(space, 2000, 0.5);
    let [a, b] = prm.add([Point2::new(-2.3, -2.7), Point2::new(2.5, 2.3)], 1.0);
    search_and_spawn(commands, meshes, a, b, y, prm);
}

fn sphere(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, y: f32) {
    let space = SphereSpace { radius: 5.0 };
    let mut prm = PRM::with_num_samples(space, 5000, 0.5);
    let [a, b] = prm.add(
        [Point3::new(-2.3, -2.7, -1.0), Point3::new(2.5, 2.3, 1.0)],
        1.0,
    );
    search_and_spawn(commands, meshes, a, b, y, prm);
}

fn search_and_spawn<SS: StateSpace>(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    a: usize,
    b: usize,
    y: f32,
    prm: PRM<SS>,
) {
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
            transform: Transform::from_xyz((i + 1) as f32 * 14.0, y, 0.0),
            render_pipelines: RenderPipelines::from_handles(&[NON_FILL_PIPELINE.typed()]),
            ..Default::default()
        });
        commands.spawn_bundle(MeshBundle {
            mesh: meshes.add(Mesh::from(&search)),
            transform: Transform::from_xyz((i + 1) as f32 * 14.0, y, 0.0),
            render_pipelines: RenderPipelines::from_handles(&[NON_FILL_PIPELINE.typed()]),
            ..Default::default()
        });
    }
    commands.spawn_bundle(MeshBundle {
        mesh: meshes.add(Mesh::from(&prm.graph)),
        transform: Transform::from_xyz(0.0, y, 0.0),
        render_pipelines: RenderPipelines::from_handles(&[NON_FILL_PIPELINE.typed()]),
        ..Default::default()
    });
}
