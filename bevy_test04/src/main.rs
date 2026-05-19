mod game;
mod options_popup;
mod settings;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::window::{
    MonitorSelection, PresentMode, Window, WindowMode, WindowPlugin, WindowResolution,
};
use game::PongGamePlugin;
use options_popup::OptionsPopupPlugin;
use settings::GraphicsSettings;

fn main() {
    let settings = GraphicsSettings::load();

    #[cfg(not(target_arch = "wasm32"))]
    unsafe {
        if let Some(backend) = settings.renderer.env_value() {
            std::env::set_var("WGPU_BACKEND", backend);
        } else {
            std::env::remove_var("WGPU_BACKEND");
        }
    }

    App::new()
        .insert_resource(settings)
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "bevy_test04 - Single Pong".to_string(),
                    resolution: WindowResolution::new(
                        settings.resolution.width,
                        settings.resolution.height,
                    ),
                    present_mode: if settings.vsync {
                        PresentMode::Fifo
                    } else {
                        PresentMode::AutoNoVsync
                    },
                    mode: if settings.fullscreen {
                        WindowMode::BorderlessFullscreen(MonitorSelection::Primary)
                    } else {
                        WindowMode::Windowed
                    },
                    ..default()
                }),
                ..default()
            }),
            FrameTimeDiagnosticsPlugin::default(),
            PongGamePlugin,
            OptionsPopupPlugin,
        ))
        .run();
}
