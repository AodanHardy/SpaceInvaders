use macroquad::prelude::*;

// constants
const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
const PLAYER_SPEED: f32 = 300.0;

// asset file paths
const PLAYER_IMAGE: &str = "assets/player.png";
const ENEMY_IMAGE: &str = "assets/enemy.png";
const BULLET_IMAGE: &str = "assets/bullet.png";
const BACKGROUND_IMAGE: &str = "assets/background.png";
const LASER_SOUND: &str = "assets/laser.wav";
const EXPLOSION_SOUND: &str = "assets/explosion.wav";
const BACKGROUND_MUSIC: &str = "assets/background.wav";

fn window_conf() -> Conf {
    Conf {
        window_title: "Space Invaders (Rust)".to_string(),
        window_width: 800,
        window_height: 600,
        high_dpi: true,
        ..Default::default()
    }
}

// game loop
#[macroquad::main(window_conf)]
async fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use macroquad::file::set_pc_assets_folder;
        set_pc_assets_folder("assets");
    }

    loop {
        clear_background(BLACK);

        draw_text(
            "Space Invaders — skeleton",
            20.0,
            40.0,
            32.0,
            WHITE,
        );
        draw_text(
            &format!("FPS: {}", get_fps()),
            20.0,
            70.0,
            24.0,
            GRAY,
        );

        next_frame().await;
    }
}