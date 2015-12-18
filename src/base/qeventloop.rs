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
  // proto:  void QEventLoop::exit(int returnCode);
  fn _ZN10QEventLoop4exitEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QEventLoop::quit();
  fn _ZN10QEventLoop4quitEv(qthis: *mut c_void) ;
  // proto:  void QEventLoop::NewQEventLoop(QObject * parent);
  fn _ZN10QEventLoopC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QEventLoop::isRunning();
  fn _ZNK10QEventLoop9isRunningEv(qthis: *mut c_void) -> int8_t;
  // proto:  const QMetaObject * QEventLoop::metaObject();
  fn _ZNK10QEventLoop10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QEventLoop::wakeUp();
  fn _ZN10QEventLoop6wakeUpEv(qthis: *mut c_void) ;
  // proto:  void QEventLoop::FreeQEventLoop();
  fn _ZN10QEventLoopD0Ev(qthis: *mut c_void) ;
  // proto:  bool QEventLoop::event(QEvent * event);
  fn _ZN10QEventLoop5eventEP6QEvent(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
}

// body block begin
// class sizeof(QEventLoop)=1
pub struct QEventLoop {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QEventLoop {
  pub fn exit<RetType, T: QEventLoop_exit<RetType>>(&mut self, value: T) -> RetType {
    return value.exit(self);
    // return 1;
  }
}

pub trait QEventLoop_exit<RetType> {
  fn exit(self, rsthis: &mut QEventLoop) -> RetType;
}

// proto:  void QEventLoop::exit(int returnCode);
impl<'a> /*trait*/ QEventLoop_exit<()> for (i32) {
  fn exit(self, rsthis: &mut QEventLoop) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QEventLoop4exitEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QEventLoop4exitEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QEventLoop {
  pub fn quit<RetType, T: QEventLoop_quit<RetType>>(&mut self, value: T) -> RetType {
    return value.quit(self);
    // return 1;
  }
}

pub trait QEventLoop_quit<RetType> {
  fn quit(self, rsthis: &mut QEventLoop) -> RetType;
}

// proto:  void QEventLoop::quit();
impl<'a> /*trait*/ QEventLoop_quit<()> for () {
  fn quit(self, rsthis: &mut QEventLoop) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QEventLoop4quitEv()};
     unsafe {_ZN10QEventLoop4quitEv(rsthis.qclsinst)};
    // return 1;
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
  pub fn isRunning<RetType, T: QEventLoop_isRunning<RetType>>(&mut self, value: T) -> RetType {
    return value.isRunning(self);
    // return 1;
  }
}

pub trait QEventLoop_isRunning<RetType> {
  fn isRunning(self, rsthis: &mut QEventLoop) -> RetType;
}

// proto:  bool QEventLoop::isRunning();
impl<'a> /*trait*/ QEventLoop_isRunning<i8> for () {
  fn isRunning(self, rsthis: &mut QEventLoop) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QEventLoop9isRunningEv()};
    let mut ret = unsafe {_ZNK10QEventLoop9isRunningEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QEventLoop {
  pub fn metaObject<RetType, T: QEventLoop_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QEventLoop_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QEventLoop) -> RetType;
}

// proto:  const QMetaObject * QEventLoop::metaObject();
impl<'a> /*trait*/ QEventLoop_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QEventLoop) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QEventLoop10metaObjectEv()};
     unsafe {_ZNK10QEventLoop10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QEventLoop {
  pub fn wakeUp<RetType, T: QEventLoop_wakeUp<RetType>>(&mut self, value: T) -> RetType {
    return value.wakeUp(self);
    // return 1;
  }
}

pub trait QEventLoop_wakeUp<RetType> {
  fn wakeUp(self, rsthis: &mut QEventLoop) -> RetType;
}

// proto:  void QEventLoop::wakeUp();
impl<'a> /*trait*/ QEventLoop_wakeUp<()> for () {
  fn wakeUp(self, rsthis: &mut QEventLoop) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QEventLoop6wakeUpEv()};
     unsafe {_ZN10QEventLoop6wakeUpEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QEventLoop {
  pub fn FreeQEventLoop<RetType, T: QEventLoop_FreeQEventLoop<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQEventLoop(self);
    // return 1;
  }
}

pub trait QEventLoop_FreeQEventLoop<RetType> {
  fn FreeQEventLoop(self, rsthis: &mut QEventLoop) -> RetType;
}

// proto:  void QEventLoop::FreeQEventLoop();
impl<'a> /*trait*/ QEventLoop_FreeQEventLoop<()> for () {
  fn FreeQEventLoop(self, rsthis: &mut QEventLoop) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QEventLoopD0Ev()};
     unsafe {_ZN10QEventLoopD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QEventLoop {
  pub fn event<RetType, T: QEventLoop_event<RetType>>(&mut self, value: T) -> RetType {
    return value.event(self);
    // return 1;
  }
}

pub trait QEventLoop_event<RetType> {
  fn event(self, rsthis: &mut QEventLoop) -> RetType;
}

// proto:  bool QEventLoop::event(QEvent * event);
impl<'a> /*trait*/ QEventLoop_event<i8> for (&'a mut QEvent) {
  fn event(self, rsthis: &mut QEventLoop) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QEventLoop5eventEP6QEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QEventLoop5eventEP6QEvent(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

