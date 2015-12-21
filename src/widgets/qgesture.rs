// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtWidgets/qgesture.h
// dst-file: /src/widgets/qgesture.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use super::super::core::qobject::QObject; // 771
use super::super::core::qpoint::QPointF; // 771
// use super::qgesture::QGesture; // 773
use super::qwidget::QWidget; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QSwipeGesture::~QSwipeGesture();
  fn _ZN13QSwipeGestureD0Ev(qthis: *mut c_void);
  // proto:  const QMetaObject * QSwipeGesture::metaObject();
  fn _ZNK13QSwipeGesture10metaObjectEv(qthis: *mut c_void);
  // proto:  void QSwipeGesture::setSwipeAngle(qreal value);
  fn _ZN13QSwipeGesture13setSwipeAngleEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QSwipeGesture::QSwipeGesture(QObject * parent);
  fn _ZN13QSwipeGestureC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  qreal QSwipeGesture::swipeAngle();
  fn _ZNK13QSwipeGesture10swipeAngleEv(qthis: *mut c_void) -> c_double;
  // proto:  QPointF QGesture::hotSpot();
  fn _ZNK8QGesture7hotSpotEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QGesture::hasHotSpot();
  fn _ZNK8QGesture10hasHotSpotEv(qthis: *mut c_void) -> c_char;
  // proto:  void QGesture::unsetHotSpot();
  fn _ZN8QGesture12unsetHotSpotEv(qthis: *mut c_void);
  // proto:  const QMetaObject * QGesture::metaObject();
  fn _ZNK8QGesture10metaObjectEv(qthis: *mut c_void);
  // proto:  void QGesture::QGesture(QObject * parent);
  fn _ZN8QGestureC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGesture::setHotSpot(const QPointF & value);
  fn _ZN8QGesture10setHotSpotERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGesture::~QGesture();
  fn _ZN8QGestureD0Ev(qthis: *mut c_void);
  // proto:  bool QGestureEvent::isAccepted(QGesture * );
  fn _ZNK13QGestureEvent10isAcceptedEP8QGesture(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  QWidget * QGestureEvent::widget();
  fn _ZNK13QGestureEvent6widgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGestureEvent::ignore(QGesture * );
  fn _ZN13QGestureEvent6ignoreEP8QGesture(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGestureEvent::accept(QGesture * );
  fn _ZN13QGestureEvent6acceptEP8QGesture(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QList<QGesture *> QGestureEvent::activeGestures();
  fn _ZNK13QGestureEvent14activeGesturesEv(qthis: *mut c_void);
  // proto:  QList<QGesture *> QGestureEvent::gestures();
  fn _ZNK13QGestureEvent8gesturesEv(qthis: *mut c_void);
  // proto:  void QGestureEvent::setAccepted(QGesture * , bool );
  fn _ZN13QGestureEvent11setAcceptedEP8QGestureb(qthis: *mut c_void, arg0: *mut c_void, arg1: c_char);
  // proto:  void QGestureEvent::setWidget(QWidget * widget);
  fn _ZN13QGestureEvent9setWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGestureEvent::~QGestureEvent();
  fn _ZN13QGestureEventD0Ev(qthis: *mut c_void);
  // proto:  QList<QGesture *> QGestureEvent::canceledGestures();
  fn _ZNK13QGestureEvent16canceledGesturesEv(qthis: *mut c_void);
  // proto:  QPointF QGestureEvent::mapToGraphicsScene(const QPointF & gesturePoint);
  fn _ZNK13QGestureEvent18mapToGraphicsSceneERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QPointF QPanGesture::offset();
  fn _ZNK11QPanGesture6offsetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPointF QPanGesture::delta();
  fn _ZNK11QPanGesture5deltaEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPanGesture::setOffset(const QPointF & value);
  fn _ZN11QPanGesture9setOffsetERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  qreal QPanGesture::acceleration();
  fn _ZNK11QPanGesture12accelerationEv(qthis: *mut c_void) -> c_double;
  // proto:  void QPanGesture::~QPanGesture();
  fn _ZN11QPanGestureD0Ev(qthis: *mut c_void);
  // proto:  void QPanGesture::QPanGesture(QObject * parent);
  fn _ZN11QPanGestureC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QPanGesture::metaObject();
  fn _ZNK11QPanGesture10metaObjectEv(qthis: *mut c_void);
  // proto:  void QPanGesture::setAcceleration(qreal value);
  fn _ZN11QPanGesture15setAccelerationEd(qthis: *mut c_void, arg0: c_double);
  // proto:  QPointF QPanGesture::lastOffset();
  fn _ZNK11QPanGesture10lastOffsetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPanGesture::setLastOffset(const QPointF & value);
  fn _ZN11QPanGesture13setLastOffsetERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTapAndHoldGesture::QTapAndHoldGesture(QObject * parent);
  fn _ZN18QTapAndHoldGestureC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTapAndHoldGesture::~QTapAndHoldGesture();
  fn _ZN18QTapAndHoldGestureD0Ev(qthis: *mut c_void);
  // proto:  QPointF QTapAndHoldGesture::position();
  fn _ZNK18QTapAndHoldGesture8positionEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static void QTapAndHoldGesture::setTimeout(int msecs);
  fn _ZN18QTapAndHoldGesture10setTimeoutEi(arg0: c_int);
  // proto: static int QTapAndHoldGesture::timeout();
  fn _ZN18QTapAndHoldGesture7timeoutEv() -> c_int;
  // proto:  const QMetaObject * QTapAndHoldGesture::metaObject();
  fn _ZNK18QTapAndHoldGesture10metaObjectEv(qthis: *mut c_void);
  // proto:  void QTapAndHoldGesture::setPosition(const QPointF & pos);
  fn _ZN18QTapAndHoldGesture11setPositionERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QPointF QTapGesture::position();
  fn _ZNK11QTapGesture8positionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTapGesture::setPosition(const QPointF & pos);
  fn _ZN11QTapGesture11setPositionERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTapGesture::QTapGesture(QObject * parent);
  fn _ZN11QTapGestureC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QTapGesture::metaObject();
  fn _ZNK11QTapGesture10metaObjectEv(qthis: *mut c_void);
  // proto:  void QTapGesture::~QTapGesture();
  fn _ZN11QTapGestureD0Ev(qthis: *mut c_void);
  // proto:  void QPinchGesture::setRotationAngle(qreal value);
  fn _ZN13QPinchGesture16setRotationAngleEd(qthis: *mut c_void, arg0: c_double);
  // proto:  qreal QPinchGesture::lastScaleFactor();
  fn _ZNK13QPinchGesture15lastScaleFactorEv(qthis: *mut c_void) -> c_double;
  // proto:  qreal QPinchGesture::lastRotationAngle();
  fn _ZNK13QPinchGesture17lastRotationAngleEv(qthis: *mut c_void) -> c_double;
  // proto:  QPointF QPinchGesture::startCenterPoint();
  fn _ZNK13QPinchGesture16startCenterPointEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  qreal QPinchGesture::rotationAngle();
  fn _ZNK13QPinchGesture13rotationAngleEv(qthis: *mut c_void) -> c_double;
  // proto:  QPointF QPinchGesture::lastCenterPoint();
  fn _ZNK13QPinchGesture15lastCenterPointEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPinchGesture::QPinchGesture(QObject * parent);
  fn _ZN13QPinchGestureC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  qreal QPinchGesture::totalScaleFactor();
  fn _ZNK13QPinchGesture16totalScaleFactorEv(qthis: *mut c_void) -> c_double;
  // proto:  void QPinchGesture::setTotalScaleFactor(qreal value);
  fn _ZN13QPinchGesture19setTotalScaleFactorEd(qthis: *mut c_void, arg0: c_double);
  // proto:  qreal QPinchGesture::totalRotationAngle();
  fn _ZNK13QPinchGesture18totalRotationAngleEv(qthis: *mut c_void) -> c_double;
  // proto:  void QPinchGesture::setLastScaleFactor(qreal value);
  fn _ZN13QPinchGesture18setLastScaleFactorEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QPinchGesture::setLastCenterPoint(const QPointF & value);
  fn _ZN13QPinchGesture18setLastCenterPointERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QPinchGesture::metaObject();
  fn _ZNK13QPinchGesture10metaObjectEv(qthis: *mut c_void);
  // proto:  void QPinchGesture::setLastRotationAngle(qreal value);
  fn _ZN13QPinchGesture20setLastRotationAngleEd(qthis: *mut c_void, arg0: c_double);
  // proto:  QPointF QPinchGesture::centerPoint();
  fn _ZNK13QPinchGesture11centerPointEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPinchGesture::setCenterPoint(const QPointF & value);
  fn _ZN13QPinchGesture14setCenterPointERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPinchGesture::setTotalRotationAngle(qreal value);
  fn _ZN13QPinchGesture21setTotalRotationAngleEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QPinchGesture::setScaleFactor(qreal value);
  fn _ZN13QPinchGesture14setScaleFactorEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QPinchGesture::~QPinchGesture();
  fn _ZN13QPinchGestureD0Ev(qthis: *mut c_void);
  // proto:  void QPinchGesture::setStartCenterPoint(const QPointF & value);
  fn _ZN13QPinchGesture19setStartCenterPointERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  qreal QPinchGesture::scaleFactor();
  fn _ZNK13QPinchGesture11scaleFactorEv(qthis: *mut c_void) -> c_double;
} // <= ext block end

// body block begin =>
// class sizeof(QSwipeGesture)=1
pub struct QSwipeGesture {
  pub qclsinst: *mut c_void,
}

// class sizeof(QGesture)=1
pub struct QGesture {
  pub qclsinst: *mut c_void,
}

// class sizeof(QGestureEvent)=1
pub struct QGestureEvent {
  pub qclsinst: *mut c_void,
}

// class sizeof(QPanGesture)=1
pub struct QPanGesture {
  pub qclsinst: *mut c_void,
}

// class sizeof(QTapAndHoldGesture)=1
pub struct QTapAndHoldGesture {
  pub qclsinst: *mut c_void,
}

// class sizeof(QTapGesture)=1
pub struct QTapGesture {
  pub qclsinst: *mut c_void,
}

// class sizeof(QPinchGesture)=1
pub struct QPinchGesture {
  pub qclsinst: *mut c_void,
}

  // proto:  void QSwipeGesture::~QSwipeGesture();
impl /*struct*/ QSwipeGesture {
  pub fn FreeQSwipeGesture<RetType, T: QSwipeGesture_FreeQSwipeGesture<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQSwipeGesture(self);
    // return 1;
  }
}

pub trait QSwipeGesture_FreeQSwipeGesture<RetType> {
  fn FreeQSwipeGesture(self , rsthis: &mut QSwipeGesture) -> RetType;
}

  // proto:  void QSwipeGesture::~QSwipeGesture();
impl<'a> /*trait*/ QSwipeGesture_FreeQSwipeGesture<()> for () {
  fn FreeQSwipeGesture(self , rsthis: &mut QSwipeGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSwipeGestureD0Ev()};
     unsafe {_ZN13QSwipeGestureD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QSwipeGesture::metaObject();
impl /*struct*/ QSwipeGesture {
  pub fn metaObject<RetType, T: QSwipeGesture_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QSwipeGesture_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QSwipeGesture) -> RetType;
}

  // proto:  const QMetaObject * QSwipeGesture::metaObject();
impl<'a> /*trait*/ QSwipeGesture_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QSwipeGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSwipeGesture10metaObjectEv()};
     unsafe {_ZNK13QSwipeGesture10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSwipeGesture::setSwipeAngle(qreal value);
impl /*struct*/ QSwipeGesture {
  pub fn setSwipeAngle<RetType, T: QSwipeGesture_setSwipeAngle<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setSwipeAngle(self);
    // return 1;
  }
}

pub trait QSwipeGesture_setSwipeAngle<RetType> {
  fn setSwipeAngle(self , rsthis: &mut QSwipeGesture) -> RetType;
}

  // proto:  void QSwipeGesture::setSwipeAngle(qreal value);
impl<'a> /*trait*/ QSwipeGesture_setSwipeAngle<()> for (f64) {
  fn setSwipeAngle(self , rsthis: &mut QSwipeGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSwipeGesture13setSwipeAngleEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN13QSwipeGesture13setSwipeAngleEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSwipeGesture::QSwipeGesture(QObject * parent);
impl /*struct*/ QSwipeGesture {
  pub fn NewQSwipeGesture<T: QSwipeGesture_NewQSwipeGesture>(value: T) -> QSwipeGesture {
    let rsthis = value.NewQSwipeGesture();
    return rsthis;
    // return 1;
  }
}

pub trait QSwipeGesture_NewQSwipeGesture {
  fn NewQSwipeGesture(self) -> QSwipeGesture;
}

  // proto:  void QSwipeGesture::QSwipeGesture(QObject * parent);
impl<'a> /*trait*/ QSwipeGesture_NewQSwipeGesture for (QObject) {
  fn NewQSwipeGesture(self) -> QSwipeGesture {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSwipeGestureC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QSwipeGestureC1EP7QObject(qthis, arg0)};
    let rsthis = QSwipeGesture{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  qreal QSwipeGesture::swipeAngle();
impl /*struct*/ QSwipeGesture {
  pub fn swipeAngle<RetType, T: QSwipeGesture_swipeAngle<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.swipeAngle(self);
    // return 1;
  }
}

pub trait QSwipeGesture_swipeAngle<RetType> {
  fn swipeAngle(self , rsthis: &mut QSwipeGesture) -> RetType;
}

  // proto:  qreal QSwipeGesture::swipeAngle();
impl<'a> /*trait*/ QSwipeGesture_swipeAngle<f64> for () {
  fn swipeAngle(self , rsthis: &mut QSwipeGesture) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSwipeGesture10swipeAngleEv()};
    let mut ret = unsafe {_ZNK13QSwipeGesture10swipeAngleEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QPointF QGesture::hotSpot();
impl /*struct*/ QGesture {
  pub fn hotSpot<RetType, T: QGesture_hotSpot<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.hotSpot(self);
    // return 1;
  }
}

pub trait QGesture_hotSpot<RetType> {
  fn hotSpot(self , rsthis: &mut QGesture) -> RetType;
}

  // proto:  QPointF QGesture::hotSpot();
impl<'a> /*trait*/ QGesture_hotSpot<QPointF> for () {
  fn hotSpot(self , rsthis: &mut QGesture) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QGesture7hotSpotEv()};
    let mut ret = unsafe {_ZNK8QGesture7hotSpotEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QGesture::hasHotSpot();
impl /*struct*/ QGesture {
  pub fn hasHotSpot<RetType, T: QGesture_hasHotSpot<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.hasHotSpot(self);
    // return 1;
  }
}

pub trait QGesture_hasHotSpot<RetType> {
  fn hasHotSpot(self , rsthis: &mut QGesture) -> RetType;
}

  // proto:  bool QGesture::hasHotSpot();
impl<'a> /*trait*/ QGesture_hasHotSpot<i8> for () {
  fn hasHotSpot(self , rsthis: &mut QGesture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QGesture10hasHotSpotEv()};
    let mut ret = unsafe {_ZNK8QGesture10hasHotSpotEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGesture::unsetHotSpot();
impl /*struct*/ QGesture {
  pub fn unsetHotSpot<RetType, T: QGesture_unsetHotSpot<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.unsetHotSpot(self);
    // return 1;
  }
}

pub trait QGesture_unsetHotSpot<RetType> {
  fn unsetHotSpot(self , rsthis: &mut QGesture) -> RetType;
}

  // proto:  void QGesture::unsetHotSpot();
impl<'a> /*trait*/ QGesture_unsetHotSpot<()> for () {
  fn unsetHotSpot(self , rsthis: &mut QGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QGesture12unsetHotSpotEv()};
     unsafe {_ZN8QGesture12unsetHotSpotEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QGesture::metaObject();
impl /*struct*/ QGesture {
  pub fn metaObject<RetType, T: QGesture_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QGesture_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QGesture) -> RetType;
}

  // proto:  const QMetaObject * QGesture::metaObject();
impl<'a> /*trait*/ QGesture_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QGesture10metaObjectEv()};
     unsafe {_ZNK8QGesture10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGesture::QGesture(QObject * parent);
impl /*struct*/ QGesture {
  pub fn NewQGesture<T: QGesture_NewQGesture>(value: T) -> QGesture {
    let rsthis = value.NewQGesture();
    return rsthis;
    // return 1;
  }
}

pub trait QGesture_NewQGesture {
  fn NewQGesture(self) -> QGesture;
}

  // proto:  void QGesture::QGesture(QObject * parent);
impl<'a> /*trait*/ QGesture_NewQGesture for (QObject) {
  fn NewQGesture(self) -> QGesture {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QGestureC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QGestureC1EP7QObject(qthis, arg0)};
    let rsthis = QGesture{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGesture::setHotSpot(const QPointF & value);
impl /*struct*/ QGesture {
  pub fn setHotSpot<RetType, T: QGesture_setHotSpot<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setHotSpot(self);
    // return 1;
  }
}

pub trait QGesture_setHotSpot<RetType> {
  fn setHotSpot(self , rsthis: &mut QGesture) -> RetType;
}

  // proto:  void QGesture::setHotSpot(const QPointF & value);
impl<'a> /*trait*/ QGesture_setHotSpot<()> for (QPointF) {
  fn setHotSpot(self , rsthis: &mut QGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QGesture10setHotSpotERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QGesture10setHotSpotERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGesture::~QGesture();
impl /*struct*/ QGesture {
  pub fn FreeQGesture<RetType, T: QGesture_FreeQGesture<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQGesture(self);
    // return 1;
  }
}

pub trait QGesture_FreeQGesture<RetType> {
  fn FreeQGesture(self , rsthis: &mut QGesture) -> RetType;
}

  // proto:  void QGesture::~QGesture();
impl<'a> /*trait*/ QGesture_FreeQGesture<()> for () {
  fn FreeQGesture(self , rsthis: &mut QGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QGestureD0Ev()};
     unsafe {_ZN8QGestureD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QGestureEvent::isAccepted(QGesture * );
impl /*struct*/ QGestureEvent {
  pub fn isAccepted<RetType, T: QGestureEvent_isAccepted<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isAccepted(self);
    // return 1;
  }
}

pub trait QGestureEvent_isAccepted<RetType> {
  fn isAccepted(self , rsthis: &mut QGestureEvent) -> RetType;
}

  // proto:  bool QGestureEvent::isAccepted(QGesture * );
impl<'a> /*trait*/ QGestureEvent_isAccepted<i8> for (QGesture) {
  fn isAccepted(self , rsthis: &mut QGestureEvent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGestureEvent10isAcceptedEP8QGesture()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGestureEvent10isAcceptedEP8QGesture(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QWidget * QGestureEvent::widget();
impl /*struct*/ QGestureEvent {
  pub fn widget<RetType, T: QGestureEvent_widget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.widget(self);
    // return 1;
  }
}

pub trait QGestureEvent_widget<RetType> {
  fn widget(self , rsthis: &mut QGestureEvent) -> RetType;
}

  // proto:  QWidget * QGestureEvent::widget();
impl<'a> /*trait*/ QGestureEvent_widget<QWidget> for () {
  fn widget(self , rsthis: &mut QGestureEvent) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGestureEvent6widgetEv()};
    let mut ret = unsafe {_ZNK13QGestureEvent6widgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGestureEvent::ignore(QGesture * );
impl /*struct*/ QGestureEvent {
  pub fn ignore<RetType, T: QGestureEvent_ignore<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.ignore(self);
    // return 1;
  }
}

pub trait QGestureEvent_ignore<RetType> {
  fn ignore(self , rsthis: &mut QGestureEvent) -> RetType;
}

  // proto:  void QGestureEvent::ignore(QGesture * );
impl<'a> /*trait*/ QGestureEvent_ignore<()> for (QGesture) {
  fn ignore(self , rsthis: &mut QGestureEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGestureEvent6ignoreEP8QGesture()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QGestureEvent6ignoreEP8QGesture(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGestureEvent::accept(QGesture * );
impl /*struct*/ QGestureEvent {
  pub fn accept<RetType, T: QGestureEvent_accept<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.accept(self);
    // return 1;
  }
}

pub trait QGestureEvent_accept<RetType> {
  fn accept(self , rsthis: &mut QGestureEvent) -> RetType;
}

  // proto:  void QGestureEvent::accept(QGesture * );
impl<'a> /*trait*/ QGestureEvent_accept<()> for (QGesture) {
  fn accept(self , rsthis: &mut QGestureEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGestureEvent6acceptEP8QGesture()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QGestureEvent6acceptEP8QGesture(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QList<QGesture *> QGestureEvent::activeGestures();
impl /*struct*/ QGestureEvent {
  pub fn activeGestures<RetType, T: QGestureEvent_activeGestures<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.activeGestures(self);
    // return 1;
  }
}

pub trait QGestureEvent_activeGestures<RetType> {
  fn activeGestures(self , rsthis: &mut QGestureEvent) -> RetType;
}

  // proto:  QList<QGesture *> QGestureEvent::activeGestures();
impl<'a> /*trait*/ QGestureEvent_activeGestures<()> for () {
  fn activeGestures(self , rsthis: &mut QGestureEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGestureEvent14activeGesturesEv()};
     unsafe {_ZNK13QGestureEvent14activeGesturesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QList<QGesture *> QGestureEvent::gestures();
impl /*struct*/ QGestureEvent {
  pub fn gestures<RetType, T: QGestureEvent_gestures<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.gestures(self);
    // return 1;
  }
}

pub trait QGestureEvent_gestures<RetType> {
  fn gestures(self , rsthis: &mut QGestureEvent) -> RetType;
}

  // proto:  QList<QGesture *> QGestureEvent::gestures();
impl<'a> /*trait*/ QGestureEvent_gestures<()> for () {
  fn gestures(self , rsthis: &mut QGestureEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGestureEvent8gesturesEv()};
     unsafe {_ZNK13QGestureEvent8gesturesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGestureEvent::setAccepted(QGesture * , bool );
impl /*struct*/ QGestureEvent {
  pub fn setAccepted<RetType, T: QGestureEvent_setAccepted<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setAccepted(self);
    // return 1;
  }
}

pub trait QGestureEvent_setAccepted<RetType> {
  fn setAccepted(self , rsthis: &mut QGestureEvent) -> RetType;
}

  // proto:  void QGestureEvent::setAccepted(QGesture * , bool );
impl<'a> /*trait*/ QGestureEvent_setAccepted<()> for (QGesture, i8) {
  fn setAccepted(self , rsthis: &mut QGestureEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGestureEvent11setAcceptedEP8QGestureb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_char;
     unsafe {_ZN13QGestureEvent11setAcceptedEP8QGestureb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QGestureEvent::setWidget(QWidget * widget);
impl /*struct*/ QGestureEvent {
  pub fn setWidget<RetType, T: QGestureEvent_setWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setWidget(self);
    // return 1;
  }
}

pub trait QGestureEvent_setWidget<RetType> {
  fn setWidget(self , rsthis: &mut QGestureEvent) -> RetType;
}

  // proto:  void QGestureEvent::setWidget(QWidget * widget);
impl<'a> /*trait*/ QGestureEvent_setWidget<()> for (QWidget) {
  fn setWidget(self , rsthis: &mut QGestureEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGestureEvent9setWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QGestureEvent9setWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGestureEvent::~QGestureEvent();
impl /*struct*/ QGestureEvent {
  pub fn FreeQGestureEvent<RetType, T: QGestureEvent_FreeQGestureEvent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQGestureEvent(self);
    // return 1;
  }
}

pub trait QGestureEvent_FreeQGestureEvent<RetType> {
  fn FreeQGestureEvent(self , rsthis: &mut QGestureEvent) -> RetType;
}

  // proto:  void QGestureEvent::~QGestureEvent();
impl<'a> /*trait*/ QGestureEvent_FreeQGestureEvent<()> for () {
  fn FreeQGestureEvent(self , rsthis: &mut QGestureEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGestureEventD0Ev()};
     unsafe {_ZN13QGestureEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QList<QGesture *> QGestureEvent::canceledGestures();
impl /*struct*/ QGestureEvent {
  pub fn canceledGestures<RetType, T: QGestureEvent_canceledGestures<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.canceledGestures(self);
    // return 1;
  }
}

pub trait QGestureEvent_canceledGestures<RetType> {
  fn canceledGestures(self , rsthis: &mut QGestureEvent) -> RetType;
}

  // proto:  QList<QGesture *> QGestureEvent::canceledGestures();
impl<'a> /*trait*/ QGestureEvent_canceledGestures<()> for () {
  fn canceledGestures(self , rsthis: &mut QGestureEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGestureEvent16canceledGesturesEv()};
     unsafe {_ZNK13QGestureEvent16canceledGesturesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPointF QGestureEvent::mapToGraphicsScene(const QPointF & gesturePoint);
impl /*struct*/ QGestureEvent {
  pub fn mapToGraphicsScene<RetType, T: QGestureEvent_mapToGraphicsScene<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mapToGraphicsScene(self);
    // return 1;
  }
}

pub trait QGestureEvent_mapToGraphicsScene<RetType> {
  fn mapToGraphicsScene(self , rsthis: &mut QGestureEvent) -> RetType;
}

  // proto:  QPointF QGestureEvent::mapToGraphicsScene(const QPointF & gesturePoint);
impl<'a> /*trait*/ QGestureEvent_mapToGraphicsScene<QPointF> for (QPointF) {
  fn mapToGraphicsScene(self , rsthis: &mut QGestureEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGestureEvent18mapToGraphicsSceneERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGestureEvent18mapToGraphicsSceneERK7QPointF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QPointF QPanGesture::offset();
impl /*struct*/ QPanGesture {
  pub fn offset<RetType, T: QPanGesture_offset<RetType>>(&mut self,  overload_args: T) -> RetType {
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
  pub fn delta<RetType, T: QPanGesture_delta<RetType>>(&mut self,  overload_args: T) -> RetType {
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
  pub fn setOffset<RetType, T: QPanGesture_setOffset<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setOffset(self);
    // return 1;
  }
}

pub trait QPanGesture_setOffset<RetType> {
  fn setOffset(self , rsthis: &mut QPanGesture) -> RetType;
}

  // proto:  void QPanGesture::setOffset(const QPointF & value);
impl<'a> /*trait*/ QPanGesture_setOffset<()> for (QPointF) {
  fn setOffset(self , rsthis: &mut QPanGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPanGesture9setOffsetERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QPanGesture9setOffsetERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QPanGesture::acceleration();
impl /*struct*/ QPanGesture {
  pub fn acceleration<RetType, T: QPanGesture_acceleration<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.acceleration(self);
    // return 1;
  }
}

pub trait QPanGesture_acceleration<RetType> {
  fn acceleration(self , rsthis: &mut QPanGesture) -> RetType;
}

  // proto:  qreal QPanGesture::acceleration();
impl<'a> /*trait*/ QPanGesture_acceleration<f64> for () {
  fn acceleration(self , rsthis: &mut QPanGesture) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPanGesture12accelerationEv()};
    let mut ret = unsafe {_ZNK11QPanGesture12accelerationEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QPanGesture::~QPanGesture();
impl /*struct*/ QPanGesture {
  pub fn FreeQPanGesture<RetType, T: QPanGesture_FreeQPanGesture<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQPanGesture(self);
    // return 1;
  }
}

pub trait QPanGesture_FreeQPanGesture<RetType> {
  fn FreeQPanGesture(self , rsthis: &mut QPanGesture) -> RetType;
}

  // proto:  void QPanGesture::~QPanGesture();
impl<'a> /*trait*/ QPanGesture_FreeQPanGesture<()> for () {
  fn FreeQPanGesture(self , rsthis: &mut QPanGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPanGestureD0Ev()};
     unsafe {_ZN11QPanGestureD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPanGesture::QPanGesture(QObject * parent);
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

  // proto:  void QPanGesture::QPanGesture(QObject * parent);
impl<'a> /*trait*/ QPanGesture_NewQPanGesture for (QObject) {
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
  pub fn metaObject<RetType, T: QPanGesture_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
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
  pub fn setAcceleration<RetType, T: QPanGesture_setAcceleration<RetType>>(&mut self,  overload_args: T) -> RetType {
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
  pub fn lastOffset<RetType, T: QPanGesture_lastOffset<RetType>>(&mut self,  overload_args: T) -> RetType {
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
  pub fn setLastOffset<RetType, T: QPanGesture_setLastOffset<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setLastOffset(self);
    // return 1;
  }
}

pub trait QPanGesture_setLastOffset<RetType> {
  fn setLastOffset(self , rsthis: &mut QPanGesture) -> RetType;
}

  // proto:  void QPanGesture::setLastOffset(const QPointF & value);
impl<'a> /*trait*/ QPanGesture_setLastOffset<()> for (QPointF) {
  fn setLastOffset(self , rsthis: &mut QPanGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPanGesture13setLastOffsetERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QPanGesture13setLastOffsetERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTapAndHoldGesture::QTapAndHoldGesture(QObject * parent);
impl /*struct*/ QTapAndHoldGesture {
  pub fn NewQTapAndHoldGesture<T: QTapAndHoldGesture_NewQTapAndHoldGesture>(value: T) -> QTapAndHoldGesture {
    let rsthis = value.NewQTapAndHoldGesture();
    return rsthis;
    // return 1;
  }
}

pub trait QTapAndHoldGesture_NewQTapAndHoldGesture {
  fn NewQTapAndHoldGesture(self) -> QTapAndHoldGesture;
}

  // proto:  void QTapAndHoldGesture::QTapAndHoldGesture(QObject * parent);
impl<'a> /*trait*/ QTapAndHoldGesture_NewQTapAndHoldGesture for (QObject) {
  fn NewQTapAndHoldGesture(self) -> QTapAndHoldGesture {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QTapAndHoldGestureC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QTapAndHoldGestureC1EP7QObject(qthis, arg0)};
    let rsthis = QTapAndHoldGesture{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTapAndHoldGesture::~QTapAndHoldGesture();
impl /*struct*/ QTapAndHoldGesture {
  pub fn FreeQTapAndHoldGesture<RetType, T: QTapAndHoldGesture_FreeQTapAndHoldGesture<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQTapAndHoldGesture(self);
    // return 1;
  }
}

pub trait QTapAndHoldGesture_FreeQTapAndHoldGesture<RetType> {
  fn FreeQTapAndHoldGesture(self , rsthis: &mut QTapAndHoldGesture) -> RetType;
}

  // proto:  void QTapAndHoldGesture::~QTapAndHoldGesture();
impl<'a> /*trait*/ QTapAndHoldGesture_FreeQTapAndHoldGesture<()> for () {
  fn FreeQTapAndHoldGesture(self , rsthis: &mut QTapAndHoldGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QTapAndHoldGestureD0Ev()};
     unsafe {_ZN18QTapAndHoldGestureD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPointF QTapAndHoldGesture::position();
impl /*struct*/ QTapAndHoldGesture {
  pub fn position<RetType, T: QTapAndHoldGesture_position<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.position(self);
    // return 1;
  }
}

pub trait QTapAndHoldGesture_position<RetType> {
  fn position(self , rsthis: &mut QTapAndHoldGesture) -> RetType;
}

  // proto:  QPointF QTapAndHoldGesture::position();
impl<'a> /*trait*/ QTapAndHoldGesture_position<QPointF> for () {
  fn position(self , rsthis: &mut QTapAndHoldGesture) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QTapAndHoldGesture8positionEv()};
    let mut ret = unsafe {_ZNK18QTapAndHoldGesture8positionEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto: static void QTapAndHoldGesture::setTimeout(int msecs);
impl /*struct*/ QTapAndHoldGesture {
  pub fn setTimeout_s<RetType, T: QTapAndHoldGesture_setTimeout_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setTimeout_s();
    // return 1;
  }
}

pub trait QTapAndHoldGesture_setTimeout_s<RetType> {
  fn setTimeout_s(self ) -> RetType;
}

  // proto: static void QTapAndHoldGesture::setTimeout(int msecs);
impl<'a> /*trait*/ QTapAndHoldGesture_setTimeout_s<()> for (i32) {
  fn setTimeout_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QTapAndHoldGesture10setTimeoutEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN18QTapAndHoldGesture10setTimeoutEi(arg0)};
    // return 1;
  }
}

  // proto: static int QTapAndHoldGesture::timeout();
impl /*struct*/ QTapAndHoldGesture {
  pub fn timeout_s<RetType, T: QTapAndHoldGesture_timeout_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.timeout_s();
    // return 1;
  }
}

pub trait QTapAndHoldGesture_timeout_s<RetType> {
  fn timeout_s(self ) -> RetType;
}

  // proto: static int QTapAndHoldGesture::timeout();
impl<'a> /*trait*/ QTapAndHoldGesture_timeout_s<i32> for () {
  fn timeout_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QTapAndHoldGesture7timeoutEv()};
    let mut ret = unsafe {_ZN18QTapAndHoldGesture7timeoutEv()};
    return ret as i32;
    // return 1;
  }
}

  // proto:  const QMetaObject * QTapAndHoldGesture::metaObject();
impl /*struct*/ QTapAndHoldGesture {
  pub fn metaObject<RetType, T: QTapAndHoldGesture_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QTapAndHoldGesture_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QTapAndHoldGesture) -> RetType;
}

  // proto:  const QMetaObject * QTapAndHoldGesture::metaObject();
impl<'a> /*trait*/ QTapAndHoldGesture_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QTapAndHoldGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QTapAndHoldGesture10metaObjectEv()};
     unsafe {_ZNK18QTapAndHoldGesture10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTapAndHoldGesture::setPosition(const QPointF & pos);
impl /*struct*/ QTapAndHoldGesture {
  pub fn setPosition<RetType, T: QTapAndHoldGesture_setPosition<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setPosition(self);
    // return 1;
  }
}

pub trait QTapAndHoldGesture_setPosition<RetType> {
  fn setPosition(self , rsthis: &mut QTapAndHoldGesture) -> RetType;
}

  // proto:  void QTapAndHoldGesture::setPosition(const QPointF & pos);
impl<'a> /*trait*/ QTapAndHoldGesture_setPosition<()> for (QPointF) {
  fn setPosition(self , rsthis: &mut QTapAndHoldGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QTapAndHoldGesture11setPositionERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QTapAndHoldGesture11setPositionERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPointF QTapGesture::position();
impl /*struct*/ QTapGesture {
  pub fn position<RetType, T: QTapGesture_position<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.position(self);
    // return 1;
  }
}

pub trait QTapGesture_position<RetType> {
  fn position(self , rsthis: &mut QTapGesture) -> RetType;
}

  // proto:  QPointF QTapGesture::position();
impl<'a> /*trait*/ QTapGesture_position<QPointF> for () {
  fn position(self , rsthis: &mut QTapGesture) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTapGesture8positionEv()};
    let mut ret = unsafe {_ZNK11QTapGesture8positionEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QTapGesture::setPosition(const QPointF & pos);
impl /*struct*/ QTapGesture {
  pub fn setPosition<RetType, T: QTapGesture_setPosition<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setPosition(self);
    // return 1;
  }
}

pub trait QTapGesture_setPosition<RetType> {
  fn setPosition(self , rsthis: &mut QTapGesture) -> RetType;
}

  // proto:  void QTapGesture::setPosition(const QPointF & pos);
impl<'a> /*trait*/ QTapGesture_setPosition<()> for (QPointF) {
  fn setPosition(self , rsthis: &mut QTapGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTapGesture11setPositionERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTapGesture11setPositionERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTapGesture::QTapGesture(QObject * parent);
impl /*struct*/ QTapGesture {
  pub fn NewQTapGesture<T: QTapGesture_NewQTapGesture>(value: T) -> QTapGesture {
    let rsthis = value.NewQTapGesture();
    return rsthis;
    // return 1;
  }
}

pub trait QTapGesture_NewQTapGesture {
  fn NewQTapGesture(self) -> QTapGesture;
}

  // proto:  void QTapGesture::QTapGesture(QObject * parent);
impl<'a> /*trait*/ QTapGesture_NewQTapGesture for (QObject) {
  fn NewQTapGesture(self) -> QTapGesture {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTapGestureC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QTapGestureC1EP7QObject(qthis, arg0)};
    let rsthis = QTapGesture{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QTapGesture::metaObject();
impl /*struct*/ QTapGesture {
  pub fn metaObject<RetType, T: QTapGesture_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QTapGesture_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QTapGesture) -> RetType;
}

  // proto:  const QMetaObject * QTapGesture::metaObject();
impl<'a> /*trait*/ QTapGesture_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QTapGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTapGesture10metaObjectEv()};
     unsafe {_ZNK11QTapGesture10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTapGesture::~QTapGesture();
impl /*struct*/ QTapGesture {
  pub fn FreeQTapGesture<RetType, T: QTapGesture_FreeQTapGesture<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQTapGesture(self);
    // return 1;
  }
}

pub trait QTapGesture_FreeQTapGesture<RetType> {
  fn FreeQTapGesture(self , rsthis: &mut QTapGesture) -> RetType;
}

  // proto:  void QTapGesture::~QTapGesture();
impl<'a> /*trait*/ QTapGesture_FreeQTapGesture<()> for () {
  fn FreeQTapGesture(self , rsthis: &mut QTapGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTapGestureD0Ev()};
     unsafe {_ZN11QTapGestureD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPinchGesture::setRotationAngle(qreal value);
impl /*struct*/ QPinchGesture {
  pub fn setRotationAngle<RetType, T: QPinchGesture_setRotationAngle<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setRotationAngle(self);
    // return 1;
  }
}

pub trait QPinchGesture_setRotationAngle<RetType> {
  fn setRotationAngle(self , rsthis: &mut QPinchGesture) -> RetType;
}

  // proto:  void QPinchGesture::setRotationAngle(qreal value);
impl<'a> /*trait*/ QPinchGesture_setRotationAngle<()> for (f64) {
  fn setRotationAngle(self , rsthis: &mut QPinchGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGesture16setRotationAngleEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN13QPinchGesture16setRotationAngleEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QPinchGesture::lastScaleFactor();
impl /*struct*/ QPinchGesture {
  pub fn lastScaleFactor<RetType, T: QPinchGesture_lastScaleFactor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.lastScaleFactor(self);
    // return 1;
  }
}

pub trait QPinchGesture_lastScaleFactor<RetType> {
  fn lastScaleFactor(self , rsthis: &mut QPinchGesture) -> RetType;
}

  // proto:  qreal QPinchGesture::lastScaleFactor();
impl<'a> /*trait*/ QPinchGesture_lastScaleFactor<f64> for () {
  fn lastScaleFactor(self , rsthis: &mut QPinchGesture) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPinchGesture15lastScaleFactorEv()};
    let mut ret = unsafe {_ZNK13QPinchGesture15lastScaleFactorEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QPinchGesture::lastRotationAngle();
impl /*struct*/ QPinchGesture {
  pub fn lastRotationAngle<RetType, T: QPinchGesture_lastRotationAngle<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.lastRotationAngle(self);
    // return 1;
  }
}

pub trait QPinchGesture_lastRotationAngle<RetType> {
  fn lastRotationAngle(self , rsthis: &mut QPinchGesture) -> RetType;
}

  // proto:  qreal QPinchGesture::lastRotationAngle();
impl<'a> /*trait*/ QPinchGesture_lastRotationAngle<f64> for () {
  fn lastRotationAngle(self , rsthis: &mut QPinchGesture) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPinchGesture17lastRotationAngleEv()};
    let mut ret = unsafe {_ZNK13QPinchGesture17lastRotationAngleEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QPointF QPinchGesture::startCenterPoint();
impl /*struct*/ QPinchGesture {
  pub fn startCenterPoint<RetType, T: QPinchGesture_startCenterPoint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.startCenterPoint(self);
    // return 1;
  }
}

pub trait QPinchGesture_startCenterPoint<RetType> {
  fn startCenterPoint(self , rsthis: &mut QPinchGesture) -> RetType;
}

  // proto:  QPointF QPinchGesture::startCenterPoint();
impl<'a> /*trait*/ QPinchGesture_startCenterPoint<QPointF> for () {
  fn startCenterPoint(self , rsthis: &mut QPinchGesture) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPinchGesture16startCenterPointEv()};
    let mut ret = unsafe {_ZNK13QPinchGesture16startCenterPointEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QPinchGesture::rotationAngle();
impl /*struct*/ QPinchGesture {
  pub fn rotationAngle<RetType, T: QPinchGesture_rotationAngle<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rotationAngle(self);
    // return 1;
  }
}

pub trait QPinchGesture_rotationAngle<RetType> {
  fn rotationAngle(self , rsthis: &mut QPinchGesture) -> RetType;
}

  // proto:  qreal QPinchGesture::rotationAngle();
impl<'a> /*trait*/ QPinchGesture_rotationAngle<f64> for () {
  fn rotationAngle(self , rsthis: &mut QPinchGesture) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPinchGesture13rotationAngleEv()};
    let mut ret = unsafe {_ZNK13QPinchGesture13rotationAngleEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QPointF QPinchGesture::lastCenterPoint();
impl /*struct*/ QPinchGesture {
  pub fn lastCenterPoint<RetType, T: QPinchGesture_lastCenterPoint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.lastCenterPoint(self);
    // return 1;
  }
}

pub trait QPinchGesture_lastCenterPoint<RetType> {
  fn lastCenterPoint(self , rsthis: &mut QPinchGesture) -> RetType;
}

  // proto:  QPointF QPinchGesture::lastCenterPoint();
impl<'a> /*trait*/ QPinchGesture_lastCenterPoint<QPointF> for () {
  fn lastCenterPoint(self , rsthis: &mut QPinchGesture) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPinchGesture15lastCenterPointEv()};
    let mut ret = unsafe {_ZNK13QPinchGesture15lastCenterPointEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPinchGesture::QPinchGesture(QObject * parent);
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

  // proto:  void QPinchGesture::QPinchGesture(QObject * parent);
impl<'a> /*trait*/ QPinchGesture_NewQPinchGesture for (QObject) {
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

  // proto:  qreal QPinchGesture::totalScaleFactor();
impl /*struct*/ QPinchGesture {
  pub fn totalScaleFactor<RetType, T: QPinchGesture_totalScaleFactor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.totalScaleFactor(self);
    // return 1;
  }
}

pub trait QPinchGesture_totalScaleFactor<RetType> {
  fn totalScaleFactor(self , rsthis: &mut QPinchGesture) -> RetType;
}

  // proto:  qreal QPinchGesture::totalScaleFactor();
impl<'a> /*trait*/ QPinchGesture_totalScaleFactor<f64> for () {
  fn totalScaleFactor(self , rsthis: &mut QPinchGesture) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPinchGesture16totalScaleFactorEv()};
    let mut ret = unsafe {_ZNK13QPinchGesture16totalScaleFactorEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QPinchGesture::setTotalScaleFactor(qreal value);
impl /*struct*/ QPinchGesture {
  pub fn setTotalScaleFactor<RetType, T: QPinchGesture_setTotalScaleFactor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTotalScaleFactor(self);
    // return 1;
  }
}

pub trait QPinchGesture_setTotalScaleFactor<RetType> {
  fn setTotalScaleFactor(self , rsthis: &mut QPinchGesture) -> RetType;
}

  // proto:  void QPinchGesture::setTotalScaleFactor(qreal value);
impl<'a> /*trait*/ QPinchGesture_setTotalScaleFactor<()> for (f64) {
  fn setTotalScaleFactor(self , rsthis: &mut QPinchGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGesture19setTotalScaleFactorEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN13QPinchGesture19setTotalScaleFactorEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QPinchGesture::totalRotationAngle();
impl /*struct*/ QPinchGesture {
  pub fn totalRotationAngle<RetType, T: QPinchGesture_totalRotationAngle<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.totalRotationAngle(self);
    // return 1;
  }
}

pub trait QPinchGesture_totalRotationAngle<RetType> {
  fn totalRotationAngle(self , rsthis: &mut QPinchGesture) -> RetType;
}

  // proto:  qreal QPinchGesture::totalRotationAngle();
impl<'a> /*trait*/ QPinchGesture_totalRotationAngle<f64> for () {
  fn totalRotationAngle(self , rsthis: &mut QPinchGesture) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPinchGesture18totalRotationAngleEv()};
    let mut ret = unsafe {_ZNK13QPinchGesture18totalRotationAngleEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QPinchGesture::setLastScaleFactor(qreal value);
impl /*struct*/ QPinchGesture {
  pub fn setLastScaleFactor<RetType, T: QPinchGesture_setLastScaleFactor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setLastScaleFactor(self);
    // return 1;
  }
}

pub trait QPinchGesture_setLastScaleFactor<RetType> {
  fn setLastScaleFactor(self , rsthis: &mut QPinchGesture) -> RetType;
}

  // proto:  void QPinchGesture::setLastScaleFactor(qreal value);
impl<'a> /*trait*/ QPinchGesture_setLastScaleFactor<()> for (f64) {
  fn setLastScaleFactor(self , rsthis: &mut QPinchGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGesture18setLastScaleFactorEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN13QPinchGesture18setLastScaleFactorEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPinchGesture::setLastCenterPoint(const QPointF & value);
impl /*struct*/ QPinchGesture {
  pub fn setLastCenterPoint<RetType, T: QPinchGesture_setLastCenterPoint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setLastCenterPoint(self);
    // return 1;
  }
}

pub trait QPinchGesture_setLastCenterPoint<RetType> {
  fn setLastCenterPoint(self , rsthis: &mut QPinchGesture) -> RetType;
}

  // proto:  void QPinchGesture::setLastCenterPoint(const QPointF & value);
impl<'a> /*trait*/ QPinchGesture_setLastCenterPoint<()> for (QPointF) {
  fn setLastCenterPoint(self , rsthis: &mut QPinchGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGesture18setLastCenterPointERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QPinchGesture18setLastCenterPointERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QPinchGesture::metaObject();
impl /*struct*/ QPinchGesture {
  pub fn metaObject<RetType, T: QPinchGesture_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QPinchGesture_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QPinchGesture) -> RetType;
}

  // proto:  const QMetaObject * QPinchGesture::metaObject();
impl<'a> /*trait*/ QPinchGesture_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QPinchGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPinchGesture10metaObjectEv()};
     unsafe {_ZNK13QPinchGesture10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPinchGesture::setLastRotationAngle(qreal value);
impl /*struct*/ QPinchGesture {
  pub fn setLastRotationAngle<RetType, T: QPinchGesture_setLastRotationAngle<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setLastRotationAngle(self);
    // return 1;
  }
}

pub trait QPinchGesture_setLastRotationAngle<RetType> {
  fn setLastRotationAngle(self , rsthis: &mut QPinchGesture) -> RetType;
}

  // proto:  void QPinchGesture::setLastRotationAngle(qreal value);
impl<'a> /*trait*/ QPinchGesture_setLastRotationAngle<()> for (f64) {
  fn setLastRotationAngle(self , rsthis: &mut QPinchGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGesture20setLastRotationAngleEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN13QPinchGesture20setLastRotationAngleEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPointF QPinchGesture::centerPoint();
impl /*struct*/ QPinchGesture {
  pub fn centerPoint<RetType, T: QPinchGesture_centerPoint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.centerPoint(self);
    // return 1;
  }
}

pub trait QPinchGesture_centerPoint<RetType> {
  fn centerPoint(self , rsthis: &mut QPinchGesture) -> RetType;
}

  // proto:  QPointF QPinchGesture::centerPoint();
impl<'a> /*trait*/ QPinchGesture_centerPoint<QPointF> for () {
  fn centerPoint(self , rsthis: &mut QPinchGesture) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPinchGesture11centerPointEv()};
    let mut ret = unsafe {_ZNK13QPinchGesture11centerPointEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPinchGesture::setCenterPoint(const QPointF & value);
impl /*struct*/ QPinchGesture {
  pub fn setCenterPoint<RetType, T: QPinchGesture_setCenterPoint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCenterPoint(self);
    // return 1;
  }
}

pub trait QPinchGesture_setCenterPoint<RetType> {
  fn setCenterPoint(self , rsthis: &mut QPinchGesture) -> RetType;
}

  // proto:  void QPinchGesture::setCenterPoint(const QPointF & value);
impl<'a> /*trait*/ QPinchGesture_setCenterPoint<()> for (QPointF) {
  fn setCenterPoint(self , rsthis: &mut QPinchGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGesture14setCenterPointERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QPinchGesture14setCenterPointERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPinchGesture::setTotalRotationAngle(qreal value);
impl /*struct*/ QPinchGesture {
  pub fn setTotalRotationAngle<RetType, T: QPinchGesture_setTotalRotationAngle<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTotalRotationAngle(self);
    // return 1;
  }
}

pub trait QPinchGesture_setTotalRotationAngle<RetType> {
  fn setTotalRotationAngle(self , rsthis: &mut QPinchGesture) -> RetType;
}

  // proto:  void QPinchGesture::setTotalRotationAngle(qreal value);
impl<'a> /*trait*/ QPinchGesture_setTotalRotationAngle<()> for (f64) {
  fn setTotalRotationAngle(self , rsthis: &mut QPinchGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGesture21setTotalRotationAngleEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN13QPinchGesture21setTotalRotationAngleEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPinchGesture::setScaleFactor(qreal value);
impl /*struct*/ QPinchGesture {
  pub fn setScaleFactor<RetType, T: QPinchGesture_setScaleFactor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setScaleFactor(self);
    // return 1;
  }
}

pub trait QPinchGesture_setScaleFactor<RetType> {
  fn setScaleFactor(self , rsthis: &mut QPinchGesture) -> RetType;
}

  // proto:  void QPinchGesture::setScaleFactor(qreal value);
impl<'a> /*trait*/ QPinchGesture_setScaleFactor<()> for (f64) {
  fn setScaleFactor(self , rsthis: &mut QPinchGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGesture14setScaleFactorEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN13QPinchGesture14setScaleFactorEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPinchGesture::~QPinchGesture();
impl /*struct*/ QPinchGesture {
  pub fn FreeQPinchGesture<RetType, T: QPinchGesture_FreeQPinchGesture<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQPinchGesture(self);
    // return 1;
  }
}

pub trait QPinchGesture_FreeQPinchGesture<RetType> {
  fn FreeQPinchGesture(self , rsthis: &mut QPinchGesture) -> RetType;
}

  // proto:  void QPinchGesture::~QPinchGesture();
impl<'a> /*trait*/ QPinchGesture_FreeQPinchGesture<()> for () {
  fn FreeQPinchGesture(self , rsthis: &mut QPinchGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGestureD0Ev()};
     unsafe {_ZN13QPinchGestureD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPinchGesture::setStartCenterPoint(const QPointF & value);
impl /*struct*/ QPinchGesture {
  pub fn setStartCenterPoint<RetType, T: QPinchGesture_setStartCenterPoint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setStartCenterPoint(self);
    // return 1;
  }
}

pub trait QPinchGesture_setStartCenterPoint<RetType> {
  fn setStartCenterPoint(self , rsthis: &mut QPinchGesture) -> RetType;
}

  // proto:  void QPinchGesture::setStartCenterPoint(const QPointF & value);
impl<'a> /*trait*/ QPinchGesture_setStartCenterPoint<()> for (QPointF) {
  fn setStartCenterPoint(self , rsthis: &mut QPinchGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGesture19setStartCenterPointERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QPinchGesture19setStartCenterPointERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QPinchGesture::scaleFactor();
impl /*struct*/ QPinchGesture {
  pub fn scaleFactor<RetType, T: QPinchGesture_scaleFactor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.scaleFactor(self);
    // return 1;
  }
}

pub trait QPinchGesture_scaleFactor<RetType> {
  fn scaleFactor(self , rsthis: &mut QPinchGesture) -> RetType;
}

  // proto:  qreal QPinchGesture::scaleFactor();
impl<'a> /*trait*/ QPinchGesture_scaleFactor<f64> for () {
  fn scaleFactor(self , rsthis: &mut QPinchGesture) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPinchGesture11scaleFactorEv()};
    let mut ret = unsafe {_ZNK13QPinchGesture11scaleFactorEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// <= body block end

