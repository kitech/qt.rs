// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpointf::QPointF;
use super::qvector4d::QVector4D;
use super::qpoint::QPoint;
use super::qvector3d::QVector3D;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QPointF QVector2D::toPointF();
  fn _ZNK9QVector2D8toPointFEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QVector2D::setX(float x);
  fn _ZN9QVector2D4setXEf(qthis: *mut c_void, arg0: c_float) ;
  // proto:  void QVector2D::NewQVector2D(const QVector4D & vector);
  fn _ZN9QVector2DC1ERK9QVector4D(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QPoint QVector2D::toPoint();
  fn _ZNK9QVector2D7toPointEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  float QVector2D::length();
  fn _ZNK9QVector2D6lengthEv(qthis: *mut c_void) -> c_float;
  // proto:  void QVector2D::setY(float y);
  fn _ZN9QVector2D4setYEf(qthis: *mut c_void, arg0: c_float) ;
  // proto:  void QVector2D::NewQVector2D(const QPoint & point);
  fn _ZN9QVector2DC1ERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QVector2D::NewQVector2D(float xpos, float ypos);
  fn _ZN9QVector2DC1Eff(qthis: *mut c_void, arg0: c_float, arg1: c_float) ;
  // proto:  bool QVector2D::isNull();
  fn _ZNK9QVector2D6isNullEv(qthis: *mut c_void) -> int8_t;
  // proto:  float QVector2D::distanceToLine(const QVector2D & point, const QVector2D & direction);
  fn _ZNK9QVector2D14distanceToLineERKS_S1_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> c_float;
  // proto:  void QVector2D::NewQVector2D();
  fn _ZN9QVector2DC1Ev(qthis: *mut c_void) ;
  // proto:  QVector3D QVector2D::toVector3D();
  fn _ZNK9QVector2D10toVector3DEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  float QVector2D::lengthSquared();
  fn _ZNK9QVector2D13lengthSquaredEv(qthis: *mut c_void) -> c_float;
  // proto:  float QVector2D::y();
  fn _ZNK9QVector2D1yEv(qthis: *mut c_void) -> c_float;
  // proto:  void QVector2D::NewQVector2D(const QVector3D & vector);
  fn _ZN9QVector2DC1ERK9QVector3D(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  float QVector2D::x();
  fn _ZNK9QVector2D1xEv(qthis: *mut c_void) -> c_float;
  // proto:  void QVector2D::NewQVector2D(const QPointF & point);
  fn _ZN9QVector2DC1ERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  float QVector2D::distanceToPoint(const QVector2D & point);
  fn _ZNK9QVector2D15distanceToPointERKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_float;
  // proto:  QVector4D QVector2D::toVector4D();
  fn _ZNK9QVector2D10toVector4DEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QVector2D QVector2D::normalized();
  fn _ZNK9QVector2D10normalizedEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QVector2D::normalize();
  fn _ZN9QVector2D9normalizeEv(qthis: *mut c_void) ;
  // proto: static float QVector2D::dotProduct(const QVector2D & v1, const QVector2D & v2);
  fn _ZN9QVector2D10dotProductERKS_S1_(arg0: *mut c_void, arg1: *mut c_void) -> c_float;
}

// body block begin
// class sizeof(QVector2D)=8
pub struct QVector2D {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QVector2D {
  pub fn toPointF<T: QVector2D_toPointF>(&mut self, value: T) -> QPointF {
    return value.toPointF(self);
    // return 1;
  }
}

pub trait QVector2D_toPointF {
  fn toPointF(self, rsthis: &mut QVector2D) -> QPointF;
}

// proto:  QPointF QVector2D::toPointF();
impl<'a> /*trait*/ QVector2D_toPointF for () {
  fn toPointF(self, rsthis: &mut QVector2D) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector2D8toPointFEv()};
    let mut ret = unsafe {_ZNK9QVector2D8toPointFEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QVector2D {
  pub fn setX<T: QVector2D_setX>(&mut self, value: T)  {
     value.setX(self);
    // return 1;
  }
}

pub trait QVector2D_setX {
  fn setX(self, rsthis: &mut QVector2D) ;
}

// proto:  void QVector2D::setX(float x);
impl<'a> /*trait*/ QVector2D_setX for (f32) {
  fn setX(self, rsthis: &mut QVector2D)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector2D4setXEf()};
    let arg0 = self  as c_float;
     unsafe {_ZN9QVector2D4setXEf(rsthis.qclsinst, arg0)};
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QVector2DC1ERK9QVector4D(qthis, arg0)};
    let rsthis = QVector2D{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVector2D {
  pub fn toPoint<T: QVector2D_toPoint>(&mut self, value: T) -> QPoint {
    return value.toPoint(self);
    // return 1;
  }
}

pub trait QVector2D_toPoint {
  fn toPoint(self, rsthis: &mut QVector2D) -> QPoint;
}

// proto:  QPoint QVector2D::toPoint();
impl<'a> /*trait*/ QVector2D_toPoint for () {
  fn toPoint(self, rsthis: &mut QVector2D) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector2D7toPointEv()};
    let mut ret = unsafe {_ZNK9QVector2D7toPointEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QVector2D {
  pub fn length<T: QVector2D_length>(&mut self, value: T) -> f32 {
    return value.length(self);
    // return 1;
  }
}

pub trait QVector2D_length {
  fn length(self, rsthis: &mut QVector2D) -> f32;
}

// proto:  float QVector2D::length();
impl<'a> /*trait*/ QVector2D_length for () {
  fn length(self, rsthis: &mut QVector2D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector2D6lengthEv()};
    let mut ret = unsafe {_ZNK9QVector2D6lengthEv(rsthis.qclsinst)};
    return ret as f32;
    // return 1;
  }
}

impl /*struct*/ QVector2D {
  pub fn setY<T: QVector2D_setY>(&mut self, value: T)  {
     value.setY(self);
    // return 1;
  }
}

pub trait QVector2D_setY {
  fn setY(self, rsthis: &mut QVector2D) ;
}

// proto:  void QVector2D::setY(float y);
impl<'a> /*trait*/ QVector2D_setY for (f32) {
  fn setY(self, rsthis: &mut QVector2D)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector2D4setYEf()};
    let arg0 = self  as c_float;
     unsafe {_ZN9QVector2D4setYEf(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QVector2D::NewQVector2D(const QPoint & point);
impl<'a> /*trait*/ QVector2D_NewQVector2D for (&'a  QPoint) {
  fn NewQVector2D(self) -> QVector2D {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector2DC1ERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
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
  pub fn isNull<T: QVector2D_isNull>(&mut self, value: T) -> i8 {
    return value.isNull(self);
    // return 1;
  }
}

pub trait QVector2D_isNull {
  fn isNull(self, rsthis: &mut QVector2D) -> i8;
}

// proto:  bool QVector2D::isNull();
impl<'a> /*trait*/ QVector2D_isNull for () {
  fn isNull(self, rsthis: &mut QVector2D) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector2D6isNullEv()};
    let mut ret = unsafe {_ZNK9QVector2D6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QVector2D {
  pub fn distanceToLine<T: QVector2D_distanceToLine>(&mut self, value: T) -> f32 {
    return value.distanceToLine(self);
    // return 1;
  }
}

pub trait QVector2D_distanceToLine {
  fn distanceToLine(self, rsthis: &mut QVector2D) -> f32;
}

// proto:  float QVector2D::distanceToLine(const QVector2D & point, const QVector2D & direction);
impl<'a> /*trait*/ QVector2D_distanceToLine for (&'a  QVector2D, &'a  QVector2D) {
  fn distanceToLine(self, rsthis: &mut QVector2D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector2D14distanceToLineERKS_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QVector2D14distanceToLineERKS_S1_(rsthis.qclsinst, arg0, arg1)};
    return ret as f32;
    // return 1;
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
  pub fn toVector3D<T: QVector2D_toVector3D>(&mut self, value: T) -> QVector3D {
    return value.toVector3D(self);
    // return 1;
  }
}

pub trait QVector2D_toVector3D {
  fn toVector3D(self, rsthis: &mut QVector2D) -> QVector3D;
}

// proto:  QVector3D QVector2D::toVector3D();
impl<'a> /*trait*/ QVector2D_toVector3D for () {
  fn toVector3D(self, rsthis: &mut QVector2D) -> QVector3D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector2D10toVector3DEv()};
    let mut ret = unsafe {_ZNK9QVector2D10toVector3DEv(rsthis.qclsinst)};
    let mut ret1 = QVector3D{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QVector2D {
  pub fn lengthSquared<T: QVector2D_lengthSquared>(&mut self, value: T) -> f32 {
    return value.lengthSquared(self);
    // return 1;
  }
}

pub trait QVector2D_lengthSquared {
  fn lengthSquared(self, rsthis: &mut QVector2D) -> f32;
}

// proto:  float QVector2D::lengthSquared();
impl<'a> /*trait*/ QVector2D_lengthSquared for () {
  fn lengthSquared(self, rsthis: &mut QVector2D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector2D13lengthSquaredEv()};
    let mut ret = unsafe {_ZNK9QVector2D13lengthSquaredEv(rsthis.qclsinst)};
    return ret as f32;
    // return 1;
  }
}

impl /*struct*/ QVector2D {
  pub fn y<T: QVector2D_y>(&mut self, value: T) -> f32 {
    return value.y(self);
    // return 1;
  }
}

pub trait QVector2D_y {
  fn y(self, rsthis: &mut QVector2D) -> f32;
}

// proto:  float QVector2D::y();
impl<'a> /*trait*/ QVector2D_y for () {
  fn y(self, rsthis: &mut QVector2D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector2D1yEv()};
    let mut ret = unsafe {_ZNK9QVector2D1yEv(rsthis.qclsinst)};
    return ret as f32;
    // return 1;
  }
}

// proto: void QVector2D::NewQVector2D(const QVector3D & vector);
impl<'a> /*trait*/ QVector2D_NewQVector2D for (&'a  QVector3D) {
  fn NewQVector2D(self) -> QVector2D {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector2DC1ERK9QVector3D()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QVector2DC1ERK9QVector3D(qthis, arg0)};
    let rsthis = QVector2D{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVector2D {
  pub fn x<T: QVector2D_x>(&mut self, value: T) -> f32 {
    return value.x(self);
    // return 1;
  }
}

pub trait QVector2D_x {
  fn x(self, rsthis: &mut QVector2D) -> f32;
}

// proto:  float QVector2D::x();
impl<'a> /*trait*/ QVector2D_x for () {
  fn x(self, rsthis: &mut QVector2D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector2D1xEv()};
    let mut ret = unsafe {_ZNK9QVector2D1xEv(rsthis.qclsinst)};
    return ret as f32;
    // return 1;
  }
}

// proto: void QVector2D::NewQVector2D(const QPointF & point);
impl<'a> /*trait*/ QVector2D_NewQVector2D for (&'a  QPointF) {
  fn NewQVector2D(self) -> QVector2D {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector2DC1ERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QVector2DC1ERK7QPointF(qthis, arg0)};
    let rsthis = QVector2D{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVector2D {
  pub fn distanceToPoint<T: QVector2D_distanceToPoint>(&mut self, value: T) -> f32 {
    return value.distanceToPoint(self);
    // return 1;
  }
}

pub trait QVector2D_distanceToPoint {
  fn distanceToPoint(self, rsthis: &mut QVector2D) -> f32;
}

// proto:  float QVector2D::distanceToPoint(const QVector2D & point);
impl<'a> /*trait*/ QVector2D_distanceToPoint for (&'a  QVector2D) {
  fn distanceToPoint(self, rsthis: &mut QVector2D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector2D15distanceToPointERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QVector2D15distanceToPointERKS_(rsthis.qclsinst, arg0)};
    return ret as f32;
    // return 1;
  }
}

impl /*struct*/ QVector2D {
  pub fn toVector4D<T: QVector2D_toVector4D>(&mut self, value: T) -> QVector4D {
    return value.toVector4D(self);
    // return 1;
  }
}

pub trait QVector2D_toVector4D {
  fn toVector4D(self, rsthis: &mut QVector2D) -> QVector4D;
}

// proto:  QVector4D QVector2D::toVector4D();
impl<'a> /*trait*/ QVector2D_toVector4D for () {
  fn toVector4D(self, rsthis: &mut QVector2D) -> QVector4D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector2D10toVector4DEv()};
    let mut ret = unsafe {_ZNK9QVector2D10toVector4DEv(rsthis.qclsinst)};
    let mut ret1 = QVector4D{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QVector2D {
  pub fn normalized<T: QVector2D_normalized>(&mut self, value: T) -> QVector2D {
    return value.normalized(self);
    // return 1;
  }
}

pub trait QVector2D_normalized {
  fn normalized(self, rsthis: &mut QVector2D) -> QVector2D;
}

// proto:  QVector2D QVector2D::normalized();
impl<'a> /*trait*/ QVector2D_normalized for () {
  fn normalized(self, rsthis: &mut QVector2D) -> QVector2D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector2D10normalizedEv()};
    let mut ret = unsafe {_ZNK9QVector2D10normalizedEv(rsthis.qclsinst)};
    let mut ret1 = QVector2D{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QVector2D {
  pub fn normalize<T: QVector2D_normalize>(&mut self, value: T)  {
     value.normalize(self);
    // return 1;
  }
}

pub trait QVector2D_normalize {
  fn normalize(self, rsthis: &mut QVector2D) ;
}

// proto:  void QVector2D::normalize();
impl<'a> /*trait*/ QVector2D_normalize for () {
  fn normalize(self, rsthis: &mut QVector2D)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector2D9normalizeEv()};
     unsafe {_ZN9QVector2D9normalizeEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QVector2D {
  pub fn dotProduct<T: QVector2D_dotProduct>(&mut self, value: T) -> f32 {
    return value.dotProduct(self);
    // return 1;
  }
}

pub trait QVector2D_dotProduct {
  fn dotProduct(self, rsthis: &mut QVector2D) -> f32;
}

// proto: static float QVector2D::dotProduct(const QVector2D & v1, const QVector2D & v2);
impl<'a> /*trait*/ QVector2D_dotProduct for (&'a  QVector2D, &'a  QVector2D) {
  fn dotProduct(self, rsthis: &mut QVector2D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector2D10dotProductERKS_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QVector2D10dotProductERKS_S1_(arg0, arg1)};
    return ret as f32;
    // return 1;
  }
}

