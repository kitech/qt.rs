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
  // proto:  void QGraphicsSceneContextMenuEvent::QGraphicsSceneContextMenuEvent(const QGraphicsSceneContextMenuEvent & );
  fn _ZN30QGraphicsSceneContextMenuEventC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QPointF QGraphicsSceneContextMenuEvent::scenePos();
  fn _ZNK30QGraphicsSceneContextMenuEvent8scenePosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsSceneContextMenuEvent::~QGraphicsSceneContextMenuEvent();
  fn _ZN30QGraphicsSceneContextMenuEventD0Ev(qthis: *mut c_void);
  // proto:  QPointF QGraphicsSceneContextMenuEvent::pos();
  fn _ZNK30QGraphicsSceneContextMenuEvent3posEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsSceneContextMenuEvent::setScreenPos(const QPoint & pos);
  fn _ZN30QGraphicsSceneContextMenuEvent12setScreenPosERK6QPoint(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGraphicsSceneContextMenuEvent::setPos(const QPointF & pos);
  fn _ZN30QGraphicsSceneContextMenuEvent6setPosERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QPoint QGraphicsSceneContextMenuEvent::screenPos();
  fn _ZNK30QGraphicsSceneContextMenuEvent9screenPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsSceneContextMenuEvent::setScenePos(const QPointF & pos);
  fn _ZN30QGraphicsSceneContextMenuEvent11setScenePosERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
}

// body block begin
// class sizeof(QGraphicsSceneContextMenuEvent)=1
pub struct QGraphicsSceneContextMenuEvent {
  pub qclsinst: *mut c_void,
}

  // proto:  void QGraphicsSceneContextMenuEvent::QGraphicsSceneContextMenuEvent(const QGraphicsSceneContextMenuEvent & );
impl /*struct*/ QGraphicsSceneContextMenuEvent {
  pub fn NewQGraphicsSceneContextMenuEvent<T: QGraphicsSceneContextMenuEvent_NewQGraphicsSceneContextMenuEvent>(value: T) -> QGraphicsSceneContextMenuEvent {
    let rsthis = value.NewQGraphicsSceneContextMenuEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsSceneContextMenuEvent_NewQGraphicsSceneContextMenuEvent {
  fn NewQGraphicsSceneContextMenuEvent(self) -> QGraphicsSceneContextMenuEvent;
}

  // proto:  void QGraphicsSceneContextMenuEvent::QGraphicsSceneContextMenuEvent(const QGraphicsSceneContextMenuEvent & );
impl<'a> /*trait*/ QGraphicsSceneContextMenuEvent_NewQGraphicsSceneContextMenuEvent for (QGraphicsSceneContextMenuEvent) {
  fn NewQGraphicsSceneContextMenuEvent(self) -> QGraphicsSceneContextMenuEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN30QGraphicsSceneContextMenuEventC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN30QGraphicsSceneContextMenuEventC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsSceneContextMenuEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPointF QGraphicsSceneContextMenuEvent::scenePos();
impl /*struct*/ QGraphicsSceneContextMenuEvent {
  pub fn scenePos<RetType, T: QGraphicsSceneContextMenuEvent_scenePos<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.scenePos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneContextMenuEvent_scenePos<RetType> {
  fn scenePos(self , rsthis: &mut QGraphicsSceneContextMenuEvent) -> RetType;
}

  // proto:  QPointF QGraphicsSceneContextMenuEvent::scenePos();
impl<'a> /*trait*/ QGraphicsSceneContextMenuEvent_scenePos<QPointF> for () {
  fn scenePos(self , rsthis: &mut QGraphicsSceneContextMenuEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK30QGraphicsSceneContextMenuEvent8scenePosEv()};
    let mut ret = unsafe {_ZNK30QGraphicsSceneContextMenuEvent8scenePosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsSceneContextMenuEvent::~QGraphicsSceneContextMenuEvent();
impl /*struct*/ QGraphicsSceneContextMenuEvent {
  pub fn FreeQGraphicsSceneContextMenuEvent<RetType, T: QGraphicsSceneContextMenuEvent_FreeQGraphicsSceneContextMenuEvent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQGraphicsSceneContextMenuEvent(self);
    // return 1;
  }
}

pub trait QGraphicsSceneContextMenuEvent_FreeQGraphicsSceneContextMenuEvent<RetType> {
  fn FreeQGraphicsSceneContextMenuEvent(self , rsthis: &mut QGraphicsSceneContextMenuEvent) -> RetType;
}

  // proto:  void QGraphicsSceneContextMenuEvent::~QGraphicsSceneContextMenuEvent();
impl<'a> /*trait*/ QGraphicsSceneContextMenuEvent_FreeQGraphicsSceneContextMenuEvent<()> for () {
  fn FreeQGraphicsSceneContextMenuEvent(self , rsthis: &mut QGraphicsSceneContextMenuEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN30QGraphicsSceneContextMenuEventD0Ev()};
     unsafe {_ZN30QGraphicsSceneContextMenuEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPointF QGraphicsSceneContextMenuEvent::pos();
impl /*struct*/ QGraphicsSceneContextMenuEvent {
  pub fn pos<RetType, T: QGraphicsSceneContextMenuEvent_pos<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneContextMenuEvent_pos<RetType> {
  fn pos(self , rsthis: &mut QGraphicsSceneContextMenuEvent) -> RetType;
}

  // proto:  QPointF QGraphicsSceneContextMenuEvent::pos();
impl<'a> /*trait*/ QGraphicsSceneContextMenuEvent_pos<QPointF> for () {
  fn pos(self , rsthis: &mut QGraphicsSceneContextMenuEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK30QGraphicsSceneContextMenuEvent3posEv()};
    let mut ret = unsafe {_ZNK30QGraphicsSceneContextMenuEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsSceneContextMenuEvent::setScreenPos(const QPoint & pos);
impl /*struct*/ QGraphicsSceneContextMenuEvent {
  pub fn setScreenPos<RetType, T: QGraphicsSceneContextMenuEvent_setScreenPos<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setScreenPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneContextMenuEvent_setScreenPos<RetType> {
  fn setScreenPos(self , rsthis: &mut QGraphicsSceneContextMenuEvent) -> RetType;
}

  // proto:  void QGraphicsSceneContextMenuEvent::setScreenPos(const QPoint & pos);
impl<'a> /*trait*/ QGraphicsSceneContextMenuEvent_setScreenPos<()> for (QPoint) {
  fn setScreenPos(self , rsthis: &mut QGraphicsSceneContextMenuEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN30QGraphicsSceneContextMenuEvent12setScreenPosERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN30QGraphicsSceneContextMenuEvent12setScreenPosERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsSceneContextMenuEvent::setPos(const QPointF & pos);
impl /*struct*/ QGraphicsSceneContextMenuEvent {
  pub fn setPos<RetType, T: QGraphicsSceneContextMenuEvent_setPos<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneContextMenuEvent_setPos<RetType> {
  fn setPos(self , rsthis: &mut QGraphicsSceneContextMenuEvent) -> RetType;
}

  // proto:  void QGraphicsSceneContextMenuEvent::setPos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneContextMenuEvent_setPos<()> for (QPointF) {
  fn setPos(self , rsthis: &mut QGraphicsSceneContextMenuEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN30QGraphicsSceneContextMenuEvent6setPosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN30QGraphicsSceneContextMenuEvent6setPosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPoint QGraphicsSceneContextMenuEvent::screenPos();
impl /*struct*/ QGraphicsSceneContextMenuEvent {
  pub fn screenPos<RetType, T: QGraphicsSceneContextMenuEvent_screenPos<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.screenPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneContextMenuEvent_screenPos<RetType> {
  fn screenPos(self , rsthis: &mut QGraphicsSceneContextMenuEvent) -> RetType;
}

  // proto:  QPoint QGraphicsSceneContextMenuEvent::screenPos();
impl<'a> /*trait*/ QGraphicsSceneContextMenuEvent_screenPos<QPoint> for () {
  fn screenPos(self , rsthis: &mut QGraphicsSceneContextMenuEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK30QGraphicsSceneContextMenuEvent9screenPosEv()};
    let mut ret = unsafe {_ZNK30QGraphicsSceneContextMenuEvent9screenPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsSceneContextMenuEvent::setScenePos(const QPointF & pos);
impl /*struct*/ QGraphicsSceneContextMenuEvent {
  pub fn setScenePos<RetType, T: QGraphicsSceneContextMenuEvent_setScenePos<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setScenePos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneContextMenuEvent_setScenePos<RetType> {
  fn setScenePos(self , rsthis: &mut QGraphicsSceneContextMenuEvent) -> RetType;
}

  // proto:  void QGraphicsSceneContextMenuEvent::setScenePos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneContextMenuEvent_setScenePos<()> for (QPointF) {
  fn setScenePos(self , rsthis: &mut QGraphicsSceneContextMenuEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN30QGraphicsSceneContextMenuEvent11setScenePosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN30QGraphicsSceneContextMenuEvent11setScenePosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

