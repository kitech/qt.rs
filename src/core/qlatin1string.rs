// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qbytearray::QByteArray;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  const char * QLatin1String::data();
  fn _ZNK13QLatin1String4dataEv(qthis: *mut c_void) -> *mut c_char;
  // proto:  void QLatin1String::QLatin1String(const char * s);
  fn _ZN13QLatin1StringC1EPKc(qthis: *mut c_void, arg0: *mut c_char);
  // proto:  int QLatin1String::size();
  fn _ZNK13QLatin1String4sizeEv(qthis: *mut c_void) -> c_int;
  // proto:  void QLatin1String::QLatin1String(const QByteArray & s);
  fn _ZN13QLatin1StringC1ERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const char * QLatin1String::latin1();
  fn _ZNK13QLatin1String6latin1Ev(qthis: *mut c_void) -> *mut c_char;
  // proto:  void QLatin1String::QLatin1String(const char * s, int sz);
  fn _ZN13QLatin1StringC1EPKci(qthis: *mut c_void, arg0: *mut c_char, arg1: c_int);
}

// body block begin
// class sizeof(QLatin1String)=16
pub struct QLatin1String {
  pub qclsinst: *mut c_void,
}

  // proto:  const char * QLatin1String::data();
impl /*struct*/ QLatin1String {
  pub fn data<RetType, T: QLatin1String_data<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QLatin1String_data<RetType> {
  fn data(self , rsthis: &mut QLatin1String) -> RetType;
}

  // proto:  const char * QLatin1String::data();
impl<'a> /*trait*/ QLatin1String_data<String> for () {
  fn data(self , rsthis: &mut QLatin1String) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QLatin1String4dataEv()};
    let mut ret = unsafe {_ZNK13QLatin1String4dataEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto:  void QLatin1String::QLatin1String(const char * s);
impl /*struct*/ QLatin1String {
  pub fn NewQLatin1String<T: QLatin1String_NewQLatin1String>(value: T) -> QLatin1String {
    let rsthis = value.NewQLatin1String();
    return rsthis;
    // return 1;
  }
}

pub trait QLatin1String_NewQLatin1String {
  fn NewQLatin1String(self) -> QLatin1String;
}

  // proto:  void QLatin1String::QLatin1String(const char * s);
impl<'a> /*trait*/ QLatin1String_NewQLatin1String for (&'a  String) {
  fn NewQLatin1String(self) -> QLatin1String {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QLatin1StringC1EPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    unsafe {_ZN13QLatin1StringC1EPKc(qthis, arg0)};
    let rsthis = QLatin1String{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QLatin1String::size();
impl /*struct*/ QLatin1String {
  pub fn size<RetType, T: QLatin1String_size<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QLatin1String_size<RetType> {
  fn size(self , rsthis: &mut QLatin1String) -> RetType;
}

  // proto:  int QLatin1String::size();
impl<'a> /*trait*/ QLatin1String_size<i32> for () {
  fn size(self , rsthis: &mut QLatin1String) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QLatin1String4sizeEv()};
    let mut ret = unsafe {_ZNK13QLatin1String4sizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QLatin1String::QLatin1String(const QByteArray & s);
impl<'a> /*trait*/ QLatin1String_NewQLatin1String for (QByteArray) {
  fn NewQLatin1String(self) -> QLatin1String {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QLatin1StringC1ERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QLatin1StringC1ERK10QByteArray(qthis, arg0)};
    let rsthis = QLatin1String{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const char * QLatin1String::latin1();
impl /*struct*/ QLatin1String {
  pub fn latin1<RetType, T: QLatin1String_latin1<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.latin1(self);
    // return 1;
  }
}

pub trait QLatin1String_latin1<RetType> {
  fn latin1(self , rsthis: &mut QLatin1String) -> RetType;
}

  // proto:  const char * QLatin1String::latin1();
impl<'a> /*trait*/ QLatin1String_latin1<String> for () {
  fn latin1(self , rsthis: &mut QLatin1String) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QLatin1String6latin1Ev()};
    let mut ret = unsafe {_ZNK13QLatin1String6latin1Ev(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto:  void QLatin1String::QLatin1String(const char * s, int sz);
impl<'a> /*trait*/ QLatin1String_NewQLatin1String for (&'a  String, i32) {
  fn NewQLatin1String(self) -> QLatin1String {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QLatin1StringC1EPKci()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    unsafe {_ZN13QLatin1StringC1EPKci(qthis, arg0, arg1)};
    let rsthis = QLatin1String{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

