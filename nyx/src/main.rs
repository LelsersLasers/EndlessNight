mod camera;
mod light;
mod vector;

use crate::{camera::Camera, light::Light, vector::Vector2D};

use macroquad::prelude as mq;

const START_WIDTH: i32 = 1440;
const START_HEIGHT: i32 = 810;

const DITHER: [i32; 16] = [0, 8, 2, 10, 12, 4, 14, 6, 3, 11, 1, 9, 15, 7, 13, 5];

fn window_conf() -> mq::Conf {
    mq::Conf {
        window_title: "Nyx".to_owned(),
        window_width: START_WIDTH,
        window_height: START_HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut image = mq::Image::gen_image_color(START_WIDTH as u16, START_HEIGHT as u16, mq::BLACK);
    let texture = mq::Texture2D::from_image(&image);

    // let dither = DITHER.iter().map(|&x| (x as f32 / 16.) - 0.5).collect::<Vec<f32>>();

    let mut lights: Vec<Light> = Vec::new();
    lights.push(Light::new(Vector2D::new(300., 300.), 15., 1., mq::YELLOW));

    loop {
        mq::clear_background(mq::BLACK);

        mq::draw_line(40.0, 40.0, 100.0, 200.0, 15.0, mq::BLUE);
        mq::draw_rectangle(
            mq::screen_width() / 2.0 - 60.0,
            100.0,
            120.0,
            60.0,
            mq::GREEN,
        );
        for light in lights.iter() {
            mq::draw_circle(light.pt.x, light.pt.y, 20., light.color);
        }

        mq::draw_text("HELLO", 20.0, 20.0, 30.0, mq::DARKGRAY);

        let image_data = image.get_image_data_mut();
        let screen_image = mq::get_screen_data();
        let pre_lighting = screen_image.get_image_data();

        for i in 0..START_WIDTH * START_HEIGHT {
            let x = i % START_WIDTH;
            let y = i / START_WIDTH;
            for light in lights.iter() {
                let dx = light.pt.x as i32 - x; 
                let dy = light.pt.y as i32 - y;
                let dist = ((dx * dx + dy * dy) as f32).sqrt();

                if dist < light.power * 2.
                    || dist / light.power
                        <= DITHER
                            [(((dy.unsigned_abs() % 4) * 4) + (dx.unsigned_abs() % 4)) as usize]
                            as f32
                {
                    // image_data[i as usize] = pre_lighting[i as usize];
                    image_data[i as usize] = mq::GRAY.into();
                    break;
                }
            }
        }

        texture.update(&image);
        mq::draw_texture_ex(texture, 0., 0., mq::WHITE, mq::DrawTextureParams {
            dest_size: Some(mq::vec2(START_WIDTH as f32, START_HEIGHT as f32)),
            flip_y: true,
            ..Default::default()
        });
        mq::next_frame().await
    }
}
