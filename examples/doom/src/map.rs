use super::Wall;
use alloc::boxed::Box;
use alloc::vec::Vec;

pub struct Map {
    walls: Vec<Wall>,
}

impl Map {
    pub fn new(walls: Vec<Wall>) -> Self {
        Map { walls }
    }
    pub fn partition(mut self) -> Option<BinaryTreeNode<Wall>> {
        self.walls.pop().map(|wall| {
            let mut front = Vec::new();
            let mut behind = Vec::new();
            for other in self.walls {
                match wall.intersect(&other) {
                    Some(intersection) => {
                        let new_walls = other.split(intersection);
                        if wall.behind(&new_walls[0].start) {
                            assert!(!wall.behind(&new_walls[1].end));
                            behind.push(new_walls[0]);
                            front.push(new_walls[1]);
                        } else {
                            assert!(!wall.behind(&new_walls[0].start));
                            assert!(wall.behind(&new_walls[1].end));
                            front.push(new_walls[0]);
                            behind.push(new_walls[1]);
                        }
                    }
                    None => {
                        if wall.behind(&other.start) {
                            //assert!(wall.behind(&other.end));
                            behind.push(other);
                        } else {
                            assert!(!wall.behind(&other.start));
                            //assert!(!wall.behind(&other.end));
                            front.push(other);
                        }
                    }
                }
            }
            let recurse = |v| Map { walls: v }.partition().map(|res| Box::new(res));
            let left = recurse(behind);
            let right = recurse(front);
            BinaryTreeNode {
                value: wall,
                left,
                right,
            }
        })
    }
}

pub struct BinaryTreeNode<T> {
    value: T,
    left: Option<Box<BinaryTreeNode<T>>>,
    right: Option<Box<BinaryTreeNode<T>>>,
}

impl<T> BinaryTreeNode<T> {
    pub fn traverse(&self) -> Vec<&T> {
        let mut v = Vec::new();
        self.left.as_ref().map(|left| {
            v.append(&mut left.traverse());
        });
        v.push(&self.value);
        self.right.as_ref().map(|right| {
            v.append(&mut right.traverse());
        });
        v
    }
}
