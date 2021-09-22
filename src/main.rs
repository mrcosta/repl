mod reqwest;
mod vectors;

use crate::reqwest::dns_made_easy_request;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

extern crate chrono_tz;

macro_rules! read_line(
    () => {{
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Something went wrong when reading the input.");
        String::from(line.trim())
    }}
);

#[tokio::main]
async fn main() {
    // let _file_name = read_line!();

    // count_requests_by_host(&file_name);
    dns_made_easy_request().await.unwrap();
}

fn count_requests_by_host(file_name: &str) {
    // TODO: return result instead of using expect
    let path = env::current_dir().expect("Path not found");
    let file = File::open(format!("{}_{}", path.display(), file_name))
        .expect(&format!("File {} was not found.", file_name));
    let lines = BufReader::new(file).lines();

    let mut count_request_by_hostname: HashMap<String, u32> = HashMap::new();
    for line_result in lines {
        let line = line_result.expect("Something wrong happened while reading the line");
        let hostname = line
            .clone()
            .split_whitespace()
            .map(|column| column.to_owned())
            .collect::<Vec<String>>()
            .first()
            .expect("Hostname was not found")
            .clone();

        count_request_by_hostname
            .entry(hostname)
            .and_modify(|request_count| *request_count += 1)
            .or_insert(1);
    }

    // TODO: extract write file logic to its own method
    let file_content = count_request_by_hostname
        .iter()
        .map(|(hostname, request_count)| format!("{} {}\n", hostname, request_count))
        .collect::<String>();
    println!("{}", format!("records_{}", file_name));
    fs::write(format!("records_{}", file_name), file_content).expect("Unable to write file");
}

#[cfg(test)]
mod tests {
    use crate::count_requests_by_host;

    #[test]
    fn test_case_1() {
        count_requests_by_host("hosts_access_log_00.txt");
    }
}
