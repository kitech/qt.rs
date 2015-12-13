// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qvector3d::QVector3D;
use super::qrectf::QRectF;
use super::qvector4d::QVector4D;
use super::qtransform::QTransform;
use super::qrect::QRect;
use super::qquaternion::QQuaternion;
use super::qmatrix::QMatrix;
use super::qpoint::QPoint;
use super::qpointf::QPointF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QTransform QMatrix4x4::toTransform();
  fn _ZNK10QMatrix4x411toTransformEv() -> i32;
  // proto: void QMatrix4x4::scale(const QVector3D & vector);
  fn _ZN10QMatrix4x45scaleERK9QVector3D(arg0: *const c_void) -> i32;
  // proto: void QMatrix4x4::translate(float x, float y, float z);
  fn _ZN10QMatrix4x49translateEfff(arg0: c_float, arg1: c_float, arg2: c_float) -> i32;
  // proto: const float * QMatrix4x4::constData();
  fn _ZNK10QMatrix4x49constDataEv() -> i32;
  // proto: float * QMatrix4x4::data();
  fn _ZN10QMatrix4x44dataEv() -> i32;
  // proto: QMatrix4x4 QMatrix4x4::inverted(bool * invertible);
  fn _ZNK10QMatrix4x48invertedEPb(arg0: *mut int8_t) -> i32;
  // proto: QVector3D QMatrix4x4::mapVector(const QVector3D & vector);
  fn _ZNK10QMatrix4x49mapVectorERK9QVector3D(arg0: *const c_void) -> i32;
  // proto: void QMatrix4x4::ortho(float left, float right, float bottom, float top, float nearPlane, float farPlane);
  fn _ZN10QMatrix4x45orthoEffffff(arg0: c_float, arg1: c_float, arg2: c_float, arg3: c_float, arg4: c_float, arg5: c_float) -> i32;
  // proto: void QMatrix4x4::NewQMatrix4x4();
  fn _ZN10QMatrix4x4C1Ev(qthis: *mut c_void) -> i32;
  // proto: QMatrix QMatrix4x4::toAffine();
  fn _ZNK10QMatrix4x48toAffineEv() -> i32;
  // proto: QRectF QMatrix4x4::mapRect(const QRectF & rect);
  fn _ZNK10QMatrix4x47mapRectERK6QRectF(arg0: *const c_void) -> i32;
  // proto: void QMatrix4x4::setColumn(int index, const QVector4D & value);
  fn _ZN10QMatrix4x49setColumnEiRK9QVector4D(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: bool QMatrix4x4::isIdentity();
  fn _ZNK10QMatrix4x410isIdentityEv() -> i32;
  // proto: QVector4D QMatrix4x4::column(int index);
  fn _ZNK10QMatrix4x46columnEi(arg0: c_int) -> i32;
  // proto: void QMatrix4x4::setRow(int index, const QVector4D & value);
  fn _ZN10QMatrix4x46setRowEiRK9QVector4D(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: void QMatrix4x4::flipCoordinates();
  fn _ZN10QMatrix4x415flipCoordinatesEv() -> i32;
  // proto: QGenericMatrix<3, 3, float> QMatrix4x4::normalMatrix();
  fn _ZNK10QMatrix4x412normalMatrixEv() -> i32;
  // proto: void QMatrix4x4::viewport(float left, float bottom, float width, float height, float nearPlane, float farPlane);
  fn _ZN10QMatrix4x48viewportEffffff(arg0: c_float, arg1: c_float, arg2: c_float, arg3: c_float, arg4: c_float, arg5: c_float) -> i32;
  // proto: void QMatrix4x4::copyDataTo(float * values);
  fn _ZNK10QMatrix4x410copyDataToEPf(arg0: *mut c_float) -> i32;
  // proto: void QMatrix4x4::NewQMatrix4x4(const QTransform & transform);
  fn _ZN10QMatrix4x4C1ERK10QTransform(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: bool QMatrix4x4::isAffine();
  fn _ZNK10QMatrix4x48isAffineEv() -> i32;
  // proto: void QMatrix4x4::ortho(const QRect & rect);
  fn _ZN10QMatrix4x45orthoERK5QRect(arg0: *const c_void) -> i32;
  // proto: void QMatrix4x4::rotate(const QQuaternion & quaternion);
  fn _ZN10QMatrix4x46rotateERK11QQuaternion(arg0: *const c_void) -> i32;
  // proto: void QMatrix4x4::NewQMatrix4x4(const QMatrix & matrix);
  fn _ZN10QMatrix4x4C1ERK7QMatrix(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QMatrix4x4::perspective(float verticalAngle, float aspectRatio, float nearPlane, float farPlane);
  fn _ZN10QMatrix4x411perspectiveEffff(arg0: c_float, arg1: c_float, arg2: c_float, arg3: c_float) -> i32;
  // proto: void QMatrix4x4::translate(const QVector3D & vector);
  fn _ZN10QMatrix4x49translateERK9QVector3D(arg0: *const c_void) -> i32;
  // proto: double QMatrix4x4::determinant();
  fn _ZNK10QMatrix4x411determinantEv() -> i32;
  // proto: void QMatrix4x4::scale(float x, float y, float z);
  fn _ZN10QMatrix4x45scaleEfff(arg0: c_float, arg1: c_float, arg2: c_float) -> i32;
  // proto: void QMatrix4x4::frustum(float left, float right, float bottom, float top, float nearPlane, float farPlane);
  fn _ZN10QMatrix4x47frustumEffffff(arg0: c_float, arg1: c_float, arg2: c_float, arg3: c_float, arg4: c_float, arg5: c_float) -> i32;
  // proto: QPoint QMatrix4x4::map(const QPoint & point);
  fn _ZNK10QMatrix4x43mapERK6QPoint(arg0: *const c_void) -> i32;
  // proto: void QMatrix4x4::NewQMatrix4x4(int );
  fn _ZN10QMatrix4x4C1Ei(qthis: *mut c_void, arg0: c_int) -> i32;
  // proto: void QMatrix4x4::optimize();
  fn _ZN10QMatrix4x48optimizeEv() -> i32;
  // proto: void QMatrix4x4::NewQMatrix4x4(const float * values);
  fn _ZN10QMatrix4x4C1EPKf(qthis: *mut c_void, arg0: *const c_float) -> i32;
  // proto: void QMatrix4x4::translate(float x, float y);
  fn _ZN10QMatrix4x49translateEff(arg0: c_float, arg1: c_float) -> i32;
  // proto: void QMatrix4x4::setToIdentity();
  fn _ZN10QMatrix4x413setToIdentityEv() -> i32;
  // proto: QRect QMatrix4x4::mapRect(const QRect & rect);
  fn _ZNK10QMatrix4x47mapRectERK5QRect(arg0: *const c_void) -> i32;
  // proto: void QMatrix4x4::scale(float x, float y);
  fn _ZN10QMatrix4x45scaleEff(arg0: c_float, arg1: c_float) -> i32;
  // proto: void QMatrix4x4::NewQMatrix4x4(float m11, float m12, float m13, float m14, float m21, float m22, float m23, float m24, float m31, float m32, float m33, float m34, float m41, float m42, float m43, float m44);
  fn _ZN10QMatrix4x4C1Effffffffffffffff(qthis: *mut c_void, arg0: c_float, arg1: c_float, arg2: c_float, arg3: c_float, arg4: c_float, arg5: c_float, arg6: c_float, arg7: c_float, arg8: c_float, arg9: c_float, arg10: c_float, arg11: c_float, arg12: c_float, arg13: c_float, arg14: c_float, arg15: c_float) -> i32;
  // proto: QVector3D QMatrix4x4::map(const QVector3D & point);
  fn _ZNK10QMatrix4x43mapERK9QVector3D(arg0: *const c_void) -> i32;
  // proto: void QMatrix4x4::lookAt(const QVector3D & eye, const QVector3D & center, const QVector3D & up);
  fn _ZN10QMatrix4x46lookAtERK9QVector3DS2_S2_(arg0: *const c_void, arg1: *const c_void, arg2: *const c_void) -> i32;
  // proto: void QMatrix4x4::ortho(const QRectF & rect);
  fn _ZN10QMatrix4x45orthoERK6QRectF(arg0: *const c_void) -> i32;
  // proto: void QMatrix4x4::viewport(const QRectF & rect);
  fn _ZN10QMatrix4x48viewportERK6QRectF(arg0: *const c_void) -> i32;
  // proto: void QMatrix4x4::rotate(float angle, float x, float y, float z);
  fn _ZN10QMatrix4x46rotateEffff(arg0: c_float, arg1: c_float, arg2: c_float, arg3: c_float) -> i32;
  // proto: void QMatrix4x4::fill(float value);
  fn _ZN10QMatrix4x44fillEf(arg0: c_float) -> i32;
  // proto: void QMatrix4x4::NewQMatrix4x4(const float * values, int cols, int rows);
  fn _ZN10QMatrix4x4C1EPKfii(qthis: *mut c_void, arg0: *const c_float, arg1: c_int, arg2: c_int) -> i32;
  // proto: QTransform QMatrix4x4::toTransform(float distanceToPlane);
  fn _ZNK10QMatrix4x411toTransformEf(arg0: c_float) -> i32;
  // proto: QMatrix4x4 QMatrix4x4::transposed();
  fn _ZNK10QMatrix4x410transposedEv() -> i32;
  // proto: QPointF QMatrix4x4::map(const QPointF & point);
  fn _ZNK10QMatrix4x43mapERK7QPointF(arg0: *const c_void) -> i32;
  // proto: void QMatrix4x4::scale(float factor);
  fn _ZN10QMatrix4x45scaleEf(arg0: c_float) -> i32;
  // proto: QVector4D QMatrix4x4::row(int index);
  fn _ZNK10QMatrix4x43rowEi(arg0: c_int) -> i32;
  // proto: void QMatrix4x4::rotate(float angle, const QVector3D & vector);
  fn _ZN10QMatrix4x46rotateEfRK9QVector3D(arg0: c_float, arg1: *const c_void) -> i32;
  // proto: QVector4D QMatrix4x4::map(const QVector4D & point);
  fn _ZNK10QMatrix4x43mapERK9QVector4D(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QMatrix4x4)=68
pub struct QMatrix4x4 {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMatrix4x4 {
  pub fn toTransform<T: QMatrix4x4_toTransform>(&mut self, value: T) -> i32 {
    value.toTransform(self);
    return 1;
  }
}

pub trait QMatrix4x4_toTransform {
  fn toTransform(self, this: &mut QMatrix4x4) -> i32;
}

// proto: QTransform QMatrix4x4::toTransform();
impl<'a> /*trait*/ QMatrix4x4_toTransform for () {
  fn toTransform(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZNK10QMatrix4x411toTransformEv()};
    unsafe {_ZNK10QMatrix4x411toTransformEv()};
    return 1;
  }
}

impl /*struct*/ QMatrix4x4 {
  pub fn scale<T: QMatrix4x4_scale>(&mut self, value: T) -> i32 {
    value.scale(self);
    return 1;
  }
}

pub trait QMatrix4x4_scale {
  fn scale(self, this: &mut QMatrix4x4) -> i32;
}

// proto: void QMatrix4x4::scale(const QVector3D & vector);
impl<'a> /*trait*/ QMatrix4x4_scale for (&'a  QVector3D) {
  fn scale(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x45scaleERK9QVector3D()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QMatrix4x45scaleERK9QVector3D(arg0)};
    return 1;
  }
}

impl /*struct*/ QMatrix4x4 {
  pub fn translate<T: QMatrix4x4_translate>(&mut self, value: T) -> i32 {
    value.translate(self);
    return 1;
  }
}

pub trait QMatrix4x4_translate {
  fn translate(self, this: &mut QMatrix4x4) -> i32;
}

// proto: void QMatrix4x4::translate(float x, float y, float z);
impl<'a> /*trait*/ QMatrix4x4_translate for (f32, f32, f32) {
  fn translate(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x49translateEfff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    unsafe {_ZN10QMatrix4x49translateEfff(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QMatrix4x4 {
  pub fn constData<T: QMatrix4x4_constData>(&mut self, value: T) -> i32 {
    value.constData(self);
    return 1;
  }
}

pub trait QMatrix4x4_constData {
  fn constData(self, this: &mut QMatrix4x4) -> i32;
}

// proto: const float * QMatrix4x4::constData();
impl<'a> /*trait*/ QMatrix4x4_constData for () {
  fn constData(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZNK10QMatrix4x49constDataEv()};
    unsafe {_ZNK10QMatrix4x49constDataEv()};
    return 1;
  }
}

impl /*struct*/ QMatrix4x4 {
  pub fn data<T: QMatrix4x4_data>(&mut self, value: T) -> i32 {
    value.data(self);
    return 1;
  }
}

pub trait QMatrix4x4_data {
  fn data(self, this: &mut QMatrix4x4) -> i32;
}

// proto: float * QMatrix4x4::data();
impl<'a> /*trait*/ QMatrix4x4_data for () {
  fn data(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x44dataEv()};
    unsafe {_ZN10QMatrix4x44dataEv()};
    return 1;
  }
}

impl /*struct*/ QMatrix4x4 {
  pub fn inverted<T: QMatrix4x4_inverted>(&mut self, value: T) -> i32 {
    value.inverted(self);
    return 1;
  }
}

pub trait QMatrix4x4_inverted {
  fn inverted(self, this: &mut QMatrix4x4) -> i32;
}

// proto: QMatrix4x4 QMatrix4x4::inverted(bool * invertible);
impl<'a> /*trait*/ QMatrix4x4_inverted for (&'a mut i8) {
  fn inverted(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZNK10QMatrix4x48invertedEPb()};
    let arg0 = self  as *mut int8_t;
    unsafe {_ZNK10QMatrix4x48invertedEPb(arg0)};
    return 1;
  }
}

impl /*struct*/ QMatrix4x4 {
  pub fn mapVector<T: QMatrix4x4_mapVector>(&mut self, value: T) -> i32 {
    value.mapVector(self);
    return 1;
  }
}

pub trait QMatrix4x4_mapVector {
  fn mapVector(self, this: &mut QMatrix4x4) -> i32;
}

// proto: QVector3D QMatrix4x4::mapVector(const QVector3D & vector);
impl<'a> /*trait*/ QMatrix4x4_mapVector for (&'a  QVector3D) {
  fn mapVector(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZNK10QMatrix4x49mapVectorERK9QVector3D()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK10QMatrix4x49mapVectorERK9QVector3D(arg0)};
    return 1;
  }
}

impl /*struct*/ QMatrix4x4 {
  pub fn ortho<T: QMatrix4x4_ortho>(&mut self, value: T) -> i32 {
    value.ortho(self);
    return 1;
  }
}

pub trait QMatrix4x4_ortho {
  fn ortho(self, this: &mut QMatrix4x4) -> i32;
}

// proto: void QMatrix4x4::ortho(float left, float right, float bottom, float top, float nearPlane, float farPlane);
impl<'a> /*trait*/ QMatrix4x4_ortho for (f32, f32, f32, f32, f32, f32) {
  fn ortho(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x45orthoEffffff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
    let arg4 = self.4  as c_float;
    let arg5 = self.5  as c_float;
    unsafe {_ZN10QMatrix4x45orthoEffffff(arg0, arg1, arg2, arg3, arg4, arg5)};
    return 1;
  }
}

impl /*struct*/ QMatrix4x4 {
  pub fn NewQMatrix4x4<T: QMatrix4x4_NewQMatrix4x4>(value: T) -> QMatrix4x4 {
    let rsthis = value.NewQMatrix4x4();
    return rsthis;
    // return 1;
  }
}

pub trait QMatrix4x4_NewQMatrix4x4 {
  fn NewQMatrix4x4(self) -> QMatrix4x4;
}

// proto: void QMatrix4x4::NewQMatrix4x4();
impl<'a> /*trait*/ QMatrix4x4_NewQMatrix4x4 for () {
  fn NewQMatrix4x4(self) -> QMatrix4x4 {
    let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x4C1Ev()};
    unsafe {_ZN10QMatrix4x4C1Ev(qthis)};
    let rsthis = QMatrix4x4{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMatrix4x4 {
  pub fn toAffine<T: QMatrix4x4_toAffine>(&mut self, value: T) -> i32 {
    value.toAffine(self);
    return 1;
  }
}

pub trait QMatrix4x4_toAffine {
  fn toAffine(self, this: &mut QMatrix4x4) -> i32;
}

// proto: QMatrix QMatrix4x4::toAffine();
impl<'a> /*trait*/ QMatrix4x4_toAffine for () {
  fn toAffine(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZNK10QMatrix4x48toAffineEv()};
    unsafe {_ZNK10QMatrix4x48toAffineEv()};
    return 1;
  }
}

impl /*struct*/ QMatrix4x4 {
  pub fn mapRect<T: QMatrix4x4_mapRect>(&mut self, value: T) -> i32 {
    value.mapRect(self);
    return 1;
  }
}

pub trait QMatrix4x4_mapRect {
  fn mapRect(self, this: &mut QMatrix4x4) -> i32;
}

// proto: QRectF QMatrix4x4::mapRect(const QRectF & rect);
impl<'a> /*trait*/ QMatrix4x4_mapRect for (&'a  QRectF) {
  fn mapRect(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZNK10QMatrix4x47mapRectERK6QRectF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK10QMatrix4x47mapRectERK6QRectF(arg0)};
    return 1;
  }
}

impl /*struct*/ QMatrix4x4 {
  pub fn setColumn<T: QMatrix4x4_setColumn>(&mut self, value: T) -> i32 {
    value.setColumn(self);
    return 1;
  }
}

pub trait QMatrix4x4_setColumn {
  fn setColumn(self, this: &mut QMatrix4x4) -> i32;
}

// proto: void QMatrix4x4::setColumn(int index, const QVector4D & value);
impl<'a> /*trait*/ QMatrix4x4_setColumn for (i32, &'a  QVector4D) {
  fn setColumn(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x49setColumnEiRK9QVector4D()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN10QMatrix4x49setColumnEiRK9QVector4D(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QMatrix4x4 {
  pub fn isIdentity<T: QMatrix4x4_isIdentity>(&mut self, value: T) -> i32 {
    value.isIdentity(self);
    return 1;
  }
}

pub trait QMatrix4x4_isIdentity {
  fn isIdentity(self, this: &mut QMatrix4x4) -> i32;
}

// proto: bool QMatrix4x4::isIdentity();
impl<'a> /*trait*/ QMatrix4x4_isIdentity for () {
  fn isIdentity(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZNK10QMatrix4x410isIdentityEv()};
    unsafe {_ZNK10QMatrix4x410isIdentityEv()};
    return 1;
  }
}

impl /*struct*/ QMatrix4x4 {
  pub fn column<T: QMatrix4x4_column>(&mut self, value: T) -> i32 {
    value.column(self);
    return 1;
  }
}

pub trait QMatrix4x4_column {
  fn column(self, this: &mut QMatrix4x4) -> i32;
}

// proto: QVector4D QMatrix4x4::column(int index);
impl<'a> /*trait*/ QMatrix4x4_column for (i32) {
  fn column(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZNK10QMatrix4x46columnEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK10QMatrix4x46columnEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QMatrix4x4 {
  pub fn setRow<T: QMatrix4x4_setRow>(&mut self, value: T) -> i32 {
    value.setRow(self);
    return 1;
  }
}

pub trait QMatrix4x4_setRow {
  fn setRow(self, this: &mut QMatrix4x4) -> i32;
}

// proto: void QMatrix4x4::setRow(int index, const QVector4D & value);
impl<'a> /*trait*/ QMatrix4x4_setRow for (i32, &'a  QVector4D) {
  fn setRow(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x46setRowEiRK9QVector4D()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN10QMatrix4x46setRowEiRK9QVector4D(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QMatrix4x4 {
  pub fn flipCoordinates<T: QMatrix4x4_flipCoordinates>(&mut self, value: T) -> i32 {
    value.flipCoordinates(self);
    return 1;
  }
}

pub trait QMatrix4x4_flipCoordinates {
  fn flipCoordinates(self, this: &mut QMatrix4x4) -> i32;
}

// proto: void QMatrix4x4::flipCoordinates();
impl<'a> /*trait*/ QMatrix4x4_flipCoordinates for () {
  fn flipCoordinates(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x415flipCoordinatesEv()};
    unsafe {_ZN10QMatrix4x415flipCoordinatesEv()};
    return 1;
  }
}

impl /*struct*/ QMatrix4x4 {
  pub fn normalMatrix<T: QMatrix4x4_normalMatrix>(&mut self, value: T) -> i32 {
    value.normalMatrix(self);
    return 1;
  }
}

pub trait QMatrix4x4_normalMatrix {
  fn normalMatrix(self, this: &mut QMatrix4x4) -> i32;
}

// proto: QGenericMatrix<3, 3, float> QMatrix4x4::normalMatrix();
impl<'a> /*trait*/ QMatrix4x4_normalMatrix for () {
  fn normalMatrix(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZNK10QMatrix4x412normalMatrixEv()};
    unsafe {_ZNK10QMatrix4x412normalMatrixEv()};
    return 1;
  }
}

impl /*struct*/ QMatrix4x4 {
  pub fn viewport<T: QMatrix4x4_viewport>(&mut self, value: T) -> i32 {
    value.viewport(self);
    return 1;
  }
}

pub trait QMatrix4x4_viewport {
  fn viewport(self, this: &mut QMatrix4x4) -> i32;
}

// proto: void QMatrix4x4::viewport(float left, float bottom, float width, float height, float nearPlane, float farPlane);
impl<'a> /*trait*/ QMatrix4x4_viewport for (f32, f32, f32, f32, f32, f32) {
  fn viewport(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x48viewportEffffff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
    let arg4 = self.4  as c_float;
    let arg5 = self.5  as c_float;
    unsafe {_ZN10QMatrix4x48viewportEffffff(arg0, arg1, arg2, arg3, arg4, arg5)};
    return 1;
  }
}

impl /*struct*/ QMatrix4x4 {
  pub fn copyDataTo<T: QMatrix4x4_copyDataTo>(&mut self, value: T) -> i32 {
    value.copyDataTo(self);
    return 1;
  }
}

pub trait QMatrix4x4_copyDataTo {
  fn copyDataTo(self, this: &mut QMatrix4x4) -> i32;
}

// proto: void QMatrix4x4::copyDataTo(float * values);
impl<'a> /*trait*/ QMatrix4x4_copyDataTo for (&'a mut f32) {
  fn copyDataTo(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZNK10QMatrix4x410copyDataToEPf()};
    let arg0 = self  as *mut c_float;
    unsafe {_ZNK10QMatrix4x410copyDataToEPf(arg0)};
    return 1;
  }
}

// proto: void QMatrix4x4::NewQMatrix4x4(const QTransform & transform);
impl<'a> /*trait*/ QMatrix4x4_NewQMatrix4x4 for (&'a  QTransform) {
  fn NewQMatrix4x4(self) -> QMatrix4x4 {
    let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x4C1ERK10QTransform()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QMatrix4x4C1ERK10QTransform(qthis, arg0)};
    let rsthis = QMatrix4x4{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMatrix4x4 {
  pub fn isAffine<T: QMatrix4x4_isAffine>(&mut self, value: T) -> i32 {
    value.isAffine(self);
    return 1;
  }
}

pub trait QMatrix4x4_isAffine {
  fn isAffine(self, this: &mut QMatrix4x4) -> i32;
}

// proto: bool QMatrix4x4::isAffine();
impl<'a> /*trait*/ QMatrix4x4_isAffine for () {
  fn isAffine(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZNK10QMatrix4x48isAffineEv()};
    unsafe {_ZNK10QMatrix4x48isAffineEv()};
    return 1;
  }
}

// proto: void QMatrix4x4::ortho(const QRect & rect);
impl<'a> /*trait*/ QMatrix4x4_ortho for (&'a  QRect) {
  fn ortho(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x45orthoERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QMatrix4x45orthoERK5QRect(arg0)};
    return 1;
  }
}

impl /*struct*/ QMatrix4x4 {
  pub fn rotate<T: QMatrix4x4_rotate>(&mut self, value: T) -> i32 {
    value.rotate(self);
    return 1;
  }
}

pub trait QMatrix4x4_rotate {
  fn rotate(self, this: &mut QMatrix4x4) -> i32;
}

// proto: void QMatrix4x4::rotate(const QQuaternion & quaternion);
impl<'a> /*trait*/ QMatrix4x4_rotate for (&'a  QQuaternion) {
  fn rotate(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x46rotateERK11QQuaternion()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QMatrix4x46rotateERK11QQuaternion(arg0)};
    return 1;
  }
}

// proto: void QMatrix4x4::NewQMatrix4x4(const QMatrix & matrix);
impl<'a> /*trait*/ QMatrix4x4_NewQMatrix4x4 for (&'a  QMatrix) {
  fn NewQMatrix4x4(self) -> QMatrix4x4 {
    let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x4C1ERK7QMatrix()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QMatrix4x4C1ERK7QMatrix(qthis, arg0)};
    let rsthis = QMatrix4x4{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMatrix4x4 {
  pub fn perspective<T: QMatrix4x4_perspective>(&mut self, value: T) -> i32 {
    value.perspective(self);
    return 1;
  }
}

pub trait QMatrix4x4_perspective {
  fn perspective(self, this: &mut QMatrix4x4) -> i32;
}

// proto: void QMatrix4x4::perspective(float verticalAngle, float aspectRatio, float nearPlane, float farPlane);
impl<'a> /*trait*/ QMatrix4x4_perspective for (f32, f32, f32, f32) {
  fn perspective(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x411perspectiveEffff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
    unsafe {_ZN10QMatrix4x411perspectiveEffff(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

// proto: void QMatrix4x4::translate(const QVector3D & vector);
impl<'a> /*trait*/ QMatrix4x4_translate for (&'a  QVector3D) {
  fn translate(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x49translateERK9QVector3D()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QMatrix4x49translateERK9QVector3D(arg0)};
    return 1;
  }
}

impl /*struct*/ QMatrix4x4 {
  pub fn determinant<T: QMatrix4x4_determinant>(&mut self, value: T) -> i32 {
    value.determinant(self);
    return 1;
  }
}

pub trait QMatrix4x4_determinant {
  fn determinant(self, this: &mut QMatrix4x4) -> i32;
}

// proto: double QMatrix4x4::determinant();
impl<'a> /*trait*/ QMatrix4x4_determinant for () {
  fn determinant(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZNK10QMatrix4x411determinantEv()};
    unsafe {_ZNK10QMatrix4x411determinantEv()};
    return 1;
  }
}

// proto: void QMatrix4x4::scale(float x, float y, float z);
impl<'a> /*trait*/ QMatrix4x4_scale for (f32, f32, f32) {
  fn scale(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x45scaleEfff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    unsafe {_ZN10QMatrix4x45scaleEfff(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QMatrix4x4 {
  pub fn frustum<T: QMatrix4x4_frustum>(&mut self, value: T) -> i32 {
    value.frustum(self);
    return 1;
  }
}

pub trait QMatrix4x4_frustum {
  fn frustum(self, this: &mut QMatrix4x4) -> i32;
}

// proto: void QMatrix4x4::frustum(float left, float right, float bottom, float top, float nearPlane, float farPlane);
impl<'a> /*trait*/ QMatrix4x4_frustum for (f32, f32, f32, f32, f32, f32) {
  fn frustum(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x47frustumEffffff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
    let arg4 = self.4  as c_float;
    let arg5 = self.5  as c_float;
    unsafe {_ZN10QMatrix4x47frustumEffffff(arg0, arg1, arg2, arg3, arg4, arg5)};
    return 1;
  }
}

impl /*struct*/ QMatrix4x4 {
  pub fn map<T: QMatrix4x4_map>(&mut self, value: T) -> i32 {
    value.map(self);
    return 1;
  }
}

pub trait QMatrix4x4_map {
  fn map(self, this: &mut QMatrix4x4) -> i32;
}

// proto: QPoint QMatrix4x4::map(const QPoint & point);
impl<'a> /*trait*/ QMatrix4x4_map for (&'a  QPoint) {
  fn map(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZNK10QMatrix4x43mapERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK10QMatrix4x43mapERK6QPoint(arg0)};
    return 1;
  }
}

// proto: void QMatrix4x4::NewQMatrix4x4(int );
impl<'a> /*trait*/ QMatrix4x4_NewQMatrix4x4 for (i32) {
  fn NewQMatrix4x4(self) -> QMatrix4x4 {
    let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x4C1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QMatrix4x4C1Ei(qthis, arg0)};
    let rsthis = QMatrix4x4{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMatrix4x4 {
  pub fn optimize<T: QMatrix4x4_optimize>(&mut self, value: T) -> i32 {
    value.optimize(self);
    return 1;
  }
}

pub trait QMatrix4x4_optimize {
  fn optimize(self, this: &mut QMatrix4x4) -> i32;
}

// proto: void QMatrix4x4::optimize();
impl<'a> /*trait*/ QMatrix4x4_optimize for () {
  fn optimize(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x48optimizeEv()};
    unsafe {_ZN10QMatrix4x48optimizeEv()};
    return 1;
  }
}

// proto: void QMatrix4x4::NewQMatrix4x4(const float * values);
impl<'a> /*trait*/ QMatrix4x4_NewQMatrix4x4 for (&'a  f32) {
  fn NewQMatrix4x4(self) -> QMatrix4x4 {
    let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x4C1EPKf()};
    let arg0 = self  as *const c_float;
    unsafe {_ZN10QMatrix4x4C1EPKf(qthis, arg0)};
    let rsthis = QMatrix4x4{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QMatrix4x4::translate(float x, float y);
impl<'a> /*trait*/ QMatrix4x4_translate for (f32, f32) {
  fn translate(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x49translateEff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
    unsafe {_ZN10QMatrix4x49translateEff(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QMatrix4x4 {
  pub fn setToIdentity<T: QMatrix4x4_setToIdentity>(&mut self, value: T) -> i32 {
    value.setToIdentity(self);
    return 1;
  }
}

pub trait QMatrix4x4_setToIdentity {
  fn setToIdentity(self, this: &mut QMatrix4x4) -> i32;
}

// proto: void QMatrix4x4::setToIdentity();
impl<'a> /*trait*/ QMatrix4x4_setToIdentity for () {
  fn setToIdentity(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x413setToIdentityEv()};
    unsafe {_ZN10QMatrix4x413setToIdentityEv()};
    return 1;
  }
}

// proto: QRect QMatrix4x4::mapRect(const QRect & rect);
impl<'a> /*trait*/ QMatrix4x4_mapRect for (&'a  QRect) {
  fn mapRect(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZNK10QMatrix4x47mapRectERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK10QMatrix4x47mapRectERK5QRect(arg0)};
    return 1;
  }
}

// proto: void QMatrix4x4::scale(float x, float y);
impl<'a> /*trait*/ QMatrix4x4_scale for (f32, f32) {
  fn scale(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x45scaleEff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
    unsafe {_ZN10QMatrix4x45scaleEff(arg0, arg1)};
    return 1;
  }
}

// proto: void QMatrix4x4::NewQMatrix4x4(float m11, float m12, float m13, float m14, float m21, float m22, float m23, float m24, float m31, float m32, float m33, float m34, float m41, float m42, float m43, float m44);
impl<'a> /*trait*/ QMatrix4x4_NewQMatrix4x4 for (f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32) {
  fn NewQMatrix4x4(self) -> QMatrix4x4 {
    let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x4C1Effffffffffffffff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
    let arg4 = self.4  as c_float;
    let arg5 = self.5  as c_float;
    let arg6 = self.6  as c_float;
    let arg7 = self.7  as c_float;
    let arg8 = self.8  as c_float;
    let arg9 = self.9  as c_float;
    let arg10 = self.10  as c_float;
    let arg11 = self.11  as c_float;
    let arg12 = self.12  as c_float;
    let arg13 = self.13  as c_float;
    let arg14 = self.14  as c_float;
    let arg15 = self.15  as c_float;
    unsafe {_ZN10QMatrix4x4C1Effffffffffffffff(qthis, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14, arg15)};
    let rsthis = QMatrix4x4{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: QVector3D QMatrix4x4::map(const QVector3D & point);
impl<'a> /*trait*/ QMatrix4x4_map for (&'a  QVector3D) {
  fn map(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZNK10QMatrix4x43mapERK9QVector3D()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK10QMatrix4x43mapERK9QVector3D(arg0)};
    return 1;
  }
}

impl /*struct*/ QMatrix4x4 {
  pub fn lookAt<T: QMatrix4x4_lookAt>(&mut self, value: T) -> i32 {
    value.lookAt(self);
    return 1;
  }
}

pub trait QMatrix4x4_lookAt {
  fn lookAt(self, this: &mut QMatrix4x4) -> i32;
}

// proto: void QMatrix4x4::lookAt(const QVector3D & eye, const QVector3D & center, const QVector3D & up);
impl<'a> /*trait*/ QMatrix4x4_lookAt for (&'a  QVector3D, &'a  QVector3D, &'a  QVector3D) {
  fn lookAt(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x46lookAtERK9QVector3DS2_S2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN10QMatrix4x46lookAtERK9QVector3DS2_S2_(arg0, arg1, arg2)};
    return 1;
  }
}

// proto: void QMatrix4x4::ortho(const QRectF & rect);
impl<'a> /*trait*/ QMatrix4x4_ortho for (&'a  QRectF) {
  fn ortho(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x45orthoERK6QRectF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QMatrix4x45orthoERK6QRectF(arg0)};
    return 1;
  }
}

// proto: void QMatrix4x4::viewport(const QRectF & rect);
impl<'a> /*trait*/ QMatrix4x4_viewport for (&'a  QRectF) {
  fn viewport(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x48viewportERK6QRectF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QMatrix4x48viewportERK6QRectF(arg0)};
    return 1;
  }
}

// proto: void QMatrix4x4::rotate(float angle, float x, float y, float z);
impl<'a> /*trait*/ QMatrix4x4_rotate for (f32, f32, f32, f32) {
  fn rotate(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x46rotateEffff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
    unsafe {_ZN10QMatrix4x46rotateEffff(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QMatrix4x4 {
  pub fn fill<T: QMatrix4x4_fill>(&mut self, value: T) -> i32 {
    value.fill(self);
    return 1;
  }
}

pub trait QMatrix4x4_fill {
  fn fill(self, this: &mut QMatrix4x4) -> i32;
}

// proto: void QMatrix4x4::fill(float value);
impl<'a> /*trait*/ QMatrix4x4_fill for (f32) {
  fn fill(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x44fillEf()};
    let arg0 = self  as c_float;
    unsafe {_ZN10QMatrix4x44fillEf(arg0)};
    return 1;
  }
}

// proto: void QMatrix4x4::NewQMatrix4x4(const float * values, int cols, int rows);
impl<'a> /*trait*/ QMatrix4x4_NewQMatrix4x4 for (&'a  f32, i32, i32) {
  fn NewQMatrix4x4(self) -> QMatrix4x4 {
    let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x4C1EPKfii()};
    let arg0 = self.0  as *const c_float;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZN10QMatrix4x4C1EPKfii(qthis, arg0, arg1, arg2)};
    let rsthis = QMatrix4x4{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: QTransform QMatrix4x4::toTransform(float distanceToPlane);
impl<'a> /*trait*/ QMatrix4x4_toTransform for (f32) {
  fn toTransform(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZNK10QMatrix4x411toTransformEf()};
    let arg0 = self  as c_float;
    unsafe {_ZNK10QMatrix4x411toTransformEf(arg0)};
    return 1;
  }
}

impl /*struct*/ QMatrix4x4 {
  pub fn transposed<T: QMatrix4x4_transposed>(&mut self, value: T) -> i32 {
    value.transposed(self);
    return 1;
  }
}

pub trait QMatrix4x4_transposed {
  fn transposed(self, this: &mut QMatrix4x4) -> i32;
}

// proto: QMatrix4x4 QMatrix4x4::transposed();
impl<'a> /*trait*/ QMatrix4x4_transposed for () {
  fn transposed(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZNK10QMatrix4x410transposedEv()};
    unsafe {_ZNK10QMatrix4x410transposedEv()};
    return 1;
  }
}

// proto: QPointF QMatrix4x4::map(const QPointF & point);
impl<'a> /*trait*/ QMatrix4x4_map for (&'a  QPointF) {
  fn map(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZNK10QMatrix4x43mapERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK10QMatrix4x43mapERK7QPointF(arg0)};
    return 1;
  }
}

// proto: void QMatrix4x4::scale(float factor);
impl<'a> /*trait*/ QMatrix4x4_scale for (f32) {
  fn scale(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x45scaleEf()};
    let arg0 = self  as c_float;
    unsafe {_ZN10QMatrix4x45scaleEf(arg0)};
    return 1;
  }
}

impl /*struct*/ QMatrix4x4 {
  pub fn row<T: QMatrix4x4_row>(&mut self, value: T) -> i32 {
    value.row(self);
    return 1;
  }
}

pub trait QMatrix4x4_row {
  fn row(self, this: &mut QMatrix4x4) -> i32;
}

// proto: QVector4D QMatrix4x4::row(int index);
impl<'a> /*trait*/ QMatrix4x4_row for (i32) {
  fn row(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZNK10QMatrix4x43rowEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK10QMatrix4x43rowEi(arg0)};
    return 1;
  }
}

// proto: void QMatrix4x4::rotate(float angle, const QVector3D & vector);
impl<'a> /*trait*/ QMatrix4x4_rotate for (f32, &'a  QVector3D) {
  fn rotate(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x46rotateEfRK9QVector3D()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN10QMatrix4x46rotateEfRK9QVector3D(arg0, arg1)};
    return 1;
  }
}

// proto: QVector4D QMatrix4x4::map(const QVector4D & point);
impl<'a> /*trait*/ QMatrix4x4_map for (&'a  QVector4D) {
  fn map(self, this: &mut QMatrix4x4) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZNK10QMatrix4x43mapERK9QVector4D()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK10QMatrix4x43mapERK9QVector4D(arg0)};
    return 1;
  }
}

