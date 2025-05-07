use std::time::Duration;

use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResolution},
};

pub struct SimpleGamePlugin;

impl Plugin for SimpleGamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_event::<GameInputEvent>()
            .add_systems(Startup, load_assets)
            .add_systems(OnEnter(GameState::InGame), setup_world)
            .add_systems(
                Update,
                (
                    handle_dead,
                    handle_shoot_duck,
                    handle_mouse_clicks,
                    handle_dying,
                    animate_ducks,
                    spawn_ducks,
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    InGame,
}

#[derive(Resource)]
pub struct GameAssets {
    background_spritesheet: Handle<Image>,
    background_layout: Handle<TextureAtlasLayout>,
    duck_spritesheet: Handle<Image>,
    duck_layout: Handle<TextureAtlasLayout>,
    dog_spritesheet: Handle<Image>,
    dog_layout: Handle<TextureAtlasLayout>,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
pub struct Duck {
    behaviour: DuckBehaviour,
    speed: f32,
}

impl Default for Duck {
    fn default() -> Self {
        Self {
            behaviour: Default::default(),
            speed: 20.0,
        }
    }
}

#[derive(Default, PartialEq)]
pub enum DuckBehaviour {
    #[default]
    FlyingLeft,
    FlyingRight,
    Dying,
}

pub fn setup_world(mut commands: Commands, game_assets: Res<GameAssets>) {
    // Create a 2d camera
    commands.spawn((Camera2d));
    // Duck hunt background colour is #40c0ff
    commands.insert_resource(ClearColor(Color::linear_rgb(0.251, 0.753, 1.0)));
    // Duck spawn timer
    commands.insert_resource(SpawnTimer(Timer::from_seconds(5.0, TimerMode::Repeating)));
    // Duck hunt background
    commands.spawn((
        Sprite::from_atlas_image(
            game_assets.background_spritesheet.clone(),
            TextureAtlas {
                layout: game_assets.background_layout.clone(),
                index: 0,
            },
        ),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Spawn a duck
    commands.spawn((
        Sprite::from_atlas_image(
            game_assets.duck_spritesheet.clone(),
            TextureAtlas {
                layout: game_assets.duck_layout.clone(),
                index: 1,
            },
        ),
        Transform::from_xyz(40.0, 40.0, 0.0),
        Duck::default(),
        AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
    ));
}

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    println!("Loading assets");
    let bg_texture = asset_server.load("textures/background_spritesheet.png");
    let bg_layout = TextureAtlasLayout::from_grid(UVec2::new(256, 240), 3, 2, None, None);
    let bg_texture_atlas_layout = texture_atlas_layouts.add(bg_layout);

    let ducks_texture = asset_server.load("textures/duck_single_spritesheet.png");
    let ducks_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 5, 1, None, None);
    let ducks_texture_atlas_layout = texture_atlas_layouts.add(ducks_layout);

    let dog_texture = asset_server.load("textures/dog_spritesheet.png");
    let dog_layout = TextureAtlasLayout::from_grid(UVec2::new(60, 46), 4, 1, None, None);
    let dog_texture_atlas_layout = texture_atlas_layouts.add(dog_layout);

    commands.insert_resource(GameAssets {
        background_spritesheet: bg_texture,
        background_layout: bg_texture_atlas_layout,
        duck_spritesheet: ducks_texture,
        duck_layout: ducks_texture_atlas_layout,
        dog_spritesheet: dog_texture,
        dog_layout: dog_texture_atlas_layout,
    });
    println!("Finished loading");
    next_state.set(GameState::InGame);
}

fn animate_ducks(
    time: Res<Time>,
    mut duck_query: Query<(&mut AnimationTimer, &mut Sprite, &Duck)>,
) {
    for (mut timer, mut sprite, duck) in &mut duck_query {
        timer.tick(time.delta());
        // Animate duck depending on behaviour
        match duck.behaviour {
            DuckBehaviour::FlyingLeft => {
                sprite.flip_x = true;

                // If the timer finished show the next animation frame
                if timer.just_finished() {
                    if let Some(atlas) = &mut sprite.texture_atlas {
                        atlas.index += 1;
                        if atlas.index >= 3 {
                            atlas.index = 0;
                        }
                    }
                }
            }
            DuckBehaviour::FlyingRight => {
                sprite.flip_x = false;

                // If the timer finished show the next animation frame
                if timer.just_finished() {
                    if let Some(atlas) = &mut sprite.texture_atlas {
                        atlas.index += 1;
                        if atlas.index >= 3 {
                            atlas.index = 0;
                        }
                    }
                }
            }
            DuckBehaviour::Dying => {
                // Let the splat animation play once
                if let Some(atlas) = &mut sprite.texture_atlas {
                    if atlas.index < 3 {
                        atlas.index = 3;
                        timer.reset();
                    }
                }
                // If the timer finished show the next animation frame
                if timer.just_finished() {
                    sprite.flip_x = !sprite.flip_x;
                    if let Some(atlas) = &mut sprite.texture_atlas {
                        atlas.index += 1;

                        if atlas.index >= 4 {
                            atlas.index = 4;
                        }
                    }
                }
            }
        }

        // If the timer finished show the next animation frame
        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {}
        }
    }
}

#[derive(Resource)]
struct SpawnTimer(Timer);

fn spawn_ducks(mut commands: Commands, timer: Res<SpawnTimer>) {}

#[derive(Event)]
enum GameInputEvent {
    Shoot(Vec2),
}

fn handle_mouse_clicks(
    mouse_input: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut game_input_event_writer: EventWriter<GameInputEvent>,
) {
    let win = window_query.get_single().unwrap();
    if mouse_input.just_pressed(MouseButton::Left) {
        let position = win.cursor_position();
        if let Some(position) = position {
            // This will be offset by the window size, lets translate it to
            // our world. There is a more universal solution to this but since
            // we have the precise window size we can use this easier "hack"
            let position = Vec2::new(position.x - 256.0 / 2.0, 240.0 / 2.0 - position.y);
            println!("click at world position: {:?}", position);
            game_input_event_writer.write(GameInputEvent::Shoot(position));
        }
    }
}

fn handle_shoot_duck(
    mut commands: Commands,
    mut duck_query: Query<(Entity, &Transform, &mut Duck)>,
    mut game_input_event_reader: EventReader<GameInputEvent>,
) {
    for event in game_input_event_reader.read() {
        if let GameInputEvent::Shoot(shot_pos) = event {
            // Go through each duck and find one hit
            // Hitbox is the 32x32 tile of the sprite
            for (entity, transform, mut duck) in &mut duck_query {
                let pos = transform.translation.xy();
                let hitbox = Rect::new(pos.x - 32.0, pos.y - 32.0, pos.x + 32.0, pos.y + 32.0);
                println!("{:?} {:?} {:?}", pos, hitbox, shot_pos);
                if hitbox.contains(shot_pos.clone()) {
                    duck.behaviour = DuckBehaviour::Dying;
                    println!("Hit duck")
                }
            }
        }
    }
}

fn handle_dying(
    time: Res<Time>,
    mut commands: Commands,
    mut duck_query: Query<(Entity, &mut Transform, &Duck), Without<Dead>>,
) {
    for (entity, mut transform, duck) in &mut duck_query {
        if duck.behaviour == DuckBehaviour::Dying {
            transform.translation.y -= 80.0 * time.delta_secs();
            if transform.translation.y < -240.0 {
                commands.entity(entity).insert(Dead);
            }
        }
    }
}

#[derive(Component)]
struct Dead;

fn handle_dead(mut commands: Commands, dead_query: Query<Entity, With<Dead>>) {
    for (entity) in dead_query.iter() {
        commands.entity(entity).despawn();
    }
}
