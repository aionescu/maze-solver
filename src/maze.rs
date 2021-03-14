use core::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

pub struct Maze {
  start: u32,
  pub end: u32,

  up_idx: HashMap<u32, u32>,
  down_idx: HashMap<u32, u32>,
  left_idx: HashMap<u32, u32>,
  right_idx: HashMap<u32, u32>,

  up_dst: HashMap<u32, u32>,
  left_dst: HashMap<u32, u32>,
  end_dst: HashMap<u32, u32>
}

pub fn parse(width: u32, height: u32, pixels: &[u8]) -> Maze {
  fn first_empty_pixel(pixels: &[u8], start_idx: usize, end_idx: usize) -> u32 {
    for i in start_idx .. end_idx {
      if pixels[i] != 0 {
        return i as u32
      }
    }

    panic!("Invalid maze.")
  }

  let width = width as usize;
  let height = height as usize;
  let img_size = width * height;

  let start = first_empty_pixel(pixels, 1, width - 1);
  let end = first_empty_pixel(pixels, img_size - height + 1, img_size - 1);

  let end_x = height as u32 - 1;
  let end_y = end % width as u32;

  let mut top_nodes = HashMap::new();
  let mut left_node = 0u32;

  let mut up_idx = HashMap::new();
  let mut down_idx = HashMap::new();
  let mut left_idx = HashMap::new();
  let mut right_idx = HashMap::new();

  let mut up_dst = HashMap::new();
  let mut left_dst = HashMap::new();
  let mut end_dst = HashMap::new();

  end_dst.insert(start, end_x + end_y);
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

    if down == img_size - 1 {
      break
    }

    if right % width == 0 {
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

  let up_node = end as usize - width;

  if pixels[up_node] != 0 {
    up_idx.insert(end, up_node as u32);
    down_idx.insert(up_node as u32, end);

    up_dst.insert(end, 1);
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

struct Node {
  f: u32,
  idx: u32
}

impl PartialEq for Node {
  fn eq(&self, rhs: &Self) -> bool {
    self.f == rhs.f && self.idx == rhs.idx
  }
}

impl Eq for Node { }

impl PartialOrd for Node {
  fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
    Some(self.cmp(rhs))
  }
}

impl Ord for Node {
  fn cmp(&self, rhs: &Self) -> Ordering {
    rhs.f.cmp(&self.f).then_with(|| self.idx.cmp(&rhs.idx))
  }
}

#[derive(Clone, Copy)]
pub enum Dir {
  Up,
  Down,
  Left,
  Right
}

pub fn solve(maze: &Maze) -> HashMap<u32, (u32, Dir)> {
  let &Maze{
    start,
    end,

    ref up_idx,
    ref down_idx,
    ref left_idx,
    ref right_idx,

    ref up_dst,
    ref left_dst,
    ref end_dst
  } = maze;

  let mut prev = HashMap::new();

  let mut g = HashMap::new();
  g.insert(start, 0);

  let mut heap = BinaryHeap::new();
  heap.push(Node { idx: start, f: end_dst[&start] });

  loop {
    if heap.is_empty() {
      break
    }

    let Node{ idx: crr, .. } = heap.pop().unwrap();

    if crr == end {
      return prev;
    }

    if let Some(&up) = up_idx.get(&crr) {
      let new_g = g[&crr] + up_dst[&crr];

      if !g.contains_key(&up) || new_g < g[&up] {
        g.insert(up, new_g);

        prev.insert(up, (crr, Dir::Down));
        heap.push(Node { idx: up, f: new_g + end_dst[&up] });
      }
    }

    if let Some(&down) = down_idx.get(&crr) {
      let new_g = g[&crr] + up_dst[&down];

      if !g.contains_key(&down) || new_g < g[&down] {
        g.insert(down, new_g);

        prev.insert(down, (crr, Dir::Up));
        heap.push(Node { idx: down, f: new_g + end_dst[&down] });
      }
    }

    if let Some(&left) = left_idx.get(&crr) {
      let new_g = g[&crr] + left_dst[&crr];

      if !g.contains_key(&left) || new_g < g[&left] {
        g.insert(left, new_g);

        prev.insert(left, (crr, Dir::Right));
        heap.push(Node { idx: left, f: new_g + end_dst[&left] });
      }
    }

    if let Some(&right) = right_idx.get(&crr) {
      let new_g = g[&crr] + left_dst[&right];

      if !g.contains_key(&right) || new_g < g[&right] {
        g.insert(right, new_g);

        prev.insert(right, (crr, Dir::Left));
        heap.push(Node { idx: right, f: new_g + end_dst[&right] });
      }
    }
  }

  panic!("No path found.")
}

pub fn make_path(maze: &Maze, width: u32, prev: &HashMap<u32, (u32, Dir)>) -> (Vec<(u32, i32)>, u64) {
  let i_width = width as i32;
  let neg_width = -i_width;

  let &Maze{
    end,
    ref up_dst,
    ref left_dst,
    ..
  } = maze;

  let mut path = Vec::new();
  let mut path_length = 0u64;

  let mut crr = end;

  while let Some(&(prev, dir)) = prev.get(&crr) {
    match dir {
      Dir::Up => {
        let dst = up_dst[&crr];
        path.push((dst, neg_width));
        path_length += dst as u64
      },
      Dir::Down => {
        let dst = up_dst[&prev];
        path.push((dst, i_width));
        path_length += dst as u64
      },
      Dir::Left => {
        let dst = left_dst[&crr];
        path.push((dst, -1));
        path_length += dst as u64
      },
      Dir::Right => {
        let dst = left_dst[&prev];
        path.push((dst, 1));
        path_length += dst as u64
      }
    }

    crr = prev
  }

  (path, path_length)
}

pub fn draw_path(pixels: &mut Vec<u8>, path: &Vec<(u32, i32)>, path_length: u64, end: u32) {
  let mut color = 64.0;
  let color_step = 128.0 / path_length as f64;

  let mut crr = end;

  for &(times, step) in path {

    for _ in 0 .. times {
      pixels[crr as usize] = color as u8;
      color += color_step;

      crr = (crr as i64 + step as i64) as u32;
    }
  }

  pixels[crr as usize] = color as u8
}
