use image::{GrayImage, ImageFormat};
use irrgarten::{Maze, MazeGenerationError};
use rand::rngs::ThreadRng;
use std::io::Cursor;
use thiserror::Error;
use wasm_bindgen::prelude::wasm_bindgen;

const CELL_SIZE: usize = 15;
const WALL_SIZE: usize = 2;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    MazeGeneration(#[from] MazeGenerationError),
    #[error(transparent)]
    Image(#[from] image::ImageError),
}

pub fn generate(w: usize, h: usize) -> Result<Vec<u8>, Error> {
    // generate the maze
    let maze = Maze::new(w * 2 + 1, h * 2 + 1)?.generate(&mut ThreadRng::default());
    let mut image = GrayImage::from_pixel(
        (w * CELL_SIZE + (w + 1) * WALL_SIZE) as u32,
        (h * CELL_SIZE + (h + 1) * WALL_SIZE) as u32,
        [u8::MAX].into(),
    );
    image.fill(255);

    // draw the maze
    let (mut x_offset, mut y_offset) = (0, 0);
    for y in 0..maze.height {
        for x in 0..maze.width {
            if maze[x][y] == 1 {
                for i in 0..if y % 2 == 0 { WALL_SIZE } else { CELL_SIZE } {
                    for j in 0..if x % 2 == 0 { WALL_SIZE } else { CELL_SIZE } {
                        image.put_pixel((x_offset + j) as u32, (y_offset + i) as u32, [0].into());
                    }
                }
            }
            x_offset += if x % 2 == 0 { WALL_SIZE } else { CELL_SIZE };
        }
        x_offset = 0;
        y_offset += if y % 2 == 0 { WALL_SIZE } else { CELL_SIZE };
    }

    // convert the image to a PNG
    let mut bytes: Vec<u8> = Vec::new();
    image.write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)?;
    Ok(bytes)
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsError;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn generate_wasm(w: usize, h: usize) -> Result<Vec<u8>, JsError> {
    Ok(generate(w, h)?)
}
