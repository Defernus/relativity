use bevy::prelude::*;
use stars::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(ImagePlugin::default_nearest()),))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(RelativeSettings {
            speed_of_light: 10.0,
        })
        .add_systems(Startup, (sys_setup, sys_setup_ui))
        .add_systems(
            Update,
            (
                sys_control_observer,
                sys_update_camera,
                sys_update_observer,
                sys_update_relative_objects,
                sys_debug_text,
            ),
        )
        .run();
}
