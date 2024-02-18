# Rinha Zerg

Teste alternativo e não oficial de carga para a [Rinha de Backend 2024](https://github.com/zanfranceschi/rinha-de-backend-2024-q1).

O teste segue o mesmo padrão usado no [no benchmark da TechEmpower para webservers](https://www.techempower.com/benchmarks/), onde
o que é medido é principalmente quantas requisições seu serviço consegue servir com um determinado número de conexões simultâneas.

## Uso

Base instalar o executável e rodar `rinha`. Isso irá rodar um teste de 10 segundos contra sua API. Se precisar alterar algum parâmetro
essa é documentação completa da CLI.

```
Usage: rinha [OPTIONS]

Options:
  -u, --url <URL>                  [default: http://localhost:9999]
  -d, --duration <DURATION>        [default: 10s]
  -c, --concurrency <CONCURRENCY>  [default: 400]
  -t, --threads <THREADS>          [default: 4]
  -h, --help                       Print help
  -V, --version                    Print version
```

### Exemplo de saída

```
           Transações   Extrato     Total
 Requests       88036     77770    165806
 Reqs/sec       17607     15554     16581
 Avg         22.675ms  25.739ms  24.112ms
 Stdev       21.391ms  25.470ms  23.443ms
 Max         82.012ms  83.605ms  83.605ms
 p99         72.792ms  78.749ms  77.624ms
 p95         67.619ms  74.021ms  70.970ms
 p90         62.313ms  70.118ms  67.383ms
 p75         13.968ms  52.455ms  16.385ms
 p50         12.760ms  12.830ms  12.799ms
```

## Instalação

Só baixar [uma versão](https://github.com/reu/rinha-zerg/releases) compatível com seu sistema operacional e rodar o executável:

| Sistema Operacional | Download                                                                                                     |
| ------------------- | ------------------------------------------------------------------------------------------------------------ |
| Linux               | [Download](https://github.com/reu/rinha-zerg/releases/latest/download/rinha-x86_64-unknown-linux-gnu.tar.gz) |
| Mac                 | [Download](https://github.com/reu/rinha-zerg/releases/latest/download/rinha-x86_64-apple-darwin.tar.gz)      |
| Mac (ARM)           | [Download](https://github.com/reu/rinha-zerg/releases/latest/download/rinha-aarch64-apple-darwin.tar.gz)     |
| Windows             | [Download](https://github.com/reu/rinha-zerg/releases/latest/download/rinha-x86_64-pc-windows-gnu.exe.zip)    |

#### Instalar a partir do source

Se você tiver Rust e o Cargo instalados, você pode clonar esse repositório e compilar o executável:

```bash
git clone https://github.com/reu/rinha-zerg.git
cd rinha-zerg
cargo build --release
cp ./target/release/rinha /usr/local/bin/
```
