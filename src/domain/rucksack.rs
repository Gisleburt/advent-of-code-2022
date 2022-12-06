use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct ItemType(usize);

impl ItemType {
    /// Gets the priority of the item type
    /// ```rust
    /// use advent_of_code_2022::domain::rucksack::ItemType;
    /// # fn main() -> Result<(), String> {
    /// // a - z is valued 1 - 26
    /// let lower_a = ItemType::try_from('a')?;
    /// let lower_z = ItemType::try_from('z')?;
    /// assert_eq!(lower_a.priority(), 1);
    /// assert_eq!(lower_z.priority(), 26);
    ///
    /// // A - Z is valued 27 - 52
    /// let upper_a = ItemType::try_from('A')?;
    /// let upper_z = ItemType::try_from('Z')?;
    /// assert_eq!(upper_a.priority(), 27);
    /// assert_eq!(upper_z.priority(), 52);
    /// # Ok(())
    /// # }
    /// ```
    pub fn priority(&self) -> usize {
        self.0
    }
}

impl TryFrom<char> for ItemType {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A'..='Z' => Ok(Self((c as usize) - 38)),
            'a'..='z' => Ok(Self((c as usize) - 96)),
            _ => Err(format!("Expected letter, got '{}'", c)),
        }
    }
}

/// Stores items of item type. Evenly splits items into two compartments
///
/// ```rust
/// use std::str::FromStr;
/// use advent_of_code_2022::domain::rucksack::{ItemType, Rucksack};
///
/// # fn main() -> Result<(), String> {
/// let rucksack = Rucksack::from_str("abcd")?;
/// assert_eq!(rucksack.left(), &['a'.try_into()?, 'b'.try_into()?]);
/// assert_eq!(rucksack.right(), &['c'.try_into()?, 'd'.try_into()?]);
/// # Ok(())
/// # }
/// ```
pub struct Rucksack(Vec<ItemType>, Vec<ItemType>);

impl Rucksack {
    pub fn left(&self) -> &[ItemType] {
        &self.0
    }

    pub fn right(&self) -> &[ItemType] {
        &self.1
    }

    /// Stores items of item type. Evenly splits items into two compartments
    ///
    /// ```rust
    /// use std::str::FromStr;
    /// use advent_of_code_2022::domain::rucksack::{ItemType, Rucksack};
    ///
    /// # fn main() -> Result<(), String> {
    /// let rucksack = Rucksack::from_str("abaB")?;
    /// assert_eq!(rucksack.clashing_priority_value(), 1); // a => 1
    ///
    /// let rucksack = Rucksack::from_str("abab")?;
    /// assert_eq!(rucksack.clashing_priority_value(), 3); // a => 1 + b => 2 = 3
    /// # Ok(())
    /// # }
    /// ```
    pub fn clashing_priority_value(&self) -> usize {
        // TODO: Make less expensive
        let mut temp = self.left().to_vec();
        temp.sort();
        temp.dedup();

        temp.iter()
            .filter_map(|t| {
                if self.right().contains(t) {
                    Some(t.priority())
                } else {
                    None
                }
            })
            .sum()
    }
}

impl FromStr for Rucksack {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() % 2 != 0 {
            Err(format!(
                "Rucksack should have even length, actually had {}",
                s.len()
            ))
        } else {
            let (c1, c2) = s.split_at(s.len() / 2);
            Ok(Self(
                c1.chars()
                    .map(|c| c.try_into())
                    .collect::<Result<Vec<_>, _>>()?,
                c2.chars()
                    .map(|c| c.try_into())
                    .collect::<Result<Vec<_>, _>>()?,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::string_iter::StringIter;
    use std::io::Cursor;

    #[test]
    fn test_d03p1_example() {
        let input = Cursor::new("vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw");
        let iter = StringIter::<String, _>::from(input);
        let rucksacks: Result<Vec<_>, _> = iter.map(|r| Rucksack::from_str(&r)).collect();
        let sum: usize = rucksacks
            .unwrap()
            .iter()
            .map(|rs| rs.clashing_priority_value())
            .sum();
        assert_eq!(sum, 157)
    }
}
