extern crate vapor_archive as var;

use var::{
    blake2::{
        Blake2s,
        Digest
    },
    VarWriter,
    Compression
};

use std::{
    fs::{
        File
    },
    io::{
        Read,
        Write
    },
    error::Error
};

#[test]
fn test_writer_image_compressed() -> Result<(), Box<dyn Error>> {
    let file = File::create("tests/out/image_c.var")?;
    let mut writer = VarWriter::new(file)?;
    let mut var_file = writer.write_file("marbles.bmp", Compression::default())?;
    let mut input_file = File::open("tests/files/marbles.bmp")?;
    std::io::copy(&mut input_file, &mut var_file)?;
    Ok(())
}

#[test]
fn test_writer_image_uncompressed() -> Result<(), Box<dyn Error>> {
    let file = File::create("tests/out/image_u.var")?;
    let mut writer = VarWriter::new(file)?;
    let mut var_file = writer.write_file("marbles.bmp", Compression::None)?;
    let mut input_file = File::open("tests/files/marbles.bmp")?;
    std::io::copy(&mut input_file, &mut var_file)?;
    Ok(())
}

#[test]
fn test_writer_text_compressed() -> Result<(), Box<dyn Error>> {
    let file = File::create("tests/out/text_c.var")?;
    let mut writer = VarWriter::new(file)?;
    let mut var_file = writer.write_file("bsd.md", Compression::default())?;
    let mut input_file = File::open("tests/files/bsd.md")?;
    std::io::copy(&mut input_file, &mut var_file)?;
    Ok(())
}

#[test]
fn test_writer_text_uncompressed() -> Result<(), Box<dyn Error>> {
    let file = File::create("tests/out/text_u.var")?;
    let mut writer = VarWriter::new(file)?;
    let mut var_file = writer.write_file("bsd.md", Compression::None)?;
    let mut input_file = File::open("tests/files/bsd.md")?;
    std::io::copy(&mut input_file, &mut var_file)?;
    Ok(())
}

#[test]
fn test_writer_csv_compressed() -> Result<(), Box<dyn Error>> {
    let file = File::create("tests/out/csv_c.var")?;
    let mut writer = VarWriter::new(file)?;
    let mut var_file = writer.write_file("sales_records.csv", Compression::default())?;
    let mut input_file = File::open("tests/files/sales_records.csv")?;
    std::io::copy(&mut input_file, &mut var_file)?;
    Ok(())
}

#[test]
fn test_writer_csv_uncompressed() -> Result<(), Box<dyn Error>> {
    let file = File::create("tests/out/csv_u.var")?;
    let mut writer = VarWriter::new(file)?;
    let mut var_file = writer.write_file("sales_records.csv", Compression::None)?;
    let mut input_file = File::open("tests/files/sales_records.csv")?;
    std::io::copy(&mut input_file, &mut var_file)?;
    Ok(())
}

#[test]
fn test_writer_mixed_compressed() -> Result<(), Box<dyn Error>> {
    let file = File::create("tests/out/mixed_c.var")?;
    let mut writer = VarWriter::new(file)?;
    {
        let mut var_file = writer.write_file("sales_records.csv", Compression::default())?;
        let mut input_file = File::open("tests/files/sales_records.csv")?;
        std::io::copy(&mut input_file, &mut var_file)?;
    }
    {
        let mut var_file = writer.write_file("bsd.md", Compression::default())?;
        let mut input_file = File::open("tests/files/bsd.md")?;
        std::io::copy(&mut input_file, &mut var_file)?;
    }
    {
        let mut var_file = writer.write_file("marbles.bmp", Compression::default())?;
        let mut input_file = File::open("tests/files/marbles.bmp")?;
        std::io::copy(&mut input_file, &mut var_file)?;
    }
    Ok(())
}

#[test]
fn test_writer_mixed_uncompressed() -> Result<(), Box<dyn Error>> {
    let file = File::create("tests/out/mixed_u.var")?;
    let mut writer = VarWriter::new(file)?;
    {
        let mut var_file = writer.write_file("sales_records.csv", Compression::None)?;
        let mut input_file = File::open("tests/files/sales_records.csv")?;
        std::io::copy(&mut input_file, &mut var_file)?;
    }
    {
        let mut var_file = writer.write_file("bsd.md", Compression::None)?;
        let mut input_file = File::open("tests/files/bsd.md")?;
        std::io::copy(&mut input_file, &mut var_file)?;
    }
    {
        let mut var_file = writer.write_file("marbles.bmp", Compression::None)?;
        let mut input_file = File::open("tests/files/marbles.bmp")?;
        std::io::copy(&mut input_file, &mut var_file)?;
    }
    Ok(())
}