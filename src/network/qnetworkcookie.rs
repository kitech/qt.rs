// auto generated, do not modify.
// created: Tue Jan 19 21:53:37 2016
// src-file: /QtNetwork/qnetworkcookie.h
// dst-file: /src/network/qnetworkcookie.rs
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
use super::super::core::qurl::QUrl; // 771
use super::super::core::qstring::QString; // 771
use super::super::core::qdatetime::QDateTime; // 771
use super::super::core::qbytearray::QByteArray; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QNetworkCookie_Class_Size() -> c_int;
  // proto:  void QNetworkCookie::setSecure(bool enable);
  fn _ZN14QNetworkCookie9setSecureEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QNetworkCookie::normalize(const QUrl & url);
  fn _ZN14QNetworkCookie9normalizeERK4QUrl(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QNetworkCookie::domain();
  fn _ZNK14QNetworkCookie6domainEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QNetworkCookie::setExpirationDate(const QDateTime & date);
  fn _ZN14QNetworkCookie17setExpirationDateERK9QDateTime(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QNetworkCookie::QNetworkCookie(const QByteArray & name, const QByteArray & value);
  fn _ZN14QNetworkCookieC2ERK10QByteArrayS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  bool QNetworkCookie::hasSameIdentifier(const QNetworkCookie & other);
  fn _ZNK14QNetworkCookie17hasSameIdentifierERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QString QNetworkCookie::path();
  fn _ZNK14QNetworkCookie4pathEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QByteArray QNetworkCookie::name();
  fn _ZNK14QNetworkCookie4nameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QDateTime QNetworkCookie::expirationDate();
  fn _ZNK14QNetworkCookie14expirationDateEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QNetworkCookie::isSessionCookie();
  fn _ZNK14QNetworkCookie15isSessionCookieEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QNetworkCookie::setName(const QByteArray & cookieName);
  fn _ZN14QNetworkCookie7setNameERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QNetworkCookie::isSecure();
  fn _ZNK14QNetworkCookie8isSecureEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QNetworkCookie::setHttpOnly(bool enable);
  fn _ZN14QNetworkCookie11setHttpOnlyEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QNetworkCookie::setDomain(const QString & domain);
  fn _ZN14QNetworkCookie9setDomainERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QNetworkCookie::isHttpOnly();
  fn _ZNK14QNetworkCookie10isHttpOnlyEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QNetworkCookie::swap(QNetworkCookie & other);
  fn _ZN14QNetworkCookie4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QNetworkCookie::QNetworkCookie(const QNetworkCookie & other);
  fn _ZN14QNetworkCookieC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QNetworkCookie::setValue(const QByteArray & value);
  fn _ZN14QNetworkCookie8setValueERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QByteArray QNetworkCookie::value();
  fn _ZNK14QNetworkCookie5valueEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QNetworkCookie::setPath(const QString & path);
  fn _ZN14QNetworkCookie7setPathERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto: static QList<QNetworkCookie> QNetworkCookie::parseCookies(const QByteArray & cookieString);
  fn _ZN14QNetworkCookie12parseCookiesERK10QByteArray(arg0: *mut c_void);
  // proto:  void QNetworkCookie::~QNetworkCookie();
  fn _ZN14QNetworkCookieD2Ev(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QNetworkCookie)=1
#[derive(Default)]
pub struct QNetworkCookie {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QNetworkCookie {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QNetworkCookie {
    return QNetworkCookie{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QNetworkCookie::setSecure(bool enable);
impl /*struct*/ QNetworkCookie {
  pub fn setSecure<RetType, T: QNetworkCookie_setSecure<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSecure(self);
    // return 1;
  }
}

pub trait QNetworkCookie_setSecure<RetType> {
  fn setSecure(self , rsthis: & QNetworkCookie) -> RetType;
}

  // proto:  void QNetworkCookie::setSecure(bool enable);
impl<'a> /*trait*/ QNetworkCookie_setSecure<()> for (i8) {
  fn setSecure(self , rsthis: & QNetworkCookie) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QNetworkCookie9setSecureEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN14QNetworkCookie9setSecureEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QNetworkCookie::normalize(const QUrl & url);
impl /*struct*/ QNetworkCookie {
  pub fn normalize<RetType, T: QNetworkCookie_normalize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.normalize(self);
    // return 1;
  }
}

pub trait QNetworkCookie_normalize<RetType> {
  fn normalize(self , rsthis: & QNetworkCookie) -> RetType;
}

  // proto:  void QNetworkCookie::normalize(const QUrl & url);
impl<'a> /*trait*/ QNetworkCookie_normalize<()> for (&'a QUrl) {
  fn normalize(self , rsthis: & QNetworkCookie) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QNetworkCookie9normalizeERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QNetworkCookie9normalizeERK4QUrl(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QNetworkCookie::domain();
impl /*struct*/ QNetworkCookie {
  pub fn domain<RetType, T: QNetworkCookie_domain<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.domain(self);
    // return 1;
  }
}

pub trait QNetworkCookie_domain<RetType> {
  fn domain(self , rsthis: & QNetworkCookie) -> RetType;
}

  // proto:  QString QNetworkCookie::domain();
impl<'a> /*trait*/ QNetworkCookie_domain<QString> for () {
  fn domain(self , rsthis: & QNetworkCookie) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QNetworkCookie6domainEv()};
    let mut ret = unsafe {_ZNK14QNetworkCookie6domainEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QNetworkCookie::setExpirationDate(const QDateTime & date);
impl /*struct*/ QNetworkCookie {
  pub fn setExpirationDate<RetType, T: QNetworkCookie_setExpirationDate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setExpirationDate(self);
    // return 1;
  }
}

pub trait QNetworkCookie_setExpirationDate<RetType> {
  fn setExpirationDate(self , rsthis: & QNetworkCookie) -> RetType;
}

  // proto:  void QNetworkCookie::setExpirationDate(const QDateTime & date);
impl<'a> /*trait*/ QNetworkCookie_setExpirationDate<()> for (&'a QDateTime) {
  fn setExpirationDate(self , rsthis: & QNetworkCookie) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QNetworkCookie17setExpirationDateERK9QDateTime()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QNetworkCookie17setExpirationDateERK9QDateTime(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QNetworkCookie::QNetworkCookie(const QByteArray & name, const QByteArray & value);
impl /*struct*/ QNetworkCookie {
  pub fn new<T: QNetworkCookie_new>(value: T) -> QNetworkCookie {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QNetworkCookie_new {
  fn new(self) -> QNetworkCookie;
}

  // proto:  void QNetworkCookie::QNetworkCookie(const QByteArray & name, const QByteArray & value);
impl<'a> /*trait*/ QNetworkCookie_new for (&'a QByteArray, &'a QByteArray) {
  fn new(self) -> QNetworkCookie {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QNetworkCookieC2ERK10QByteArrayS2_()};
    let ctysz: c_int = unsafe{QNetworkCookie_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN14QNetworkCookieC2ERK10QByteArrayS2_(qthis_ph, arg0, arg1)};
    let qthis: u64 = qthis_ph;
    let rsthis = QNetworkCookie{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QNetworkCookie::hasSameIdentifier(const QNetworkCookie & other);
impl /*struct*/ QNetworkCookie {
  pub fn hasSameIdentifier<RetType, T: QNetworkCookie_hasSameIdentifier<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasSameIdentifier(self);
    // return 1;
  }
}

pub trait QNetworkCookie_hasSameIdentifier<RetType> {
  fn hasSameIdentifier(self , rsthis: & QNetworkCookie) -> RetType;
}

  // proto:  bool QNetworkCookie::hasSameIdentifier(const QNetworkCookie & other);
impl<'a> /*trait*/ QNetworkCookie_hasSameIdentifier<i8> for (&'a QNetworkCookie) {
  fn hasSameIdentifier(self , rsthis: & QNetworkCookie) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QNetworkCookie17hasSameIdentifierERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK14QNetworkCookie17hasSameIdentifierERKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QNetworkCookie::path();
impl /*struct*/ QNetworkCookie {
  pub fn path<RetType, T: QNetworkCookie_path<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.path(self);
    // return 1;
  }
}

pub trait QNetworkCookie_path<RetType> {
  fn path(self , rsthis: & QNetworkCookie) -> RetType;
}

  // proto:  QString QNetworkCookie::path();
impl<'a> /*trait*/ QNetworkCookie_path<QString> for () {
  fn path(self , rsthis: & QNetworkCookie) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QNetworkCookie4pathEv()};
    let mut ret = unsafe {_ZNK14QNetworkCookie4pathEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray QNetworkCookie::name();
impl /*struct*/ QNetworkCookie {
  pub fn name<RetType, T: QNetworkCookie_name<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QNetworkCookie_name<RetType> {
  fn name(self , rsthis: & QNetworkCookie) -> RetType;
}

  // proto:  QByteArray QNetworkCookie::name();
impl<'a> /*trait*/ QNetworkCookie_name<QByteArray> for () {
  fn name(self , rsthis: & QNetworkCookie) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QNetworkCookie4nameEv()};
    let mut ret = unsafe {_ZNK14QNetworkCookie4nameEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QDateTime QNetworkCookie::expirationDate();
impl /*struct*/ QNetworkCookie {
  pub fn expirationDate<RetType, T: QNetworkCookie_expirationDate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.expirationDate(self);
    // return 1;
  }
}

pub trait QNetworkCookie_expirationDate<RetType> {
  fn expirationDate(self , rsthis: & QNetworkCookie) -> RetType;
}

  // proto:  QDateTime QNetworkCookie::expirationDate();
impl<'a> /*trait*/ QNetworkCookie_expirationDate<QDateTime> for () {
  fn expirationDate(self , rsthis: & QNetworkCookie) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QNetworkCookie14expirationDateEv()};
    let mut ret = unsafe {_ZNK14QNetworkCookie14expirationDateEv(rsthis.qclsinst)};
    let mut ret1 = QDateTime::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QNetworkCookie::isSessionCookie();
impl /*struct*/ QNetworkCookie {
  pub fn isSessionCookie<RetType, T: QNetworkCookie_isSessionCookie<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSessionCookie(self);
    // return 1;
  }
}

pub trait QNetworkCookie_isSessionCookie<RetType> {
  fn isSessionCookie(self , rsthis: & QNetworkCookie) -> RetType;
}

  // proto:  bool QNetworkCookie::isSessionCookie();
impl<'a> /*trait*/ QNetworkCookie_isSessionCookie<i8> for () {
  fn isSessionCookie(self , rsthis: & QNetworkCookie) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QNetworkCookie15isSessionCookieEv()};
    let mut ret = unsafe {_ZNK14QNetworkCookie15isSessionCookieEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QNetworkCookie::setName(const QByteArray & cookieName);
impl /*struct*/ QNetworkCookie {
  pub fn setName<RetType, T: QNetworkCookie_setName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setName(self);
    // return 1;
  }
}

pub trait QNetworkCookie_setName<RetType> {
  fn setName(self , rsthis: & QNetworkCookie) -> RetType;
}

  // proto:  void QNetworkCookie::setName(const QByteArray & cookieName);
impl<'a> /*trait*/ QNetworkCookie_setName<()> for (&'a QByteArray) {
  fn setName(self , rsthis: & QNetworkCookie) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QNetworkCookie7setNameERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QNetworkCookie7setNameERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QNetworkCookie::isSecure();
impl /*struct*/ QNetworkCookie {
  pub fn isSecure<RetType, T: QNetworkCookie_isSecure<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSecure(self);
    // return 1;
  }
}

pub trait QNetworkCookie_isSecure<RetType> {
  fn isSecure(self , rsthis: & QNetworkCookie) -> RetType;
}

  // proto:  bool QNetworkCookie::isSecure();
impl<'a> /*trait*/ QNetworkCookie_isSecure<i8> for () {
  fn isSecure(self , rsthis: & QNetworkCookie) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QNetworkCookie8isSecureEv()};
    let mut ret = unsafe {_ZNK14QNetworkCookie8isSecureEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QNetworkCookie::setHttpOnly(bool enable);
impl /*struct*/ QNetworkCookie {
  pub fn setHttpOnly<RetType, T: QNetworkCookie_setHttpOnly<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setHttpOnly(self);
    // return 1;
  }
}

pub trait QNetworkCookie_setHttpOnly<RetType> {
  fn setHttpOnly(self , rsthis: & QNetworkCookie) -> RetType;
}

  // proto:  void QNetworkCookie::setHttpOnly(bool enable);
impl<'a> /*trait*/ QNetworkCookie_setHttpOnly<()> for (i8) {
  fn setHttpOnly(self , rsthis: & QNetworkCookie) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QNetworkCookie11setHttpOnlyEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN14QNetworkCookie11setHttpOnlyEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QNetworkCookie::setDomain(const QString & domain);
impl /*struct*/ QNetworkCookie {
  pub fn setDomain<RetType, T: QNetworkCookie_setDomain<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDomain(self);
    // return 1;
  }
}

pub trait QNetworkCookie_setDomain<RetType> {
  fn setDomain(self , rsthis: & QNetworkCookie) -> RetType;
}

  // proto:  void QNetworkCookie::setDomain(const QString & domain);
impl<'a> /*trait*/ QNetworkCookie_setDomain<()> for (&'a QString) {
  fn setDomain(self , rsthis: & QNetworkCookie) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QNetworkCookie9setDomainERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QNetworkCookie9setDomainERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QNetworkCookie::isHttpOnly();
impl /*struct*/ QNetworkCookie {
  pub fn isHttpOnly<RetType, T: QNetworkCookie_isHttpOnly<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isHttpOnly(self);
    // return 1;
  }
}

pub trait QNetworkCookie_isHttpOnly<RetType> {
  fn isHttpOnly(self , rsthis: & QNetworkCookie) -> RetType;
}

  // proto:  bool QNetworkCookie::isHttpOnly();
impl<'a> /*trait*/ QNetworkCookie_isHttpOnly<i8> for () {
  fn isHttpOnly(self , rsthis: & QNetworkCookie) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QNetworkCookie10isHttpOnlyEv()};
    let mut ret = unsafe {_ZNK14QNetworkCookie10isHttpOnlyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QNetworkCookie::swap(QNetworkCookie & other);
impl /*struct*/ QNetworkCookie {
  pub fn swap<RetType, T: QNetworkCookie_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QNetworkCookie_swap<RetType> {
  fn swap(self , rsthis: & QNetworkCookie) -> RetType;
}

  // proto:  void QNetworkCookie::swap(QNetworkCookie & other);
impl<'a> /*trait*/ QNetworkCookie_swap<()> for (&'a QNetworkCookie) {
  fn swap(self , rsthis: & QNetworkCookie) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QNetworkCookie4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QNetworkCookie4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QNetworkCookie::QNetworkCookie(const QNetworkCookie & other);
impl<'a> /*trait*/ QNetworkCookie_new for (&'a QNetworkCookie) {
  fn new(self) -> QNetworkCookie {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QNetworkCookieC2ERKS_()};
    let ctysz: c_int = unsafe{QNetworkCookie_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QNetworkCookieC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QNetworkCookie{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QNetworkCookie::setValue(const QByteArray & value);
impl /*struct*/ QNetworkCookie {
  pub fn setValue<RetType, T: QNetworkCookie_setValue<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setValue(self);
    // return 1;
  }
}

pub trait QNetworkCookie_setValue<RetType> {
  fn setValue(self , rsthis: & QNetworkCookie) -> RetType;
}

  // proto:  void QNetworkCookie::setValue(const QByteArray & value);
impl<'a> /*trait*/ QNetworkCookie_setValue<()> for (&'a QByteArray) {
  fn setValue(self , rsthis: & QNetworkCookie) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QNetworkCookie8setValueERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QNetworkCookie8setValueERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QByteArray QNetworkCookie::value();
impl /*struct*/ QNetworkCookie {
  pub fn value<RetType, T: QNetworkCookie_value<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QNetworkCookie_value<RetType> {
  fn value(self , rsthis: & QNetworkCookie) -> RetType;
}

  // proto:  QByteArray QNetworkCookie::value();
impl<'a> /*trait*/ QNetworkCookie_value<QByteArray> for () {
  fn value(self , rsthis: & QNetworkCookie) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QNetworkCookie5valueEv()};
    let mut ret = unsafe {_ZNK14QNetworkCookie5valueEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QNetworkCookie::setPath(const QString & path);
impl /*struct*/ QNetworkCookie {
  pub fn setPath<RetType, T: QNetworkCookie_setPath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPath(self);
    // return 1;
  }
}

pub trait QNetworkCookie_setPath<RetType> {
  fn setPath(self , rsthis: & QNetworkCookie) -> RetType;
}

  // proto:  void QNetworkCookie::setPath(const QString & path);
impl<'a> /*trait*/ QNetworkCookie_setPath<()> for (&'a QString) {
  fn setPath(self , rsthis: & QNetworkCookie) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QNetworkCookie7setPathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QNetworkCookie7setPathERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static QList<QNetworkCookie> QNetworkCookie::parseCookies(const QByteArray & cookieString);
impl /*struct*/ QNetworkCookie {
  pub fn parseCookies_s<RetType, T: QNetworkCookie_parseCookies_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.parseCookies_s();
    // return 1;
  }
}

pub trait QNetworkCookie_parseCookies_s<RetType> {
  fn parseCookies_s(self ) -> RetType;
}

  // proto: static QList<QNetworkCookie> QNetworkCookie::parseCookies(const QByteArray & cookieString);
impl<'a> /*trait*/ QNetworkCookie_parseCookies_s<()> for (&'a QByteArray) {
  fn parseCookies_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QNetworkCookie12parseCookiesERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QNetworkCookie12parseCookiesERK10QByteArray(arg0)};
    // return 1;
  }
}

  // proto:  void QNetworkCookie::~QNetworkCookie();
impl /*struct*/ QNetworkCookie {
  pub fn free<RetType, T: QNetworkCookie_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QNetworkCookie_free<RetType> {
  fn free(self , rsthis: & QNetworkCookie) -> RetType;
}

  // proto:  void QNetworkCookie::~QNetworkCookie();
impl<'a> /*trait*/ QNetworkCookie_free<()> for () {
  fn free(self , rsthis: & QNetworkCookie) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QNetworkCookieD2Ev()};
     unsafe {_ZN14QNetworkCookieD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

