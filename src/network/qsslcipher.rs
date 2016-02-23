// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtNetwork/qsslcipher.h
// dst-file: /src/network/qsslcipher.rs
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
use super::super::core::qstring::QString; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSslCipher_Class_Size() -> c_int;
  // proto:  QString QSslCipher::name();
  fn _ZNK10QSslCipher4nameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSslCipher::~QSslCipher();
  fn _ZN10QSslCipherD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QSslCipher::QSslCipher();
  fn _ZN10QSslCipherC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QSslCipher::QSslCipher(const QString & name);
  fn _ZN10QSslCipherC2ERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QSslCipher::protocolString();
  fn _ZNK10QSslCipher14protocolStringEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QSslCipher::supportedBits();
  fn _ZNK10QSslCipher13supportedBitsEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QSslCipher::usedBits();
  fn _ZNK10QSslCipher8usedBitsEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QSslCipher::swap(QSslCipher & other);
  fn _ZN10QSslCipher4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QSslCipher::keyExchangeMethod();
  fn _ZNK10QSslCipher17keyExchangeMethodEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSslCipher::QSslCipher(const QSslCipher & other);
  fn _ZN10QSslCipherC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QSslCipher::authenticationMethod();
  fn _ZNK10QSslCipher20authenticationMethodEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QSslCipher::isNull();
  fn _ZNK10QSslCipher6isNullEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QString QSslCipher::encryptionMethod();
  fn _ZNK10QSslCipher16encryptionMethodEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QSslCipher)=1
#[derive(Default)]
pub struct QSslCipher {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QSslCipher {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSslCipher {
    return QSslCipher{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QString QSslCipher::name();
impl /*struct*/ QSslCipher {
  pub fn name<RetType, T: QSslCipher_name<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QSslCipher_name<RetType> {
  fn name(self , rsthis: & QSslCipher) -> RetType;
}

  // proto:  QString QSslCipher::name();
impl<'a> /*trait*/ QSslCipher_name<QString> for () {
  fn name(self , rsthis: & QSslCipher) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QSslCipher4nameEv()};
    let mut ret = unsafe {_ZNK10QSslCipher4nameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSslCipher::~QSslCipher();
impl /*struct*/ QSslCipher {
  pub fn free<RetType, T: QSslCipher_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QSslCipher_free<RetType> {
  fn free(self , rsthis: & QSslCipher) -> RetType;
}

  // proto:  void QSslCipher::~QSslCipher();
impl<'a> /*trait*/ QSslCipher_free<()> for () {
  fn free(self , rsthis: & QSslCipher) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSslCipherD2Ev()};
     unsafe {_ZN10QSslCipherD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSslCipher::QSslCipher();
impl /*struct*/ QSslCipher {
  pub fn new<T: QSslCipher_new>(value: T) -> QSslCipher {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QSslCipher_new {
  fn new(self) -> QSslCipher;
}

  // proto:  void QSslCipher::QSslCipher();
impl<'a> /*trait*/ QSslCipher_new for () {
  fn new(self) -> QSslCipher {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSslCipherC2Ev()};
    let ctysz: c_int = unsafe{QSslCipher_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN10QSslCipherC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QSslCipher{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSslCipher::QSslCipher(const QString & name);
impl<'a> /*trait*/ QSslCipher_new for (&'a QString) {
  fn new(self) -> QSslCipher {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSslCipherC2ERK7QString()};
    let ctysz: c_int = unsafe{QSslCipher_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QSslCipherC2ERK7QString(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QSslCipher{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QSslCipher::protocolString();
impl /*struct*/ QSslCipher {
  pub fn protocolString<RetType, T: QSslCipher_protocolString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.protocolString(self);
    // return 1;
  }
}

pub trait QSslCipher_protocolString<RetType> {
  fn protocolString(self , rsthis: & QSslCipher) -> RetType;
}

  // proto:  QString QSslCipher::protocolString();
impl<'a> /*trait*/ QSslCipher_protocolString<QString> for () {
  fn protocolString(self , rsthis: & QSslCipher) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QSslCipher14protocolStringEv()};
    let mut ret = unsafe {_ZNK10QSslCipher14protocolStringEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QSslCipher::supportedBits();
impl /*struct*/ QSslCipher {
  pub fn supportedBits<RetType, T: QSslCipher_supportedBits<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.supportedBits(self);
    // return 1;
  }
}

pub trait QSslCipher_supportedBits<RetType> {
  fn supportedBits(self , rsthis: & QSslCipher) -> RetType;
}

  // proto:  int QSslCipher::supportedBits();
impl<'a> /*trait*/ QSslCipher_supportedBits<i32> for () {
  fn supportedBits(self , rsthis: & QSslCipher) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QSslCipher13supportedBitsEv()};
    let mut ret = unsafe {_ZNK10QSslCipher13supportedBitsEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QSslCipher::usedBits();
impl /*struct*/ QSslCipher {
  pub fn usedBits<RetType, T: QSslCipher_usedBits<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.usedBits(self);
    // return 1;
  }
}

pub trait QSslCipher_usedBits<RetType> {
  fn usedBits(self , rsthis: & QSslCipher) -> RetType;
}

  // proto:  int QSslCipher::usedBits();
impl<'a> /*trait*/ QSslCipher_usedBits<i32> for () {
  fn usedBits(self , rsthis: & QSslCipher) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QSslCipher8usedBitsEv()};
    let mut ret = unsafe {_ZNK10QSslCipher8usedBitsEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QSslCipher::swap(QSslCipher & other);
impl /*struct*/ QSslCipher {
  pub fn swap<RetType, T: QSslCipher_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QSslCipher_swap<RetType> {
  fn swap(self , rsthis: & QSslCipher) -> RetType;
}

  // proto:  void QSslCipher::swap(QSslCipher & other);
impl<'a> /*trait*/ QSslCipher_swap<()> for (&'a QSslCipher) {
  fn swap(self , rsthis: & QSslCipher) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSslCipher4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QSslCipher4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QSslCipher::keyExchangeMethod();
impl /*struct*/ QSslCipher {
  pub fn keyExchangeMethod<RetType, T: QSslCipher_keyExchangeMethod<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.keyExchangeMethod(self);
    // return 1;
  }
}

pub trait QSslCipher_keyExchangeMethod<RetType> {
  fn keyExchangeMethod(self , rsthis: & QSslCipher) -> RetType;
}

  // proto:  QString QSslCipher::keyExchangeMethod();
impl<'a> /*trait*/ QSslCipher_keyExchangeMethod<QString> for () {
  fn keyExchangeMethod(self , rsthis: & QSslCipher) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QSslCipher17keyExchangeMethodEv()};
    let mut ret = unsafe {_ZNK10QSslCipher17keyExchangeMethodEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSslCipher::QSslCipher(const QSslCipher & other);
impl<'a> /*trait*/ QSslCipher_new for (&'a QSslCipher) {
  fn new(self) -> QSslCipher {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSslCipherC2ERKS_()};
    let ctysz: c_int = unsafe{QSslCipher_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QSslCipherC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QSslCipher{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QSslCipher::authenticationMethod();
impl /*struct*/ QSslCipher {
  pub fn authenticationMethod<RetType, T: QSslCipher_authenticationMethod<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.authenticationMethod(self);
    // return 1;
  }
}

pub trait QSslCipher_authenticationMethod<RetType> {
  fn authenticationMethod(self , rsthis: & QSslCipher) -> RetType;
}

  // proto:  QString QSslCipher::authenticationMethod();
impl<'a> /*trait*/ QSslCipher_authenticationMethod<QString> for () {
  fn authenticationMethod(self , rsthis: & QSslCipher) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QSslCipher20authenticationMethodEv()};
    let mut ret = unsafe {_ZNK10QSslCipher20authenticationMethodEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QSslCipher::isNull();
impl /*struct*/ QSslCipher {
  pub fn isNull<RetType, T: QSslCipher_isNull<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QSslCipher_isNull<RetType> {
  fn isNull(self , rsthis: & QSslCipher) -> RetType;
}

  // proto:  bool QSslCipher::isNull();
impl<'a> /*trait*/ QSslCipher_isNull<i8> for () {
  fn isNull(self , rsthis: & QSslCipher) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QSslCipher6isNullEv()};
    let mut ret = unsafe {_ZNK10QSslCipher6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QSslCipher::encryptionMethod();
impl /*struct*/ QSslCipher {
  pub fn encryptionMethod<RetType, T: QSslCipher_encryptionMethod<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.encryptionMethod(self);
    // return 1;
  }
}

pub trait QSslCipher_encryptionMethod<RetType> {
  fn encryptionMethod(self , rsthis: & QSslCipher) -> RetType;
}

  // proto:  QString QSslCipher::encryptionMethod();
impl<'a> /*trait*/ QSslCipher_encryptionMethod<QString> for () {
  fn encryptionMethod(self , rsthis: & QSslCipher) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QSslCipher16encryptionMethodEv()};
    let mut ret = unsafe {_ZNK10QSslCipher16encryptionMethodEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

