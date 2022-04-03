pub mod plugins;
pub use bevy;

use bevy::prelude::*;

pub trait AsEntity: Sized {
    fn into_mesh_bundles(&self, meshes: &mut ResMut<Assets<Mesh>>) -> Vec<MeshBundle>;

    fn spawn(
        &self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        transform: Transform,
    ) -> Entity {
        commands
            .spawn_bundle(MeshBundle {
                transform,
                ..Default::default()
            })
            .with_children(|parent| {
                let child_bundle_list: Vec<MeshBundle> = self.into_mesh_bundles(meshes);
                for child_bundle in child_bundle_list.into_iter() {
                    parent.spawn_bundle(child_bundle);
                }
            })
            .id()
    }
}
