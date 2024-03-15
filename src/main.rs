use std::{
    io,
    sync::atomic::{AtomicU8, Ordering},
    thread,
    time::Duration,
};

use clap::Parser;
use zerg::{
    http::{Body, Method, Request},
    json, UriExt,
};

mod json;
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

    #[arg(long, default_value_t = false)]
    json: bool,
}

fn main() {
    let Args {
        url,
        duration,
        concurrency,
        threads,
        json: json_output,
    } = Args::parse();

    let create_transactions = zerg::swarm(&url)
        .threads(threads / 2)
        .concurrency(concurrency / 2)
        .duration(duration)
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
        });

    let view_account = zerg::swarm(&url)
        .threads(threads / 2)
        .concurrency(concurrency / 2)
        .duration(duration)
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
        });

    let create_transactions = thread::spawn(move || create_transactions.zerg().unwrap());
    let view_account = thread::spawn(move || view_account.zerg().unwrap());

    let create_transactions = create_transactions.join().unwrap();
    let view_account = view_account.join().unwrap();

    let total = create_transactions.clone() + view_account.clone();

    let table = crate::table::results_table(&total, &create_transactions, &view_account);

    if json_output {
        table.print(&mut std::io::stderr()).unwrap();
        serde_json::to_writer_pretty(
            io::stdout(),
            &crate::json::results_json(&total, &create_transactions, &view_account),
        )
        .unwrap();
    } else {
        table.printstd();
    }
}
