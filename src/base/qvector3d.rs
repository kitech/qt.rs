// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpoint::QPoint;
use super::qpointf::QPointF;
use super::qmatrix4x4::QMatrix4x4;
use super::qrect::QRect;
use super::qvector2d::QVector2D;
use super::qvector4d::QVector4D;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  float QVector3D::x();
  fn _ZNK9QVector3D1xEv(qthis: *mut c_void) -> c_float;
  // proto:  QPoint QVector3D::toPoint();
  fn _ZNK9QVector3D7toPointEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  float QVector3D::distanceToLine(const QVector3D & point, const QVector3D & direction);
  fn _ZNK9QVector3D14distanceToLineERKS_S1_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> c_float;
  // proto:  float QVector3D::y();
  fn _ZNK9QVector3D1yEv(qthis: *mut c_void) -> c_float;
  // proto: static QVector3D QVector3D::normal(const QVector3D & v1, const QVector3D & v2);
  fn _ZN9QVector3D6normalERKS_S1_(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QPointF QVector3D::toPointF();
  fn _ZNK9QVector3D8toPointFEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QVector3D::normalize();
  fn _ZN9QVector3D9normalizeEv(qthis: *mut c_void) ;
  // proto:  QVector3D QVector3D::unproject(const QMatrix4x4 & modelView, const QMatrix4x4 & projection, const QRect & viewport);
  fn _ZNK9QVector3D9unprojectERK10QMatrix4x4S2_RK5QRect(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;
  // proto:  void QVector3D::setY(float y);
  fn _ZN9QVector3D4setYEf(qthis: *mut c_void, arg0: c_float) ;
  // proto:  QVector3D QVector3D::project(const QMatrix4x4 & modelView, const QMatrix4x4 & projection, const QRect & viewport);
  fn _ZNK9QVector3D7projectERK10QMatrix4x4S2_RK5QRect(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;
  // proto: static QVector3D QVector3D::crossProduct(const QVector3D & v1, const QVector3D & v2);
  fn _ZN9QVector3D12crossProductERKS_S1_(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  float QVector3D::z();
  fn _ZNK9QVector3D1zEv(qthis: *mut c_void) -> c_float;
  // proto:  void QVector3D::setZ(float z);
  fn _ZN9QVector3D4setZEf(qthis: *mut c_void, arg0: c_float) ;
  // proto:  QVector2D QVector3D::toVector2D();
  fn _ZNK9QVector3D10toVector2DEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  float QVector3D::distanceToPlane(const QVector3D & plane1, const QVector3D & plane2, const QVector3D & plane3);
  fn _ZNK9QVector3D15distanceToPlaneERKS_S1_S1_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> c_float;
  // proto:  void QVector3D::NewQVector3D(const QPointF & point);
  fn _ZN9QVector3DC1ERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  float QVector3D::distanceToPoint(const QVector3D & point);
  fn _ZNK9QVector3D15distanceToPointERKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_float;
  // proto:  void QVector3D::NewQVector3D(const QVector2D & vector);
  fn _ZN9QVector3DC1ERK9QVector2D(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QVector3D::NewQVector3D(const QPoint & point);
  fn _ZN9QVector3DC1ERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  float QVector3D::lengthSquared();
  fn _ZNK9QVector3D13lengthSquaredEv(qthis: *mut c_void) -> c_float;
  // proto: static QVector3D QVector3D::normal(const QVector3D & v1, const QVector3D & v2, const QVector3D & v3);
  fn _ZN9QVector3D6normalERKS_S1_S1_(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;
  // proto:  float QVector3D::distanceToPlane(const QVector3D & plane, const QVector3D & normal);
  fn _ZNK9QVector3D15distanceToPlaneERKS_S1_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> c_float;
  // proto:  QVector3D QVector3D::normalized();
  fn _ZNK9QVector3D10normalizedEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QVector3D::NewQVector3D(const QVector4D & vector);
  fn _ZN9QVector3DC1ERK9QVector4D(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QVector3D::isNull();
  fn _ZNK9QVector3D6isNullEv(qthis: *mut c_void) -> int8_t;
  // proto:  float QVector3D::length();
  fn _ZNK9QVector3D6lengthEv(qthis: *mut c_void) -> c_float;
  // proto: static float QVector3D::dotProduct(const QVector3D & v1, const QVector3D & v2);
  fn _ZN9QVector3D10dotProductERKS_S1_(arg0: *mut c_void, arg1: *mut c_void) -> c_float;
  // proto:  void QVector3D::NewQVector3D();
  fn _ZN9QVector3DC1Ev(qthis: *mut c_void) ;
  // proto:  void QVector3D::NewQVector3D(float xpos, float ypos, float zpos);
  fn _ZN9QVector3DC1Efff(qthis: *mut c_void, arg0: c_float, arg1: c_float, arg2: c_float) ;
  // proto:  QVector4D QVector3D::toVector4D();
  fn _ZNK9QVector3D10toVector4DEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QVector3D::NewQVector3D(const QVector2D & vector, float zpos);
  fn _ZN9QVector3DC1ERK9QVector2Df(qthis: *mut c_void, arg0: *mut c_void, arg1: c_float) ;
  // proto:  void QVector3D::setX(float x);
  fn _ZN9QVector3D4setXEf(qthis: *mut c_void, arg0: c_float) ;
}

// body block begin
// class sizeof(QVector3D)=12
pub struct QVector3D {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QVector3D {
  pub fn x<T: QVector3D_x>(&mut self, value: T) -> f32 {
    return value.x(self);
    // return 1;
  }
}

pub trait QVector3D_x {
  fn x(self, rsthis: &mut QVector3D) -> f32;
}

// proto:  float QVector3D::x();
impl<'a> /*trait*/ QVector3D_x for () {
  fn x(self, rsthis: &mut QVector3D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector3D1xEv()};
    let mut ret = unsafe {_ZNK9QVector3D1xEv(rsthis.qclsinst)};
    return ret as f32;
    // return 1;
  }
}

impl /*struct*/ QVector3D {
  pub fn toPoint<T: QVector3D_toPoint>(&mut self, value: T) -> QPoint {
    return value.toPoint(self);
    // return 1;
  }
}

pub trait QVector3D_toPoint {
  fn toPoint(self, rsthis: &mut QVector3D) -> QPoint;
}

// proto:  QPoint QVector3D::toPoint();
impl<'a> /*trait*/ QVector3D_toPoint for () {
  fn toPoint(self, rsthis: &mut QVector3D) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector3D7toPointEv()};
    let mut ret = unsafe {_ZNK9QVector3D7toPointEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QVector3D {
  pub fn distanceToLine<T: QVector3D_distanceToLine>(&mut self, value: T) -> f32 {
    return value.distanceToLine(self);
    // return 1;
  }
}

pub trait QVector3D_distanceToLine {
  fn distanceToLine(self, rsthis: &mut QVector3D) -> f32;
}

// proto:  float QVector3D::distanceToLine(const QVector3D & point, const QVector3D & direction);
impl<'a> /*trait*/ QVector3D_distanceToLine for (&'a  QVector3D, &'a  QVector3D) {
  fn distanceToLine(self, rsthis: &mut QVector3D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector3D14distanceToLineERKS_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QVector3D14distanceToLineERKS_S1_(rsthis.qclsinst, arg0, arg1)};
    return ret as f32;
    // return 1;
  }
}

impl /*struct*/ QVector3D {
  pub fn y<T: QVector3D_y>(&mut self, value: T) -> f32 {
    return value.y(self);
    // return 1;
  }
}

pub trait QVector3D_y {
  fn y(self, rsthis: &mut QVector3D) -> f32;
}

// proto:  float QVector3D::y();
impl<'a> /*trait*/ QVector3D_y for () {
  fn y(self, rsthis: &mut QVector3D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector3D1yEv()};
    let mut ret = unsafe {_ZNK9QVector3D1yEv(rsthis.qclsinst)};
    return ret as f32;
    // return 1;
  }
}

impl /*struct*/ QVector3D {
  pub fn normal<T: QVector3D_normal>(&mut self, value: T) -> QVector3D {
    return value.normal(self);
    // return 1;
  }
}

pub trait QVector3D_normal {
  fn normal(self, rsthis: &mut QVector3D) -> QVector3D;
}

// proto: static QVector3D QVector3D::normal(const QVector3D & v1, const QVector3D & v2);
impl<'a> /*trait*/ QVector3D_normal for (&'a  QVector3D, &'a  QVector3D) {
  fn normal(self, rsthis: &mut QVector3D) -> QVector3D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector3D6normalERKS_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QVector3D6normalERKS_S1_(arg0, arg1)};
    let mut ret1 = QVector3D{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QVector3D {
  pub fn toPointF<T: QVector3D_toPointF>(&mut self, value: T) -> QPointF {
    return value.toPointF(self);
    // return 1;
  }
}

pub trait QVector3D_toPointF {
  fn toPointF(self, rsthis: &mut QVector3D) -> QPointF;
}

// proto:  QPointF QVector3D::toPointF();
impl<'a> /*trait*/ QVector3D_toPointF for () {
  fn toPointF(self, rsthis: &mut QVector3D) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector3D8toPointFEv()};
    let mut ret = unsafe {_ZNK9QVector3D8toPointFEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QVector3D {
  pub fn normalize<T: QVector3D_normalize>(&mut self, value: T)  {
     value.normalize(self);
    // return 1;
  }
}

pub trait QVector3D_normalize {
  fn normalize(self, rsthis: &mut QVector3D) ;
}

// proto:  void QVector3D::normalize();
impl<'a> /*trait*/ QVector3D_normalize for () {
  fn normalize(self, rsthis: &mut QVector3D)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector3D9normalizeEv()};
     unsafe {_ZN9QVector3D9normalizeEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QVector3D {
  pub fn unproject<T: QVector3D_unproject>(&mut self, value: T) -> QVector3D {
    return value.unproject(self);
    // return 1;
  }
}

pub trait QVector3D_unproject {
  fn unproject(self, rsthis: &mut QVector3D) -> QVector3D;
}

// proto:  QVector3D QVector3D::unproject(const QMatrix4x4 & modelView, const QMatrix4x4 & projection, const QRect & viewport);
impl<'a> /*trait*/ QVector3D_unproject for (&'a  QMatrix4x4, &'a  QMatrix4x4, &'a  QRect) {
  fn unproject(self, rsthis: &mut QVector3D) -> QVector3D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector3D9unprojectERK10QMatrix4x4S2_RK5QRect()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QVector3D9unprojectERK10QMatrix4x4S2_RK5QRect(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QVector3D{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QVector3D {
  pub fn setY<T: QVector3D_setY>(&mut self, value: T)  {
     value.setY(self);
    // return 1;
  }
}

pub trait QVector3D_setY {
  fn setY(self, rsthis: &mut QVector3D) ;
}

// proto:  void QVector3D::setY(float y);
impl<'a> /*trait*/ QVector3D_setY for (f32) {
  fn setY(self, rsthis: &mut QVector3D)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector3D4setYEf()};
    let arg0 = self  as c_float;
     unsafe {_ZN9QVector3D4setYEf(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QVector3D {
  pub fn project<T: QVector3D_project>(&mut self, value: T) -> QVector3D {
    return value.project(self);
    // return 1;
  }
}

pub trait QVector3D_project {
  fn project(self, rsthis: &mut QVector3D) -> QVector3D;
}

// proto:  QVector3D QVector3D::project(const QMatrix4x4 & modelView, const QMatrix4x4 & projection, const QRect & viewport);
impl<'a> /*trait*/ QVector3D_project for (&'a  QMatrix4x4, &'a  QMatrix4x4, &'a  QRect) {
  fn project(self, rsthis: &mut QVector3D) -> QVector3D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector3D7projectERK10QMatrix4x4S2_RK5QRect()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QVector3D7projectERK10QMatrix4x4S2_RK5QRect(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QVector3D{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QVector3D {
  pub fn crossProduct<T: QVector3D_crossProduct>(&mut self, value: T) -> QVector3D {
    return value.crossProduct(self);
    // return 1;
  }
}

pub trait QVector3D_crossProduct {
  fn crossProduct(self, rsthis: &mut QVector3D) -> QVector3D;
}

// proto: static QVector3D QVector3D::crossProduct(const QVector3D & v1, const QVector3D & v2);
impl<'a> /*trait*/ QVector3D_crossProduct for (&'a  QVector3D, &'a  QVector3D) {
  fn crossProduct(self, rsthis: &mut QVector3D) -> QVector3D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector3D12crossProductERKS_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QVector3D12crossProductERKS_S1_(arg0, arg1)};
    let mut ret1 = QVector3D{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QVector3D {
  pub fn z<T: QVector3D_z>(&mut self, value: T) -> f32 {
    return value.z(self);
    // return 1;
  }
}

pub trait QVector3D_z {
  fn z(self, rsthis: &mut QVector3D) -> f32;
}

// proto:  float QVector3D::z();
impl<'a> /*trait*/ QVector3D_z for () {
  fn z(self, rsthis: &mut QVector3D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector3D1zEv()};
    let mut ret = unsafe {_ZNK9QVector3D1zEv(rsthis.qclsinst)};
    return ret as f32;
    // return 1;
  }
}

impl /*struct*/ QVector3D {
  pub fn setZ<T: QVector3D_setZ>(&mut self, value: T)  {
     value.setZ(self);
    // return 1;
  }
}

pub trait QVector3D_setZ {
  fn setZ(self, rsthis: &mut QVector3D) ;
}

// proto:  void QVector3D::setZ(float z);
impl<'a> /*trait*/ QVector3D_setZ for (f32) {
  fn setZ(self, rsthis: &mut QVector3D)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector3D4setZEf()};
    let arg0 = self  as c_float;
     unsafe {_ZN9QVector3D4setZEf(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QVector3D {
  pub fn toVector2D<T: QVector3D_toVector2D>(&mut self, value: T) -> QVector2D {
    return value.toVector2D(self);
    // return 1;
  }
}

pub trait QVector3D_toVector2D {
  fn toVector2D(self, rsthis: &mut QVector3D) -> QVector2D;
}

// proto:  QVector2D QVector3D::toVector2D();
impl<'a> /*trait*/ QVector3D_toVector2D for () {
  fn toVector2D(self, rsthis: &mut QVector3D) -> QVector2D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector3D10toVector2DEv()};
    let mut ret = unsafe {_ZNK9QVector3D10toVector2DEv(rsthis.qclsinst)};
    let mut ret1 = QVector2D{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QVector3D {
  pub fn distanceToPlane<T: QVector3D_distanceToPlane>(&mut self, value: T) -> f32 {
    return value.distanceToPlane(self);
    // return 1;
  }
}

pub trait QVector3D_distanceToPlane {
  fn distanceToPlane(self, rsthis: &mut QVector3D) -> f32;
}

// proto:  float QVector3D::distanceToPlane(const QVector3D & plane1, const QVector3D & plane2, const QVector3D & plane3);
impl<'a> /*trait*/ QVector3D_distanceToPlane for (&'a  QVector3D, &'a  QVector3D, &'a  QVector3D) {
  fn distanceToPlane(self, rsthis: &mut QVector3D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector3D15distanceToPlaneERKS_S1_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QVector3D15distanceToPlaneERKS_S1_S1_(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as f32;
    // return 1;
  }
}

impl /*struct*/ QVector3D {
  pub fn NewQVector3D<T: QVector3D_NewQVector3D>(value: T) -> QVector3D {
    let rsthis = value.NewQVector3D();
    return rsthis;
    // return 1;
  }
}

pub trait QVector3D_NewQVector3D {
  fn NewQVector3D(self) -> QVector3D;
}

// proto: void QVector3D::NewQVector3D(const QPointF & point);
impl<'a> /*trait*/ QVector3D_NewQVector3D for (&'a  QPointF) {
  fn NewQVector3D(self) -> QVector3D {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector3DC1ERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QVector3DC1ERK7QPointF(qthis, arg0)};
    let rsthis = QVector3D{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVector3D {
  pub fn distanceToPoint<T: QVector3D_distanceToPoint>(&mut self, value: T) -> f32 {
    return value.distanceToPoint(self);
    // return 1;
  }
}

pub trait QVector3D_distanceToPoint {
  fn distanceToPoint(self, rsthis: &mut QVector3D) -> f32;
}

// proto:  float QVector3D::distanceToPoint(const QVector3D & point);
impl<'a> /*trait*/ QVector3D_distanceToPoint for (&'a  QVector3D) {
  fn distanceToPoint(self, rsthis: &mut QVector3D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector3D15distanceToPointERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QVector3D15distanceToPointERKS_(rsthis.qclsinst, arg0)};
    return ret as f32;
    // return 1;
  }
}

// proto: void QVector3D::NewQVector3D(const QVector2D & vector);
impl<'a> /*trait*/ QVector3D_NewQVector3D for (&'a  QVector2D) {
  fn NewQVector3D(self) -> QVector3D {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector3DC1ERK9QVector2D()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QVector3DC1ERK9QVector2D(qthis, arg0)};
    let rsthis = QVector3D{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QVector3D::NewQVector3D(const QPoint & point);
impl<'a> /*trait*/ QVector3D_NewQVector3D for (&'a  QPoint) {
  fn NewQVector3D(self) -> QVector3D {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector3DC1ERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QVector3DC1ERK6QPoint(qthis, arg0)};
    let rsthis = QVector3D{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVector3D {
  pub fn lengthSquared<T: QVector3D_lengthSquared>(&mut self, value: T) -> f32 {
    return value.lengthSquared(self);
    // return 1;
  }
}

pub trait QVector3D_lengthSquared {
  fn lengthSquared(self, rsthis: &mut QVector3D) -> f32;
}

// proto:  float QVector3D::lengthSquared();
impl<'a> /*trait*/ QVector3D_lengthSquared for () {
  fn lengthSquared(self, rsthis: &mut QVector3D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector3D13lengthSquaredEv()};
    let mut ret = unsafe {_ZNK9QVector3D13lengthSquaredEv(rsthis.qclsinst)};
    return ret as f32;
    // return 1;
  }
}

// proto: static QVector3D QVector3D::normal(const QVector3D & v1, const QVector3D & v2, const QVector3D & v3);
impl<'a> /*trait*/ QVector3D_normal for (&'a  QVector3D, &'a  QVector3D, &'a  QVector3D) {
  fn normal(self, rsthis: &mut QVector3D) -> QVector3D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector3D6normalERKS_S1_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QVector3D6normalERKS_S1_S1_(arg0, arg1, arg2)};
    let mut ret1 = QVector3D{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  float QVector3D::distanceToPlane(const QVector3D & plane, const QVector3D & normal);
impl<'a> /*trait*/ QVector3D_distanceToPlane for (&'a  QVector3D, &'a  QVector3D) {
  fn distanceToPlane(self, rsthis: &mut QVector3D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector3D15distanceToPlaneERKS_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QVector3D15distanceToPlaneERKS_S1_(rsthis.qclsinst, arg0, arg1)};
    return ret as f32;
    // return 1;
  }
}

impl /*struct*/ QVector3D {
  pub fn normalized<T: QVector3D_normalized>(&mut self, value: T) -> QVector3D {
    return value.normalized(self);
    // return 1;
  }
}

pub trait QVector3D_normalized {
  fn normalized(self, rsthis: &mut QVector3D) -> QVector3D;
}

// proto:  QVector3D QVector3D::normalized();
impl<'a> /*trait*/ QVector3D_normalized for () {
  fn normalized(self, rsthis: &mut QVector3D) -> QVector3D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector3D10normalizedEv()};
    let mut ret = unsafe {_ZNK9QVector3D10normalizedEv(rsthis.qclsinst)};
    let mut ret1 = QVector3D{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QVector3D::NewQVector3D(const QVector4D & vector);
impl<'a> /*trait*/ QVector3D_NewQVector3D for (&'a  QVector4D) {
  fn NewQVector3D(self) -> QVector3D {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector3DC1ERK9QVector4D()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QVector3DC1ERK9QVector4D(qthis, arg0)};
    let rsthis = QVector3D{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVector3D {
  pub fn isNull<T: QVector3D_isNull>(&mut self, value: T) -> i8 {
    return value.isNull(self);
    // return 1;
  }
}

pub trait QVector3D_isNull {
  fn isNull(self, rsthis: &mut QVector3D) -> i8;
}

// proto:  bool QVector3D::isNull();
impl<'a> /*trait*/ QVector3D_isNull for () {
  fn isNull(self, rsthis: &mut QVector3D) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector3D6isNullEv()};
    let mut ret = unsafe {_ZNK9QVector3D6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QVector3D {
  pub fn length<T: QVector3D_length>(&mut self, value: T) -> f32 {
    return value.length(self);
    // return 1;
  }
}

pub trait QVector3D_length {
  fn length(self, rsthis: &mut QVector3D) -> f32;
}

// proto:  float QVector3D::length();
impl<'a> /*trait*/ QVector3D_length for () {
  fn length(self, rsthis: &mut QVector3D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector3D6lengthEv()};
    let mut ret = unsafe {_ZNK9QVector3D6lengthEv(rsthis.qclsinst)};
    return ret as f32;
    // return 1;
  }
}

impl /*struct*/ QVector3D {
  pub fn dotProduct<T: QVector3D_dotProduct>(&mut self, value: T) -> f32 {
    return value.dotProduct(self);
    // return 1;
  }
}

pub trait QVector3D_dotProduct {
  fn dotProduct(self, rsthis: &mut QVector3D) -> f32;
}

// proto: static float QVector3D::dotProduct(const QVector3D & v1, const QVector3D & v2);
impl<'a> /*trait*/ QVector3D_dotProduct for (&'a  QVector3D, &'a  QVector3D) {
  fn dotProduct(self, rsthis: &mut QVector3D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector3D10dotProductERKS_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QVector3D10dotProductERKS_S1_(arg0, arg1)};
    return ret as f32;
    // return 1;
  }
}

// proto: void QVector3D::NewQVector3D();
impl<'a> /*trait*/ QVector3D_NewQVector3D for () {
  fn NewQVector3D(self) -> QVector3D {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector3DC1Ev()};
    unsafe {_ZN9QVector3DC1Ev(qthis)};
    let rsthis = QVector3D{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QVector3D::NewQVector3D(float xpos, float ypos, float zpos);
impl<'a> /*trait*/ QVector3D_NewQVector3D for (f32, f32, f32) {
  fn NewQVector3D(self) -> QVector3D {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector3DC1Efff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    unsafe {_ZN9QVector3DC1Efff(qthis, arg0, arg1, arg2)};
    let rsthis = QVector3D{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVector3D {
  pub fn toVector4D<T: QVector3D_toVector4D>(&mut self, value: T) -> QVector4D {
    return value.toVector4D(self);
    // return 1;
  }
}

pub trait QVector3D_toVector4D {
  fn toVector4D(self, rsthis: &mut QVector3D) -> QVector4D;
}

// proto:  QVector4D QVector3D::toVector4D();
impl<'a> /*trait*/ QVector3D_toVector4D for () {
  fn toVector4D(self, rsthis: &mut QVector3D) -> QVector4D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector3D10toVector4DEv()};
    let mut ret = unsafe {_ZNK9QVector3D10toVector4DEv(rsthis.qclsinst)};
    let mut ret1 = QVector4D{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QVector3D::NewQVector3D(const QVector2D & vector, float zpos);
impl<'a> /*trait*/ QVector3D_NewQVector3D for (&'a  QVector2D, f32) {
  fn NewQVector3D(self) -> QVector3D {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector3DC1ERK9QVector2Df()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_float;
    unsafe {_ZN9QVector3DC1ERK9QVector2Df(qthis, arg0, arg1)};
    let rsthis = QVector3D{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVector3D {
  pub fn setX<T: QVector3D_setX>(&mut self, value: T)  {
     value.setX(self);
    // return 1;
  }
}

pub trait QVector3D_setX {
  fn setX(self, rsthis: &mut QVector3D) ;
}

// proto:  void QVector3D::setX(float x);
impl<'a> /*trait*/ QVector3D_setX for (f32) {
  fn setX(self, rsthis: &mut QVector3D)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector3D4setXEf()};
    let arg0 = self  as c_float;
     unsafe {_ZN9QVector3D4setXEf(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

