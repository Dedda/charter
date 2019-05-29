pub mod chart;
pub mod plot;

trait InsertBlanks {
    fn insert_blanks(self) -> Vec<String>;
}

impl InsertBlanks for Vec<String> {
    fn insert_blanks(self) -> Vec<String> {
        if self.is_empty() {
            return vec![];
        }
        let mut blanked = vec![];
        self.into_iter().for_each(|l| {
            blanked.push(l);
            blanked.push(String::new());
        });
        blanked.pop();
        blanked
    }
}

#[cfg(test)]
mod tests {
    use crate::InsertBlanks;

    #[test]
    fn insert_blanks() {
        let v: Vec<String> = vec!["a", "b", "cde"].into_iter().map(|s| s.to_string()).collect();
        assert_eq!("a\n\nb\n\ncde", v.insert_blanks().join("\n"));
    }
}