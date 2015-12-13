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
  fn _ZN7QThreadC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZNK7QThread10metaObjectEv() -> i32;
  fn _ZN7QThread18yieldCurrentThreadEv() -> i32;
  fn _ZNK7QThread23isInterruptionRequestedEv() -> i32;
  fn _ZN7QThread6msleepEm(arg0: c_ulong) -> i32;
  fn _ZN7QThread19requestInterruptionEv() -> i32;
  fn _ZN7QThread4exitEi(arg0: c_int) -> i32;
  fn _ZN7QThread5eventEP6QEvent(arg0: *mut c_void) -> i32;
  fn _ZNK7QThread9stackSizeEv() -> i32;
  fn _ZNK7QThread15eventDispatcherEv() -> i32;
  fn _ZN7QThread12setStackSizeEj(arg0: c_uint) -> i32;
  fn _ZNK7QThread10isFinishedEv() -> i32;
  fn _ZN7QThread5sleepEm(arg0: c_ulong) -> i32;
  fn _ZN7QThread6usleepEm(arg0: c_ulong) -> i32;
  fn _ZN7QThread16idealThreadCountEv() -> i32;
  fn _ZN7QThread4waitEm(arg0: c_ulong) -> i32;
  fn _ZN7QThread13currentThreadEv() -> i32;
  fn _ZNK7QThread9isRunningEv() -> i32;
  fn _ZN7QThread9terminateEv() -> i32;
  fn _ZN7QThreadD0Ev() -> i32;
  fn _ZN7QThread4quitEv() -> i32;
  fn _ZNK7QThread9loopLevelEv() -> i32;
}

// body block begin
// class sizeof(QThread)=1
pub struct QThread {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QThread {
  pub fn NewQThread<T: QThread_NewQThread>(value: T) -> QThread {
    let rsthis = value.NewQThread();
    return rsthis;
    // return 1;
  }
}

pub trait QThread_NewQThread {
  fn NewQThread(self) -> QThread;
}

// proto: void QThread::NewQThread(QObject * parent);
impl<'a> /*trait*/ QThread_NewQThread for (&'a mut QObject) {
  fn NewQThread(self) -> QThread {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThreadC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QThreadC1EP7QObject(qthis, arg0)};
    let rsthis = QThread{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QThread {
  pub fn metaObject<T: QThread_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QThread_metaObject {
  fn metaObject(self, this: &mut QThread) -> i32;
}

// proto: const QMetaObject * QThread::metaObject();
impl<'a> /*trait*/ QThread_metaObject for () {
  fn metaObject(self, this: &mut QThread) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QThread10metaObjectEv()};
    unsafe {_ZNK7QThread10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QThread {
  pub fn yieldCurrentThread<T: QThread_yieldCurrentThread>(&mut self, value: T) -> i32 {
    value.yieldCurrentThread(self);
    return 1;
  }
}

pub trait QThread_yieldCurrentThread {
  fn yieldCurrentThread(self, this: &mut QThread) -> i32;
}

// proto: void QThread::yieldCurrentThread();
impl<'a> /*trait*/ QThread_yieldCurrentThread for () {
  fn yieldCurrentThread(self, this: &mut QThread) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThread18yieldCurrentThreadEv()};
    unsafe {_ZN7QThread18yieldCurrentThreadEv()};
    return 1;
  }
}

impl /*struct*/ QThread {
  pub fn isInterruptionRequested<T: QThread_isInterruptionRequested>(&mut self, value: T) -> i32 {
    value.isInterruptionRequested(self);
    return 1;
  }
}

pub trait QThread_isInterruptionRequested {
  fn isInterruptionRequested(self, this: &mut QThread) -> i32;
}

// proto: bool QThread::isInterruptionRequested();
impl<'a> /*trait*/ QThread_isInterruptionRequested for () {
  fn isInterruptionRequested(self, this: &mut QThread) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QThread23isInterruptionRequestedEv()};
    unsafe {_ZNK7QThread23isInterruptionRequestedEv()};
    return 1;
  }
}

impl /*struct*/ QThread {
  pub fn msleep<T: QThread_msleep>(&mut self, value: T) -> i32 {
    value.msleep(self);
    return 1;
  }
}

pub trait QThread_msleep {
  fn msleep(self, this: &mut QThread) -> i32;
}

// proto: void QThread::msleep(unsigned long );
impl<'a> /*trait*/ QThread_msleep for (u32) {
  fn msleep(self, this: &mut QThread) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThread6msleepEm()};
    let arg0 = self  as c_ulong;
    unsafe {_ZN7QThread6msleepEm(arg0)};
    return 1;
  }
}

impl /*struct*/ QThread {
  pub fn requestInterruption<T: QThread_requestInterruption>(&mut self, value: T) -> i32 {
    value.requestInterruption(self);
    return 1;
  }
}

pub trait QThread_requestInterruption {
  fn requestInterruption(self, this: &mut QThread) -> i32;
}

// proto: void QThread::requestInterruption();
impl<'a> /*trait*/ QThread_requestInterruption for () {
  fn requestInterruption(self, this: &mut QThread) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThread19requestInterruptionEv()};
    unsafe {_ZN7QThread19requestInterruptionEv()};
    return 1;
  }
}

impl /*struct*/ QThread {
  pub fn exit<T: QThread_exit>(&mut self, value: T) -> i32 {
    value.exit(self);
    return 1;
  }
}

pub trait QThread_exit {
  fn exit(self, this: &mut QThread) -> i32;
}

// proto: void QThread::exit(int retcode);
impl<'a> /*trait*/ QThread_exit for (i32) {
  fn exit(self, this: &mut QThread) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThread4exitEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QThread4exitEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QThread {
  pub fn event<T: QThread_event>(&mut self, value: T) -> i32 {
    value.event(self);
    return 1;
  }
}

pub trait QThread_event {
  fn event(self, this: &mut QThread) -> i32;
}

// proto: bool QThread::event(QEvent * event);
impl<'a> /*trait*/ QThread_event for (&'a mut QEvent) {
  fn event(self, this: &mut QThread) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThread5eventEP6QEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QThread5eventEP6QEvent(arg0)};
    return 1;
  }
}

impl /*struct*/ QThread {
  pub fn stackSize<T: QThread_stackSize>(&mut self, value: T) -> i32 {
    value.stackSize(self);
    return 1;
  }
}

pub trait QThread_stackSize {
  fn stackSize(self, this: &mut QThread) -> i32;
}

// proto: unsigned int QThread::stackSize();
impl<'a> /*trait*/ QThread_stackSize for () {
  fn stackSize(self, this: &mut QThread) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QThread9stackSizeEv()};
    unsafe {_ZNK7QThread9stackSizeEv()};
    return 1;
  }
}

impl /*struct*/ QThread {
  pub fn eventDispatcher<T: QThread_eventDispatcher>(&mut self, value: T) -> i32 {
    value.eventDispatcher(self);
    return 1;
  }
}

pub trait QThread_eventDispatcher {
  fn eventDispatcher(self, this: &mut QThread) -> i32;
}

// proto: QAbstractEventDispatcher * QThread::eventDispatcher();
impl<'a> /*trait*/ QThread_eventDispatcher for () {
  fn eventDispatcher(self, this: &mut QThread) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QThread15eventDispatcherEv()};
    unsafe {_ZNK7QThread15eventDispatcherEv()};
    return 1;
  }
}

impl /*struct*/ QThread {
  pub fn setStackSize<T: QThread_setStackSize>(&mut self, value: T) -> i32 {
    value.setStackSize(self);
    return 1;
  }
}

pub trait QThread_setStackSize {
  fn setStackSize(self, this: &mut QThread) -> i32;
}

// proto: void QThread::setStackSize(uint stackSize);
impl<'a> /*trait*/ QThread_setStackSize for (u32) {
  fn setStackSize(self, this: &mut QThread) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThread12setStackSizeEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN7QThread12setStackSizeEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QThread {
  pub fn isFinished<T: QThread_isFinished>(&mut self, value: T) -> i32 {
    value.isFinished(self);
    return 1;
  }
}

pub trait QThread_isFinished {
  fn isFinished(self, this: &mut QThread) -> i32;
}

// proto: bool QThread::isFinished();
impl<'a> /*trait*/ QThread_isFinished for () {
  fn isFinished(self, this: &mut QThread) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QThread10isFinishedEv()};
    unsafe {_ZNK7QThread10isFinishedEv()};
    return 1;
  }
}

impl /*struct*/ QThread {
  pub fn sleep<T: QThread_sleep>(&mut self, value: T) -> i32 {
    value.sleep(self);
    return 1;
  }
}

pub trait QThread_sleep {
  fn sleep(self, this: &mut QThread) -> i32;
}

// proto: void QThread::sleep(unsigned long );
impl<'a> /*trait*/ QThread_sleep for (u32) {
  fn sleep(self, this: &mut QThread) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThread5sleepEm()};
    let arg0 = self  as c_ulong;
    unsafe {_ZN7QThread5sleepEm(arg0)};
    return 1;
  }
}

impl /*struct*/ QThread {
  pub fn usleep<T: QThread_usleep>(&mut self, value: T) -> i32 {
    value.usleep(self);
    return 1;
  }
}

pub trait QThread_usleep {
  fn usleep(self, this: &mut QThread) -> i32;
}

// proto: void QThread::usleep(unsigned long );
impl<'a> /*trait*/ QThread_usleep for (u32) {
  fn usleep(self, this: &mut QThread) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThread6usleepEm()};
    let arg0 = self  as c_ulong;
    unsafe {_ZN7QThread6usleepEm(arg0)};
    return 1;
  }
}

impl /*struct*/ QThread {
  pub fn idealThreadCount<T: QThread_idealThreadCount>(&mut self, value: T) -> i32 {
    value.idealThreadCount(self);
    return 1;
  }
}

pub trait QThread_idealThreadCount {
  fn idealThreadCount(self, this: &mut QThread) -> i32;
}

// proto: int QThread::idealThreadCount();
impl<'a> /*trait*/ QThread_idealThreadCount for () {
  fn idealThreadCount(self, this: &mut QThread) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThread16idealThreadCountEv()};
    unsafe {_ZN7QThread16idealThreadCountEv()};
    return 1;
  }
}

impl /*struct*/ QThread {
  pub fn wait<T: QThread_wait>(&mut self, value: T) -> i32 {
    value.wait(self);
    return 1;
  }
}

pub trait QThread_wait {
  fn wait(self, this: &mut QThread) -> i32;
}

// proto: bool QThread::wait(unsigned long time);
impl<'a> /*trait*/ QThread_wait for (u32) {
  fn wait(self, this: &mut QThread) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThread4waitEm()};
    let arg0 = self  as c_ulong;
    unsafe {_ZN7QThread4waitEm(arg0)};
    return 1;
  }
}

impl /*struct*/ QThread {
  pub fn currentThread<T: QThread_currentThread>(&mut self, value: T) -> i32 {
    value.currentThread(self);
    return 1;
  }
}

pub trait QThread_currentThread {
  fn currentThread(self, this: &mut QThread) -> i32;
}

// proto: QThread * QThread::currentThread();
impl<'a> /*trait*/ QThread_currentThread for () {
  fn currentThread(self, this: &mut QThread) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThread13currentThreadEv()};
    unsafe {_ZN7QThread13currentThreadEv()};
    return 1;
  }
}

impl /*struct*/ QThread {
  pub fn isRunning<T: QThread_isRunning>(&mut self, value: T) -> i32 {
    value.isRunning(self);
    return 1;
  }
}

pub trait QThread_isRunning {
  fn isRunning(self, this: &mut QThread) -> i32;
}

// proto: bool QThread::isRunning();
impl<'a> /*trait*/ QThread_isRunning for () {
  fn isRunning(self, this: &mut QThread) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QThread9isRunningEv()};
    unsafe {_ZNK7QThread9isRunningEv()};
    return 1;
  }
}

impl /*struct*/ QThread {
  pub fn terminate<T: QThread_terminate>(&mut self, value: T) -> i32 {
    value.terminate(self);
    return 1;
  }
}

pub trait QThread_terminate {
  fn terminate(self, this: &mut QThread) -> i32;
}

// proto: void QThread::terminate();
impl<'a> /*trait*/ QThread_terminate for () {
  fn terminate(self, this: &mut QThread) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThread9terminateEv()};
    unsafe {_ZN7QThread9terminateEv()};
    return 1;
  }
}

impl /*struct*/ QThread {
  pub fn FreeQThread<T: QThread_FreeQThread>(&mut self, value: T) -> i32 {
    value.FreeQThread(self);
    return 1;
  }
}

pub trait QThread_FreeQThread {
  fn FreeQThread(self, this: &mut QThread) -> i32;
}

// proto: void QThread::FreeQThread();
impl<'a> /*trait*/ QThread_FreeQThread for () {
  fn FreeQThread(self, this: &mut QThread) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThreadD0Ev()};
    unsafe {_ZN7QThreadD0Ev()};
    return 1;
  }
}

impl /*struct*/ QThread {
  pub fn quit<T: QThread_quit>(&mut self, value: T) -> i32 {
    value.quit(self);
    return 1;
  }
}

pub trait QThread_quit {
  fn quit(self, this: &mut QThread) -> i32;
}

// proto: void QThread::quit();
impl<'a> /*trait*/ QThread_quit for () {
  fn quit(self, this: &mut QThread) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThread4quitEv()};
    unsafe {_ZN7QThread4quitEv()};
    return 1;
  }
}

impl /*struct*/ QThread {
  pub fn loopLevel<T: QThread_loopLevel>(&mut self, value: T) -> i32 {
    value.loopLevel(self);
    return 1;
  }
}

pub trait QThread_loopLevel {
  fn loopLevel(self, this: &mut QThread) -> i32;
}

// proto: int QThread::loopLevel();
impl<'a> /*trait*/ QThread_loopLevel for () {
  fn loopLevel(self, this: &mut QThread) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QThread9loopLevelEv()};
    unsafe {_ZNK7QThread9loopLevelEv()};
    return 1;
  }
}

