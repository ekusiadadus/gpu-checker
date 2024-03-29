use clap::{Parser, Subcommand};
use std::process::Command;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(about = "Show version information")]
    Version,
    #[clap(about = "Show GPU memory usage")]
    Memory,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Version => {
            println!("nvidia-smi-rs v{}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        Commands::Memory => loop {
            match get_gpu_memory_usage() {
                Ok(_) => {}
                Err(e) => eprintln!("Error: {}", e),
            }
            std::thread::sleep(std::time::Duration::from_secs(1));
            println!("\x1B[2J\x1B[1;1H");
        },
    }
}

fn get_gpu_memory_usage() -> std::io::Result<()> {
    let output = Command::new("nvidia-smi")
        .arg("--query-gpu=memory.total,memory.used")
        .arg("--format=csv,noheader,nounits")
        .output()?;

    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        for line in output_str.split('\n').filter(|line| !line.is_empty()) {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 2 {
                if let (Ok(total), Ok(used)) = (
                    parts[0].trim().parse::<f32>(),
                    parts[1].trim().parse::<f32>(),
                ) {
                    let usage = (used / total) * 100.0;
                    println!(
                        "GPU Memory Usage: {:.2}% {}",
                        usage,
                        generate_colored_gauge(usage)
                    );
                }
            }
        }
    } else {
        eprintln!("Failed to execute nvidia-smi");
    }

    Ok(())
}

fn generate_colored_gauge(usage: f32) -> String {
    let gauge_width = 50; // ゲージの幅
    let filled_length = (gauge_width as f32 * usage / 100.0).round() as usize;

    let (color, reset) = ("\x1B[48;5;", "\x1B[0m"); // ANSI背景色とリセット
    let color_code = if usage < 50.0 {
        "22m" // 緑
    } else if usage < 75.0 {
        "226m" // 黄色
    } else {
        "196m" // 赤
    };

    let filled = format!(
        "{}{}{}{}",
        color,
        color_code,
        " ".repeat(filled_length),
        reset
    );
    let empty = " ".repeat(gauge_width - filled_length);
    format!("[{}{}]", filled, empty)
}
