#![allow(dead_code)]
mod hash;
use std::fmt::Debug;
use hash::hash::Hashable;

pub struct HashMap<Key: Hashable, Value> {
    pub kv_pairs: Vec<HashCell<Key, Value>>,
    taken_count: usize
}

impl<Key, Value> HashMap<Key, Value>
where
    Key: Default + Hashable + Clone + Debug + PartialEq + Eq,
    Value: Default + Clone + Debug
{
    pub fn new() -> Self {
        const INITIAL_CAPACITY: usize = 69; // Nice
        Self {
            kv_pairs: vec![
                HashCell::default();
                INITIAL_CAPACITY
            ],
            taken_count: 0
        }
    }

    pub fn debug_dump(&self) { // Prints all the keys value pairs in the table
        self.kv_pairs.iter()
            .filter(|x| x.status == CellStatus::Taken)
            .for_each(|x| println!("{:?}->{:?}", x.key, x.value));
    }

    // Double the size of the array if full, To make insertion O(1) amortized
    fn extend(&mut self) {
        let mut new_self = Self {
            kv_pairs: vec![HashCell::default(); self.kv_pairs.len() * 2],
            taken_count: 0
        };

        self.kv_pairs
            .iter()
            .filter(|x| x.status == CellStatus::Taken)
            .for_each(|x| {new_self.insert(x.key.clone(), x.value.clone());});

        *self = new_self;
    }

    // Finds the index of the current key, or the first empty index for new key
    fn find_index(&mut self, key: &Key) -> usize {
        if self.taken_count == self.kv_pairs.len() { // Too smol HashTable
            self.extend();
        }

        let mut index = key.hash() & self.kv_pairs.len(); // natural positino

        while (self.kv_pairs[index].status == CellStatus::Taken) &
              (self.kv_pairs[index].key != *key)
        {
            index = (index + 1) % self.kv_pairs.len();
        }
        index
    }

    pub fn insert(&mut self, key: Key, value: Value) -> Option<Value>{
        let index = self.find_index(&key);

        match self.kv_pairs[index].status {
            // Add an element if the index corresponding to the key is empty
            CellStatus::Empty => {
                self.taken_count += 1;
                self.kv_pairs[index] = HashCell {
                    key,
                    value,
                    status: CellStatus::Taken
                };
                None
            },
            // Return the old value and replace with new if not empty
            CellStatus::Taken => {
                let old_val = self.kv_pairs[index].value.clone();
                self.kv_pairs[index].value = value;
                Some(old_val)
            }
        }
    }

    pub fn get(&mut self, key: Key) -> Option<&Value> {
        self.get_mut(key).map(|val| &*val)
    }

    pub fn get_mut(&mut self, key: Key) -> Option<&mut Value> {
        let index = self.find_index(&key);
        match self.kv_pairs[index].status {
            CellStatus::Empty => None,
            CellStatus::Taken => Some(&mut self.kv_pairs[index].value)
        }
    }

    // Removes an element given an index, but shifts other elements so that
    // The natural position of any element is never empty, which would stop
    // The find_index process
    pub fn remove(&mut self, key: Key) -> Option<Value> {
        let mut index = self.find_index(&key);
        match self.kv_pairs[index].status {
            CellStatus::Empty => None, // empty slot
            CellStatus::Taken => {
                // Exiting Value which is returned
                let value = self.kv_pairs[index].value.clone();
                self.kv_pairs[index].status = CellStatus::Empty; // making empty
                let mut j = index.clone();
                loop {
                    j += 1;
                    if self.kv_pairs[j].status == CellStatus::Empty { break; }
                    // Natual index of key at j is k
                    let k = self.kv_pairs[j].key.hash() % self.kv_pairs.len();
                    // To find all elements not at their natural index
                    if index <= j {
                        if index < k && k <= j { continue; }
                    } else {
                        if index < k || k <= k { continue; }
                    }
                    // after finding the last elment that was carried forward
                    // put it in the natural index, to make find_index work
                    self.kv_pairs[index] = self.kv_pairs[j].clone();
                    self.kv_pairs[j].status = CellStatus::Empty;
                    index = j;
                }
                Some(value)
            }
        }
    }
}

#[derive(Default, Clone)]
pub struct HashCell<Key, Value>
where
{
    key: Key,
    value: Value,
    status: CellStatus
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum CellStatus {
    #[default]
    Empty,
    Taken
}

#[cfg(test)]
mod tests {
    use crate::HashMap;

    #[test]
    fn output_testing() {
        let mut phone_book = HashMap::new();
        phone_book.insert("Shubh".to_string(), "8850873712".to_string());
        phone_book.insert("Sbubh".to_string(), "8850873712".to_string());
        phone_book.remove("Sbubh".to_string());
        phone_book.insert("Hershey".to_string(), "8369254766".to_string());
        phone_book.insert("Yash".to_string(), "8458467872".to_string());
        phone_book.debug_dump();
        println!("-------------");
        println!("{:?}", phone_book.get("Shubh".to_string()));
        println!("{:?}", phone_book.get("Sbubh".to_string()));
        assert!(true); // Dummy assert
    }
}
