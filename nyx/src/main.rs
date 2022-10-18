mod vector;
mod light;
mod camera;

use macroquad::prelude as mq;

fn window_conf() -> mq::Conf {
    mq::Conf {
        window_title: "Nyx".to_owned(),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        mq::clear_background(mq::LIGHTGRAY);

        mq::draw_line(40.0, 40.0, 100.0, 200.0, 15.0, mq::BLUE);
        mq::draw_rectangle(mq::screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, mq::GREEN);
        mq::draw_circle(mq::screen_width() - 30.0, mq::screen_height() - 30.0, 15.0, mq::YELLOW);

        mq::draw_text("HELLO", 20.0, 20.0, 30.0, mq::DARKGRAY);

        mq::next_frame().await
    }
}
