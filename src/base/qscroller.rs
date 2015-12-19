// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpointf::QPointF;
use super::qscrollerproperties::QScrollerProperties;
use super::qrectf::QRectF;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QScroller::FreeQScroller();
  fn _ZN9QScrollerD0Ev(qthis: *mut c_void) ;
  // proto:  void QScroller::setSnapPositionsY(qreal first, qreal interval);
  fn _ZN9QScroller17setSnapPositionsYEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) ;
  // proto:  QPointF QScroller::finalPosition();
  fn _ZNK9QScroller13finalPositionEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static QList<QScroller *> QScroller::activeScrollers();
  fn _ZN9QScroller15activeScrollersEv() ;
  // proto:  void QScroller::setScrollerProperties(const QScrollerProperties & prop);
  fn _ZN9QScroller21setScrollerPropertiesERK19QScrollerProperties(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QMetaObject * QScroller::metaObject();
  fn _ZNK9QScroller10metaObjectEv(qthis: *mut c_void) ;
  // proto:  QPointF QScroller::velocity();
  fn _ZNK9QScroller8velocityEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QScroller::resendPrepareEvent();
  fn _ZN9QScroller18resendPrepareEventEv(qthis: *mut c_void) ;
  // proto:  void QScroller::ensureVisible(const QRectF & rect, qreal xmargin, qreal ymargin);
  fn _ZN9QScroller13ensureVisibleERK6QRectFdd(qthis: *mut c_void, arg0: *mut c_void, arg1: c_double, arg2: c_double) ;
  // proto:  void QScroller::setSnapPositionsX(qreal first, qreal interval);
  fn _ZN9QScroller17setSnapPositionsXEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) ;
  // proto: static bool QScroller::hasScroller(QObject * target);
  fn _ZN9QScroller11hasScrollerEP7QObject(arg0: *mut c_void) -> int8_t;
  // proto:  void QScroller::NewQScroller(const QScroller & );
  fn _ZN9QScrollerC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QScroller::scrollTo(const QPointF & pos, int scrollTime);
  fn _ZN9QScroller8scrollToERK7QPointFi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  void QScroller::stop();
  fn _ZN9QScroller4stopEv(qthis: *mut c_void) ;
  // proto:  void QScroller::ensureVisible(const QRectF & rect, qreal xmargin, qreal ymargin, int scrollTime);
  fn _ZN9QScroller13ensureVisibleERK6QRectFddi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_double, arg2: c_double, arg3: c_int) ;
  // proto:  void QScroller::NewQScroller(QObject * target);
  fn _ZN9QScrollerC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static void QScroller::ungrabGesture(QObject * target);
  fn _ZN9QScroller13ungrabGestureEP7QObject(arg0: *mut c_void) ;
  // proto:  void QScroller::scrollerPropertiesChanged(const QScrollerProperties & );
  fn _ZN9QScroller25scrollerPropertiesChangedERK19QScrollerProperties(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QScrollerProperties QScroller::scrollerProperties();
  fn _ZNK9QScroller18scrollerPropertiesEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static QScroller * QScroller::scroller(QObject * target);
  fn _ZN9QScroller8scrollerEP7QObject(arg0: *mut c_void) -> *mut c_void;
  // proto: static const QScroller * QScroller::scroller(const QObject * target);
  fn _ZN9QScroller8scrollerEPK7QObject(arg0: *mut c_void) -> *mut c_void;
  // proto:  void QScroller::scrollTo(const QPointF & pos);
  fn _ZN9QScroller8scrollToERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QObject * QScroller::target();
  fn _ZNK9QScroller6targetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPointF QScroller::pixelPerMeter();
  fn _ZNK9QScroller13pixelPerMeterEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QScroller)=1
pub struct QScroller {
  pub qclsinst: *mut c_void,
}

// proto:  void QScroller::FreeQScroller();
impl /*struct*/ QScroller {
  pub fn FreeQScroller<RetType, T: QScroller_FreeQScroller<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQScroller(self);
    // return 1;
  }
}

pub trait QScroller_FreeQScroller<RetType> {
  fn FreeQScroller(self , rsthis: &mut QScroller) -> RetType;
}

// proto:  void QScroller::FreeQScroller();
impl<'a> /*trait*/ QScroller_FreeQScroller<()> for () {
  fn FreeQScroller(self , rsthis: &mut QScroller) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScrollerD0Ev()};
     unsafe {_ZN9QScrollerD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QScroller::setSnapPositionsY(qreal first, qreal interval);
impl /*struct*/ QScroller {
  pub fn setSnapPositionsY<RetType, T: QScroller_setSnapPositionsY<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setSnapPositionsY(self);
    // return 1;
  }
}

pub trait QScroller_setSnapPositionsY<RetType> {
  fn setSnapPositionsY(self , rsthis: &mut QScroller) -> RetType;
}

// proto:  void QScroller::setSnapPositionsY(qreal first, qreal interval);
impl<'a> /*trait*/ QScroller_setSnapPositionsY<()> for (f64, f64) {
  fn setSnapPositionsY(self , rsthis: &mut QScroller) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller17setSnapPositionsYEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN9QScroller17setSnapPositionsYEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  QPointF QScroller::finalPosition();
impl /*struct*/ QScroller {
  pub fn finalPosition<RetType, T: QScroller_finalPosition<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.finalPosition(self);
    // return 1;
  }
}

pub trait QScroller_finalPosition<RetType> {
  fn finalPosition(self , rsthis: &mut QScroller) -> RetType;
}

// proto:  QPointF QScroller::finalPosition();
impl<'a> /*trait*/ QScroller_finalPosition<QPointF> for () {
  fn finalPosition(self , rsthis: &mut QScroller) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QScroller13finalPositionEv()};
    let mut ret = unsafe {_ZNK9QScroller13finalPositionEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
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
     unsafe {_ZN9QScroller15activeScrollersEv()};
    // return 1;
  }
}

// proto:  void QScroller::setScrollerProperties(const QScrollerProperties & prop);
impl /*struct*/ QScroller {
  pub fn setScrollerProperties<RetType, T: QScroller_setScrollerProperties<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setScrollerProperties(self);
    // return 1;
  }
}

pub trait QScroller_setScrollerProperties<RetType> {
  fn setScrollerProperties(self , rsthis: &mut QScroller) -> RetType;
}

// proto:  void QScroller::setScrollerProperties(const QScrollerProperties & prop);
impl<'a> /*trait*/ QScroller_setScrollerProperties<()> for (&'a  QScrollerProperties) {
  fn setScrollerProperties(self , rsthis: &mut QScroller) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller21setScrollerPropertiesERK19QScrollerProperties()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QScroller21setScrollerPropertiesERK19QScrollerProperties(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  const QMetaObject * QScroller::metaObject();
impl /*struct*/ QScroller {
  pub fn metaObject<RetType, T: QScroller_metaObject<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QScroller_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QScroller) -> RetType;
}

// proto:  const QMetaObject * QScroller::metaObject();
impl<'a> /*trait*/ QScroller_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QScroller) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QScroller10metaObjectEv()};
     unsafe {_ZNK9QScroller10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QPointF QScroller::velocity();
impl /*struct*/ QScroller {
  pub fn velocity<RetType, T: QScroller_velocity<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.velocity(self);
    // return 1;
  }
}

pub trait QScroller_velocity<RetType> {
  fn velocity(self , rsthis: &mut QScroller) -> RetType;
}

// proto:  QPointF QScroller::velocity();
impl<'a> /*trait*/ QScroller_velocity<QPointF> for () {
  fn velocity(self , rsthis: &mut QScroller) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QScroller8velocityEv()};
    let mut ret = unsafe {_ZNK9QScroller8velocityEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QScroller::resendPrepareEvent();
impl /*struct*/ QScroller {
  pub fn resendPrepareEvent<RetType, T: QScroller_resendPrepareEvent<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.resendPrepareEvent(self);
    // return 1;
  }
}

pub trait QScroller_resendPrepareEvent<RetType> {
  fn resendPrepareEvent(self , rsthis: &mut QScroller) -> RetType;
}

// proto:  void QScroller::resendPrepareEvent();
impl<'a> /*trait*/ QScroller_resendPrepareEvent<()> for () {
  fn resendPrepareEvent(self , rsthis: &mut QScroller) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller18resendPrepareEventEv()};
     unsafe {_ZN9QScroller18resendPrepareEventEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QScroller::ensureVisible(const QRectF & rect, qreal xmargin, qreal ymargin);
impl /*struct*/ QScroller {
  pub fn ensureVisible<RetType, T: QScroller_ensureVisible<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.ensureVisible(self);
    // return 1;
  }
}

pub trait QScroller_ensureVisible<RetType> {
  fn ensureVisible(self , rsthis: &mut QScroller) -> RetType;
}

// proto:  void QScroller::ensureVisible(const QRectF & rect, qreal xmargin, qreal ymargin);
impl<'a> /*trait*/ QScroller_ensureVisible<()> for (&'a  QRectF, f64, f64) {
  fn ensureVisible(self , rsthis: &mut QScroller) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller13ensureVisibleERK6QRectFdd()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
     unsafe {_ZN9QScroller13ensureVisibleERK6QRectFdd(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

// proto:  void QScroller::setSnapPositionsX(qreal first, qreal interval);
impl /*struct*/ QScroller {
  pub fn setSnapPositionsX<RetType, T: QScroller_setSnapPositionsX<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setSnapPositionsX(self);
    // return 1;
  }
}

pub trait QScroller_setSnapPositionsX<RetType> {
  fn setSnapPositionsX(self , rsthis: &mut QScroller) -> RetType;
}

// proto:  void QScroller::setSnapPositionsX(qreal first, qreal interval);
impl<'a> /*trait*/ QScroller_setSnapPositionsX<()> for (f64, f64) {
  fn setSnapPositionsX(self , rsthis: &mut QScroller) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller17setSnapPositionsXEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN9QScroller17setSnapPositionsXEdd(rsthis.qclsinst, arg0, arg1)};
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
impl<'a> /*trait*/ QScroller_hasScroller_s<i8> for (&'a mut QObject) {
  fn hasScroller_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller11hasScrollerEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QScroller11hasScrollerEP7QObject(arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QScroller {
  pub fn NewQScroller<T: QScroller_NewQScroller>(value: T) -> QScroller {
    let rsthis = value.NewQScroller();
    return rsthis;
    // return 1;
  }
}

pub trait QScroller_NewQScroller {
  fn NewQScroller(self) -> QScroller;
}

// proto: void QScroller::NewQScroller(const QScroller & );
impl<'a> /*trait*/ QScroller_NewQScroller for (&'a  QScroller) {
  fn NewQScroller(self) -> QScroller {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScrollerC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QScrollerC1ERKS_(qthis, arg0)};
    let rsthis = QScroller{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QScroller::scrollTo(const QPointF & pos, int scrollTime);
impl /*struct*/ QScroller {
  pub fn scrollTo<RetType, T: QScroller_scrollTo<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.scrollTo(self);
    // return 1;
  }
}

pub trait QScroller_scrollTo<RetType> {
  fn scrollTo(self , rsthis: &mut QScroller) -> RetType;
}

// proto:  void QScroller::scrollTo(const QPointF & pos, int scrollTime);
impl<'a> /*trait*/ QScroller_scrollTo<()> for (&'a  QPointF, i32) {
  fn scrollTo(self , rsthis: &mut QScroller) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller8scrollToERK7QPointFi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN9QScroller8scrollToERK7QPointFi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QScroller::stop();
impl /*struct*/ QScroller {
  pub fn stop<RetType, T: QScroller_stop<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.stop(self);
    // return 1;
  }
}

pub trait QScroller_stop<RetType> {
  fn stop(self , rsthis: &mut QScroller) -> RetType;
}

// proto:  void QScroller::stop();
impl<'a> /*trait*/ QScroller_stop<()> for () {
  fn stop(self , rsthis: &mut QScroller) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller4stopEv()};
     unsafe {_ZN9QScroller4stopEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QScroller::ensureVisible(const QRectF & rect, qreal xmargin, qreal ymargin, int scrollTime);
impl<'a> /*trait*/ QScroller_ensureVisible<()> for (&'a  QRectF, f64, f64, i32) {
  fn ensureVisible(self , rsthis: &mut QScroller) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller13ensureVisibleERK6QRectFddi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_int;
     unsafe {_ZN9QScroller13ensureVisibleERK6QRectFddi(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

// proto: void QScroller::NewQScroller(QObject * target);
impl<'a> /*trait*/ QScroller_NewQScroller for (&'a mut QObject) {
  fn NewQScroller(self) -> QScroller {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScrollerC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QScrollerC1EP7QObject(qthis, arg0)};
    let rsthis = QScroller{qclsinst: qthis};
    return rsthis;
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
impl<'a> /*trait*/ QScroller_ungrabGesture_s<()> for (&'a mut QObject) {
  fn ungrabGesture_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller13ungrabGestureEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QScroller13ungrabGestureEP7QObject(arg0)};
    // return 1;
  }
}

// proto:  void QScroller::scrollerPropertiesChanged(const QScrollerProperties & );
impl /*struct*/ QScroller {
  pub fn scrollerPropertiesChanged<RetType, T: QScroller_scrollerPropertiesChanged<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.scrollerPropertiesChanged(self);
    // return 1;
  }
}

pub trait QScroller_scrollerPropertiesChanged<RetType> {
  fn scrollerPropertiesChanged(self , rsthis: &mut QScroller) -> RetType;
}

// proto:  void QScroller::scrollerPropertiesChanged(const QScrollerProperties & );
impl<'a> /*trait*/ QScroller_scrollerPropertiesChanged<()> for (&'a  QScrollerProperties) {
  fn scrollerPropertiesChanged(self , rsthis: &mut QScroller) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller25scrollerPropertiesChangedERK19QScrollerProperties()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QScroller25scrollerPropertiesChangedERK19QScrollerProperties(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QScrollerProperties QScroller::scrollerProperties();
impl /*struct*/ QScroller {
  pub fn scrollerProperties<RetType, T: QScroller_scrollerProperties<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.scrollerProperties(self);
    // return 1;
  }
}

pub trait QScroller_scrollerProperties<RetType> {
  fn scrollerProperties(self , rsthis: &mut QScroller) -> RetType;
}

// proto:  QScrollerProperties QScroller::scrollerProperties();
impl<'a> /*trait*/ QScroller_scrollerProperties<QScrollerProperties> for () {
  fn scrollerProperties(self , rsthis: &mut QScroller) -> QScrollerProperties {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QScroller18scrollerPropertiesEv()};
    let mut ret = unsafe {_ZNK9QScroller18scrollerPropertiesEv(rsthis.qclsinst)};
    let mut ret1 = QScrollerProperties{qclsinst: ret};
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
impl<'a> /*trait*/ QScroller_scroller_s<QScroller> for (&'a mut QObject) {
  fn scroller_s(self ) -> QScroller {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller8scrollerEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QScroller8scrollerEP7QObject(arg0)};
    let mut ret1 = QScroller{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: static const QScroller * QScroller::scroller(const QObject * target);
impl<'a> /*trait*/ QScroller_scroller_s<QScroller> for (&'a  QObject) {
  fn scroller_s(self ) -> QScroller {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller8scrollerEPK7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QScroller8scrollerEPK7QObject(arg0)};
    let mut ret1 = QScroller{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QScroller::scrollTo(const QPointF & pos);
impl<'a> /*trait*/ QScroller_scrollTo<()> for (&'a  QPointF) {
  fn scrollTo(self , rsthis: &mut QScroller) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller8scrollToERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QScroller8scrollToERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QObject * QScroller::target();
impl /*struct*/ QScroller {
  pub fn target<RetType, T: QScroller_target<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.target(self);
    // return 1;
  }
}

pub trait QScroller_target<RetType> {
  fn target(self , rsthis: &mut QScroller) -> RetType;
}

// proto:  QObject * QScroller::target();
impl<'a> /*trait*/ QScroller_target<QObject> for () {
  fn target(self , rsthis: &mut QScroller) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QScroller6targetEv()};
    let mut ret = unsafe {_ZNK9QScroller6targetEv(rsthis.qclsinst)};
    let mut ret1 = QObject{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QPointF QScroller::pixelPerMeter();
impl /*struct*/ QScroller {
  pub fn pixelPerMeter<RetType, T: QScroller_pixelPerMeter<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.pixelPerMeter(self);
    // return 1;
  }
}

pub trait QScroller_pixelPerMeter<RetType> {
  fn pixelPerMeter(self , rsthis: &mut QScroller) -> RetType;
}

// proto:  QPointF QScroller::pixelPerMeter();
impl<'a> /*trait*/ QScroller_pixelPerMeter<QPointF> for () {
  fn pixelPerMeter(self , rsthis: &mut QScroller) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QScroller13pixelPerMeterEv()};
    let mut ret = unsafe {_ZNK9QScroller13pixelPerMeterEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

