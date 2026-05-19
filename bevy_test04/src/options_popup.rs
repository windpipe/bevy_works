use crate::settings::{GraphicsSettings, on_off};
use bevy::prelude::*;

pub struct OptionsPopupPlugin;

impl Plugin for OptionsPopupPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<OptionsPopupState>().add_systems(
            Update,
            (
                toggle_options_popup,
                handle_options_buttons,
                update_options_text,
                update_button_colors,
            ),
        );
    }
}

#[derive(Resource, Default)]
struct OptionsPopupState {
    visible: bool,
    draft: GraphicsSettings,
    message: String,
}

#[derive(Component)]
struct OptionsPopupRoot;

#[derive(Component)]
struct OptionsText;

#[derive(Component)]
enum OptionsButton {
    ToggleVsync,
    ToggleFullscreen,
    NextResolution,
    NextRenderer,
    Ok,
    Cancel,
}

fn toggle_options_popup(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    settings: Res<GraphicsSettings>,
    mut popup_state: ResMut<OptionsPopupState>,
    mut commands: Commands,
    popup_query: Query<Entity, With<OptionsPopupRoot>>,
) {
    if !keyboard_input.just_pressed(KeyCode::F10) {
        return;
    }

    if popup_state.visible {
        close_options_popup(&mut commands, &popup_query, &mut popup_state);
    } else {
        popup_state.visible = true;
        popup_state.draft = *settings;
        popup_state.message.clear();
        spawn_options_popup(&mut commands, &popup_state);
    }
}

fn spawn_options_popup(commands: &mut Commands, popup_state: &OptionsPopupState) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.55)),
            OptionsPopupRoot,
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    width: Val::Px(640.0),
                    padding: UiRect::all(Val::Px(24.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(12.0),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.10, 0.11, 0.16, 0.98)),
            ))
            .with_children(|panel| {
                panel.spawn((
                    Text::new("Options"),
                    TextFont {
                        font_size: 34.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));

                panel.spawn((
                    Text::new(options_description(
                        &popup_state.draft,
                        &popup_state.message,
                    )),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.95, 1.0)),
                    OptionsText,
                ));

                spawn_options_button(panel, OptionsButton::ToggleVsync, "Toggle VSync");
                spawn_options_button(panel, OptionsButton::ToggleFullscreen, "Toggle Fullscreen");
                spawn_options_button(panel, OptionsButton::NextResolution, "Next Resolution");
                spawn_options_button(panel, OptionsButton::NextRenderer, "Next Renderer");

                panel
                    .spawn((Node {
                        width: Val::Percent(100.0),
                        column_gap: Val::Px(12.0),
                        ..default()
                    },))
                    .with_children(|row| {
                        spawn_options_button(row, OptionsButton::Ok, "OK - Save for next run");
                        spawn_options_button(row, OptionsButton::Cancel, "Cancel");
                    });
            });
        });
}

fn spawn_options_button(parent: &mut ChildSpawnerCommands, action: OptionsButton, label: &str) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(44.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.22, 0.24, 0.34)),
            action,
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(label),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn handle_options_buttons(
    mut commands: Commands,
    mut settings: ResMut<GraphicsSettings>,
    mut popup_state: ResMut<OptionsPopupState>,
    popup_query: Query<Entity, With<OptionsPopupRoot>>,
    interaction_query: Query<(&Interaction, &OptionsButton), (Changed<Interaction>, With<Button>)>,
) {
    if !popup_state.visible {
        return;
    }

    for (interaction, action) in &interaction_query {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match action {
            OptionsButton::ToggleVsync => popup_state.draft.vsync = !popup_state.draft.vsync,
            OptionsButton::ToggleFullscreen => {
                popup_state.draft.fullscreen = !popup_state.draft.fullscreen
            }
            OptionsButton::NextResolution => {
                popup_state.draft.resolution = popup_state.draft.resolution.next()
            }
            OptionsButton::NextRenderer => {
                popup_state.draft.renderer = popup_state.draft.renderer.next()
            }
            OptionsButton::Ok => match popup_state.draft.save() {
                Ok(()) => {
                    *settings = popup_state.draft;
                    close_options_popup(&mut commands, &popup_query, &mut popup_state);
                }
                Err(error) => {
                    popup_state.message = format!("Failed to save config: {error}");
                }
            },
            OptionsButton::Cancel => {
                close_options_popup(&mut commands, &popup_query, &mut popup_state)
            }
        }
    }
}

fn update_options_text(
    popup_state: Res<OptionsPopupState>,
    mut query: Query<&mut Text, With<OptionsText>>,
) {
    if !popup_state.is_changed() {
        return;
    }

    for mut text in &mut query {
        **text = options_description(&popup_state.draft, &popup_state.message);
    }
}

fn update_button_colors(
    mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in &mut query {
        *color = match *interaction {
            Interaction::Pressed => BackgroundColor(Color::srgb(0.12, 0.55, 0.32)),
            Interaction::Hovered => BackgroundColor(Color::srgb(0.32, 0.35, 0.48)),
            Interaction::None => BackgroundColor(Color::srgb(0.22, 0.24, 0.34)),
        };
    }
}

fn close_options_popup(
    commands: &mut Commands,
    popup_query: &Query<Entity, With<OptionsPopupRoot>>,
    popup_state: &mut OptionsPopupState,
) {
    for entity in popup_query.iter() {
        commands.entity(entity).despawn();
    }
    popup_state.visible = false;
}

fn options_description(settings: &GraphicsSettings, message: &str) -> String {
    let suffix = if message.is_empty() {
        "Changes are saved to config only when OK is pressed.\nThey are applied on the next run."
            .to_string()
    } else {
        message.to_string()
    };

    format!(
        "VSync: {}\nFullscreen: {}\nResolution: {}\nRenderer: {}\n\n{suffix}",
        on_off(settings.vsync),
        on_off(settings.fullscreen),
        settings.resolution,
        settings.renderer,
    )
}
