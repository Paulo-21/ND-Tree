use crate::ndtree::NDTree;

mod ndtree;

fn main() {
    let mut ndtree = NDTree::new();
    let x = Vec::from([0, 9, 3]);
    ndtree.update(x);
}
