pub trait VecExt<T> {
    /// Checks if both vectors contain the same elements.
    /// Returns true if they're equal
    fn verify_elements(&self, vec2: &Vec<T>) -> bool;
    /// Returns each element, seperated by a space, in a String
    fn to_string(&self) -> String;
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