use super::*;
use crate::utils::*;
use bevy::app::AppExit;

#[derive(Component)]
struct OnMainMenuScreen;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Menu).with_system(main_menu_setup))
            .add_system_set(
                SystemSet::on_update(GameState::Menu)
                    .with_system(menu_action)
                    .with_system(button_system),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Menu)
                    .with_system(despawn_entities::<OnMainMenuScreen>),
            );
    }
}

#[allow(clippy::type_complexity)]
fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<State<GameState>>,
) {
    for (interaction, menu_button_action) in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            match menu_button_action {
                MenuButtonAction::Quit => app_exit_events.send(AppExit),
                MenuButtonAction::Play => {
                    game_state.set(GameState::Setup).unwrap();
                }
            }
        }
    }
}

fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    // Common style for all buttons on the screen
    let button_style = Style {
        size: Size::new(Val::Px(250.0), Val::Px(65.0)),
        margin: Rect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
    };
    let button_icon_style = Style {
        size: Size::new(Val::Px(30.0), Val::Auto),
        // This takes the icons out of the flexbox flow, to be positionned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        position: Rect {
            left: Val::Px(10.0),
            right: Val::Auto,
            top: Val::Auto,
            bottom: Val::Auto,
        },
        ..Default::default()
    };
    let button_text_style = TextStyle {
        font: font.clone(),
        font_size: 40.0,
        color: TEXT_COLOR,
    };

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: Rect::all(Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: Color::DARK_GRAY.into(),
            ..Default::default()
        })
        .insert(OnMainMenuScreen)
        .with_children(|parent| {
            // Display the game name
            parent.spawn_bundle(TextBundle {
                style: Style {
                    margin: Rect::all(Val::Px(50.0)),
                    ..Default::default()
                },
                text: Text::with_section(
                    "AutoSlasher",
                    TextStyle {
                        font: font.clone(),
                        font_size: 80.0,
                        color: TEXT_COLOR,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });

            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    color: NORMAL_BUTTON.into(),
                    ..Default::default()
                })
                .insert(MenuButtonAction::Play)
                .with_children(|parent| {
                    let icon = asset_server.load("textures/Game Icons/right.png");
                    parent.spawn_bundle(ImageBundle {
                        style: button_icon_style.clone(),
                        image: UiImage(icon),
                        ..Default::default()
                    });
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Play",
                            button_text_style.clone(),
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });
            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style,
                    color: NORMAL_BUTTON.into(),
                    ..Default::default()
                })
                .insert(MenuButtonAction::Quit)
                .with_children(|parent| {
                    let icon = asset_server.load("textures/Game Icons/exitRight.png");
                    parent.spawn_bundle(ImageBundle {
                        style: button_icon_style,
                        image: UiImage(icon),
                        ..Default::default()
                    });
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section("Quit", button_text_style, Default::default()),
                        ..Default::default()
                    });
                });
        });
}
