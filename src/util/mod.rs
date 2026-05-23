extern crate unicode_width;
use unicode_width::UnicodeWidthStr;

fn pad_string(input: &str, width: usize) -> String {
	let length = UnicodeWidthStr::width(input);
	if length >= width {
		input.to_string()
	} else {
		let padding = " ".repeat(width - length);
		format!("{input}{padding}")
	}
}

pub fn create_discord_markdown_table(rows: Vec<Vec<String>>) -> String {
	let mut result = String::new();

	if rows.is_empty() {
		return result;
	}

	let columns = rows[0].len();

	let mut max_widths = vec![0; columns];
	for row in &rows {
		for (index, cell) in row.iter().enumerate() {
			max_widths[index] =
				std::cmp::max(max_widths[index], UnicodeWidthStr::width(cell.as_str()) + 2);
		}
	}

	let header_separator: Vec<String> = max_widths.iter().map(|&width| "-".repeat(width)).collect();

	for (i, row) in rows.iter().enumerate() {
		let padded_row: Vec<String> = row
			.iter()
			.enumerate()
			.map(|(index, cell)| pad_string(cell, max_widths[index]))
			.collect();

		result.push_str(&format!("|{}|\n", padded_row.join("|")));

		if i == 0 {
			result.push_str(&format!("|{}|\n", header_separator.join("|")));
		}
	}

	// Wrap in a fenced code block so Discord renders it monospaced.
	format!("```\n{result}\n```")
}
