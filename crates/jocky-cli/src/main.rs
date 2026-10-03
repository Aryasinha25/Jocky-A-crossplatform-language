mod client;

use clap::{Parser, Subcommand};
use jocky_api::{create_router, InMemoryStore};
use jocky_ir::generate_ir;
use jocky_lexer::lex;
use jocky_parser::parse;
use jocky_runtime::Runtime;
use std::fs;
use std::sync::Arc;
use tokio::net::TcpListener;

#[derive(Parser)]
#[command(name = "jocky")]
#[command(about = "JOCKY Forensic DSL CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        /// The path to the script to run
        file: String,
        /// Submit the result to the JOCKY Control Plane
        #[arg(long)]
        submit: bool,
    },
    /// Check the syntax of a JOCKY script without running it
    Check {
        /// The path to the script to check
        file: String,
    },
    /// Compile a JOCKY script to IR and print it
    Compile {
        /// The path to the script to compile
        file: String,
    },
    /// Serve the Investigation API
    Serve {
        /// Port to bind to
        #[arg(short, long, default_value = "8787")]
        port: u16,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run { file, submit } => {
            let content = fs::read_to_string(file).expect("Failed to read file");
            let tokens = lex(&content).expect("Lexer error");
            let ast = parse(&tokens).expect("Parser error");
            let ir = generate_ir(&ast);

            let mut runtime = Runtime::new();
            if let Err(e) = runtime.execute(&ir) {
                eprintln!("Runtime error: {}", e);
                return;
            }

            let result = runtime.generate_result();
            let evidence = runtime.get_evidence();
            let timeline = runtime.build_timeline();

            println!("JOCKY v0.1\n");
            println!("Target: {}", result.target);
            println!("Platform: {}\n", result.platform);

            println!("Collection");

            let mut sys_count = 0;
            let mut proc_count = 0;
            let mut net_count = 0;
            let mut file_count = 0;

            for ev in evidence {
                match ev.evidence_type.as_str() {
                    "system" => sys_count += 1,
                    "process" => proc_count += 1,
                    "network" => net_count += 1,
                    "file" => file_count += 1,
                    _ => {}
                }
            }

            if sys_count > 0 {
                println!("[✓] SYSTEM       {} records", sys_count);
            }
            if proc_count > 0 {
                println!("[✓] PROCESS      {} records", proc_count);
            }
            if net_count > 0 {
                println!("[✓] NETWORK      {} records", net_count);
            }
            if file_count > 0 {
                println!("[✓] FILES        {} records", file_count);
            }

            println!("\nCorrelation");
            if result.graph.edges.len() > 0 {
                let process_network_correlations = result
                    .graph
                    .edges
                    .iter()
                    .filter(|r| format!("{:?}", r.relationship) == "ConnectedTo")
                    .count();
                if process_network_correlations > 0 {
                    println!("[✓] PROCESS → NETWORK");
                } else {
                    println!("[✓] Relationships generated");
                }
            } else {
                println!("[-] No correlations found");
            }

            println!("\nDetection");
            let finding_count = runtime.get_findings().len();
            if finding_count > 0 {
                println!("[!] {} findings", finding_count);
            } else {
                println!("[-] 0 findings");
            }

            println!("\nTimeline");
            if timeline.events.len() > 0 {
                println!(
                    "[✓] {} events ordered chronologically",
                    timeline.events.len()
                );
            } else {
                println!("[-] Timeline empty");
            }

            println!("\nInvestigation");
            println!("Priority: {}", result.risk_score.priority);
            println!("Risk Score: {}\n", result.risk_score.score);

            let json_str = serde_json::to_string_pretty(&result).unwrap();
            fs::write("investigation.json", json_str)
                .expect("Failed to write investigation output");

            let timeline_str = serde_json::to_string_pretty(&timeline).unwrap();
            fs::write("timeline.json", timeline_str).expect("Failed to write timeline output");

            println!("Report\ninvestigation.json\ntimeline.json");

            if *submit {
                println!("\nSubmitting to Control Plane...");
                match client::ControlPlaneClient::from_env() {
                    Ok(client) => {
                        if let Err(e) = client.submit_investigation(&result).await {
                            eprintln!("[-] Submission failed: {}", e);
                            std::process::exit(1);
                        } else {
                            println!(
                                "[✓] Successfully submitted investigation {}",
                                result.investigation_id
                            );
                        }
                    }
                    Err(e) => {
                        eprintln!("[-] Configuration error: {}", e);
                        std::process::exit(1);
                    }
                }
            }
        }
        Commands::Check { file } => {
            let content = fs::read_to_string(file).expect("Failed to read file");
            let tokens = lex(&content).expect("Lexer error");
            let _ast = parse(&tokens).expect("Parser error");
            println!("Syntax check passed.");
        }
        Commands::Compile { file } => {
            let content = fs::read_to_string(file).expect("Failed to read file");
            let tokens = lex(&content).expect("Lexer error");
            let ast = parse(&tokens).expect("Parser error");
            let ir = generate_ir(&ast);
            let json = serde_json::to_string_pretty(&ir).unwrap();
            println!("{}", json);
        }
        Commands::Serve { port } => {
            let addr = format!("127.0.0.1:{}", port);
            println!("JOCKY API Server\n");
            println!("Host: 127.0.0.1");
            println!("Port: {}\n", port);
            println!("API:\nhttp://{}/api/v1", addr);

            let store = Arc::new(InMemoryStore::new());
            let app = create_router(store);

            let listener = TcpListener::bind(&addr).await.unwrap();
            axum::serve(listener, app).await.unwrap();
        }
    }
}
