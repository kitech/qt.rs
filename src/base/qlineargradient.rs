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
  // proto: void QLinearGradient::setFinalStop(qreal x, qreal y);
  fn _ZN15QLinearGradient12setFinalStopEdd(arg0: c_double, arg1: c_double) -> i32;
  // proto: QPointF QLinearGradient::start();
  fn _ZNK15QLinearGradient5startEv() -> i32;
  // proto: void QLinearGradient::NewQLinearGradient(qreal xStart, qreal yStart, qreal xFinalStop, qreal yFinalStop);
  fn _ZN15QLinearGradientC1Edddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> i32;
  // proto: void QLinearGradient::NewQLinearGradient(const QPointF & start, const QPointF & finalStop);
  fn _ZN15QLinearGradientC1ERK7QPointFS2_(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QLinearGradient::setStart(qreal x, qreal y);
  fn _ZN15QLinearGradient8setStartEdd(arg0: c_double, arg1: c_double) -> i32;
  // proto: void QLinearGradient::setStart(const QPointF & start);
  fn _ZN15QLinearGradient8setStartERK7QPointF(arg0: *const c_void) -> i32;
  // proto: void QLinearGradient::NewQLinearGradient();
  fn _ZN15QLinearGradientC1Ev(qthis: *mut c_void) -> i32;
  // proto: QPointF QLinearGradient::finalStop();
  fn _ZNK15QLinearGradient9finalStopEv() -> i32;
  // proto: void QLinearGradient::setFinalStop(const QPointF & stop);
  fn _ZN15QLinearGradient12setFinalStopERK7QPointF(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QLinearGradient)=1
pub struct QLinearGradient {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QLinearGradient {
  pub fn setFinalStop<T: QLinearGradient_setFinalStop>(&mut self, value: T) -> i32 {
    value.setFinalStop(self);
    return 1;
  }
}

pub trait QLinearGradient_setFinalStop {
  fn setFinalStop(self, this: &mut QLinearGradient) -> i32;
}

// proto: void QLinearGradient::setFinalStop(qreal x, qreal y);
impl<'a> /*trait*/ QLinearGradient_setFinalStop for (f64, f64) {
  fn setFinalStop(self, this: &mut QLinearGradient) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QLinearGradient12setFinalStopEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN15QLinearGradient12setFinalStopEdd(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QLinearGradient {
  pub fn start<T: QLinearGradient_start>(&mut self, value: T) -> i32 {
    value.start(self);
    return 1;
  }
}

pub trait QLinearGradient_start {
  fn start(self, this: &mut QLinearGradient) -> i32;
}

// proto: QPointF QLinearGradient::start();
impl<'a> /*trait*/ QLinearGradient_start for () {
  fn start(self, this: &mut QLinearGradient) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QLinearGradient5startEv()};
    unsafe {_ZNK15QLinearGradient5startEv()};
    return 1;
  }
}

impl /*struct*/ QLinearGradient {
  pub fn NewQLinearGradient<T: QLinearGradient_NewQLinearGradient>(value: T) -> QLinearGradient {
    let rsthis = value.NewQLinearGradient();
    return rsthis;
    // return 1;
  }
}

pub trait QLinearGradient_NewQLinearGradient {
  fn NewQLinearGradient(self) -> QLinearGradient;
}

// proto: void QLinearGradient::NewQLinearGradient(qreal xStart, qreal yStart, qreal xFinalStop, qreal yFinalStop);
impl<'a> /*trait*/ QLinearGradient_NewQLinearGradient for (f64, f64, f64, f64) {
  fn NewQLinearGradient(self) -> QLinearGradient {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QLinearGradientC1Edddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZN15QLinearGradientC1Edddd(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QLinearGradient{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QLinearGradient::NewQLinearGradient(const QPointF & start, const QPointF & finalStop);
impl<'a> /*trait*/ QLinearGradient_NewQLinearGradient for (&'a  QPointF, &'a  QPointF) {
  fn NewQLinearGradient(self) -> QLinearGradient {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QLinearGradientC1ERK7QPointFS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN15QLinearGradientC1ERK7QPointFS2_(qthis, arg0, arg1)};
    let rsthis = QLinearGradient{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLinearGradient {
  pub fn setStart<T: QLinearGradient_setStart>(&mut self, value: T) -> i32 {
    value.setStart(self);
    return 1;
  }
}

pub trait QLinearGradient_setStart {
  fn setStart(self, this: &mut QLinearGradient) -> i32;
}

// proto: void QLinearGradient::setStart(qreal x, qreal y);
impl<'a> /*trait*/ QLinearGradient_setStart for (f64, f64) {
  fn setStart(self, this: &mut QLinearGradient) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QLinearGradient8setStartEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN15QLinearGradient8setStartEdd(arg0, arg1)};
    return 1;
  }
}

// proto: void QLinearGradient::setStart(const QPointF & start);
impl<'a> /*trait*/ QLinearGradient_setStart for (&'a  QPointF) {
  fn setStart(self, this: &mut QLinearGradient) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QLinearGradient8setStartERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QLinearGradient8setStartERK7QPointF(arg0)};
    return 1;
  }
}

// proto: void QLinearGradient::NewQLinearGradient();
impl<'a> /*trait*/ QLinearGradient_NewQLinearGradient for () {
  fn NewQLinearGradient(self) -> QLinearGradient {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QLinearGradientC1Ev()};
    unsafe {_ZN15QLinearGradientC1Ev(qthis)};
    let rsthis = QLinearGradient{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLinearGradient {
  pub fn finalStop<T: QLinearGradient_finalStop>(&mut self, value: T) -> i32 {
    value.finalStop(self);
    return 1;
  }
}

pub trait QLinearGradient_finalStop {
  fn finalStop(self, this: &mut QLinearGradient) -> i32;
}

// proto: QPointF QLinearGradient::finalStop();
impl<'a> /*trait*/ QLinearGradient_finalStop for () {
  fn finalStop(self, this: &mut QLinearGradient) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QLinearGradient9finalStopEv()};
    unsafe {_ZNK15QLinearGradient9finalStopEv()};
    return 1;
  }
}

// proto: void QLinearGradient::setFinalStop(const QPointF & stop);
impl<'a> /*trait*/ QLinearGradient_setFinalStop for (&'a  QPointF) {
  fn setFinalStop(self, this: &mut QLinearGradient) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QLinearGradient12setFinalStopERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QLinearGradient12setFinalStopERK7QPointF(arg0)};
    return 1;
  }
}

