use crate::*;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::math::DVec3;
use bevy::prelude::*;
use relativity::SpacetimeEvent;

struct ObjectInit {
    coord: SpacetimeEvent,
    velocity: DVec3,
}

const OBJECT_SIZE: f32 = 1.0;

pub fn sys_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sphere = meshes.add(Sphere {
        radius: OBJECT_SIZE * 0.5,
    });

    let star_material = materials.add(StandardMaterial {
        emissive: LinearRgba::rgb(20.0, 20.0, 0.0),
        ..Default::default()
    });

    let next_id = 0;

    let _next_id = grid_objects(
        &mut commands,
        SpacetimeEvent::new(DVec3::X * 4.0).with_time(-1000.0),
        DVec3::ZERO,
        32,
        100.0,
        next_id,
        sphere.clone(),
        star_material.clone(),
    );

    commands
        .spawn(Name::new("observer"))
        .insert(ObserverData {
            proper_time: 0.0,
            velocity: DVec3::ZERO,
            coord: SpacetimeEvent::ZERO,
            acceleration: 10.0,
        })
        .insert(Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..Default::default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            ..Default::default()
        })
        .insert(BloomSettings::NATURAL)
        .insert(MainCamera);
}

/// Spawn objects in grid pattern
fn grid_objects(
    commands: &mut Commands,
    coord: SpacetimeEvent,
    velocity: DVec3,
    size: u32,
    spacing: f64,
    id_start: u32,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
) -> u32 {
    let offset = (size + 1) as f64 * spacing * 0.5;

    for i in 0..size {
        for j in 0..size {
            for k in 0..size {
                let id = id_start + i * size * size + j * size + k;

                let mut object_coord = coord;
                object_coord.pos += DVec3::new(
                    k as f64 * spacing - offset,
                    j as f64 * spacing - offset,
                    i as f64 * spacing - offset,
                );

                ObjectInit {
                    coord: object_coord,
                    velocity,
                }
                .spawn(commands, id, mesh.clone(), material.clone());
            }
        }
    }

    id_start + size * size
}

impl ObjectInit {
    fn spawn(
        &self,
        commands: &mut Commands,
        id: u32,
        mesh: Handle<Mesh>,
        material: Handle<StandardMaterial>,
    ) {
        commands
            .spawn(PbrBundle {
                mesh,
                material,
                ..default()
            })
            .insert(Name::new(format!("object_{}", id)))
            .insert(RelativeObject::new(id, self.coord, self.velocity));
    }
}
