use initerse::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "My game".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}
fn main() {
    color_eyre::install().unwrap();
    macroquad::Window::from_config(window_conf(), async {_main().await.unwrap()});
}
