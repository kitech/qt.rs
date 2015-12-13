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
  // proto: QPointF QPanGesture::offset();
  fn _ZNK11QPanGesture6offsetEv() -> i32;
  // proto: QPointF QPanGesture::delta();
  fn _ZNK11QPanGesture5deltaEv() -> i32;
  // proto: void QPanGesture::setOffset(const QPointF & value);
  fn _ZN11QPanGesture9setOffsetERK7QPointF(arg0: *const c_void) -> i32;
  // proto: double QPanGesture::acceleration();
  fn _ZNK11QPanGesture12accelerationEv() -> i32;
  // proto: void QPanGesture::FreeQPanGesture();
  fn _ZN11QPanGestureD0Ev() -> i32;
  // proto: void QPanGesture::NewQPanGesture(QObject * parent);
  fn _ZN11QPanGestureC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: const QMetaObject * QPanGesture::metaObject();
  fn _ZNK11QPanGesture10metaObjectEv() -> i32;
  // proto: void QPanGesture::setAcceleration(qreal value);
  fn _ZN11QPanGesture15setAccelerationEd(arg0: c_double) -> i32;
  // proto: QPointF QPanGesture::lastOffset();
  fn _ZNK11QPanGesture10lastOffsetEv() -> i32;
  // proto: void QPanGesture::setLastOffset(const QPointF & value);
  fn _ZN11QPanGesture13setLastOffsetERK7QPointF(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QPanGesture)=1
pub struct QPanGesture {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPanGesture {
  pub fn offset<T: QPanGesture_offset>(&mut self, value: T) -> i32 {
    value.offset(self);
    return 1;
  }
}

pub trait QPanGesture_offset {
  fn offset(self, this: &mut QPanGesture) -> i32;
}

// proto: QPointF QPanGesture::offset();
impl<'a> /*trait*/ QPanGesture_offset for () {
  fn offset(self, this: &mut QPanGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPanGesture6offsetEv()};
    unsafe {_ZNK11QPanGesture6offsetEv()};
    return 1;
  }
}

impl /*struct*/ QPanGesture {
  pub fn delta<T: QPanGesture_delta>(&mut self, value: T) -> i32 {
    value.delta(self);
    return 1;
  }
}

pub trait QPanGesture_delta {
  fn delta(self, this: &mut QPanGesture) -> i32;
}

// proto: QPointF QPanGesture::delta();
impl<'a> /*trait*/ QPanGesture_delta for () {
  fn delta(self, this: &mut QPanGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPanGesture5deltaEv()};
    unsafe {_ZNK11QPanGesture5deltaEv()};
    return 1;
  }
}

impl /*struct*/ QPanGesture {
  pub fn setOffset<T: QPanGesture_setOffset>(&mut self, value: T) -> i32 {
    value.setOffset(self);
    return 1;
  }
}

pub trait QPanGesture_setOffset {
  fn setOffset(self, this: &mut QPanGesture) -> i32;
}

// proto: void QPanGesture::setOffset(const QPointF & value);
impl<'a> /*trait*/ QPanGesture_setOffset for (&'a  QPointF) {
  fn setOffset(self, this: &mut QPanGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPanGesture9setOffsetERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QPanGesture9setOffsetERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QPanGesture {
  pub fn acceleration<T: QPanGesture_acceleration>(&mut self, value: T) -> i32 {
    value.acceleration(self);
    return 1;
  }
}

pub trait QPanGesture_acceleration {
  fn acceleration(self, this: &mut QPanGesture) -> i32;
}

// proto: double QPanGesture::acceleration();
impl<'a> /*trait*/ QPanGesture_acceleration for () {
  fn acceleration(self, this: &mut QPanGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPanGesture12accelerationEv()};
    unsafe {_ZNK11QPanGesture12accelerationEv()};
    return 1;
  }
}

impl /*struct*/ QPanGesture {
  pub fn FreeQPanGesture<T: QPanGesture_FreeQPanGesture>(&mut self, value: T) -> i32 {
    value.FreeQPanGesture(self);
    return 1;
  }
}

pub trait QPanGesture_FreeQPanGesture {
  fn FreeQPanGesture(self, this: &mut QPanGesture) -> i32;
}

// proto: void QPanGesture::FreeQPanGesture();
impl<'a> /*trait*/ QPanGesture_FreeQPanGesture for () {
  fn FreeQPanGesture(self, this: &mut QPanGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPanGestureD0Ev()};
    unsafe {_ZN11QPanGestureD0Ev()};
    return 1;
  }
}

impl /*struct*/ QPanGesture {
  pub fn NewQPanGesture<T: QPanGesture_NewQPanGesture>(value: T) -> QPanGesture {
    let rsthis = value.NewQPanGesture();
    return rsthis;
    // return 1;
  }
}

pub trait QPanGesture_NewQPanGesture {
  fn NewQPanGesture(self) -> QPanGesture;
}

// proto: void QPanGesture::NewQPanGesture(QObject * parent);
impl<'a> /*trait*/ QPanGesture_NewQPanGesture for (&'a mut QObject) {
  fn NewQPanGesture(self) -> QPanGesture {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPanGestureC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QPanGestureC1EP7QObject(qthis, arg0)};
    let rsthis = QPanGesture{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPanGesture {
  pub fn metaObject<T: QPanGesture_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QPanGesture_metaObject {
  fn metaObject(self, this: &mut QPanGesture) -> i32;
}

// proto: const QMetaObject * QPanGesture::metaObject();
impl<'a> /*trait*/ QPanGesture_metaObject for () {
  fn metaObject(self, this: &mut QPanGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPanGesture10metaObjectEv()};
    unsafe {_ZNK11QPanGesture10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QPanGesture {
  pub fn setAcceleration<T: QPanGesture_setAcceleration>(&mut self, value: T) -> i32 {
    value.setAcceleration(self);
    return 1;
  }
}

pub trait QPanGesture_setAcceleration {
  fn setAcceleration(self, this: &mut QPanGesture) -> i32;
}

// proto: void QPanGesture::setAcceleration(qreal value);
impl<'a> /*trait*/ QPanGesture_setAcceleration for (f64) {
  fn setAcceleration(self, this: &mut QPanGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPanGesture15setAccelerationEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN11QPanGesture15setAccelerationEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QPanGesture {
  pub fn lastOffset<T: QPanGesture_lastOffset>(&mut self, value: T) -> i32 {
    value.lastOffset(self);
    return 1;
  }
}

pub trait QPanGesture_lastOffset {
  fn lastOffset(self, this: &mut QPanGesture) -> i32;
}

// proto: QPointF QPanGesture::lastOffset();
impl<'a> /*trait*/ QPanGesture_lastOffset for () {
  fn lastOffset(self, this: &mut QPanGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPanGesture10lastOffsetEv()};
    unsafe {_ZNK11QPanGesture10lastOffsetEv()};
    return 1;
  }
}

impl /*struct*/ QPanGesture {
  pub fn setLastOffset<T: QPanGesture_setLastOffset>(&mut self, value: T) -> i32 {
    value.setLastOffset(self);
    return 1;
  }
}

pub trait QPanGesture_setLastOffset {
  fn setLastOffset(self, this: &mut QPanGesture) -> i32;
}

// proto: void QPanGesture::setLastOffset(const QPointF & value);
impl<'a> /*trait*/ QPanGesture_setLastOffset for (&'a  QPointF) {
  fn setLastOffset(self, this: &mut QPanGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPanGesture13setLastOffsetERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QPanGesture13setLastOffsetERK7QPointF(arg0)};
    return 1;
  }
}

