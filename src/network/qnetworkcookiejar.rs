// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtNetwork/qnetworkcookiejar.h
// dst-file: /src/network/qnetworkcookiejar.rs
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
use super::super::core::qobject::QObject; // 771
use std::ops::Deref;
use super::super::core::qurl::QUrl; // 771
use super::qnetworkcookie::QNetworkCookie; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QNetworkCookieJar_Class_Size() -> c_int;
  // proto:  const QMetaObject * QNetworkCookieJar::metaObject();
  fn _ZNK17QNetworkCookieJar10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QNetworkCookieJar::QNetworkCookieJar(const QNetworkCookieJar & );
  fn _ZN17QNetworkCookieJarC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QNetworkCookieJar::QNetworkCookieJar(QObject * parent);
  fn _ZN17QNetworkCookieJarC2EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QNetworkCookieJar::insertCookie(const QNetworkCookie & cookie);
  fn _ZN17QNetworkCookieJar12insertCookieERK14QNetworkCookie(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QList<QNetworkCookie> QNetworkCookieJar::cookiesForUrl(const QUrl & url);
  fn _ZNK17QNetworkCookieJar13cookiesForUrlERK4QUrl(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QNetworkCookieJar::~QNetworkCookieJar();
  fn _ZN17QNetworkCookieJarD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QNetworkCookieJar::deleteCookie(const QNetworkCookie & cookie);
  fn _ZN17QNetworkCookieJar12deleteCookieERK14QNetworkCookie(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  bool QNetworkCookieJar::updateCookie(const QNetworkCookie & cookie);
  fn _ZN17QNetworkCookieJar12updateCookieERK14QNetworkCookie(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
} // <= ext block end

// body block begin =>
// class sizeof(QNetworkCookieJar)=1
#[derive(Default)]
pub struct QNetworkCookieJar {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QNetworkCookieJar {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QNetworkCookieJar {
    return QNetworkCookieJar{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QNetworkCookieJar {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QNetworkCookieJar {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  const QMetaObject * QNetworkCookieJar::metaObject();
impl /*struct*/ QNetworkCookieJar {
  pub fn metaObject<RetType, T: QNetworkCookieJar_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QNetworkCookieJar_metaObject<RetType> {
  fn metaObject(self , rsthis: & QNetworkCookieJar) -> RetType;
}

  // proto:  const QMetaObject * QNetworkCookieJar::metaObject();
impl<'a> /*trait*/ QNetworkCookieJar_metaObject<()> for () {
  fn metaObject(self , rsthis: & QNetworkCookieJar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QNetworkCookieJar10metaObjectEv()};
     unsafe {_ZNK17QNetworkCookieJar10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QNetworkCookieJar::QNetworkCookieJar(const QNetworkCookieJar & );
impl /*struct*/ QNetworkCookieJar {
  pub fn new<T: QNetworkCookieJar_new>(value: T) -> QNetworkCookieJar {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QNetworkCookieJar_new {
  fn new(self) -> QNetworkCookieJar;
}

  // proto:  void QNetworkCookieJar::QNetworkCookieJar(const QNetworkCookieJar & );
impl<'a> /*trait*/ QNetworkCookieJar_new for (&'a QNetworkCookieJar) {
  fn new(self) -> QNetworkCookieJar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QNetworkCookieJarC2ERKS_()};
    let ctysz: c_int = unsafe{QNetworkCookieJar_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QNetworkCookieJarC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QNetworkCookieJar{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QNetworkCookieJar::QNetworkCookieJar(QObject * parent);
impl<'a> /*trait*/ QNetworkCookieJar_new for (&'a QObject) {
  fn new(self) -> QNetworkCookieJar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QNetworkCookieJarC2EP7QObject()};
    let ctysz: c_int = unsafe{QNetworkCookieJar_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QNetworkCookieJarC2EP7QObject(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QNetworkCookieJar{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QNetworkCookieJar::insertCookie(const QNetworkCookie & cookie);
impl /*struct*/ QNetworkCookieJar {
  pub fn insertCookie<RetType, T: QNetworkCookieJar_insertCookie<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertCookie(self);
    // return 1;
  }
}

pub trait QNetworkCookieJar_insertCookie<RetType> {
  fn insertCookie(self , rsthis: & QNetworkCookieJar) -> RetType;
}

  // proto:  bool QNetworkCookieJar::insertCookie(const QNetworkCookie & cookie);
impl<'a> /*trait*/ QNetworkCookieJar_insertCookie<i8> for (&'a QNetworkCookie) {
  fn insertCookie(self , rsthis: & QNetworkCookieJar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QNetworkCookieJar12insertCookieERK14QNetworkCookie()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN17QNetworkCookieJar12insertCookieERK14QNetworkCookie(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QList<QNetworkCookie> QNetworkCookieJar::cookiesForUrl(const QUrl & url);
impl /*struct*/ QNetworkCookieJar {
  pub fn cookiesForUrl<RetType, T: QNetworkCookieJar_cookiesForUrl<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cookiesForUrl(self);
    // return 1;
  }
}

pub trait QNetworkCookieJar_cookiesForUrl<RetType> {
  fn cookiesForUrl(self , rsthis: & QNetworkCookieJar) -> RetType;
}

  // proto:  QList<QNetworkCookie> QNetworkCookieJar::cookiesForUrl(const QUrl & url);
impl<'a> /*trait*/ QNetworkCookieJar_cookiesForUrl<()> for (&'a QUrl) {
  fn cookiesForUrl(self , rsthis: & QNetworkCookieJar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QNetworkCookieJar13cookiesForUrlERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK17QNetworkCookieJar13cookiesForUrlERK4QUrl(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QNetworkCookieJar::~QNetworkCookieJar();
impl /*struct*/ QNetworkCookieJar {
  pub fn free<RetType, T: QNetworkCookieJar_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QNetworkCookieJar_free<RetType> {
  fn free(self , rsthis: & QNetworkCookieJar) -> RetType;
}

  // proto:  void QNetworkCookieJar::~QNetworkCookieJar();
impl<'a> /*trait*/ QNetworkCookieJar_free<()> for () {
  fn free(self , rsthis: & QNetworkCookieJar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QNetworkCookieJarD2Ev()};
     unsafe {_ZN17QNetworkCookieJarD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QNetworkCookieJar::deleteCookie(const QNetworkCookie & cookie);
impl /*struct*/ QNetworkCookieJar {
  pub fn deleteCookie<RetType, T: QNetworkCookieJar_deleteCookie<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.deleteCookie(self);
    // return 1;
  }
}

pub trait QNetworkCookieJar_deleteCookie<RetType> {
  fn deleteCookie(self , rsthis: & QNetworkCookieJar) -> RetType;
}

  // proto:  bool QNetworkCookieJar::deleteCookie(const QNetworkCookie & cookie);
impl<'a> /*trait*/ QNetworkCookieJar_deleteCookie<i8> for (&'a QNetworkCookie) {
  fn deleteCookie(self , rsthis: & QNetworkCookieJar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QNetworkCookieJar12deleteCookieERK14QNetworkCookie()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN17QNetworkCookieJar12deleteCookieERK14QNetworkCookie(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QNetworkCookieJar::updateCookie(const QNetworkCookie & cookie);
impl /*struct*/ QNetworkCookieJar {
  pub fn updateCookie<RetType, T: QNetworkCookieJar_updateCookie<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.updateCookie(self);
    // return 1;
  }
}

pub trait QNetworkCookieJar_updateCookie<RetType> {
  fn updateCookie(self , rsthis: & QNetworkCookieJar) -> RetType;
}

  // proto:  bool QNetworkCookieJar::updateCookie(const QNetworkCookie & cookie);
impl<'a> /*trait*/ QNetworkCookieJar_updateCookie<i8> for (&'a QNetworkCookie) {
  fn updateCookie(self , rsthis: & QNetworkCookieJar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QNetworkCookieJar12updateCookieERK14QNetworkCookie()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN17QNetworkCookieJar12updateCookieERK14QNetworkCookie(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// <= body block end

