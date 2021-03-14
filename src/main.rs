mod io;
mod maze;

use crate::io::*;
use crate::maze::*;

fn main() {
  let img_path =
    std::env::args()
    .nth(1)
    .expect("Please specify the maze file as a command-line argument.");

  let (width, height, mut pixels) = load_luma8_parts(&img_path);

  let (prev, end) = solve(width, height, &pixels);

  draw_path(&mut pixels, &prev, end);
  save_solved(width, height, pixels, &img_path)
}
