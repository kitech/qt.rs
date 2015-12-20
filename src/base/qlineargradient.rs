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
  // proto:  void QLinearGradient::setFinalStop(qreal x, qreal y);
  fn _ZN15QLinearGradient12setFinalStopEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double);
  // proto:  QPointF QLinearGradient::start();
  fn _ZNK15QLinearGradient5startEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QLinearGradient::QLinearGradient(qreal xStart, qreal yStart, qreal xFinalStop, qreal yFinalStop);
  fn _ZN15QLinearGradientC1Edddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double);
  // proto:  void QLinearGradient::QLinearGradient(const QPointF & start, const QPointF & finalStop);
  fn _ZN15QLinearGradientC1ERK7QPointFS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QLinearGradient::setStart(qreal x, qreal y);
  fn _ZN15QLinearGradient8setStartEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double);
  // proto:  void QLinearGradient::setStart(const QPointF & start);
  fn _ZN15QLinearGradient8setStartERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QLinearGradient::QLinearGradient();
  fn _ZN15QLinearGradientC1Ev(qthis: *mut c_void);
  // proto:  QPointF QLinearGradient::finalStop();
  fn _ZNK15QLinearGradient9finalStopEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QLinearGradient::setFinalStop(const QPointF & stop);
  fn _ZN15QLinearGradient12setFinalStopERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
}

// body block begin
// class sizeof(QLinearGradient)=1
pub struct QLinearGradient {
  pub qclsinst: *mut c_void,
}

  // proto:  void QLinearGradient::setFinalStop(qreal x, qreal y);
impl /*struct*/ QLinearGradient {
  pub fn setFinalStop<RetType, T: QLinearGradient_setFinalStop<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFinalStop(self);
    // return 1;
  }
}

pub trait QLinearGradient_setFinalStop<RetType> {
  fn setFinalStop(self , rsthis: &mut QLinearGradient) -> RetType;
}

  // proto:  void QLinearGradient::setFinalStop(qreal x, qreal y);
impl<'a> /*trait*/ QLinearGradient_setFinalStop<()> for (f64, f64) {
  fn setFinalStop(self , rsthis: &mut QLinearGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QLinearGradient12setFinalStopEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN15QLinearGradient12setFinalStopEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QPointF QLinearGradient::start();
impl /*struct*/ QLinearGradient {
  pub fn start<RetType, T: QLinearGradient_start<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.start(self);
    // return 1;
  }
}

pub trait QLinearGradient_start<RetType> {
  fn start(self , rsthis: &mut QLinearGradient) -> RetType;
}

  // proto:  QPointF QLinearGradient::start();
impl<'a> /*trait*/ QLinearGradient_start<QPointF> for () {
  fn start(self , rsthis: &mut QLinearGradient) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QLinearGradient5startEv()};
    let mut ret = unsafe {_ZNK15QLinearGradient5startEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QLinearGradient::QLinearGradient(qreal xStart, qreal yStart, qreal xFinalStop, qreal yFinalStop);
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

  // proto:  void QLinearGradient::QLinearGradient(qreal xStart, qreal yStart, qreal xFinalStop, qreal yFinalStop);
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

  // proto:  void QLinearGradient::QLinearGradient(const QPointF & start, const QPointF & finalStop);
impl<'a> /*trait*/ QLinearGradient_NewQLinearGradient for (QPointF, QPointF) {
  fn NewQLinearGradient(self) -> QLinearGradient {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QLinearGradientC1ERK7QPointFS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN15QLinearGradientC1ERK7QPointFS2_(qthis, arg0, arg1)};
    let rsthis = QLinearGradient{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QLinearGradient::setStart(qreal x, qreal y);
impl /*struct*/ QLinearGradient {
  pub fn setStart<RetType, T: QLinearGradient_setStart<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setStart(self);
    // return 1;
  }
}

pub trait QLinearGradient_setStart<RetType> {
  fn setStart(self , rsthis: &mut QLinearGradient) -> RetType;
}

  // proto:  void QLinearGradient::setStart(qreal x, qreal y);
impl<'a> /*trait*/ QLinearGradient_setStart<()> for (f64, f64) {
  fn setStart(self , rsthis: &mut QLinearGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QLinearGradient8setStartEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN15QLinearGradient8setStartEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QLinearGradient::setStart(const QPointF & start);
impl<'a> /*trait*/ QLinearGradient_setStart<()> for (QPointF) {
  fn setStart(self , rsthis: &mut QLinearGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QLinearGradient8setStartERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QLinearGradient8setStartERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QLinearGradient::QLinearGradient();
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

  // proto:  QPointF QLinearGradient::finalStop();
impl /*struct*/ QLinearGradient {
  pub fn finalStop<RetType, T: QLinearGradient_finalStop<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.finalStop(self);
    // return 1;
  }
}

pub trait QLinearGradient_finalStop<RetType> {
  fn finalStop(self , rsthis: &mut QLinearGradient) -> RetType;
}

  // proto:  QPointF QLinearGradient::finalStop();
impl<'a> /*trait*/ QLinearGradient_finalStop<QPointF> for () {
  fn finalStop(self , rsthis: &mut QLinearGradient) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QLinearGradient9finalStopEv()};
    let mut ret = unsafe {_ZNK15QLinearGradient9finalStopEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QLinearGradient::setFinalStop(const QPointF & stop);
impl<'a> /*trait*/ QLinearGradient_setFinalStop<()> for (QPointF) {
  fn setFinalStop(self , rsthis: &mut QLinearGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QLinearGradient12setFinalStopERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QLinearGradient12setFinalStopERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

