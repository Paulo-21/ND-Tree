pub struct NDTree {
    pub root: Option<Node>,
    pub nb_point: u32,
}
impl NDTree {
    pub fn new() -> Self {
        Self {
            root: None,
            nb_point: 0,
        }
    }
    pub fn update(&mut self, x: Vec<u32>) {
        if self.root.is_none() {
            self.root = Some(Node::Leaf {
                sol_list: Vec::from([x]),
            });
        } else {
            self.updateNode(x);
        }
    }
    pub fn updateNode(&self) {}
    pub fn insert(&self) {}
    pub fn split(&self) {}
}
#[derive(Debug)]
enum Node {
    Leaf { sol_list: Vec<Vec<u32>> },
    Inter { son_list: Vec<Box<Node>> },
}
impl Node::Leaf {}
impl Default for Node {
    fn default() -> Self {
        Node::Leaf {
            sol_list: Vec::new(),
        }
    }
}
/*
pub struct Node {}

impl Node {
    pub fn new() -> Self {
        Self {}
    }
}
*/
