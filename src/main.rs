use std::collections::HashMap;
use std::str::FromStr;

use plotly::{Layout, Plot, Scatter};
use plotly::common::{Mode, Title};
use plotly::layout::Axis;
use regex::Regex;

fn main() {
    let mut plot = Plot::new();
    let map = get_names();
    let map = get_traffic(map);
    let map = get_quality(map);
    let mut traffic = vec![];
    let mut quality = vec![];
    let mut min = u64::MAX;
    let mut max = 0;
    for entry in map {
        for traffic_data in entry.1.0 {
            for quality_data in entry.1.1.clone() {
                traffic.push(traffic_data);
                quality.push(quality_data);
            }
            min = u64::min(min, traffic_data);
            max = u64::max(max, traffic_data);
        }
        // if entry.1.0 == 0 || entry.1.1 == 0 {
        //     continue;
        // }
        // min = u64::min(min, entry.1.0);
        // max = u64::max(max, entry.1.0);
        // traffic.push(entry.1.0);
        // quality.push(entry.1.1);
        // println!("{}, {}, {}", entry.0, entry.1.0, entry.1.1);
    }

    let lr = linear_regression(traffic.clone(), quality.clone());
    let lr = (lr.0 as u64, lr.1 as u64, lr.2 as u64);
    let best_fit = Scatter::new(vec![min, max], vec![min * lr.0 + lr.1, max * lr.0 + lr.1]);
    let best_fit = best_fit.name("Line of Best Fit");
    plot.add_trace(best_fit);
    let mut trace = Scatter::new(traffic, quality);
    trace = trace.mode(Mode::Markers).name("Data");
    plot.add_trace(trace);
    let mut layout = Layout::new();
    let axis = Axis::new().title(Title::new("Average Daily Traffic"));
    layout = layout.x_axis(axis);
    let axis = Axis::new().title(Title::new("Road Quality, Higher is better"));
    layout = layout.y_axis(axis);
    layout = layout.width(720);
    plot.set_layout(layout);
    plot.write_html("out.html");
}

fn get_names<>() -> HashMap<String, (Vec<u64>, Vec<u64>, u64, u64)> {
    let file = String::from_utf8(std::fs::read("E:\\SJSU\\CS\\131\\finalproj\\traffic.txt").unwrap()).unwrap();
    let regex = Regex::new(r"(?m)([\r\n])+").unwrap();
    let split = regex.split(file.as_str());
    let mut map: HashMap<String, (Vec<u64>, Vec<u64>, u64, u64)> = HashMap::new(); // Traffic Count, Road Quality, # of traffic entries, # of quality entries
    for s in split {
        let regex = Regex::new(r"(?m),").unwrap();
        let mut s = regex.split(s).into_iter();
        let s = s.next().unwrap();
        if !map.contains_key(s) {
            map.insert(String::from(s), (vec![], vec![], 1, 0));
        } else {
            let mut tuple = map.get(s).unwrap().clone();
            tuple.2 += 1;
            map.insert(String::from(s), tuple);
        }
    }
    let file = String::from_utf8(std::fs::read("E:\\SJSU\\CS\\131\\finalproj\\quality.txt").unwrap()).unwrap();
    let split = regex.split(file.as_str());
    for s in split {
        let regex = Regex::new(r"(?m),").unwrap();
        let mut s = regex.split(s).into_iter();
        let s = s.next().unwrap();
        if !map.contains_key(s) {
            map.insert(String::from(s), (vec![], vec![], 0, 1));
        } else {
            let mut tuple = map.get(s).unwrap().clone();
            tuple.3 += 1;
            map.insert(String::from(s), tuple);
        }
    }
    map
}

fn get_traffic(mut map: HashMap<String, (Vec<u64>, Vec<u64>, u64, u64)>) -> HashMap<String, (Vec<u64>, Vec<u64>, u64, u64)> {
    let file = String::from_utf8(std::fs::read("E:\\SJSU\\CS\\131\\finalproj\\traffic.txt").unwrap()).unwrap();
    let regex = Regex::new(r"(?m)([\r\n])+").unwrap();
    let split = regex.split(file.as_str());
    for s in split {
        let regex = Regex::new(r"(?m),").unwrap();
        let mut s = regex.split(s).into_iter();
        let name = s.next().unwrap();
        let name = String::from(name);
        let next = s.next();
        if next.is_none() {
            continue;
        }
        let next = next.unwrap();
        let count = u64::from_str(next);
        if count.is_err() {
            continue;
        }
        let count = count.unwrap();
        let mut tuple: &mut (Vec<u64>, Vec<u64>, u64, u64) = map.get_mut(&name.clone()).unwrap();
        let new_traffic = count ;// map.get(&String::from(name.clone())).unwrap().2.clone();
        tuple.0.push(new_traffic);
        // map.insert(name, tuple.clone());
    }
    map
}

fn get_quality(mut map: HashMap<String, (Vec<u64>, Vec<u64>, u64, u64)>) -> HashMap<String, (Vec<u64>, Vec<u64>, u64, u64)> {
    let file = String::from_utf8(std::fs::read("E:\\SJSU\\CS\\131\\finalproj\\quality.txt").unwrap()).unwrap();
    let regex = Regex::new(r"(?m)([\r\n])+").unwrap();
    let split = regex.split(file.as_str());
    for s in split {
        let regex = Regex::new(r"(?m),").unwrap();
        let mut s = regex.split(s).into_iter();
        let name = s.next().unwrap();
        let name = String::from(name);
        let next = s.next();
        if next.is_none() {
            continue;
        }
        let next = next.unwrap();
        let quality = u64::from_str(next);
        if quality.is_err() {
            continue;
        }
        let quality = quality.unwrap();
        let mut tuple: &mut (Vec<u64>, Vec<u64>, u64, u64) = map.get_mut(&name.clone()).unwrap();
        let new_quality = quality; // / map.get(&String::from(name.clone())).unwrap().3;
        tuple.1.push(new_quality);
        // map.insert(name, tuple);
    }
    map
}

fn linear_regression(x: Vec<u64>, y: Vec<u64>) -> (i128, i128, i128) {
    let n = y.len() as i128;
    let mut sum_x: i128 = 0;
    let mut sum_y: i128 = 0;
    let mut sum_xy: i128 = 0;
    let mut sum_xx: i128 = 0;
    let mut sum_yy: i128 = 0;

    for i in 0..y.len() {
        sum_x += x[i] as i128;
        sum_y += y[i] as i128;
        sum_xy += x[i] as i128 * y[i]  as i128;
        sum_xx += x[i] as i128 * x[i] as i128;
        sum_yy += y[i] as i128 * y[i] as i128;
    }

    let mut result = (0, 0, 0);
    result.0 = (n * sum_xy - sum_x * sum_y) / (n * sum_xx - sum_x * sum_x);
    result.1 = (sum_y - result.0 * sum_x) / n;
    result.2 = i128::pow((n * sum_xy - sum_x * sum_y) / f64::sqrt(((n * sum_xx - sum_x * sum_x) * (n * sum_yy - sum_y * sum_y)) as f64) as i128, 2);
    result
}