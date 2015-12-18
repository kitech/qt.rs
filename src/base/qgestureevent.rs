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
  pub fn isAccepted<RetType, T: QGestureEvent_isAccepted<RetType>>(&mut self, value: T) -> RetType {
    return value.isAccepted(self);
    // return 1;
  }
}

pub trait QGestureEvent_isAccepted<RetType> {
  fn isAccepted(self, rsthis: &mut QGestureEvent) -> RetType;
}

// proto:  bool QGestureEvent::isAccepted(QGesture * );
impl<'a> /*trait*/ QGestureEvent_isAccepted<i8> for (&'a mut QGesture) {
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
  pub fn widget<RetType, T: QGestureEvent_widget<RetType>>(&mut self, value: T) -> RetType {
    return value.widget(self);
    // return 1;
  }
}

pub trait QGestureEvent_widget<RetType> {
  fn widget(self, rsthis: &mut QGestureEvent) -> RetType;
}

// proto:  QWidget * QGestureEvent::widget();
impl<'a> /*trait*/ QGestureEvent_widget<QWidget> for () {
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
  pub fn ignore<RetType, T: QGestureEvent_ignore<RetType>>(&mut self, value: T) -> RetType {
    return value.ignore(self);
    // return 1;
  }
}

pub trait QGestureEvent_ignore<RetType> {
  fn ignore(self, rsthis: &mut QGestureEvent) -> RetType;
}

// proto:  void QGestureEvent::ignore(QGesture * );
impl<'a> /*trait*/ QGestureEvent_ignore<()> for (&'a mut QGesture) {
  fn ignore(self, rsthis: &mut QGestureEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGestureEvent6ignoreEP8QGesture()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QGestureEvent6ignoreEP8QGesture(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGestureEvent {
  pub fn accept<RetType, T: QGestureEvent_accept<RetType>>(&mut self, value: T) -> RetType {
    return value.accept(self);
    // return 1;
  }
}

pub trait QGestureEvent_accept<RetType> {
  fn accept(self, rsthis: &mut QGestureEvent) -> RetType;
}

// proto:  void QGestureEvent::accept(QGesture * );
impl<'a> /*trait*/ QGestureEvent_accept<()> for (&'a mut QGesture) {
  fn accept(self, rsthis: &mut QGestureEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGestureEvent6acceptEP8QGesture()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QGestureEvent6acceptEP8QGesture(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGestureEvent {
  pub fn activeGestures<RetType, T: QGestureEvent_activeGestures<RetType>>(&mut self, value: T) -> RetType {
    return value.activeGestures(self);
    // return 1;
  }
}

pub trait QGestureEvent_activeGestures<RetType> {
  fn activeGestures(self, rsthis: &mut QGestureEvent) -> RetType;
}

// proto:  QList<QGesture *> QGestureEvent::activeGestures();
impl<'a> /*trait*/ QGestureEvent_activeGestures<()> for () {
  fn activeGestures(self, rsthis: &mut QGestureEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGestureEvent14activeGesturesEv()};
     unsafe {_ZNK13QGestureEvent14activeGesturesEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGestureEvent {
  pub fn gestures<RetType, T: QGestureEvent_gestures<RetType>>(&mut self, value: T) -> RetType {
    return value.gestures(self);
    // return 1;
  }
}

pub trait QGestureEvent_gestures<RetType> {
  fn gestures(self, rsthis: &mut QGestureEvent) -> RetType;
}

// proto:  QList<QGesture *> QGestureEvent::gestures();
impl<'a> /*trait*/ QGestureEvent_gestures<()> for () {
  fn gestures(self, rsthis: &mut QGestureEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGestureEvent8gesturesEv()};
     unsafe {_ZNK13QGestureEvent8gesturesEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGestureEvent {
  pub fn setAccepted<RetType, T: QGestureEvent_setAccepted<RetType>>(&mut self, value: T) -> RetType {
    return value.setAccepted(self);
    // return 1;
  }
}

pub trait QGestureEvent_setAccepted<RetType> {
  fn setAccepted(self, rsthis: &mut QGestureEvent) -> RetType;
}

// proto:  void QGestureEvent::setAccepted(QGesture * , bool );
impl<'a> /*trait*/ QGestureEvent_setAccepted<()> for (&'a mut QGesture, i8) {
  fn setAccepted(self, rsthis: &mut QGestureEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGestureEvent11setAcceptedEP8QGestureb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as int8_t;
     unsafe {_ZN13QGestureEvent11setAcceptedEP8QGestureb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGestureEvent {
  pub fn setWidget<RetType, T: QGestureEvent_setWidget<RetType>>(&mut self, value: T) -> RetType {
    return value.setWidget(self);
    // return 1;
  }
}

pub trait QGestureEvent_setWidget<RetType> {
  fn setWidget(self, rsthis: &mut QGestureEvent) -> RetType;
}

// proto:  void QGestureEvent::setWidget(QWidget * widget);
impl<'a> /*trait*/ QGestureEvent_setWidget<()> for (&'a mut QWidget) {
  fn setWidget(self, rsthis: &mut QGestureEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGestureEvent9setWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QGestureEvent9setWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGestureEvent {
  pub fn FreeQGestureEvent<RetType, T: QGestureEvent_FreeQGestureEvent<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQGestureEvent(self);
    // return 1;
  }
}

pub trait QGestureEvent_FreeQGestureEvent<RetType> {
  fn FreeQGestureEvent(self, rsthis: &mut QGestureEvent) -> RetType;
}

// proto:  void QGestureEvent::FreeQGestureEvent();
impl<'a> /*trait*/ QGestureEvent_FreeQGestureEvent<()> for () {
  fn FreeQGestureEvent(self, rsthis: &mut QGestureEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGestureEventD0Ev()};
     unsafe {_ZN13QGestureEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGestureEvent {
  pub fn canceledGestures<RetType, T: QGestureEvent_canceledGestures<RetType>>(&mut self, value: T) -> RetType {
    return value.canceledGestures(self);
    // return 1;
  }
}

pub trait QGestureEvent_canceledGestures<RetType> {
  fn canceledGestures(self, rsthis: &mut QGestureEvent) -> RetType;
}

// proto:  QList<QGesture *> QGestureEvent::canceledGestures();
impl<'a> /*trait*/ QGestureEvent_canceledGestures<()> for () {
  fn canceledGestures(self, rsthis: &mut QGestureEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGestureEvent16canceledGesturesEv()};
     unsafe {_ZNK13QGestureEvent16canceledGesturesEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGestureEvent {
  pub fn mapToGraphicsScene<RetType, T: QGestureEvent_mapToGraphicsScene<RetType>>(&mut self, value: T) -> RetType {
    return value.mapToGraphicsScene(self);
    // return 1;
  }
}

pub trait QGestureEvent_mapToGraphicsScene<RetType> {
  fn mapToGraphicsScene(self, rsthis: &mut QGestureEvent) -> RetType;
}

// proto:  QPointF QGestureEvent::mapToGraphicsScene(const QPointF & gesturePoint);
impl<'a> /*trait*/ QGestureEvent_mapToGraphicsScene<QPointF> for (&'a  QPointF) {
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

