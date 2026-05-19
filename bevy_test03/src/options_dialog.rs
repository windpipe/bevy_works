use crate::{GraphicsSettings, project_working_dir};
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use std::process::Command;

pub struct OptionsDialogPlugin;

impl Plugin for OptionsDialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_options_dialog).add_systems(
            Update,
            (
                handle_option_buttons,
                update_settings_text,
                update_fps_text,
                update_button_colors,
            ),
        );
    }
}

#[derive(Component)]
struct SettingsText;

#[derive(Component)]
struct FpsText;

#[derive(Component)]
enum OptionButton {
    ToggleVsync,
    ToggleFullscreen,
    NextResolution,
    NextRenderer,
    ApplyAndRestart,
}

fn setup_options_dialog(mut commands: Commands, settings: Res<GraphicsSettings>) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.04, 0.04, 0.06)),
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    width: Val::Px(620.0),
                    padding: UiRect::all(Val::Px(24.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(14.0),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.12, 0.12, 0.16, 0.95)),
            ))
            .with_children(|panel| {
                panel.spawn((
                    Text::new("Graphics Options"),
                    TextFont {
                        font_size: 34.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.95, 1.0)),
                ));

                panel.spawn((
                    Text::new(settings_description(&settings)),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    SettingsText,
                ));

                spawn_button(panel, OptionButton::ToggleVsync, "Toggle VSync");
                spawn_button(panel, OptionButton::ToggleFullscreen, "Toggle Fullscreen");
                spawn_button(panel, OptionButton::NextResolution, "Next Resolution");
                spawn_button(panel, OptionButton::NextRenderer, "Next Renderer");
                spawn_button(panel, OptionButton::ApplyAndRestart, "Apply & Restart");

                panel.spawn((
                    Text::new("FPS: ..."),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.7, 0.9, 0.7)),
                    FpsText,
                ));
            });
        });
}

fn spawn_button(parent: &mut ChildSpawnerCommands, action: OptionButton, label: &str) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(44.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.22, 0.24, 0.32)),
            action,
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(label),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn handle_option_buttons(
    mut settings: ResMut<GraphicsSettings>,
    interaction_query: Query<(&Interaction, &OptionButton), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, action) in &interaction_query {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match action {
            OptionButton::ToggleVsync => settings.vsync = !settings.vsync,
            OptionButton::ToggleFullscreen => settings.fullscreen = !settings.fullscreen,
            OptionButton::NextResolution => settings.resolution = settings.resolution.next(),
            OptionButton::NextRenderer => settings.renderer = settings.renderer.next(),
            OptionButton::ApplyAndRestart => apply_and_restart(*settings),
        }
    }
}

fn update_settings_text(
    settings: Res<GraphicsSettings>,
    mut query: Query<&mut Text, With<SettingsText>>,
) {
    if !settings.is_changed() {
        return;
    }

    for mut text in &mut query {
        **text = settings_description(&settings);
    }
}

fn update_fps_text(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<FpsText>>) {
    let fps_text = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|diagnostic| diagnostic.smoothed())
        .map(|fps| format!("{fps:.1}"))
        .unwrap_or_else(|| "...".to_string());

    for mut text in &mut query {
        **text = format!("FPS: {fps_text}");
    }
}

fn update_button_colors(
    mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in &mut query {
        *color = match *interaction {
            Interaction::Pressed => BackgroundColor(Color::srgb(0.12, 0.55, 0.32)),
            Interaction::Hovered => BackgroundColor(Color::srgb(0.30, 0.34, 0.46)),
            Interaction::None => BackgroundColor(Color::srgb(0.22, 0.24, 0.32)),
        };
    }
}

fn apply_and_restart(settings: GraphicsSettings) {
    if let Err(error) = settings.save() {
        eprintln!("Failed to save settings: {error}");
        return;
    }

    let Ok(exe_path) = std::env::current_exe() else {
        eprintln!("Failed to get current executable path");
        return;
    };

    let mut command = Command::new(exe_path);
    command.current_dir(project_working_dir());

    if let Some(backend) = settings.renderer.env_value() {
        command.env("WGPU_BACKEND", backend);
    } else {
        command.env_remove("WGPU_BACKEND");
    }

    if let Err(error) = command.spawn() {
        eprintln!("Failed to restart application: {error}");
        return;
    }

    std::process::exit(0);
}

fn settings_description(settings: &GraphicsSettings) -> String {
    format!(
        "Current selection\n\nVSync: {}\nFullscreen: {}\nResolution: {}\nRenderer: {}\n\nClick options, then Apply & Restart.\nRenderer changes must be selected before Bevy starts, so they require restart.",
        on_off(settings.vsync),
        on_off(settings.fullscreen),
        settings.resolution,
        settings.renderer,
    )
}

fn on_off(value: bool) -> &'static str {
    if value { "On" } else { "Off" }
}
