// auto generated, do not modify.
// created: Wed Dec 30 23:22:52 2015
// src-file: /QtCore/qobjectcleanuphandler.h
// dst-file: /src/core/qobjectcleanuphandler.rs
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
use super::qobject::QObject; // 773
use std::ops::Deref;
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QObjectCleanupHandler_Class_Size() -> c_int;
  // proto:  void QObjectCleanupHandler::clear();
  fn _ZN21QObjectCleanupHandler5clearEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QObjectCleanupHandler::isEmpty();
  fn _ZNK21QObjectCleanupHandler7isEmptyEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QObjectCleanupHandler::~QObjectCleanupHandler();
  fn _ZN21QObjectCleanupHandlerD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  const QMetaObject * QObjectCleanupHandler::metaObject();
  fn _ZNK21QObjectCleanupHandler10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QObjectCleanupHandler::remove(QObject * object);
  fn _ZN21QObjectCleanupHandler6removeEP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QObject * QObjectCleanupHandler::add(QObject * object);
  fn _ZN21QObjectCleanupHandler3addEP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QObjectCleanupHandler::QObjectCleanupHandler();
  fn dector_ZN21QObjectCleanupHandlerC1Ev() -> *mut c_void;
  fn _ZN21QObjectCleanupHandlerC1Ev(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QObjectCleanupHandler)=1
#[derive(Default)]
pub struct QObjectCleanupHandler {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QObjectCleanupHandler {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QObjectCleanupHandler {
    return QObjectCleanupHandler{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QObjectCleanupHandler {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QObjectCleanupHandler {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QObjectCleanupHandler::clear();
impl /*struct*/ QObjectCleanupHandler {
  pub fn clear<RetType, T: QObjectCleanupHandler_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QObjectCleanupHandler_clear<RetType> {
  fn clear(self , rsthis: & QObjectCleanupHandler) -> RetType;
}

  // proto:  void QObjectCleanupHandler::clear();
impl<'a> /*trait*/ QObjectCleanupHandler_clear<()> for () {
  fn clear(self , rsthis: & QObjectCleanupHandler) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QObjectCleanupHandler5clearEv()};
     unsafe {_ZN21QObjectCleanupHandler5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QObjectCleanupHandler::isEmpty();
impl /*struct*/ QObjectCleanupHandler {
  pub fn isEmpty<RetType, T: QObjectCleanupHandler_isEmpty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QObjectCleanupHandler_isEmpty<RetType> {
  fn isEmpty(self , rsthis: & QObjectCleanupHandler) -> RetType;
}

  // proto:  bool QObjectCleanupHandler::isEmpty();
impl<'a> /*trait*/ QObjectCleanupHandler_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: & QObjectCleanupHandler) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QObjectCleanupHandler7isEmptyEv()};
    let mut ret = unsafe {_ZNK21QObjectCleanupHandler7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QObjectCleanupHandler::~QObjectCleanupHandler();
impl /*struct*/ QObjectCleanupHandler {
  pub fn Free<RetType, T: QObjectCleanupHandler_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QObjectCleanupHandler_Free<RetType> {
  fn Free(self , rsthis: & QObjectCleanupHandler) -> RetType;
}

  // proto:  void QObjectCleanupHandler::~QObjectCleanupHandler();
impl<'a> /*trait*/ QObjectCleanupHandler_Free<()> for () {
  fn Free(self , rsthis: & QObjectCleanupHandler) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QObjectCleanupHandlerD0Ev()};
     unsafe {_ZN21QObjectCleanupHandlerD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QObjectCleanupHandler::metaObject();
impl /*struct*/ QObjectCleanupHandler {
  pub fn metaObject<RetType, T: QObjectCleanupHandler_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QObjectCleanupHandler_metaObject<RetType> {
  fn metaObject(self , rsthis: & QObjectCleanupHandler) -> RetType;
}

  // proto:  const QMetaObject * QObjectCleanupHandler::metaObject();
impl<'a> /*trait*/ QObjectCleanupHandler_metaObject<()> for () {
  fn metaObject(self , rsthis: & QObjectCleanupHandler) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QObjectCleanupHandler10metaObjectEv()};
     unsafe {_ZNK21QObjectCleanupHandler10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QObjectCleanupHandler::remove(QObject * object);
impl /*struct*/ QObjectCleanupHandler {
  pub fn remove<RetType, T: QObjectCleanupHandler_remove<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.remove(self);
    // return 1;
  }
}

pub trait QObjectCleanupHandler_remove<RetType> {
  fn remove(self , rsthis: & QObjectCleanupHandler) -> RetType;
}

  // proto:  void QObjectCleanupHandler::remove(QObject * object);
impl<'a> /*trait*/ QObjectCleanupHandler_remove<()> for (&'a QObject) {
  fn remove(self , rsthis: & QObjectCleanupHandler) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QObjectCleanupHandler6removeEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN21QObjectCleanupHandler6removeEP7QObject(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QObject * QObjectCleanupHandler::add(QObject * object);
impl /*struct*/ QObjectCleanupHandler {
  pub fn add<RetType, T: QObjectCleanupHandler_add<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.add(self);
    // return 1;
  }
}

pub trait QObjectCleanupHandler_add<RetType> {
  fn add(self , rsthis: & QObjectCleanupHandler) -> RetType;
}

  // proto:  QObject * QObjectCleanupHandler::add(QObject * object);
impl<'a> /*trait*/ QObjectCleanupHandler_add<QObject> for (&'a QObject) {
  fn add(self , rsthis: & QObjectCleanupHandler) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QObjectCleanupHandler3addEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN21QObjectCleanupHandler3addEP7QObject(rsthis.qclsinst, arg0)};
    let mut ret1 = QObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QObjectCleanupHandler::QObjectCleanupHandler();
impl /*struct*/ QObjectCleanupHandler {
  pub fn New<T: QObjectCleanupHandler_New>(value: T) -> QObjectCleanupHandler {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QObjectCleanupHandler_New {
  fn New(self) -> QObjectCleanupHandler;
}

  // proto:  void QObjectCleanupHandler::QObjectCleanupHandler();
impl<'a> /*trait*/ QObjectCleanupHandler_New for () {
  fn New(self) -> QObjectCleanupHandler {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QObjectCleanupHandlerC1Ev()};
    let ctysz: c_int = unsafe{QObjectCleanupHandler_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN21QObjectCleanupHandlerC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN21QObjectCleanupHandlerC1Ev()} as u64;
    let rsthis = QObjectCleanupHandler{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

