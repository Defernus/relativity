use crate::*;
use bevy::math::DVec3;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

const OBJECTS: &[ObjectInit] = &[
    ObjectInit {
        pos: DVec3::new(-2.0, 0.0, 0.0),
        vel: DVec3::new(0.0, 1.0, 0.0),
    },
    ObjectInit {
        pos: DVec3::new(2.0, 0.0, 0.01),
        vel: DVec3::new(1.0, 1.0, 0.0),
    },
    ObjectInit {
        pos: DVec3::new(2.0, 1.0, 0.02),
        vel: DVec3::new(-1.0, 0.0, 0.0),
    },
    ObjectInit {
        pos: DVec3::new(0.0, -3.0, 0.03),
        vel: DVec3::new(0.0, 2.0, 0.0),
    },
];

struct ObjectInit {
    pos: DVec3,
    vel: DVec3,
}

pub fn sys_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default()).insert(MainCamera);

    let circle = Mesh2dHandle(meshes.add(Circle { radius: 0.5 }));
    let rect = Mesh2dHandle(meshes.add(Rectangle::new(1.0, 1.0)));

    for (i, object) in OBJECTS.iter().enumerate() {
        let color = Color::srgb(1.0, 0.0, 0.0);

        let text_entity = commands
            .spawn(Text2dBundle {
                transform: Transform::from_translation(Vec3::Z * 0.7).with_scale(Vec3::splat(0.01)),
                text: Text::from_section(format!("object"), TextStyle::default()),
                ..Default::default()
            })
            .id();

        commands
            .spawn(MaterialMesh2dBundle {
                mesh: circle.clone(),
                material: materials.add(color),
                ..default()
            })
            .insert(Name::new(format!("object_{}", i)))
            .insert(RelativeObject::new(i as u32, object.pos, object.vel))
            .add_child(text_entity);
    }

    let text_entity = commands
        .spawn(Text2dBundle {
            transform: Transform::from_translation(Vec3::Z * 0.7).with_scale(Vec3::splat(0.01)),
            text: Text::from_section(format!("observer"), TextStyle::default()),
            ..Default::default()
        })
        .id();
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: rect,
            material: materials.add(Color::srgb(0.0, 0.0, 1.0)),
            ..default()
        })
        .insert(Name::new("observer"))
        .insert(ObserverData {
            proper_time: 0.0,
            velocity: DVec3::ZERO,
            pos: DVec3::ZERO,
        })
        .add_child(text_entity);
}
