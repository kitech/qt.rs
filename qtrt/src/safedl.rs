extern crate libc;

//use libc;
use std::ffi::CString;

pub fn dlopen(filename : &str) ->*mut libc::c_void {
    let _filename = CString::new(filename).unwrap();
    // println!("{:?}", _filename);
    unsafe {
        let dlh = libc::dlopen(_filename.as_ptr(), libc::RTLD_NOW);
        // println!("dlh@{:?}", dlh);
        return dlh;
    }
}
pub fn dlopeni(filename :&str) -> usize { return dlopen(filename) as usize; }

pub fn dlclose(handle : *mut libc::c_void) {
    unsafe {
        libc::dlclose(handle);
    }
}
pub fn dlclosei(handle : usize) { dlclose(handle as *mut libc::c_void); }

pub fn dlsym(handle : *mut libc::c_void, symbol : &str) ->*mut libc::c_void{
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
pub fn dlsymi(handle :usize, symbol : &str) -> usize{ return dlsym(handle as *mut libc::c_void, symbol) as usize; }

pub fn dlerror(handle : *mut libc::c_void) -> String {
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
pub fn dlerrori(handle:usize) -> String {return dlerror(handle as *mut libc::c_void); }

