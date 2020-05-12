use clap::{clap_app, crate_version};
use regex::Regex;
use std::path::Path;
use failure::{Error, Fail};
use std::fmt;

#[derive(Debug)]
struct Record {
    line: usize,
    tx:String,
}

#[derive(Debug)]
struct ArgErr {
    arg: &'static str,
}

impl Fail for ArgErr { }

impl fmt::Display for ArgErr {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "Argument Not provided!{}", self.arg)
    }
}


fn main() -> Result<(), Error> {
    let cp = clap_app!{
        pgrep => 
            (version: crate_version!())
            (about: "A Grep like program")
            (author: "JackY")
            (@arg file : -f --file +takes_value "The file to test")
            (@arg pattern : +required "The regexpattern to search for")
    }.get_matches();

    let re = Regex::new( cp.value_of("pattern").unwrap())?;
    //let p = process_file(
    let p = process_file(
        //cp.value_of("file").ok_or("No file chosen")?, 
        cp.value_of("file").ok_or(ArgErr {arg: "file"})?, 
        &re
    );
    println!("{:?}", p);
    Ok(())
}

fn process_file<P:AsRef<Path>>(p:P, re: &Regex) -> Result<Vec<Record>, Error>
{
    let mut res = Vec::new();
    let bts = std::fs::read(p)?;
    if let Ok(ss) = String::from_utf8(bts) {
        for (i, l) in ss.lines().enumerate() {
            if re.is_match(l) {
                res.push(Record {
                    line: i,
                    tx: l.to_string(),
                });
            }

        }

    };
    Ok(res)
}

/*
fn process_path<P, FF, EF>(p:P, re:&Regex, ff:&FF, ef:&EF) -> Result<(), Error> 
where 
    P:AsRef<Path>, 
    FF:Fn(&Path, Vec<Record>),
    EF: Fn(Error),
{
    let p = p.as_ref();
    let md = p.metadata()?;
    let ft = md.file_type();
    if ft.is_file() {
        let dt =  process_file(p ,re)?;
        ff(p, dt);
    }

    if ft.is_dir() {
        let dd = std::fs::read_dir(p)?;

        for d in dd {
            if let Err(e) = process_path(d?.path(), re, ff) {
                ef(e);
            }

        }
    }
    Ok(())

}
*/
