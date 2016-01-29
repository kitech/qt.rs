// auto generated, do not modify.
// created: Tue Jan 19 21:53:37 2016
// src-file: /QtNetwork/qsslcertificate.h
// dst-file: /src/network/qsslcertificate.rs
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
use super::qsslkey::QSslKey; // 773
use super::super::core::qdatetime::QDateTime; // 771
use super::super::core::qiodevice::QIODevice; // 771
use super::super::core::qstring::QString; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSslCertificate_Class_Size() -> c_int;
  // proto:  QByteArray QSslCertificate::toPem();
  fn _ZNK15QSslCertificate5toPemEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSslCertificate::swap(QSslCertificate & other);
  fn _ZN15QSslCertificate4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QSslCertificate::~QSslCertificate();
  fn _ZN15QSslCertificateD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QSslKey QSslCertificate::publicKey();
  fn _ZNK15QSslCertificate9publicKeyEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QList<QByteArray> QSslCertificate::issuerInfoAttributes();
  fn _ZNK15QSslCertificate20issuerInfoAttributesEv(qthis: u64 /* *mut c_void*/);
  // proto:  QStringList QSslCertificate::issuerInfo(const QByteArray & attribute);
  fn _ZNK15QSslCertificate10issuerInfoERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QSslCertificate::isNull();
  fn _ZNK15QSslCertificate6isNullEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QByteArray QSslCertificate::toDer();
  fn _ZNK15QSslCertificate5toDerEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSslCertificate::clear();
  fn _ZN15QSslCertificate5clearEv(qthis: u64 /* *mut c_void*/);
  // proto:  QDateTime QSslCertificate::expiryDate();
  fn _ZNK15QSslCertificate10expiryDateEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QByteArray QSslCertificate::version();
  fn _ZNK15QSslCertificate7versionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QSslCertificate::toText();
  fn _ZNK15QSslCertificate6toTextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QStringList QSslCertificate::subjectInfo(const QByteArray & attribute);
  fn _ZNK15QSslCertificate11subjectInfoERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QList<QSslCertificateExtension> QSslCertificate::extensions();
  fn _ZNK15QSslCertificate10extensionsEv(qthis: u64 /* *mut c_void*/);
  // proto:  QByteArray QSslCertificate::serialNumber();
  fn _ZNK15QSslCertificate12serialNumberEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QSslCertificate::isSelfSigned();
  fn _ZNK15QSslCertificate12isSelfSignedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QSslCertificate::QSslCertificate(const QSslCertificate & other);
  fn _ZN15QSslCertificateC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QSslCertificate::isBlacklisted();
  fn _ZNK15QSslCertificate13isBlacklistedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QList<QByteArray> QSslCertificate::subjectInfoAttributes();
  fn _ZNK15QSslCertificate21subjectInfoAttributesEv(qthis: u64 /* *mut c_void*/);
  // proto:  QDateTime QSslCertificate::effectiveDate();
  fn _ZNK15QSslCertificate13effectiveDateEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QSslCertificate)=1
#[derive(Default)]
pub struct QSslCertificate {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QSslCertificate {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSslCertificate {
    return QSslCertificate{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QByteArray QSslCertificate::toPem();
impl /*struct*/ QSslCertificate {
  pub fn toPem<RetType, T: QSslCertificate_toPem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toPem(self);
    // return 1;
  }
}

pub trait QSslCertificate_toPem<RetType> {
  fn toPem(self , rsthis: & QSslCertificate) -> RetType;
}

  // proto:  QByteArray QSslCertificate::toPem();
impl<'a> /*trait*/ QSslCertificate_toPem<QByteArray> for () {
  fn toPem(self , rsthis: & QSslCertificate) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSslCertificate5toPemEv()};
    let mut ret = unsafe {_ZNK15QSslCertificate5toPemEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSslCertificate::swap(QSslCertificate & other);
impl /*struct*/ QSslCertificate {
  pub fn swap<RetType, T: QSslCertificate_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QSslCertificate_swap<RetType> {
  fn swap(self , rsthis: & QSslCertificate) -> RetType;
}

  // proto:  void QSslCertificate::swap(QSslCertificate & other);
impl<'a> /*trait*/ QSslCertificate_swap<()> for (&'a QSslCertificate) {
  fn swap(self , rsthis: & QSslCertificate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSslCertificate4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QSslCertificate4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSslCertificate::~QSslCertificate();
impl /*struct*/ QSslCertificate {
  pub fn free<RetType, T: QSslCertificate_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QSslCertificate_free<RetType> {
  fn free(self , rsthis: & QSslCertificate) -> RetType;
}

  // proto:  void QSslCertificate::~QSslCertificate();
impl<'a> /*trait*/ QSslCertificate_free<()> for () {
  fn free(self , rsthis: & QSslCertificate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSslCertificateD2Ev()};
     unsafe {_ZN15QSslCertificateD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSslKey QSslCertificate::publicKey();
impl /*struct*/ QSslCertificate {
  pub fn publicKey<RetType, T: QSslCertificate_publicKey<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.publicKey(self);
    // return 1;
  }
}

pub trait QSslCertificate_publicKey<RetType> {
  fn publicKey(self , rsthis: & QSslCertificate) -> RetType;
}

  // proto:  QSslKey QSslCertificate::publicKey();
impl<'a> /*trait*/ QSslCertificate_publicKey<QSslKey> for () {
  fn publicKey(self , rsthis: & QSslCertificate) -> QSslKey {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSslCertificate9publicKeyEv()};
    let mut ret = unsafe {_ZNK15QSslCertificate9publicKeyEv(rsthis.qclsinst)};
    let mut ret1 = QSslKey::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QList<QByteArray> QSslCertificate::issuerInfoAttributes();
impl /*struct*/ QSslCertificate {
  pub fn issuerInfoAttributes<RetType, T: QSslCertificate_issuerInfoAttributes<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.issuerInfoAttributes(self);
    // return 1;
  }
}

pub trait QSslCertificate_issuerInfoAttributes<RetType> {
  fn issuerInfoAttributes(self , rsthis: & QSslCertificate) -> RetType;
}

  // proto:  QList<QByteArray> QSslCertificate::issuerInfoAttributes();
impl<'a> /*trait*/ QSslCertificate_issuerInfoAttributes<()> for () {
  fn issuerInfoAttributes(self , rsthis: & QSslCertificate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSslCertificate20issuerInfoAttributesEv()};
     unsafe {_ZNK15QSslCertificate20issuerInfoAttributesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringList QSslCertificate::issuerInfo(const QByteArray & attribute);
impl /*struct*/ QSslCertificate {
  pub fn issuerInfo<RetType, T: QSslCertificate_issuerInfo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.issuerInfo(self);
    // return 1;
  }
}

pub trait QSslCertificate_issuerInfo<RetType> {
  fn issuerInfo(self , rsthis: & QSslCertificate) -> RetType;
}

  // proto:  QStringList QSslCertificate::issuerInfo(const QByteArray & attribute);
impl<'a> /*trait*/ QSslCertificate_issuerInfo<()> for (&'a QByteArray) {
  fn issuerInfo(self , rsthis: & QSslCertificate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSslCertificate10issuerInfoERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK15QSslCertificate10issuerInfoERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QSslCertificate::isNull();
impl /*struct*/ QSslCertificate {
  pub fn isNull<RetType, T: QSslCertificate_isNull<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QSslCertificate_isNull<RetType> {
  fn isNull(self , rsthis: & QSslCertificate) -> RetType;
}

  // proto:  bool QSslCertificate::isNull();
impl<'a> /*trait*/ QSslCertificate_isNull<i8> for () {
  fn isNull(self , rsthis: & QSslCertificate) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSslCertificate6isNullEv()};
    let mut ret = unsafe {_ZNK15QSslCertificate6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QByteArray QSslCertificate::toDer();
impl /*struct*/ QSslCertificate {
  pub fn toDer<RetType, T: QSslCertificate_toDer<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toDer(self);
    // return 1;
  }
}

pub trait QSslCertificate_toDer<RetType> {
  fn toDer(self , rsthis: & QSslCertificate) -> RetType;
}

  // proto:  QByteArray QSslCertificate::toDer();
impl<'a> /*trait*/ QSslCertificate_toDer<QByteArray> for () {
  fn toDer(self , rsthis: & QSslCertificate) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSslCertificate5toDerEv()};
    let mut ret = unsafe {_ZNK15QSslCertificate5toDerEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSslCertificate::clear();
impl /*struct*/ QSslCertificate {
  pub fn clear<RetType, T: QSslCertificate_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QSslCertificate_clear<RetType> {
  fn clear(self , rsthis: & QSslCertificate) -> RetType;
}

  // proto:  void QSslCertificate::clear();
impl<'a> /*trait*/ QSslCertificate_clear<()> for () {
  fn clear(self , rsthis: & QSslCertificate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSslCertificate5clearEv()};
     unsafe {_ZN15QSslCertificate5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QDateTime QSslCertificate::expiryDate();
impl /*struct*/ QSslCertificate {
  pub fn expiryDate<RetType, T: QSslCertificate_expiryDate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.expiryDate(self);
    // return 1;
  }
}

pub trait QSslCertificate_expiryDate<RetType> {
  fn expiryDate(self , rsthis: & QSslCertificate) -> RetType;
}

  // proto:  QDateTime QSslCertificate::expiryDate();
impl<'a> /*trait*/ QSslCertificate_expiryDate<QDateTime> for () {
  fn expiryDate(self , rsthis: & QSslCertificate) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSslCertificate10expiryDateEv()};
    let mut ret = unsafe {_ZNK15QSslCertificate10expiryDateEv(rsthis.qclsinst)};
    let mut ret1 = QDateTime::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray QSslCertificate::version();
impl /*struct*/ QSslCertificate {
  pub fn version<RetType, T: QSslCertificate_version<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.version(self);
    // return 1;
  }
}

pub trait QSslCertificate_version<RetType> {
  fn version(self , rsthis: & QSslCertificate) -> RetType;
}

  // proto:  QByteArray QSslCertificate::version();
impl<'a> /*trait*/ QSslCertificate_version<QByteArray> for () {
  fn version(self , rsthis: & QSslCertificate) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSslCertificate7versionEv()};
    let mut ret = unsafe {_ZNK15QSslCertificate7versionEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QSslCertificate::toText();
impl /*struct*/ QSslCertificate {
  pub fn toText<RetType, T: QSslCertificate_toText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toText(self);
    // return 1;
  }
}

pub trait QSslCertificate_toText<RetType> {
  fn toText(self , rsthis: & QSslCertificate) -> RetType;
}

  // proto:  QString QSslCertificate::toText();
impl<'a> /*trait*/ QSslCertificate_toText<QString> for () {
  fn toText(self , rsthis: & QSslCertificate) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSslCertificate6toTextEv()};
    let mut ret = unsafe {_ZNK15QSslCertificate6toTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QStringList QSslCertificate::subjectInfo(const QByteArray & attribute);
impl /*struct*/ QSslCertificate {
  pub fn subjectInfo<RetType, T: QSslCertificate_subjectInfo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.subjectInfo(self);
    // return 1;
  }
}

pub trait QSslCertificate_subjectInfo<RetType> {
  fn subjectInfo(self , rsthis: & QSslCertificate) -> RetType;
}

  // proto:  QStringList QSslCertificate::subjectInfo(const QByteArray & attribute);
impl<'a> /*trait*/ QSslCertificate_subjectInfo<()> for (&'a QByteArray) {
  fn subjectInfo(self , rsthis: & QSslCertificate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSslCertificate11subjectInfoERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK15QSslCertificate11subjectInfoERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QList<QSslCertificateExtension> QSslCertificate::extensions();
impl /*struct*/ QSslCertificate {
  pub fn extensions<RetType, T: QSslCertificate_extensions<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.extensions(self);
    // return 1;
  }
}

pub trait QSslCertificate_extensions<RetType> {
  fn extensions(self , rsthis: & QSslCertificate) -> RetType;
}

  // proto:  QList<QSslCertificateExtension> QSslCertificate::extensions();
impl<'a> /*trait*/ QSslCertificate_extensions<()> for () {
  fn extensions(self , rsthis: & QSslCertificate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSslCertificate10extensionsEv()};
     unsafe {_ZNK15QSslCertificate10extensionsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QByteArray QSslCertificate::serialNumber();
impl /*struct*/ QSslCertificate {
  pub fn serialNumber<RetType, T: QSslCertificate_serialNumber<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.serialNumber(self);
    // return 1;
  }
}

pub trait QSslCertificate_serialNumber<RetType> {
  fn serialNumber(self , rsthis: & QSslCertificate) -> RetType;
}

  // proto:  QByteArray QSslCertificate::serialNumber();
impl<'a> /*trait*/ QSslCertificate_serialNumber<QByteArray> for () {
  fn serialNumber(self , rsthis: & QSslCertificate) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSslCertificate12serialNumberEv()};
    let mut ret = unsafe {_ZNK15QSslCertificate12serialNumberEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QSslCertificate::isSelfSigned();
impl /*struct*/ QSslCertificate {
  pub fn isSelfSigned<RetType, T: QSslCertificate_isSelfSigned<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSelfSigned(self);
    // return 1;
  }
}

pub trait QSslCertificate_isSelfSigned<RetType> {
  fn isSelfSigned(self , rsthis: & QSslCertificate) -> RetType;
}

  // proto:  bool QSslCertificate::isSelfSigned();
impl<'a> /*trait*/ QSslCertificate_isSelfSigned<i8> for () {
  fn isSelfSigned(self , rsthis: & QSslCertificate) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSslCertificate12isSelfSignedEv()};
    let mut ret = unsafe {_ZNK15QSslCertificate12isSelfSignedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QSslCertificate::QSslCertificate(const QSslCertificate & other);
impl /*struct*/ QSslCertificate {
  pub fn new<T: QSslCertificate_new>(value: T) -> QSslCertificate {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QSslCertificate_new {
  fn new(self) -> QSslCertificate;
}

  // proto:  void QSslCertificate::QSslCertificate(const QSslCertificate & other);
impl<'a> /*trait*/ QSslCertificate_new for (&'a QSslCertificate) {
  fn new(self) -> QSslCertificate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSslCertificateC2ERKS_()};
    let ctysz: c_int = unsafe{QSslCertificate_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QSslCertificateC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QSslCertificate{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QSslCertificate::isBlacklisted();
impl /*struct*/ QSslCertificate {
  pub fn isBlacklisted<RetType, T: QSslCertificate_isBlacklisted<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isBlacklisted(self);
    // return 1;
  }
}

pub trait QSslCertificate_isBlacklisted<RetType> {
  fn isBlacklisted(self , rsthis: & QSslCertificate) -> RetType;
}

  // proto:  bool QSslCertificate::isBlacklisted();
impl<'a> /*trait*/ QSslCertificate_isBlacklisted<i8> for () {
  fn isBlacklisted(self , rsthis: & QSslCertificate) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSslCertificate13isBlacklistedEv()};
    let mut ret = unsafe {_ZNK15QSslCertificate13isBlacklistedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QList<QByteArray> QSslCertificate::subjectInfoAttributes();
impl /*struct*/ QSslCertificate {
  pub fn subjectInfoAttributes<RetType, T: QSslCertificate_subjectInfoAttributes<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.subjectInfoAttributes(self);
    // return 1;
  }
}

pub trait QSslCertificate_subjectInfoAttributes<RetType> {
  fn subjectInfoAttributes(self , rsthis: & QSslCertificate) -> RetType;
}

  // proto:  QList<QByteArray> QSslCertificate::subjectInfoAttributes();
impl<'a> /*trait*/ QSslCertificate_subjectInfoAttributes<()> for () {
  fn subjectInfoAttributes(self , rsthis: & QSslCertificate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSslCertificate21subjectInfoAttributesEv()};
     unsafe {_ZNK15QSslCertificate21subjectInfoAttributesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QDateTime QSslCertificate::effectiveDate();
impl /*struct*/ QSslCertificate {
  pub fn effectiveDate<RetType, T: QSslCertificate_effectiveDate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.effectiveDate(self);
    // return 1;
  }
}

pub trait QSslCertificate_effectiveDate<RetType> {
  fn effectiveDate(self , rsthis: & QSslCertificate) -> RetType;
}

  // proto:  QDateTime QSslCertificate::effectiveDate();
impl<'a> /*trait*/ QSslCertificate_effectiveDate<QDateTime> for () {
  fn effectiveDate(self , rsthis: & QSslCertificate) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSslCertificate13effectiveDateEv()};
    let mut ret = unsafe {_ZNK15QSslCertificate13effectiveDateEv(rsthis.qclsinst)};
    let mut ret1 = QDateTime::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

