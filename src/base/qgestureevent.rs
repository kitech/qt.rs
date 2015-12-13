// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qgesture::QGesture;
use super::qwidget::QWidget;
use super::qpointf::QPointF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: bool QGestureEvent::isAccepted(QGesture * );
  fn _ZNK13QGestureEvent10isAcceptedEP8QGesture(arg0: *mut c_void) -> i32;
  // proto: QWidget * QGestureEvent::widget();
  fn _ZNK13QGestureEvent6widgetEv() -> i32;
  // proto: void QGestureEvent::ignore(QGesture * );
  fn _ZN13QGestureEvent6ignoreEP8QGesture(arg0: *mut c_void) -> i32;
  // proto: void QGestureEvent::accept(QGesture * );
  fn _ZN13QGestureEvent6acceptEP8QGesture(arg0: *mut c_void) -> i32;
  // proto: QList<QGesture *> QGestureEvent::activeGestures();
  fn _ZNK13QGestureEvent14activeGesturesEv() -> i32;
  // proto: QList<QGesture *> QGestureEvent::gestures();
  fn _ZNK13QGestureEvent8gesturesEv() -> i32;
  // proto: void QGestureEvent::setAccepted(QGesture * , bool );
  fn _ZN13QGestureEvent11setAcceptedEP8QGestureb(arg0: *mut c_void, arg1: int8_t) -> i32;
  // proto: void QGestureEvent::setWidget(QWidget * widget);
  fn _ZN13QGestureEvent9setWidgetEP7QWidget(arg0: *mut c_void) -> i32;
  // proto: void QGestureEvent::FreeQGestureEvent();
  fn _ZN13QGestureEventD0Ev() -> i32;
  // proto: QList<QGesture *> QGestureEvent::canceledGestures();
  fn _ZNK13QGestureEvent16canceledGesturesEv() -> i32;
  // proto: QPointF QGestureEvent::mapToGraphicsScene(const QPointF & gesturePoint);
  fn _ZNK13QGestureEvent18mapToGraphicsSceneERK7QPointF(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QGestureEvent)=1
pub struct QGestureEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGestureEvent {
  pub fn isAccepted<T: QGestureEvent_isAccepted>(&mut self, value: T) -> i32 {
    value.isAccepted(self);
    return 1;
  }
}

pub trait QGestureEvent_isAccepted {
  fn isAccepted(self, this: &mut QGestureEvent) -> i32;
}

// proto: bool QGestureEvent::isAccepted(QGesture * );
impl<'a> /*trait*/ QGestureEvent_isAccepted for (&'a mut QGesture) {
  fn isAccepted(self, this: &mut QGestureEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGestureEvent10isAcceptedEP8QGesture()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK13QGestureEvent10isAcceptedEP8QGesture(arg0)};
    return 1;
  }
}

impl /*struct*/ QGestureEvent {
  pub fn widget<T: QGestureEvent_widget>(&mut self, value: T) -> i32 {
    value.widget(self);
    return 1;
  }
}

pub trait QGestureEvent_widget {
  fn widget(self, this: &mut QGestureEvent) -> i32;
}

// proto: QWidget * QGestureEvent::widget();
impl<'a> /*trait*/ QGestureEvent_widget for () {
  fn widget(self, this: &mut QGestureEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGestureEvent6widgetEv()};
    unsafe {_ZNK13QGestureEvent6widgetEv()};
    return 1;
  }
}

impl /*struct*/ QGestureEvent {
  pub fn ignore<T: QGestureEvent_ignore>(&mut self, value: T) -> i32 {
    value.ignore(self);
    return 1;
  }
}

pub trait QGestureEvent_ignore {
  fn ignore(self, this: &mut QGestureEvent) -> i32;
}

// proto: void QGestureEvent::ignore(QGesture * );
impl<'a> /*trait*/ QGestureEvent_ignore for (&'a mut QGesture) {
  fn ignore(self, this: &mut QGestureEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGestureEvent6ignoreEP8QGesture()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QGestureEvent6ignoreEP8QGesture(arg0)};
    return 1;
  }
}

impl /*struct*/ QGestureEvent {
  pub fn accept<T: QGestureEvent_accept>(&mut self, value: T) -> i32 {
    value.accept(self);
    return 1;
  }
}

pub trait QGestureEvent_accept {
  fn accept(self, this: &mut QGestureEvent) -> i32;
}

// proto: void QGestureEvent::accept(QGesture * );
impl<'a> /*trait*/ QGestureEvent_accept for (&'a mut QGesture) {
  fn accept(self, this: &mut QGestureEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGestureEvent6acceptEP8QGesture()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QGestureEvent6acceptEP8QGesture(arg0)};
    return 1;
  }
}

impl /*struct*/ QGestureEvent {
  pub fn activeGestures<T: QGestureEvent_activeGestures>(&mut self, value: T) -> i32 {
    value.activeGestures(self);
    return 1;
  }
}

pub trait QGestureEvent_activeGestures {
  fn activeGestures(self, this: &mut QGestureEvent) -> i32;
}

// proto: QList<QGesture *> QGestureEvent::activeGestures();
impl<'a> /*trait*/ QGestureEvent_activeGestures for () {
  fn activeGestures(self, this: &mut QGestureEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGestureEvent14activeGesturesEv()};
    unsafe {_ZNK13QGestureEvent14activeGesturesEv()};
    return 1;
  }
}

impl /*struct*/ QGestureEvent {
  pub fn gestures<T: QGestureEvent_gestures>(&mut self, value: T) -> i32 {
    value.gestures(self);
    return 1;
  }
}

pub trait QGestureEvent_gestures {
  fn gestures(self, this: &mut QGestureEvent) -> i32;
}

// proto: QList<QGesture *> QGestureEvent::gestures();
impl<'a> /*trait*/ QGestureEvent_gestures for () {
  fn gestures(self, this: &mut QGestureEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGestureEvent8gesturesEv()};
    unsafe {_ZNK13QGestureEvent8gesturesEv()};
    return 1;
  }
}

impl /*struct*/ QGestureEvent {
  pub fn setAccepted<T: QGestureEvent_setAccepted>(&mut self, value: T) -> i32 {
    value.setAccepted(self);
    return 1;
  }
}

pub trait QGestureEvent_setAccepted {
  fn setAccepted(self, this: &mut QGestureEvent) -> i32;
}

// proto: void QGestureEvent::setAccepted(QGesture * , bool );
impl<'a> /*trait*/ QGestureEvent_setAccepted for (&'a mut QGesture, i8) {
  fn setAccepted(self, this: &mut QGestureEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGestureEvent11setAcceptedEP8QGestureb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as int8_t;
    unsafe {_ZN13QGestureEvent11setAcceptedEP8QGestureb(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGestureEvent {
  pub fn setWidget<T: QGestureEvent_setWidget>(&mut self, value: T) -> i32 {
    value.setWidget(self);
    return 1;
  }
}

pub trait QGestureEvent_setWidget {
  fn setWidget(self, this: &mut QGestureEvent) -> i32;
}

// proto: void QGestureEvent::setWidget(QWidget * widget);
impl<'a> /*trait*/ QGestureEvent_setWidget for (&'a mut QWidget) {
  fn setWidget(self, this: &mut QGestureEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGestureEvent9setWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QGestureEvent9setWidgetEP7QWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QGestureEvent {
  pub fn FreeQGestureEvent<T: QGestureEvent_FreeQGestureEvent>(&mut self, value: T) -> i32 {
    value.FreeQGestureEvent(self);
    return 1;
  }
}

pub trait QGestureEvent_FreeQGestureEvent {
  fn FreeQGestureEvent(self, this: &mut QGestureEvent) -> i32;
}

// proto: void QGestureEvent::FreeQGestureEvent();
impl<'a> /*trait*/ QGestureEvent_FreeQGestureEvent for () {
  fn FreeQGestureEvent(self, this: &mut QGestureEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGestureEventD0Ev()};
    unsafe {_ZN13QGestureEventD0Ev()};
    return 1;
  }
}

impl /*struct*/ QGestureEvent {
  pub fn canceledGestures<T: QGestureEvent_canceledGestures>(&mut self, value: T) -> i32 {
    value.canceledGestures(self);
    return 1;
  }
}

pub trait QGestureEvent_canceledGestures {
  fn canceledGestures(self, this: &mut QGestureEvent) -> i32;
}

// proto: QList<QGesture *> QGestureEvent::canceledGestures();
impl<'a> /*trait*/ QGestureEvent_canceledGestures for () {
  fn canceledGestures(self, this: &mut QGestureEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGestureEvent16canceledGesturesEv()};
    unsafe {_ZNK13QGestureEvent16canceledGesturesEv()};
    return 1;
  }
}

impl /*struct*/ QGestureEvent {
  pub fn mapToGraphicsScene<T: QGestureEvent_mapToGraphicsScene>(&mut self, value: T) -> i32 {
    value.mapToGraphicsScene(self);
    return 1;
  }
}

pub trait QGestureEvent_mapToGraphicsScene {
  fn mapToGraphicsScene(self, this: &mut QGestureEvent) -> i32;
}

// proto: QPointF QGestureEvent::mapToGraphicsScene(const QPointF & gesturePoint);
impl<'a> /*trait*/ QGestureEvent_mapToGraphicsScene for (&'a  QPointF) {
  fn mapToGraphicsScene(self, this: &mut QGestureEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGestureEvent18mapToGraphicsSceneERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QGestureEvent18mapToGraphicsSceneERK7QPointF(arg0)};
    return 1;
  }
}

