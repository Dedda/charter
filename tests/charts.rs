use charter::chart::{HorizontalBarChart, BarData};

#[test]
fn plot_horizontal_bar_graph_internals() {
    let mut hbc = HorizontalBarChart::new(false);
    hbc.push(BarData("five".to_string(), 5));
    hbc.push(BarData("nine".to_string(), 9));
    hbc.push(BarData("three".to_string(), 3));
    let plotted = hbc.plot_internal();
    assert_eq!("#####\n#########\n###", plotted.join("\n"));
}

#[test]
fn plot_horizontal_bar_graph_internals_padded() {
    let mut hbc = HorizontalBarChart::new(true);
    hbc.push(BarData("five".to_string(), 5));
    hbc.push(BarData("nine".to_string(), 9));
    hbc.push(BarData("three".to_string(), 3));
    let plotted = hbc.plot_internal();
    assert_eq!("#####\n\n#########\n\n###", plotted.join("\n"));
}

#[test]
fn plot_bar_labels_horizontal_bar_graph_internals() {
    let mut hbc = HorizontalBarChart::new(false);
    hbc.push(BarData("five".to_string(), 5));
    hbc.push(BarData("nine".to_string(), 9));
    hbc.push(BarData("three".to_string(), 3));
    let plotted = hbc.plot_bar_labels();
    assert_eq!("five  | #####\nnine  | #########\nthree | ###", plotted.join("\n"));
}


#[test]
fn plot_bar_labels_horizontal_bar_graph_internals_padded() {
    let mut hbc = HorizontalBarChart::new(true);
    hbc.push(BarData("five".to_string(), 5));
    hbc.push(BarData("nine".to_string(), 9));
    hbc.push(BarData("three".to_string(), 3));
    let plotted = hbc.plot_bar_labels();
    assert_eq!("five  | #####\n      |\nnine  | #########\n      |\nthree | ###", plotted.join("\n"));
}

#[test]
fn plot_bar_labels_horizontal_bar_graph_scaled() {
    let mut hbc = HorizontalBarChart::new(false);
    hbc.push(BarData("testvalue".to_string(), 16));
    hbc.push(BarData("testval".to_string(), 5));
    hbc.push(BarData("testval2".to_string(), 10));
    hbc.width = 20;
    let plotted = hbc.plot_bar_labels();
    assert_eq!("testvalue | ########\ntestval   | ##\ntestval2  | #####", plotted.join("\n"));
}