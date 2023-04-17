mod parser;
mod utils;

use chrono::Local;
use clap::{App, Arg};
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::io;
use std::io::BufRead;

use crate::parser::json::*;
use crate::utils::character_profiles::*;
use crate::utils::masks::*;

fn main() {
    let matches = App::new("Bytefreq Data Profiler")
        .version("1.0")
        .author("Andrew Morgan <andrew.morgan@6point6.co.uk>\nEdward Jones <edward.jones@6point6.co.uk>\n")
        .about("A command-line tool to generate data profiling reports based on various masking strategies.")
        .arg(
            Arg::new("grain")
	        .short('g')
  	        .long("grain")
	        .value_name("GRAIN")
                .help("Sets the grain type for masking:\n\
                   'H' - High grain (A for uppercase letters, a for lowercase letters, 9 for digits)\n\
                   'L' - Low grain (repeated pattern characters will be compressed to one)\n\
                   'U' - Unicode (uses Unicode general categories for masking\n\
                   'LU'- Low grain Unicode (repeated pattern classes compressed to one\n)")
	        .takes_value(true)
	        .default_value("LU"),
        )
        .arg(
            Arg::new("delimiter")
                .short('d')
                .long("delimiter")
                .value_name("DELIMITER")
                .help("Sets the delimiter used to separate fields in input tabular data.\n\
                   Default: '|' (pipe character)")
                .takes_value(true)
                .default_value("|"),
        )
        .arg(
            Arg::new("format")
                .short('f')
                .long("format")
                .value_name("FORMAT")
                .help("Sets the format of the input data:\n\
                   'json' - JSON data (each line should contain a JSON object)\n\
                   'tabular' - Tabular data (first line should be the header)")
                .takes_value(true)
                .default_value("tabular"),
        )
        .arg(
	    Arg::new("report")
		.short('r')
		.long("report")
		.value_name("REPORT")
		.help("Sets the type of report to generate:\n\
		       'DQ' - Data Quality (default)\n\
		       'CP' - Character Profiling")
		.takes_value(true)
		.default_value("DQ"),
	)
        .arg(
            Arg::new("pathdepth")
                .short('p')
                .long("pathdepth")
                .value_name("PATHDEPTH")
                .help("Sets the depth for JSON paths (applicable for JSON data only).")
                .takes_value(true)
                .default_value("2"),
        )
        .arg(
            Arg::new("remove_array_numbers")
                .short('a')
                .long("remove-array-numbers")
                .value_name("REMOVE_ARRAY_NUMBERS")
                .help("Remove array numbers when set to true")
                .takes_value(true)
                .default_value("false"),
        )
        .get_matches();

    let report = matches.value_of("report").unwrap();

    if report == "CP" {
        //character_profiling();
        match character_profiling() {
            Ok(_) => println!("--------END OF REPORT--------"),
            Err(e) => eprintln!("Error occurred during character profiling: {}", e),
        }
    } else {
        let grain = matches.value_of("grain").unwrap();
        let delimiter = matches.value_of("delimiter").unwrap();
        let format = matches.value_of("format").unwrap();

        // new code to process tabular or json data
        let stdin = io::stdin();
        let mut frequency_maps: Vec<HashMap<String, usize>> = Vec::new();
        let mut example_maps: Vec<HashMap<String, String>> = Vec::new();
        let mut column_names: HashMap<String, usize> = HashMap::new();
        let mut field_count_map: HashMap<usize, usize> = HashMap::new();
        let mut record_count: usize = 0;
        let pathdepth = matches
            .value_of("pathdepth")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let remove_array_numbers = matches.value_of("remove_array_numbers").unwrap() != "false";

        for line in stdin.lock().lines().filter_map(Result::ok) {
            if !line.is_empty() {
                if format == "json" {
                    //process_json_line(&line, &mut frequency_maps, &mut example_maps, grain, &mut column_names, pathdepth);
                    //process_json_line(&line, &mut frequency_maps, &mut example_maps, grain, &mut column_names, remove_array_numbers);
                    process_json_line(
                        &line,
                        &mut frequency_maps,
                        &mut example_maps,
                        grain,
                        &mut column_names,
                        pathdepth,
                        remove_array_numbers,
                    );
                } else {
                    if record_count == 0 {
                        let header = line; //+ delimiter + "Err1" + delimiter + "Err2";
                                           // Process header for tabular data
                        for (idx, name) in header
                            .split(delimiter)
                            .map(|s| s.trim().replace(" ", "_"))
                            .enumerate()
                        {
                            column_names.insert(name.to_string(), idx);
                            frequency_maps.push(HashMap::new());
                            example_maps.push(HashMap::new());
                        }
                    } else {
                        // Process tabular data
                        if !column_names.is_empty() {
                            let fields = line.split(delimiter).collect::<Vec<&str>>();
                            let mut processed_fields = Vec::new();

                            for (i, field) in fields.iter().enumerate() {
                                let column_name = match column_names.iter().find(|(_, &v)| v == i) {
                                    Some((name, _)) => name.clone(),
                                    None => {
                                        let extra_column_index = i + 1 - column_names.len();
                                        let new_name = format!("RaggedErr{}", extra_column_index);

                                        // Update column_names, frequency_maps, and example_maps for the new column
                                        column_names.insert(new_name.clone(), column_names.len());
                                        frequency_maps.push(HashMap::new());
                                        example_maps.push(HashMap::new());

                                        new_name
                                    }
                                };
                                processed_fields.push((column_name, field));
                            }

                            let field_count = processed_fields.len();
                            *field_count_map.entry(field_count).or_insert(0) += 1;

                            for (name, value) in &processed_fields {
                                let masked_value = mask_value(value, grain);

                                if let Some(idx) = column_names.get(name) {
                                    let count = frequency_maps[*idx]
                                        .entry(masked_value.clone())
                                        .or_insert(0);
                                    *count += 1;

                                    // Reservoir sampling
                                    let mut rng = thread_rng();
                                    if rng.gen::<f64>() < 1.0 / (*count as f64) {
                                        example_maps[*idx]
                                            .insert(masked_value.clone(), value.to_string());
                                    }
                                } else {
                                    // Handle the case when the column name is not found in the HashMap
                                    println!(
                                        "Warning: Column name not found in the HashMap: {}",
                                        name
                                    );
                                }
                            }
                        }
                    } //end of else
                }
                record_count += 1;
            }
        }

        let now = Local::now();
        let now_string = now.format("%Y%m%d %H:%M:%S").to_string();
        println!();
        println!("Data Profiling Report: {}", now_string);
        println!("Examined rows: {}", record_count);
        println!();
        println!("FieldsPerLine:");
        // Print the field count map
        for (field_count, frequency) in &field_count_map {
            println!("{} fields: {} rows", field_count, frequency);
        }

        println!();
        println!(
            "{:<32}\t{:<8}\t{:<8}\t{:<32}",
            "column", "count", "pattern", "example"
        );
        println!("{:-<32}\t{:-<8}\t{:-<8}\t{:-<32}", "", "", "", "");

        for (name, idx) in column_names.iter() {
            if let Some(frequency_map) = frequency_maps.get(*idx) {
                let mut column_counts = frequency_map
                    .iter()
                    .map(|(value, count)| (value, count))
                    .collect::<Vec<(&String, &usize)>>();

                column_counts.sort_unstable_by(|a, b| b.1.cmp(a.1));

                for (value, count) in column_counts {
                    let empty_string = "".to_string();
                    let example = example_maps[*idx].get(value).unwrap_or(&empty_string);

                    println!(
                        "col_{:05}_{}\t{:<8}\t{:<8}\t{:<32}",
                        idx, name, count, value, example
                    );
                }
            }
        }
    }
} // end of main
