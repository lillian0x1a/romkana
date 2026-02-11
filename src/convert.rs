use std::collections::HashMap;
use std::io;
use csv::ReaderBuilder;

pub struct RomKana {
    table: HashMap<String, (String, String)>,
}

impl RomKana {
    pub fn new(path: &str) -> io::Result<Self> {
        let mut rdr = ReaderBuilder::new()
            .has_headers(false).flexible(true).from_path(path)?;

        let mut map = HashMap::new();
    
        for result in rdr.records() {
            let record = result?;
            let key = record.get(0).unwrap_or("").to_string();
            let output = record.get(1).unwrap_or("").to_string();
            let next = record.get(2).unwrap_or("").to_string();

            if !key.is_empty() {
                map.insert(key, (output, next));
            }
        }
        Ok(Self {table: map})
    }
    
    pub fn convert(&self, input: &str) -> String {
        let mut result = String::new();
        let mut prev_output = String::new();
        let input_chars: Vec<char> = input.chars().collect();
        let mut i = 0;

        while i < input_chars.len() {
            let mut matched = false;
            let mut longest_match = 0;
            let mut replacement = String::new();
            let mut next_output = String::new();

            for j in (i+1..=input_chars.len()).rev() {
                let key: String = if prev_output.is_empty() {
                    input_chars[i..j].iter().collect()
                } else {
                    let mut s = prev_output.clone();
                    s.extend(input_chars[i..j].iter());
                    s
                };

                if let Some((output, next)) = self.table.get(&key) {
                    matched = true;
                    longest_match = j - i;
                    replacement = output.clone();
                    next_output = next.clone();
                    break;
                }
            }

            if matched {
                result.push_str(&replacement);
                prev_output = next_output;
                i += longest_match;
            } else {
                if !prev_output.is_empty() {
                    result.push_str(&prev_output);
                    prev_output.clear();
                } else {
                    result.push(input_chars[i]);
                    i += 1;
                }
            }

            if !prev_output.is_empty() {
                result.push_str(&prev_output);
            }
        }
        result
    }
}
