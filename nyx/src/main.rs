mod camera;
mod light;
mod vector;

use crate::{/*camera::Camera,*/ light::Light, vector::Vector2D};

use macroquad::prelude as mq;

const PX_WIDTH: u32 = 192;
const PX_HEIGHT: u32 = 108;

const START_WIDTH: u32 = PX_WIDTH * 8;
const START_HEIGHT: u32 = PX_HEIGHT * 8;

const DITHER: [i32; 16] = [0, 8, 2, 10, 12, 4, 14, 6, 3, 11, 1, 9, 15, 7, 13, 5];
const DITHER_SIZE: u32 = 4;

fn window_conf() -> mq::Conf {
    mq::Conf {
        window_title: "Nyx".to_owned(),
        window_width: START_WIDTH as i32,
        window_height: START_HEIGHT as i32,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut camera = mq::Camera2D::from_display_rect(mq::Rect::new(
        0.0,
        0.0,
        PX_WIDTH as f32,
        PX_HEIGHT as f32,
    ));
    camera.render_target = Some(mq::render_target(PX_WIDTH as u32, PX_HEIGHT as u32));
    camera.render_target.unwrap().texture.set_filter(mq::FilterMode::Nearest);

    let mut last_frame = instant::now();
    let mut delta: f32 = 1.0 / 60.0;

    let mut lights: Vec<Light> = Vec::new();
    lights.push(Light::new(Vector2D::new(40., 40.), 2., 1., mq::GRAY));
    lights.push(Light::new(Vector2D::new(10., 10.), 1.2, 1., mq::GRAY));

    loop {
        // ------------------------------------------------------------------ //
        let mut move_vec = Vector2D::new(0., 0.);
        if mq::is_key_down(mq::KeyCode::W) {
            move_vec.y -= 1.;
        }
        if mq::is_key_down(mq::KeyCode::S) {
            move_vec.y += 1.;
        }
        if mq::is_key_down(mq::KeyCode::A) {
            move_vec.x -= 1.;
        }
        if mq::is_key_down(mq::KeyCode::D) {
            move_vec.x += 1.;
        }
        lights[0].pt += move_vec.with_len(15. * delta);
        // ------------------------------------------------------------------ //

        mq::set_camera(&camera);

        mq::clear_background(mq::BLACK);

        mq::draw_line(0., 0., 30., 25., 20.0, mq::BLUE);
        mq::draw_rectangle(
            100.,
            60.,
            15.,
            20.,
            mq::GREEN,
        );

        for light in lights.iter() {
            mq::draw_circle(light.pt.x, light.pt.y, 2., mq::YELLOW);
        }

        mq::set_camera(&mq::Camera2D::from_display_rect(mq::Rect::new(
            0.,
            0.,
            START_WIDTH as f32,
            START_HEIGHT as f32,
        )));

        let image_in = camera.render_target.unwrap().texture.get_texture_data();
        let mut image_out =
            mq::Image::gen_image_color(PX_WIDTH as u16, PX_HEIGHT as u16, mq::BLACK);

        for i in 0..PX_WIDTH * PX_HEIGHT {
            let x = i % PX_WIDTH;
            let y = i / PX_WIDTH;
            let src_y = PX_HEIGHT - y - 1;

            for light in lights.iter() {
                let dx = light.pt.x as i32 - x as i32;
                let dy = light.pt.y as i32 - y as i32;
                let dist = ((dx * dx + dy * dy) as f32).sqrt();

                if dist < light.power * 4.
                    || dist / light.power
                        <= DITHER[(((dy.unsigned_abs() % DITHER_SIZE) * DITHER_SIZE)
                            + (dx.unsigned_abs() % DITHER_SIZE))
                            as usize] as f32
                {
                    let screen_px_color = image_in.get_pixel(x as u32, src_y as u32);
                    image_out.set_pixel(
                        x as u32,
                        y as u32,
                        if screen_px_color == mq::BLACK {
                            light.color
                        } else {
                            screen_px_color
                        },
                    );

                    break;
                }
            }
        }
        // overwrite the texture with the new image
        camera.render_target.unwrap().texture.update(&image_out);

        delta = (instant::now() - last_frame) as f32 / 1000.;
        last_frame = instant::now();

        mq::draw_texture_ex(
            camera.render_target.unwrap().texture,
            0.,
            0.,
            mq::WHITE,
            mq::DrawTextureParams {
                dest_size: Some(mq::vec2(START_WIDTH as f32, START_HEIGHT as f32)),
                ..Default::default()
            },
        );

        mq::draw_text(
            &(format!("FPS {:.0}", 1. / delta)),
            20.0,
            20.0,
            30.0,
            mq::WHITE,
        );
        mq::next_frame().await
    }
}
