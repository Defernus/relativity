use crate::*;
use bevy::color::palettes::css::{BLUE, GREEN, LIGHT_GRAY, RED};
use bevy::prelude::*;

pub fn sys_debug_text(
    mut text_query: Query<&mut Text, With<DebugText>>,
    observer_query: Query<&ObserverData>,
    settings: Res<RelativeSettings>,
) {
    let mut text = text_query.single_mut();
    let observer = observer_query.single();

    let c = settings.speed_of_light;

    *text = Text::from_sections([
        section("c", LIGHT_GRAY),
        section(format!(" = {}", c), Color::WHITE),
        line_break(),
        section("Velocity", LIGHT_GRAY),
        line_break(),
        section("\t|V|", LIGHT_GRAY),
        section(
            format!(" = {:.6} * c", observer.velocity.length() / c),
            Color::WHITE,
        ),
        line_break(),
        section("\tV.x", RED),
        section(
            format!(" = {:.6} * c", observer.velocity.x / c),
            Color::WHITE,
        ),
        line_break(),
        section("\tV.y", GREEN),
        section(
            format!(" = {:.6} * c", observer.velocity.y / c),
            Color::WHITE,
        ),
        line_break(),
        section("\tV.z", BLUE),
        section(
            format!(" = {:.6} * c", observer.velocity.z / c),
            Color::WHITE,
        ),
    ]);
}

fn line_break() -> TextSection {
    TextSection::new("\n", TextStyle::default())
}

fn section(text: impl Into<String>, color: impl Into<Color>) -> TextSection {
    TextSection::new(
        text,
        TextStyle {
            color: color.into(),
            ..Default::default()
        },
    )
}
