use std::path::Path;
use image;

pub fn load_luma8_parts(file: &str) -> (u32, u32, Vec<u8>) {
  let img = image::open(file).unwrap();

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

fn append_solved(path: &str) -> String {
  let path = Path::new(&path);
  let parent = path.parent().unwrap();
  let stem = path.file_stem().unwrap().to_str().unwrap();
  let ext = path.extension().unwrap().to_str().unwrap();
  format!("{}/{}-solved.{}", parent.display(), stem, ext)
}

pub fn draw_and_save(width: u32, height: u32, mut pixels: Vec<u8>, path: &[u32], file: &str) {
  let mut color = 192f64;
  let step = 128f64 / path.len() as f64;

  path.iter().for_each(|&pixel| {
    pixels[pixel as usize] = color as u8;
    color -= step;
  });

  let img = image::GrayImage::from_raw(width , height , pixels).unwrap();
  img.save(&append_solved(file)).unwrap();
}
