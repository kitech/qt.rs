// auto generated, do not modify.
// created: Sat Dec 26 12:15:38 2015
// src-file: /QtGui/qvector4d.h
// dst-file: /src/gui/qvector4d.rs
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
use super::qvector2d::QVector2D; // 773
use super::super::core::qpoint::QPointF; // 771
use super::qvector3d::QVector3D; // 773
use super::super::core::qpoint::QPoint; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QVector4D_Class_Size() -> c_int;
  // proto:  QVector4D QVector4D::normalized();
  fn _ZNK9QVector4D10normalizedEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QVector4D::setW(float w);
  fn _ZN9QVector4D4setWEf(qthis: *mut c_void, arg0: c_float);
  // proto:  void QVector4D::QVector4D(const QVector2D & vector, float zpos, float wpos);
  fn dector_ZN9QVector4DC1ERK9QVector2Dff(arg0: *mut c_void, arg1: c_float, arg2: c_float) -> *mut c_void;
  fn _ZN9QVector4DC1ERK9QVector2Dff(qthis: *mut c_void, arg0: *mut c_void, arg1: c_float, arg2: c_float);
  // proto:  QPointF QVector4D::toPointF();
  fn _ZNK9QVector4D8toPointFEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  float QVector4D::y();
  fn _ZNK9QVector4D1yEv(qthis: *mut c_void);
  // proto:  QVector2D QVector4D::toVector2D();
  fn _ZNK9QVector4D10toVector2DEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QVector4D::setZ(float z);
  fn _ZN9QVector4D4setZEf(qthis: *mut c_void, arg0: c_float);
  // proto:  void QVector4D::QVector4D(const QVector2D & vector);
  fn dector_ZN9QVector4DC1ERK9QVector2D(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QVector4DC1ERK9QVector2D(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QVector4D::normalize();
  fn _ZN9QVector4D9normalizeEv(qthis: *mut c_void);
  // proto:  void QVector4D::QVector4D(float xpos, float ypos, float zpos, float wpos);
  fn dector_ZN9QVector4DC1Effff(arg0: c_float, arg1: c_float, arg2: c_float, arg3: c_float) -> *mut c_void;
  fn _ZN9QVector4DC1Effff(qthis: *mut c_void, arg0: c_float, arg1: c_float, arg2: c_float, arg3: c_float);
  // proto:  void QVector4D::QVector4D(const QVector3D & vector, float wpos);
  fn dector_ZN9QVector4DC1ERK9QVector3Df(arg0: *mut c_void, arg1: c_float) -> *mut c_void;
  fn _ZN9QVector4DC1ERK9QVector3Df(qthis: *mut c_void, arg0: *mut c_void, arg1: c_float);
  // proto:  void QVector4D::QVector4D(const QPointF & point);
  fn dector_ZN9QVector4DC1ERK7QPointF(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QVector4DC1ERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  float QVector4D::z();
  fn _ZNK9QVector4D1zEv(qthis: *mut c_void) -> c_float;
  // proto:  void QVector4D::QVector4D();
  fn dector_ZN9QVector4DC1Ev() -> *mut c_void;
  fn _ZN9QVector4DC1Ev(qthis: *mut c_void);
  // proto:  void QVector4D::setX(float x);
  fn _ZN9QVector4D4setXEf(qthis: *mut c_void, arg0: c_float);
  // proto:  void QVector4D::setY(float y);
  fn _ZN9QVector4D4setYEf(qthis: *mut c_void, arg0: c_float);
  // proto:  void QVector4D::QVector4D(const QPoint & point);
  fn dector_ZN9QVector4DC1ERK6QPoint(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QVector4DC1ERK6QPoint(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QVector3D QVector4D::toVector3D();
  fn _ZNK9QVector4D10toVector3DEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  float QVector4D::x();
  fn _ZNK9QVector4D1xEv(qthis: *mut c_void);
  // proto:  QVector2D QVector4D::toVector2DAffine();
  fn _ZNK9QVector4D16toVector2DAffineEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  float QVector4D::length();
  fn _ZNK9QVector4D6lengthEv(qthis: *mut c_void) -> c_float;
  // proto:  void QVector4D::QVector4D(const QVector3D & vector);
  fn dector_ZN9QVector4DC1ERK9QVector3D(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QVector4DC1ERK9QVector3D(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static float QVector4D::dotProduct(const QVector4D & v1, const QVector4D & v2);
  fn _ZN9QVector4D10dotProductERKS_S1_(arg0: *mut c_void, arg1: *mut c_void) -> c_float;
  // proto:  bool QVector4D::isNull();
  fn _ZNK9QVector4D6isNullEv(qthis: *mut c_void) -> c_char;
  // proto:  float QVector4D::lengthSquared();
  fn _ZNK9QVector4D13lengthSquaredEv(qthis: *mut c_void) -> c_float;
  // proto:  QVector3D QVector4D::toVector3DAffine();
  fn _ZNK9QVector4D16toVector3DAffineEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPoint QVector4D::toPoint();
  fn _ZNK9QVector4D7toPointEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  float QVector4D::w();
  fn _ZNK9QVector4D1wEv(qthis: *mut c_void) -> c_float;
} // <= ext block end

// body block begin =>
// class sizeof(QVector4D)=16
pub struct QVector4D {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QVector4D {
  pub fn inheritFrom(qthis: *mut c_void) -> QVector4D {
    return QVector4D{qclsinst: qthis};
  }
}
  // proto:  QVector4D QVector4D::normalized();
impl /*struct*/ QVector4D {
  pub fn normalized<RetType, T: QVector4D_normalized<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.normalized(self);
    // return 1;
  }
}

pub trait QVector4D_normalized<RetType> {
  fn normalized(self , rsthis: & QVector4D) -> RetType;
}

  // proto:  QVector4D QVector4D::normalized();
impl<'a> /*trait*/ QVector4D_normalized<QVector4D> for () {
  fn normalized(self , rsthis: & QVector4D) -> QVector4D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector4D10normalizedEv()};
    let mut ret = unsafe {_ZNK9QVector4D10normalizedEv(rsthis.qclsinst)};
    let mut ret1 = QVector4D::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QVector4D::setW(float w);
impl /*struct*/ QVector4D {
  pub fn setW<RetType, T: QVector4D_setW<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setW(self);
    // return 1;
  }
}

pub trait QVector4D_setW<RetType> {
  fn setW(self , rsthis: & QVector4D) -> RetType;
}

  // proto:  void QVector4D::setW(float w);
impl<'a> /*trait*/ QVector4D_setW<()> for (f32) {
  fn setW(self , rsthis: & QVector4D) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector4D4setWEf()};
    let arg0 = self  as c_float;
     unsafe {_ZN9QVector4D4setWEf(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QVector4D::QVector4D(const QVector2D & vector, float zpos, float wpos);
impl /*struct*/ QVector4D {
  pub fn New<T: QVector4D_New>(value: T) -> QVector4D {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QVector4D_New {
  fn New(self) -> QVector4D;
}

  // proto:  void QVector4D::QVector4D(const QVector2D & vector, float zpos, float wpos);
impl<'a> /*trait*/ QVector4D_New for (&'a QVector2D, f32, f32) {
  fn New(self) -> QVector4D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector4DC1ERK9QVector2Dff()};
    let ctysz: c_int = unsafe{QVector4D_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    // unsafe {_ZN9QVector4DC1ERK9QVector2Dff(qthis, arg0, arg1, arg2)};
    let qthis: *mut c_void = unsafe {dector_ZN9QVector4DC1ERK9QVector2Dff(arg0, arg1, arg2)};
    let rsthis = QVector4D{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPointF QVector4D::toPointF();
impl /*struct*/ QVector4D {
  pub fn toPointF<RetType, T: QVector4D_toPointF<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toPointF(self);
    // return 1;
  }
}

pub trait QVector4D_toPointF<RetType> {
  fn toPointF(self , rsthis: & QVector4D) -> RetType;
}

  // proto:  QPointF QVector4D::toPointF();
impl<'a> /*trait*/ QVector4D_toPointF<QPointF> for () {
  fn toPointF(self , rsthis: & QVector4D) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector4D8toPointFEv()};
    let mut ret = unsafe {_ZNK9QVector4D8toPointFEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  float QVector4D::y();
impl /*struct*/ QVector4D {
  pub fn y<RetType, T: QVector4D_y<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.y(self);
    // return 1;
  }
}

pub trait QVector4D_y<RetType> {
  fn y(self , rsthis: & QVector4D) -> RetType;
}

  // proto:  float QVector4D::y();
impl<'a> /*trait*/ QVector4D_y<()> for () {
  fn y(self , rsthis: & QVector4D) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector4D1yEv()};
     unsafe {_ZNK9QVector4D1yEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QVector2D QVector4D::toVector2D();
impl /*struct*/ QVector4D {
  pub fn toVector2D<RetType, T: QVector4D_toVector2D<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toVector2D(self);
    // return 1;
  }
}

pub trait QVector4D_toVector2D<RetType> {
  fn toVector2D(self , rsthis: & QVector4D) -> RetType;
}

  // proto:  QVector2D QVector4D::toVector2D();
impl<'a> /*trait*/ QVector4D_toVector2D<QVector2D> for () {
  fn toVector2D(self , rsthis: & QVector4D) -> QVector2D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector4D10toVector2DEv()};
    let mut ret = unsafe {_ZNK9QVector4D10toVector2DEv(rsthis.qclsinst)};
    let mut ret1 = QVector2D::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QVector4D::setZ(float z);
impl /*struct*/ QVector4D {
  pub fn setZ<RetType, T: QVector4D_setZ<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setZ(self);
    // return 1;
  }
}

pub trait QVector4D_setZ<RetType> {
  fn setZ(self , rsthis: & QVector4D) -> RetType;
}

  // proto:  void QVector4D::setZ(float z);
impl<'a> /*trait*/ QVector4D_setZ<()> for (f32) {
  fn setZ(self , rsthis: & QVector4D) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector4D4setZEf()};
    let arg0 = self  as c_float;
     unsafe {_ZN9QVector4D4setZEf(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QVector4D::QVector4D(const QVector2D & vector);
impl<'a> /*trait*/ QVector4D_New for (&'a QVector2D) {
  fn New(self) -> QVector4D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector4DC1ERK9QVector2D()};
    let ctysz: c_int = unsafe{QVector4D_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QVector4DC1ERK9QVector2D(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN9QVector4DC1ERK9QVector2D(arg0)};
    let rsthis = QVector4D{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QVector4D::normalize();
impl /*struct*/ QVector4D {
  pub fn normalize<RetType, T: QVector4D_normalize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.normalize(self);
    // return 1;
  }
}

pub trait QVector4D_normalize<RetType> {
  fn normalize(self , rsthis: & QVector4D) -> RetType;
}

  // proto:  void QVector4D::normalize();
impl<'a> /*trait*/ QVector4D_normalize<()> for () {
  fn normalize(self , rsthis: & QVector4D) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector4D9normalizeEv()};
     unsafe {_ZN9QVector4D9normalizeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QVector4D::QVector4D(float xpos, float ypos, float zpos, float wpos);
impl<'a> /*trait*/ QVector4D_New for (f32, f32, f32, f32) {
  fn New(self) -> QVector4D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector4DC1Effff()};
    let ctysz: c_int = unsafe{QVector4D_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
    // unsafe {_ZN9QVector4DC1Effff(qthis, arg0, arg1, arg2, arg3)};
    let qthis: *mut c_void = unsafe {dector_ZN9QVector4DC1Effff(arg0, arg1, arg2, arg3)};
    let rsthis = QVector4D{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QVector4D::QVector4D(const QVector3D & vector, float wpos);
impl<'a> /*trait*/ QVector4D_New for (&'a QVector3D, f32) {
  fn New(self) -> QVector4D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector4DC1ERK9QVector3Df()};
    let ctysz: c_int = unsafe{QVector4D_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_float;
    // unsafe {_ZN9QVector4DC1ERK9QVector3Df(qthis, arg0, arg1)};
    let qthis: *mut c_void = unsafe {dector_ZN9QVector4DC1ERK9QVector3Df(arg0, arg1)};
    let rsthis = QVector4D{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QVector4D::QVector4D(const QPointF & point);
impl<'a> /*trait*/ QVector4D_New for (&'a QPointF) {
  fn New(self) -> QVector4D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector4DC1ERK7QPointF()};
    let ctysz: c_int = unsafe{QVector4D_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QVector4DC1ERK7QPointF(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN9QVector4DC1ERK7QPointF(arg0)};
    let rsthis = QVector4D{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  float QVector4D::z();
impl /*struct*/ QVector4D {
  pub fn z<RetType, T: QVector4D_z<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.z(self);
    // return 1;
  }
}

pub trait QVector4D_z<RetType> {
  fn z(self , rsthis: & QVector4D) -> RetType;
}

  // proto:  float QVector4D::z();
impl<'a> /*trait*/ QVector4D_z<f32> for () {
  fn z(self , rsthis: & QVector4D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector4D1zEv()};
    let mut ret = unsafe {_ZNK9QVector4D1zEv(rsthis.qclsinst)};
    return ret as f32;
    // return 1;
  }
}

  // proto:  void QVector4D::QVector4D();
impl<'a> /*trait*/ QVector4D_New for () {
  fn New(self) -> QVector4D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector4DC1Ev()};
    let ctysz: c_int = unsafe{QVector4D_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN9QVector4DC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN9QVector4DC1Ev()};
    let rsthis = QVector4D{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QVector4D::setX(float x);
impl /*struct*/ QVector4D {
  pub fn setX<RetType, T: QVector4D_setX<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setX(self);
    // return 1;
  }
}

pub trait QVector4D_setX<RetType> {
  fn setX(self , rsthis: & QVector4D) -> RetType;
}

  // proto:  void QVector4D::setX(float x);
impl<'a> /*trait*/ QVector4D_setX<()> for (f32) {
  fn setX(self , rsthis: & QVector4D) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector4D4setXEf()};
    let arg0 = self  as c_float;
     unsafe {_ZN9QVector4D4setXEf(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QVector4D::setY(float y);
impl /*struct*/ QVector4D {
  pub fn setY<RetType, T: QVector4D_setY<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setY(self);
    // return 1;
  }
}

pub trait QVector4D_setY<RetType> {
  fn setY(self , rsthis: & QVector4D) -> RetType;
}

  // proto:  void QVector4D::setY(float y);
impl<'a> /*trait*/ QVector4D_setY<()> for (f32) {
  fn setY(self , rsthis: & QVector4D) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector4D4setYEf()};
    let arg0 = self  as c_float;
     unsafe {_ZN9QVector4D4setYEf(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QVector4D::QVector4D(const QPoint & point);
impl<'a> /*trait*/ QVector4D_New for (&'a QPoint) {
  fn New(self) -> QVector4D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector4DC1ERK6QPoint()};
    let ctysz: c_int = unsafe{QVector4D_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QVector4DC1ERK6QPoint(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN9QVector4DC1ERK6QPoint(arg0)};
    let rsthis = QVector4D{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QVector3D QVector4D::toVector3D();
impl /*struct*/ QVector4D {
  pub fn toVector3D<RetType, T: QVector4D_toVector3D<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toVector3D(self);
    // return 1;
  }
}

pub trait QVector4D_toVector3D<RetType> {
  fn toVector3D(self , rsthis: & QVector4D) -> RetType;
}

  // proto:  QVector3D QVector4D::toVector3D();
impl<'a> /*trait*/ QVector4D_toVector3D<QVector3D> for () {
  fn toVector3D(self , rsthis: & QVector4D) -> QVector3D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector4D10toVector3DEv()};
    let mut ret = unsafe {_ZNK9QVector4D10toVector3DEv(rsthis.qclsinst)};
    let mut ret1 = QVector3D::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  float QVector4D::x();
impl /*struct*/ QVector4D {
  pub fn x<RetType, T: QVector4D_x<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.x(self);
    // return 1;
  }
}

pub trait QVector4D_x<RetType> {
  fn x(self , rsthis: & QVector4D) -> RetType;
}

  // proto:  float QVector4D::x();
impl<'a> /*trait*/ QVector4D_x<()> for () {
  fn x(self , rsthis: & QVector4D) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector4D1xEv()};
     unsafe {_ZNK9QVector4D1xEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QVector2D QVector4D::toVector2DAffine();
impl /*struct*/ QVector4D {
  pub fn toVector2DAffine<RetType, T: QVector4D_toVector2DAffine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toVector2DAffine(self);
    // return 1;
  }
}

pub trait QVector4D_toVector2DAffine<RetType> {
  fn toVector2DAffine(self , rsthis: & QVector4D) -> RetType;
}

  // proto:  QVector2D QVector4D::toVector2DAffine();
impl<'a> /*trait*/ QVector4D_toVector2DAffine<QVector2D> for () {
  fn toVector2DAffine(self , rsthis: & QVector4D) -> QVector2D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector4D16toVector2DAffineEv()};
    let mut ret = unsafe {_ZNK9QVector4D16toVector2DAffineEv(rsthis.qclsinst)};
    let mut ret1 = QVector2D::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  float QVector4D::length();
impl /*struct*/ QVector4D {
  pub fn length<RetType, T: QVector4D_length<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.length(self);
    // return 1;
  }
}

pub trait QVector4D_length<RetType> {
  fn length(self , rsthis: & QVector4D) -> RetType;
}

  // proto:  float QVector4D::length();
impl<'a> /*trait*/ QVector4D_length<f32> for () {
  fn length(self , rsthis: & QVector4D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector4D6lengthEv()};
    let mut ret = unsafe {_ZNK9QVector4D6lengthEv(rsthis.qclsinst)};
    return ret as f32;
    // return 1;
  }
}

  // proto:  void QVector4D::QVector4D(const QVector3D & vector);
impl<'a> /*trait*/ QVector4D_New for (&'a QVector3D) {
  fn New(self) -> QVector4D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector4DC1ERK9QVector3D()};
    let ctysz: c_int = unsafe{QVector4D_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QVector4DC1ERK9QVector3D(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN9QVector4DC1ERK9QVector3D(arg0)};
    let rsthis = QVector4D{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto: static float QVector4D::dotProduct(const QVector4D & v1, const QVector4D & v2);
impl /*struct*/ QVector4D {
  pub fn dotProduct_s<RetType, T: QVector4D_dotProduct_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.dotProduct_s();
    // return 1;
  }
}

pub trait QVector4D_dotProduct_s<RetType> {
  fn dotProduct_s(self ) -> RetType;
}

  // proto: static float QVector4D::dotProduct(const QVector4D & v1, const QVector4D & v2);
impl<'a> /*trait*/ QVector4D_dotProduct_s<f32> for (&'a QVector4D, &'a QVector4D) {
  fn dotProduct_s(self ) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector4D10dotProductERKS_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QVector4D10dotProductERKS_S1_(arg0, arg1)};
    return ret as f32;
    // return 1;
  }
}

  // proto:  bool QVector4D::isNull();
impl /*struct*/ QVector4D {
  pub fn isNull<RetType, T: QVector4D_isNull<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QVector4D_isNull<RetType> {
  fn isNull(self , rsthis: & QVector4D) -> RetType;
}

  // proto:  bool QVector4D::isNull();
impl<'a> /*trait*/ QVector4D_isNull<i8> for () {
  fn isNull(self , rsthis: & QVector4D) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector4D6isNullEv()};
    let mut ret = unsafe {_ZNK9QVector4D6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  float QVector4D::lengthSquared();
impl /*struct*/ QVector4D {
  pub fn lengthSquared<RetType, T: QVector4D_lengthSquared<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lengthSquared(self);
    // return 1;
  }
}

pub trait QVector4D_lengthSquared<RetType> {
  fn lengthSquared(self , rsthis: & QVector4D) -> RetType;
}

  // proto:  float QVector4D::lengthSquared();
impl<'a> /*trait*/ QVector4D_lengthSquared<f32> for () {
  fn lengthSquared(self , rsthis: & QVector4D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector4D13lengthSquaredEv()};
    let mut ret = unsafe {_ZNK9QVector4D13lengthSquaredEv(rsthis.qclsinst)};
    return ret as f32;
    // return 1;
  }
}

  // proto:  QVector3D QVector4D::toVector3DAffine();
impl /*struct*/ QVector4D {
  pub fn toVector3DAffine<RetType, T: QVector4D_toVector3DAffine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toVector3DAffine(self);
    // return 1;
  }
}

pub trait QVector4D_toVector3DAffine<RetType> {
  fn toVector3DAffine(self , rsthis: & QVector4D) -> RetType;
}

  // proto:  QVector3D QVector4D::toVector3DAffine();
impl<'a> /*trait*/ QVector4D_toVector3DAffine<QVector3D> for () {
  fn toVector3DAffine(self , rsthis: & QVector4D) -> QVector3D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector4D16toVector3DAffineEv()};
    let mut ret = unsafe {_ZNK9QVector4D16toVector3DAffineEv(rsthis.qclsinst)};
    let mut ret1 = QVector3D::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QPoint QVector4D::toPoint();
impl /*struct*/ QVector4D {
  pub fn toPoint<RetType, T: QVector4D_toPoint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toPoint(self);
    // return 1;
  }
}

pub trait QVector4D_toPoint<RetType> {
  fn toPoint(self , rsthis: & QVector4D) -> RetType;
}

  // proto:  QPoint QVector4D::toPoint();
impl<'a> /*trait*/ QVector4D_toPoint<QPoint> for () {
  fn toPoint(self , rsthis: & QVector4D) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector4D7toPointEv()};
    let mut ret = unsafe {_ZNK9QVector4D7toPointEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  float QVector4D::w();
impl /*struct*/ QVector4D {
  pub fn w<RetType, T: QVector4D_w<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.w(self);
    // return 1;
  }
}

pub trait QVector4D_w<RetType> {
  fn w(self , rsthis: & QVector4D) -> RetType;
}

  // proto:  float QVector4D::w();
impl<'a> /*trait*/ QVector4D_w<f32> for () {
  fn w(self , rsthis: & QVector4D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector4D1wEv()};
    let mut ret = unsafe {_ZNK9QVector4D1wEv(rsthis.qclsinst)};
    return ret as f32;
    // return 1;
  }
}

// <= body block end

