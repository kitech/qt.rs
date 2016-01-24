// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
// src-file: /QtWidgets/qscroller.h
// dst-file: /src/widgets/qscroller.rs
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
use std::ops::Deref;
use super::super::core::qpoint::QPointF; // 771
use super::qscrollerproperties::QScrollerProperties; // 773
use super::super::core::qobjectdefs::QMetaObject; // 771
use super::super::core::qrect::QRectF; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QScroller_Class_Size() -> c_int;
  // proto:  void QScroller::setSnapPositionsY(qreal first, qreal interval);
  fn C_ZN9QScroller17setSnapPositionsYEdd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double);
  // proto:  QPointF QScroller::finalPosition();
  fn C_ZNK9QScroller13finalPositionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static QList<QScroller *> QScroller::activeScrollers();
  fn C_ZN9QScroller15activeScrollersEv();
  // proto:  void QScroller::setScrollerProperties(const QScrollerProperties & prop);
  fn C_ZN9QScroller21setScrollerPropertiesERK19QScrollerProperties(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QMetaObject * QScroller::metaObject();
  fn C_ZNK9QScroller10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPointF QScroller::velocity();
  fn C_ZNK9QScroller8velocityEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QScroller::resendPrepareEvent();
  fn C_ZN9QScroller18resendPrepareEventEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QScroller::ensureVisible(const QRectF & rect, qreal xmargin, qreal ymargin);
  fn C_ZN9QScroller13ensureVisibleERK6QRectFdd(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_double, arg2: c_double);
  // proto:  void QScroller::setSnapPositionsX(qreal first, qreal interval);
  fn C_ZN9QScroller17setSnapPositionsXEdd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double);
  // proto: static bool QScroller::hasScroller(QObject * target);
  fn C_ZN9QScroller11hasScrollerEP7QObject(arg0: *mut c_void) -> c_char;
  // proto:  void QScroller::scrollTo(const QPointF & pos, int scrollTime);
  fn C_ZN9QScroller8scrollToERK7QPointFi(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int);
  // proto:  void QScroller::stop();
  fn C_ZN9QScroller4stopEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QScroller::ensureVisible(const QRectF & rect, qreal xmargin, qreal ymargin, int scrollTime);
  fn C_ZN9QScroller13ensureVisibleERK6QRectFddi(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_double, arg2: c_double, arg3: c_int);
  // proto: static void QScroller::ungrabGesture(QObject * target);
  fn C_ZN9QScroller13ungrabGestureEP7QObject(arg0: *mut c_void);
  // proto:  QScrollerProperties QScroller::scrollerProperties();
  fn C_ZNK9QScroller18scrollerPropertiesEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static QScroller * QScroller::scroller(QObject * target);
  fn C_ZN9QScroller8scrollerEP7QObject(arg0: *mut c_void) -> *mut c_void;
  // proto: static const QScroller * QScroller::scroller(const QObject * target);
  fn C_ZN9QScroller8scrollerEPK7QObject(arg0: *mut c_void) -> *mut c_void;
  // proto:  void QScroller::scrollTo(const QPointF & pos);
  fn C_ZN9QScroller8scrollToERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QObject * QScroller::target();
  fn C_ZNK9QScroller6targetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPointF QScroller::pixelPerMeter();
  fn C_ZNK9QScroller13pixelPerMeterEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QScroller_SlotProxy_connect__ZN9QScroller25scrollerPropertiesChangedERK19QScrollerProperties(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QScroller)=1
#[derive(Default)]
pub struct QScroller {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _stateChanged: QScroller_stateChanged_signal,
  pub _scrollerPropertiesChanged: QScroller_scrollerPropertiesChanged_signal,
}

impl /*struct*/ QScroller {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QScroller {
    return QScroller{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QScroller {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QScroller {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QScroller::setSnapPositionsY(qreal first, qreal interval);
impl /*struct*/ QScroller {
  pub fn setSnapPositionsY<RetType, T: QScroller_setSnapPositionsY<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSnapPositionsY(self);
    // return 1;
  }
}

pub trait QScroller_setSnapPositionsY<RetType> {
  fn setSnapPositionsY(self , rsthis: & QScroller) -> RetType;
}

  // proto:  void QScroller::setSnapPositionsY(qreal first, qreal interval);
impl<'a> /*trait*/ QScroller_setSnapPositionsY<()> for (f64, f64) {
  fn setSnapPositionsY(self , rsthis: & QScroller) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller17setSnapPositionsYEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {C_ZN9QScroller17setSnapPositionsYEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QPointF QScroller::finalPosition();
impl /*struct*/ QScroller {
  pub fn finalPosition<RetType, T: QScroller_finalPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.finalPosition(self);
    // return 1;
  }
}

pub trait QScroller_finalPosition<RetType> {
  fn finalPosition(self , rsthis: & QScroller) -> RetType;
}

  // proto:  QPointF QScroller::finalPosition();
impl<'a> /*trait*/ QScroller_finalPosition<QPointF> for () {
  fn finalPosition(self , rsthis: & QScroller) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QScroller13finalPositionEv()};
    let mut ret = unsafe {C_ZNK9QScroller13finalPositionEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QList<QScroller *> QScroller::activeScrollers();
impl /*struct*/ QScroller {
  pub fn activeScrollers_s<RetType, T: QScroller_activeScrollers_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.activeScrollers_s();
    // return 1;
  }
}

pub trait QScroller_activeScrollers_s<RetType> {
  fn activeScrollers_s(self ) -> RetType;
}

  // proto: static QList<QScroller *> QScroller::activeScrollers();
impl<'a> /*trait*/ QScroller_activeScrollers_s<()> for () {
  fn activeScrollers_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller15activeScrollersEv()};
     unsafe {C_ZN9QScroller15activeScrollersEv()};
    // return 1;
  }
}

  // proto:  void QScroller::setScrollerProperties(const QScrollerProperties & prop);
impl /*struct*/ QScroller {
  pub fn setScrollerProperties<RetType, T: QScroller_setScrollerProperties<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setScrollerProperties(self);
    // return 1;
  }
}

pub trait QScroller_setScrollerProperties<RetType> {
  fn setScrollerProperties(self , rsthis: & QScroller) -> RetType;
}

  // proto:  void QScroller::setScrollerProperties(const QScrollerProperties & prop);
impl<'a> /*trait*/ QScroller_setScrollerProperties<()> for (&'a QScrollerProperties) {
  fn setScrollerProperties(self , rsthis: & QScroller) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller21setScrollerPropertiesERK19QScrollerProperties()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN9QScroller21setScrollerPropertiesERK19QScrollerProperties(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QScroller::metaObject();
impl /*struct*/ QScroller {
  pub fn metaObject<RetType, T: QScroller_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QScroller_metaObject<RetType> {
  fn metaObject(self , rsthis: & QScroller) -> RetType;
}

  // proto:  const QMetaObject * QScroller::metaObject();
impl<'a> /*trait*/ QScroller_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QScroller) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QScroller10metaObjectEv()};
    let mut ret = unsafe {C_ZNK9QScroller10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPointF QScroller::velocity();
impl /*struct*/ QScroller {
  pub fn velocity<RetType, T: QScroller_velocity<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.velocity(self);
    // return 1;
  }
}

pub trait QScroller_velocity<RetType> {
  fn velocity(self , rsthis: & QScroller) -> RetType;
}

  // proto:  QPointF QScroller::velocity();
impl<'a> /*trait*/ QScroller_velocity<QPointF> for () {
  fn velocity(self , rsthis: & QScroller) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QScroller8velocityEv()};
    let mut ret = unsafe {C_ZNK9QScroller8velocityEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QScroller::resendPrepareEvent();
impl /*struct*/ QScroller {
  pub fn resendPrepareEvent<RetType, T: QScroller_resendPrepareEvent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resendPrepareEvent(self);
    // return 1;
  }
}

pub trait QScroller_resendPrepareEvent<RetType> {
  fn resendPrepareEvent(self , rsthis: & QScroller) -> RetType;
}

  // proto:  void QScroller::resendPrepareEvent();
impl<'a> /*trait*/ QScroller_resendPrepareEvent<()> for () {
  fn resendPrepareEvent(self , rsthis: & QScroller) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller18resendPrepareEventEv()};
     unsafe {C_ZN9QScroller18resendPrepareEventEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QScroller::ensureVisible(const QRectF & rect, qreal xmargin, qreal ymargin);
impl /*struct*/ QScroller {
  pub fn ensureVisible<RetType, T: QScroller_ensureVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ensureVisible(self);
    // return 1;
  }
}

pub trait QScroller_ensureVisible<RetType> {
  fn ensureVisible(self , rsthis: & QScroller) -> RetType;
}

  // proto:  void QScroller::ensureVisible(const QRectF & rect, qreal xmargin, qreal ymargin);
impl<'a> /*trait*/ QScroller_ensureVisible<()> for (&'a QRectF, f64, f64) {
  fn ensureVisible(self , rsthis: & QScroller) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller13ensureVisibleERK6QRectFdd()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
     unsafe {C_ZN9QScroller13ensureVisibleERK6QRectFdd(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QScroller::setSnapPositionsX(qreal first, qreal interval);
impl /*struct*/ QScroller {
  pub fn setSnapPositionsX<RetType, T: QScroller_setSnapPositionsX<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSnapPositionsX(self);
    // return 1;
  }
}

pub trait QScroller_setSnapPositionsX<RetType> {
  fn setSnapPositionsX(self , rsthis: & QScroller) -> RetType;
}

  // proto:  void QScroller::setSnapPositionsX(qreal first, qreal interval);
impl<'a> /*trait*/ QScroller_setSnapPositionsX<()> for (f64, f64) {
  fn setSnapPositionsX(self , rsthis: & QScroller) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller17setSnapPositionsXEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {C_ZN9QScroller17setSnapPositionsXEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto: static bool QScroller::hasScroller(QObject * target);
impl /*struct*/ QScroller {
  pub fn hasScroller_s<RetType, T: QScroller_hasScroller_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.hasScroller_s();
    // return 1;
  }
}

pub trait QScroller_hasScroller_s<RetType> {
  fn hasScroller_s(self ) -> RetType;
}

  // proto: static bool QScroller::hasScroller(QObject * target);
impl<'a> /*trait*/ QScroller_hasScroller_s<i8> for (&'a QObject) {
  fn hasScroller_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller11hasScrollerEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN9QScroller11hasScrollerEP7QObject(arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QScroller::scrollTo(const QPointF & pos, int scrollTime);
impl /*struct*/ QScroller {
  pub fn scrollTo<RetType, T: QScroller_scrollTo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.scrollTo(self);
    // return 1;
  }
}

pub trait QScroller_scrollTo<RetType> {
  fn scrollTo(self , rsthis: & QScroller) -> RetType;
}

  // proto:  void QScroller::scrollTo(const QPointF & pos, int scrollTime);
impl<'a> /*trait*/ QScroller_scrollTo<()> for (&'a QPointF, i32) {
  fn scrollTo(self , rsthis: & QScroller) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller8scrollToERK7QPointFi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN9QScroller8scrollToERK7QPointFi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QScroller::stop();
impl /*struct*/ QScroller {
  pub fn stop<RetType, T: QScroller_stop<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.stop(self);
    // return 1;
  }
}

pub trait QScroller_stop<RetType> {
  fn stop(self , rsthis: & QScroller) -> RetType;
}

  // proto:  void QScroller::stop();
impl<'a> /*trait*/ QScroller_stop<()> for () {
  fn stop(self , rsthis: & QScroller) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller4stopEv()};
     unsafe {C_ZN9QScroller4stopEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QScroller::ensureVisible(const QRectF & rect, qreal xmargin, qreal ymargin, int scrollTime);
impl<'a> /*trait*/ QScroller_ensureVisible<()> for (&'a QRectF, f64, f64, i32) {
  fn ensureVisible(self , rsthis: & QScroller) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller13ensureVisibleERK6QRectFddi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_int;
     unsafe {C_ZN9QScroller13ensureVisibleERK6QRectFddi(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto: static void QScroller::ungrabGesture(QObject * target);
impl /*struct*/ QScroller {
  pub fn ungrabGesture_s<RetType, T: QScroller_ungrabGesture_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.ungrabGesture_s();
    // return 1;
  }
}

pub trait QScroller_ungrabGesture_s<RetType> {
  fn ungrabGesture_s(self ) -> RetType;
}

  // proto: static void QScroller::ungrabGesture(QObject * target);
impl<'a> /*trait*/ QScroller_ungrabGesture_s<()> for (&'a QObject) {
  fn ungrabGesture_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller13ungrabGestureEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN9QScroller13ungrabGestureEP7QObject(arg0)};
    // return 1;
  }
}

  // proto:  QScrollerProperties QScroller::scrollerProperties();
impl /*struct*/ QScroller {
  pub fn scrollerProperties<RetType, T: QScroller_scrollerProperties<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.scrollerProperties(self);
    // return 1;
  }
}

pub trait QScroller_scrollerProperties<RetType> {
  fn scrollerProperties(self , rsthis: & QScroller) -> RetType;
}

  // proto:  QScrollerProperties QScroller::scrollerProperties();
impl<'a> /*trait*/ QScroller_scrollerProperties<QScrollerProperties> for () {
  fn scrollerProperties(self , rsthis: & QScroller) -> QScrollerProperties {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QScroller18scrollerPropertiesEv()};
    let mut ret = unsafe {C_ZNK9QScroller18scrollerPropertiesEv(rsthis.qclsinst)};
    let mut ret1 = QScrollerProperties::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QScroller * QScroller::scroller(QObject * target);
impl /*struct*/ QScroller {
  pub fn scroller_s<RetType, T: QScroller_scroller_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.scroller_s();
    // return 1;
  }
}

pub trait QScroller_scroller_s<RetType> {
  fn scroller_s(self ) -> RetType;
}

  // proto: static QScroller * QScroller::scroller(QObject * target);
impl<'a> /*trait*/ QScroller_scroller_s<QScroller> for (&'a QObject) {
  fn scroller_s(self ) -> QScroller {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller8scrollerEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN9QScroller8scrollerEP7QObject(arg0)};
    let mut ret1 = QScroller::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QScroller::scrollTo(const QPointF & pos);
impl<'a> /*trait*/ QScroller_scrollTo<()> for (&'a QPointF) {
  fn scrollTo(self , rsthis: & QScroller) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller8scrollToERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN9QScroller8scrollToERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QObject * QScroller::target();
impl /*struct*/ QScroller {
  pub fn target<RetType, T: QScroller_target<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.target(self);
    // return 1;
  }
}

pub trait QScroller_target<RetType> {
  fn target(self , rsthis: & QScroller) -> RetType;
}

  // proto:  QObject * QScroller::target();
impl<'a> /*trait*/ QScroller_target<QObject> for () {
  fn target(self , rsthis: & QScroller) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QScroller6targetEv()};
    let mut ret = unsafe {C_ZNK9QScroller6targetEv(rsthis.qclsinst)};
    let mut ret1 = QObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPointF QScroller::pixelPerMeter();
impl /*struct*/ QScroller {
  pub fn pixelPerMeter<RetType, T: QScroller_pixelPerMeter<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pixelPerMeter(self);
    // return 1;
  }
}

pub trait QScroller_pixelPerMeter<RetType> {
  fn pixelPerMeter(self , rsthis: & QScroller) -> RetType;
}

  // proto:  QPointF QScroller::pixelPerMeter();
impl<'a> /*trait*/ QScroller_pixelPerMeter<QPointF> for () {
  fn pixelPerMeter(self , rsthis: & QScroller) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QScroller13pixelPerMeterEv()};
    let mut ret = unsafe {C_ZNK9QScroller13pixelPerMeterEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

#[derive(Default)] // for QScroller_stateChanged
pub struct QScroller_stateChanged_signal{poi:u64}
impl /* struct */ QScroller {
  pub fn stateChanged(&self) -> QScroller_stateChanged_signal {
     return QScroller_stateChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QScroller_stateChanged_signal {
  pub fn connect<T: QScroller_stateChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QScroller_stateChanged_signal_connect {
  fn connect(self, sigthis: QScroller_stateChanged_signal);
}

#[derive(Default)] // for QScroller_scrollerPropertiesChanged
pub struct QScroller_scrollerPropertiesChanged_signal{poi:u64}
impl /* struct */ QScroller {
  pub fn scrollerPropertiesChanged(&self) -> QScroller_scrollerPropertiesChanged_signal {
     return QScroller_scrollerPropertiesChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QScroller_scrollerPropertiesChanged_signal {
  pub fn connect<T: QScroller_scrollerPropertiesChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QScroller_scrollerPropertiesChanged_signal_connect {
  fn connect(self, sigthis: QScroller_scrollerPropertiesChanged_signal);
}

// scrollerPropertiesChanged(const class QScrollerProperties &)
extern fn QScroller_scrollerPropertiesChanged_signal_connect_cb_0(rsfptr:fn(QScrollerProperties), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QScrollerProperties::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QScroller_scrollerPropertiesChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(QScrollerProperties)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QScrollerProperties::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QScroller_scrollerPropertiesChanged_signal_connect for fn(QScrollerProperties) {
  fn connect(self, sigthis: QScroller_scrollerPropertiesChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QScroller_scrollerPropertiesChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QScroller_SlotProxy_connect__ZN9QScroller25scrollerPropertiesChangedERK19QScrollerProperties(arg0, arg1, arg2)};
  }
}
impl /* trait */ QScroller_scrollerPropertiesChanged_signal_connect for Box<Fn(QScrollerProperties)> {
  fn connect(self, sigthis: QScroller_scrollerPropertiesChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QScroller_scrollerPropertiesChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QScroller_SlotProxy_connect__ZN9QScroller25scrollerPropertiesChangedERK19QScrollerProperties(arg0, arg1, arg2)};
  }
}
// <= body block end

