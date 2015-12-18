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
  // proto:  double QConicalGradient::angle();
  fn _ZNK16QConicalGradient5angleEv(qthis: *mut c_void) -> c_double;
  // proto:  QPointF QConicalGradient::center();
  fn _ZNK16QConicalGradient6centerEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QConicalGradient::NewQConicalGradient(const QPointF & center, qreal startAngle);
  fn _ZN16QConicalGradientC1ERK7QPointFd(qthis: *mut c_void, arg0: *mut c_void, arg1: c_double) ;
  // proto:  void QConicalGradient::NewQConicalGradient();
  fn _ZN16QConicalGradientC1Ev(qthis: *mut c_void) ;
  // proto:  void QConicalGradient::setAngle(qreal angle);
  fn _ZN16QConicalGradient8setAngleEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QConicalGradient::setCenter(qreal x, qreal y);
  fn _ZN16QConicalGradient9setCenterEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) ;
  // proto:  void QConicalGradient::setCenter(const QPointF & center);
  fn _ZN16QConicalGradient9setCenterERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QConicalGradient::NewQConicalGradient(qreal cx, qreal cy, qreal startAngle);
  fn _ZN16QConicalGradientC1Eddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double) ;
}

// body block begin
// class sizeof(QConicalGradient)=1
pub struct QConicalGradient {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QConicalGradient {
  pub fn angle<RetType, T: QConicalGradient_angle<RetType>>(&mut self, value: T) -> RetType {
    return value.angle(self);
    // return 1;
  }
}

pub trait QConicalGradient_angle<RetType> {
  fn angle(self, rsthis: &mut QConicalGradient) -> RetType;
}

// proto:  double QConicalGradient::angle();
impl<'a> /*trait*/ QConicalGradient_angle<f64> for () {
  fn angle(self, rsthis: &mut QConicalGradient) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QConicalGradient5angleEv()};
    let mut ret = unsafe {_ZNK16QConicalGradient5angleEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QConicalGradient {
  pub fn center<RetType, T: QConicalGradient_center<RetType>>(&mut self, value: T) -> RetType {
    return value.center(self);
    // return 1;
  }
}

pub trait QConicalGradient_center<RetType> {
  fn center(self, rsthis: &mut QConicalGradient) -> RetType;
}

// proto:  QPointF QConicalGradient::center();
impl<'a> /*trait*/ QConicalGradient_center<QPointF> for () {
  fn center(self, rsthis: &mut QConicalGradient) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QConicalGradient6centerEv()};
    let mut ret = unsafe {_ZNK16QConicalGradient6centerEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
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
    let arg0 = self.0.qclsinst  as *mut c_void;
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
  pub fn setAngle<RetType, T: QConicalGradient_setAngle<RetType>>(&mut self, value: T) -> RetType {
    return value.setAngle(self);
    // return 1;
  }
}

pub trait QConicalGradient_setAngle<RetType> {
  fn setAngle(self, rsthis: &mut QConicalGradient) -> RetType;
}

// proto:  void QConicalGradient::setAngle(qreal angle);
impl<'a> /*trait*/ QConicalGradient_setAngle<()> for (f64) {
  fn setAngle(self, rsthis: &mut QConicalGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QConicalGradient8setAngleEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QConicalGradient8setAngleEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QConicalGradient {
  pub fn setCenter<RetType, T: QConicalGradient_setCenter<RetType>>(&mut self, value: T) -> RetType {
    return value.setCenter(self);
    // return 1;
  }
}

pub trait QConicalGradient_setCenter<RetType> {
  fn setCenter(self, rsthis: &mut QConicalGradient) -> RetType;
}

// proto:  void QConicalGradient::setCenter(qreal x, qreal y);
impl<'a> /*trait*/ QConicalGradient_setCenter<()> for (f64, f64) {
  fn setCenter(self, rsthis: &mut QConicalGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QConicalGradient9setCenterEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN16QConicalGradient9setCenterEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QConicalGradient::setCenter(const QPointF & center);
impl<'a> /*trait*/ QConicalGradient_setCenter<()> for (&'a  QPointF) {
  fn setCenter(self, rsthis: &mut QConicalGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QConicalGradient9setCenterERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QConicalGradient9setCenterERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
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

