// auto generated, do not modify.
// created: Sat Dec 26 10:16:52 2015
// src-file: /QtGui/qmatrix.h
// dst-file: /src/gui/qmatrix.rs
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
use super::super::core::qrect::QRect; // 771
use super::qpolygon::QPolygon; // 773
use super::qpainterpath::QPainterPath; // 773
use super::super::core::qpoint::QPointF; // 771
use super::qpolygon::QPolygonF; // 773
use super::qregion::QRegion; // 773
use super::super::core::qline::QLineF; // 771
use super::super::core::qpoint::QPoint; // 771
use super::super::core::qline::QLine; // 771
use super::super::core::qrect::QRectF; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QMatrix_Class_Size() -> c_int;
  // proto:  QPolygon QMatrix::mapToPolygon(const QRect & r);
  fn _ZNK7QMatrix12mapToPolygonERK5QRect(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QMatrix & QMatrix::scale(qreal sx, qreal sy);
  fn _ZN7QMatrix5scaleEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) -> *mut c_void;
  // proto:  QMatrix & QMatrix::translate(qreal dx, qreal dy);
  fn _ZN7QMatrix9translateEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) -> *mut c_void;
  // proto:  QMatrix & QMatrix::shear(qreal sh, qreal sv);
  fn _ZN7QMatrix5shearEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) -> *mut c_void;
  // proto:  void QMatrix::QMatrix();
  fn dector_ZN7QMatrixC1Ev() -> *mut c_void;
  fn _ZN7QMatrixC1Ev(qthis: *mut c_void);
  // proto:  QPainterPath QMatrix::map(const QPainterPath & p);
  fn _ZNK7QMatrix3mapERK12QPainterPath(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QPointF QMatrix::map(const QPointF & p);
  fn _ZNK7QMatrix3mapERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QPolygonF QMatrix::map(const QPolygonF & a);
  fn _ZNK7QMatrix3mapERK9QPolygonF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QMatrix::map(qreal x, qreal y, qreal * tx, qreal * ty);
  fn _ZNK7QMatrix3mapEddPdS0_(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: *mut c_double, arg3: *mut c_double);
  // proto:  QMatrix & QMatrix::rotate(qreal a);
  fn _ZN7QMatrix6rotateEd(qthis: *mut c_void, arg0: c_double) -> *mut c_void;
  // proto:  QRegion QMatrix::map(const QRegion & r);
  fn _ZNK7QMatrix3mapERK7QRegion(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QMatrix::QMatrix(const QMatrix & matrix);
  fn dector_ZN7QMatrixC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN7QMatrixC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QMatrix::reset();
  fn _ZN7QMatrix5resetEv(qthis: *mut c_void);
  // proto:  QLineF QMatrix::map(const QLineF & l);
  fn _ZNK7QMatrix3mapERK6QLineF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QMatrix QMatrix::inverted(bool * invertible);
  fn _ZNK7QMatrix8invertedEPb(qthis: *mut c_void, arg0: *mut c_char) -> *mut c_void;
  // proto:  QPoint QMatrix::map(const QPoint & p);
  fn _ZNK7QMatrix3mapERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QMatrix::map(int x, int y, int * tx, int * ty);
  fn _ZNK7QMatrix3mapEiiPiS0_(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_int, arg3: *mut c_int);
  // proto:  QLine QMatrix::map(const QLine & l);
  fn _ZNK7QMatrix3mapERK5QLine(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QRectF QMatrix::mapRect(const QRectF & );
  fn _ZNK7QMatrix7mapRectERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QMatrix::setMatrix(qreal m11, qreal m12, qreal m21, qreal m22, qreal dx, qreal dy);
  fn _ZN7QMatrix9setMatrixEdddddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double, arg5: c_double);
  // proto:  bool QMatrix::isIdentity();
  fn _ZNK7QMatrix10isIdentityEv(qthis: *mut c_void) -> c_char;
  // proto:  QRect QMatrix::mapRect(const QRect & );
  fn _ZNK7QMatrix7mapRectERK5QRect(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QMatrix::QMatrix(qreal m11, qreal m12, qreal m21, qreal m22, qreal dx, qreal dy);
  fn dector_ZN7QMatrixC1Edddddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double, arg5: c_double) -> *mut c_void;
  fn _ZN7QMatrixC1Edddddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double, arg5: c_double);
  // proto:  QPolygon QMatrix::map(const QPolygon & a);
  fn _ZNK7QMatrix3mapERK8QPolygon(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QMatrix)=48
pub struct QMatrix {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMatrix {
  pub fn inheritFrom(qthis: *mut c_void) -> QMatrix {
    return QMatrix{qclsinst: qthis};
  }
}
  // proto:  QPolygon QMatrix::mapToPolygon(const QRect & r);
impl /*struct*/ QMatrix {
  pub fn mapToPolygon<RetType, T: QMatrix_mapToPolygon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapToPolygon(self);
    // return 1;
  }
}

pub trait QMatrix_mapToPolygon<RetType> {
  fn mapToPolygon(self , rsthis: & QMatrix) -> RetType;
}

  // proto:  QPolygon QMatrix::mapToPolygon(const QRect & r);
impl<'a> /*trait*/ QMatrix_mapToPolygon<QPolygon> for (&'a QRect) {
  fn mapToPolygon(self , rsthis: & QMatrix) -> QPolygon {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix12mapToPolygonERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QMatrix12mapToPolygonERK5QRect(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygon::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QMatrix & QMatrix::scale(qreal sx, qreal sy);
impl /*struct*/ QMatrix {
  pub fn scale<RetType, T: QMatrix_scale<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.scale(self);
    // return 1;
  }
}

pub trait QMatrix_scale<RetType> {
  fn scale(self , rsthis: & QMatrix) -> RetType;
}

  // proto:  QMatrix & QMatrix::scale(qreal sx, qreal sy);
impl<'a> /*trait*/ QMatrix_scale<QMatrix> for (f64, f64) {
  fn scale(self , rsthis: & QMatrix) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN7QMatrix5scaleEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {_ZN7QMatrix5scaleEdd(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QMatrix::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QMatrix & QMatrix::translate(qreal dx, qreal dy);
impl /*struct*/ QMatrix {
  pub fn translate<RetType, T: QMatrix_translate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.translate(self);
    // return 1;
  }
}

pub trait QMatrix_translate<RetType> {
  fn translate(self , rsthis: & QMatrix) -> RetType;
}

  // proto:  QMatrix & QMatrix::translate(qreal dx, qreal dy);
impl<'a> /*trait*/ QMatrix_translate<QMatrix> for (f64, f64) {
  fn translate(self , rsthis: & QMatrix) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN7QMatrix9translateEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {_ZN7QMatrix9translateEdd(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QMatrix::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QMatrix & QMatrix::shear(qreal sh, qreal sv);
impl /*struct*/ QMatrix {
  pub fn shear<RetType, T: QMatrix_shear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.shear(self);
    // return 1;
  }
}

pub trait QMatrix_shear<RetType> {
  fn shear(self , rsthis: & QMatrix) -> RetType;
}

  // proto:  QMatrix & QMatrix::shear(qreal sh, qreal sv);
impl<'a> /*trait*/ QMatrix_shear<QMatrix> for (f64, f64) {
  fn shear(self , rsthis: & QMatrix) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN7QMatrix5shearEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {_ZN7QMatrix5shearEdd(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QMatrix::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMatrix::QMatrix();
impl /*struct*/ QMatrix {
  pub fn New<T: QMatrix_New>(value: T) -> QMatrix {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QMatrix_New {
  fn New(self) -> QMatrix;
}

  // proto:  void QMatrix::QMatrix();
impl<'a> /*trait*/ QMatrix_New for () {
  fn New(self) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN7QMatrixC1Ev()};
    let ctysz: c_int = unsafe{QMatrix_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN7QMatrixC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN7QMatrixC1Ev()};
    let rsthis = QMatrix{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPainterPath QMatrix::map(const QPainterPath & p);
impl /*struct*/ QMatrix {
  pub fn map<RetType, T: QMatrix_map<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.map(self);
    // return 1;
  }
}

pub trait QMatrix_map<RetType> {
  fn map(self , rsthis: & QMatrix) -> RetType;
}

  // proto:  QPainterPath QMatrix::map(const QPainterPath & p);
impl<'a> /*trait*/ QMatrix_map<QPainterPath> for (&'a QPainterPath) {
  fn map(self , rsthis: & QMatrix) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix3mapERK12QPainterPath()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QMatrix3mapERK12QPainterPath(rsthis.qclsinst, arg0)};
    let mut ret1 = QPainterPath::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QPointF QMatrix::map(const QPointF & p);
impl<'a> /*trait*/ QMatrix_map<QPointF> for (&'a QPointF) {
  fn map(self , rsthis: & QMatrix) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix3mapERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QMatrix3mapERK7QPointF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPointF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QPolygonF QMatrix::map(const QPolygonF & a);
impl<'a> /*trait*/ QMatrix_map<QPolygonF> for (&'a QPolygonF) {
  fn map(self , rsthis: & QMatrix) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix3mapERK9QPolygonF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QMatrix3mapERK9QPolygonF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygonF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMatrix::map(qreal x, qreal y, qreal * tx, qreal * ty);
impl<'a> /*trait*/ QMatrix_map<()> for (f64, f64, &'a mut Vec<f64>, &'a mut Vec<f64>) {
  fn map(self , rsthis: & QMatrix) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix3mapEddPdS0_()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2.as_ptr()  as *mut c_double;
    let arg3 = self.3.as_ptr()  as *mut c_double;
     unsafe {_ZNK7QMatrix3mapEddPdS0_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  QMatrix & QMatrix::rotate(qreal a);
impl /*struct*/ QMatrix {
  pub fn rotate<RetType, T: QMatrix_rotate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rotate(self);
    // return 1;
  }
}

pub trait QMatrix_rotate<RetType> {
  fn rotate(self , rsthis: & QMatrix) -> RetType;
}

  // proto:  QMatrix & QMatrix::rotate(qreal a);
impl<'a> /*trait*/ QMatrix_rotate<QMatrix> for (f64) {
  fn rotate(self , rsthis: & QMatrix) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN7QMatrix6rotateEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZN7QMatrix6rotateEd(rsthis.qclsinst, arg0)};
    let mut ret1 = QMatrix::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QRegion QMatrix::map(const QRegion & r);
impl<'a> /*trait*/ QMatrix_map<QRegion> for (&'a QRegion) {
  fn map(self , rsthis: & QMatrix) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix3mapERK7QRegion()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QMatrix3mapERK7QRegion(rsthis.qclsinst, arg0)};
    let mut ret1 = QRegion::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMatrix::QMatrix(const QMatrix & matrix);
impl<'a> /*trait*/ QMatrix_New for (&'a QMatrix) {
  fn New(self) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN7QMatrixC1ERKS_()};
    let ctysz: c_int = unsafe{QMatrix_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN7QMatrixC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN7QMatrixC1ERKS_(arg0)};
    let rsthis = QMatrix{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QMatrix::reset();
impl /*struct*/ QMatrix {
  pub fn reset<RetType, T: QMatrix_reset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.reset(self);
    // return 1;
  }
}

pub trait QMatrix_reset<RetType> {
  fn reset(self , rsthis: & QMatrix) -> RetType;
}

  // proto:  void QMatrix::reset();
impl<'a> /*trait*/ QMatrix_reset<()> for () {
  fn reset(self , rsthis: & QMatrix) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN7QMatrix5resetEv()};
     unsafe {_ZN7QMatrix5resetEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QLineF QMatrix::map(const QLineF & l);
impl<'a> /*trait*/ QMatrix_map<QLineF> for (&'a QLineF) {
  fn map(self , rsthis: & QMatrix) -> QLineF {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix3mapERK6QLineF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QMatrix3mapERK6QLineF(rsthis.qclsinst, arg0)};
    let mut ret1 = QLineF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QMatrix QMatrix::inverted(bool * invertible);
impl /*struct*/ QMatrix {
  pub fn inverted<RetType, T: QMatrix_inverted<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.inverted(self);
    // return 1;
  }
}

pub trait QMatrix_inverted<RetType> {
  fn inverted(self , rsthis: & QMatrix) -> RetType;
}

  // proto:  QMatrix QMatrix::inverted(bool * invertible);
impl<'a> /*trait*/ QMatrix_inverted<QMatrix> for (&'a mut Vec<i8>) {
  fn inverted(self , rsthis: & QMatrix) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix8invertedEPb()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZNK7QMatrix8invertedEPb(rsthis.qclsinst, arg0)};
    let mut ret1 = QMatrix::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QPoint QMatrix::map(const QPoint & p);
impl<'a> /*trait*/ QMatrix_map<QPoint> for (&'a QPoint) {
  fn map(self , rsthis: & QMatrix) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix3mapERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QMatrix3mapERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QPoint::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMatrix::map(int x, int y, int * tx, int * ty);
impl<'a> /*trait*/ QMatrix_map<()> for (i32, i32, &'a mut Vec<i32>, &'a mut Vec<i32>) {
  fn map(self , rsthis: & QMatrix) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix3mapEiiPiS0_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.as_ptr()  as *mut c_int;
    let arg3 = self.3.as_ptr()  as *mut c_int;
     unsafe {_ZNK7QMatrix3mapEiiPiS0_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  QLine QMatrix::map(const QLine & l);
impl<'a> /*trait*/ QMatrix_map<QLine> for (&'a QLine) {
  fn map(self , rsthis: & QMatrix) -> QLine {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix3mapERK5QLine()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QMatrix3mapERK5QLine(rsthis.qclsinst, arg0)};
    let mut ret1 = QLine::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QRectF QMatrix::mapRect(const QRectF & );
impl /*struct*/ QMatrix {
  pub fn mapRect<RetType, T: QMatrix_mapRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapRect(self);
    // return 1;
  }
}

pub trait QMatrix_mapRect<RetType> {
  fn mapRect(self , rsthis: & QMatrix) -> RetType;
}

  // proto:  QRectF QMatrix::mapRect(const QRectF & );
impl<'a> /*trait*/ QMatrix_mapRect<QRectF> for (&'a QRectF) {
  fn mapRect(self , rsthis: & QMatrix) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix7mapRectERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QMatrix7mapRectERK6QRectF(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMatrix::setMatrix(qreal m11, qreal m12, qreal m21, qreal m22, qreal dx, qreal dy);
impl /*struct*/ QMatrix {
  pub fn setMatrix<RetType, T: QMatrix_setMatrix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMatrix(self);
    // return 1;
  }
}

pub trait QMatrix_setMatrix<RetType> {
  fn setMatrix(self , rsthis: & QMatrix) -> RetType;
}

  // proto:  void QMatrix::setMatrix(qreal m11, qreal m12, qreal m21, qreal m22, qreal dx, qreal dy);
impl<'a> /*trait*/ QMatrix_setMatrix<()> for (f64, f64, f64, f64, f64, f64) {
  fn setMatrix(self , rsthis: & QMatrix) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN7QMatrix9setMatrixEdddddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    let arg5 = self.5  as c_double;
     unsafe {_ZN7QMatrix9setMatrixEdddddd(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5)};
    // return 1;
  }
}

  // proto:  bool QMatrix::isIdentity();
impl /*struct*/ QMatrix {
  pub fn isIdentity<RetType, T: QMatrix_isIdentity<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isIdentity(self);
    // return 1;
  }
}

pub trait QMatrix_isIdentity<RetType> {
  fn isIdentity(self , rsthis: & QMatrix) -> RetType;
}

  // proto:  bool QMatrix::isIdentity();
impl<'a> /*trait*/ QMatrix_isIdentity<i8> for () {
  fn isIdentity(self , rsthis: & QMatrix) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix10isIdentityEv()};
    let mut ret = unsafe {_ZNK7QMatrix10isIdentityEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QRect QMatrix::mapRect(const QRect & );
impl<'a> /*trait*/ QMatrix_mapRect<QRect> for (&'a QRect) {
  fn mapRect(self , rsthis: & QMatrix) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix7mapRectERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QMatrix7mapRectERK5QRect(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMatrix::QMatrix(qreal m11, qreal m12, qreal m21, qreal m22, qreal dx, qreal dy);
impl<'a> /*trait*/ QMatrix_New for (f64, f64, f64, f64, f64, f64) {
  fn New(self) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN7QMatrixC1Edddddd()};
    let ctysz: c_int = unsafe{QMatrix_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    let arg5 = self.5  as c_double;
    // unsafe {_ZN7QMatrixC1Edddddd(qthis, arg0, arg1, arg2, arg3, arg4, arg5)};
    let qthis: *mut c_void = unsafe {dector_ZN7QMatrixC1Edddddd(arg0, arg1, arg2, arg3, arg4, arg5)};
    let rsthis = QMatrix{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPolygon QMatrix::map(const QPolygon & a);
impl<'a> /*trait*/ QMatrix_map<QPolygon> for (&'a QPolygon) {
  fn map(self , rsthis: & QMatrix) -> QPolygon {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix3mapERK8QPolygon()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QMatrix3mapERK8QPolygon(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygon::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

// <= body block end

