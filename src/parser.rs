use timed_proc_macro::timed;

#[derive(Clone, Copy)]
pub enum Dir { Up, Down, Left, Right }

#[derive(Clone, Copy, Default)]
pub struct Node {
  pub left_dst: u32,
  pub right_dst: u32,

  pub up_dst: u32,
  pub up_idx: u32,
  pub down_dst: u32,
  pub down_idx: u32,

  pub g: u32,
  pub prev: Option<Dir>,

  pub end_dst: u32
}

fn first_empty_pixel(pixels: &[u8], start_idx: usize, end_idx: usize) -> u32 {
  for i in start_idx .. end_idx {
    if pixels[i] != 0 {
      return i as u32
    }
  }

  panic!("Invalid maze.")
}

#[timed]
pub fn parse(width: u32, height: u32, pixels: &[u8]) -> (Vec<Node>, u32) {
  let max_y = width - 1;

  let width = width as usize;
  let height = height as usize;
  let img_size = width * height;

  let start = first_empty_pixel(pixels, 1, width - 1);
  let end = first_empty_pixel(pixels, img_size - width + 1, img_size - 1);

  let end_x = height as u32 - 1;
  let end_y = end % width as u32;
  let y_diff = (start as i32 - end_y as i32).abs() as u32;

  let mut nodes = Vec::with_capacity(width);
  nodes.push(Node::default());

  let mut top_nodes = vec![(0, 0); width];
  let mut left_node_idx = 0u32;

  nodes.push(Node{end_dst: end_x + y_diff, g: 0, ..Node::default()});
  top_nodes[start as usize] = (1, 0);

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

    let mut new_node = Node{g: u32::MAX, ..Node::default()};
    let new_node_idx = nodes.len() as u32;

    if up_white {
      let (top_node_idx, top_x) = top_nodes[y as usize];
      let dst = x - top_x;

      new_node.up_dst = dst;
      new_node.up_idx = top_node_idx;

      nodes[top_node_idx as usize].down_dst = dst;
      nodes[top_node_idx as usize].down_idx = new_node_idx
    }

    if down_white {
      top_nodes[y as usize] = (new_node_idx, x)
    }

    if left_white {
      let dst = crr as u32 - left_node_idx;

      new_node.left_dst = dst;
      nodes[new_node_idx as usize - 1].right_dst = dst
    }

    if right_white {
      left_node_idx = crr as u32
    }

    let y_diff = (end_y as i32 - y as i32).abs() as u32;
    new_node.end_dst = end_x - x + y_diff;

    nodes.push(new_node)
  }

  let mut end_node = Node{end_dst: 0, g: u32::MAX, ..Node::default()};
  let end_node_idx = nodes.len() as u32;

  if pixels[end as usize - width] != 0 {
    let (top_node_idx, top_x) = top_nodes[end_y as usize];
    let dst = end_x - top_x;

    end_node.up_dst = dst;
    end_node.up_idx = top_node_idx;
    nodes[top_node_idx as usize].down_dst = dst;
    nodes[top_node_idx as usize].down_idx = end_node_idx
  }

  nodes.push(end_node);
  (nodes, end)
}
