use charter::chart::{HorizontalBarChart, BarData};

fn main() {
    let mut hbc = HorizontalBarChart::new(false);
    hbc.push(BarData("three".to_string(), 3));
    hbc.push(BarData("five".to_string(), 5));
    hbc.push(BarData("fifteen".to_string(), 15));
    hbc.width = 20;
    hbc.character = '=';
    println!("{}", hbc.plot_bar_labels().join("\n"));
}