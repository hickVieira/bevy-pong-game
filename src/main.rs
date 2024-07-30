use bevy::prelude::*;

mod pong;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (
                pong::sys_spawn_camera,
                pong::sys_spawn_players,
                pong::sys_spawn_ball,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                pong::sys_move_paddles,
                pong::sys_move_ball,
                pong::sys_collide_ball_walls,
                pong::sys_collide_ball_paddle,
            )
                .chain(),
        )
        .run();
}
