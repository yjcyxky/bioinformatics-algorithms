// Graph Data Structure

#[derive(Debug,Clone)]
struct Node {
  idx: usize,
  name: String,
}

#[derive(Debug,Clone)]
struct Edge {
  edge: bool,
}

#[derive(Debug,Clone)]
struct Graph {
  nums: usize,
  graph: Vec<Vec<Edge>>,
}

impl Node {
  fn new(idx: usize, name: String) -> Node {
    Node {
      idx: idx,
      name: name,
    }
  }
}

impl Edge {
  fn new() -> Edge {
    Edge { edge: false }
  }

  fn is_connection() -> Edge {
    Edge { edge: true }
  }
}

impl Graph {
  fn new(nums: usize) -> Graph {
    Graph {
      nums: nums,
      graph: vec![vec![Edge::new(); nums]; nums],
    }
  }

  fn connect(&mut self, node1: &Node, node2: &Node) {
    if node1.idx < self.nums && node2.idx < self.nums {
      self.graph[node1.idx][node2.idx] = Edge::is_connection()
    } else {
      panic!("The index is bigger than nums");
    }
  }

  fn has_edge(&self, node1: &Node, node2: &Node) -> bool {
    if node1.idx < self.nums && node2.idx < self.nums {
      return self.graph[node1.idx][node2.idx].edge;
    } else {
      panic!("The index is bigger than nums");
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn create_graph() {
    let mut g = Graph::new(2);
    let v1 = Node::new(0, "v1".to_string());
    let v2 = Node::new(1, "v2".to_string());
    g.connect(&v1, &v2);
    assert!(true, "{}", g.has_edge(&v1, &v2));
  }
}
