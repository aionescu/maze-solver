mod io;
mod parser;
mod solver;

use crate::io::*;
use crate::parser::*;
use crate::solver::*;

fn main() {
  let img_path =
    std::env::args()
    .nth(1)
    .expect("Please specify the maze file as a command-line argument.");

  let (width, height, mut pixels) = load_luma8_parts(&img_path);

  let maze = parse(width, height, &pixels);
  let prev = solve(&maze);

  let (path, path_length) = make_path(&maze, width, &prev);
  draw_path(&mut pixels, &path, path_length, maze.end);

  save_solved(width, height, pixels, &img_path)
}
