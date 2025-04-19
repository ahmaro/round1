pub mod hash {
    const NULL: usize = 0;
    use sha3::{Digest, Sha3_256};

    // The status flag of every slot.
    enum Status {
        Free,
        Used
    }

    // Struct of the properties of every single slot in the hash table.
    pub struct Slot {
        key: String,
        val: usize,
        status: Status,
        count: usize
    }

    // Struct of the hash table.
    pub struct HashTable {
        table: Vec<Slot>,
        table_size: usize,
        counter: usize
    }

    impl HashTable {
        // Method to create a new hash table of a certain size.
        pub fn new(new_size: usize) -> Self {
            let mut slots = Vec::with_capacity(new_size.try_into().unwrap_or(0));
            for _ in 0.. new_size {
                slots.push(Slot {
                    key: String::new(),
                    val: 0,
                    status: Status::Free,
                    count: 0,
                });
            }
            HashTable{table: slots, table_size: new_size, counter: NULL}
        }

        // Private method to find the potential index of a key.
        fn find_index(&self, key: String) -> usize {
            let mut hasher = sha3::Sha3_256::new();
            hasher.update(key);
            let sha3_256_hash = hasher.finalize();
            let num = usize::from_be_bytes(sha3_256_hash[..8].try_into().unwrap());
            num % self.table_size
        }

        // Function to add key value pair
        pub fn insert(&mut self, bucket: (String, usize)) {
            let mut hash_index = self.find_index(bucket.0.clone());
            for _ in 0..self.table_size {
                match self.table[hash_index].status {
                    Status::Free => {
                        self.counter += 1;
                        self.table[hash_index].key = bucket.0.clone();
                        self.table[hash_index].val = bucket.1;
                        self.table[hash_index].status = Status::Used;
                        self.table[hash_index].count = self.counter;
                        // println!("Key {} and its count: {}", self.table[hash_index].key, self.table[hash_index].count);
                        return;
                    },
                    Status::Used => {
                        self.table[hash_index].count += 1;
                    },
                }
                hash_index = (hash_index + 1) % self.table_size;
            }
        }

        // Function to remove key value pair
        pub fn remove(&mut self, remove_key: String) -> Option<usize> {
            for index in 0..self.table_size {
                if self.table[index].key == remove_key {
                    self.table[index].key = String::new();
                    self.table[index].val = NULL;
                    self.table[index].status = Status::Free;
                    self.table[index].count = NULL;
                    return Some(NULL);
                }
            }
            None
        }

        // Function to remove key value pair
        pub fn get(&mut self, get_key: String) -> Option<usize> {
            for index in 0..self.table_size {
                if self.table[index].key == get_key {
                    return Some(self.table[index].val);
                }
            }
            None
        }

        // Function to get the latest key value pair
        pub fn get_last(&mut self) -> (String, usize) {
            let mut last_key: (String, usize) = (String::new(), 0);
            let mut biggest_count: usize = 0;
            for index in 0..self.table_size {
                let current_count = self.table[index].count;
                if current_count > biggest_count {
                    biggest_count = current_count;
                    last_key = (self.table[index].key.clone(), self.table[index].val);
                }
            }
            last_key
        }

        // Function to get the latest key value pair
        pub fn get_first(&mut self) -> (String, usize) {
            let mut first_key: (String, usize) = (String::new(), 0);
            let mut least_count: usize = self.table_size;
            for index in 0..self.table_size {
                let current_count = self.table[index].count;
                if current_count < least_count {
                    least_count = current_count;
                    first_key = (self.table[index].key.clone(), self.table[index].val);
                }
            }
            first_key
        }
    } 
}
