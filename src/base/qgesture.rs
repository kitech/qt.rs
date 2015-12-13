// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;
use super::qpointf::QPointF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QPointF QGesture::hotSpot();
  fn _ZNK8QGesture7hotSpotEv() -> i32;
  // proto: bool QGesture::hasHotSpot();
  fn _ZNK8QGesture10hasHotSpotEv() -> i32;
  // proto: void QGesture::unsetHotSpot();
  fn _ZN8QGesture12unsetHotSpotEv() -> i32;
  // proto: const QMetaObject * QGesture::metaObject();
  fn _ZNK8QGesture10metaObjectEv() -> i32;
  // proto: void QGesture::NewQGesture(QObject * parent);
  fn _ZN8QGestureC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QGesture::setHotSpot(const QPointF & value);
  fn _ZN8QGesture10setHotSpotERK7QPointF(arg0: *const c_void) -> i32;
  // proto: void QGesture::FreeQGesture();
  fn _ZN8QGestureD0Ev() -> i32;
}

// body block begin
// class sizeof(QGesture)=1
pub struct QGesture {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGesture {
  pub fn hotSpot<T: QGesture_hotSpot>(&mut self, value: T) -> i32 {
    value.hotSpot(self);
    return 1;
  }
}

pub trait QGesture_hotSpot {
  fn hotSpot(self, this: &mut QGesture) -> i32;
}

// proto: QPointF QGesture::hotSpot();
impl<'a> /*trait*/ QGesture_hotSpot for () {
  fn hotSpot(self, this: &mut QGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QGesture7hotSpotEv()};
    unsafe {_ZNK8QGesture7hotSpotEv()};
    return 1;
  }
}

impl /*struct*/ QGesture {
  pub fn hasHotSpot<T: QGesture_hasHotSpot>(&mut self, value: T) -> i32 {
    value.hasHotSpot(self);
    return 1;
  }
}

pub trait QGesture_hasHotSpot {
  fn hasHotSpot(self, this: &mut QGesture) -> i32;
}

// proto: bool QGesture::hasHotSpot();
impl<'a> /*trait*/ QGesture_hasHotSpot for () {
  fn hasHotSpot(self, this: &mut QGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QGesture10hasHotSpotEv()};
    unsafe {_ZNK8QGesture10hasHotSpotEv()};
    return 1;
  }
}

impl /*struct*/ QGesture {
  pub fn unsetHotSpot<T: QGesture_unsetHotSpot>(&mut self, value: T) -> i32 {
    value.unsetHotSpot(self);
    return 1;
  }
}

pub trait QGesture_unsetHotSpot {
  fn unsetHotSpot(self, this: &mut QGesture) -> i32;
}

// proto: void QGesture::unsetHotSpot();
impl<'a> /*trait*/ QGesture_unsetHotSpot for () {
  fn unsetHotSpot(self, this: &mut QGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QGesture12unsetHotSpotEv()};
    unsafe {_ZN8QGesture12unsetHotSpotEv()};
    return 1;
  }
}

impl /*struct*/ QGesture {
  pub fn metaObject<T: QGesture_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QGesture_metaObject {
  fn metaObject(self, this: &mut QGesture) -> i32;
}

// proto: const QMetaObject * QGesture::metaObject();
impl<'a> /*trait*/ QGesture_metaObject for () {
  fn metaObject(self, this: &mut QGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QGesture10metaObjectEv()};
    unsafe {_ZNK8QGesture10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QGesture {
  pub fn NewQGesture<T: QGesture_NewQGesture>(value: T) -> QGesture {
    let rsthis = value.NewQGesture();
    return rsthis;
    // return 1;
  }
}

pub trait QGesture_NewQGesture {
  fn NewQGesture(self) -> QGesture;
}

// proto: void QGesture::NewQGesture(QObject * parent);
impl<'a> /*trait*/ QGesture_NewQGesture for (&'a mut QObject) {
  fn NewQGesture(self) -> QGesture {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QGestureC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QGestureC1EP7QObject(qthis, arg0)};
    let rsthis = QGesture{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGesture {
  pub fn setHotSpot<T: QGesture_setHotSpot>(&mut self, value: T) -> i32 {
    value.setHotSpot(self);
    return 1;
  }
}

pub trait QGesture_setHotSpot {
  fn setHotSpot(self, this: &mut QGesture) -> i32;
}

// proto: void QGesture::setHotSpot(const QPointF & value);
impl<'a> /*trait*/ QGesture_setHotSpot for (&'a  QPointF) {
  fn setHotSpot(self, this: &mut QGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QGesture10setHotSpotERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QGesture10setHotSpotERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGesture {
  pub fn FreeQGesture<T: QGesture_FreeQGesture>(&mut self, value: T) -> i32 {
    value.FreeQGesture(self);
    return 1;
  }
}

pub trait QGesture_FreeQGesture {
  fn FreeQGesture(self, this: &mut QGesture) -> i32;
}

// proto: void QGesture::FreeQGesture();
impl<'a> /*trait*/ QGesture_FreeQGesture for () {
  fn FreeQGesture(self, this: &mut QGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QGestureD0Ev()};
    unsafe {_ZN8QGestureD0Ev()};
    return 1;
  }
}

