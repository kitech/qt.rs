// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qmatrix::QMatrix;
use super::qrect::QRect;
use super::qpolygon::QPolygon;
use super::qpolygonf::QPolygonF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  double QTransform::det();
  fn _ZNK10QTransform3detEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTransform::setMatrix(qreal m11, qreal m12, qreal m13, qreal m21, qreal m22, qreal m23, qreal m31, qreal m32, qreal m33);
  fn _ZN10QTransform9setMatrixEddddddddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double, arg5: c_double, arg6: c_double, arg7: c_double, arg8: c_double) ;
  // proto:  const QMatrix & QTransform::toAffine();
  fn _ZNK10QTransform8toAffineEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTransform::reset();
  fn _ZN10QTransform5resetEv(qthis: *mut c_void) ;
  // proto:  double QTransform::determinant();
  fn _ZNK10QTransform11determinantEv(qthis: *mut c_void) -> c_double;
  // proto: static QTransform QTransform::fromScale(qreal dx, qreal dy);
  fn _ZN10QTransform9fromScaleEdd(arg0: c_double, arg1: c_double) -> *mut c_void;
  // proto:  bool QTransform::isTranslating();
  fn _ZNK10QTransform13isTranslatingEv(qthis: *mut c_void) -> int8_t;
  // proto:  QPolygon QTransform::mapToPolygon(const QRect & r);
  fn _ZNK10QTransform12mapToPolygonERK5QRect(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  double QTransform::m22();
  fn _ZNK10QTransform3m22Ev(qthis: *mut c_void) -> c_double;
  // proto:  void QTransform::NewQTransform();
  fn _ZN10QTransformC1Ev(qthis: *mut c_void) ;
  // proto:  double QTransform::m32();
  fn _ZNK10QTransform3m32Ev(qthis: *mut c_void) -> c_double;
  // proto:  QTransform & QTransform::shear(qreal sh, qreal sv);
  fn _ZN10QTransform5shearEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) -> *mut c_void;
  // proto:  void QTransform::NewQTransform(qreal h11, qreal h12, qreal h21, qreal h22, qreal dx, qreal dy);
  fn _ZN10QTransformC1Edddddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double, arg5: c_double) ;
  // proto:  QTransform & QTransform::scale(qreal sx, qreal sy);
  fn _ZN10QTransform5scaleEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) -> *mut c_void;
  // proto:  QTransform QTransform::transposed();
  fn _ZNK10QTransform10transposedEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTransform::NewQTransform(bool );
  fn _ZN10QTransformC1Eb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QTransform & QTransform::translate(qreal dx, qreal dy);
  fn _ZN10QTransform9translateEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) -> *mut c_void;
  // proto: static QTransform QTransform::fromTranslate(qreal dx, qreal dy);
  fn _ZN10QTransform13fromTranslateEdd(arg0: c_double, arg1: c_double) -> *mut c_void;
  // proto:  bool QTransform::isInvertible();
  fn _ZNK10QTransform12isInvertibleEv(qthis: *mut c_void) -> int8_t;
  // proto: static bool QTransform::quadToQuad(const QPolygonF & one, const QPolygonF & two, QTransform & result);
  fn _ZN10QTransform10quadToQuadERK9QPolygonFS2_RS_(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> int8_t;
  // proto: static bool QTransform::squareToQuad(const QPolygonF & square, QTransform & result);
  fn _ZN10QTransform12squareToQuadERK9QPolygonFRS_(arg0: *mut c_void, arg1: *mut c_void) -> int8_t;
  // proto:  double QTransform::m31();
  fn _ZNK10QTransform3m31Ev(qthis: *mut c_void) -> c_double;
  // proto:  void QTransform::NewQTransform(const QMatrix & mtx);
  fn _ZN10QTransformC1ERK7QMatrix(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTransform::NewQTransform(qreal h11, qreal h12, qreal h13, qreal h21, qreal h22, qreal h23, qreal h31, qreal h32, qreal h33);
  fn _ZN10QTransformC1Eddddddddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double, arg5: c_double, arg6: c_double, arg7: c_double, arg8: c_double) ;
  // proto:  bool QTransform::isRotating();
  fn _ZNK10QTransform10isRotatingEv(qthis: *mut c_void) -> int8_t;
  // proto:  double QTransform::m33();
  fn _ZNK10QTransform3m33Ev(qthis: *mut c_void) -> c_double;
  // proto:  double QTransform::m13();
  fn _ZNK10QTransform3m13Ev(qthis: *mut c_void) -> c_double;
  // proto:  double QTransform::m21();
  fn _ZNK10QTransform3m21Ev(qthis: *mut c_void) -> c_double;
  // proto:  bool QTransform::isScaling();
  fn _ZNK10QTransform9isScalingEv(qthis: *mut c_void) -> int8_t;
  // proto:  QTransform QTransform::inverted(bool * invertible);
  fn _ZNK10QTransform8invertedEPb(qthis: *mut c_void, arg0: *mut int8_t) -> *mut c_void;
  // proto:  bool QTransform::isAffine();
  fn _ZNK10QTransform8isAffineEv(qthis: *mut c_void) -> int8_t;
  // proto:  double QTransform::m11();
  fn _ZNK10QTransform3m11Ev(qthis: *mut c_void) -> c_double;
  // proto:  bool QTransform::isIdentity();
  fn _ZNK10QTransform10isIdentityEv(qthis: *mut c_void) -> int8_t;
  // proto: static bool QTransform::quadToSquare(const QPolygonF & quad, QTransform & result);
  fn _ZN10QTransform12quadToSquareERK9QPolygonFRS_(arg0: *mut c_void, arg1: *mut c_void) -> int8_t;
  // proto:  QTransform QTransform::adjoint();
  fn _ZNK10QTransform7adjointEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  double QTransform::dx();
  fn _ZNK10QTransform2dxEv(qthis: *mut c_void) -> c_double;
  // proto:  double QTransform::m23();
  fn _ZNK10QTransform3m23Ev(qthis: *mut c_void) -> c_double;
  // proto:  double QTransform::dy();
  fn _ZNK10QTransform2dyEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTransform::NewQTransform(qreal h11, qreal h12, qreal h13, qreal h21, qreal h22, qreal h23, qreal h31, qreal h32, qreal h33, bool );
  fn _ZN10QTransformC1Edddddddddb(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double, arg5: c_double, arg6: c_double, arg7: c_double, arg8: c_double, arg9: int8_t) ;
  // proto:  double QTransform::m12();
  fn _ZNK10QTransform3m12Ev(qthis: *mut c_void) -> c_double;
}

// body block begin
// class sizeof(QTransform)=88
pub struct QTransform {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTransform {
  pub fn det<T: QTransform_det>(&mut self, value: T) -> f64 {
    return value.det(self);
    // return 1;
  }
}

pub trait QTransform_det {
  fn det(self, rsthis: &mut QTransform) -> f64;
}

// proto:  double QTransform::det();
impl<'a> /*trait*/ QTransform_det for () {
  fn det(self, rsthis: &mut QTransform) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3detEv()};
    let mut ret = unsafe {_ZNK10QTransform3detEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn setMatrix<T: QTransform_setMatrix>(&mut self, value: T)  {
     value.setMatrix(self);
    // return 1;
  }
}

pub trait QTransform_setMatrix {
  fn setMatrix(self, rsthis: &mut QTransform) ;
}

// proto:  void QTransform::setMatrix(qreal m11, qreal m12, qreal m13, qreal m21, qreal m22, qreal m23, qreal m31, qreal m32, qreal m33);
impl<'a> /*trait*/ QTransform_setMatrix for (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
  fn setMatrix(self, rsthis: &mut QTransform)  {
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

impl /*struct*/ QTransform {
  pub fn toAffine<T: QTransform_toAffine>(&mut self, value: T) -> QMatrix {
    return value.toAffine(self);
    // return 1;
  }
}

pub trait QTransform_toAffine {
  fn toAffine(self, rsthis: &mut QTransform) -> QMatrix;
}

// proto:  const QMatrix & QTransform::toAffine();
impl<'a> /*trait*/ QTransform_toAffine for () {
  fn toAffine(self, rsthis: &mut QTransform) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform8toAffineEv()};
    let mut ret = unsafe {_ZNK10QTransform8toAffineEv(rsthis.qclsinst)};
    let mut ret1 = QMatrix{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn reset<T: QTransform_reset>(&mut self, value: T)  {
     value.reset(self);
    // return 1;
  }
}

pub trait QTransform_reset {
  fn reset(self, rsthis: &mut QTransform) ;
}

// proto:  void QTransform::reset();
impl<'a> /*trait*/ QTransform_reset for () {
  fn reset(self, rsthis: &mut QTransform)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN10QTransform5resetEv()};
     unsafe {_ZN10QTransform5resetEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn determinant<T: QTransform_determinant>(&mut self, value: T) -> f64 {
    return value.determinant(self);
    // return 1;
  }
}

pub trait QTransform_determinant {
  fn determinant(self, rsthis: &mut QTransform) -> f64;
}

// proto:  double QTransform::determinant();
impl<'a> /*trait*/ QTransform_determinant for () {
  fn determinant(self, rsthis: &mut QTransform) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform11determinantEv()};
    let mut ret = unsafe {_ZNK10QTransform11determinantEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn fromScale<T: QTransform_fromScale>(&mut self, value: T) -> QTransform {
    return value.fromScale(self);
    // return 1;
  }
}

pub trait QTransform_fromScale {
  fn fromScale(self, rsthis: &mut QTransform) -> QTransform;
}

// proto: static QTransform QTransform::fromScale(qreal dx, qreal dy);
impl<'a> /*trait*/ QTransform_fromScale for (f64, f64) {
  fn fromScale(self, rsthis: &mut QTransform) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN10QTransform9fromScaleEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {_ZN10QTransform9fromScaleEdd(arg0, arg1)};
    let mut ret1 = QTransform{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn isTranslating<T: QTransform_isTranslating>(&mut self, value: T) -> i8 {
    return value.isTranslating(self);
    // return 1;
  }
}

pub trait QTransform_isTranslating {
  fn isTranslating(self, rsthis: &mut QTransform) -> i8;
}

// proto:  bool QTransform::isTranslating();
impl<'a> /*trait*/ QTransform_isTranslating for () {
  fn isTranslating(self, rsthis: &mut QTransform) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform13isTranslatingEv()};
    let mut ret = unsafe {_ZNK10QTransform13isTranslatingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn mapToPolygon<T: QTransform_mapToPolygon>(&mut self, value: T) -> QPolygon {
    return value.mapToPolygon(self);
    // return 1;
  }
}

pub trait QTransform_mapToPolygon {
  fn mapToPolygon(self, rsthis: &mut QTransform) -> QPolygon;
}

// proto:  QPolygon QTransform::mapToPolygon(const QRect & r);
impl<'a> /*trait*/ QTransform_mapToPolygon for (&'a  QRect) {
  fn mapToPolygon(self, rsthis: &mut QTransform) -> QPolygon {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform12mapToPolygonERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QTransform12mapToPolygonERK5QRect(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygon{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn m22<T: QTransform_m22>(&mut self, value: T) -> f64 {
    return value.m22(self);
    // return 1;
  }
}

pub trait QTransform_m22 {
  fn m22(self, rsthis: &mut QTransform) -> f64;
}

// proto:  double QTransform::m22();
impl<'a> /*trait*/ QTransform_m22 for () {
  fn m22(self, rsthis: &mut QTransform) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3m22Ev()};
    let mut ret = unsafe {_ZNK10QTransform3m22Ev(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
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
  pub fn m32<T: QTransform_m32>(&mut self, value: T) -> f64 {
    return value.m32(self);
    // return 1;
  }
}

pub trait QTransform_m32 {
  fn m32(self, rsthis: &mut QTransform) -> f64;
}

// proto:  double QTransform::m32();
impl<'a> /*trait*/ QTransform_m32 for () {
  fn m32(self, rsthis: &mut QTransform) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3m32Ev()};
    let mut ret = unsafe {_ZNK10QTransform3m32Ev(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn shear<T: QTransform_shear>(&mut self, value: T) -> QTransform {
    return value.shear(self);
    // return 1;
  }
}

pub trait QTransform_shear {
  fn shear(self, rsthis: &mut QTransform) -> QTransform;
}

// proto:  QTransform & QTransform::shear(qreal sh, qreal sv);
impl<'a> /*trait*/ QTransform_shear for (f64, f64) {
  fn shear(self, rsthis: &mut QTransform) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN10QTransform5shearEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {_ZN10QTransform5shearEdd(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QTransform{qclsinst: ret};
    return ret1;
    // return 1;
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
  pub fn scale<T: QTransform_scale>(&mut self, value: T) -> QTransform {
    return value.scale(self);
    // return 1;
  }
}

pub trait QTransform_scale {
  fn scale(self, rsthis: &mut QTransform) -> QTransform;
}

// proto:  QTransform & QTransform::scale(qreal sx, qreal sy);
impl<'a> /*trait*/ QTransform_scale for (f64, f64) {
  fn scale(self, rsthis: &mut QTransform) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN10QTransform5scaleEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {_ZN10QTransform5scaleEdd(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QTransform{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn transposed<T: QTransform_transposed>(&mut self, value: T) -> QTransform {
    return value.transposed(self);
    // return 1;
  }
}

pub trait QTransform_transposed {
  fn transposed(self, rsthis: &mut QTransform) -> QTransform;
}

// proto:  QTransform QTransform::transposed();
impl<'a> /*trait*/ QTransform_transposed for () {
  fn transposed(self, rsthis: &mut QTransform) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform10transposedEv()};
    let mut ret = unsafe {_ZNK10QTransform10transposedEv(rsthis.qclsinst)};
    let mut ret1 = QTransform{qclsinst: ret};
    return ret1;
    // return 1;
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
  pub fn translate<T: QTransform_translate>(&mut self, value: T) -> QTransform {
    return value.translate(self);
    // return 1;
  }
}

pub trait QTransform_translate {
  fn translate(self, rsthis: &mut QTransform) -> QTransform;
}

// proto:  QTransform & QTransform::translate(qreal dx, qreal dy);
impl<'a> /*trait*/ QTransform_translate for (f64, f64) {
  fn translate(self, rsthis: &mut QTransform) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN10QTransform9translateEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {_ZN10QTransform9translateEdd(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QTransform{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn fromTranslate<T: QTransform_fromTranslate>(&mut self, value: T) -> QTransform {
    return value.fromTranslate(self);
    // return 1;
  }
}

pub trait QTransform_fromTranslate {
  fn fromTranslate(self, rsthis: &mut QTransform) -> QTransform;
}

// proto: static QTransform QTransform::fromTranslate(qreal dx, qreal dy);
impl<'a> /*trait*/ QTransform_fromTranslate for (f64, f64) {
  fn fromTranslate(self, rsthis: &mut QTransform) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN10QTransform13fromTranslateEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {_ZN10QTransform13fromTranslateEdd(arg0, arg1)};
    let mut ret1 = QTransform{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn isInvertible<T: QTransform_isInvertible>(&mut self, value: T) -> i8 {
    return value.isInvertible(self);
    // return 1;
  }
}

pub trait QTransform_isInvertible {
  fn isInvertible(self, rsthis: &mut QTransform) -> i8;
}

// proto:  bool QTransform::isInvertible();
impl<'a> /*trait*/ QTransform_isInvertible for () {
  fn isInvertible(self, rsthis: &mut QTransform) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform12isInvertibleEv()};
    let mut ret = unsafe {_ZNK10QTransform12isInvertibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn quadToQuad<T: QTransform_quadToQuad>(&mut self, value: T) -> i8 {
    return value.quadToQuad(self);
    // return 1;
  }
}

pub trait QTransform_quadToQuad {
  fn quadToQuad(self, rsthis: &mut QTransform) -> i8;
}

// proto: static bool QTransform::quadToQuad(const QPolygonF & one, const QPolygonF & two, QTransform & result);
impl<'a> /*trait*/ QTransform_quadToQuad for (&'a  QPolygonF, &'a  QPolygonF, &'a mut QTransform) {
  fn quadToQuad(self, rsthis: &mut QTransform) -> i8 {
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

impl /*struct*/ QTransform {
  pub fn squareToQuad<T: QTransform_squareToQuad>(&mut self, value: T) -> i8 {
    return value.squareToQuad(self);
    // return 1;
  }
}

pub trait QTransform_squareToQuad {
  fn squareToQuad(self, rsthis: &mut QTransform) -> i8;
}

// proto: static bool QTransform::squareToQuad(const QPolygonF & square, QTransform & result);
impl<'a> /*trait*/ QTransform_squareToQuad for (&'a  QPolygonF, &'a mut QTransform) {
  fn squareToQuad(self, rsthis: &mut QTransform) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN10QTransform12squareToQuadERK9QPolygonFRS_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QTransform12squareToQuadERK9QPolygonFRS_(arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn m31<T: QTransform_m31>(&mut self, value: T) -> f64 {
    return value.m31(self);
    // return 1;
  }
}

pub trait QTransform_m31 {
  fn m31(self, rsthis: &mut QTransform) -> f64;
}

// proto:  double QTransform::m31();
impl<'a> /*trait*/ QTransform_m31 for () {
  fn m31(self, rsthis: &mut QTransform) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3m31Ev()};
    let mut ret = unsafe {_ZNK10QTransform3m31Ev(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto: void QTransform::NewQTransform(const QMatrix & mtx);
impl<'a> /*trait*/ QTransform_NewQTransform for (&'a  QMatrix) {
  fn NewQTransform(self) -> QTransform {
    let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN10QTransformC1ERK7QMatrix()};
    let arg0 = self.qclsinst  as *mut c_void;
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

impl /*struct*/ QTransform {
  pub fn isRotating<T: QTransform_isRotating>(&mut self, value: T) -> i8 {
    return value.isRotating(self);
    // return 1;
  }
}

pub trait QTransform_isRotating {
  fn isRotating(self, rsthis: &mut QTransform) -> i8;
}

// proto:  bool QTransform::isRotating();
impl<'a> /*trait*/ QTransform_isRotating for () {
  fn isRotating(self, rsthis: &mut QTransform) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform10isRotatingEv()};
    let mut ret = unsafe {_ZNK10QTransform10isRotatingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn m33<T: QTransform_m33>(&mut self, value: T) -> f64 {
    return value.m33(self);
    // return 1;
  }
}

pub trait QTransform_m33 {
  fn m33(self, rsthis: &mut QTransform) -> f64;
}

// proto:  double QTransform::m33();
impl<'a> /*trait*/ QTransform_m33 for () {
  fn m33(self, rsthis: &mut QTransform) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3m33Ev()};
    let mut ret = unsafe {_ZNK10QTransform3m33Ev(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn m13<T: QTransform_m13>(&mut self, value: T) -> f64 {
    return value.m13(self);
    // return 1;
  }
}

pub trait QTransform_m13 {
  fn m13(self, rsthis: &mut QTransform) -> f64;
}

// proto:  double QTransform::m13();
impl<'a> /*trait*/ QTransform_m13 for () {
  fn m13(self, rsthis: &mut QTransform) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3m13Ev()};
    let mut ret = unsafe {_ZNK10QTransform3m13Ev(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn m21<T: QTransform_m21>(&mut self, value: T) -> f64 {
    return value.m21(self);
    // return 1;
  }
}

pub trait QTransform_m21 {
  fn m21(self, rsthis: &mut QTransform) -> f64;
}

// proto:  double QTransform::m21();
impl<'a> /*trait*/ QTransform_m21 for () {
  fn m21(self, rsthis: &mut QTransform) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3m21Ev()};
    let mut ret = unsafe {_ZNK10QTransform3m21Ev(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn isScaling<T: QTransform_isScaling>(&mut self, value: T) -> i8 {
    return value.isScaling(self);
    // return 1;
  }
}

pub trait QTransform_isScaling {
  fn isScaling(self, rsthis: &mut QTransform) -> i8;
}

// proto:  bool QTransform::isScaling();
impl<'a> /*trait*/ QTransform_isScaling for () {
  fn isScaling(self, rsthis: &mut QTransform) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform9isScalingEv()};
    let mut ret = unsafe {_ZNK10QTransform9isScalingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn inverted<T: QTransform_inverted>(&mut self, value: T) -> QTransform {
    return value.inverted(self);
    // return 1;
  }
}

pub trait QTransform_inverted {
  fn inverted(self, rsthis: &mut QTransform) -> QTransform;
}

// proto:  QTransform QTransform::inverted(bool * invertible);
impl<'a> /*trait*/ QTransform_inverted for (&'a mut i8) {
  fn inverted(self, rsthis: &mut QTransform) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform8invertedEPb()};
    let arg0 = self  as *mut int8_t;
    let mut ret = unsafe {_ZNK10QTransform8invertedEPb(rsthis.qclsinst, arg0)};
    let mut ret1 = QTransform{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn isAffine<T: QTransform_isAffine>(&mut self, value: T) -> i8 {
    return value.isAffine(self);
    // return 1;
  }
}

pub trait QTransform_isAffine {
  fn isAffine(self, rsthis: &mut QTransform) -> i8;
}

// proto:  bool QTransform::isAffine();
impl<'a> /*trait*/ QTransform_isAffine for () {
  fn isAffine(self, rsthis: &mut QTransform) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform8isAffineEv()};
    let mut ret = unsafe {_ZNK10QTransform8isAffineEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn m11<T: QTransform_m11>(&mut self, value: T) -> f64 {
    return value.m11(self);
    // return 1;
  }
}

pub trait QTransform_m11 {
  fn m11(self, rsthis: &mut QTransform) -> f64;
}

// proto:  double QTransform::m11();
impl<'a> /*trait*/ QTransform_m11 for () {
  fn m11(self, rsthis: &mut QTransform) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3m11Ev()};
    let mut ret = unsafe {_ZNK10QTransform3m11Ev(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn isIdentity<T: QTransform_isIdentity>(&mut self, value: T) -> i8 {
    return value.isIdentity(self);
    // return 1;
  }
}

pub trait QTransform_isIdentity {
  fn isIdentity(self, rsthis: &mut QTransform) -> i8;
}

// proto:  bool QTransform::isIdentity();
impl<'a> /*trait*/ QTransform_isIdentity for () {
  fn isIdentity(self, rsthis: &mut QTransform) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform10isIdentityEv()};
    let mut ret = unsafe {_ZNK10QTransform10isIdentityEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn quadToSquare<T: QTransform_quadToSquare>(&mut self, value: T) -> i8 {
    return value.quadToSquare(self);
    // return 1;
  }
}

pub trait QTransform_quadToSquare {
  fn quadToSquare(self, rsthis: &mut QTransform) -> i8;
}

// proto: static bool QTransform::quadToSquare(const QPolygonF & quad, QTransform & result);
impl<'a> /*trait*/ QTransform_quadToSquare for (&'a  QPolygonF, &'a mut QTransform) {
  fn quadToSquare(self, rsthis: &mut QTransform) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN10QTransform12quadToSquareERK9QPolygonFRS_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QTransform12quadToSquareERK9QPolygonFRS_(arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn adjoint<T: QTransform_adjoint>(&mut self, value: T) -> QTransform {
    return value.adjoint(self);
    // return 1;
  }
}

pub trait QTransform_adjoint {
  fn adjoint(self, rsthis: &mut QTransform) -> QTransform;
}

// proto:  QTransform QTransform::adjoint();
impl<'a> /*trait*/ QTransform_adjoint for () {
  fn adjoint(self, rsthis: &mut QTransform) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform7adjointEv()};
    let mut ret = unsafe {_ZNK10QTransform7adjointEv(rsthis.qclsinst)};
    let mut ret1 = QTransform{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn dx<T: QTransform_dx>(&mut self, value: T) -> f64 {
    return value.dx(self);
    // return 1;
  }
}

pub trait QTransform_dx {
  fn dx(self, rsthis: &mut QTransform) -> f64;
}

// proto:  double QTransform::dx();
impl<'a> /*trait*/ QTransform_dx for () {
  fn dx(self, rsthis: &mut QTransform) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform2dxEv()};
    let mut ret = unsafe {_ZNK10QTransform2dxEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn m23<T: QTransform_m23>(&mut self, value: T) -> f64 {
    return value.m23(self);
    // return 1;
  }
}

pub trait QTransform_m23 {
  fn m23(self, rsthis: &mut QTransform) -> f64;
}

// proto:  double QTransform::m23();
impl<'a> /*trait*/ QTransform_m23 for () {
  fn m23(self, rsthis: &mut QTransform) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3m23Ev()};
    let mut ret = unsafe {_ZNK10QTransform3m23Ev(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTransform {
  pub fn dy<T: QTransform_dy>(&mut self, value: T) -> f64 {
    return value.dy(self);
    // return 1;
  }
}

pub trait QTransform_dy {
  fn dy(self, rsthis: &mut QTransform) -> f64;
}

// proto:  double QTransform::dy();
impl<'a> /*trait*/ QTransform_dy for () {
  fn dy(self, rsthis: &mut QTransform) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform2dyEv()};
    let mut ret = unsafe {_ZNK10QTransform2dyEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
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
  pub fn m12<T: QTransform_m12>(&mut self, value: T) -> f64 {
    return value.m12(self);
    // return 1;
  }
}

pub trait QTransform_m12 {
  fn m12(self, rsthis: &mut QTransform) -> f64;
}

// proto:  double QTransform::m12();
impl<'a> /*trait*/ QTransform_m12 for () {
  fn m12(self, rsthis: &mut QTransform) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK10QTransform3m12Ev()};
    let mut ret = unsafe {_ZNK10QTransform3m12Ev(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

