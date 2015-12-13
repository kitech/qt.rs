// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;
use super::qpointf::QPointF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QPinchGesture::setRotationAngle(qreal value);
  fn _ZN13QPinchGesture16setRotationAngleEd(arg0: c_double) -> i32;
  // proto: double QPinchGesture::lastScaleFactor();
  fn _ZNK13QPinchGesture15lastScaleFactorEv() -> i32;
  // proto: double QPinchGesture::lastRotationAngle();
  fn _ZNK13QPinchGesture17lastRotationAngleEv() -> i32;
  // proto: QPointF QPinchGesture::startCenterPoint();
  fn _ZNK13QPinchGesture16startCenterPointEv() -> i32;
  // proto: double QPinchGesture::rotationAngle();
  fn _ZNK13QPinchGesture13rotationAngleEv() -> i32;
  // proto: QPointF QPinchGesture::lastCenterPoint();
  fn _ZNK13QPinchGesture15lastCenterPointEv() -> i32;
  // proto: void QPinchGesture::NewQPinchGesture(QObject * parent);
  fn _ZN13QPinchGestureC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: double QPinchGesture::totalScaleFactor();
  fn _ZNK13QPinchGesture16totalScaleFactorEv() -> i32;
  // proto: void QPinchGesture::setTotalScaleFactor(qreal value);
  fn _ZN13QPinchGesture19setTotalScaleFactorEd(arg0: c_double) -> i32;
  // proto: double QPinchGesture::totalRotationAngle();
  fn _ZNK13QPinchGesture18totalRotationAngleEv() -> i32;
  // proto: void QPinchGesture::setLastScaleFactor(qreal value);
  fn _ZN13QPinchGesture18setLastScaleFactorEd(arg0: c_double) -> i32;
  // proto: void QPinchGesture::setLastCenterPoint(const QPointF & value);
  fn _ZN13QPinchGesture18setLastCenterPointERK7QPointF(arg0: *const c_void) -> i32;
  // proto: const QMetaObject * QPinchGesture::metaObject();
  fn _ZNK13QPinchGesture10metaObjectEv() -> i32;
  // proto: void QPinchGesture::setLastRotationAngle(qreal value);
  fn _ZN13QPinchGesture20setLastRotationAngleEd(arg0: c_double) -> i32;
  // proto: QPointF QPinchGesture::centerPoint();
  fn _ZNK13QPinchGesture11centerPointEv() -> i32;
  // proto: void QPinchGesture::setCenterPoint(const QPointF & value);
  fn _ZN13QPinchGesture14setCenterPointERK7QPointF(arg0: *const c_void) -> i32;
  // proto: void QPinchGesture::setTotalRotationAngle(qreal value);
  fn _ZN13QPinchGesture21setTotalRotationAngleEd(arg0: c_double) -> i32;
  // proto: void QPinchGesture::setScaleFactor(qreal value);
  fn _ZN13QPinchGesture14setScaleFactorEd(arg0: c_double) -> i32;
  // proto: void QPinchGesture::FreeQPinchGesture();
  fn _ZN13QPinchGestureD0Ev() -> i32;
  // proto: void QPinchGesture::setStartCenterPoint(const QPointF & value);
  fn _ZN13QPinchGesture19setStartCenterPointERK7QPointF(arg0: *const c_void) -> i32;
  // proto: double QPinchGesture::scaleFactor();
  fn _ZNK13QPinchGesture11scaleFactorEv() -> i32;
}

// body block begin
// class sizeof(QPinchGesture)=1
pub struct QPinchGesture {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPinchGesture {
  pub fn setRotationAngle<T: QPinchGesture_setRotationAngle>(&mut self, value: T) -> i32 {
    value.setRotationAngle(self);
    return 1;
  }
}

pub trait QPinchGesture_setRotationAngle {
  fn setRotationAngle(self, this: &mut QPinchGesture) -> i32;
}

// proto: void QPinchGesture::setRotationAngle(qreal value);
impl<'a> /*trait*/ QPinchGesture_setRotationAngle for (f64) {
  fn setRotationAngle(self, this: &mut QPinchGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGesture16setRotationAngleEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN13QPinchGesture16setRotationAngleEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QPinchGesture {
  pub fn lastScaleFactor<T: QPinchGesture_lastScaleFactor>(&mut self, value: T) -> i32 {
    value.lastScaleFactor(self);
    return 1;
  }
}

pub trait QPinchGesture_lastScaleFactor {
  fn lastScaleFactor(self, this: &mut QPinchGesture) -> i32;
}

// proto: double QPinchGesture::lastScaleFactor();
impl<'a> /*trait*/ QPinchGesture_lastScaleFactor for () {
  fn lastScaleFactor(self, this: &mut QPinchGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPinchGesture15lastScaleFactorEv()};
    unsafe {_ZNK13QPinchGesture15lastScaleFactorEv()};
    return 1;
  }
}

impl /*struct*/ QPinchGesture {
  pub fn lastRotationAngle<T: QPinchGesture_lastRotationAngle>(&mut self, value: T) -> i32 {
    value.lastRotationAngle(self);
    return 1;
  }
}

pub trait QPinchGesture_lastRotationAngle {
  fn lastRotationAngle(self, this: &mut QPinchGesture) -> i32;
}

// proto: double QPinchGesture::lastRotationAngle();
impl<'a> /*trait*/ QPinchGesture_lastRotationAngle for () {
  fn lastRotationAngle(self, this: &mut QPinchGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPinchGesture17lastRotationAngleEv()};
    unsafe {_ZNK13QPinchGesture17lastRotationAngleEv()};
    return 1;
  }
}

impl /*struct*/ QPinchGesture {
  pub fn startCenterPoint<T: QPinchGesture_startCenterPoint>(&mut self, value: T) -> i32 {
    value.startCenterPoint(self);
    return 1;
  }
}

pub trait QPinchGesture_startCenterPoint {
  fn startCenterPoint(self, this: &mut QPinchGesture) -> i32;
}

// proto: QPointF QPinchGesture::startCenterPoint();
impl<'a> /*trait*/ QPinchGesture_startCenterPoint for () {
  fn startCenterPoint(self, this: &mut QPinchGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPinchGesture16startCenterPointEv()};
    unsafe {_ZNK13QPinchGesture16startCenterPointEv()};
    return 1;
  }
}

impl /*struct*/ QPinchGesture {
  pub fn rotationAngle<T: QPinchGesture_rotationAngle>(&mut self, value: T) -> i32 {
    value.rotationAngle(self);
    return 1;
  }
}

pub trait QPinchGesture_rotationAngle {
  fn rotationAngle(self, this: &mut QPinchGesture) -> i32;
}

// proto: double QPinchGesture::rotationAngle();
impl<'a> /*trait*/ QPinchGesture_rotationAngle for () {
  fn rotationAngle(self, this: &mut QPinchGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPinchGesture13rotationAngleEv()};
    unsafe {_ZNK13QPinchGesture13rotationAngleEv()};
    return 1;
  }
}

impl /*struct*/ QPinchGesture {
  pub fn lastCenterPoint<T: QPinchGesture_lastCenterPoint>(&mut self, value: T) -> i32 {
    value.lastCenterPoint(self);
    return 1;
  }
}

pub trait QPinchGesture_lastCenterPoint {
  fn lastCenterPoint(self, this: &mut QPinchGesture) -> i32;
}

// proto: QPointF QPinchGesture::lastCenterPoint();
impl<'a> /*trait*/ QPinchGesture_lastCenterPoint for () {
  fn lastCenterPoint(self, this: &mut QPinchGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPinchGesture15lastCenterPointEv()};
    unsafe {_ZNK13QPinchGesture15lastCenterPointEv()};
    return 1;
  }
}

impl /*struct*/ QPinchGesture {
  pub fn NewQPinchGesture<T: QPinchGesture_NewQPinchGesture>(value: T) -> QPinchGesture {
    let rsthis = value.NewQPinchGesture();
    return rsthis;
    // return 1;
  }
}

pub trait QPinchGesture_NewQPinchGesture {
  fn NewQPinchGesture(self) -> QPinchGesture;
}

// proto: void QPinchGesture::NewQPinchGesture(QObject * parent);
impl<'a> /*trait*/ QPinchGesture_NewQPinchGesture for (&'a mut QObject) {
  fn NewQPinchGesture(self) -> QPinchGesture {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGestureC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QPinchGestureC1EP7QObject(qthis, arg0)};
    let rsthis = QPinchGesture{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPinchGesture {
  pub fn totalScaleFactor<T: QPinchGesture_totalScaleFactor>(&mut self, value: T) -> i32 {
    value.totalScaleFactor(self);
    return 1;
  }
}

pub trait QPinchGesture_totalScaleFactor {
  fn totalScaleFactor(self, this: &mut QPinchGesture) -> i32;
}

// proto: double QPinchGesture::totalScaleFactor();
impl<'a> /*trait*/ QPinchGesture_totalScaleFactor for () {
  fn totalScaleFactor(self, this: &mut QPinchGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPinchGesture16totalScaleFactorEv()};
    unsafe {_ZNK13QPinchGesture16totalScaleFactorEv()};
    return 1;
  }
}

impl /*struct*/ QPinchGesture {
  pub fn setTotalScaleFactor<T: QPinchGesture_setTotalScaleFactor>(&mut self, value: T) -> i32 {
    value.setTotalScaleFactor(self);
    return 1;
  }
}

pub trait QPinchGesture_setTotalScaleFactor {
  fn setTotalScaleFactor(self, this: &mut QPinchGesture) -> i32;
}

// proto: void QPinchGesture::setTotalScaleFactor(qreal value);
impl<'a> /*trait*/ QPinchGesture_setTotalScaleFactor for (f64) {
  fn setTotalScaleFactor(self, this: &mut QPinchGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGesture19setTotalScaleFactorEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN13QPinchGesture19setTotalScaleFactorEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QPinchGesture {
  pub fn totalRotationAngle<T: QPinchGesture_totalRotationAngle>(&mut self, value: T) -> i32 {
    value.totalRotationAngle(self);
    return 1;
  }
}

pub trait QPinchGesture_totalRotationAngle {
  fn totalRotationAngle(self, this: &mut QPinchGesture) -> i32;
}

// proto: double QPinchGesture::totalRotationAngle();
impl<'a> /*trait*/ QPinchGesture_totalRotationAngle for () {
  fn totalRotationAngle(self, this: &mut QPinchGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPinchGesture18totalRotationAngleEv()};
    unsafe {_ZNK13QPinchGesture18totalRotationAngleEv()};
    return 1;
  }
}

impl /*struct*/ QPinchGesture {
  pub fn setLastScaleFactor<T: QPinchGesture_setLastScaleFactor>(&mut self, value: T) -> i32 {
    value.setLastScaleFactor(self);
    return 1;
  }
}

pub trait QPinchGesture_setLastScaleFactor {
  fn setLastScaleFactor(self, this: &mut QPinchGesture) -> i32;
}

// proto: void QPinchGesture::setLastScaleFactor(qreal value);
impl<'a> /*trait*/ QPinchGesture_setLastScaleFactor for (f64) {
  fn setLastScaleFactor(self, this: &mut QPinchGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGesture18setLastScaleFactorEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN13QPinchGesture18setLastScaleFactorEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QPinchGesture {
  pub fn setLastCenterPoint<T: QPinchGesture_setLastCenterPoint>(&mut self, value: T) -> i32 {
    value.setLastCenterPoint(self);
    return 1;
  }
}

pub trait QPinchGesture_setLastCenterPoint {
  fn setLastCenterPoint(self, this: &mut QPinchGesture) -> i32;
}

// proto: void QPinchGesture::setLastCenterPoint(const QPointF & value);
impl<'a> /*trait*/ QPinchGesture_setLastCenterPoint for (&'a  QPointF) {
  fn setLastCenterPoint(self, this: &mut QPinchGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGesture18setLastCenterPointERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QPinchGesture18setLastCenterPointERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QPinchGesture {
  pub fn metaObject<T: QPinchGesture_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QPinchGesture_metaObject {
  fn metaObject(self, this: &mut QPinchGesture) -> i32;
}

// proto: const QMetaObject * QPinchGesture::metaObject();
impl<'a> /*trait*/ QPinchGesture_metaObject for () {
  fn metaObject(self, this: &mut QPinchGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPinchGesture10metaObjectEv()};
    unsafe {_ZNK13QPinchGesture10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QPinchGesture {
  pub fn setLastRotationAngle<T: QPinchGesture_setLastRotationAngle>(&mut self, value: T) -> i32 {
    value.setLastRotationAngle(self);
    return 1;
  }
}

pub trait QPinchGesture_setLastRotationAngle {
  fn setLastRotationAngle(self, this: &mut QPinchGesture) -> i32;
}

// proto: void QPinchGesture::setLastRotationAngle(qreal value);
impl<'a> /*trait*/ QPinchGesture_setLastRotationAngle for (f64) {
  fn setLastRotationAngle(self, this: &mut QPinchGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGesture20setLastRotationAngleEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN13QPinchGesture20setLastRotationAngleEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QPinchGesture {
  pub fn centerPoint<T: QPinchGesture_centerPoint>(&mut self, value: T) -> i32 {
    value.centerPoint(self);
    return 1;
  }
}

pub trait QPinchGesture_centerPoint {
  fn centerPoint(self, this: &mut QPinchGesture) -> i32;
}

// proto: QPointF QPinchGesture::centerPoint();
impl<'a> /*trait*/ QPinchGesture_centerPoint for () {
  fn centerPoint(self, this: &mut QPinchGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPinchGesture11centerPointEv()};
    unsafe {_ZNK13QPinchGesture11centerPointEv()};
    return 1;
  }
}

impl /*struct*/ QPinchGesture {
  pub fn setCenterPoint<T: QPinchGesture_setCenterPoint>(&mut self, value: T) -> i32 {
    value.setCenterPoint(self);
    return 1;
  }
}

pub trait QPinchGesture_setCenterPoint {
  fn setCenterPoint(self, this: &mut QPinchGesture) -> i32;
}

// proto: void QPinchGesture::setCenterPoint(const QPointF & value);
impl<'a> /*trait*/ QPinchGesture_setCenterPoint for (&'a  QPointF) {
  fn setCenterPoint(self, this: &mut QPinchGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGesture14setCenterPointERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QPinchGesture14setCenterPointERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QPinchGesture {
  pub fn setTotalRotationAngle<T: QPinchGesture_setTotalRotationAngle>(&mut self, value: T) -> i32 {
    value.setTotalRotationAngle(self);
    return 1;
  }
}

pub trait QPinchGesture_setTotalRotationAngle {
  fn setTotalRotationAngle(self, this: &mut QPinchGesture) -> i32;
}

// proto: void QPinchGesture::setTotalRotationAngle(qreal value);
impl<'a> /*trait*/ QPinchGesture_setTotalRotationAngle for (f64) {
  fn setTotalRotationAngle(self, this: &mut QPinchGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGesture21setTotalRotationAngleEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN13QPinchGesture21setTotalRotationAngleEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QPinchGesture {
  pub fn setScaleFactor<T: QPinchGesture_setScaleFactor>(&mut self, value: T) -> i32 {
    value.setScaleFactor(self);
    return 1;
  }
}

pub trait QPinchGesture_setScaleFactor {
  fn setScaleFactor(self, this: &mut QPinchGesture) -> i32;
}

// proto: void QPinchGesture::setScaleFactor(qreal value);
impl<'a> /*trait*/ QPinchGesture_setScaleFactor for (f64) {
  fn setScaleFactor(self, this: &mut QPinchGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGesture14setScaleFactorEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN13QPinchGesture14setScaleFactorEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QPinchGesture {
  pub fn FreeQPinchGesture<T: QPinchGesture_FreeQPinchGesture>(&mut self, value: T) -> i32 {
    value.FreeQPinchGesture(self);
    return 1;
  }
}

pub trait QPinchGesture_FreeQPinchGesture {
  fn FreeQPinchGesture(self, this: &mut QPinchGesture) -> i32;
}

// proto: void QPinchGesture::FreeQPinchGesture();
impl<'a> /*trait*/ QPinchGesture_FreeQPinchGesture for () {
  fn FreeQPinchGesture(self, this: &mut QPinchGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGestureD0Ev()};
    unsafe {_ZN13QPinchGestureD0Ev()};
    return 1;
  }
}

impl /*struct*/ QPinchGesture {
  pub fn setStartCenterPoint<T: QPinchGesture_setStartCenterPoint>(&mut self, value: T) -> i32 {
    value.setStartCenterPoint(self);
    return 1;
  }
}

pub trait QPinchGesture_setStartCenterPoint {
  fn setStartCenterPoint(self, this: &mut QPinchGesture) -> i32;
}

// proto: void QPinchGesture::setStartCenterPoint(const QPointF & value);
impl<'a> /*trait*/ QPinchGesture_setStartCenterPoint for (&'a  QPointF) {
  fn setStartCenterPoint(self, this: &mut QPinchGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGesture19setStartCenterPointERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QPinchGesture19setStartCenterPointERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QPinchGesture {
  pub fn scaleFactor<T: QPinchGesture_scaleFactor>(&mut self, value: T) -> i32 {
    value.scaleFactor(self);
    return 1;
  }
}

pub trait QPinchGesture_scaleFactor {
  fn scaleFactor(self, this: &mut QPinchGesture) -> i32;
}

// proto: double QPinchGesture::scaleFactor();
impl<'a> /*trait*/ QPinchGesture_scaleFactor for () {
  fn scaleFactor(self, this: &mut QPinchGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPinchGesture11scaleFactorEv()};
    unsafe {_ZNK13QPinchGesture11scaleFactorEv()};
    return 1;
  }
}

