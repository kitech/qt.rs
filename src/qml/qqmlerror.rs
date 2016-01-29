// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQml/qqmlerror.h
// dst-file: /src/qml/qqmlerror.rs
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
use super::super::core::qurl::QUrl; // 771
use super::super::core::qobject::QObject; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QQmlError_Class_Size() -> c_int;
  // proto:  QString QQmlError::description();
  fn _ZNK9QQmlError11descriptionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QQmlError::QQmlError(const QQmlError & );
  fn _ZN9QQmlErrorC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QQmlError::line();
  fn _ZNK9QQmlError4lineEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QString QQmlError::toString();
  fn _ZNK9QQmlError8toStringEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QQmlError::~QQmlError();
  fn _ZN9QQmlErrorD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QQmlError::QQmlError();
  fn _ZN9QQmlErrorC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QQmlError::setLine(int );
  fn _ZN9QQmlError7setLineEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QQmlError::setUrl(const QUrl & );
  fn _ZN9QQmlError6setUrlERK4QUrl(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QQmlError::column();
  fn _ZNK9QQmlError6columnEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QQmlError::setColumn(int );
  fn _ZN9QQmlError9setColumnEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  bool QQmlError::isValid();
  fn _ZNK9QQmlError7isValidEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QObject * QQmlError::object();
  fn _ZNK9QQmlError6objectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QQmlError::setDescription(const QString & );
  fn _ZN9QQmlError14setDescriptionERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQmlError::setObject(QObject * );
  fn _ZN9QQmlError9setObjectEP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QUrl QQmlError::url();
  fn _ZNK9QQmlError3urlEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QQmlError)=8
#[derive(Default)]
pub struct QQmlError {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QQmlError {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QQmlError {
    return QQmlError{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QString QQmlError::description();
impl /*struct*/ QQmlError {
  pub fn description<RetType, T: QQmlError_description<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.description(self);
    // return 1;
  }
}

pub trait QQmlError_description<RetType> {
  fn description(self , rsthis: & QQmlError) -> RetType;
}

  // proto:  QString QQmlError::description();
impl<'a> /*trait*/ QQmlError_description<QString> for () {
  fn description(self , rsthis: & QQmlError) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QQmlError11descriptionEv()};
    let mut ret = unsafe {_ZNK9QQmlError11descriptionEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQmlError::QQmlError(const QQmlError & );
impl /*struct*/ QQmlError {
  pub fn new<T: QQmlError_new>(value: T) -> QQmlError {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QQmlError_new {
  fn new(self) -> QQmlError;
}

  // proto:  void QQmlError::QQmlError(const QQmlError & );
impl<'a> /*trait*/ QQmlError_new for (&'a QQmlError) {
  fn new(self) -> QQmlError {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QQmlErrorC2ERKS_()};
    let ctysz: c_int = unsafe{QQmlError_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QQmlErrorC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlError{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QQmlError::line();
impl /*struct*/ QQmlError {
  pub fn line<RetType, T: QQmlError_line<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.line(self);
    // return 1;
  }
}

pub trait QQmlError_line<RetType> {
  fn line(self , rsthis: & QQmlError) -> RetType;
}

  // proto:  int QQmlError::line();
impl<'a> /*trait*/ QQmlError_line<i32> for () {
  fn line(self , rsthis: & QQmlError) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QQmlError4lineEv()};
    let mut ret = unsafe {_ZNK9QQmlError4lineEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QString QQmlError::toString();
impl /*struct*/ QQmlError {
  pub fn toString<RetType, T: QQmlError_toString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toString(self);
    // return 1;
  }
}

pub trait QQmlError_toString<RetType> {
  fn toString(self , rsthis: & QQmlError) -> RetType;
}

  // proto:  QString QQmlError::toString();
impl<'a> /*trait*/ QQmlError_toString<QString> for () {
  fn toString(self , rsthis: & QQmlError) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QQmlError8toStringEv()};
    let mut ret = unsafe {_ZNK9QQmlError8toStringEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQmlError::~QQmlError();
impl /*struct*/ QQmlError {
  pub fn free<RetType, T: QQmlError_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QQmlError_free<RetType> {
  fn free(self , rsthis: & QQmlError) -> RetType;
}

  // proto:  void QQmlError::~QQmlError();
impl<'a> /*trait*/ QQmlError_free<()> for () {
  fn free(self , rsthis: & QQmlError) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QQmlErrorD2Ev()};
     unsafe {_ZN9QQmlErrorD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QQmlError::QQmlError();
impl<'a> /*trait*/ QQmlError_new for () {
  fn new(self) -> QQmlError {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QQmlErrorC2Ev()};
    let ctysz: c_int = unsafe{QQmlError_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN9QQmlErrorC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlError{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QQmlError::setLine(int );
impl /*struct*/ QQmlError {
  pub fn setLine<RetType, T: QQmlError_setLine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLine(self);
    // return 1;
  }
}

pub trait QQmlError_setLine<RetType> {
  fn setLine(self , rsthis: & QQmlError) -> RetType;
}

  // proto:  void QQmlError::setLine(int );
impl<'a> /*trait*/ QQmlError_setLine<()> for (i32) {
  fn setLine(self , rsthis: & QQmlError) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QQmlError7setLineEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QQmlError7setLineEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQmlError::setUrl(const QUrl & );
impl /*struct*/ QQmlError {
  pub fn setUrl<RetType, T: QQmlError_setUrl<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setUrl(self);
    // return 1;
  }
}

pub trait QQmlError_setUrl<RetType> {
  fn setUrl(self , rsthis: & QQmlError) -> RetType;
}

  // proto:  void QQmlError::setUrl(const QUrl & );
impl<'a> /*trait*/ QQmlError_setUrl<()> for (&'a QUrl) {
  fn setUrl(self , rsthis: & QQmlError) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QQmlError6setUrlERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QQmlError6setUrlERK4QUrl(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QQmlError::column();
impl /*struct*/ QQmlError {
  pub fn column<RetType, T: QQmlError_column<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.column(self);
    // return 1;
  }
}

pub trait QQmlError_column<RetType> {
  fn column(self , rsthis: & QQmlError) -> RetType;
}

  // proto:  int QQmlError::column();
impl<'a> /*trait*/ QQmlError_column<i32> for () {
  fn column(self , rsthis: & QQmlError) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QQmlError6columnEv()};
    let mut ret = unsafe {_ZNK9QQmlError6columnEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QQmlError::setColumn(int );
impl /*struct*/ QQmlError {
  pub fn setColumn<RetType, T: QQmlError_setColumn<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setColumn(self);
    // return 1;
  }
}

pub trait QQmlError_setColumn<RetType> {
  fn setColumn(self , rsthis: & QQmlError) -> RetType;
}

  // proto:  void QQmlError::setColumn(int );
impl<'a> /*trait*/ QQmlError_setColumn<()> for (i32) {
  fn setColumn(self , rsthis: & QQmlError) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QQmlError9setColumnEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QQmlError9setColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QQmlError::isValid();
impl /*struct*/ QQmlError {
  pub fn isValid<RetType, T: QQmlError_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QQmlError_isValid<RetType> {
  fn isValid(self , rsthis: & QQmlError) -> RetType;
}

  // proto:  bool QQmlError::isValid();
impl<'a> /*trait*/ QQmlError_isValid<i8> for () {
  fn isValid(self , rsthis: & QQmlError) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QQmlError7isValidEv()};
    let mut ret = unsafe {_ZNK9QQmlError7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QObject * QQmlError::object();
impl /*struct*/ QQmlError {
  pub fn object<RetType, T: QQmlError_object<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.object(self);
    // return 1;
  }
}

pub trait QQmlError_object<RetType> {
  fn object(self , rsthis: & QQmlError) -> RetType;
}

  // proto:  QObject * QQmlError::object();
impl<'a> /*trait*/ QQmlError_object<QObject> for () {
  fn object(self , rsthis: & QQmlError) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QQmlError6objectEv()};
    let mut ret = unsafe {_ZNK9QQmlError6objectEv(rsthis.qclsinst)};
    let mut ret1 = QObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQmlError::setDescription(const QString & );
impl /*struct*/ QQmlError {
  pub fn setDescription<RetType, T: QQmlError_setDescription<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDescription(self);
    // return 1;
  }
}

pub trait QQmlError_setDescription<RetType> {
  fn setDescription(self , rsthis: & QQmlError) -> RetType;
}

  // proto:  void QQmlError::setDescription(const QString & );
impl<'a> /*trait*/ QQmlError_setDescription<()> for (&'a QString) {
  fn setDescription(self , rsthis: & QQmlError) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QQmlError14setDescriptionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QQmlError14setDescriptionERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQmlError::setObject(QObject * );
impl /*struct*/ QQmlError {
  pub fn setObject<RetType, T: QQmlError_setObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setObject(self);
    // return 1;
  }
}

pub trait QQmlError_setObject<RetType> {
  fn setObject(self , rsthis: & QQmlError) -> RetType;
}

  // proto:  void QQmlError::setObject(QObject * );
impl<'a> /*trait*/ QQmlError_setObject<()> for (&'a QObject) {
  fn setObject(self , rsthis: & QQmlError) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QQmlError9setObjectEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QQmlError9setObjectEP7QObject(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QUrl QQmlError::url();
impl /*struct*/ QQmlError {
  pub fn url<RetType, T: QQmlError_url<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.url(self);
    // return 1;
  }
}

pub trait QQmlError_url<RetType> {
  fn url(self , rsthis: & QQmlError) -> RetType;
}

  // proto:  QUrl QQmlError::url();
impl<'a> /*trait*/ QQmlError_url<QUrl> for () {
  fn url(self , rsthis: & QQmlError) -> QUrl {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QQmlError3urlEv()};
    let mut ret = unsafe {_ZNK9QQmlError3urlEv(rsthis.qclsinst)};
    let mut ret1 = QUrl::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

