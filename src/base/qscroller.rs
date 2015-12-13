// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qscrollerproperties::QScrollerProperties;
use super::qrectf::QRectF;
use super::qobject::QObject;
use super::qpointf::QPointF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QScroller::FreeQScroller();
  fn _ZN9QScrollerD0Ev() -> i32;
  // proto: void QScroller::setSnapPositionsY(qreal first, qreal interval);
  fn _ZN9QScroller17setSnapPositionsYEdd(arg0: c_double, arg1: c_double) -> i32;
  // proto: QPointF QScroller::finalPosition();
  fn _ZNK9QScroller13finalPositionEv() -> i32;
  // proto: QList<QScroller *> QScroller::activeScrollers();
  fn _ZN9QScroller15activeScrollersEv() -> i32;
  // proto: void QScroller::setScrollerProperties(const QScrollerProperties & prop);
  fn _ZN9QScroller21setScrollerPropertiesERK19QScrollerProperties(arg0: *const c_void) -> i32;
  // proto: const QMetaObject * QScroller::metaObject();
  fn _ZNK9QScroller10metaObjectEv() -> i32;
  // proto: QPointF QScroller::velocity();
  fn _ZNK9QScroller8velocityEv() -> i32;
  // proto: void QScroller::resendPrepareEvent();
  fn _ZN9QScroller18resendPrepareEventEv() -> i32;
  // proto: void QScroller::ensureVisible(const QRectF & rect, qreal xmargin, qreal ymargin);
  fn _ZN9QScroller13ensureVisibleERK6QRectFdd(arg0: *const c_void, arg1: c_double, arg2: c_double) -> i32;
  // proto: void QScroller::setSnapPositionsX(qreal first, qreal interval);
  fn _ZN9QScroller17setSnapPositionsXEdd(arg0: c_double, arg1: c_double) -> i32;
  // proto: bool QScroller::hasScroller(QObject * target);
  fn _ZN9QScroller11hasScrollerEP7QObject(arg0: *mut c_void) -> i32;
  // proto: void QScroller::NewQScroller(const QScroller & );
  fn _ZN9QScrollerC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QScroller::scrollTo(const QPointF & pos, int scrollTime);
  fn _ZN9QScroller8scrollToERK7QPointFi(arg0: *const c_void, arg1: c_int) -> i32;
  // proto: void QScroller::stop();
  fn _ZN9QScroller4stopEv() -> i32;
  // proto: void QScroller::ensureVisible(const QRectF & rect, qreal xmargin, qreal ymargin, int scrollTime);
  fn _ZN9QScroller13ensureVisibleERK6QRectFddi(arg0: *const c_void, arg1: c_double, arg2: c_double, arg3: c_int) -> i32;
  // proto: void QScroller::NewQScroller(QObject * target);
  fn _ZN9QScrollerC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QScroller::ungrabGesture(QObject * target);
  fn _ZN9QScroller13ungrabGestureEP7QObject(arg0: *mut c_void) -> i32;
  // proto: void QScroller::scrollerPropertiesChanged(const QScrollerProperties & );
  fn _ZN9QScroller25scrollerPropertiesChangedERK19QScrollerProperties(arg0: *const c_void) -> i32;
  // proto: QScrollerProperties QScroller::scrollerProperties();
  fn _ZNK9QScroller18scrollerPropertiesEv() -> i32;
  // proto: QScroller * QScroller::scroller(QObject * target);
  fn _ZN9QScroller8scrollerEP7QObject(arg0: *mut c_void) -> i32;
  // proto: const QScroller * QScroller::scroller(const QObject * target);
  fn _ZN9QScroller8scrollerEPK7QObject(arg0: *const c_void) -> i32;
  // proto: void QScroller::scrollTo(const QPointF & pos);
  fn _ZN9QScroller8scrollToERK7QPointF(arg0: *const c_void) -> i32;
  // proto: QObject * QScroller::target();
  fn _ZNK9QScroller6targetEv() -> i32;
  // proto: QPointF QScroller::pixelPerMeter();
  fn _ZNK9QScroller13pixelPerMeterEv() -> i32;
}

// body block begin
// class sizeof(QScroller)=1
pub struct QScroller {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QScroller {
  pub fn FreeQScroller<T: QScroller_FreeQScroller>(&mut self, value: T) -> i32 {
    value.FreeQScroller(self);
    return 1;
  }
}

pub trait QScroller_FreeQScroller {
  fn FreeQScroller(self, this: &mut QScroller) -> i32;
}

// proto: void QScroller::FreeQScroller();
impl<'a> /*trait*/ QScroller_FreeQScroller for () {
  fn FreeQScroller(self, this: &mut QScroller) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScrollerD0Ev()};
    unsafe {_ZN9QScrollerD0Ev()};
    return 1;
  }
}

impl /*struct*/ QScroller {
  pub fn setSnapPositionsY<T: QScroller_setSnapPositionsY>(&mut self, value: T) -> i32 {
    value.setSnapPositionsY(self);
    return 1;
  }
}

pub trait QScroller_setSnapPositionsY {
  fn setSnapPositionsY(self, this: &mut QScroller) -> i32;
}

// proto: void QScroller::setSnapPositionsY(qreal first, qreal interval);
impl<'a> /*trait*/ QScroller_setSnapPositionsY for (f64, f64) {
  fn setSnapPositionsY(self, this: &mut QScroller) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller17setSnapPositionsYEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN9QScroller17setSnapPositionsYEdd(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QScroller {
  pub fn finalPosition<T: QScroller_finalPosition>(&mut self, value: T) -> i32 {
    value.finalPosition(self);
    return 1;
  }
}

pub trait QScroller_finalPosition {
  fn finalPosition(self, this: &mut QScroller) -> i32;
}

// proto: QPointF QScroller::finalPosition();
impl<'a> /*trait*/ QScroller_finalPosition for () {
  fn finalPosition(self, this: &mut QScroller) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QScroller13finalPositionEv()};
    unsafe {_ZNK9QScroller13finalPositionEv()};
    return 1;
  }
}

impl /*struct*/ QScroller {
  pub fn activeScrollers<T: QScroller_activeScrollers>(&mut self, value: T) -> i32 {
    value.activeScrollers(self);
    return 1;
  }
}

pub trait QScroller_activeScrollers {
  fn activeScrollers(self, this: &mut QScroller) -> i32;
}

// proto: QList<QScroller *> QScroller::activeScrollers();
impl<'a> /*trait*/ QScroller_activeScrollers for () {
  fn activeScrollers(self, this: &mut QScroller) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller15activeScrollersEv()};
    unsafe {_ZN9QScroller15activeScrollersEv()};
    return 1;
  }
}

impl /*struct*/ QScroller {
  pub fn setScrollerProperties<T: QScroller_setScrollerProperties>(&mut self, value: T) -> i32 {
    value.setScrollerProperties(self);
    return 1;
  }
}

pub trait QScroller_setScrollerProperties {
  fn setScrollerProperties(self, this: &mut QScroller) -> i32;
}

// proto: void QScroller::setScrollerProperties(const QScrollerProperties & prop);
impl<'a> /*trait*/ QScroller_setScrollerProperties for (&'a  QScrollerProperties) {
  fn setScrollerProperties(self, this: &mut QScroller) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller21setScrollerPropertiesERK19QScrollerProperties()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QScroller21setScrollerPropertiesERK19QScrollerProperties(arg0)};
    return 1;
  }
}

impl /*struct*/ QScroller {
  pub fn metaObject<T: QScroller_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QScroller_metaObject {
  fn metaObject(self, this: &mut QScroller) -> i32;
}

// proto: const QMetaObject * QScroller::metaObject();
impl<'a> /*trait*/ QScroller_metaObject for () {
  fn metaObject(self, this: &mut QScroller) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QScroller10metaObjectEv()};
    unsafe {_ZNK9QScroller10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QScroller {
  pub fn velocity<T: QScroller_velocity>(&mut self, value: T) -> i32 {
    value.velocity(self);
    return 1;
  }
}

pub trait QScroller_velocity {
  fn velocity(self, this: &mut QScroller) -> i32;
}

// proto: QPointF QScroller::velocity();
impl<'a> /*trait*/ QScroller_velocity for () {
  fn velocity(self, this: &mut QScroller) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QScroller8velocityEv()};
    unsafe {_ZNK9QScroller8velocityEv()};
    return 1;
  }
}

impl /*struct*/ QScroller {
  pub fn resendPrepareEvent<T: QScroller_resendPrepareEvent>(&mut self, value: T) -> i32 {
    value.resendPrepareEvent(self);
    return 1;
  }
}

pub trait QScroller_resendPrepareEvent {
  fn resendPrepareEvent(self, this: &mut QScroller) -> i32;
}

// proto: void QScroller::resendPrepareEvent();
impl<'a> /*trait*/ QScroller_resendPrepareEvent for () {
  fn resendPrepareEvent(self, this: &mut QScroller) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller18resendPrepareEventEv()};
    unsafe {_ZN9QScroller18resendPrepareEventEv()};
    return 1;
  }
}

impl /*struct*/ QScroller {
  pub fn ensureVisible<T: QScroller_ensureVisible>(&mut self, value: T) -> i32 {
    value.ensureVisible(self);
    return 1;
  }
}

pub trait QScroller_ensureVisible {
  fn ensureVisible(self, this: &mut QScroller) -> i32;
}

// proto: void QScroller::ensureVisible(const QRectF & rect, qreal xmargin, qreal ymargin);
impl<'a> /*trait*/ QScroller_ensureVisible for (&'a  QRectF, f64, f64) {
  fn ensureVisible(self, this: &mut QScroller) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller13ensureVisibleERK6QRectFdd()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    unsafe {_ZN9QScroller13ensureVisibleERK6QRectFdd(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QScroller {
  pub fn setSnapPositionsX<T: QScroller_setSnapPositionsX>(&mut self, value: T) -> i32 {
    value.setSnapPositionsX(self);
    return 1;
  }
}

pub trait QScroller_setSnapPositionsX {
  fn setSnapPositionsX(self, this: &mut QScroller) -> i32;
}

// proto: void QScroller::setSnapPositionsX(qreal first, qreal interval);
impl<'a> /*trait*/ QScroller_setSnapPositionsX for (f64, f64) {
  fn setSnapPositionsX(self, this: &mut QScroller) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller17setSnapPositionsXEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN9QScroller17setSnapPositionsXEdd(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QScroller {
  pub fn hasScroller<T: QScroller_hasScroller>(&mut self, value: T) -> i32 {
    value.hasScroller(self);
    return 1;
  }
}

pub trait QScroller_hasScroller {
  fn hasScroller(self, this: &mut QScroller) -> i32;
}

// proto: bool QScroller::hasScroller(QObject * target);
impl<'a> /*trait*/ QScroller_hasScroller for (&'a mut QObject) {
  fn hasScroller(self, this: &mut QScroller) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller11hasScrollerEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QScroller11hasScrollerEP7QObject(arg0)};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QScrollerC1ERKS_(qthis, arg0)};
    let rsthis = QScroller{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QScroller {
  pub fn scrollTo<T: QScroller_scrollTo>(&mut self, value: T) -> i32 {
    value.scrollTo(self);
    return 1;
  }
}

pub trait QScroller_scrollTo {
  fn scrollTo(self, this: &mut QScroller) -> i32;
}

// proto: void QScroller::scrollTo(const QPointF & pos, int scrollTime);
impl<'a> /*trait*/ QScroller_scrollTo for (&'a  QPointF, i32) {
  fn scrollTo(self, this: &mut QScroller) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller8scrollToERK7QPointFi()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN9QScroller8scrollToERK7QPointFi(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QScroller {
  pub fn stop<T: QScroller_stop>(&mut self, value: T) -> i32 {
    value.stop(self);
    return 1;
  }
}

pub trait QScroller_stop {
  fn stop(self, this: &mut QScroller) -> i32;
}

// proto: void QScroller::stop();
impl<'a> /*trait*/ QScroller_stop for () {
  fn stop(self, this: &mut QScroller) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller4stopEv()};
    unsafe {_ZN9QScroller4stopEv()};
    return 1;
  }
}

// proto: void QScroller::ensureVisible(const QRectF & rect, qreal xmargin, qreal ymargin, int scrollTime);
impl<'a> /*trait*/ QScroller_ensureVisible for (&'a  QRectF, f64, f64, i32) {
  fn ensureVisible(self, this: &mut QScroller) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller13ensureVisibleERK6QRectFddi()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_int;
    unsafe {_ZN9QScroller13ensureVisibleERK6QRectFddi(arg0, arg1, arg2, arg3)};
    return 1;
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
  pub fn ungrabGesture<T: QScroller_ungrabGesture>(&mut self, value: T) -> i32 {
    value.ungrabGesture(self);
    return 1;
  }
}

pub trait QScroller_ungrabGesture {
  fn ungrabGesture(self, this: &mut QScroller) -> i32;
}

// proto: void QScroller::ungrabGesture(QObject * target);
impl<'a> /*trait*/ QScroller_ungrabGesture for (&'a mut QObject) {
  fn ungrabGesture(self, this: &mut QScroller) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller13ungrabGestureEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QScroller13ungrabGestureEP7QObject(arg0)};
    return 1;
  }
}

impl /*struct*/ QScroller {
  pub fn scrollerPropertiesChanged<T: QScroller_scrollerPropertiesChanged>(&mut self, value: T) -> i32 {
    value.scrollerPropertiesChanged(self);
    return 1;
  }
}

pub trait QScroller_scrollerPropertiesChanged {
  fn scrollerPropertiesChanged(self, this: &mut QScroller) -> i32;
}

// proto: void QScroller::scrollerPropertiesChanged(const QScrollerProperties & );
impl<'a> /*trait*/ QScroller_scrollerPropertiesChanged for (&'a  QScrollerProperties) {
  fn scrollerPropertiesChanged(self, this: &mut QScroller) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller25scrollerPropertiesChangedERK19QScrollerProperties()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QScroller25scrollerPropertiesChangedERK19QScrollerProperties(arg0)};
    return 1;
  }
}

impl /*struct*/ QScroller {
  pub fn scrollerProperties<T: QScroller_scrollerProperties>(&mut self, value: T) -> i32 {
    value.scrollerProperties(self);
    return 1;
  }
}

pub trait QScroller_scrollerProperties {
  fn scrollerProperties(self, this: &mut QScroller) -> i32;
}

// proto: QScrollerProperties QScroller::scrollerProperties();
impl<'a> /*trait*/ QScroller_scrollerProperties for () {
  fn scrollerProperties(self, this: &mut QScroller) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QScroller18scrollerPropertiesEv()};
    unsafe {_ZNK9QScroller18scrollerPropertiesEv()};
    return 1;
  }
}

impl /*struct*/ QScroller {
  pub fn scroller<T: QScroller_scroller>(&mut self, value: T) -> i32 {
    value.scroller(self);
    return 1;
  }
}

pub trait QScroller_scroller {
  fn scroller(self, this: &mut QScroller) -> i32;
}

// proto: QScroller * QScroller::scroller(QObject * target);
impl<'a> /*trait*/ QScroller_scroller for (&'a mut QObject) {
  fn scroller(self, this: &mut QScroller) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller8scrollerEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QScroller8scrollerEP7QObject(arg0)};
    return 1;
  }
}

// proto: const QScroller * QScroller::scroller(const QObject * target);
impl<'a> /*trait*/ QScroller_scroller for (&'a  QObject) {
  fn scroller(self, this: &mut QScroller) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller8scrollerEPK7QObject()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QScroller8scrollerEPK7QObject(arg0)};
    return 1;
  }
}

// proto: void QScroller::scrollTo(const QPointF & pos);
impl<'a> /*trait*/ QScroller_scrollTo for (&'a  QPointF) {
  fn scrollTo(self, this: &mut QScroller) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QScroller8scrollToERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QScroller8scrollToERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QScroller {
  pub fn target<T: QScroller_target>(&mut self, value: T) -> i32 {
    value.target(self);
    return 1;
  }
}

pub trait QScroller_target {
  fn target(self, this: &mut QScroller) -> i32;
}

// proto: QObject * QScroller::target();
impl<'a> /*trait*/ QScroller_target for () {
  fn target(self, this: &mut QScroller) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QScroller6targetEv()};
    unsafe {_ZNK9QScroller6targetEv()};
    return 1;
  }
}

impl /*struct*/ QScroller {
  pub fn pixelPerMeter<T: QScroller_pixelPerMeter>(&mut self, value: T) -> i32 {
    value.pixelPerMeter(self);
    return 1;
  }
}

pub trait QScroller_pixelPerMeter {
  fn pixelPerMeter(self, this: &mut QScroller) -> i32;
}

// proto: QPointF QScroller::pixelPerMeter();
impl<'a> /*trait*/ QScroller_pixelPerMeter for () {
  fn pixelPerMeter(self, this: &mut QScroller) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QScroller13pixelPerMeterEv()};
    unsafe {_ZNK9QScroller13pixelPerMeterEv()};
    return 1;
  }
}

