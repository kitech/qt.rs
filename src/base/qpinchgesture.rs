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
  // proto:  void QPinchGesture::setRotationAngle(qreal value);
  fn _ZN13QPinchGesture16setRotationAngleEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  double QPinchGesture::lastScaleFactor();
  fn _ZNK13QPinchGesture15lastScaleFactorEv(qthis: *mut c_void) -> c_double;
  // proto:  double QPinchGesture::lastRotationAngle();
  fn _ZNK13QPinchGesture17lastRotationAngleEv(qthis: *mut c_void) -> c_double;
  // proto:  QPointF QPinchGesture::startCenterPoint();
  fn _ZNK13QPinchGesture16startCenterPointEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  double QPinchGesture::rotationAngle();
  fn _ZNK13QPinchGesture13rotationAngleEv(qthis: *mut c_void) -> c_double;
  // proto:  QPointF QPinchGesture::lastCenterPoint();
  fn _ZNK13QPinchGesture15lastCenterPointEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPinchGesture::NewQPinchGesture(QObject * parent);
  fn _ZN13QPinchGestureC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  double QPinchGesture::totalScaleFactor();
  fn _ZNK13QPinchGesture16totalScaleFactorEv(qthis: *mut c_void) -> c_double;
  // proto:  void QPinchGesture::setTotalScaleFactor(qreal value);
  fn _ZN13QPinchGesture19setTotalScaleFactorEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  double QPinchGesture::totalRotationAngle();
  fn _ZNK13QPinchGesture18totalRotationAngleEv(qthis: *mut c_void) -> c_double;
  // proto:  void QPinchGesture::setLastScaleFactor(qreal value);
  fn _ZN13QPinchGesture18setLastScaleFactorEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QPinchGesture::setLastCenterPoint(const QPointF & value);
  fn _ZN13QPinchGesture18setLastCenterPointERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QMetaObject * QPinchGesture::metaObject();
  fn _ZNK13QPinchGesture10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QPinchGesture::setLastRotationAngle(qreal value);
  fn _ZN13QPinchGesture20setLastRotationAngleEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  QPointF QPinchGesture::centerPoint();
  fn _ZNK13QPinchGesture11centerPointEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPinchGesture::setCenterPoint(const QPointF & value);
  fn _ZN13QPinchGesture14setCenterPointERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPinchGesture::setTotalRotationAngle(qreal value);
  fn _ZN13QPinchGesture21setTotalRotationAngleEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QPinchGesture::setScaleFactor(qreal value);
  fn _ZN13QPinchGesture14setScaleFactorEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QPinchGesture::FreeQPinchGesture();
  fn _ZN13QPinchGestureD0Ev(qthis: *mut c_void) ;
  // proto:  void QPinchGesture::setStartCenterPoint(const QPointF & value);
  fn _ZN13QPinchGesture19setStartCenterPointERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  double QPinchGesture::scaleFactor();
  fn _ZNK13QPinchGesture11scaleFactorEv(qthis: *mut c_void) -> c_double;
}

// body block begin
// class sizeof(QPinchGesture)=1
pub struct QPinchGesture {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPinchGesture {
  pub fn setRotationAngle<T: QPinchGesture_setRotationAngle>(&mut self, value: T)  {
     value.setRotationAngle(self);
    // return 1;
  }
}

pub trait QPinchGesture_setRotationAngle {
  fn setRotationAngle(self, rsthis: &mut QPinchGesture) ;
}

// proto:  void QPinchGesture::setRotationAngle(qreal value);
impl<'a> /*trait*/ QPinchGesture_setRotationAngle for (f64) {
  fn setRotationAngle(self, rsthis: &mut QPinchGesture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGesture16setRotationAngleEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN13QPinchGesture16setRotationAngleEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPinchGesture {
  pub fn lastScaleFactor<T: QPinchGesture_lastScaleFactor>(&mut self, value: T) -> f64 {
    return value.lastScaleFactor(self);
    // return 1;
  }
}

pub trait QPinchGesture_lastScaleFactor {
  fn lastScaleFactor(self, rsthis: &mut QPinchGesture) -> f64;
}

// proto:  double QPinchGesture::lastScaleFactor();
impl<'a> /*trait*/ QPinchGesture_lastScaleFactor for () {
  fn lastScaleFactor(self, rsthis: &mut QPinchGesture) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPinchGesture15lastScaleFactorEv()};
    let mut ret = unsafe {_ZNK13QPinchGesture15lastScaleFactorEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QPinchGesture {
  pub fn lastRotationAngle<T: QPinchGesture_lastRotationAngle>(&mut self, value: T) -> f64 {
    return value.lastRotationAngle(self);
    // return 1;
  }
}

pub trait QPinchGesture_lastRotationAngle {
  fn lastRotationAngle(self, rsthis: &mut QPinchGesture) -> f64;
}

// proto:  double QPinchGesture::lastRotationAngle();
impl<'a> /*trait*/ QPinchGesture_lastRotationAngle for () {
  fn lastRotationAngle(self, rsthis: &mut QPinchGesture) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPinchGesture17lastRotationAngleEv()};
    let mut ret = unsafe {_ZNK13QPinchGesture17lastRotationAngleEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QPinchGesture {
  pub fn startCenterPoint<T: QPinchGesture_startCenterPoint>(&mut self, value: T) -> QPointF {
    return value.startCenterPoint(self);
    // return 1;
  }
}

pub trait QPinchGesture_startCenterPoint {
  fn startCenterPoint(self, rsthis: &mut QPinchGesture) -> QPointF;
}

// proto:  QPointF QPinchGesture::startCenterPoint();
impl<'a> /*trait*/ QPinchGesture_startCenterPoint for () {
  fn startCenterPoint(self, rsthis: &mut QPinchGesture) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPinchGesture16startCenterPointEv()};
    let mut ret = unsafe {_ZNK13QPinchGesture16startCenterPointEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPinchGesture {
  pub fn rotationAngle<T: QPinchGesture_rotationAngle>(&mut self, value: T) -> f64 {
    return value.rotationAngle(self);
    // return 1;
  }
}

pub trait QPinchGesture_rotationAngle {
  fn rotationAngle(self, rsthis: &mut QPinchGesture) -> f64;
}

// proto:  double QPinchGesture::rotationAngle();
impl<'a> /*trait*/ QPinchGesture_rotationAngle for () {
  fn rotationAngle(self, rsthis: &mut QPinchGesture) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPinchGesture13rotationAngleEv()};
    let mut ret = unsafe {_ZNK13QPinchGesture13rotationAngleEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QPinchGesture {
  pub fn lastCenterPoint<T: QPinchGesture_lastCenterPoint>(&mut self, value: T) -> QPointF {
    return value.lastCenterPoint(self);
    // return 1;
  }
}

pub trait QPinchGesture_lastCenterPoint {
  fn lastCenterPoint(self, rsthis: &mut QPinchGesture) -> QPointF;
}

// proto:  QPointF QPinchGesture::lastCenterPoint();
impl<'a> /*trait*/ QPinchGesture_lastCenterPoint for () {
  fn lastCenterPoint(self, rsthis: &mut QPinchGesture) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPinchGesture15lastCenterPointEv()};
    let mut ret = unsafe {_ZNK13QPinchGesture15lastCenterPointEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
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
  pub fn totalScaleFactor<T: QPinchGesture_totalScaleFactor>(&mut self, value: T) -> f64 {
    return value.totalScaleFactor(self);
    // return 1;
  }
}

pub trait QPinchGesture_totalScaleFactor {
  fn totalScaleFactor(self, rsthis: &mut QPinchGesture) -> f64;
}

// proto:  double QPinchGesture::totalScaleFactor();
impl<'a> /*trait*/ QPinchGesture_totalScaleFactor for () {
  fn totalScaleFactor(self, rsthis: &mut QPinchGesture) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPinchGesture16totalScaleFactorEv()};
    let mut ret = unsafe {_ZNK13QPinchGesture16totalScaleFactorEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QPinchGesture {
  pub fn setTotalScaleFactor<T: QPinchGesture_setTotalScaleFactor>(&mut self, value: T)  {
     value.setTotalScaleFactor(self);
    // return 1;
  }
}

pub trait QPinchGesture_setTotalScaleFactor {
  fn setTotalScaleFactor(self, rsthis: &mut QPinchGesture) ;
}

// proto:  void QPinchGesture::setTotalScaleFactor(qreal value);
impl<'a> /*trait*/ QPinchGesture_setTotalScaleFactor for (f64) {
  fn setTotalScaleFactor(self, rsthis: &mut QPinchGesture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGesture19setTotalScaleFactorEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN13QPinchGesture19setTotalScaleFactorEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPinchGesture {
  pub fn totalRotationAngle<T: QPinchGesture_totalRotationAngle>(&mut self, value: T) -> f64 {
    return value.totalRotationAngle(self);
    // return 1;
  }
}

pub trait QPinchGesture_totalRotationAngle {
  fn totalRotationAngle(self, rsthis: &mut QPinchGesture) -> f64;
}

// proto:  double QPinchGesture::totalRotationAngle();
impl<'a> /*trait*/ QPinchGesture_totalRotationAngle for () {
  fn totalRotationAngle(self, rsthis: &mut QPinchGesture) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPinchGesture18totalRotationAngleEv()};
    let mut ret = unsafe {_ZNK13QPinchGesture18totalRotationAngleEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QPinchGesture {
  pub fn setLastScaleFactor<T: QPinchGesture_setLastScaleFactor>(&mut self, value: T)  {
     value.setLastScaleFactor(self);
    // return 1;
  }
}

pub trait QPinchGesture_setLastScaleFactor {
  fn setLastScaleFactor(self, rsthis: &mut QPinchGesture) ;
}

// proto:  void QPinchGesture::setLastScaleFactor(qreal value);
impl<'a> /*trait*/ QPinchGesture_setLastScaleFactor for (f64) {
  fn setLastScaleFactor(self, rsthis: &mut QPinchGesture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGesture18setLastScaleFactorEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN13QPinchGesture18setLastScaleFactorEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPinchGesture {
  pub fn setLastCenterPoint<T: QPinchGesture_setLastCenterPoint>(&mut self, value: T)  {
     value.setLastCenterPoint(self);
    // return 1;
  }
}

pub trait QPinchGesture_setLastCenterPoint {
  fn setLastCenterPoint(self, rsthis: &mut QPinchGesture) ;
}

// proto:  void QPinchGesture::setLastCenterPoint(const QPointF & value);
impl<'a> /*trait*/ QPinchGesture_setLastCenterPoint for (&'a  QPointF) {
  fn setLastCenterPoint(self, rsthis: &mut QPinchGesture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGesture18setLastCenterPointERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QPinchGesture18setLastCenterPointERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPinchGesture {
  pub fn metaObject<T: QPinchGesture_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QPinchGesture_metaObject {
  fn metaObject(self, rsthis: &mut QPinchGesture) ;
}

// proto:  const QMetaObject * QPinchGesture::metaObject();
impl<'a> /*trait*/ QPinchGesture_metaObject for () {
  fn metaObject(self, rsthis: &mut QPinchGesture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPinchGesture10metaObjectEv()};
     unsafe {_ZNK13QPinchGesture10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPinchGesture {
  pub fn setLastRotationAngle<T: QPinchGesture_setLastRotationAngle>(&mut self, value: T)  {
     value.setLastRotationAngle(self);
    // return 1;
  }
}

pub trait QPinchGesture_setLastRotationAngle {
  fn setLastRotationAngle(self, rsthis: &mut QPinchGesture) ;
}

// proto:  void QPinchGesture::setLastRotationAngle(qreal value);
impl<'a> /*trait*/ QPinchGesture_setLastRotationAngle for (f64) {
  fn setLastRotationAngle(self, rsthis: &mut QPinchGesture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGesture20setLastRotationAngleEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN13QPinchGesture20setLastRotationAngleEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPinchGesture {
  pub fn centerPoint<T: QPinchGesture_centerPoint>(&mut self, value: T) -> QPointF {
    return value.centerPoint(self);
    // return 1;
  }
}

pub trait QPinchGesture_centerPoint {
  fn centerPoint(self, rsthis: &mut QPinchGesture) -> QPointF;
}

// proto:  QPointF QPinchGesture::centerPoint();
impl<'a> /*trait*/ QPinchGesture_centerPoint for () {
  fn centerPoint(self, rsthis: &mut QPinchGesture) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPinchGesture11centerPointEv()};
    let mut ret = unsafe {_ZNK13QPinchGesture11centerPointEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPinchGesture {
  pub fn setCenterPoint<T: QPinchGesture_setCenterPoint>(&mut self, value: T)  {
     value.setCenterPoint(self);
    // return 1;
  }
}

pub trait QPinchGesture_setCenterPoint {
  fn setCenterPoint(self, rsthis: &mut QPinchGesture) ;
}

// proto:  void QPinchGesture::setCenterPoint(const QPointF & value);
impl<'a> /*trait*/ QPinchGesture_setCenterPoint for (&'a  QPointF) {
  fn setCenterPoint(self, rsthis: &mut QPinchGesture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGesture14setCenterPointERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QPinchGesture14setCenterPointERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPinchGesture {
  pub fn setTotalRotationAngle<T: QPinchGesture_setTotalRotationAngle>(&mut self, value: T)  {
     value.setTotalRotationAngle(self);
    // return 1;
  }
}

pub trait QPinchGesture_setTotalRotationAngle {
  fn setTotalRotationAngle(self, rsthis: &mut QPinchGesture) ;
}

// proto:  void QPinchGesture::setTotalRotationAngle(qreal value);
impl<'a> /*trait*/ QPinchGesture_setTotalRotationAngle for (f64) {
  fn setTotalRotationAngle(self, rsthis: &mut QPinchGesture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGesture21setTotalRotationAngleEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN13QPinchGesture21setTotalRotationAngleEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPinchGesture {
  pub fn setScaleFactor<T: QPinchGesture_setScaleFactor>(&mut self, value: T)  {
     value.setScaleFactor(self);
    // return 1;
  }
}

pub trait QPinchGesture_setScaleFactor {
  fn setScaleFactor(self, rsthis: &mut QPinchGesture) ;
}

// proto:  void QPinchGesture::setScaleFactor(qreal value);
impl<'a> /*trait*/ QPinchGesture_setScaleFactor for (f64) {
  fn setScaleFactor(self, rsthis: &mut QPinchGesture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGesture14setScaleFactorEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN13QPinchGesture14setScaleFactorEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPinchGesture {
  pub fn FreeQPinchGesture<T: QPinchGesture_FreeQPinchGesture>(&mut self, value: T)  {
     value.FreeQPinchGesture(self);
    // return 1;
  }
}

pub trait QPinchGesture_FreeQPinchGesture {
  fn FreeQPinchGesture(self, rsthis: &mut QPinchGesture) ;
}

// proto:  void QPinchGesture::FreeQPinchGesture();
impl<'a> /*trait*/ QPinchGesture_FreeQPinchGesture for () {
  fn FreeQPinchGesture(self, rsthis: &mut QPinchGesture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGestureD0Ev()};
     unsafe {_ZN13QPinchGestureD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPinchGesture {
  pub fn setStartCenterPoint<T: QPinchGesture_setStartCenterPoint>(&mut self, value: T)  {
     value.setStartCenterPoint(self);
    // return 1;
  }
}

pub trait QPinchGesture_setStartCenterPoint {
  fn setStartCenterPoint(self, rsthis: &mut QPinchGesture) ;
}

// proto:  void QPinchGesture::setStartCenterPoint(const QPointF & value);
impl<'a> /*trait*/ QPinchGesture_setStartCenterPoint for (&'a  QPointF) {
  fn setStartCenterPoint(self, rsthis: &mut QPinchGesture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGesture19setStartCenterPointERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QPinchGesture19setStartCenterPointERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPinchGesture {
  pub fn scaleFactor<T: QPinchGesture_scaleFactor>(&mut self, value: T) -> f64 {
    return value.scaleFactor(self);
    // return 1;
  }
}

pub trait QPinchGesture_scaleFactor {
  fn scaleFactor(self, rsthis: &mut QPinchGesture) -> f64;
}

// proto:  double QPinchGesture::scaleFactor();
impl<'a> /*trait*/ QPinchGesture_scaleFactor for () {
  fn scaleFactor(self, rsthis: &mut QPinchGesture) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPinchGesture11scaleFactorEv()};
    let mut ret = unsafe {_ZNK13QPinchGesture11scaleFactorEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

