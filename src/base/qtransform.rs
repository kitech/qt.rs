// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpoint::QPoint;
use super::qpainterpath::QPainterPath;
use super::qrect::QRect;
use super::qpolygon::QPolygon;
use super::qlinef::QLineF;
use super::qrectf::QRectF;
use super::qline::QLine;
use super::qpolygonf::QPolygonF;
use super::qpointf::QPointF;
use super::qmatrix::QMatrix;
use super::qregion::QRegion;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QPoint QTransform::map(const QPoint & p);
  fn _ZNK10QTransform3mapERK6QPoint(arg0: *const c_void) -> i32;
  // proto: QPainterPath QTransform::map(const QPainterPath & p);
  fn _ZNK10QTransform3mapERK12QPainterPath(arg0: *const c_void) -> i32;
  // proto: double QTransform::det();
  fn _ZNK10QTransform3detEv() -> i32;
  // proto: void QTransform::map(qreal x, qreal y, qreal * tx, qreal * ty);
  fn _ZNK10QTransform3mapEddPdS0_(arg0: c_double, arg1: c_double, arg2: *mut c_double, arg3: *mut c_double) -> i32;
  // proto: void QTransform::setMatrix(qreal m11, qreal m12, qreal m13, qreal m21, qreal m22, qreal m23, qreal m31, qreal m32, qreal m33);
  fn _ZN10QTransform9setMatrixEddddddddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double, arg5: c_double, arg6: c_double, arg7: c_double, arg8: c_double) -> i32;
  // proto: const QMatrix & QTransform::toAffine();
  fn _ZNK10QTransform8toAffineEv() -> i32;
  // proto: void QTransform::reset();
  fn _ZN10QTransform5resetEv() -> i32;
  // proto: double QTransform::determinant();
  fn _ZNK10QTransform11determinantEv() -> i32;
  // proto: QTransform QTransform::fromScale(qreal dx, qreal dy);
  fn _ZN10QTransform9fromScaleEdd(arg0: c_double, arg1: c_double) -> i32;
  // proto: bool QTransform::isTranslating();
  fn _ZNK10QTransform13isTranslatingEv() -> i32;
  // proto: QPolygon QTransform::mapToPolygon(const QRect & r);
  fn _ZNK10QTransform12mapToPolygonERK5QRect(arg0: *const c_void) -> i32;
  // proto: double QTransform::m22();
  fn _ZNK10QTransform3m22Ev() -> i32;
  // proto: QRect QTransform::mapRect(const QRect & );
  fn _ZNK10QTransform7mapRectERK5QRect(arg0: *const c_void) -> i32;
  // proto: void QTransform::NewQTransform();
  fn _ZN10QTransformC1Ev(qthis: *mut c_void) -> i32;
  // proto: double QTransform::m32();
  fn _ZNK10QTransform3m32Ev() -> i32;
  // proto: void QTransform::map(int x, int y, int * tx, int * ty);
  fn _ZNK10QTransform3mapEiiPiS0_(arg0: c_int, arg1: c_int, arg2: *mut c_int, arg3: *mut c_int) -> i32;
  // proto: QTransform & QTransform::shear(qreal sh, qreal sv);
  fn _ZN10QTransform5shearEdd(arg0: c_double, arg1: c_double) -> i32;
  // proto: void QTransform::NewQTransform(qreal h11, qreal h12, qreal h21, qreal h22, qreal dx, qreal dy);
  fn _ZN10QTransformC1Edddddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double, arg5: c_double) -> i32;
  // proto: QTransform & QTransform::scale(qreal sx, qreal sy);
  fn _ZN10QTransform5scaleEdd(arg0: c_double, arg1: c_double) -> i32;
  // proto: QPolygon QTransform::map(const QPolygon & a);
  fn _ZNK10QTransform3mapERK8QPolygon(arg0: *const c_void) -> i32;
  // proto: QTransform QTransform::transposed();
  fn _ZNK10QTransform10transposedEv() -> i32;
  // proto: QLineF QTransform::map(const QLineF & l);
  fn _ZNK10QTransform3mapERK6QLineF(arg0: *const c_void) -> i32;
  // proto: void QTransform::NewQTransform(bool );
  fn _ZN10QTransformC1Eb(qthis: *mut c_void, arg0: int8_t) -> i32;
  // proto: QTransform & QTransform::translate(qreal dx, qreal dy);
  fn _ZN10QTransform9translateEdd(arg0: c_double, arg1: c_double) -> i32;
  // proto: QRectF QTransform::mapRect(const QRectF & );
  fn _ZNK10QTransform7mapRectERK6QRectF(arg0: *const c_void) -> i32;
  // proto: QTransform QTransform::fromTranslate(qreal dx, qreal dy);
  fn _ZN10QTransform13fromTranslateEdd(arg0: c_double, arg1: c_double) -> i32;
  // proto: QLine QTransform::map(const QLine & l);
  fn _ZNK10QTransform3mapERK5QLine(arg0: *const c_void) -> i32;
  // proto: bool QTransform::isInvertible();
  fn _ZNK10QTransform12isInvertibleEv() -> i32;
  // proto: bool QTransform::quadToQuad(const QPolygonF & one, const QPolygonF & two, QTransform & result);
  fn _ZN10QTransform10quadToQuadERK9QPolygonFS2_RS_(arg0: *const c_void, arg1: *const c_void, arg2: *mut c_void) -> i32;
  // proto: bool QTransform::squareToQuad(const QPolygonF & square, QTransform & result);
  fn _ZN10QTransform12squareToQuadERK9QPolygonFRS_(arg0: *const c_void, arg1: *mut c_void) -> i32;
  // proto: QPointF QTransform::map(const QPointF & p);
  fn _ZNK10QTransform3mapERK7QPointF(arg0: *const c_void) -> i32;
  // proto: QPolygonF QTransform::map(const QPolygonF & a);
  fn _ZNK10QTransform3mapERK9QPolygonF(arg0: *const c_void) -> i32;
  // proto: double QTransform::m31();
  fn _ZNK10QTransform3m31Ev() -> i32;
  // proto: void QTransform::NewQTransform(const QMatrix & mtx);
  fn _ZN10QTransformC1ERK7QMatrix(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QTransform::NewQTransform(qreal h11, qreal h12, qreal h13, qreal h21, qreal h22, qreal h23, qreal h31, qreal h32, qreal h33);
  fn _ZN10QTransformC1Eddddddddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double, arg5: c_double, arg6: c_double, arg7: c_double, arg8: c_double) -> i32;
  // proto: QRegion QTransform::map(const QRegion & r);
  fn _ZNK10QTransform3mapERK7QRegion(arg0: *const c_void) -> i32;
  // proto: bool QTransform::isRotating();
  fn _ZNK10QTransform10isRotatingEv() -> i32;
  // proto: double QTransform::m33();
  fn _ZNK10QTransform3m33Ev() -> i32;
  // proto: double QTransform::m13();
  fn _ZNK10QTransform3m13Ev() -> i32;
  // proto: double QTransform::m21();
  fn _ZNK10QTransform3m21Ev() -> i32;
  // proto: bool QTransform::isScaling();
  fn _ZNK10QTransform9isScalingEv() -> i32;
  // proto: QTransform QTransform::inverted(bool * invertible);
  fn _ZNK10QTransform8invertedEPb(arg0: *mut int8_t) -> i32;
  // proto: bool QTransform::isAffine();
  fn _ZNK10QTransform8isAffineEv() -> i32;
  // proto: double QTransform::m11();
  fn _ZNK10QTransform3m11Ev() -> i32;
  // proto: bool QTransform::isIdentity();
  fn _ZNK10QTransform10isIdentityEv() -> i32;
  // proto: bool QTransform::quadToSquare(const QPolygonF & quad, QTransform & result);
  fn _ZN10QTransform12quadToSquareERK9QPolygonFRS_(arg0: *const c_void, arg1: *mut c_void) -> i32;
  // proto: QTransform QTransform::adjoint();
  fn _ZNK10QTransform7adjointEv() -> i32;
  // proto: double QTransform::dx();
  fn _ZNK10QTransform2dxEv() -> i32;
  // proto: double QTransform::m23();
  fn _ZNK10QTransform3m23Ev() -> i32;
  // proto: double QTransform::dy();
  fn _ZNK10QTransform2dyEv() -> i32;
  // proto: void QTransform::NewQTransform(qreal h11, qreal h12, qreal h13, qreal h21, qreal h22, qreal h23, qreal h31, qreal h32, qreal h33, bool );
  fn _ZN10QTransformC1Edddddddddb(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double, arg5: c_double, arg6: c_double, arg7: c_double, arg8: c_double, arg9: int8_t) -> i32;
  // proto: double QTransform::m12();
  fn _ZNK10QTransform3m12Ev() -> i32;
}

// body block begin
// class sizeof(QTransform)=88
pub struct QTransform {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTransform {
  pub fn map<T: QTransform_map>(&mut self, value: T) -> i32 {
    value.map(self);
    return 1;
  }
}

pub trait QTransform_map {
  fn map(self, this: &mut QTransform) -> i32;
}

// proto: QPoint QTransform::map(const QPoint & p);
impl<'a> /*trait*/ QTransform_map for (&'a  QPoint) {
  fn map(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3mapERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK10QTransform3mapERK6QPoint(arg0)};
    return 1;
  }
}

// proto: QPainterPath QTransform::map(const QPainterPath & p);
impl<'a> /*trait*/ QTransform_map for (&'a  QPainterPath) {
  fn map(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3mapERK12QPainterPath()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK10QTransform3mapERK12QPainterPath(arg0)};
    return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn det<T: QTransform_det>(&mut self, value: T) -> i32 {
    value.det(self);
    return 1;
  }
}

pub trait QTransform_det {
  fn det(self, this: &mut QTransform) -> i32;
}

// proto: double QTransform::det();
impl<'a> /*trait*/ QTransform_det for () {
  fn det(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3detEv()};
    unsafe {_ZNK10QTransform3detEv()};
    return 1;
  }
}

// proto: void QTransform::map(qreal x, qreal y, qreal * tx, qreal * ty);
impl<'a> /*trait*/ QTransform_map for (f64, f64, &'a mut f64, &'a mut f64) {
  fn map(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3mapEddPdS0_()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as *mut c_double;
    let arg3 = self.3  as *mut c_double;
    unsafe {_ZNK10QTransform3mapEddPdS0_(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn setMatrix<T: QTransform_setMatrix>(&mut self, value: T) -> i32 {
    value.setMatrix(self);
    return 1;
  }
}

pub trait QTransform_setMatrix {
  fn setMatrix(self, this: &mut QTransform) -> i32;
}

// proto: void QTransform::setMatrix(qreal m11, qreal m12, qreal m13, qreal m21, qreal m22, qreal m23, qreal m31, qreal m32, qreal m33);
impl<'a> /*trait*/ QTransform_setMatrix for (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
  fn setMatrix(self, this: &mut QTransform) -> i32 {
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
    unsafe {_ZN10QTransform9setMatrixEddddddddd(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)};
    return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn toAffine<T: QTransform_toAffine>(&mut self, value: T) -> i32 {
    value.toAffine(self);
    return 1;
  }
}

pub trait QTransform_toAffine {
  fn toAffine(self, this: &mut QTransform) -> i32;
}

// proto: const QMatrix & QTransform::toAffine();
impl<'a> /*trait*/ QTransform_toAffine for () {
  fn toAffine(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform8toAffineEv()};
    unsafe {_ZNK10QTransform8toAffineEv()};
    return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn reset<T: QTransform_reset>(&mut self, value: T) -> i32 {
    value.reset(self);
    return 1;
  }
}

pub trait QTransform_reset {
  fn reset(self, this: &mut QTransform) -> i32;
}

// proto: void QTransform::reset();
impl<'a> /*trait*/ QTransform_reset for () {
  fn reset(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN10QTransform5resetEv()};
    unsafe {_ZN10QTransform5resetEv()};
    return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn determinant<T: QTransform_determinant>(&mut self, value: T) -> i32 {
    value.determinant(self);
    return 1;
  }
}

pub trait QTransform_determinant {
  fn determinant(self, this: &mut QTransform) -> i32;
}

// proto: double QTransform::determinant();
impl<'a> /*trait*/ QTransform_determinant for () {
  fn determinant(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform11determinantEv()};
    unsafe {_ZNK10QTransform11determinantEv()};
    return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn fromScale<T: QTransform_fromScale>(&mut self, value: T) -> i32 {
    value.fromScale(self);
    return 1;
  }
}

pub trait QTransform_fromScale {
  fn fromScale(self, this: &mut QTransform) -> i32;
}

// proto: QTransform QTransform::fromScale(qreal dx, qreal dy);
impl<'a> /*trait*/ QTransform_fromScale for (f64, f64) {
  fn fromScale(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN10QTransform9fromScaleEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN10QTransform9fromScaleEdd(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn isTranslating<T: QTransform_isTranslating>(&mut self, value: T) -> i32 {
    value.isTranslating(self);
    return 1;
  }
}

pub trait QTransform_isTranslating {
  fn isTranslating(self, this: &mut QTransform) -> i32;
}

// proto: bool QTransform::isTranslating();
impl<'a> /*trait*/ QTransform_isTranslating for () {
  fn isTranslating(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform13isTranslatingEv()};
    unsafe {_ZNK10QTransform13isTranslatingEv()};
    return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn mapToPolygon<T: QTransform_mapToPolygon>(&mut self, value: T) -> i32 {
    value.mapToPolygon(self);
    return 1;
  }
}

pub trait QTransform_mapToPolygon {
  fn mapToPolygon(self, this: &mut QTransform) -> i32;
}

// proto: QPolygon QTransform::mapToPolygon(const QRect & r);
impl<'a> /*trait*/ QTransform_mapToPolygon for (&'a  QRect) {
  fn mapToPolygon(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform12mapToPolygonERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK10QTransform12mapToPolygonERK5QRect(arg0)};
    return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn m22<T: QTransform_m22>(&mut self, value: T) -> i32 {
    value.m22(self);
    return 1;
  }
}

pub trait QTransform_m22 {
  fn m22(self, this: &mut QTransform) -> i32;
}

// proto: double QTransform::m22();
impl<'a> /*trait*/ QTransform_m22 for () {
  fn m22(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3m22Ev()};
    unsafe {_ZNK10QTransform3m22Ev()};
    return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn mapRect<T: QTransform_mapRect>(&mut self, value: T) -> i32 {
    value.mapRect(self);
    return 1;
  }
}

pub trait QTransform_mapRect {
  fn mapRect(self, this: &mut QTransform) -> i32;
}

// proto: QRect QTransform::mapRect(const QRect & );
impl<'a> /*trait*/ QTransform_mapRect for (&'a  QRect) {
  fn mapRect(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform7mapRectERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK10QTransform7mapRectERK5QRect(arg0)};
    return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn NewQTransform<T: QTransform_NewQTransform>(value: T) -> QTransform {
    let rsthis = value.NewQTransform();
    return rsthis;
    // return 1;
  }
}

pub trait QTransform_NewQTransform {
  fn NewQTransform(self) -> QTransform;
}

// proto: void QTransform::NewQTransform();
impl<'a> /*trait*/ QTransform_NewQTransform for () {
  fn NewQTransform(self) -> QTransform {
    let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN10QTransformC1Ev()};
    unsafe {_ZN10QTransformC1Ev(qthis)};
    let rsthis = QTransform{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn m32<T: QTransform_m32>(&mut self, value: T) -> i32 {
    value.m32(self);
    return 1;
  }
}

pub trait QTransform_m32 {
  fn m32(self, this: &mut QTransform) -> i32;
}

// proto: double QTransform::m32();
impl<'a> /*trait*/ QTransform_m32 for () {
  fn m32(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3m32Ev()};
    unsafe {_ZNK10QTransform3m32Ev()};
    return 1;
  }
}

// proto: void QTransform::map(int x, int y, int * tx, int * ty);
impl<'a> /*trait*/ QTransform_map for (i32, i32, &'a mut i32, &'a mut i32) {
  fn map(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3mapEiiPiS0_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *mut c_int;
    let arg3 = self.3  as *mut c_int;
    unsafe {_ZNK10QTransform3mapEiiPiS0_(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn shear<T: QTransform_shear>(&mut self, value: T) -> i32 {
    value.shear(self);
    return 1;
  }
}

pub trait QTransform_shear {
  fn shear(self, this: &mut QTransform) -> i32;
}

// proto: QTransform & QTransform::shear(qreal sh, qreal sv);
impl<'a> /*trait*/ QTransform_shear for (f64, f64) {
  fn shear(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN10QTransform5shearEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN10QTransform5shearEdd(arg0, arg1)};
    return 1;
  }
}

// proto: void QTransform::NewQTransform(qreal h11, qreal h12, qreal h21, qreal h22, qreal dx, qreal dy);
impl<'a> /*trait*/ QTransform_NewQTransform for (f64, f64, f64, f64, f64, f64) {
  fn NewQTransform(self) -> QTransform {
    let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN10QTransformC1Edddddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    let arg5 = self.5  as c_double;
    unsafe {_ZN10QTransformC1Edddddd(qthis, arg0, arg1, arg2, arg3, arg4, arg5)};
    let rsthis = QTransform{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn scale<T: QTransform_scale>(&mut self, value: T) -> i32 {
    value.scale(self);
    return 1;
  }
}

pub trait QTransform_scale {
  fn scale(self, this: &mut QTransform) -> i32;
}

// proto: QTransform & QTransform::scale(qreal sx, qreal sy);
impl<'a> /*trait*/ QTransform_scale for (f64, f64) {
  fn scale(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN10QTransform5scaleEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN10QTransform5scaleEdd(arg0, arg1)};
    return 1;
  }
}

// proto: QPolygon QTransform::map(const QPolygon & a);
impl<'a> /*trait*/ QTransform_map for (&'a  QPolygon) {
  fn map(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3mapERK8QPolygon()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK10QTransform3mapERK8QPolygon(arg0)};
    return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn transposed<T: QTransform_transposed>(&mut self, value: T) -> i32 {
    value.transposed(self);
    return 1;
  }
}

pub trait QTransform_transposed {
  fn transposed(self, this: &mut QTransform) -> i32;
}

// proto: QTransform QTransform::transposed();
impl<'a> /*trait*/ QTransform_transposed for () {
  fn transposed(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform10transposedEv()};
    unsafe {_ZNK10QTransform10transposedEv()};
    return 1;
  }
}

// proto: QLineF QTransform::map(const QLineF & l);
impl<'a> /*trait*/ QTransform_map for (&'a  QLineF) {
  fn map(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3mapERK6QLineF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK10QTransform3mapERK6QLineF(arg0)};
    return 1;
  }
}

// proto: void QTransform::NewQTransform(bool );
impl<'a> /*trait*/ QTransform_NewQTransform for (i8) {
  fn NewQTransform(self) -> QTransform {
    let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN10QTransformC1Eb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN10QTransformC1Eb(qthis, arg0)};
    let rsthis = QTransform{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn translate<T: QTransform_translate>(&mut self, value: T) -> i32 {
    value.translate(self);
    return 1;
  }
}

pub trait QTransform_translate {
  fn translate(self, this: &mut QTransform) -> i32;
}

// proto: QTransform & QTransform::translate(qreal dx, qreal dy);
impl<'a> /*trait*/ QTransform_translate for (f64, f64) {
  fn translate(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN10QTransform9translateEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN10QTransform9translateEdd(arg0, arg1)};
    return 1;
  }
}

// proto: QRectF QTransform::mapRect(const QRectF & );
impl<'a> /*trait*/ QTransform_mapRect for (&'a  QRectF) {
  fn mapRect(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform7mapRectERK6QRectF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK10QTransform7mapRectERK6QRectF(arg0)};
    return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn fromTranslate<T: QTransform_fromTranslate>(&mut self, value: T) -> i32 {
    value.fromTranslate(self);
    return 1;
  }
}

pub trait QTransform_fromTranslate {
  fn fromTranslate(self, this: &mut QTransform) -> i32;
}

// proto: QTransform QTransform::fromTranslate(qreal dx, qreal dy);
impl<'a> /*trait*/ QTransform_fromTranslate for (f64, f64) {
  fn fromTranslate(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN10QTransform13fromTranslateEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN10QTransform13fromTranslateEdd(arg0, arg1)};
    return 1;
  }
}

// proto: QLine QTransform::map(const QLine & l);
impl<'a> /*trait*/ QTransform_map for (&'a  QLine) {
  fn map(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3mapERK5QLine()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK10QTransform3mapERK5QLine(arg0)};
    return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn isInvertible<T: QTransform_isInvertible>(&mut self, value: T) -> i32 {
    value.isInvertible(self);
    return 1;
  }
}

pub trait QTransform_isInvertible {
  fn isInvertible(self, this: &mut QTransform) -> i32;
}

// proto: bool QTransform::isInvertible();
impl<'a> /*trait*/ QTransform_isInvertible for () {
  fn isInvertible(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform12isInvertibleEv()};
    unsafe {_ZNK10QTransform12isInvertibleEv()};
    return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn quadToQuad<T: QTransform_quadToQuad>(&mut self, value: T) -> i32 {
    value.quadToQuad(self);
    return 1;
  }
}

pub trait QTransform_quadToQuad {
  fn quadToQuad(self, this: &mut QTransform) -> i32;
}

// proto: bool QTransform::quadToQuad(const QPolygonF & one, const QPolygonF & two, QTransform & result);
impl<'a> /*trait*/ QTransform_quadToQuad for (&'a  QPolygonF, &'a  QPolygonF, &'a mut QTransform) {
  fn quadToQuad(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN10QTransform10quadToQuadERK9QPolygonFS2_RS_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN10QTransform10quadToQuadERK9QPolygonFS2_RS_(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn squareToQuad<T: QTransform_squareToQuad>(&mut self, value: T) -> i32 {
    value.squareToQuad(self);
    return 1;
  }
}

pub trait QTransform_squareToQuad {
  fn squareToQuad(self, this: &mut QTransform) -> i32;
}

// proto: bool QTransform::squareToQuad(const QPolygonF & square, QTransform & result);
impl<'a> /*trait*/ QTransform_squareToQuad for (&'a  QPolygonF, &'a mut QTransform) {
  fn squareToQuad(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN10QTransform12squareToQuadERK9QPolygonFRS_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN10QTransform12squareToQuadERK9QPolygonFRS_(arg0, arg1)};
    return 1;
  }
}

// proto: QPointF QTransform::map(const QPointF & p);
impl<'a> /*trait*/ QTransform_map for (&'a  QPointF) {
  fn map(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3mapERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK10QTransform3mapERK7QPointF(arg0)};
    return 1;
  }
}

// proto: QPolygonF QTransform::map(const QPolygonF & a);
impl<'a> /*trait*/ QTransform_map for (&'a  QPolygonF) {
  fn map(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3mapERK9QPolygonF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK10QTransform3mapERK9QPolygonF(arg0)};
    return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn m31<T: QTransform_m31>(&mut self, value: T) -> i32 {
    value.m31(self);
    return 1;
  }
}

pub trait QTransform_m31 {
  fn m31(self, this: &mut QTransform) -> i32;
}

// proto: double QTransform::m31();
impl<'a> /*trait*/ QTransform_m31 for () {
  fn m31(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3m31Ev()};
    unsafe {_ZNK10QTransform3m31Ev()};
    return 1;
  }
}

// proto: void QTransform::NewQTransform(const QMatrix & mtx);
impl<'a> /*trait*/ QTransform_NewQTransform for (&'a  QMatrix) {
  fn NewQTransform(self) -> QTransform {
    let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN10QTransformC1ERK7QMatrix()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QTransformC1ERK7QMatrix(qthis, arg0)};
    let rsthis = QTransform{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QTransform::NewQTransform(qreal h11, qreal h12, qreal h13, qreal h21, qreal h22, qreal h23, qreal h31, qreal h32, qreal h33);
impl<'a> /*trait*/ QTransform_NewQTransform for (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
  fn NewQTransform(self) -> QTransform {
    let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN10QTransformC1Eddddddddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    let arg5 = self.5  as c_double;
    let arg6 = self.6  as c_double;
    let arg7 = self.7  as c_double;
    let arg8 = self.8  as c_double;
    unsafe {_ZN10QTransformC1Eddddddddd(qthis, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)};
    let rsthis = QTransform{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: QRegion QTransform::map(const QRegion & r);
impl<'a> /*trait*/ QTransform_map for (&'a  QRegion) {
  fn map(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3mapERK7QRegion()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK10QTransform3mapERK7QRegion(arg0)};
    return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn isRotating<T: QTransform_isRotating>(&mut self, value: T) -> i32 {
    value.isRotating(self);
    return 1;
  }
}

pub trait QTransform_isRotating {
  fn isRotating(self, this: &mut QTransform) -> i32;
}

// proto: bool QTransform::isRotating();
impl<'a> /*trait*/ QTransform_isRotating for () {
  fn isRotating(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform10isRotatingEv()};
    unsafe {_ZNK10QTransform10isRotatingEv()};
    return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn m33<T: QTransform_m33>(&mut self, value: T) -> i32 {
    value.m33(self);
    return 1;
  }
}

pub trait QTransform_m33 {
  fn m33(self, this: &mut QTransform) -> i32;
}

// proto: double QTransform::m33();
impl<'a> /*trait*/ QTransform_m33 for () {
  fn m33(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3m33Ev()};
    unsafe {_ZNK10QTransform3m33Ev()};
    return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn m13<T: QTransform_m13>(&mut self, value: T) -> i32 {
    value.m13(self);
    return 1;
  }
}

pub trait QTransform_m13 {
  fn m13(self, this: &mut QTransform) -> i32;
}

// proto: double QTransform::m13();
impl<'a> /*trait*/ QTransform_m13 for () {
  fn m13(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3m13Ev()};
    unsafe {_ZNK10QTransform3m13Ev()};
    return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn m21<T: QTransform_m21>(&mut self, value: T) -> i32 {
    value.m21(self);
    return 1;
  }
}

pub trait QTransform_m21 {
  fn m21(self, this: &mut QTransform) -> i32;
}

// proto: double QTransform::m21();
impl<'a> /*trait*/ QTransform_m21 for () {
  fn m21(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3m21Ev()};
    unsafe {_ZNK10QTransform3m21Ev()};
    return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn isScaling<T: QTransform_isScaling>(&mut self, value: T) -> i32 {
    value.isScaling(self);
    return 1;
  }
}

pub trait QTransform_isScaling {
  fn isScaling(self, this: &mut QTransform) -> i32;
}

// proto: bool QTransform::isScaling();
impl<'a> /*trait*/ QTransform_isScaling for () {
  fn isScaling(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform9isScalingEv()};
    unsafe {_ZNK10QTransform9isScalingEv()};
    return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn inverted<T: QTransform_inverted>(&mut self, value: T) -> i32 {
    value.inverted(self);
    return 1;
  }
}

pub trait QTransform_inverted {
  fn inverted(self, this: &mut QTransform) -> i32;
}

// proto: QTransform QTransform::inverted(bool * invertible);
impl<'a> /*trait*/ QTransform_inverted for (&'a mut i8) {
  fn inverted(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform8invertedEPb()};
    let arg0 = self  as *mut int8_t;
    unsafe {_ZNK10QTransform8invertedEPb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn isAffine<T: QTransform_isAffine>(&mut self, value: T) -> i32 {
    value.isAffine(self);
    return 1;
  }
}

pub trait QTransform_isAffine {
  fn isAffine(self, this: &mut QTransform) -> i32;
}

// proto: bool QTransform::isAffine();
impl<'a> /*trait*/ QTransform_isAffine for () {
  fn isAffine(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform8isAffineEv()};
    unsafe {_ZNK10QTransform8isAffineEv()};
    return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn m11<T: QTransform_m11>(&mut self, value: T) -> i32 {
    value.m11(self);
    return 1;
  }
}

pub trait QTransform_m11 {
  fn m11(self, this: &mut QTransform) -> i32;
}

// proto: double QTransform::m11();
impl<'a> /*trait*/ QTransform_m11 for () {
  fn m11(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3m11Ev()};
    unsafe {_ZNK10QTransform3m11Ev()};
    return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn isIdentity<T: QTransform_isIdentity>(&mut self, value: T) -> i32 {
    value.isIdentity(self);
    return 1;
  }
}

pub trait QTransform_isIdentity {
  fn isIdentity(self, this: &mut QTransform) -> i32;
}

// proto: bool QTransform::isIdentity();
impl<'a> /*trait*/ QTransform_isIdentity for () {
  fn isIdentity(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform10isIdentityEv()};
    unsafe {_ZNK10QTransform10isIdentityEv()};
    return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn quadToSquare<T: QTransform_quadToSquare>(&mut self, value: T) -> i32 {
    value.quadToSquare(self);
    return 1;
  }
}

pub trait QTransform_quadToSquare {
  fn quadToSquare(self, this: &mut QTransform) -> i32;
}

// proto: bool QTransform::quadToSquare(const QPolygonF & quad, QTransform & result);
impl<'a> /*trait*/ QTransform_quadToSquare for (&'a  QPolygonF, &'a mut QTransform) {
  fn quadToSquare(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN10QTransform12quadToSquareERK9QPolygonFRS_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN10QTransform12quadToSquareERK9QPolygonFRS_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn adjoint<T: QTransform_adjoint>(&mut self, value: T) -> i32 {
    value.adjoint(self);
    return 1;
  }
}

pub trait QTransform_adjoint {
  fn adjoint(self, this: &mut QTransform) -> i32;
}

// proto: QTransform QTransform::adjoint();
impl<'a> /*trait*/ QTransform_adjoint for () {
  fn adjoint(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform7adjointEv()};
    unsafe {_ZNK10QTransform7adjointEv()};
    return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn dx<T: QTransform_dx>(&mut self, value: T) -> i32 {
    value.dx(self);
    return 1;
  }
}

pub trait QTransform_dx {
  fn dx(self, this: &mut QTransform) -> i32;
}

// proto: double QTransform::dx();
impl<'a> /*trait*/ QTransform_dx for () {
  fn dx(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform2dxEv()};
    unsafe {_ZNK10QTransform2dxEv()};
    return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn m23<T: QTransform_m23>(&mut self, value: T) -> i32 {
    value.m23(self);
    return 1;
  }
}

pub trait QTransform_m23 {
  fn m23(self, this: &mut QTransform) -> i32;
}

// proto: double QTransform::m23();
impl<'a> /*trait*/ QTransform_m23 for () {
  fn m23(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3m23Ev()};
    unsafe {_ZNK10QTransform3m23Ev()};
    return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn dy<T: QTransform_dy>(&mut self, value: T) -> i32 {
    value.dy(self);
    return 1;
  }
}

pub trait QTransform_dy {
  fn dy(self, this: &mut QTransform) -> i32;
}

// proto: double QTransform::dy();
impl<'a> /*trait*/ QTransform_dy for () {
  fn dy(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform2dyEv()};
    unsafe {_ZNK10QTransform2dyEv()};
    return 1;
  }
}

// proto: void QTransform::NewQTransform(qreal h11, qreal h12, qreal h13, qreal h21, qreal h22, qreal h23, qreal h31, qreal h32, qreal h33, bool );
impl<'a> /*trait*/ QTransform_NewQTransform for (f64, f64, f64, f64, f64, f64, f64, f64, f64, i8) {
  fn NewQTransform(self) -> QTransform {
    let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN10QTransformC1Edddddddddb()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    let arg5 = self.5  as c_double;
    let arg6 = self.6  as c_double;
    let arg7 = self.7  as c_double;
    let arg8 = self.8  as c_double;
    let arg9 = self.9  as int8_t;
    unsafe {_ZN10QTransformC1Edddddddddb(qthis, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)};
    let rsthis = QTransform{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn m12<T: QTransform_m12>(&mut self, value: T) -> i32 {
    value.m12(self);
    return 1;
  }
}

pub trait QTransform_m12 {
  fn m12(self, this: &mut QTransform) -> i32;
}

// proto: double QTransform::m12();
impl<'a> /*trait*/ QTransform_m12 for () {
  fn m12(self, this: &mut QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3m12Ev()};
    unsafe {_ZNK10QTransform3m12Ev()};
    return 1;
  }
}

