use std::{cmp::Reverse, collections::BinaryHeap, collections::HashMap};

// https://github.com/edvujic/LZ77-DEFLATE-Compression
enum HuffmanNode {
    Data {
        character: char,
        frequency: u32
    },
    InternalNode {
        left: Box<HuffmanNode>,
        right: Box<HuffmanNode>,
        combined_frequency: u32,
    }
}

impl HuffmanNode {
    fn frequency(&self) -> u32{
        match self {
            HuffmanNode::Data { frequency, .. } => *frequency,
            HuffmanNode::InternalNode { combined_frequency, .. } => *combined_frequency
        }
    }
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverses order
        other.frequency().cmp(&self.frequency())
    }
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HuffmanNode {
    fn eq(&self, other: &Self) -> bool {
        self.frequency() == other.frequency()
    }
}

impl Eq for HuffmanNode {}


pub fn generate_canonical_huffman_tree(data: &str) {
    let mut map: HashMap<char, u32> = HashMap::new();
    for character in data.chars() {
        // Increase frequency by 1
        map.entry(character).and_modify(|item| *item += 1).or_insert(1);
    }

    // Turn map into (Min) Heap
    let mut heap: BinaryHeap<HuffmanNode> = map.iter().map(|(key, value)| HuffmanNode::Data { character: (*key), frequency: (*value) }).collect();


    while heap.len() > 1 {
        // Grab 2 smallest values and put them together
        let item1: Box<HuffmanNode> = Box::new(heap.pop().unwrap());
        let item2: Box<HuffmanNode> = Box::new(heap.pop().unwrap());
        let combined_frequency = item1.frequency() + item2.frequency();
        let combined = HuffmanNode::InternalNode { left: (item1), right: (item2), combined_frequency: (combined_frequency) };
        
        heap.push(combined);
    }

    let codes = assign_huffman_codes(heap.peek().unwrap());
    println!("{:?}", codes);
}

fn assign_huffman_codes(root_node: &HuffmanNode) -> Vec<(char, String)> {
    let mut huffman_codes: Vec<(char, String)> = Vec::new();

    // Saves lowest number that's not already been taken in a previous level
    let mut lowest_available_number: usize = 0;


    // Holds all nodes on the current level of the tree that's being iterated through
    let mut current_level_nodes: Vec<&HuffmanNode> = vec![root_node];
    let mut current_level: u32 = 0;
    loop {
        let mut next_level_nodes: Vec<&HuffmanNode> = Vec::new();
        // Saves characters from all "end-nodes" on current level
        let mut current_level_characters: Vec<char> = Vec::new();

        for node in &current_level_nodes {
            match node {
                HuffmanNode::InternalNode { left, right, .. } => {
                    next_level_nodes.push(left);
                    next_level_nodes.push(right);
                },
                HuffmanNode::Data { character, .. } => {
                    current_level_characters.push(*character);
                }
            };
        }

        if current_level_characters.len() > 0 {
            // Sort chars lexigographically
            current_level_characters.sort();

            'outer: for i in lowest_available_number..32 {
                let mut is_available = true;
                for (_, code) in &huffman_codes {
                    if get_binary_string(i.try_into().unwrap(), current_level as usize)
                        .starts_with(code) {
                            is_available = false;
                            break;
                    }
                }
                if is_available {
                    lowest_available_number = i;
                    break 'outer;
                }
            }

            for (i, char) in current_level_characters.iter().enumerate() {
                let binary_string = get_binary_string((i + lowest_available_number).try_into().unwrap(), current_level as usize);
                huffman_codes.push((*char, binary_string));
            }
        }

        if next_level_nodes.len() == 0 {
            break;
        }
        current_level_nodes = next_level_nodes;
        current_level += 1;
    }
    huffman_codes
}

fn get_binary_string(number: u32, amount_of_digits: usize) -> String {
    let mut binary: String = String::new();
    for i in (0..32).rev() {
        binary += if ((number >> i) & 1) != 0 {"1"} else {"0"};
    }
    // Trim binary number
    binary[(binary.len() - amount_of_digits)..].to_string()
}