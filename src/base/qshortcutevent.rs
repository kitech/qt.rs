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
  // proto:  const QKeySequence & QShortcutEvent::key();
  fn _ZNK14QShortcutEvent3keyEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QShortcutEvent::FreeQShortcutEvent();
  fn _ZN14QShortcutEventD0Ev(qthis: *mut c_void) ;
  // proto:  bool QShortcutEvent::isAmbiguous();
  fn _ZNK14QShortcutEvent11isAmbiguousEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QShortcutEvent::NewQShortcutEvent(const QKeySequence & key, int id, bool ambiguous);
  fn _ZN14QShortcutEventC1ERK12QKeySequenceib(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: int8_t) ;
  // proto:  int QShortcutEvent::shortcutId();
  fn _ZNK14QShortcutEvent10shortcutIdEv(qthis: *mut c_void) -> c_int;
}

// body block begin
// class sizeof(QShortcutEvent)=40
pub struct QShortcutEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QShortcutEvent {
  pub fn key<T: QShortcutEvent_key>(&mut self, value: T) -> QKeySequence {
    return value.key(self);
    // return 1;
  }
}

pub trait QShortcutEvent_key {
  fn key(self, rsthis: &mut QShortcutEvent) -> QKeySequence;
}

// proto:  const QKeySequence & QShortcutEvent::key();
impl<'a> /*trait*/ QShortcutEvent_key for () {
  fn key(self, rsthis: &mut QShortcutEvent) -> QKeySequence {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK14QShortcutEvent3keyEv()};
    let mut ret = unsafe {_ZNK14QShortcutEvent3keyEv(rsthis.qclsinst)};
    let mut ret1 = QKeySequence{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QShortcutEvent {
  pub fn FreeQShortcutEvent<T: QShortcutEvent_FreeQShortcutEvent>(&mut self, value: T)  {
     value.FreeQShortcutEvent(self);
    // return 1;
  }
}

pub trait QShortcutEvent_FreeQShortcutEvent {
  fn FreeQShortcutEvent(self, rsthis: &mut QShortcutEvent) ;
}

// proto:  void QShortcutEvent::FreeQShortcutEvent();
impl<'a> /*trait*/ QShortcutEvent_FreeQShortcutEvent for () {
  fn FreeQShortcutEvent(self, rsthis: &mut QShortcutEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN14QShortcutEventD0Ev()};
     unsafe {_ZN14QShortcutEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QShortcutEvent {
  pub fn isAmbiguous<T: QShortcutEvent_isAmbiguous>(&mut self, value: T) -> i8 {
    return value.isAmbiguous(self);
    // return 1;
  }
}

pub trait QShortcutEvent_isAmbiguous {
  fn isAmbiguous(self, rsthis: &mut QShortcutEvent) -> i8;
}

// proto:  bool QShortcutEvent::isAmbiguous();
impl<'a> /*trait*/ QShortcutEvent_isAmbiguous for () {
  fn isAmbiguous(self, rsthis: &mut QShortcutEvent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK14QShortcutEvent11isAmbiguousEv()};
    let mut ret = unsafe {_ZNK14QShortcutEvent11isAmbiguousEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
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
    let arg0 = self.0.qclsinst  as *mut c_void;
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
    return value.shortcutId(self);
    // return 1;
  }
}

pub trait QShortcutEvent_shortcutId {
  fn shortcutId(self, rsthis: &mut QShortcutEvent) -> i32;
}

// proto:  int QShortcutEvent::shortcutId();
impl<'a> /*trait*/ QShortcutEvent_shortcutId for () {
  fn shortcutId(self, rsthis: &mut QShortcutEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK14QShortcutEvent10shortcutIdEv()};
    let mut ret = unsafe {_ZNK14QShortcutEvent10shortcutIdEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

