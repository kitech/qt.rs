// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpointf::QPointF;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QPointF QPanGesture::offset();
  fn _ZNK11QPanGesture6offsetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPointF QPanGesture::delta();
  fn _ZNK11QPanGesture5deltaEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPanGesture::setOffset(const QPointF & value);
  fn _ZN11QPanGesture9setOffsetERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  double QPanGesture::acceleration();
  fn _ZNK11QPanGesture12accelerationEv(qthis: *mut c_void) -> c_double;
  // proto:  void QPanGesture::FreeQPanGesture();
  fn _ZN11QPanGestureD0Ev(qthis: *mut c_void) ;
  // proto:  void QPanGesture::NewQPanGesture(QObject * parent);
  fn _ZN11QPanGestureC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QMetaObject * QPanGesture::metaObject();
  fn _ZNK11QPanGesture10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QPanGesture::setAcceleration(qreal value);
  fn _ZN11QPanGesture15setAccelerationEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  QPointF QPanGesture::lastOffset();
  fn _ZNK11QPanGesture10lastOffsetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPanGesture::setLastOffset(const QPointF & value);
  fn _ZN11QPanGesture13setLastOffsetERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QPanGesture)=1
pub struct QPanGesture {
  pub qclsinst: *mut c_void,
}

// proto:  QPointF QPanGesture::offset();
impl /*struct*/ QPanGesture {
  pub fn offset<RetType, T: QPanGesture_offset<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.offset(self);
    // return 1;
  }
}

pub trait QPanGesture_offset<RetType> {
  fn offset(self , rsthis: &mut QPanGesture) -> RetType;
}

// proto:  QPointF QPanGesture::offset();
impl<'a> /*trait*/ QPanGesture_offset<QPointF> for () {
  fn offset(self , rsthis: &mut QPanGesture) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPanGesture6offsetEv()};
    let mut ret = unsafe {_ZNK11QPanGesture6offsetEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QPointF QPanGesture::delta();
impl /*struct*/ QPanGesture {
  pub fn delta<RetType, T: QPanGesture_delta<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.delta(self);
    // return 1;
  }
}

pub trait QPanGesture_delta<RetType> {
  fn delta(self , rsthis: &mut QPanGesture) -> RetType;
}

// proto:  QPointF QPanGesture::delta();
impl<'a> /*trait*/ QPanGesture_delta<QPointF> for () {
  fn delta(self , rsthis: &mut QPanGesture) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPanGesture5deltaEv()};
    let mut ret = unsafe {_ZNK11QPanGesture5deltaEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QPanGesture::setOffset(const QPointF & value);
impl /*struct*/ QPanGesture {
  pub fn setOffset<RetType, T: QPanGesture_setOffset<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setOffset(self);
    // return 1;
  }
}

pub trait QPanGesture_setOffset<RetType> {
  fn setOffset(self , rsthis: &mut QPanGesture) -> RetType;
}

// proto:  void QPanGesture::setOffset(const QPointF & value);
impl<'a> /*trait*/ QPanGesture_setOffset<()> for (&'a  QPointF) {
  fn setOffset(self , rsthis: &mut QPanGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPanGesture9setOffsetERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QPanGesture9setOffsetERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  double QPanGesture::acceleration();
impl /*struct*/ QPanGesture {
  pub fn acceleration<RetType, T: QPanGesture_acceleration<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.acceleration(self);
    // return 1;
  }
}

pub trait QPanGesture_acceleration<RetType> {
  fn acceleration(self , rsthis: &mut QPanGesture) -> RetType;
}

// proto:  double QPanGesture::acceleration();
impl<'a> /*trait*/ QPanGesture_acceleration<f64> for () {
  fn acceleration(self , rsthis: &mut QPanGesture) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPanGesture12accelerationEv()};
    let mut ret = unsafe {_ZNK11QPanGesture12accelerationEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  void QPanGesture::FreeQPanGesture();
impl /*struct*/ QPanGesture {
  pub fn FreeQPanGesture<RetType, T: QPanGesture_FreeQPanGesture<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQPanGesture(self);
    // return 1;
  }
}

pub trait QPanGesture_FreeQPanGesture<RetType> {
  fn FreeQPanGesture(self , rsthis: &mut QPanGesture) -> RetType;
}

// proto:  void QPanGesture::FreeQPanGesture();
impl<'a> /*trait*/ QPanGesture_FreeQPanGesture<()> for () {
  fn FreeQPanGesture(self , rsthis: &mut QPanGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPanGestureD0Ev()};
     unsafe {_ZN11QPanGestureD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPanGesture {
  pub fn NewQPanGesture<T: QPanGesture_NewQPanGesture>(value: T) -> QPanGesture {
    let rsthis = value.NewQPanGesture();
    return rsthis;
    // return 1;
  }
}

pub trait QPanGesture_NewQPanGesture {
  fn NewQPanGesture(self) -> QPanGesture;
}

// proto: void QPanGesture::NewQPanGesture(QObject * parent);
impl<'a> /*trait*/ QPanGesture_NewQPanGesture for (&'a mut QObject) {
  fn NewQPanGesture(self) -> QPanGesture {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPanGestureC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QPanGestureC1EP7QObject(qthis, arg0)};
    let rsthis = QPanGesture{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  const QMetaObject * QPanGesture::metaObject();
impl /*struct*/ QPanGesture {
  pub fn metaObject<RetType, T: QPanGesture_metaObject<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QPanGesture_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QPanGesture) -> RetType;
}

// proto:  const QMetaObject * QPanGesture::metaObject();
impl<'a> /*trait*/ QPanGesture_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QPanGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPanGesture10metaObjectEv()};
     unsafe {_ZNK11QPanGesture10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QPanGesture::setAcceleration(qreal value);
impl /*struct*/ QPanGesture {
  pub fn setAcceleration<RetType, T: QPanGesture_setAcceleration<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setAcceleration(self);
    // return 1;
  }
}

pub trait QPanGesture_setAcceleration<RetType> {
  fn setAcceleration(self , rsthis: &mut QPanGesture) -> RetType;
}

// proto:  void QPanGesture::setAcceleration(qreal value);
impl<'a> /*trait*/ QPanGesture_setAcceleration<()> for (f64) {
  fn setAcceleration(self , rsthis: &mut QPanGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPanGesture15setAccelerationEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN11QPanGesture15setAccelerationEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QPointF QPanGesture::lastOffset();
impl /*struct*/ QPanGesture {
  pub fn lastOffset<RetType, T: QPanGesture_lastOffset<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.lastOffset(self);
    // return 1;
  }
}

pub trait QPanGesture_lastOffset<RetType> {
  fn lastOffset(self , rsthis: &mut QPanGesture) -> RetType;
}

// proto:  QPointF QPanGesture::lastOffset();
impl<'a> /*trait*/ QPanGesture_lastOffset<QPointF> for () {
  fn lastOffset(self , rsthis: &mut QPanGesture) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPanGesture10lastOffsetEv()};
    let mut ret = unsafe {_ZNK11QPanGesture10lastOffsetEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QPanGesture::setLastOffset(const QPointF & value);
impl /*struct*/ QPanGesture {
  pub fn setLastOffset<RetType, T: QPanGesture_setLastOffset<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setLastOffset(self);
    // return 1;
  }
}

pub trait QPanGesture_setLastOffset<RetType> {
  fn setLastOffset(self , rsthis: &mut QPanGesture) -> RetType;
}

// proto:  void QPanGesture::setLastOffset(const QPointF & value);
impl<'a> /*trait*/ QPanGesture_setLastOffset<()> for (&'a  QPointF) {
  fn setLastOffset(self , rsthis: &mut QPanGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPanGesture13setLastOffsetERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QPanGesture13setLastOffsetERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

