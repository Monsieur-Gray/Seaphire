use crate::defkeys::*;
use crate::fetch_data::fetch_num;

//---------------MAIN_SEC & VAR_SEC
pub fn split_code(code: Vec<Vec<Builtins>>) -> (Vec<Vec<Builtins>>, Vec<Vec<Builtins>>) {
    let mut line_num: usize = 0;
    let mut VAR_SEC: Vec<Vec<Builtins>> = Vec::new();
    let mut MAIN_SEC: Vec<Vec<Builtins>> = Vec::new();

    for line in &code {
         match &line.first().unwrap() {
        //---------- Main-section-------------------------------------------------
            Builtins::Section(Section::MAIN) => 
                MAIN_SEC = {
                    match &code.last().unwrap()[0]  {     
                        Builtins::Section(Section::END) => &code[line_num+1..code.len()-1],
                        _ => crate::Throw!("\n Missing _END: at the end of file\n")
                    }.to_owned()
                },

        //---------- Variable-section-------------------------------------------------
            Builtins::Section(Section::VARS) =>
            VAR_SEC = {
                    let sec_size: usize = fetch_num(&line[1]).unwrap() as usize;
                    let out = match &code[line_num + sec_size+1][0]  //CHECK EOS
                    {  
                        Builtins::Section(Section::EOS) => &code[ line_num+1..=line_num+sec_size ],
                        _ => crate::Throw!("\n Incorrect number of variables stated or Missing EOS\n")
                    }.to_owned();
            
                    out
                },

            _ => (),
        };

        line_num += 1; 
    };

    if MAIN_SEC.is_empty() {
        crate::Throw!("\tsplit_sec->!>Main section not found!")
    }
    else if VAR_SEC.is_empty() {
        crate::Throw!("\tsplit_sec->!>Variable Section not found!")
    }
    else {
        return(MAIN_SEC, VAR_SEC);
    }

}