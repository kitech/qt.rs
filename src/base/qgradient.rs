// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qcolor::QColor;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QGradient::setColorAt(qreal pos, const QColor & color);
  fn _ZN9QGradient10setColorAtEdRK6QColor(arg0: c_double, arg1: *const c_void) -> i32;
  // proto: QVector<QGradientStop> QGradient::stops();
  fn _ZNK9QGradient5stopsEv() -> i32;
  // proto: void QGradient::NewQGradient();
  fn _ZN9QGradientC1Ev(qthis: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QGradient)=1
pub struct QGradient {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGradient {
  pub fn setColorAt<T: QGradient_setColorAt>(&mut self, value: T) -> i32 {
    value.setColorAt(self);
    return 1;
  }
}

pub trait QGradient_setColorAt {
  fn setColorAt(self, this: &mut QGradient) -> i32;
}

// proto: void QGradient::setColorAt(qreal pos, const QColor & color);
impl<'a> /*trait*/ QGradient_setColorAt for (f64, &'a  QColor) {
  fn setColorAt(self, this: &mut QGradient) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGradient10setColorAtEdRK6QColor()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN9QGradient10setColorAtEdRK6QColor(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGradient {
  pub fn stops<T: QGradient_stops>(&mut self, value: T) -> i32 {
    value.stops(self);
    return 1;
  }
}

pub trait QGradient_stops {
  fn stops(self, this: &mut QGradient) -> i32;
}

// proto: QVector<QGradientStop> QGradient::stops();
impl<'a> /*trait*/ QGradient_stops for () {
  fn stops(self, this: &mut QGradient) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGradient5stopsEv()};
    unsafe {_ZNK9QGradient5stopsEv()};
    return 1;
  }
}

impl /*struct*/ QGradient {
  pub fn NewQGradient<T: QGradient_NewQGradient>(value: T) -> QGradient {
    let rsthis = value.NewQGradient();
    return rsthis;
    // return 1;
  }
}

pub trait QGradient_NewQGradient {
  fn NewQGradient(self) -> QGradient;
}

// proto: void QGradient::NewQGradient();
impl<'a> /*trait*/ QGradient_NewQGradient for () {
  fn NewQGradient(self) -> QGradient {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGradientC1Ev()};
    unsafe {_ZN9QGradientC1Ev(qthis)};
    let rsthis = QGradient{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

