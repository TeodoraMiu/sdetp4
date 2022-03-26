use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs;
use std::os::linux::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::fs::FileTypeExt;
use permissions::is_executable;
use permissions::is_writable;
use permissions::is_readable;

fn main()->std::io::Result<()>{
    let args:Vec<String> = env::args().collect();
    let file = &args[1];
    let oFile = File::open(file)?;
    let mut br = BufReader::new(oFile);
    let command = &args[2];

    // let paths = fs::read_dir("./").unwrap();
    // for path in paths {
    //     println!("Name: {}", path.unwrap().path().display())
    // }
    // println!("{:?}", args);


    if command == "print"{
        let mut contents = String::new();
        br.read_to_string(&mut contents)?;
        println!("{}", contents);
    } else if command == "size"{
        let metadata = fs::metadata(file)?;
        println!("{}", metadata.len());
    } else if command == "owner"{
        let metadata = fs::metadata(file)?;
        println!("{}", metadata.st_uid());
        println!("{}", metadata.st_gid());
    } else if command == "mode_number"{
        let metadata = fs::metadata(file)?;
        println!("octal: {:o}", metadata.permissions().mode());
        println!("binary: {:b}", metadata.permissions().mode());
    } else if command == "type"{
        let metadata = fs::metadata(file)?;
        let file_type = metadata.file_type();
        
        if file_type.is_dir(){
            println!("{} is a directory", file);
        } else if file_type.is_file(){
            println!("{} is a file", file);
        } else if file_type.is_symlink(){
            println!("{} is a symbolic link", file);
        } else if file_type.is_block_device(){
            println!("{} is a block device", file);
        } else if file_type.is_char_device(){
            println!("{} is a char device", file);
        } else if file_type.is_fifo(){
            println!("{} is a fifo", file);
        } else if file_type.is_socket(){
            println!("{} is a socket", file);
        } else {
            println!("{} is an unknown file type", file);
        }
    } else if command == "mode_text"{
        let metadata = fs::metadata(file)?;
        let permissions = metadata.permissions();
        permissions.unwrap();
    }

    Ok(())

}
