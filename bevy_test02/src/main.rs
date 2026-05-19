use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::{PresentMode, Window, WindowPlugin};

#[derive(Component)]
struct PerformanceText;

#[derive(Resource)]
struct VsyncState {
    enabled: bool,
}

fn main() {
    let vsync_enabled = std::env::var("BEVY_TEST02_VSYNC")
        .map(|value| !matches!(value.as_str(), "0" | "false" | "False" | "off" | "Off"))
        .unwrap_or(true);

    let present_mode = if vsync_enabled {
        PresentMode::Fifo
    } else {
        PresentMode::AutoNoVsync
    };

    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "bevy_test02 - FPS / VSync Toggle".to_string(),
                    present_mode,
                    ..default()
                }),
                ..default()
            }),
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .insert_resource(VsyncState {
            enabled: vsync_enabled,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, update_performance_text)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Text::new("FPS: ...\nVSync: ...\nSet BEVY_TEST02_VSYNC=0 to disable"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(12.0),
            top: Val::Px(12.0),
            ..default()
        },
        PerformanceText,
    ));
}

fn update_performance_text(
    diagnostics: Res<DiagnosticsStore>,
    vsync_state: Res<VsyncState>,
    mut text_query: Query<&mut Text, With<PerformanceText>>,
) {
    let fps_text = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|diagnostic| diagnostic.smoothed())
        .map(|fps| format!("{fps:.1}"))
        .unwrap_or_else(|| "...".to_string());

    let vsync_text = if vsync_state.enabled { "On" } else { "Off" };
    let hint_text = if vsync_state.enabled {
        "Restart with BEVY_TEST02_VSYNC=0 to disable"
    } else {
        "Restart without BEVY_TEST02_VSYNC=0 to enable"
    };

    for mut text in &mut text_query {
        **text = format!("FPS: {fps_text}\nVSync: {vsync_text}\n{hint_text}");
    }
}
