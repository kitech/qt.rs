// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpoint::QPoint;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QMoveEvent::FreeQMoveEvent();
  fn _ZN10QMoveEventD0Ev() -> i32;
  // proto: const QPoint & QMoveEvent::oldPos();
  fn _ZNK10QMoveEvent6oldPosEv() -> i32;
  // proto: void QMoveEvent::NewQMoveEvent(const QPoint & pos, const QPoint & oldPos);
  fn _ZN10QMoveEventC1ERK6QPointS2_(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: const QPoint & QMoveEvent::pos();
  fn _ZNK10QMoveEvent3posEv() -> i32;
}

// body block begin
// class sizeof(QMoveEvent)=40
pub struct QMoveEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMoveEvent {
  pub fn FreeQMoveEvent<T: QMoveEvent_FreeQMoveEvent>(&mut self, value: T) -> i32 {
    value.FreeQMoveEvent(self);
    return 1;
  }
}

pub trait QMoveEvent_FreeQMoveEvent {
  fn FreeQMoveEvent(self, this: &mut QMoveEvent) -> i32;
}

// proto: void QMoveEvent::FreeQMoveEvent();
impl<'a> /*trait*/ QMoveEvent_FreeQMoveEvent for () {
  fn FreeQMoveEvent(self, this: &mut QMoveEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN10QMoveEventD0Ev()};
    unsafe {_ZN10QMoveEventD0Ev()};
    return 1;
  }
}

impl /*struct*/ QMoveEvent {
  pub fn oldPos<T: QMoveEvent_oldPos>(&mut self, value: T) -> i32 {
    value.oldPos(self);
    return 1;
  }
}

pub trait QMoveEvent_oldPos {
  fn oldPos(self, this: &mut QMoveEvent) -> i32;
}

// proto: const QPoint & QMoveEvent::oldPos();
impl<'a> /*trait*/ QMoveEvent_oldPos for () {
  fn oldPos(self, this: &mut QMoveEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK10QMoveEvent6oldPosEv()};
    unsafe {_ZNK10QMoveEvent6oldPosEv()};
    return 1;
  }
}

impl /*struct*/ QMoveEvent {
  pub fn NewQMoveEvent<T: QMoveEvent_NewQMoveEvent>(value: T) -> QMoveEvent {
    let rsthis = value.NewQMoveEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QMoveEvent_NewQMoveEvent {
  fn NewQMoveEvent(self) -> QMoveEvent;
}

// proto: void QMoveEvent::NewQMoveEvent(const QPoint & pos, const QPoint & oldPos);
impl<'a> /*trait*/ QMoveEvent_NewQMoveEvent for (&'a  QPoint, &'a  QPoint) {
  fn NewQMoveEvent(self) -> QMoveEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN10QMoveEventC1ERK6QPointS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN10QMoveEventC1ERK6QPointS2_(qthis, arg0, arg1)};
    let rsthis = QMoveEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMoveEvent {
  pub fn pos<T: QMoveEvent_pos>(&mut self, value: T) -> i32 {
    value.pos(self);
    return 1;
  }
}

pub trait QMoveEvent_pos {
  fn pos(self, this: &mut QMoveEvent) -> i32;
}

// proto: const QPoint & QMoveEvent::pos();
impl<'a> /*trait*/ QMoveEvent_pos for () {
  fn pos(self, this: &mut QMoveEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK10QMoveEvent3posEv()};
    unsafe {_ZNK10QMoveEvent3posEv()};
    return 1;
  }
}

