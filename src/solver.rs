use core::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::parser::*;

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

#[derive(Clone, Copy)]
pub enum Dir {
  Up,
  Down,
  Left,
  Right
}

pub fn solve(nodes: &[Node]) -> Vec<(u32, Dir)> {
  let mut prev = vec![(0, Dir::Up); nodes.len()];

  let mut g = vec![u32::MAX; nodes.len()];
  g[1] = 0;

  let mut heap = BinaryHeap::new();
  heap.push(Entry { idx: 1, f: nodes[1].end_dst });

  let end_idx = nodes.len() as u32 - 1;

  while let Some(Entry{ idx: crr_idx, .. }) = heap.pop() {
    if crr_idx == end_idx {
      return prev
    }

    let g_crr = g[crr_idx as usize];
    let crr_node = nodes[crr_idx as usize];

    if crr_node.up_idx != 0 {
      let new_g = g_crr + crr_node.up_dst;

      if g[crr_node.up_idx as usize] > new_g {
        g[crr_node.up_idx as usize] = new_g;

        prev[crr_node.up_idx as usize] = (crr_idx, Dir::Down);
        heap.push(Entry { idx: crr_node.up_idx, f: new_g + nodes[crr_node.up_idx as usize].end_dst })
      }
    }

    if crr_node.down_idx != 0 {
      let new_g = g_crr + nodes[crr_node.down_idx as usize].up_dst;

      if g[crr_node.down_idx as usize] > new_g {
        g[crr_node.down_idx as usize] = new_g;

        prev[crr_node.down_idx as usize] = (crr_idx, Dir::Up);
        heap.push(Entry { idx: crr_node.down_idx, f: new_g + nodes[crr_node.down_idx as usize].end_dst })
      }
    }

    if crr_node.left_idx != 0 {
      let new_g = g_crr + crr_node.left_dst;

      if g[crr_node.left_idx as usize] > new_g {
        g[crr_node.left_idx as usize] = new_g;

        prev[crr_node.left_idx as usize] = (crr_idx, Dir::Right);
        heap.push(Entry { idx: crr_node.left_idx, f: new_g + nodes[crr_node.left_idx as usize].end_dst })
      }
    }

    if crr_node.right_idx != 0 {
      let new_g = g_crr + nodes[crr_node.right_idx as usize].left_dst;

      if g[crr_node.right_idx as usize] > new_g {
        g[crr_node.right_idx as usize] = new_g;

        prev[crr_node.right_idx as usize] = (crr_idx, Dir::Left);
        heap.push(Entry { idx: crr_node.right_idx, f: new_g + nodes[crr_node.right_idx as usize].end_dst })
      }
    }
  }

  panic!("No path found.")
}

pub fn make_path(width: u32, nodes: &[Node], prev: &Vec<(u32, Dir)>) -> (Vec<(u32, i32)>, u64) {
  let i_width = width as i32;
  let neg_width = -i_width;

  let mut path = vec![];
  let mut path_length = 0u64;

  let mut crr_idx = nodes.len() as u32 - 1;

  while prev[crr_idx as usize].0 != 0 {
    let (prev_idx, dir) = prev[crr_idx as usize];

    match dir {
      Dir::Up => {
        let dst = nodes[crr_idx as usize].up_dst;
        path.push((dst, neg_width));
        path_length += dst as u64
      },
      Dir::Down => {
        let dst = nodes[prev_idx as usize].up_dst;
        path.push((dst, i_width));
        path_length += dst as u64
      },
      Dir::Left => {
        let dst = nodes[crr_idx as usize].left_dst;
        path.push((dst, -1));
        path_length += dst as u64
      },
      Dir::Right => {
        let dst = nodes[prev_idx as usize].left_dst;
        path.push((dst, 1));
        path_length += dst as u64
      }
    }

    crr_idx = prev_idx
  }

  (path, path_length)
}

pub fn draw_path(pixels: &mut Vec<u8>, path: &Vec<(u32, i32)>, path_length: u64, end: u32) {
  let mut color = 64.0;
  let color_step = 128.0 / path_length as f64;

  let mut crr_idx = end;

  for &(times, step) in path {
    for _ in 0 .. times {
      pixels[crr_idx as usize] = color as u8;
      color += color_step;

      crr_idx = (crr_idx as i64 + step as i64) as u32;
    }
  }

  pixels[crr_idx as usize] = color as u8
}
