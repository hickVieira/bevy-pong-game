use bevy::prelude::*;

#[derive(Component)]
pub struct Paddle {
    move_up: KeyCode,
    move_down: KeyCode,
    dir: i8,
}

#[derive(Component)]
pub struct Ball {
    dir: Vec2,
}

#[derive(Component)]
pub struct Score {
    left: u32,
    right: u32,
}

pub fn sys_spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn sys_spawn_paddles(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(700., 500.)),
                ..Default::default()
            },
            ..Default::default()
        },
        Score { left: 0, right: 0 },
    ));

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
                0.5 * rand::random::<f32>().signum(),
                0.5 * rand::random::<f32>().signum(),
            )
            .normalize(),
        },
    ));
}

pub fn sys_spawn_score(mut commands: Commands) {
    commands.spawn(Text2dBundle {
        transform: Transform::from_translation(Vec3::new(0., 300., 0.)),
        text: Text {
            sections: vec![TextSection {
                value: "p1:p2".to_string(),
                style: TextStyle {
                    font_size: 40.,
                    color: Color::WHITE,
                    ..Default::default()
                },
            }],
            ..Default::default()
        },
        ..Default::default()
    });
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

        transform.translation.y = transform.translation.y.clamp(-175., 175.);
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

pub fn sys_process_score(mut balls: Query<(&Ball, &Transform)>, mut scores: Query<&mut Score>) {
    for (_ball, transform) in &mut balls {
        if transform.translation.x >= 350. {
            scores.get_single_mut().unwrap().left += 1;
        } else if transform.translation.x <= -350. {
            scores.get_single_mut().unwrap().right += 1;
        }
    }
}

pub fn sys_process_text(scores: Query<&Score>, mut texts: Query<&mut Text>) {
    let score = scores.get_single().unwrap();
    texts.get_single_mut().unwrap().sections[0].value = format!("{} : {}", score.left, score.right);
}
