
extern crate libc;

//use libc;
use std::ffi::CString;

fn dlopen(filename : &str) ->*mut libc::c_void {
    let _filename = CString::new(filename).unwrap();
    // println!("{:?}", _filename);
    unsafe {
        let dlh = libc::dlopen(_filename.as_ptr(), libc::RTLD_NOW);
        // println!("dlh@{:?}", dlh);
        return dlh;
    }
}
fn dlopeni(filename :&str) -> usize { return dlopen(filename) as usize; }

fn dlclose(handle : *mut libc::c_void) {
    unsafe {
        libc::dlclose(handle);
    }
}
fn dlclosei(handle : usize) { dlclose(handle as *mut libc::c_void); }

fn dlsym(handle : *mut libc::c_void, symbol : &str) ->*mut libc::c_void{
    let _symbol = CString::new(symbol).unwrap();
    // println!("{:?}", _symbol);
    unsafe {
        let f1 = libc::dlsym(handle, _symbol.as_ptr());
        let errstr = libc::dlerror();
        if errstr != 0 as *mut i8 {
            println!("f1@{:?}", f1);
            println!("{:?}", errstr);
            println!("{:?}", CString::from_raw(errstr));
        }
        return f1;
    }
}
fn dlsymi(handle :usize, symbol : &str) -> usize{ return dlsym(handle as *mut libc::c_void, symbol) as usize; }

fn dlerror(handle : *mut libc::c_void) -> String {
    unsafe {
        let errstr = libc::dlerror();
        if errstr != 0 as *mut i8 {
            println!("{:?}", errstr);
            println!("{:?}", CString::from_raw(errstr));
            return String::from(CString::from_raw(errstr).to_str().unwrap());
        }
    }
    return String::new();
}
fn dlerrori(handle:usize) -> String {return dlerror(handle as *mut libc::c_void); }

// fn dlopen(filename : str, flag : )
fn str2ci8(s : &str) -> *const i8 {
    CString::new(s).unwrap().as_ptr()
}
fn main () {
    let _dlh = dlopen("/usr/lib/libQt5Inline.so");
    let _f1 = dlsym(_dlh, "ffi_call_ex");
    println!("{:?}", dlerror(_dlh));
    dlclose(_dlh);
    println!("heheheh");
}
