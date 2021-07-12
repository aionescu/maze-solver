use core::cmp::Ordering;
use std::collections::BinaryHeap;
use timed_proc_macro::timed;
use crate::parser::{Dir, Node};

#[derive(PartialEq, Eq)]
struct Entry {
  f: u32,
  idx: u32
}

impl PartialOrd for Entry {
  fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
    Some(self.cmp(rhs))
  }
}

impl Ord for Entry {
  fn cmp(&self, rhs: &Self) -> Ordering {
    rhs.f.cmp(&self.f).then_with(|| self.idx.cmp(&rhs.idx))
  }
}

#[timed("Solving")]
pub fn solve(nodes: &mut [Node]) {
  let mut heap = BinaryHeap::new();
  heap.push(Entry { idx: 1, f: nodes[1].end_dst });

  let end_idx = nodes.len() as u32 - 1;

  while let Some(Entry{ idx: crr_idx, .. }) = heap.pop() {
    if crr_idx == end_idx {
      return
    }

    let crr_node = nodes[crr_idx as usize];

    if crr_node.left_dst != 0 {
      let new_g = crr_node.g + crr_node.left_dst;
      let left_node = &mut nodes[crr_idx as usize - 1];

      if left_node.g > new_g {
        left_node.g = new_g;
        left_node.prev = Some(Dir::Right);

        heap.push(Entry { idx: crr_idx - 1, f: new_g + left_node.end_dst })
      }
    }

    if crr_node.right_dst != 0 {
      let new_g = crr_node.g + crr_node.right_dst;
      let right_node = &mut nodes[crr_idx as usize + 1];

      if right_node.g > new_g {
        right_node.g = new_g;
        right_node.prev = Some(Dir::Left);

        heap.push(Entry { idx: crr_idx + 1, f: new_g + right_node.end_dst })
      }
    }

    if crr_node.up_dst != 0 {
      let new_g = crr_node.g + crr_node.up_dst;
      let up_node = &mut nodes[crr_node.up_idx as usize];

      if up_node.g > new_g {
        up_node.g = new_g;
        up_node.prev = Some(Dir::Down);

        heap.push(Entry { idx: crr_node.up_idx, f: new_g + up_node.end_dst })
      }
    }

    if crr_node.down_dst != 0 {
      let new_g = crr_node.g + crr_node.down_dst;
      let down_node = &mut nodes[crr_node.down_idx as usize];

      if down_node.g > new_g {
        down_node.g = new_g;
        down_node.prev = Some(Dir::Up);

        heap.push(Entry { idx: crr_node.down_idx, f: new_g + down_node.end_dst })
      }
    }
  }

  panic!("No path found.")
}

#[timed("Creating path")]
pub fn make_path(width: u32, nodes: &[Node]) -> (Vec<(u32, i32)>, u64) {
  let i_width = width as i32;
  let neg_width = -i_width;

  let mut path = vec![];
  let mut path_length = 0u64;

  let mut crr_idx = nodes.len() - 1;
  let mut crr_node = nodes[crr_idx];

  while let Some(dir) = crr_node.prev {
    let dst;
    let step;

    match dir {
      Dir::Up => {
        dst = crr_node.up_dst;
        step = neg_width;
        crr_idx = crr_node.up_idx as usize
      },
      Dir::Down => {
        dst = crr_node.down_dst;
        step = i_width;
        crr_idx = crr_node.down_idx as usize
      },
      Dir::Left => {
        dst = crr_node.left_dst;
        step = -1;
        crr_idx = crr_idx - 1
      },
      Dir::Right => {
        dst = crr_node.right_dst;
        step = 1;
        crr_idx = crr_idx + 1
      }
    }

    path.push((dst, step));
    path_length += dst as u64;
    crr_node = nodes[crr_idx]
  }

  (path, path_length)
}

#[timed("Drawing path")]
pub fn draw_path(pixels: &mut Vec<u8>, path: &Vec<(u32, i32)>, path_length: u64, end: u32) {
  let mut color = 64.0;
  let color_step = 128.0 / path_length as f64;

  let mut crr_idx = end;

  for &(times, step) in path {
    for _ in 0 .. times {
      pixels[crr_idx as usize] = color as u8;
      color += color_step;

      crr_idx = (crr_idx as i64 + step as i64) as u32
    }
  }

  pixels[crr_idx as usize] = color as u8
}
