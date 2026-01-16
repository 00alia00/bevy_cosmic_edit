#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use bevy_cosmic_edit::{
    cosmic_text::{Attrs, Family, Metrics, Weight as FontWeight},
    prelude::*,
};

fn setup(mut commands: Commands, mut font_system: ResMut<CosmicFontSystem>) {
    commands.spawn(Camera2d);
    let root = commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..default()
        })
        .id();

    let attrs = Attrs::new();
    let serif_attrs = attrs.clone().family(Family::Serif);
    let mono_attrs = attrs.clone().family(Family::Monospace);
    let comic_attrs = attrs.clone().family(Family::Name("Comic Neue"));
    let lines = vec![
        ("B", attrs.clone().weight(FontWeight::BOLD)),
        ("old ", attrs.clone()),
        ("I", attrs.clone().style(FontStyle::Italic)),
        ("talic ", attrs.clone()),
        ("f", attrs.clone()),
        ("i ", attrs.clone()),
        ("f", attrs.clone().weight(FontWeight::BOLD)),
        ("i ", attrs.clone()),
        ("f", attrs.clone().style(FontStyle::Italic)),
        ("i ", attrs.clone()),
        ("Sans-Serif Normal ", attrs.clone()),
        ("Sans-Serif Bold ", attrs.clone().weight(FontWeight::BOLD)),
        ("Sans-Serif Italic ", attrs.clone().style(FontStyle::Italic)),
        (
            "Sans-Serif Bold Italic",
            attrs.clone().weight(FontWeight::BOLD).style(FontStyle::Italic),
        ),
        ("Serif Normal ", serif_attrs.clone()),
        ("Serif Bold ", serif_attrs.clone().weight(FontWeight::BOLD)),
        ("Serif Italic ", serif_attrs.clone().style(FontStyle::Italic)),
        (
            "Serif Bold Italic",
            serif_attrs
                .clone()
                .weight(FontWeight::BOLD)
                .style(FontStyle::Italic),
        ),
        ("\n", attrs.clone()),
        ("Mono Normal ", mono_attrs.clone()),
        ("Mono Bold ", mono_attrs.clone().weight(FontWeight::BOLD)),
        ("Mono Italic ", mono_attrs.clone().style(FontStyle::Italic)),
        (
            "Mono Bold Italic",
            mono_attrs.clone().weight(FontWeight::BOLD).style(FontStyle::Italic),
        ),
        ("Comic Normal ", comic_attrs.clone()),
        ("Comic Bold ", comic_attrs.clone().weight(FontWeight::BOLD)),
        ("Comic Italic ", comic_attrs.clone().style(FontStyle::Italic)),
        (
            "Comic Bold Italic",
            comic_attrs
                .clone()
                .weight(FontWeight::BOLD)
                .style(FontStyle::Italic),
        ),
        ("\n", attrs.clone()),
        (
            "R",
            attrs.clone().color(bevy::color::palettes::css::RED.to_cosmic()),
        ),
        (
            "A",
            attrs.clone().color(bevy::color::palettes::css::ORANGE.to_cosmic()),
        ),
        (
            "I",
            attrs.clone().color(bevy::color::palettes::css::YELLOW.to_cosmic()),
        ),
        (
            "N",
            attrs.clone().color(bevy::color::palettes::css::LIMEGREEN.to_cosmic()),
        ),
        (
            "B",
            attrs.clone().color(bevy::color::palettes::css::BLUE.to_cosmic()),
        ),
        (
            "O",
            attrs.clone().color(bevy::color::palettes::css::INDIGO.to_cosmic()),
        ),
        (
            "W ",
            attrs.clone().color(bevy::color::palettes::css::PURPLE.to_cosmic()),
        ),
        (
            "Red ",
            attrs.clone().color(bevy::color::palettes::css::RED.to_cosmic()),
        ),
        (
            "Orange ",
            attrs.clone().color(bevy::color::palettes::css::ORANGE.to_cosmic()),
        ),
        (
            "Yellow ",
            attrs.clone().color(bevy::color::palettes::css::YELLOW.to_cosmic()),
        ),
        (
            "Green ",
            attrs.clone().color(bevy::color::palettes::css::LIMEGREEN.to_cosmic()),
        ),
        (
            "Blue ",
            attrs.clone().color(bevy::color::palettes::css::BLUE.to_cosmic()),
        ),
        (
            "Indigo ",
            attrs.clone().color(bevy::color::palettes::css::INDIGO.to_cosmic()),
        ),
        (
            "Violet ",
            attrs.clone().color(bevy::color::palettes::css::PURPLE.to_cosmic()),
        ),
        (
            "U",
            attrs.clone().color(bevy::color::palettes::css::PURPLE.to_cosmic()),
        ),
        (
            "N",
            attrs.clone().color(bevy::color::palettes::css::INDIGO.to_cosmic()),
        ),
        (
            "I",
            attrs.clone().color(bevy::color::palettes::css::BLUE.to_cosmic()),
        ),
        (
            "C",
            attrs.clone().color(bevy::color::palettes::css::LIMEGREEN.to_cosmic()),
        ),
        (
            "O",
            attrs.clone().color(bevy::color::palettes::css::YELLOW.to_cosmic()),
        ),
        (
            "R",
            attrs.clone().color(bevy::color::palettes::css::ORANGE.to_cosmic()),
        ),
        (
            "N",
            attrs.clone().color(bevy::color::palettes::css::RED.to_cosmic()),
        ),
        (
            "生活,삶,जिंदगी 😀 FPS",
            attrs.clone().color(bevy::color::palettes::css::RED.to_cosmic()),
        ),
    ];

    commands.entity(root).with_children(|parent| {
        parent
            .spawn((
                TextEdit,
                CosmicEditBuffer::new(&mut font_system, Metrics::new(18., 22.)).with_rich_text(
                    &mut font_system,
                    lines,
                    attrs,
                ),
                Node {
                    width: Val::Percent(50.),
                    height: Val::Percent(100.),
                    ..default()
                },
                BackgroundColor(Color::WHITE),
            ))
            .observe(focus_on_click);
    });

    let mut attrs_2 = Attrs::new();
    attrs_2 = attrs_2.family(Family::Name("Times New Roman"));
    attrs_2.color_opt = Some(bevy::color::palettes::css::PURPLE.to_cosmic());
    commands.entity(root).with_children(|parent| {
        parent
            .spawn((
                TextEdit,
                CosmicEditBuffer::new(&mut font_system, Metrics::new(28., 36.)).with_text(
                    &mut font_system,
                    "Widget 2.\nClick on me =>",
                    attrs_2,
                ),
                Node {
                    width: Val::Percent(50.),
                    height: Val::Percent(100.),
                    ..default()
                },
                BackgroundColor(Color::WHITE.with_alpha(0.8)),
            ))
            .observe(focus_on_click);
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CosmicEditPlugin { ..default() })
        .add_systems(Startup, setup)
        .add_systems(Update, deselect_editor_on_esc)
        .run();
}
