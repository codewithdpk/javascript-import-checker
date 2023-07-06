use std::fs::File;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() -> io::Result<()> {
    // Read the JavaScript file
    let file_path = Path::new("hello.js");
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);


    // Regular expression to match import statements
    let import_regex = Regex::new(r#"import \{(\w+)\} from "@mui/material""#).unwrap();

    // Iterate over each line in the file
    for line in reader.lines() {
        let line = line?;

        if let Some(captures) = import_regex.captures(&line) {

            let imported_module = captures.get(1).unwrap().as_str();

            let dynamic_import_string = format!("import {} from \"@mui/material/{}\"",imported_module,imported_module);
        
            let replaced_content = import_regex.replace_all(&line, dynamic_import_string);

            fs::write("hello.js", replaced_content.to_string()).expect("Failed to write file");

            println!("Imported module: {}", imported_module);
        }

    }

    Ok(())
}
