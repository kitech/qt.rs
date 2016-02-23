// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtNetwork/qsslpresharedkeyauthenticator.h
// dst-file: /src/network/qsslpresharedkeyauthenticator.rs
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
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSslPreSharedKeyAuthenticator_Class_Size() -> c_int;
  // proto:  void QSslPreSharedKeyAuthenticator::QSslPreSharedKeyAuthenticator(const QSslPreSharedKeyAuthenticator & authenticator);
  fn _ZN29QSslPreSharedKeyAuthenticatorC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QSslPreSharedKeyAuthenticator::QSslPreSharedKeyAuthenticator();
  fn _ZN29QSslPreSharedKeyAuthenticatorC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QByteArray QSslPreSharedKeyAuthenticator::identityHint();
  fn _ZNK29QSslPreSharedKeyAuthenticator12identityHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSslPreSharedKeyAuthenticator::swap(QSslPreSharedKeyAuthenticator & authenticator);
  fn _ZN29QSslPreSharedKeyAuthenticator4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QByteArray QSslPreSharedKeyAuthenticator::identity();
  fn _ZNK29QSslPreSharedKeyAuthenticator8identityEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSslPreSharedKeyAuthenticator::setIdentity(const QByteArray & identity);
  fn _ZN29QSslPreSharedKeyAuthenticator11setIdentityERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QSslPreSharedKeyAuthenticator::setPreSharedKey(const QByteArray & preSharedKey);
  fn _ZN29QSslPreSharedKeyAuthenticator15setPreSharedKeyERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QSslPreSharedKeyAuthenticator::maximumPreSharedKeyLength();
  fn _ZNK29QSslPreSharedKeyAuthenticator25maximumPreSharedKeyLengthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QSslPreSharedKeyAuthenticator::maximumIdentityLength();
  fn _ZNK29QSslPreSharedKeyAuthenticator21maximumIdentityLengthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QByteArray QSslPreSharedKeyAuthenticator::preSharedKey();
  fn _ZNK29QSslPreSharedKeyAuthenticator12preSharedKeyEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSslPreSharedKeyAuthenticator::~QSslPreSharedKeyAuthenticator();
  fn _ZN29QSslPreSharedKeyAuthenticatorD2Ev(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QSslPreSharedKeyAuthenticator)=1
#[derive(Default)]
pub struct QSslPreSharedKeyAuthenticator {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QSslPreSharedKeyAuthenticator {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSslPreSharedKeyAuthenticator {
    return QSslPreSharedKeyAuthenticator{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QSslPreSharedKeyAuthenticator::QSslPreSharedKeyAuthenticator(const QSslPreSharedKeyAuthenticator & authenticator);
impl /*struct*/ QSslPreSharedKeyAuthenticator {
  pub fn new<T: QSslPreSharedKeyAuthenticator_new>(value: T) -> QSslPreSharedKeyAuthenticator {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QSslPreSharedKeyAuthenticator_new {
  fn new(self) -> QSslPreSharedKeyAuthenticator;
}

  // proto:  void QSslPreSharedKeyAuthenticator::QSslPreSharedKeyAuthenticator(const QSslPreSharedKeyAuthenticator & authenticator);
impl<'a> /*trait*/ QSslPreSharedKeyAuthenticator_new for (&'a QSslPreSharedKeyAuthenticator) {
  fn new(self) -> QSslPreSharedKeyAuthenticator {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN29QSslPreSharedKeyAuthenticatorC2ERKS_()};
    let ctysz: c_int = unsafe{QSslPreSharedKeyAuthenticator_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN29QSslPreSharedKeyAuthenticatorC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QSslPreSharedKeyAuthenticator{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSslPreSharedKeyAuthenticator::QSslPreSharedKeyAuthenticator();
impl<'a> /*trait*/ QSslPreSharedKeyAuthenticator_new for () {
  fn new(self) -> QSslPreSharedKeyAuthenticator {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN29QSslPreSharedKeyAuthenticatorC2Ev()};
    let ctysz: c_int = unsafe{QSslPreSharedKeyAuthenticator_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN29QSslPreSharedKeyAuthenticatorC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QSslPreSharedKeyAuthenticator{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QByteArray QSslPreSharedKeyAuthenticator::identityHint();
impl /*struct*/ QSslPreSharedKeyAuthenticator {
  pub fn identityHint<RetType, T: QSslPreSharedKeyAuthenticator_identityHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.identityHint(self);
    // return 1;
  }
}

pub trait QSslPreSharedKeyAuthenticator_identityHint<RetType> {
  fn identityHint(self , rsthis: & QSslPreSharedKeyAuthenticator) -> RetType;
}

  // proto:  QByteArray QSslPreSharedKeyAuthenticator::identityHint();
impl<'a> /*trait*/ QSslPreSharedKeyAuthenticator_identityHint<QByteArray> for () {
  fn identityHint(self , rsthis: & QSslPreSharedKeyAuthenticator) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK29QSslPreSharedKeyAuthenticator12identityHintEv()};
    let mut ret = unsafe {_ZNK29QSslPreSharedKeyAuthenticator12identityHintEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSslPreSharedKeyAuthenticator::swap(QSslPreSharedKeyAuthenticator & authenticator);
impl /*struct*/ QSslPreSharedKeyAuthenticator {
  pub fn swap<RetType, T: QSslPreSharedKeyAuthenticator_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QSslPreSharedKeyAuthenticator_swap<RetType> {
  fn swap(self , rsthis: & QSslPreSharedKeyAuthenticator) -> RetType;
}

  // proto:  void QSslPreSharedKeyAuthenticator::swap(QSslPreSharedKeyAuthenticator & authenticator);
impl<'a> /*trait*/ QSslPreSharedKeyAuthenticator_swap<()> for (&'a QSslPreSharedKeyAuthenticator) {
  fn swap(self , rsthis: & QSslPreSharedKeyAuthenticator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN29QSslPreSharedKeyAuthenticator4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN29QSslPreSharedKeyAuthenticator4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QByteArray QSslPreSharedKeyAuthenticator::identity();
impl /*struct*/ QSslPreSharedKeyAuthenticator {
  pub fn identity<RetType, T: QSslPreSharedKeyAuthenticator_identity<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.identity(self);
    // return 1;
  }
}

pub trait QSslPreSharedKeyAuthenticator_identity<RetType> {
  fn identity(self , rsthis: & QSslPreSharedKeyAuthenticator) -> RetType;
}

  // proto:  QByteArray QSslPreSharedKeyAuthenticator::identity();
impl<'a> /*trait*/ QSslPreSharedKeyAuthenticator_identity<QByteArray> for () {
  fn identity(self , rsthis: & QSslPreSharedKeyAuthenticator) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK29QSslPreSharedKeyAuthenticator8identityEv()};
    let mut ret = unsafe {_ZNK29QSslPreSharedKeyAuthenticator8identityEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSslPreSharedKeyAuthenticator::setIdentity(const QByteArray & identity);
impl /*struct*/ QSslPreSharedKeyAuthenticator {
  pub fn setIdentity<RetType, T: QSslPreSharedKeyAuthenticator_setIdentity<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIdentity(self);
    // return 1;
  }
}

pub trait QSslPreSharedKeyAuthenticator_setIdentity<RetType> {
  fn setIdentity(self , rsthis: & QSslPreSharedKeyAuthenticator) -> RetType;
}

  // proto:  void QSslPreSharedKeyAuthenticator::setIdentity(const QByteArray & identity);
impl<'a> /*trait*/ QSslPreSharedKeyAuthenticator_setIdentity<()> for (&'a QByteArray) {
  fn setIdentity(self , rsthis: & QSslPreSharedKeyAuthenticator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN29QSslPreSharedKeyAuthenticator11setIdentityERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN29QSslPreSharedKeyAuthenticator11setIdentityERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSslPreSharedKeyAuthenticator::setPreSharedKey(const QByteArray & preSharedKey);
impl /*struct*/ QSslPreSharedKeyAuthenticator {
  pub fn setPreSharedKey<RetType, T: QSslPreSharedKeyAuthenticator_setPreSharedKey<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPreSharedKey(self);
    // return 1;
  }
}

pub trait QSslPreSharedKeyAuthenticator_setPreSharedKey<RetType> {
  fn setPreSharedKey(self , rsthis: & QSslPreSharedKeyAuthenticator) -> RetType;
}

  // proto:  void QSslPreSharedKeyAuthenticator::setPreSharedKey(const QByteArray & preSharedKey);
impl<'a> /*trait*/ QSslPreSharedKeyAuthenticator_setPreSharedKey<()> for (&'a QByteArray) {
  fn setPreSharedKey(self , rsthis: & QSslPreSharedKeyAuthenticator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN29QSslPreSharedKeyAuthenticator15setPreSharedKeyERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN29QSslPreSharedKeyAuthenticator15setPreSharedKeyERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QSslPreSharedKeyAuthenticator::maximumPreSharedKeyLength();
impl /*struct*/ QSslPreSharedKeyAuthenticator {
  pub fn maximumPreSharedKeyLength<RetType, T: QSslPreSharedKeyAuthenticator_maximumPreSharedKeyLength<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maximumPreSharedKeyLength(self);
    // return 1;
  }
}

pub trait QSslPreSharedKeyAuthenticator_maximumPreSharedKeyLength<RetType> {
  fn maximumPreSharedKeyLength(self , rsthis: & QSslPreSharedKeyAuthenticator) -> RetType;
}

  // proto:  int QSslPreSharedKeyAuthenticator::maximumPreSharedKeyLength();
impl<'a> /*trait*/ QSslPreSharedKeyAuthenticator_maximumPreSharedKeyLength<i32> for () {
  fn maximumPreSharedKeyLength(self , rsthis: & QSslPreSharedKeyAuthenticator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK29QSslPreSharedKeyAuthenticator25maximumPreSharedKeyLengthEv()};
    let mut ret = unsafe {_ZNK29QSslPreSharedKeyAuthenticator25maximumPreSharedKeyLengthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QSslPreSharedKeyAuthenticator::maximumIdentityLength();
impl /*struct*/ QSslPreSharedKeyAuthenticator {
  pub fn maximumIdentityLength<RetType, T: QSslPreSharedKeyAuthenticator_maximumIdentityLength<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maximumIdentityLength(self);
    // return 1;
  }
}

pub trait QSslPreSharedKeyAuthenticator_maximumIdentityLength<RetType> {
  fn maximumIdentityLength(self , rsthis: & QSslPreSharedKeyAuthenticator) -> RetType;
}

  // proto:  int QSslPreSharedKeyAuthenticator::maximumIdentityLength();
impl<'a> /*trait*/ QSslPreSharedKeyAuthenticator_maximumIdentityLength<i32> for () {
  fn maximumIdentityLength(self , rsthis: & QSslPreSharedKeyAuthenticator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK29QSslPreSharedKeyAuthenticator21maximumIdentityLengthEv()};
    let mut ret = unsafe {_ZNK29QSslPreSharedKeyAuthenticator21maximumIdentityLengthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QByteArray QSslPreSharedKeyAuthenticator::preSharedKey();
impl /*struct*/ QSslPreSharedKeyAuthenticator {
  pub fn preSharedKey<RetType, T: QSslPreSharedKeyAuthenticator_preSharedKey<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.preSharedKey(self);
    // return 1;
  }
}

pub trait QSslPreSharedKeyAuthenticator_preSharedKey<RetType> {
  fn preSharedKey(self , rsthis: & QSslPreSharedKeyAuthenticator) -> RetType;
}

  // proto:  QByteArray QSslPreSharedKeyAuthenticator::preSharedKey();
impl<'a> /*trait*/ QSslPreSharedKeyAuthenticator_preSharedKey<QByteArray> for () {
  fn preSharedKey(self , rsthis: & QSslPreSharedKeyAuthenticator) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK29QSslPreSharedKeyAuthenticator12preSharedKeyEv()};
    let mut ret = unsafe {_ZNK29QSslPreSharedKeyAuthenticator12preSharedKeyEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSslPreSharedKeyAuthenticator::~QSslPreSharedKeyAuthenticator();
impl /*struct*/ QSslPreSharedKeyAuthenticator {
  pub fn free<RetType, T: QSslPreSharedKeyAuthenticator_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QSslPreSharedKeyAuthenticator_free<RetType> {
  fn free(self , rsthis: & QSslPreSharedKeyAuthenticator) -> RetType;
}

  // proto:  void QSslPreSharedKeyAuthenticator::~QSslPreSharedKeyAuthenticator();
impl<'a> /*trait*/ QSslPreSharedKeyAuthenticator_free<()> for () {
  fn free(self , rsthis: & QSslPreSharedKeyAuthenticator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN29QSslPreSharedKeyAuthenticatorD2Ev()};
     unsafe {_ZN29QSslPreSharedKeyAuthenticatorD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

