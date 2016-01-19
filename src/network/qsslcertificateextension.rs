// auto generated, do not modify.
// created: Tue Jan 19 21:53:37 2016
// src-file: /QtNetwork/qsslcertificateextension.h
// dst-file: /src/network/qsslcertificateextension.rs
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
use super::super::core::qvariant::QVariant; // 771
use super::super::core::qstring::QString; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSslCertificateExtension_Class_Size() -> c_int;
  // proto:  void QSslCertificateExtension::~QSslCertificateExtension();
  fn _ZN24QSslCertificateExtensionD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QSslCertificateExtension::isSupported();
  fn _ZNK24QSslCertificateExtension11isSupportedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QSslCertificateExtension::QSslCertificateExtension();
  fn _ZN24QSslCertificateExtensionC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QVariant QSslCertificateExtension::value();
  fn _ZNK24QSslCertificateExtension5valueEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QSslCertificateExtension::name();
  fn _ZNK24QSslCertificateExtension4nameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QSslCertificateExtension::oid();
  fn _ZNK24QSslCertificateExtension3oidEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QSslCertificateExtension::isCritical();
  fn _ZNK24QSslCertificateExtension10isCriticalEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QSslCertificateExtension::swap(QSslCertificateExtension & other);
  fn _ZN24QSslCertificateExtension4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QSslCertificateExtension::QSslCertificateExtension(const QSslCertificateExtension & other);
  fn _ZN24QSslCertificateExtensionC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QSslCertificateExtension)=1
#[derive(Default)]
pub struct QSslCertificateExtension {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QSslCertificateExtension {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSslCertificateExtension {
    return QSslCertificateExtension{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QSslCertificateExtension::~QSslCertificateExtension();
impl /*struct*/ QSslCertificateExtension {
  pub fn free<RetType, T: QSslCertificateExtension_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QSslCertificateExtension_free<RetType> {
  fn free(self , rsthis: & QSslCertificateExtension) -> RetType;
}

  // proto:  void QSslCertificateExtension::~QSslCertificateExtension();
impl<'a> /*trait*/ QSslCertificateExtension_free<()> for () {
  fn free(self , rsthis: & QSslCertificateExtension) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QSslCertificateExtensionD2Ev()};
     unsafe {_ZN24QSslCertificateExtensionD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QSslCertificateExtension::isSupported();
impl /*struct*/ QSslCertificateExtension {
  pub fn isSupported<RetType, T: QSslCertificateExtension_isSupported<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSupported(self);
    // return 1;
  }
}

pub trait QSslCertificateExtension_isSupported<RetType> {
  fn isSupported(self , rsthis: & QSslCertificateExtension) -> RetType;
}

  // proto:  bool QSslCertificateExtension::isSupported();
impl<'a> /*trait*/ QSslCertificateExtension_isSupported<i8> for () {
  fn isSupported(self , rsthis: & QSslCertificateExtension) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QSslCertificateExtension11isSupportedEv()};
    let mut ret = unsafe {_ZNK24QSslCertificateExtension11isSupportedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QSslCertificateExtension::QSslCertificateExtension();
impl /*struct*/ QSslCertificateExtension {
  pub fn new<T: QSslCertificateExtension_new>(value: T) -> QSslCertificateExtension {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QSslCertificateExtension_new {
  fn new(self) -> QSslCertificateExtension;
}

  // proto:  void QSslCertificateExtension::QSslCertificateExtension();
impl<'a> /*trait*/ QSslCertificateExtension_new for () {
  fn new(self) -> QSslCertificateExtension {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QSslCertificateExtensionC2Ev()};
    let ctysz: c_int = unsafe{QSslCertificateExtension_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN24QSslCertificateExtensionC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QSslCertificateExtension{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QVariant QSslCertificateExtension::value();
impl /*struct*/ QSslCertificateExtension {
  pub fn value<RetType, T: QSslCertificateExtension_value<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QSslCertificateExtension_value<RetType> {
  fn value(self , rsthis: & QSslCertificateExtension) -> RetType;
}

  // proto:  QVariant QSslCertificateExtension::value();
impl<'a> /*trait*/ QSslCertificateExtension_value<QVariant> for () {
  fn value(self , rsthis: & QSslCertificateExtension) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QSslCertificateExtension5valueEv()};
    let mut ret = unsafe {_ZNK24QSslCertificateExtension5valueEv(rsthis.qclsinst)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QSslCertificateExtension::name();
impl /*struct*/ QSslCertificateExtension {
  pub fn name<RetType, T: QSslCertificateExtension_name<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QSslCertificateExtension_name<RetType> {
  fn name(self , rsthis: & QSslCertificateExtension) -> RetType;
}

  // proto:  QString QSslCertificateExtension::name();
impl<'a> /*trait*/ QSslCertificateExtension_name<QString> for () {
  fn name(self , rsthis: & QSslCertificateExtension) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QSslCertificateExtension4nameEv()};
    let mut ret = unsafe {_ZNK24QSslCertificateExtension4nameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QSslCertificateExtension::oid();
impl /*struct*/ QSslCertificateExtension {
  pub fn oid<RetType, T: QSslCertificateExtension_oid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.oid(self);
    // return 1;
  }
}

pub trait QSslCertificateExtension_oid<RetType> {
  fn oid(self , rsthis: & QSslCertificateExtension) -> RetType;
}

  // proto:  QString QSslCertificateExtension::oid();
impl<'a> /*trait*/ QSslCertificateExtension_oid<QString> for () {
  fn oid(self , rsthis: & QSslCertificateExtension) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QSslCertificateExtension3oidEv()};
    let mut ret = unsafe {_ZNK24QSslCertificateExtension3oidEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QSslCertificateExtension::isCritical();
impl /*struct*/ QSslCertificateExtension {
  pub fn isCritical<RetType, T: QSslCertificateExtension_isCritical<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isCritical(self);
    // return 1;
  }
}

pub trait QSslCertificateExtension_isCritical<RetType> {
  fn isCritical(self , rsthis: & QSslCertificateExtension) -> RetType;
}

  // proto:  bool QSslCertificateExtension::isCritical();
impl<'a> /*trait*/ QSslCertificateExtension_isCritical<i8> for () {
  fn isCritical(self , rsthis: & QSslCertificateExtension) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QSslCertificateExtension10isCriticalEv()};
    let mut ret = unsafe {_ZNK24QSslCertificateExtension10isCriticalEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QSslCertificateExtension::swap(QSslCertificateExtension & other);
impl /*struct*/ QSslCertificateExtension {
  pub fn swap<RetType, T: QSslCertificateExtension_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QSslCertificateExtension_swap<RetType> {
  fn swap(self , rsthis: & QSslCertificateExtension) -> RetType;
}

  // proto:  void QSslCertificateExtension::swap(QSslCertificateExtension & other);
impl<'a> /*trait*/ QSslCertificateExtension_swap<()> for (&'a QSslCertificateExtension) {
  fn swap(self , rsthis: & QSslCertificateExtension) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QSslCertificateExtension4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QSslCertificateExtension4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSslCertificateExtension::QSslCertificateExtension(const QSslCertificateExtension & other);
impl<'a> /*trait*/ QSslCertificateExtension_new for (&'a QSslCertificateExtension) {
  fn new(self) -> QSslCertificateExtension {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QSslCertificateExtensionC2ERKS_()};
    let ctysz: c_int = unsafe{QSslCertificateExtension_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN24QSslCertificateExtensionC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QSslCertificateExtension{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

