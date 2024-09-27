use std::fs::OpenOptions;
use std::io::{self, Read, Write};

struct DiaryEntry {
    date: String,
    content: String,
}

impl DiaryEntry {
    fn new(date: String, content: String) -> Self {
        Self { date, content }
    }

    fn save(&self, filename: &str) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(filename)?;
        writeln!(file, "{}\n{}", self.date, &self.content)?;
        Ok(())
    }

    fn load(filename: &str) -> io::Result<Vec<Self>> {
        let mut file = OpenOptions::new().read(true).open(filename)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let entries = content
            .split("\n\n")
            .map(|entry| {
                let mut lines = entry.splitn(2, '\n');
                let date = lines.next().unwrap_or("").to_string();
                let content = lines.next().unwrap_or("").to_string();
                Self { date, content }
            })
            .collect();
        Ok(entries)
    }
}

fn main() {
    let entry = DiaryEntry::new("2024-09-27".to_string(), "test".to_string());
    entry.save("diary.txt").unwrap();
    let entries = DiaryEntry::load("diary.txt").unwrap();
    for entry in entries {
        println!("Date: {}", entry.date);
        println!("Content: {}", entry.content);
    }
}
