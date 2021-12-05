use assert_cmd::prelude::*;
use std::error::Error;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

#[test]
fn read_short_file() -> Result<(), Box<dyn Error>> {
    let mut tempfile = NamedTempFile::new()?;
    writeln!(tempfile, "This is the first line.")?;
    writeln!(tempfile, "This is the second line!!")?;
    let temppath = tempfile.path().to_path_buf();

    let mut cmd = Command::cargo_bin("enigma")?;
    cmd.arg(temppath);
    cmd.assert()
        .success()
        .stdout("FLSVSVGLACSRVGKSQAGLSVSVGLAVAMBQPKSQA");

    Ok(())
}
