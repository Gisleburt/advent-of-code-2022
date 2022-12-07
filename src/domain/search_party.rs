use std::ops::Range;
use std::str::FromStr;

pub struct Search(Range<usize>);

impl Search {
    /// Can compares with another search area to see if this one completely contains the other
    ///
    /// ```rust
    /// use std::str::FromStr;
    /// use advent_of_code_2022::domain::search_party::Search;
    ///
    /// # fn main() -> Result<(), String> {
    /// let outer = Search::from_str("3-7")?;
    /// let inner = Search::from_str("5-6")?;
    /// assert_eq!(outer.contains_completely(&inner), true);
    /// assert_eq!(inner.contains_completely(&outer), false);
    ///
    /// // A single entry contains itself
    /// let single = Search::from_str("6-6")?;
    /// assert_eq!(single.contains_completely(&single), true);
    /// # Ok(())
    /// # }
    /// ```
    pub fn contains_completely(&self, other: &Search) -> bool {
        for i in other.0.clone() {
            if !self.0.contains(&i) {
                return false;
            }
        }
        true
    }
    /// Can compares with another search area to see if this one completely contains the other
    ///
    /// ```rust
    /// use std::str::FromStr;
    /// use advent_of_code_2022::domain::search_party::Search;
    ///
    /// # fn main() -> Result<(), String> {
    /// let first = Search::from_str("3-5")?;
    /// let second = Search::from_str("5-6")?;
    /// assert_eq!(first.overlaps(&second), true);
    /// assert_eq!(second.overlaps(&first), true);
    /// # Ok(())
    /// # }
    /// ```
    pub fn overlaps(&self, other: &Search) -> bool {
        for i in other.0.clone() {
            if self.0.contains(&i) {
                return true;
            }
        }
        false
    }
}

impl FromStr for Search {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('-');
        let start = split
            .next()
            .ok_or_else(|| format!("Invalid input for search {}", s))?
            .parse::<usize>()
            .map_err(|_| format!("Invalid input for search {}", s))?;
        let end = split
            .next()
            .ok_or_else(|| format!("Invalid input for search {}", s))?
            .parse::<usize>()
            .map_err(|_| format!("Invalid input for search {}", s))?
            + 1;
        Ok(Self(Range { start, end }))
    }
}

pub struct SearchPair(Search, Search);

impl SearchPair {
    /// Checks if in a pair of search areas, one completely contains the other
    ///
    /// ```rust
    /// use std::str::FromStr;
    /// use advent_of_code_2022::domain::search_party::SearchPair;
    ///
    /// # fn main() -> Result<(), String> {
    /// let pair1 = SearchPair::from_str("3-7,5-6")?;
    /// assert_eq!(pair1.contains_complete_overlap(), true);
    ///
    /// let pair2 = SearchPair::from_str("100-100,200-200")?;
    /// assert_eq!(pair2.contains_complete_overlap(), false);
    /// # Ok(())
    /// # }
    /// ```
    pub fn contains_complete_overlap(&self) -> bool {
        self.0.contains_completely(&self.1) || self.1.contains_completely(&self.0)
    }

    /// Checks if a pair of search areas overlaps at all
    ///
    /// ```rust
    /// use std::str::FromStr;
    /// use advent_of_code_2022::domain::search_party::SearchPair;
    ///
    /// # fn main() -> Result<(), String> {
    /// let pair1 = SearchPair::from_str("3-5,5-6")?;
    /// assert_eq!(pair1.contains_overlap(), true);
    ///
    /// let pair2 = SearchPair::from_str("100-100,200-200")?;
    /// assert_eq!(pair2.contains_overlap(), false);
    /// # Ok(())
    /// # }
    /// ```
    pub fn contains_overlap(&self) -> bool {
        self.0.overlaps(&self.1)
    }
}

impl FromStr for SearchPair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(',');
        let left = split
            .next()
            .ok_or_else(|| format!("Invalid input for group {}", s))?;
        let right = split
            .next()
            .ok_or_else(|| format!("Invalid input for group {}", s))?;
        Ok(Self(Search::from_str(left)?, Search::from_str(right)?))
    }
}
