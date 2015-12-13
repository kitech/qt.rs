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
  fn _ZN21QObjectCleanupHandler5clearEv() -> i32;
  fn _ZNK21QObjectCleanupHandler7isEmptyEv() -> i32;
  fn _ZN21QObjectCleanupHandlerD0Ev() -> i32;
  fn _ZNK21QObjectCleanupHandler10metaObjectEv() -> i32;
  fn _ZN21QObjectCleanupHandler6removeEP7QObject(arg0: *mut c_void) -> i32;
  fn _ZN21QObjectCleanupHandler3addEP7QObject(arg0: *mut c_void) -> i32;
  fn _ZN21QObjectCleanupHandlerC1Ev(qthis: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QObjectCleanupHandler)=1
pub struct QObjectCleanupHandler {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QObjectCleanupHandler {
  pub fn clear<T: QObjectCleanupHandler_clear>(&mut self, value: T) -> i32 {
    value.clear(self);
    return 1;
  }
}

pub trait QObjectCleanupHandler_clear {
  fn clear(self, this: &mut QObjectCleanupHandler) -> i32;
}

// proto: void QObjectCleanupHandler::clear();
impl<'a> /*trait*/ QObjectCleanupHandler_clear for () {
  fn clear(self, this: &mut QObjectCleanupHandler) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QObjectCleanupHandler5clearEv()};
    unsafe {_ZN21QObjectCleanupHandler5clearEv()};
    return 1;
  }
}

impl /*struct*/ QObjectCleanupHandler {
  pub fn isEmpty<T: QObjectCleanupHandler_isEmpty>(&mut self, value: T) -> i32 {
    value.isEmpty(self);
    return 1;
  }
}

pub trait QObjectCleanupHandler_isEmpty {
  fn isEmpty(self, this: &mut QObjectCleanupHandler) -> i32;
}

// proto: bool QObjectCleanupHandler::isEmpty();
impl<'a> /*trait*/ QObjectCleanupHandler_isEmpty for () {
  fn isEmpty(self, this: &mut QObjectCleanupHandler) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QObjectCleanupHandler7isEmptyEv()};
    unsafe {_ZNK21QObjectCleanupHandler7isEmptyEv()};
    return 1;
  }
}

impl /*struct*/ QObjectCleanupHandler {
  pub fn FreeQObjectCleanupHandler<T: QObjectCleanupHandler_FreeQObjectCleanupHandler>(&mut self, value: T) -> i32 {
    value.FreeQObjectCleanupHandler(self);
    return 1;
  }
}

pub trait QObjectCleanupHandler_FreeQObjectCleanupHandler {
  fn FreeQObjectCleanupHandler(self, this: &mut QObjectCleanupHandler) -> i32;
}

// proto: void QObjectCleanupHandler::FreeQObjectCleanupHandler();
impl<'a> /*trait*/ QObjectCleanupHandler_FreeQObjectCleanupHandler for () {
  fn FreeQObjectCleanupHandler(self, this: &mut QObjectCleanupHandler) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QObjectCleanupHandlerD0Ev()};
    unsafe {_ZN21QObjectCleanupHandlerD0Ev()};
    return 1;
  }
}

impl /*struct*/ QObjectCleanupHandler {
  pub fn metaObject<T: QObjectCleanupHandler_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QObjectCleanupHandler_metaObject {
  fn metaObject(self, this: &mut QObjectCleanupHandler) -> i32;
}

// proto: const QMetaObject * QObjectCleanupHandler::metaObject();
impl<'a> /*trait*/ QObjectCleanupHandler_metaObject for () {
  fn metaObject(self, this: &mut QObjectCleanupHandler) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QObjectCleanupHandler10metaObjectEv()};
    unsafe {_ZNK21QObjectCleanupHandler10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QObjectCleanupHandler {
  pub fn remove<T: QObjectCleanupHandler_remove>(&mut self, value: T) -> i32 {
    value.remove(self);
    return 1;
  }
}

pub trait QObjectCleanupHandler_remove {
  fn remove(self, this: &mut QObjectCleanupHandler) -> i32;
}

// proto: void QObjectCleanupHandler::remove(QObject * object);
impl<'a> /*trait*/ QObjectCleanupHandler_remove for (&'a mut QObject) {
  fn remove(self, this: &mut QObjectCleanupHandler) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QObjectCleanupHandler6removeEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN21QObjectCleanupHandler6removeEP7QObject(arg0)};
    return 1;
  }
}

impl /*struct*/ QObjectCleanupHandler {
  pub fn add<T: QObjectCleanupHandler_add>(&mut self, value: T) -> i32 {
    value.add(self);
    return 1;
  }
}

pub trait QObjectCleanupHandler_add {
  fn add(self, this: &mut QObjectCleanupHandler) -> i32;
}

// proto: QObject * QObjectCleanupHandler::add(QObject * object);
impl<'a> /*trait*/ QObjectCleanupHandler_add for (&'a mut QObject) {
  fn add(self, this: &mut QObjectCleanupHandler) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QObjectCleanupHandler3addEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN21QObjectCleanupHandler3addEP7QObject(arg0)};
    return 1;
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

