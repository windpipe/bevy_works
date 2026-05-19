# bevy_test04 개발 기록

## 1. 목표

`bevy_test04`에서는 싱글 플레이 퐁 게임을 구현했습니다.

요구사항:

- 싱글 모드 퐁 게임
- 한쪽에만 플레이어가 움직이는 패드 존재
- 공이 벽과 패드에 튕김
- F10 키를 누르면 옵션 팝업 표시
- 옵션 팝업에서 OK / Cancel 선택
- OK를 누르면 변경사항을 config 파일에 저장
- 저장된 옵션은 다음 실행 때 적용

## 2. 프로젝트 생성

작업 디렉터리:

```text
E:\works\works_rust\bevy_works
```

실행한 명령:

```text
cargo new bevy_test04
```

## 3. Bevy 의존성 추가

프로젝트 디렉터리:

```text
E:\works\works_rust\bevy_works\bevy_test04
```

실행한 명령:

```text
cargo add bevy
```

추가된 Bevy 버전:

```text
bevy v0.18.1
```

## 4. 구현된 게임 기능

### 조작

- `W` 또는 `ArrowUp`: 패드 위로 이동
- `S` 또는 `ArrowDown`: 패드 아래로 이동
- `F10`: 옵션 팝업 열기/닫기

### 게임 규칙

- 왼쪽에 플레이어 패드가 있습니다.
- 공은 오른쪽 벽, 위쪽 벽, 아래쪽 벽에 튕깁니다.
- 공이 왼쪽 패드에 닿으면 오른쪽으로 튕깁니다.
- 패드에 맞출 때마다 점수가 1 증가합니다.
- 공을 놓치면 공은 중앙으로 리셋되고 점수는 0이 됩니다.

## 5. 옵션 팝업

F10을 누르면 화면 중앙에 옵션 팝업이 표시됩니다.

옵션 항목:

- VSync On/Off
- Fullscreen On/Off
- Resolution 변경
  - `1280x720`
  - `1600x900`
  - `1920x1080`
  - `2560x1440`
- Renderer 변경
  - `Auto`
  - `DX12`
  - `Vulkan`

버튼:

- `Toggle VSync`
- `Toggle Fullscreen`
- `Next Resolution`
- `Next Renderer`
- `OK - Save for next run`
- `Cancel`

### OK 동작

`OK - Save for next run`을 누르면 현재 팝업에서 선택한 옵션이 설정 파일에 저장됩니다.

단, VSync, Fullscreen, Resolution, Renderer는 현재 실행 중인 앱에 즉시 적용하지 않습니다.

이유:

- VSync와 렌더러 변경은 swapchain 또는 렌더러 초기화와 관련되어 있습니다.
- `bevy_test02`, `bevy_test03` 실험에서 실행 중 변경 시 프로그램이 죽을 수 있음을 확인했습니다.
- 따라서 안정성을 위해 다음 실행 때 적용하는 방식으로 구현했습니다.

### Cancel 동작

`Cancel`을 누르면 팝업에서 선택한 변경사항은 버려지고 설정 파일에도 저장하지 않습니다.

## 6. 설정 파일

설정 파일 이름:

```text
bevy_test04_settings.txt
```

저장 위치:

```text
E:\works\works_rust\bevy_works\bevy_test04\bevy_test04_settings.txt
```

예시:

```text
vsync=true
fullscreen=false
resolution=1280x720
renderer=Auto
```

이 파일은 실행 시 읽히고, OK를 누를 때 갱신됩니다.

## 7. 모듈 구조

초기 구현은 `src/main.rs` 한 파일에 들어 있었지만, 이전 버전과 같은 방향으로 모듈화했습니다.

현재 파일 구조:

```text
src/
├── main.rs
├── settings.rs
├── game.rs
└── options_popup.rs
```

### `main.rs`

앱 시작과 Bevy 플러그인 조립만 담당합니다.

담당:

- 설정 로드
- `WGPU_BACKEND` 환경변수 설정
- `WindowPlugin` 설정
- `FrameTimeDiagnosticsPlugin` 추가
- `PongGamePlugin` 추가
- `OptionsPopupPlugin` 추가

### `settings.rs`

그래픽 설정과 config 파일 저장/로드를 담당합니다.

주요 구성 요소:

- `GraphicsSettings`
  - VSync, Fullscreen, Resolution, Renderer 설정 저장
- `RendererBackend`
  - `Auto`, `DX12`, `Vulkan` 선택
- `ResolutionOption`
  - 지원 해상도 목록 관리
- `GraphicsSettings::load()`
  - config 파일에서 설정 읽기
- `GraphicsSettings::save()`
  - config 파일에 설정 저장
- `settings_description()` / `on_off()`
  - UI 표시용 문자열 생성

### `game.rs`

퐁 게임 본체를 담당합니다.

주요 구성 요소:

- `PongGamePlugin`
  - 게임 시스템을 Bevy 앱에 등록하는 플러그인
- `Paddle`
  - 플레이어 패드 컴포넌트
- `Ball`
  - 공 컴포넌트와 속도 저장
- `GameScore`
  - 현재 점수 리소스
- `setup_game`
  - 카메라, 경기장, 패드, 공, 점수 UI 생성
- `move_paddle`
  - 플레이어 입력 처리
- `move_ball`
  - 공 이동과 충돌 처리
- `update_score_text`
  - 점수 UI 갱신
- `update_fps_text`
  - FPS/조작 안내 UI 갱신

### `options_popup.rs`

F10 옵션 팝업을 담당합니다.

주요 구성 요소:

- `OptionsPopupPlugin`
  - 옵션 팝업 시스템을 Bevy 앱에 등록하는 플러그인
- `OptionsPopupState`
  - 팝업 표시 여부와 draft 설정 저장
- `OptionsButton`
  - 옵션 팝업 버튼 종류
- `toggle_options_popup`
  - F10으로 팝업 열기/닫기
- `handle_options_buttons`
  - OK/Cancel 및 옵션 변경 버튼 처리
- `update_options_text`
  - draft 설정 변경 시 팝업 텍스트 갱신
- `update_button_colors`
  - 버튼 hover/press 색상 처리

## 8. 주요 시스템

### `setup`

앱 시작 시 한 번 실행됩니다.

담당:

- 2D 카메라 생성
- 배경 생성
- 중앙 라인 생성
- 플레이어 패드 생성
- 공 생성
- 점수 UI 생성
- FPS/조작 안내 UI 생성

### `move_paddle`

매 프레임 키보드 입력을 읽어 패드를 위아래로 이동합니다.

패드는 경기장 밖으로 나가지 않도록 y 좌표를 clamp합니다.

### `move_ball`

매 프레임 공 위치를 갱신합니다.

담당:

- 공 이동
- 위/아래 벽 충돌
- 오른쪽 벽 충돌
- 패드 충돌
- 패드 충돌 시 점수 증가
- 공을 놓쳤을 때 공/점수 리셋

`move_ball`은 패드의 `Transform`을 읽고 공의 `Transform`을 수정합니다. 둘 다 같은 `Transform` 컴포넌트를 다루므로 Bevy ECS 입장에서는 쿼리 충돌 가능성이 있습니다.

그래서 다음처럼 `Without<T>` 필터로 두 쿼리가 서로 다른 엔티티 집합임을 명시했습니다.

```rust
paddle_query: Query<&Transform, (With<Paddle>, Without<Ball>)>,
mut ball_query: Query<(&mut Transform, &mut Ball), Without<Paddle>>,
```

이렇게 하지 않으면 실행 시 Bevy의 B0001 쿼리 충돌 에러가 발생할 수 있습니다.

### `toggle_options_popup`

F10 입력을 감지해 옵션 팝업을 열거나 닫습니다.

팝업을 열 때는 현재 설정을 draft로 복사합니다.

### `handle_options_buttons`

옵션 팝업 버튼 클릭을 처리합니다.

- 토글/변경 버튼은 draft 설정만 변경
- OK는 draft를 config 파일에 저장
- Cancel은 draft를 버리고 팝업 닫기

### `update_options_text`

옵션 draft가 바뀔 때 팝업 텍스트를 갱신합니다.

### `update_fps_text`

`FrameTimeDiagnosticsPlugin`에서 FPS 값을 읽어 화면에 표시합니다.

## 9. 빌드 확인

실행한 명령:

```text
cargo check
```

결과:

```text
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.96s
```

즉, `bevy_test04`는 컴파일 검사를 통과했습니다.

## 10. 실행 방법

프로젝트 디렉터리:

```text
E:\works\works_rust\bevy_works\bevy_test04
```

실행 명령:

```text
cargo run
```

## 11. 다음 단계 제안

다음 단계로 개선할 수 있는 내용:

1. 옵션 팝업을 마우스뿐 아니라 키보드로 조작 가능하게 만들기
2. 공 속도/패드 크기 난이도 옵션 추가
3. 사운드 효과 추가
4. 오른쪽에 AI 패드 추가해서 실제 퐁 게임 형태로 확장
5. 게임 상태를 메뉴/플레이/일시정지로 분리
