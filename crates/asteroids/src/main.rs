use asteroids::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(ImagePlugin::default_nearest()),))
        .insert_resource(CameraSettings { scale: 20.0 })
        .insert_resource(RelativeSettings {
            speed_of_light: 5.0,
        })
        .add_systems(Startup, (sys_setup,))
        .add_systems(
            Update,
            (
                sys_control_observer,
                sys_update_camera,
                sys_update_observer,
                sys_update_relative_objects,
            ),
        )
        .run();
}
