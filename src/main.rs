use LinkLike_CLI::Commands;
use std::env;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let commands = Commands::new();
    
    if args.len() >= 2 && args[1] == "download" {
        match commands.execute_download_command(&args).await {
            Ok(_) => {},
            Err(e) => eprintln!("Error: {}", e),
        }
    } else {
        commands.execute(&args)?;
    }
    
    Ok(())
}