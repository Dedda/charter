use crate::InsertBlanks;

pub struct BarData(pub String, pub i128);

pub struct HorizontalBarChart {
    empty_lines: bool,
    data: Vec<BarData>,
    pub width: usize,
    pub character: char,
}

impl HorizontalBarChart {
    pub fn new(empty_lines: bool) -> Self {
        HorizontalBarChart {
            empty_lines,
            data: vec![],
            width: 0,
            character: '#'
        }
    }

    pub fn push(&mut self, data: BarData) {
        self.data.push(data);
    }

    pub fn plot_internal(&self) -> Vec<String> {
        let scale = self.internals_scale();
        let lines: Vec<String> = self.data.iter().map(|BarData(_, data)| (0..(data.clone() as f64 * scale) as i64).map(|_| self.character).collect::<String>()).collect();
        if self.empty_lines {
            lines.insert_blanks()
        } else {
            lines
        }
    }

    pub fn plot_bar_labels(&self) -> Vec<String> {
        let mut lines = self.plot_internal();
        let mut cnt = 0;
        let max_label_length = self.max_label_length();
        self.data.iter().for_each(|BarData(label, _)| {
            lines[cnt] = format!("{:labelwidth$} | {}", label, lines[cnt], labelwidth = max_label_length);
            cnt += 1;
            if self.empty_lines && cnt < lines.len() {
                lines[cnt] = format!("{:labelwidth$} |", "", labelwidth = max_label_length);
                cnt += 1;
            }
        });
        lines
    }

    fn max_data(&self) -> i128 {
        self.data.iter().map(|BarData(_, x)| x.clone()).max().or(Some(0i128)).unwrap()
    }

    fn max_label_length(&self) -> usize {
        match self.data.iter().map(|BarData(label, _)| label.len()).max() {
            Some(x) => x,
            _ => 0,
        }
    }

    fn internals_width(&self) -> usize {
        if self.width == 0 {
            self.max_data() as usize
        } else {
            self.width - self.max_label_length() - 3
        }
    }

    fn internals_scale(&self) -> f64 {
        if self.width == 0 {
            return 1f64;
        }
        (self.internals_width() as f64) / (self.max_data() as f64)
    }
}

#[cfg(test)]
mod tests {
    use crate::chart::*;
    #[test]
    fn internal_scale() {
        let mut hbc = HorizontalBarChart::new(false);
        hbc.push(BarData("testvalue".to_string(), 16));
        hbc.width = 20;
        let scale = hbc.internals_scale();
        assert_eq!(0.5, scale);
    }
}