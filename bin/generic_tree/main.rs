use std::fmt::Display;
#[derive(Debug, Clone)]
struct Tree<T> {
    value: T, 
    children: Vec<Tree<T>>,
}

impl<T: Display + Eq + Clone> Tree<T> {
    fn new(value: T) -> Self {
        Self { value, children: Vec::new() } 
    }
    fn add_child(&mut self, child: Tree<T>) {
        self.children.push(child)
    }
    fn remove_child(&mut self, value: T) {
        let to_remove: Vec<usize> = self.children
            .iter()
            .enumerate()
            .filter_map(|(i, child)| if child.value == value { Some(i) } else { None })
            .collect();
    
        for &index in to_remove.iter().rev() {
            self.children.remove(index);
        }
    
        for child in self.children.iter_mut() {
            child.remove_child(value.clone())
        }
    }
    fn traverse(&self) {
        println!("Node value {}", self.value);
        for child in &self.children {
            child.traverse();
        }
    }
}

fn main() {
    let mut root = Tree::new(1);
    let mut child1 = Tree::new(2);
    let mut child2 = Tree::new(3);
    let child4 = Tree::new(15);
    child2.add_child(child4);
    child1.add_child(child2);
    let child3 = Tree::new(4);

    root.add_child(child1);
    println!("Tree after adding children:");
    root.traverse();

    root.remove_child(15);
    root.add_child(child3);

    println!("\nTree after removal and adding another child:");
    root.traverse();
}