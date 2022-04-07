use std::io::{self, Cursor};

use hltas::HLTAS;
use hltas_framebulk_analyzer::analyzer::analyze_hltas;

fn main() {
    loop {
        // wait for input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // parse input
        let hltas = match HLTAS::from_str(&input) {
            Ok(hltas) => hltas,
            Err(_) => {
                // attempt to parse as section of hltas
                let hltas = hltas_header_on_framebulks_str(&input);

                match HLTAS::from_str(&hltas) {
                    Ok(hltas) => hltas,
                    Err(err) => {
                        // print error and loop continue
                        println!("{}", err);
                        continue;
                    }
                }
            }
        };

        // analyze hltas
        let analysis = match analyze_hltas(&hltas) {
            Ok(analysis) => analysis,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };

        // print analysis
        println!("{}", analysis);
        println!();
    }
}

pub fn hltas_header_on_framebulks_str(framebulks: &str) -> String {
    // making a hltas string from the default hltas to append the framebulks to
    let mut hltas = Cursor::new(Vec::new());
    HLTAS::default().to_writer(&mut hltas).unwrap();
    let hltas = hltas.into_inner();
    let mut hltas = String::from_utf8(hltas).unwrap();

    // append the framebulks to the hltas
    hltas.push('\n');
    hltas.push_str(framebulks);

    hltas
}
