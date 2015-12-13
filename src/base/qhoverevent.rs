// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QHoverEvent::FreeQHoverEvent();
  fn _ZN11QHoverEventD0Ev() -> i32;
  // proto: const QPointF & QHoverEvent::posF();
  fn _ZNK11QHoverEvent4posFEv() -> i32;
  // proto: QPoint QHoverEvent::oldPos();
  fn _ZNK11QHoverEvent6oldPosEv() -> i32;
  // proto: const QPointF & QHoverEvent::oldPosF();
  fn _ZNK11QHoverEvent7oldPosFEv() -> i32;
  // proto: QPoint QHoverEvent::pos();
  fn _ZNK11QHoverEvent3posEv() -> i32;
}

// body block begin
// class sizeof(QHoverEvent)=1
pub struct QHoverEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QHoverEvent {
  pub fn FreeQHoverEvent<T: QHoverEvent_FreeQHoverEvent>(&mut self, value: T) -> i32 {
    value.FreeQHoverEvent(self);
    return 1;
  }
}

pub trait QHoverEvent_FreeQHoverEvent {
  fn FreeQHoverEvent(self, this: &mut QHoverEvent) -> i32;
}

// proto: void QHoverEvent::FreeQHoverEvent();
impl<'a> /*trait*/ QHoverEvent_FreeQHoverEvent for () {
  fn FreeQHoverEvent(self, this: &mut QHoverEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHoverEventD0Ev()};
    unsafe {_ZN11QHoverEventD0Ev()};
    return 1;
  }
}

impl /*struct*/ QHoverEvent {
  pub fn posF<T: QHoverEvent_posF>(&mut self, value: T) -> i32 {
    value.posF(self);
    return 1;
  }
}

pub trait QHoverEvent_posF {
  fn posF(self, this: &mut QHoverEvent) -> i32;
}

// proto: const QPointF & QHoverEvent::posF();
impl<'a> /*trait*/ QHoverEvent_posF for () {
  fn posF(self, this: &mut QHoverEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHoverEvent4posFEv()};
    unsafe {_ZNK11QHoverEvent4posFEv()};
    return 1;
  }
}

impl /*struct*/ QHoverEvent {
  pub fn oldPos<T: QHoverEvent_oldPos>(&mut self, value: T) -> i32 {
    value.oldPos(self);
    return 1;
  }
}

pub trait QHoverEvent_oldPos {
  fn oldPos(self, this: &mut QHoverEvent) -> i32;
}

// proto: QPoint QHoverEvent::oldPos();
impl<'a> /*trait*/ QHoverEvent_oldPos for () {
  fn oldPos(self, this: &mut QHoverEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHoverEvent6oldPosEv()};
    unsafe {_ZNK11QHoverEvent6oldPosEv()};
    return 1;
  }
}

impl /*struct*/ QHoverEvent {
  pub fn oldPosF<T: QHoverEvent_oldPosF>(&mut self, value: T) -> i32 {
    value.oldPosF(self);
    return 1;
  }
}

pub trait QHoverEvent_oldPosF {
  fn oldPosF(self, this: &mut QHoverEvent) -> i32;
}

// proto: const QPointF & QHoverEvent::oldPosF();
impl<'a> /*trait*/ QHoverEvent_oldPosF for () {
  fn oldPosF(self, this: &mut QHoverEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHoverEvent7oldPosFEv()};
    unsafe {_ZNK11QHoverEvent7oldPosFEv()};
    return 1;
  }
}

impl /*struct*/ QHoverEvent {
  pub fn pos<T: QHoverEvent_pos>(&mut self, value: T) -> i32 {
    value.pos(self);
    return 1;
  }
}

pub trait QHoverEvent_pos {
  fn pos(self, this: &mut QHoverEvent) -> i32;
}

// proto: QPoint QHoverEvent::pos();
impl<'a> /*trait*/ QHoverEvent_pos for () {
  fn pos(self, this: &mut QHoverEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHoverEvent3posEv()};
    unsafe {_ZNK11QHoverEvent3posEv()};
    return 1;
  }
}

