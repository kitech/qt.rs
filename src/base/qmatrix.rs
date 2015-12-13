// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpointf::QPointF;
use super::qpolygonf::QPolygonF;
use super::qregion::QRegion;
use super::qlinef::QLineF;
use super::qpainterpath::QPainterPath;
use super::qrect::QRect;
use super::qpoint::QPoint;
use super::qline::QLine;
use super::qrectf::QRectF;
use super::qpolygon::QPolygon;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: double QMatrix::dx();
  fn _ZNK7QMatrix2dxEv() -> i32;
  // proto: void QMatrix::NewQMatrix(bool );
  fn _ZN7QMatrixC1Eb(qthis: *mut c_void, arg0: int8_t) -> i32;
  // proto: double QMatrix::dy();
  fn _ZNK7QMatrix2dyEv() -> i32;
  // proto: QMatrix & QMatrix::scale(qreal sx, qreal sy);
  fn _ZN7QMatrix5scaleEdd(arg0: c_double, arg1: c_double) -> i32;
  // proto: QMatrix & QMatrix::translate(qreal dx, qreal dy);
  fn _ZN7QMatrix9translateEdd(arg0: c_double, arg1: c_double) -> i32;
  // proto: double QMatrix::determinant();
  fn _ZNK7QMatrix11determinantEv() -> i32;
  // proto: QMatrix & QMatrix::shear(qreal sh, qreal sv);
  fn _ZN7QMatrix5shearEdd(arg0: c_double, arg1: c_double) -> i32;
  // proto: void QMatrix::NewQMatrix();
  fn _ZN7QMatrixC1Ev(qthis: *mut c_void) -> i32;
  // proto: double QMatrix::m21();
  fn _ZNK7QMatrix3m21Ev() -> i32;
  // proto: QPointF QMatrix::map(const QPointF & p);
  fn _ZNK7QMatrix3mapERK7QPointF(arg0: *const c_void) -> i32;
  // proto: QPolygonF QMatrix::map(const QPolygonF & a);
  fn _ZNK7QMatrix3mapERK9QPolygonF(arg0: *const c_void) -> i32;
  // proto: void QMatrix::map(qreal x, qreal y, qreal * tx, qreal * ty);
  fn _ZNK7QMatrix3mapEddPdS0_(arg0: c_double, arg1: c_double, arg2: *mut c_double, arg3: *mut c_double) -> i32;
  // proto: QMatrix & QMatrix::rotate(qreal a);
  fn _ZN7QMatrix6rotateEd(arg0: c_double) -> i32;
  // proto: QRegion QMatrix::map(const QRegion & r);
  fn _ZNK7QMatrix3mapERK7QRegion(arg0: *const c_void) -> i32;
  // proto: void QMatrix::setMatrix(qreal m11, qreal m12, qreal m21, qreal m22, qreal dx, qreal dy);
  fn _ZN7QMatrix9setMatrixEdddddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double, arg5: c_double) -> i32;
  // proto: void QMatrix::NewQMatrix(const QMatrix & matrix);
  fn _ZN7QMatrixC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QMatrix::reset();
  fn _ZN7QMatrix5resetEv() -> i32;
  // proto: QLineF QMatrix::map(const QLineF & l);
  fn _ZNK7QMatrix3mapERK6QLineF(arg0: *const c_void) -> i32;
  // proto: QPainterPath QMatrix::map(const QPainterPath & p);
  fn _ZNK7QMatrix3mapERK12QPainterPath(arg0: *const c_void) -> i32;
  // proto: double QMatrix::m11();
  fn _ZNK7QMatrix3m11Ev() -> i32;
  // proto: QPolygon QMatrix::mapToPolygon(const QRect & r);
  fn _ZNK7QMatrix12mapToPolygonERK5QRect(arg0: *const c_void) -> i32;
  // proto: QMatrix QMatrix::inverted(bool * invertible);
  fn _ZNK7QMatrix8invertedEPb(arg0: *mut int8_t) -> i32;
  // proto: QPoint QMatrix::map(const QPoint & p);
  fn _ZNK7QMatrix3mapERK6QPoint(arg0: *const c_void) -> i32;
  // proto: void QMatrix::map(int x, int y, int * tx, int * ty);
  fn _ZNK7QMatrix3mapEiiPiS0_(arg0: c_int, arg1: c_int, arg2: *mut c_int, arg3: *mut c_int) -> i32;
  // proto: QLine QMatrix::map(const QLine & l);
  fn _ZNK7QMatrix3mapERK5QLine(arg0: *const c_void) -> i32;
  // proto: QRectF QMatrix::mapRect(const QRectF & );
  fn _ZNK7QMatrix7mapRectERK6QRectF(arg0: *const c_void) -> i32;
  // proto: bool QMatrix::isIdentity();
  fn _ZNK7QMatrix10isIdentityEv() -> i32;
  // proto: void QMatrix::NewQMatrix(qreal am11, qreal am12, qreal am21, qreal am22, qreal adx, qreal ady, bool );
  fn _ZN7QMatrixC1Eddddddb(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double, arg5: c_double, arg6: int8_t) -> i32;
  // proto: double QMatrix::m12();
  fn _ZNK7QMatrix3m12Ev() -> i32;
  // proto: bool QMatrix::isInvertible();
  fn _ZNK7QMatrix12isInvertibleEv() -> i32;
  // proto: QRect QMatrix::mapRect(const QRect & );
  fn _ZNK7QMatrix7mapRectERK5QRect(arg0: *const c_void) -> i32;
  // proto: void QMatrix::NewQMatrix(qreal m11, qreal m12, qreal m21, qreal m22, qreal dx, qreal dy);
  fn _ZN7QMatrixC1Edddddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double, arg5: c_double) -> i32;
  // proto: double QMatrix::m22();
  fn _ZNK7QMatrix3m22Ev() -> i32;
  // proto: QPolygon QMatrix::map(const QPolygon & a);
  fn _ZNK7QMatrix3mapERK8QPolygon(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QMatrix)=48
pub struct QMatrix {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMatrix {
  pub fn dx<T: QMatrix_dx>(&mut self, value: T) -> i32 {
    value.dx(self);
    return 1;
  }
}

pub trait QMatrix_dx {
  fn dx(self, this: &mut QMatrix) -> i32;
}

// proto: double QMatrix::dx();
impl<'a> /*trait*/ QMatrix_dx for () {
  fn dx(self, this: &mut QMatrix) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix2dxEv()};
    unsafe {_ZNK7QMatrix2dxEv()};
    return 1;
  }
}

impl /*struct*/ QMatrix {
  pub fn NewQMatrix<T: QMatrix_NewQMatrix>(value: T) -> QMatrix {
    let rsthis = value.NewQMatrix();
    return rsthis;
    // return 1;
  }
}

pub trait QMatrix_NewQMatrix {
  fn NewQMatrix(self) -> QMatrix;
}

// proto: void QMatrix::NewQMatrix(bool );
impl<'a> /*trait*/ QMatrix_NewQMatrix for (i8) {
  fn NewQMatrix(self) -> QMatrix {
    let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN7QMatrixC1Eb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QMatrixC1Eb(qthis, arg0)};
    let rsthis = QMatrix{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMatrix {
  pub fn dy<T: QMatrix_dy>(&mut self, value: T) -> i32 {
    value.dy(self);
    return 1;
  }
}

pub trait QMatrix_dy {
  fn dy(self, this: &mut QMatrix) -> i32;
}

// proto: double QMatrix::dy();
impl<'a> /*trait*/ QMatrix_dy for () {
  fn dy(self, this: &mut QMatrix) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix2dyEv()};
    unsafe {_ZNK7QMatrix2dyEv()};
    return 1;
  }
}

impl /*struct*/ QMatrix {
  pub fn scale<T: QMatrix_scale>(&mut self, value: T) -> i32 {
    value.scale(self);
    return 1;
  }
}

pub trait QMatrix_scale {
  fn scale(self, this: &mut QMatrix) -> i32;
}

// proto: QMatrix & QMatrix::scale(qreal sx, qreal sy);
impl<'a> /*trait*/ QMatrix_scale for (f64, f64) {
  fn scale(self, this: &mut QMatrix) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN7QMatrix5scaleEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN7QMatrix5scaleEdd(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QMatrix {
  pub fn translate<T: QMatrix_translate>(&mut self, value: T) -> i32 {
    value.translate(self);
    return 1;
  }
}

pub trait QMatrix_translate {
  fn translate(self, this: &mut QMatrix) -> i32;
}

// proto: QMatrix & QMatrix::translate(qreal dx, qreal dy);
impl<'a> /*trait*/ QMatrix_translate for (f64, f64) {
  fn translate(self, this: &mut QMatrix) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN7QMatrix9translateEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN7QMatrix9translateEdd(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QMatrix {
  pub fn determinant<T: QMatrix_determinant>(&mut self, value: T) -> i32 {
    value.determinant(self);
    return 1;
  }
}

pub trait QMatrix_determinant {
  fn determinant(self, this: &mut QMatrix) -> i32;
}

// proto: double QMatrix::determinant();
impl<'a> /*trait*/ QMatrix_determinant for () {
  fn determinant(self, this: &mut QMatrix) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix11determinantEv()};
    unsafe {_ZNK7QMatrix11determinantEv()};
    return 1;
  }
}

impl /*struct*/ QMatrix {
  pub fn shear<T: QMatrix_shear>(&mut self, value: T) -> i32 {
    value.shear(self);
    return 1;
  }
}

pub trait QMatrix_shear {
  fn shear(self, this: &mut QMatrix) -> i32;
}

// proto: QMatrix & QMatrix::shear(qreal sh, qreal sv);
impl<'a> /*trait*/ QMatrix_shear for (f64, f64) {
  fn shear(self, this: &mut QMatrix) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN7QMatrix5shearEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN7QMatrix5shearEdd(arg0, arg1)};
    return 1;
  }
}

// proto: void QMatrix::NewQMatrix();
impl<'a> /*trait*/ QMatrix_NewQMatrix for () {
  fn NewQMatrix(self) -> QMatrix {
    let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN7QMatrixC1Ev()};
    unsafe {_ZN7QMatrixC1Ev(qthis)};
    let rsthis = QMatrix{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMatrix {
  pub fn m21<T: QMatrix_m21>(&mut self, value: T) -> i32 {
    value.m21(self);
    return 1;
  }
}

pub trait QMatrix_m21 {
  fn m21(self, this: &mut QMatrix) -> i32;
}

// proto: double QMatrix::m21();
impl<'a> /*trait*/ QMatrix_m21 for () {
  fn m21(self, this: &mut QMatrix) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix3m21Ev()};
    unsafe {_ZNK7QMatrix3m21Ev()};
    return 1;
  }
}

impl /*struct*/ QMatrix {
  pub fn map<T: QMatrix_map>(&mut self, value: T) -> i32 {
    value.map(self);
    return 1;
  }
}

pub trait QMatrix_map {
  fn map(self, this: &mut QMatrix) -> i32;
}

// proto: QPointF QMatrix::map(const QPointF & p);
impl<'a> /*trait*/ QMatrix_map for (&'a  QPointF) {
  fn map(self, this: &mut QMatrix) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix3mapERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QMatrix3mapERK7QPointF(arg0)};
    return 1;
  }
}

// proto: QPolygonF QMatrix::map(const QPolygonF & a);
impl<'a> /*trait*/ QMatrix_map for (&'a  QPolygonF) {
  fn map(self, this: &mut QMatrix) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix3mapERK9QPolygonF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QMatrix3mapERK9QPolygonF(arg0)};
    return 1;
  }
}

// proto: void QMatrix::map(qreal x, qreal y, qreal * tx, qreal * ty);
impl<'a> /*trait*/ QMatrix_map for (f64, f64, &'a mut f64, &'a mut f64) {
  fn map(self, this: &mut QMatrix) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix3mapEddPdS0_()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as *mut c_double;
    let arg3 = self.3  as *mut c_double;
    unsafe {_ZNK7QMatrix3mapEddPdS0_(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QMatrix {
  pub fn rotate<T: QMatrix_rotate>(&mut self, value: T) -> i32 {
    value.rotate(self);
    return 1;
  }
}

pub trait QMatrix_rotate {
  fn rotate(self, this: &mut QMatrix) -> i32;
}

// proto: QMatrix & QMatrix::rotate(qreal a);
impl<'a> /*trait*/ QMatrix_rotate for (f64) {
  fn rotate(self, this: &mut QMatrix) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN7QMatrix6rotateEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN7QMatrix6rotateEd(arg0)};
    return 1;
  }
}

// proto: QRegion QMatrix::map(const QRegion & r);
impl<'a> /*trait*/ QMatrix_map for (&'a  QRegion) {
  fn map(self, this: &mut QMatrix) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix3mapERK7QRegion()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QMatrix3mapERK7QRegion(arg0)};
    return 1;
  }
}

impl /*struct*/ QMatrix {
  pub fn setMatrix<T: QMatrix_setMatrix>(&mut self, value: T) -> i32 {
    value.setMatrix(self);
    return 1;
  }
}

pub trait QMatrix_setMatrix {
  fn setMatrix(self, this: &mut QMatrix) -> i32;
}

// proto: void QMatrix::setMatrix(qreal m11, qreal m12, qreal m21, qreal m22, qreal dx, qreal dy);
impl<'a> /*trait*/ QMatrix_setMatrix for (f64, f64, f64, f64, f64, f64) {
  fn setMatrix(self, this: &mut QMatrix) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN7QMatrix9setMatrixEdddddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    let arg5 = self.5  as c_double;
    unsafe {_ZN7QMatrix9setMatrixEdddddd(arg0, arg1, arg2, arg3, arg4, arg5)};
    return 1;
  }
}

// proto: void QMatrix::NewQMatrix(const QMatrix & matrix);
impl<'a> /*trait*/ QMatrix_NewQMatrix for (&'a  QMatrix) {
  fn NewQMatrix(self) -> QMatrix {
    let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN7QMatrixC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QMatrixC1ERKS_(qthis, arg0)};
    let rsthis = QMatrix{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMatrix {
  pub fn reset<T: QMatrix_reset>(&mut self, value: T) -> i32 {
    value.reset(self);
    return 1;
  }
}

pub trait QMatrix_reset {
  fn reset(self, this: &mut QMatrix) -> i32;
}

// proto: void QMatrix::reset();
impl<'a> /*trait*/ QMatrix_reset for () {
  fn reset(self, this: &mut QMatrix) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN7QMatrix5resetEv()};
    unsafe {_ZN7QMatrix5resetEv()};
    return 1;
  }
}

// proto: QLineF QMatrix::map(const QLineF & l);
impl<'a> /*trait*/ QMatrix_map for (&'a  QLineF) {
  fn map(self, this: &mut QMatrix) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix3mapERK6QLineF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QMatrix3mapERK6QLineF(arg0)};
    return 1;
  }
}

// proto: QPainterPath QMatrix::map(const QPainterPath & p);
impl<'a> /*trait*/ QMatrix_map for (&'a  QPainterPath) {
  fn map(self, this: &mut QMatrix) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix3mapERK12QPainterPath()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QMatrix3mapERK12QPainterPath(arg0)};
    return 1;
  }
}

impl /*struct*/ QMatrix {
  pub fn m11<T: QMatrix_m11>(&mut self, value: T) -> i32 {
    value.m11(self);
    return 1;
  }
}

pub trait QMatrix_m11 {
  fn m11(self, this: &mut QMatrix) -> i32;
}

// proto: double QMatrix::m11();
impl<'a> /*trait*/ QMatrix_m11 for () {
  fn m11(self, this: &mut QMatrix) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix3m11Ev()};
    unsafe {_ZNK7QMatrix3m11Ev()};
    return 1;
  }
}

impl /*struct*/ QMatrix {
  pub fn mapToPolygon<T: QMatrix_mapToPolygon>(&mut self, value: T) -> i32 {
    value.mapToPolygon(self);
    return 1;
  }
}

pub trait QMatrix_mapToPolygon {
  fn mapToPolygon(self, this: &mut QMatrix) -> i32;
}

// proto: QPolygon QMatrix::mapToPolygon(const QRect & r);
impl<'a> /*trait*/ QMatrix_mapToPolygon for (&'a  QRect) {
  fn mapToPolygon(self, this: &mut QMatrix) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix12mapToPolygonERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QMatrix12mapToPolygonERK5QRect(arg0)};
    return 1;
  }
}

impl /*struct*/ QMatrix {
  pub fn inverted<T: QMatrix_inverted>(&mut self, value: T) -> i32 {
    value.inverted(self);
    return 1;
  }
}

pub trait QMatrix_inverted {
  fn inverted(self, this: &mut QMatrix) -> i32;
}

// proto: QMatrix QMatrix::inverted(bool * invertible);
impl<'a> /*trait*/ QMatrix_inverted for (&'a mut i8) {
  fn inverted(self, this: &mut QMatrix) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix8invertedEPb()};
    let arg0 = self  as *mut int8_t;
    unsafe {_ZNK7QMatrix8invertedEPb(arg0)};
    return 1;
  }
}

// proto: QPoint QMatrix::map(const QPoint & p);
impl<'a> /*trait*/ QMatrix_map for (&'a  QPoint) {
  fn map(self, this: &mut QMatrix) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix3mapERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QMatrix3mapERK6QPoint(arg0)};
    return 1;
  }
}

// proto: void QMatrix::map(int x, int y, int * tx, int * ty);
impl<'a> /*trait*/ QMatrix_map for (i32, i32, &'a mut i32, &'a mut i32) {
  fn map(self, this: &mut QMatrix) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix3mapEiiPiS0_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *mut c_int;
    let arg3 = self.3  as *mut c_int;
    unsafe {_ZNK7QMatrix3mapEiiPiS0_(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

// proto: QLine QMatrix::map(const QLine & l);
impl<'a> /*trait*/ QMatrix_map for (&'a  QLine) {
  fn map(self, this: &mut QMatrix) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix3mapERK5QLine()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QMatrix3mapERK5QLine(arg0)};
    return 1;
  }
}

impl /*struct*/ QMatrix {
  pub fn mapRect<T: QMatrix_mapRect>(&mut self, value: T) -> i32 {
    value.mapRect(self);
    return 1;
  }
}

pub trait QMatrix_mapRect {
  fn mapRect(self, this: &mut QMatrix) -> i32;
}

// proto: QRectF QMatrix::mapRect(const QRectF & );
impl<'a> /*trait*/ QMatrix_mapRect for (&'a  QRectF) {
  fn mapRect(self, this: &mut QMatrix) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix7mapRectERK6QRectF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QMatrix7mapRectERK6QRectF(arg0)};
    return 1;
  }
}

impl /*struct*/ QMatrix {
  pub fn isIdentity<T: QMatrix_isIdentity>(&mut self, value: T) -> i32 {
    value.isIdentity(self);
    return 1;
  }
}

pub trait QMatrix_isIdentity {
  fn isIdentity(self, this: &mut QMatrix) -> i32;
}

// proto: bool QMatrix::isIdentity();
impl<'a> /*trait*/ QMatrix_isIdentity for () {
  fn isIdentity(self, this: &mut QMatrix) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix10isIdentityEv()};
    unsafe {_ZNK7QMatrix10isIdentityEv()};
    return 1;
  }
}

// proto: void QMatrix::NewQMatrix(qreal am11, qreal am12, qreal am21, qreal am22, qreal adx, qreal ady, bool );
impl<'a> /*trait*/ QMatrix_NewQMatrix for (f64, f64, f64, f64, f64, f64, i8) {
  fn NewQMatrix(self) -> QMatrix {
    let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN7QMatrixC1Eddddddb()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    let arg5 = self.5  as c_double;
    let arg6 = self.6  as int8_t;
    unsafe {_ZN7QMatrixC1Eddddddb(qthis, arg0, arg1, arg2, arg3, arg4, arg5, arg6)};
    let rsthis = QMatrix{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMatrix {
  pub fn m12<T: QMatrix_m12>(&mut self, value: T) -> i32 {
    value.m12(self);
    return 1;
  }
}

pub trait QMatrix_m12 {
  fn m12(self, this: &mut QMatrix) -> i32;
}

// proto: double QMatrix::m12();
impl<'a> /*trait*/ QMatrix_m12 for () {
  fn m12(self, this: &mut QMatrix) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix3m12Ev()};
    unsafe {_ZNK7QMatrix3m12Ev()};
    return 1;
  }
}

impl /*struct*/ QMatrix {
  pub fn isInvertible<T: QMatrix_isInvertible>(&mut self, value: T) -> i32 {
    value.isInvertible(self);
    return 1;
  }
}

pub trait QMatrix_isInvertible {
  fn isInvertible(self, this: &mut QMatrix) -> i32;
}

// proto: bool QMatrix::isInvertible();
impl<'a> /*trait*/ QMatrix_isInvertible for () {
  fn isInvertible(self, this: &mut QMatrix) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix12isInvertibleEv()};
    unsafe {_ZNK7QMatrix12isInvertibleEv()};
    return 1;
  }
}

// proto: QRect QMatrix::mapRect(const QRect & );
impl<'a> /*trait*/ QMatrix_mapRect for (&'a  QRect) {
  fn mapRect(self, this: &mut QMatrix) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix7mapRectERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QMatrix7mapRectERK5QRect(arg0)};
    return 1;
  }
}

// proto: void QMatrix::NewQMatrix(qreal m11, qreal m12, qreal m21, qreal m22, qreal dx, qreal dy);
impl<'a> /*trait*/ QMatrix_NewQMatrix for (f64, f64, f64, f64, f64, f64) {
  fn NewQMatrix(self) -> QMatrix {
    let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN7QMatrixC1Edddddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    let arg5 = self.5  as c_double;
    unsafe {_ZN7QMatrixC1Edddddd(qthis, arg0, arg1, arg2, arg3, arg4, arg5)};
    let rsthis = QMatrix{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMatrix {
  pub fn m22<T: QMatrix_m22>(&mut self, value: T) -> i32 {
    value.m22(self);
    return 1;
  }
}

pub trait QMatrix_m22 {
  fn m22(self, this: &mut QMatrix) -> i32;
}

// proto: double QMatrix::m22();
impl<'a> /*trait*/ QMatrix_m22 for () {
  fn m22(self, this: &mut QMatrix) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix3m22Ev()};
    unsafe {_ZNK7QMatrix3m22Ev()};
    return 1;
  }
}

// proto: QPolygon QMatrix::map(const QPolygon & a);
impl<'a> /*trait*/ QMatrix_map for (&'a  QPolygon) {
  fn map(self, this: &mut QMatrix) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix3mapERK8QPolygon()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QMatrix3mapERK8QPolygon(arg0)};
    return 1;
  }
}

