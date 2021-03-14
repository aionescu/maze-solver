use std::path::Path;
use image;

pub fn load_luma8_parts(path: &str) -> (u32, u32, Vec<u8>) {
  let img = image::open(path).unwrap();

  let img =
    match img {
      image::ImageLuma8(luma8) => luma8,
      _ => img.to_luma()
    };

  let width = img.width();
  let height = img.height();
  let pixels = img.into_raw();

  (width, height, pixels)
}

pub fn save_solved(width: u32, height: u32, pixels: Vec<u8>, path: &str) {
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

  let img = image::GrayImage::from_raw(width, height, pixels).unwrap();
  let solution_path = append_solved(path);

  img.save(&solution_path).unwrap();
  println!("Solution saved to {}.", solution_path)
}
