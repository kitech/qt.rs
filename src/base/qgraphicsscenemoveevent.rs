// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpointf::QPointF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QPointF QGraphicsSceneMoveEvent::newPos();
  fn _ZNK23QGraphicsSceneMoveEvent6newPosEv() -> i32;
  // proto: QPointF QGraphicsSceneMoveEvent::oldPos();
  fn _ZNK23QGraphicsSceneMoveEvent6oldPosEv() -> i32;
  // proto: void QGraphicsSceneMoveEvent::FreeQGraphicsSceneMoveEvent();
  fn _ZN23QGraphicsSceneMoveEventD0Ev() -> i32;
  // proto: void QGraphicsSceneMoveEvent::NewQGraphicsSceneMoveEvent(const QGraphicsSceneMoveEvent & );
  fn _ZN23QGraphicsSceneMoveEventC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QGraphicsSceneMoveEvent::setNewPos(const QPointF & pos);
  fn _ZN23QGraphicsSceneMoveEvent9setNewPosERK7QPointF(arg0: *const c_void) -> i32;
  // proto: void QGraphicsSceneMoveEvent::NewQGraphicsSceneMoveEvent();
  fn _ZN23QGraphicsSceneMoveEventC1Ev(qthis: *mut c_void) -> i32;
  // proto: void QGraphicsSceneMoveEvent::setOldPos(const QPointF & pos);
  fn _ZN23QGraphicsSceneMoveEvent9setOldPosERK7QPointF(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QGraphicsSceneMoveEvent)=1
pub struct QGraphicsSceneMoveEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsSceneMoveEvent {
  pub fn newPos<T: QGraphicsSceneMoveEvent_newPos>(&mut self, value: T) -> i32 {
    value.newPos(self);
    return 1;
  }
}

pub trait QGraphicsSceneMoveEvent_newPos {
  fn newPos(self, this: &mut QGraphicsSceneMoveEvent) -> i32;
}

// proto: QPointF QGraphicsSceneMoveEvent::newPos();
impl<'a> /*trait*/ QGraphicsSceneMoveEvent_newPos for () {
  fn newPos(self, this: &mut QGraphicsSceneMoveEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsSceneMoveEvent6newPosEv()};
    unsafe {_ZNK23QGraphicsSceneMoveEvent6newPosEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsSceneMoveEvent {
  pub fn oldPos<T: QGraphicsSceneMoveEvent_oldPos>(&mut self, value: T) -> i32 {
    value.oldPos(self);
    return 1;
  }
}

pub trait QGraphicsSceneMoveEvent_oldPos {
  fn oldPos(self, this: &mut QGraphicsSceneMoveEvent) -> i32;
}

// proto: QPointF QGraphicsSceneMoveEvent::oldPos();
impl<'a> /*trait*/ QGraphicsSceneMoveEvent_oldPos for () {
  fn oldPos(self, this: &mut QGraphicsSceneMoveEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsSceneMoveEvent6oldPosEv()};
    unsafe {_ZNK23QGraphicsSceneMoveEvent6oldPosEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsSceneMoveEvent {
  pub fn FreeQGraphicsSceneMoveEvent<T: QGraphicsSceneMoveEvent_FreeQGraphicsSceneMoveEvent>(&mut self, value: T) -> i32 {
    value.FreeQGraphicsSceneMoveEvent(self);
    return 1;
  }
}

pub trait QGraphicsSceneMoveEvent_FreeQGraphicsSceneMoveEvent {
  fn FreeQGraphicsSceneMoveEvent(self, this: &mut QGraphicsSceneMoveEvent) -> i32;
}

// proto: void QGraphicsSceneMoveEvent::FreeQGraphicsSceneMoveEvent();
impl<'a> /*trait*/ QGraphicsSceneMoveEvent_FreeQGraphicsSceneMoveEvent for () {
  fn FreeQGraphicsSceneMoveEvent(self, this: &mut QGraphicsSceneMoveEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSceneMoveEventD0Ev()};
    unsafe {_ZN23QGraphicsSceneMoveEventD0Ev()};
    return 1;
  }
}

impl /*struct*/ QGraphicsSceneMoveEvent {
  pub fn NewQGraphicsSceneMoveEvent<T: QGraphicsSceneMoveEvent_NewQGraphicsSceneMoveEvent>(value: T) -> QGraphicsSceneMoveEvent {
    let rsthis = value.NewQGraphicsSceneMoveEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsSceneMoveEvent_NewQGraphicsSceneMoveEvent {
  fn NewQGraphicsSceneMoveEvent(self) -> QGraphicsSceneMoveEvent;
}

// proto: void QGraphicsSceneMoveEvent::NewQGraphicsSceneMoveEvent(const QGraphicsSceneMoveEvent & );
impl<'a> /*trait*/ QGraphicsSceneMoveEvent_NewQGraphicsSceneMoveEvent for (&'a  QGraphicsSceneMoveEvent) {
  fn NewQGraphicsSceneMoveEvent(self) -> QGraphicsSceneMoveEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSceneMoveEventC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN23QGraphicsSceneMoveEventC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsSceneMoveEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneMoveEvent {
  pub fn setNewPos<T: QGraphicsSceneMoveEvent_setNewPos>(&mut self, value: T) -> i32 {
    value.setNewPos(self);
    return 1;
  }
}

pub trait QGraphicsSceneMoveEvent_setNewPos {
  fn setNewPos(self, this: &mut QGraphicsSceneMoveEvent) -> i32;
}

// proto: void QGraphicsSceneMoveEvent::setNewPos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneMoveEvent_setNewPos for (&'a  QPointF) {
  fn setNewPos(self, this: &mut QGraphicsSceneMoveEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSceneMoveEvent9setNewPosERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN23QGraphicsSceneMoveEvent9setNewPosERK7QPointF(arg0)};
    return 1;
  }
}

// proto: void QGraphicsSceneMoveEvent::NewQGraphicsSceneMoveEvent();
impl<'a> /*trait*/ QGraphicsSceneMoveEvent_NewQGraphicsSceneMoveEvent for () {
  fn NewQGraphicsSceneMoveEvent(self) -> QGraphicsSceneMoveEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSceneMoveEventC1Ev()};
    unsafe {_ZN23QGraphicsSceneMoveEventC1Ev(qthis)};
    let rsthis = QGraphicsSceneMoveEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneMoveEvent {
  pub fn setOldPos<T: QGraphicsSceneMoveEvent_setOldPos>(&mut self, value: T) -> i32 {
    value.setOldPos(self);
    return 1;
  }
}

pub trait QGraphicsSceneMoveEvent_setOldPos {
  fn setOldPos(self, this: &mut QGraphicsSceneMoveEvent) -> i32;
}

// proto: void QGraphicsSceneMoveEvent::setOldPos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneMoveEvent_setOldPos for (&'a  QPointF) {
  fn setOldPos(self, this: &mut QGraphicsSceneMoveEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSceneMoveEvent9setOldPosERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN23QGraphicsSceneMoveEvent9setOldPosERK7QPointF(arg0)};
    return 1;
  }
}

