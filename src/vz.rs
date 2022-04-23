pub use bevy;
use bevy::app::{AppBuilder, Plugin, PluginGroup, PluginGroupBuilder};
use bevy::diagnostic::Diagnostics;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::wgpu::{WgpuFeature, WgpuFeatures, WgpuOptions};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};

pub struct BasePlugins;

impl PluginGroup for BasePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(DefaultResourcesPlugin);
        group.add(bevy::core::CorePlugin::default());
        group.add(bevy::transform::TransformPlugin::default());
        group.add(bevy::diagnostic::DiagnosticsPlugin::default());
        group.add(bevy::input::InputPlugin::default());
        group.add(bevy::window::WindowPlugin::default());
        group.add(bevy::asset::AssetPlugin::default());
        group.add(bevy::render::RenderPlugin::default());
        group.add(bevy::winit::WinitPlugin::default());
        group.add(WGPUOptionsPlugin);
        group.add(bevy::wgpu::WgpuPlugin::default());
        group.add(MinimalRenderPlugin);
        group.add(CameraPlugin);
        group.add(FPSTitlePlugin);
    }
}

pub struct DefaultResourcesPlugin;

impl Plugin for DefaultResourcesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Msaa { samples: 1 })
            .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    }
}

pub struct WGPUOptionsPlugin;

impl Plugin for WGPUOptionsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(WgpuOptions {
            features: WgpuFeatures {
                features: vec![WgpuFeature::NonFillPolygonMode],
            },
            ..Default::default()
        });
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(FlyCameraPlugin);
        app.add_startup_system(init_camera.system());
    }
}

fn init_camera(mut commands: Commands) {
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(FlyCamera {
            key_up: KeyCode::E,
            key_down: KeyCode::Q,
            ..Default::default()
        });
}

pub struct FPSTitlePlugin;

impl Plugin for FPSTitlePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default());
        app.add_system(fps_title_plugin.system());
    }
}

fn fps_title_plugin(diagnostics: Res<Diagnostics>, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_title(format!(
        "Δt: {:.3}s",
        diagnostics
            .get(FrameTimeDiagnosticsPlugin::FRAME_TIME)
            .unwrap()
            .average()
            .unwrap_or(0.0),
    ));
}

use bevy::asset::Assets;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::pipeline::{
    CullMode, FrontFace, PipelineDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology,
};
use bevy::render::render_graph::{base, RenderGraph, RenderResourcesNode};
use bevy::render::shader::{Shader, ShaderStage, ShaderStages};

pub const VERT: &'static str = "#version 450
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
