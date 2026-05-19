# bevy_test02 개발 기록

## 1. 목표

`bevy_test01` 다음 단계로 `bevy_test02` 프로젝트를 만들고, 화면에 FPS를 표시합니다.

목표 기능:

- Bevy 기본 앱 실행
- 2D 카메라 생성
- 좌상단에 FPS 텍스트 표시
- Bevy 진단 플러그인을 사용해 프레임레이트 측정

## 2. 프로젝트 생성

작업 디렉터리:

- `E:\works\works_rust\bevy_works`

실행한 명령:

```text
cargo new bevy_test02
```

결과:

- 바이너리 애플리케이션 패키지 `bevy_test02` 생성
- 기본 `Cargo.toml`, `src/main.rs`, `.gitignore` 생성

## 3. Bevy 의존성 추가

프로젝트 디렉터리:

- `E:\works\works_rust\bevy_works\bevy_test02`

실행한 명령:

```text
cargo add bevy
```

추가된 Bevy 버전:

- `bevy v0.18.1`

현재 `Cargo.toml`:

```toml
[package]
name = "bevy_test02"
version = "0.1.0"
edition = "2024"

[dependencies]
bevy = "0.18.1"
```

## 4. FPS 표시 코드 작성

최종 `src/main.rs`:

```rust
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

#[derive(Component)]
struct FpsText;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FrameTimeDiagnosticsPlugin::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, update_fps_text)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Text::new("FPS: ..."),
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
        FpsText,
    ));
}

fn update_fps_text(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    let Some(fps) = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|diagnostic| diagnostic.smoothed())
    else {
        return;
    };

    for mut text in &mut query {
        **text = format!("FPS: {fps:.1}");
    }
}
```

## 5. 코드 리뷰

### import

```rust
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
```

- `FrameTimeDiagnosticsPlugin`
  - Bevy가 프레임 시간과 FPS를 측정하게 해주는 진단 플러그인입니다.
- `DiagnosticsStore`
  - 측정된 진단값을 읽어오는 리소스입니다.
- `bevy::prelude::*`
  - `App`, `Commands`, `Component`, `Text`, `Node`, `Color`, `Startup`, `Update` 등 자주 쓰는 Bevy 타입을 가져옵니다.

### `FpsText` 컴포넌트

```rust
#[derive(Component)]
struct FpsText;
```

- FPS를 표시하는 텍스트 엔티티에 붙이는 마커 컴포넌트입니다.
- 나중에 `Query<&mut Text, With<FpsText>>`로 FPS 텍스트만 찾아서 수정하기 위해 사용합니다.

### 앱 구성

```rust
App::new()
    .add_plugins((DefaultPlugins, FrameTimeDiagnosticsPlugin::default()))
    .add_systems(Startup, setup)
    .add_systems(Update, update_fps_text)
    .run();
```

- `DefaultPlugins`
  - 창, 렌더링, 입력, 시간, UI, 텍스트 등 Bevy 기본 기능을 추가합니다.
- `FrameTimeDiagnosticsPlugin::default()`
  - FPS 측정 기능을 추가합니다.
- `Startup`
  - 앱 시작 시 한 번 실행할 시스템입니다.
  - 여기서는 `setup`을 연결했습니다.
- `Update`
  - 매 프레임 실행할 시스템입니다.
  - 여기서는 FPS 텍스트를 갱신하는 `update_fps_text`를 연결했습니다.
- `.run()`
  - Bevy 메인 루프를 시작합니다.

### `setup`

```rust
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Text::new("FPS: ..."),
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
        FpsText,
    ));
}
```

- `commands.spawn(Camera2d)`
  - 2D 카메라를 생성합니다.
  - UI 텍스트 표시에는 Bevy UI 시스템이 사용되지만, 이후 2D 오브젝트를 추가할 때도 사용할 수 있도록 카메라를 함께 생성했습니다.
- `Text::new("FPS: ...")`
  - 초기 표시 문자열입니다.
  - 실제 FPS 값은 몇 프레임 뒤 진단값이 준비되면 업데이트됩니다.
- `TextFont`
  - 텍스트 크기를 설정합니다.
- `TextColor(Color::WHITE)`
  - 글자색을 흰색으로 설정합니다.
- `Node`
  - UI 배치 정보입니다.
  - `PositionType::Absolute`와 `left`, `top`을 사용해 좌상단에서 12픽셀 떨어진 위치에 배치합니다.
- `FpsText`
  - 이 텍스트가 FPS 표시용임을 나타내는 마커입니다.

### `update_fps_text`

```rust
fn update_fps_text(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    let Some(fps) = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|diagnostic| diagnostic.smoothed())
    else {
        return;
    };

    for mut text in &mut query {
        **text = format!("FPS: {fps:.1}");
    }
}
```

- `diagnostics: Res<DiagnosticsStore>`
  - Bevy가 측정한 진단 데이터를 읽습니다.
- `FrameTimeDiagnosticsPlugin::FPS`
  - FPS 진단 항목을 가져옵니다.
- `diagnostic.smoothed()`
  - 순간값 대신 부드럽게 보정된 FPS 값을 가져옵니다.
- `else { return; }`
  - 앱 시작 직후에는 아직 FPS 값이 준비되지 않았을 수 있으므로, 값이 없으면 그냥 반환합니다.
- `Query<&mut Text, With<FpsText>>`
  - `FpsText`가 붙은 텍스트만 찾아 수정합니다.
- `**text = format!("FPS: {fps:.1}");`
  - 텍스트 내용을 `FPS: 60.0` 같은 형식으로 갱신합니다.

## 6. 빌드 확인

실행한 명령:

```text
cargo check
```

결과:

```text
Finished `dev` profile [unoptimized + debuginfo] target(s) in 1m 47s
```

즉, `bevy_test02`는 컴파일 검사를 통과했습니다.

## 7. 실행 방법

프로젝트 디렉터리:

```text
E:\works\works_rust\bevy_works\bevy_test02
```

실행 명령:

```text
cargo run
```

Windows에서 Vulkan validation 로그가 거슬리면 DX12 백엔드로 실행할 수 있습니다.

PowerShell:

```text
$env:WGPU_BACKEND="dx12"; cargo run
```

cmd:

```text
set WGPU_BACKEND=dx12 && cargo run
```

## 8. VSync 온/오프 설정

처음 실행했을 때 FPS가 60 근처로 고정되어 보이는 이유는 보통 VSync, 즉 수직 동기화가 켜져 있기 때문입니다.

VSync가 켜져 있으면 모니터 주사율에 맞춰 프레임 출력이 제한됩니다. 예를 들어 60Hz 모니터에서는 대체로 60 FPS 근처로 고정됩니다.

처음에는 실행 중 `V` 키로 `present_mode`를 바꾸는 방식으로 구현했습니다. 하지만 실제 테스트에서 Off 후 다시 On으로 바꿀 때 프로그램이 종료되는 문제가 발생했습니다.

원인은 런타임 중 `present_mode`를 변경하면 wgpu/그래픽 백엔드가 swapchain, 즉 화면 출력 체인을 다시 구성해야 하기 때문입니다. 이 과정은 Vulkan, DX12, 드라이버, 모니터 설정에 따라 불안정할 수 있습니다. 특히 현재 환경에서는 프로그램이 죽는 수준으로 나타났으므로 런타임 토글은 제거했습니다.

현재 버전에서는 앱 시작 시 환경변수로 VSync On/Off를 선택합니다.

- 기본값: VSync On
- `BEVY_TEST02_VSYNC=0`: VSync Off
- 화면 텍스트에 현재 FPS와 VSync 상태 표시

### 핵심 설정

```rust
DefaultPlugins.set(WindowPlugin {
    primary_window: Some(Window {
        title: "bevy_test02 - FPS / VSync Toggle".to_string(),
        present_mode,
        ..default()
    }),
    ..default()
})
```

- `PresentMode::Fifo`
  - VSync를 사용하는 모드입니다.
  - Vulkan에서 반드시 지원되는 안정적인 present mode입니다.
  - 일반적으로 모니터 주사율에 맞춰 FPS가 제한됩니다.

- `PresentMode::AutoNoVsync`
  - VSync를 사용하지 않는 자동 선택 모드입니다.
  - 가능한 경우 VSync 없는 present mode를 선택합니다.
  - FPS가 60을 넘을 수 있지만 화면 찢어짐, tearing이 보일 수 있습니다.

### 환경변수 기반 설정

현재 코드는 앱 시작 시 `BEVY_TEST02_VSYNC` 환경변수를 읽습니다.

```rust
let vsync_enabled = std::env::var("BEVY_TEST02_VSYNC")
    .map(|value| !matches!(value.as_str(), "0" | "false" | "False" | "off" | "Off"))
    .unwrap_or(true);

let present_mode = if vsync_enabled {
    PresentMode::Fifo
} else {
    PresentMode::AutoNoVsync
};
```

- 환경변수가 없으면 기본값은 VSync On입니다.
- `BEVY_TEST02_VSYNC=0`이면 VSync Off로 시작합니다.
- 런타임 중 `present_mode`를 바꾸지 않으므로 swapchain 재구성으로 인한 종료 문제를 피합니다.

### 화면 표시

VSync On 상태에서는 다음처럼 표시됩니다.

```text
FPS: 60.0
VSync: On
Restart with BEVY_TEST02_VSYNC=0 to disable
```

VSync Off로 시작하면 다음처럼 표시될 수 있습니다.

```text
FPS: 240.0
VSync: Off
Restart without BEVY_TEST02_VSYNC=0 to enable
```

단, 실제 FPS는 GPU, 모니터, 드라이버, 전원 설정, 백엔드, 창 포커스 상태에 따라 달라질 수 있습니다.

### 실행 방법

기본 실행, VSync On:

```text
cargo run
```

PowerShell에서 VSync Off로 실행:

```text
$env:BEVY_TEST02_VSYNC="0"; cargo run
```

PowerShell에서 다시 VSync On으로 실행하려면 환경변수를 제거하거나 새 터미널에서 실행합니다.

```text
Remove-Item Env:BEVY_TEST02_VSYNC
cargo run
```

cmd에서 VSync Off로 실행:

```text
set BEVY_TEST02_VSYNC=0 && cargo run
```

### 빌드 확인

런타임 토글 제거 후 다시 확인했습니다.

```text
cargo check
```

결과:

```text
Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.28s
```

즉, 수정 후에도 컴파일 검사를 통과했습니다.

## 9. 다음 단계 제안

다음 단계로는 아래 기능을 붙일 수 있습니다.

1. 배경색 변경
2. 움직이는 2D 사각형 추가
3. 키보드 입력으로 사각형 이동
4. FPS 외에 프레임 시간(ms) 표시
5. VSync 상태를 더 보기 좋은 UI 패널로 표시
