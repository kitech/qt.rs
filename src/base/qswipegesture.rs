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
  // proto: void QSwipeGesture::FreeQSwipeGesture();
  fn _ZN13QSwipeGestureD0Ev() -> i32;
  // proto: const QMetaObject * QSwipeGesture::metaObject();
  fn _ZNK13QSwipeGesture10metaObjectEv() -> i32;
  // proto: void QSwipeGesture::setSwipeAngle(qreal value);
  fn _ZN13QSwipeGesture13setSwipeAngleEd(arg0: c_double) -> i32;
  // proto: void QSwipeGesture::NewQSwipeGesture(QObject * parent);
  fn _ZN13QSwipeGestureC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: double QSwipeGesture::swipeAngle();
  fn _ZNK13QSwipeGesture10swipeAngleEv() -> i32;
}

// body block begin
// class sizeof(QSwipeGesture)=1
pub struct QSwipeGesture {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSwipeGesture {
  pub fn FreeQSwipeGesture<T: QSwipeGesture_FreeQSwipeGesture>(&mut self, value: T) -> i32 {
    value.FreeQSwipeGesture(self);
    return 1;
  }
}

pub trait QSwipeGesture_FreeQSwipeGesture {
  fn FreeQSwipeGesture(self, this: &mut QSwipeGesture) -> i32;
}

// proto: void QSwipeGesture::FreeQSwipeGesture();
impl<'a> /*trait*/ QSwipeGesture_FreeQSwipeGesture for () {
  fn FreeQSwipeGesture(self, this: &mut QSwipeGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSwipeGestureD0Ev()};
    unsafe {_ZN13QSwipeGestureD0Ev()};
    return 1;
  }
}

impl /*struct*/ QSwipeGesture {
  pub fn metaObject<T: QSwipeGesture_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QSwipeGesture_metaObject {
  fn metaObject(self, this: &mut QSwipeGesture) -> i32;
}

// proto: const QMetaObject * QSwipeGesture::metaObject();
impl<'a> /*trait*/ QSwipeGesture_metaObject for () {
  fn metaObject(self, this: &mut QSwipeGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSwipeGesture10metaObjectEv()};
    unsafe {_ZNK13QSwipeGesture10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QSwipeGesture {
  pub fn setSwipeAngle<T: QSwipeGesture_setSwipeAngle>(&mut self, value: T) -> i32 {
    value.setSwipeAngle(self);
    return 1;
  }
}

pub trait QSwipeGesture_setSwipeAngle {
  fn setSwipeAngle(self, this: &mut QSwipeGesture) -> i32;
}

// proto: void QSwipeGesture::setSwipeAngle(qreal value);
impl<'a> /*trait*/ QSwipeGesture_setSwipeAngle for (f64) {
  fn setSwipeAngle(self, this: &mut QSwipeGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSwipeGesture13setSwipeAngleEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN13QSwipeGesture13setSwipeAngleEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QSwipeGesture {
  pub fn NewQSwipeGesture<T: QSwipeGesture_NewQSwipeGesture>(value: T) -> QSwipeGesture {
    let rsthis = value.NewQSwipeGesture();
    return rsthis;
    // return 1;
  }
}

pub trait QSwipeGesture_NewQSwipeGesture {
  fn NewQSwipeGesture(self) -> QSwipeGesture;
}

// proto: void QSwipeGesture::NewQSwipeGesture(QObject * parent);
impl<'a> /*trait*/ QSwipeGesture_NewQSwipeGesture for (&'a mut QObject) {
  fn NewQSwipeGesture(self) -> QSwipeGesture {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSwipeGestureC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QSwipeGestureC1EP7QObject(qthis, arg0)};
    let rsthis = QSwipeGesture{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSwipeGesture {
  pub fn swipeAngle<T: QSwipeGesture_swipeAngle>(&mut self, value: T) -> i32 {
    value.swipeAngle(self);
    return 1;
  }
}

pub trait QSwipeGesture_swipeAngle {
  fn swipeAngle(self, this: &mut QSwipeGesture) -> i32;
}

// proto: double QSwipeGesture::swipeAngle();
impl<'a> /*trait*/ QSwipeGesture_swipeAngle for () {
  fn swipeAngle(self, this: &mut QSwipeGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSwipeGesture10swipeAngleEv()};
    unsafe {_ZNK13QSwipeGesture10swipeAngleEv()};
    return 1;
  }
}

