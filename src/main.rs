use LinkLike_CLI::{Banner, Commands};
use LinkLike_Core::AssetBundle;

fn main() -> std::io::Result<()> {
    let banner = Banner::new();
    let commands = Commands::new();
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        banner.print_banner();
        banner.print_summary();
        return Ok(());
    }

    match args[1].as_str() {
        "decrypt" => {
            if args.len() < 4 || args[2] != "ab" {
                banner.print_summary();
                return Ok(());
            }
            commands.decrypt_ab(&args[3])?;
        }
        "crypt" => {
            if args.len() < 4 || args[2] != "ab" {
                banner.print_summary();
                return Ok(());
            }
            commands.crypt_ab(&args[3])?;
        }
        _ => {
            banner.print_banner();
            banner.print_summary();
        }
    }

    Ok(())
}