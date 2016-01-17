// auto generated, do not modify.
// created: Sun Jan 17 17:37:11 2016
// src-file: /QtGui/qtransform.h
// dst-file: /src/gui/qtransform.rs
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
use super::qpainterpath::QPainterPath; // 773
use super::qmatrix::QMatrix; // 773
use super::super::core::qrect::QRect; // 771
use super::qpolygon::QPolygon; // 773
use super::super::core::qline::QLineF; // 771
use super::super::core::qrect::QRectF; // 771
use super::super::core::qline::QLine; // 771
use super::qpolygon::QPolygonF; // 773
use super::super::core::qpoint::QPointF; // 771
use super::qregion::QRegion; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QTransform_Class_Size() -> c_int;
  // proto:  QPoint QTransform::map(const QPoint & p);
  fn _ZNK10QTransform3mapERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QPainterPath QTransform::map(const QPainterPath & p);
  fn _ZNK10QTransform3mapERK12QPainterPath(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  qreal QTransform::det();
  fn _ZNK10QTransform3detEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QTransform::map(qreal x, qreal y, qreal * tx, qreal * ty);
  fn _ZNK10QTransform3mapEddPdS0_(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: *mut c_double, arg3: *mut c_double);
  // proto:  void QTransform::setMatrix(qreal m11, qreal m12, qreal m13, qreal m21, qreal m22, qreal m23, qreal m31, qreal m32, qreal m33);
  fn _ZN10QTransform9setMatrixEddddddddd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double, arg5: c_double, arg6: c_double, arg7: c_double, arg8: c_double);
  // proto:  const QMatrix & QTransform::toAffine();
  fn _ZNK10QTransform8toAffineEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTransform::reset();
  fn _ZN10QTransform5resetEv(qthis: u64 /* *mut c_void*/);
  // proto:  qreal QTransform::determinant();
  fn _ZNK10QTransform11determinantEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto: static QTransform QTransform::fromScale(qreal dx, qreal dy);
  fn _ZN10QTransform9fromScaleEdd(arg0: c_double, arg1: c_double) -> *mut c_void;
  // proto:  bool QTransform::isTranslating();
  fn _ZNK10QTransform13isTranslatingEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QPolygon QTransform::mapToPolygon(const QRect & r);
  fn _ZNK10QTransform12mapToPolygonERK5QRect(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  qreal QTransform::m22();
  fn _ZNK10QTransform3m22Ev(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  QRect QTransform::mapRect(const QRect & );
  fn _ZNK10QTransform7mapRectERK5QRect(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTransform::QTransform();
  fn _ZN10QTransformC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  qreal QTransform::m32();
  fn _ZNK10QTransform3m32Ev(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QTransform::map(int x, int y, int * tx, int * ty);
  fn _ZNK10QTransform3mapEiiPiS0_(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_int, arg3: *mut c_int);
  // proto:  QTransform & QTransform::shear(qreal sh, qreal sv);
  fn _ZN10QTransform5shearEdd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double) -> *mut c_void;
  // proto:  void QTransform::QTransform(qreal h11, qreal h12, qreal h21, qreal h22, qreal dx, qreal dy);
  fn _ZN10QTransformC2Edddddd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double, arg5: c_double);
  // proto:  QTransform & QTransform::scale(qreal sx, qreal sy);
  fn _ZN10QTransform5scaleEdd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double) -> *mut c_void;
  // proto:  QPolygon QTransform::map(const QPolygon & a);
  fn _ZNK10QTransform3mapERK8QPolygon(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QTransform QTransform::transposed();
  fn _ZNK10QTransform10transposedEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QLineF QTransform::map(const QLineF & l);
  fn _ZNK10QTransform3mapERK6QLineF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTransform::QTransform(bool );
  fn _ZN10QTransformC2Eb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QTransform & QTransform::translate(qreal dx, qreal dy);
  fn _ZN10QTransform9translateEdd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double) -> *mut c_void;
  // proto:  QRectF QTransform::mapRect(const QRectF & );
  fn _ZNK10QTransform7mapRectERK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto: static QTransform QTransform::fromTranslate(qreal dx, qreal dy);
  fn _ZN10QTransform13fromTranslateEdd(arg0: c_double, arg1: c_double) -> *mut c_void;
  // proto:  QLine QTransform::map(const QLine & l);
  fn _ZNK10QTransform3mapERK5QLine(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QTransform::isInvertible();
  fn _ZNK10QTransform12isInvertibleEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto: static bool QTransform::quadToQuad(const QPolygonF & one, const QPolygonF & two, QTransform & result);
  fn _ZN10QTransform10quadToQuadERK9QPolygonFS2_RS_(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> c_char;
  // proto: static bool QTransform::squareToQuad(const QPolygonF & square, QTransform & result);
  fn _ZN10QTransform12squareToQuadERK9QPolygonFRS_(arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto:  QPointF QTransform::map(const QPointF & p);
  fn _ZNK10QTransform3mapERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QPolygonF QTransform::map(const QPolygonF & a);
  fn _ZNK10QTransform3mapERK9QPolygonF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  qreal QTransform::m31();
  fn _ZNK10QTransform3m31Ev(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QTransform::QTransform(const QMatrix & mtx);
  fn _ZN10QTransformC2ERK7QMatrix(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTransform::QTransform(qreal h11, qreal h12, qreal h13, qreal h21, qreal h22, qreal h23, qreal h31, qreal h32, qreal h33);
  fn _ZN10QTransformC2Eddddddddd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double, arg5: c_double, arg6: c_double, arg7: c_double, arg8: c_double);
  // proto:  QRegion QTransform::map(const QRegion & r);
  fn _ZNK10QTransform3mapERK7QRegion(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QTransform::isRotating();
  fn _ZNK10QTransform10isRotatingEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  qreal QTransform::m33();
  fn _ZNK10QTransform3m33Ev(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  qreal QTransform::m13();
  fn _ZNK10QTransform3m13Ev(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  qreal QTransform::m21();
  fn _ZNK10QTransform3m21Ev(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  bool QTransform::isScaling();
  fn _ZNK10QTransform9isScalingEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QTransform QTransform::inverted(bool * invertible);
  fn _ZNK10QTransform8invertedEPb(qthis: u64 /* *mut c_void*/, arg0: *mut c_char) -> *mut c_void;
  // proto:  bool QTransform::isAffine();
  fn _ZNK10QTransform8isAffineEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  qreal QTransform::m11();
  fn _ZNK10QTransform3m11Ev(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  bool QTransform::isIdentity();
  fn _ZNK10QTransform10isIdentityEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto: static bool QTransform::quadToSquare(const QPolygonF & quad, QTransform & result);
  fn _ZN10QTransform12quadToSquareERK9QPolygonFRS_(arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto:  QTransform QTransform::adjoint();
  fn _ZNK10QTransform7adjointEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  qreal QTransform::dx();
  fn _ZNK10QTransform2dxEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  qreal QTransform::m23();
  fn _ZNK10QTransform3m23Ev(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  qreal QTransform::dy();
  fn _ZNK10QTransform2dyEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QTransform::QTransform(qreal h11, qreal h12, qreal h13, qreal h21, qreal h22, qreal h23, qreal h31, qreal h32, qreal h33, bool );
  fn _ZN10QTransformC2Edddddddddb(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double, arg5: c_double, arg6: c_double, arg7: c_double, arg8: c_double, arg9: c_char);
  // proto:  qreal QTransform::m12();
  fn _ZNK10QTransform3m12Ev(qthis: u64 /* *mut c_void*/) -> c_double;
} // <= ext block end

// body block begin =>
// class sizeof(QTransform)=88
#[derive(Default)]
pub struct QTransform {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QTransform {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTransform {
    return QTransform{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QPoint QTransform::map(const QPoint & p);
impl /*struct*/ QTransform {
  pub fn map<RetType, T: QTransform_map<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.map(self);
    // return 1;
  }
}

pub trait QTransform_map<RetType> {
  fn map(self , rsthis: & QTransform) -> RetType;
}

  // proto:  QPoint QTransform::map(const QPoint & p);
impl<'a> /*trait*/ QTransform_map<QPoint> for (&'a QPoint) {
  fn map(self , rsthis: & QTransform) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3mapERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QTransform3mapERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPainterPath QTransform::map(const QPainterPath & p);
impl<'a> /*trait*/ QTransform_map<QPainterPath> for (&'a QPainterPath) {
  fn map(self , rsthis: & QTransform) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3mapERK12QPainterPath()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QTransform3mapERK12QPainterPath(rsthis.qclsinst, arg0)};
    let mut ret1 = QPainterPath::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QTransform::det();
impl /*struct*/ QTransform {
  pub fn det<RetType, T: QTransform_det<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.det(self);
    // return 1;
  }
}

pub trait QTransform_det<RetType> {
  fn det(self , rsthis: & QTransform) -> RetType;
}

  // proto:  qreal QTransform::det();
impl<'a> /*trait*/ QTransform_det<f64> for () {
  fn det(self , rsthis: & QTransform) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3detEv()};
    let mut ret = unsafe {_ZNK10QTransform3detEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QTransform::map(qreal x, qreal y, qreal * tx, qreal * ty);
impl<'a> /*trait*/ QTransform_map<()> for (f64, f64, &'a mut Vec<f64>, &'a mut Vec<f64>) {
  fn map(self , rsthis: & QTransform) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3mapEddPdS0_()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2.as_ptr()  as *mut c_double;
    let arg3 = self.3.as_ptr()  as *mut c_double;
     unsafe {_ZNK10QTransform3mapEddPdS0_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QTransform::setMatrix(qreal m11, qreal m12, qreal m13, qreal m21, qreal m22, qreal m23, qreal m31, qreal m32, qreal m33);
impl /*struct*/ QTransform {
  pub fn setMatrix<RetType, T: QTransform_setMatrix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMatrix(self);
    // return 1;
  }
}

pub trait QTransform_setMatrix<RetType> {
  fn setMatrix(self , rsthis: & QTransform) -> RetType;
}

  // proto:  void QTransform::setMatrix(qreal m11, qreal m12, qreal m13, qreal m21, qreal m22, qreal m23, qreal m31, qreal m32, qreal m33);
impl<'a> /*trait*/ QTransform_setMatrix<()> for (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
  fn setMatrix(self , rsthis: & QTransform) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN10QTransform9setMatrixEddddddddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    let arg5 = self.5  as c_double;
    let arg6 = self.6  as c_double;
    let arg7 = self.7  as c_double;
    let arg8 = self.8  as c_double;
     unsafe {_ZN10QTransform9setMatrixEddddddddd(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)};
    // return 1;
  }
}

  // proto:  const QMatrix & QTransform::toAffine();
impl /*struct*/ QTransform {
  pub fn toAffine<RetType, T: QTransform_toAffine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toAffine(self);
    // return 1;
  }
}

pub trait QTransform_toAffine<RetType> {
  fn toAffine(self , rsthis: & QTransform) -> RetType;
}

  // proto:  const QMatrix & QTransform::toAffine();
impl<'a> /*trait*/ QTransform_toAffine<QMatrix> for () {
  fn toAffine(self , rsthis: & QTransform) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform8toAffineEv()};
    let mut ret = unsafe {_ZNK10QTransform8toAffineEv(rsthis.qclsinst)};
    let mut ret1 = QMatrix::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTransform::reset();
impl /*struct*/ QTransform {
  pub fn reset<RetType, T: QTransform_reset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.reset(self);
    // return 1;
  }
}

pub trait QTransform_reset<RetType> {
  fn reset(self , rsthis: & QTransform) -> RetType;
}

  // proto:  void QTransform::reset();
impl<'a> /*trait*/ QTransform_reset<()> for () {
  fn reset(self , rsthis: & QTransform) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN10QTransform5resetEv()};
     unsafe {_ZN10QTransform5resetEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qreal QTransform::determinant();
impl /*struct*/ QTransform {
  pub fn determinant<RetType, T: QTransform_determinant<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.determinant(self);
    // return 1;
  }
}

pub trait QTransform_determinant<RetType> {
  fn determinant(self , rsthis: & QTransform) -> RetType;
}

  // proto:  qreal QTransform::determinant();
impl<'a> /*trait*/ QTransform_determinant<f64> for () {
  fn determinant(self , rsthis: & QTransform) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform11determinantEv()};
    let mut ret = unsafe {_ZNK10QTransform11determinantEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto: static QTransform QTransform::fromScale(qreal dx, qreal dy);
impl /*struct*/ QTransform {
  pub fn fromScale_s<RetType, T: QTransform_fromScale_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromScale_s();
    // return 1;
  }
}

pub trait QTransform_fromScale_s<RetType> {
  fn fromScale_s(self ) -> RetType;
}

  // proto: static QTransform QTransform::fromScale(qreal dx, qreal dy);
impl<'a> /*trait*/ QTransform_fromScale_s<QTransform> for (f64, f64) {
  fn fromScale_s(self ) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN10QTransform9fromScaleEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {_ZN10QTransform9fromScaleEdd(arg0, arg1)};
    let mut ret1 = QTransform::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QTransform::isTranslating();
impl /*struct*/ QTransform {
  pub fn isTranslating<RetType, T: QTransform_isTranslating<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isTranslating(self);
    // return 1;
  }
}

pub trait QTransform_isTranslating<RetType> {
  fn isTranslating(self , rsthis: & QTransform) -> RetType;
}

  // proto:  bool QTransform::isTranslating();
impl<'a> /*trait*/ QTransform_isTranslating<i8> for () {
  fn isTranslating(self , rsthis: & QTransform) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform13isTranslatingEv()};
    let mut ret = unsafe {_ZNK10QTransform13isTranslatingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QPolygon QTransform::mapToPolygon(const QRect & r);
impl /*struct*/ QTransform {
  pub fn mapToPolygon<RetType, T: QTransform_mapToPolygon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapToPolygon(self);
    // return 1;
  }
}

pub trait QTransform_mapToPolygon<RetType> {
  fn mapToPolygon(self , rsthis: & QTransform) -> RetType;
}

  // proto:  QPolygon QTransform::mapToPolygon(const QRect & r);
impl<'a> /*trait*/ QTransform_mapToPolygon<QPolygon> for (&'a QRect) {
  fn mapToPolygon(self , rsthis: & QTransform) -> QPolygon {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform12mapToPolygonERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QTransform12mapToPolygonERK5QRect(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygon::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QTransform::m22();
impl /*struct*/ QTransform {
  pub fn m22<RetType, T: QTransform_m22<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.m22(self);
    // return 1;
  }
}

pub trait QTransform_m22<RetType> {
  fn m22(self , rsthis: & QTransform) -> RetType;
}

  // proto:  qreal QTransform::m22();
impl<'a> /*trait*/ QTransform_m22<f64> for () {
  fn m22(self , rsthis: & QTransform) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3m22Ev()};
    let mut ret = unsafe {_ZNK10QTransform3m22Ev(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QRect QTransform::mapRect(const QRect & );
impl /*struct*/ QTransform {
  pub fn mapRect<RetType, T: QTransform_mapRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapRect(self);
    // return 1;
  }
}

pub trait QTransform_mapRect<RetType> {
  fn mapRect(self , rsthis: & QTransform) -> RetType;
}

  // proto:  QRect QTransform::mapRect(const QRect & );
impl<'a> /*trait*/ QTransform_mapRect<QRect> for (&'a QRect) {
  fn mapRect(self , rsthis: & QTransform) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform7mapRectERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QTransform7mapRectERK5QRect(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTransform::QTransform();
impl /*struct*/ QTransform {
  pub fn new<T: QTransform_new>(value: T) -> QTransform {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QTransform_new {
  fn new(self) -> QTransform;
}

  // proto:  void QTransform::QTransform();
impl<'a> /*trait*/ QTransform_new for () {
  fn new(self) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN10QTransformC2Ev()};
    let ctysz: c_int = unsafe{QTransform_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN10QTransformC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QTransform{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  qreal QTransform::m32();
impl /*struct*/ QTransform {
  pub fn m32<RetType, T: QTransform_m32<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.m32(self);
    // return 1;
  }
}

pub trait QTransform_m32<RetType> {
  fn m32(self , rsthis: & QTransform) -> RetType;
}

  // proto:  qreal QTransform::m32();
impl<'a> /*trait*/ QTransform_m32<f64> for () {
  fn m32(self , rsthis: & QTransform) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3m32Ev()};
    let mut ret = unsafe {_ZNK10QTransform3m32Ev(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QTransform::map(int x, int y, int * tx, int * ty);
impl<'a> /*trait*/ QTransform_map<()> for (i32, i32, &'a mut Vec<i32>, &'a mut Vec<i32>) {
  fn map(self , rsthis: & QTransform) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3mapEiiPiS0_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.as_ptr()  as *mut c_int;
    let arg3 = self.3.as_ptr()  as *mut c_int;
     unsafe {_ZNK10QTransform3mapEiiPiS0_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  QTransform & QTransform::shear(qreal sh, qreal sv);
impl /*struct*/ QTransform {
  pub fn shear<RetType, T: QTransform_shear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.shear(self);
    // return 1;
  }
}

pub trait QTransform_shear<RetType> {
  fn shear(self , rsthis: & QTransform) -> RetType;
}

  // proto:  QTransform & QTransform::shear(qreal sh, qreal sv);
impl<'a> /*trait*/ QTransform_shear<QTransform> for (f64, f64) {
  fn shear(self , rsthis: & QTransform) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN10QTransform5shearEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {_ZN10QTransform5shearEdd(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QTransform::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTransform::QTransform(qreal h11, qreal h12, qreal h21, qreal h22, qreal dx, qreal dy);
impl<'a> /*trait*/ QTransform_new for (f64, f64, f64, f64, f64, f64) {
  fn new(self) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN10QTransformC2Edddddd()};
    let ctysz: c_int = unsafe{QTransform_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    let arg5 = self.5  as c_double;
    unsafe {_ZN10QTransformC2Edddddd(qthis_ph, arg0, arg1, arg2, arg3, arg4, arg5)};
    let qthis: u64 = qthis_ph;
    let rsthis = QTransform{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QTransform & QTransform::scale(qreal sx, qreal sy);
impl /*struct*/ QTransform {
  pub fn scale<RetType, T: QTransform_scale<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.scale(self);
    // return 1;
  }
}

pub trait QTransform_scale<RetType> {
  fn scale(self , rsthis: & QTransform) -> RetType;
}

  // proto:  QTransform & QTransform::scale(qreal sx, qreal sy);
impl<'a> /*trait*/ QTransform_scale<QTransform> for (f64, f64) {
  fn scale(self , rsthis: & QTransform) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN10QTransform5scaleEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {_ZN10QTransform5scaleEdd(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QTransform::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPolygon QTransform::map(const QPolygon & a);
impl<'a> /*trait*/ QTransform_map<QPolygon> for (&'a QPolygon) {
  fn map(self , rsthis: & QTransform) -> QPolygon {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3mapERK8QPolygon()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QTransform3mapERK8QPolygon(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygon::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QTransform QTransform::transposed();
impl /*struct*/ QTransform {
  pub fn transposed<RetType, T: QTransform_transposed<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.transposed(self);
    // return 1;
  }
}

pub trait QTransform_transposed<RetType> {
  fn transposed(self , rsthis: & QTransform) -> RetType;
}

  // proto:  QTransform QTransform::transposed();
impl<'a> /*trait*/ QTransform_transposed<QTransform> for () {
  fn transposed(self , rsthis: & QTransform) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform10transposedEv()};
    let mut ret = unsafe {_ZNK10QTransform10transposedEv(rsthis.qclsinst)};
    let mut ret1 = QTransform::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QLineF QTransform::map(const QLineF & l);
impl<'a> /*trait*/ QTransform_map<QLineF> for (&'a QLineF) {
  fn map(self , rsthis: & QTransform) -> QLineF {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3mapERK6QLineF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QTransform3mapERK6QLineF(rsthis.qclsinst, arg0)};
    let mut ret1 = QLineF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTransform::QTransform(bool );
impl<'a> /*trait*/ QTransform_new for (i8) {
  fn new(self) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN10QTransformC2Eb()};
    let ctysz: c_int = unsafe{QTransform_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_char;
    unsafe {_ZN10QTransformC2Eb(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QTransform{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QTransform & QTransform::translate(qreal dx, qreal dy);
impl /*struct*/ QTransform {
  pub fn translate<RetType, T: QTransform_translate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.translate(self);
    // return 1;
  }
}

pub trait QTransform_translate<RetType> {
  fn translate(self , rsthis: & QTransform) -> RetType;
}

  // proto:  QTransform & QTransform::translate(qreal dx, qreal dy);
impl<'a> /*trait*/ QTransform_translate<QTransform> for (f64, f64) {
  fn translate(self , rsthis: & QTransform) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN10QTransform9translateEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {_ZN10QTransform9translateEdd(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QTransform::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QRectF QTransform::mapRect(const QRectF & );
impl<'a> /*trait*/ QTransform_mapRect<QRectF> for (&'a QRectF) {
  fn mapRect(self , rsthis: & QTransform) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform7mapRectERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QTransform7mapRectERK6QRectF(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QTransform QTransform::fromTranslate(qreal dx, qreal dy);
impl /*struct*/ QTransform {
  pub fn fromTranslate_s<RetType, T: QTransform_fromTranslate_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromTranslate_s();
    // return 1;
  }
}

pub trait QTransform_fromTranslate_s<RetType> {
  fn fromTranslate_s(self ) -> RetType;
}

  // proto: static QTransform QTransform::fromTranslate(qreal dx, qreal dy);
impl<'a> /*trait*/ QTransform_fromTranslate_s<QTransform> for (f64, f64) {
  fn fromTranslate_s(self ) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN10QTransform13fromTranslateEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {_ZN10QTransform13fromTranslateEdd(arg0, arg1)};
    let mut ret1 = QTransform::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QLine QTransform::map(const QLine & l);
impl<'a> /*trait*/ QTransform_map<QLine> for (&'a QLine) {
  fn map(self , rsthis: & QTransform) -> QLine {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3mapERK5QLine()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QTransform3mapERK5QLine(rsthis.qclsinst, arg0)};
    let mut ret1 = QLine::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QTransform::isInvertible();
impl /*struct*/ QTransform {
  pub fn isInvertible<RetType, T: QTransform_isInvertible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isInvertible(self);
    // return 1;
  }
}

pub trait QTransform_isInvertible<RetType> {
  fn isInvertible(self , rsthis: & QTransform) -> RetType;
}

  // proto:  bool QTransform::isInvertible();
impl<'a> /*trait*/ QTransform_isInvertible<i8> for () {
  fn isInvertible(self , rsthis: & QTransform) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform12isInvertibleEv()};
    let mut ret = unsafe {_ZNK10QTransform12isInvertibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static bool QTransform::quadToQuad(const QPolygonF & one, const QPolygonF & two, QTransform & result);
impl /*struct*/ QTransform {
  pub fn quadToQuad_s<RetType, T: QTransform_quadToQuad_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.quadToQuad_s();
    // return 1;
  }
}

pub trait QTransform_quadToQuad_s<RetType> {
  fn quadToQuad_s(self ) -> RetType;
}

  // proto: static bool QTransform::quadToQuad(const QPolygonF & one, const QPolygonF & two, QTransform & result);
impl<'a> /*trait*/ QTransform_quadToQuad_s<i8> for (&'a QPolygonF, &'a QPolygonF, &'a QTransform) {
  fn quadToQuad_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN10QTransform10quadToQuadERK9QPolygonFS2_RS_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QTransform10quadToQuadERK9QPolygonFS2_RS_(arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static bool QTransform::squareToQuad(const QPolygonF & square, QTransform & result);
impl /*struct*/ QTransform {
  pub fn squareToQuad_s<RetType, T: QTransform_squareToQuad_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.squareToQuad_s();
    // return 1;
  }
}

pub trait QTransform_squareToQuad_s<RetType> {
  fn squareToQuad_s(self ) -> RetType;
}

  // proto: static bool QTransform::squareToQuad(const QPolygonF & square, QTransform & result);
impl<'a> /*trait*/ QTransform_squareToQuad_s<i8> for (&'a QPolygonF, &'a QTransform) {
  fn squareToQuad_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN10QTransform12squareToQuadERK9QPolygonFRS_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QTransform12squareToQuadERK9QPolygonFRS_(arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QPointF QTransform::map(const QPointF & p);
impl<'a> /*trait*/ QTransform_map<QPointF> for (&'a QPointF) {
  fn map(self , rsthis: & QTransform) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3mapERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QTransform3mapERK7QPointF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPolygonF QTransform::map(const QPolygonF & a);
impl<'a> /*trait*/ QTransform_map<QPolygonF> for (&'a QPolygonF) {
  fn map(self , rsthis: & QTransform) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3mapERK9QPolygonF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QTransform3mapERK9QPolygonF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygonF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QTransform::m31();
impl /*struct*/ QTransform {
  pub fn m31<RetType, T: QTransform_m31<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.m31(self);
    // return 1;
  }
}

pub trait QTransform_m31<RetType> {
  fn m31(self , rsthis: & QTransform) -> RetType;
}

  // proto:  qreal QTransform::m31();
impl<'a> /*trait*/ QTransform_m31<f64> for () {
  fn m31(self , rsthis: & QTransform) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3m31Ev()};
    let mut ret = unsafe {_ZNK10QTransform3m31Ev(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QTransform::QTransform(const QMatrix & mtx);
impl<'a> /*trait*/ QTransform_new for (&'a QMatrix) {
  fn new(self) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN10QTransformC2ERK7QMatrix()};
    let ctysz: c_int = unsafe{QTransform_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QTransformC2ERK7QMatrix(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QTransform{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTransform::QTransform(qreal h11, qreal h12, qreal h13, qreal h21, qreal h22, qreal h23, qreal h31, qreal h32, qreal h33);
impl<'a> /*trait*/ QTransform_new for (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
  fn new(self) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN10QTransformC2Eddddddddd()};
    let ctysz: c_int = unsafe{QTransform_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    let arg5 = self.5  as c_double;
    let arg6 = self.6  as c_double;
    let arg7 = self.7  as c_double;
    let arg8 = self.8  as c_double;
    unsafe {_ZN10QTransformC2Eddddddddd(qthis_ph, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)};
    let qthis: u64 = qthis_ph;
    let rsthis = QTransform{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QRegion QTransform::map(const QRegion & r);
impl<'a> /*trait*/ QTransform_map<QRegion> for (&'a QRegion) {
  fn map(self , rsthis: & QTransform) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3mapERK7QRegion()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QTransform3mapERK7QRegion(rsthis.qclsinst, arg0)};
    let mut ret1 = QRegion::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QTransform::isRotating();
impl /*struct*/ QTransform {
  pub fn isRotating<RetType, T: QTransform_isRotating<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isRotating(self);
    // return 1;
  }
}

pub trait QTransform_isRotating<RetType> {
  fn isRotating(self , rsthis: & QTransform) -> RetType;
}

  // proto:  bool QTransform::isRotating();
impl<'a> /*trait*/ QTransform_isRotating<i8> for () {
  fn isRotating(self , rsthis: & QTransform) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform10isRotatingEv()};
    let mut ret = unsafe {_ZNK10QTransform10isRotatingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qreal QTransform::m33();
impl /*struct*/ QTransform {
  pub fn m33<RetType, T: QTransform_m33<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.m33(self);
    // return 1;
  }
}

pub trait QTransform_m33<RetType> {
  fn m33(self , rsthis: & QTransform) -> RetType;
}

  // proto:  qreal QTransform::m33();
impl<'a> /*trait*/ QTransform_m33<f64> for () {
  fn m33(self , rsthis: & QTransform) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3m33Ev()};
    let mut ret = unsafe {_ZNK10QTransform3m33Ev(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QTransform::m13();
impl /*struct*/ QTransform {
  pub fn m13<RetType, T: QTransform_m13<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.m13(self);
    // return 1;
  }
}

pub trait QTransform_m13<RetType> {
  fn m13(self , rsthis: & QTransform) -> RetType;
}

  // proto:  qreal QTransform::m13();
impl<'a> /*trait*/ QTransform_m13<f64> for () {
  fn m13(self , rsthis: & QTransform) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3m13Ev()};
    let mut ret = unsafe {_ZNK10QTransform3m13Ev(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QTransform::m21();
impl /*struct*/ QTransform {
  pub fn m21<RetType, T: QTransform_m21<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.m21(self);
    // return 1;
  }
}

pub trait QTransform_m21<RetType> {
  fn m21(self , rsthis: & QTransform) -> RetType;
}

  // proto:  qreal QTransform::m21();
impl<'a> /*trait*/ QTransform_m21<f64> for () {
  fn m21(self , rsthis: & QTransform) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3m21Ev()};
    let mut ret = unsafe {_ZNK10QTransform3m21Ev(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  bool QTransform::isScaling();
impl /*struct*/ QTransform {
  pub fn isScaling<RetType, T: QTransform_isScaling<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isScaling(self);
    // return 1;
  }
}

pub trait QTransform_isScaling<RetType> {
  fn isScaling(self , rsthis: & QTransform) -> RetType;
}

  // proto:  bool QTransform::isScaling();
impl<'a> /*trait*/ QTransform_isScaling<i8> for () {
  fn isScaling(self , rsthis: & QTransform) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform9isScalingEv()};
    let mut ret = unsafe {_ZNK10QTransform9isScalingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QTransform QTransform::inverted(bool * invertible);
impl /*struct*/ QTransform {
  pub fn inverted<RetType, T: QTransform_inverted<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.inverted(self);
    // return 1;
  }
}

pub trait QTransform_inverted<RetType> {
  fn inverted(self , rsthis: & QTransform) -> RetType;
}

  // proto:  QTransform QTransform::inverted(bool * invertible);
impl<'a> /*trait*/ QTransform_inverted<QTransform> for (&'a mut Vec<i8>) {
  fn inverted(self , rsthis: & QTransform) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform8invertedEPb()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZNK10QTransform8invertedEPb(rsthis.qclsinst, arg0)};
    let mut ret1 = QTransform::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QTransform::isAffine();
impl /*struct*/ QTransform {
  pub fn isAffine<RetType, T: QTransform_isAffine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isAffine(self);
    // return 1;
  }
}

pub trait QTransform_isAffine<RetType> {
  fn isAffine(self , rsthis: & QTransform) -> RetType;
}

  // proto:  bool QTransform::isAffine();
impl<'a> /*trait*/ QTransform_isAffine<i8> for () {
  fn isAffine(self , rsthis: & QTransform) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform8isAffineEv()};
    let mut ret = unsafe {_ZNK10QTransform8isAffineEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qreal QTransform::m11();
impl /*struct*/ QTransform {
  pub fn m11<RetType, T: QTransform_m11<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.m11(self);
    // return 1;
  }
}

pub trait QTransform_m11<RetType> {
  fn m11(self , rsthis: & QTransform) -> RetType;
}

  // proto:  qreal QTransform::m11();
impl<'a> /*trait*/ QTransform_m11<f64> for () {
  fn m11(self , rsthis: & QTransform) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3m11Ev()};
    let mut ret = unsafe {_ZNK10QTransform3m11Ev(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  bool QTransform::isIdentity();
impl /*struct*/ QTransform {
  pub fn isIdentity<RetType, T: QTransform_isIdentity<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isIdentity(self);
    // return 1;
  }
}

pub trait QTransform_isIdentity<RetType> {
  fn isIdentity(self , rsthis: & QTransform) -> RetType;
}

  // proto:  bool QTransform::isIdentity();
impl<'a> /*trait*/ QTransform_isIdentity<i8> for () {
  fn isIdentity(self , rsthis: & QTransform) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform10isIdentityEv()};
    let mut ret = unsafe {_ZNK10QTransform10isIdentityEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static bool QTransform::quadToSquare(const QPolygonF & quad, QTransform & result);
impl /*struct*/ QTransform {
  pub fn quadToSquare_s<RetType, T: QTransform_quadToSquare_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.quadToSquare_s();
    // return 1;
  }
}

pub trait QTransform_quadToSquare_s<RetType> {
  fn quadToSquare_s(self ) -> RetType;
}

  // proto: static bool QTransform::quadToSquare(const QPolygonF & quad, QTransform & result);
impl<'a> /*trait*/ QTransform_quadToSquare_s<i8> for (&'a QPolygonF, &'a QTransform) {
  fn quadToSquare_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN10QTransform12quadToSquareERK9QPolygonFRS_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QTransform12quadToSquareERK9QPolygonFRS_(arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QTransform QTransform::adjoint();
impl /*struct*/ QTransform {
  pub fn adjoint<RetType, T: QTransform_adjoint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.adjoint(self);
    // return 1;
  }
}

pub trait QTransform_adjoint<RetType> {
  fn adjoint(self , rsthis: & QTransform) -> RetType;
}

  // proto:  QTransform QTransform::adjoint();
impl<'a> /*trait*/ QTransform_adjoint<QTransform> for () {
  fn adjoint(self , rsthis: & QTransform) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform7adjointEv()};
    let mut ret = unsafe {_ZNK10QTransform7adjointEv(rsthis.qclsinst)};
    let mut ret1 = QTransform::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QTransform::dx();
impl /*struct*/ QTransform {
  pub fn dx<RetType, T: QTransform_dx<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.dx(self);
    // return 1;
  }
}

pub trait QTransform_dx<RetType> {
  fn dx(self , rsthis: & QTransform) -> RetType;
}

  // proto:  qreal QTransform::dx();
impl<'a> /*trait*/ QTransform_dx<f64> for () {
  fn dx(self , rsthis: & QTransform) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform2dxEv()};
    let mut ret = unsafe {_ZNK10QTransform2dxEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QTransform::m23();
impl /*struct*/ QTransform {
  pub fn m23<RetType, T: QTransform_m23<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.m23(self);
    // return 1;
  }
}

pub trait QTransform_m23<RetType> {
  fn m23(self , rsthis: & QTransform) -> RetType;
}

  // proto:  qreal QTransform::m23();
impl<'a> /*trait*/ QTransform_m23<f64> for () {
  fn m23(self , rsthis: & QTransform) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3m23Ev()};
    let mut ret = unsafe {_ZNK10QTransform3m23Ev(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QTransform::dy();
impl /*struct*/ QTransform {
  pub fn dy<RetType, T: QTransform_dy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.dy(self);
    // return 1;
  }
}

pub trait QTransform_dy<RetType> {
  fn dy(self , rsthis: & QTransform) -> RetType;
}

  // proto:  qreal QTransform::dy();
impl<'a> /*trait*/ QTransform_dy<f64> for () {
  fn dy(self , rsthis: & QTransform) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform2dyEv()};
    let mut ret = unsafe {_ZNK10QTransform2dyEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QTransform::QTransform(qreal h11, qreal h12, qreal h13, qreal h21, qreal h22, qreal h23, qreal h31, qreal h32, qreal h33, bool );
impl<'a> /*trait*/ QTransform_new for (f64, f64, f64, f64, f64, f64, f64, f64, f64, i8) {
  fn new(self) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN10QTransformC2Edddddddddb()};
    let ctysz: c_int = unsafe{QTransform_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    let arg5 = self.5  as c_double;
    let arg6 = self.6  as c_double;
    let arg7 = self.7  as c_double;
    let arg8 = self.8  as c_double;
    let arg9 = self.9  as c_char;
    unsafe {_ZN10QTransformC2Edddddddddb(qthis_ph, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)};
    let qthis: u64 = qthis_ph;
    let rsthis = QTransform{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  qreal QTransform::m12();
impl /*struct*/ QTransform {
  pub fn m12<RetType, T: QTransform_m12<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.m12(self);
    // return 1;
  }
}

pub trait QTransform_m12<RetType> {
  fn m12(self , rsthis: & QTransform) -> RetType;
}

  // proto:  qreal QTransform::m12();
impl<'a> /*trait*/ QTransform_m12<f64> for () {
  fn m12(self , rsthis: & QTransform) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3m12Ev()};
    let mut ret = unsafe {_ZNK10QTransform3m12Ev(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// <= body block end

