mod camera_manager;
mod light;
mod light_modes;
mod maze;
mod player;
mod toggle;

use crate::{camera_manager::CameraManager, light::Light, light_modes::LightMode, player::Player};

use macroquad::prelude as mq;

const PX_WIDTH: u32 = 256;
const PX_HEIGHT: u32 = 144;

const START_WIDTH: u32 = 1440;
const START_HEIGHT: u32 = 810;

const MAZE_SIZE: f32 = 101.;
const MAZE_START: mq::Vec2 = mq::vec2(
    ((MAZE_SIZE / 2.) as u32) as f32,
    ((MAZE_SIZE / 2.) as u32) as f32,
);
const MAZE_TILE_SIZE: f32 = 50.;

const PLAYER_W: f32 = 8.;
const PLAYER_H: f32 = 10.;
const PLAYER_START: mq::Vec2 = mq::vec2(
    (PX_WIDTH as f32 - PLAYER_W) / 2.,
    (PX_HEIGHT as f32 - PLAYER_H) / 2.,
);

const MAZE_PT: mq::Vec2 = mq::vec2(
    -MAZE_SIZE * MAZE_TILE_SIZE / 2. + PLAYER_START.x,
    -MAZE_SIZE * MAZE_TILE_SIZE / 2. + PLAYER_START.y,
);

const DITHER: [i32; 16] = [0, 8, 2, 10, 12, 4, 14, 6, 3, 11, 1, 9, 15, 7, 13, 5];
const DITHER_SIZE: u32 = 4;

// const COLOR_WHITE: mq::Color = mq::Color::new(236. / 255., 239. / 255., 244. / 255., 1.);
// const COLOR_BLACK: mq::Color = mq::Color::new(40. / 255., 42. / 255., 54. / 255., 1.);
// const COLOR_GREY: mq::Color = mq::Color::new(68. / 255., 71. / 255., 90. / 255., 1.);

const COLOR_WHITE: mq::Color = mq::Color::new(1., 1., 1., 1.);
const COLOR_GREY: mq::Color = mq::Color::new(0.1, 0.1, 0.1, 1.);
const COLOR_BLACK: mq::Color = mq::Color::new(0., 0., 0., 1.);
const COLOR_GOLD: mq::Color = mq::Color::new(235. / 255., 203. / 255., 139. / 255., 1.);

fn window_conf() -> mq::Conf {
    mq::Conf {
        window_title: "Nyx".to_owned(),
        window_width: START_WIDTH as i32,
        window_height: START_HEIGHT as i32,
        window_resizable: true,
        ..Default::default()
    }
}

fn px_to_screen(x: f32, ratio: f32, offset: f32) -> f32 {
    x * ratio + offset
}

fn dither_idx(x: u32, y: u32) -> usize {
    ((y % DITHER_SIZE) * DITHER_SIZE + (x % DITHER_SIZE)) as usize
}

#[macroquad::main(window_conf)]
async fn main() {
    // ---------------------------------------------------------------------- //

    mq::rand::srand(instant::now() as u64);

    let mut camera =
        mq::Camera2D::from_display_rect(mq::Rect::new(0.0, 0.0, PX_WIDTH as f32, PX_HEIGHT as f32));
    camera.render_target = Some(mq::render_target(PX_WIDTH as u32, PX_HEIGHT as u32));
    camera
        .render_target
        .unwrap()
        .texture
        .set_filter(mq::FilterMode::Nearest);

    let font = mq::load_ttf_font("assets/AnnieUseYourTelescope.ttf")
        .await
        .unwrap();

    let mut cm = CameraManager::new(mq::Vec2::ZERO, -PLAYER_START);
    // ---------------------------------------------------------------------- //

    // ---------------------------------------------------------------------- //
    let maze_image = maze::create_maze_texture(
        MAZE_SIZE,
        MAZE_TILE_SIZE,
        MAZE_START,
        COLOR_WHITE,
        COLOR_BLACK,
    );
    let maze_texture = mq::Texture2D::from_image(&maze_image);
    maze_texture.set_filter(mq::FilterMode::Nearest);
    // ---------------------------------------------------------------------- //

    let mut player = Player::new(
        PLAYER_START - mq::vec2(PLAYER_W, PLAYER_H) / 2.,
        PLAYER_W,
        PLAYER_H,
        Light::new(
            mq::Vec2::ZERO,
            3.6,
            LightMode::Sin(0.15, 4., 0.),
            COLOR_GREY,
        ),
    );
    player.update_light_pt();

    let /*mut*/ lights: Vec<Light> = vec![Light::new(
        PLAYER_START,
        1.2,
        LightMode::Sin(0.05, 5., 0.),
        COLOR_GREY,
    )];

    // let mut objects: Vec<mq::Rect> = vec![
    //     mq::Rect::new(0., 0., 20., 30.),
    //     mq::Rect::new(100., 60., 15., 20.),
    // ];

    loop {
        let delta = mq::get_frame_time();

        // ------------------------------------------------------------------ //
        let draw_width = mq::screen_width().min(mq::screen_height() * 16. / 9.);
        let draw_height = mq::screen_height().min(mq::screen_width() * 9. / 16.);
        let ratio = draw_width / PX_WIDTH as f32;

        let top_offset = (mq::screen_height() - draw_height) / 2.;
        let left_offset = (mq::screen_width() - draw_width) / 2.;
        // ------------------------------------------------------------------ //

        // ------------------------------------------------------------------ //
        player.update(&mut cm, delta); // moves player

        // ------------------------------------------------------------------ //

        // ------------------------------------------------------------------ //
        mq::set_camera(&camera);
        mq::clear_background(COLOR_BLACK);

        let maze_pt = cm.calc_offset(MAZE_PT);
        mq::draw_texture_ex(
            maze_texture,
            maze_pt.x,
            maze_pt.y,
            mq::WHITE,
            mq::DrawTextureParams {
                dest_size: Some(mq::Vec2::splat(MAZE_SIZE * MAZE_TILE_SIZE)),
                flip_y: true,
                ..Default::default()
            },
        );

        // for object in objects.iter() {
        //     let obj_pt = cm.calc_offset(mq::vec2(object.x, object.y));
        //     mq::draw_rectangle(
        //         obj_pt.x,
        //         obj_pt.y,
        //         object.w,
        //         object.h,
        //         COLOR_WHITE,
        //     );
        // }

        for light in lights.iter() {
            let light_pt = cm.calc_offset(light.pt);
            mq::draw_rectangle(light_pt.x - 1., light_pt.y - 1., 2., 2., COLOR_GOLD);
        }

        player.draw(COLOR_GOLD, &cm);
        // ------------------------------------------------------------------ //

        mq::set_camera(&mq::Camera2D::from_display_rect(mq::Rect::new(
            0.,
            0.,
            mq::screen_width(),
            mq::screen_height(),
        )));
        mq::clear_background(COLOR_GOLD);

        // ------------------------------------------------------------------ //
        let image_in = camera.render_target.unwrap().texture.get_texture_data();
        let mut image_out =
            mq::Image::gen_image_color(PX_WIDTH as u16, PX_HEIGHT as u16, COLOR_BLACK);

        let mut draw_lights: Vec<Light> = lights
            .clone()
            .into_iter()
            .filter(|light| {
                let p = light.power * 16.;
                let pt = cm.calc_offset(light.pt);
                p + pt.x > 0.
                    && p + pt.y > 0.
                    && pt.x - p < PX_WIDTH as f32
                    && pt.y - p < PX_HEIGHT as f32
            })
            .collect();
        draw_lights.push(player.light);

        let light_powers = draw_lights
            .iter()
            .map(|light| light.calc_power(mq::get_time() as f32))
            .collect::<Vec<f32>>();
        let light_pts = draw_lights
            .iter()
            .map(|light| cm.calc_offset(light.pt))
            .collect::<Vec<mq::Vec2>>();

        for x in 0..PX_WIDTH {
            for y in 0..PX_HEIGHT {
                let src_y = PX_HEIGHT - y - 1;
                for (i, light) in draw_lights.iter().enumerate() {
                    let dx = light_pts[i].x as i32 - x as i32;
                    let dy = light_pts[i].y as i32 - y as i32;
                    let dist = ((dx * dx + dy * dy) as f32).sqrt();

                    if dist < light_powers[i] * 4.
                        || dist / light_powers[i] <= DITHER[dither_idx(x, y)] as f32
                    {
                        // if true {
                        let screen_px_color = image_in.get_pixel(x, src_y);
                        image_out.set_pixel(
                            x,
                            y,
                            if screen_px_color == COLOR_BLACK {
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
        // ------------------------------------------------------------------ //

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

        // ------------------------------------------------------------------ //
        let font_size = (7. * ratio) as u16;
        let text_str = format!("FPS {:.0}", mq::get_fps());
        // let text_size = mq::measure_text(&text_str, Some(font), font_size, 1.);
        mq::draw_text_ex(
            &text_str,
            px_to_screen(1., ratio, left_offset),
            px_to_screen(PX_HEIGHT as f32 - 1., ratio, top_offset),
            mq::TextParams {
                font,
                color: COLOR_WHITE,
                font_size,
                ..Default::default()
            },
        );
        // ------------------------------------------------------------------ //

        mq::next_frame().await
    }
}
