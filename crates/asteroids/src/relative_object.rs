use crate::*;
use bevy::math::DVec3;
use bevy::prelude::*;
use relativity::{SpacetimeEvent, VisibleWorldLineEvent, WorldLine, WorldLineEvent};

#[derive(Debug, Component)]
pub struct RelativeObject {
    pub id: u32,
    pub world_line: WorldLine,
}

impl RelativeObject {
    pub fn new(id: u32, coord: SpacetimeEvent, velocity: DVec3) -> Self {
        Self {
            id,
            world_line: WorldLine::new(WorldLineEvent {
                coord,
                velocity,
                object_proper_time: 0.0,
            }),
        }
    }
}

#[derive(Debug, Resource)]
pub struct RelativeSettings {
    pub speed_of_light: f64,
}

pub fn sys_update_relative_objects(
    mut query: Query<(&RelativeObject, &mut Transform, &Children, &mut Visibility)>,
    mut text_query: Query<&mut Text>,
    settings: Res<RelativeSettings>,
    observer_query: Query<&ObserverData>,
) {
    let c = settings.speed_of_light;
    let observer = observer_query.single();

    for (object, mut transform, children, mut visible) in query.iter_mut() {
        let Some(VisibleWorldLineEvent {
            relative_coord,
            proper_time: object_proper_time,
            velocity,
            relative_velocity,
        }) = object
            .world_line
            .get_visible_event(observer.coord, observer.velocity, c)
        else {
            *visible = Visibility::Hidden;
            continue;
        };

        *visible = Visibility::Visible;

        transform.translation.x = relative_coord.pos.x as f32;
        transform.translation.y = relative_coord.pos.y as f32;
        transform.translation.z = relative_coord.pos.z as f32;

        for child in children.iter() {
            if let Ok(mut text) = text_query.get_mut(*child) {
                text.sections[0].value = format!(
                    "\
                    [{}]\n\
                    t={:.3}\n\
                    v={:.4}\n\
                    rv={:.4}",
                    object.id,
                    object_proper_time,
                    velocity.length(),
                    relative_velocity.length(),
                );
            }
        }
    }
}
