pub trait VecExt<T> {
    /// Checks if both vectors contain the same elements.
    /// Returns true if they're equal
    fn verify_elements(&self, vec2: &Vec<T>) -> bool;

    /// Returns each element, seperated by a space, in a String
    fn to_string(&self) -> String;
}

pub trait VecExtU8 {
    /// Converts a vector of a single u8 to a u8
    fn to_u8(&self) -> u8;

    /// Converts a vector of 2 * u8 to a u16
    fn to_u16(&self) -> u16;

    /// Converts a vector of 4 * u8 to a u32
    fn to_u32(&self) -> u32;
}

impl<T> VecExt<T> for Vec<T> 
where T: PartialEq + std::fmt::Display
{
    fn verify_elements(&self, vec2: &Vec<T>) -> bool{
        if self.len() != vec2.len() {
            return false;
        }
        for (item1, item2) in self.iter().zip(vec2.iter()) {
            if item1 != item2 {
                return false;
            }
        }
        true
    }

    fn to_string(&self) -> String {
        self.iter().map(|i| i.to_string())
                                 .collect::<Vec<_>>()
                                 .join(" ")
    }
}

impl VecExtU8 for Vec<u8> {
    fn to_u8(&self) -> u8 {
        if self.len() != 4 {
            panic!("Vector must have a length of 1");
        }
        self[0]
    }

    fn to_u16(&self) -> u16 {
        if self.len() != 2 {
            panic!("Vector must have a length of 2");
        }

        u16::from_ne_bytes([self[0], self[1]])
    }

    fn to_u32(&self) -> u32 {
        if self.len() != 4 {
            panic!("Vector must have a length of 4");
        }
        // Convert vector to array
        let bytes: [u8; 4] = [self[0], self[1], self[2], self[3]];

        u32::from_be_bytes(bytes)
    }
}