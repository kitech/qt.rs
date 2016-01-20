// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQml/qjsengine.h
// dst-file: /src/qml/qjsengine.rs
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
use super::qjsvalue::QJSValue; // 773
use super::super::core::qstring::QString; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QJSEngine_Class_Size() -> c_int;
  // proto:  QJSValue QJSEngine::newObject();
  fn _ZN9QJSEngine9newObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QJSEngine::QJSEngine(QObject * parent);
  fn _ZN9QJSEngineC2EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QJSValue QJSEngine::newArray(uint length);
  fn _ZN9QJSEngine8newArrayEj(qthis: u64 /* *mut c_void*/, arg0: c_uint) -> *mut c_void;
  // proto:  const QMetaObject * QJSEngine::metaObject();
  fn _ZNK9QJSEngine10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  QV8Engine * QJSEngine::handle();
  fn _ZNK9QJSEngine6handleEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QJSEngine::QJSEngine(const QJSEngine & );
  fn _ZN9QJSEngineC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QJSEngine::~QJSEngine();
  fn _ZN9QJSEngineD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QJSEngine::collectGarbage();
  fn _ZN9QJSEngine14collectGarbageEv(qthis: u64 /* *mut c_void*/);
  // proto:  QJSValue QJSEngine::globalObject();
  fn _ZNK9QJSEngine12globalObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QJSEngine::QJSEngine();
  fn _ZN9QJSEngineC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QJSEngine::installTranslatorFunctions(const QJSValue & object);
  fn _ZN9QJSEngine26installTranslatorFunctionsERK8QJSValue(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QJSValue QJSEngine::evaluate(const QString & program, const QString & fileName, int lineNumber);
  fn _ZN9QJSEngine8evaluateERK7QStringS2_i(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int) -> *mut c_void;
  // proto:  QJSValue QJSEngine::newQObject(QObject * object);
  fn _ZN9QJSEngine10newQObjectEP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QJSEngine)=1
#[derive(Default)]
pub struct QJSEngine {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QJSEngine {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QJSEngine {
    return QJSEngine{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QJSEngine {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QJSEngine {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  QJSValue QJSEngine::newObject();
impl /*struct*/ QJSEngine {
  pub fn newObject<RetType, T: QJSEngine_newObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.newObject(self);
    // return 1;
  }
}

pub trait QJSEngine_newObject<RetType> {
  fn newObject(self , rsthis: & QJSEngine) -> RetType;
}

  // proto:  QJSValue QJSEngine::newObject();
impl<'a> /*trait*/ QJSEngine_newObject<QJSValue> for () {
  fn newObject(self , rsthis: & QJSEngine) -> QJSValue {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QJSEngine9newObjectEv()};
    let mut ret = unsafe {_ZN9QJSEngine9newObjectEv(rsthis.qclsinst)};
    let mut ret1 = QJSValue::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QJSEngine::QJSEngine(QObject * parent);
impl /*struct*/ QJSEngine {
  pub fn new<T: QJSEngine_new>(value: T) -> QJSEngine {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QJSEngine_new {
  fn new(self) -> QJSEngine;
}

  // proto:  void QJSEngine::QJSEngine(QObject * parent);
impl<'a> /*trait*/ QJSEngine_new for (&'a QObject) {
  fn new(self) -> QJSEngine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QJSEngineC2EP7QObject()};
    let ctysz: c_int = unsafe{QJSEngine_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QJSEngineC2EP7QObject(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QJSEngine{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QJSValue QJSEngine::newArray(uint length);
impl /*struct*/ QJSEngine {
  pub fn newArray<RetType, T: QJSEngine_newArray<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.newArray(self);
    // return 1;
  }
}

pub trait QJSEngine_newArray<RetType> {
  fn newArray(self , rsthis: & QJSEngine) -> RetType;
}

  // proto:  QJSValue QJSEngine::newArray(uint length);
impl<'a> /*trait*/ QJSEngine_newArray<QJSValue> for (u32) {
  fn newArray(self , rsthis: & QJSEngine) -> QJSValue {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QJSEngine8newArrayEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN9QJSEngine8newArrayEj(rsthis.qclsinst, arg0)};
    let mut ret1 = QJSValue::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QJSEngine::metaObject();
impl /*struct*/ QJSEngine {
  pub fn metaObject<RetType, T: QJSEngine_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QJSEngine_metaObject<RetType> {
  fn metaObject(self , rsthis: & QJSEngine) -> RetType;
}

  // proto:  const QMetaObject * QJSEngine::metaObject();
impl<'a> /*trait*/ QJSEngine_metaObject<()> for () {
  fn metaObject(self , rsthis: & QJSEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QJSEngine10metaObjectEv()};
     unsafe {_ZNK9QJSEngine10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QV8Engine * QJSEngine::handle();
impl /*struct*/ QJSEngine {
  pub fn handle<RetType, T: QJSEngine_handle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.handle(self);
    // return 1;
  }
}

pub trait QJSEngine_handle<RetType> {
  fn handle(self , rsthis: & QJSEngine) -> RetType;
}

  // proto:  QV8Engine * QJSEngine::handle();
impl<'a> /*trait*/ QJSEngine_handle<()> for () {
  fn handle(self , rsthis: & QJSEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QJSEngine6handleEv()};
     unsafe {_ZNK9QJSEngine6handleEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QJSEngine::QJSEngine(const QJSEngine & );
impl<'a> /*trait*/ QJSEngine_new for (&'a QJSEngine) {
  fn new(self) -> QJSEngine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QJSEngineC2ERKS_()};
    let ctysz: c_int = unsafe{QJSEngine_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QJSEngineC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QJSEngine{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QJSEngine::~QJSEngine();
impl /*struct*/ QJSEngine {
  pub fn free<RetType, T: QJSEngine_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QJSEngine_free<RetType> {
  fn free(self , rsthis: & QJSEngine) -> RetType;
}

  // proto:  void QJSEngine::~QJSEngine();
impl<'a> /*trait*/ QJSEngine_free<()> for () {
  fn free(self , rsthis: & QJSEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QJSEngineD2Ev()};
     unsafe {_ZN9QJSEngineD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QJSEngine::collectGarbage();
impl /*struct*/ QJSEngine {
  pub fn collectGarbage<RetType, T: QJSEngine_collectGarbage<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.collectGarbage(self);
    // return 1;
  }
}

pub trait QJSEngine_collectGarbage<RetType> {
  fn collectGarbage(self , rsthis: & QJSEngine) -> RetType;
}

  // proto:  void QJSEngine::collectGarbage();
impl<'a> /*trait*/ QJSEngine_collectGarbage<()> for () {
  fn collectGarbage(self , rsthis: & QJSEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QJSEngine14collectGarbageEv()};
     unsafe {_ZN9QJSEngine14collectGarbageEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QJSValue QJSEngine::globalObject();
impl /*struct*/ QJSEngine {
  pub fn globalObject<RetType, T: QJSEngine_globalObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalObject(self);
    // return 1;
  }
}

pub trait QJSEngine_globalObject<RetType> {
  fn globalObject(self , rsthis: & QJSEngine) -> RetType;
}

  // proto:  QJSValue QJSEngine::globalObject();
impl<'a> /*trait*/ QJSEngine_globalObject<QJSValue> for () {
  fn globalObject(self , rsthis: & QJSEngine) -> QJSValue {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QJSEngine12globalObjectEv()};
    let mut ret = unsafe {_ZNK9QJSEngine12globalObjectEv(rsthis.qclsinst)};
    let mut ret1 = QJSValue::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QJSEngine::QJSEngine();
impl<'a> /*trait*/ QJSEngine_new for () {
  fn new(self) -> QJSEngine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QJSEngineC2Ev()};
    let ctysz: c_int = unsafe{QJSEngine_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN9QJSEngineC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QJSEngine{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QJSEngine::installTranslatorFunctions(const QJSValue & object);
impl /*struct*/ QJSEngine {
  pub fn installTranslatorFunctions<RetType, T: QJSEngine_installTranslatorFunctions<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.installTranslatorFunctions(self);
    // return 1;
  }
}

pub trait QJSEngine_installTranslatorFunctions<RetType> {
  fn installTranslatorFunctions(self , rsthis: & QJSEngine) -> RetType;
}

  // proto:  void QJSEngine::installTranslatorFunctions(const QJSValue & object);
impl<'a> /*trait*/ QJSEngine_installTranslatorFunctions<()> for (&'a QJSValue) {
  fn installTranslatorFunctions(self , rsthis: & QJSEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QJSEngine26installTranslatorFunctionsERK8QJSValue()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QJSEngine26installTranslatorFunctionsERK8QJSValue(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QJSValue QJSEngine::evaluate(const QString & program, const QString & fileName, int lineNumber);
impl /*struct*/ QJSEngine {
  pub fn evaluate<RetType, T: QJSEngine_evaluate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.evaluate(self);
    // return 1;
  }
}

pub trait QJSEngine_evaluate<RetType> {
  fn evaluate(self , rsthis: & QJSEngine) -> RetType;
}

  // proto:  QJSValue QJSEngine::evaluate(const QString & program, const QString & fileName, int lineNumber);
impl<'a> /*trait*/ QJSEngine_evaluate<QJSValue> for (&'a QString, &'a QString, i32) {
  fn evaluate(self , rsthis: & QJSEngine) -> QJSValue {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QJSEngine8evaluateERK7QStringS2_i()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZN9QJSEngine8evaluateERK7QStringS2_i(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QJSValue::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QJSValue QJSEngine::newQObject(QObject * object);
impl /*struct*/ QJSEngine {
  pub fn newQObject<RetType, T: QJSEngine_newQObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.newQObject(self);
    // return 1;
  }
}

pub trait QJSEngine_newQObject<RetType> {
  fn newQObject(self , rsthis: & QJSEngine) -> RetType;
}

  // proto:  QJSValue QJSEngine::newQObject(QObject * object);
impl<'a> /*trait*/ QJSEngine_newQObject<QJSValue> for (&'a QObject) {
  fn newQObject(self , rsthis: & QJSEngine) -> QJSValue {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QJSEngine10newQObjectEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QJSEngine10newQObjectEP7QObject(rsthis.qclsinst, arg0)};
    let mut ret1 = QJSValue::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

