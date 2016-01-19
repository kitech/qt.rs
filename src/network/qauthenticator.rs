// auto generated, do not modify.
// created: Tue Jan 19 21:53:37 2016
// src-file: /QtNetwork/qauthenticator.h
// dst-file: /src/network/qauthenticator.rs
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
use super::super::core::qvariant::QVariant; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QAuthenticator_Class_Size() -> c_int;
  // proto:  void QAuthenticator::~QAuthenticator();
  fn _ZN14QAuthenticatorD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QAuthenticator::setRealm(const QString & realm);
  fn _ZN14QAuthenticator8setRealmERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QAuthenticator::password();
  fn _ZNK14QAuthenticator8passwordEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QVariantHash QAuthenticator::options();
  fn _ZNK14QAuthenticator7optionsEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QAuthenticator::QAuthenticator();
  fn _ZN14QAuthenticatorC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QString QAuthenticator::realm();
  fn _ZNK14QAuthenticator5realmEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QAuthenticator::setOption(const QString & opt, const QVariant & value);
  fn _ZN14QAuthenticator9setOptionERK7QStringRK8QVariant(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QString QAuthenticator::user();
  fn _ZNK14QAuthenticator4userEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QAuthenticator::QAuthenticator(const QAuthenticator & other);
  fn _ZN14QAuthenticatorC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QVariant QAuthenticator::option(const QString & opt);
  fn _ZNK14QAuthenticator6optionERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QAuthenticator::setPassword(const QString & password);
  fn _ZN14QAuthenticator11setPasswordERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QAuthenticator::setUser(const QString & user);
  fn _ZN14QAuthenticator7setUserERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QAuthenticator::detach();
  fn _ZN14QAuthenticator6detachEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QAuthenticator::isNull();
  fn _ZNK14QAuthenticator6isNullEv(qthis: u64 /* *mut c_void*/) -> c_char;
} // <= ext block end

// body block begin =>
// class sizeof(QAuthenticator)=8
#[derive(Default)]
pub struct QAuthenticator {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QAuthenticator {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QAuthenticator {
    return QAuthenticator{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QAuthenticator::~QAuthenticator();
impl /*struct*/ QAuthenticator {
  pub fn free<RetType, T: QAuthenticator_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QAuthenticator_free<RetType> {
  fn free(self , rsthis: & QAuthenticator) -> RetType;
}

  // proto:  void QAuthenticator::~QAuthenticator();
impl<'a> /*trait*/ QAuthenticator_free<()> for () {
  fn free(self , rsthis: & QAuthenticator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QAuthenticatorD2Ev()};
     unsafe {_ZN14QAuthenticatorD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAuthenticator::setRealm(const QString & realm);
impl /*struct*/ QAuthenticator {
  pub fn setRealm<RetType, T: QAuthenticator_setRealm<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRealm(self);
    // return 1;
  }
}

pub trait QAuthenticator_setRealm<RetType> {
  fn setRealm(self , rsthis: & QAuthenticator) -> RetType;
}

  // proto:  void QAuthenticator::setRealm(const QString & realm);
impl<'a> /*trait*/ QAuthenticator_setRealm<()> for (&'a QString) {
  fn setRealm(self , rsthis: & QAuthenticator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QAuthenticator8setRealmERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QAuthenticator8setRealmERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QAuthenticator::password();
impl /*struct*/ QAuthenticator {
  pub fn password<RetType, T: QAuthenticator_password<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.password(self);
    // return 1;
  }
}

pub trait QAuthenticator_password<RetType> {
  fn password(self , rsthis: & QAuthenticator) -> RetType;
}

  // proto:  QString QAuthenticator::password();
impl<'a> /*trait*/ QAuthenticator_password<QString> for () {
  fn password(self , rsthis: & QAuthenticator) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QAuthenticator8passwordEv()};
    let mut ret = unsafe {_ZNK14QAuthenticator8passwordEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QVariantHash QAuthenticator::options();
impl /*struct*/ QAuthenticator {
  pub fn options<RetType, T: QAuthenticator_options<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.options(self);
    // return 1;
  }
}

pub trait QAuthenticator_options<RetType> {
  fn options(self , rsthis: & QAuthenticator) -> RetType;
}

  // proto:  QVariantHash QAuthenticator::options();
impl<'a> /*trait*/ QAuthenticator_options<()> for () {
  fn options(self , rsthis: & QAuthenticator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QAuthenticator7optionsEv()};
     unsafe {_ZNK14QAuthenticator7optionsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAuthenticator::QAuthenticator();
impl /*struct*/ QAuthenticator {
  pub fn new<T: QAuthenticator_new>(value: T) -> QAuthenticator {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QAuthenticator_new {
  fn new(self) -> QAuthenticator;
}

  // proto:  void QAuthenticator::QAuthenticator();
impl<'a> /*trait*/ QAuthenticator_new for () {
  fn new(self) -> QAuthenticator {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QAuthenticatorC2Ev()};
    let ctysz: c_int = unsafe{QAuthenticator_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN14QAuthenticatorC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QAuthenticator{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QAuthenticator::realm();
impl /*struct*/ QAuthenticator {
  pub fn realm<RetType, T: QAuthenticator_realm<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.realm(self);
    // return 1;
  }
}

pub trait QAuthenticator_realm<RetType> {
  fn realm(self , rsthis: & QAuthenticator) -> RetType;
}

  // proto:  QString QAuthenticator::realm();
impl<'a> /*trait*/ QAuthenticator_realm<QString> for () {
  fn realm(self , rsthis: & QAuthenticator) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QAuthenticator5realmEv()};
    let mut ret = unsafe {_ZNK14QAuthenticator5realmEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAuthenticator::setOption(const QString & opt, const QVariant & value);
impl /*struct*/ QAuthenticator {
  pub fn setOption<RetType, T: QAuthenticator_setOption<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setOption(self);
    // return 1;
  }
}

pub trait QAuthenticator_setOption<RetType> {
  fn setOption(self , rsthis: & QAuthenticator) -> RetType;
}

  // proto:  void QAuthenticator::setOption(const QString & opt, const QVariant & value);
impl<'a> /*trait*/ QAuthenticator_setOption<()> for (&'a QString, &'a QVariant) {
  fn setOption(self , rsthis: & QAuthenticator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QAuthenticator9setOptionERK7QStringRK8QVariant()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN14QAuthenticator9setOptionERK7QStringRK8QVariant(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QString QAuthenticator::user();
impl /*struct*/ QAuthenticator {
  pub fn user<RetType, T: QAuthenticator_user<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.user(self);
    // return 1;
  }
}

pub trait QAuthenticator_user<RetType> {
  fn user(self , rsthis: & QAuthenticator) -> RetType;
}

  // proto:  QString QAuthenticator::user();
impl<'a> /*trait*/ QAuthenticator_user<QString> for () {
  fn user(self , rsthis: & QAuthenticator) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QAuthenticator4userEv()};
    let mut ret = unsafe {_ZNK14QAuthenticator4userEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAuthenticator::QAuthenticator(const QAuthenticator & other);
impl<'a> /*trait*/ QAuthenticator_new for (&'a QAuthenticator) {
  fn new(self) -> QAuthenticator {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QAuthenticatorC2ERKS_()};
    let ctysz: c_int = unsafe{QAuthenticator_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QAuthenticatorC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QAuthenticator{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QVariant QAuthenticator::option(const QString & opt);
impl /*struct*/ QAuthenticator {
  pub fn option<RetType, T: QAuthenticator_option<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.option(self);
    // return 1;
  }
}

pub trait QAuthenticator_option<RetType> {
  fn option(self , rsthis: & QAuthenticator) -> RetType;
}

  // proto:  QVariant QAuthenticator::option(const QString & opt);
impl<'a> /*trait*/ QAuthenticator_option<QVariant> for (&'a QString) {
  fn option(self , rsthis: & QAuthenticator) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QAuthenticator6optionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK14QAuthenticator6optionERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAuthenticator::setPassword(const QString & password);
impl /*struct*/ QAuthenticator {
  pub fn setPassword<RetType, T: QAuthenticator_setPassword<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPassword(self);
    // return 1;
  }
}

pub trait QAuthenticator_setPassword<RetType> {
  fn setPassword(self , rsthis: & QAuthenticator) -> RetType;
}

  // proto:  void QAuthenticator::setPassword(const QString & password);
impl<'a> /*trait*/ QAuthenticator_setPassword<()> for (&'a QString) {
  fn setPassword(self , rsthis: & QAuthenticator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QAuthenticator11setPasswordERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QAuthenticator11setPasswordERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAuthenticator::setUser(const QString & user);
impl /*struct*/ QAuthenticator {
  pub fn setUser<RetType, T: QAuthenticator_setUser<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setUser(self);
    // return 1;
  }
}

pub trait QAuthenticator_setUser<RetType> {
  fn setUser(self , rsthis: & QAuthenticator) -> RetType;
}

  // proto:  void QAuthenticator::setUser(const QString & user);
impl<'a> /*trait*/ QAuthenticator_setUser<()> for (&'a QString) {
  fn setUser(self , rsthis: & QAuthenticator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QAuthenticator7setUserERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QAuthenticator7setUserERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAuthenticator::detach();
impl /*struct*/ QAuthenticator {
  pub fn detach<RetType, T: QAuthenticator_detach<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.detach(self);
    // return 1;
  }
}

pub trait QAuthenticator_detach<RetType> {
  fn detach(self , rsthis: & QAuthenticator) -> RetType;
}

  // proto:  void QAuthenticator::detach();
impl<'a> /*trait*/ QAuthenticator_detach<()> for () {
  fn detach(self , rsthis: & QAuthenticator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QAuthenticator6detachEv()};
     unsafe {_ZN14QAuthenticator6detachEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QAuthenticator::isNull();
impl /*struct*/ QAuthenticator {
  pub fn isNull<RetType, T: QAuthenticator_isNull<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QAuthenticator_isNull<RetType> {
  fn isNull(self , rsthis: & QAuthenticator) -> RetType;
}

  // proto:  bool QAuthenticator::isNull();
impl<'a> /*trait*/ QAuthenticator_isNull<i8> for () {
  fn isNull(self , rsthis: & QAuthenticator) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QAuthenticator6isNullEv()};
    let mut ret = unsafe {_ZNK14QAuthenticator6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// <= body block end

