use clap::Parser;
use std::process::Command;

#[derive(Parser)]
struct Cli {
    pattern: String,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    println!("pattern: {:?}", args.pattern);

    loop {
        match get_gpu_memory_usage() {
            Ok(_) => {}
            Err(e) => eprintln!("Error: {}", e),
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("\x1B[2J\x1B[1;1H");
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
                        "GPU Memory Usage: {:.2}% [{}]",
                        usage,
                        generate_gauge(usage)
                    );
                }
            }
        }
    } else {
        eprintln!("Failed to execute nvidia-smi");
    }

    Ok(())
}
fn generate_gauge(usage: f32) -> String {
    let gauge_width = 50; // ゲージの幅
    let filled_length = (gauge_width as f32 * usage / 100.0).round() as usize;
    let filled = "#".repeat(filled_length);
    let empty = " ".repeat(gauge_width - filled_length);
    format!("[{}{}]", filled, empty)
}
