use core::cmp::Ordering;
use core::cmp::max;
use core::cmp::min;
use std::collections::BinaryHeap;
use std::collections::HashMap;

struct Tile {
  f: u32,
  pos: u32
}

impl PartialEq for Tile {
  fn eq(&self, rhs: &Self) -> bool {
    self.f == rhs.f && self.pos == rhs.pos
  }
}

impl Eq for Tile { }

impl PartialOrd for Tile {
  fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
    Some(self.cmp(rhs))
  }
}

impl Ord for Tile {
  fn cmp(&self, rhs: &Self) -> Ordering {
    rhs.f.cmp(&self.f).then_with(|| self.pos.cmp(&rhs.pos))
  }
}

fn manhattan(pos: u32, target: u32, width: u32) -> u32 {
  let x = pos / width;
  let y = pos % width;

  let tx = target / width;
  let ty = target % width;

  max(x, tx) - min(x, tx) + max(y, ty) - min(y, ty)
}

pub fn solve(width: u32, height: u32, pixels: &[u8]) -> Option<(HashMap<u32, u32>, u32)> {
  let img_size = width * height;
  let mut start = 0;

  for i in 1 .. width - 1 {
    if pixels[i as usize] == 255 {
      start = i;
      break
    }
  }

  if start == 0 {
    panic!("No empty pixel on first line.");
  }

  let mut end = 0;

  for i in width * (height - 1) + 1 .. width * height - 1 {
    if pixels[i as usize] == 255 {
      end = i;
      break
    }
  }

  if end == 0 {
    panic!("No empty pixel on last line.");
  }

  let mut prev = HashMap::<u32, u32>::new();
  prev.insert(start, 0);

  let mut g = HashMap::<u32, u32>::new();
  g.insert(start, 0);

  let mut f = HashMap::<u32, u32>::new();
  f.insert(start, manhattan(start, end, width));

  let mut heap = BinaryHeap::<Tile>::new();
  heap.push(Tile { pos: start, f: *f.get(&start).unwrap_or(&u32::MAX) });

  while !heap.is_empty() {
    let node = heap.pop().unwrap();
    let pos = node.pos;

    if pos == end {
      return Some((prev, pos));
    }

    let up = pos - width;
    let down = pos + width;
    let left = pos - 1;
    let right = pos + 1;

    for &node in &[up, down, left, right] {
      if node < img_size && pixels[node as usize] != 0 {
        let g_ = g[&pos] + 1;

        if !g.contains_key(&node) || g_ < g[&node] {
          g.insert(node, g_);
          f.insert(node, g_ + manhattan(node, end, width));

          prev.insert(node, pos);
          heap.push(Tile { pos: node, f: f[&node] })
        }
      }
    }
  }

  None
}
