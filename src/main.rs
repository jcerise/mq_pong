use macroquad::prelude::*;

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

struct Paddle<'a> {
    texture: &'a Texture2D,
    point: Point,
}

impl Paddle<'_> {
    fn update_position(&mut self, p: Point) {
        self.point.y = p.y
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

    let divider: Texture2D = load_texture("resources/full_divider.png").await.unwrap();
    let player_paddle: Texture2D = load_texture("resources/player_paddle.png").await.unwrap();
    let opponent_paddle: Texture2D = load_texture("resources/opponent_paddle.png").await.unwrap();

    let mut player: Paddle = Paddle{
        texture: &player_paddle,
        point: Point::new(10., (screen_height() / 2.) - player_paddle.height())
    };

    let mut opponent: Paddle = Paddle{
        texture: &opponent_paddle,
        point: Point::new(screen_width() - 20., (screen_height() / 2.) - opponent_paddle.height())
    };

    loop {
        clear_background(BLACK);

        handle_input(&mut player);
        move_opponent(&mut player, &mut opponent);

        // The divider is static and does not move, its purpose is to visually divide the screen
        draw_texture(&divider, screen_width() / 2., 0., WHITE);
        draw_texture(&player.texture, player.point.x, player.point.y, WHITE);
        draw_texture(&opponent.texture, opponent.point.x, opponent.point.y, WHITE);

        next_frame().await
    }
}

/// Handles two cases of input from the player, the up arrow key, and the down arrow key
/// Movement is restricted to the screen height
fn handle_input(p: &mut Paddle) {
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

    p.update_position(Point::new(p.point.x, p.point.y + delta))
}

/// Moves the opponent paddle. For now, this simply mirrors the players movement
/// Intention is for this track the ball
fn move_opponent(player: &Paddle, opponent: &mut Paddle) {
    opponent.update_position(player.point)
}