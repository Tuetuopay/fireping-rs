use crate::{PingResult, Target};
use core::cmp;
use std::collections::HashMap;

fn print_columns(indent: usize,
                 header: Vec<String>, columns: Vec<Vec<String>>) {
    let mut lens: Vec<usize> = columns.clone()
                                      .into_iter()
                                      .map(|c| c.into_iter()
                                                .map(|s| s.len())
                                                .max()
                                                .unwrap())
                                      .collect();

    print!("{}", " ".to_string().repeat(indent));
    for i in 0..header.len() {
        lens[i] = cmp::max(lens[i], header[i].len());
        let s = &header[i];
        print!("{}{} ", s, " ".to_string().repeat(lens[i] - s.len()));
    }
    println!("");
    let width: usize = lens.clone().into_iter().sum();
    println!("{}{}",
             " ".to_string().repeat(indent),
             "-".to_string().repeat(width + lens.len() - 1));
    for i in 0..columns[0].len() {
        // Indent
        print!("{}", " ".to_string().repeat(indent));

        for j in 0..columns.len() {
            let s = &columns[j][i];
            print!("{}{} ", s, " ".to_string().repeat(lens[j] - s.len()));
        }

        println!("");
    }
}

fn format_field<F, G>(ips: &Vec<Target>,
                      res: &HashMap<String, PingResult>,
                      f: F)
    -> Vec<String> where F: FnMut(&PingResult) -> G,
                         G: std::fmt::Debug {
    ips.into_iter()
       .map(|t| res.get(&t.target).unwrap())
       .map(f)
       .map(|t| format!("{:?}", t).replace("µ", "u"))
       .collect()
}

pub fn print_summary(ips: &Vec<Target>, res: &HashMap<String, PingResult>) {
    if ips.len() < 1 { return }

    print_columns(4, vec![
        "Target".to_string(),
        "IP".to_string(),
        "RTT".to_string(),
        "Loss".to_string()
    ], vec![
        ips.into_iter().map(|t| format!("{}", t.name)).collect(),
        ips.into_iter().map(|t| format!("{}", t.target)).collect(),
        format_field(&ips, &res, |r| r.rtt()),
        format_field(&ips, &res, |r| r.loss())
    ]);
}
