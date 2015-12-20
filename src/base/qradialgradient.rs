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
  // proto:  void QRadialGradient::QRadialGradient(qreal cx, qreal cy, qreal radius, qreal fx, qreal fy);
  fn _ZN15QRadialGradientC1Eddddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double);
  // proto:  void QRadialGradient::setFocalPoint(qreal x, qreal y);
  fn _ZN15QRadialGradient13setFocalPointEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double);
  // proto:  void QRadialGradient::QRadialGradient();
  fn _ZN15QRadialGradientC1Ev(qthis: *mut c_void);
  // proto:  void QRadialGradient::QRadialGradient(const QPointF & center, qreal radius, const QPointF & focalPoint);
  fn _ZN15QRadialGradientC1ERK7QPointFdS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: c_double, arg2: *mut c_void);
  // proto:  qreal QRadialGradient::radius();
  fn _ZNK15QRadialGradient6radiusEv(qthis: *mut c_void) -> c_double;
  // proto:  void QRadialGradient::setFocalPoint(const QPointF & focalPoint);
  fn _ZN15QRadialGradient13setFocalPointERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QRadialGradient::QRadialGradient(const QPointF & center, qreal centerRadius, const QPointF & focalPoint, qreal focalRadius);
  fn _ZN15QRadialGradientC1ERK7QPointFdS2_d(qthis: *mut c_void, arg0: *mut c_void, arg1: c_double, arg2: *mut c_void, arg3: c_double);
  // proto:  void QRadialGradient::QRadialGradient(qreal cx, qreal cy, qreal centerRadius, qreal fx, qreal fy, qreal focalRadius);
  fn _ZN15QRadialGradientC1Edddddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double, arg5: c_double);
  // proto:  qreal QRadialGradient::centerRadius();
  fn _ZNK15QRadialGradient12centerRadiusEv(qthis: *mut c_void) -> c_double;
  // proto:  QPointF QRadialGradient::focalPoint();
  fn _ZNK15QRadialGradient10focalPointEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  qreal QRadialGradient::focalRadius();
  fn _ZNK15QRadialGradient11focalRadiusEv(qthis: *mut c_void) -> c_double;
  // proto:  QPointF QRadialGradient::center();
  fn _ZNK15QRadialGradient6centerEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRadialGradient::setCenter(const QPointF & center);
  fn _ZN15QRadialGradient9setCenterERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QRadialGradient::QRadialGradient(const QPointF & center, qreal radius);
  fn _ZN15QRadialGradientC1ERK7QPointFd(qthis: *mut c_void, arg0: *mut c_void, arg1: c_double);
  // proto:  void QRadialGradient::setCenterRadius(qreal radius);
  fn _ZN15QRadialGradient15setCenterRadiusEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QRadialGradient::setFocalRadius(qreal radius);
  fn _ZN15QRadialGradient14setFocalRadiusEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QRadialGradient::setRadius(qreal radius);
  fn _ZN15QRadialGradient9setRadiusEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QRadialGradient::QRadialGradient(qreal cx, qreal cy, qreal radius);
  fn _ZN15QRadialGradientC1Eddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double);
  // proto:  void QRadialGradient::setCenter(qreal x, qreal y);
  fn _ZN15QRadialGradient9setCenterEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double);
}

// body block begin
// class sizeof(QRadialGradient)=1
pub struct QRadialGradient {
  pub qclsinst: *mut c_void,
}

  // proto:  void QRadialGradient::QRadialGradient(qreal cx, qreal cy, qreal radius, qreal fx, qreal fy);
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

  // proto:  void QRadialGradient::QRadialGradient(qreal cx, qreal cy, qreal radius, qreal fx, qreal fy);
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

  // proto:  void QRadialGradient::setFocalPoint(qreal x, qreal y);
impl /*struct*/ QRadialGradient {
  pub fn setFocalPoint<RetType, T: QRadialGradient_setFocalPoint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFocalPoint(self);
    // return 1;
  }
}

pub trait QRadialGradient_setFocalPoint<RetType> {
  fn setFocalPoint(self , rsthis: &mut QRadialGradient) -> RetType;
}

  // proto:  void QRadialGradient::setFocalPoint(qreal x, qreal y);
impl<'a> /*trait*/ QRadialGradient_setFocalPoint<()> for (f64, f64) {
  fn setFocalPoint(self , rsthis: &mut QRadialGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradient13setFocalPointEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN15QRadialGradient13setFocalPointEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QRadialGradient::QRadialGradient();
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

  // proto:  void QRadialGradient::QRadialGradient(const QPointF & center, qreal radius, const QPointF & focalPoint);
impl<'a> /*trait*/ QRadialGradient_NewQRadialGradient for (QPointF, f64, QPointF) {
  fn NewQRadialGradient(self) -> QRadialGradient {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradientC1ERK7QPointFdS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN15QRadialGradientC1ERK7QPointFdS2_(qthis, arg0, arg1, arg2)};
    let rsthis = QRadialGradient{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  qreal QRadialGradient::radius();
impl /*struct*/ QRadialGradient {
  pub fn radius<RetType, T: QRadialGradient_radius<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.radius(self);
    // return 1;
  }
}

pub trait QRadialGradient_radius<RetType> {
  fn radius(self , rsthis: &mut QRadialGradient) -> RetType;
}

  // proto:  qreal QRadialGradient::radius();
impl<'a> /*trait*/ QRadialGradient_radius<f64> for () {
  fn radius(self , rsthis: &mut QRadialGradient) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QRadialGradient6radiusEv()};
    let mut ret = unsafe {_ZNK15QRadialGradient6radiusEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QRadialGradient::setFocalPoint(const QPointF & focalPoint);
impl<'a> /*trait*/ QRadialGradient_setFocalPoint<()> for (QPointF) {
  fn setFocalPoint(self , rsthis: &mut QRadialGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradient13setFocalPointERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QRadialGradient13setFocalPointERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRadialGradient::QRadialGradient(const QPointF & center, qreal centerRadius, const QPointF & focalPoint, qreal focalRadius);
impl<'a> /*trait*/ QRadialGradient_NewQRadialGradient for (QPointF, f64, QPointF, f64) {
  fn NewQRadialGradient(self) -> QRadialGradient {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradientC1ERK7QPointFdS2_d()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3  as c_double;
    unsafe {_ZN15QRadialGradientC1ERK7QPointFdS2_d(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QRadialGradient{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QRadialGradient::QRadialGradient(qreal cx, qreal cy, qreal centerRadius, qreal fx, qreal fy, qreal focalRadius);
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

  // proto:  qreal QRadialGradient::centerRadius();
impl /*struct*/ QRadialGradient {
  pub fn centerRadius<RetType, T: QRadialGradient_centerRadius<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.centerRadius(self);
    // return 1;
  }
}

pub trait QRadialGradient_centerRadius<RetType> {
  fn centerRadius(self , rsthis: &mut QRadialGradient) -> RetType;
}

  // proto:  qreal QRadialGradient::centerRadius();
impl<'a> /*trait*/ QRadialGradient_centerRadius<f64> for () {
  fn centerRadius(self , rsthis: &mut QRadialGradient) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QRadialGradient12centerRadiusEv()};
    let mut ret = unsafe {_ZNK15QRadialGradient12centerRadiusEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QPointF QRadialGradient::focalPoint();
impl /*struct*/ QRadialGradient {
  pub fn focalPoint<RetType, T: QRadialGradient_focalPoint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.focalPoint(self);
    // return 1;
  }
}

pub trait QRadialGradient_focalPoint<RetType> {
  fn focalPoint(self , rsthis: &mut QRadialGradient) -> RetType;
}

  // proto:  QPointF QRadialGradient::focalPoint();
impl<'a> /*trait*/ QRadialGradient_focalPoint<QPointF> for () {
  fn focalPoint(self , rsthis: &mut QRadialGradient) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QRadialGradient10focalPointEv()};
    let mut ret = unsafe {_ZNK15QRadialGradient10focalPointEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QRadialGradient::focalRadius();
impl /*struct*/ QRadialGradient {
  pub fn focalRadius<RetType, T: QRadialGradient_focalRadius<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.focalRadius(self);
    // return 1;
  }
}

pub trait QRadialGradient_focalRadius<RetType> {
  fn focalRadius(self , rsthis: &mut QRadialGradient) -> RetType;
}

  // proto:  qreal QRadialGradient::focalRadius();
impl<'a> /*trait*/ QRadialGradient_focalRadius<f64> for () {
  fn focalRadius(self , rsthis: &mut QRadialGradient) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QRadialGradient11focalRadiusEv()};
    let mut ret = unsafe {_ZNK15QRadialGradient11focalRadiusEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QPointF QRadialGradient::center();
impl /*struct*/ QRadialGradient {
  pub fn center<RetType, T: QRadialGradient_center<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.center(self);
    // return 1;
  }
}

pub trait QRadialGradient_center<RetType> {
  fn center(self , rsthis: &mut QRadialGradient) -> RetType;
}

  // proto:  QPointF QRadialGradient::center();
impl<'a> /*trait*/ QRadialGradient_center<QPointF> for () {
  fn center(self , rsthis: &mut QRadialGradient) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QRadialGradient6centerEv()};
    let mut ret = unsafe {_ZNK15QRadialGradient6centerEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QRadialGradient::setCenter(const QPointF & center);
impl /*struct*/ QRadialGradient {
  pub fn setCenter<RetType, T: QRadialGradient_setCenter<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCenter(self);
    // return 1;
  }
}

pub trait QRadialGradient_setCenter<RetType> {
  fn setCenter(self , rsthis: &mut QRadialGradient) -> RetType;
}

  // proto:  void QRadialGradient::setCenter(const QPointF & center);
impl<'a> /*trait*/ QRadialGradient_setCenter<()> for (QPointF) {
  fn setCenter(self , rsthis: &mut QRadialGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradient9setCenterERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QRadialGradient9setCenterERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRadialGradient::QRadialGradient(const QPointF & center, qreal radius);
impl<'a> /*trait*/ QRadialGradient_NewQRadialGradient for (QPointF, f64) {
  fn NewQRadialGradient(self) -> QRadialGradient {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradientC1ERK7QPointFd()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    unsafe {_ZN15QRadialGradientC1ERK7QPointFd(qthis, arg0, arg1)};
    let rsthis = QRadialGradient{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QRadialGradient::setCenterRadius(qreal radius);
impl /*struct*/ QRadialGradient {
  pub fn setCenterRadius<RetType, T: QRadialGradient_setCenterRadius<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCenterRadius(self);
    // return 1;
  }
}

pub trait QRadialGradient_setCenterRadius<RetType> {
  fn setCenterRadius(self , rsthis: &mut QRadialGradient) -> RetType;
}

  // proto:  void QRadialGradient::setCenterRadius(qreal radius);
impl<'a> /*trait*/ QRadialGradient_setCenterRadius<()> for (f64) {
  fn setCenterRadius(self , rsthis: &mut QRadialGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradient15setCenterRadiusEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN15QRadialGradient15setCenterRadiusEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRadialGradient::setFocalRadius(qreal radius);
impl /*struct*/ QRadialGradient {
  pub fn setFocalRadius<RetType, T: QRadialGradient_setFocalRadius<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFocalRadius(self);
    // return 1;
  }
}

pub trait QRadialGradient_setFocalRadius<RetType> {
  fn setFocalRadius(self , rsthis: &mut QRadialGradient) -> RetType;
}

  // proto:  void QRadialGradient::setFocalRadius(qreal radius);
impl<'a> /*trait*/ QRadialGradient_setFocalRadius<()> for (f64) {
  fn setFocalRadius(self , rsthis: &mut QRadialGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradient14setFocalRadiusEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN15QRadialGradient14setFocalRadiusEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRadialGradient::setRadius(qreal radius);
impl /*struct*/ QRadialGradient {
  pub fn setRadius<RetType, T: QRadialGradient_setRadius<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setRadius(self);
    // return 1;
  }
}

pub trait QRadialGradient_setRadius<RetType> {
  fn setRadius(self , rsthis: &mut QRadialGradient) -> RetType;
}

  // proto:  void QRadialGradient::setRadius(qreal radius);
impl<'a> /*trait*/ QRadialGradient_setRadius<()> for (f64) {
  fn setRadius(self , rsthis: &mut QRadialGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradient9setRadiusEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN15QRadialGradient9setRadiusEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRadialGradient::QRadialGradient(qreal cx, qreal cy, qreal radius);
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

  // proto:  void QRadialGradient::setCenter(qreal x, qreal y);
impl<'a> /*trait*/ QRadialGradient_setCenter<()> for (f64, f64) {
  fn setCenter(self , rsthis: &mut QRadialGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradient9setCenterEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN15QRadialGradient9setCenterEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

