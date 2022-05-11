use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
    render::{
        mesh::{Indices, Mesh, VertexAttributeValues},
        pipeline::{PrimitiveTopology, RenderPipeline},
    },
    wgpu::{WgpuFeature, WgpuFeatures, WgpuOptions},
};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use bricks::vz::*;

#[derive(Debug)]
struct DiffDrive {
    radius: f32,
}

impl DiffDrive {
    const V: f32 = 0.1;
    const W: f32 = 0.2;
    const DELTA_T: f32 = 0.2;

    fn update(transform: &mut Transform) {
        let (axis, orient_in_rad) = transform.rotation.to_axis_angle();
        transform.translation.x += DiffDrive::V * orient_in_rad.cos() * DiffDrive::DELTA_T;
        transform.translation.y += DiffDrive::V * orient_in_rad.sin() * DiffDrive::DELTA_T;
        transform.rotation *= Quat::from_rotation_z(DiffDrive::W * DiffDrive::DELTA_T);
    }
}

impl From<&DiffDrive> for Mesh {
    fn from(diff_drive: &DiffDrive) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);
        let num_partitions: usize = 18;
        let mut positions: Vec<[f32; 3]> = (0..=num_partitions)
            .map(|i| 2.0 * std::f32::consts::PI / num_partitions as f32 * i as f32)
            .map(|theta| {
                [
                    diff_drive.radius * theta.cos(),
                    diff_drive.radius * theta.sin(),
                    0.0,
                ]
            })
            .collect();
        positions.push([0.0, 0.0, 0.0]);
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        let indices = Indices::U32((0..=(num_partitions + 1)).map(|i| i as u32).collect());
        mesh.set_indices(Some(indices));
        let colors = vec![[1.0, 1.0, 0.0, 0.1]; num_partitions + 2];
        mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        mesh
    }
}

struct Path;

impl Path {
    fn add_point(mesh: &mut Mesh, x: f32, y: f32, z: f32) {
        if let Some(VertexAttributeValues::Float3(ref mut positions)) =
            mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
        {
            positions.push([x, y, z]);
        }
        if let Some(VertexAttributeValues::Float4(ref mut colors)) =
            mesh.attribute_mut(Mesh::ATTRIBUTE_COLOR)
        {
            colors.push([1.0, 1.0, 1.0, 1.0]);
        }
        if let Some(ref mut indices) = mesh.indices_mut() {
            if let Indices::U32(ref mut indices) = indices {
                indices.push(indices.len() as u32);
            }
        }
    }
}

impl From<&Path> for Mesh {
    fn from(_: &Path) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, vec![[0.0, 0.0, 0.0]]);
        mesh.set_indices(Some(Indices::U32(vec![0])));
        let colors = vec![[1.0, 1.0, 1.0, 0.1]];
        mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        mesh
    }
}

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
            (|mut commands: Commands, mut mesh_assets: ResMut<Assets<Mesh>>| {
                let diff_drive = DiffDrive { radius: 2.0 };
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
                let path = Path;
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
                let (_, mut diff_drive_transform) = diff_drive_query.single_mut().unwrap();
                DiffDrive::update(&mut *diff_drive_transform);

                let (_, path_mesh_handle) = path_query.single_mut().unwrap();
                let mesh = mesh_assets.get_mut(path_mesh_handle).unwrap();
                Path::add_point(
                    mesh,
                    diff_drive_transform.translation.x,
                    diff_drive_transform.translation.y,
                    diff_drive_transform.translation.z,
                );
            })
            .system(),
        )
        .run();
}
