use bevy::app::prelude::*;
use bevy::asset::{Assets, HandleUntyped};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::pipeline::{
    CullMode, FrontFace, PipelineDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology,
};
use bevy::render::render_graph::{base, RenderGraph, RenderResourcesNode};
use bevy::render::shader::{Shader, ShaderStage, ShaderStages};

pub const WIREFRAME_PIPELINE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 0xa95223f8a53b6f66);

#[derive(Debug, Default)]
pub struct MinimalRenderPlugin;

impl Plugin for MinimalRenderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        {
            let world = app.world_mut();
            let mut graph = world.get_resource_mut::<RenderGraph>().unwrap();
            graph.add_system_node(
                "transform",
                RenderResourcesNode::<GlobalTransform>::new(true),
            );
            graph
                .add_node_edge("transform", base::node::MAIN_PASS)
                .unwrap();
        }
        let world = app.world_mut().cell();
        let mut shaders = world.get_resource_mut::<Assets<Shader>>().unwrap();
        let mut pipelines = world
            .get_resource_mut::<Assets<PipelineDescriptor>>()
            .unwrap();
        pipelines.set_untracked(
            WIREFRAME_PIPELINE_HANDLE,
            build_wireframe_pipeline(&mut shaders),
        );
    }
}

fn build_wireframe_pipeline(shaders: &mut Assets<Shader>) -> PipelineDescriptor {
    PipelineDescriptor {
        name: Some("wireframe".into()),
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
                include_str!("wireframe.vert"),
            )),
            fragment: Some(shaders.add(Shader::from_glsl(
                ShaderStage::Fragment,
                include_str!("wireframe.frag"),
            ))),
        })
    }
}
