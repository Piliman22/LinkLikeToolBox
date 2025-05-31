use crate::color::Output;

pub struct Banner {
    output: Output,
}

impl Banner {
    pub fn new() -> Self {
        Self {
            output: Output::new(),
        }
    }

    pub fn print_banner(&self) {
        let version = env!("CARGO_PKG_VERSION");

        self.output.print_highlight("\n==========================================");
        self.output.print_highlight("          LinkLikeToolBox");
        self.output.print_highlight(format!("             v{}", version).as_str());
        self.output.print_highlight("         created by pim4n");
        self.output.print_highlight("==========================================\n");
    }

    pub fn print_summary(&self) {
        self.output.print_info("You can use these commands");
        println!("   decrypt ab <file>     - Decrypts AssetBundle files");
        println!("   decrypt chart <file>  - Decompresses chart files to JSON");
        println!("   crypt ab <file>       - Encrypts AssetBundle files");
        println!("   crypt chart <file> [level] - Compresses JSON to chart files (optional compression level 0-9)");
        println!("   help                  - Displays this help message");
    }
}