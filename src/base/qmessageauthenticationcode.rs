// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qbytearray::QByteArray;
use super::qiodevice::QIODevice;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZNK26QMessageAuthenticationCode6resultEv() -> i32;
  fn _ZN26QMessageAuthenticationCode7addDataERK10QByteArray(arg0: *const c_void) -> i32;
  fn _ZN26QMessageAuthenticationCodeC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN26QMessageAuthenticationCode7addDataEPKci(arg0: *const c_char, arg1: c_int) -> i32;
  fn _ZN26QMessageAuthenticationCodeD0Ev() -> i32;
  fn _ZN26QMessageAuthenticationCode5resetEv() -> i32;
  fn _ZN26QMessageAuthenticationCode7addDataEP9QIODevice(arg0: *mut c_void) -> i32;
  fn _ZN26QMessageAuthenticationCode6setKeyERK10QByteArray(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QMessageAuthenticationCode)=8
pub struct QMessageAuthenticationCode {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMessageAuthenticationCode {
  pub fn result<T: QMessageAuthenticationCode_result>(&mut self, value: T) -> i32 {
    value.result(self);
    return 1;
  }
}

pub trait QMessageAuthenticationCode_result {
  fn result(self, this: &mut QMessageAuthenticationCode) -> i32;
}

// proto: QByteArray QMessageAuthenticationCode::result();
impl<'a> /*trait*/ QMessageAuthenticationCode_result for () {
  fn result(self, this: &mut QMessageAuthenticationCode) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK26QMessageAuthenticationCode6resultEv()};
    unsafe {_ZNK26QMessageAuthenticationCode6resultEv()};
    return 1;
  }
}

impl /*struct*/ QMessageAuthenticationCode {
  pub fn addData<T: QMessageAuthenticationCode_addData>(&mut self, value: T) -> i32 {
    value.addData(self);
    return 1;
  }
}

pub trait QMessageAuthenticationCode_addData {
  fn addData(self, this: &mut QMessageAuthenticationCode) -> i32;
}

// proto: void QMessageAuthenticationCode::addData(const QByteArray & data);
impl<'a> /*trait*/ QMessageAuthenticationCode_addData for (&'a  QByteArray) {
  fn addData(self, this: &mut QMessageAuthenticationCode) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QMessageAuthenticationCode7addDataERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN26QMessageAuthenticationCode7addDataERK10QByteArray(arg0)};
    return 1;
  }
}

impl /*struct*/ QMessageAuthenticationCode {
  pub fn NewQMessageAuthenticationCode<T: QMessageAuthenticationCode_NewQMessageAuthenticationCode>(value: T) -> QMessageAuthenticationCode {
    let rsthis = value.NewQMessageAuthenticationCode();
    return rsthis;
    // return 1;
  }
}

pub trait QMessageAuthenticationCode_NewQMessageAuthenticationCode {
  fn NewQMessageAuthenticationCode(self) -> QMessageAuthenticationCode;
}

// proto: void QMessageAuthenticationCode::NewQMessageAuthenticationCode(const QMessageAuthenticationCode & );
impl<'a> /*trait*/ QMessageAuthenticationCode_NewQMessageAuthenticationCode for (&'a  QMessageAuthenticationCode) {
  fn NewQMessageAuthenticationCode(self) -> QMessageAuthenticationCode {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QMessageAuthenticationCodeC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN26QMessageAuthenticationCodeC1ERKS_(qthis, arg0)};
    let rsthis = QMessageAuthenticationCode{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QMessageAuthenticationCode::addData(const char * data, int length);
impl<'a> /*trait*/ QMessageAuthenticationCode_addData for (&'a  String, i32) {
  fn addData(self, this: &mut QMessageAuthenticationCode) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QMessageAuthenticationCode7addDataEPKci()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_int;
    unsafe {_ZN26QMessageAuthenticationCode7addDataEPKci(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QMessageAuthenticationCode {
  pub fn FreeQMessageAuthenticationCode<T: QMessageAuthenticationCode_FreeQMessageAuthenticationCode>(&mut self, value: T) -> i32 {
    value.FreeQMessageAuthenticationCode(self);
    return 1;
  }
}

pub trait QMessageAuthenticationCode_FreeQMessageAuthenticationCode {
  fn FreeQMessageAuthenticationCode(self, this: &mut QMessageAuthenticationCode) -> i32;
}

// proto: void QMessageAuthenticationCode::FreeQMessageAuthenticationCode();
impl<'a> /*trait*/ QMessageAuthenticationCode_FreeQMessageAuthenticationCode for () {
  fn FreeQMessageAuthenticationCode(self, this: &mut QMessageAuthenticationCode) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QMessageAuthenticationCodeD0Ev()};
    unsafe {_ZN26QMessageAuthenticationCodeD0Ev()};
    return 1;
  }
}

impl /*struct*/ QMessageAuthenticationCode {
  pub fn reset<T: QMessageAuthenticationCode_reset>(&mut self, value: T) -> i32 {
    value.reset(self);
    return 1;
  }
}

pub trait QMessageAuthenticationCode_reset {
  fn reset(self, this: &mut QMessageAuthenticationCode) -> i32;
}

// proto: void QMessageAuthenticationCode::reset();
impl<'a> /*trait*/ QMessageAuthenticationCode_reset for () {
  fn reset(self, this: &mut QMessageAuthenticationCode) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QMessageAuthenticationCode5resetEv()};
    unsafe {_ZN26QMessageAuthenticationCode5resetEv()};
    return 1;
  }
}

// proto: bool QMessageAuthenticationCode::addData(QIODevice * device);
impl<'a> /*trait*/ QMessageAuthenticationCode_addData for (&'a mut QIODevice) {
  fn addData(self, this: &mut QMessageAuthenticationCode) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QMessageAuthenticationCode7addDataEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN26QMessageAuthenticationCode7addDataEP9QIODevice(arg0)};
    return 1;
  }
}

impl /*struct*/ QMessageAuthenticationCode {
  pub fn setKey<T: QMessageAuthenticationCode_setKey>(&mut self, value: T) -> i32 {
    value.setKey(self);
    return 1;
  }
}

pub trait QMessageAuthenticationCode_setKey {
  fn setKey(self, this: &mut QMessageAuthenticationCode) -> i32;
}

// proto: void QMessageAuthenticationCode::setKey(const QByteArray & key);
impl<'a> /*trait*/ QMessageAuthenticationCode_setKey for (&'a  QByteArray) {
  fn setKey(self, this: &mut QMessageAuthenticationCode) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QMessageAuthenticationCode6setKeyERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN26QMessageAuthenticationCode6setKeyERK10QByteArray(arg0)};
    return 1;
  }
}

