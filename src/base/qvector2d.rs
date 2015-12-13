// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qvector4d::QVector4D;
use super::qpoint::QPoint;
use super::qvector3d::QVector3D;
use super::qpointf::QPointF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QPointF QVector2D::toPointF();
  fn _ZNK9QVector2D8toPointFEv() -> i32;
  // proto: void QVector2D::setX(float x);
  fn _ZN9QVector2D4setXEf(arg0: c_float) -> i32;
  // proto: void QVector2D::NewQVector2D(const QVector4D & vector);
  fn _ZN9QVector2DC1ERK9QVector4D(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: QPoint QVector2D::toPoint();
  fn _ZNK9QVector2D7toPointEv() -> i32;
  // proto: float QVector2D::length();
  fn _ZNK9QVector2D6lengthEv() -> i32;
  // proto: void QVector2D::setY(float y);
  fn _ZN9QVector2D4setYEf(arg0: c_float) -> i32;
  // proto: void QVector2D::NewQVector2D(const QPoint & point);
  fn _ZN9QVector2DC1ERK6QPoint(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QVector2D::NewQVector2D(float xpos, float ypos);
  fn _ZN9QVector2DC1Eff(qthis: *mut c_void, arg0: c_float, arg1: c_float) -> i32;
  // proto: bool QVector2D::isNull();
  fn _ZNK9QVector2D6isNullEv() -> i32;
  // proto: float QVector2D::distanceToLine(const QVector2D & point, const QVector2D & direction);
  fn _ZNK9QVector2D14distanceToLineERKS_S1_(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QVector2D::NewQVector2D();
  fn _ZN9QVector2DC1Ev(qthis: *mut c_void) -> i32;
  // proto: QVector3D QVector2D::toVector3D();
  fn _ZNK9QVector2D10toVector3DEv() -> i32;
  // proto: float QVector2D::lengthSquared();
  fn _ZNK9QVector2D13lengthSquaredEv() -> i32;
  // proto: float QVector2D::y();
  fn _ZNK9QVector2D1yEv() -> i32;
  // proto: void QVector2D::NewQVector2D(const QVector3D & vector);
  fn _ZN9QVector2DC1ERK9QVector3D(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: float QVector2D::x();
  fn _ZNK9QVector2D1xEv() -> i32;
  // proto: void QVector2D::NewQVector2D(const QPointF & point);
  fn _ZN9QVector2DC1ERK7QPointF(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: float QVector2D::distanceToPoint(const QVector2D & point);
  fn _ZNK9QVector2D15distanceToPointERKS_(arg0: *const c_void) -> i32;
  // proto: QVector4D QVector2D::toVector4D();
  fn _ZNK9QVector2D10toVector4DEv() -> i32;
  // proto: QVector2D QVector2D::normalized();
  fn _ZNK9QVector2D10normalizedEv() -> i32;
  // proto: void QVector2D::normalize();
  fn _ZN9QVector2D9normalizeEv() -> i32;
  // proto: float QVector2D::dotProduct(const QVector2D & v1, const QVector2D & v2);
  fn _ZN9QVector2D10dotProductERKS_S1_(arg0: *const c_void, arg1: *const c_void) -> i32;
}

// body block begin
// class sizeof(QVector2D)=8
pub struct QVector2D {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QVector2D {
  pub fn toPointF<T: QVector2D_toPointF>(&mut self, value: T) -> i32 {
    value.toPointF(self);
    return 1;
  }
}

pub trait QVector2D_toPointF {
  fn toPointF(self, this: &mut QVector2D) -> i32;
}

// proto: QPointF QVector2D::toPointF();
impl<'a> /*trait*/ QVector2D_toPointF for () {
  fn toPointF(self, this: &mut QVector2D) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector2D8toPointFEv()};
    unsafe {_ZNK9QVector2D8toPointFEv()};
    return 1;
  }
}

impl /*struct*/ QVector2D {
  pub fn setX<T: QVector2D_setX>(&mut self, value: T) -> i32 {
    value.setX(self);
    return 1;
  }
}

pub trait QVector2D_setX {
  fn setX(self, this: &mut QVector2D) -> i32;
}

// proto: void QVector2D::setX(float x);
impl<'a> /*trait*/ QVector2D_setX for (f32) {
  fn setX(self, this: &mut QVector2D) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector2D4setXEf()};
    let arg0 = self  as c_float;
    unsafe {_ZN9QVector2D4setXEf(arg0)};
    return 1;
  }
}

impl /*struct*/ QVector2D {
  pub fn NewQVector2D<T: QVector2D_NewQVector2D>(value: T) -> QVector2D {
    let rsthis = value.NewQVector2D();
    return rsthis;
    // return 1;
  }
}

pub trait QVector2D_NewQVector2D {
  fn NewQVector2D(self) -> QVector2D;
}

// proto: void QVector2D::NewQVector2D(const QVector4D & vector);
impl<'a> /*trait*/ QVector2D_NewQVector2D for (&'a  QVector4D) {
  fn NewQVector2D(self) -> QVector2D {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector2DC1ERK9QVector4D()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QVector2DC1ERK9QVector4D(qthis, arg0)};
    let rsthis = QVector2D{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVector2D {
  pub fn toPoint<T: QVector2D_toPoint>(&mut self, value: T) -> i32 {
    value.toPoint(self);
    return 1;
  }
}

pub trait QVector2D_toPoint {
  fn toPoint(self, this: &mut QVector2D) -> i32;
}

// proto: QPoint QVector2D::toPoint();
impl<'a> /*trait*/ QVector2D_toPoint for () {
  fn toPoint(self, this: &mut QVector2D) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector2D7toPointEv()};
    unsafe {_ZNK9QVector2D7toPointEv()};
    return 1;
  }
}

impl /*struct*/ QVector2D {
  pub fn length<T: QVector2D_length>(&mut self, value: T) -> i32 {
    value.length(self);
    return 1;
  }
}

pub trait QVector2D_length {
  fn length(self, this: &mut QVector2D) -> i32;
}

// proto: float QVector2D::length();
impl<'a> /*trait*/ QVector2D_length for () {
  fn length(self, this: &mut QVector2D) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector2D6lengthEv()};
    unsafe {_ZNK9QVector2D6lengthEv()};
    return 1;
  }
}

impl /*struct*/ QVector2D {
  pub fn setY<T: QVector2D_setY>(&mut self, value: T) -> i32 {
    value.setY(self);
    return 1;
  }
}

pub trait QVector2D_setY {
  fn setY(self, this: &mut QVector2D) -> i32;
}

// proto: void QVector2D::setY(float y);
impl<'a> /*trait*/ QVector2D_setY for (f32) {
  fn setY(self, this: &mut QVector2D) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector2D4setYEf()};
    let arg0 = self  as c_float;
    unsafe {_ZN9QVector2D4setYEf(arg0)};
    return 1;
  }
}

// proto: void QVector2D::NewQVector2D(const QPoint & point);
impl<'a> /*trait*/ QVector2D_NewQVector2D for (&'a  QPoint) {
  fn NewQVector2D(self) -> QVector2D {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector2DC1ERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QVector2DC1ERK6QPoint(qthis, arg0)};
    let rsthis = QVector2D{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QVector2D::NewQVector2D(float xpos, float ypos);
impl<'a> /*trait*/ QVector2D_NewQVector2D for (f32, f32) {
  fn NewQVector2D(self) -> QVector2D {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector2DC1Eff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
    unsafe {_ZN9QVector2DC1Eff(qthis, arg0, arg1)};
    let rsthis = QVector2D{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVector2D {
  pub fn isNull<T: QVector2D_isNull>(&mut self, value: T) -> i32 {
    value.isNull(self);
    return 1;
  }
}

pub trait QVector2D_isNull {
  fn isNull(self, this: &mut QVector2D) -> i32;
}

// proto: bool QVector2D::isNull();
impl<'a> /*trait*/ QVector2D_isNull for () {
  fn isNull(self, this: &mut QVector2D) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector2D6isNullEv()};
    unsafe {_ZNK9QVector2D6isNullEv()};
    return 1;
  }
}

impl /*struct*/ QVector2D {
  pub fn distanceToLine<T: QVector2D_distanceToLine>(&mut self, value: T) -> i32 {
    value.distanceToLine(self);
    return 1;
  }
}

pub trait QVector2D_distanceToLine {
  fn distanceToLine(self, this: &mut QVector2D) -> i32;
}

// proto: float QVector2D::distanceToLine(const QVector2D & point, const QVector2D & direction);
impl<'a> /*trait*/ QVector2D_distanceToLine for (&'a  QVector2D, &'a  QVector2D) {
  fn distanceToLine(self, this: &mut QVector2D) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector2D14distanceToLineERKS_S1_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK9QVector2D14distanceToLineERKS_S1_(arg0, arg1)};
    return 1;
  }
}

// proto: void QVector2D::NewQVector2D();
impl<'a> /*trait*/ QVector2D_NewQVector2D for () {
  fn NewQVector2D(self) -> QVector2D {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector2DC1Ev()};
    unsafe {_ZN9QVector2DC1Ev(qthis)};
    let rsthis = QVector2D{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVector2D {
  pub fn toVector3D<T: QVector2D_toVector3D>(&mut self, value: T) -> i32 {
    value.toVector3D(self);
    return 1;
  }
}

pub trait QVector2D_toVector3D {
  fn toVector3D(self, this: &mut QVector2D) -> i32;
}

// proto: QVector3D QVector2D::toVector3D();
impl<'a> /*trait*/ QVector2D_toVector3D for () {
  fn toVector3D(self, this: &mut QVector2D) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector2D10toVector3DEv()};
    unsafe {_ZNK9QVector2D10toVector3DEv()};
    return 1;
  }
}

impl /*struct*/ QVector2D {
  pub fn lengthSquared<T: QVector2D_lengthSquared>(&mut self, value: T) -> i32 {
    value.lengthSquared(self);
    return 1;
  }
}

pub trait QVector2D_lengthSquared {
  fn lengthSquared(self, this: &mut QVector2D) -> i32;
}

// proto: float QVector2D::lengthSquared();
impl<'a> /*trait*/ QVector2D_lengthSquared for () {
  fn lengthSquared(self, this: &mut QVector2D) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector2D13lengthSquaredEv()};
    unsafe {_ZNK9QVector2D13lengthSquaredEv()};
    return 1;
  }
}

impl /*struct*/ QVector2D {
  pub fn y<T: QVector2D_y>(&mut self, value: T) -> i32 {
    value.y(self);
    return 1;
  }
}

pub trait QVector2D_y {
  fn y(self, this: &mut QVector2D) -> i32;
}

// proto: float QVector2D::y();
impl<'a> /*trait*/ QVector2D_y for () {
  fn y(self, this: &mut QVector2D) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector2D1yEv()};
    unsafe {_ZNK9QVector2D1yEv()};
    return 1;
  }
}

// proto: void QVector2D::NewQVector2D(const QVector3D & vector);
impl<'a> /*trait*/ QVector2D_NewQVector2D for (&'a  QVector3D) {
  fn NewQVector2D(self) -> QVector2D {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector2DC1ERK9QVector3D()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QVector2DC1ERK9QVector3D(qthis, arg0)};
    let rsthis = QVector2D{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVector2D {
  pub fn x<T: QVector2D_x>(&mut self, value: T) -> i32 {
    value.x(self);
    return 1;
  }
}

pub trait QVector2D_x {
  fn x(self, this: &mut QVector2D) -> i32;
}

// proto: float QVector2D::x();
impl<'a> /*trait*/ QVector2D_x for () {
  fn x(self, this: &mut QVector2D) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector2D1xEv()};
    unsafe {_ZNK9QVector2D1xEv()};
    return 1;
  }
}

// proto: void QVector2D::NewQVector2D(const QPointF & point);
impl<'a> /*trait*/ QVector2D_NewQVector2D for (&'a  QPointF) {
  fn NewQVector2D(self) -> QVector2D {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector2DC1ERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QVector2DC1ERK7QPointF(qthis, arg0)};
    let rsthis = QVector2D{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVector2D {
  pub fn distanceToPoint<T: QVector2D_distanceToPoint>(&mut self, value: T) -> i32 {
    value.distanceToPoint(self);
    return 1;
  }
}

pub trait QVector2D_distanceToPoint {
  fn distanceToPoint(self, this: &mut QVector2D) -> i32;
}

// proto: float QVector2D::distanceToPoint(const QVector2D & point);
impl<'a> /*trait*/ QVector2D_distanceToPoint for (&'a  QVector2D) {
  fn distanceToPoint(self, this: &mut QVector2D) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector2D15distanceToPointERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK9QVector2D15distanceToPointERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QVector2D {
  pub fn toVector4D<T: QVector2D_toVector4D>(&mut self, value: T) -> i32 {
    value.toVector4D(self);
    return 1;
  }
}

pub trait QVector2D_toVector4D {
  fn toVector4D(self, this: &mut QVector2D) -> i32;
}

// proto: QVector4D QVector2D::toVector4D();
impl<'a> /*trait*/ QVector2D_toVector4D for () {
  fn toVector4D(self, this: &mut QVector2D) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector2D10toVector4DEv()};
    unsafe {_ZNK9QVector2D10toVector4DEv()};
    return 1;
  }
}

impl /*struct*/ QVector2D {
  pub fn normalized<T: QVector2D_normalized>(&mut self, value: T) -> i32 {
    value.normalized(self);
    return 1;
  }
}

pub trait QVector2D_normalized {
  fn normalized(self, this: &mut QVector2D) -> i32;
}

// proto: QVector2D QVector2D::normalized();
impl<'a> /*trait*/ QVector2D_normalized for () {
  fn normalized(self, this: &mut QVector2D) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector2D10normalizedEv()};
    unsafe {_ZNK9QVector2D10normalizedEv()};
    return 1;
  }
}

impl /*struct*/ QVector2D {
  pub fn normalize<T: QVector2D_normalize>(&mut self, value: T) -> i32 {
    value.normalize(self);
    return 1;
  }
}

pub trait QVector2D_normalize {
  fn normalize(self, this: &mut QVector2D) -> i32;
}

// proto: void QVector2D::normalize();
impl<'a> /*trait*/ QVector2D_normalize for () {
  fn normalize(self, this: &mut QVector2D) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector2D9normalizeEv()};
    unsafe {_ZN9QVector2D9normalizeEv()};
    return 1;
  }
}

impl /*struct*/ QVector2D {
  pub fn dotProduct<T: QVector2D_dotProduct>(&mut self, value: T) -> i32 {
    value.dotProduct(self);
    return 1;
  }
}

pub trait QVector2D_dotProduct {
  fn dotProduct(self, this: &mut QVector2D) -> i32;
}

// proto: float QVector2D::dotProduct(const QVector2D & v1, const QVector2D & v2);
impl<'a> /*trait*/ QVector2D_dotProduct for (&'a  QVector2D, &'a  QVector2D) {
  fn dotProduct(self, this: &mut QVector2D) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector2D10dotProductERKS_S1_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN9QVector2D10dotProductERKS_S1_(arg0, arg1)};
    return 1;
  }
}

