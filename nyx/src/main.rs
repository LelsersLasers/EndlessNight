mod camera_manager;
mod light;
mod light_modes;
mod player;

use crate::{
    camera_manager::CameraManager, light::Light, light_modes::LightMode, player::Player,
};

use macroquad::prelude as mq;
use rand::Rng;

const PX_WIDTH: u32 = 256;
const PX_HEIGHT: u32 = 144;

const START_WIDTH: u32 = 1440;
const START_HEIGHT: u32 = 810;

const MAZE_SIZE: f32 = 101.;
const MAZE_START: mq::Vec2 = mq::vec2(
    ((MAZE_SIZE / 2.) as u32) as f32,
    ((MAZE_SIZE / 2.) as u32) as f32,
);

const PLAYER_START: mq::Vec2 = mq::vec2(PX_WIDTH as f32 / 2., PX_HEIGHT as f32 / 2.);

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

fn create_maze() -> mq::Image {
    let neighbor_offsets = [
        mq::Vec2::new(0., -2.),
        mq::Vec2::new(2., 0.),
        mq::Vec2::new(0., 2.),
        mq::Vec2::new(-2., 0.),
    ];
    let mut stack: Vec<mq::Vec2> = vec![MAZE_START];

    let mut maze_image =
        mq::Image::gen_image_color(MAZE_SIZE as u16, MAZE_SIZE as u16, COLOR_WHITE);

    let mut rng = rand::thread_rng();
    let mut first = true;

    while !stack.is_empty() {
        let current_cell = stack.pop().unwrap();
        let offset_locs = neighbor_offsets
            .iter()
            .map(|offset| current_cell + *offset)
            .filter(|new_pos| {
                new_pos.x >= 1. && new_pos.x < MAZE_SIZE - 1. && new_pos.y >= 1. && new_pos.y < MAZE_SIZE - 1.
            })
            .filter(|new_pos| {
                maze_image.get_pixel(new_pos.x as u32, new_pos.y as u32) == COLOR_WHITE
            })
            .collect::<Vec<mq::Vec2>>();

        if !offset_locs.is_empty() {
            stack.push(current_cell);
            if first {
                first = false;
            } else {
                maze_image.set_pixel(current_cell.x as u32, current_cell.y as u32, COLOR_BLACK);
            }

            let offset_loc = offset_locs[rng.gen_range(0..offset_locs.len())];
            let offset = offset_loc - current_cell;

            let new_pos = current_cell + offset;
            stack.push(new_pos);
            let wall_pos = current_cell + offset / 2.;
            maze_image.set_pixel(new_pos.x as u32, new_pos.y as u32, COLOR_BLACK);
            maze_image.set_pixel(wall_pos.x as u32, wall_pos.y as u32, COLOR_BLACK);
        }
    }
    // maze_image.set_pixel(MAZE_START.x as u32, MAZE_START.y as u32, mq::BLUE);
    maze_image.export_png("maze.png");

    maze_image
}

#[macroquad::main(window_conf)]
async fn main() {
    // ---------------------------------------------------------------------- //
    let mut camera =
        mq::Camera2D::from_display_rect(mq::Rect::new(0.0, 0.0, PX_WIDTH as f32, PX_HEIGHT as f32));
    camera.render_target = Some(mq::render_target(PX_WIDTH as u32, PX_HEIGHT as u32));
    camera
        .render_target
        .unwrap()
        .texture
        .set_filter(mq::FilterMode::Nearest);

    let mut cm = CameraManager::new(mq::Vec2::ZERO, -PLAYER_START);
    // ---------------------------------------------------------------------- //

    // ---------------------------------------------------------------------- //
    let maze_image = create_maze();
    let maze_texture = mq::Texture2D::from_image(&maze_image);
    maze_texture.set_filter(mq::FilterMode::Nearest);
    // ---------------------------------------------------------------------- //

    let font = mq::load_ttf_font("assets/AnnieUseYourTelescope.ttf")
        .await
        .unwrap();

    let mut player = Player::new(
        PLAYER_START,
        Light::new(
            mq::Vec2::ZERO,
            3.5,
            LightMode::Sin(0.15, 3., 0.),
            COLOR_GREY,
        ),
    );
    player.update_light_pt();

    let mut lights: Vec<Light> = vec![Light::new(
        mq::vec2(10., 10.),
        1.2,
        LightMode::Sin(0.05, 5., 0.),
        COLOR_GREY,
    )];

    let mut objects: Vec<mq::Rect> = vec![
        mq::Rect::new(0., 0., 20., 30.),
        mq::Rect::new(100., 60., 15., 20.),
    ];

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

        // mq::draw_line(0., 0., 30., 25., 20.0, COLOR_WHITE);
        // mq::draw_rectangle(100., 60., 15., 20., COLOR_WHITE);
        for object in objects.iter() {
            let obj_pt = cm.calc_offset(mq::vec2(object.x, object.y));
            mq::draw_rectangle(
                obj_pt.x,
                obj_pt.y,
                object.w,
                object.h,
                COLOR_WHITE,
            );
        }

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

        let mut draw_lights = lights.clone();
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

        // mq::draw_texture_ex(
        //     maze_texture,
        //     left_offset,
        //     top_offset,
        //     mq::WHITE,
        //     mq::DrawTextureParams {
        //         dest_size: Some(mq::vec2(draw_width, draw_height)),
        //         ..Default::default()
        //     },
        // );

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
