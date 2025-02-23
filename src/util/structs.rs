use std::fmt;

#[derive(Debug)]
pub struct Csv {
    pub headers: Vec<String>,
    pub values: Vec<Vec<String>>,
}

impl fmt::Display for Csv {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let distance = 3;

        let mut column_sizes: Vec<u32> = self
            .headers
            .iter()
            .map(|h| h.chars().count() as u32)
            .collect();

        for row in &self.values {
            column_sizes = row
                .iter()
                .map(|s| s.chars().count() as u32)
                .zip(column_sizes.iter())
                .map(|(a, b)| std::cmp::max(a, *b))
                .collect();
        }

        let format_row = |row: &Vec<String>| {
            row.iter()
                .zip(column_sizes.iter())
                .fold(String::new(), |acc, (entry, max)| {
                    let entry_length = entry.chars().count() as u32;
                    acc + entry + &" ".repeat((*max - entry_length + distance) as usize) + "| "
                })
        };

        let mut output = String::from("\n");

        output = output + &format_row(&self.headers) + "\n\n";

        for row in &self.values {
            output = output + &format_row(row) + "\n";
        }

        write!(f, "{}", output)
    }
}
