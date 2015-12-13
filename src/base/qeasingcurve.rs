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
  fn _ZN12QEasingCurveC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN12QEasingCurveD0Ev() -> i32;
  fn _ZNK12QEasingCurve10customTypeEv() -> i32;
  fn _ZNK12QEasingCurve9overshootEv() -> i32;
  fn _ZN12QEasingCurve9setPeriodEd(arg0: c_double) -> i32;
  fn _ZN12QEasingCurve13addTCBSegmentERK7QPointFddd(arg0: *const c_void, arg1: c_double, arg2: c_double, arg3: c_double) -> i32;
  fn _ZN12QEasingCurve21addCubicBezierSegmentERK7QPointFS2_S2_(arg0: *const c_void, arg1: *const c_void, arg2: *const c_void) -> i32;
  fn _ZNK12QEasingCurve6periodEv() -> i32;
  fn _ZNK12QEasingCurve16valueForProgressEd(arg0: c_double) -> i32;
  fn _ZN12QEasingCurve12setAmplitudeEd(arg0: c_double) -> i32;
  fn _ZN12QEasingCurve4swapERS_(arg0: *mut c_void) -> i32;
  fn _ZN12QEasingCurve12setOvershootEd(arg0: c_double) -> i32;
  fn _ZNK12QEasingCurve13toCubicSplineEv() -> i32;
  fn _ZNK12QEasingCurve9amplitudeEv() -> i32;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QEasingCurveC1ERKS_(qthis, arg0)};
    let rsthis = QEasingCurve{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QEasingCurve {
  pub fn FreeQEasingCurve<T: QEasingCurve_FreeQEasingCurve>(&mut self, value: T) -> i32 {
    value.FreeQEasingCurve(self);
    return 1;
  }
}

pub trait QEasingCurve_FreeQEasingCurve {
  fn FreeQEasingCurve(self, this: &mut QEasingCurve) -> i32;
}

// proto: void QEasingCurve::FreeQEasingCurve();
impl<'a> /*trait*/ QEasingCurve_FreeQEasingCurve for () {
  fn FreeQEasingCurve(self, this: &mut QEasingCurve) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QEasingCurveD0Ev()};
    unsafe {_ZN12QEasingCurveD0Ev()};
    return 1;
  }
}

impl /*struct*/ QEasingCurve {
  pub fn customType<T: QEasingCurve_customType>(&mut self, value: T) -> i32 {
    value.customType(self);
    return 1;
  }
}

pub trait QEasingCurve_customType {
  fn customType(self, this: &mut QEasingCurve) -> i32;
}

// proto: QEasingCurve::EasingFunction QEasingCurve::customType();
impl<'a> /*trait*/ QEasingCurve_customType for () {
  fn customType(self, this: &mut QEasingCurve) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QEasingCurve10customTypeEv()};
    unsafe {_ZNK12QEasingCurve10customTypeEv()};
    return 1;
  }
}

impl /*struct*/ QEasingCurve {
  pub fn overshoot<T: QEasingCurve_overshoot>(&mut self, value: T) -> i32 {
    value.overshoot(self);
    return 1;
  }
}

pub trait QEasingCurve_overshoot {
  fn overshoot(self, this: &mut QEasingCurve) -> i32;
}

// proto: double QEasingCurve::overshoot();
impl<'a> /*trait*/ QEasingCurve_overshoot for () {
  fn overshoot(self, this: &mut QEasingCurve) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QEasingCurve9overshootEv()};
    unsafe {_ZNK12QEasingCurve9overshootEv()};
    return 1;
  }
}

impl /*struct*/ QEasingCurve {
  pub fn setPeriod<T: QEasingCurve_setPeriod>(&mut self, value: T) -> i32 {
    value.setPeriod(self);
    return 1;
  }
}

pub trait QEasingCurve_setPeriod {
  fn setPeriod(self, this: &mut QEasingCurve) -> i32;
}

// proto: void QEasingCurve::setPeriod(qreal period);
impl<'a> /*trait*/ QEasingCurve_setPeriod for (f64) {
  fn setPeriod(self, this: &mut QEasingCurve) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QEasingCurve9setPeriodEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN12QEasingCurve9setPeriodEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QEasingCurve {
  pub fn addTCBSegment<T: QEasingCurve_addTCBSegment>(&mut self, value: T) -> i32 {
    value.addTCBSegment(self);
    return 1;
  }
}

pub trait QEasingCurve_addTCBSegment {
  fn addTCBSegment(self, this: &mut QEasingCurve) -> i32;
}

// proto: void QEasingCurve::addTCBSegment(const QPointF & nextPoint, qreal t, qreal c, qreal b);
impl<'a> /*trait*/ QEasingCurve_addTCBSegment for (&'a  QPointF, f64, f64, f64) {
  fn addTCBSegment(self, this: &mut QEasingCurve) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QEasingCurve13addTCBSegmentERK7QPointFddd()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZN12QEasingCurve13addTCBSegmentERK7QPointFddd(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QEasingCurve {
  pub fn addCubicBezierSegment<T: QEasingCurve_addCubicBezierSegment>(&mut self, value: T) -> i32 {
    value.addCubicBezierSegment(self);
    return 1;
  }
}

pub trait QEasingCurve_addCubicBezierSegment {
  fn addCubicBezierSegment(self, this: &mut QEasingCurve) -> i32;
}

// proto: void QEasingCurve::addCubicBezierSegment(const QPointF & c1, const QPointF & c2, const QPointF & endPoint);
impl<'a> /*trait*/ QEasingCurve_addCubicBezierSegment for (&'a  QPointF, &'a  QPointF, &'a  QPointF) {
  fn addCubicBezierSegment(self, this: &mut QEasingCurve) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QEasingCurve21addCubicBezierSegmentERK7QPointFS2_S2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN12QEasingCurve21addCubicBezierSegmentERK7QPointFS2_S2_(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QEasingCurve {
  pub fn period<T: QEasingCurve_period>(&mut self, value: T) -> i32 {
    value.period(self);
    return 1;
  }
}

pub trait QEasingCurve_period {
  fn period(self, this: &mut QEasingCurve) -> i32;
}

// proto: double QEasingCurve::period();
impl<'a> /*trait*/ QEasingCurve_period for () {
  fn period(self, this: &mut QEasingCurve) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QEasingCurve6periodEv()};
    unsafe {_ZNK12QEasingCurve6periodEv()};
    return 1;
  }
}

impl /*struct*/ QEasingCurve {
  pub fn valueForProgress<T: QEasingCurve_valueForProgress>(&mut self, value: T) -> i32 {
    value.valueForProgress(self);
    return 1;
  }
}

pub trait QEasingCurve_valueForProgress {
  fn valueForProgress(self, this: &mut QEasingCurve) -> i32;
}

// proto: double QEasingCurve::valueForProgress(qreal progress);
impl<'a> /*trait*/ QEasingCurve_valueForProgress for (f64) {
  fn valueForProgress(self, this: &mut QEasingCurve) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QEasingCurve16valueForProgressEd()};
    let arg0 = self  as c_double;
    unsafe {_ZNK12QEasingCurve16valueForProgressEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QEasingCurve {
  pub fn setAmplitude<T: QEasingCurve_setAmplitude>(&mut self, value: T) -> i32 {
    value.setAmplitude(self);
    return 1;
  }
}

pub trait QEasingCurve_setAmplitude {
  fn setAmplitude(self, this: &mut QEasingCurve) -> i32;
}

// proto: void QEasingCurve::setAmplitude(qreal amplitude);
impl<'a> /*trait*/ QEasingCurve_setAmplitude for (f64) {
  fn setAmplitude(self, this: &mut QEasingCurve) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QEasingCurve12setAmplitudeEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN12QEasingCurve12setAmplitudeEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QEasingCurve {
  pub fn swap<T: QEasingCurve_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QEasingCurve_swap {
  fn swap(self, this: &mut QEasingCurve) -> i32;
}

// proto: void QEasingCurve::swap(QEasingCurve & other);
impl<'a> /*trait*/ QEasingCurve_swap for (&'a mut QEasingCurve) {
  fn swap(self, this: &mut QEasingCurve) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QEasingCurve4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QEasingCurve4swapERS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QEasingCurve {
  pub fn setOvershoot<T: QEasingCurve_setOvershoot>(&mut self, value: T) -> i32 {
    value.setOvershoot(self);
    return 1;
  }
}

pub trait QEasingCurve_setOvershoot {
  fn setOvershoot(self, this: &mut QEasingCurve) -> i32;
}

// proto: void QEasingCurve::setOvershoot(qreal overshoot);
impl<'a> /*trait*/ QEasingCurve_setOvershoot for (f64) {
  fn setOvershoot(self, this: &mut QEasingCurve) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QEasingCurve12setOvershootEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN12QEasingCurve12setOvershootEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QEasingCurve {
  pub fn toCubicSpline<T: QEasingCurve_toCubicSpline>(&mut self, value: T) -> i32 {
    value.toCubicSpline(self);
    return 1;
  }
}

pub trait QEasingCurve_toCubicSpline {
  fn toCubicSpline(self, this: &mut QEasingCurve) -> i32;
}

// proto: QVector<QPointF> QEasingCurve::toCubicSpline();
impl<'a> /*trait*/ QEasingCurve_toCubicSpline for () {
  fn toCubicSpline(self, this: &mut QEasingCurve) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QEasingCurve13toCubicSplineEv()};
    unsafe {_ZNK12QEasingCurve13toCubicSplineEv()};
    return 1;
  }
}

impl /*struct*/ QEasingCurve {
  pub fn amplitude<T: QEasingCurve_amplitude>(&mut self, value: T) -> i32 {
    value.amplitude(self);
    return 1;
  }
}

pub trait QEasingCurve_amplitude {
  fn amplitude(self, this: &mut QEasingCurve) -> i32;
}

// proto: double QEasingCurve::amplitude();
impl<'a> /*trait*/ QEasingCurve_amplitude for () {
  fn amplitude(self, this: &mut QEasingCurve) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QEasingCurve9amplitudeEv()};
    unsafe {_ZNK12QEasingCurve9amplitudeEv()};
    return 1;
  }
}

