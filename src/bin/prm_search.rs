use bevy::prelude::*;
use bricks::{
    pl::{
        path::Path,
        prm::PRM,
        search::{
            AStarLike, BFSLike, CostGuidedSpanningTreeSearch, DFSLike, UCSLike, W2AStarLike,
            WeightedAStarLike,
        },
        spaces::CuboidSpace,
    },
    vz::{BasePlugins, NON_FILL_PIPELINE},
};
use nalgebra::{Point3, Vector3};

fn main() {
    App::build()
        .add_plugins(BasePlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let space = CuboidSpace {
        size: Vector3::new(12.0, 10.0, 5.0),
    };
    let mut prm = PRM::with_num_samples(&space, 2000, 1.0);
    let [a, b] = prm.add(
        [Point3::new(0.3, 0.7, 0.5), Point3::new(9.5, 7.3, 4.0)],
        1.0,
    );
    let searches = [
        DFSLike::try_on(&prm.graph, a, b),
        BFSLike::try_on(&prm.graph, a, b),
        UCSLike::try_on(&prm.graph, a, b),
        AStarLike::try_on(&prm.graph, a, b),
        WeightedAStarLike::<11, 10>::try_on(&prm.graph, a, b),
        W2AStarLike::try_on(&prm.graph, a, b),
    ];
    commands.spawn_bundle(MeshBundle {
        mesh: meshes.add(Mesh::from(&space)),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        render_pipelines: RenderPipelines::from_handles(&[NON_FILL_PIPELINE.typed()]),
        ..Default::default()
    });
    for (i, search) in IntoIterator::into_iter(searches).enumerate() {
        commands.spawn_bundle(MeshBundle {
            mesh: meshes.add(Mesh::from(&Path::from(&search))),
            transform: Transform::from_xyz((i + 1) as f32 * 14.0, 0.0, 0.0),
            render_pipelines: RenderPipelines::from_handles(&[NON_FILL_PIPELINE.typed()]),
            ..Default::default()
        });
        commands.spawn_bundle(MeshBundle {
            mesh: meshes.add(Mesh::from(&search)),
            transform: Transform::from_xyz((i + 1) as f32 * 14.0, 0.0, 0.0),
            render_pipelines: RenderPipelines::from_handles(&[NON_FILL_PIPELINE.typed()]),
            ..Default::default()
        });
    }
    commands.spawn_bundle(MeshBundle {
        mesh: meshes.add(Mesh::from(&prm.graph)),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        render_pipelines: RenderPipelines::from_handles(&[NON_FILL_PIPELINE.typed()]),
        ..Default::default()
    });
}
