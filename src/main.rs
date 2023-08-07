use rayon::prelude::*;
use std::path::Path;
use std::fs;
use regex::Regex;
use chrono::Local;

#[derive(Debug)]
struct Coordata{
    name: String,
    raw: String,
    _content: Option<String>,
    _toml: Option<String>,
    _csv: Option<String>,
}

impl Coordata{
    fn new(n:&str, c: &str) -> Coordata{
        Coordata{
            name:n.to_string(), 
            raw: c.to_string(), 
            _content: None, 
            _toml: None, 
            _csv: None
        }
    }

    fn from_file(file: &fs::DirEntry) -> Coordata{
        let _name = file.file_name();
        let _content = fs::read_to_string(file.path()).unwrap();
        Coordata::new(&_name.to_string_lossy(), &_content)
    }

    fn trim(mut self) -> Coordata{
        let re = Regex::new(r"(?m)^\s*\n").unwrap();
        let _content = re.replace_all(&self.raw,"");
        self._content = Some(_content.to_string());
        self
    }

    fn save(self, dir: &str){
        let _path = format!("{}/Finished_{}", dir, self.name);
        let _ = fs::write(_path, self._content.unwrap());
    }
}

trait CoordFile {
    fn is_txt(&self) -> bool;
}

impl CoordFile for fs::DirEntry {
    fn is_txt(&self) -> bool {
        self.path().extension() == Some("txt".as_ref())
    }
}

fn main() ->  Result<(), std::io::Error>{
    let date = Local::now();
    let input_dir = "./input";
    let _output_dir = "./output";
    let output_dir: &str = &format!("{}/{}", _output_dir, date.format("%Y%m%d%H%M").to_string());
    fs::create_dir_all(input_dir)?;
    
    let _input_dir = Path::new(input_dir);

    let _coordata: Vec<_> = fs::read_dir(input_dir)?
        .filter_map(Result::ok)
        .par_bridge()
        .filter(|f| f.is_txt())
        .map(|d| Coordata::from_file(&d))
        .collect();

    fs::create_dir_all(output_dir)?;
    let _ = _coordata.into_iter()
        .par_bridge()
        .for_each(|d| d.trim().save(output_dir));
    Ok(())
}