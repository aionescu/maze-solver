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
  println!("Img loaded.");

  let maze = parse(width, height, &pixels);
  let end = maze.end;
  println!("Parsed.");

  let prev = solve(&maze);
  println!("Solved.");

  let (path, path_length) = make_path(&maze, width, &prev);
  println!("Path computed.");

  draw_path(&mut pixels, &path, path_length, end);
  println!("Path drawn.");

  save_solved(width, height, pixels, &img_path)
}
