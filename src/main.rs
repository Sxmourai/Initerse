use initerse::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "My game".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    color_eyre::install().unwrap();
    _main().await.unwrap()
}
