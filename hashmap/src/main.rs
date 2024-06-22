use std::collections::HashMap;
use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Debug)]
struct MyMap<K, V> {
    map: HashMap<K, V>,
}

impl<K, V> fmt::Display for MyMap<K, V>
where
    K: fmt::Display,
    V: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{")?;
        for (key, value) in &self.map {
            write!(f, "{}: {}, ", key, value)?;
        }
        write!(f, "}}")
    }
}

impl<K, V> MyMap<K, V>
where
    K: std::hash::Hash + Eq,
{
    fn new() -> Self {
        return MyMap {
            map: HashMap::new(),
        };
    }

    fn insert(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }
}

impl<K, V> Index<K> for MyMap<K, V>
where
    K: std::hash::Hash + Eq,
{
    type Output = V;
    fn index(&self, key: K) -> &Self::Output {
        return self.map.get(&key).expect("key not found");
    }
}

impl<K, V> IndexMut<K> for MyMap<K, V>
where
    K: std::hash::Hash + Eq,
{
    fn index_mut(&mut self, k: K) -> &mut Self::Output {
        return self.map.get_mut(&k).expect("key not found");
    }
}

fn main() {
    let mut mymap = MyMap::new();
    mymap.insert("a", "sfsds");
    mymap.insert("b", "bbffd");

    // Modify value for key "a"
    mymap["a"] = "we changed";

    // Print the map
    print!("the map is: {}", &mymap);
}
