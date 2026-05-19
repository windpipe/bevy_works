use bevy::prelude::Resource;
use std::fmt;
#[cfg(not(target_arch = "wasm32"))]
use std::fs;
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;

const CONFIG_FILE: &str = "bevy_test04_settings.txt";

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RendererBackend {
    Auto,
    Dx12,
    Vulkan,
}

impl RendererBackend {
    const ALL: [RendererBackend; 3] = [Self::Auto, Self::Dx12, Self::Vulkan];

    pub fn next(self) -> Self {
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

    #[cfg(not(target_arch = "wasm32"))]
    pub fn env_value(self) -> Option<&'static str> {
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
pub struct ResolutionOption {
    pub width: u32,
    pub height: u32,
}

impl ResolutionOption {
    const ALL: [ResolutionOption; 4] = [
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
    ];

    pub fn next(self) -> Self {
        let index = Self::ALL
            .iter()
            .position(|resolution| *resolution == self)
            .unwrap_or(0);
        Self::ALL[(index + 1) % Self::ALL.len()]
    }

    fn from_str(value: &str) -> Self {
        let Some((width, height)) = value.trim().split_once('x') else {
            return Self::default();
        };
        let Ok(width) = width.parse::<u32>() else {
            return Self::default();
        };
        let Ok(height) = height.parse::<u32>() else {
            return Self::default();
        };
        Self { width, height }
    }
}

impl Default for ResolutionOption {
    fn default() -> Self {
        Self {
            width: 1280,
            height: 720,
        }
    }
}

impl fmt::Display for ResolutionOption {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}x{}", self.width, self.height)
    }
}

#[derive(Resource, Clone, Copy)]
pub struct GraphicsSettings {
    pub vsync: bool,
    pub fullscreen: bool,
    pub resolution: ResolutionOption,
    pub renderer: RendererBackend,
}

impl Default for GraphicsSettings {
    fn default() -> Self {
        Self {
            vsync: true,
            fullscreen: false,
            resolution: ResolutionOption::default(),
            renderer: RendererBackend::Auto,
        }
    }
}

impl GraphicsSettings {
    pub fn load() -> Self {
        load_settings_content()
            .map(|content| Self::from_config_text(&content))
            .unwrap_or_default()
    }

    pub fn save(self) -> std::io::Result<()> {
        save_settings_content(&self.to_config_text())
    }

    fn from_config_text(content: &str) -> Self {
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

    fn to_config_text(self) -> String {
        format!(
            "vsync={}\nfullscreen={}\nresolution={}\nrenderer={}\n",
            self.vsync, self.fullscreen, self.resolution, self.renderer
        )
    }
}

pub fn settings_description(settings: &GraphicsSettings) -> String {
    format!(
        "VSync: {}, Fullscreen: {}, Resolution: {}, Renderer: {}",
        on_off(settings.vsync),
        on_off(settings.fullscreen),
        settings.resolution,
        settings.renderer,
    )
}

pub fn on_off(value: bool) -> &'static str {
    if value { "On" } else { "Off" }
}

fn parse_bool(value: &str, default_value: bool) -> bool {
    match value.trim().to_ascii_lowercase().as_str() {
        "true" | "1" | "yes" | "on" => true,
        "false" | "0" | "no" | "off" => false,
        _ => default_value,
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn load_settings_content() -> Option<String> {
    fs::read_to_string(config_path()).ok()
}

#[cfg(target_arch = "wasm32")]
fn load_settings_content() -> Option<String> {
    web_sys::window()
        .and_then(|window| window.local_storage().ok().flatten())
        .and_then(|storage| storage.get_item(CONFIG_FILE).ok().flatten())
}

#[cfg(not(target_arch = "wasm32"))]
fn save_settings_content(content: &str) -> std::io::Result<()> {
    fs::write(config_path(), content)
}

#[cfg(target_arch = "wasm32")]
fn save_settings_content(content: &str) -> std::io::Result<()> {
    let Some(storage) = web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "localStorage is not available",
        ));
    };

    storage
        .set_item(CONFIG_FILE, content)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "failed to write localStorage"))
}

#[cfg(not(target_arch = "wasm32"))]
fn config_path() -> PathBuf {
    project_working_dir().join(CONFIG_FILE)
}

#[cfg(not(target_arch = "wasm32"))]
fn project_working_dir() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}
