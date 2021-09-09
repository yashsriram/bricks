use bevy::app::prelude::*;
use bevy::asset::{Assets, HandleUntyped};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::draw::DrawContext;
use bevy::render::draw::OutsideFrustum;
use bevy::render::mesh::{Indices, Mesh};
use bevy::render::pipeline::{
    CullMode, FrontFace, PipelineDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology,
};
use bevy::render::render_graph::{base, RenderGraph, RenderResourcesNode};
use bevy::render::renderer::RenderResourceBindings;
use bevy::render::shader::{Shader, ShaderStage, ShaderStages};
use bevy::render::RenderStage;
use bevy::utils::HashSet;

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
        // app.add_system_to_stage(
        //     RenderStage::Draw,
        //     custom_draw_render_pipelines_system.system(),
        // );
        let world = app.world_mut().cell();
        let mut pipelines = world
            .get_resource_mut::<Assets<PipelineDescriptor>>()
            .unwrap();
        let mut shaders = world.get_resource_mut::<Assets<Shader>>().unwrap();
        pipelines.set_untracked(
            WIREFRAME_PIPELINE_HANDLE,
            build_minimal_pipeline(&mut shaders),
        );
    }
}

fn build_minimal_pipeline(shaders: &mut Assets<Shader>) -> PipelineDescriptor {
    PipelineDescriptor {
        name: Some("minimal".into()),
        primitive: PrimitiveState {
            topology: PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: FrontFace::Ccw,
            cull_mode: CullMode::None,
            polygon_mode: PolygonMode::Fill,
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
    }
}

pub fn custom_draw_render_pipelines_system(
    mut draw_context: DrawContext,
    mut render_resource_bindings: ResMut<RenderResourceBindings>,
    msaa: Res<Msaa>,
    meshes: Res<Assets<Mesh>>,
    mut query: Query<
        (
            &mut Draw,
            &mut RenderPipelines,
            &Vec<Handle<Mesh>>,
            &Visible,
        ),
        Without<OutsideFrustum>,
    >,
) {
    println!("custom_draw_render_pipelines_system");
    for (mut draw, mut render_pipelines, mesh_handles, visible) in query.iter_mut() {
        println!("multi mesh with size {}", mesh_handles.len());
        if !visible.is_visible {
            continue;
        }
        let mesh_handle = mesh_handles[0].clone();

        // don't render if the mesh isn't loaded yet
        let mesh = if let Some(mesh) = meshes.get(mesh_handle) {
            mesh
        } else {
            continue;
        };
        println!("drawing mesh");

        let index_range = match mesh.indices() {
            Some(Indices::U32(indices)) => Some(0..indices.len() as u32),
            Some(Indices::U16(indices)) => Some(0..indices.len() as u32),
            None => None,
        };

        let render_pipelines = &mut *render_pipelines;
        for pipeline in render_pipelines.pipelines.iter_mut() {
            pipeline.specialization.sample_count = msaa.samples;
            if pipeline.dynamic_bindings_generation
                != render_pipelines.bindings.dynamic_bindings_generation()
            {
                pipeline.specialization.dynamic_bindings = render_pipelines
                    .bindings
                    .iter_dynamic_bindings()
                    .map(|name| name.to_string())
                    .collect::<HashSet<String>>();
                pipeline.dynamic_bindings_generation =
                    render_pipelines.bindings.dynamic_bindings_generation();
                for (handle, _) in render_pipelines.bindings.iter_assets() {
                    if let Some(bindings) = draw_context
                        .asset_render_resource_bindings
                        .get_untyped(handle)
                    {
                        for binding in bindings.iter_dynamic_bindings() {
                            pipeline
                                .specialization
                                .dynamic_bindings
                                .insert(binding.to_string());
                        }
                    }
                }
            }
        }

        for render_pipeline in render_pipelines.pipelines.iter_mut() {
            let render_resource_bindings = &mut [
                &mut render_pipelines.bindings,
                &mut render_resource_bindings,
            ];
            draw_context
                .set_pipeline(
                    &mut draw,
                    &render_pipeline.pipeline,
                    &render_pipeline.specialization,
                )
                .unwrap();
            draw_context
                .set_bind_groups_from_bindings(&mut draw, render_resource_bindings)
                .unwrap();
            draw_context
                .set_vertex_buffers_from_bindings(&mut draw, &[&render_pipelines.bindings])
                .unwrap();

            if let Some(indices) = index_range.clone() {
                draw.draw_indexed(indices, 0, 0..1);
            } else {
                draw.draw(0..mesh.count_vertices() as u32, 0..1)
            }
        }
    }
}
