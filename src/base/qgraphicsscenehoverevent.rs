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
  // proto: QPointF QGraphicsSceneHoverEvent::scenePos();
  fn _ZNK24QGraphicsSceneHoverEvent8scenePosEv() -> i32;
  // proto: void QGraphicsSceneHoverEvent::setLastPos(const QPointF & pos);
  fn _ZN24QGraphicsSceneHoverEvent10setLastPosERK7QPointF(arg0: *const c_void) -> i32;
  // proto: QPointF QGraphicsSceneHoverEvent::lastPos();
  fn _ZNK24QGraphicsSceneHoverEvent7lastPosEv() -> i32;
  // proto: void QGraphicsSceneHoverEvent::NewQGraphicsSceneHoverEvent(const QGraphicsSceneHoverEvent & );
  fn _ZN24QGraphicsSceneHoverEventC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: QPointF QGraphicsSceneHoverEvent::lastScenePos();
  fn _ZNK24QGraphicsSceneHoverEvent12lastScenePosEv() -> i32;
  // proto: void QGraphicsSceneHoverEvent::setLastScreenPos(const QPoint & pos);
  fn _ZN24QGraphicsSceneHoverEvent16setLastScreenPosERK6QPoint(arg0: *const c_void) -> i32;
  // proto: void QGraphicsSceneHoverEvent::setScenePos(const QPointF & pos);
  fn _ZN24QGraphicsSceneHoverEvent11setScenePosERK7QPointF(arg0: *const c_void) -> i32;
  // proto: void QGraphicsSceneHoverEvent::setPos(const QPointF & pos);
  fn _ZN24QGraphicsSceneHoverEvent6setPosERK7QPointF(arg0: *const c_void) -> i32;
  // proto: QPoint QGraphicsSceneHoverEvent::screenPos();
  fn _ZNK24QGraphicsSceneHoverEvent9screenPosEv() -> i32;
  // proto: QPoint QGraphicsSceneHoverEvent::lastScreenPos();
  fn _ZNK24QGraphicsSceneHoverEvent13lastScreenPosEv() -> i32;
  // proto: void QGraphicsSceneHoverEvent::setLastScenePos(const QPointF & pos);
  fn _ZN24QGraphicsSceneHoverEvent15setLastScenePosERK7QPointF(arg0: *const c_void) -> i32;
  // proto: QPointF QGraphicsSceneHoverEvent::pos();
  fn _ZNK24QGraphicsSceneHoverEvent3posEv() -> i32;
  // proto: void QGraphicsSceneHoverEvent::setScreenPos(const QPoint & pos);
  fn _ZN24QGraphicsSceneHoverEvent12setScreenPosERK6QPoint(arg0: *const c_void) -> i32;
  // proto: void QGraphicsSceneHoverEvent::FreeQGraphicsSceneHoverEvent();
  fn _ZN24QGraphicsSceneHoverEventD0Ev() -> i32;
}

// body block begin
// class sizeof(QGraphicsSceneHoverEvent)=1
pub struct QGraphicsSceneHoverEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn scenePos<T: QGraphicsSceneHoverEvent_scenePos>(&mut self, value: T) -> i32 {
    value.scenePos(self);
    return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_scenePos {
  fn scenePos(self, this: &mut QGraphicsSceneHoverEvent) -> i32;
}

// proto: QPointF QGraphicsSceneHoverEvent::scenePos();
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_scenePos for () {
  fn scenePos(self, this: &mut QGraphicsSceneHoverEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneHoverEvent8scenePosEv()};
    unsafe {_ZNK24QGraphicsSceneHoverEvent8scenePosEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn setLastPos<T: QGraphicsSceneHoverEvent_setLastPos>(&mut self, value: T) -> i32 {
    value.setLastPos(self);
    return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_setLastPos {
  fn setLastPos(self, this: &mut QGraphicsSceneHoverEvent) -> i32;
}

// proto: void QGraphicsSceneHoverEvent::setLastPos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_setLastPos for (&'a  QPointF) {
  fn setLastPos(self, this: &mut QGraphicsSceneHoverEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneHoverEvent10setLastPosERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN24QGraphicsSceneHoverEvent10setLastPosERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn lastPos<T: QGraphicsSceneHoverEvent_lastPos>(&mut self, value: T) -> i32 {
    value.lastPos(self);
    return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_lastPos {
  fn lastPos(self, this: &mut QGraphicsSceneHoverEvent) -> i32;
}

// proto: QPointF QGraphicsSceneHoverEvent::lastPos();
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_lastPos for () {
  fn lastPos(self, this: &mut QGraphicsSceneHoverEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneHoverEvent7lastPosEv()};
    unsafe {_ZNK24QGraphicsSceneHoverEvent7lastPosEv()};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN24QGraphicsSceneHoverEventC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsSceneHoverEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn lastScenePos<T: QGraphicsSceneHoverEvent_lastScenePos>(&mut self, value: T) -> i32 {
    value.lastScenePos(self);
    return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_lastScenePos {
  fn lastScenePos(self, this: &mut QGraphicsSceneHoverEvent) -> i32;
}

// proto: QPointF QGraphicsSceneHoverEvent::lastScenePos();
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_lastScenePos for () {
  fn lastScenePos(self, this: &mut QGraphicsSceneHoverEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneHoverEvent12lastScenePosEv()};
    unsafe {_ZNK24QGraphicsSceneHoverEvent12lastScenePosEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn setLastScreenPos<T: QGraphicsSceneHoverEvent_setLastScreenPos>(&mut self, value: T) -> i32 {
    value.setLastScreenPos(self);
    return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_setLastScreenPos {
  fn setLastScreenPos(self, this: &mut QGraphicsSceneHoverEvent) -> i32;
}

// proto: void QGraphicsSceneHoverEvent::setLastScreenPos(const QPoint & pos);
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_setLastScreenPos for (&'a  QPoint) {
  fn setLastScreenPos(self, this: &mut QGraphicsSceneHoverEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneHoverEvent16setLastScreenPosERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN24QGraphicsSceneHoverEvent16setLastScreenPosERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn setScenePos<T: QGraphicsSceneHoverEvent_setScenePos>(&mut self, value: T) -> i32 {
    value.setScenePos(self);
    return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_setScenePos {
  fn setScenePos(self, this: &mut QGraphicsSceneHoverEvent) -> i32;
}

// proto: void QGraphicsSceneHoverEvent::setScenePos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_setScenePos for (&'a  QPointF) {
  fn setScenePos(self, this: &mut QGraphicsSceneHoverEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneHoverEvent11setScenePosERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN24QGraphicsSceneHoverEvent11setScenePosERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn setPos<T: QGraphicsSceneHoverEvent_setPos>(&mut self, value: T) -> i32 {
    value.setPos(self);
    return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_setPos {
  fn setPos(self, this: &mut QGraphicsSceneHoverEvent) -> i32;
}

// proto: void QGraphicsSceneHoverEvent::setPos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_setPos for (&'a  QPointF) {
  fn setPos(self, this: &mut QGraphicsSceneHoverEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneHoverEvent6setPosERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN24QGraphicsSceneHoverEvent6setPosERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn screenPos<T: QGraphicsSceneHoverEvent_screenPos>(&mut self, value: T) -> i32 {
    value.screenPos(self);
    return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_screenPos {
  fn screenPos(self, this: &mut QGraphicsSceneHoverEvent) -> i32;
}

// proto: QPoint QGraphicsSceneHoverEvent::screenPos();
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_screenPos for () {
  fn screenPos(self, this: &mut QGraphicsSceneHoverEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneHoverEvent9screenPosEv()};
    unsafe {_ZNK24QGraphicsSceneHoverEvent9screenPosEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn lastScreenPos<T: QGraphicsSceneHoverEvent_lastScreenPos>(&mut self, value: T) -> i32 {
    value.lastScreenPos(self);
    return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_lastScreenPos {
  fn lastScreenPos(self, this: &mut QGraphicsSceneHoverEvent) -> i32;
}

// proto: QPoint QGraphicsSceneHoverEvent::lastScreenPos();
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_lastScreenPos for () {
  fn lastScreenPos(self, this: &mut QGraphicsSceneHoverEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneHoverEvent13lastScreenPosEv()};
    unsafe {_ZNK24QGraphicsSceneHoverEvent13lastScreenPosEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn setLastScenePos<T: QGraphicsSceneHoverEvent_setLastScenePos>(&mut self, value: T) -> i32 {
    value.setLastScenePos(self);
    return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_setLastScenePos {
  fn setLastScenePos(self, this: &mut QGraphicsSceneHoverEvent) -> i32;
}

// proto: void QGraphicsSceneHoverEvent::setLastScenePos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_setLastScenePos for (&'a  QPointF) {
  fn setLastScenePos(self, this: &mut QGraphicsSceneHoverEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneHoverEvent15setLastScenePosERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN24QGraphicsSceneHoverEvent15setLastScenePosERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn pos<T: QGraphicsSceneHoverEvent_pos>(&mut self, value: T) -> i32 {
    value.pos(self);
    return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_pos {
  fn pos(self, this: &mut QGraphicsSceneHoverEvent) -> i32;
}

// proto: QPointF QGraphicsSceneHoverEvent::pos();
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_pos for () {
  fn pos(self, this: &mut QGraphicsSceneHoverEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneHoverEvent3posEv()};
    unsafe {_ZNK24QGraphicsSceneHoverEvent3posEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn setScreenPos<T: QGraphicsSceneHoverEvent_setScreenPos>(&mut self, value: T) -> i32 {
    value.setScreenPos(self);
    return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_setScreenPos {
  fn setScreenPos(self, this: &mut QGraphicsSceneHoverEvent) -> i32;
}

// proto: void QGraphicsSceneHoverEvent::setScreenPos(const QPoint & pos);
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_setScreenPos for (&'a  QPoint) {
  fn setScreenPos(self, this: &mut QGraphicsSceneHoverEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneHoverEvent12setScreenPosERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN24QGraphicsSceneHoverEvent12setScreenPosERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn FreeQGraphicsSceneHoverEvent<T: QGraphicsSceneHoverEvent_FreeQGraphicsSceneHoverEvent>(&mut self, value: T) -> i32 {
    value.FreeQGraphicsSceneHoverEvent(self);
    return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_FreeQGraphicsSceneHoverEvent {
  fn FreeQGraphicsSceneHoverEvent(self, this: &mut QGraphicsSceneHoverEvent) -> i32;
}

// proto: void QGraphicsSceneHoverEvent::FreeQGraphicsSceneHoverEvent();
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_FreeQGraphicsSceneHoverEvent for () {
  fn FreeQGraphicsSceneHoverEvent(self, this: &mut QGraphicsSceneHoverEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneHoverEventD0Ev()};
    unsafe {_ZN24QGraphicsSceneHoverEventD0Ev()};
    return 1;
  }
}

