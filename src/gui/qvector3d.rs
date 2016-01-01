// auto generated, do not modify.
// created: Fri Jan  1 12:13:41 2016
// src-file: /QtGui/qvector3d.h
// dst-file: /src/gui/qvector3d.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use std::ops::Deref;
use super::super::core::qpoint::QPoint; // 771
use super::super::core::qpoint::QPointF; // 771
use super::qmatrix4x4::QMatrix4x4; // 773
use super::super::core::qrect::QRect; // 771
use super::qvector2d::QVector2D; // 773
use super::qvector4d::QVector4D; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QVector3D_Class_Size() -> c_int;
  // proto:  float QVector3D::x();
  fn _ZNK9QVector3D1xEv(qthis: u64 /* *mut c_void*/);
  // proto:  QPoint QVector3D::toPoint();
  fn _ZNK9QVector3D7toPointEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  float QVector3D::distanceToLine(const QVector3D & point, const QVector3D & direction);
  fn _ZNK9QVector3D14distanceToLineERKS_S1_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> c_float;
  // proto:  float QVector3D::y();
  fn _ZNK9QVector3D1yEv(qthis: u64 /* *mut c_void*/);
  // proto: static QVector3D QVector3D::normal(const QVector3D & v1, const QVector3D & v2);
  fn _ZN9QVector3D6normalERKS_S1_(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QPointF QVector3D::toPointF();
  fn _ZNK9QVector3D8toPointFEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QVector3D::normalize();
  fn _ZN9QVector3D9normalizeEv(qthis: u64 /* *mut c_void*/);
  // proto:  QVector3D QVector3D::unproject(const QMatrix4x4 & modelView, const QMatrix4x4 & projection, const QRect & viewport);
  fn _ZNK9QVector3D9unprojectERK10QMatrix4x4S2_RK5QRect(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;
  // proto:  void QVector3D::setY(float y);
  fn _ZN9QVector3D4setYEf(qthis: u64 /* *mut c_void*/, arg0: c_float);
  // proto:  QVector3D QVector3D::project(const QMatrix4x4 & modelView, const QMatrix4x4 & projection, const QRect & viewport);
  fn _ZNK9QVector3D7projectERK10QMatrix4x4S2_RK5QRect(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;
  // proto: static QVector3D QVector3D::crossProduct(const QVector3D & v1, const QVector3D & v2);
  fn _ZN9QVector3D12crossProductERKS_S1_(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  float QVector3D::z();
  fn _ZNK9QVector3D1zEv(qthis: u64 /* *mut c_void*/) -> c_float;
  // proto:  void QVector3D::setZ(float z);
  fn _ZN9QVector3D4setZEf(qthis: u64 /* *mut c_void*/, arg0: c_float);
  // proto:  QVector2D QVector3D::toVector2D();
  fn _ZNK9QVector3D10toVector2DEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  float QVector3D::distanceToPlane(const QVector3D & plane1, const QVector3D & plane2, const QVector3D & plane3);
  fn _ZNK9QVector3D15distanceToPlaneERKS_S1_S1_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> c_float;
  // proto:  void QVector3D::QVector3D(const QPointF & point);
  fn dector_ZN9QVector3DC1ERK7QPointF(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QVector3DC1ERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  float QVector3D::distanceToPoint(const QVector3D & point);
  fn _ZNK9QVector3D15distanceToPointERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_float;
  // proto:  void QVector3D::QVector3D(const QVector2D & vector);
  fn dector_ZN9QVector3DC1ERK9QVector2D(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QVector3DC1ERK9QVector2D(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QVector3D::QVector3D(const QPoint & point);
  fn dector_ZN9QVector3DC1ERK6QPoint(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QVector3DC1ERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  float QVector3D::lengthSquared();
  fn _ZNK9QVector3D13lengthSquaredEv(qthis: u64 /* *mut c_void*/) -> c_float;
  // proto: static QVector3D QVector3D::normal(const QVector3D & v1, const QVector3D & v2, const QVector3D & v3);
  fn _ZN9QVector3D6normalERKS_S1_S1_(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;
  // proto:  float QVector3D::distanceToPlane(const QVector3D & plane, const QVector3D & normal);
  fn _ZNK9QVector3D15distanceToPlaneERKS_S1_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> c_float;
  // proto:  QVector3D QVector3D::normalized();
  fn _ZNK9QVector3D10normalizedEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QVector3D::QVector3D(const QVector4D & vector);
  fn dector_ZN9QVector3DC1ERK9QVector4D(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QVector3DC1ERK9QVector4D(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QVector3D::isNull();
  fn _ZNK9QVector3D6isNullEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  float QVector3D::length();
  fn _ZNK9QVector3D6lengthEv(qthis: u64 /* *mut c_void*/) -> c_float;
  // proto: static float QVector3D::dotProduct(const QVector3D & v1, const QVector3D & v2);
  fn _ZN9QVector3D10dotProductERKS_S1_(arg0: *mut c_void, arg1: *mut c_void) -> c_float;
  // proto:  void QVector3D::QVector3D();
  fn dector_ZN9QVector3DC1Ev() -> *mut c_void;
  fn _ZN9QVector3DC1Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QVector3D::QVector3D(float xpos, float ypos, float zpos);
  fn dector_ZN9QVector3DC1Efff(arg0: c_float, arg1: c_float, arg2: c_float) -> *mut c_void;
  fn _ZN9QVector3DC1Efff(qthis: u64 /* *mut c_void*/, arg0: c_float, arg1: c_float, arg2: c_float);
  // proto:  QVector4D QVector3D::toVector4D();
  fn _ZNK9QVector3D10toVector4DEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QVector3D::QVector3D(const QVector2D & vector, float zpos);
  fn dector_ZN9QVector3DC1ERK9QVector2Df(arg0: *mut c_void, arg1: c_float) -> *mut c_void;
  fn _ZN9QVector3DC1ERK9QVector2Df(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_float);
  // proto:  void QVector3D::setX(float x);
  fn _ZN9QVector3D4setXEf(qthis: u64 /* *mut c_void*/, arg0: c_float);
} // <= ext block end

// body block begin =>
// class sizeof(QVector3D)=12
#[derive(Default)]
pub struct QVector3D {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QVector3D {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QVector3D {
    return QVector3D{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  float QVector3D::x();
impl /*struct*/ QVector3D {
  pub fn x<RetType, T: QVector3D_x<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.x(self);
    // return 1;
  }
}

pub trait QVector3D_x<RetType> {
  fn x(self , rsthis: & QVector3D) -> RetType;
}

  // proto:  float QVector3D::x();
impl<'a> /*trait*/ QVector3D_x<()> for () {
  fn x(self , rsthis: & QVector3D) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector3D1xEv()};
     unsafe {_ZNK9QVector3D1xEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPoint QVector3D::toPoint();
impl /*struct*/ QVector3D {
  pub fn toPoint<RetType, T: QVector3D_toPoint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toPoint(self);
    // return 1;
  }
}

pub trait QVector3D_toPoint<RetType> {
  fn toPoint(self , rsthis: & QVector3D) -> RetType;
}

  // proto:  QPoint QVector3D::toPoint();
impl<'a> /*trait*/ QVector3D_toPoint<QPoint> for () {
  fn toPoint(self , rsthis: & QVector3D) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector3D7toPointEv()};
    let mut ret = unsafe {_ZNK9QVector3D7toPointEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  float QVector3D::distanceToLine(const QVector3D & point, const QVector3D & direction);
impl /*struct*/ QVector3D {
  pub fn distanceToLine<RetType, T: QVector3D_distanceToLine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.distanceToLine(self);
    // return 1;
  }
}

pub trait QVector3D_distanceToLine<RetType> {
  fn distanceToLine(self , rsthis: & QVector3D) -> RetType;
}

  // proto:  float QVector3D::distanceToLine(const QVector3D & point, const QVector3D & direction);
impl<'a> /*trait*/ QVector3D_distanceToLine<f32> for (&'a QVector3D, &'a QVector3D) {
  fn distanceToLine(self , rsthis: & QVector3D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector3D14distanceToLineERKS_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QVector3D14distanceToLineERKS_S1_(rsthis.qclsinst, arg0, arg1)};
    return ret as f32;
    // return 1;
  }
}

  // proto:  float QVector3D::y();
impl /*struct*/ QVector3D {
  pub fn y<RetType, T: QVector3D_y<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.y(self);
    // return 1;
  }
}

pub trait QVector3D_y<RetType> {
  fn y(self , rsthis: & QVector3D) -> RetType;
}

  // proto:  float QVector3D::y();
impl<'a> /*trait*/ QVector3D_y<()> for () {
  fn y(self , rsthis: & QVector3D) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector3D1yEv()};
     unsafe {_ZNK9QVector3D1yEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static QVector3D QVector3D::normal(const QVector3D & v1, const QVector3D & v2);
impl /*struct*/ QVector3D {
  pub fn normal_s<RetType, T: QVector3D_normal_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.normal_s();
    // return 1;
  }
}

pub trait QVector3D_normal_s<RetType> {
  fn normal_s(self ) -> RetType;
}

  // proto: static QVector3D QVector3D::normal(const QVector3D & v1, const QVector3D & v2);
impl<'a> /*trait*/ QVector3D_normal_s<QVector3D> for (&'a QVector3D, &'a QVector3D) {
  fn normal_s(self ) -> QVector3D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector3D6normalERKS_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QVector3D6normalERKS_S1_(arg0, arg1)};
    let mut ret1 = QVector3D::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPointF QVector3D::toPointF();
impl /*struct*/ QVector3D {
  pub fn toPointF<RetType, T: QVector3D_toPointF<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toPointF(self);
    // return 1;
  }
}

pub trait QVector3D_toPointF<RetType> {
  fn toPointF(self , rsthis: & QVector3D) -> RetType;
}

  // proto:  QPointF QVector3D::toPointF();
impl<'a> /*trait*/ QVector3D_toPointF<QPointF> for () {
  fn toPointF(self , rsthis: & QVector3D) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector3D8toPointFEv()};
    let mut ret = unsafe {_ZNK9QVector3D8toPointFEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QVector3D::normalize();
impl /*struct*/ QVector3D {
  pub fn normalize<RetType, T: QVector3D_normalize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.normalize(self);
    // return 1;
  }
}

pub trait QVector3D_normalize<RetType> {
  fn normalize(self , rsthis: & QVector3D) -> RetType;
}

  // proto:  void QVector3D::normalize();
impl<'a> /*trait*/ QVector3D_normalize<()> for () {
  fn normalize(self , rsthis: & QVector3D) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector3D9normalizeEv()};
     unsafe {_ZN9QVector3D9normalizeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QVector3D QVector3D::unproject(const QMatrix4x4 & modelView, const QMatrix4x4 & projection, const QRect & viewport);
impl /*struct*/ QVector3D {
  pub fn unproject<RetType, T: QVector3D_unproject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.unproject(self);
    // return 1;
  }
}

pub trait QVector3D_unproject<RetType> {
  fn unproject(self , rsthis: & QVector3D) -> RetType;
}

  // proto:  QVector3D QVector3D::unproject(const QMatrix4x4 & modelView, const QMatrix4x4 & projection, const QRect & viewport);
impl<'a> /*trait*/ QVector3D_unproject<QVector3D> for (&'a QMatrix4x4, &'a QMatrix4x4, &'a QRect) {
  fn unproject(self , rsthis: & QVector3D) -> QVector3D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector3D9unprojectERK10QMatrix4x4S2_RK5QRect()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QVector3D9unprojectERK10QMatrix4x4S2_RK5QRect(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QVector3D::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QVector3D::setY(float y);
impl /*struct*/ QVector3D {
  pub fn setY<RetType, T: QVector3D_setY<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setY(self);
    // return 1;
  }
}

pub trait QVector3D_setY<RetType> {
  fn setY(self , rsthis: & QVector3D) -> RetType;
}

  // proto:  void QVector3D::setY(float y);
impl<'a> /*trait*/ QVector3D_setY<()> for (f32) {
  fn setY(self , rsthis: & QVector3D) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector3D4setYEf()};
    let arg0 = self  as c_float;
     unsafe {_ZN9QVector3D4setYEf(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QVector3D QVector3D::project(const QMatrix4x4 & modelView, const QMatrix4x4 & projection, const QRect & viewport);
impl /*struct*/ QVector3D {
  pub fn project<RetType, T: QVector3D_project<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.project(self);
    // return 1;
  }
}

pub trait QVector3D_project<RetType> {
  fn project(self , rsthis: & QVector3D) -> RetType;
}

  // proto:  QVector3D QVector3D::project(const QMatrix4x4 & modelView, const QMatrix4x4 & projection, const QRect & viewport);
impl<'a> /*trait*/ QVector3D_project<QVector3D> for (&'a QMatrix4x4, &'a QMatrix4x4, &'a QRect) {
  fn project(self , rsthis: & QVector3D) -> QVector3D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector3D7projectERK10QMatrix4x4S2_RK5QRect()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QVector3D7projectERK10QMatrix4x4S2_RK5QRect(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QVector3D::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QVector3D QVector3D::crossProduct(const QVector3D & v1, const QVector3D & v2);
impl /*struct*/ QVector3D {
  pub fn crossProduct_s<RetType, T: QVector3D_crossProduct_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.crossProduct_s();
    // return 1;
  }
}

pub trait QVector3D_crossProduct_s<RetType> {
  fn crossProduct_s(self ) -> RetType;
}

  // proto: static QVector3D QVector3D::crossProduct(const QVector3D & v1, const QVector3D & v2);
impl<'a> /*trait*/ QVector3D_crossProduct_s<QVector3D> for (&'a QVector3D, &'a QVector3D) {
  fn crossProduct_s(self ) -> QVector3D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector3D12crossProductERKS_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QVector3D12crossProductERKS_S1_(arg0, arg1)};
    let mut ret1 = QVector3D::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  float QVector3D::z();
impl /*struct*/ QVector3D {
  pub fn z<RetType, T: QVector3D_z<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.z(self);
    // return 1;
  }
}

pub trait QVector3D_z<RetType> {
  fn z(self , rsthis: & QVector3D) -> RetType;
}

  // proto:  float QVector3D::z();
impl<'a> /*trait*/ QVector3D_z<f32> for () {
  fn z(self , rsthis: & QVector3D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector3D1zEv()};
    let mut ret = unsafe {_ZNK9QVector3D1zEv(rsthis.qclsinst)};
    return ret as f32;
    // return 1;
  }
}

  // proto:  void QVector3D::setZ(float z);
impl /*struct*/ QVector3D {
  pub fn setZ<RetType, T: QVector3D_setZ<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setZ(self);
    // return 1;
  }
}

pub trait QVector3D_setZ<RetType> {
  fn setZ(self , rsthis: & QVector3D) -> RetType;
}

  // proto:  void QVector3D::setZ(float z);
impl<'a> /*trait*/ QVector3D_setZ<()> for (f32) {
  fn setZ(self , rsthis: & QVector3D) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector3D4setZEf()};
    let arg0 = self  as c_float;
     unsafe {_ZN9QVector3D4setZEf(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QVector2D QVector3D::toVector2D();
impl /*struct*/ QVector3D {
  pub fn toVector2D<RetType, T: QVector3D_toVector2D<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toVector2D(self);
    // return 1;
  }
}

pub trait QVector3D_toVector2D<RetType> {
  fn toVector2D(self , rsthis: & QVector3D) -> RetType;
}

  // proto:  QVector2D QVector3D::toVector2D();
impl<'a> /*trait*/ QVector3D_toVector2D<QVector2D> for () {
  fn toVector2D(self , rsthis: & QVector3D) -> QVector2D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector3D10toVector2DEv()};
    let mut ret = unsafe {_ZNK9QVector3D10toVector2DEv(rsthis.qclsinst)};
    let mut ret1 = QVector2D::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  float QVector3D::distanceToPlane(const QVector3D & plane1, const QVector3D & plane2, const QVector3D & plane3);
impl /*struct*/ QVector3D {
  pub fn distanceToPlane<RetType, T: QVector3D_distanceToPlane<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.distanceToPlane(self);
    // return 1;
  }
}

pub trait QVector3D_distanceToPlane<RetType> {
  fn distanceToPlane(self , rsthis: & QVector3D) -> RetType;
}

  // proto:  float QVector3D::distanceToPlane(const QVector3D & plane1, const QVector3D & plane2, const QVector3D & plane3);
impl<'a> /*trait*/ QVector3D_distanceToPlane<f32> for (&'a QVector3D, &'a QVector3D, &'a QVector3D) {
  fn distanceToPlane(self , rsthis: & QVector3D) -> f32 {
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

  // proto:  void QVector3D::QVector3D(const QPointF & point);
impl /*struct*/ QVector3D {
  pub fn new<T: QVector3D_new>(value: T) -> QVector3D {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QVector3D_new {
  fn new(self) -> QVector3D;
}

  // proto:  void QVector3D::QVector3D(const QPointF & point);
impl<'a> /*trait*/ QVector3D_new for (&'a QPointF) {
  fn new(self) -> QVector3D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector3DC1ERK7QPointF()};
    let ctysz: c_int = unsafe{QVector3D_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QVector3DC1ERK7QPointF(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN9QVector3DC1ERK7QPointF(arg0)} as u64;
    let rsthis = QVector3D{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  float QVector3D::distanceToPoint(const QVector3D & point);
impl /*struct*/ QVector3D {
  pub fn distanceToPoint<RetType, T: QVector3D_distanceToPoint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.distanceToPoint(self);
    // return 1;
  }
}

pub trait QVector3D_distanceToPoint<RetType> {
  fn distanceToPoint(self , rsthis: & QVector3D) -> RetType;
}

  // proto:  float QVector3D::distanceToPoint(const QVector3D & point);
impl<'a> /*trait*/ QVector3D_distanceToPoint<f32> for (&'a QVector3D) {
  fn distanceToPoint(self , rsthis: & QVector3D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector3D15distanceToPointERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QVector3D15distanceToPointERKS_(rsthis.qclsinst, arg0)};
    return ret as f32;
    // return 1;
  }
}

  // proto:  void QVector3D::QVector3D(const QVector2D & vector);
impl<'a> /*trait*/ QVector3D_new for (&'a QVector2D) {
  fn new(self) -> QVector3D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector3DC1ERK9QVector2D()};
    let ctysz: c_int = unsafe{QVector3D_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QVector3DC1ERK9QVector2D(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN9QVector3DC1ERK9QVector2D(arg0)} as u64;
    let rsthis = QVector3D{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QVector3D::QVector3D(const QPoint & point);
impl<'a> /*trait*/ QVector3D_new for (&'a QPoint) {
  fn new(self) -> QVector3D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector3DC1ERK6QPoint()};
    let ctysz: c_int = unsafe{QVector3D_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QVector3DC1ERK6QPoint(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN9QVector3DC1ERK6QPoint(arg0)} as u64;
    let rsthis = QVector3D{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  float QVector3D::lengthSquared();
impl /*struct*/ QVector3D {
  pub fn lengthSquared<RetType, T: QVector3D_lengthSquared<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lengthSquared(self);
    // return 1;
  }
}

pub trait QVector3D_lengthSquared<RetType> {
  fn lengthSquared(self , rsthis: & QVector3D) -> RetType;
}

  // proto:  float QVector3D::lengthSquared();
impl<'a> /*trait*/ QVector3D_lengthSquared<f32> for () {
  fn lengthSquared(self , rsthis: & QVector3D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector3D13lengthSquaredEv()};
    let mut ret = unsafe {_ZNK9QVector3D13lengthSquaredEv(rsthis.qclsinst)};
    return ret as f32;
    // return 1;
  }
}

  // proto: static QVector3D QVector3D::normal(const QVector3D & v1, const QVector3D & v2, const QVector3D & v3);
impl<'a> /*trait*/ QVector3D_normal_s<QVector3D> for (&'a QVector3D, &'a QVector3D, &'a QVector3D) {
  fn normal_s(self ) -> QVector3D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector3D6normalERKS_S1_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QVector3D6normalERKS_S1_S1_(arg0, arg1, arg2)};
    let mut ret1 = QVector3D::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  float QVector3D::distanceToPlane(const QVector3D & plane, const QVector3D & normal);
impl<'a> /*trait*/ QVector3D_distanceToPlane<f32> for (&'a QVector3D, &'a QVector3D) {
  fn distanceToPlane(self , rsthis: & QVector3D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector3D15distanceToPlaneERKS_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QVector3D15distanceToPlaneERKS_S1_(rsthis.qclsinst, arg0, arg1)};
    return ret as f32;
    // return 1;
  }
}

  // proto:  QVector3D QVector3D::normalized();
impl /*struct*/ QVector3D {
  pub fn normalized<RetType, T: QVector3D_normalized<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.normalized(self);
    // return 1;
  }
}

pub trait QVector3D_normalized<RetType> {
  fn normalized(self , rsthis: & QVector3D) -> RetType;
}

  // proto:  QVector3D QVector3D::normalized();
impl<'a> /*trait*/ QVector3D_normalized<QVector3D> for () {
  fn normalized(self , rsthis: & QVector3D) -> QVector3D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector3D10normalizedEv()};
    let mut ret = unsafe {_ZNK9QVector3D10normalizedEv(rsthis.qclsinst)};
    let mut ret1 = QVector3D::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QVector3D::QVector3D(const QVector4D & vector);
impl<'a> /*trait*/ QVector3D_new for (&'a QVector4D) {
  fn new(self) -> QVector3D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector3DC1ERK9QVector4D()};
    let ctysz: c_int = unsafe{QVector3D_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QVector3DC1ERK9QVector4D(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN9QVector3DC1ERK9QVector4D(arg0)} as u64;
    let rsthis = QVector3D{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QVector3D::isNull();
impl /*struct*/ QVector3D {
  pub fn isNull<RetType, T: QVector3D_isNull<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QVector3D_isNull<RetType> {
  fn isNull(self , rsthis: & QVector3D) -> RetType;
}

  // proto:  bool QVector3D::isNull();
impl<'a> /*trait*/ QVector3D_isNull<i8> for () {
  fn isNull(self , rsthis: & QVector3D) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector3D6isNullEv()};
    let mut ret = unsafe {_ZNK9QVector3D6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  float QVector3D::length();
impl /*struct*/ QVector3D {
  pub fn length<RetType, T: QVector3D_length<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.length(self);
    // return 1;
  }
}

pub trait QVector3D_length<RetType> {
  fn length(self , rsthis: & QVector3D) -> RetType;
}

  // proto:  float QVector3D::length();
impl<'a> /*trait*/ QVector3D_length<f32> for () {
  fn length(self , rsthis: & QVector3D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector3D6lengthEv()};
    let mut ret = unsafe {_ZNK9QVector3D6lengthEv(rsthis.qclsinst)};
    return ret as f32;
    // return 1;
  }
}

  // proto: static float QVector3D::dotProduct(const QVector3D & v1, const QVector3D & v2);
impl /*struct*/ QVector3D {
  pub fn dotProduct_s<RetType, T: QVector3D_dotProduct_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.dotProduct_s();
    // return 1;
  }
}

pub trait QVector3D_dotProduct_s<RetType> {
  fn dotProduct_s(self ) -> RetType;
}

  // proto: static float QVector3D::dotProduct(const QVector3D & v1, const QVector3D & v2);
impl<'a> /*trait*/ QVector3D_dotProduct_s<f32> for (&'a QVector3D, &'a QVector3D) {
  fn dotProduct_s(self ) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector3D10dotProductERKS_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QVector3D10dotProductERKS_S1_(arg0, arg1)};
    return ret as f32;
    // return 1;
  }
}

  // proto:  void QVector3D::QVector3D();
impl<'a> /*trait*/ QVector3D_new for () {
  fn new(self) -> QVector3D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector3DC1Ev()};
    let ctysz: c_int = unsafe{QVector3D_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN9QVector3DC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN9QVector3DC1Ev()} as u64;
    let rsthis = QVector3D{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QVector3D::QVector3D(float xpos, float ypos, float zpos);
impl<'a> /*trait*/ QVector3D_new for (f32, f32, f32) {
  fn new(self) -> QVector3D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector3DC1Efff()};
    let ctysz: c_int = unsafe{QVector3D_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    // unsafe {_ZN9QVector3DC1Efff(qthis, arg0, arg1, arg2)};
    let qthis: u64 = unsafe {dector_ZN9QVector3DC1Efff(arg0, arg1, arg2)} as u64;
    let rsthis = QVector3D{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QVector4D QVector3D::toVector4D();
impl /*struct*/ QVector3D {
  pub fn toVector4D<RetType, T: QVector3D_toVector4D<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toVector4D(self);
    // return 1;
  }
}

pub trait QVector3D_toVector4D<RetType> {
  fn toVector4D(self , rsthis: & QVector3D) -> RetType;
}

  // proto:  QVector4D QVector3D::toVector4D();
impl<'a> /*trait*/ QVector3D_toVector4D<QVector4D> for () {
  fn toVector4D(self , rsthis: & QVector3D) -> QVector4D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector3D10toVector4DEv()};
    let mut ret = unsafe {_ZNK9QVector3D10toVector4DEv(rsthis.qclsinst)};
    let mut ret1 = QVector4D::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QVector3D::QVector3D(const QVector2D & vector, float zpos);
impl<'a> /*trait*/ QVector3D_new for (&'a QVector2D, f32) {
  fn new(self) -> QVector3D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector3DC1ERK9QVector2Df()};
    let ctysz: c_int = unsafe{QVector3D_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_float;
    // unsafe {_ZN9QVector3DC1ERK9QVector2Df(qthis, arg0, arg1)};
    let qthis: u64 = unsafe {dector_ZN9QVector3DC1ERK9QVector2Df(arg0, arg1)} as u64;
    let rsthis = QVector3D{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QVector3D::setX(float x);
impl /*struct*/ QVector3D {
  pub fn setX<RetType, T: QVector3D_setX<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setX(self);
    // return 1;
  }
}

pub trait QVector3D_setX<RetType> {
  fn setX(self , rsthis: & QVector3D) -> RetType;
}

  // proto:  void QVector3D::setX(float x);
impl<'a> /*trait*/ QVector3D_setX<()> for (f32) {
  fn setX(self , rsthis: & QVector3D) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector3D4setXEf()};
    let arg0 = self  as c_float;
     unsafe {_ZN9QVector3D4setXEf(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

