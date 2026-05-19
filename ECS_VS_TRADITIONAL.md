# Bevy ECS 방식과 Unity식 전통 구조 비교

## 1. 이 문서의 목적

`bevy_test04`를 만들면서 Bevy ECS의 쿼리 충돌 에러를 경험했습니다.

문제가 되었던 핵심은 다음과 같은 구조였습니다.

```rust
fn move_ball(
    time: Res<Time>,
    mut score: ResMut<GameScore>,
    paddle_query: Query<&Transform, With<Paddle>>,
    mut ball_query: Query<(&mut Transform, &mut Ball)>,
) {
    // ...
}
```

이 시스템은 패드의 `Transform`은 읽고, 공의 `Transform`은 수정합니다.

사람이 보기에는 패드와 공은 서로 다른 엔티티이므로 문제가 없어 보입니다.
하지만 Bevy ECS는 다음 가능성을 배제할 수 없습니다.

```text
어떤 엔티티가 Transform + Paddle + Ball을 동시에 가지고 있을 수도 있지 않나?
```

그 경우 같은 `Transform`을 한 쿼리에서는 읽고, 다른 쿼리에서는 수정하게 됩니다.
Bevy는 이런 접근을 안전하지 않다고 판단하고 `B0001` 에러를 발생시킵니다.

이를 해결하기 위해 다음처럼 `Without<T>` 필터를 추가했습니다.

```rust
fn move_ball(
    time: Res<Time>,
    mut score: ResMut<GameScore>,
    paddle_query: Query<&Transform, (With<Paddle>, Without<Ball>)>,
    mut ball_query: Query<(&mut Transform, &mut Ball), Without<Paddle>>,
) {
    // ...
}
```

이 문서는 다음 질문에 답하기 위해 작성했습니다.

> 이런 식으로 `Without<T>`까지 써가면서 관리하는 방식이 Unity 같은 전통적인 방식보다 효율이 좋은가?
> 코드가 더 컴팩트한가?

결론부터 말하면 다음과 같습니다.

> Bevy의 ECS 방식은 작은 게임에서는 전통적인 Unity 방식보다 더 장황하고 덜 직관적일 수 있습니다.
> 하지만 규모가 커질수록 데이터 접근이 명확해지고, 병렬 실행과 성능 최적화에 유리합니다.
> 코드가 항상 더 컴팩트한 것은 아니지만, 구조가 잘 잡히면 시스템 단위의 재사용성과 확장성이 좋아집니다.

---

## 2. Unity식 전통 구조의 특징

Unity의 전통적인 개발 방식은 대체로 다음 개념을 중심으로 합니다.

- `GameObject`
- `Component`
- `MonoBehaviour`
- `Transform`
- `Update()`
- 직접 참조

예를 들어 퐁 게임에서 공과 패드를 처리한다면 Unity식 코드는 개념적으로 다음처럼 보일 수 있습니다.

```csharp
void Update() {
    paddle.transform.position += paddleVelocity * Time.deltaTime;
    ball.transform.position += ballVelocity * Time.deltaTime;

    if (CheckCollision(ball, paddle)) {
        ballVelocity.x *= -1;
    }
}
```

이 방식은 매우 직관적입니다.

- 이 오브젝트를 움직인다.
- 저 오브젝트와 충돌을 검사한다.
- 충돌하면 속도를 바꾼다.

작은 게임이나 빠른 프로토타입에서는 Unity식 구조가 훨씬 이해하기 쉽고 빠르게 작성됩니다.

### 2.1 Unity식 구조의 장점

Unity식 구조의 장점은 다음과 같습니다.

1. 오브젝트 중심이라 이해하기 쉽습니다.
2. 에디터에서 오브젝트와 컴포넌트를 직접 연결할 수 있습니다.
3. `Update()`에 코드를 작성하면 매 프레임 실행됩니다.
4. 작은 게임에서는 코드가 짧고 직관적입니다.
5. 디버깅할 때 씬의 오브젝트를 바로 확인할 수 있습니다.

예를 들어 플레이어 스크립트 안에서 다음처럼 작성할 수 있습니다.

```csharp
public class Player : MonoBehaviour {
    void Update() {
        float y = Input.GetAxis("Vertical");
        transform.position += new Vector3(0, y, 0) * speed * Time.deltaTime;
    }
}
```

이것은 초보자에게 매우 자연스럽습니다.

### 2.2 Unity식 구조의 단점

하지만 규모가 커질수록 다음 문제가 생길 수 있습니다.

1. 여러 스크립트가 같은 오브젝트를 동시에 수정할 수 있습니다.
2. `Update()` 실행 순서에 따라 결과가 달라질 수 있습니다.
3. 참조가 `null`이 되는 문제가 발생할 수 있습니다.
4. 오브젝트가 파괴된 뒤 참조가 남아 있을 수 있습니다.
5. 수많은 오브젝트를 처리할 때 성능 최적화가 어려울 수 있습니다.
6. 데이터가 오브젝트 단위로 흩어져 있어 CPU 캐시 효율이 떨어질 수 있습니다.
7. 병렬 실행을 자동으로 판단하기 어렵습니다.

예를 들어 하나의 캐릭터 위치를 다음 스크립트들이 모두 수정한다고 생각해봅니다.

- `PlayerMovement`
- `KnockbackSystem`
- `PhysicsSync`
- `AnimationRootMotion`
- `CameraFollowTargetAdjuster`

이 경우 실제 위치는 어떤 스크립트가 먼저 실행되는지에 따라 달라질 수 있습니다.
Unity에서도 실행 순서를 지정할 수 있지만, 프로젝트가 커질수록 관리 비용이 커집니다.

---

## 3. Bevy ECS 방식의 특징

Bevy는 ECS, 즉 Entity Component System 구조를 중심으로 동작합니다.

ECS의 핵심 요소는 다음 세 가지입니다.

### 3.1 Entity

`Entity`는 단순한 ID입니다.

예를 들어 다음과 같은 엔티티가 있을 수 있습니다.

```text
Entity(1)
Entity(2)
Entity(3)
```

Entity 자체는 데이터가 아닙니다.
데이터는 Component에 들어 있습니다.

### 3.2 Component

`Component`는 엔티티에 붙는 데이터 조각입니다.

예를 들면 다음과 같습니다.

```rust
#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball {
    velocity: Vec2,
}
```

또는 Bevy 기본 컴포넌트인 `Transform`, `Sprite`, `Text` 등도 있습니다.

엔티티는 여러 컴포넌트의 조합으로 표현됩니다.

```text
Entity A: Transform + Sprite + Paddle
Entity B: Transform + Sprite + Ball
Entity C: Transform + Camera2d
```

### 3.3 System

`System`은 컴포넌트 데이터를 읽거나 수정하는 함수입니다.

예를 들어 패드를 움직이는 시스템은 다음과 같습니다.

```rust
fn move_paddle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Paddle>>,
) {
    // Paddle이 붙은 엔티티의 Transform만 수정
}
```

Bevy는 등록된 시스템들을 스케줄에 따라 실행합니다.

- `Startup`: 앱 시작 시 한 번 실행
- `Update`: 매 프레임 실행

---

## 4. 왜 `Without<T>`가 필요한가?

문제가 된 코드는 패드의 위치를 읽고 공의 위치를 수정하는 시스템이었습니다.

처음에는 다음과 같은 구조였습니다.

```rust
fn move_ball(
    paddle_query: Query<&Transform, With<Paddle>>,
    mut ball_query: Query<(&mut Transform, &mut Ball)>,
) {
    // ...
}
```

이 코드는 사람에게는 명확합니다.

- `paddle_query`는 패드만 가져온다.
- `ball_query`는 공만 가져온다.
- 패드와 공은 다른 엔티티다.

하지만 Bevy ECS 입장에서는 다릅니다.

`paddle_query`는 다음 조건을 만족하는 엔티티를 찾습니다.

```text
Transform이 있고 Paddle이 있는 엔티티
```

`ball_query`는 다음 조건을 만족하는 엔티티를 찾습니다.

```text
Transform이 있고 Ball이 있는 엔티티
```

여기서 Bevy는 다음 엔티티의 가능성을 배제할 수 없습니다.

```text
Entity X: Transform + Paddle + Ball
```

이런 엔티티가 있다면 양쪽 쿼리에 모두 잡힙니다.
그 결과 같은 `Transform`에 대해 다음 접근이 동시에 발생할 수 있습니다.

- `paddle_query`: `&Transform` 읽기
- `ball_query`: `&mut Transform` 수정

Rust의 기본 규칙은 다음과 같습니다.

> 하나의 값에 대해 동시에 여러 개의 읽기 참조는 가능하다.
> 하나의 값에 대해 하나의 수정 참조는 가능하다.
> 읽기 참조와 수정 참조가 동시에 존재하면 안 된다.

Bevy ECS도 이 원칙을 시스템 파라미터 수준에서 지키려고 합니다.
그래서 쿼리 충돌 가능성이 있으면 실행을 막습니다.

### 4.1 해결 방법: `Without<T>`

수정 후 코드는 다음과 같습니다.

```rust
fn move_ball(
    paddle_query: Query<&Transform, (With<Paddle>, Without<Ball>)>,
    mut ball_query: Query<(&mut Transform, &mut Ball), Without<Paddle>>,
) {
    // ...
}
```

이제 `paddle_query`는 다음 조건을 의미합니다.

```text
Transform이 있고 Paddle이 있으며 Ball은 없는 엔티티
```

`ball_query`는 다음 조건을 의미합니다.

```text
Transform과 Ball이 있으며 Paddle은 없는 엔티티
```

따라서 두 쿼리는 절대 같은 엔티티를 잡을 수 없습니다.
Bevy는 이 정보를 보고 안전하다고 판단합니다.

### 4.2 `Without<T>`는 관리 비용인가?

네, 관리 비용이 맞습니다.
하지만 목적이 있습니다.

`Without<T>`는 개발자의 머릿속에 있던 전제를 코드에 명시하는 것입니다.

Unity식 사고에서는 다음처럼 생각합니다.

```text
패드는 패드고 공은 공이니까 당연히 다르다.
```

Bevy ECS에서는 다음처럼 코드로 적습니다.

```rust
With<Paddle>, Without<Ball>
With<Ball>, Without<Paddle>
```

즉, ECS에서는 데이터 접근 조건을 더 명확하게 선언합니다.
그 대가로 코드가 조금 더 길어집니다.

---

## 5. `Without<T>` 없이 해결하는 다른 방법: `ParamSet`

`Without<T>`만이 유일한 해결책은 아닙니다.

같은 컴포넌트를 여러 쿼리에서 다뤄야 하는데, 쿼리 대상이 명확히 분리되지 않거나 순서대로 접근해야 한다면 `ParamSet`을 사용할 수 있습니다.

개념적으로 다음과 같습니다.

```rust
fn move_ball(
    mut set: ParamSet<(
        Query<&Transform, With<Paddle>>,
        Query<(&mut Transform, &mut Ball)>,
    )>,
) {
    // 먼저 set.p0()로 패드 위치를 읽고
    // 그 다음 set.p1()로 공을 수정한다
}
```

`ParamSet`은 Bevy에게 다음을 알려줍니다.

```text
이 쿼리들이 충돌할 수 있다는 것을 알고 있다.
하지만 내가 접근 순서를 직접 관리하겠다.
```

이번 퐁 게임에서는 패드와 공이 명확히 다른 엔티티이므로 `Without<T>`가 더 적절합니다.

---

## 6. Bevy ECS는 더 효율적인가?

성능 관점에서는 Bevy ECS가 유리한 경우가 많습니다.
특히 많은 엔티티를 반복적으로 처리하는 경우에 강합니다.

예를 들어 총알 10,000개를 움직인다고 생각해봅니다.

ECS에서는 다음처럼 처리할 수 있습니다.

```rust
fn move_bullets(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity), With<Bullet>>,
) {
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.0 * time.delta_secs();
    }
}
```

이 시스템은 `Bullet` 컴포넌트를 가진 모든 엔티티를 한 번에 처리합니다.

### 6.1 데이터 중심 처리

전통적인 OOP 방식에서는 보통 오브젝트가 여러 데이터를 함께 가집니다.

```text
Enemy {
    Transform
    Health
    AI
    RenderData
    AudioSource
}
```

메모리에는 여러 종류의 데이터가 섞여 있을 수 있습니다.

반면 ECS에서는 같은 컴포넌트 타입끼리 모아서 처리하기 쉽습니다.

```text
Transform: [Transform, Transform, Transform, ...]
Velocity:  [Velocity, Velocity, Velocity, ...]
Health:    [Health, Health, Health, ...]
```

CPU는 메모리를 순차적으로 읽을 때 빠릅니다.
따라서 같은 종류의 데이터를 대량으로 처리할 때 ECS는 캐시 효율이 좋아질 수 있습니다.

### 6.2 병렬 실행 가능성

Bevy는 시스템들이 어떤 데이터에 접근하는지 알고 있습니다.

예를 들어 다음 두 시스템이 있다고 가정합니다.

```rust
fn move_bullets(mut bullets: Query<&mut Transform, With<Bullet>>) {}
fn update_ui(mut texts: Query<&mut Text>) {}
```

첫 번째 시스템은 `Transform`을 수정합니다.
두 번째 시스템은 `Text`를 수정합니다.

두 시스템은 서로 다른 컴포넌트에 접근하므로 Bevy가 병렬로 실행할 수 있습니다.

반대로 두 시스템이 모두 `Transform`을 수정한다면 Bevy는 충돌 가능성을 보고 순서를 조정하거나 병렬 실행을 피합니다.

즉, Bevy ECS의 시스템 파라미터는 단순한 함수 인자가 아니라, 스케줄러에게 주는 데이터 접근 정보입니다.

---

## 7. Bevy ECS는 코드가 더 컴팩트한가?

항상 그렇지는 않습니다.

작은 게임에서는 오히려 Unity식 코드가 더 짧고 직관적입니다.

Unity식 사고에서는 다음처럼 작성할 수 있습니다.

```csharp
ball.position += velocity * Time.deltaTime;
if (Collides(ball, paddle)) {
    velocity.x *= -1;
}
```

Bevy ECS에서는 시스템 파라미터가 더 길어질 수 있습니다.

```rust
fn move_ball(
    time: Res<Time>,
    mut score: ResMut<GameScore>,
    paddle_query: Query<&Transform, (With<Paddle>, Without<Ball>)>,
    mut ball_query: Query<(&mut Transform, &mut Ball), Without<Paddle>>,
) {
    // ...
}
```

이것만 보면 Bevy 쪽이 더 장황합니다.

하지만 엔티티가 많아지고 동작이 공통화되면 ECS가 더 깔끔해질 수 있습니다.

예를 들어 여러 종류의 적이 있다고 생각해봅니다.

Unity/OOP에서는 다음처럼 클래스가 늘어날 수 있습니다.

```text
Enemy
FastEnemy
FlyingEnemy
BossEnemy
PoisonEnemy
FlyingFastEnemy
```

ECS에서는 컴포넌트를 조합할 수 있습니다.

```text
Enemy
Fast
Flying
Boss
Poison
Health
Velocity
```

그러면 엔티티는 다음처럼 조합됩니다.

```text
Entity A: Enemy + Health + Velocity
Entity B: Enemy + Fast + Health + Velocity
Entity C: Enemy + Flying + Health + Velocity
Entity D: Enemy + Flying + Fast + Poison + Health + Velocity
```

이 경우 상속 구조보다 조합 구조가 더 유연해질 수 있습니다.

### 7.1 코드가 길어지는 부분

Bevy ECS에서 코드가 길어지는 부분은 다음입니다.

- `Component` 마커 선언
- `Resource` 선언
- `Query` 타입
- `With<T>` 필터
- `Without<T>` 필터
- 시스템 등록 코드
- 플러그인 구조

### 7.2 코드가 짧아질 수 있는 부분

반대로 코드가 짧아질 수 있는 부분은 다음입니다.

- 많은 엔티티를 같은 시스템으로 일괄 처리할 때
- 상속 대신 컴포넌트 조합으로 동작을 표현할 때
- 상태별 시스템을 분리할 때
- 특정 컴포넌트를 가진 엔티티만 자동으로 처리할 때
- 데이터 중심으로 공통 로직을 만들 때

---

## 8. 안전성 관점의 차이

Bevy ECS는 데이터 접근 안전성에 매우 민감합니다.
이번에 발생한 `B0001` 에러는 그 예입니다.

처음에는 불편해 보입니다.
하지만 조용히 이상한 동작을 하는 것보다, 실행 초기에 문제를 알려주는 것이 더 안전할 수 있습니다.

Unity식 구조에서는 여러 스크립트가 같은 오브젝트를 수정해도 엔진이 항상 막아주지는 않습니다.
결과가 이상하면 개발자가 직접 원인을 찾아야 합니다.

Bevy는 시스템 파라미터만 보고도 다음을 판단합니다.

- 어떤 시스템이 어떤 컴포넌트를 읽는가
- 어떤 시스템이 어떤 컴포넌트를 수정하는가
- 같은 시스템 안에서 쿼리가 충돌할 가능성이 있는가
- 시스템들을 병렬로 돌릴 수 있는가
- 순서를 강제해야 하는가

그래서 Bevy는 더 엄격합니다.
엄격하다는 것은 초반에는 귀찮지만, 큰 프로젝트에서는 버그를 줄이는 데 도움이 됩니다.

---

## 9. 생산성 관점의 차이

### 9.1 작은 게임

작은 게임에서는 Unity식 구조가 더 빠를 수 있습니다.

예:

- 퐁
- 간단한 슈팅 게임
- 짧은 프로토타입
- 게임잼 프로젝트

이 경우에는 오브젝트 수가 적고, 동작도 단순합니다.
따라서 ECS의 엄격한 데이터 접근 모델이 오히려 부담이 될 수 있습니다.

### 9.2 중대형 게임 또는 시뮬레이션

반면 다음 경우에는 ECS가 유리해질 수 있습니다.

- 총알이 매우 많다.
- 적이 매우 많다.
- 파티클이 많다.
- 상태이상, 버프, 디버프가 많다.
- AI가 많다.
- 물리적 상호작용이 많다.
- 시스템을 병렬로 돌리고 싶다.
- 데이터 흐름을 명확하게 유지하고 싶다.

이런 경우에는 ECS의 구조적 장점이 커집니다.

---

## 10. Bevy ECS를 쓸 때의 사고방식

Bevy ECS에서는 다음처럼 생각하는 것이 좋습니다.

Unity식 사고:

```text
이 오브젝트가 자기 Update에서 자기 동작을 처리한다.
```

Bevy ECS식 사고:

```text
이 컴포넌트를 가진 엔티티들은 이 시스템이 처리한다.
```

Unity식 사고:

```text
Player 클래스가 이동하고, Enemy 클래스가 이동하고, Bullet 클래스가 이동한다.
```

Bevy ECS식 사고:

```text
Velocity가 있는 엔티티는 movement 시스템이 이동시킨다.
Paddle이 있는 엔티티는 paddle input 시스템이 처리한다.
Ball이 있는 엔티티는 ball physics 시스템이 처리한다.
```

이 관점에 익숙해지면 Bevy 코드는 더 자연스러워집니다.

---

## 11. 이번 퐁 게임에서 얻은 교훈

`bevy_test04`의 `move_ball`에서 발생한 에러는 좋은 학습 사례입니다.

문제는 다음이었습니다.

```rust
paddle_query: Query<&Transform, With<Paddle>>,
mut ball_query: Query<(&mut Transform, &mut Ball)>,
```

이 코드는 사람에게는 안전해 보이지만 Bevy에게는 안전하지 않습니다.

수정은 다음이었습니다.

```rust
paddle_query: Query<&Transform, (With<Paddle>, Without<Ball>)>,
mut ball_query: Query<(&mut Transform, &mut Ball), Without<Paddle>>,
```

이 수정은 다음 의미를 가집니다.

```text
패드 쿼리와 공 쿼리는 서로 절대 같은 엔티티를 대상으로 하지 않는다.
```

이 한 줄의 필터가 Bevy ECS에게 매우 중요한 정보를 제공합니다.

---

## 12. 결론

Bevy ECS 방식은 Unity식 전통 구조보다 항상 더 간단하거나 컴팩트하지는 않습니다.

작은 게임에서는 오히려 다음 단점이 더 크게 느껴질 수 있습니다.

- 코드가 길다.
- 쿼리 타입이 복잡하다.
- `With`, `Without`, `Res`, `ResMut` 같은 개념을 배워야 한다.
- 시스템 충돌 에러가 초반에는 낯설다.

하지만 ECS는 다음 장점이 있습니다.

- 데이터 접근이 명확하다.
- 같은 종류의 데이터를 일괄 처리하기 좋다.
- 많은 엔티티를 처리할 때 성능상 유리하다.
- 시스템 병렬 실행 가능성을 엔진이 판단할 수 있다.
- 상속보다 조합 중심 설계에 강하다.
- 데이터 접근 충돌을 조기에 발견할 수 있다.

따라서 최종적으로는 다음처럼 정리할 수 있습니다.

```text
작은 게임 / 빠른 프로토타입:
    Unity식 구조가 더 빠르고 직관적일 수 있음

많은 엔티티 / 복잡한 데이터 흐름 / 병렬 처리 / 성능 중요:
    Bevy ECS 방식이 더 유리할 수 있음
```

한 줄 요약:

```text
Bevy ECS는 코드의 즉각적인 간결함보다 데이터 접근의 명확성, 안전성, 확장성, 병렬 처리 가능성을 얻는 방식이다.
```
