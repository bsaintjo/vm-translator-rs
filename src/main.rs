#![allow(non_snake_case)]
mod assembly;
mod babel;
mod latt;
mod parse;
mod pointer;
mod segment;
mod temp;
mod statics;

use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use babel::Translation;

use crate::babel::{Babel, Command};

fn run<P: AsRef<Path>>(path: P) -> eyre::Result<()> {
    let basename = path
        .as_ref()
        .file_stem()
        .ok_or(eyre::eyre!("Not a file"))?
        .to_str()
        .ok_or(eyre::eyre!("Invalid filename bytes"))?;
    let mut babel = Babel::empty(basename);
    let reader = BufReader::new(File::open(path)?);
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
    Ok(())
}

fn main() -> eyre::Result<()> {
    if let Some(filepath) = env::args().nth(1).map(PathBuf::from) {
        run(&filepath)?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic() {
        run("extra/BasicTest/BasicTest.vm").unwrap();
    }
}
