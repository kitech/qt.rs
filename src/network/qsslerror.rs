// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtNetwork/qsslerror.h
// dst-file: /src/network/qsslerror.rs
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
use super::qsslcertificate::QSslCertificate; // 773
use super::super::core::qstring::QString; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSslError_Class_Size() -> c_int;
  // proto:  void QSslError::QSslError();
  fn _ZN9QSslErrorC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QSslError::swap(QSslError & other);
  fn _ZN9QSslError4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QSslError::QSslError(const QSslError & other);
  fn _ZN9QSslErrorC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QSslCertificate QSslError::certificate();
  fn _ZNK9QSslError11certificateEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QSslError::errorString();
  fn _ZNK9QSslError11errorStringEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSslError::~QSslError();
  fn _ZN9QSslErrorD2Ev(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QSslError)=1
#[derive(Default)]
pub struct QSslError {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QSslError {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSslError {
    return QSslError{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QSslError::QSslError();
impl /*struct*/ QSslError {
  pub fn new<T: QSslError_new>(value: T) -> QSslError {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QSslError_new {
  fn new(self) -> QSslError;
}

  // proto:  void QSslError::QSslError();
impl<'a> /*trait*/ QSslError_new for () {
  fn new(self) -> QSslError {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSslErrorC2Ev()};
    let ctysz: c_int = unsafe{QSslError_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN9QSslErrorC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QSslError{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSslError::swap(QSslError & other);
impl /*struct*/ QSslError {
  pub fn swap<RetType, T: QSslError_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QSslError_swap<RetType> {
  fn swap(self , rsthis: & QSslError) -> RetType;
}

  // proto:  void QSslError::swap(QSslError & other);
impl<'a> /*trait*/ QSslError_swap<()> for (&'a QSslError) {
  fn swap(self , rsthis: & QSslError) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSslError4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QSslError4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSslError::QSslError(const QSslError & other);
impl<'a> /*trait*/ QSslError_new for (&'a QSslError) {
  fn new(self) -> QSslError {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSslErrorC2ERKS_()};
    let ctysz: c_int = unsafe{QSslError_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QSslErrorC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QSslError{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QSslCertificate QSslError::certificate();
impl /*struct*/ QSslError {
  pub fn certificate<RetType, T: QSslError_certificate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.certificate(self);
    // return 1;
  }
}

pub trait QSslError_certificate<RetType> {
  fn certificate(self , rsthis: & QSslError) -> RetType;
}

  // proto:  QSslCertificate QSslError::certificate();
impl<'a> /*trait*/ QSslError_certificate<QSslCertificate> for () {
  fn certificate(self , rsthis: & QSslError) -> QSslCertificate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSslError11certificateEv()};
    let mut ret = unsafe {_ZNK9QSslError11certificateEv(rsthis.qclsinst)};
    let mut ret1 = QSslCertificate::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QSslError::errorString();
impl /*struct*/ QSslError {
  pub fn errorString<RetType, T: QSslError_errorString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.errorString(self);
    // return 1;
  }
}

pub trait QSslError_errorString<RetType> {
  fn errorString(self , rsthis: & QSslError) -> RetType;
}

  // proto:  QString QSslError::errorString();
impl<'a> /*trait*/ QSslError_errorString<QString> for () {
  fn errorString(self , rsthis: & QSslError) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSslError11errorStringEv()};
    let mut ret = unsafe {_ZNK9QSslError11errorStringEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSslError::~QSslError();
impl /*struct*/ QSslError {
  pub fn free<RetType, T: QSslError_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QSslError_free<RetType> {
  fn free(self , rsthis: & QSslError) -> RetType;
}

  // proto:  void QSslError::~QSslError();
impl<'a> /*trait*/ QSslError_free<()> for () {
  fn free(self , rsthis: & QSslError) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSslErrorD2Ev()};
     unsafe {_ZN9QSslErrorD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

