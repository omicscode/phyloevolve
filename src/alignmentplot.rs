use crate::astruct::Plotter;
use crate::filesearch::read;
use charts::{Chart, HorizontalBarView, ScaleBand, ScaleLinear};
use std::error::Error;
use std::fs::File;
use std::io::Write;

/*
Author Gaurav Sablok,
Email: codeprog@icloud.com
*/

#[tokio::main]
pub async fn plotter(path: &str) -> Result<String, Box<dyn Error>> {
    let (header, sequence) = read(path).unwrap();
    let mut plotread: Vec<Plotter> = Vec::new();
    for i in 0..sequence.len() {
        let seqvec: Vec<_> = sequence[i].chars().map(String::from).collect::<Vec<_>>();
        let mut counta: usize = 0usize;
        let mut countt: usize = 0usize;
        let mut countc: usize = 0usize;
        let mut countg: usize = 0usize;
        for i in seqvec.iter() {
            if i == "A" {
                counta += 1usize;
            } else if i == "T" {
                countt += 1usize;
            } else if i == "G" {
                countg += 1usize;
            } else if i == "C" {
                countc += 1usize;
            } else {
                continue;
            }
        }
        plotread.push(Plotter {
            name: header[i].clone(),
            count_a: counta,
            count_t: countt,
            count_c: countc,
            count_g: countg,
        })
    }

    let mut filecreate_a = File::create("heatplot.txt").expect("file not found");
    for i in plotread.iter() {
        writeln!(
            filecreate_a,
            "{:?}\t{:?}\t{:?}\t{:?}\t{:?}",
            i.name, i.count_a, i.count_t, i.count_c, i.count_g
        )
        .expect("file not found");
    }

    let mut max_a: Vec<usize> = Vec::new();
    let mut max_t: Vec<usize> = Vec::new();
    let mut max_g: Vec<usize> = Vec::new();
    let mut max_c: Vec<usize> = Vec::new();

    for i in plotread.iter() {
        max_a.push(i.count_a);
        max_t.push(i.count_t);
        max_g.push(i.count_g);
        max_c.push(i.count_c);
    }

    let mut chainiter: Vec<(&str, i32)> = Vec::new();
    for i in plotread.iter() {
        chainiter.push((&"A", i.count_a as i32));
        chainiter.push((&"T", i.count_t as i32));
        chainiter.push((&"G", i.count_g as i32));
        chainiter.push((&"C", i.count_c as i32));
    }

    let mut max_arr: Vec<usize> = Vec::new();
    let mut min_arr: Vec<usize> = Vec::new();
    max_arr.push(*max_a.iter().max().unwrap());
    max_arr.push(*max_t.iter().max().unwrap());
    max_arr.push(*max_g.iter().max().unwrap());
    max_arr.push(*max_c.iter().max().unwrap());

    min_arr.push(*max_a.iter().min().unwrap());
    min_arr.push(*max_t.iter().min().unwrap());
    min_arr.push(*max_g.iter().min().unwrap());
    min_arr.push(*max_c.iter().min().unwrap());

    let domainrangemax: usize = *max_arr.iter().max().unwrap();
    let domainrangemin: usize = *min_arr.iter().min().unwrap();

    let width = 1000;
    let height = 600;
    let (top, right, bottom, left) = (100, 40, 60, 80);
    let linearscalex = ScaleLinear::new()
        .set_domain(vec![domainrangemin as f32, domainrangemax as f32])
        .set_range(vec![0, width - left - right]);
    let linearscaley = ScaleBand::new()
        .set_domain(vec![
            String::from("A"),
            String::from("T"),
            String::from("G"),
            String::from("C"),
        ])
        .set_range(vec![height - top - bottom]);

    let linechart = HorizontalBarView::new()
        .set_x_scale(&linearscalex)
        .set_y_scale(&linearscaley)
        .load_data(&chainiter)
        .unwrap();
    Chart::new()
        .set_width(width)
        .set_height(height)
        .set_margins(top, right, bottom, left)
        .add_title(String::from("Line Chart"))
        .add_view(&linechart)
        .add_axis_bottom(&linearscalex)
        .add_axis_left(&linearscaley)
        .add_left_axis_label("Custom Y Axis Label")
        .add_bottom_axis_label("Custom X Axis Label")
        .save("line-chart.svg")
        .unwrap();

    Ok("the frequency plots have been written".to_string())
}
