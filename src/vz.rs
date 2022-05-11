use bevy::app::{AppBuilder, Plugin};
use bevy::asset::Assets;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::pipeline::{
    CullMode, FrontFace, PipelineDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology,
};
use bevy::render::render_graph::{base, RenderGraph, RenderResourcesNode};
use bevy::render::shader::{Shader, ShaderStage, ShaderStages};

const VERT: &'static str = "#version 450
layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec4 Vertex_Color;
layout(location = 1) out vec4 v_Color;
layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};
/* layout(set = 2, binding = 1) uniform Sprite { */
/*     vec2 size; */
/*     uint flip; */
/* }; */
void main() {
    v_Color = Vertex_Color;
    vec3 v_Position = (Model * vec4(Vertex_Position, 1.0)).xyz;
    gl_Position = ViewProj * vec4(v_Position, 1.0);
}
";
pub const FRAG: &'static str = "#version 450
layout(location = 1) in vec4 v_Color;
layout(location = 0) out vec4 o_Target;
void main() {
    o_Target = v_Color;
}
";

pub const NON_FILL_PIPELINE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 3314895629064977204);
pub const FILL_PIPELINE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 4208000235735356853);

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
                    vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERT)),
                    fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAG))),
                })
            },
        );
        pipelines.set_untracked(
            FILL_PIPELINE,
            PipelineDescriptor {
                name: Some("FILL_PIPELINE".into()),
                primitive: PrimitiveState {
                    topology: PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: FrontFace::Ccw,
                    cull_mode: CullMode::None,
                    polygon_mode: PolygonMode::Fill,
                },
                ..PipelineDescriptor::default_config(ShaderStages {
                    vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERT)),
                    fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAG))),
                })
            },
        );
    }
}
