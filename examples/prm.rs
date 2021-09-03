use bevy::prelude::*;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(BasePlugins)
        .add_startup_system(setup.system())
        .add_system(update.system())
        .run();
}

use bricks::plan::graph::prm::PRM;
use bricks::plan::graph::search::ripple::RippleSearch;
use bricks::plan::graph::search::ripple::VertexSearchState;
use bricks::plan::graph::search::ripple::{CostPriority, Propagate};
use bricks::*;
use plan::planar::RectangleSpace;
use plan::StateSpace;

#[derive(Debug, Clone)]
pub struct JumpsFromStart {
    jumps: usize,
}

impl CostPriority for JumpsFromStart {
    fn cost(&self) -> f32 {
        self.jumps as f32
    }
}

impl Propagate<RectangleSpace> for JumpsFromStart {
    fn as_start(
        _start_vertex_idx: usize,
        _start_state: &<RectangleSpace as StateSpace>::State,
    ) -> Self {
        Self { jumps: 0 }
    }

    fn as_adj(
        _prev_vertex_idx: usize,
        _prev_state: &<RectangleSpace as StateSpace>::State,
        _my_vertex_idx: usize,
        _my_state: &<RectangleSpace as StateSpace>::State,
        prev_search_state: &Self,
    ) -> Self {
        Self {
            jumps: prev_search_state.jumps + 1,
        }
    }
}

impl VertexSearchState<RectangleSpace> for JumpsFromStart {}

struct Marked;
fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let prm = PRM::with_num_samples(
        RectangleSpace {
            size: Vec2::new(2.0, 3.0),
        },
        1000,
        0.2,
    );
    let search = RippleSearch::<RectangleSpace, JumpsFromStart>::try_search(&prm.graph, 27, 83);
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
    commands
        .spawn_bundle(MeshBundle {
            mesh: meshes.add(prm.graph.into()),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                vis::WIREFRAME_PIPELINE_HANDLE.typed(),
            )]),
            ..Default::default()
        })
        .insert(Marked);
}

fn update(time: Res<Time>, mut query: Query<(&mut Transform, &Marked)>) {
    // for (mut transform, _) in query.iter_mut() {
    //     transform.translation += Vec3::new(1.0, 0.0, 0.0) * time.delta_seconds();
    // }
}
