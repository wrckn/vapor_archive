use std::{
    io::{
        Read,
        Write
    },
    fs::{
        File
    },
    path::PathBuf,
    error::Error
};

extern crate vapor_archive as var;

use var::{
    VarArchive
};

#[test]
fn test_reader_image_compressed() -> Result<(), Box<dyn Error>> {
    let file = File::open("tests/out/image_c.var")?;
    let mut archive = VarArchive::new(file)?;
    println!("Files in this archive:");
    for file in archive.get_file_list() {
        println!("\t{}", file);
    }
    let mut archive_file = archive.get_file("marbles.bmp")?;
    let mut file_content = vec![];
    archive_file.read_to_end(&mut file_content)?;
    let mut out_file = File::create("tests/out/marbles.bmp")?;
    out_file.write_all(&file_content)?;
    Ok(())
}

#[test]
fn test_reader_image_uncompressed() -> Result<(), Box<dyn Error>> {
    let file = File::open("tests/out/image_u.var")?;
    let mut archive = VarArchive::new(file)?;
    println!("Files in this archive:");
    for file in archive.get_file_list() {
        println!("\t{}", file);
    }
    let mut archive_file = archive.get_file("marbles.bmp")?;
    let mut file_content = vec![];
    archive_file.read_to_end(&mut file_content)?;
    let mut out_file = File::create("tests/out/marbles.bmp")?;
    out_file.write_all(&file_content)?;
    Ok(())
}

#[test]
fn test_reader_text_compressed() -> Result<(), Box<dyn Error>> {
    let file = File::open("tests/out/text_c.var")?;
    let mut archive = VarArchive::new(file)?;
    println!("Files in this archive:");
    for file in archive.get_file_list() {
        println!("\t{}", file);
    }
    let mut archive_file = archive.get_file("bsd.md")?;
    let mut file_content = vec![];
    archive_file.read_to_end(&mut file_content)?;
    let mut out_file = File::create("tests/out/bsd.md")?;
    out_file.write_all(&file_content)?;
    Ok(())
}

#[test]
fn test_reader_text_uncompressed() -> Result<(), Box<dyn Error>> {
    let file = File::open("tests/out/text_u.var")?;
    let mut archive = VarArchive::new(file)?;
    println!("Files in this archive:");
    for file in archive.get_file_list() {
        println!("\t{}", file);
    }
    let mut archive_file = archive.get_file("bsd.md")?;
    let mut file_content = vec![];
    archive_file.read_to_end(&mut file_content)?;
    let mut out_file = File::create("tests/out/bsd.md")?;
    out_file.write_all(&file_content)?;
    Ok(())
}

#[test]
fn test_reader_info_image_compressed() -> Result<(), Box<dyn Error>> {
    let file = File::open("tests/out/image_c.var")?;
    let mut archive = VarArchive::new(file)?;
    println!("Files in this archive:");
    for file in archive.get_file_list() {
        println!("\t{}", file);
        println!("{}", archive.get_file_info("marbles.bmp")?);
    }
    Ok(())
}

#[test]
fn test_reader_info_image_uncompressed() -> Result<(), Box<dyn Error>> {
    let file = File::open("tests/out/image_u.var")?;
    let mut archive = VarArchive::new(file)?;
    println!("Files in this archive:");
    for file in archive.get_file_list() {
        println!("\t{}", file);
        println!("{}", archive.get_file_info("marbles.bmp")?);
    }
    Ok(())
}

#[test]
fn test_reader_info_text_compressed() -> Result<(), Box<dyn Error>> {
    let file = File::open("tests/out/text_c.var")?;
    let mut archive = VarArchive::new(file)?;
    println!("Files in this archive:");
    for file in archive.get_file_list() {
        println!("\t{}", file);
        println!("{}", archive.get_file_info(&file)?);
    }
    Ok(())
}

#[test]
fn test_reader_info_text_uncompressed() -> Result<(), Box<dyn Error>> {
    let file = File::open("tests/out/text_u.var")?;
    let mut archive = VarArchive::new(file)?;
    println!("Files in this archive:");
    for file in archive.get_file_list() {
        println!("\t{}", file);
        println!("{}", archive.get_file_info(&file)?);
    }
    Ok(())
}

#[test]
fn test_reader_info_mixed_compressed() -> Result<(), Box<dyn Error>> {
    let file = File::open("tests/out/mixed_c.var")?;
    let mut archive = VarArchive::new(file)?;
    println!("Files in this archive:");
    for file in archive.get_file_list() {
        println!("\t{}", file);
        println!("{}", archive.get_file_info(&file)?);
    }
    Ok(())
}

#[test]
fn test_reader_info_mixed_uncompressed() -> Result<(), Box<dyn Error>> {
    let file = File::open("tests/out/mixed_u.var")?;
    let mut archive = VarArchive::new(file)?;
    println!("Files in this archive:");
    for file in archive.get_file_list() {
        println!("\t{}", file);
        println!("{}", archive.get_file_info(&file)?);
    }
    Ok(())
}

#[test]
fn test_reader_mixed_compressed() -> Result<(), Box<dyn Error>> {
    let file = File::open("tests/out/mixed_c.var")?;
    let mut archive = VarArchive::new(file)?;
    let out_dir = PathBuf::from("tests/out");
    println!("Files in this archive:");
    for file in archive.get_file_list() {
        println!("\t{}", file);
        println!("{}", archive.get_file_info(&file)?);
        let mut var_file = archive.get_file(&file)?;
        let mut out_file = File::create(out_dir.join(&file))?;
        std::io::copy(&mut var_file, &mut out_file)?;
    }
    Ok(())
}

#[test]
fn test_reader_mixed_uncompressed() -> Result<(), Box<dyn Error>> {
    let file = File::open("tests/out/mixed_u.var")?;
    let mut archive = VarArchive::new(file)?;
    let out_dir = PathBuf::from("tests/out");
    println!("Files in this archive:");
    for file in archive.get_file_list() {
        println!("\t{}", file);
        println!("{}", archive.get_file_info(&file)?);
        let mut var_file = archive.get_file(&file)?;
        let mut out_file = File::create(out_dir.join(&file))?;
        std::io::copy(&mut var_file, &mut out_file)?;
    }
    Ok(())
}