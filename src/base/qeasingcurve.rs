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
  // proto:  void QEasingCurve::NewQEasingCurve(const QEasingCurve & other);
  fn _ZN12QEasingCurveC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QEasingCurve::FreeQEasingCurve();
  fn _ZN12QEasingCurveD0Ev(qthis: *mut c_void) ;
  // proto:  QEasingCurve::EasingFunction QEasingCurve::customType();
  fn _ZNK12QEasingCurve10customTypeEv(qthis: *mut c_void) ;
  // proto:  double QEasingCurve::overshoot();
  fn _ZNK12QEasingCurve9overshootEv(qthis: *mut c_void) -> c_double;
  // proto:  void QEasingCurve::setPeriod(qreal period);
  fn _ZN12QEasingCurve9setPeriodEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QEasingCurve::addTCBSegment(const QPointF & nextPoint, qreal t, qreal c, qreal b);
  fn _ZN12QEasingCurve13addTCBSegmentERK7QPointFddd(qthis: *mut c_void, arg0: *mut c_void, arg1: c_double, arg2: c_double, arg3: c_double) ;
  // proto:  void QEasingCurve::addCubicBezierSegment(const QPointF & c1, const QPointF & c2, const QPointF & endPoint);
  fn _ZN12QEasingCurve21addCubicBezierSegmentERK7QPointFS2_S2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  double QEasingCurve::period();
  fn _ZNK12QEasingCurve6periodEv(qthis: *mut c_void) -> c_double;
  // proto:  double QEasingCurve::valueForProgress(qreal progress);
  fn _ZNK12QEasingCurve16valueForProgressEd(qthis: *mut c_void, arg0: c_double) -> c_double;
  // proto:  void QEasingCurve::setAmplitude(qreal amplitude);
  fn _ZN12QEasingCurve12setAmplitudeEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QEasingCurve::swap(QEasingCurve & other);
  fn _ZN12QEasingCurve4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QEasingCurve::setOvershoot(qreal overshoot);
  fn _ZN12QEasingCurve12setOvershootEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  QVector<QPointF> QEasingCurve::toCubicSpline();
  fn _ZNK12QEasingCurve13toCubicSplineEv(qthis: *mut c_void) ;
  // proto:  double QEasingCurve::amplitude();
  fn _ZNK12QEasingCurve9amplitudeEv(qthis: *mut c_void) -> c_double;
}

// body block begin
// class sizeof(QEasingCurve)=8
pub struct QEasingCurve {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QEasingCurve {
  pub fn NewQEasingCurve<T: QEasingCurve_NewQEasingCurve>(value: T) -> QEasingCurve {
    let rsthis = value.NewQEasingCurve();
    return rsthis;
    // return 1;
  }
}

pub trait QEasingCurve_NewQEasingCurve {
  fn NewQEasingCurve(self) -> QEasingCurve;
}

// proto: void QEasingCurve::NewQEasingCurve(const QEasingCurve & other);
impl<'a> /*trait*/ QEasingCurve_NewQEasingCurve for (&'a  QEasingCurve) {
  fn NewQEasingCurve(self) -> QEasingCurve {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QEasingCurveC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QEasingCurveC1ERKS_(qthis, arg0)};
    let rsthis = QEasingCurve{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QEasingCurve {
  pub fn FreeQEasingCurve<T: QEasingCurve_FreeQEasingCurve>(&mut self, value: T)  {
     value.FreeQEasingCurve(self);
    // return 1;
  }
}

pub trait QEasingCurve_FreeQEasingCurve {
  fn FreeQEasingCurve(self, rsthis: &mut QEasingCurve) ;
}

// proto:  void QEasingCurve::FreeQEasingCurve();
impl<'a> /*trait*/ QEasingCurve_FreeQEasingCurve for () {
  fn FreeQEasingCurve(self, rsthis: &mut QEasingCurve)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QEasingCurveD0Ev()};
     unsafe {_ZN12QEasingCurveD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QEasingCurve {
  pub fn customType<T: QEasingCurve_customType>(&mut self, value: T)  {
     value.customType(self);
    // return 1;
  }
}

pub trait QEasingCurve_customType {
  fn customType(self, rsthis: &mut QEasingCurve) ;
}

// proto:  QEasingCurve::EasingFunction QEasingCurve::customType();
impl<'a> /*trait*/ QEasingCurve_customType for () {
  fn customType(self, rsthis: &mut QEasingCurve)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QEasingCurve10customTypeEv()};
     unsafe {_ZNK12QEasingCurve10customTypeEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QEasingCurve {
  pub fn overshoot<T: QEasingCurve_overshoot>(&mut self, value: T) -> f64 {
    return value.overshoot(self);
    // return 1;
  }
}

pub trait QEasingCurve_overshoot {
  fn overshoot(self, rsthis: &mut QEasingCurve) -> f64;
}

// proto:  double QEasingCurve::overshoot();
impl<'a> /*trait*/ QEasingCurve_overshoot for () {
  fn overshoot(self, rsthis: &mut QEasingCurve) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QEasingCurve9overshootEv()};
    let mut ret = unsafe {_ZNK12QEasingCurve9overshootEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QEasingCurve {
  pub fn setPeriod<T: QEasingCurve_setPeriod>(&mut self, value: T)  {
     value.setPeriod(self);
    // return 1;
  }
}

pub trait QEasingCurve_setPeriod {
  fn setPeriod(self, rsthis: &mut QEasingCurve) ;
}

// proto:  void QEasingCurve::setPeriod(qreal period);
impl<'a> /*trait*/ QEasingCurve_setPeriod for (f64) {
  fn setPeriod(self, rsthis: &mut QEasingCurve)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QEasingCurve9setPeriodEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN12QEasingCurve9setPeriodEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QEasingCurve {
  pub fn addTCBSegment<T: QEasingCurve_addTCBSegment>(&mut self, value: T)  {
     value.addTCBSegment(self);
    // return 1;
  }
}

pub trait QEasingCurve_addTCBSegment {
  fn addTCBSegment(self, rsthis: &mut QEasingCurve) ;
}

// proto:  void QEasingCurve::addTCBSegment(const QPointF & nextPoint, qreal t, qreal c, qreal b);
impl<'a> /*trait*/ QEasingCurve_addTCBSegment for (&'a  QPointF, f64, f64, f64) {
  fn addTCBSegment(self, rsthis: &mut QEasingCurve)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QEasingCurve13addTCBSegmentERK7QPointFddd()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {_ZN12QEasingCurve13addTCBSegmentERK7QPointFddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QEasingCurve {
  pub fn addCubicBezierSegment<T: QEasingCurve_addCubicBezierSegment>(&mut self, value: T)  {
     value.addCubicBezierSegment(self);
    // return 1;
  }
}

pub trait QEasingCurve_addCubicBezierSegment {
  fn addCubicBezierSegment(self, rsthis: &mut QEasingCurve) ;
}

// proto:  void QEasingCurve::addCubicBezierSegment(const QPointF & c1, const QPointF & c2, const QPointF & endPoint);
impl<'a> /*trait*/ QEasingCurve_addCubicBezierSegment for (&'a  QPointF, &'a  QPointF, &'a  QPointF) {
  fn addCubicBezierSegment(self, rsthis: &mut QEasingCurve)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QEasingCurve21addCubicBezierSegmentERK7QPointFS2_S2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN12QEasingCurve21addCubicBezierSegmentERK7QPointFS2_S2_(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QEasingCurve {
  pub fn period<T: QEasingCurve_period>(&mut self, value: T) -> f64 {
    return value.period(self);
    // return 1;
  }
}

pub trait QEasingCurve_period {
  fn period(self, rsthis: &mut QEasingCurve) -> f64;
}

// proto:  double QEasingCurve::period();
impl<'a> /*trait*/ QEasingCurve_period for () {
  fn period(self, rsthis: &mut QEasingCurve) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QEasingCurve6periodEv()};
    let mut ret = unsafe {_ZNK12QEasingCurve6periodEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QEasingCurve {
  pub fn valueForProgress<T: QEasingCurve_valueForProgress>(&mut self, value: T) -> f64 {
    return value.valueForProgress(self);
    // return 1;
  }
}

pub trait QEasingCurve_valueForProgress {
  fn valueForProgress(self, rsthis: &mut QEasingCurve) -> f64;
}

// proto:  double QEasingCurve::valueForProgress(qreal progress);
impl<'a> /*trait*/ QEasingCurve_valueForProgress for (f64) {
  fn valueForProgress(self, rsthis: &mut QEasingCurve) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QEasingCurve16valueForProgressEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZNK12QEasingCurve16valueForProgressEd(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QEasingCurve {
  pub fn setAmplitude<T: QEasingCurve_setAmplitude>(&mut self, value: T)  {
     value.setAmplitude(self);
    // return 1;
  }
}

pub trait QEasingCurve_setAmplitude {
  fn setAmplitude(self, rsthis: &mut QEasingCurve) ;
}

// proto:  void QEasingCurve::setAmplitude(qreal amplitude);
impl<'a> /*trait*/ QEasingCurve_setAmplitude for (f64) {
  fn setAmplitude(self, rsthis: &mut QEasingCurve)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QEasingCurve12setAmplitudeEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN12QEasingCurve12setAmplitudeEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QEasingCurve {
  pub fn swap<T: QEasingCurve_swap>(&mut self, value: T)  {
     value.swap(self);
    // return 1;
  }
}

pub trait QEasingCurve_swap {
  fn swap(self, rsthis: &mut QEasingCurve) ;
}

// proto:  void QEasingCurve::swap(QEasingCurve & other);
impl<'a> /*trait*/ QEasingCurve_swap for (&'a mut QEasingCurve) {
  fn swap(self, rsthis: &mut QEasingCurve)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QEasingCurve4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QEasingCurve4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QEasingCurve {
  pub fn setOvershoot<T: QEasingCurve_setOvershoot>(&mut self, value: T)  {
     value.setOvershoot(self);
    // return 1;
  }
}

pub trait QEasingCurve_setOvershoot {
  fn setOvershoot(self, rsthis: &mut QEasingCurve) ;
}

// proto:  void QEasingCurve::setOvershoot(qreal overshoot);
impl<'a> /*trait*/ QEasingCurve_setOvershoot for (f64) {
  fn setOvershoot(self, rsthis: &mut QEasingCurve)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QEasingCurve12setOvershootEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN12QEasingCurve12setOvershootEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QEasingCurve {
  pub fn toCubicSpline<T: QEasingCurve_toCubicSpline>(&mut self, value: T)  {
     value.toCubicSpline(self);
    // return 1;
  }
}

pub trait QEasingCurve_toCubicSpline {
  fn toCubicSpline(self, rsthis: &mut QEasingCurve) ;
}

// proto:  QVector<QPointF> QEasingCurve::toCubicSpline();
impl<'a> /*trait*/ QEasingCurve_toCubicSpline for () {
  fn toCubicSpline(self, rsthis: &mut QEasingCurve)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QEasingCurve13toCubicSplineEv()};
     unsafe {_ZNK12QEasingCurve13toCubicSplineEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QEasingCurve {
  pub fn amplitude<T: QEasingCurve_amplitude>(&mut self, value: T) -> f64 {
    return value.amplitude(self);
    // return 1;
  }
}

pub trait QEasingCurve_amplitude {
  fn amplitude(self, rsthis: &mut QEasingCurve) -> f64;
}

// proto:  double QEasingCurve::amplitude();
impl<'a> /*trait*/ QEasingCurve_amplitude for () {
  fn amplitude(self, rsthis: &mut QEasingCurve) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QEasingCurve9amplitudeEv()};
    let mut ret = unsafe {_ZNK12QEasingCurve9amplitudeEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

