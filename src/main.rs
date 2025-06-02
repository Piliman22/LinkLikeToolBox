use LinkLike_CLI::Commands;

fn main() -> std::io::Result<()> {
    let commands = Commands::new();
    let args: Vec<String> = std::env::args().collect();
    commands.execute(&args)
}