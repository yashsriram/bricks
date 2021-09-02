pub mod plan;
pub mod vis;

use bevy::app::{AppBuilder, Plugin, PluginGroup, PluginGroupBuilder};
use bevy::diagnostic::Diagnostics;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::wgpu::{WgpuFeature, WgpuFeatures, WgpuOptions};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};

pub struct BasePlugins;

impl PluginGroup for BasePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(WGPUOptionsPlugin);
        group.add(bevy::core::CorePlugin::default());
        group.add(bevy::transform::TransformPlugin::default());
        group.add(bevy::diagnostic::DiagnosticsPlugin::default());
        group.add(bevy::input::InputPlugin::default());
        group.add(bevy::window::WindowPlugin::default());
        group.add(bevy::asset::AssetPlugin::default());
        group.add(bevy::render::RenderPlugin::default());
        group.add(bevy::winit::WinitPlugin::default());
        group.add(bevy::wgpu::WgpuPlugin::default());
        group.add(vis::WireframePlugin);
        group.add(CameraPlugin);
        group.add(FPSTitlePlugin);
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
        "FPS: {:.0}",
        diagnostics
            .get(FrameTimeDiagnosticsPlugin::FPS)
            .unwrap()
            .average()
            .unwrap_or(0.0)
    ));
}
