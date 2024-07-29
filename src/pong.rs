use bevy::prelude::*;

#[derive(Component)]
pub struct Paddle {
    move_up: KeyCode,
    move_down: KeyCode,
    dir: i8,
}

#[derive(Component, Default)]
pub struct Ball {
    dir: Vec2,
}

pub fn sys_spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn sys_spawn_players(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(700., 500.)),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(-300., 0., 0.)),
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(10., 150.)),
                ..Default::default()
            },
            ..Default::default()
        },
        Paddle {
            move_up: KeyCode::KeyW,
            move_down: KeyCode::KeyS,
            dir: 1,
        },
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(300., 0., 0.)),
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(10., 150.)),
                ..Default::default()
            },
            ..Default::default()
        },
        Paddle {
            move_up: KeyCode::ArrowUp,
            move_down: KeyCode::ArrowDown,
            dir: -1,
        },
    ));
}

pub fn sys_spawn_ball(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(10., 10.)),
                ..Default::default()
            },
            ..Default::default()
        },
        Ball {
            dir: Vec2::new(
                rand::random::<f32>() * 2. - 1.,
                rand::random::<f32>() * 2. - 1.,
            )
            .normalize(),
        },
    ));
}

pub fn sys_move_paddles(
    mut paddles: Query<(&mut Transform, &Paddle)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, paddle) in &mut paddles {
        if input.pressed(paddle.move_up) {
            transform.translation.y += 400. * time.delta_seconds();
        }

        if input.pressed(paddle.move_down) {
            transform.translation.y -= 400. * time.delta_seconds();
        }
    }
}

pub fn sys_move_ball(mut balls: Query<(&mut Transform, &Ball)>, time: Res<Time>) {
    for (mut transform, ball) in &mut balls {
        transform.translation += 500. * ball.dir.extend(0.) * time.delta_seconds();
    }
}

pub fn sys_collide_ball_paddle(
    mut balls: Query<(&Transform, &mut Ball)>,
    paddles: Query<(&Transform, &Paddle), With<Paddle>>,
) {
    for (ball_t, mut ball_d) in &mut balls {
        for (paddle_t, paddle_d) in &paddles {
            if (ball_t.translation.y >= paddle_t.translation.y - 75.)
                && (ball_t.translation.y <= paddle_t.translation.y + 75.)
                && (ball_t.translation.x >= paddle_t.translation.x - 5.)
                && (ball_t.translation.x <= paddle_t.translation.x + 5.)
            {
                ball_d.dir.x = (paddle_d.dir as f32) * ball_d.dir.x.abs();
            }
        }
    }
}

pub fn sys_collide_ball_walls(mut balls: Query<(&Transform, &mut Ball)>) {
    for (ball_t, mut ball_d) in &mut balls {
        if ball_t.translation.x >= 350. {
            ball_d.dir.x = -ball_d.dir.x.abs()
        } else if ball_t.translation.x <= -350. {
            ball_d.dir.x = ball_d.dir.x.abs()
        }

        if ball_t.translation.y >= 250. {
            ball_d.dir.y = -ball_d.dir.y.abs()
        } else if ball_t.translation.y <= -250. {
            ball_d.dir.y = ball_d.dir.y.abs()
        }
    }
}
