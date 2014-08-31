extern crate collections;

use collections::string::String ;
use std::vec::Vec;

#[allow(dead_code)]
struct Doc {
    crateName: collections::string::String,
    vec : std::vec::Vec<String>,
    version: collections::string::String,
}


// download doc if not exists
fn main(){
    //string is size-fixed
    //    #[allow(dead_code)]
    print_crate_list();


}

#[allow(dead_code)]

fn crate_name() ->   Vec<&'static str>  {
    let libvec:std::vec::Vec<&'static str>;
    libvec = vec!["std","arena","flate","flate","fourcc","getopts","glob","green","hexfloat","hexfloat","num","rand","regex","semver","serialize","sync","syntax","term","test","time","uuid","url","log"];
    return libvec;

}

fn print_crate_list() {
    let cl = crate_name();
    for c in cl.iter() {
        println!("{}",c);
    }
}


#[allow(dead_code)]
fn down_crate_module_list(name : String) -> Vec<String> {
    //    let url =  doc_path();
    wget_html(url)
}



//crate
//module 
//
fn down_doc(base:String, v:String, c:String, m:String){
    let doc_path = down_doc(base,c,m,v);


}

fn doc_path(base : String, c : String, m : String, version : String) -> String {
    let path = String::new();

    if base.len() <= 0 {
        return String::new();
    }
    
    path.append("/").append(base);
    if version.len > 0 {
        version.append("/");
        path.append(version);
    }
    if c.len > 0 {
        c.append("/");
        if m.len > 0 {
            m.append("/")
        }
        c.append(m);
    }
    
    return path.append(c);
}

fn wget_html(url : String) -> String {
    
}

fn poll_modules(htmlTag : String, html : String) {
    
}




