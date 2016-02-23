// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtNetwork/qsslkey.h
// dst-file: /src/network/qsslkey.rs
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
use super::super::core::qbytearray::QByteArray; // 771
use super::super::core::qiodevice::QIODevice; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSslKey_Class_Size() -> c_int;
  // proto:  int QSslKey::length();
  fn _ZNK7QSslKey6lengthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QSslKey::isNull();
  fn _ZNK7QSslKey6isNullEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QSslKey::QSslKey();
  fn _ZN7QSslKeyC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QSslKey::QSslKey(const QSslKey & other);
  fn _ZN7QSslKeyC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QByteArray QSslKey::toPem(const QByteArray & passPhrase);
  fn _ZNK7QSslKey5toPemERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QSslKey::clear();
  fn _ZN7QSslKey5clearEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QSslKey::swap(QSslKey & other);
  fn _ZN7QSslKey4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QSslKey::~QSslKey();
  fn _ZN7QSslKeyD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QByteArray QSslKey::toDer(const QByteArray & passPhrase);
  fn _ZNK7QSslKey5toDerERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QSslKey)=1
#[derive(Default)]
pub struct QSslKey {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QSslKey {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSslKey {
    return QSslKey{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  int QSslKey::length();
impl /*struct*/ QSslKey {
  pub fn length<RetType, T: QSslKey_length<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.length(self);
    // return 1;
  }
}

pub trait QSslKey_length<RetType> {
  fn length(self , rsthis: & QSslKey) -> RetType;
}

  // proto:  int QSslKey::length();
impl<'a> /*trait*/ QSslKey_length<i32> for () {
  fn length(self , rsthis: & QSslKey) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QSslKey6lengthEv()};
    let mut ret = unsafe {_ZNK7QSslKey6lengthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QSslKey::isNull();
impl /*struct*/ QSslKey {
  pub fn isNull<RetType, T: QSslKey_isNull<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QSslKey_isNull<RetType> {
  fn isNull(self , rsthis: & QSslKey) -> RetType;
}

  // proto:  bool QSslKey::isNull();
impl<'a> /*trait*/ QSslKey_isNull<i8> for () {
  fn isNull(self , rsthis: & QSslKey) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QSslKey6isNullEv()};
    let mut ret = unsafe {_ZNK7QSslKey6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QSslKey::QSslKey();
impl /*struct*/ QSslKey {
  pub fn new<T: QSslKey_new>(value: T) -> QSslKey {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QSslKey_new {
  fn new(self) -> QSslKey;
}

  // proto:  void QSslKey::QSslKey();
impl<'a> /*trait*/ QSslKey_new for () {
  fn new(self) -> QSslKey {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QSslKeyC2Ev()};
    let ctysz: c_int = unsafe{QSslKey_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN7QSslKeyC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QSslKey{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSslKey::QSslKey(const QSslKey & other);
impl<'a> /*trait*/ QSslKey_new for (&'a QSslKey) {
  fn new(self) -> QSslKey {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QSslKeyC2ERKS_()};
    let ctysz: c_int = unsafe{QSslKey_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QSslKeyC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QSslKey{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QByteArray QSslKey::toPem(const QByteArray & passPhrase);
impl /*struct*/ QSslKey {
  pub fn toPem<RetType, T: QSslKey_toPem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toPem(self);
    // return 1;
  }
}

pub trait QSslKey_toPem<RetType> {
  fn toPem(self , rsthis: & QSslKey) -> RetType;
}

  // proto:  QByteArray QSslKey::toPem(const QByteArray & passPhrase);
impl<'a> /*trait*/ QSslKey_toPem<QByteArray> for (&'a QByteArray) {
  fn toPem(self , rsthis: & QSslKey) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QSslKey5toPemERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QSslKey5toPemERK10QByteArray(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSslKey::clear();
impl /*struct*/ QSslKey {
  pub fn clear<RetType, T: QSslKey_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QSslKey_clear<RetType> {
  fn clear(self , rsthis: & QSslKey) -> RetType;
}

  // proto:  void QSslKey::clear();
impl<'a> /*trait*/ QSslKey_clear<()> for () {
  fn clear(self , rsthis: & QSslKey) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QSslKey5clearEv()};
     unsafe {_ZN7QSslKey5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSslKey::swap(QSslKey & other);
impl /*struct*/ QSslKey {
  pub fn swap<RetType, T: QSslKey_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QSslKey_swap<RetType> {
  fn swap(self , rsthis: & QSslKey) -> RetType;
}

  // proto:  void QSslKey::swap(QSslKey & other);
impl<'a> /*trait*/ QSslKey_swap<()> for (&'a QSslKey) {
  fn swap(self , rsthis: & QSslKey) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QSslKey4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QSslKey4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSslKey::~QSslKey();
impl /*struct*/ QSslKey {
  pub fn free<RetType, T: QSslKey_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QSslKey_free<RetType> {
  fn free(self , rsthis: & QSslKey) -> RetType;
}

  // proto:  void QSslKey::~QSslKey();
impl<'a> /*trait*/ QSslKey_free<()> for () {
  fn free(self , rsthis: & QSslKey) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QSslKeyD2Ev()};
     unsafe {_ZN7QSslKeyD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QByteArray QSslKey::toDer(const QByteArray & passPhrase);
impl /*struct*/ QSslKey {
  pub fn toDer<RetType, T: QSslKey_toDer<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toDer(self);
    // return 1;
  }
}

pub trait QSslKey_toDer<RetType> {
  fn toDer(self , rsthis: & QSslKey) -> RetType;
}

  // proto:  QByteArray QSslKey::toDer(const QByteArray & passPhrase);
impl<'a> /*trait*/ QSslKey_toDer<QByteArray> for (&'a QByteArray) {
  fn toDer(self , rsthis: & QSslKey) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QSslKey5toDerERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QSslKey5toDerERK10QByteArray(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

