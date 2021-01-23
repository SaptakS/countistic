use std::fs;
use std::collections::HashMap;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let filename = arguments.next().expect("Please enter the access log filename");
    let options = arguments.next();
    let count_statistics = CountStatistics::new(filename).expect("Error creating count statistics");

    match options.as_deref() {
        Some("--sorted") | Some("-s") => {
            count_statistics.display_count_sorted("asc");
        },
        Some("--reverse-sorted") | Some("-rs") => {
            count_statistics.display_count_sorted("desc");
        },
        Some(s) => {
            println!("Invalid option: {}", s)
        }
        None => {
            count_statistics.display_count();
        }
    }
}

struct CountStatistics {
    statistics: HashMap<String, u16>
}

impl CountStatistics {
    fn new(log_file: String) -> Result<CountStatistics, std::io::Error> {
        let access_log = fs::read_to_string(log_file)?;
        let statistics = Self::insert_page_visits(access_log);
        return Ok(CountStatistics { statistics });
    }

    fn insert_page_visits(access_log: String) -> HashMap<String, u16> {
        let mut statistics = HashMap::new();
        for log in access_log.lines() {
            let log_chunks: Vec<&str> = log.split(' ').collect();
            // let date = log_chunks[3];
            let path = log_chunks[6];
            let status = log_chunks[8];
            // println!("Date: {}\nPath: {}\nStatus: {}", date, path, status);
            match path {
                "/" | "/index.html" => {
                    statistics = Self::count_path_visits(statistics, "/".to_string(), status);
                },
                "/rss.xml"=> {
                    statistics = Self::count_path_visits(statistics, "/rss.xml".to_string(), status);
                },
                _ => {
                    if path.starts_with("/posts/") && path.ends_with(".html") {
                        statistics = Self::count_path_visits(statistics, path.to_string(), status);
                    }
                }
            }
        }
        return statistics;
    }

    fn count_path_visits(mut statistics: HashMap<String, u16>, path: String, status: &str) -> HashMap<String, u16> {
        match status.parse::<i32>().unwrap() {
            200 => {
                let count = statistics.entry(path).or_insert(0);
                *count += 1;
            },
            _ => {}
        }
        return statistics;
    }

    fn display_count(&self) {
        for (path, visits) in self.statistics.iter() {
            println!("Visits for {} is: {}", path, visits);
        }
    }

    fn display_count_sorted(&self, order: &str) {
        let mut stats_vec: Vec<_> = self.statistics.iter().collect();
        match order {
            "asc" => stats_vec.sort_by(|a, b| a.1.cmp(b.1)),
            "desc" => stats_vec.sort_by(|a, b| b.1.cmp(a.1)),
            _ => println!("Invalid sort order! The valid options are: 'asc' or 'desc'")
        }
        for (path, visits) in stats_vec.iter() {
            println!("Visits for {} is: {}", path, visits);
        }
    }
}
