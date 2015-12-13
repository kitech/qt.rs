// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpointf::QPointF;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QPointF QTapGesture::position();
  fn _ZNK11QTapGesture8positionEv() -> i32;
  // proto: void QTapGesture::setPosition(const QPointF & pos);
  fn _ZN11QTapGesture11setPositionERK7QPointF(arg0: *const c_void) -> i32;
  // proto: void QTapGesture::NewQTapGesture(QObject * parent);
  fn _ZN11QTapGestureC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: const QMetaObject * QTapGesture::metaObject();
  fn _ZNK11QTapGesture10metaObjectEv() -> i32;
  // proto: void QTapGesture::FreeQTapGesture();
  fn _ZN11QTapGestureD0Ev() -> i32;
}

// body block begin
// class sizeof(QTapGesture)=1
pub struct QTapGesture {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTapGesture {
  pub fn position<T: QTapGesture_position>(&mut self, value: T) -> i32 {
    value.position(self);
    return 1;
  }
}

pub trait QTapGesture_position {
  fn position(self, this: &mut QTapGesture) -> i32;
}

// proto: QPointF QTapGesture::position();
impl<'a> /*trait*/ QTapGesture_position for () {
  fn position(self, this: &mut QTapGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTapGesture8positionEv()};
    unsafe {_ZNK11QTapGesture8positionEv()};
    return 1;
  }
}

impl /*struct*/ QTapGesture {
  pub fn setPosition<T: QTapGesture_setPosition>(&mut self, value: T) -> i32 {
    value.setPosition(self);
    return 1;
  }
}

pub trait QTapGesture_setPosition {
  fn setPosition(self, this: &mut QTapGesture) -> i32;
}

// proto: void QTapGesture::setPosition(const QPointF & pos);
impl<'a> /*trait*/ QTapGesture_setPosition for (&'a  QPointF) {
  fn setPosition(self, this: &mut QTapGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTapGesture11setPositionERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QTapGesture11setPositionERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QTapGesture {
  pub fn NewQTapGesture<T: QTapGesture_NewQTapGesture>(value: T) -> QTapGesture {
    let rsthis = value.NewQTapGesture();
    return rsthis;
    // return 1;
  }
}

pub trait QTapGesture_NewQTapGesture {
  fn NewQTapGesture(self) -> QTapGesture;
}

// proto: void QTapGesture::NewQTapGesture(QObject * parent);
impl<'a> /*trait*/ QTapGesture_NewQTapGesture for (&'a mut QObject) {
  fn NewQTapGesture(self) -> QTapGesture {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTapGestureC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QTapGestureC1EP7QObject(qthis, arg0)};
    let rsthis = QTapGesture{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTapGesture {
  pub fn metaObject<T: QTapGesture_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QTapGesture_metaObject {
  fn metaObject(self, this: &mut QTapGesture) -> i32;
}

// proto: const QMetaObject * QTapGesture::metaObject();
impl<'a> /*trait*/ QTapGesture_metaObject for () {
  fn metaObject(self, this: &mut QTapGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTapGesture10metaObjectEv()};
    unsafe {_ZNK11QTapGesture10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QTapGesture {
  pub fn FreeQTapGesture<T: QTapGesture_FreeQTapGesture>(&mut self, value: T) -> i32 {
    value.FreeQTapGesture(self);
    return 1;
  }
}

pub trait QTapGesture_FreeQTapGesture {
  fn FreeQTapGesture(self, this: &mut QTapGesture) -> i32;
}

// proto: void QTapGesture::FreeQTapGesture();
impl<'a> /*trait*/ QTapGesture_FreeQTapGesture for () {
  fn FreeQTapGesture(self, this: &mut QTapGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTapGestureD0Ev()};
    unsafe {_ZN11QTapGestureD0Ev()};
    return 1;
  }
}

