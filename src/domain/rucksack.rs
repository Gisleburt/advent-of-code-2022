use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct ItemType(usize);

impl ItemType {
    /// Gets the priority of the item type
    /// ```rust
    /// use advent_of_code_2022::domain::rucksack::ItemType;
    /// # fn main() -> Result<(), String> {
    /// // a - z is valued 1 - 26
    /// let lower_a = ItemType::from('a');
    /// let lower_z = ItemType::from('z');
    /// assert_eq!(lower_a.priority(), 1);
    /// assert_eq!(lower_z.priority(), 26);
    ///
    /// // A - Z is valued 27 - 52
    /// let upper_a = ItemType::from('A');
    /// let upper_z = ItemType::from('Z');
    /// assert_eq!(upper_a.priority(), 27);
    /// assert_eq!(upper_z.priority(), 52);
    /// # Ok(())
    /// # }
    /// ```
    pub fn priority(&self) -> usize {
        self.0
    }
}

impl From<char> for ItemType {
    fn from(c: char) -> Self {
        match c {
            'A'..='Z' => Self((c as usize) - 38),
            'a'..='z' => Self((c as usize) - 96),
            _ => Self(0),
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
/// assert_eq!(rucksack.left(), &['a'.into(), 'b'.into()]);
/// assert_eq!(rucksack.right(), &['c'.into(), 'd'.into()]);
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

    pub fn unique_items(&self) -> Vec<ItemType> {
        let mut v = Vec::with_capacity(2 * self.left().len());
        v.extend_from_slice(self.left());
        v.extend_from_slice(self.right());
        v.sort();
        v.dedup();
        v
    }

    pub fn contains(&self, i: &ItemType) -> bool {
        self.left().contains(i) || self.right().contains(i)
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
                c1.chars().map(|c| c.into()).collect::<Vec<_>>(),
                c2.chars().map(|c| c.into()).collect::<Vec<_>>(),
            ))
        }
    }
}

pub struct GroupRucksacks(Vec<Rucksack>);

impl GroupRucksacks {
    pub fn find_badge(&self) -> ItemType {
        let mut iter = self.0.iter();
        let mut remaining_items = iter
            .next()
            .expect("Called find_badge on empty group")
            .unique_items();
        for next in iter {
            remaining_items.retain(|i| next.contains(i));
        }
        if remaining_items.len() == 1 {
            remaining_items[0]
        } else {
            panic!("More than one item remained")
        }
    }
}

impl From<Vec<Rucksack>> for GroupRucksacks {
    fn from(v: Vec<Rucksack>) -> Self {
        Self(v)
    }
}
