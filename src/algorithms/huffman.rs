use std::{cmp::{Ordering}, collections::{BinaryHeap, HashMap}};

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
        match self.frequency().cmp(&other.frequency()) {
            Ordering::Equal => {
                // Compare lexicographically by character if both are Data nodes
                match (self, other) {
                    (
                        HuffmanNode::Data { character: c1, .. },
                        HuffmanNode::Data { character: c2, .. },
                    ) => c1.cmp(c2),

                    // If one is internal and one is data
                    (HuffmanNode::Data { .. }, _) => Ordering::Less,
                    (_, HuffmanNode::Data { .. }) => Ordering::Greater,

                    // Both internal nodes, treat them as equal
                    _ => Ordering::Equal,
                }
            }
            // Reverse to make BinaryHeap a min-heap
            other_order => other_order.reverse(),
        }
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

/// Encodes the input data string using a canonical Huffmman tree.
/// Canonical Huffman trees order every character on the same level on the tree lexicographically.
/// Additionaly, if the tree branches and characters on the same level, the characters are further the left (and sorted lexicographically) 
/// 
/// Example:
/// ```text
///     Normal Tree               Canonical Tree
///        /     \                   /     \
///      /  \    'c'               'c'    /  \    
///    'b'  'a'                     ↑   'a'  'b'
///                                 ↑    ↑    ↑
///                                 ↑   Sorted by alphabet
///                     Character's further on left than branch
/// ```
pub fn canonically_encode_data(data: &str) -> String {
    if data.len() == 0 {
        return String::from("");
    }

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
    encode_data(data, &codes)
}

fn assign_huffman_codes(root_node: &HuffmanNode) -> HashMap<char, String> {
    let mut huffman_codes: HashMap<char, String> = HashMap::new();

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

            for (character_index, char) in current_level_characters.iter().enumerate() {
                // Handles edge case of only having the same character in string
                if current_level == 0 {
                    current_level += 1;
                }

                let binary_string = get_binary_string((character_index + lowest_available_number).try_into().unwrap(), current_level as usize);
                huffman_codes.insert(*char, binary_string);
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
    if amount_of_digits > 32 {
        panic!("'amount_of_digits' can't exceed 32, supplied: {}", amount_of_digits);
    }

    let mut binary: String = String::new();
    for i in (0..32).rev() {
        binary += if ((number >> i) & 1) != 0 {"1"} else {"0"};
    }
    // Trim binary number
    binary[(binary.len() - amount_of_digits)..].to_string()
}

// Encodes a string of data using a Hashmap of Huffman codes
fn encode_data(data: &str, huffman_codes: &HashMap<char, String>) -> String {
    let mut encoded_data = String::new();
    for character in data.chars() {
        encoded_data += huffman_codes.get(&character).expect("Character to encode not found in Huffman tree");
    }
    encoded_data
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_binary_string() {
        assert_eq!(get_binary_string(0, 1), "0");
        assert_eq!(get_binary_string(0, 4), "0000");
        assert_eq!(get_binary_string(0, 8), "00000000");
        assert_eq!(get_binary_string(0, 16), "0000000000000000");
        assert_eq!(get_binary_string(0, 32), "00000000000000000000000000000000");
        assert_eq!(get_binary_string(u32::MAX, 32), "11111111111111111111111111111111");
        assert_eq!(get_binary_string(u32::MAX, 16), "1111111111111111");
        assert_eq!(get_binary_string(u32::MAX, 8), "11111111");
        assert_eq!(get_binary_string(u32::MAX, 4), "1111");
        assert_eq!(get_binary_string(u32::MAX, 1), "1");
    }

    #[test]
    fn test_huffman_encoding() {
        assert_eq!(canonically_encode_data("aaaa"), "0000");
        assert_eq!(canonically_encode_data("abab"), "0101");
        assert_eq!(canonically_encode_data("aaab"), "0001");
        assert_eq!(canonically_encode_data(""), "");
        assert_eq!(canonically_encode_data("aaaaaaaab"), "000000001");
    }
}