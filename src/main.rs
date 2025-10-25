use macroquad::prelude::*;

// ******** CONSTANTS *********
const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;

// Player
const PLAYER_SPEED: f32 = 300.0;
const PLAYER_START_POS_X: f32 = (SCREEN_WIDTH / 2) as f32;
const PLAYER_START_POS_Y: f32 = 500.0;

// asset file paths
const PLAYER_IMAGE: &str = "player.png";
const BULLET_IMAGE: &str = "bullet.png";
const BACKGROUND_IMAGE: &str = "background.png";

// ****** STRUCTS *******
struct Bullet {
    x: f32,
    y: f32,
    speed: f32,
    active: bool,
}

struct Textures {
    player: Texture2D,
    bullet: Texture2D,
    background: Texture2D,
}

struct Game {
    // positions
    player_x: f32,
    player_y: f32,

    bullet: Option<Bullet>,

    tex: Textures,
}

// ******* METHODS *******
impl Game {
    fn handle_input(&mut self, dt: f32) {
        if is_key_down(KeyCode::Left) {
            self.player_x -= PLAYER_SPEED * dt;
        }
        if is_key_down(KeyCode::Right) {
            self.player_x += PLAYER_SPEED * dt;
        }

        // keep inside screen bounds
        self.player_x = self
            .player_x
            .clamp(0.0, SCREEN_WIDTH as f32 - self.tex.player.width());

        // fire bullet (only if none active)
        if is_key_pressed(KeyCode::Space) && self.bullet.is_none() {
            let bw = self.tex.bullet.width();
            let px_center = self.player_x + self.tex.player.width() * 0.5;
            self.bullet = Some(Bullet {
                x: px_center - bw * 0.5,
                y: self.player_y - self.tex.bullet.height(),
                speed: 600.0,
                active: true,
            });
        }
    }

    fn update(&mut self, dt: f32) {
        // move bullet upward
        if let Some(b) = &mut self.bullet {
            if b.active {
                b.y -= b.speed * dt;
                if b.y + self.tex.bullet.height() < 0.0 {
                    b.active = false;
                }
            }
            if !b.active {
                self.bullet = None;
            }
        }
    }

    fn draw(&self) {
        draw_texture(&self.tex.background, 0.0, 0.0, WHITE);

        if let Some(b) = &self.bullet {
            if b.active {
                draw_texture(&self.tex.bullet, b.x, b.y, WHITE);
            }
        }

        draw_texture(&self.tex.player, self.player_x, self.player_y, WHITE);
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Space Invaders (Rust)".to_string(),
        window_width: SCREEN_WIDTH,
        window_height: SCREEN_HEIGHT,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use macroquad::file::set_pc_assets_folder;
        set_pc_assets_folder("assets");
    }

    // textures
    let player = load_texture(PLAYER_IMAGE).await.unwrap();
    let bullet = load_texture(BULLET_IMAGE).await.unwrap();
    let background = load_texture(BACKGROUND_IMAGE).await.unwrap();

    let tex = Textures {
        player,
        bullet,
        background,
    };

    let mut game = Game {
        player_x: PLAYER_START_POS_X - (tex.player.width() / 2.0),
        player_y: PLAYER_START_POS_Y,
        bullet: None,
        tex,
    };

    // main loop
    loop {
        // delta time makes framerate consistent
        let dt = get_frame_time();

        game.handle_input(dt);
        game.update(dt);
        game.draw();

        next_frame().await;
    }
}