use std::collections::{BTreeSet, HashMap};

pub struct MultiSet {
    counter: HashMap<usize, usize>,
    set: BTreeSet<usize>,
    count: usize,
}

impl MultiSet {
    pub fn new() -> Self {
        MultiSet {
            counter: HashMap::new(),
            set: BTreeSet::new(),
            count: 0,
        }
    }

    pub fn from(items: &[usize]) -> Self {
        let count = items.len();
        let mut counter = HashMap::new();
        let mut set = BTreeSet::new();
        for &item in items {
            *counter.entry(item).or_insert(0usize) += 1;
            set.insert(item);
        }
        Self {
            counter,
            set,
            count,
        }
    }

    pub fn insert(&mut self, item: usize) {
        self.count += 1;
        self.set.insert(item);
        *self.counter.entry(item).or_insert(0) += 1;
    }

    pub fn remove(&mut self, item: usize) -> Option<usize> {
        if self.set.contains(&item) {
            *self.counter.entry(item).or_default() -= 1;
            if self.counter[&item] == 0 {
                self.counter.remove(&item);
                self.set.remove(&item);
            }
            self.count -= 1;
            Some(item)
        } else {
            None
        }
    }

    pub fn remove_set(&mut self, item: usize) -> Option<usize> {
        if self.set.contains(&item) {
            self.set.remove(&item);
            let cnt = self.counter.remove(&item).unwrap();
            self.count -= cnt;
            Some(item)
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn get_max(&self) -> Option<usize> {
        if let Some(&max) = self.set.iter().next_back() {
            Some(max)
        } else {
            None
        }
    }

    pub fn get_min(&self) -> Option<usize> {
        if let Some(&min) = self.set.iter().next() {
            Some(min)
        } else {
            None
        }
    }

    pub fn contains(&self, item: usize) -> bool {
        self.set.contains(&item)
    }

    pub fn print(&self) {
        print!("{{");
        let mut buf = "".to_string();
        for item in self.set.iter() {
            let cnt = self.counter.get(&item).unwrap_or(&0);
            for _ in 0..*cnt {
                buf += &format!(" {}", item);
            }
        }
        print!("{}", buf);
        println!(" }}")
    }
}
