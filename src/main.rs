use bevy::prelude::*;

mod pong;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    app.add_systems(
        Startup,
        (
            pong::sys_spawn_camera,
            pong::sys_spawn_players,
            pong::sys_spawn_ball,
        ),
    );
    app.add_systems(Update, pong::sys_move_paddles);
    app.add_systems(
        Update,
        (
            pong::sys_move_ball,
            pong::sys_collide_ball_walls,
            pong::sys_collide_ball_paddle,
        ),
    );

    app.run();
}
