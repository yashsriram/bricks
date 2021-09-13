use bevy::app::prelude::*;
use bevy::asset::{Assets, HandleUntyped};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::pipeline::{
    CullMode, FrontFace, PipelineDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology,
};
use bevy::render::render_graph::{base, RenderGraph, RenderResourcesNode};
use bevy::render::shader::{Shader, ShaderStage, ShaderStages};

pub const NON_FILL_PIPELINE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 0xa95223f8a53b6f66);

#[derive(Debug, Default)]
pub struct MinimalRenderPlugin;

impl Plugin for MinimalRenderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let world = app.world_mut().cell();
        let mut graph = world.get_resource_mut::<RenderGraph>().unwrap();
        graph.add_system_node(
            "transform",
            RenderResourcesNode::<GlobalTransform>::new(true),
        );
        graph
            .add_node_edge("transform", base::node::MAIN_PASS)
            .unwrap();
        let mut pipelines = world
            .get_resource_mut::<Assets<PipelineDescriptor>>()
            .unwrap();
        let mut shaders = world.get_resource_mut::<Assets<Shader>>().unwrap();
        pipelines.set_untracked(
            NON_FILL_PIPELINE,
            PipelineDescriptor {
                name: Some("NON_FILL_PIPELINE".into()),
                primitive: PrimitiveState {
                    topology: PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: FrontFace::Ccw,
                    cull_mode: CullMode::None,
                    polygon_mode: PolygonMode::Line,
                },
                ..PipelineDescriptor::default_config(ShaderStages {
                    vertex: shaders.add(Shader::from_glsl(
                        ShaderStage::Vertex,
                        include_str!("minimal.vert"),
                    )),
                    fragment: Some(shaders.add(Shader::from_glsl(
                        ShaderStage::Fragment,
                        include_str!("minimal.frag"),
                    ))),
                })
            },
        );
    }
}
