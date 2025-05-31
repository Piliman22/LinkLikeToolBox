use LinkLike_CLI::{Banner, Commands};
use LinkLike_Core::{AssetBundle, Chart};

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
            if args.len() < 3 {
                banner.print_summary();
                return Ok(());
            }
            
            match args[2].as_str() {
                "ab" => {
                    if args.len() < 4 {
                        banner.print_summary();
                        return Ok(());
                    }
                    commands.decrypt_ab(&args[3])?;
                },
                "chart" => {
                    if args.len() < 4 {
                        banner.print_summary();
                        return Ok(());
                    }
                    commands.decompress_chart(&args[3])?;
                },
                _ => {
                    banner.print_summary();
                }
            }
        },
        "crypt" => {
            if args.len() < 3 {
                banner.print_summary();
                return Ok(());
            }
            
            match args[2].as_str() {
                "ab" => {
                    if args.len() < 4 {
                        banner.print_summary();
                        return Ok(());
                    }
                    commands.crypt_ab(&args[3])?;
                },
                "chart" => {
                    if args.len() < 4 {
                        banner.print_summary();
                        return Ok(());
                    }
                    
                    let level = if args.len() >= 5 {
                        args[4].parse::<u32>().unwrap_or(6)
                    } else {
                        6
                    };
                    
                    commands.compress_chart(&args[3], level)?;
                },
                _ => {
                    banner.print_summary();
                }
            }
        },
        _ => {
            banner.print_banner();
            banner.print_summary();
        }
    }

    Ok(())
}