extern crate unicode_width;
use unicode_width::UnicodeWidthStr;

// Function to pad a string to a specified width with spaces
fn pad_string(input: &str, width: usize) -> String {
	let length = UnicodeWidthStr::width(input);
	if length >= width {
		input.to_string()
	} else {
		let padding = " ".repeat(width - length);
		format!("{}{}", input, padding)
	}
}

// Define a function to create a Markdown table from a vector of rows with formatting for Discord.
pub fn create_discord_markdown_table(rows: Vec<Vec<String>>) -> String {
	let mut result = String::new();

	if rows.is_empty() {
		return result;
	}

	// Determine the number of columns from the first row
	let columns = rows[0].len();

	// Find the maximum width for each column
	let mut max_widths = vec![0; columns];
	for row in &rows {
		for (index, cell) in row.iter().enumerate() {
			max_widths[index] =
				std::cmp::max(max_widths[index], UnicodeWidthStr::width(cell.as_str()) + 2);
			// Adjust for proper Unicode width
		}
	}

	// Create the header separator
	let header_separator: Vec<String> = max_widths.iter().map(|&width| "-".repeat(width)).collect();

	// Process each row, padding each cell to the max width of its column
	for (i, row) in rows.iter().enumerate() {
		let padded_row: Vec<String> = row
			.iter()
			.enumerate()
			.map(|(index, cell)| pad_string(cell, max_widths[index]))
			.collect();

		// Add row to the result
		result.push_str(&format!("|{}|\n", padded_row.join("|")));

		// Add the header separator after the first row (header)
		if i == 0 {
			result.push_str(&format!("|{}|\n", header_separator.join("|")));
		}
	}

	// Encapsulate in a monospace code block for Discord
	format!("```\n{}\n```", result)
}
