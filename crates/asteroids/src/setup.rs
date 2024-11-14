use crate::*;
use bevy::math::DVec3;
use bevy::prelude::*;
use bevy::sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle};
use relativity::SpacetimeEvent;

struct ObjectInit {
    coord: SpacetimeEvent,
    velocity: DVec3,
}

const OBJECT_SIZE: f32 = 0.2;

pub fn sys_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default()).insert(MainCamera);

    let circle = Mesh2dHandle(meshes.add(Circle {
        radius: OBJECT_SIZE * 0.5,
    }));
    let rect = Mesh2dHandle(meshes.add(Rectangle::new(OBJECT_SIZE, OBJECT_SIZE)));

    let color = Color::srgb(1.0, 0.0, 0.0);

    let next_id = 0;

    let _next_id = grid_objects(
        &mut commands,
        SpacetimeEvent::new(DVec3::X * 4.0).with_time(-1000.0),
        DVec3::ZERO,
        32,
        2.0,
        32,
        2.0,
        next_id,
        circle.clone(),
        materials.add(color),
    );

    let object_entity = commands
        .spawn(MaterialMesh2dBundle {
            mesh: rect,
            material: materials.add(Color::srgb(0.0, 0.0, 1.0)),
            ..default()
        })
        .insert(Name::new("observer"))
        .insert(ObserverData {
            proper_time: 0.0,
            velocity: DVec3::ZERO,
            coord: SpacetimeEvent::ZERO,
        })
        .id();

    commands
        .spawn(Text2dBundle {
            transform: Transform::from_translation(Vec3::Z * 0.7).with_scale(Vec3::splat(0.01)),
            text: Text::from_section(format!("observer"), TextStyle::default()),
            visibility: Visibility::Inherited,
            ..Default::default()
        })
        .set_parent(object_entity);
}

/// Spawn objects in grid pattern
fn grid_objects<M: Material2d>(
    commands: &mut Commands,
    coord: SpacetimeEvent,
    velocity: DVec3,
    rows: u32,
    row_spacing: f64,
    cols: u32,
    col_spacing: f64,
    id_start: u32,
    mesh: Mesh2dHandle,
    material: Handle<M>,
) -> u32 {
    let x_offset = cols as f64 * col_spacing * 0.5;
    let y_offset = rows as f64 * row_spacing * 0.5;

    for i in 0..rows {
        for j in 0..cols {
            let id = id_start + i * cols + j;

            let mut object_coord = coord;
            object_coord.pos += DVec3::new(
                j as f64 * col_spacing - x_offset,
                i as f64 * row_spacing - y_offset,
                0.0,
            );

            ObjectInit {
                coord: object_coord,
                velocity,
            }
            .spawn(commands, id, mesh.clone(), material.clone(), true);
        }
    }

    id_start + rows * cols
}

impl ObjectInit {
    fn spawn<M: Material2d>(
        &self,
        commands: &mut Commands,
        id: u32,
        mesh: Mesh2dHandle,
        material: Handle<M>,
        is_text_visible: bool,
    ) {
        let object_entity = commands
            .spawn(MaterialMesh2dBundle {
                mesh,
                material,
                ..default()
            })
            .insert(Name::new(format!("object_{}", id)))
            .insert(RelativeObject::new(id, self.coord, self.velocity))
            .id();

        let visibility = if is_text_visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };

        commands
            .spawn(Text2dBundle {
                transform: Transform::from_translation(Vec3::Z * 0.7).with_scale(Vec3::splat(0.01)),
                text: Text::from_section(format!("object"), TextStyle::default()),
                visibility,
                ..Default::default()
            })
            .set_parent(object_entity);
    }
}
