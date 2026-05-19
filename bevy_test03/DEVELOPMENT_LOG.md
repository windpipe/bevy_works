# bevy_test03 개발 기록

## 1. 목표

`bevy_test03`에서는 그래픽 옵션 선택 화면을 만들고, 선택한 옵션을 저장한 뒤 앱을 재시작해서 적용하도록 구성했습니다.

요구 옵션:

- VSync 여부
- Fullscreen 여부
- 해상도 선택
- 렌더러 선택: Auto, DX12, Vulkan
- 설정 적용 시 재시작

## 2. 프로젝트 생성

작업 디렉터리:

```text
E:\works\works_rust\bevy_works
```

실행한 명령:

```text
cargo new bevy_test03
```

## 3. Bevy 의존성 추가

프로젝트 디렉터리:

```text
E:\works\works_rust\bevy_works\bevy_test03
```

실행한 명령:

```text
cargo add bevy
```

추가된 Bevy 버전:

```text
bevy v0.18.1
```

현재 `Cargo.toml` 핵심 내용:

```toml
[package]
name = "bevy_test03"
version = "0.1.0"
edition = "2024"

[dependencies]
bevy = "0.18.1"
```

## 4. 구현 방향

`bevy_test02`에서 실행 중 `present_mode`를 바꿔 VSync를 토글하면, 일부 환경에서 프로그램이 종료되는 문제가 있었습니다.

그래서 `bevy_test03`에서는 런타임 중 그래픽 백엔드나 swapchain 관련 옵션을 직접 바꾸지 않습니다.

대신 다음 구조로 구현했습니다.

1. 앱 시작 시 설정 파일 읽기
2. 설정에 따라 창 옵션과 렌더러 백엔드 설정
3. 옵션 다이얼로그 UI 표시
4. 사용자가 옵션 버튼 클릭
5. `Apply & Restart` 클릭
6. 설정 파일 저장
7. 현재 실행 파일을 새 설정으로 다시 실행
8. 기존 프로세스 종료

이 방식은 렌더러나 VSync 같은 민감한 설정을 Bevy/wgpu 초기화 전에 결정할 수 있어 더 안정적입니다.

## 5. 설정 파일

설정 파일 이름:

```text
bevy_test03_settings.txt
```

저장 위치:

```text
E:\works\works_rust\bevy_works\bevy_test03\bevy_test03_settings.txt
```

예시 내용:

```text
vsync=true
fullscreen=false
resolution=1280x720
renderer=Auto
```

지원 값:

- `vsync=true` 또는 `false`
- `fullscreen=true` 또는 `false`
- `resolution=1280x720`, `1600x900`, `1920x1080`, `2560x1440`, `3840x2160`
- `renderer=Auto`, `DX12`, `Vulkan`

## 6. UI 구성

앱을 실행하면 중앙에 그래픽 옵션 패널이 표시됩니다.

표시 내용:

```text
Graphics Options

Current selection

VSync: On
Fullscreen: Off
Resolution: 1280x720
Renderer: Auto

Click options, then Apply & Restart.
Renderer changes must be selected before Bevy starts, so they require restart.

[Toggle VSync]
[Toggle Fullscreen]
[Next Resolution]
[Next Renderer]
[Apply & Restart]

FPS: ...
```

버튼 기능:

- `Toggle VSync`
  - VSync On/Off 선택값 변경
- `Toggle Fullscreen`
  - 창 모드/전체화면 선택값 변경
- `Next Resolution`
  - 해상도 후보를 순서대로 변경
- `Next Renderer`
  - Auto → DX12 → Vulkan 순서로 변경
- `Apply & Restart`
  - 설정 저장 후 앱 재시작

## 7. 주요 코드 리뷰

### 렌더러 선택 enum

```rust
#[derive(Clone, Copy, PartialEq, Eq)]
enum RendererBackend {
    Auto,
    Dx12,
    Vulkan,
}
```

- 렌더러 선택값을 표현합니다.
- `Auto`는 Bevy/wgpu가 자동으로 백엔드를 선택하게 합니다.
- `Dx12`, `Vulkan`은 `WGPU_BACKEND` 환경변수로 전달됩니다.
- OpenGL은 현재 환경에서 GPU 탐색 실패를 일으킬 수 있어 선택지에서 제외했습니다.

### 렌더러 환경변수 값

```rust
fn env_value(self) -> Option<&'static str> {
    match self {
        Self::Auto => None,
        Self::Dx12 => Some("dx12"),
        Self::Vulkan => Some("vulkan"),
    }
}
```

- `Auto`는 `WGPU_BACKEND` 환경변수를 제거해서 wgpu 자동 선택을 사용합니다.
- 나머지는 `WGPU_BACKEND` 값으로 사용합니다.

### 앱 시작 시 렌더러 적용

```rust
unsafe {
    if let Some(backend) = settings.renderer.env_value() {
        std::env::set_var("WGPU_BACKEND", backend);
    } else {
        std::env::remove_var("WGPU_BACKEND");
    }
}
```

Rust 2024 에디션에서는 환경변수 변경이 `unsafe`입니다.

이 코드는 Bevy가 시작되기 전, 즉 워커 스레드와 렌더러가 초기화되기 전에 실행되므로 이 프로젝트에서는 안전한 시점으로 사용했습니다.

### 창 설정 적용

```rust
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
```

적용 내용:

- `resolution`
  - 저장된 해상도로 창 크기 설정
- `present_mode`
  - `Fifo`: VSync On
  - `AutoNoVsync`: VSync Off
- `mode`
  - `Windowed`: 창 모드
  - `BorderlessFullscreen`: 기본 모니터 전체화면

### 재시작 처리

```rust
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
```

동작:

1. 현재 선택한 설정 저장
2. 현재 실행 파일 경로 확인
3. 새 프로세스 실행
4. 렌더러 선택에 맞게 `WGPU_BACKEND` 설정
5. 기존 프로세스 종료

## 8. 옵션 선택 다이얼로그 모듈 분리

옵션 선택 다이얼로그 관련 코드를 별도 모듈로 분리했습니다.

새 파일:

```text
src/options_dialog.rs
```

분리 후 역할은 다음과 같습니다.

### `src/main.rs`

`main.rs`는 이제 앱 전체 설정과 Bevy 초기화에 집중합니다.

담당 내용:

- 설정 파일 로드
- 렌더러 환경변수 설정
- `WindowPlugin` 설정
- `FrameTimeDiagnosticsPlugin` 추가
- `OptionsDialogPlugin` 추가
- 설정 구조체와 저장/로드 로직 관리

핵심 구조:

```rust
mod options_dialog;

use options_dialog::OptionsDialogPlugin;

App::new()
    .insert_resource(settings)
    .add_plugins((
        DefaultPlugins.set(WindowPlugin { ... }),
        FrameTimeDiagnosticsPlugin::default(),
        OptionsDialogPlugin,
    ))
    .run();
```

### `src/options_dialog.rs`

`options_dialog.rs`는 옵션 다이얼로그 UI와 버튼 동작을 담당합니다.

담당 내용:

- 옵션 패널 UI 생성
- 버튼 생성
- VSync 선택값 변경
- Fullscreen 선택값 변경
- 해상도 선택값 변경
- 렌더러 선택값 변경
- `Apply & Restart` 처리
- FPS 텍스트 갱신
- 버튼 hover/press 색상 변경

핵심 구조:

```rust
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
```

분리 효과:

- `main.rs`가 짧아짐
- 옵션 UI 코드를 독립적으로 수정하기 쉬워짐
- 나중에 실제 게임 화면과 옵션 화면을 분리하기 쉬워짐
- 옵션 다이얼로그를 플러그인처럼 추가/제거할 수 있음

분리 후 파일 구조:

```text
bevy_test03/
├── Cargo.toml
├── DEVELOPMENT_LOG.md
└── src/
    ├── main.rs
    └── options_dialog.rs
```

## 9. 빌드 확인

실행한 명령:

```text
cargo check
```

결과:

```text
Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.03s
```

즉, `bevy_test03`는 모듈 분리 후에도 컴파일 검사를 통과했습니다.

## 10. 실행 방법

프로젝트 디렉터리:

```text
E:\works\works_rust\bevy_works\bevy_test03
```

실행 명령:

```text
cargo run
```

실행 후 옵션 버튼을 클릭하고 `Apply & Restart`를 누르면 설정이 저장되고 앱이 재시작됩니다.

## 11. 주의사항

### 렌더러 변경은 재시작이 필요함

DX12, Vulkan 같은 렌더러 백엔드는 Bevy/wgpu 초기화 전에 선택되어야 합니다.

그래서 실행 중 즉시 바꾸는 것이 아니라 재시작 후 적용합니다.

### VSync도 재시작 방식으로 처리

`bevy_test02`에서 확인한 것처럼 실행 중 `present_mode`를 바꾸면 일부 환경에서 프로그램이 죽을 수 있습니다.

따라서 `bevy_test03`에서는 VSync도 재시작 후 적용합니다.

### Fullscreen과 해상도

Fullscreen은 `BorderlessFullscreen` 방식을 사용합니다.

이 방식은 일반적인 독점 전체화면보다 안정적이고 빠르게 전환됩니다. 다만 borderless fullscreen에서는 실제 모니터 모드 변경이 아니라 데스크톱 해상도에 맞춘 전체화면 창으로 동작할 수 있습니다.

## 12. 다음 단계 제안

다음 단계로 개선할 수 있는 내용:

1. 옵션별 좌우 화살표 버튼 추가
2. 실제 적용 중인 렌더러를 화면에 표시
3. 설정 저장 실패 시 UI에 에러 메시지 표시
4. 전체화면 모드에서 모니터 선택 추가
5. 창 모드와 독점 전체화면 모드 분리
