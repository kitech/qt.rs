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
  // proto:  QPointF QGraphicsSceneHoverEvent::scenePos();
  fn _ZNK24QGraphicsSceneHoverEvent8scenePosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsSceneHoverEvent::setLastPos(const QPointF & pos);
  fn _ZN24QGraphicsSceneHoverEvent10setLastPosERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QPointF QGraphicsSceneHoverEvent::lastPos();
  fn _ZNK24QGraphicsSceneHoverEvent7lastPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsSceneHoverEvent::NewQGraphicsSceneHoverEvent(const QGraphicsSceneHoverEvent & );
  fn _ZN24QGraphicsSceneHoverEventC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QPointF QGraphicsSceneHoverEvent::lastScenePos();
  fn _ZNK24QGraphicsSceneHoverEvent12lastScenePosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsSceneHoverEvent::setLastScreenPos(const QPoint & pos);
  fn _ZN24QGraphicsSceneHoverEvent16setLastScreenPosERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsSceneHoverEvent::setScenePos(const QPointF & pos);
  fn _ZN24QGraphicsSceneHoverEvent11setScenePosERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsSceneHoverEvent::setPos(const QPointF & pos);
  fn _ZN24QGraphicsSceneHoverEvent6setPosERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QPoint QGraphicsSceneHoverEvent::screenPos();
  fn _ZNK24QGraphicsSceneHoverEvent9screenPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPoint QGraphicsSceneHoverEvent::lastScreenPos();
  fn _ZNK24QGraphicsSceneHoverEvent13lastScreenPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsSceneHoverEvent::setLastScenePos(const QPointF & pos);
  fn _ZN24QGraphicsSceneHoverEvent15setLastScenePosERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QPointF QGraphicsSceneHoverEvent::pos();
  fn _ZNK24QGraphicsSceneHoverEvent3posEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsSceneHoverEvent::setScreenPos(const QPoint & pos);
  fn _ZN24QGraphicsSceneHoverEvent12setScreenPosERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsSceneHoverEvent::FreeQGraphicsSceneHoverEvent();
  fn _ZN24QGraphicsSceneHoverEventD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QGraphicsSceneHoverEvent)=1
pub struct QGraphicsSceneHoverEvent {
  pub qclsinst: *mut c_void,
}

// proto:  QPointF QGraphicsSceneHoverEvent::scenePos();
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn scenePos<RetType, T: QGraphicsSceneHoverEvent_scenePos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.scenePos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_scenePos<RetType> {
  fn scenePos(self , rsthis: &mut QGraphicsSceneHoverEvent) -> RetType;
}

// proto:  QPointF QGraphicsSceneHoverEvent::scenePos();
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_scenePos<QPointF> for () {
  fn scenePos(self , rsthis: &mut QGraphicsSceneHoverEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneHoverEvent8scenePosEv()};
    let mut ret = unsafe {_ZNK24QGraphicsSceneHoverEvent8scenePosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QGraphicsSceneHoverEvent::setLastPos(const QPointF & pos);
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn setLastPos<RetType, T: QGraphicsSceneHoverEvent_setLastPos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setLastPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_setLastPos<RetType> {
  fn setLastPos(self , rsthis: &mut QGraphicsSceneHoverEvent) -> RetType;
}

// proto:  void QGraphicsSceneHoverEvent::setLastPos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_setLastPos<()> for (&'a  QPointF) {
  fn setLastPos(self , rsthis: &mut QGraphicsSceneHoverEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneHoverEvent10setLastPosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QGraphicsSceneHoverEvent10setLastPosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QPointF QGraphicsSceneHoverEvent::lastPos();
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn lastPos<RetType, T: QGraphicsSceneHoverEvent_lastPos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.lastPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_lastPos<RetType> {
  fn lastPos(self , rsthis: &mut QGraphicsSceneHoverEvent) -> RetType;
}

// proto:  QPointF QGraphicsSceneHoverEvent::lastPos();
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_lastPos<QPointF> for () {
  fn lastPos(self , rsthis: &mut QGraphicsSceneHoverEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneHoverEvent7lastPosEv()};
    let mut ret = unsafe {_ZNK24QGraphicsSceneHoverEvent7lastPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn NewQGraphicsSceneHoverEvent<T: QGraphicsSceneHoverEvent_NewQGraphicsSceneHoverEvent>(value: T) -> QGraphicsSceneHoverEvent {
    let rsthis = value.NewQGraphicsSceneHoverEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_NewQGraphicsSceneHoverEvent {
  fn NewQGraphicsSceneHoverEvent(self) -> QGraphicsSceneHoverEvent;
}

// proto: void QGraphicsSceneHoverEvent::NewQGraphicsSceneHoverEvent(const QGraphicsSceneHoverEvent & );
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_NewQGraphicsSceneHoverEvent for (&'a  QGraphicsSceneHoverEvent) {
  fn NewQGraphicsSceneHoverEvent(self) -> QGraphicsSceneHoverEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneHoverEventC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN24QGraphicsSceneHoverEventC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsSceneHoverEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  QPointF QGraphicsSceneHoverEvent::lastScenePos();
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn lastScenePos<RetType, T: QGraphicsSceneHoverEvent_lastScenePos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.lastScenePos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_lastScenePos<RetType> {
  fn lastScenePos(self , rsthis: &mut QGraphicsSceneHoverEvent) -> RetType;
}

// proto:  QPointF QGraphicsSceneHoverEvent::lastScenePos();
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_lastScenePos<QPointF> for () {
  fn lastScenePos(self , rsthis: &mut QGraphicsSceneHoverEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneHoverEvent12lastScenePosEv()};
    let mut ret = unsafe {_ZNK24QGraphicsSceneHoverEvent12lastScenePosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QGraphicsSceneHoverEvent::setLastScreenPos(const QPoint & pos);
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn setLastScreenPos<RetType, T: QGraphicsSceneHoverEvent_setLastScreenPos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setLastScreenPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_setLastScreenPos<RetType> {
  fn setLastScreenPos(self , rsthis: &mut QGraphicsSceneHoverEvent) -> RetType;
}

// proto:  void QGraphicsSceneHoverEvent::setLastScreenPos(const QPoint & pos);
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_setLastScreenPos<()> for (&'a  QPoint) {
  fn setLastScreenPos(self , rsthis: &mut QGraphicsSceneHoverEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneHoverEvent16setLastScreenPosERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QGraphicsSceneHoverEvent16setLastScreenPosERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QGraphicsSceneHoverEvent::setScenePos(const QPointF & pos);
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn setScenePos<RetType, T: QGraphicsSceneHoverEvent_setScenePos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setScenePos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_setScenePos<RetType> {
  fn setScenePos(self , rsthis: &mut QGraphicsSceneHoverEvent) -> RetType;
}

// proto:  void QGraphicsSceneHoverEvent::setScenePos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_setScenePos<()> for (&'a  QPointF) {
  fn setScenePos(self , rsthis: &mut QGraphicsSceneHoverEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneHoverEvent11setScenePosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QGraphicsSceneHoverEvent11setScenePosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QGraphicsSceneHoverEvent::setPos(const QPointF & pos);
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn setPos<RetType, T: QGraphicsSceneHoverEvent_setPos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_setPos<RetType> {
  fn setPos(self , rsthis: &mut QGraphicsSceneHoverEvent) -> RetType;
}

// proto:  void QGraphicsSceneHoverEvent::setPos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_setPos<()> for (&'a  QPointF) {
  fn setPos(self , rsthis: &mut QGraphicsSceneHoverEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneHoverEvent6setPosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QGraphicsSceneHoverEvent6setPosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QPoint QGraphicsSceneHoverEvent::screenPos();
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn screenPos<RetType, T: QGraphicsSceneHoverEvent_screenPos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.screenPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_screenPos<RetType> {
  fn screenPos(self , rsthis: &mut QGraphicsSceneHoverEvent) -> RetType;
}

// proto:  QPoint QGraphicsSceneHoverEvent::screenPos();
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_screenPos<QPoint> for () {
  fn screenPos(self , rsthis: &mut QGraphicsSceneHoverEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneHoverEvent9screenPosEv()};
    let mut ret = unsafe {_ZNK24QGraphicsSceneHoverEvent9screenPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QPoint QGraphicsSceneHoverEvent::lastScreenPos();
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn lastScreenPos<RetType, T: QGraphicsSceneHoverEvent_lastScreenPos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.lastScreenPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_lastScreenPos<RetType> {
  fn lastScreenPos(self , rsthis: &mut QGraphicsSceneHoverEvent) -> RetType;
}

// proto:  QPoint QGraphicsSceneHoverEvent::lastScreenPos();
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_lastScreenPos<QPoint> for () {
  fn lastScreenPos(self , rsthis: &mut QGraphicsSceneHoverEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneHoverEvent13lastScreenPosEv()};
    let mut ret = unsafe {_ZNK24QGraphicsSceneHoverEvent13lastScreenPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QGraphicsSceneHoverEvent::setLastScenePos(const QPointF & pos);
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn setLastScenePos<RetType, T: QGraphicsSceneHoverEvent_setLastScenePos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setLastScenePos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_setLastScenePos<RetType> {
  fn setLastScenePos(self , rsthis: &mut QGraphicsSceneHoverEvent) -> RetType;
}

// proto:  void QGraphicsSceneHoverEvent::setLastScenePos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_setLastScenePos<()> for (&'a  QPointF) {
  fn setLastScenePos(self , rsthis: &mut QGraphicsSceneHoverEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneHoverEvent15setLastScenePosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QGraphicsSceneHoverEvent15setLastScenePosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QPointF QGraphicsSceneHoverEvent::pos();
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn pos<RetType, T: QGraphicsSceneHoverEvent_pos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_pos<RetType> {
  fn pos(self , rsthis: &mut QGraphicsSceneHoverEvent) -> RetType;
}

// proto:  QPointF QGraphicsSceneHoverEvent::pos();
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_pos<QPointF> for () {
  fn pos(self , rsthis: &mut QGraphicsSceneHoverEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneHoverEvent3posEv()};
    let mut ret = unsafe {_ZNK24QGraphicsSceneHoverEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QGraphicsSceneHoverEvent::setScreenPos(const QPoint & pos);
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn setScreenPos<RetType, T: QGraphicsSceneHoverEvent_setScreenPos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setScreenPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_setScreenPos<RetType> {
  fn setScreenPos(self , rsthis: &mut QGraphicsSceneHoverEvent) -> RetType;
}

// proto:  void QGraphicsSceneHoverEvent::setScreenPos(const QPoint & pos);
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_setScreenPos<()> for (&'a  QPoint) {
  fn setScreenPos(self , rsthis: &mut QGraphicsSceneHoverEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneHoverEvent12setScreenPosERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QGraphicsSceneHoverEvent12setScreenPosERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QGraphicsSceneHoverEvent::FreeQGraphicsSceneHoverEvent();
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn FreeQGraphicsSceneHoverEvent<RetType, T: QGraphicsSceneHoverEvent_FreeQGraphicsSceneHoverEvent<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQGraphicsSceneHoverEvent(self);
    // return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_FreeQGraphicsSceneHoverEvent<RetType> {
  fn FreeQGraphicsSceneHoverEvent(self , rsthis: &mut QGraphicsSceneHoverEvent) -> RetType;
}

// proto:  void QGraphicsSceneHoverEvent::FreeQGraphicsSceneHoverEvent();
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_FreeQGraphicsSceneHoverEvent<()> for () {
  fn FreeQGraphicsSceneHoverEvent(self , rsthis: &mut QGraphicsSceneHoverEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneHoverEventD0Ev()};
     unsafe {_ZN24QGraphicsSceneHoverEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

