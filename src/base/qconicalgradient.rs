// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpointf::QPointF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: double QConicalGradient::angle();
  fn _ZNK16QConicalGradient5angleEv() -> i32;
  // proto: QPointF QConicalGradient::center();
  fn _ZNK16QConicalGradient6centerEv() -> i32;
  // proto: void QConicalGradient::NewQConicalGradient(const QPointF & center, qreal startAngle);
  fn _ZN16QConicalGradientC1ERK7QPointFd(qthis: *mut c_void, arg0: *const c_void, arg1: c_double) -> i32;
  // proto: void QConicalGradient::NewQConicalGradient();
  fn _ZN16QConicalGradientC1Ev(qthis: *mut c_void) -> i32;
  // proto: void QConicalGradient::setAngle(qreal angle);
  fn _ZN16QConicalGradient8setAngleEd(arg0: c_double) -> i32;
  // proto: void QConicalGradient::setCenter(qreal x, qreal y);
  fn _ZN16QConicalGradient9setCenterEdd(arg0: c_double, arg1: c_double) -> i32;
  // proto: void QConicalGradient::setCenter(const QPointF & center);
  fn _ZN16QConicalGradient9setCenterERK7QPointF(arg0: *const c_void) -> i32;
  // proto: void QConicalGradient::NewQConicalGradient(qreal cx, qreal cy, qreal startAngle);
  fn _ZN16QConicalGradientC1Eddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double) -> i32;
}

// body block begin
// class sizeof(QConicalGradient)=1
pub struct QConicalGradient {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QConicalGradient {
  pub fn angle<T: QConicalGradient_angle>(&mut self, value: T) -> i32 {
    value.angle(self);
    return 1;
  }
}

pub trait QConicalGradient_angle {
  fn angle(self, this: &mut QConicalGradient) -> i32;
}

// proto: double QConicalGradient::angle();
impl<'a> /*trait*/ QConicalGradient_angle for () {
  fn angle(self, this: &mut QConicalGradient) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QConicalGradient5angleEv()};
    unsafe {_ZNK16QConicalGradient5angleEv()};
    return 1;
  }
}

impl /*struct*/ QConicalGradient {
  pub fn center<T: QConicalGradient_center>(&mut self, value: T) -> i32 {
    value.center(self);
    return 1;
  }
}

pub trait QConicalGradient_center {
  fn center(self, this: &mut QConicalGradient) -> i32;
}

// proto: QPointF QConicalGradient::center();
impl<'a> /*trait*/ QConicalGradient_center for () {
  fn center(self, this: &mut QConicalGradient) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QConicalGradient6centerEv()};
    unsafe {_ZNK16QConicalGradient6centerEv()};
    return 1;
  }
}

impl /*struct*/ QConicalGradient {
  pub fn NewQConicalGradient<T: QConicalGradient_NewQConicalGradient>(value: T) -> QConicalGradient {
    let rsthis = value.NewQConicalGradient();
    return rsthis;
    // return 1;
  }
}

pub trait QConicalGradient_NewQConicalGradient {
  fn NewQConicalGradient(self) -> QConicalGradient;
}

// proto: void QConicalGradient::NewQConicalGradient(const QPointF & center, qreal startAngle);
impl<'a> /*trait*/ QConicalGradient_NewQConicalGradient for (&'a  QPointF, f64) {
  fn NewQConicalGradient(self) -> QConicalGradient {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QConicalGradientC1ERK7QPointFd()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_double;
    unsafe {_ZN16QConicalGradientC1ERK7QPointFd(qthis, arg0, arg1)};
    let rsthis = QConicalGradient{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QConicalGradient::NewQConicalGradient();
impl<'a> /*trait*/ QConicalGradient_NewQConicalGradient for () {
  fn NewQConicalGradient(self) -> QConicalGradient {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QConicalGradientC1Ev()};
    unsafe {_ZN16QConicalGradientC1Ev(qthis)};
    let rsthis = QConicalGradient{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QConicalGradient {
  pub fn setAngle<T: QConicalGradient_setAngle>(&mut self, value: T) -> i32 {
    value.setAngle(self);
    return 1;
  }
}

pub trait QConicalGradient_setAngle {
  fn setAngle(self, this: &mut QConicalGradient) -> i32;
}

// proto: void QConicalGradient::setAngle(qreal angle);
impl<'a> /*trait*/ QConicalGradient_setAngle for (f64) {
  fn setAngle(self, this: &mut QConicalGradient) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QConicalGradient8setAngleEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN16QConicalGradient8setAngleEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QConicalGradient {
  pub fn setCenter<T: QConicalGradient_setCenter>(&mut self, value: T) -> i32 {
    value.setCenter(self);
    return 1;
  }
}

pub trait QConicalGradient_setCenter {
  fn setCenter(self, this: &mut QConicalGradient) -> i32;
}

// proto: void QConicalGradient::setCenter(qreal x, qreal y);
impl<'a> /*trait*/ QConicalGradient_setCenter for (f64, f64) {
  fn setCenter(self, this: &mut QConicalGradient) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QConicalGradient9setCenterEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN16QConicalGradient9setCenterEdd(arg0, arg1)};
    return 1;
  }
}

// proto: void QConicalGradient::setCenter(const QPointF & center);
impl<'a> /*trait*/ QConicalGradient_setCenter for (&'a  QPointF) {
  fn setCenter(self, this: &mut QConicalGradient) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QConicalGradient9setCenterERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QConicalGradient9setCenterERK7QPointF(arg0)};
    return 1;
  }
}

// proto: void QConicalGradient::NewQConicalGradient(qreal cx, qreal cy, qreal startAngle);
impl<'a> /*trait*/ QConicalGradient_NewQConicalGradient for (f64, f64, f64) {
  fn NewQConicalGradient(self) -> QConicalGradient {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QConicalGradientC1Eddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    unsafe {_ZN16QConicalGradientC1Eddd(qthis, arg0, arg1, arg2)};
    let rsthis = QConicalGradient{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

