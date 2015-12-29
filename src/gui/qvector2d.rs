// auto generated, do not modify.
// created: Tue Dec 29 22:57:40 2015
// src-file: /QtGui/qvector2d.h
// dst-file: /src/gui/qvector2d.rs
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
use super::super::core::qpoint::QPointF; // 771
use super::qvector4d::QVector4D; // 773
use super::super::core::qpoint::QPoint; // 771
use super::qvector3d::QVector3D; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QVector2D_Class_Size() -> c_int;
  // proto:  QPointF QVector2D::toPointF();
  fn _ZNK9QVector2D8toPointFEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QVector2D::setX(float x);
  fn _ZN9QVector2D4setXEf(qthis: u64 /* *mut c_void*/, arg0: c_float);
  // proto:  void QVector2D::QVector2D(const QVector4D & vector);
  fn dector_ZN9QVector2DC1ERK9QVector4D(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QVector2DC1ERK9QVector4D(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QPoint QVector2D::toPoint();
  fn _ZNK9QVector2D7toPointEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  float QVector2D::length();
  fn _ZNK9QVector2D6lengthEv(qthis: u64 /* *mut c_void*/) -> c_float;
  // proto:  void QVector2D::setY(float y);
  fn _ZN9QVector2D4setYEf(qthis: u64 /* *mut c_void*/, arg0: c_float);
  // proto:  void QVector2D::QVector2D(const QPoint & point);
  fn dector_ZN9QVector2DC1ERK6QPoint(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QVector2DC1ERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QVector2D::QVector2D(float xpos, float ypos);
  fn dector_ZN9QVector2DC1Eff(arg0: c_float, arg1: c_float) -> *mut c_void;
  fn _ZN9QVector2DC1Eff(qthis: u64 /* *mut c_void*/, arg0: c_float, arg1: c_float);
  // proto:  bool QVector2D::isNull();
  fn _ZNK9QVector2D6isNullEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  float QVector2D::distanceToLine(const QVector2D & point, const QVector2D & direction);
  fn _ZNK9QVector2D14distanceToLineERKS_S1_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> c_float;
  // proto:  void QVector2D::QVector2D();
  fn dector_ZN9QVector2DC1Ev() -> *mut c_void;
  fn _ZN9QVector2DC1Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QVector3D QVector2D::toVector3D();
  fn _ZNK9QVector2D10toVector3DEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  float QVector2D::lengthSquared();
  fn _ZNK9QVector2D13lengthSquaredEv(qthis: u64 /* *mut c_void*/) -> c_float;
  // proto:  float QVector2D::y();
  fn _ZNK9QVector2D1yEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QVector2D::QVector2D(const QVector3D & vector);
  fn dector_ZN9QVector2DC1ERK9QVector3D(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QVector2DC1ERK9QVector3D(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  float QVector2D::x();
  fn _ZNK9QVector2D1xEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QVector2D::QVector2D(const QPointF & point);
  fn dector_ZN9QVector2DC1ERK7QPointF(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QVector2DC1ERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  float QVector2D::distanceToPoint(const QVector2D & point);
  fn _ZNK9QVector2D15distanceToPointERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_float;
  // proto:  QVector4D QVector2D::toVector4D();
  fn _ZNK9QVector2D10toVector4DEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QVector2D QVector2D::normalized();
  fn _ZNK9QVector2D10normalizedEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QVector2D::normalize();
  fn _ZN9QVector2D9normalizeEv(qthis: u64 /* *mut c_void*/);
  // proto: static float QVector2D::dotProduct(const QVector2D & v1, const QVector2D & v2);
  fn _ZN9QVector2D10dotProductERKS_S1_(arg0: *mut c_void, arg1: *mut c_void) -> c_float;
} // <= ext block end

// body block begin =>
// class sizeof(QVector2D)=8
#[derive(Default)]
pub struct QVector2D {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QVector2D {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QVector2D {
    return QVector2D{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QPointF QVector2D::toPointF();
impl /*struct*/ QVector2D {
  pub fn toPointF<RetType, T: QVector2D_toPointF<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toPointF(self);
    // return 1;
  }
}

pub trait QVector2D_toPointF<RetType> {
  fn toPointF(self , rsthis: & QVector2D) -> RetType;
}

  // proto:  QPointF QVector2D::toPointF();
impl<'a> /*trait*/ QVector2D_toPointF<QPointF> for () {
  fn toPointF(self , rsthis: & QVector2D) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector2D8toPointFEv()};
    let mut ret = unsafe {_ZNK9QVector2D8toPointFEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QVector2D::setX(float x);
impl /*struct*/ QVector2D {
  pub fn setX<RetType, T: QVector2D_setX<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setX(self);
    // return 1;
  }
}

pub trait QVector2D_setX<RetType> {
  fn setX(self , rsthis: & QVector2D) -> RetType;
}

  // proto:  void QVector2D::setX(float x);
impl<'a> /*trait*/ QVector2D_setX<()> for (f32) {
  fn setX(self , rsthis: & QVector2D) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector2D4setXEf()};
    let arg0 = self  as c_float;
     unsafe {_ZN9QVector2D4setXEf(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QVector2D::QVector2D(const QVector4D & vector);
impl /*struct*/ QVector2D {
  pub fn New<T: QVector2D_New>(value: T) -> QVector2D {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QVector2D_New {
  fn New(self) -> QVector2D;
}

  // proto:  void QVector2D::QVector2D(const QVector4D & vector);
impl<'a> /*trait*/ QVector2D_New for (&'a QVector4D) {
  fn New(self) -> QVector2D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector2DC1ERK9QVector4D()};
    let ctysz: c_int = unsafe{QVector2D_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QVector2DC1ERK9QVector4D(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN9QVector2DC1ERK9QVector4D(arg0)} as u64;
    let rsthis = QVector2D{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPoint QVector2D::toPoint();
impl /*struct*/ QVector2D {
  pub fn toPoint<RetType, T: QVector2D_toPoint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toPoint(self);
    // return 1;
  }
}

pub trait QVector2D_toPoint<RetType> {
  fn toPoint(self , rsthis: & QVector2D) -> RetType;
}

  // proto:  QPoint QVector2D::toPoint();
impl<'a> /*trait*/ QVector2D_toPoint<QPoint> for () {
  fn toPoint(self , rsthis: & QVector2D) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector2D7toPointEv()};
    let mut ret = unsafe {_ZNK9QVector2D7toPointEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  float QVector2D::length();
impl /*struct*/ QVector2D {
  pub fn length<RetType, T: QVector2D_length<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.length(self);
    // return 1;
  }
}

pub trait QVector2D_length<RetType> {
  fn length(self , rsthis: & QVector2D) -> RetType;
}

  // proto:  float QVector2D::length();
impl<'a> /*trait*/ QVector2D_length<f32> for () {
  fn length(self , rsthis: & QVector2D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector2D6lengthEv()};
    let mut ret = unsafe {_ZNK9QVector2D6lengthEv(rsthis.qclsinst)};
    return ret as f32;
    // return 1;
  }
}

  // proto:  void QVector2D::setY(float y);
impl /*struct*/ QVector2D {
  pub fn setY<RetType, T: QVector2D_setY<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setY(self);
    // return 1;
  }
}

pub trait QVector2D_setY<RetType> {
  fn setY(self , rsthis: & QVector2D) -> RetType;
}

  // proto:  void QVector2D::setY(float y);
impl<'a> /*trait*/ QVector2D_setY<()> for (f32) {
  fn setY(self , rsthis: & QVector2D) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector2D4setYEf()};
    let arg0 = self  as c_float;
     unsafe {_ZN9QVector2D4setYEf(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QVector2D::QVector2D(const QPoint & point);
impl<'a> /*trait*/ QVector2D_New for (&'a QPoint) {
  fn New(self) -> QVector2D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector2DC1ERK6QPoint()};
    let ctysz: c_int = unsafe{QVector2D_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QVector2DC1ERK6QPoint(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN9QVector2DC1ERK6QPoint(arg0)} as u64;
    let rsthis = QVector2D{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QVector2D::QVector2D(float xpos, float ypos);
impl<'a> /*trait*/ QVector2D_New for (f32, f32) {
  fn New(self) -> QVector2D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector2DC1Eff()};
    let ctysz: c_int = unsafe{QVector2D_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
    // unsafe {_ZN9QVector2DC1Eff(qthis, arg0, arg1)};
    let qthis: u64 = unsafe {dector_ZN9QVector2DC1Eff(arg0, arg1)} as u64;
    let rsthis = QVector2D{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QVector2D::isNull();
impl /*struct*/ QVector2D {
  pub fn isNull<RetType, T: QVector2D_isNull<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QVector2D_isNull<RetType> {
  fn isNull(self , rsthis: & QVector2D) -> RetType;
}

  // proto:  bool QVector2D::isNull();
impl<'a> /*trait*/ QVector2D_isNull<i8> for () {
  fn isNull(self , rsthis: & QVector2D) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector2D6isNullEv()};
    let mut ret = unsafe {_ZNK9QVector2D6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  float QVector2D::distanceToLine(const QVector2D & point, const QVector2D & direction);
impl /*struct*/ QVector2D {
  pub fn distanceToLine<RetType, T: QVector2D_distanceToLine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.distanceToLine(self);
    // return 1;
  }
}

pub trait QVector2D_distanceToLine<RetType> {
  fn distanceToLine(self , rsthis: & QVector2D) -> RetType;
}

  // proto:  float QVector2D::distanceToLine(const QVector2D & point, const QVector2D & direction);
impl<'a> /*trait*/ QVector2D_distanceToLine<f32> for (&'a QVector2D, &'a QVector2D) {
  fn distanceToLine(self , rsthis: & QVector2D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector2D14distanceToLineERKS_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QVector2D14distanceToLineERKS_S1_(rsthis.qclsinst, arg0, arg1)};
    return ret as f32;
    // return 1;
  }
}

  // proto:  void QVector2D::QVector2D();
impl<'a> /*trait*/ QVector2D_New for () {
  fn New(self) -> QVector2D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector2DC1Ev()};
    let ctysz: c_int = unsafe{QVector2D_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN9QVector2DC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN9QVector2DC1Ev()} as u64;
    let rsthis = QVector2D{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QVector3D QVector2D::toVector3D();
impl /*struct*/ QVector2D {
  pub fn toVector3D<RetType, T: QVector2D_toVector3D<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toVector3D(self);
    // return 1;
  }
}

pub trait QVector2D_toVector3D<RetType> {
  fn toVector3D(self , rsthis: & QVector2D) -> RetType;
}

  // proto:  QVector3D QVector2D::toVector3D();
impl<'a> /*trait*/ QVector2D_toVector3D<QVector3D> for () {
  fn toVector3D(self , rsthis: & QVector2D) -> QVector3D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector2D10toVector3DEv()};
    let mut ret = unsafe {_ZNK9QVector2D10toVector3DEv(rsthis.qclsinst)};
    let mut ret1 = QVector3D::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  float QVector2D::lengthSquared();
impl /*struct*/ QVector2D {
  pub fn lengthSquared<RetType, T: QVector2D_lengthSquared<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lengthSquared(self);
    // return 1;
  }
}

pub trait QVector2D_lengthSquared<RetType> {
  fn lengthSquared(self , rsthis: & QVector2D) -> RetType;
}

  // proto:  float QVector2D::lengthSquared();
impl<'a> /*trait*/ QVector2D_lengthSquared<f32> for () {
  fn lengthSquared(self , rsthis: & QVector2D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector2D13lengthSquaredEv()};
    let mut ret = unsafe {_ZNK9QVector2D13lengthSquaredEv(rsthis.qclsinst)};
    return ret as f32;
    // return 1;
  }
}

  // proto:  float QVector2D::y();
impl /*struct*/ QVector2D {
  pub fn y<RetType, T: QVector2D_y<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.y(self);
    // return 1;
  }
}

pub trait QVector2D_y<RetType> {
  fn y(self , rsthis: & QVector2D) -> RetType;
}

  // proto:  float QVector2D::y();
impl<'a> /*trait*/ QVector2D_y<()> for () {
  fn y(self , rsthis: & QVector2D) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector2D1yEv()};
     unsafe {_ZNK9QVector2D1yEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QVector2D::QVector2D(const QVector3D & vector);
impl<'a> /*trait*/ QVector2D_New for (&'a QVector3D) {
  fn New(self) -> QVector2D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector2DC1ERK9QVector3D()};
    let ctysz: c_int = unsafe{QVector2D_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QVector2DC1ERK9QVector3D(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN9QVector2DC1ERK9QVector3D(arg0)} as u64;
    let rsthis = QVector2D{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  float QVector2D::x();
impl /*struct*/ QVector2D {
  pub fn x<RetType, T: QVector2D_x<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.x(self);
    // return 1;
  }
}

pub trait QVector2D_x<RetType> {
  fn x(self , rsthis: & QVector2D) -> RetType;
}

  // proto:  float QVector2D::x();
impl<'a> /*trait*/ QVector2D_x<()> for () {
  fn x(self , rsthis: & QVector2D) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector2D1xEv()};
     unsafe {_ZNK9QVector2D1xEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QVector2D::QVector2D(const QPointF & point);
impl<'a> /*trait*/ QVector2D_New for (&'a QPointF) {
  fn New(self) -> QVector2D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector2DC1ERK7QPointF()};
    let ctysz: c_int = unsafe{QVector2D_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QVector2DC1ERK7QPointF(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN9QVector2DC1ERK7QPointF(arg0)} as u64;
    let rsthis = QVector2D{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  float QVector2D::distanceToPoint(const QVector2D & point);
impl /*struct*/ QVector2D {
  pub fn distanceToPoint<RetType, T: QVector2D_distanceToPoint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.distanceToPoint(self);
    // return 1;
  }
}

pub trait QVector2D_distanceToPoint<RetType> {
  fn distanceToPoint(self , rsthis: & QVector2D) -> RetType;
}

  // proto:  float QVector2D::distanceToPoint(const QVector2D & point);
impl<'a> /*trait*/ QVector2D_distanceToPoint<f32> for (&'a QVector2D) {
  fn distanceToPoint(self , rsthis: & QVector2D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector2D15distanceToPointERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QVector2D15distanceToPointERKS_(rsthis.qclsinst, arg0)};
    return ret as f32;
    // return 1;
  }
}

  // proto:  QVector4D QVector2D::toVector4D();
impl /*struct*/ QVector2D {
  pub fn toVector4D<RetType, T: QVector2D_toVector4D<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toVector4D(self);
    // return 1;
  }
}

pub trait QVector2D_toVector4D<RetType> {
  fn toVector4D(self , rsthis: & QVector2D) -> RetType;
}

  // proto:  QVector4D QVector2D::toVector4D();
impl<'a> /*trait*/ QVector2D_toVector4D<QVector4D> for () {
  fn toVector4D(self , rsthis: & QVector2D) -> QVector4D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector2D10toVector4DEv()};
    let mut ret = unsafe {_ZNK9QVector2D10toVector4DEv(rsthis.qclsinst)};
    let mut ret1 = QVector4D::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QVector2D QVector2D::normalized();
impl /*struct*/ QVector2D {
  pub fn normalized<RetType, T: QVector2D_normalized<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.normalized(self);
    // return 1;
  }
}

pub trait QVector2D_normalized<RetType> {
  fn normalized(self , rsthis: & QVector2D) -> RetType;
}

  // proto:  QVector2D QVector2D::normalized();
impl<'a> /*trait*/ QVector2D_normalized<QVector2D> for () {
  fn normalized(self , rsthis: & QVector2D) -> QVector2D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector2D10normalizedEv()};
    let mut ret = unsafe {_ZNK9QVector2D10normalizedEv(rsthis.qclsinst)};
    let mut ret1 = QVector2D::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QVector2D::normalize();
impl /*struct*/ QVector2D {
  pub fn normalize<RetType, T: QVector2D_normalize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.normalize(self);
    // return 1;
  }
}

pub trait QVector2D_normalize<RetType> {
  fn normalize(self , rsthis: & QVector2D) -> RetType;
}

  // proto:  void QVector2D::normalize();
impl<'a> /*trait*/ QVector2D_normalize<()> for () {
  fn normalize(self , rsthis: & QVector2D) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector2D9normalizeEv()};
     unsafe {_ZN9QVector2D9normalizeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static float QVector2D::dotProduct(const QVector2D & v1, const QVector2D & v2);
impl /*struct*/ QVector2D {
  pub fn dotProduct_s<RetType, T: QVector2D_dotProduct_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.dotProduct_s();
    // return 1;
  }
}

pub trait QVector2D_dotProduct_s<RetType> {
  fn dotProduct_s(self ) -> RetType;
}

  // proto: static float QVector2D::dotProduct(const QVector2D & v1, const QVector2D & v2);
impl<'a> /*trait*/ QVector2D_dotProduct_s<f32> for (&'a QVector2D, &'a QVector2D) {
  fn dotProduct_s(self ) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector2D10dotProductERKS_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QVector2D10dotProductERKS_S1_(arg0, arg1)};
    return ret as f32;
    // return 1;
  }
}

// <= body block end

