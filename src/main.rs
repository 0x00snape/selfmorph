#[link_section = "ar.p"]
static mut VALUE: u32 = 0;

use object::{File, Object, ObjectSection};
use std::{env, fs::{self, rename, set_permissions, OpenOptions}, io::{Read, Write, Seek, SeekFrom}};

fn main() {
    
    let path = env::current_exe().expect("Failed to get binary path");
    let tmp = path.with_extension("tmp");

    fs::copy(&path, &tmp).expect("Failed to copy the file on tmp");
  
    let mut file = OpenOptions::new().read(true).write(true).open(&tmp).expect("Failed to open file");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("Failed to read the file");
 
    let obj_file = object::File::parse(&*buf).expect("Failed to parse object File");    
    
    if let Some(index) = section(obj_file, "ar.p") {
       
        let offset = index.0 as usize;
        buf[offset..(offset+4)].copy_from_slice(&(unsafe {VALUE}+1).to_ne_bytes());

        file.seek(SeekFrom::Start(0)).expect("Failed to move the cursor 0 bytes from the start of the file");
        file.write_all(&buf).expect("Failed to write to the buffer");
        
    }
         
    let permissions = fs::metadata(&path).unwrap().permissions();
    set_permissions(&tmp, permissions).expect("Failed to set_permissions");
    rename(&tmp, &path).expect("Failed to rename the file");

    println!("VALUE: {}", unsafe {VALUE});

}


// Getting range and section size 
fn section(file: File, name: &str) -> Option<(u64, u64)> {
    for section in file.sections() {
        if section.name().expect("Failed to get section name").to_string() == name {
            return section.file_range(); 
        }
    }
    None
}



