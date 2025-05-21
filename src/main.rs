#![allow(missing_docs)]

use codespan_reporting::files::SimpleFile;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use codespan_reporting::term::{self, Config};
use edn_parser::edn_parse;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        std::process::exit(1);
    }

    let writer = StandardStream::stderr(ColorChoice::Auto);
    let config = Config::default();
    let source = std::fs::read_to_string(&args[1])?;
    let cst = edn_parse(&source);
    let file = SimpleFile::new(&args[1], &source);

    if let Err(diags) = cst {
        for diag in &diags {
            term::emit(&mut writer.lock(), &config, &file, diag).unwrap();
        }
        std::process::exit(1);
    }
    let cst = cst.unwrap();
    for diag in cst.warnings.iter().flatten() {
        term::emit(&mut writer.lock(), &config, &file, diag).unwrap();
    }

    println!("{}", cst.cst);
    Ok(())
}
