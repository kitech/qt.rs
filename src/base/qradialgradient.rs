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
  // proto: void QRadialGradient::NewQRadialGradient(qreal cx, qreal cy, qreal radius, qreal fx, qreal fy);
  fn _ZN15QRadialGradientC1Eddddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double) -> i32;
  // proto: void QRadialGradient::setFocalPoint(qreal x, qreal y);
  fn _ZN15QRadialGradient13setFocalPointEdd(arg0: c_double, arg1: c_double) -> i32;
  // proto: void QRadialGradient::NewQRadialGradient();
  fn _ZN15QRadialGradientC1Ev(qthis: *mut c_void) -> i32;
  // proto: void QRadialGradient::NewQRadialGradient(const QPointF & center, qreal radius, const QPointF & focalPoint);
  fn _ZN15QRadialGradientC1ERK7QPointFdS2_(qthis: *mut c_void, arg0: *const c_void, arg1: c_double, arg2: *const c_void) -> i32;
  // proto: double QRadialGradient::radius();
  fn _ZNK15QRadialGradient6radiusEv() -> i32;
  // proto: void QRadialGradient::setFocalPoint(const QPointF & focalPoint);
  fn _ZN15QRadialGradient13setFocalPointERK7QPointF(arg0: *const c_void) -> i32;
  // proto: void QRadialGradient::NewQRadialGradient(const QPointF & center, qreal centerRadius, const QPointF & focalPoint, qreal focalRadius);
  fn _ZN15QRadialGradientC1ERK7QPointFdS2_d(qthis: *mut c_void, arg0: *const c_void, arg1: c_double, arg2: *const c_void, arg3: c_double) -> i32;
  // proto: void QRadialGradient::NewQRadialGradient(qreal cx, qreal cy, qreal centerRadius, qreal fx, qreal fy, qreal focalRadius);
  fn _ZN15QRadialGradientC1Edddddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double, arg5: c_double) -> i32;
  // proto: double QRadialGradient::centerRadius();
  fn _ZNK15QRadialGradient12centerRadiusEv() -> i32;
  // proto: QPointF QRadialGradient::focalPoint();
  fn _ZNK15QRadialGradient10focalPointEv() -> i32;
  // proto: double QRadialGradient::focalRadius();
  fn _ZNK15QRadialGradient11focalRadiusEv() -> i32;
  // proto: QPointF QRadialGradient::center();
  fn _ZNK15QRadialGradient6centerEv() -> i32;
  // proto: void QRadialGradient::setCenter(const QPointF & center);
  fn _ZN15QRadialGradient9setCenterERK7QPointF(arg0: *const c_void) -> i32;
  // proto: void QRadialGradient::NewQRadialGradient(const QPointF & center, qreal radius);
  fn _ZN15QRadialGradientC1ERK7QPointFd(qthis: *mut c_void, arg0: *const c_void, arg1: c_double) -> i32;
  // proto: void QRadialGradient::setCenterRadius(qreal radius);
  fn _ZN15QRadialGradient15setCenterRadiusEd(arg0: c_double) -> i32;
  // proto: void QRadialGradient::setFocalRadius(qreal radius);
  fn _ZN15QRadialGradient14setFocalRadiusEd(arg0: c_double) -> i32;
  // proto: void QRadialGradient::setRadius(qreal radius);
  fn _ZN15QRadialGradient9setRadiusEd(arg0: c_double) -> i32;
  // proto: void QRadialGradient::NewQRadialGradient(qreal cx, qreal cy, qreal radius);
  fn _ZN15QRadialGradientC1Eddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double) -> i32;
  // proto: void QRadialGradient::setCenter(qreal x, qreal y);
  fn _ZN15QRadialGradient9setCenterEdd(arg0: c_double, arg1: c_double) -> i32;
}

// body block begin
// class sizeof(QRadialGradient)=1
pub struct QRadialGradient {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QRadialGradient {
  pub fn NewQRadialGradient<T: QRadialGradient_NewQRadialGradient>(value: T) -> QRadialGradient {
    let rsthis = value.NewQRadialGradient();
    return rsthis;
    // return 1;
  }
}

pub trait QRadialGradient_NewQRadialGradient {
  fn NewQRadialGradient(self) -> QRadialGradient;
}

// proto: void QRadialGradient::NewQRadialGradient(qreal cx, qreal cy, qreal radius, qreal fx, qreal fy);
impl<'a> /*trait*/ QRadialGradient_NewQRadialGradient for (f64, f64, f64, f64, f64) {
  fn NewQRadialGradient(self) -> QRadialGradient {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradientC1Eddddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    unsafe {_ZN15QRadialGradientC1Eddddd(qthis, arg0, arg1, arg2, arg3, arg4)};
    let rsthis = QRadialGradient{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QRadialGradient {
  pub fn setFocalPoint<T: QRadialGradient_setFocalPoint>(&mut self, value: T) -> i32 {
    value.setFocalPoint(self);
    return 1;
  }
}

pub trait QRadialGradient_setFocalPoint {
  fn setFocalPoint(self, this: &mut QRadialGradient) -> i32;
}

// proto: void QRadialGradient::setFocalPoint(qreal x, qreal y);
impl<'a> /*trait*/ QRadialGradient_setFocalPoint for (f64, f64) {
  fn setFocalPoint(self, this: &mut QRadialGradient) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradient13setFocalPointEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN15QRadialGradient13setFocalPointEdd(arg0, arg1)};
    return 1;
  }
}

// proto: void QRadialGradient::NewQRadialGradient();
impl<'a> /*trait*/ QRadialGradient_NewQRadialGradient for () {
  fn NewQRadialGradient(self) -> QRadialGradient {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradientC1Ev()};
    unsafe {_ZN15QRadialGradientC1Ev(qthis)};
    let rsthis = QRadialGradient{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QRadialGradient::NewQRadialGradient(const QPointF & center, qreal radius, const QPointF & focalPoint);
impl<'a> /*trait*/ QRadialGradient_NewQRadialGradient for (&'a  QPointF, f64, &'a  QPointF) {
  fn NewQRadialGradient(self) -> QRadialGradient {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradientC1ERK7QPointFdS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN15QRadialGradientC1ERK7QPointFdS2_(qthis, arg0, arg1, arg2)};
    let rsthis = QRadialGradient{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QRadialGradient {
  pub fn radius<T: QRadialGradient_radius>(&mut self, value: T) -> i32 {
    value.radius(self);
    return 1;
  }
}

pub trait QRadialGradient_radius {
  fn radius(self, this: &mut QRadialGradient) -> i32;
}

// proto: double QRadialGradient::radius();
impl<'a> /*trait*/ QRadialGradient_radius for () {
  fn radius(self, this: &mut QRadialGradient) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QRadialGradient6radiusEv()};
    unsafe {_ZNK15QRadialGradient6radiusEv()};
    return 1;
  }
}

// proto: void QRadialGradient::setFocalPoint(const QPointF & focalPoint);
impl<'a> /*trait*/ QRadialGradient_setFocalPoint for (&'a  QPointF) {
  fn setFocalPoint(self, this: &mut QRadialGradient) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradient13setFocalPointERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QRadialGradient13setFocalPointERK7QPointF(arg0)};
    return 1;
  }
}

// proto: void QRadialGradient::NewQRadialGradient(const QPointF & center, qreal centerRadius, const QPointF & focalPoint, qreal focalRadius);
impl<'a> /*trait*/ QRadialGradient_NewQRadialGradient for (&'a  QPointF, f64, &'a  QPointF, f64) {
  fn NewQRadialGradient(self) -> QRadialGradient {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradientC1ERK7QPointFdS2_d()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2.qclsinst  as *const c_void;
    let arg3 = self.3  as c_double;
    unsafe {_ZN15QRadialGradientC1ERK7QPointFdS2_d(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QRadialGradient{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QRadialGradient::NewQRadialGradient(qreal cx, qreal cy, qreal centerRadius, qreal fx, qreal fy, qreal focalRadius);
impl<'a> /*trait*/ QRadialGradient_NewQRadialGradient for (f64, f64, f64, f64, f64, f64) {
  fn NewQRadialGradient(self) -> QRadialGradient {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradientC1Edddddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    let arg5 = self.5  as c_double;
    unsafe {_ZN15QRadialGradientC1Edddddd(qthis, arg0, arg1, arg2, arg3, arg4, arg5)};
    let rsthis = QRadialGradient{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QRadialGradient {
  pub fn centerRadius<T: QRadialGradient_centerRadius>(&mut self, value: T) -> i32 {
    value.centerRadius(self);
    return 1;
  }
}

pub trait QRadialGradient_centerRadius {
  fn centerRadius(self, this: &mut QRadialGradient) -> i32;
}

// proto: double QRadialGradient::centerRadius();
impl<'a> /*trait*/ QRadialGradient_centerRadius for () {
  fn centerRadius(self, this: &mut QRadialGradient) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QRadialGradient12centerRadiusEv()};
    unsafe {_ZNK15QRadialGradient12centerRadiusEv()};
    return 1;
  }
}

impl /*struct*/ QRadialGradient {
  pub fn focalPoint<T: QRadialGradient_focalPoint>(&mut self, value: T) -> i32 {
    value.focalPoint(self);
    return 1;
  }
}

pub trait QRadialGradient_focalPoint {
  fn focalPoint(self, this: &mut QRadialGradient) -> i32;
}

// proto: QPointF QRadialGradient::focalPoint();
impl<'a> /*trait*/ QRadialGradient_focalPoint for () {
  fn focalPoint(self, this: &mut QRadialGradient) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QRadialGradient10focalPointEv()};
    unsafe {_ZNK15QRadialGradient10focalPointEv()};
    return 1;
  }
}

impl /*struct*/ QRadialGradient {
  pub fn focalRadius<T: QRadialGradient_focalRadius>(&mut self, value: T) -> i32 {
    value.focalRadius(self);
    return 1;
  }
}

pub trait QRadialGradient_focalRadius {
  fn focalRadius(self, this: &mut QRadialGradient) -> i32;
}

// proto: double QRadialGradient::focalRadius();
impl<'a> /*trait*/ QRadialGradient_focalRadius for () {
  fn focalRadius(self, this: &mut QRadialGradient) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QRadialGradient11focalRadiusEv()};
    unsafe {_ZNK15QRadialGradient11focalRadiusEv()};
    return 1;
  }
}

impl /*struct*/ QRadialGradient {
  pub fn center<T: QRadialGradient_center>(&mut self, value: T) -> i32 {
    value.center(self);
    return 1;
  }
}

pub trait QRadialGradient_center {
  fn center(self, this: &mut QRadialGradient) -> i32;
}

// proto: QPointF QRadialGradient::center();
impl<'a> /*trait*/ QRadialGradient_center for () {
  fn center(self, this: &mut QRadialGradient) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QRadialGradient6centerEv()};
    unsafe {_ZNK15QRadialGradient6centerEv()};
    return 1;
  }
}

impl /*struct*/ QRadialGradient {
  pub fn setCenter<T: QRadialGradient_setCenter>(&mut self, value: T) -> i32 {
    value.setCenter(self);
    return 1;
  }
}

pub trait QRadialGradient_setCenter {
  fn setCenter(self, this: &mut QRadialGradient) -> i32;
}

// proto: void QRadialGradient::setCenter(const QPointF & center);
impl<'a> /*trait*/ QRadialGradient_setCenter for (&'a  QPointF) {
  fn setCenter(self, this: &mut QRadialGradient) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradient9setCenterERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QRadialGradient9setCenterERK7QPointF(arg0)};
    return 1;
  }
}

// proto: void QRadialGradient::NewQRadialGradient(const QPointF & center, qreal radius);
impl<'a> /*trait*/ QRadialGradient_NewQRadialGradient for (&'a  QPointF, f64) {
  fn NewQRadialGradient(self) -> QRadialGradient {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradientC1ERK7QPointFd()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_double;
    unsafe {_ZN15QRadialGradientC1ERK7QPointFd(qthis, arg0, arg1)};
    let rsthis = QRadialGradient{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QRadialGradient {
  pub fn setCenterRadius<T: QRadialGradient_setCenterRadius>(&mut self, value: T) -> i32 {
    value.setCenterRadius(self);
    return 1;
  }
}

pub trait QRadialGradient_setCenterRadius {
  fn setCenterRadius(self, this: &mut QRadialGradient) -> i32;
}

// proto: void QRadialGradient::setCenterRadius(qreal radius);
impl<'a> /*trait*/ QRadialGradient_setCenterRadius for (f64) {
  fn setCenterRadius(self, this: &mut QRadialGradient) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradient15setCenterRadiusEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN15QRadialGradient15setCenterRadiusEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QRadialGradient {
  pub fn setFocalRadius<T: QRadialGradient_setFocalRadius>(&mut self, value: T) -> i32 {
    value.setFocalRadius(self);
    return 1;
  }
}

pub trait QRadialGradient_setFocalRadius {
  fn setFocalRadius(self, this: &mut QRadialGradient) -> i32;
}

// proto: void QRadialGradient::setFocalRadius(qreal radius);
impl<'a> /*trait*/ QRadialGradient_setFocalRadius for (f64) {
  fn setFocalRadius(self, this: &mut QRadialGradient) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradient14setFocalRadiusEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN15QRadialGradient14setFocalRadiusEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QRadialGradient {
  pub fn setRadius<T: QRadialGradient_setRadius>(&mut self, value: T) -> i32 {
    value.setRadius(self);
    return 1;
  }
}

pub trait QRadialGradient_setRadius {
  fn setRadius(self, this: &mut QRadialGradient) -> i32;
}

// proto: void QRadialGradient::setRadius(qreal radius);
impl<'a> /*trait*/ QRadialGradient_setRadius for (f64) {
  fn setRadius(self, this: &mut QRadialGradient) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradient9setRadiusEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN15QRadialGradient9setRadiusEd(arg0)};
    return 1;
  }
}

// proto: void QRadialGradient::NewQRadialGradient(qreal cx, qreal cy, qreal radius);
impl<'a> /*trait*/ QRadialGradient_NewQRadialGradient for (f64, f64, f64) {
  fn NewQRadialGradient(self) -> QRadialGradient {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradientC1Eddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    unsafe {_ZN15QRadialGradientC1Eddd(qthis, arg0, arg1, arg2)};
    let rsthis = QRadialGradient{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QRadialGradient::setCenter(qreal x, qreal y);
impl<'a> /*trait*/ QRadialGradient_setCenter for (f64, f64) {
  fn setCenter(self, this: &mut QRadialGradient) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradient9setCenterEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN15QRadialGradient9setCenterEdd(arg0, arg1)};
    return 1;
  }
}

