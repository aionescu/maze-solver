mod io;
mod parser;
mod solver;

use std::time::Instant;

use crate::io::*;
use crate::parser::*;
use crate::solver::*;

macro_rules! timed {
  ($s:expr, $e:expr) => {
    {
      let now = Instant::now();
      let r = $e;
      let elapsed = now.elapsed();
      println!("{} took {:?}.", $s, elapsed);
      r
    }
  }
}

fn main() {
  let img_path =
    std::env::args()
    .nth(1)
    .expect("Please specify the maze file as a command-line argument.");

  let solution_path = timed!("Total", {
    let (width, height, mut pixels) = timed!("Loading", load_luma8_parts(&img_path));

    let (nodes, end) = timed!("Parsing", parse(width, height, &pixels));
    let prev = timed!("Solving", solve(&nodes));

    let (path, path_length) = timed!("Creating path", make_path(width, &nodes, &prev));
    timed!("Drawing path", draw_path(&mut pixels, &path, path_length, end));

    timed!("Saving", save_solved(width, height, pixels, &img_path))
  });

  println!("Solution saved to {}.", solution_path);
}
