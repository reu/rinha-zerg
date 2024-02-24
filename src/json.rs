use serde::Serialize;
use serde_json::{json, Value};
use zerg::BenchmarkResult;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct JsonResult {
    request_count: usize,
    success_count: usize,
    error_count: usize,
    #[serde(rename = "reqsPerSec")]
    reqs_per_second: f64,
    avg: f64,
    stdev: f64,
    max: f64,
    min: f64,
    p99: f64,
    p95: f64,
    p90: f64,
    p75: f64,
    p50: f64,
}

impl From<&BenchmarkResult> for JsonResult {
    #[rustfmt::skip]
    fn from(res: &BenchmarkResult) -> Self {
        Self {
            request_count: res.total_request_count(),
            success_count: res.success_count(),
            error_count: res.http_error_count() + res.tcp_error_count(),
            reqs_per_second: res.requests_per_second(),
            avg: res.average_time().map(|dur| dur.as_secs_f64()).unwrap_or_default(),
            stdev: res.standard_deviation().map(|dur| dur.as_secs_f64()).unwrap_or_default(),
            max: res.timings().max().map(|dur| dur.as_secs_f64()).unwrap_or_default(),
            min: res.timings().min().map(|dur| dur.as_secs_f64()).unwrap_or_default(),
            p99: res.percentiles().percentile(0.99).as_secs_f64(),
            p95: res.percentiles().percentile(0.95).as_secs_f64(),
            p90: res.percentiles().percentile(0.90).as_secs_f64(),
            p75: res.percentiles().percentile(0.75).as_secs_f64(),
            p50: res.percentiles().percentile(0.50).as_secs_f64(),
        }
    }
}

pub fn results_json(
    total: &BenchmarkResult,
    transactions: &BenchmarkResult,
    accounts: &BenchmarkResult,
) -> Value {
    json!({
        "total": JsonResult::from(total),
        "write": JsonResult::from(transactions),
        "read": JsonResult::from(accounts),
    })
}
