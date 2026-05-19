mod options_dialog;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::window::{
    MonitorSelection, PresentMode, Window, WindowMode, WindowPlugin, WindowResolution,
};
use options_dialog::OptionsDialogPlugin;
use std::fmt;
use std::fs;
use std::path::PathBuf;

const CONFIG_FILE: &str = "bevy_test03_settings.txt";

#[derive(Clone, Copy, PartialEq, Eq)]
enum RendererBackend {
    Auto,
    Dx12,
    Vulkan,
}

impl RendererBackend {
    const ALL: [RendererBackend; 3] = [
        RendererBackend::Auto,
        RendererBackend::Dx12,
        RendererBackend::Vulkan,
    ];

    fn next(self) -> Self {
        let index = Self::ALL
            .iter()
            .position(|backend| *backend == self)
            .unwrap_or(0);
        Self::ALL[(index + 1) % Self::ALL.len()]
    }

    fn from_str(value: &str) -> Self {
        match value.trim().to_ascii_lowercase().as_str() {
            "dx12" => Self::Dx12,
            "vulkan" => Self::Vulkan,
            _ => Self::Auto,
        }
    }

    fn env_value(self) -> Option<&'static str> {
        match self {
            Self::Auto => None,
            Self::Dx12 => Some("dx12"),
            Self::Vulkan => Some("vulkan"),
        }
    }
}

impl fmt::Display for RendererBackend {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::Auto => "Auto",
            Self::Dx12 => "DX12",
            Self::Vulkan => "Vulkan",
        };
        write!(formatter, "{label}")
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct ResolutionOption {
    width: u32,
    height: u32,
}

impl ResolutionOption {
    const ALL: [ResolutionOption; 5] = [
        ResolutionOption {
            width: 1280,
            height: 720,
        },
        ResolutionOption {
            width: 1600,
            height: 900,
        },
        ResolutionOption {
            width: 1920,
            height: 1080,
        },
        ResolutionOption {
            width: 2560,
            height: 1440,
        },
        ResolutionOption {
            width: 3840,
            height: 2160,
        },
    ];

    fn next(self) -> Self {
        let index = Self::ALL
            .iter()
            .position(|resolution| *resolution == self)
            .unwrap_or(0);
        Self::ALL[(index + 1) % Self::ALL.len()]
    }

    fn from_str(value: &str) -> Self {
        let Some((width, height)) = value.trim().split_once('x') else {
            return Self::ALL[2];
        };

        let Ok(width) = width.parse::<u32>() else {
            return Self::ALL[2];
        };

        let Ok(height) = height.parse::<u32>() else {
            return Self::ALL[2];
        };

        Self { width, height }
    }
}

impl fmt::Display for ResolutionOption {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}x{}", self.width, self.height)
    }
}

#[derive(Resource, Clone, Copy)]
struct GraphicsSettings {
    vsync: bool,
    fullscreen: bool,
    resolution: ResolutionOption,
    renderer: RendererBackend,
}

impl Default for GraphicsSettings {
    fn default() -> Self {
        Self {
            vsync: true,
            fullscreen: false,
            resolution: ResolutionOption {
                width: 1280,
                height: 720,
            },
            renderer: RendererBackend::Auto,
        }
    }
}

impl GraphicsSettings {
    fn load() -> Self {
        let Ok(content) = fs::read_to_string(config_path()) else {
            return Self::default();
        };

        let mut settings = Self::default();

        for line in content.lines() {
            let Some((key, value)) = line.split_once('=') else {
                continue;
            };

            match key.trim() {
                "vsync" => settings.vsync = parse_bool(value, settings.vsync),
                "fullscreen" => settings.fullscreen = parse_bool(value, settings.fullscreen),
                "resolution" => settings.resolution = ResolutionOption::from_str(value),
                "renderer" => settings.renderer = RendererBackend::from_str(value),
                _ => {}
            }
        }

        settings
    }

    fn save(self) -> std::io::Result<()> {
        fs::write(
            config_path(),
            format!(
                "vsync={}\nfullscreen={}\nresolution={}\nrenderer={}\n",
                self.vsync, self.fullscreen, self.resolution, self.renderer
            ),
        )
    }
}

fn main() {
    let settings = GraphicsSettings::load();

    // This happens before Bevy starts worker threads, so it is the safe point to update
    // the process environment for wgpu backend selection.
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
                    title: "bevy_test03 - Graphics Options".to_string(),
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
            OptionsDialogPlugin,
        ))
        .run();
}

fn parse_bool(value: &str, default_value: bool) -> bool {
    match value.trim().to_ascii_lowercase().as_str() {
        "true" | "1" | "yes" | "on" => true,
        "false" | "0" | "no" | "off" => false,
        _ => default_value,
    }
}

fn config_path() -> PathBuf {
    project_working_dir().join(CONFIG_FILE)
}

fn project_working_dir() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}
