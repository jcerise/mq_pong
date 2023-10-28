use macroquad::prelude::*;

extern crate rand;
use rand::prelude::*;

#[derive(Copy, Clone, PartialEq)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self{ x, y }
    }
}

#[derive(Copy, Clone, PartialEq)]
struct Velocity {
    x: f32,
    y: f32
}

impl Velocity {
    fn new(x: f32, y: f32) -> Self {
        Self{x, y}
    }
    fn update(&mut self, x:f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}

struct MoveableObject<'a> {
    texture: &'a Texture2D,
    point: Point,
    velocity: Option<Velocity>,
}

impl MoveableObject<'_> {
    fn update_position(&mut self, x: f32, y: f32) {
        self.point.x = x;
        self.point.y = y;
    }
    fn update_velocity(&mut self, vel: Velocity) {
        self.velocity = Option::from(vel);
    }
}

fn conf() -> Conf {
    Conf {
        window_title: "MQ Pong".to_string(),
        window_width: 300,
        window_height: 240,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {

    let mut rng = rand::thread_rng();

    let divider: Texture2D = load_texture("resources/full_divider.png").await.unwrap();
    let player_paddle: Texture2D = load_texture("resources/player_paddle.png").await.unwrap();
    let opponent_paddle: Texture2D = load_texture("resources/opponent_paddle.png").await.unwrap();
    let ball: Texture2D = load_texture("resources/ball.png").await.unwrap();

    let mut player: MoveableObject = MoveableObject {
        texture: &player_paddle,
        point: Point::new(10., (screen_height() / 2.) - player_paddle.height()),
        velocity: None
    };

    let mut opponent: MoveableObject = MoveableObject {
        texture: &opponent_paddle,
        point: Point::new(screen_width() - 20., (screen_height() / 2.) - opponent_paddle.height()),
        velocity: None
    };

    let mut ball: MoveableObject = MoveableObject{
        texture: &ball,
        point: Point::new(screen_width() / 2., screen_height() / 2.),
        velocity: Option::from(Velocity::new(rng.gen_range(0.0..5.0), rng.gen_range(0.0..5.0)))
    };

    loop {
        clear_background(BLACK);

        update_ball_position(&mut ball);

        handle_input(&mut player);
        move_opponent(&mut player, &mut opponent);

        // The divider is static and does not move, its purpose is to visually divide the screen
        draw_texture(&divider, screen_width() / 2., 0., WHITE);
        draw_texture(&player.texture, player.point.x, player.point.y, WHITE);
        draw_texture(&opponent.texture, opponent.point.x, opponent.point.y, WHITE);

        draw_texture(&ball.texture, ball.point.x, ball.point.y, WHITE);

        next_frame().await
    }
}

/// Handles two cases of input from the player, the up arrow key, and the down arrow key
/// Movement is restricted to the screen height
fn handle_input(p: &mut MoveableObject) {
    let mut delta = 0.;
    if is_key_down(KeyCode::Up) {
        delta += -2.;
    }
    if is_key_down(KeyCode::Down) {
        delta += 2.;
    }

    if (p.point.y + delta) <= 0. || (p.point.y + delta) >= (screen_height() - p.texture.height()) {
        delta = 0.
    }

    p.update_position(p.point.x, p.point.y + delta)
}

/// Moves the opponent paddle. For now, this simply mirrors the players movement
/// Intention is for this track the ball
fn move_opponent(player: &MoveableObject, opponent: &mut MoveableObject) {
    opponent.update_position(opponent.point.x, player.point.y)
}

fn update_ball_position(ball: &mut MoveableObject) {
    if ball.velocity.is_some() {
        let mut vel = ball.velocity.unwrap();

        if (ball.point.x + vel.x) >= (screen_width() - ball.texture.width() / 2.) || (ball.point.x + vel.x) <= (0. - ball.texture.width() / 2.) {
            vel.update(-vel.x, vel.y);
        }

        if (ball.point.y + vel.y) <= (0. - ball.texture.width() / 2.) || (ball.point.y + vel.y) >= (screen_height()- ball.texture.width() / 2.) {
            vel.update(vel.x, -vel.y);
        }

        ball.update_position(ball.point.x + vel.x, ball.point.y + vel.y);
        ball.update_velocity(vel);
    }
}