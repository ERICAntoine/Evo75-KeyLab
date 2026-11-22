use std::io::{self, IsTerminal};
use std::process::ExitCode;

fn main() -> ExitCode {
    let status = match evo75_keylab::scan_receiver() {
        Ok(collections) => {
            if collections.is_empty() {
                println!("Aucun récepteur 36B0:3002 détecté.");
            }
            for item in collections {
                println!(
                    "{} | interface {} | page 0x{:04X} | usage 0x{:04X}",
                    item.product, item.interface, item.usage_page, item.usage
                );
            }
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("Le diagnostic HID a échoué : {error}");
            ExitCode::FAILURE
        }
    };

    if io::stdin().is_terminal() && io::stdout().is_terminal() {
        println!("\nAppuie sur Entrée pour fermer.");
        let _ = io::stdin().read_line(&mut String::new());
    }
    status
}
