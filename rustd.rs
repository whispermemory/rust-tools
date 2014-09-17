extern crate collections;
extern crate libc;
extern crate getopts;
extern crate regex;
extern crate regex_macros;

use collections::string::String ;
use std::vec::Vec;
use getopts::{optflag,getopts,optopt};
use std::os;
use libc::funcs::c95::stdlib::exit;
use std::io::TcpStream;
use regex::Regex;

#[allow(dead_code)]
struct Doc {
    crateName: collections::string::String,
    vec : std::vec::Vec<String>,
    version: collections::string::String,
}


// download doc if not exists

fn usage() ->String {
    println!("rustd -l : show crate list");
    println!("rustd cratename : show webdoc named cratename in terminal");
    return String::new();
}

fn main(){
    //string is size-fixed
    //    #[allow(dead_code)]
    let args: Vec<String> = os::args();

    let opts = [
        optflag("l", "list", "list crates"),
        optflag("h", "help", "print help"),
        optopt("d", "","download", "download doc"),
        ];
    
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => {m},
        Err(f) => {fail!(f.to_string())}
    };

    let optstr =  matches.opt_str("l");
    match optstr {
        Some(_)=> {},
        None => print_crate_list(),
    };

    let downDoc = matches.opt_str("d");
    match downDoc {
        Some(x) => wget_crate(x),
        _ => {usage()},
    };

    wget_crate(String::from_str("std"));

    all_refs_in_html(&String::from_str("abc"),&mut vec!(String::new()));
}

#[allow(dead_code)]

fn crate_name() ->   Vec<&'static str>  {
    let libvec:std::vec::Vec<&'static str>;
    libvec = vec!["std","arena","flate","flate","fourcc","getopts","glob","green","hexfloat","hexfloat","num","rand","regex","semver","serialize","sync","syntax","term","test","time","uuid","url","log"];
    return libvec;

}

#[allow(dead_code)]
fn print_crate_list() {
    let cl = crate_name();
    for c in cl.iter() {
        println!("{}",c);
    }
    //    unsafe{exit(0);}
}


fn wget_crate(name : String) -> String  {
    let baseurl = "54.215.0.2";
    let  mut s = TcpStream::connect(baseurl,80);
    let mut stream = match s {
        Ok(t) => t,
        Err(f) => {fail!(f.desc)},
    };
    
    let req_data = "GET / HTTP/1.1\r\nHost: doc.rust-lang.org\r\nConnection: Close\r\nUser-Agent: Wget/1.1.5\r\n\r\n";
    stream.write_str(req_data);
    let back = stream.read_to_string();
    stream.close_read();
    let doccontent = back.unwrap();
    drop(stream);
    let mut urlvec: Vec<String> = vec!();
    all_refs_in_html(&doccontent, &mut urlvec);
    return doccontent;
}

fn all_refs_in_html(content : &String, v: & mut Vec<String>)  {
    //<a href=""
    let re = match Regex::new(r"href=.*?.html") {
        Ok(re) => re,
        Err(err) => fail!("{}",err),
    };
    let str1=String::from_str("<a href=\"xxx.html\" <b href=\"abc.html\"");
    let str2="<a href=\"xxx.html\" <b href=\"abc.html\"";
    for pos in re.find_iter(str2) {
        println!("{}",pos);
        match pos {
            (x,y)=> {v.push(String::from_str(str2.slice_from(x+6).slice_to(y-x-6).as_slice()))},
            
        }
    }
}

