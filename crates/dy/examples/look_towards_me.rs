use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
    render::{mesh::Mesh, pipeline::RenderPipeline},
    wgpu::{WgpuFeature, WgpuFeatures, WgpuOptions},
};
use bevy_fly_camera::{FlyCamera2d, FlyCameraPlugin};
use dy::*;
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
        .add_startup_system(init.system())
        .add_system(look_towards_me.system())
        .run();
}

#[derive(Copy, Clone)]
struct TargetAngleRad(f32);

fn init(mut commands: Commands, mut mesh_assets: ResMut<Assets<Mesh>>) {
    let diff_drive = DiffDrive { radius: 50.0 };
    commands
        .spawn_bundle(MeshBundle {
            mesh: mesh_assets.add(Mesh::from(&diff_drive)),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                NON_FILL_PIPELINE.typed(),
            )]),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(diff_drive);

    let target_angle_rad = TargetAngleRad(3.5);
    commands.insert_resource(target_angle_rad);
    let path = Path { len: 200 };
    let mut mesh = Mesh::from(&path);
    path.add_point(
        &mut mesh,
        [
            100.0 * target_angle_rad.0.cos(),
            100.0 * target_angle_rad.0.sin(),
            0.0,
        ],
    );
    commands
        .spawn_bundle(MeshBundle {
            mesh: mesh_assets.add(mesh),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                NON_FILL_PIPELINE.typed(),
            )]),
            ..Default::default()
        })
        .insert(path);
}

fn look_towards_me(
    mut diff_drive_query: Query<(&DiffDrive, &mut Transform)>,
    target_angle_rad: Res<TargetAngleRad>,
) {
    let (_, mut diff_drive_transform) = diff_drive_query.single_mut().unwrap();
    let (axis, angle) = diff_drive_transform.rotation.to_axis_angle();
    let orient_in_rad = axis.z.signum() * angle;
    let unit_vec = Vec3::new(orient_in_rad.cos(), orient_in_rad.sin(), 0.0);
    let target_unit_vec = Vec3::new(target_angle_rad.0.cos(), target_angle_rad.0.sin(), 0.0);
    let cross_product = unit_vec.cross(target_unit_vec);
    let w = cross_product.length().min(0.1) * cross_product.z.signum();
    DiffDrive::update(&mut *diff_drive_transform, 0.0, w, 0.1);
}
