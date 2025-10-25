use macroquad::prelude::*;

// constants
const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;

// Player
const PLAYER_SPEED: i32 = 300;
const PLAYER_START_POS_X: f32 = (SCREEN_WIDTH / 2) as f32;
const PLAYER_START_POS_Y: f32 = 500.0;

// asset file paths
const PLAYER_IMAGE: &str = "player.png";
const ENEMY_IMAGE: &str = "enemy.png";
const BULLET_IMAGE: &str = "bullet.png";
const BACKGROUND_IMAGE: &str = "background.png";
const LASER_SOUND: &str = "laser.wav";
const EXPLOSION_SOUND: &str = "explosion.wav";
const BACKGROUND_MUSIC: &str = "background.wav";



fn window_conf() -> Conf {
    Conf {
        window_title: "Space Invaders (Rust)".to_string(),
        window_width: SCREEN_WIDTH,
        window_height: SCREEN_HEIGHT,
        high_dpi: true,
        ..Default::default()
    }
}

// main window
#[macroquad::main(window_conf)]
async fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use macroquad::file::set_pc_assets_folder;
        set_pc_assets_folder("assets");
    }

    // variables
    let player = load_texture(PLAYER_IMAGE).await.unwrap();
    let mut player_x:f32 = (PLAYER_START_POS_X) - (player.width()/2.0);

    // main loop
    loop {
        clear_background(BLACK);
        draw_texture(
            &player,
            player_x,
            PLAYER_START_POS_Y,
             WHITE
        );

        next_frame().await;
    }
}