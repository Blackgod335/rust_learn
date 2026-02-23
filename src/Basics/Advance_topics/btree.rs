/*
1. What is a B-tree?

A B-tree is a self-balancing tree designed to maintain sorted data and allow
 operations like search, insert, and delete in O(log n) time.
*/

//BTreeMap is like a HashMap, but keys are always sorted.

use std::collections::BTreeMap;

fn main() {
    let mut map = BTreeMap::new();

    map.insert(3, "three");
    map.insert(1, "one");
    map.insert(2, "two");

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
}

/*
3. BTreeSet

BTreeSet is like a HashSet, but values are stored sorted and unique.
 */

 use std::collections::BTreeSet;

fn btreeset_example() {
    let mut set = BTreeSet::new();
    
    set.insert(5);
    set.insert(1);
    set.insert(3);
    
    for val in &set {
        println!("{}", val);
    }
}


/*
Do you need key-value pairs? → BTreeMap

Do you just need a collection of unique items? → BTreeSet

Both allow sorted iteration and range queries.
*/