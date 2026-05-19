use crate::settings::{GraphicsSettings, settings_description};
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

const ARENA_WIDTH: f32 = 800.0;
const ARENA_HEIGHT: f32 = 600.0;
const PADDLE_SIZE: Vec2 = Vec2::new(24.0, 120.0);
const BALL_SIZE: Vec2 = Vec2::new(20.0, 20.0);
const PADDLE_X: f32 = -360.0;
const BALL_START_SPEED: f32 = 330.0;
const PADDLE_SPEED: f32 = 480.0;

pub struct PongGamePlugin;

impl Plugin for PongGamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameScore { value: 0 })
            .add_systems(Startup, setup_game)
            .add_systems(
                Update,
                (move_paddle, move_ball, update_score_text, update_fps_text),
            );
    }
}

#[derive(Resource)]
struct GameScore {
    value: u32,
}

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball {
    velocity: Vec2,
}

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct FpsText;

fn setup_game(mut commands: Commands, settings: Res<GraphicsSettings>) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite {
            color: Color::srgb(0.1, 0.1, 0.14),
            custom_size: Some(Vec2::new(ARENA_WIDTH, ARENA_HEIGHT)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -5.0),
    ));

    commands.spawn((
        Sprite {
            color: Color::srgb(0.7, 0.7, 0.75),
            custom_size: Some(Vec2::new(4.0, ARENA_HEIGHT)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -4.0),
    ));

    commands.spawn((
        Sprite {
            color: Color::srgb(0.35, 0.75, 1.0),
            custom_size: Some(PADDLE_SIZE),
            ..default()
        },
        Transform::from_xyz(PADDLE_X, 0.0, 0.0),
        Paddle,
    ));

    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.95, 0.35),
            custom_size: Some(BALL_SIZE),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Ball {
            velocity: Vec2::new(-BALL_START_SPEED, BALL_START_SPEED * 0.35),
        },
    ));

    commands.spawn((
        Text::new("Score: 0"),
        TextFont {
            font_size: 28.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(16.0),
            top: Val::Px(12.0),
            ..default()
        },
        ScoreText,
    ));

    commands.spawn((
        Text::new("FPS: ...\nMove: W/S or Up/Down\nOptions: F10"),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::srgb(0.75, 0.9, 0.75)),
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(16.0),
            top: Val::Px(12.0),
            ..default()
        },
        FpsText,
    ));

    info!("Loaded settings: {}", settings_description(&settings));
}

fn move_paddle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Paddle>>,
) {
    let mut direction = 0.0;
    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        direction += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        direction -= 1.0;
    }

    for mut transform in &mut query {
        transform.translation.y += direction * PADDLE_SPEED * time.delta_secs();
        let limit = ARENA_HEIGHT * 0.5 - PADDLE_SIZE.y * 0.5;
        transform.translation.y = transform.translation.y.clamp(-limit, limit);
    }
}

fn move_ball(
    time: Res<Time>,
    mut score: ResMut<GameScore>,
    paddle_query: Query<&Transform, (With<Paddle>, Without<Ball>)>,
    mut ball_query: Query<(&mut Transform, &mut Ball), Without<Paddle>>,
) {
    let Ok(paddle_transform) = paddle_query.single() else {
        return;
    };

    for (mut ball_transform, mut ball) in &mut ball_query {
        ball_transform.translation.x += ball.velocity.x * time.delta_secs();
        ball_transform.translation.y += ball.velocity.y * time.delta_secs();

        let half_ball = BALL_SIZE * 0.5;
        let top = ARENA_HEIGHT * 0.5 - half_ball.y;
        let bottom = -top;
        let right = ARENA_WIDTH * 0.5 - half_ball.x;
        let left = -ARENA_WIDTH * 0.5 - half_ball.x;

        if ball_transform.translation.y >= top {
            ball_transform.translation.y = top;
            ball.velocity.y = -ball.velocity.y.abs();
        } else if ball_transform.translation.y <= bottom {
            ball_transform.translation.y = bottom;
            ball.velocity.y = ball.velocity.y.abs();
        }

        if ball_transform.translation.x >= right {
            ball_transform.translation.x = right;
            ball.velocity.x = -ball.velocity.x.abs();
        }

        let paddle_min_x = PADDLE_X - PADDLE_SIZE.x * 0.5 - half_ball.x;
        let paddle_max_x = PADDLE_X + PADDLE_SIZE.x * 0.5 + half_ball.x;
        let paddle_min_y = paddle_transform.translation.y - PADDLE_SIZE.y * 0.5 - half_ball.y;
        let paddle_max_y = paddle_transform.translation.y + PADDLE_SIZE.y * 0.5 + half_ball.y;

        let hits_paddle = ball.velocity.x < 0.0
            && ball_transform.translation.x >= paddle_min_x
            && ball_transform.translation.x <= paddle_max_x
            && ball_transform.translation.y >= paddle_min_y
            && ball_transform.translation.y <= paddle_max_y;

        if hits_paddle {
            let relative_y = (ball_transform.translation.y - paddle_transform.translation.y)
                / (PADDLE_SIZE.y * 0.5);
            let speed = ball.velocity.length() * 1.04;
            ball.velocity = Vec2::new(speed, relative_y * BALL_START_SPEED).normalize() * speed;
            ball_transform.translation.x = paddle_max_x;
            score.value += 1;
        }

        if ball_transform.translation.x < left {
            ball_transform.translation = Vec3::ZERO;
            ball.velocity = Vec2::new(-BALL_START_SPEED, BALL_START_SPEED * 0.35);
            score.value = 0;
        }
    }
}

fn update_score_text(score: Res<GameScore>, mut query: Query<&mut Text, With<ScoreText>>) {
    if !score.is_changed() {
        return;
    }

    for mut text in &mut query {
        **text = format!("Score: {}", score.value);
    }
}

fn update_fps_text(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<FpsText>>) {
    let fps_text = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|diagnostic| diagnostic.smoothed())
        .map(|fps| format!("{fps:.1}"))
        .unwrap_or_else(|| "...".to_string());

    for mut text in &mut query {
        **text = format!("FPS: {fps_text}\nMove: W/S or Up/Down\nOptions: F10");
    }
}
