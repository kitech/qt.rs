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

impl /*struct*/ QScroller {
  pub fn FreeQScroller<T: QScroller_FreeQScroller>(&mut self, value: T)  {
     value.FreeQScroller(self);
    // return 1;
  }
}

pub trait QScroller_FreeQScroller {
  fn FreeQScroller(self, rsthis: &mut QScroller) ;
}

// proto:  void QScroller::FreeQScroller();
impl<'a> /*trait*/ QScroller_FreeQScroller for () {
  fn FreeQScroller(self, rsthis: &mut QScroller)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScrollerD0Ev()};
     unsafe {_ZN9QScrollerD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QScroller {
  pub fn setSnapPositionsY<T: QScroller_setSnapPositionsY>(&mut self, value: T)  {
     value.setSnapPositionsY(self);
    // return 1;
  }
}

pub trait QScroller_setSnapPositionsY {
  fn setSnapPositionsY(self, rsthis: &mut QScroller) ;
}

// proto:  void QScroller::setSnapPositionsY(qreal first, qreal interval);
impl<'a> /*trait*/ QScroller_setSnapPositionsY for (f64, f64) {
  fn setSnapPositionsY(self, rsthis: &mut QScroller)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller17setSnapPositionsYEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN9QScroller17setSnapPositionsYEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QScroller {
  pub fn finalPosition<T: QScroller_finalPosition>(&mut self, value: T) -> QPointF {
    return value.finalPosition(self);
    // return 1;
  }
}

pub trait QScroller_finalPosition {
  fn finalPosition(self, rsthis: &mut QScroller) -> QPointF;
}

// proto:  QPointF QScroller::finalPosition();
impl<'a> /*trait*/ QScroller_finalPosition for () {
  fn finalPosition(self, rsthis: &mut QScroller) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QScroller13finalPositionEv()};
    let mut ret = unsafe {_ZNK9QScroller13finalPositionEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QScroller {
  pub fn activeScrollers<T: QScroller_activeScrollers>(&mut self, value: T)  {
     value.activeScrollers(self);
    // return 1;
  }
}

pub trait QScroller_activeScrollers {
  fn activeScrollers(self, rsthis: &mut QScroller) ;
}

// proto: static QList<QScroller *> QScroller::activeScrollers();
impl<'a> /*trait*/ QScroller_activeScrollers for () {
  fn activeScrollers(self, rsthis: &mut QScroller)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller15activeScrollersEv()};
     unsafe {_ZN9QScroller15activeScrollersEv()};
    // return 1;
  }
}

impl /*struct*/ QScroller {
  pub fn setScrollerProperties<T: QScroller_setScrollerProperties>(&mut self, value: T)  {
     value.setScrollerProperties(self);
    // return 1;
  }
}

pub trait QScroller_setScrollerProperties {
  fn setScrollerProperties(self, rsthis: &mut QScroller) ;
}

// proto:  void QScroller::setScrollerProperties(const QScrollerProperties & prop);
impl<'a> /*trait*/ QScroller_setScrollerProperties for (&'a  QScrollerProperties) {
  fn setScrollerProperties(self, rsthis: &mut QScroller)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller21setScrollerPropertiesERK19QScrollerProperties()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QScroller21setScrollerPropertiesERK19QScrollerProperties(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QScroller {
  pub fn metaObject<T: QScroller_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QScroller_metaObject {
  fn metaObject(self, rsthis: &mut QScroller) ;
}

// proto:  const QMetaObject * QScroller::metaObject();
impl<'a> /*trait*/ QScroller_metaObject for () {
  fn metaObject(self, rsthis: &mut QScroller)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QScroller10metaObjectEv()};
     unsafe {_ZNK9QScroller10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QScroller {
  pub fn velocity<T: QScroller_velocity>(&mut self, value: T) -> QPointF {
    return value.velocity(self);
    // return 1;
  }
}

pub trait QScroller_velocity {
  fn velocity(self, rsthis: &mut QScroller) -> QPointF;
}

// proto:  QPointF QScroller::velocity();
impl<'a> /*trait*/ QScroller_velocity for () {
  fn velocity(self, rsthis: &mut QScroller) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QScroller8velocityEv()};
    let mut ret = unsafe {_ZNK9QScroller8velocityEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QScroller {
  pub fn resendPrepareEvent<T: QScroller_resendPrepareEvent>(&mut self, value: T)  {
     value.resendPrepareEvent(self);
    // return 1;
  }
}

pub trait QScroller_resendPrepareEvent {
  fn resendPrepareEvent(self, rsthis: &mut QScroller) ;
}

// proto:  void QScroller::resendPrepareEvent();
impl<'a> /*trait*/ QScroller_resendPrepareEvent for () {
  fn resendPrepareEvent(self, rsthis: &mut QScroller)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller18resendPrepareEventEv()};
     unsafe {_ZN9QScroller18resendPrepareEventEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QScroller {
  pub fn ensureVisible<T: QScroller_ensureVisible>(&mut self, value: T)  {
     value.ensureVisible(self);
    // return 1;
  }
}

pub trait QScroller_ensureVisible {
  fn ensureVisible(self, rsthis: &mut QScroller) ;
}

// proto:  void QScroller::ensureVisible(const QRectF & rect, qreal xmargin, qreal ymargin);
impl<'a> /*trait*/ QScroller_ensureVisible for (&'a  QRectF, f64, f64) {
  fn ensureVisible(self, rsthis: &mut QScroller)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller13ensureVisibleERK6QRectFdd()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
     unsafe {_ZN9QScroller13ensureVisibleERK6QRectFdd(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QScroller {
  pub fn setSnapPositionsX<T: QScroller_setSnapPositionsX>(&mut self, value: T)  {
     value.setSnapPositionsX(self);
    // return 1;
  }
}

pub trait QScroller_setSnapPositionsX {
  fn setSnapPositionsX(self, rsthis: &mut QScroller) ;
}

// proto:  void QScroller::setSnapPositionsX(qreal first, qreal interval);
impl<'a> /*trait*/ QScroller_setSnapPositionsX for (f64, f64) {
  fn setSnapPositionsX(self, rsthis: &mut QScroller)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller17setSnapPositionsXEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN9QScroller17setSnapPositionsXEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QScroller {
  pub fn hasScroller<T: QScroller_hasScroller>(&mut self, value: T) -> i8 {
    return value.hasScroller(self);
    // return 1;
  }
}

pub trait QScroller_hasScroller {
  fn hasScroller(self, rsthis: &mut QScroller) -> i8;
}

// proto: static bool QScroller::hasScroller(QObject * target);
impl<'a> /*trait*/ QScroller_hasScroller for (&'a mut QObject) {
  fn hasScroller(self, rsthis: &mut QScroller) -> i8 {
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

impl /*struct*/ QScroller {
  pub fn scrollTo<T: QScroller_scrollTo>(&mut self, value: T)  {
     value.scrollTo(self);
    // return 1;
  }
}

pub trait QScroller_scrollTo {
  fn scrollTo(self, rsthis: &mut QScroller) ;
}

// proto:  void QScroller::scrollTo(const QPointF & pos, int scrollTime);
impl<'a> /*trait*/ QScroller_scrollTo for (&'a  QPointF, i32) {
  fn scrollTo(self, rsthis: &mut QScroller)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller8scrollToERK7QPointFi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN9QScroller8scrollToERK7QPointFi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QScroller {
  pub fn stop<T: QScroller_stop>(&mut self, value: T)  {
     value.stop(self);
    // return 1;
  }
}

pub trait QScroller_stop {
  fn stop(self, rsthis: &mut QScroller) ;
}

// proto:  void QScroller::stop();
impl<'a> /*trait*/ QScroller_stop for () {
  fn stop(self, rsthis: &mut QScroller)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller4stopEv()};
     unsafe {_ZN9QScroller4stopEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QScroller::ensureVisible(const QRectF & rect, qreal xmargin, qreal ymargin, int scrollTime);
impl<'a> /*trait*/ QScroller_ensureVisible for (&'a  QRectF, f64, f64, i32) {
  fn ensureVisible(self, rsthis: &mut QScroller)  {
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

impl /*struct*/ QScroller {
  pub fn ungrabGesture<T: QScroller_ungrabGesture>(&mut self, value: T)  {
     value.ungrabGesture(self);
    // return 1;
  }
}

pub trait QScroller_ungrabGesture {
  fn ungrabGesture(self, rsthis: &mut QScroller) ;
}

// proto: static void QScroller::ungrabGesture(QObject * target);
impl<'a> /*trait*/ QScroller_ungrabGesture for (&'a mut QObject) {
  fn ungrabGesture(self, rsthis: &mut QScroller)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller13ungrabGestureEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QScroller13ungrabGestureEP7QObject(arg0)};
    // return 1;
  }
}

impl /*struct*/ QScroller {
  pub fn scrollerPropertiesChanged<T: QScroller_scrollerPropertiesChanged>(&mut self, value: T)  {
     value.scrollerPropertiesChanged(self);
    // return 1;
  }
}

pub trait QScroller_scrollerPropertiesChanged {
  fn scrollerPropertiesChanged(self, rsthis: &mut QScroller) ;
}

// proto:  void QScroller::scrollerPropertiesChanged(const QScrollerProperties & );
impl<'a> /*trait*/ QScroller_scrollerPropertiesChanged for (&'a  QScrollerProperties) {
  fn scrollerPropertiesChanged(self, rsthis: &mut QScroller)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller25scrollerPropertiesChangedERK19QScrollerProperties()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QScroller25scrollerPropertiesChangedERK19QScrollerProperties(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QScroller {
  pub fn scrollerProperties<T: QScroller_scrollerProperties>(&mut self, value: T) -> QScrollerProperties {
    return value.scrollerProperties(self);
    // return 1;
  }
}

pub trait QScroller_scrollerProperties {
  fn scrollerProperties(self, rsthis: &mut QScroller) -> QScrollerProperties;
}

// proto:  QScrollerProperties QScroller::scrollerProperties();
impl<'a> /*trait*/ QScroller_scrollerProperties for () {
  fn scrollerProperties(self, rsthis: &mut QScroller) -> QScrollerProperties {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QScroller18scrollerPropertiesEv()};
    let mut ret = unsafe {_ZNK9QScroller18scrollerPropertiesEv(rsthis.qclsinst)};
    let mut ret1 = QScrollerProperties{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QScroller {
  pub fn scroller<T: QScroller_scroller>(&mut self, value: T) -> QScroller {
    return value.scroller(self);
    // return 1;
  }
}

pub trait QScroller_scroller {
  fn scroller(self, rsthis: &mut QScroller) -> QScroller;
}

// proto: static QScroller * QScroller::scroller(QObject * target);
impl<'a> /*trait*/ QScroller_scroller for (&'a mut QObject) {
  fn scroller(self, rsthis: &mut QScroller) -> QScroller {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller8scrollerEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QScroller8scrollerEP7QObject(arg0)};
    let mut ret1 = QScroller{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QScroller::scrollTo(const QPointF & pos);
impl<'a> /*trait*/ QScroller_scrollTo for (&'a  QPointF) {
  fn scrollTo(self, rsthis: &mut QScroller)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller8scrollToERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QScroller8scrollToERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QScroller {
  pub fn target<T: QScroller_target>(&mut self, value: T) -> QObject {
    return value.target(self);
    // return 1;
  }
}

pub trait QScroller_target {
  fn target(self, rsthis: &mut QScroller) -> QObject;
}

// proto:  QObject * QScroller::target();
impl<'a> /*trait*/ QScroller_target for () {
  fn target(self, rsthis: &mut QScroller) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QScroller6targetEv()};
    let mut ret = unsafe {_ZNK9QScroller6targetEv(rsthis.qclsinst)};
    let mut ret1 = QObject{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QScroller {
  pub fn pixelPerMeter<T: QScroller_pixelPerMeter>(&mut self, value: T) -> QPointF {
    return value.pixelPerMeter(self);
    // return 1;
  }
}

pub trait QScroller_pixelPerMeter {
  fn pixelPerMeter(self, rsthis: &mut QScroller) -> QPointF;
}

// proto:  QPointF QScroller::pixelPerMeter();
impl<'a> /*trait*/ QScroller_pixelPerMeter for () {
  fn pixelPerMeter(self, rsthis: &mut QScroller) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QScroller13pixelPerMeterEv()};
    let mut ret = unsafe {_ZNK9QScroller13pixelPerMeterEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

