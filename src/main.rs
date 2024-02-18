use std::{
    sync::atomic::{AtomicU8, Ordering},
    time::Duration,
};

use clap::Parser;
use zerg::{
    http::{Body, Method, Request},
    json, UriExt,
};

mod table;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("http://localhost:9999"))]
    url: String,

    #[arg(short, long, value_parser = humantime::parse_duration, default_value = "10s")]
    duration: Duration,

    #[arg(short, long, default_value_t = 400)]
    concurrency: usize,

    #[arg(short, long, default_value_t = 4)]
    threads: usize,
}

fn main() {
    let Args {
        url,
        duration,
        concurrency,
        threads,
    } = Args::parse();

    let create_transactions = zerg::Swarm::builder()
        .uri(&url)
        .threads(threads)
        .concurrency(concurrency)
        .duration(duration / 2)
        .request({
            let id = AtomicU8::new(0);
            move |uri| {
                let id = id.fetch_add(1, Ordering::Relaxed) % 5 + 1;
                Request::builder()
                    .uri(uri.with_path(format!("/clientes/{id}/transacoes")))
                    .method(Method::POST)
                    .header("content-type", "application/json")
                    .body(Body::from(json!({
                        "valor": 1,
                        "tipo": "c",
                        "descricao": "Dinheiros",
                    })))
                    .unwrap()
            }
        })
        .zerg()
        .unwrap();

    let view_account = zerg::Swarm::builder()
        .uri(&url)
        .threads(threads)
        .concurrency(concurrency)
        .duration(duration / 2)
        .request({
            let id = AtomicU8::new(0);
            move |uri| {
                let id = id.fetch_add(1, Ordering::Relaxed) % 5 + 1;
                Request::builder()
                    .uri(uri.with_path(format!("/clientes/{id}/extrato")))
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap()
            }
        })
        .zerg()
        .unwrap();

    let total = create_transactions.clone() + view_account.clone();

    crate::table::print_results_table(&total, &create_transactions, &view_account);
}
