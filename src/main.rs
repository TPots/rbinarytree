use std::cmp::Ordering;
use rand::Rng;

struct BinaryTree {
    value: Option<i64>,
    left: Option<Box<BinaryTree>>,
    right: Option<Box<BinaryTree>>,
}


impl BinaryTree {
    fn new(value: Option<i64>) -> Self {
        Self{ value, left: None, right: None }
    }

    fn push(&mut self, value: i64) {
        match self.value {
            Some(n) => {
                let target: &mut Option<Box<BinaryTree>> = match n.cmp(&value) {
                    Ordering::Less => &mut self.left,
                    Ordering::Greater => &mut self.right,
                    Ordering::Equal => return,
                };
                match target {
                    Some(binary_tree) => binary_tree.push(value),
                    None => *target = Some(Box::new(BinaryTree::new(Some(value))))
                }
            },
            None => {
                self.value = Some(value)
            }
        }
    }

    fn flatten(&self) -> Vec<i64> {
        let mut structure: Vec<i64> = vec!();
        self._flatten(&mut structure);
        structure.reverse();
        structure
    }

    fn _flatten(&self, structure: &mut Vec<i64>){
        match &self.left {
            Some(node) => node._flatten(structure),
            None => (),
        }
        structure.push(self.value.unwrap());
        match &self.right {
            Some(node) => node._flatten(structure),
            None => (),
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let numbers: Vec<i64> = (0..50).map(|_| rng.gen_range(0..1000)).collect();
    let mut bt = BinaryTree::new(None);
    
    println!("{:?}", numbers);
    for number in numbers {
        bt.push(number);
    }

    let res = bt.flatten();
    println!("{:?}", res);
}
