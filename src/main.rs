mod io;
mod maze;

use crate::io::*;
use crate::maze::*;

fn main() {
  let img_path = std::env::args().nth(1).expect("Please specify the maze file as a command-line argument.");
  let (width, height, pixels) = load_luma8_parts(&img_path);

  match solve(width, height, &pixels) {
    None => println!("No path found."),
    Some((prev, end)) => draw_and_save(width, height, pixels, &img_path, &prev, end)
  }
}
