// auto generated, do not modify.
// created: Tue Dec 29 22:57:40 2015
// src-file: /QtCore/qmessageauthenticationcode.h
// dst-file: /src/core/qmessageauthenticationcode.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use std::ops::Deref;
use super::qbytearray::QByteArray; // 773
use super::qiodevice::QIODevice; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QMessageAuthenticationCode_Class_Size() -> c_int;
  // proto:  QByteArray QMessageAuthenticationCode::result();
  fn _ZNK26QMessageAuthenticationCode6resultEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QMessageAuthenticationCode::addData(const QByteArray & data);
  fn _ZN26QMessageAuthenticationCode7addDataERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QMessageAuthenticationCode::QMessageAuthenticationCode(const QMessageAuthenticationCode & );
  fn dector_ZN26QMessageAuthenticationCodeC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN26QMessageAuthenticationCodeC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QMessageAuthenticationCode::addData(const char * data, int length);
  fn _ZN26QMessageAuthenticationCode7addDataEPKci(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_int);
  // proto:  void QMessageAuthenticationCode::~QMessageAuthenticationCode();
  fn _ZN26QMessageAuthenticationCodeD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QMessageAuthenticationCode::reset();
  fn _ZN26QMessageAuthenticationCode5resetEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QMessageAuthenticationCode::addData(QIODevice * device);
  fn _ZN26QMessageAuthenticationCode7addDataEP9QIODevice(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  void QMessageAuthenticationCode::setKey(const QByteArray & key);
  fn _ZN26QMessageAuthenticationCode6setKeyERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QMessageAuthenticationCode)=8
#[derive(Default)]
pub struct QMessageAuthenticationCode {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QMessageAuthenticationCode {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QMessageAuthenticationCode {
    return QMessageAuthenticationCode{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QByteArray QMessageAuthenticationCode::result();
impl /*struct*/ QMessageAuthenticationCode {
  pub fn result<RetType, T: QMessageAuthenticationCode_result<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.result(self);
    // return 1;
  }
}

pub trait QMessageAuthenticationCode_result<RetType> {
  fn result(self , rsthis: & QMessageAuthenticationCode) -> RetType;
}

  // proto:  QByteArray QMessageAuthenticationCode::result();
impl<'a> /*trait*/ QMessageAuthenticationCode_result<QByteArray> for () {
  fn result(self , rsthis: & QMessageAuthenticationCode) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK26QMessageAuthenticationCode6resultEv()};
    let mut ret = unsafe {_ZNK26QMessageAuthenticationCode6resultEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMessageAuthenticationCode::addData(const QByteArray & data);
impl /*struct*/ QMessageAuthenticationCode {
  pub fn addData<RetType, T: QMessageAuthenticationCode_addData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addData(self);
    // return 1;
  }
}

pub trait QMessageAuthenticationCode_addData<RetType> {
  fn addData(self , rsthis: & QMessageAuthenticationCode) -> RetType;
}

  // proto:  void QMessageAuthenticationCode::addData(const QByteArray & data);
impl<'a> /*trait*/ QMessageAuthenticationCode_addData<()> for (&'a QByteArray) {
  fn addData(self , rsthis: & QMessageAuthenticationCode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QMessageAuthenticationCode7addDataERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN26QMessageAuthenticationCode7addDataERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMessageAuthenticationCode::QMessageAuthenticationCode(const QMessageAuthenticationCode & );
impl /*struct*/ QMessageAuthenticationCode {
  pub fn New<T: QMessageAuthenticationCode_New>(value: T) -> QMessageAuthenticationCode {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QMessageAuthenticationCode_New {
  fn New(self) -> QMessageAuthenticationCode;
}

  // proto:  void QMessageAuthenticationCode::QMessageAuthenticationCode(const QMessageAuthenticationCode & );
impl<'a> /*trait*/ QMessageAuthenticationCode_New for (&'a QMessageAuthenticationCode) {
  fn New(self) -> QMessageAuthenticationCode {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QMessageAuthenticationCodeC1ERKS_()};
    let ctysz: c_int = unsafe{QMessageAuthenticationCode_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN26QMessageAuthenticationCodeC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN26QMessageAuthenticationCodeC1ERKS_(arg0)} as u64;
    let rsthis = QMessageAuthenticationCode{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QMessageAuthenticationCode::addData(const char * data, int length);
impl<'a> /*trait*/ QMessageAuthenticationCode_addData<()> for (&'a  String, i32) {
  fn addData(self , rsthis: & QMessageAuthenticationCode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QMessageAuthenticationCode7addDataEPKci()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
     unsafe {_ZN26QMessageAuthenticationCode7addDataEPKci(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QMessageAuthenticationCode::~QMessageAuthenticationCode();
impl /*struct*/ QMessageAuthenticationCode {
  pub fn Free<RetType, T: QMessageAuthenticationCode_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QMessageAuthenticationCode_Free<RetType> {
  fn Free(self , rsthis: & QMessageAuthenticationCode) -> RetType;
}

  // proto:  void QMessageAuthenticationCode::~QMessageAuthenticationCode();
impl<'a> /*trait*/ QMessageAuthenticationCode_Free<()> for () {
  fn Free(self , rsthis: & QMessageAuthenticationCode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QMessageAuthenticationCodeD0Ev()};
     unsafe {_ZN26QMessageAuthenticationCodeD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QMessageAuthenticationCode::reset();
impl /*struct*/ QMessageAuthenticationCode {
  pub fn reset<RetType, T: QMessageAuthenticationCode_reset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.reset(self);
    // return 1;
  }
}

pub trait QMessageAuthenticationCode_reset<RetType> {
  fn reset(self , rsthis: & QMessageAuthenticationCode) -> RetType;
}

  // proto:  void QMessageAuthenticationCode::reset();
impl<'a> /*trait*/ QMessageAuthenticationCode_reset<()> for () {
  fn reset(self , rsthis: & QMessageAuthenticationCode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QMessageAuthenticationCode5resetEv()};
     unsafe {_ZN26QMessageAuthenticationCode5resetEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QMessageAuthenticationCode::addData(QIODevice * device);
impl<'a> /*trait*/ QMessageAuthenticationCode_addData<i8> for (&'a QIODevice) {
  fn addData(self , rsthis: & QMessageAuthenticationCode) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QMessageAuthenticationCode7addDataEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN26QMessageAuthenticationCode7addDataEP9QIODevice(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QMessageAuthenticationCode::setKey(const QByteArray & key);
impl /*struct*/ QMessageAuthenticationCode {
  pub fn setKey<RetType, T: QMessageAuthenticationCode_setKey<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setKey(self);
    // return 1;
  }
}

pub trait QMessageAuthenticationCode_setKey<RetType> {
  fn setKey(self , rsthis: & QMessageAuthenticationCode) -> RetType;
}

  // proto:  void QMessageAuthenticationCode::setKey(const QByteArray & key);
impl<'a> /*trait*/ QMessageAuthenticationCode_setKey<()> for (&'a QByteArray) {
  fn setKey(self , rsthis: & QMessageAuthenticationCode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QMessageAuthenticationCode6setKeyERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN26QMessageAuthenticationCode6setKeyERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

