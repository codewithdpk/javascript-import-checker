use std::fs::File;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use std::io::Write;

fn main() -> io::Result<()> {
    // Read the JavaScript file
    let file_path = Path::new("hello.js");
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);


    // Regular expression to match import statements
    let import_regex = Regex::new(r#"import \{(\w+)\} from "@mui/material""#).unwrap();

    let temp_file_path = Path::new("hello_tmp.js");
    let mut temp_file = File::create(temp_file_path)?;

    let lines = reader.lines();
    
    // Iterate over each line in the file
    for line in lines {
        let line = line?;

        if let Some(captures) = import_regex.captures(&line) {

            let imported_module = captures.get(1).unwrap().as_str();

            let dynamic_import_string = format!("import {} from \"@mui/material/{}\"",imported_module,imported_module);
        
            let replaced_content = import_regex.replace(&line, dynamic_import_string.as_str());

            writeln!(temp_file, "{}", replaced_content)?;

            println!("Imported module: {}", imported_module);
        }else{
            println!("Running {}",line);
            writeln!(temp_file, "{}", line)?;
        }
    }

    fs::rename("hello_tmp.js","hello.js")?;


    Ok(())
}
