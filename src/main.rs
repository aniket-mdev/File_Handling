use core::fmt;
use std::{env, error::{self, Error}, fmt::write, fs::{self, File}, io::Read};
use std::io::{self, BufRead, BufReader, Write};
#[allow(non_snake_case)]
trait FileHandling {
    fn OpenFile(&self, filePath:&str) -> Result<File, Box<dyn error::Error>>;
    fn ReadFile(&self, file:&mut File) -> Result<String, Box<dyn error::Error>>;
    fn WriteFile(&self, file:&mut File, data:&str) -> Result<String, Box<dyn error::Error>>;
    fn DeleteFile(&self, filePath:&str) -> Result<bool, Box<dyn error::Error>>;
}

#[allow(non_snake_case)]
struct TxtFile {}

#[allow(non_snake_case)]
impl FileHandling for TxtFile {
    fn OpenFile(&self, filePath:&str) -> Result<File, Box<dyn error::Error>> {
        match fs::OpenOptions::new()
                .create(true)
                .append(true)
                .read(true)
                .write(true)
                .open(filePath){
                    Ok(file) => Ok(file),
                    Err(e) => Err(Box::new(e)),
                }
    }

    fn ReadFile(&self, file:&mut File) -> Result<String, Box<dyn error::Error>> {
        let mut content:String = String::new();
        let result = file.read_to_string(&mut content);
        if result.is_err() {
            let err_msg = format!("{:?}", result.err());
            return Err(Box::new(io::Error::new(io::ErrorKind::Other, err_msg)));
        } 
        Ok(content)
    }

    fn WriteFile(&self, file:&mut File, data:&str) -> Result<String, Box<dyn error::Error>> {
        match file.write_all(data.as_bytes()) {
            Ok(_) => Ok("File has been written".to_string()),
            Err(e) => Err(Box::new(e)),
        }
    }

    fn DeleteFile(&self, filePath:&str) -> Result<bool, Box<dyn error::Error>> {
        match fs::remove_file(filePath) {
            Ok(_) => Ok(true),
            Err(e) => Err(Box::new(e)),
        }
    }
}

fn main(){
  let input:Vec<String> = env::args().collect();
  let file_name = input.get(1);
  let file_type = input.get(2);

  if let (Some(path), Some(f_type)) = (file_name, file_type){
    if f_type == FileType::TXT.as_str() {
        let txt_file_handler = TxtFile{};
        let file_path = format!("{}.{}", path, f_type);
        match txt_file_handler.OpenFile(&file_path) {
            Ok(mut file) => {
                // read the file
                match txt_file_handler.ReadFile(&mut file) {
                    Ok(content) => println!("File content \n {content:?}"),
                    Err(e) => println!("Error while reading the file {e:?}"),
                }

                // write data in the same file
                match txt_file_handler.WriteFile(&mut file, "New Data\n") {
                    Ok(result) => println!("{result:?}"),
                    Err(e) => println!("Error while writing the file {e:?}"),
                }
            },
            Err(e) => println!("Error while Open the file : {e:?}"),
        }
    }
  }else {println!("provide a required params")}

}

enum FileType {
    TXT,
}

impl  FileType {
    fn as_str(&self) -> &'static str {
        match self {
            &FileType::TXT => "txt"
        }
    } 
}
