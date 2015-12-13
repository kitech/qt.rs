// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qvector3d::QVector3D;
use super::qvector4d::QVector4D;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QQuaternion::getAxisAndAngle(float * x, float * y, float * z, float * angle);
  fn _ZNK11QQuaternion15getAxisAndAngleEPfS0_S0_S0_(arg0: *mut c_float, arg1: *mut c_float, arg2: *mut c_float, arg3: *mut c_float) -> i32;
  // proto: float QQuaternion::scalar();
  fn _ZNK11QQuaternion6scalarEv() -> i32;
  // proto: void QQuaternion::setX(float x);
  fn _ZN11QQuaternion4setXEf(arg0: c_float) -> i32;
  // proto: void QQuaternion::setVector(const QVector3D & vector);
  fn _ZN11QQuaternion9setVectorERK9QVector3D(arg0: *const c_void) -> i32;
  // proto: void QQuaternion::NewQQuaternion(const QVector4D & vector);
  fn _ZN11QQuaternionC1ERK9QVector4D(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: QQuaternion QQuaternion::rotationTo(const QVector3D & from, const QVector3D & to);
  fn _ZN11QQuaternion10rotationToERK9QVector3DS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QQuaternion::getEulerAngles(float * pitch, float * yaw, float * roll);
  fn _ZNK11QQuaternion14getEulerAnglesEPfS0_S0_(arg0: *mut c_float, arg1: *mut c_float, arg2: *mut c_float) -> i32;
  // proto: void QQuaternion::setY(float y);
  fn _ZN11QQuaternion4setYEf(arg0: c_float) -> i32;
  // proto: QVector3D QQuaternion::toEulerAngles();
  fn _ZNK11QQuaternion13toEulerAnglesEv() -> i32;
  // proto: QQuaternion QQuaternion::inverted();
  fn _ZNK11QQuaternion8invertedEv() -> i32;
  // proto: void QQuaternion::setZ(float z);
  fn _ZN11QQuaternion4setZEf(arg0: c_float) -> i32;
  // proto: void QQuaternion::getAxes(QVector3D * xAxis, QVector3D * yAxis, QVector3D * zAxis);
  fn _ZNK11QQuaternion7getAxesEP9QVector3DS1_S1_(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> i32;
  // proto: QQuaternion QQuaternion::nlerp(const QQuaternion & q1, const QQuaternion & q2, float t);
  fn _ZN11QQuaternion5nlerpERKS_S1_f(arg0: *const c_void, arg1: *const c_void, arg2: c_float) -> i32;
  // proto: bool QQuaternion::isIdentity();
  fn _ZNK11QQuaternion10isIdentityEv() -> i32;
  // proto: QQuaternion QQuaternion::fromAxes(const QVector3D & xAxis, const QVector3D & yAxis, const QVector3D & zAxis);
  fn _ZN11QQuaternion8fromAxesERK9QVector3DS2_S2_(arg0: *const c_void, arg1: *const c_void, arg2: *const c_void) -> i32;
  // proto: QQuaternion QQuaternion::slerp(const QQuaternion & q1, const QQuaternion & q2, float t);
  fn _ZN11QQuaternion5slerpERKS_S1_f(arg0: *const c_void, arg1: *const c_void, arg2: c_float) -> i32;
  // proto: QQuaternion QQuaternion::fromDirection(const QVector3D & direction, const QVector3D & up);
  fn _ZN11QQuaternion13fromDirectionERK9QVector3DS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QQuaternion::NewQQuaternion();
  fn _ZN11QQuaternionC1Ev(qthis: *mut c_void) -> i32;
  // proto: QQuaternion QQuaternion::normalized();
  fn _ZNK11QQuaternion10normalizedEv() -> i32;
  // proto: QVector4D QQuaternion::toVector4D();
  fn _ZNK11QQuaternion10toVector4DEv() -> i32;
  // proto: QQuaternion QQuaternion::fromEulerAngles(float pitch, float yaw, float roll);
  fn _ZN11QQuaternion15fromEulerAnglesEfff(arg0: c_float, arg1: c_float, arg2: c_float) -> i32;
  // proto: QQuaternion QQuaternion::conjugate();
  fn _ZNK11QQuaternion9conjugateEv() -> i32;
  // proto: bool QQuaternion::isNull();
  fn _ZNK11QQuaternion6isNullEv() -> i32;
  // proto: void QQuaternion::getAxisAndAngle(QVector3D * axis, float * angle);
  fn _ZNK11QQuaternion15getAxisAndAngleEP9QVector3DPf(arg0: *mut c_void, arg1: *mut c_float) -> i32;
  // proto: QGenericMatrix<3, 3, float> QQuaternion::toRotationMatrix();
  fn _ZNK11QQuaternion16toRotationMatrixEv() -> i32;
  // proto: QQuaternion QQuaternion::fromEulerAngles(const QVector3D & eulerAngles);
  fn _ZN11QQuaternion15fromEulerAnglesERK9QVector3D(arg0: *const c_void) -> i32;
  // proto: QVector3D QQuaternion::rotatedVector(const QVector3D & vector);
  fn _ZNK11QQuaternion13rotatedVectorERK9QVector3D(arg0: *const c_void) -> i32;
  // proto: float QQuaternion::lengthSquared();
  fn _ZNK11QQuaternion13lengthSquaredEv() -> i32;
  // proto: void QQuaternion::setScalar(float scalar);
  fn _ZN11QQuaternion9setScalarEf(arg0: c_float) -> i32;
  // proto: float QQuaternion::y();
  fn _ZNK11QQuaternion1yEv() -> i32;
  // proto: QVector3D QQuaternion::vector();
  fn _ZNK11QQuaternion6vectorEv() -> i32;
  // proto: float QQuaternion::dotProduct(const QQuaternion & q1, const QQuaternion & q2);
  fn _ZN11QQuaternion10dotProductERKS_S1_(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QQuaternion::setVector(float x, float y, float z);
  fn _ZN11QQuaternion9setVectorEfff(arg0: c_float, arg1: c_float, arg2: c_float) -> i32;
  // proto: void QQuaternion::NewQQuaternion(float scalar, float xpos, float ypos, float zpos);
  fn _ZN11QQuaternionC1Effff(qthis: *mut c_void, arg0: c_float, arg1: c_float, arg2: c_float, arg3: c_float) -> i32;
  // proto: QQuaternion QQuaternion::fromAxisAndAngle(const QVector3D & axis, float angle);
  fn _ZN11QQuaternion16fromAxisAndAngleERK9QVector3Df(arg0: *const c_void, arg1: c_float) -> i32;
  // proto: void QQuaternion::NewQQuaternion(float scalar, const QVector3D & vector);
  fn _ZN11QQuaternionC1EfRK9QVector3D(qthis: *mut c_void, arg0: c_float, arg1: *const c_void) -> i32;
  // proto: float QQuaternion::length();
  fn _ZNK11QQuaternion6lengthEv() -> i32;
  // proto: void QQuaternion::normalize();
  fn _ZN11QQuaternion9normalizeEv() -> i32;
  // proto: QQuaternion QQuaternion::conjugated();
  fn _ZNK11QQuaternion10conjugatedEv() -> i32;
  // proto: QQuaternion QQuaternion::fromAxisAndAngle(float x, float y, float z, float angle);
  fn _ZN11QQuaternion16fromAxisAndAngleEffff(arg0: c_float, arg1: c_float, arg2: c_float, arg3: c_float) -> i32;
  // proto: float QQuaternion::x();
  fn _ZNK11QQuaternion1xEv() -> i32;
  // proto: float QQuaternion::z();
  fn _ZNK11QQuaternion1zEv() -> i32;
}

// body block begin
// class sizeof(QQuaternion)=16
pub struct QQuaternion {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QQuaternion {
  pub fn getAxisAndAngle<T: QQuaternion_getAxisAndAngle>(&mut self, value: T) -> i32 {
    value.getAxisAndAngle(self);
    return 1;
  }
}

pub trait QQuaternion_getAxisAndAngle {
  fn getAxisAndAngle(self, this: &mut QQuaternion) -> i32;
}

// proto: void QQuaternion::getAxisAndAngle(float * x, float * y, float * z, float * angle);
impl<'a> /*trait*/ QQuaternion_getAxisAndAngle for (&'a mut f32, &'a mut f32, &'a mut f32, &'a mut f32) {
  fn getAxisAndAngle(self, this: &mut QQuaternion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QQuaternion15getAxisAndAngleEPfS0_S0_S0_()};
    let arg0 = self.0  as *mut c_float;
    let arg1 = self.1  as *mut c_float;
    let arg2 = self.2  as *mut c_float;
    let arg3 = self.3  as *mut c_float;
    unsafe {_ZNK11QQuaternion15getAxisAndAngleEPfS0_S0_S0_(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QQuaternion {
  pub fn scalar<T: QQuaternion_scalar>(&mut self, value: T) -> i32 {
    value.scalar(self);
    return 1;
  }
}

pub trait QQuaternion_scalar {
  fn scalar(self, this: &mut QQuaternion) -> i32;
}

// proto: float QQuaternion::scalar();
impl<'a> /*trait*/ QQuaternion_scalar for () {
  fn scalar(self, this: &mut QQuaternion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QQuaternion6scalarEv()};
    unsafe {_ZNK11QQuaternion6scalarEv()};
    return 1;
  }
}

impl /*struct*/ QQuaternion {
  pub fn setX<T: QQuaternion_setX>(&mut self, value: T) -> i32 {
    value.setX(self);
    return 1;
  }
}

pub trait QQuaternion_setX {
  fn setX(self, this: &mut QQuaternion) -> i32;
}

// proto: void QQuaternion::setX(float x);
impl<'a> /*trait*/ QQuaternion_setX for (f32) {
  fn setX(self, this: &mut QQuaternion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QQuaternion4setXEf()};
    let arg0 = self  as c_float;
    unsafe {_ZN11QQuaternion4setXEf(arg0)};
    return 1;
  }
}

impl /*struct*/ QQuaternion {
  pub fn setVector<T: QQuaternion_setVector>(&mut self, value: T) -> i32 {
    value.setVector(self);
    return 1;
  }
}

pub trait QQuaternion_setVector {
  fn setVector(self, this: &mut QQuaternion) -> i32;
}

// proto: void QQuaternion::setVector(const QVector3D & vector);
impl<'a> /*trait*/ QQuaternion_setVector for (&'a  QVector3D) {
  fn setVector(self, this: &mut QQuaternion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QQuaternion9setVectorERK9QVector3D()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QQuaternion9setVectorERK9QVector3D(arg0)};
    return 1;
  }
}

impl /*struct*/ QQuaternion {
  pub fn NewQQuaternion<T: QQuaternion_NewQQuaternion>(value: T) -> QQuaternion {
    let rsthis = value.NewQQuaternion();
    return rsthis;
    // return 1;
  }
}

pub trait QQuaternion_NewQQuaternion {
  fn NewQQuaternion(self) -> QQuaternion;
}

// proto: void QQuaternion::NewQQuaternion(const QVector4D & vector);
impl<'a> /*trait*/ QQuaternion_NewQQuaternion for (&'a  QVector4D) {
  fn NewQQuaternion(self) -> QQuaternion {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QQuaternionC1ERK9QVector4D()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QQuaternionC1ERK9QVector4D(qthis, arg0)};
    let rsthis = QQuaternion{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QQuaternion {
  pub fn rotationTo<T: QQuaternion_rotationTo>(&mut self, value: T) -> i32 {
    value.rotationTo(self);
    return 1;
  }
}

pub trait QQuaternion_rotationTo {
  fn rotationTo(self, this: &mut QQuaternion) -> i32;
}

// proto: QQuaternion QQuaternion::rotationTo(const QVector3D & from, const QVector3D & to);
impl<'a> /*trait*/ QQuaternion_rotationTo for (&'a  QVector3D, &'a  QVector3D) {
  fn rotationTo(self, this: &mut QQuaternion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QQuaternion10rotationToERK9QVector3DS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN11QQuaternion10rotationToERK9QVector3DS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QQuaternion {
  pub fn getEulerAngles<T: QQuaternion_getEulerAngles>(&mut self, value: T) -> i32 {
    value.getEulerAngles(self);
    return 1;
  }
}

pub trait QQuaternion_getEulerAngles {
  fn getEulerAngles(self, this: &mut QQuaternion) -> i32;
}

// proto: void QQuaternion::getEulerAngles(float * pitch, float * yaw, float * roll);
impl<'a> /*trait*/ QQuaternion_getEulerAngles for (&'a mut f32, &'a mut f32, &'a mut f32) {
  fn getEulerAngles(self, this: &mut QQuaternion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QQuaternion14getEulerAnglesEPfS0_S0_()};
    let arg0 = self.0  as *mut c_float;
    let arg1 = self.1  as *mut c_float;
    let arg2 = self.2  as *mut c_float;
    unsafe {_ZNK11QQuaternion14getEulerAnglesEPfS0_S0_(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QQuaternion {
  pub fn setY<T: QQuaternion_setY>(&mut self, value: T) -> i32 {
    value.setY(self);
    return 1;
  }
}

pub trait QQuaternion_setY {
  fn setY(self, this: &mut QQuaternion) -> i32;
}

// proto: void QQuaternion::setY(float y);
impl<'a> /*trait*/ QQuaternion_setY for (f32) {
  fn setY(self, this: &mut QQuaternion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QQuaternion4setYEf()};
    let arg0 = self  as c_float;
    unsafe {_ZN11QQuaternion4setYEf(arg0)};
    return 1;
  }
}

impl /*struct*/ QQuaternion {
  pub fn toEulerAngles<T: QQuaternion_toEulerAngles>(&mut self, value: T) -> i32 {
    value.toEulerAngles(self);
    return 1;
  }
}

pub trait QQuaternion_toEulerAngles {
  fn toEulerAngles(self, this: &mut QQuaternion) -> i32;
}

// proto: QVector3D QQuaternion::toEulerAngles();
impl<'a> /*trait*/ QQuaternion_toEulerAngles for () {
  fn toEulerAngles(self, this: &mut QQuaternion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QQuaternion13toEulerAnglesEv()};
    unsafe {_ZNK11QQuaternion13toEulerAnglesEv()};
    return 1;
  }
}

impl /*struct*/ QQuaternion {
  pub fn inverted<T: QQuaternion_inverted>(&mut self, value: T) -> i32 {
    value.inverted(self);
    return 1;
  }
}

pub trait QQuaternion_inverted {
  fn inverted(self, this: &mut QQuaternion) -> i32;
}

// proto: QQuaternion QQuaternion::inverted();
impl<'a> /*trait*/ QQuaternion_inverted for () {
  fn inverted(self, this: &mut QQuaternion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QQuaternion8invertedEv()};
    unsafe {_ZNK11QQuaternion8invertedEv()};
    return 1;
  }
}

impl /*struct*/ QQuaternion {
  pub fn setZ<T: QQuaternion_setZ>(&mut self, value: T) -> i32 {
    value.setZ(self);
    return 1;
  }
}

pub trait QQuaternion_setZ {
  fn setZ(self, this: &mut QQuaternion) -> i32;
}

// proto: void QQuaternion::setZ(float z);
impl<'a> /*trait*/ QQuaternion_setZ for (f32) {
  fn setZ(self, this: &mut QQuaternion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QQuaternion4setZEf()};
    let arg0 = self  as c_float;
    unsafe {_ZN11QQuaternion4setZEf(arg0)};
    return 1;
  }
}

impl /*struct*/ QQuaternion {
  pub fn getAxes<T: QQuaternion_getAxes>(&mut self, value: T) -> i32 {
    value.getAxes(self);
    return 1;
  }
}

pub trait QQuaternion_getAxes {
  fn getAxes(self, this: &mut QQuaternion) -> i32;
}

// proto: void QQuaternion::getAxes(QVector3D * xAxis, QVector3D * yAxis, QVector3D * zAxis);
impl<'a> /*trait*/ QQuaternion_getAxes for (&'a mut QVector3D, &'a mut QVector3D, &'a mut QVector3D) {
  fn getAxes(self, this: &mut QQuaternion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QQuaternion7getAxesEP9QVector3DS1_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZNK11QQuaternion7getAxesEP9QVector3DS1_S1_(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QQuaternion {
  pub fn nlerp<T: QQuaternion_nlerp>(&mut self, value: T) -> i32 {
    value.nlerp(self);
    return 1;
  }
}

pub trait QQuaternion_nlerp {
  fn nlerp(self, this: &mut QQuaternion) -> i32;
}

// proto: QQuaternion QQuaternion::nlerp(const QQuaternion & q1, const QQuaternion & q2, float t);
impl<'a> /*trait*/ QQuaternion_nlerp for (&'a  QQuaternion, &'a  QQuaternion, f32) {
  fn nlerp(self, this: &mut QQuaternion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QQuaternion5nlerpERKS_S1_f()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2  as c_float;
    unsafe {_ZN11QQuaternion5nlerpERKS_S1_f(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QQuaternion {
  pub fn isIdentity<T: QQuaternion_isIdentity>(&mut self, value: T) -> i32 {
    value.isIdentity(self);
    return 1;
  }
}

pub trait QQuaternion_isIdentity {
  fn isIdentity(self, this: &mut QQuaternion) -> i32;
}

// proto: bool QQuaternion::isIdentity();
impl<'a> /*trait*/ QQuaternion_isIdentity for () {
  fn isIdentity(self, this: &mut QQuaternion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QQuaternion10isIdentityEv()};
    unsafe {_ZNK11QQuaternion10isIdentityEv()};
    return 1;
  }
}

impl /*struct*/ QQuaternion {
  pub fn fromAxes<T: QQuaternion_fromAxes>(&mut self, value: T) -> i32 {
    value.fromAxes(self);
    return 1;
  }
}

pub trait QQuaternion_fromAxes {
  fn fromAxes(self, this: &mut QQuaternion) -> i32;
}

// proto: QQuaternion QQuaternion::fromAxes(const QVector3D & xAxis, const QVector3D & yAxis, const QVector3D & zAxis);
impl<'a> /*trait*/ QQuaternion_fromAxes for (&'a  QVector3D, &'a  QVector3D, &'a  QVector3D) {
  fn fromAxes(self, this: &mut QQuaternion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QQuaternion8fromAxesERK9QVector3DS2_S2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN11QQuaternion8fromAxesERK9QVector3DS2_S2_(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QQuaternion {
  pub fn slerp<T: QQuaternion_slerp>(&mut self, value: T) -> i32 {
    value.slerp(self);
    return 1;
  }
}

pub trait QQuaternion_slerp {
  fn slerp(self, this: &mut QQuaternion) -> i32;
}

// proto: QQuaternion QQuaternion::slerp(const QQuaternion & q1, const QQuaternion & q2, float t);
impl<'a> /*trait*/ QQuaternion_slerp for (&'a  QQuaternion, &'a  QQuaternion, f32) {
  fn slerp(self, this: &mut QQuaternion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QQuaternion5slerpERKS_S1_f()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2  as c_float;
    unsafe {_ZN11QQuaternion5slerpERKS_S1_f(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QQuaternion {
  pub fn fromDirection<T: QQuaternion_fromDirection>(&mut self, value: T) -> i32 {
    value.fromDirection(self);
    return 1;
  }
}

pub trait QQuaternion_fromDirection {
  fn fromDirection(self, this: &mut QQuaternion) -> i32;
}

// proto: QQuaternion QQuaternion::fromDirection(const QVector3D & direction, const QVector3D & up);
impl<'a> /*trait*/ QQuaternion_fromDirection for (&'a  QVector3D, &'a  QVector3D) {
  fn fromDirection(self, this: &mut QQuaternion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QQuaternion13fromDirectionERK9QVector3DS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN11QQuaternion13fromDirectionERK9QVector3DS2_(arg0, arg1)};
    return 1;
  }
}

// proto: void QQuaternion::NewQQuaternion();
impl<'a> /*trait*/ QQuaternion_NewQQuaternion for () {
  fn NewQQuaternion(self) -> QQuaternion {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QQuaternionC1Ev()};
    unsafe {_ZN11QQuaternionC1Ev(qthis)};
    let rsthis = QQuaternion{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QQuaternion {
  pub fn normalized<T: QQuaternion_normalized>(&mut self, value: T) -> i32 {
    value.normalized(self);
    return 1;
  }
}

pub trait QQuaternion_normalized {
  fn normalized(self, this: &mut QQuaternion) -> i32;
}

// proto: QQuaternion QQuaternion::normalized();
impl<'a> /*trait*/ QQuaternion_normalized for () {
  fn normalized(self, this: &mut QQuaternion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QQuaternion10normalizedEv()};
    unsafe {_ZNK11QQuaternion10normalizedEv()};
    return 1;
  }
}

impl /*struct*/ QQuaternion {
  pub fn toVector4D<T: QQuaternion_toVector4D>(&mut self, value: T) -> i32 {
    value.toVector4D(self);
    return 1;
  }
}

pub trait QQuaternion_toVector4D {
  fn toVector4D(self, this: &mut QQuaternion) -> i32;
}

// proto: QVector4D QQuaternion::toVector4D();
impl<'a> /*trait*/ QQuaternion_toVector4D for () {
  fn toVector4D(self, this: &mut QQuaternion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QQuaternion10toVector4DEv()};
    unsafe {_ZNK11QQuaternion10toVector4DEv()};
    return 1;
  }
}

impl /*struct*/ QQuaternion {
  pub fn fromEulerAngles<T: QQuaternion_fromEulerAngles>(&mut self, value: T) -> i32 {
    value.fromEulerAngles(self);
    return 1;
  }
}

pub trait QQuaternion_fromEulerAngles {
  fn fromEulerAngles(self, this: &mut QQuaternion) -> i32;
}

// proto: QQuaternion QQuaternion::fromEulerAngles(float pitch, float yaw, float roll);
impl<'a> /*trait*/ QQuaternion_fromEulerAngles for (f32, f32, f32) {
  fn fromEulerAngles(self, this: &mut QQuaternion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QQuaternion15fromEulerAnglesEfff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    unsafe {_ZN11QQuaternion15fromEulerAnglesEfff(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QQuaternion {
  pub fn conjugate<T: QQuaternion_conjugate>(&mut self, value: T) -> i32 {
    value.conjugate(self);
    return 1;
  }
}

pub trait QQuaternion_conjugate {
  fn conjugate(self, this: &mut QQuaternion) -> i32;
}

// proto: QQuaternion QQuaternion::conjugate();
impl<'a> /*trait*/ QQuaternion_conjugate for () {
  fn conjugate(self, this: &mut QQuaternion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QQuaternion9conjugateEv()};
    unsafe {_ZNK11QQuaternion9conjugateEv()};
    return 1;
  }
}

impl /*struct*/ QQuaternion {
  pub fn isNull<T: QQuaternion_isNull>(&mut self, value: T) -> i32 {
    value.isNull(self);
    return 1;
  }
}

pub trait QQuaternion_isNull {
  fn isNull(self, this: &mut QQuaternion) -> i32;
}

// proto: bool QQuaternion::isNull();
impl<'a> /*trait*/ QQuaternion_isNull for () {
  fn isNull(self, this: &mut QQuaternion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QQuaternion6isNullEv()};
    unsafe {_ZNK11QQuaternion6isNullEv()};
    return 1;
  }
}

// proto: void QQuaternion::getAxisAndAngle(QVector3D * axis, float * angle);
impl<'a> /*trait*/ QQuaternion_getAxisAndAngle for (&'a mut QVector3D, &'a mut f32) {
  fn getAxisAndAngle(self, this: &mut QQuaternion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QQuaternion15getAxisAndAngleEP9QVector3DPf()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as *mut c_float;
    unsafe {_ZNK11QQuaternion15getAxisAndAngleEP9QVector3DPf(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QQuaternion {
  pub fn toRotationMatrix<T: QQuaternion_toRotationMatrix>(&mut self, value: T) -> i32 {
    value.toRotationMatrix(self);
    return 1;
  }
}

pub trait QQuaternion_toRotationMatrix {
  fn toRotationMatrix(self, this: &mut QQuaternion) -> i32;
}

// proto: QGenericMatrix<3, 3, float> QQuaternion::toRotationMatrix();
impl<'a> /*trait*/ QQuaternion_toRotationMatrix for () {
  fn toRotationMatrix(self, this: &mut QQuaternion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QQuaternion16toRotationMatrixEv()};
    unsafe {_ZNK11QQuaternion16toRotationMatrixEv()};
    return 1;
  }
}

// proto: QQuaternion QQuaternion::fromEulerAngles(const QVector3D & eulerAngles);
impl<'a> /*trait*/ QQuaternion_fromEulerAngles for (&'a  QVector3D) {
  fn fromEulerAngles(self, this: &mut QQuaternion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QQuaternion15fromEulerAnglesERK9QVector3D()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QQuaternion15fromEulerAnglesERK9QVector3D(arg0)};
    return 1;
  }
}

impl /*struct*/ QQuaternion {
  pub fn rotatedVector<T: QQuaternion_rotatedVector>(&mut self, value: T) -> i32 {
    value.rotatedVector(self);
    return 1;
  }
}

pub trait QQuaternion_rotatedVector {
  fn rotatedVector(self, this: &mut QQuaternion) -> i32;
}

// proto: QVector3D QQuaternion::rotatedVector(const QVector3D & vector);
impl<'a> /*trait*/ QQuaternion_rotatedVector for (&'a  QVector3D) {
  fn rotatedVector(self, this: &mut QQuaternion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QQuaternion13rotatedVectorERK9QVector3D()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK11QQuaternion13rotatedVectorERK9QVector3D(arg0)};
    return 1;
  }
}

impl /*struct*/ QQuaternion {
  pub fn lengthSquared<T: QQuaternion_lengthSquared>(&mut self, value: T) -> i32 {
    value.lengthSquared(self);
    return 1;
  }
}

pub trait QQuaternion_lengthSquared {
  fn lengthSquared(self, this: &mut QQuaternion) -> i32;
}

// proto: float QQuaternion::lengthSquared();
impl<'a> /*trait*/ QQuaternion_lengthSquared for () {
  fn lengthSquared(self, this: &mut QQuaternion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QQuaternion13lengthSquaredEv()};
    unsafe {_ZNK11QQuaternion13lengthSquaredEv()};
    return 1;
  }
}

impl /*struct*/ QQuaternion {
  pub fn setScalar<T: QQuaternion_setScalar>(&mut self, value: T) -> i32 {
    value.setScalar(self);
    return 1;
  }
}

pub trait QQuaternion_setScalar {
  fn setScalar(self, this: &mut QQuaternion) -> i32;
}

// proto: void QQuaternion::setScalar(float scalar);
impl<'a> /*trait*/ QQuaternion_setScalar for (f32) {
  fn setScalar(self, this: &mut QQuaternion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QQuaternion9setScalarEf()};
    let arg0 = self  as c_float;
    unsafe {_ZN11QQuaternion9setScalarEf(arg0)};
    return 1;
  }
}

impl /*struct*/ QQuaternion {
  pub fn y<T: QQuaternion_y>(&mut self, value: T) -> i32 {
    value.y(self);
    return 1;
  }
}

pub trait QQuaternion_y {
  fn y(self, this: &mut QQuaternion) -> i32;
}

// proto: float QQuaternion::y();
impl<'a> /*trait*/ QQuaternion_y for () {
  fn y(self, this: &mut QQuaternion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QQuaternion1yEv()};
    unsafe {_ZNK11QQuaternion1yEv()};
    return 1;
  }
}

impl /*struct*/ QQuaternion {
  pub fn vector<T: QQuaternion_vector>(&mut self, value: T) -> i32 {
    value.vector(self);
    return 1;
  }
}

pub trait QQuaternion_vector {
  fn vector(self, this: &mut QQuaternion) -> i32;
}

// proto: QVector3D QQuaternion::vector();
impl<'a> /*trait*/ QQuaternion_vector for () {
  fn vector(self, this: &mut QQuaternion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QQuaternion6vectorEv()};
    unsafe {_ZNK11QQuaternion6vectorEv()};
    return 1;
  }
}

impl /*struct*/ QQuaternion {
  pub fn dotProduct<T: QQuaternion_dotProduct>(&mut self, value: T) -> i32 {
    value.dotProduct(self);
    return 1;
  }
}

pub trait QQuaternion_dotProduct {
  fn dotProduct(self, this: &mut QQuaternion) -> i32;
}

// proto: float QQuaternion::dotProduct(const QQuaternion & q1, const QQuaternion & q2);
impl<'a> /*trait*/ QQuaternion_dotProduct for (&'a  QQuaternion, &'a  QQuaternion) {
  fn dotProduct(self, this: &mut QQuaternion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QQuaternion10dotProductERKS_S1_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN11QQuaternion10dotProductERKS_S1_(arg0, arg1)};
    return 1;
  }
}

// proto: void QQuaternion::setVector(float x, float y, float z);
impl<'a> /*trait*/ QQuaternion_setVector for (f32, f32, f32) {
  fn setVector(self, this: &mut QQuaternion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QQuaternion9setVectorEfff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    unsafe {_ZN11QQuaternion9setVectorEfff(arg0, arg1, arg2)};
    return 1;
  }
}

// proto: void QQuaternion::NewQQuaternion(float scalar, float xpos, float ypos, float zpos);
impl<'a> /*trait*/ QQuaternion_NewQQuaternion for (f32, f32, f32, f32) {
  fn NewQQuaternion(self) -> QQuaternion {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QQuaternionC1Effff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
    unsafe {_ZN11QQuaternionC1Effff(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QQuaternion{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QQuaternion {
  pub fn fromAxisAndAngle<T: QQuaternion_fromAxisAndAngle>(&mut self, value: T) -> i32 {
    value.fromAxisAndAngle(self);
    return 1;
  }
}

pub trait QQuaternion_fromAxisAndAngle {
  fn fromAxisAndAngle(self, this: &mut QQuaternion) -> i32;
}

// proto: QQuaternion QQuaternion::fromAxisAndAngle(const QVector3D & axis, float angle);
impl<'a> /*trait*/ QQuaternion_fromAxisAndAngle for (&'a  QVector3D, f32) {
  fn fromAxisAndAngle(self, this: &mut QQuaternion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QQuaternion16fromAxisAndAngleERK9QVector3Df()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_float;
    unsafe {_ZN11QQuaternion16fromAxisAndAngleERK9QVector3Df(arg0, arg1)};
    return 1;
  }
}

// proto: void QQuaternion::NewQQuaternion(float scalar, const QVector3D & vector);
impl<'a> /*trait*/ QQuaternion_NewQQuaternion for (f32, &'a  QVector3D) {
  fn NewQQuaternion(self) -> QQuaternion {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QQuaternionC1EfRK9QVector3D()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN11QQuaternionC1EfRK9QVector3D(qthis, arg0, arg1)};
    let rsthis = QQuaternion{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QQuaternion {
  pub fn length<T: QQuaternion_length>(&mut self, value: T) -> i32 {
    value.length(self);
    return 1;
  }
}

pub trait QQuaternion_length {
  fn length(self, this: &mut QQuaternion) -> i32;
}

// proto: float QQuaternion::length();
impl<'a> /*trait*/ QQuaternion_length for () {
  fn length(self, this: &mut QQuaternion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QQuaternion6lengthEv()};
    unsafe {_ZNK11QQuaternion6lengthEv()};
    return 1;
  }
}

impl /*struct*/ QQuaternion {
  pub fn normalize<T: QQuaternion_normalize>(&mut self, value: T) -> i32 {
    value.normalize(self);
    return 1;
  }
}

pub trait QQuaternion_normalize {
  fn normalize(self, this: &mut QQuaternion) -> i32;
}

// proto: void QQuaternion::normalize();
impl<'a> /*trait*/ QQuaternion_normalize for () {
  fn normalize(self, this: &mut QQuaternion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QQuaternion9normalizeEv()};
    unsafe {_ZN11QQuaternion9normalizeEv()};
    return 1;
  }
}

impl /*struct*/ QQuaternion {
  pub fn conjugated<T: QQuaternion_conjugated>(&mut self, value: T) -> i32 {
    value.conjugated(self);
    return 1;
  }
}

pub trait QQuaternion_conjugated {
  fn conjugated(self, this: &mut QQuaternion) -> i32;
}

// proto: QQuaternion QQuaternion::conjugated();
impl<'a> /*trait*/ QQuaternion_conjugated for () {
  fn conjugated(self, this: &mut QQuaternion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QQuaternion10conjugatedEv()};
    unsafe {_ZNK11QQuaternion10conjugatedEv()};
    return 1;
  }
}

// proto: QQuaternion QQuaternion::fromAxisAndAngle(float x, float y, float z, float angle);
impl<'a> /*trait*/ QQuaternion_fromAxisAndAngle for (f32, f32, f32, f32) {
  fn fromAxisAndAngle(self, this: &mut QQuaternion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QQuaternion16fromAxisAndAngleEffff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
    unsafe {_ZN11QQuaternion16fromAxisAndAngleEffff(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QQuaternion {
  pub fn x<T: QQuaternion_x>(&mut self, value: T) -> i32 {
    value.x(self);
    return 1;
  }
}

pub trait QQuaternion_x {
  fn x(self, this: &mut QQuaternion) -> i32;
}

// proto: float QQuaternion::x();
impl<'a> /*trait*/ QQuaternion_x for () {
  fn x(self, this: &mut QQuaternion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QQuaternion1xEv()};
    unsafe {_ZNK11QQuaternion1xEv()};
    return 1;
  }
}

impl /*struct*/ QQuaternion {
  pub fn z<T: QQuaternion_z>(&mut self, value: T) -> i32 {
    value.z(self);
    return 1;
  }
}

pub trait QQuaternion_z {
  fn z(self, this: &mut QQuaternion) -> i32;
}

// proto: float QQuaternion::z();
impl<'a> /*trait*/ QQuaternion_z for () {
  fn z(self, this: &mut QQuaternion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QQuaternion1zEv()};
    unsafe {_ZNK11QQuaternion1zEv()};
    return 1;
  }
}

