use core::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

use crate::parser::*;

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

  while let Some(Node{ idx: crr, .. }) = heap.pop() {
    if crr == end {
      return prev
    }

    let g_crr = g[&crr];

    if let Some(&up) = up_idx.get(&crr) {
      let new_g = g_crr + up_dst[&crr];

      match g.get(&up) {
        Some(&g_) if g_ <= new_g => {},
        _ => {
          g.insert(up, new_g);

          prev.insert(up, (crr, Dir::Down));
          heap.push(Node { idx: up, f: new_g + end_dst[&up] })
        }
      }
    }

    if let Some(&down) = down_idx.get(&crr) {
      let new_g = g_crr + up_dst[&down];

      match g.get(&down) {
        Some(&g_) if g_ <= new_g => {},
        _ => {
          g.insert(down, new_g);

          prev.insert(down, (crr, Dir::Up));
          heap.push(Node { idx: down, f: new_g + end_dst[&down] })
        }
      }
    }

    if let Some(&left) = left_idx.get(&crr) {
      let new_g = g_crr + left_dst[&crr];

      match g.get(&left) {
        Some(&g_) if g_ <= new_g => {},
        _ => {
          g.insert(left, new_g);

          prev.insert(left, (crr, Dir::Right));
          heap.push(Node { idx: left, f: new_g + end_dst[&left] })
        }
      }
    }

    if let Some(&right) = right_idx.get(&crr) {
      let new_g = g_crr + left_dst[&right];

      match g.get(&right) {
        Some(&g_) if g_ <= new_g => {},
        _ => {
          g.insert(right, new_g);

          prev.insert(right, (crr, Dir::Left));
          heap.push(Node { idx: right, f: new_g + end_dst[&right] })
        }
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
