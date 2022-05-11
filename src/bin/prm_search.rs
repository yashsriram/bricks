use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
    wgpu::{WgpuFeature, WgpuFeatures, WgpuOptions},
};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use bricks::{
    pl::{
        path::Path,
        prm::PRM,
        search::{AStar, AStarWeighted2, CostGuidedWaveTreeSearch, WeightableAStar, BFS, DFS, UCS},
        spaces::CuboidSpace,
    },
    vz::{MinimalRenderPlugin, NON_FILL_PIPELINE},
};
use nalgebra::{Point3, Vector3};

fn main() {
    App::build()
        .add_plugin(bevy::core::CorePlugin::default())
        .add_plugin(bevy::transform::TransformPlugin::default())
        .add_plugin(bevy::diagnostic::DiagnosticsPlugin::default())
        .add_plugin(bevy::input::InputPlugin::default())
        .add_plugin(bevy::window::WindowPlugin::default())
        .add_plugin(bevy::asset::AssetPlugin::default())
        .add_plugin(bevy::render::RenderPlugin::default())
        .add_plugin(bevy::text::TextPlugin::default())
        .add_plugin(bevy::sprite::SpritePlugin::default())
        .add_plugin(bevy::ui::UiPlugin::default())
        .add_plugin(bevy::winit::WinitPlugin::default())
        .insert_resource(WgpuOptions {
            features: WgpuFeatures {
                features: vec![WgpuFeature::NonFillPolygonMode],
            },
            ..Default::default()
        })
        .add_plugin(bevy::wgpu::WgpuPlugin::default())
        .add_plugin(MinimalRenderPlugin)
        .add_startup_system(
            (|mut commands: Commands| {
                commands
                    .spawn_bundle(PerspectiveCameraBundle {
                        transform: Transform::from_xyz(0.0, 0.0, 10.0)
                            .looking_at(Vec3::ZERO, Vec3::Y),
                        ..Default::default()
                    })
                    .insert(FlyCamera {
                        key_up: KeyCode::E,
                        key_down: KeyCode::Q,
                        ..Default::default()
                    });
            })
            .system(),
        )
        .add_plugin(FlyCameraPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_system(
            (|diagnostics: Res<Diagnostics>, mut windows: ResMut<Windows>| {
                let window = windows.get_primary_mut().unwrap();
                window.set_title(format!(
                    "Δt: {:.3}s",
                    diagnostics
                        .get(FrameTimeDiagnosticsPlugin::FRAME_TIME)
                        .unwrap()
                        .average()
                        .unwrap_or(0.0),
                ));
            })
            .system(),
        )
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(
            (|mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>| {
                let space = CuboidSpace {
                    size: Vector3::new(12.0, 10.0, 5.0),
                };
                let mut prm = PRM::with_num_samples(&space, 2000, 1.0);
                let [a, b] = prm.add(
                    [Point3::new(0.3, 0.7, 0.5), Point3::new(9.5, 7.3, 4.0)],
                    1.0,
                );
                let searches = [
                    DFS::try_on(&prm.graph, a, b),
                    BFS::try_on(&prm.graph, a, b),
                    UCS::try_on(&prm.graph, a, b),
                    AStar::try_on(&prm.graph, a, b),
                    WeightableAStar::<11, 10>::try_on(&prm.graph, a, b),
                    AStarWeighted2::try_on(&prm.graph, a, b),
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
                        render_pipelines: RenderPipelines::from_handles(&[
                            NON_FILL_PIPELINE.typed()
                        ]),
                        ..Default::default()
                    });
                    commands.spawn_bundle(MeshBundle {
                        mesh: meshes.add(Mesh::from(&search)),
                        transform: Transform::from_xyz((i + 1) as f32 * 14.0, 0.0, 0.0),
                        render_pipelines: RenderPipelines::from_handles(&[
                            NON_FILL_PIPELINE.typed()
                        ]),
                        ..Default::default()
                    });
                }
                commands.spawn_bundle(MeshBundle {
                    mesh: meshes.add(Mesh::from(&prm.graph)),
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    render_pipelines: RenderPipelines::from_handles(&[NON_FILL_PIPELINE.typed()]),
                    ..Default::default()
                });
            })
            .system(),
        )
        .run();
}
