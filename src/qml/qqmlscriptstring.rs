// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQml/qqmlscriptstring.h
// dst-file: /src/qml/qqmlscriptstring.rs
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
use super::qqmlcontext::QQmlContext; // 773
use super::super::core::qobject::QObject; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QQmlScriptString_Class_Size() -> c_int;
  // proto:  bool QQmlScriptString::isEmpty();
  fn _ZNK16QQmlScriptString7isEmptyEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QQmlScriptString::~QQmlScriptString();
  fn _ZN16QQmlScriptStringD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QQmlScriptString::QQmlScriptString(const QString & script, QQmlContext * context, QObject * scope);
  fn _ZN16QQmlScriptStringC2ERK7QStringP11QQmlContextP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  bool QQmlScriptString::booleanLiteral(bool * ok);
  fn _ZNK16QQmlScriptString14booleanLiteralEPb(qthis: u64 /* *mut c_void*/, arg0: *mut c_char) -> c_char;
  // proto:  bool QQmlScriptString::isNullLiteral();
  fn _ZNK16QQmlScriptString13isNullLiteralEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QQmlScriptString::isUndefinedLiteral();
  fn _ZNK16QQmlScriptString18isUndefinedLiteralEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QQmlScriptString::QQmlScriptString(const QQmlScriptString & );
  fn _ZN16QQmlScriptStringC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QQmlScriptString::stringLiteral();
  fn _ZNK16QQmlScriptString13stringLiteralEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  qreal QQmlScriptString::numberLiteral(bool * ok);
  fn _ZNK16QQmlScriptString13numberLiteralEPb(qthis: u64 /* *mut c_void*/, arg0: *mut c_char) -> c_double;
  // proto:  void QQmlScriptString::QQmlScriptString();
  fn _ZN16QQmlScriptStringC2Ev(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QQmlScriptString)=1
#[derive(Default)]
pub struct QQmlScriptString {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QQmlScriptString {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QQmlScriptString {
    return QQmlScriptString{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  bool QQmlScriptString::isEmpty();
impl /*struct*/ QQmlScriptString {
  pub fn isEmpty<RetType, T: QQmlScriptString_isEmpty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QQmlScriptString_isEmpty<RetType> {
  fn isEmpty(self , rsthis: & QQmlScriptString) -> RetType;
}

  // proto:  bool QQmlScriptString::isEmpty();
impl<'a> /*trait*/ QQmlScriptString_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: & QQmlScriptString) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QQmlScriptString7isEmptyEv()};
    let mut ret = unsafe {_ZNK16QQmlScriptString7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QQmlScriptString::~QQmlScriptString();
impl /*struct*/ QQmlScriptString {
  pub fn free<RetType, T: QQmlScriptString_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QQmlScriptString_free<RetType> {
  fn free(self , rsthis: & QQmlScriptString) -> RetType;
}

  // proto:  void QQmlScriptString::~QQmlScriptString();
impl<'a> /*trait*/ QQmlScriptString_free<()> for () {
  fn free(self , rsthis: & QQmlScriptString) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QQmlScriptStringD2Ev()};
     unsafe {_ZN16QQmlScriptStringD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QQmlScriptString::QQmlScriptString(const QString & script, QQmlContext * context, QObject * scope);
impl /*struct*/ QQmlScriptString {
  pub fn new<T: QQmlScriptString_new>(value: T) -> QQmlScriptString {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QQmlScriptString_new {
  fn new(self) -> QQmlScriptString;
}

  // proto:  void QQmlScriptString::QQmlScriptString(const QString & script, QQmlContext * context, QObject * scope);
impl<'a> /*trait*/ QQmlScriptString_new for (&'a QString, &'a QQmlContext, &'a QObject) {
  fn new(self) -> QQmlScriptString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QQmlScriptStringC2ERK7QStringP11QQmlContextP7QObject()};
    let ctysz: c_int = unsafe{QQmlScriptString_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN16QQmlScriptStringC2ERK7QStringP11QQmlContextP7QObject(qthis_ph, arg0, arg1, arg2)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlScriptString{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QQmlScriptString::booleanLiteral(bool * ok);
impl /*struct*/ QQmlScriptString {
  pub fn booleanLiteral<RetType, T: QQmlScriptString_booleanLiteral<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.booleanLiteral(self);
    // return 1;
  }
}

pub trait QQmlScriptString_booleanLiteral<RetType> {
  fn booleanLiteral(self , rsthis: & QQmlScriptString) -> RetType;
}

  // proto:  bool QQmlScriptString::booleanLiteral(bool * ok);
impl<'a> /*trait*/ QQmlScriptString_booleanLiteral<i8> for (&'a mut Vec<i8>) {
  fn booleanLiteral(self , rsthis: & QQmlScriptString) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QQmlScriptString14booleanLiteralEPb()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZNK16QQmlScriptString14booleanLiteralEPb(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QQmlScriptString::isNullLiteral();
impl /*struct*/ QQmlScriptString {
  pub fn isNullLiteral<RetType, T: QQmlScriptString_isNullLiteral<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNullLiteral(self);
    // return 1;
  }
}

pub trait QQmlScriptString_isNullLiteral<RetType> {
  fn isNullLiteral(self , rsthis: & QQmlScriptString) -> RetType;
}

  // proto:  bool QQmlScriptString::isNullLiteral();
impl<'a> /*trait*/ QQmlScriptString_isNullLiteral<i8> for () {
  fn isNullLiteral(self , rsthis: & QQmlScriptString) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QQmlScriptString13isNullLiteralEv()};
    let mut ret = unsafe {_ZNK16QQmlScriptString13isNullLiteralEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QQmlScriptString::isUndefinedLiteral();
impl /*struct*/ QQmlScriptString {
  pub fn isUndefinedLiteral<RetType, T: QQmlScriptString_isUndefinedLiteral<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isUndefinedLiteral(self);
    // return 1;
  }
}

pub trait QQmlScriptString_isUndefinedLiteral<RetType> {
  fn isUndefinedLiteral(self , rsthis: & QQmlScriptString) -> RetType;
}

  // proto:  bool QQmlScriptString::isUndefinedLiteral();
impl<'a> /*trait*/ QQmlScriptString_isUndefinedLiteral<i8> for () {
  fn isUndefinedLiteral(self , rsthis: & QQmlScriptString) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QQmlScriptString18isUndefinedLiteralEv()};
    let mut ret = unsafe {_ZNK16QQmlScriptString18isUndefinedLiteralEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QQmlScriptString::QQmlScriptString(const QQmlScriptString & );
impl<'a> /*trait*/ QQmlScriptString_new for (&'a QQmlScriptString) {
  fn new(self) -> QQmlScriptString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QQmlScriptStringC2ERKS_()};
    let ctysz: c_int = unsafe{QQmlScriptString_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QQmlScriptStringC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlScriptString{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QQmlScriptString::stringLiteral();
impl /*struct*/ QQmlScriptString {
  pub fn stringLiteral<RetType, T: QQmlScriptString_stringLiteral<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.stringLiteral(self);
    // return 1;
  }
}

pub trait QQmlScriptString_stringLiteral<RetType> {
  fn stringLiteral(self , rsthis: & QQmlScriptString) -> RetType;
}

  // proto:  QString QQmlScriptString::stringLiteral();
impl<'a> /*trait*/ QQmlScriptString_stringLiteral<QString> for () {
  fn stringLiteral(self , rsthis: & QQmlScriptString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QQmlScriptString13stringLiteralEv()};
    let mut ret = unsafe {_ZNK16QQmlScriptString13stringLiteralEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QQmlScriptString::numberLiteral(bool * ok);
impl /*struct*/ QQmlScriptString {
  pub fn numberLiteral<RetType, T: QQmlScriptString_numberLiteral<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.numberLiteral(self);
    // return 1;
  }
}

pub trait QQmlScriptString_numberLiteral<RetType> {
  fn numberLiteral(self , rsthis: & QQmlScriptString) -> RetType;
}

  // proto:  qreal QQmlScriptString::numberLiteral(bool * ok);
impl<'a> /*trait*/ QQmlScriptString_numberLiteral<f64> for (&'a mut Vec<i8>) {
  fn numberLiteral(self , rsthis: & QQmlScriptString) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QQmlScriptString13numberLiteralEPb()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZNK16QQmlScriptString13numberLiteralEPb(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QQmlScriptString::QQmlScriptString();
impl<'a> /*trait*/ QQmlScriptString_new for () {
  fn new(self) -> QQmlScriptString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QQmlScriptStringC2Ev()};
    let ctysz: c_int = unsafe{QQmlScriptString_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN16QQmlScriptStringC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlScriptString{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

