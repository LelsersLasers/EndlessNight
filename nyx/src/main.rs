mod camera;
mod light;
mod vector;
mod light_modes;

use crate::{/*camera::Camera,*/ light::Light, vector::Vector2D, light_modes::LightMode};

use macroquad::prelude as mq;

const PX_WIDTH: u32 = 256;
const PX_HEIGHT: u32 = 144;

const START_WIDTH: u32 = 1440;
const START_HEIGHT: u32 = 810;

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
    let mut camera =
        mq::Camera2D::from_display_rect(mq::Rect::new(0.0, 0.0, PX_WIDTH as f32, PX_HEIGHT as f32));
    camera.render_target = Some(mq::render_target(PX_WIDTH as u32, PX_HEIGHT as u32));
    camera
        .render_target
        .unwrap()
        .texture
        .set_filter(mq::FilterMode::Nearest);

    let mut lights: Vec<Light> = Vec::new();
    lights.push(Light::new(Vector2D::new(40., 40.), 4.25, LightMode::Sin(0.05, 3., 0.), mq::GRAY,));
    lights.push(Light::new(Vector2D::new(10., 10.), 1.2, LightMode::Sin(0.03, 5., 0.), mq::GRAY));

    loop {
        let delta = mq::get_frame_time();

        let draw_width = mq::screen_width().min(mq::screen_height() * 16. / 9.);
        let draw_height = mq::screen_height().min(mq::screen_width() * 9. / 16.);

        let top_offset = (mq::screen_height() - draw_height) / 2.;
        let left_offset = (mq::screen_width() - draw_width) / 2.;

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
        mq::draw_rectangle(100., 60., 15., 20., mq::GREEN);

        for light in lights.iter() {
            mq::draw_rectangle(light.pt.x - 1., light.pt.y - 1., 2., 2., mq::YELLOW);
        }

        mq::set_camera(&mq::Camera2D::from_display_rect(mq::Rect::new(
            0.,
            0.,
            mq::screen_width(),
            mq::screen_height(),
        )));

        mq::clear_background(mq::WHITE);

        let image_in = camera.render_target.unwrap().texture.get_texture_data();
        let mut image_out =
            mq::Image::gen_image_color(PX_WIDTH as u16, PX_HEIGHT as u16, mq::BLACK);

        let light_powers = lights.iter().map(|light| light.get_power(mq::get_time() as f32)).collect::<Vec<f32>>();

        for x in 0..PX_WIDTH {
            for y in 0..PX_HEIGHT {
                let src_y = PX_HEIGHT - y - 1;
                for (i, light) in lights.iter().enumerate() {
                    let dx = light.pt.x as i32 - x as i32;
                    let dy = light.pt.y as i32 - y as i32;
                    let dist = ((dx * dx + dy * dy) as f32).sqrt();

                    // get_power(mq::get_time() as f32)

                    if dist < light_powers[i] * 4.
                        || dist / light_powers[i]
                            <= DITHER[(((dy.unsigned_abs() % DITHER_SIZE) * DITHER_SIZE)
                                + (dx.unsigned_abs() % DITHER_SIZE))
                                as usize] as f32
                    {
                        let screen_px_color = image_in.get_pixel(x, src_y);
                        image_out.set_pixel(
                            x,
                            y,
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
        }
        // overwrite the texture with the new image
        camera.render_target.unwrap().texture.update(&image_out);

        mq::draw_texture_ex(
            camera.render_target.unwrap().texture,
            left_offset,
            top_offset,
            mq::WHITE,
            mq::DrawTextureParams {
                dest_size: Some(mq::vec2(draw_width, draw_height)),
                ..Default::default()
            },
        );

        mq::draw_text(
            &(format!("FPS {:.0}", 1. / delta)),
            20.0 + left_offset,
            20.0 + top_offset,
            30.0,
            mq::WHITE,
        );
        mq::next_frame().await
    }
}
