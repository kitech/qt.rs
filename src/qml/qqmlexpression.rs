// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQml/qqmlexpression.h
// dst-file: /src/qml/qqmlexpression.rs
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
use super::super::core::qstring::QString; // 771
use super::qqmlscriptstring::QQmlScriptString; // 773
use super::qqmlcontext::QQmlContext; // 773
use super::qqmlengine::QQmlEngine; // 773
use super::super::core::qvariant::QVariant; // 771
use super::qqmlerror::QQmlError; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QQmlExpression_Class_Size() -> c_int;
  // proto:  void QQmlExpression::QQmlExpression(const QQmlScriptString & , QQmlContext * , QObject * , QObject * );
  fn _ZN14QQmlExpressionC2ERK16QQmlScriptStringP11QQmlContextP7QObjectS6_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void);
  // proto:  bool QQmlExpression::hasError();
  fn _ZNK14QQmlExpression8hasErrorEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QQmlExpression::QQmlExpression(const QQmlExpression & );
  fn _ZN14QQmlExpressionC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QQmlExpression::sourceFile();
  fn _ZNK14QQmlExpression10sourceFileEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QMetaObject * QQmlExpression::metaObject();
  fn _ZNK14QQmlExpression10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  QQmlEngine * QQmlExpression::engine();
  fn _ZNK14QQmlExpression6engineEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QQmlExpression::expression();
  fn _ZNK14QQmlExpression10expressionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QQmlExpression::QQmlExpression();
  fn _ZN14QQmlExpressionC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  int QQmlExpression::lineNumber();
  fn _ZNK14QQmlExpression10lineNumberEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QQmlExpression::~QQmlExpression();
  fn _ZN14QQmlExpressionD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QQmlExpression::clearError();
  fn _ZN14QQmlExpression10clearErrorEv(qthis: u64 /* *mut c_void*/);
  // proto:  QVariant QQmlExpression::evaluate(bool * valueIsUndefined);
  fn _ZN14QQmlExpression8evaluateEPb(qthis: u64 /* *mut c_void*/, arg0: *mut c_char) -> *mut c_void;
  // proto:  int QQmlExpression::columnNumber();
  fn _ZNK14QQmlExpression12columnNumberEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QQmlExpression::setSourceLocation(const QString & fileName, int line, int column);
  fn _ZN14QQmlExpression17setSourceLocationERK7QStringii(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int, arg2: c_int);
  // proto:  bool QQmlExpression::notifyOnValueChanged();
  fn _ZNK14QQmlExpression20notifyOnValueChangedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QQmlExpression::QQmlExpression(QQmlContext * , QObject * , const QString & , QObject * );
  fn _ZN14QQmlExpressionC2EP11QQmlContextP7QObjectRK7QStringS3_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void);
  // proto:  QQmlError QQmlExpression::error();
  fn _ZNK14QQmlExpression5errorEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QQmlContext * QQmlExpression::context();
  fn _ZNK14QQmlExpression7contextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QQmlExpression::setExpression(const QString & );
  fn _ZN14QQmlExpression13setExpressionERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQmlExpression::setNotifyOnValueChanged(bool );
  fn _ZN14QQmlExpression23setNotifyOnValueChangedEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QObject * QQmlExpression::scopeObject();
  fn _ZNK14QQmlExpression11scopeObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QQmlExpression_SlotProxy_connect__ZN14QQmlExpression12valueChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QQmlExpression)=1
#[derive(Default)]
pub struct QQmlExpression {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _valueChanged: QQmlExpression_valueChanged_signal,
}

impl /*struct*/ QQmlExpression {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QQmlExpression {
    return QQmlExpression{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QQmlExpression {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QQmlExpression {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QQmlExpression::QQmlExpression(const QQmlScriptString & , QQmlContext * , QObject * , QObject * );
impl /*struct*/ QQmlExpression {
  pub fn new<T: QQmlExpression_new>(value: T) -> QQmlExpression {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QQmlExpression_new {
  fn new(self) -> QQmlExpression;
}

  // proto:  void QQmlExpression::QQmlExpression(const QQmlScriptString & , QQmlContext * , QObject * , QObject * );
impl<'a> /*trait*/ QQmlExpression_new for (&'a QQmlScriptString, &'a QQmlContext, &'a QObject, &'a QObject) {
  fn new(self) -> QQmlExpression {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QQmlExpressionC2ERK16QQmlScriptStringP11QQmlContextP7QObjectS6_()};
    let ctysz: c_int = unsafe{QQmlExpression_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    unsafe {_ZN14QQmlExpressionC2ERK16QQmlScriptStringP11QQmlContextP7QObjectS6_(qthis_ph, arg0, arg1, arg2, arg3)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlExpression{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QQmlExpression::hasError();
impl /*struct*/ QQmlExpression {
  pub fn hasError<RetType, T: QQmlExpression_hasError<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasError(self);
    // return 1;
  }
}

pub trait QQmlExpression_hasError<RetType> {
  fn hasError(self , rsthis: & QQmlExpression) -> RetType;
}

  // proto:  bool QQmlExpression::hasError();
impl<'a> /*trait*/ QQmlExpression_hasError<i8> for () {
  fn hasError(self , rsthis: & QQmlExpression) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QQmlExpression8hasErrorEv()};
    let mut ret = unsafe {_ZNK14QQmlExpression8hasErrorEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QQmlExpression::QQmlExpression(const QQmlExpression & );
impl<'a> /*trait*/ QQmlExpression_new for (&'a QQmlExpression) {
  fn new(self) -> QQmlExpression {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QQmlExpressionC2ERKS_()};
    let ctysz: c_int = unsafe{QQmlExpression_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QQmlExpressionC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlExpression{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QQmlExpression::sourceFile();
impl /*struct*/ QQmlExpression {
  pub fn sourceFile<RetType, T: QQmlExpression_sourceFile<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sourceFile(self);
    // return 1;
  }
}

pub trait QQmlExpression_sourceFile<RetType> {
  fn sourceFile(self , rsthis: & QQmlExpression) -> RetType;
}

  // proto:  QString QQmlExpression::sourceFile();
impl<'a> /*trait*/ QQmlExpression_sourceFile<QString> for () {
  fn sourceFile(self , rsthis: & QQmlExpression) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QQmlExpression10sourceFileEv()};
    let mut ret = unsafe {_ZNK14QQmlExpression10sourceFileEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QQmlExpression::metaObject();
impl /*struct*/ QQmlExpression {
  pub fn metaObject<RetType, T: QQmlExpression_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QQmlExpression_metaObject<RetType> {
  fn metaObject(self , rsthis: & QQmlExpression) -> RetType;
}

  // proto:  const QMetaObject * QQmlExpression::metaObject();
impl<'a> /*trait*/ QQmlExpression_metaObject<()> for () {
  fn metaObject(self , rsthis: & QQmlExpression) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QQmlExpression10metaObjectEv()};
     unsafe {_ZNK14QQmlExpression10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QQmlEngine * QQmlExpression::engine();
impl /*struct*/ QQmlExpression {
  pub fn engine<RetType, T: QQmlExpression_engine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.engine(self);
    // return 1;
  }
}

pub trait QQmlExpression_engine<RetType> {
  fn engine(self , rsthis: & QQmlExpression) -> RetType;
}

  // proto:  QQmlEngine * QQmlExpression::engine();
impl<'a> /*trait*/ QQmlExpression_engine<QQmlEngine> for () {
  fn engine(self , rsthis: & QQmlExpression) -> QQmlEngine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QQmlExpression6engineEv()};
    let mut ret = unsafe {_ZNK14QQmlExpression6engineEv(rsthis.qclsinst)};
    let mut ret1 = QQmlEngine::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QQmlExpression::expression();
impl /*struct*/ QQmlExpression {
  pub fn expression<RetType, T: QQmlExpression_expression<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.expression(self);
    // return 1;
  }
}

pub trait QQmlExpression_expression<RetType> {
  fn expression(self , rsthis: & QQmlExpression) -> RetType;
}

  // proto:  QString QQmlExpression::expression();
impl<'a> /*trait*/ QQmlExpression_expression<QString> for () {
  fn expression(self , rsthis: & QQmlExpression) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QQmlExpression10expressionEv()};
    let mut ret = unsafe {_ZNK14QQmlExpression10expressionEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQmlExpression::QQmlExpression();
impl<'a> /*trait*/ QQmlExpression_new for () {
  fn new(self) -> QQmlExpression {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QQmlExpressionC2Ev()};
    let ctysz: c_int = unsafe{QQmlExpression_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN14QQmlExpressionC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlExpression{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QQmlExpression::lineNumber();
impl /*struct*/ QQmlExpression {
  pub fn lineNumber<RetType, T: QQmlExpression_lineNumber<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lineNumber(self);
    // return 1;
  }
}

pub trait QQmlExpression_lineNumber<RetType> {
  fn lineNumber(self , rsthis: & QQmlExpression) -> RetType;
}

  // proto:  int QQmlExpression::lineNumber();
impl<'a> /*trait*/ QQmlExpression_lineNumber<i32> for () {
  fn lineNumber(self , rsthis: & QQmlExpression) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QQmlExpression10lineNumberEv()};
    let mut ret = unsafe {_ZNK14QQmlExpression10lineNumberEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QQmlExpression::~QQmlExpression();
impl /*struct*/ QQmlExpression {
  pub fn free<RetType, T: QQmlExpression_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QQmlExpression_free<RetType> {
  fn free(self , rsthis: & QQmlExpression) -> RetType;
}

  // proto:  void QQmlExpression::~QQmlExpression();
impl<'a> /*trait*/ QQmlExpression_free<()> for () {
  fn free(self , rsthis: & QQmlExpression) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QQmlExpressionD2Ev()};
     unsafe {_ZN14QQmlExpressionD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QQmlExpression::clearError();
impl /*struct*/ QQmlExpression {
  pub fn clearError<RetType, T: QQmlExpression_clearError<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clearError(self);
    // return 1;
  }
}

pub trait QQmlExpression_clearError<RetType> {
  fn clearError(self , rsthis: & QQmlExpression) -> RetType;
}

  // proto:  void QQmlExpression::clearError();
impl<'a> /*trait*/ QQmlExpression_clearError<()> for () {
  fn clearError(self , rsthis: & QQmlExpression) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QQmlExpression10clearErrorEv()};
     unsafe {_ZN14QQmlExpression10clearErrorEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QVariant QQmlExpression::evaluate(bool * valueIsUndefined);
impl /*struct*/ QQmlExpression {
  pub fn evaluate<RetType, T: QQmlExpression_evaluate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.evaluate(self);
    // return 1;
  }
}

pub trait QQmlExpression_evaluate<RetType> {
  fn evaluate(self , rsthis: & QQmlExpression) -> RetType;
}

  // proto:  QVariant QQmlExpression::evaluate(bool * valueIsUndefined);
impl<'a> /*trait*/ QQmlExpression_evaluate<QVariant> for (&'a mut Vec<i8>) {
  fn evaluate(self , rsthis: & QQmlExpression) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QQmlExpression8evaluateEPb()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN14QQmlExpression8evaluateEPb(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QQmlExpression::columnNumber();
impl /*struct*/ QQmlExpression {
  pub fn columnNumber<RetType, T: QQmlExpression_columnNumber<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.columnNumber(self);
    // return 1;
  }
}

pub trait QQmlExpression_columnNumber<RetType> {
  fn columnNumber(self , rsthis: & QQmlExpression) -> RetType;
}

  // proto:  int QQmlExpression::columnNumber();
impl<'a> /*trait*/ QQmlExpression_columnNumber<i32> for () {
  fn columnNumber(self , rsthis: & QQmlExpression) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QQmlExpression12columnNumberEv()};
    let mut ret = unsafe {_ZNK14QQmlExpression12columnNumberEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QQmlExpression::setSourceLocation(const QString & fileName, int line, int column);
impl /*struct*/ QQmlExpression {
  pub fn setSourceLocation<RetType, T: QQmlExpression_setSourceLocation<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSourceLocation(self);
    // return 1;
  }
}

pub trait QQmlExpression_setSourceLocation<RetType> {
  fn setSourceLocation(self , rsthis: & QQmlExpression) -> RetType;
}

  // proto:  void QQmlExpression::setSourceLocation(const QString & fileName, int line, int column);
impl<'a> /*trait*/ QQmlExpression_setSourceLocation<()> for (&'a QString, i32, i32) {
  fn setSourceLocation(self , rsthis: & QQmlExpression) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QQmlExpression17setSourceLocationERK7QStringii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN14QQmlExpression17setSourceLocationERK7QStringii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  bool QQmlExpression::notifyOnValueChanged();
impl /*struct*/ QQmlExpression {
  pub fn notifyOnValueChanged<RetType, T: QQmlExpression_notifyOnValueChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.notifyOnValueChanged(self);
    // return 1;
  }
}

pub trait QQmlExpression_notifyOnValueChanged<RetType> {
  fn notifyOnValueChanged(self , rsthis: & QQmlExpression) -> RetType;
}

  // proto:  bool QQmlExpression::notifyOnValueChanged();
impl<'a> /*trait*/ QQmlExpression_notifyOnValueChanged<i8> for () {
  fn notifyOnValueChanged(self , rsthis: & QQmlExpression) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QQmlExpression20notifyOnValueChangedEv()};
    let mut ret = unsafe {_ZNK14QQmlExpression20notifyOnValueChangedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QQmlExpression::QQmlExpression(QQmlContext * , QObject * , const QString & , QObject * );
impl<'a> /*trait*/ QQmlExpression_new for (&'a QQmlContext, &'a QObject, &'a QString, &'a QObject) {
  fn new(self) -> QQmlExpression {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QQmlExpressionC2EP11QQmlContextP7QObjectRK7QStringS3_()};
    let ctysz: c_int = unsafe{QQmlExpression_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    unsafe {_ZN14QQmlExpressionC2EP11QQmlContextP7QObjectRK7QStringS3_(qthis_ph, arg0, arg1, arg2, arg3)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlExpression{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QQmlError QQmlExpression::error();
impl /*struct*/ QQmlExpression {
  pub fn error<RetType, T: QQmlExpression_error<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.error(self);
    // return 1;
  }
}

pub trait QQmlExpression_error<RetType> {
  fn error(self , rsthis: & QQmlExpression) -> RetType;
}

  // proto:  QQmlError QQmlExpression::error();
impl<'a> /*trait*/ QQmlExpression_error<QQmlError> for () {
  fn error(self , rsthis: & QQmlExpression) -> QQmlError {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QQmlExpression5errorEv()};
    let mut ret = unsafe {_ZNK14QQmlExpression5errorEv(rsthis.qclsinst)};
    let mut ret1 = QQmlError::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QQmlContext * QQmlExpression::context();
impl /*struct*/ QQmlExpression {
  pub fn context<RetType, T: QQmlExpression_context<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.context(self);
    // return 1;
  }
}

pub trait QQmlExpression_context<RetType> {
  fn context(self , rsthis: & QQmlExpression) -> RetType;
}

  // proto:  QQmlContext * QQmlExpression::context();
impl<'a> /*trait*/ QQmlExpression_context<QQmlContext> for () {
  fn context(self , rsthis: & QQmlExpression) -> QQmlContext {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QQmlExpression7contextEv()};
    let mut ret = unsafe {_ZNK14QQmlExpression7contextEv(rsthis.qclsinst)};
    let mut ret1 = QQmlContext::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQmlExpression::setExpression(const QString & );
impl /*struct*/ QQmlExpression {
  pub fn setExpression<RetType, T: QQmlExpression_setExpression<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setExpression(self);
    // return 1;
  }
}

pub trait QQmlExpression_setExpression<RetType> {
  fn setExpression(self , rsthis: & QQmlExpression) -> RetType;
}

  // proto:  void QQmlExpression::setExpression(const QString & );
impl<'a> /*trait*/ QQmlExpression_setExpression<()> for (&'a QString) {
  fn setExpression(self , rsthis: & QQmlExpression) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QQmlExpression13setExpressionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QQmlExpression13setExpressionERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQmlExpression::setNotifyOnValueChanged(bool );
impl /*struct*/ QQmlExpression {
  pub fn setNotifyOnValueChanged<RetType, T: QQmlExpression_setNotifyOnValueChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setNotifyOnValueChanged(self);
    // return 1;
  }
}

pub trait QQmlExpression_setNotifyOnValueChanged<RetType> {
  fn setNotifyOnValueChanged(self , rsthis: & QQmlExpression) -> RetType;
}

  // proto:  void QQmlExpression::setNotifyOnValueChanged(bool );
impl<'a> /*trait*/ QQmlExpression_setNotifyOnValueChanged<()> for (i8) {
  fn setNotifyOnValueChanged(self , rsthis: & QQmlExpression) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QQmlExpression23setNotifyOnValueChangedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN14QQmlExpression23setNotifyOnValueChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QObject * QQmlExpression::scopeObject();
impl /*struct*/ QQmlExpression {
  pub fn scopeObject<RetType, T: QQmlExpression_scopeObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.scopeObject(self);
    // return 1;
  }
}

pub trait QQmlExpression_scopeObject<RetType> {
  fn scopeObject(self , rsthis: & QQmlExpression) -> RetType;
}

  // proto:  QObject * QQmlExpression::scopeObject();
impl<'a> /*trait*/ QQmlExpression_scopeObject<QObject> for () {
  fn scopeObject(self , rsthis: & QQmlExpression) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QQmlExpression11scopeObjectEv()};
    let mut ret = unsafe {_ZNK14QQmlExpression11scopeObjectEv(rsthis.qclsinst)};
    let mut ret1 = QObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

#[derive(Default)] // for QQmlExpression_valueChanged
pub struct QQmlExpression_valueChanged_signal{poi:u64}
impl /* struct */ QQmlExpression {
  pub fn valueChanged(&self) -> QQmlExpression_valueChanged_signal {
     return QQmlExpression_valueChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQmlExpression_valueChanged_signal {
  pub fn connect<T: QQmlExpression_valueChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQmlExpression_valueChanged_signal_connect {
  fn connect(self, sigthis: QQmlExpression_valueChanged_signal);
}

// valueChanged()
extern fn QQmlExpression_valueChanged_signal_connect_cb_0(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QQmlExpression_valueChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QQmlExpression_valueChanged_signal_connect for fn() {
  fn connect(self, sigthis: QQmlExpression_valueChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQmlExpression_valueChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQmlExpression_SlotProxy_connect__ZN14QQmlExpression12valueChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQmlExpression_valueChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QQmlExpression_valueChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQmlExpression_valueChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQmlExpression_SlotProxy_connect__ZN14QQmlExpression12valueChangedEv(arg0, arg1, arg2)};
  }
}
// <= body block end

