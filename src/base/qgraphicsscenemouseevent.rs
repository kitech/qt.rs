// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpoint::QPoint;
use super::qpointf::QPointF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QPoint QGraphicsSceneMouseEvent::screenPos();
  fn _ZNK24QGraphicsSceneMouseEvent9screenPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsSceneMouseEvent::NewQGraphicsSceneMouseEvent(const QGraphicsSceneMouseEvent & );
  fn _ZN24QGraphicsSceneMouseEventC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QPointF QGraphicsSceneMouseEvent::lastScenePos();
  fn _ZNK24QGraphicsSceneMouseEvent12lastScenePosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsSceneMouseEvent::FreeQGraphicsSceneMouseEvent();
  fn _ZN24QGraphicsSceneMouseEventD0Ev(qthis: *mut c_void) ;
  // proto:  QPointF QGraphicsSceneMouseEvent::pos();
  fn _ZNK24QGraphicsSceneMouseEvent3posEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsSceneMouseEvent::setLastPos(const QPointF & pos);
  fn _ZN24QGraphicsSceneMouseEvent10setLastPosERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsSceneMouseEvent::setLastScenePos(const QPointF & pos);
  fn _ZN24QGraphicsSceneMouseEvent15setLastScenePosERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QPoint QGraphicsSceneMouseEvent::lastScreenPos();
  fn _ZNK24QGraphicsSceneMouseEvent13lastScreenPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsSceneMouseEvent::setScreenPos(const QPoint & pos);
  fn _ZN24QGraphicsSceneMouseEvent12setScreenPosERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsSceneMouseEvent::setLastScreenPos(const QPoint & pos);
  fn _ZN24QGraphicsSceneMouseEvent16setLastScreenPosERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsSceneMouseEvent::setScenePos(const QPointF & pos);
  fn _ZN24QGraphicsSceneMouseEvent11setScenePosERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QPointF QGraphicsSceneMouseEvent::lastPos();
  fn _ZNK24QGraphicsSceneMouseEvent7lastPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPointF QGraphicsSceneMouseEvent::scenePos();
  fn _ZNK24QGraphicsSceneMouseEvent8scenePosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsSceneMouseEvent::setPos(const QPointF & pos);
  fn _ZN24QGraphicsSceneMouseEvent6setPosERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QGraphicsSceneMouseEvent)=1
pub struct QGraphicsSceneMouseEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn screenPos<T: QGraphicsSceneMouseEvent_screenPos>(&mut self, value: T) -> QPoint {
    return value.screenPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_screenPos {
  fn screenPos(self, rsthis: &mut QGraphicsSceneMouseEvent) -> QPoint;
}

// proto:  QPoint QGraphicsSceneMouseEvent::screenPos();
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_screenPos for () {
  fn screenPos(self, rsthis: &mut QGraphicsSceneMouseEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneMouseEvent9screenPosEv()};
    let mut ret = unsafe {_ZNK24QGraphicsSceneMouseEvent9screenPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn NewQGraphicsSceneMouseEvent<T: QGraphicsSceneMouseEvent_NewQGraphicsSceneMouseEvent>(value: T) -> QGraphicsSceneMouseEvent {
    let rsthis = value.NewQGraphicsSceneMouseEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_NewQGraphicsSceneMouseEvent {
  fn NewQGraphicsSceneMouseEvent(self) -> QGraphicsSceneMouseEvent;
}

// proto: void QGraphicsSceneMouseEvent::NewQGraphicsSceneMouseEvent(const QGraphicsSceneMouseEvent & );
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_NewQGraphicsSceneMouseEvent for (&'a  QGraphicsSceneMouseEvent) {
  fn NewQGraphicsSceneMouseEvent(self) -> QGraphicsSceneMouseEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneMouseEventC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN24QGraphicsSceneMouseEventC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsSceneMouseEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn lastScenePos<T: QGraphicsSceneMouseEvent_lastScenePos>(&mut self, value: T) -> QPointF {
    return value.lastScenePos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_lastScenePos {
  fn lastScenePos(self, rsthis: &mut QGraphicsSceneMouseEvent) -> QPointF;
}

// proto:  QPointF QGraphicsSceneMouseEvent::lastScenePos();
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_lastScenePos for () {
  fn lastScenePos(self, rsthis: &mut QGraphicsSceneMouseEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneMouseEvent12lastScenePosEv()};
    let mut ret = unsafe {_ZNK24QGraphicsSceneMouseEvent12lastScenePosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn FreeQGraphicsSceneMouseEvent<T: QGraphicsSceneMouseEvent_FreeQGraphicsSceneMouseEvent>(&mut self, value: T)  {
     value.FreeQGraphicsSceneMouseEvent(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_FreeQGraphicsSceneMouseEvent {
  fn FreeQGraphicsSceneMouseEvent(self, rsthis: &mut QGraphicsSceneMouseEvent) ;
}

// proto:  void QGraphicsSceneMouseEvent::FreeQGraphicsSceneMouseEvent();
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_FreeQGraphicsSceneMouseEvent for () {
  fn FreeQGraphicsSceneMouseEvent(self, rsthis: &mut QGraphicsSceneMouseEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneMouseEventD0Ev()};
     unsafe {_ZN24QGraphicsSceneMouseEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn pos<T: QGraphicsSceneMouseEvent_pos>(&mut self, value: T) -> QPointF {
    return value.pos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_pos {
  fn pos(self, rsthis: &mut QGraphicsSceneMouseEvent) -> QPointF;
}

// proto:  QPointF QGraphicsSceneMouseEvent::pos();
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_pos for () {
  fn pos(self, rsthis: &mut QGraphicsSceneMouseEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneMouseEvent3posEv()};
    let mut ret = unsafe {_ZNK24QGraphicsSceneMouseEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn setLastPos<T: QGraphicsSceneMouseEvent_setLastPos>(&mut self, value: T)  {
     value.setLastPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_setLastPos {
  fn setLastPos(self, rsthis: &mut QGraphicsSceneMouseEvent) ;
}

// proto:  void QGraphicsSceneMouseEvent::setLastPos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_setLastPos for (&'a  QPointF) {
  fn setLastPos(self, rsthis: &mut QGraphicsSceneMouseEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneMouseEvent10setLastPosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QGraphicsSceneMouseEvent10setLastPosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn setLastScenePos<T: QGraphicsSceneMouseEvent_setLastScenePos>(&mut self, value: T)  {
     value.setLastScenePos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_setLastScenePos {
  fn setLastScenePos(self, rsthis: &mut QGraphicsSceneMouseEvent) ;
}

// proto:  void QGraphicsSceneMouseEvent::setLastScenePos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_setLastScenePos for (&'a  QPointF) {
  fn setLastScenePos(self, rsthis: &mut QGraphicsSceneMouseEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneMouseEvent15setLastScenePosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QGraphicsSceneMouseEvent15setLastScenePosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn lastScreenPos<T: QGraphicsSceneMouseEvent_lastScreenPos>(&mut self, value: T) -> QPoint {
    return value.lastScreenPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_lastScreenPos {
  fn lastScreenPos(self, rsthis: &mut QGraphicsSceneMouseEvent) -> QPoint;
}

// proto:  QPoint QGraphicsSceneMouseEvent::lastScreenPos();
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_lastScreenPos for () {
  fn lastScreenPos(self, rsthis: &mut QGraphicsSceneMouseEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneMouseEvent13lastScreenPosEv()};
    let mut ret = unsafe {_ZNK24QGraphicsSceneMouseEvent13lastScreenPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn setScreenPos<T: QGraphicsSceneMouseEvent_setScreenPos>(&mut self, value: T)  {
     value.setScreenPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_setScreenPos {
  fn setScreenPos(self, rsthis: &mut QGraphicsSceneMouseEvent) ;
}

// proto:  void QGraphicsSceneMouseEvent::setScreenPos(const QPoint & pos);
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_setScreenPos for (&'a  QPoint) {
  fn setScreenPos(self, rsthis: &mut QGraphicsSceneMouseEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneMouseEvent12setScreenPosERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QGraphicsSceneMouseEvent12setScreenPosERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn setLastScreenPos<T: QGraphicsSceneMouseEvent_setLastScreenPos>(&mut self, value: T)  {
     value.setLastScreenPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_setLastScreenPos {
  fn setLastScreenPos(self, rsthis: &mut QGraphicsSceneMouseEvent) ;
}

// proto:  void QGraphicsSceneMouseEvent::setLastScreenPos(const QPoint & pos);
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_setLastScreenPos for (&'a  QPoint) {
  fn setLastScreenPos(self, rsthis: &mut QGraphicsSceneMouseEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneMouseEvent16setLastScreenPosERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QGraphicsSceneMouseEvent16setLastScreenPosERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn setScenePos<T: QGraphicsSceneMouseEvent_setScenePos>(&mut self, value: T)  {
     value.setScenePos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_setScenePos {
  fn setScenePos(self, rsthis: &mut QGraphicsSceneMouseEvent) ;
}

// proto:  void QGraphicsSceneMouseEvent::setScenePos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_setScenePos for (&'a  QPointF) {
  fn setScenePos(self, rsthis: &mut QGraphicsSceneMouseEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneMouseEvent11setScenePosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QGraphicsSceneMouseEvent11setScenePosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn lastPos<T: QGraphicsSceneMouseEvent_lastPos>(&mut self, value: T) -> QPointF {
    return value.lastPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_lastPos {
  fn lastPos(self, rsthis: &mut QGraphicsSceneMouseEvent) -> QPointF;
}

// proto:  QPointF QGraphicsSceneMouseEvent::lastPos();
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_lastPos for () {
  fn lastPos(self, rsthis: &mut QGraphicsSceneMouseEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneMouseEvent7lastPosEv()};
    let mut ret = unsafe {_ZNK24QGraphicsSceneMouseEvent7lastPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn scenePos<T: QGraphicsSceneMouseEvent_scenePos>(&mut self, value: T) -> QPointF {
    return value.scenePos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_scenePos {
  fn scenePos(self, rsthis: &mut QGraphicsSceneMouseEvent) -> QPointF;
}

// proto:  QPointF QGraphicsSceneMouseEvent::scenePos();
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_scenePos for () {
  fn scenePos(self, rsthis: &mut QGraphicsSceneMouseEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneMouseEvent8scenePosEv()};
    let mut ret = unsafe {_ZNK24QGraphicsSceneMouseEvent8scenePosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn setPos<T: QGraphicsSceneMouseEvent_setPos>(&mut self, value: T)  {
     value.setPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_setPos {
  fn setPos(self, rsthis: &mut QGraphicsSceneMouseEvent) ;
}

// proto:  void QGraphicsSceneMouseEvent::setPos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_setPos for (&'a  QPointF) {
  fn setPos(self, rsthis: &mut QGraphicsSceneMouseEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneMouseEvent6setPosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QGraphicsSceneMouseEvent6setPosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

