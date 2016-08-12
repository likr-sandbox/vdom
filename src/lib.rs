use std::collections::HashMap;
use std::fmt::Display;
use std::iter;
use std::rc::Rc;

pub struct Node<T: Eq> {
    key: Option<String>,
    value: T,
    children: Vec<Rc<Node<T>>>,
}

pub enum Patch<T: Eq> {
    Update(Rc<Node<T>>),
    Insert(Vec<Rc<Node<T>>>),
    Remove,
}

impl<T: Eq> Node<T> {
    pub fn new(value: T, key: Option<String>, children: Vec<Rc<Node<T>>>) -> Rc<Node<T>> {
        Rc::new(Node {
            key: key,
            value: value,
            children: children,
        })
    }
}

pub fn diff<T: Eq>(
    a: &Rc<Node<T>>,
    b: &Rc<Node<T>>
) -> HashMap<u32, Patch<T>> {
    let mut result =HashMap::new();
    let mut index = 0;
    walk(a, b, &mut result, &mut index);
    result
}

fn walk<T: Eq>(
    a: &Rc<Node<T>>,
    b: &Rc<Node<T>>,
    mut result: &mut HashMap<u32, Patch<T>>,
    mut index: &mut u32
) {
    if a.value == b.value {
        diff_children(&a.children, &b.children, &mut result, &mut index);
    } else {
        let patch = Patch::Update((*b).clone());
        result.insert(*index, patch);
    }
}

fn diff_children<T: Eq>(
    a_children: &Vec<Rc<Node<T>>>,
    b_children: &Vec<Rc<Node<T>>>,
    mut result: &mut HashMap<u32, Patch<T>>,
    mut index: &mut u32
) {
    let parent_index = *index;
    let a_len = a_children.len();
    let b_len = b_children.len();
    for i in 0..a_len {
        *index = *index + 1;
        if i >= b_len {
            let patch = Patch::Remove;
            result.insert(*index, patch);
        } else {
            walk(&a_children[i], &b_children[i], &mut result, &mut index);
        }
    }
    if b_len > a_len {
        let mut inserts = vec![];
        for i in a_len..b_len {
            inserts.push(b_children[i].clone());
        }
        let patch = Patch::Insert(inserts);
        result.insert(parent_index, patch);
    }
}

pub fn patch<T: Display + Eq>(
    root: &Rc<Node<T>>,
    patches: &HashMap<u32, Patch<T>>
) {
    let mut index = 0;
    apply(root, patches, &mut index, 0);
}

fn apply<T: Display + Eq>(
    node: &Rc<Node<T>>,
    patches: &HashMap<u32, Patch<T>>,
    mut index: &mut u32,
    depth: usize
) {
    match patches.get(index) {
        Some(patch) => {
            match *patch {
                Patch::Update(ref node) => {
                    print_rec(node, depth);
                },
                Patch::Insert(ref nodes) => {
                    print(node, depth, false);
                    for child in &node.children {
                        *index = *index + 1;
                        apply(&child, patches, &mut index, depth + 1);
                    }
                    for node in nodes {
                        print_rec(node, depth + 1);
                    }
                },
                Patch::Remove => {
                },
            }
        },
        None => {
            print(node, depth, false);
            for child in &node.children {
                *index = *index + 1;
                apply(&child, patches, &mut index, depth + 1);
            }
        },
    }
}

fn print<T: Display + Eq>(node: &Rc<Node<T>>, depth: usize, change: bool) {
    if change {
        println!("{}{}(*)", indent(depth), node.value);
    } else {
        println!("{}{}", indent(depth), node.value);
    }
}

fn print_rec<T: Display + Eq>(node: &Rc<Node<T>>, depth: usize) {
    print(node, depth, true);
    for child in &node.children {
        print_rec(&child, depth + 1);
    }
}

fn indent(n: usize) -> String {
    iter::repeat("  ").take(n).collect()
}

fn key(s: &str) -> Option<String> {
    Some(String::from(s))
}

#[cfg(test)]
mod tests {
    use super::{diff, patch, key, Node, Patch};

    #[test]
    fn it_works() {
        let node1 = Node::new("root", None, vec![
            Node::new("child1", key("child1"), vec![
                Node::new("child1-1", None, vec![]),
            ]),
            Node::new("child2", key("child2"), vec![
                Node::new("child2-1", None, vec![]),
                Node::new("child2-2", None, vec![]),
            ]),
            Node::new("child3", key("child3"), vec![]),
        ]);
        let node2 = Node::new("root", None, vec![
            Node::new("child1", key("child1"), vec![
            ]),
            Node::new("child2", key("child2"), vec![
                Node::new("child2-2", None, vec![]),
                Node::new("child2-1", None, vec![
                    Node::new("child2-1-1", None, vec![]),
                ]),
            ]),
            Node::new("child4", key("child4"), vec![]),
            Node::new("child5", key("child5"), vec![]),
            Node::new("child6", key("child6"), vec![
                Node::new("child6-1", None, vec![]),
            ]),
        ]);
        let result = diff(&node1, &node2);
        for (index, patch) in &result {
            match *patch {
                Patch::Update(ref node) => {
                    println!("{} update {}", index, node.value);
                },
                Patch::Insert(ref nodes) => {
                    for node in nodes {
                        println!("{} insert {}", index, node.value);
                    }
                },
                Patch::Remove => {
                    println!("{} remove", index);
                },
            }
        }
        patch(&node1, &result);
    }
}
