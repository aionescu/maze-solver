mod parser;
mod solver;

use std::env::args;
use std::path::Path;

use image::{open, GrayImage};

use timed_proc_macro::timed;
use crate::parser::parse;
use crate::solver::{solve, make_path, draw_path};

#[timed("Loading")]
fn load_luma8_parts(path: &str) -> (u32, u32, Vec<u8>) {
  let img = open(path).unwrap().into_luma8();
  (img.width(), img.height(), img.into_raw())
}

#[timed("Saving")]
fn save_luma8_parts(width: u32, height: u32, pixels: Vec<u8>, path: &str) {
  let img = GrayImage::from_raw(width, height, pixels).unwrap();
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

#[timed("Total")]
fn main() {
  let img_path = args().nth(1).expect("Please specify the maze file as a command-line argument.");
  let solution_path = append_solved(&img_path);

  let (width, height, mut pixels) = load_luma8_parts(&img_path);

  let (mut nodes, end) = parse(width, height, &pixels);
  solve(&mut nodes);

  let (path, path_length) = make_path(width, &nodes);
  draw_path(&mut pixels, &path, path_length, end);

  save_luma8_parts(width, height, pixels, &solution_path);
  println!("Solution saved to {}.", solution_path)
}
