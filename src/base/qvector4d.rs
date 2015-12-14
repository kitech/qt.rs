// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qvector2d::QVector2D;
use super::qpointf::QPointF;
use super::qvector3d::QVector3D;
use super::qpoint::QPoint;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QVector4D QVector4D::normalized();
  fn _ZNK9QVector4D10normalizedEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QVector4D::setW(float w);
  fn _ZN9QVector4D4setWEf(qthis: *mut c_void, arg0: c_float) ;
  // proto:  void QVector4D::NewQVector4D(const QVector2D & vector, float zpos, float wpos);
  fn _ZN9QVector4DC1ERK9QVector2Dff(qthis: *mut c_void, arg0: *mut c_void, arg1: c_float, arg2: c_float) ;
  // proto:  QPointF QVector4D::toPointF();
  fn _ZNK9QVector4D8toPointFEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  float QVector4D::y();
  fn _ZNK9QVector4D1yEv(qthis: *mut c_void) -> c_float;
  // proto:  QVector2D QVector4D::toVector2D();
  fn _ZNK9QVector4D10toVector2DEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QVector4D::setZ(float z);
  fn _ZN9QVector4D4setZEf(qthis: *mut c_void, arg0: c_float) ;
  // proto:  void QVector4D::NewQVector4D(const QVector2D & vector);
  fn _ZN9QVector4DC1ERK9QVector2D(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QVector4D::normalize();
  fn _ZN9QVector4D9normalizeEv(qthis: *mut c_void) ;
  // proto:  void QVector4D::NewQVector4D(float xpos, float ypos, float zpos, float wpos);
  fn _ZN9QVector4DC1Effff(qthis: *mut c_void, arg0: c_float, arg1: c_float, arg2: c_float, arg3: c_float) ;
  // proto:  void QVector4D::NewQVector4D(const QVector3D & vector, float wpos);
  fn _ZN9QVector4DC1ERK9QVector3Df(qthis: *mut c_void, arg0: *mut c_void, arg1: c_float) ;
  // proto:  void QVector4D::NewQVector4D(const QPointF & point);
  fn _ZN9QVector4DC1ERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  float QVector4D::z();
  fn _ZNK9QVector4D1zEv(qthis: *mut c_void) -> c_float;
  // proto:  void QVector4D::NewQVector4D();
  fn _ZN9QVector4DC1Ev(qthis: *mut c_void) ;
  // proto:  void QVector4D::setX(float x);
  fn _ZN9QVector4D4setXEf(qthis: *mut c_void, arg0: c_float) ;
  // proto:  void QVector4D::setY(float y);
  fn _ZN9QVector4D4setYEf(qthis: *mut c_void, arg0: c_float) ;
  // proto:  void QVector4D::NewQVector4D(const QPoint & point);
  fn _ZN9QVector4DC1ERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QVector3D QVector4D::toVector3D();
  fn _ZNK9QVector4D10toVector3DEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  float QVector4D::x();
  fn _ZNK9QVector4D1xEv(qthis: *mut c_void) -> c_float;
  // proto:  QVector2D QVector4D::toVector2DAffine();
  fn _ZNK9QVector4D16toVector2DAffineEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  float QVector4D::length();
  fn _ZNK9QVector4D6lengthEv(qthis: *mut c_void) -> c_float;
  // proto:  void QVector4D::NewQVector4D(const QVector3D & vector);
  fn _ZN9QVector4DC1ERK9QVector3D(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static float QVector4D::dotProduct(const QVector4D & v1, const QVector4D & v2);
  fn _ZN9QVector4D10dotProductERKS_S1_(arg0: *mut c_void, arg1: *mut c_void) -> c_float;
  // proto:  bool QVector4D::isNull();
  fn _ZNK9QVector4D6isNullEv(qthis: *mut c_void) -> int8_t;
  // proto:  float QVector4D::lengthSquared();
  fn _ZNK9QVector4D13lengthSquaredEv(qthis: *mut c_void) -> c_float;
  // proto:  QVector3D QVector4D::toVector3DAffine();
  fn _ZNK9QVector4D16toVector3DAffineEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPoint QVector4D::toPoint();
  fn _ZNK9QVector4D7toPointEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  float QVector4D::w();
  fn _ZNK9QVector4D1wEv(qthis: *mut c_void) -> c_float;
}

// body block begin
// class sizeof(QVector4D)=16
pub struct QVector4D {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QVector4D {
  pub fn normalized<T: QVector4D_normalized>(&mut self, value: T) -> QVector4D {
    return value.normalized(self);
    // return 1;
  }
}

pub trait QVector4D_normalized {
  fn normalized(self, rsthis: &mut QVector4D) -> QVector4D;
}

// proto:  QVector4D QVector4D::normalized();
impl<'a> /*trait*/ QVector4D_normalized for () {
  fn normalized(self, rsthis: &mut QVector4D) -> QVector4D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector4D10normalizedEv()};
    let mut ret = unsafe {_ZNK9QVector4D10normalizedEv(rsthis.qclsinst)};
    let mut ret1 = QVector4D{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QVector4D {
  pub fn setW<T: QVector4D_setW>(&mut self, value: T)  {
     value.setW(self);
    // return 1;
  }
}

pub trait QVector4D_setW {
  fn setW(self, rsthis: &mut QVector4D) ;
}

// proto:  void QVector4D::setW(float w);
impl<'a> /*trait*/ QVector4D_setW for (f32) {
  fn setW(self, rsthis: &mut QVector4D)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector4D4setWEf()};
    let arg0 = self  as c_float;
     unsafe {_ZN9QVector4D4setWEf(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QVector4D {
  pub fn NewQVector4D<T: QVector4D_NewQVector4D>(value: T) -> QVector4D {
    let rsthis = value.NewQVector4D();
    return rsthis;
    // return 1;
  }
}

pub trait QVector4D_NewQVector4D {
  fn NewQVector4D(self) -> QVector4D;
}

// proto: void QVector4D::NewQVector4D(const QVector2D & vector, float zpos, float wpos);
impl<'a> /*trait*/ QVector4D_NewQVector4D for (&'a  QVector2D, f32, f32) {
  fn NewQVector4D(self) -> QVector4D {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector4DC1ERK9QVector2Dff()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    unsafe {_ZN9QVector4DC1ERK9QVector2Dff(qthis, arg0, arg1, arg2)};
    let rsthis = QVector4D{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVector4D {
  pub fn toPointF<T: QVector4D_toPointF>(&mut self, value: T) -> QPointF {
    return value.toPointF(self);
    // return 1;
  }
}

pub trait QVector4D_toPointF {
  fn toPointF(self, rsthis: &mut QVector4D) -> QPointF;
}

// proto:  QPointF QVector4D::toPointF();
impl<'a> /*trait*/ QVector4D_toPointF for () {
  fn toPointF(self, rsthis: &mut QVector4D) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector4D8toPointFEv()};
    let mut ret = unsafe {_ZNK9QVector4D8toPointFEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QVector4D {
  pub fn y<T: QVector4D_y>(&mut self, value: T) -> f32 {
    return value.y(self);
    // return 1;
  }
}

pub trait QVector4D_y {
  fn y(self, rsthis: &mut QVector4D) -> f32;
}

// proto:  float QVector4D::y();
impl<'a> /*trait*/ QVector4D_y for () {
  fn y(self, rsthis: &mut QVector4D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector4D1yEv()};
    let mut ret = unsafe {_ZNK9QVector4D1yEv(rsthis.qclsinst)};
    return ret as f32;
    // return 1;
  }
}

impl /*struct*/ QVector4D {
  pub fn toVector2D<T: QVector4D_toVector2D>(&mut self, value: T) -> QVector2D {
    return value.toVector2D(self);
    // return 1;
  }
}

pub trait QVector4D_toVector2D {
  fn toVector2D(self, rsthis: &mut QVector4D) -> QVector2D;
}

// proto:  QVector2D QVector4D::toVector2D();
impl<'a> /*trait*/ QVector4D_toVector2D for () {
  fn toVector2D(self, rsthis: &mut QVector4D) -> QVector2D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector4D10toVector2DEv()};
    let mut ret = unsafe {_ZNK9QVector4D10toVector2DEv(rsthis.qclsinst)};
    let mut ret1 = QVector2D{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QVector4D {
  pub fn setZ<T: QVector4D_setZ>(&mut self, value: T)  {
     value.setZ(self);
    // return 1;
  }
}

pub trait QVector4D_setZ {
  fn setZ(self, rsthis: &mut QVector4D) ;
}

// proto:  void QVector4D::setZ(float z);
impl<'a> /*trait*/ QVector4D_setZ for (f32) {
  fn setZ(self, rsthis: &mut QVector4D)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector4D4setZEf()};
    let arg0 = self  as c_float;
     unsafe {_ZN9QVector4D4setZEf(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QVector4D::NewQVector4D(const QVector2D & vector);
impl<'a> /*trait*/ QVector4D_NewQVector4D for (&'a  QVector2D) {
  fn NewQVector4D(self) -> QVector4D {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector4DC1ERK9QVector2D()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QVector4DC1ERK9QVector2D(qthis, arg0)};
    let rsthis = QVector4D{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVector4D {
  pub fn normalize<T: QVector4D_normalize>(&mut self, value: T)  {
     value.normalize(self);
    // return 1;
  }
}

pub trait QVector4D_normalize {
  fn normalize(self, rsthis: &mut QVector4D) ;
}

// proto:  void QVector4D::normalize();
impl<'a> /*trait*/ QVector4D_normalize for () {
  fn normalize(self, rsthis: &mut QVector4D)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector4D9normalizeEv()};
     unsafe {_ZN9QVector4D9normalizeEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QVector4D::NewQVector4D(float xpos, float ypos, float zpos, float wpos);
impl<'a> /*trait*/ QVector4D_NewQVector4D for (f32, f32, f32, f32) {
  fn NewQVector4D(self) -> QVector4D {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector4DC1Effff()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_float;
    let arg2 = self.2  as c_float;
    let arg3 = self.3  as c_float;
    unsafe {_ZN9QVector4DC1Effff(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QVector4D{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QVector4D::NewQVector4D(const QVector3D & vector, float wpos);
impl<'a> /*trait*/ QVector4D_NewQVector4D for (&'a  QVector3D, f32) {
  fn NewQVector4D(self) -> QVector4D {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector4DC1ERK9QVector3Df()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_float;
    unsafe {_ZN9QVector4DC1ERK9QVector3Df(qthis, arg0, arg1)};
    let rsthis = QVector4D{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QVector4D::NewQVector4D(const QPointF & point);
impl<'a> /*trait*/ QVector4D_NewQVector4D for (&'a  QPointF) {
  fn NewQVector4D(self) -> QVector4D {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector4DC1ERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QVector4DC1ERK7QPointF(qthis, arg0)};
    let rsthis = QVector4D{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVector4D {
  pub fn z<T: QVector4D_z>(&mut self, value: T) -> f32 {
    return value.z(self);
    // return 1;
  }
}

pub trait QVector4D_z {
  fn z(self, rsthis: &mut QVector4D) -> f32;
}

// proto:  float QVector4D::z();
impl<'a> /*trait*/ QVector4D_z for () {
  fn z(self, rsthis: &mut QVector4D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector4D1zEv()};
    let mut ret = unsafe {_ZNK9QVector4D1zEv(rsthis.qclsinst)};
    return ret as f32;
    // return 1;
  }
}

// proto: void QVector4D::NewQVector4D();
impl<'a> /*trait*/ QVector4D_NewQVector4D for () {
  fn NewQVector4D(self) -> QVector4D {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector4DC1Ev()};
    unsafe {_ZN9QVector4DC1Ev(qthis)};
    let rsthis = QVector4D{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVector4D {
  pub fn setX<T: QVector4D_setX>(&mut self, value: T)  {
     value.setX(self);
    // return 1;
  }
}

pub trait QVector4D_setX {
  fn setX(self, rsthis: &mut QVector4D) ;
}

// proto:  void QVector4D::setX(float x);
impl<'a> /*trait*/ QVector4D_setX for (f32) {
  fn setX(self, rsthis: &mut QVector4D)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector4D4setXEf()};
    let arg0 = self  as c_float;
     unsafe {_ZN9QVector4D4setXEf(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QVector4D {
  pub fn setY<T: QVector4D_setY>(&mut self, value: T)  {
     value.setY(self);
    // return 1;
  }
}

pub trait QVector4D_setY {
  fn setY(self, rsthis: &mut QVector4D) ;
}

// proto:  void QVector4D::setY(float y);
impl<'a> /*trait*/ QVector4D_setY for (f32) {
  fn setY(self, rsthis: &mut QVector4D)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector4D4setYEf()};
    let arg0 = self  as c_float;
     unsafe {_ZN9QVector4D4setYEf(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QVector4D::NewQVector4D(const QPoint & point);
impl<'a> /*trait*/ QVector4D_NewQVector4D for (&'a  QPoint) {
  fn NewQVector4D(self) -> QVector4D {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector4DC1ERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QVector4DC1ERK6QPoint(qthis, arg0)};
    let rsthis = QVector4D{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVector4D {
  pub fn toVector3D<T: QVector4D_toVector3D>(&mut self, value: T) -> QVector3D {
    return value.toVector3D(self);
    // return 1;
  }
}

pub trait QVector4D_toVector3D {
  fn toVector3D(self, rsthis: &mut QVector4D) -> QVector3D;
}

// proto:  QVector3D QVector4D::toVector3D();
impl<'a> /*trait*/ QVector4D_toVector3D for () {
  fn toVector3D(self, rsthis: &mut QVector4D) -> QVector3D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector4D10toVector3DEv()};
    let mut ret = unsafe {_ZNK9QVector4D10toVector3DEv(rsthis.qclsinst)};
    let mut ret1 = QVector3D{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QVector4D {
  pub fn x<T: QVector4D_x>(&mut self, value: T) -> f32 {
    return value.x(self);
    // return 1;
  }
}

pub trait QVector4D_x {
  fn x(self, rsthis: &mut QVector4D) -> f32;
}

// proto:  float QVector4D::x();
impl<'a> /*trait*/ QVector4D_x for () {
  fn x(self, rsthis: &mut QVector4D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector4D1xEv()};
    let mut ret = unsafe {_ZNK9QVector4D1xEv(rsthis.qclsinst)};
    return ret as f32;
    // return 1;
  }
}

impl /*struct*/ QVector4D {
  pub fn toVector2DAffine<T: QVector4D_toVector2DAffine>(&mut self, value: T) -> QVector2D {
    return value.toVector2DAffine(self);
    // return 1;
  }
}

pub trait QVector4D_toVector2DAffine {
  fn toVector2DAffine(self, rsthis: &mut QVector4D) -> QVector2D;
}

// proto:  QVector2D QVector4D::toVector2DAffine();
impl<'a> /*trait*/ QVector4D_toVector2DAffine for () {
  fn toVector2DAffine(self, rsthis: &mut QVector4D) -> QVector2D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector4D16toVector2DAffineEv()};
    let mut ret = unsafe {_ZNK9QVector4D16toVector2DAffineEv(rsthis.qclsinst)};
    let mut ret1 = QVector2D{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QVector4D {
  pub fn length<T: QVector4D_length>(&mut self, value: T) -> f32 {
    return value.length(self);
    // return 1;
  }
}

pub trait QVector4D_length {
  fn length(self, rsthis: &mut QVector4D) -> f32;
}

// proto:  float QVector4D::length();
impl<'a> /*trait*/ QVector4D_length for () {
  fn length(self, rsthis: &mut QVector4D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector4D6lengthEv()};
    let mut ret = unsafe {_ZNK9QVector4D6lengthEv(rsthis.qclsinst)};
    return ret as f32;
    // return 1;
  }
}

// proto: void QVector4D::NewQVector4D(const QVector3D & vector);
impl<'a> /*trait*/ QVector4D_NewQVector4D for (&'a  QVector3D) {
  fn NewQVector4D(self) -> QVector4D {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector4DC1ERK9QVector3D()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QVector4DC1ERK9QVector3D(qthis, arg0)};
    let rsthis = QVector4D{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVector4D {
  pub fn dotProduct<T: QVector4D_dotProduct>(&mut self, value: T) -> f32 {
    return value.dotProduct(self);
    // return 1;
  }
}

pub trait QVector4D_dotProduct {
  fn dotProduct(self, rsthis: &mut QVector4D) -> f32;
}

// proto: static float QVector4D::dotProduct(const QVector4D & v1, const QVector4D & v2);
impl<'a> /*trait*/ QVector4D_dotProduct for (&'a  QVector4D, &'a  QVector4D) {
  fn dotProduct(self, rsthis: &mut QVector4D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QVector4D10dotProductERKS_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QVector4D10dotProductERKS_S1_(arg0, arg1)};
    return ret as f32;
    // return 1;
  }
}

impl /*struct*/ QVector4D {
  pub fn isNull<T: QVector4D_isNull>(&mut self, value: T) -> i8 {
    return value.isNull(self);
    // return 1;
  }
}

pub trait QVector4D_isNull {
  fn isNull(self, rsthis: &mut QVector4D) -> i8;
}

// proto:  bool QVector4D::isNull();
impl<'a> /*trait*/ QVector4D_isNull for () {
  fn isNull(self, rsthis: &mut QVector4D) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector4D6isNullEv()};
    let mut ret = unsafe {_ZNK9QVector4D6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QVector4D {
  pub fn lengthSquared<T: QVector4D_lengthSquared>(&mut self, value: T) -> f32 {
    return value.lengthSquared(self);
    // return 1;
  }
}

pub trait QVector4D_lengthSquared {
  fn lengthSquared(self, rsthis: &mut QVector4D) -> f32;
}

// proto:  float QVector4D::lengthSquared();
impl<'a> /*trait*/ QVector4D_lengthSquared for () {
  fn lengthSquared(self, rsthis: &mut QVector4D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector4D13lengthSquaredEv()};
    let mut ret = unsafe {_ZNK9QVector4D13lengthSquaredEv(rsthis.qclsinst)};
    return ret as f32;
    // return 1;
  }
}

impl /*struct*/ QVector4D {
  pub fn toVector3DAffine<T: QVector4D_toVector3DAffine>(&mut self, value: T) -> QVector3D {
    return value.toVector3DAffine(self);
    // return 1;
  }
}

pub trait QVector4D_toVector3DAffine {
  fn toVector3DAffine(self, rsthis: &mut QVector4D) -> QVector3D;
}

// proto:  QVector3D QVector4D::toVector3DAffine();
impl<'a> /*trait*/ QVector4D_toVector3DAffine for () {
  fn toVector3DAffine(self, rsthis: &mut QVector4D) -> QVector3D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector4D16toVector3DAffineEv()};
    let mut ret = unsafe {_ZNK9QVector4D16toVector3DAffineEv(rsthis.qclsinst)};
    let mut ret1 = QVector3D{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QVector4D {
  pub fn toPoint<T: QVector4D_toPoint>(&mut self, value: T) -> QPoint {
    return value.toPoint(self);
    // return 1;
  }
}

pub trait QVector4D_toPoint {
  fn toPoint(self, rsthis: &mut QVector4D) -> QPoint;
}

// proto:  QPoint QVector4D::toPoint();
impl<'a> /*trait*/ QVector4D_toPoint for () {
  fn toPoint(self, rsthis: &mut QVector4D) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector4D7toPointEv()};
    let mut ret = unsafe {_ZNK9QVector4D7toPointEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QVector4D {
  pub fn w<T: QVector4D_w>(&mut self, value: T) -> f32 {
    return value.w(self);
    // return 1;
  }
}

pub trait QVector4D_w {
  fn w(self, rsthis: &mut QVector4D) -> f32;
}

// proto:  float QVector4D::w();
impl<'a> /*trait*/ QVector4D_w for () {
  fn w(self, rsthis: &mut QVector4D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QVector4D1wEv()};
    let mut ret = unsafe {_ZNK9QVector4D1wEv(rsthis.qclsinst)};
    return ret as f32;
    // return 1;
  }
}

