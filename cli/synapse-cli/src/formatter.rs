use serde::Serialize;

/// Output mode selected by the caller.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Table,
    Json,
}

/// Trait that types implement to render themselves as a human-readable table.
pub trait TableDisplay {
    fn headers() -> Vec<&'static str>;
    fn row(&self) -> Vec<String>;
}

/// Print `rows` as an aligned text table or as pretty-printed JSON,
/// depending on `format`.
pub fn print<T>(items: &[T], format: OutputFormat)
where
    T: Serialize + TableDisplay,
{
    if format == OutputFormat::Json {
        println!("{}", serde_json::to_string_pretty(items).unwrap());
        return;
    }

    // Table output
    let headers = T::headers();
    let rows: Vec<Vec<String>> = items.iter().map(|i| i.row()).collect();

    // Compute column widths
    let mut widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();
    for row in &rows {
        for (i, cell) in row.iter().enumerate() {
            if i < widths.len() {
                widths[i] = widths[i].max(cell.len());
            }
        }
    }

    // Header
    let header_line: Vec<String> = headers
        .iter()
        .zip(&widths)
        .map(|(h, w)| format!("{:<width$}", h, width = w))
        .collect();
    println!("{}", header_line.join("  "));

    // Separator
    let sep: Vec<String> = widths.iter().map(|w| "-".repeat(*w)).collect();
    println!("{}", sep.join("  "));

    // Rows
    for row in &rows {
        let cells: Vec<String> = row
            .iter()
            .zip(&widths)
            .map(|(c, w)| format!("{:<width$}", c, width = w))
            .collect();
        println!("{}", cells.join("  "));
    }

    if items.is_empty() {
        println!("(no data)");
    }
}

/// Print a single serialisable value as JSON or as a flat key/value table.
pub fn print_one<T: Serialize>(item: &T, format: OutputFormat) {
    if format == OutputFormat::Json {
        println!("{}", serde_json::to_string_pretty(item).unwrap());
        return;
    }
    // For single objects, pretty-print as JSON even in table mode since
    // the structures vary too much for a generic table.
    println!("{}", serde_json::to_string_pretty(item).unwrap());
}
