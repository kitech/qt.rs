// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpointf::QPointF;
use super::qpoint::QPoint;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QGraphicsSceneHelpEvent::setScenePos(const QPointF & pos);
  fn _ZN23QGraphicsSceneHelpEvent11setScenePosERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QPoint QGraphicsSceneHelpEvent::screenPos();
  fn _ZNK23QGraphicsSceneHelpEvent9screenPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsSceneHelpEvent::FreeQGraphicsSceneHelpEvent();
  fn _ZN23QGraphicsSceneHelpEventD0Ev(qthis: *mut c_void) ;
  // proto:  void QGraphicsSceneHelpEvent::NewQGraphicsSceneHelpEvent(const QGraphicsSceneHelpEvent & );
  fn _ZN23QGraphicsSceneHelpEventC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsSceneHelpEvent::setScreenPos(const QPoint & pos);
  fn _ZN23QGraphicsSceneHelpEvent12setScreenPosERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QPointF QGraphicsSceneHelpEvent::scenePos();
  fn _ZNK23QGraphicsSceneHelpEvent8scenePosEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QGraphicsSceneHelpEvent)=1
pub struct QGraphicsSceneHelpEvent {
  pub qclsinst: *mut c_void,
}

// proto:  void QGraphicsSceneHelpEvent::setScenePos(const QPointF & pos);
impl /*struct*/ QGraphicsSceneHelpEvent {
  pub fn setScenePos<RetType, T: QGraphicsSceneHelpEvent_setScenePos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setScenePos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneHelpEvent_setScenePos<RetType> {
  fn setScenePos(self , rsthis: &mut QGraphicsSceneHelpEvent) -> RetType;
}

// proto:  void QGraphicsSceneHelpEvent::setScenePos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneHelpEvent_setScenePos<()> for (&'a  QPointF) {
  fn setScenePos(self , rsthis: &mut QGraphicsSceneHelpEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSceneHelpEvent11setScenePosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN23QGraphicsSceneHelpEvent11setScenePosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QPoint QGraphicsSceneHelpEvent::screenPos();
impl /*struct*/ QGraphicsSceneHelpEvent {
  pub fn screenPos<RetType, T: QGraphicsSceneHelpEvent_screenPos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.screenPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneHelpEvent_screenPos<RetType> {
  fn screenPos(self , rsthis: &mut QGraphicsSceneHelpEvent) -> RetType;
}

// proto:  QPoint QGraphicsSceneHelpEvent::screenPos();
impl<'a> /*trait*/ QGraphicsSceneHelpEvent_screenPos<QPoint> for () {
  fn screenPos(self , rsthis: &mut QGraphicsSceneHelpEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsSceneHelpEvent9screenPosEv()};
    let mut ret = unsafe {_ZNK23QGraphicsSceneHelpEvent9screenPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QGraphicsSceneHelpEvent::FreeQGraphicsSceneHelpEvent();
impl /*struct*/ QGraphicsSceneHelpEvent {
  pub fn FreeQGraphicsSceneHelpEvent<RetType, T: QGraphicsSceneHelpEvent_FreeQGraphicsSceneHelpEvent<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQGraphicsSceneHelpEvent(self);
    // return 1;
  }
}

pub trait QGraphicsSceneHelpEvent_FreeQGraphicsSceneHelpEvent<RetType> {
  fn FreeQGraphicsSceneHelpEvent(self , rsthis: &mut QGraphicsSceneHelpEvent) -> RetType;
}

// proto:  void QGraphicsSceneHelpEvent::FreeQGraphicsSceneHelpEvent();
impl<'a> /*trait*/ QGraphicsSceneHelpEvent_FreeQGraphicsSceneHelpEvent<()> for () {
  fn FreeQGraphicsSceneHelpEvent(self , rsthis: &mut QGraphicsSceneHelpEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSceneHelpEventD0Ev()};
     unsafe {_ZN23QGraphicsSceneHelpEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneHelpEvent {
  pub fn NewQGraphicsSceneHelpEvent<T: QGraphicsSceneHelpEvent_NewQGraphicsSceneHelpEvent>(value: T) -> QGraphicsSceneHelpEvent {
    let rsthis = value.NewQGraphicsSceneHelpEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsSceneHelpEvent_NewQGraphicsSceneHelpEvent {
  fn NewQGraphicsSceneHelpEvent(self) -> QGraphicsSceneHelpEvent;
}

// proto: void QGraphicsSceneHelpEvent::NewQGraphicsSceneHelpEvent(const QGraphicsSceneHelpEvent & );
impl<'a> /*trait*/ QGraphicsSceneHelpEvent_NewQGraphicsSceneHelpEvent for (&'a  QGraphicsSceneHelpEvent) {
  fn NewQGraphicsSceneHelpEvent(self) -> QGraphicsSceneHelpEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSceneHelpEventC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN23QGraphicsSceneHelpEventC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsSceneHelpEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QGraphicsSceneHelpEvent::setScreenPos(const QPoint & pos);
impl /*struct*/ QGraphicsSceneHelpEvent {
  pub fn setScreenPos<RetType, T: QGraphicsSceneHelpEvent_setScreenPos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setScreenPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneHelpEvent_setScreenPos<RetType> {
  fn setScreenPos(self , rsthis: &mut QGraphicsSceneHelpEvent) -> RetType;
}

// proto:  void QGraphicsSceneHelpEvent::setScreenPos(const QPoint & pos);
impl<'a> /*trait*/ QGraphicsSceneHelpEvent_setScreenPos<()> for (&'a  QPoint) {
  fn setScreenPos(self , rsthis: &mut QGraphicsSceneHelpEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSceneHelpEvent12setScreenPosERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN23QGraphicsSceneHelpEvent12setScreenPosERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QPointF QGraphicsSceneHelpEvent::scenePos();
impl /*struct*/ QGraphicsSceneHelpEvent {
  pub fn scenePos<RetType, T: QGraphicsSceneHelpEvent_scenePos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.scenePos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneHelpEvent_scenePos<RetType> {
  fn scenePos(self , rsthis: &mut QGraphicsSceneHelpEvent) -> RetType;
}

// proto:  QPointF QGraphicsSceneHelpEvent::scenePos();
impl<'a> /*trait*/ QGraphicsSceneHelpEvent_scenePos<QPointF> for () {
  fn scenePos(self , rsthis: &mut QGraphicsSceneHelpEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsSceneHelpEvent8scenePosEv()};
    let mut ret = unsafe {_ZNK23QGraphicsSceneHelpEvent8scenePosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

