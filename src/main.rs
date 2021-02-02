use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;
use std::env;
use std::time::{Duration, Instant};

// USAGE : remove_timestamp_osm <input.osm> <output.osm>

fn main(){

    let start = Instant::now();
    let args: Vec<String> = env::args().collect();

    let re_timestamp = Regex::new(r#"timestamp="[0123456789:TZ-]+" "#).unwrap();
    let re_version = Regex::new(r#"version="[0123456789]+" "#).unwrap();
    let re_created_by = Regex::new(r#"<tag k="created_by".*/>"#).unwrap();

    let file = File::open(&args[1]).unwrap();
    let reader = BufReader::new(file);

    let mut output = File::create(&args[2]).unwrap();
    
    for line in reader.lines() {
        let s = line.unwrap();
        let result_t = re_timestamp.replace_all(s.as_str(), "");
        
        let mut tmp_str: String = "".to_string();
        tmp_str.push_str(&result_t);
        let result_v = re_version.replace_all(&tmp_str, "");
        
        let mut tmp_str_2: String = "".to_string();
        tmp_str_2.push_str(&result_v);
        let result_c = re_created_by.replace_all(&tmp_str_2, "");

        write!(output, "{}",result_c).unwrap();
    }

    let duration = start.elapsed();

    println!("Time elapsed : {:?}", duration);
}