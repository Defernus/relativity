use crate::*;
use bevy::math::DVec3;
use bevy::prelude::*;
use bevy::sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle};
use relativity::SpacetimeEvent;

struct ObjectInit {
    pos: DVec3,
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

    let next_id = circle_objects(
        &mut commands,
        DVec3::X * 4.0,
        2.0,
        DVec3::ZERO,
        4,
        next_id,
        circle.clone(),
        materials.add(color),
    );

    let _next_id = circle_objects(
        &mut commands,
        -DVec3::X * 16.0,
        2.0,
        DVec3::X * 0.9,
        4,
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

fn circle_objects<M: Material2d>(
    commands: &mut Commands,
    pos: DVec3,
    radius: f64,
    velocity: DVec3,
    segments: u32,
    id_start: u32,
    mesh: Mesh2dHandle,
    material: Handle<M>,
) -> u32 {
    for i in 0..segments {
        let id = id_start + i;

        let angle = i as f64 * 2.0 * std::f64::consts::PI / segments as f64;

        ObjectInit {
            pos: pos + DVec3::new(angle.cos() * radius, angle.sin() * radius, 0.0),
            velocity,
        }
        .spawn(commands, id, mesh.clone(), material.clone(), true);
    }

    ObjectInit { pos, velocity }.spawn(
        commands,
        id_start + segments,
        mesh.clone(),
        material.clone(),
        true,
    );

    id_start + segments + 1
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
        let z_offset = DVec3::Z * (id + 1) as f64 * 0.0001;

        let object_entity = commands
            .spawn(MaterialMesh2dBundle {
                mesh,
                material,
                ..default()
            })
            .insert(Name::new(format!("object_{}", id)))
            .insert(RelativeObject::new(id, self.pos + z_offset, self.velocity))
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
