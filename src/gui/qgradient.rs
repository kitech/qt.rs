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
  // proto:  void QGradient::setColorAt(qreal pos, const QColor & color);
  fn _ZN9QGradient10setColorAtEdRK6QColor(qthis: *mut c_void, arg0: c_double, arg1: *mut c_void);
  // proto:  QGradientStops QGradient::stops();
  fn _ZNK9QGradient5stopsEv(qthis: *mut c_void);
  // proto:  void QGradient::QGradient();
  fn _ZN9QGradientC1Ev(qthis: *mut c_void);
}

// body block begin
// class sizeof(QGradient)=1
pub struct QGradient {
  pub qclsinst: *mut c_void,
}

  // proto:  void QGradient::setColorAt(qreal pos, const QColor & color);
impl /*struct*/ QGradient {
  pub fn setColorAt<RetType, T: QGradient_setColorAt<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setColorAt(self);
    // return 1;
  }
}

pub trait QGradient_setColorAt<RetType> {
  fn setColorAt(self , rsthis: &mut QGradient) -> RetType;
}

  // proto:  void QGradient::setColorAt(qreal pos, const QColor & color);
impl<'a> /*trait*/ QGradient_setColorAt<()> for (f64, QColor) {
  fn setColorAt(self , rsthis: &mut QGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGradient10setColorAtEdRK6QColor()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN9QGradient10setColorAtEdRK6QColor(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QGradientStops QGradient::stops();
impl /*struct*/ QGradient {
  pub fn stops<RetType, T: QGradient_stops<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.stops(self);
    // return 1;
  }
}

pub trait QGradient_stops<RetType> {
  fn stops(self , rsthis: &mut QGradient) -> RetType;
}

  // proto:  QGradientStops QGradient::stops();
impl<'a> /*trait*/ QGradient_stops<()> for () {
  fn stops(self , rsthis: &mut QGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGradient5stopsEv()};
     unsafe {_ZNK9QGradient5stopsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGradient::QGradient();
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

  // proto:  void QGradient::QGradient();
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

