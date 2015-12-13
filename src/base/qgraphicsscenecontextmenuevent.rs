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
  // proto:  void QGraphicsSceneContextMenuEvent::NewQGraphicsSceneContextMenuEvent(const QGraphicsSceneContextMenuEvent & );
  fn _ZN30QGraphicsSceneContextMenuEventC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QPointF QGraphicsSceneContextMenuEvent::scenePos();
  fn _ZNK30QGraphicsSceneContextMenuEvent8scenePosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsSceneContextMenuEvent::FreeQGraphicsSceneContextMenuEvent();
  fn _ZN30QGraphicsSceneContextMenuEventD0Ev(qthis: *mut c_void) ;
  // proto:  QPointF QGraphicsSceneContextMenuEvent::pos();
  fn _ZNK30QGraphicsSceneContextMenuEvent3posEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsSceneContextMenuEvent::setScreenPos(const QPoint & pos);
  fn _ZN30QGraphicsSceneContextMenuEvent12setScreenPosERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsSceneContextMenuEvent::setPos(const QPointF & pos);
  fn _ZN30QGraphicsSceneContextMenuEvent6setPosERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QPoint QGraphicsSceneContextMenuEvent::screenPos();
  fn _ZNK30QGraphicsSceneContextMenuEvent9screenPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsSceneContextMenuEvent::setScenePos(const QPointF & pos);
  fn _ZN30QGraphicsSceneContextMenuEvent11setScenePosERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QGraphicsSceneContextMenuEvent)=1
pub struct QGraphicsSceneContextMenuEvent {
  pub qclsinst: *mut c_void,
}

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

// proto: void QGraphicsSceneContextMenuEvent::NewQGraphicsSceneContextMenuEvent(const QGraphicsSceneContextMenuEvent & );
impl<'a> /*trait*/ QGraphicsSceneContextMenuEvent_NewQGraphicsSceneContextMenuEvent for (&'a  QGraphicsSceneContextMenuEvent) {
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

impl /*struct*/ QGraphicsSceneContextMenuEvent {
  pub fn scenePos<T: QGraphicsSceneContextMenuEvent_scenePos>(&mut self, value: T) -> QPointF {
    return value.scenePos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneContextMenuEvent_scenePos {
  fn scenePos(self, rsthis: &mut QGraphicsSceneContextMenuEvent) -> QPointF;
}

// proto:  QPointF QGraphicsSceneContextMenuEvent::scenePos();
impl<'a> /*trait*/ QGraphicsSceneContextMenuEvent_scenePos for () {
  fn scenePos(self, rsthis: &mut QGraphicsSceneContextMenuEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK30QGraphicsSceneContextMenuEvent8scenePosEv()};
    let mut ret = unsafe {_ZNK30QGraphicsSceneContextMenuEvent8scenePosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneContextMenuEvent {
  pub fn FreeQGraphicsSceneContextMenuEvent<T: QGraphicsSceneContextMenuEvent_FreeQGraphicsSceneContextMenuEvent>(&mut self, value: T)  {
     value.FreeQGraphicsSceneContextMenuEvent(self);
    // return 1;
  }
}

pub trait QGraphicsSceneContextMenuEvent_FreeQGraphicsSceneContextMenuEvent {
  fn FreeQGraphicsSceneContextMenuEvent(self, rsthis: &mut QGraphicsSceneContextMenuEvent) ;
}

// proto:  void QGraphicsSceneContextMenuEvent::FreeQGraphicsSceneContextMenuEvent();
impl<'a> /*trait*/ QGraphicsSceneContextMenuEvent_FreeQGraphicsSceneContextMenuEvent for () {
  fn FreeQGraphicsSceneContextMenuEvent(self, rsthis: &mut QGraphicsSceneContextMenuEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN30QGraphicsSceneContextMenuEventD0Ev()};
     unsafe {_ZN30QGraphicsSceneContextMenuEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneContextMenuEvent {
  pub fn pos<T: QGraphicsSceneContextMenuEvent_pos>(&mut self, value: T) -> QPointF {
    return value.pos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneContextMenuEvent_pos {
  fn pos(self, rsthis: &mut QGraphicsSceneContextMenuEvent) -> QPointF;
}

// proto:  QPointF QGraphicsSceneContextMenuEvent::pos();
impl<'a> /*trait*/ QGraphicsSceneContextMenuEvent_pos for () {
  fn pos(self, rsthis: &mut QGraphicsSceneContextMenuEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK30QGraphicsSceneContextMenuEvent3posEv()};
    let mut ret = unsafe {_ZNK30QGraphicsSceneContextMenuEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneContextMenuEvent {
  pub fn setScreenPos<T: QGraphicsSceneContextMenuEvent_setScreenPos>(&mut self, value: T)  {
     value.setScreenPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneContextMenuEvent_setScreenPos {
  fn setScreenPos(self, rsthis: &mut QGraphicsSceneContextMenuEvent) ;
}

// proto:  void QGraphicsSceneContextMenuEvent::setScreenPos(const QPoint & pos);
impl<'a> /*trait*/ QGraphicsSceneContextMenuEvent_setScreenPos for (&'a  QPoint) {
  fn setScreenPos(self, rsthis: &mut QGraphicsSceneContextMenuEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN30QGraphicsSceneContextMenuEvent12setScreenPosERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN30QGraphicsSceneContextMenuEvent12setScreenPosERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneContextMenuEvent {
  pub fn setPos<T: QGraphicsSceneContextMenuEvent_setPos>(&mut self, value: T)  {
     value.setPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneContextMenuEvent_setPos {
  fn setPos(self, rsthis: &mut QGraphicsSceneContextMenuEvent) ;
}

// proto:  void QGraphicsSceneContextMenuEvent::setPos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneContextMenuEvent_setPos for (&'a  QPointF) {
  fn setPos(self, rsthis: &mut QGraphicsSceneContextMenuEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN30QGraphicsSceneContextMenuEvent6setPosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN30QGraphicsSceneContextMenuEvent6setPosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneContextMenuEvent {
  pub fn screenPos<T: QGraphicsSceneContextMenuEvent_screenPos>(&mut self, value: T) -> QPoint {
    return value.screenPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneContextMenuEvent_screenPos {
  fn screenPos(self, rsthis: &mut QGraphicsSceneContextMenuEvent) -> QPoint;
}

// proto:  QPoint QGraphicsSceneContextMenuEvent::screenPos();
impl<'a> /*trait*/ QGraphicsSceneContextMenuEvent_screenPos for () {
  fn screenPos(self, rsthis: &mut QGraphicsSceneContextMenuEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK30QGraphicsSceneContextMenuEvent9screenPosEv()};
    let mut ret = unsafe {_ZNK30QGraphicsSceneContextMenuEvent9screenPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneContextMenuEvent {
  pub fn setScenePos<T: QGraphicsSceneContextMenuEvent_setScenePos>(&mut self, value: T)  {
     value.setScenePos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneContextMenuEvent_setScenePos {
  fn setScenePos(self, rsthis: &mut QGraphicsSceneContextMenuEvent) ;
}

// proto:  void QGraphicsSceneContextMenuEvent::setScenePos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneContextMenuEvent_setScenePos for (&'a  QPointF) {
  fn setScenePos(self, rsthis: &mut QGraphicsSceneContextMenuEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN30QGraphicsSceneContextMenuEvent11setScenePosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN30QGraphicsSceneContextMenuEvent11setScenePosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

