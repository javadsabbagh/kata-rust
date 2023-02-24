use std::collections::{BTreeSet, HashSet};
use std::hash::Hash;
use std::ptr::hash;

fn main() {

    // A set has two varieties: BTreeSet and HashSet
    let mut hash_set = HashSet::new();
    let another_hash_Set: HashSet<&str> = HashSet::new();
    let mut tree_set = BTreeSet::new();

    hash_set.insert("ABC");
    tree_set.insert("EFG");

    hash_set.intersection(&another_hash_Set);
}