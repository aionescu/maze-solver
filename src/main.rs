mod parser;
mod solver;

use std::env::args;
use std::path::Path;
use std::time::Instant;

use image;

use crate::parser::parse;
use crate::solver::{solve, make_path, draw_path};

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

fn load_luma8_parts(path: &str) -> (u32, u32, Vec<u8>) {
  let img = image::open(path).unwrap();

  let img =
    match img {
      image::ImageLuma8(luma8) => luma8,
      _ => img.to_luma()
    };

  (img.width(), img.height(), img.into_raw())
}

fn save_luma8_parts(width: u32, height: u32, pixels: Vec<u8>, path: &str) {
  let img = image::GrayImage::from_raw(width, height, pixels).unwrap();
  img.save(&path).unwrap()
}

fn append_solved(path: &str) -> String {
  let path = Path::new(&path);
  let parent = path.parent().unwrap().to_str().unwrap();
  let stem = path.file_stem().unwrap().to_str().unwrap();
  let ext = path.extension().unwrap().to_str().unwrap();

  if !parent.is_empty() {
    format!("{}/{}-Solved.{}", parent, stem, ext)
  } else {
    format!("{}-Solved.{}", stem, ext)
  }
}

fn main() {
  let img_path = args().nth(1).expect("Please specify the maze file as a command-line argument.");
  let solution_path = append_solved(&img_path);

  timed!("Total", {
    let (width, height, mut pixels) = timed!("Loading", load_luma8_parts(&img_path));

    let (nodes, end) = timed!("Parsing", parse(width, height, &pixels));
    let prev = timed!("Solving", solve(&nodes));

    let (path, path_length) = timed!("Creating path", make_path(width, &nodes, &prev));
    timed!("Drawing path", draw_path(&mut pixels, &path, path_length, end));

    timed!("Saving", save_luma8_parts(width, height, pixels, &solution_path))
  });

  println!("Solution saved to {}.", solution_path)
}
