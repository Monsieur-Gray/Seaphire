fn return_vec(path: &str) -> Vec<Vec<String>>{
    use std::fs::read_to_string;
    let txt = if let Ok(bruh) = read_to_string(path) {
            bruh
        }
        else {
            crate::SysThrow!("I can't find this fucking file!\n\tYOU HAD ONE JOB!")
        };

    let mut str_v: Vec<Vec<String>> = Vec::new();
    let mut final_output: Vec<Vec<String>> = Vec::new();

    for line in txt.lines() {       // first split according to quotes ("")
        if !line.is_empty(){  
            let wrd: Vec<String> = line
            .split_terminator('\'')
            .map(|s| s.to_string())
            .filter(|f| !f.trim().is_empty())   // removes strings made up of only whitespaces
            .collect();
        
            str_v.push( wrd );  
        }
    };

    for i in str_v.iter() {
        if i.len() >= 2 {       // push first part (normal stuff) and secodn part (in quotes/strings) if it has a STRING
            let mut x: Vec<String> = i.first().unwrap()
                    .split_ascii_whitespace()
                    .filter(|a| *a != "\t")
                    .map(|s| s.to_string())
                    .collect();

            x.push( format!("'{}'", i.last().unwrap()) );
                final_output.push(x);
            }

            else {      // else push the entire line
                let x: Vec<String> = i.first().unwrap()
                    .split_ascii_whitespace()
                
                    .map( |f| f.to_string()).collect();
                if x.len() >= 1{
                    final_output.push(x);
                }
                else {continue;}
            };
    };

    return final_output;
}

pub fn read_from_str(path: &str) -> Vec<Vec<String>>{
    return_vec(path)
}

pub fn read_from_cli() -> Vec<Vec<String>> {
    use std::env;
    let path = env::args().collect::<Vec<String>>();
    if path.len() == 2 {
        return_vec(path[1].as_str())
    }
    else {
        crate::SysThrow!("Please enter the File path");
    }
}