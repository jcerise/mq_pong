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

#[derive(Clone, Copy)]
struct MoveableObject<'a> {
    texture: &'a Texture2D,
    point: Point,
    velocity: Option<Velocity>,
    rect: Rect
}

impl MoveableObject<'_> {
    fn update_position(&mut self, x: f32, y: f32) {
        self.point.x = x;
        self.point.y = y;
        self.rect.move_to(Vec2::new(x, y));
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
    let ball_texture: Texture2D = load_texture("resources/ball.png").await.unwrap();

    let player_point = Point::new(10., (screen_height() / 2.) - player_paddle.height());
    let mut player: MoveableObject = MoveableObject {
        texture: &player_paddle,
        point: player_point,
        velocity: None,
        rect: Rect::new(player_point.x, player_point.y, player_paddle.width() / 2., player_paddle.height() / 2.)
    };

    let opponent_point = Point::new(screen_width() - 20., (screen_height() / 2.) - opponent_paddle.height());
    let mut opponent: MoveableObject = MoveableObject {
        texture: &opponent_paddle,
        point: opponent_point,
        velocity: None,
        rect: Rect::new(opponent_point.x, opponent_point.y, opponent_paddle.width() / 2., opponent_paddle.height() / 2.)
    };

    let ball_point = Point::new(screen_width() / 2. - ball_texture.width(), screen_height() / 2. - ball_texture.height());
    let mut ball: MoveableObject = MoveableObject{
        texture: &ball_texture,
        point: ball_point,
        velocity: Option::from(Velocity::new(rng.gen_range(0.0..3.0), rng.gen_range(0.0..3.0))),
        rect: Rect::new(ball_point.x, ball_point.y, ball_texture.width() / 2., ball_texture.height() / 2.)
    };

    let mut player_score = 0;
    let mut opponent_score = 0;

    loop {
        clear_background(BLACK);

        update_ball_position(&mut ball, player, opponent, &mut player_score, &mut opponent_score);

        handle_input(&mut player);
        move_opponent(&mut ball, &mut opponent);

        // The divider is static and does not move, its purpose is to visually divide the screen
        draw_texture(&divider, screen_width() / 2., 0., WHITE);

        // Draw scores
        draw_text_ex(
            &player_score.to_string(),
            100.,
            30.,
            TextParams{
                font_size: 50,
                color: WHITE,
                ..Default::default()
            });

        draw_text_ex(
            &opponent_score.to_string(),
            185.,
            30.,
            TextParams{
                font_size: 50,
                color: WHITE,
                ..Default::default()
            });

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
        delta += -3.;
    }
    if is_key_down(KeyCode::Down) {
        delta += 3.;
    }

    if (p.point.y + delta) <= 0. || (p.point.y + delta) >= (screen_height() - p.texture.height()) {
        delta = 0.
    }

    p.update_position(p.point.x, p.point.y + delta)
}

/// Moves the opponent paddle. For now, this simply mirrors the players movement
/// Intention is for this track the ball
fn move_opponent(ball: &MoveableObject, opponent: &mut MoveableObject) {
    opponent.update_position(opponent.point.x, ball.point.y)
}

/// Moves the ball, and handles a few distinct cases.
/// - If the ball has left the right side of the screen, the player has scored, reset the ball and update the players score
/// - If the ball has left the left side of the screen, the opponent has scored, reset the ball and the opponents score
/// - If the ball is about to leave the top or bottom of the screen, bounce it instead, inverting its current velocity
fn update_ball_position(ball: &mut MoveableObject, player_paddle: MoveableObject, opponent_paddle: MoveableObject, player_score: &mut i32, opponent_score: &mut i32) {
    if ball.velocity.is_some() {
        let mut vel = ball.velocity.unwrap();

        if (ball.point.x + vel.x) >= (screen_width())  {
            reset_ball(ball);
            *player_score += 1;
        }

        if (ball.point.x + vel.x) <= (0. - ball.texture.width() / 2.) {
            reset_ball(ball);
            *opponent_score += 1;
        }

        if (ball.point.y + vel.y) <= (0. - ball.texture.width() / 2.) || (ball.point.y + vel.y) >= (screen_height()- ball.texture.width() / 2.) {
            vel.update(vel.x, -vel.y);
        }

        if player_paddle.rect.overlaps(&ball.rect) || opponent_paddle.rect.overlaps(&ball.rect) {
            vel.update(-vel.x, vel.y);
        }

        ball.update_position(ball.point.x + vel.x, ball.point.y + vel.y);
        ball.update_velocity(vel);
    }
}

/// Resets the ball to its original position, and its existing velocity
fn reset_ball(ball: &mut MoveableObject) {
    let ball_point = Point::new(screen_width() / 2. - ball.texture.width(), screen_height() / 2. - ball.texture.height());
    ball.update_position(ball_point.x, ball_point.y);
}