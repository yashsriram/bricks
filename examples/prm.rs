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

use lego::plan::graph::prm::PRM;
use lego::plan::graph::FringeBasedSearch;
use lego::plan::graph::VertexSearchState;
use lego::*;
use plan::planar::RectangleSpace;
use plan::StateSpace;

#[derive(Debug, Clone)]
pub struct JumpsFromStart {
    jumps: usize,
    parent: Option<usize>,
}

impl VertexSearchState<RectangleSpace> for JumpsFromStart {
    fn as_start(
        _start_vertex_idx: usize,
        _start_state: &<RectangleSpace as StateSpace>::State,
    ) -> Self {
        Self {
            jumps: 0,
            parent: None,
        }
    }

    fn as_adj(
        prev_vertex_idx: usize,
        _prev_state: &<RectangleSpace as StateSpace>::State,
        prev_search_state: &Self,
        _my_vertex_idx: usize,
        _my_state: &<RectangleSpace as StateSpace>::State,
    ) -> Self {
        Self {
            jumps: prev_search_state.jumps + 1,
            parent: Some(prev_vertex_idx),
        }
    }

    fn cost(&self) -> f32 {
        self.jumps as f32
    }
}

struct Marked;
fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let prm = PRM::with_num_samples(
        RectangleSpace {
            size: Vec2::new(2.0, 3.0),
        },
        1000,
        0.2,
    );
    let mut node = 1;
    let search: FringeBasedSearch<RectangleSpace, JumpsFromStart> =
        FringeBasedSearch::search(&prm.graph, 0, node);
    let mut path = vec![node];
    while let Some(parent) = search.vertex_search_states[&node].parent {
        path.push(parent);
        node = parent;
    }
    path = path.into_iter().rev().collect();
    println!("{:?}", path);

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
