// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QObjectCleanupHandler::clear();
  fn _ZN21QObjectCleanupHandler5clearEv(qthis: *mut c_void) ;
  // proto:  bool QObjectCleanupHandler::isEmpty();
  fn _ZNK21QObjectCleanupHandler7isEmptyEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QObjectCleanupHandler::FreeQObjectCleanupHandler();
  fn _ZN21QObjectCleanupHandlerD0Ev(qthis: *mut c_void) ;
  // proto:  const QMetaObject * QObjectCleanupHandler::metaObject();
  fn _ZNK21QObjectCleanupHandler10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QObjectCleanupHandler::remove(QObject * object);
  fn _ZN21QObjectCleanupHandler6removeEP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QObject * QObjectCleanupHandler::add(QObject * object);
  fn _ZN21QObjectCleanupHandler3addEP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QObjectCleanupHandler::NewQObjectCleanupHandler();
  fn _ZN21QObjectCleanupHandlerC1Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QObjectCleanupHandler)=1
pub struct QObjectCleanupHandler {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QObjectCleanupHandler {
  pub fn clear<RetType, T: QObjectCleanupHandler_clear<RetType>>(&mut self, value: T) -> RetType {
    return value.clear(self);
    // return 1;
  }
}

pub trait QObjectCleanupHandler_clear<RetType> {
  fn clear(self, rsthis: &mut QObjectCleanupHandler) -> RetType;
}

// proto:  void QObjectCleanupHandler::clear();
impl<'a> /*trait*/ QObjectCleanupHandler_clear<()> for () {
  fn clear(self, rsthis: &mut QObjectCleanupHandler) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QObjectCleanupHandler5clearEv()};
     unsafe {_ZN21QObjectCleanupHandler5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QObjectCleanupHandler {
  pub fn isEmpty<RetType, T: QObjectCleanupHandler_isEmpty<RetType>>(&mut self, value: T) -> RetType {
    return value.isEmpty(self);
    // return 1;
  }
}

pub trait QObjectCleanupHandler_isEmpty<RetType> {
  fn isEmpty(self, rsthis: &mut QObjectCleanupHandler) -> RetType;
}

// proto:  bool QObjectCleanupHandler::isEmpty();
impl<'a> /*trait*/ QObjectCleanupHandler_isEmpty<i8> for () {
  fn isEmpty(self, rsthis: &mut QObjectCleanupHandler) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QObjectCleanupHandler7isEmptyEv()};
    let mut ret = unsafe {_ZNK21QObjectCleanupHandler7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QObjectCleanupHandler {
  pub fn FreeQObjectCleanupHandler<RetType, T: QObjectCleanupHandler_FreeQObjectCleanupHandler<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQObjectCleanupHandler(self);
    // return 1;
  }
}

pub trait QObjectCleanupHandler_FreeQObjectCleanupHandler<RetType> {
  fn FreeQObjectCleanupHandler(self, rsthis: &mut QObjectCleanupHandler) -> RetType;
}

// proto:  void QObjectCleanupHandler::FreeQObjectCleanupHandler();
impl<'a> /*trait*/ QObjectCleanupHandler_FreeQObjectCleanupHandler<()> for () {
  fn FreeQObjectCleanupHandler(self, rsthis: &mut QObjectCleanupHandler) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QObjectCleanupHandlerD0Ev()};
     unsafe {_ZN21QObjectCleanupHandlerD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QObjectCleanupHandler {
  pub fn metaObject<RetType, T: QObjectCleanupHandler_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QObjectCleanupHandler_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QObjectCleanupHandler) -> RetType;
}

// proto:  const QMetaObject * QObjectCleanupHandler::metaObject();
impl<'a> /*trait*/ QObjectCleanupHandler_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QObjectCleanupHandler) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QObjectCleanupHandler10metaObjectEv()};
     unsafe {_ZNK21QObjectCleanupHandler10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QObjectCleanupHandler {
  pub fn remove<RetType, T: QObjectCleanupHandler_remove<RetType>>(&mut self, value: T) -> RetType {
    return value.remove(self);
    // return 1;
  }
}

pub trait QObjectCleanupHandler_remove<RetType> {
  fn remove(self, rsthis: &mut QObjectCleanupHandler) -> RetType;
}

// proto:  void QObjectCleanupHandler::remove(QObject * object);
impl<'a> /*trait*/ QObjectCleanupHandler_remove<()> for (&'a mut QObject) {
  fn remove(self, rsthis: &mut QObjectCleanupHandler) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QObjectCleanupHandler6removeEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN21QObjectCleanupHandler6removeEP7QObject(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QObjectCleanupHandler {
  pub fn add<RetType, T: QObjectCleanupHandler_add<RetType>>(&mut self, value: T) -> RetType {
    return value.add(self);
    // return 1;
  }
}

pub trait QObjectCleanupHandler_add<RetType> {
  fn add(self, rsthis: &mut QObjectCleanupHandler) -> RetType;
}

// proto:  QObject * QObjectCleanupHandler::add(QObject * object);
impl<'a> /*trait*/ QObjectCleanupHandler_add<QObject> for (&'a mut QObject) {
  fn add(self, rsthis: &mut QObjectCleanupHandler) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QObjectCleanupHandler3addEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN21QObjectCleanupHandler3addEP7QObject(rsthis.qclsinst, arg0)};
    let mut ret1 = QObject{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QObjectCleanupHandler {
  pub fn NewQObjectCleanupHandler<T: QObjectCleanupHandler_NewQObjectCleanupHandler>(value: T) -> QObjectCleanupHandler {
    let rsthis = value.NewQObjectCleanupHandler();
    return rsthis;
    // return 1;
  }
}

pub trait QObjectCleanupHandler_NewQObjectCleanupHandler {
  fn NewQObjectCleanupHandler(self) -> QObjectCleanupHandler;
}

// proto: void QObjectCleanupHandler::NewQObjectCleanupHandler();
impl<'a> /*trait*/ QObjectCleanupHandler_NewQObjectCleanupHandler for () {
  fn NewQObjectCleanupHandler(self) -> QObjectCleanupHandler {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QObjectCleanupHandlerC1Ev()};
    unsafe {_ZN21QObjectCleanupHandlerC1Ev(qthis)};
    let rsthis = QObjectCleanupHandler{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

