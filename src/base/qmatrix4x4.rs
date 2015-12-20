// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qtransform::QTransform;
use super::qvector3d::QVector3D;
use super::qmatrix::QMatrix;
use super::qrectf::QRectF;
use super::qvector4d::QVector4D;
use super::qrect::QRect;
use super::qquaternion::QQuaternion;
use super::qpoint::QPoint;
use super::qpointf::QPointF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QTransform QMatrix4x4::toTransform();
  fn _ZNK10QMatrix4x411toTransformEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMatrix4x4::scale(const QVector3D & vector);
  fn _ZN10QMatrix4x45scaleERK9QVector3D(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QMatrix4x4::translate(float x, float y, float z);
  fn _ZN10QMatrix4x49translateEfff(qthis: *mut c_void, arg0: c_float, arg1: c_float, arg2: c_float);
  // proto:  const float * QMatrix4x4::constData();
  fn _ZNK10QMatrix4x49constDataEv(qthis: *mut c_void) -> *mut c_float;
  // proto:  float * QMatrix4x4::data();
  fn _ZN10QMatrix4x44dataEv(qthis: *mut c_void) -> *mut c_float;
  // proto:  QMatrix4x4 QMatrix4x4::inverted(bool * invertible);
  fn _ZNK10QMatrix4x48invertedEPb(qthis: *mut c_void, arg0: *mut c_char) -> *mut c_void;
  // proto:  QVector3D QMatrix4x4::mapVector(const QVector3D & vector);
  fn _ZNK10QMatrix4x49mapVectorERK9QVector3D(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QMatrix4x4::ortho(float left, float right, float bottom, float top, float nearPlane, float farPlane);
  fn _ZN10QMatrix4x45orthoEffffff(qthis: *mut c_void, arg0: c_float, arg1: c_float, arg2: c_float, arg3: c_float, arg4: c_float, arg5: c_float);
  // proto:  void QMatrix4x4::QMatrix4x4();
  fn _ZN10QMatrix4x4C1Ev(qthis: *mut c_void);
  // proto:  QMatrix QMatrix4x4::toAffine();
  fn _ZNK10QMatrix4x48toAffineEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QRectF QMatrix4x4::mapRect(const QRectF & rect);
  fn _ZNK10QMatrix4x47mapRectERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QMatrix4x4::setColumn(int index, const QVector4D & value);
  fn _ZN10QMatrix4x49setColumnEiRK9QVector4D(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  bool QMatrix4x4::isIdentity();
  fn _ZNK10QMatrix4x410isIdentityEv(qthis: *mut c_void) -> c_char;
  // proto:  QVector4D QMatrix4x4::column(int index);
  fn _ZNK10QMatrix4x46columnEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QMatrix4x4::setRow(int index, const QVector4D & value);
  fn _ZN10QMatrix4x46setRowEiRK9QVector4D(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  void QMatrix4x4::flipCoordinates();
  fn _ZN10QMatrix4x415flipCoordinatesEv(qthis: *mut c_void);
  // proto:  QMatrix3x3 QMatrix4x4::normalMatrix();
  fn _ZNK10QMatrix4x412normalMatrixEv(qthis: *mut c_void);
  // proto:  void QMatrix4x4::viewport(float left, float bottom, float width, float height, float nearPlane, float farPlane);
  fn _ZN10QMatrix4x48viewportEffffff(qthis: *mut c_void, arg0: c_float, arg1: c_float, arg2: c_float, arg3: c_float, arg4: c_float, arg5: c_float);
  // proto:  void QMatrix4x4::copyDataTo(float * values);
  fn _ZNK10QMatrix4x410copyDataToEPf(qthis: *mut c_void, arg0: *mut c_float);
  // proto:  void QMatrix4x4::QMatrix4x4(const QTransform & transform);
  fn _ZN10QMatrix4x4C1ERK10QTransform(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QMatrix4x4::isAffine();
  fn _ZNK10QMatrix4x48isAffineEv(qthis: *mut c_void) -> c_char;
  // proto:  void QMatrix4x4::ortho(const QRect & rect);
  fn _ZN10QMatrix4x45orthoERK5QRect(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QMatrix4x4::rotate(const QQuaternion & quaternion);
  fn _ZN10QMatrix4x46rotateERK11QQuaternion(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QMatrix4x4::QMatrix4x4(const QMatrix & matrix);
  fn _ZN10QMatrix4x4C1ERK7QMatrix(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QMatrix4x4::perspective(float verticalAngle, float aspectRatio, float nearPlane, float farPlane);
  fn _ZN10QMatrix4x411perspectiveEffff(qthis: *mut c_void, arg0: c_float, arg1: c_float, arg2: c_float, arg3: c_float);
  // proto:  void QMatrix4x4::translate(const QVector3D & vector);
  fn _ZN10QMatrix4x49translateERK9QVector3D(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  double QMatrix4x4::determinant();
  fn _ZNK10QMatrix4x411determinantEv(qthis: *mut c_void) -> c_double;
  // proto:  void QMatrix4x4::scale(float x, float y, float z);
  fn _ZN10QMatrix4x45scaleEfff(qthis: *mut c_void, arg0: c_float, arg1: c_float, arg2: c_float);
  // proto:  void QMatrix4x4::frustum(float left, float right, float bottom, float top, float nearPlane, float farPlane);
  fn _ZN10QMatrix4x47frustumEffffff(qthis: *mut c_void, arg0: c_float, arg1: c_float, arg2: c_float, arg3: c_float, arg4: c_float, arg5: c_float);
  // proto:  QPoint QMatrix4x4::map(const QPoint & point);
  fn _ZNK10QMatrix4x43mapERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QMatrix4x4::QMatrix4x4(int );
  fn _ZN10QMatrix4x4C1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QMatrix4x4::optimize();
  fn _ZN10QMatrix4x48optimizeEv(qthis: *mut c_void);
  // proto:  void QMatrix4x4::QMatrix4x4(const float * values);
  fn _ZN10QMatrix4x4C1EPKf(qthis: *mut c_void, arg0: *mut c_float);
  // proto:  void QMatrix4x4::translate(float x, float y);
  fn _ZN10QMatrix4x49translateEff(qthis: *mut c_void, arg0: c_float, arg1: c_float);
  // proto:  void QMatrix4x4::setToIdentity();
  fn _ZN10QMatrix4x413setToIdentityEv(qthis: *mut c_void);
  // proto:  QRect QMatrix4x4::mapRect(const QRect & rect);
  fn _ZNK10QMatrix4x47mapRectERK5QRect(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QMatrix4x4::scale(float x, float y);
  fn _ZN10QMatrix4x45scaleEff(qthis: *mut c_void, arg0: c_float, arg1: c_float);
  // proto:  void QMatrix4x4::QMatrix4x4(float m11, float m12, float m13, float m14, float m21, float m22, float m23, float m24, float m31, float m32, float m33, float m34, float m41, float m42, float m43, float m44);
  fn _ZN10QMatrix4x4C1Effffffffffffffff(qthis: *mut c_void, arg0: c_float, arg1: c_float, arg2: c_float, arg3: c_float, arg4: c_float, arg5: c_float, arg6: c_float, arg7: c_float, arg8: c_float, arg9: c_float, arg10: c_float, arg11: c_float, arg12: c_float, arg13: c_float, arg14: c_float, arg15: c_float);
  // proto:  QVector3D QMatrix4x4::map(const QVector3D & point);
  fn _ZNK10QMatrix4x43mapERK9QVector3D(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QMatrix4x4::lookAt(const QVector3D & eye, const QVector3D & center, const QVector3D & up);
  fn _ZN10QMatrix4x46lookAtERK9QVector3DS2_S2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QMatrix4x4::ortho(const QRectF & rect);
  fn _ZN10QMatrix4x45orthoERK6QRectF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QMatrix4x4::viewport(const QRectF & rect);
  fn _ZN10QMatrix4x48viewportERK6QRectF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QMatrix4x4::rotate(float angle, float x, float y, float z);
  fn _ZN10QMatrix4x46rotateEffff(qthis: *mut c_void, arg0: c_float, arg1: c_float, arg2: c_float, arg3: c_float);
  // proto:  void QMatrix4x4::fill(float value);
  fn _ZN10QMatrix4x44fillEf(qthis: *mut c_void, arg0: c_float);
  // proto:  void QMatrix4x4::QMatrix4x4(const float * values, int cols, int rows);
  fn _ZN10QMatrix4x4C1EPKfii(qthis: *mut c_void, arg0: *mut c_float, arg1: c_int, arg2: c_int);
  // proto:  QTransform QMatrix4x4::toTransform(float distanceToPlane);
  fn _ZNK10QMatrix4x411toTransformEf(qthis: *mut c_void, arg0: c_float) -> *mut c_void;
  // proto:  QMatrix4x4 QMatrix4x4::transposed();
  fn _ZNK10QMatrix4x410transposedEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPointF QMatrix4x4::map(const QPointF & point);
  fn _ZNK10QMatrix4x43mapERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QMatrix4x4::scale(float factor);
  fn _ZN10QMatrix4x45scaleEf(qthis: *mut c_void, arg0: c_float);
  // proto:  QVector4D QMatrix4x4::row(int index);
  fn _ZNK10QMatrix4x43rowEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QMatrix4x4::rotate(float angle, const QVector3D & vector);
  fn _ZN10QMatrix4x46rotateEfRK9QVector3D(qthis: *mut c_void, arg0: c_float, arg1: *mut c_void);
  // proto:  QVector4D QMatrix4x4::map(const QVector4D & point);
  fn _ZNK10QMatrix4x43mapERK9QVector4D(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QMatrix4x4)=68
pub struct QMatrix4x4 {
  pub qclsinst: *mut c_void,
}

  // proto:  QTransform QMatrix4x4::toTransform();
impl /*struct*/ QMatrix4x4 {
  pub fn toTransform<RetType, T: QMatrix4x4_toTransform<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toTransform(self);
    // return 1;
  }
}

pub trait QMatrix4x4_toTransform<RetType> {
  fn toTransform(self , rsthis: &mut QMatrix4x4) -> RetType;
}

  // proto:  QTransform QMatrix4x4::toTransform();
impl<'a> /*trait*/ QMatrix4x4_toTransform<QTransform> for () {
  fn toTransform(self , rsthis: &mut QMatrix4x4) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZNK10QMatrix4x411toTransformEv()};
    let mut ret = unsafe {_ZNK10QMatrix4x411toTransformEv(rsthis.qclsinst)};
    let mut ret1 = QTransform{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QMatrix4x4::scale(const QVector3D & vector);
impl /*struct*/ QMatrix4x4 {
  pub fn scale<RetType, T: QMatrix4x4_scale<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.scale(self);
    // return 1;
  }
}

pub trait QMatrix4x4_scale<RetType> {
  fn scale(self , rsthis: &mut QMatrix4x4) -> RetType;
}

  // proto:  void QMatrix4x4::scale(const QVector3D & vector);
impl<'a> /*trait*/ QMatrix4x4_scale<()> for (QVector3D) {
  fn scale(self , rsthis: &mut QMatrix4x4) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x45scaleERK9QVector3D()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QMatrix4x45scaleERK9QVector3D(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMatrix4x4::translate(float x, float y, float z);
impl /*struct*/ QMatrix4x4 {
  pub fn translate<RetType, T: QMatrix4x4_translate<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.translate(self);
    // return 1;
  }
}

pub trait QMatrix4x4_translate<RetType> {
  fn translate(self , rsthis: &mut QMatrix4x4) -> RetType;
}

  // proto:  void QMatrix4x4::translate(float x, float y, float z);
impl<'a> /*trait*/ QMatrix4x4_translate<()> for (f32, f32, f32) {
  fn translate(self , rsthis: &mut QMatrix4x4) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x49translateEfff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
     unsafe {_ZN10QMatrix4x49translateEfff(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  const float * QMatrix4x4::constData();
impl /*struct*/ QMatrix4x4 {
  pub fn constData<RetType, T: QMatrix4x4_constData<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.constData(self);
    // return 1;
  }
}

pub trait QMatrix4x4_constData<RetType> {
  fn constData(self , rsthis: &mut QMatrix4x4) -> RetType;
}

  // proto:  const float * QMatrix4x4::constData();
impl<'a> /*trait*/ QMatrix4x4_constData<*mut f32> for () {
  fn constData(self , rsthis: &mut QMatrix4x4) -> *mut f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZNK10QMatrix4x49constDataEv()};
    let mut ret = unsafe {_ZNK10QMatrix4x49constDataEv(rsthis.qclsinst)};
    return ret as *mut f32;
    // return 1;
  }
}

  // proto:  float * QMatrix4x4::data();
impl /*struct*/ QMatrix4x4 {
  pub fn data<RetType, T: QMatrix4x4_data<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QMatrix4x4_data<RetType> {
  fn data(self , rsthis: &mut QMatrix4x4) -> RetType;
}

  // proto:  float * QMatrix4x4::data();
impl<'a> /*trait*/ QMatrix4x4_data<*mut f32> for () {
  fn data(self , rsthis: &mut QMatrix4x4) -> *mut f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x44dataEv()};
    let mut ret = unsafe {_ZN10QMatrix4x44dataEv(rsthis.qclsinst)};
    return ret as *mut f32;
    // return 1;
  }
}

  // proto:  QMatrix4x4 QMatrix4x4::inverted(bool * invertible);
impl /*struct*/ QMatrix4x4 {
  pub fn inverted<RetType, T: QMatrix4x4_inverted<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.inverted(self);
    // return 1;
  }
}

pub trait QMatrix4x4_inverted<RetType> {
  fn inverted(self , rsthis: &mut QMatrix4x4) -> RetType;
}

  // proto:  QMatrix4x4 QMatrix4x4::inverted(bool * invertible);
impl<'a> /*trait*/ QMatrix4x4_inverted<QMatrix4x4> for (&'a mut Vec<i8>) {
  fn inverted(self , rsthis: &mut QMatrix4x4) -> QMatrix4x4 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZNK10QMatrix4x48invertedEPb()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZNK10QMatrix4x48invertedEPb(rsthis.qclsinst, arg0)};
    let mut ret1 = QMatrix4x4{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QVector3D QMatrix4x4::mapVector(const QVector3D & vector);
impl /*struct*/ QMatrix4x4 {
  pub fn mapVector<RetType, T: QMatrix4x4_mapVector<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mapVector(self);
    // return 1;
  }
}

pub trait QMatrix4x4_mapVector<RetType> {
  fn mapVector(self , rsthis: &mut QMatrix4x4) -> RetType;
}

  // proto:  QVector3D QMatrix4x4::mapVector(const QVector3D & vector);
impl<'a> /*trait*/ QMatrix4x4_mapVector<QVector3D> for (QVector3D) {
  fn mapVector(self , rsthis: &mut QMatrix4x4) -> QVector3D {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZNK10QMatrix4x49mapVectorERK9QVector3D()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QMatrix4x49mapVectorERK9QVector3D(rsthis.qclsinst, arg0)};
    let mut ret1 = QVector3D{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QMatrix4x4::ortho(float left, float right, float bottom, float top, float nearPlane, float farPlane);
impl /*struct*/ QMatrix4x4 {
  pub fn ortho<RetType, T: QMatrix4x4_ortho<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.ortho(self);
    // return 1;
  }
}

pub trait QMatrix4x4_ortho<RetType> {
  fn ortho(self , rsthis: &mut QMatrix4x4) -> RetType;
}

  // proto:  void QMatrix4x4::ortho(float left, float right, float bottom, float top, float nearPlane, float farPlane);
impl<'a> /*trait*/ QMatrix4x4_ortho<()> for (f32, f32, f32, f32, f32, f32) {
  fn ortho(self , rsthis: &mut QMatrix4x4) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x45orthoEffffff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
    let arg4 = self.4  as c_float;
    let arg5 = self.5  as c_float;
     unsafe {_ZN10QMatrix4x45orthoEffffff(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5)};
    // return 1;
  }
}

  // proto:  void QMatrix4x4::QMatrix4x4();
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

  // proto:  void QMatrix4x4::QMatrix4x4();
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

  // proto:  QMatrix QMatrix4x4::toAffine();
impl /*struct*/ QMatrix4x4 {
  pub fn toAffine<RetType, T: QMatrix4x4_toAffine<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toAffine(self);
    // return 1;
  }
}

pub trait QMatrix4x4_toAffine<RetType> {
  fn toAffine(self , rsthis: &mut QMatrix4x4) -> RetType;
}

  // proto:  QMatrix QMatrix4x4::toAffine();
impl<'a> /*trait*/ QMatrix4x4_toAffine<QMatrix> for () {
  fn toAffine(self , rsthis: &mut QMatrix4x4) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZNK10QMatrix4x48toAffineEv()};
    let mut ret = unsafe {_ZNK10QMatrix4x48toAffineEv(rsthis.qclsinst)};
    let mut ret1 = QMatrix{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QRectF QMatrix4x4::mapRect(const QRectF & rect);
impl /*struct*/ QMatrix4x4 {
  pub fn mapRect<RetType, T: QMatrix4x4_mapRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mapRect(self);
    // return 1;
  }
}

pub trait QMatrix4x4_mapRect<RetType> {
  fn mapRect(self , rsthis: &mut QMatrix4x4) -> RetType;
}

  // proto:  QRectF QMatrix4x4::mapRect(const QRectF & rect);
impl<'a> /*trait*/ QMatrix4x4_mapRect<QRectF> for (QRectF) {
  fn mapRect(self , rsthis: &mut QMatrix4x4) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZNK10QMatrix4x47mapRectERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QMatrix4x47mapRectERK6QRectF(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QMatrix4x4::setColumn(int index, const QVector4D & value);
impl /*struct*/ QMatrix4x4 {
  pub fn setColumn<RetType, T: QMatrix4x4_setColumn<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setColumn(self);
    // return 1;
  }
}

pub trait QMatrix4x4_setColumn<RetType> {
  fn setColumn(self , rsthis: &mut QMatrix4x4) -> RetType;
}

  // proto:  void QMatrix4x4::setColumn(int index, const QVector4D & value);
impl<'a> /*trait*/ QMatrix4x4_setColumn<()> for (i32, QVector4D) {
  fn setColumn(self , rsthis: &mut QMatrix4x4) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x49setColumnEiRK9QVector4D()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN10QMatrix4x49setColumnEiRK9QVector4D(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QMatrix4x4::isIdentity();
impl /*struct*/ QMatrix4x4 {
  pub fn isIdentity<RetType, T: QMatrix4x4_isIdentity<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isIdentity(self);
    // return 1;
  }
}

pub trait QMatrix4x4_isIdentity<RetType> {
  fn isIdentity(self , rsthis: &mut QMatrix4x4) -> RetType;
}

  // proto:  bool QMatrix4x4::isIdentity();
impl<'a> /*trait*/ QMatrix4x4_isIdentity<i8> for () {
  fn isIdentity(self , rsthis: &mut QMatrix4x4) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZNK10QMatrix4x410isIdentityEv()};
    let mut ret = unsafe {_ZNK10QMatrix4x410isIdentityEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QVector4D QMatrix4x4::column(int index);
impl /*struct*/ QMatrix4x4 {
  pub fn column<RetType, T: QMatrix4x4_column<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.column(self);
    // return 1;
  }
}

pub trait QMatrix4x4_column<RetType> {
  fn column(self , rsthis: &mut QMatrix4x4) -> RetType;
}

  // proto:  QVector4D QMatrix4x4::column(int index);
impl<'a> /*trait*/ QMatrix4x4_column<QVector4D> for (i32) {
  fn column(self , rsthis: &mut QMatrix4x4) -> QVector4D {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZNK10QMatrix4x46columnEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QMatrix4x46columnEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QVector4D{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QMatrix4x4::setRow(int index, const QVector4D & value);
impl /*struct*/ QMatrix4x4 {
  pub fn setRow<RetType, T: QMatrix4x4_setRow<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setRow(self);
    // return 1;
  }
}

pub trait QMatrix4x4_setRow<RetType> {
  fn setRow(self , rsthis: &mut QMatrix4x4) -> RetType;
}

  // proto:  void QMatrix4x4::setRow(int index, const QVector4D & value);
impl<'a> /*trait*/ QMatrix4x4_setRow<()> for (i32, QVector4D) {
  fn setRow(self , rsthis: &mut QMatrix4x4) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x46setRowEiRK9QVector4D()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN10QMatrix4x46setRowEiRK9QVector4D(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QMatrix4x4::flipCoordinates();
impl /*struct*/ QMatrix4x4 {
  pub fn flipCoordinates<RetType, T: QMatrix4x4_flipCoordinates<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.flipCoordinates(self);
    // return 1;
  }
}

pub trait QMatrix4x4_flipCoordinates<RetType> {
  fn flipCoordinates(self , rsthis: &mut QMatrix4x4) -> RetType;
}

  // proto:  void QMatrix4x4::flipCoordinates();
impl<'a> /*trait*/ QMatrix4x4_flipCoordinates<()> for () {
  fn flipCoordinates(self , rsthis: &mut QMatrix4x4) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x415flipCoordinatesEv()};
     unsafe {_ZN10QMatrix4x415flipCoordinatesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QMatrix3x3 QMatrix4x4::normalMatrix();
impl /*struct*/ QMatrix4x4 {
  pub fn normalMatrix<RetType, T: QMatrix4x4_normalMatrix<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.normalMatrix(self);
    // return 1;
  }
}

pub trait QMatrix4x4_normalMatrix<RetType> {
  fn normalMatrix(self , rsthis: &mut QMatrix4x4) -> RetType;
}

  // proto:  QMatrix3x3 QMatrix4x4::normalMatrix();
impl<'a> /*trait*/ QMatrix4x4_normalMatrix<()> for () {
  fn normalMatrix(self , rsthis: &mut QMatrix4x4) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZNK10QMatrix4x412normalMatrixEv()};
     unsafe {_ZNK10QMatrix4x412normalMatrixEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QMatrix4x4::viewport(float left, float bottom, float width, float height, float nearPlane, float farPlane);
impl /*struct*/ QMatrix4x4 {
  pub fn viewport<RetType, T: QMatrix4x4_viewport<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.viewport(self);
    // return 1;
  }
}

pub trait QMatrix4x4_viewport<RetType> {
  fn viewport(self , rsthis: &mut QMatrix4x4) -> RetType;
}

  // proto:  void QMatrix4x4::viewport(float left, float bottom, float width, float height, float nearPlane, float farPlane);
impl<'a> /*trait*/ QMatrix4x4_viewport<()> for (f32, f32, f32, f32, f32, f32) {
  fn viewport(self , rsthis: &mut QMatrix4x4) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x48viewportEffffff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
    let arg4 = self.4  as c_float;
    let arg5 = self.5  as c_float;
     unsafe {_ZN10QMatrix4x48viewportEffffff(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5)};
    // return 1;
  }
}

  // proto:  void QMatrix4x4::copyDataTo(float * values);
impl /*struct*/ QMatrix4x4 {
  pub fn copyDataTo<RetType, T: QMatrix4x4_copyDataTo<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.copyDataTo(self);
    // return 1;
  }
}

pub trait QMatrix4x4_copyDataTo<RetType> {
  fn copyDataTo(self , rsthis: &mut QMatrix4x4) -> RetType;
}

  // proto:  void QMatrix4x4::copyDataTo(float * values);
impl<'a> /*trait*/ QMatrix4x4_copyDataTo<()> for (&'a mut Vec<f32>) {
  fn copyDataTo(self , rsthis: &mut QMatrix4x4) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZNK10QMatrix4x410copyDataToEPf()};
    let arg0 = self.as_ptr()  as *mut c_float;
     unsafe {_ZNK10QMatrix4x410copyDataToEPf(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMatrix4x4::QMatrix4x4(const QTransform & transform);
impl<'a> /*trait*/ QMatrix4x4_NewQMatrix4x4 for (QTransform) {
  fn NewQMatrix4x4(self) -> QMatrix4x4 {
    let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x4C1ERK10QTransform()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QMatrix4x4C1ERK10QTransform(qthis, arg0)};
    let rsthis = QMatrix4x4{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QMatrix4x4::isAffine();
impl /*struct*/ QMatrix4x4 {
  pub fn isAffine<RetType, T: QMatrix4x4_isAffine<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isAffine(self);
    // return 1;
  }
}

pub trait QMatrix4x4_isAffine<RetType> {
  fn isAffine(self , rsthis: &mut QMatrix4x4) -> RetType;
}

  // proto:  bool QMatrix4x4::isAffine();
impl<'a> /*trait*/ QMatrix4x4_isAffine<i8> for () {
  fn isAffine(self , rsthis: &mut QMatrix4x4) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZNK10QMatrix4x48isAffineEv()};
    let mut ret = unsafe {_ZNK10QMatrix4x48isAffineEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QMatrix4x4::ortho(const QRect & rect);
impl<'a> /*trait*/ QMatrix4x4_ortho<()> for (QRect) {
  fn ortho(self , rsthis: &mut QMatrix4x4) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x45orthoERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QMatrix4x45orthoERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMatrix4x4::rotate(const QQuaternion & quaternion);
impl /*struct*/ QMatrix4x4 {
  pub fn rotate<RetType, T: QMatrix4x4_rotate<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rotate(self);
    // return 1;
  }
}

pub trait QMatrix4x4_rotate<RetType> {
  fn rotate(self , rsthis: &mut QMatrix4x4) -> RetType;
}

  // proto:  void QMatrix4x4::rotate(const QQuaternion & quaternion);
impl<'a> /*trait*/ QMatrix4x4_rotate<()> for (QQuaternion) {
  fn rotate(self , rsthis: &mut QMatrix4x4) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x46rotateERK11QQuaternion()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QMatrix4x46rotateERK11QQuaternion(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMatrix4x4::QMatrix4x4(const QMatrix & matrix);
impl<'a> /*trait*/ QMatrix4x4_NewQMatrix4x4 for (QMatrix) {
  fn NewQMatrix4x4(self) -> QMatrix4x4 {
    let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x4C1ERK7QMatrix()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QMatrix4x4C1ERK7QMatrix(qthis, arg0)};
    let rsthis = QMatrix4x4{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QMatrix4x4::perspective(float verticalAngle, float aspectRatio, float nearPlane, float farPlane);
impl /*struct*/ QMatrix4x4 {
  pub fn perspective<RetType, T: QMatrix4x4_perspective<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.perspective(self);
    // return 1;
  }
}

pub trait QMatrix4x4_perspective<RetType> {
  fn perspective(self , rsthis: &mut QMatrix4x4) -> RetType;
}

  // proto:  void QMatrix4x4::perspective(float verticalAngle, float aspectRatio, float nearPlane, float farPlane);
impl<'a> /*trait*/ QMatrix4x4_perspective<()> for (f32, f32, f32, f32) {
  fn perspective(self , rsthis: &mut QMatrix4x4) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x411perspectiveEffff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
     unsafe {_ZN10QMatrix4x411perspectiveEffff(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QMatrix4x4::translate(const QVector3D & vector);
impl<'a> /*trait*/ QMatrix4x4_translate<()> for (QVector3D) {
  fn translate(self , rsthis: &mut QMatrix4x4) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x49translateERK9QVector3D()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QMatrix4x49translateERK9QVector3D(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  double QMatrix4x4::determinant();
impl /*struct*/ QMatrix4x4 {
  pub fn determinant<RetType, T: QMatrix4x4_determinant<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.determinant(self);
    // return 1;
  }
}

pub trait QMatrix4x4_determinant<RetType> {
  fn determinant(self , rsthis: &mut QMatrix4x4) -> RetType;
}

  // proto:  double QMatrix4x4::determinant();
impl<'a> /*trait*/ QMatrix4x4_determinant<f64> for () {
  fn determinant(self , rsthis: &mut QMatrix4x4) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZNK10QMatrix4x411determinantEv()};
    let mut ret = unsafe {_ZNK10QMatrix4x411determinantEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QMatrix4x4::scale(float x, float y, float z);
impl<'a> /*trait*/ QMatrix4x4_scale<()> for (f32, f32, f32) {
  fn scale(self , rsthis: &mut QMatrix4x4) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x45scaleEfff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
     unsafe {_ZN10QMatrix4x45scaleEfff(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QMatrix4x4::frustum(float left, float right, float bottom, float top, float nearPlane, float farPlane);
impl /*struct*/ QMatrix4x4 {
  pub fn frustum<RetType, T: QMatrix4x4_frustum<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.frustum(self);
    // return 1;
  }
}

pub trait QMatrix4x4_frustum<RetType> {
  fn frustum(self , rsthis: &mut QMatrix4x4) -> RetType;
}

  // proto:  void QMatrix4x4::frustum(float left, float right, float bottom, float top, float nearPlane, float farPlane);
impl<'a> /*trait*/ QMatrix4x4_frustum<()> for (f32, f32, f32, f32, f32, f32) {
  fn frustum(self , rsthis: &mut QMatrix4x4) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x47frustumEffffff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
    let arg4 = self.4  as c_float;
    let arg5 = self.5  as c_float;
     unsafe {_ZN10QMatrix4x47frustumEffffff(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5)};
    // return 1;
  }
}

  // proto:  QPoint QMatrix4x4::map(const QPoint & point);
impl /*struct*/ QMatrix4x4 {
  pub fn map<RetType, T: QMatrix4x4_map<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.map(self);
    // return 1;
  }
}

pub trait QMatrix4x4_map<RetType> {
  fn map(self , rsthis: &mut QMatrix4x4) -> RetType;
}

  // proto:  QPoint QMatrix4x4::map(const QPoint & point);
impl<'a> /*trait*/ QMatrix4x4_map<QPoint> for (QPoint) {
  fn map(self , rsthis: &mut QMatrix4x4) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZNK10QMatrix4x43mapERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QMatrix4x43mapERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QMatrix4x4::QMatrix4x4(int );
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

  // proto:  void QMatrix4x4::optimize();
impl /*struct*/ QMatrix4x4 {
  pub fn optimize<RetType, T: QMatrix4x4_optimize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.optimize(self);
    // return 1;
  }
}

pub trait QMatrix4x4_optimize<RetType> {
  fn optimize(self , rsthis: &mut QMatrix4x4) -> RetType;
}

  // proto:  void QMatrix4x4::optimize();
impl<'a> /*trait*/ QMatrix4x4_optimize<()> for () {
  fn optimize(self , rsthis: &mut QMatrix4x4) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x48optimizeEv()};
     unsafe {_ZN10QMatrix4x48optimizeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QMatrix4x4::QMatrix4x4(const float * values);
impl<'a> /*trait*/ QMatrix4x4_NewQMatrix4x4 for (&'a  Vec<f32>) {
  fn NewQMatrix4x4(self) -> QMatrix4x4 {
    let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x4C1EPKf()};
    let arg0 = self.as_ptr()  as *mut c_float;
    unsafe {_ZN10QMatrix4x4C1EPKf(qthis, arg0)};
    let rsthis = QMatrix4x4{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QMatrix4x4::translate(float x, float y);
impl<'a> /*trait*/ QMatrix4x4_translate<()> for (f32, f32) {
  fn translate(self , rsthis: &mut QMatrix4x4) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x49translateEff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
     unsafe {_ZN10QMatrix4x49translateEff(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QMatrix4x4::setToIdentity();
impl /*struct*/ QMatrix4x4 {
  pub fn setToIdentity<RetType, T: QMatrix4x4_setToIdentity<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setToIdentity(self);
    // return 1;
  }
}

pub trait QMatrix4x4_setToIdentity<RetType> {
  fn setToIdentity(self , rsthis: &mut QMatrix4x4) -> RetType;
}

  // proto:  void QMatrix4x4::setToIdentity();
impl<'a> /*trait*/ QMatrix4x4_setToIdentity<()> for () {
  fn setToIdentity(self , rsthis: &mut QMatrix4x4) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x413setToIdentityEv()};
     unsafe {_ZN10QMatrix4x413setToIdentityEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QRect QMatrix4x4::mapRect(const QRect & rect);
impl<'a> /*trait*/ QMatrix4x4_mapRect<QRect> for (QRect) {
  fn mapRect(self , rsthis: &mut QMatrix4x4) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZNK10QMatrix4x47mapRectERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QMatrix4x47mapRectERK5QRect(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QMatrix4x4::scale(float x, float y);
impl<'a> /*trait*/ QMatrix4x4_scale<()> for (f32, f32) {
  fn scale(self , rsthis: &mut QMatrix4x4) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x45scaleEff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
     unsafe {_ZN10QMatrix4x45scaleEff(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QMatrix4x4::QMatrix4x4(float m11, float m12, float m13, float m14, float m21, float m22, float m23, float m24, float m31, float m32, float m33, float m34, float m41, float m42, float m43, float m44);
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

  // proto:  QVector3D QMatrix4x4::map(const QVector3D & point);
impl<'a> /*trait*/ QMatrix4x4_map<QVector3D> for (QVector3D) {
  fn map(self , rsthis: &mut QMatrix4x4) -> QVector3D {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZNK10QMatrix4x43mapERK9QVector3D()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QMatrix4x43mapERK9QVector3D(rsthis.qclsinst, arg0)};
    let mut ret1 = QVector3D{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QMatrix4x4::lookAt(const QVector3D & eye, const QVector3D & center, const QVector3D & up);
impl /*struct*/ QMatrix4x4 {
  pub fn lookAt<RetType, T: QMatrix4x4_lookAt<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.lookAt(self);
    // return 1;
  }
}

pub trait QMatrix4x4_lookAt<RetType> {
  fn lookAt(self , rsthis: &mut QMatrix4x4) -> RetType;
}

  // proto:  void QMatrix4x4::lookAt(const QVector3D & eye, const QVector3D & center, const QVector3D & up);
impl<'a> /*trait*/ QMatrix4x4_lookAt<()> for (QVector3D, QVector3D, QVector3D) {
  fn lookAt(self , rsthis: &mut QMatrix4x4) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x46lookAtERK9QVector3DS2_S2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN10QMatrix4x46lookAtERK9QVector3DS2_S2_(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QMatrix4x4::ortho(const QRectF & rect);
impl<'a> /*trait*/ QMatrix4x4_ortho<()> for (QRectF) {
  fn ortho(self , rsthis: &mut QMatrix4x4) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x45orthoERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QMatrix4x45orthoERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMatrix4x4::viewport(const QRectF & rect);
impl<'a> /*trait*/ QMatrix4x4_viewport<()> for (QRectF) {
  fn viewport(self , rsthis: &mut QMatrix4x4) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x48viewportERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QMatrix4x48viewportERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMatrix4x4::rotate(float angle, float x, float y, float z);
impl<'a> /*trait*/ QMatrix4x4_rotate<()> for (f32, f32, f32, f32) {
  fn rotate(self , rsthis: &mut QMatrix4x4) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x46rotateEffff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
     unsafe {_ZN10QMatrix4x46rotateEffff(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QMatrix4x4::fill(float value);
impl /*struct*/ QMatrix4x4 {
  pub fn fill<RetType, T: QMatrix4x4_fill<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.fill(self);
    // return 1;
  }
}

pub trait QMatrix4x4_fill<RetType> {
  fn fill(self , rsthis: &mut QMatrix4x4) -> RetType;
}

  // proto:  void QMatrix4x4::fill(float value);
impl<'a> /*trait*/ QMatrix4x4_fill<()> for (f32) {
  fn fill(self , rsthis: &mut QMatrix4x4) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x44fillEf()};
    let arg0 = self  as c_float;
     unsafe {_ZN10QMatrix4x44fillEf(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMatrix4x4::QMatrix4x4(const float * values, int cols, int rows);
impl<'a> /*trait*/ QMatrix4x4_NewQMatrix4x4 for (&'a  Vec<f32>, i32, i32) {
  fn NewQMatrix4x4(self) -> QMatrix4x4 {
    let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x4C1EPKfii()};
    let arg0 = self.0.as_ptr()  as *mut c_float;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZN10QMatrix4x4C1EPKfii(qthis, arg0, arg1, arg2)};
    let rsthis = QMatrix4x4{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QTransform QMatrix4x4::toTransform(float distanceToPlane);
impl<'a> /*trait*/ QMatrix4x4_toTransform<QTransform> for (f32) {
  fn toTransform(self , rsthis: &mut QMatrix4x4) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZNK10QMatrix4x411toTransformEf()};
    let arg0 = self  as c_float;
    let mut ret = unsafe {_ZNK10QMatrix4x411toTransformEf(rsthis.qclsinst, arg0)};
    let mut ret1 = QTransform{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QMatrix4x4 QMatrix4x4::transposed();
impl /*struct*/ QMatrix4x4 {
  pub fn transposed<RetType, T: QMatrix4x4_transposed<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.transposed(self);
    // return 1;
  }
}

pub trait QMatrix4x4_transposed<RetType> {
  fn transposed(self , rsthis: &mut QMatrix4x4) -> RetType;
}

  // proto:  QMatrix4x4 QMatrix4x4::transposed();
impl<'a> /*trait*/ QMatrix4x4_transposed<QMatrix4x4> for () {
  fn transposed(self , rsthis: &mut QMatrix4x4) -> QMatrix4x4 {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZNK10QMatrix4x410transposedEv()};
    let mut ret = unsafe {_ZNK10QMatrix4x410transposedEv(rsthis.qclsinst)};
    let mut ret1 = QMatrix4x4{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QPointF QMatrix4x4::map(const QPointF & point);
impl<'a> /*trait*/ QMatrix4x4_map<QPointF> for (QPointF) {
  fn map(self , rsthis: &mut QMatrix4x4) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZNK10QMatrix4x43mapERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QMatrix4x43mapERK7QPointF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QMatrix4x4::scale(float factor);
impl<'a> /*trait*/ QMatrix4x4_scale<()> for (f32) {
  fn scale(self , rsthis: &mut QMatrix4x4) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x45scaleEf()};
    let arg0 = self  as c_float;
     unsafe {_ZN10QMatrix4x45scaleEf(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QVector4D QMatrix4x4::row(int index);
impl /*struct*/ QMatrix4x4 {
  pub fn row<RetType, T: QMatrix4x4_row<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.row(self);
    // return 1;
  }
}

pub trait QMatrix4x4_row<RetType> {
  fn row(self , rsthis: &mut QMatrix4x4) -> RetType;
}

  // proto:  QVector4D QMatrix4x4::row(int index);
impl<'a> /*trait*/ QMatrix4x4_row<QVector4D> for (i32) {
  fn row(self , rsthis: &mut QMatrix4x4) -> QVector4D {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZNK10QMatrix4x43rowEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QMatrix4x43rowEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QVector4D{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QMatrix4x4::rotate(float angle, const QVector3D & vector);
impl<'a> /*trait*/ QMatrix4x4_rotate<()> for (f32, QVector3D) {
  fn rotate(self , rsthis: &mut QMatrix4x4) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZN10QMatrix4x46rotateEfRK9QVector3D()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN10QMatrix4x46rotateEfRK9QVector3D(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QVector4D QMatrix4x4::map(const QVector4D & point);
impl<'a> /*trait*/ QMatrix4x4_map<QVector4D> for (QVector4D) {
  fn map(self , rsthis: &mut QMatrix4x4) -> QVector4D {
    // let qthis: *mut c_void = unsafe{calloc(1, 68)};
    // unsafe{_ZNK10QMatrix4x43mapERK9QVector4D()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QMatrix4x43mapERK9QVector4D(rsthis.qclsinst, arg0)};
    let mut ret1 = QVector4D{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

