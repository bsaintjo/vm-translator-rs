#![allow(non_snake_case)]
mod assembly;
mod parse;
mod babel;

use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use babel::Translation;

use crate::babel::{Babel, Command};

fn main() -> eyre::Result<()> {
    let mut babel = Babel::empty();
    if let Some(filepath) = env::args().nth(1).map(PathBuf::from) {
        let reader = BufReader::new(File::open(filepath)?);
        for line in reader.lines() {
            let line = line?;
            if let Some(line) = parse::remove_whitespace_comments(&line) {
                let cmd = line.parse::<Command>()?;
                let asm = babel.translate(&cmd);
                for instruction in asm {
                    println!("{}", instruction);
                }
            }
        }
        for instruction in Translation::finish() {
            println!("{}", instruction);
        }
    }
    Ok(())
}
