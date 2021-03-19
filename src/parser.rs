use std::collections::HashMap;

pub struct Maze {
  pub start: u32,
  pub end: u32,

  pub up_idx: HashMap<u32, u32>,
  pub down_idx: HashMap<u32, u32>,
  pub left_idx: HashMap<u32, u32>,
  pub right_idx: HashMap<u32, u32>,

  pub up_dst: HashMap<u32, u32>,
  pub left_dst: HashMap<u32, u32>,
  pub end_dst: HashMap<u32, u32>
}

fn first_empty_pixel(pixels: &[u8], start_idx: usize, end_idx: usize) -> u32 {
  for i in start_idx .. end_idx {
    if pixels[i] != 0 {
      return i as u32
    }
  }

  panic!("Invalid maze.")
}

pub fn parse(width: u32, height: u32, pixels: &[u8]) -> Maze {
  let max_y = width - 1;

  let width = width as usize;
  let height = height as usize;
  let img_size = width * height;

  let start = first_empty_pixel(pixels, 1, width - 1);
  let end = first_empty_pixel(pixels, img_size - width + 1, img_size - 1);

  let end_x = height as u32 - 1;
  let end_y = end % width as u32;
  let y_diff = (start as i32 - end_y as i32).abs() as u32;

  let mut top_nodes = HashMap::new();
  let mut left_node = 0u32;

  let mut up_idx = HashMap::new();
  let mut down_idx = HashMap::new();
  let mut left_idx = HashMap::new();
  let mut right_idx = HashMap::new();

  let mut up_dst = HashMap::new();
  let mut left_dst = HashMap::new();
  let mut end_dst = HashMap::new();

  end_dst.insert(start, end_x + y_diff);
  end_dst.insert(end, 0);

  top_nodes.insert(start, (start, 0));

  let mut crr = width;
  let mut left = crr - 1;
  let mut right = crr + 1;
  let mut up = 0;
  let mut down = crr + width;
  let mut y = 0u32;
  let mut x = 1u32;

  loop {
    crr += 1;
    left += 1;
    right += 1;
    up += 1;
    down += 1;
    y += 1;

    if y == max_y {
      if down == img_size - 1 {
        break
      }

      crr += 1;
      left += 1;
      right += 1;
      up += 1;
      down += 1;
      y = 0;
      x += 1;

      continue
    }

    if pixels[crr] == 0 {
      continue
    }

    let up_white = pixels[up] != 0;
    let left_white = pixels[left] != 0;
    let right_white = pixels[right] != 0;
    let down_white = pixels[down] != 0;

    if up_white == down_white && left_white == right_white && up_white != left_white {
      continue
    }

    if up_white {
      let (top_node, top_x) = top_nodes[&y];

      up_idx.insert(crr as u32, top_node);
      down_idx.insert(top_node, crr as u32);

      up_dst.insert(crr as u32, x - top_x);
    }

    if down_white {
      top_nodes.insert(y, (crr as u32, x));
    }

    if left_white {
      left_idx.insert(crr as u32, left_node);
      right_idx.insert(left_node, crr as u32);
      left_dst.insert(crr as u32, crr as u32 - left_node);
    }

    if right_white {
      left_node = crr as u32;
    }

    let y_diff = (end_y as i32 - y as i32).abs() as u32;
    end_dst.insert(crr as u32, end_x - x + y_diff);
  }

  if pixels[end as usize - width] != 0 {
    let (top_node, top_x) = top_nodes[&end_y];

    up_idx.insert(end, top_node);
    down_idx.insert(top_node, end);

    up_dst.insert(end, end_x - top_x);
  }

  Maze {
    start,
    end,

    up_idx,
    down_idx,
    left_idx,
    right_idx,

    up_dst,
    left_dst,
    end_dst
  }
}
