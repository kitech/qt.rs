// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;
use super::qevent::QEvent;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN10QEventLoop4exitEi(arg0: c_int) -> i32;
  fn _ZN10QEventLoop4quitEv() -> i32;
  fn _ZN10QEventLoop4execE6QFlagsINS_17ProcessEventsFlagEE(arg0: c_int) -> i32;
  fn _ZN10QEventLoopC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZN10QEventLoop13processEventsE6QFlagsINS_17ProcessEventsFlagEE(arg0: c_int) -> i32;
  fn _ZNK10QEventLoop9isRunningEv() -> i32;
  fn _ZNK10QEventLoop10metaObjectEv() -> i32;
  fn _ZN10QEventLoop6wakeUpEv() -> i32;
  fn _ZN10QEventLoop13processEventsE6QFlagsINS_17ProcessEventsFlagEEi(arg0: c_int, arg1: c_int) -> i32;
  fn _ZN10QEventLoopD0Ev() -> i32;
  fn _ZN10QEventLoop5eventEP6QEvent(arg0: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QEventLoop)=1
pub struct QEventLoop {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QEventLoop {
  pub fn exit<T: QEventLoop_exit>(&mut self, value: T) -> i32 {
    value.exit(self);
    return 1;
  }
}

pub trait QEventLoop_exit {
  fn exit(self, this: &mut QEventLoop) -> i32;
}

// proto: void QEventLoop::exit(int returnCode);
impl<'a> /*trait*/ QEventLoop_exit for (i32) {
  fn exit(self, this: &mut QEventLoop) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QEventLoop4exitEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QEventLoop4exitEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QEventLoop {
  pub fn quit<T: QEventLoop_quit>(&mut self, value: T) -> i32 {
    value.quit(self);
    return 1;
  }
}

pub trait QEventLoop_quit {
  fn quit(self, this: &mut QEventLoop) -> i32;
}

// proto: void QEventLoop::quit();
impl<'a> /*trait*/ QEventLoop_quit for () {
  fn quit(self, this: &mut QEventLoop) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QEventLoop4quitEv()};
    unsafe {_ZN10QEventLoop4quitEv()};
    return 1;
  }
}

impl /*struct*/ QEventLoop {
  pub fn exec<T: QEventLoop_exec>(&mut self, value: T) -> i32 {
    value.exec(self);
    return 1;
  }
}

pub trait QEventLoop_exec {
  fn exec(self, this: &mut QEventLoop) -> i32;
}

// proto: int QEventLoop::exec(ProcessEventsFlags flags);
impl<'a> /*trait*/ QEventLoop_exec for (i32) {
  fn exec(self, this: &mut QEventLoop) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QEventLoop4execE6QFlagsINS_17ProcessEventsFlagEE()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QEventLoop4execE6QFlagsINS_17ProcessEventsFlagEE(arg0)};
    return 1;
  }
}

impl /*struct*/ QEventLoop {
  pub fn NewQEventLoop<T: QEventLoop_NewQEventLoop>(value: T) -> QEventLoop {
    let rsthis = value.NewQEventLoop();
    return rsthis;
    // return 1;
  }
}

pub trait QEventLoop_NewQEventLoop {
  fn NewQEventLoop(self) -> QEventLoop;
}

// proto: void QEventLoop::NewQEventLoop(QObject * parent);
impl<'a> /*trait*/ QEventLoop_NewQEventLoop for (&'a mut QObject) {
  fn NewQEventLoop(self) -> QEventLoop {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QEventLoopC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QEventLoopC1EP7QObject(qthis, arg0)};
    let rsthis = QEventLoop{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QEventLoop {
  pub fn processEvents<T: QEventLoop_processEvents>(&mut self, value: T) -> i32 {
    value.processEvents(self);
    return 1;
  }
}

pub trait QEventLoop_processEvents {
  fn processEvents(self, this: &mut QEventLoop) -> i32;
}

// proto: bool QEventLoop::processEvents(ProcessEventsFlags flags);
impl<'a> /*trait*/ QEventLoop_processEvents for (i32) {
  fn processEvents(self, this: &mut QEventLoop) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QEventLoop13processEventsE6QFlagsINS_17ProcessEventsFlagEE()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QEventLoop13processEventsE6QFlagsINS_17ProcessEventsFlagEE(arg0)};
    return 1;
  }
}

impl /*struct*/ QEventLoop {
  pub fn isRunning<T: QEventLoop_isRunning>(&mut self, value: T) -> i32 {
    value.isRunning(self);
    return 1;
  }
}

pub trait QEventLoop_isRunning {
  fn isRunning(self, this: &mut QEventLoop) -> i32;
}

// proto: bool QEventLoop::isRunning();
impl<'a> /*trait*/ QEventLoop_isRunning for () {
  fn isRunning(self, this: &mut QEventLoop) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QEventLoop9isRunningEv()};
    unsafe {_ZNK10QEventLoop9isRunningEv()};
    return 1;
  }
}

impl /*struct*/ QEventLoop {
  pub fn metaObject<T: QEventLoop_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QEventLoop_metaObject {
  fn metaObject(self, this: &mut QEventLoop) -> i32;
}

// proto: const QMetaObject * QEventLoop::metaObject();
impl<'a> /*trait*/ QEventLoop_metaObject for () {
  fn metaObject(self, this: &mut QEventLoop) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QEventLoop10metaObjectEv()};
    unsafe {_ZNK10QEventLoop10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QEventLoop {
  pub fn wakeUp<T: QEventLoop_wakeUp>(&mut self, value: T) -> i32 {
    value.wakeUp(self);
    return 1;
  }
}

pub trait QEventLoop_wakeUp {
  fn wakeUp(self, this: &mut QEventLoop) -> i32;
}

// proto: void QEventLoop::wakeUp();
impl<'a> /*trait*/ QEventLoop_wakeUp for () {
  fn wakeUp(self, this: &mut QEventLoop) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QEventLoop6wakeUpEv()};
    unsafe {_ZN10QEventLoop6wakeUpEv()};
    return 1;
  }
}

// proto: void QEventLoop::processEvents(ProcessEventsFlags flags, int maximumTime);
impl<'a> /*trait*/ QEventLoop_processEvents for (i32, i32) {
  fn processEvents(self, this: &mut QEventLoop) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QEventLoop13processEventsE6QFlagsINS_17ProcessEventsFlagEEi()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN10QEventLoop13processEventsE6QFlagsINS_17ProcessEventsFlagEEi(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QEventLoop {
  pub fn FreeQEventLoop<T: QEventLoop_FreeQEventLoop>(&mut self, value: T) -> i32 {
    value.FreeQEventLoop(self);
    return 1;
  }
}

pub trait QEventLoop_FreeQEventLoop {
  fn FreeQEventLoop(self, this: &mut QEventLoop) -> i32;
}

// proto: void QEventLoop::FreeQEventLoop();
impl<'a> /*trait*/ QEventLoop_FreeQEventLoop for () {
  fn FreeQEventLoop(self, this: &mut QEventLoop) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QEventLoopD0Ev()};
    unsafe {_ZN10QEventLoopD0Ev()};
    return 1;
  }
}

impl /*struct*/ QEventLoop {
  pub fn event<T: QEventLoop_event>(&mut self, value: T) -> i32 {
    value.event(self);
    return 1;
  }
}

pub trait QEventLoop_event {
  fn event(self, this: &mut QEventLoop) -> i32;
}

// proto: bool QEventLoop::event(QEvent * event);
impl<'a> /*trait*/ QEventLoop_event for (&'a mut QEvent) {
  fn event(self, this: &mut QEventLoop) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QEventLoop5eventEP6QEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QEventLoop5eventEP6QEvent(arg0)};
    return 1;
  }
}

