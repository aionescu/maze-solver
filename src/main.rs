mod parser;
mod solver;

use std::env::args;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use image::{open, GrayImage};

use timed_proc_macro::timed;
use crate::parser::parse;
use crate::solver::{solve, make_path, draw_path};

#[timed("Loading")]
fn load_luma8_parts<P: AsRef<Path>>(path: P) -> (u32, u32, Vec<u8>) {
  let img = open(path).unwrap().into_luma8();
  (img.width(), img.height(), img.into_raw())
}

#[timed("Saving")]
fn save_luma8_parts<P: AsRef<Path>>(width: u32, height: u32, pixels: Vec<u8>, path: P) {
  let img = GrayImage::from_raw(width, height, pixels).unwrap();
  img.save(&path).unwrap()
}

fn append_solved(path: &str) -> PathBuf {
  let path = Path::new(&path);

  let stem = path.file_stem().and_then(OsStr::to_str).unwrap_or("");
  let ext = path.extension().and_then(OsStr::to_str).unwrap_or("");

  path
    .with_file_name(format!("{}-Solved", stem))
    .with_extension(ext)
    .to_owned()
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
  println!("Solution saved to {}.", solution_path.display())
}
