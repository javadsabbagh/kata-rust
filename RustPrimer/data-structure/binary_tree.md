# Binary tree

## Introduction to Binary Trees
In computer science, a binary tree is a tree structure with at most two subtrees per node. Usually subtrees are called "left subtree" and "right subtree". Binary trees are often used to implement binary search trees and binary heaps.

>The keys of the child nodes and the parent nodes of the binary search tree generally satisfy a certain order relationship. It is customary that the keys of the left node are less than the keys of the parent node, and the keys of the right node are greater than the keys of the parent node.

>Binary heap is a special heap. Binary heap is a complete binary tree (binary tree) or an approximate complete binary tree (binary tree). There are two types of binary heaps: max heap and min heap. Max heap: The key of the parent node is always greater than or equal to the key of any child node; Min heap: The key of the parent node is always less than or equal to the key of any child node.

>Each node of the binary tree has at most two subtrees (there is no node with a degree greater than 2). The subtrees of the binary tree are divided into left and right, and the order cannot be reversed. The i-th layer of the binary tree has at most 2^{i-1} nodes; the binary tree with a depth of k has at most 2^k-1 nodes; for any binary tree T, if the number of terminal nodes is n_0, the degree The number of nodes that is 2 is n_2, then n_0=n_2+1.

>A full binary tree with a depth of k and 2^k-1 nodes is called a full binary tree; a binary tree with a depth of k and n nodes, if and only if each of its nodes is in the full binary tree with a depth of k , when the nodes with sequence numbers from 1 to n correspond to each other, it is called a complete binary tree.

## The difference between a binary tree and a tree
A binary tree is *not* a special case of a tree, and while it has many similarities to a tree, there are two main differences between a tree and a binary tree:

1. There is no limit to the maximum degree of a node in a tree, but the maximum degree of a node in a binary tree is 2.
2. The nodes of the tree are not divided into left and right, but the nodes of the binary tree are divided into left and right.

## Define the structure of the binary tree
Each node of the binary tree is composed of key key, value value and left/right subtree left/right. Here we declare the node as a generic structure.

```rust
type TreeNode<K,V> = Option<Box<Node<K,V>>>;
#[derive(Debug)]
struct Node<K,V: std::fmt::Display> {
   left: TreeNode<K,V>,
   right: TreeNode<K,V>,
   key: K,
   value: V,
}
```

## Realize the initialization of the binary tree and the insertion of the binary search tree
Since binary search trees require keys to be sortable, we require K to implement PartialOrd

```rust
trait BinaryTree<K,V> {
	fn pre_order(&self);
	fn in_order(&self);
	fn pos_order(&self);
}
trait BinarySearchTree<K:PartialOrd,V>:BinaryTree<K,V> {
	fn insert(&mut self, key:K,value: V);
}
impl<K,V:std::fmt::Display> Node<K,V> {
    fn new(key: K,value: V) -> Self {
        Node{
            left: None,
            right: None,
            value: value,
			key: key,
        }
    }
}
impl<K:PartialOrd,V:std::fmt::Display> BinarySearchTree<K,V> for Node<K,V>{
    fn insert(&mut self, key:K,value:V) {
        if self.key < key {
            if let Some(ref mut right) = self.right {
                right.insert(key,value);
            } else {
                self.right = Some(Box::new(Node::new(key,value)));
            }
        } else {
            if let Some(ref mut left) = self.left {
                left.insert(key,value);
            } else {
                self.left = Some(Box::new(Node::new(key,value)));
            }
        }
    }
}
```

## Binary tree traversal

- Preorder traversal: first visit the root, then traverse the left (right) subtree in preorder, and finally traverse the right (left) subtree in preorder.
- Inorder traversal: first inorder traverse the left (right) subtree, then visit the root, and finally inorder traverse the right (left) subtree.
- Post-order traversal: first post-order traverse the left (right) subtree, then post-order traverse the right (left) subtree, and finally visit the root.

The following is the code implementation:

```rust
impl<K,V:std::fmt::Display> BinaryTree<K,V> for Node<K,V> {
    fn pre_order(&self) {
        println!("{}", self.value);

        if let Some(ref left) = self.left {
            left.pre_order();
        }
        if let Some(ref right) = self.right {
            right.pre_order();
        }
    }

    fn in_order(&self) {
        if let Some(ref left) = self.left {
            left.in_order();
        }
        println!("{}", self.value);
        if let Some(ref right) = self.right {
            right.in_order();
        }
    }
    fn pos_order(&self) {
        if let Some(ref left) = self.left {
            left.pos_order();
        }
        if let Some(ref right) = self.right {
            right.pos_order();
        }
        println!("{}", self.value);
    }
}
```

## test code

```rust
type BST<K,V> = Node<K,V>;

fn test_insert() {
    let mut root = BST::<i32,i32>::new(3,4);
    root.insert(2,3);
    root.insert(4,6);
    root.insert(5,5);
    root.insert(6,6);
    root.insert(1,8);
    if let Some(ref left) = root.left {
        assert_eq!(left.value, 3);
    }

    if let Some(ref right) = root.right {
        assert_eq!(right.value, 6);
        if let Some(ref right) = right.right {
            assert_eq!(right.value, 5);
        }
    }
    println!("Pre Order traversal");
    root.pre_order();
    println!("In Order traversal");
    root.in_order();
    println!("Pos Order traversal");
    root.pos_order();
}

fn main() {
    test_insert();
}
```

## practise
Based on the above code, it is modified into the form of a binary heap.
