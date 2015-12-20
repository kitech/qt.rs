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
  // proto:  QPointF QGraphicsSceneWheelEvent::pos();
  fn _ZNK24QGraphicsSceneWheelEvent3posEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsSceneWheelEvent::~QGraphicsSceneWheelEvent();
  fn _ZN24QGraphicsSceneWheelEventD0Ev(qthis: *mut c_void);
  // proto:  void QGraphicsSceneWheelEvent::setDelta(int delta);
  fn _ZN24QGraphicsSceneWheelEvent8setDeltaEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QGraphicsSceneWheelEvent::setScenePos(const QPointF & pos);
  fn _ZN24QGraphicsSceneWheelEvent11setScenePosERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGraphicsSceneWheelEvent::QGraphicsSceneWheelEvent(const QGraphicsSceneWheelEvent & );
  fn _ZN24QGraphicsSceneWheelEventC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGraphicsSceneWheelEvent::setPos(const QPointF & pos);
  fn _ZN24QGraphicsSceneWheelEvent6setPosERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGraphicsSceneWheelEvent::setScreenPos(const QPoint & pos);
  fn _ZN24QGraphicsSceneWheelEvent12setScreenPosERK6QPoint(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QGraphicsSceneWheelEvent::delta();
  fn _ZNK24QGraphicsSceneWheelEvent5deltaEv(qthis: *mut c_void) -> c_int;
  // proto:  QPointF QGraphicsSceneWheelEvent::scenePos();
  fn _ZNK24QGraphicsSceneWheelEvent8scenePosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPoint QGraphicsSceneWheelEvent::screenPos();
  fn _ZNK24QGraphicsSceneWheelEvent9screenPosEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QGraphicsSceneWheelEvent)=1
pub struct QGraphicsSceneWheelEvent {
  pub qclsinst: *mut c_void,
}

  // proto:  QPointF QGraphicsSceneWheelEvent::pos();
impl /*struct*/ QGraphicsSceneWheelEvent {
  pub fn pos<RetType, T: QGraphicsSceneWheelEvent_pos<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneWheelEvent_pos<RetType> {
  fn pos(self , rsthis: &mut QGraphicsSceneWheelEvent) -> RetType;
}

  // proto:  QPointF QGraphicsSceneWheelEvent::pos();
impl<'a> /*trait*/ QGraphicsSceneWheelEvent_pos<QPointF> for () {
  fn pos(self , rsthis: &mut QGraphicsSceneWheelEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneWheelEvent3posEv()};
    let mut ret = unsafe {_ZNK24QGraphicsSceneWheelEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsSceneWheelEvent::~QGraphicsSceneWheelEvent();
impl /*struct*/ QGraphicsSceneWheelEvent {
  pub fn FreeQGraphicsSceneWheelEvent<RetType, T: QGraphicsSceneWheelEvent_FreeQGraphicsSceneWheelEvent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQGraphicsSceneWheelEvent(self);
    // return 1;
  }
}

pub trait QGraphicsSceneWheelEvent_FreeQGraphicsSceneWheelEvent<RetType> {
  fn FreeQGraphicsSceneWheelEvent(self , rsthis: &mut QGraphicsSceneWheelEvent) -> RetType;
}

  // proto:  void QGraphicsSceneWheelEvent::~QGraphicsSceneWheelEvent();
impl<'a> /*trait*/ QGraphicsSceneWheelEvent_FreeQGraphicsSceneWheelEvent<()> for () {
  fn FreeQGraphicsSceneWheelEvent(self , rsthis: &mut QGraphicsSceneWheelEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneWheelEventD0Ev()};
     unsafe {_ZN24QGraphicsSceneWheelEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsSceneWheelEvent::setDelta(int delta);
impl /*struct*/ QGraphicsSceneWheelEvent {
  pub fn setDelta<RetType, T: QGraphicsSceneWheelEvent_setDelta<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDelta(self);
    // return 1;
  }
}

pub trait QGraphicsSceneWheelEvent_setDelta<RetType> {
  fn setDelta(self , rsthis: &mut QGraphicsSceneWheelEvent) -> RetType;
}

  // proto:  void QGraphicsSceneWheelEvent::setDelta(int delta);
impl<'a> /*trait*/ QGraphicsSceneWheelEvent_setDelta<()> for (i32) {
  fn setDelta(self , rsthis: &mut QGraphicsSceneWheelEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneWheelEvent8setDeltaEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN24QGraphicsSceneWheelEvent8setDeltaEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsSceneWheelEvent::setScenePos(const QPointF & pos);
impl /*struct*/ QGraphicsSceneWheelEvent {
  pub fn setScenePos<RetType, T: QGraphicsSceneWheelEvent_setScenePos<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setScenePos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneWheelEvent_setScenePos<RetType> {
  fn setScenePos(self , rsthis: &mut QGraphicsSceneWheelEvent) -> RetType;
}

  // proto:  void QGraphicsSceneWheelEvent::setScenePos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneWheelEvent_setScenePos<()> for (QPointF) {
  fn setScenePos(self , rsthis: &mut QGraphicsSceneWheelEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneWheelEvent11setScenePosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QGraphicsSceneWheelEvent11setScenePosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsSceneWheelEvent::QGraphicsSceneWheelEvent(const QGraphicsSceneWheelEvent & );
impl /*struct*/ QGraphicsSceneWheelEvent {
  pub fn NewQGraphicsSceneWheelEvent<T: QGraphicsSceneWheelEvent_NewQGraphicsSceneWheelEvent>(value: T) -> QGraphicsSceneWheelEvent {
    let rsthis = value.NewQGraphicsSceneWheelEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsSceneWheelEvent_NewQGraphicsSceneWheelEvent {
  fn NewQGraphicsSceneWheelEvent(self) -> QGraphicsSceneWheelEvent;
}

  // proto:  void QGraphicsSceneWheelEvent::QGraphicsSceneWheelEvent(const QGraphicsSceneWheelEvent & );
impl<'a> /*trait*/ QGraphicsSceneWheelEvent_NewQGraphicsSceneWheelEvent for (QGraphicsSceneWheelEvent) {
  fn NewQGraphicsSceneWheelEvent(self) -> QGraphicsSceneWheelEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneWheelEventC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN24QGraphicsSceneWheelEventC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsSceneWheelEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGraphicsSceneWheelEvent::setPos(const QPointF & pos);
impl /*struct*/ QGraphicsSceneWheelEvent {
  pub fn setPos<RetType, T: QGraphicsSceneWheelEvent_setPos<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneWheelEvent_setPos<RetType> {
  fn setPos(self , rsthis: &mut QGraphicsSceneWheelEvent) -> RetType;
}

  // proto:  void QGraphicsSceneWheelEvent::setPos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneWheelEvent_setPos<()> for (QPointF) {
  fn setPos(self , rsthis: &mut QGraphicsSceneWheelEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneWheelEvent6setPosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QGraphicsSceneWheelEvent6setPosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsSceneWheelEvent::setScreenPos(const QPoint & pos);
impl /*struct*/ QGraphicsSceneWheelEvent {
  pub fn setScreenPos<RetType, T: QGraphicsSceneWheelEvent_setScreenPos<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setScreenPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneWheelEvent_setScreenPos<RetType> {
  fn setScreenPos(self , rsthis: &mut QGraphicsSceneWheelEvent) -> RetType;
}

  // proto:  void QGraphicsSceneWheelEvent::setScreenPos(const QPoint & pos);
impl<'a> /*trait*/ QGraphicsSceneWheelEvent_setScreenPos<()> for (QPoint) {
  fn setScreenPos(self , rsthis: &mut QGraphicsSceneWheelEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneWheelEvent12setScreenPosERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QGraphicsSceneWheelEvent12setScreenPosERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QGraphicsSceneWheelEvent::delta();
impl /*struct*/ QGraphicsSceneWheelEvent {
  pub fn delta<RetType, T: QGraphicsSceneWheelEvent_delta<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.delta(self);
    // return 1;
  }
}

pub trait QGraphicsSceneWheelEvent_delta<RetType> {
  fn delta(self , rsthis: &mut QGraphicsSceneWheelEvent) -> RetType;
}

  // proto:  int QGraphicsSceneWheelEvent::delta();
impl<'a> /*trait*/ QGraphicsSceneWheelEvent_delta<i32> for () {
  fn delta(self , rsthis: &mut QGraphicsSceneWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneWheelEvent5deltaEv()};
    let mut ret = unsafe {_ZNK24QGraphicsSceneWheelEvent5deltaEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QPointF QGraphicsSceneWheelEvent::scenePos();
impl /*struct*/ QGraphicsSceneWheelEvent {
  pub fn scenePos<RetType, T: QGraphicsSceneWheelEvent_scenePos<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.scenePos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneWheelEvent_scenePos<RetType> {
  fn scenePos(self , rsthis: &mut QGraphicsSceneWheelEvent) -> RetType;
}

  // proto:  QPointF QGraphicsSceneWheelEvent::scenePos();
impl<'a> /*trait*/ QGraphicsSceneWheelEvent_scenePos<QPointF> for () {
  fn scenePos(self , rsthis: &mut QGraphicsSceneWheelEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneWheelEvent8scenePosEv()};
    let mut ret = unsafe {_ZNK24QGraphicsSceneWheelEvent8scenePosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QPoint QGraphicsSceneWheelEvent::screenPos();
impl /*struct*/ QGraphicsSceneWheelEvent {
  pub fn screenPos<RetType, T: QGraphicsSceneWheelEvent_screenPos<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.screenPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneWheelEvent_screenPos<RetType> {
  fn screenPos(self , rsthis: &mut QGraphicsSceneWheelEvent) -> RetType;
}

  // proto:  QPoint QGraphicsSceneWheelEvent::screenPos();
impl<'a> /*trait*/ QGraphicsSceneWheelEvent_screenPos<QPoint> for () {
  fn screenPos(self , rsthis: &mut QGraphicsSceneWheelEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneWheelEvent9screenPosEv()};
    let mut ret = unsafe {_ZNK24QGraphicsSceneWheelEvent9screenPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

