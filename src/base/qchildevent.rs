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
  fn _ZNK11QChildEvent5addedEv() -> i32;
  fn _ZNK11QChildEvent8polishedEv() -> i32;
  fn _ZN11QChildEventD0Ev() -> i32;
  fn _ZNK11QChildEvent7removedEv() -> i32;
  fn _ZNK11QChildEvent5childEv() -> i32;
}

// body block begin
// class sizeof(QChildEvent)=32
pub struct QChildEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QChildEvent {
  pub fn added<T: QChildEvent_added>(&mut self, value: T) -> i32 {
    value.added(self);
    return 1;
  }
}

pub trait QChildEvent_added {
  fn added(self, this: &mut QChildEvent) -> i32;
}

// proto: bool QChildEvent::added();
impl<'a> /*trait*/ QChildEvent_added for () {
  fn added(self, this: &mut QChildEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QChildEvent5addedEv()};
    unsafe {_ZNK11QChildEvent5addedEv()};
    return 1;
  }
}

impl /*struct*/ QChildEvent {
  pub fn polished<T: QChildEvent_polished>(&mut self, value: T) -> i32 {
    value.polished(self);
    return 1;
  }
}

pub trait QChildEvent_polished {
  fn polished(self, this: &mut QChildEvent) -> i32;
}

// proto: bool QChildEvent::polished();
impl<'a> /*trait*/ QChildEvent_polished for () {
  fn polished(self, this: &mut QChildEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QChildEvent8polishedEv()};
    unsafe {_ZNK11QChildEvent8polishedEv()};
    return 1;
  }
}

impl /*struct*/ QChildEvent {
  pub fn FreeQChildEvent<T: QChildEvent_FreeQChildEvent>(&mut self, value: T) -> i32 {
    value.FreeQChildEvent(self);
    return 1;
  }
}

pub trait QChildEvent_FreeQChildEvent {
  fn FreeQChildEvent(self, this: &mut QChildEvent) -> i32;
}

// proto: void QChildEvent::FreeQChildEvent();
impl<'a> /*trait*/ QChildEvent_FreeQChildEvent for () {
  fn FreeQChildEvent(self, this: &mut QChildEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QChildEventD0Ev()};
    unsafe {_ZN11QChildEventD0Ev()};
    return 1;
  }
}

impl /*struct*/ QChildEvent {
  pub fn removed<T: QChildEvent_removed>(&mut self, value: T) -> i32 {
    value.removed(self);
    return 1;
  }
}

pub trait QChildEvent_removed {
  fn removed(self, this: &mut QChildEvent) -> i32;
}

// proto: bool QChildEvent::removed();
impl<'a> /*trait*/ QChildEvent_removed for () {
  fn removed(self, this: &mut QChildEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QChildEvent7removedEv()};
    unsafe {_ZNK11QChildEvent7removedEv()};
    return 1;
  }
}

impl /*struct*/ QChildEvent {
  pub fn child<T: QChildEvent_child>(&mut self, value: T) -> i32 {
    value.child(self);
    return 1;
  }
}

pub trait QChildEvent_child {
  fn child(self, this: &mut QChildEvent) -> i32;
}

// proto: QObject * QChildEvent::child();
impl<'a> /*trait*/ QChildEvent_child for () {
  fn child(self, this: &mut QChildEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QChildEvent5childEv()};
    unsafe {_ZNK11QChildEvent5childEv()};
    return 1;
  }
}

