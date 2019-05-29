pub mod chart;
pub mod plot;

trait InsertBlanks {
    fn insert_blanks(self) -> Vec<String>;
}

impl InsertBlanks for Vec<String> {
    fn insert_blanks(self) -> Vec<String> {
        let len = self.len();
        let blanked = self.zip_flat(&vec![String::new(); len - 1]);
        blanked
    }
}

pub trait ZipFlat<T> where T: Clone {
    fn zip_flat(self, other: &Vec<T>) -> Vec<T>;
}

impl<T> ZipFlat<T> for Vec<T> where T: Clone {
    fn zip_flat(mut self, other: &Vec<T>) -> Vec<T>{
        if self.is_empty() {
            return other.clone();
        } else if other.is_empty() {
            return self;
        }
        let mut cnt = 0;
        loop {
            let inser_index = cnt * 2 + 1;
            if inser_index < self.len() && cnt < other.len() {
                self.insert(inser_index, other[cnt].clone());
            } else if cnt < other.len() {
                self.push(other[cnt].clone());
            }
            cnt += 1;
            if cnt >= self.len() && cnt >= other.len() {
                break;
            }
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::{InsertBlanks, ZipFlat};

    #[test]
    fn insert_blanks() {
        let v: Vec<String> = vec!["a", "b", "cde"].into_iter().map(|s| s.to_string()).collect();
        assert_eq!("a\n\nb\n\ncde", v.insert_blanks().join("\n"));
    }

    #[test]
    fn zip_flat() {
        let v1 = vec![1, 2, 3, 4, 5];
        let v2 = vec![6, 7, 8];
        assert_eq!(vec![1, 6, 2, 7, 3, 8, 4, 5], v1.clone().zip_flat(&v2));
        assert_eq!(vec![6, 1, 7, 2, 8, 3, 4, 5], v2.zip_flat(&v1));
    }
}