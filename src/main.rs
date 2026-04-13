use std::collections::HashMap;

fn fingerprint(query: &str) -> String {
    query
        .split_whitespace()
        .map(|token| {
            if token.chars().any(|c| c.is_ascii_digit()) {
                "?".to_string()
            } else {
                token.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() {
    let logs = vec![
        "SELECT * FROM users WHERE id = 1",
        "SELECT * FROM users WHERE id = 2",
        "SELECT * FROM users WHERE id = 3",
        "SELECT * FROM orders WHERE user_id = 10",
    ];

    let mut counts: HashMap<String, usize> = HashMap::new();

    for query in logs {
        let fp = fingerprint(query);
        *counts.entry(fp).or_insert(0) += 1;
    }

    for (fp, count) in counts {
        println!("{count} -> {fp}");
    }
}