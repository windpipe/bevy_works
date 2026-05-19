# bevy_test01 개발 기록

## 1. 프로젝트 생성 요청

사용자 요청:

- Rust 새 프로젝트 생성
- 프로젝트 이름: `bevy_test01`
- Cargo 사용
- 가장 단순한 Bevy 프로젝트 구성

## 2. 프로젝트 생성

작업 디렉터리:

- `E:\works\works_rust\bevy_works`

실행한 명령:

```text
cargo new bevy_test01
```

결과:

- 바이너리 애플리케이션 패키지 `bevy_test01` 생성
- 기본 파일 생성:
  - `Cargo.toml`
  - `src/main.rs`
  - `.gitignore`

초기 `src/main.rs`는 Cargo 기본 템플릿이었습니다.

```rust
fn main() {
    println!("Hello, world!");
}
```

## 3. Bevy 의존성 추가

프로젝트 디렉터리:

- `E:\works\works_rust\bevy_works\bevy_test01`

실행한 명령:

```text
cargo add bevy
```

처음에는 Cargo 패키지 캐시 파일 잠금 때문에 대기 상태가 발생했습니다.

```text
Blocking waiting for file lock on package cache
```

이후 다시 실행하여 Bevy 의존성을 정상적으로 추가했습니다.

추가된 Bevy 버전:

- `bevy v0.18.1`

현재 `Cargo.toml` 내용:

```toml
[package]
name = "bevy_test01"
version = "0.1.0"
edition = "2024"

[dependencies]
bevy = "0.18.1"
```

## 4. 가장 단순한 Bevy 앱으로 소스 수정

기존 Cargo 기본 출력 코드:

```rust
fn main() {
    println!("Hello, world!");
}
```

수정 후 `src/main.rs`:

```rust
use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}
```

## 5. 빌드 확인

실행한 명령:

```text
cargo check
```

결과:

```text
Finished `dev` profile [unoptimized + debuginfo] target(s) in 3m 37s
```

즉, 현재 프로젝트는 컴파일 검사를 통과했습니다.

## 6. 현재 프로젝트 구조

```text
bevy_test01/
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── DEVELOPMENT_LOG.md
├── src/
│   └── main.rs
└── target/
```

## 7. 소스 리뷰

### `Cargo.toml`

```toml
[package]
name = "bevy_test01"
version = "0.1.0"
edition = "2024"

[dependencies]
bevy = "0.18.1"
```

리뷰:

- `name = "bevy_test01"`
  - Cargo 패키지 이름입니다.
  - 생성한 프로젝트 이름과 일치합니다.

- `version = "0.1.0"`
  - Cargo 기본 초기 버전입니다.
  - 학습용 또는 초기 프로젝트로 적절합니다.

- `edition = "2024"`
  - 현재 Rust 2024 에디션을 사용합니다.
  - 최신 Rust 문법과 호환됩니다.

- `bevy = "0.18.1"`
  - Bevy 게임 엔진 의존성입니다.
  - `cargo add bevy`로 추가되었습니다.
  - 기본 기능이 많이 활성화되어 창 생성, 렌더링, 입력, 오디오 등 Bevy 기본 플러그인 사용이 가능합니다.

### `src/main.rs`

```rust
use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}
```

리뷰:

- `use bevy::prelude::*;`
  - Bevy에서 자주 쓰는 타입과 함수들을 한 번에 가져옵니다.
  - `App`, `DefaultPlugins` 등을 별도 경로 없이 사용할 수 있게 해줍니다.

- `fn main()`
  - Rust 프로그램의 시작 지점입니다.
  - `cargo run`을 실행하면 이 함수가 호출됩니다.

- `App::new()`
  - 새로운 Bevy 앱 인스턴스를 생성합니다.
  - Bevy의 ECS, 스케줄, 플러그인 시스템의 중심이 되는 객체입니다.

- `.add_plugins(DefaultPlugins)`
  - Bevy의 기본 플러그인 묶음을 앱에 추가합니다.
  - 기본 창 생성, 렌더링, 입력 처리, 시간 관리, 에셋 관리, 로그 시스템 등이 포함됩니다.
  - 이 한 줄 때문에 실행 시 기본 Bevy 창이 뜰 수 있습니다.

- `.run()`
  - Bevy 앱의 메인 루프를 시작합니다.
  - 창이 닫힐 때까지 앱이 계속 실행됩니다.

## 8. 현재 상태 요약

현재 `bevy_test01`은 가장 단순한 형태의 Bevy 애플리케이션입니다.

특징:

- 별도 엔티티 없음
- 카메라 없음
- 스프라이트나 3D 오브젝트 없음
- 기본 Bevy 플러그인만 실행
- 실행하면 Bevy 기본 창을 띄우는 최소 구성

실행 명령:

```text
cargo run
```

실행 위치:

```text
E:\works\works_rust\bevy_works\bevy_test01
```

## 9. Bevy의 렌더 루프 이해

Bevy에는 전통적인 의미의 `while running { update(); render(); }` 형태의 렌더 루프가 사용자 코드에 직접 드러나지 않습니다.

현재 프로젝트의 핵심 코드는 다음과 같습니다.

```rust
use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}
```

여기서 Bevy 앱의 메인 루프를 시작하는 부분은 마지막의 `.run()`입니다.

```rust
App::new().add_plugins(DefaultPlugins).run();
```

전통적인 게임 루프는 보통 다음과 같은 구조입니다.

```rust
while running {
    handle_input();
    update_game();
    render();
}
```

하지만 Bevy에서는 이 루프를 직접 작성하지 않습니다. 대신 Bevy 내부의 앱 러너가 루프를 실행하고, 개발자는 그 루프 안에서 실행될 시스템을 등록합니다.

개념적으로 Bevy 내부 루프는 다음과 비슷하게 동작한다고 볼 수 있습니다.

```rust
loop {
    process_window_events();
    run_update_schedule();
    extract_render_data();
    prepare_render_resources();
    queue_render_commands();
    render_frame();
}
```

단, 위 코드는 설명을 위한 의사 코드이며 실제 사용자 프로젝트에 직접 작성하는 코드는 아닙니다.

### 전통적인 루프와 Bevy 방식 비교

| 전통적인 방식 | Bevy 방식 |
| --- | --- |
| `while` 루프를 직접 작성 | `.run()`이 내부 루프 실행 |
| `update()` 직접 호출 | `Update` 스케줄에 system 등록 |
| `render()` 직접 호출 | Bevy render plugin이 내부에서 처리 |
| 입력 처리 직접 호출 | input resource를 system에서 읽음 |
| delta time 직접 계산 | `Time` resource 사용 |

### `DefaultPlugins`와 렌더링

현재 코드의 `.add_plugins(DefaultPlugins)`는 Bevy의 기본 플러그인 묶음을 추가합니다.

이 안에는 창 생성, 이벤트 처리, 렌더링, 입력, 시간, 에셋, 로그 등 여러 기능이 포함됩니다.

렌더링과 관련된 대표적인 역할은 다음과 같습니다.

- 창 생성과 OS 이벤트 처리
- GPU 렌더링 시스템 준비
- 렌더 월드 관리
- 2D/3D 렌더링 파이프라인 제공
- UI, sprite, mesh 등을 그릴 수 있는 기반 제공

즉, 현재 프로젝트에는 사용자가 직접 작성한 `render()` 함수는 없지만, Bevy 내부의 렌더 시스템은 `.run()` 이후 계속 실행됩니다.

### 개발자는 어디에 코드를 넣는가

전통적인 `update()`에 해당하는 코드는 Bevy의 `Update` 스케줄에 시스템으로 등록합니다.

예시:

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, update)
        .run();
}

fn update() {
    println!("매 프레임 실행");
}
```

위 코드에서 `update` 함수는 Bevy 메인 루프 안에서 매 프레임 실행됩니다.

초기화 코드는 보통 `Startup` 스케줄에 등록합니다.

예시:

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup() {
    println!("앱 시작 시 한 번 실행");
}

fn update() {
    println!("매 프레임 실행");
}
```

### 현재 프로젝트의 상태

현재 `bevy_test01`은 다음 코드만 가지고 있습니다.

```rust
App::new().add_plugins(DefaultPlugins).run();
```

따라서 현재 상태는 다음과 같습니다.

- Bevy 앱은 생성됨
- 기본 플러그인은 등록됨
- 내부 메인 루프와 렌더 시스템은 실행됨
- 하지만 사용자가 만든 엔티티는 없음
- 카메라도 없음
- 스프라이트나 3D 오브젝트도 없음

즉, 렌더 루프는 돌고 있지만 사용자가 만든 렌더 대상은 아직 없는 상태입니다.

### 한 줄 요약

Bevy에서 전통적인 렌더 루프는 직접 작성하지 않습니다.

현재 프로젝트에서는 다음 코드의 `.run()` 안에서 Bevy가 내부적으로 메인 루프와 렌더 루프를 관리합니다.

```rust
App::new().add_plugins(DefaultPlugins).run();
```

개발자는 루프를 직접 작성하는 대신 다음 작업을 합니다.

- `Startup`에 초기화 시스템 등록
- `Update`에 매 프레임 실행 시스템 등록
- 엔티티와 컴포넌트 생성
- Bevy의 렌더 시스템이 해당 엔티티를 자동으로 그리게 함

## 10. 다음 단계 제안

원한다면 다음 단계로 확장할 수 있습니다.

1. 창 제목과 크기 설정
2. 배경색 변경
3. 2D 카메라 추가
4. 간단한 사각형 또는 원 스프라이트 표시
5. 키보드 입력으로 오브젝트 움직이기
6. 게임 상태 관리 추가
