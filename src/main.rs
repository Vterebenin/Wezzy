use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_matchbox::prelude::*;

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(10.);
    commands.spawn(camera_bundle);
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0., 0.47, 1., 0.8),
                custom_size: Some(Vec2::new(1., 1.)),
                ..default()
            },
            ..default()
        },
    ));
}

fn move_player(
    mut players: Query<&mut Transform, With<Player>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut direction = Vec2::ZERO;
    if keys.any_pressed([KeyCode::Up, KeyCode::W]) {
        direction.y += 1.;
    }
    if keys.any_pressed([KeyCode::Down, KeyCode::S]) {
        direction.y -= 1.;
    }
    if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
        direction.x -= 1.;
    }
    if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
        direction.x += 1.;
    }

    if direction == Vec2::ZERO {
        return;
    }

    let move_speed = 7.;
    let move_delta = direction * move_speed * time.delta_seconds();

    for mut transform in &mut players {
        transform.translation += move_delta.extend(0.);
    }
}

fn start_matchbox_socket(mut commands: Commands) {
    let room_url = "ws://127.0.0.1:3536/extreme_bevy?next=2";
    info!("connecting to matchbox server: {room_url}");
    commands.insert_resource(MatchboxSocket::new_ggrs(room_url));
}

fn wait_for_players(mut socket: ResMut<MatchboxSocket<SingleChannel>>) {
    if socket.get_channel(0).is_err() {
        return;
    }

    socket.update_peers();
    let players = socket.players();
    let num_players = 2;
    if players.len() < num_players {
        return;
    }

    info!("All peers have joined, going in-game");
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb(0.53, 0.53, 0.53)))
        .add_systems(Startup, (setup, spawn_player, start_matchbox_socket))
        .add_systems(Update, (move_player, wait_for_players))
        .run();
}
