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

// proto:  QPoint QGraphicsSceneMouseEvent::screenPos();
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn screenPos<RetType, T: QGraphicsSceneMouseEvent_screenPos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.screenPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_screenPos<RetType> {
  fn screenPos(self , rsthis: &mut QGraphicsSceneMouseEvent) -> RetType;
}

// proto:  QPoint QGraphicsSceneMouseEvent::screenPos();
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_screenPos<QPoint> for () {
  fn screenPos(self , rsthis: &mut QGraphicsSceneMouseEvent) -> QPoint {
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

// proto:  QPointF QGraphicsSceneMouseEvent::lastScenePos();
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn lastScenePos<RetType, T: QGraphicsSceneMouseEvent_lastScenePos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.lastScenePos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_lastScenePos<RetType> {
  fn lastScenePos(self , rsthis: &mut QGraphicsSceneMouseEvent) -> RetType;
}

// proto:  QPointF QGraphicsSceneMouseEvent::lastScenePos();
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_lastScenePos<QPointF> for () {
  fn lastScenePos(self , rsthis: &mut QGraphicsSceneMouseEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneMouseEvent12lastScenePosEv()};
    let mut ret = unsafe {_ZNK24QGraphicsSceneMouseEvent12lastScenePosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QGraphicsSceneMouseEvent::FreeQGraphicsSceneMouseEvent();
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn FreeQGraphicsSceneMouseEvent<RetType, T: QGraphicsSceneMouseEvent_FreeQGraphicsSceneMouseEvent<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQGraphicsSceneMouseEvent(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_FreeQGraphicsSceneMouseEvent<RetType> {
  fn FreeQGraphicsSceneMouseEvent(self , rsthis: &mut QGraphicsSceneMouseEvent) -> RetType;
}

// proto:  void QGraphicsSceneMouseEvent::FreeQGraphicsSceneMouseEvent();
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_FreeQGraphicsSceneMouseEvent<()> for () {
  fn FreeQGraphicsSceneMouseEvent(self , rsthis: &mut QGraphicsSceneMouseEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneMouseEventD0Ev()};
     unsafe {_ZN24QGraphicsSceneMouseEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QPointF QGraphicsSceneMouseEvent::pos();
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn pos<RetType, T: QGraphicsSceneMouseEvent_pos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_pos<RetType> {
  fn pos(self , rsthis: &mut QGraphicsSceneMouseEvent) -> RetType;
}

// proto:  QPointF QGraphicsSceneMouseEvent::pos();
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_pos<QPointF> for () {
  fn pos(self , rsthis: &mut QGraphicsSceneMouseEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneMouseEvent3posEv()};
    let mut ret = unsafe {_ZNK24QGraphicsSceneMouseEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QGraphicsSceneMouseEvent::setLastPos(const QPointF & pos);
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn setLastPos<RetType, T: QGraphicsSceneMouseEvent_setLastPos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setLastPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_setLastPos<RetType> {
  fn setLastPos(self , rsthis: &mut QGraphicsSceneMouseEvent) -> RetType;
}

// proto:  void QGraphicsSceneMouseEvent::setLastPos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_setLastPos<()> for (&'a  QPointF) {
  fn setLastPos(self , rsthis: &mut QGraphicsSceneMouseEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneMouseEvent10setLastPosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QGraphicsSceneMouseEvent10setLastPosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QGraphicsSceneMouseEvent::setLastScenePos(const QPointF & pos);
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn setLastScenePos<RetType, T: QGraphicsSceneMouseEvent_setLastScenePos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setLastScenePos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_setLastScenePos<RetType> {
  fn setLastScenePos(self , rsthis: &mut QGraphicsSceneMouseEvent) -> RetType;
}

// proto:  void QGraphicsSceneMouseEvent::setLastScenePos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_setLastScenePos<()> for (&'a  QPointF) {
  fn setLastScenePos(self , rsthis: &mut QGraphicsSceneMouseEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneMouseEvent15setLastScenePosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QGraphicsSceneMouseEvent15setLastScenePosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QPoint QGraphicsSceneMouseEvent::lastScreenPos();
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn lastScreenPos<RetType, T: QGraphicsSceneMouseEvent_lastScreenPos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.lastScreenPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_lastScreenPos<RetType> {
  fn lastScreenPos(self , rsthis: &mut QGraphicsSceneMouseEvent) -> RetType;
}

// proto:  QPoint QGraphicsSceneMouseEvent::lastScreenPos();
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_lastScreenPos<QPoint> for () {
  fn lastScreenPos(self , rsthis: &mut QGraphicsSceneMouseEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneMouseEvent13lastScreenPosEv()};
    let mut ret = unsafe {_ZNK24QGraphicsSceneMouseEvent13lastScreenPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QGraphicsSceneMouseEvent::setScreenPos(const QPoint & pos);
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn setScreenPos<RetType, T: QGraphicsSceneMouseEvent_setScreenPos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setScreenPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_setScreenPos<RetType> {
  fn setScreenPos(self , rsthis: &mut QGraphicsSceneMouseEvent) -> RetType;
}

// proto:  void QGraphicsSceneMouseEvent::setScreenPos(const QPoint & pos);
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_setScreenPos<()> for (&'a  QPoint) {
  fn setScreenPos(self , rsthis: &mut QGraphicsSceneMouseEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneMouseEvent12setScreenPosERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QGraphicsSceneMouseEvent12setScreenPosERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QGraphicsSceneMouseEvent::setLastScreenPos(const QPoint & pos);
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn setLastScreenPos<RetType, T: QGraphicsSceneMouseEvent_setLastScreenPos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setLastScreenPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_setLastScreenPos<RetType> {
  fn setLastScreenPos(self , rsthis: &mut QGraphicsSceneMouseEvent) -> RetType;
}

// proto:  void QGraphicsSceneMouseEvent::setLastScreenPos(const QPoint & pos);
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_setLastScreenPos<()> for (&'a  QPoint) {
  fn setLastScreenPos(self , rsthis: &mut QGraphicsSceneMouseEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneMouseEvent16setLastScreenPosERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QGraphicsSceneMouseEvent16setLastScreenPosERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QGraphicsSceneMouseEvent::setScenePos(const QPointF & pos);
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn setScenePos<RetType, T: QGraphicsSceneMouseEvent_setScenePos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setScenePos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_setScenePos<RetType> {
  fn setScenePos(self , rsthis: &mut QGraphicsSceneMouseEvent) -> RetType;
}

// proto:  void QGraphicsSceneMouseEvent::setScenePos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_setScenePos<()> for (&'a  QPointF) {
  fn setScenePos(self , rsthis: &mut QGraphicsSceneMouseEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneMouseEvent11setScenePosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QGraphicsSceneMouseEvent11setScenePosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QPointF QGraphicsSceneMouseEvent::lastPos();
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn lastPos<RetType, T: QGraphicsSceneMouseEvent_lastPos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.lastPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_lastPos<RetType> {
  fn lastPos(self , rsthis: &mut QGraphicsSceneMouseEvent) -> RetType;
}

// proto:  QPointF QGraphicsSceneMouseEvent::lastPos();
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_lastPos<QPointF> for () {
  fn lastPos(self , rsthis: &mut QGraphicsSceneMouseEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneMouseEvent7lastPosEv()};
    let mut ret = unsafe {_ZNK24QGraphicsSceneMouseEvent7lastPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QPointF QGraphicsSceneMouseEvent::scenePos();
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn scenePos<RetType, T: QGraphicsSceneMouseEvent_scenePos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.scenePos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_scenePos<RetType> {
  fn scenePos(self , rsthis: &mut QGraphicsSceneMouseEvent) -> RetType;
}

// proto:  QPointF QGraphicsSceneMouseEvent::scenePos();
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_scenePos<QPointF> for () {
  fn scenePos(self , rsthis: &mut QGraphicsSceneMouseEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneMouseEvent8scenePosEv()};
    let mut ret = unsafe {_ZNK24QGraphicsSceneMouseEvent8scenePosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QGraphicsSceneMouseEvent::setPos(const QPointF & pos);
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn setPos<RetType, T: QGraphicsSceneMouseEvent_setPos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_setPos<RetType> {
  fn setPos(self , rsthis: &mut QGraphicsSceneMouseEvent) -> RetType;
}

// proto:  void QGraphicsSceneMouseEvent::setPos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_setPos<()> for (&'a  QPointF) {
  fn setPos(self , rsthis: &mut QGraphicsSceneMouseEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneMouseEvent6setPosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QGraphicsSceneMouseEvent6setPosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

