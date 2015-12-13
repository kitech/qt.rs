// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qkeysequence::QKeySequence;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: const QKeySequence & QShortcutEvent::key();
  fn _ZNK14QShortcutEvent3keyEv() -> i32;
  // proto: void QShortcutEvent::FreeQShortcutEvent();
  fn _ZN14QShortcutEventD0Ev() -> i32;
  // proto: bool QShortcutEvent::isAmbiguous();
  fn _ZNK14QShortcutEvent11isAmbiguousEv() -> i32;
  // proto: void QShortcutEvent::NewQShortcutEvent(const QKeySequence & key, int id, bool ambiguous);
  fn _ZN14QShortcutEventC1ERK12QKeySequenceib(qthis: *mut c_void, arg0: *const c_void, arg1: c_int, arg2: int8_t) -> i32;
  // proto: int QShortcutEvent::shortcutId();
  fn _ZNK14QShortcutEvent10shortcutIdEv() -> i32;
}

// body block begin
// class sizeof(QShortcutEvent)=40
pub struct QShortcutEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QShortcutEvent {
  pub fn key<T: QShortcutEvent_key>(&mut self, value: T) -> i32 {
    value.key(self);
    return 1;
  }
}

pub trait QShortcutEvent_key {
  fn key(self, this: &mut QShortcutEvent) -> i32;
}

// proto: const QKeySequence & QShortcutEvent::key();
impl<'a> /*trait*/ QShortcutEvent_key for () {
  fn key(self, this: &mut QShortcutEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK14QShortcutEvent3keyEv()};
    unsafe {_ZNK14QShortcutEvent3keyEv()};
    return 1;
  }
}

impl /*struct*/ QShortcutEvent {
  pub fn FreeQShortcutEvent<T: QShortcutEvent_FreeQShortcutEvent>(&mut self, value: T) -> i32 {
    value.FreeQShortcutEvent(self);
    return 1;
  }
}

pub trait QShortcutEvent_FreeQShortcutEvent {
  fn FreeQShortcutEvent(self, this: &mut QShortcutEvent) -> i32;
}

// proto: void QShortcutEvent::FreeQShortcutEvent();
impl<'a> /*trait*/ QShortcutEvent_FreeQShortcutEvent for () {
  fn FreeQShortcutEvent(self, this: &mut QShortcutEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN14QShortcutEventD0Ev()};
    unsafe {_ZN14QShortcutEventD0Ev()};
    return 1;
  }
}

impl /*struct*/ QShortcutEvent {
  pub fn isAmbiguous<T: QShortcutEvent_isAmbiguous>(&mut self, value: T) -> i32 {
    value.isAmbiguous(self);
    return 1;
  }
}

pub trait QShortcutEvent_isAmbiguous {
  fn isAmbiguous(self, this: &mut QShortcutEvent) -> i32;
}

// proto: bool QShortcutEvent::isAmbiguous();
impl<'a> /*trait*/ QShortcutEvent_isAmbiguous for () {
  fn isAmbiguous(self, this: &mut QShortcutEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK14QShortcutEvent11isAmbiguousEv()};
    unsafe {_ZNK14QShortcutEvent11isAmbiguousEv()};
    return 1;
  }
}

impl /*struct*/ QShortcutEvent {
  pub fn NewQShortcutEvent<T: QShortcutEvent_NewQShortcutEvent>(value: T) -> QShortcutEvent {
    let rsthis = value.NewQShortcutEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QShortcutEvent_NewQShortcutEvent {
  fn NewQShortcutEvent(self) -> QShortcutEvent;
}

// proto: void QShortcutEvent::NewQShortcutEvent(const QKeySequence & key, int id, bool ambiguous);
impl<'a> /*trait*/ QShortcutEvent_NewQShortcutEvent for (&'a  QKeySequence, i32, i8) {
  fn NewQShortcutEvent(self) -> QShortcutEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN14QShortcutEventC1ERK12QKeySequenceib()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as int8_t;
    unsafe {_ZN14QShortcutEventC1ERK12QKeySequenceib(qthis, arg0, arg1, arg2)};
    let rsthis = QShortcutEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QShortcutEvent {
  pub fn shortcutId<T: QShortcutEvent_shortcutId>(&mut self, value: T) -> i32 {
    value.shortcutId(self);
    return 1;
  }
}

pub trait QShortcutEvent_shortcutId {
  fn shortcutId(self, this: &mut QShortcutEvent) -> i32;
}

// proto: int QShortcutEvent::shortcutId();
impl<'a> /*trait*/ QShortcutEvent_shortcutId for () {
  fn shortcutId(self, this: &mut QShortcutEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK14QShortcutEvent10shortcutIdEv()};
    unsafe {_ZNK14QShortcutEvent10shortcutIdEv()};
    return 1;
  }
}

