// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qiodevice::QIODevice;
use super::qbytearray::QByteArray;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN18QCryptographicHash7addDataEP9QIODevice(arg0: *mut c_void) -> i32;
  fn _ZN18QCryptographicHashD0Ev() -> i32;
  fn _ZN18QCryptographicHash5resetEv() -> i32;
  fn _ZN18QCryptographicHash7addDataEPKci(arg0: *const c_char, arg1: c_int) -> i32;
  fn _ZNK18QCryptographicHash6resultEv() -> i32;
  fn _ZN18QCryptographicHash7addDataERK10QByteArray(arg0: *const c_void) -> i32;
  fn _ZN18QCryptographicHashC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QCryptographicHash)=8
pub struct QCryptographicHash {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QCryptographicHash {
  pub fn addData<T: QCryptographicHash_addData>(&mut self, value: T) -> i32 {
    value.addData(self);
    return 1;
  }
}

pub trait QCryptographicHash_addData {
  fn addData(self, this: &mut QCryptographicHash) -> i32;
}

// proto: bool QCryptographicHash::addData(QIODevice * device);
impl<'a> /*trait*/ QCryptographicHash_addData for (&'a mut QIODevice) {
  fn addData(self, this: &mut QCryptographicHash) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCryptographicHash7addDataEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QCryptographicHash7addDataEP9QIODevice(arg0)};
    return 1;
  }
}

impl /*struct*/ QCryptographicHash {
  pub fn FreeQCryptographicHash<T: QCryptographicHash_FreeQCryptographicHash>(&mut self, value: T) -> i32 {
    value.FreeQCryptographicHash(self);
    return 1;
  }
}

pub trait QCryptographicHash_FreeQCryptographicHash {
  fn FreeQCryptographicHash(self, this: &mut QCryptographicHash) -> i32;
}

// proto: void QCryptographicHash::FreeQCryptographicHash();
impl<'a> /*trait*/ QCryptographicHash_FreeQCryptographicHash for () {
  fn FreeQCryptographicHash(self, this: &mut QCryptographicHash) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCryptographicHashD0Ev()};
    unsafe {_ZN18QCryptographicHashD0Ev()};
    return 1;
  }
}

impl /*struct*/ QCryptographicHash {
  pub fn reset<T: QCryptographicHash_reset>(&mut self, value: T) -> i32 {
    value.reset(self);
    return 1;
  }
}

pub trait QCryptographicHash_reset {
  fn reset(self, this: &mut QCryptographicHash) -> i32;
}

// proto: void QCryptographicHash::reset();
impl<'a> /*trait*/ QCryptographicHash_reset for () {
  fn reset(self, this: &mut QCryptographicHash) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCryptographicHash5resetEv()};
    unsafe {_ZN18QCryptographicHash5resetEv()};
    return 1;
  }
}

// proto: void QCryptographicHash::addData(const char * data, int length);
impl<'a> /*trait*/ QCryptographicHash_addData for (&'a  String, i32) {
  fn addData(self, this: &mut QCryptographicHash) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCryptographicHash7addDataEPKci()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_int;
    unsafe {_ZN18QCryptographicHash7addDataEPKci(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QCryptographicHash {
  pub fn result<T: QCryptographicHash_result>(&mut self, value: T) -> i32 {
    value.result(self);
    return 1;
  }
}

pub trait QCryptographicHash_result {
  fn result(self, this: &mut QCryptographicHash) -> i32;
}

// proto: QByteArray QCryptographicHash::result();
impl<'a> /*trait*/ QCryptographicHash_result for () {
  fn result(self, this: &mut QCryptographicHash) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QCryptographicHash6resultEv()};
    unsafe {_ZNK18QCryptographicHash6resultEv()};
    return 1;
  }
}

// proto: void QCryptographicHash::addData(const QByteArray & data);
impl<'a> /*trait*/ QCryptographicHash_addData for (&'a  QByteArray) {
  fn addData(self, this: &mut QCryptographicHash) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCryptographicHash7addDataERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN18QCryptographicHash7addDataERK10QByteArray(arg0)};
    return 1;
  }
}

impl /*struct*/ QCryptographicHash {
  pub fn NewQCryptographicHash<T: QCryptographicHash_NewQCryptographicHash>(value: T) -> QCryptographicHash {
    let rsthis = value.NewQCryptographicHash();
    return rsthis;
    // return 1;
  }
}

pub trait QCryptographicHash_NewQCryptographicHash {
  fn NewQCryptographicHash(self) -> QCryptographicHash;
}

// proto: void QCryptographicHash::NewQCryptographicHash(const QCryptographicHash & );
impl<'a> /*trait*/ QCryptographicHash_NewQCryptographicHash for (&'a  QCryptographicHash) {
  fn NewQCryptographicHash(self) -> QCryptographicHash {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCryptographicHashC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN18QCryptographicHashC1ERKS_(qthis, arg0)};
    let rsthis = QCryptographicHash{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

