use bevy::prelude::*;
use bricks::plan::graph::prm::PRM;
use bricks::plan::graph::search::tree::propagations::*;
use bricks::plan::graph::search::tree::TreeSearch;
use bricks::plan::planar::RectangleSpace;
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
    let rectangular_space = RectangleSpace {
        size: Vec2::new(5.0, 3.0),
    };
    let mut prm = PRM::with_num_samples(rectangular_space, 1000, 0.2);
    let idxes = prm.add(vec![Vec2::new(0.3, 0.7), Vec2::new(2.5, 3.0)], 0.5);
    let search =
        TreeSearch::<RectangleSpace, JumpsFromStart>::try_search(&prm.graph, idxes[0], idxes[1]);
    println!("{:?}", search.path_to_stop());
    println!("{:?}", search.path_to(search.start_idx()));
    println!("{:?}", search.path_to(search.stop_idx()));
    println!("{:?}", search.path_to(50));
    println!("{:?}", search.path_to(100));
    println!("{:?}", search.path_to(search.max_idx()));

    use bevy::render::pipeline::RenderPipeline;
    commands.spawn_bundle(MeshBundle {
        mesh: meshes.add(prm.state_space.into()),
        render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
            vis::WIREFRAME_PIPELINE_HANDLE.typed(),
        )]),
        ..Default::default()
    });
    commands.spawn_bundle(MeshBundle {
        mesh: meshes.add(search.into()),
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
