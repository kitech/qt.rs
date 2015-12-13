// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZNK16QGenericArgument4nameEv() -> i32;
  fn _ZNK16QGenericArgument4dataEv() -> i32;
  fn _ZN16QGenericArgumentC1EPKcPKv(qthis: *mut c_void, arg0: *const c_char, arg1: *const uint8_t) -> i32;
}

// body block begin
// class sizeof(QGenericArgument)=16
pub struct QGenericArgument {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGenericArgument {
  pub fn name<T: QGenericArgument_name>(&mut self, value: T) -> i32 {
    value.name(self);
    return 1;
  }
}

pub trait QGenericArgument_name {
  fn name(self, this: &mut QGenericArgument) -> i32;
}

// proto: const char * QGenericArgument::name();
impl<'a> /*trait*/ QGenericArgument_name for () {
  fn name(self, this: &mut QGenericArgument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QGenericArgument4nameEv()};
    unsafe {_ZNK16QGenericArgument4nameEv()};
    return 1;
  }
}

impl /*struct*/ QGenericArgument {
  pub fn data<T: QGenericArgument_data>(&mut self, value: T) -> i32 {
    value.data(self);
    return 1;
  }
}

pub trait QGenericArgument_data {
  fn data(self, this: &mut QGenericArgument) -> i32;
}

// proto: void * QGenericArgument::data();
impl<'a> /*trait*/ QGenericArgument_data for () {
  fn data(self, this: &mut QGenericArgument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QGenericArgument4dataEv()};
    unsafe {_ZNK16QGenericArgument4dataEv()};
    return 1;
  }
}

impl /*struct*/ QGenericArgument {
  pub fn NewQGenericArgument<T: QGenericArgument_NewQGenericArgument>(value: T) -> QGenericArgument {
    let rsthis = value.NewQGenericArgument();
    return rsthis;
    // return 1;
  }
}

pub trait QGenericArgument_NewQGenericArgument {
  fn NewQGenericArgument(self) -> QGenericArgument;
}

// proto: void QGenericArgument::NewQGenericArgument(const char * aName, const void * aData);
impl<'a> /*trait*/ QGenericArgument_NewQGenericArgument for (&'a  String, &'a  u8) {
  fn NewQGenericArgument(self) -> QGenericArgument {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QGenericArgumentC1EPKcPKv()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as *const uint8_t;
    unsafe {_ZN16QGenericArgumentC1EPKcPKv(qthis, arg0, arg1)};
    let rsthis = QGenericArgument{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

