use bevy::prelude::*;
use bricks::plan::graph::prm::PRM;
use bricks::plan::graph::search::ripple::propagations::*;
use bricks::plan::graph::search::ripple::TreeSearch;
use bricks::plan::spaces::CircleSpace;
use bricks::*;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(BasePlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let space = CircleSpace { radius: 5.0 };
    let mut prm = PRM::with_num_samples(space, 2000, 0.5);
    let idxes = prm.add(vec![Vec2::new(-2.3, -2.7), Vec2::new(2.5, 2.3)], 1.0);

    use bevy::render::pipeline::RenderPipeline;
    commands.spawn_bundle(MeshBundle {
        mesh: meshes.add(
            TreeSearch::<CircleSpace, DFSLike>::try_search(&prm.graph, idxes[0], idxes[1]).into(),
        ),
        render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
            vis::WIREFRAME_PIPELINE_HANDLE.typed(),
        )]),
        transform: Transform::from_xyz(12.0, 0.0, 0.0),
        ..Default::default()
    });
    commands.spawn_bundle(MeshBundle {
        mesh: meshes.add(
            TreeSearch::<CircleSpace, BFSLike>::try_search(&prm.graph, idxes[0], idxes[1]).into(),
        ),
        render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
            vis::WIREFRAME_PIPELINE_HANDLE.typed(),
        )]),
        transform: Transform::from_xyz(24.0, 0.0, 0.0),
        ..Default::default()
    });
    commands.spawn_bundle(MeshBundle {
        mesh: meshes.add(
            TreeSearch::<CircleSpace, UCSLike>::try_search(&prm.graph, idxes[0], idxes[1]).into(),
        ),
        render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
            vis::WIREFRAME_PIPELINE_HANDLE.typed(),
        )]),
        transform: Transform::from_xyz(36.0, 0.0, 0.0),
        ..Default::default()
    });
    commands.spawn_bundle(MeshBundle {
        mesh: meshes.add(
            TreeSearch::<CircleSpace, AStarLike>::try_search(&prm.graph, idxes[0], idxes[1]).into(),
        ),
        render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
            vis::WIREFRAME_PIPELINE_HANDLE.typed(),
        )]),
        transform: Transform::from_xyz(48.0, 0.0, 0.0),
        ..Default::default()
    });
    commands.spawn_bundle(MeshBundle {
        mesh: meshes.add(
            TreeSearch::<CircleSpace, W2AStarLike>::try_search(&prm.graph, idxes[0], idxes[1])
                .into(),
        ),
        render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
            vis::WIREFRAME_PIPELINE_HANDLE.typed(),
        )]),
        transform: Transform::from_xyz(60.0, 0.0, 0.0),
        ..Default::default()
    });
    commands.spawn_bundle(MeshBundle {
        mesh: meshes.add(prm.state_space.into()),
        render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
            vis::WIREFRAME_PIPELINE_HANDLE.typed(),
        )]),
        ..Default::default()
    });
    commands.spawn_bundle(MeshBundle {
        mesh: meshes.add(prm.graph.into()),
        render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
            vis::WIREFRAME_PIPELINE_HANDLE.typed(),
        )]),
        ..Default::default()
    });
}
