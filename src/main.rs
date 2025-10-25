use macroquad::prelude::*;

// ******** CONSTANTS *********
const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;

// Player
const PLAYER_SPEED: f32 = 300.0;
const PLAYER_START_POS_X: f32 = (SCREEN_WIDTH / 2) as f32;
const PLAYER_START_POS_Y: f32 = 500.0;

// Aliens
const ALIEN_SPEED: f32 = 0.1;
const ALIEN_STEP_DOWN: f32 = 0.1;
const ALIEN_ROWS: usize = 3;
const ALIEN_COLS: usize = 11;

// assets
const PLAYER_IMAGE: &str = "player.png";
const BULLET_IMAGE: &str = "bullet.png";
const BACKGROUND_IMAGE: &str = "background.png";
const ENEMY_IMAGE: &str = "enemy.png";


// ****** STRUCTS *******
struct Bullet {
    x: f32,
    y: f32,
    speed: f32,
    active: bool,
}

struct Alien {
    x: f32,
    y: f32,
    alive: bool,
}

struct Textures {
    player: Texture2D,
    bullet: Texture2D,
    background: Texture2D,
    enemy: Texture2D,
}

struct Game {
    // positions
    player_x: f32,
    player_y: f32,

    // entities
    bullet: Option<Bullet>,
    aliens: Vec<Alien>,

    // alien movement
    alien_dx: f32,
    step_down: f32,

    tex: Textures,
}


// ******* METHODS *******
impl Game {
    fn handle_input(&mut self, dt: f32) {
        // check key input and move player
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.player_x -= PLAYER_SPEED * dt;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.player_x += PLAYER_SPEED * dt;
        }

        // keep inside screen bounds
        self.player_x = self
            .player_x
            .clamp(0.0, SCREEN_WIDTH as f32 - self.tex.player.width());

        // spacebar shoots a bullet
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
        // move bullet upward each frame
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

        // move aliens horizontally
        let alien_w = self.tex.enemy.width();
        let alien_h = self.tex.enemy.height();
        let sw = SCREEN_WIDTH as f32;

        let mut hit_edge = false;
        for a in self.aliens.iter_mut().filter(|a| a.alive) {
            a.x += self.alien_dx * dt;
            if a.x <= 10.0 || a.x + alien_w >= sw - 10.0 {
                hit_edge = true;
            }
        }

        // reverse direction and step down
        if hit_edge {
            self.alien_dx = -self.alien_dx;
            for a in self.aliens.iter_mut().filter(|a| a.alive) {
                a.y += self.step_down;
            }
        }

        // check bullet collisions with aliens
        if let Some(b) = &mut self.bullet {
            if b.active {
                let bw = self.tex.bullet.width();
                let bh = self.tex.bullet.height();
                for a in self.aliens.iter_mut().filter(|a| a.alive) {
                    if rects_overlap(b.x, b.y, bw, bh, a.x, a.y, alien_w, alien_h) {
                        a.alive = false;
                        b.active = false;
                        break;
                    }
                }
                if !b.active {
                    self.bullet = None;
                }
            }
        }
        // todo check alien collision with player
    }

    fn draw(&self) {
        draw_texture(&self.tex.background, 0.0, 0.0, WHITE);

        // draw aliens
        for a in self.aliens.iter().filter(|a| a.alive) {
            draw_texture(&self.tex.enemy, a.x, a.y, WHITE);
        }

        // draw bullet if active
        if let Some(b) = &self.bullet {
            if b.active {
                draw_texture(&self.tex.bullet, b.x, b.y, WHITE);
            }
        }

        // draw player ship
        draw_texture(&self.tex.player, self.player_x, self.player_y, WHITE);
    }
}


// basic rectangle collision
fn rects_overlap(ax: f32, ay: f32, aw: f32, ah: f32, bx: f32, by: f32, bw: f32, bh: f32) -> bool {
    ax < bx + bw && ax + aw > bx && ay < by + bh && ay + ah > by
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

    // load textures
    let player = load_texture(PLAYER_IMAGE).await.unwrap();
    let bullet = load_texture(BULLET_IMAGE).await.unwrap();
    let background = load_texture(BACKGROUND_IMAGE).await.unwrap();
    let enemy = load_texture(ENEMY_IMAGE).await.unwrap();

    // make pixel art crisp
    player.set_filter(FilterMode::Nearest);
    bullet.set_filter(FilterMode::Nearest);
    background.set_filter(FilterMode::Nearest);
    enemy.set_filter(FilterMode::Nearest);

    // store in struct
    let tex = Textures {
        player,
        bullet,
        background,
        enemy,
    };

    // create alien grid
    let h_spacing = tex.enemy.width() + 12.0;
    let v_spacing = tex.enemy.height() + 10.0;
    let grid_width = ALIEN_COLS as f32 * h_spacing;
    let start_x = (SCREEN_WIDTH as f32 - grid_width) * 0.5;
    let start_y = 80.0;

    let mut aliens = Vec::with_capacity(ALIEN_ROWS * ALIEN_COLS);
    for r in 0..ALIEN_ROWS {
        for c in 0..ALIEN_COLS {
            aliens.push(Alien {
                x: start_x + c as f32 * h_spacing,
                y: start_y + r as f32 * v_spacing,
                alive: true,
            });
        }
    }

    // create game object
    let mut game = Game {
        player_x: PLAYER_START_POS_X - (tex.player.width() / 2.0),
        player_y: PLAYER_START_POS_Y,
        bullet: None,
        aliens,
        alien_dx: ALIEN_SPEED,
        step_down: ALIEN_STEP_DOWN,
        tex,
    };

    // ******** MAIN LOOP ********
    loop {
        let dt = get_frame_time();
        game.handle_input(dt);
        game.update(dt);
        game.draw();

        next_frame().await;
    }
}