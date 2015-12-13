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
  // proto:  bool QGestureEvent::isAccepted(QGesture * );
  fn _ZNK13QGestureEvent10isAcceptedEP8QGesture(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QWidget * QGestureEvent::widget();
  fn _ZNK13QGestureEvent6widgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGestureEvent::ignore(QGesture * );
  fn _ZN13QGestureEvent6ignoreEP8QGesture(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGestureEvent::accept(QGesture * );
  fn _ZN13QGestureEvent6acceptEP8QGesture(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QList<QGesture *> QGestureEvent::activeGestures();
  fn _ZNK13QGestureEvent14activeGesturesEv(qthis: *mut c_void) ;
  // proto:  QList<QGesture *> QGestureEvent::gestures();
  fn _ZNK13QGestureEvent8gesturesEv(qthis: *mut c_void) ;
  // proto:  void QGestureEvent::setAccepted(QGesture * , bool );
  fn _ZN13QGestureEvent11setAcceptedEP8QGestureb(qthis: *mut c_void, arg0: *mut c_void, arg1: int8_t) ;
  // proto:  void QGestureEvent::setWidget(QWidget * widget);
  fn _ZN13QGestureEvent9setWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGestureEvent::FreeQGestureEvent();
  fn _ZN13QGestureEventD0Ev(qthis: *mut c_void) ;
  // proto:  QList<QGesture *> QGestureEvent::canceledGestures();
  fn _ZNK13QGestureEvent16canceledGesturesEv(qthis: *mut c_void) ;
  // proto:  QPointF QGestureEvent::mapToGraphicsScene(const QPointF & gesturePoint);
  fn _ZNK13QGestureEvent18mapToGraphicsSceneERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QGestureEvent)=1
pub struct QGestureEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGestureEvent {
  pub fn isAccepted<T: QGestureEvent_isAccepted>(&mut self, value: T) -> i8 {
    return value.isAccepted(self);
    // return 1;
  }
}

pub trait QGestureEvent_isAccepted {
  fn isAccepted(self, rsthis: &mut QGestureEvent) -> i8;
}

// proto:  bool QGestureEvent::isAccepted(QGesture * );
impl<'a> /*trait*/ QGestureEvent_isAccepted for (&'a mut QGesture) {
  fn isAccepted(self, rsthis: &mut QGestureEvent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGestureEvent10isAcceptedEP8QGesture()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGestureEvent10isAcceptedEP8QGesture(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGestureEvent {
  pub fn widget<T: QGestureEvent_widget>(&mut self, value: T) -> QWidget {
    return value.widget(self);
    // return 1;
  }
}

pub trait QGestureEvent_widget {
  fn widget(self, rsthis: &mut QGestureEvent) -> QWidget;
}

// proto:  QWidget * QGestureEvent::widget();
impl<'a> /*trait*/ QGestureEvent_widget for () {
  fn widget(self, rsthis: &mut QGestureEvent) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGestureEvent6widgetEv()};
    let mut ret = unsafe {_ZNK13QGestureEvent6widgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGestureEvent {
  pub fn ignore<T: QGestureEvent_ignore>(&mut self, value: T)  {
     value.ignore(self);
    // return 1;
  }
}

pub trait QGestureEvent_ignore {
  fn ignore(self, rsthis: &mut QGestureEvent) ;
}

// proto:  void QGestureEvent::ignore(QGesture * );
impl<'a> /*trait*/ QGestureEvent_ignore for (&'a mut QGesture) {
  fn ignore(self, rsthis: &mut QGestureEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGestureEvent6ignoreEP8QGesture()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QGestureEvent6ignoreEP8QGesture(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGestureEvent {
  pub fn accept<T: QGestureEvent_accept>(&mut self, value: T)  {
     value.accept(self);
    // return 1;
  }
}

pub trait QGestureEvent_accept {
  fn accept(self, rsthis: &mut QGestureEvent) ;
}

// proto:  void QGestureEvent::accept(QGesture * );
impl<'a> /*trait*/ QGestureEvent_accept for (&'a mut QGesture) {
  fn accept(self, rsthis: &mut QGestureEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGestureEvent6acceptEP8QGesture()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QGestureEvent6acceptEP8QGesture(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGestureEvent {
  pub fn activeGestures<T: QGestureEvent_activeGestures>(&mut self, value: T)  {
     value.activeGestures(self);
    // return 1;
  }
}

pub trait QGestureEvent_activeGestures {
  fn activeGestures(self, rsthis: &mut QGestureEvent) ;
}

// proto:  QList<QGesture *> QGestureEvent::activeGestures();
impl<'a> /*trait*/ QGestureEvent_activeGestures for () {
  fn activeGestures(self, rsthis: &mut QGestureEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGestureEvent14activeGesturesEv()};
     unsafe {_ZNK13QGestureEvent14activeGesturesEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGestureEvent {
  pub fn gestures<T: QGestureEvent_gestures>(&mut self, value: T)  {
     value.gestures(self);
    // return 1;
  }
}

pub trait QGestureEvent_gestures {
  fn gestures(self, rsthis: &mut QGestureEvent) ;
}

// proto:  QList<QGesture *> QGestureEvent::gestures();
impl<'a> /*trait*/ QGestureEvent_gestures for () {
  fn gestures(self, rsthis: &mut QGestureEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGestureEvent8gesturesEv()};
     unsafe {_ZNK13QGestureEvent8gesturesEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGestureEvent {
  pub fn setAccepted<T: QGestureEvent_setAccepted>(&mut self, value: T)  {
     value.setAccepted(self);
    // return 1;
  }
}

pub trait QGestureEvent_setAccepted {
  fn setAccepted(self, rsthis: &mut QGestureEvent) ;
}

// proto:  void QGestureEvent::setAccepted(QGesture * , bool );
impl<'a> /*trait*/ QGestureEvent_setAccepted for (&'a mut QGesture, i8) {
  fn setAccepted(self, rsthis: &mut QGestureEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGestureEvent11setAcceptedEP8QGestureb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as int8_t;
     unsafe {_ZN13QGestureEvent11setAcceptedEP8QGestureb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGestureEvent {
  pub fn setWidget<T: QGestureEvent_setWidget>(&mut self, value: T)  {
     value.setWidget(self);
    // return 1;
  }
}

pub trait QGestureEvent_setWidget {
  fn setWidget(self, rsthis: &mut QGestureEvent) ;
}

// proto:  void QGestureEvent::setWidget(QWidget * widget);
impl<'a> /*trait*/ QGestureEvent_setWidget for (&'a mut QWidget) {
  fn setWidget(self, rsthis: &mut QGestureEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGestureEvent9setWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QGestureEvent9setWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGestureEvent {
  pub fn FreeQGestureEvent<T: QGestureEvent_FreeQGestureEvent>(&mut self, value: T)  {
     value.FreeQGestureEvent(self);
    // return 1;
  }
}

pub trait QGestureEvent_FreeQGestureEvent {
  fn FreeQGestureEvent(self, rsthis: &mut QGestureEvent) ;
}

// proto:  void QGestureEvent::FreeQGestureEvent();
impl<'a> /*trait*/ QGestureEvent_FreeQGestureEvent for () {
  fn FreeQGestureEvent(self, rsthis: &mut QGestureEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGestureEventD0Ev()};
     unsafe {_ZN13QGestureEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGestureEvent {
  pub fn canceledGestures<T: QGestureEvent_canceledGestures>(&mut self, value: T)  {
     value.canceledGestures(self);
    // return 1;
  }
}

pub trait QGestureEvent_canceledGestures {
  fn canceledGestures(self, rsthis: &mut QGestureEvent) ;
}

// proto:  QList<QGesture *> QGestureEvent::canceledGestures();
impl<'a> /*trait*/ QGestureEvent_canceledGestures for () {
  fn canceledGestures(self, rsthis: &mut QGestureEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGestureEvent16canceledGesturesEv()};
     unsafe {_ZNK13QGestureEvent16canceledGesturesEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGestureEvent {
  pub fn mapToGraphicsScene<T: QGestureEvent_mapToGraphicsScene>(&mut self, value: T) -> QPointF {
    return value.mapToGraphicsScene(self);
    // return 1;
  }
}

pub trait QGestureEvent_mapToGraphicsScene {
  fn mapToGraphicsScene(self, rsthis: &mut QGestureEvent) -> QPointF;
}

// proto:  QPointF QGestureEvent::mapToGraphicsScene(const QPointF & gesturePoint);
impl<'a> /*trait*/ QGestureEvent_mapToGraphicsScene for (&'a  QPointF) {
  fn mapToGraphicsScene(self, rsthis: &mut QGestureEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGestureEvent18mapToGraphicsSceneERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGestureEvent18mapToGraphicsSceneERK7QPointF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

