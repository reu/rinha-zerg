use std::time::Duration;

use prettytable::{format::Alignment, row, Row, Table};
use zerg::BenchmarkResult;

pub fn results_table(
    total: &BenchmarkResult,
    transactions: &BenchmarkResult,
    accounts: &BenchmarkResult,
) -> Table {
    let mut table = Table::new();

    table.add_row(row![
        "", "Requests", "Reqs/sec", "Avg", "Stdev", "Max", "p99", "p95", "p90", "p75", "p50",
    ]);

    for (title, result) in [
        ("Transações", transactions),
        ("Extrato", accounts),
        ("Total", total),
    ] {
        let percentiles = result.percentiles();
        let total = if result.http_error_count() > 0 || result.tcp_error_count() > 0 {
            format!(
                "{} ({} errors)",
                result.success_count(),
                result.http_error_count() + result.tcp_error_count()
            )
        } else {
            result.success_count().to_string()
        };
        table.add_row(row![
            title,
            total,
            format!("{:.0}", result.requests_per_second()),
            format_duration(result.average_time().unwrap_or_default()),
            format_duration(result.standard_deviation().unwrap_or_default()),
            format_duration(result.timings().max().unwrap_or_default()),
            format_duration(percentiles.percentile(0.99)),
            format_duration(percentiles.percentile(0.95)),
            format_duration(percentiles.percentile(0.90)),
            format_duration(percentiles.percentile(0.75)),
            format_duration(percentiles.percentile(0.50)),
        ]);
    }

    let mut print_table = Table::new();
    print_table.set_format(*prettytable::format::consts::FORMAT_CLEAN);
    let rows = table.row_iter().len();
    let cols = table.get_row(0).unwrap().len();

    for col in 0..cols {
        let mut current_row = Row::new(Vec::new());
        for row in 0..rows {
            let mut cell = table.get_row(row).unwrap().get_cell(col).unwrap().clone();
            if row != 0 {
                cell.align(Alignment::RIGHT);
            }
            current_row.add_cell(cell);
        }
        print_table.add_row(current_row);
    }

    print_table
}

fn format_duration(d: Duration) -> String {
    format!("{:.3}ms", d.as_secs_f64() * 1E3)
}
