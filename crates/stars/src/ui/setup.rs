use bevy::prelude::*;

#[derive(Component)]
pub struct DebugText;

pub fn sys_setup_ui(mut commands: Commands) {
    commands
        .spawn(
            TextBundle::from_section("", TextStyle::default()).with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(12.0),
                left: Val::Px(12.0),
                ..default()
            }),
        )
        .insert(DebugText);
}
