use macroquad::prelude as mq;

fn create_maze_map(
    maze_size: f32,
    maze_start: mq::Vec2,
    color_white: mq::Color,
    color_black: mq::Color,
) -> mq::Image {
    let neighbor_offsets = [
        mq::Vec2::new(0., -2.),
        mq::Vec2::new(2., 0.),
        mq::Vec2::new(0., 2.),
        mq::Vec2::new(-2., 0.),
    ];
    let mut stack: Vec<mq::Vec2> = vec![maze_start];

    let mut maze_image =
        mq::Image::gen_image_color(maze_size as u16, maze_size as u16, color_white);

    let mut first = true;

    while !stack.is_empty() {
        let current_cell = stack.pop().unwrap();
        let offset_locs = neighbor_offsets
            .iter()
            .map(|offset| current_cell + *offset)
            .filter(|new_pos| {
                new_pos.x >= 1.
                    && new_pos.x < maze_size - 1.
                    && new_pos.y >= 1.
                    && new_pos.y < maze_size - 1.
            })
            .filter(|new_pos| {
                maze_image.get_pixel(new_pos.x as u32, new_pos.y as u32) == color_white
            })
            .collect::<Vec<mq::Vec2>>();

        if !offset_locs.is_empty() {
            stack.push(current_cell);
            if first {
                first = false;
            } else {
                maze_image.set_pixel(current_cell.x as u32, current_cell.y as u32, color_black);
            }

            let offset_loc = offset_locs[mq::rand::gen_range(0, offset_locs.len())];
            let offset = offset_loc - current_cell;

            let new_pos = current_cell + offset;
            stack.push(new_pos);
            let wall_pos = current_cell + offset / 2.;
            maze_image.set_pixel(new_pos.x as u32, new_pos.y as u32, color_black);
            maze_image.set_pixel(wall_pos.x as u32, wall_pos.y as u32, color_black);
        }
    }
    // maze_image.set_pixel(MAZE_START.x as u32, MAZE_START.y as u32, mq::BLUE);
    // maze_image.export_png("maze.png");

    maze_image
}

pub fn create_maze_texture(
    maze_size: f32,
    maze_tile_size: f32,
    maze_start: mq::Vec2,
    color_white: mq::Color,
    color_black: mq::Color,
) -> mq::Image {
    let maze_map = create_maze_map(maze_size, maze_start, color_white, color_black);
    let mut maze_texture = mq::Image::gen_image_color(
        maze_size as u16 * maze_tile_size as u16,
        maze_size as u16 * maze_tile_size as u16,
        color_white,
    );
    for x in 0..maze_size as u32 {
        for y in 0..maze_size as u32 {
            if maze_map.get_pixel(x, y) == color_black {
                for tile_x in 0..maze_tile_size as u32 {
                    for tile_y in 0..maze_tile_size as u32 {
                        if mq::rand::gen_range(0, 50) != 0 {
                            maze_texture.set_pixel(
                                x * maze_tile_size as u32 + tile_x,
                                y * maze_tile_size as u32 + tile_y,
                                color_black,
                            );
                        }
                    }
                }
            }
        }
    }

    maze_texture
}
