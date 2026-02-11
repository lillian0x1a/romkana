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
        let mut buffer = String::new();
        let input_chars: Vec<char> = input.chars().collect();
        let mut i = 0;

        let max_key_len = self.table.keys()
            .map(|k| k.chars().count()).max().unwrap_or(0);
        while i < input_chars.len() {
            let mut matched = false;
            let mut match_len = 0;
            let mut replacement = String::new();
            let mut next_output = String::new();

            let max_j = (i + max_key_len).min(input_chars.len());
            for j in (i+1..=max_j).rev() {
                let key: String = if buffer.is_empty() {
                    input_chars[i..j].iter().collect()
                } else {
                    let mut s = buffer.clone();
                    s.extend(input_chars[i..j].iter());
                    s
                };

                if let Some((output, next)) = self.table.get(&key) {
                    matched = true;
                    match_len = j - i;
                    replacement = output.clone();
                    next_output = next.clone();
                    break;
                }
            }

            if matched {
                result.push_str(&replacement);
                buffer = next_output;
                i += match_len;
            } else {
                if !buffer.is_empty() {
                    result.push_str(&buffer);
                    buffer.clear();
                } else {
                    result.push(input_chars[i]);
                    i += 1;
                }
            }
        }
        if !buffer.is_empty() {
            result.push_str(&buffer);
        }
        result
    }
}
