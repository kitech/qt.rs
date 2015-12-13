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
  fn _ZN6QEvent11setAcceptedEb(arg0: int8_t) -> i32;
  fn _ZN6QEvent6ignoreEv() -> i32;
  fn _ZNK6QEvent10isAcceptedEv() -> i32;
  fn _ZN6QEventD0Ev() -> i32;
  fn _ZN6QEventC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN6QEvent6acceptEv() -> i32;
  fn _ZN6QEvent17registerEventTypeEi(arg0: c_int) -> i32;
  fn _ZNK6QEvent11spontaneousEv() -> i32;
}

// body block begin
// class sizeof(QEvent)=24
pub struct QEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QEvent {
  pub fn setAccepted<T: QEvent_setAccepted>(&mut self, value: T) -> i32 {
    value.setAccepted(self);
    return 1;
  }
}

pub trait QEvent_setAccepted {
  fn setAccepted(self, this: &mut QEvent) -> i32;
}

// proto: void QEvent::setAccepted(bool accepted);
impl<'a> /*trait*/ QEvent_setAccepted for (i8) {
  fn setAccepted(self, this: &mut QEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QEvent11setAcceptedEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN6QEvent11setAcceptedEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QEvent {
  pub fn ignore<T: QEvent_ignore>(&mut self, value: T) -> i32 {
    value.ignore(self);
    return 1;
  }
}

pub trait QEvent_ignore {
  fn ignore(self, this: &mut QEvent) -> i32;
}

// proto: void QEvent::ignore();
impl<'a> /*trait*/ QEvent_ignore for () {
  fn ignore(self, this: &mut QEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QEvent6ignoreEv()};
    unsafe {_ZN6QEvent6ignoreEv()};
    return 1;
  }
}

impl /*struct*/ QEvent {
  pub fn isAccepted<T: QEvent_isAccepted>(&mut self, value: T) -> i32 {
    value.isAccepted(self);
    return 1;
  }
}

pub trait QEvent_isAccepted {
  fn isAccepted(self, this: &mut QEvent) -> i32;
}

// proto: bool QEvent::isAccepted();
impl<'a> /*trait*/ QEvent_isAccepted for () {
  fn isAccepted(self, this: &mut QEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QEvent10isAcceptedEv()};
    unsafe {_ZNK6QEvent10isAcceptedEv()};
    return 1;
  }
}

impl /*struct*/ QEvent {
  pub fn FreeQEvent<T: QEvent_FreeQEvent>(&mut self, value: T) -> i32 {
    value.FreeQEvent(self);
    return 1;
  }
}

pub trait QEvent_FreeQEvent {
  fn FreeQEvent(self, this: &mut QEvent) -> i32;
}

// proto: void QEvent::FreeQEvent();
impl<'a> /*trait*/ QEvent_FreeQEvent for () {
  fn FreeQEvent(self, this: &mut QEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QEventD0Ev()};
    unsafe {_ZN6QEventD0Ev()};
    return 1;
  }
}

impl /*struct*/ QEvent {
  pub fn NewQEvent<T: QEvent_NewQEvent>(value: T) -> QEvent {
    let rsthis = value.NewQEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QEvent_NewQEvent {
  fn NewQEvent(self) -> QEvent;
}

// proto: void QEvent::NewQEvent(const QEvent & other);
impl<'a> /*trait*/ QEvent_NewQEvent for (&'a  QEvent) {
  fn NewQEvent(self) -> QEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QEventC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QEventC1ERKS_(qthis, arg0)};
    let rsthis = QEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QEvent {
  pub fn accept<T: QEvent_accept>(&mut self, value: T) -> i32 {
    value.accept(self);
    return 1;
  }
}

pub trait QEvent_accept {
  fn accept(self, this: &mut QEvent) -> i32;
}

// proto: void QEvent::accept();
impl<'a> /*trait*/ QEvent_accept for () {
  fn accept(self, this: &mut QEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QEvent6acceptEv()};
    unsafe {_ZN6QEvent6acceptEv()};
    return 1;
  }
}

impl /*struct*/ QEvent {
  pub fn registerEventType<T: QEvent_registerEventType>(&mut self, value: T) -> i32 {
    value.registerEventType(self);
    return 1;
  }
}

pub trait QEvent_registerEventType {
  fn registerEventType(self, this: &mut QEvent) -> i32;
}

// proto: int QEvent::registerEventType(int hint);
impl<'a> /*trait*/ QEvent_registerEventType for (i32) {
  fn registerEventType(self, this: &mut QEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QEvent17registerEventTypeEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN6QEvent17registerEventTypeEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QEvent {
  pub fn spontaneous<T: QEvent_spontaneous>(&mut self, value: T) -> i32 {
    value.spontaneous(self);
    return 1;
  }
}

pub trait QEvent_spontaneous {
  fn spontaneous(self, this: &mut QEvent) -> i32;
}

// proto: bool QEvent::spontaneous();
impl<'a> /*trait*/ QEvent_spontaneous for () {
  fn spontaneous(self, this: &mut QEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QEvent11spontaneousEv()};
    unsafe {_ZNK6QEvent11spontaneousEv()};
    return 1;
  }
}

