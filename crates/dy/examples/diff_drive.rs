use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
    render::{mesh::Mesh, pipeline::RenderPipeline},
    wgpu::{WgpuFeature, WgpuFeatures, WgpuOptions},
};
use bevy_fly_camera::{FlyCamera2d, FlyCameraPlugin};
use dy::*;
use rand::{thread_rng, Rng};
use vz::*;

fn main() {
    App::build()
        .add_plugin(bevy::core::CorePlugin::default())
        .add_plugin(bevy::transform::TransformPlugin::default())
        .add_plugin(bevy::diagnostic::DiagnosticsPlugin::default())
        .add_plugin(bevy::input::InputPlugin::default())
        .add_plugin(bevy::window::WindowPlugin::default())
        .add_plugin(bevy::asset::AssetPlugin::default())
        .add_plugin(bevy::render::RenderPlugin::default())
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
                    .spawn_bundle(OrthographicCameraBundle::new_2d())
                    .insert(FlyCamera2d::default());
            })
            .system(),
        )
        .add_plugin(FlyCameraPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_system(
            (|diagnostics: Res<Diagnostics>, mut windows: ResMut<Windows>| {
                let window = windows.get_primary_mut().unwrap();
                window.set_title(format!(
                    "{:.3}s({:.0}Hz)",
                    diagnostics
                        .get(FrameTimeDiagnosticsPlugin::FRAME_TIME)
                        .unwrap()
                        .average()
                        .unwrap_or(0.0),
                    diagnostics
                        .get(FrameTimeDiagnosticsPlugin::FPS)
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
            (|mut commands: Commands, mut mesh_assets: ResMut<Assets<Mesh>>| {
                let diff_drive = DiffDrive { radius: 10.0 };
                commands
                    .spawn_bundle(MeshBundle {
                        mesh: mesh_assets.add(Mesh::from(&diff_drive)),
                        render_pipelines: RenderPipelines::from_pipelines(vec![
                            RenderPipeline::new(NON_FILL_PIPELINE.typed()),
                        ]),
                        transform: Transform::from_xyz(0.0, 0.0, 0.0),
                        ..Default::default()
                    })
                    .insert(diff_drive);
                let path = Path { len: 200 };
                commands
                    .spawn_bundle(MeshBundle {
                        mesh: mesh_assets.add(Mesh::from(&path)),
                        render_pipelines: RenderPipelines::from_pipelines(vec![
                            RenderPipeline::new(NON_FILL_PIPELINE.typed()),
                        ]),
                        transform: Transform::from_xyz(0.0, 0.0, 0.0),
                        ..Default::default()
                    })
                    .insert(path);
            })
            .system(),
        )
        .add_system(
            (|mut mesh_assets: ResMut<Assets<Mesh>>,
              mut diff_drive_query: Query<(&DiffDrive, &mut Transform)>,
              mut path_query: Query<(&Path, &Handle<Mesh>)>| {
                let mut rng = thread_rng();
                let (_, mut diff_drive_transform) = diff_drive_query.single_mut().unwrap();
                DiffDrive::update(
                    &mut *diff_drive_transform,
                    rng.gen_range(50.0..80.0),
                    rng.gen_range(-5.0..1.5),
                    0.1,
                );

                let (path, path_mesh_handle) = path_query.single_mut().unwrap();
                let mesh = mesh_assets.get_mut(path_mesh_handle).unwrap();
                path.add_point(
                    mesh,
                    [
                        diff_drive_transform.translation.x,
                        diff_drive_transform.translation.y,
                        diff_drive_transform.translation.z,
                    ],
                );
            })
            .system(),
        )
        .run();
}
