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
  fn _ZNK13QLatin1String4dataEv() -> i32;
  fn _ZN13QLatin1StringC1EPKc(qthis: *mut c_void, arg0: *const c_char) -> i32;
  fn _ZNK13QLatin1String4sizeEv() -> i32;
  fn _ZN13QLatin1StringC1ERK10QByteArray(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK13QLatin1String6latin1Ev() -> i32;
  fn _ZN13QLatin1StringC1EPKci(qthis: *mut c_void, arg0: *const c_char, arg1: c_int) -> i32;
}

// body block begin
// class sizeof(QLatin1String)=16
pub struct QLatin1String {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QLatin1String {
  pub fn data<T: QLatin1String_data>(&mut self, value: T) -> i32 {
    value.data(self);
    return 1;
  }
}

pub trait QLatin1String_data {
  fn data(self, this: &mut QLatin1String) -> i32;
}

// proto: const char * QLatin1String::data();
impl<'a> /*trait*/ QLatin1String_data for () {
  fn data(self, this: &mut QLatin1String) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QLatin1String4dataEv()};
    unsafe {_ZNK13QLatin1String4dataEv()};
    return 1;
  }
}

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

// proto: void QLatin1String::NewQLatin1String(const char * s);
impl<'a> /*trait*/ QLatin1String_NewQLatin1String for (&'a  String) {
  fn NewQLatin1String(self) -> QLatin1String {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QLatin1StringC1EPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    unsafe {_ZN13QLatin1StringC1EPKc(qthis, arg0)};
    let rsthis = QLatin1String{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLatin1String {
  pub fn size<T: QLatin1String_size>(&mut self, value: T) -> i32 {
    value.size(self);
    return 1;
  }
}

pub trait QLatin1String_size {
  fn size(self, this: &mut QLatin1String) -> i32;
}

// proto: int QLatin1String::size();
impl<'a> /*trait*/ QLatin1String_size for () {
  fn size(self, this: &mut QLatin1String) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QLatin1String4sizeEv()};
    unsafe {_ZNK13QLatin1String4sizeEv()};
    return 1;
  }
}

// proto: void QLatin1String::NewQLatin1String(const QByteArray & s);
impl<'a> /*trait*/ QLatin1String_NewQLatin1String for (&'a  QByteArray) {
  fn NewQLatin1String(self) -> QLatin1String {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QLatin1StringC1ERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QLatin1StringC1ERK10QByteArray(qthis, arg0)};
    let rsthis = QLatin1String{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLatin1String {
  pub fn latin1<T: QLatin1String_latin1>(&mut self, value: T) -> i32 {
    value.latin1(self);
    return 1;
  }
}

pub trait QLatin1String_latin1 {
  fn latin1(self, this: &mut QLatin1String) -> i32;
}

// proto: const char * QLatin1String::latin1();
impl<'a> /*trait*/ QLatin1String_latin1 for () {
  fn latin1(self, this: &mut QLatin1String) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QLatin1String6latin1Ev()};
    unsafe {_ZNK13QLatin1String6latin1Ev()};
    return 1;
  }
}

// proto: void QLatin1String::NewQLatin1String(const char * s, int sz);
impl<'a> /*trait*/ QLatin1String_NewQLatin1String for (&'a  String, i32) {
  fn NewQLatin1String(self) -> QLatin1String {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QLatin1StringC1EPKci()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_int;
    unsafe {_ZN13QLatin1StringC1EPKci(qthis, arg0, arg1)};
    let rsthis = QLatin1String{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

