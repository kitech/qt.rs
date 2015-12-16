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
  // proto:  void QThread::NewQThread(QObject * parent);
  fn _ZN7QThreadC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QMetaObject * QThread::metaObject();
  fn _ZNK7QThread10metaObjectEv(qthis: *mut c_void) ;
  // proto: static void QThread::yieldCurrentThread();
  fn _ZN7QThread18yieldCurrentThreadEv() ;
  // proto:  bool QThread::isInterruptionRequested();
  fn _ZNK7QThread23isInterruptionRequestedEv(qthis: *mut c_void) -> int8_t;
  // proto: static void QThread::msleep(unsigned long );
  fn _ZN7QThread6msleepEm(arg0: c_ulong) ;
  // proto:  void QThread::requestInterruption();
  fn _ZN7QThread19requestInterruptionEv(qthis: *mut c_void) ;
  // proto:  void QThread::exit(int retcode);
  fn _ZN7QThread4exitEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  bool QThread::event(QEvent * event);
  fn _ZN7QThread5eventEP6QEvent(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  unsigned int QThread::stackSize();
  fn _ZNK7QThread9stackSizeEv(qthis: *mut c_void) -> c_uint;
  // proto:  QAbstractEventDispatcher * QThread::eventDispatcher();
  fn _ZNK7QThread15eventDispatcherEv(qthis: *mut c_void) ;
  // proto:  void QThread::setStackSize(uint stackSize);
  fn _ZN7QThread12setStackSizeEj(qthis: *mut c_void, arg0: c_uint) ;
  // proto:  bool QThread::isFinished();
  fn _ZNK7QThread10isFinishedEv(qthis: *mut c_void) -> int8_t;
  // proto: static void QThread::sleep(unsigned long );
  fn _ZN7QThread5sleepEm(arg0: c_ulong) ;
  // proto: static void QThread::usleep(unsigned long );
  fn _ZN7QThread6usleepEm(arg0: c_ulong) ;
  // proto: static int QThread::idealThreadCount();
  fn _ZN7QThread16idealThreadCountEv() -> c_int;
  // proto:  bool QThread::wait(unsigned long time);
  fn _ZN7QThread4waitEm(qthis: *mut c_void, arg0: c_ulong) -> int8_t;
  // proto: static QThread * QThread::currentThread();
  fn _ZN7QThread13currentThreadEv() -> *mut c_void;
  // proto:  bool QThread::isRunning();
  fn _ZNK7QThread9isRunningEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QThread::terminate();
  fn _ZN7QThread9terminateEv(qthis: *mut c_void) ;
  // proto:  void QThread::FreeQThread();
  fn _ZN7QThreadD0Ev(qthis: *mut c_void) ;
  // proto:  void QThread::quit();
  fn _ZN7QThread4quitEv(qthis: *mut c_void) ;
  // proto:  int QThread::loopLevel();
  fn _ZNK7QThread9loopLevelEv(qthis: *mut c_void) -> c_int;
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
  pub fn metaObject<T: QThread_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QThread_metaObject {
  fn metaObject(self, rsthis: &mut QThread) ;
}

// proto:  const QMetaObject * QThread::metaObject();
impl<'a> /*trait*/ QThread_metaObject for () {
  fn metaObject(self, rsthis: &mut QThread)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QThread10metaObjectEv()};
     unsafe {_ZNK7QThread10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QThread {
  pub fn yieldCurrentThread<T: QThread_yieldCurrentThread>(&mut self, value: T)  {
     value.yieldCurrentThread(self);
    // return 1;
  }
}

pub trait QThread_yieldCurrentThread {
  fn yieldCurrentThread(self, rsthis: &mut QThread) ;
}

// proto: static void QThread::yieldCurrentThread();
impl<'a> /*trait*/ QThread_yieldCurrentThread for () {
  fn yieldCurrentThread(self, rsthis: &mut QThread)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThread18yieldCurrentThreadEv()};
     unsafe {_ZN7QThread18yieldCurrentThreadEv()};
    // return 1;
  }
}

impl /*struct*/ QThread {
  pub fn isInterruptionRequested<T: QThread_isInterruptionRequested>(&mut self, value: T) -> i8 {
    return value.isInterruptionRequested(self);
    // return 1;
  }
}

pub trait QThread_isInterruptionRequested {
  fn isInterruptionRequested(self, rsthis: &mut QThread) -> i8;
}

// proto:  bool QThread::isInterruptionRequested();
impl<'a> /*trait*/ QThread_isInterruptionRequested for () {
  fn isInterruptionRequested(self, rsthis: &mut QThread) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QThread23isInterruptionRequestedEv()};
    let mut ret = unsafe {_ZNK7QThread23isInterruptionRequestedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QThread {
  pub fn msleep<T: QThread_msleep>(&mut self, value: T)  {
     value.msleep(self);
    // return 1;
  }
}

pub trait QThread_msleep {
  fn msleep(self, rsthis: &mut QThread) ;
}

// proto: static void QThread::msleep(unsigned long );
impl<'a> /*trait*/ QThread_msleep for (u32) {
  fn msleep(self, rsthis: &mut QThread)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThread6msleepEm()};
    let arg0 = self  as c_ulong;
     unsafe {_ZN7QThread6msleepEm(arg0)};
    // return 1;
  }
}

impl /*struct*/ QThread {
  pub fn requestInterruption<T: QThread_requestInterruption>(&mut self, value: T)  {
     value.requestInterruption(self);
    // return 1;
  }
}

pub trait QThread_requestInterruption {
  fn requestInterruption(self, rsthis: &mut QThread) ;
}

// proto:  void QThread::requestInterruption();
impl<'a> /*trait*/ QThread_requestInterruption for () {
  fn requestInterruption(self, rsthis: &mut QThread)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThread19requestInterruptionEv()};
     unsafe {_ZN7QThread19requestInterruptionEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QThread {
  pub fn exit<T: QThread_exit>(&mut self, value: T)  {
     value.exit(self);
    // return 1;
  }
}

pub trait QThread_exit {
  fn exit(self, rsthis: &mut QThread) ;
}

// proto:  void QThread::exit(int retcode);
impl<'a> /*trait*/ QThread_exit for (i32) {
  fn exit(self, rsthis: &mut QThread)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThread4exitEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QThread4exitEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QThread {
  pub fn event<T: QThread_event>(&mut self, value: T) -> i8 {
    return value.event(self);
    // return 1;
  }
}

pub trait QThread_event {
  fn event(self, rsthis: &mut QThread) -> i8;
}

// proto:  bool QThread::event(QEvent * event);
impl<'a> /*trait*/ QThread_event for (&'a mut QEvent) {
  fn event(self, rsthis: &mut QThread) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThread5eventEP6QEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QThread5eventEP6QEvent(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QThread {
  pub fn stackSize<T: QThread_stackSize>(&mut self, value: T) -> u32 {
    return value.stackSize(self);
    // return 1;
  }
}

pub trait QThread_stackSize {
  fn stackSize(self, rsthis: &mut QThread) -> u32;
}

// proto:  unsigned int QThread::stackSize();
impl<'a> /*trait*/ QThread_stackSize for () {
  fn stackSize(self, rsthis: &mut QThread) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QThread9stackSizeEv()};
    let mut ret = unsafe {_ZNK7QThread9stackSizeEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

impl /*struct*/ QThread {
  pub fn eventDispatcher<T: QThread_eventDispatcher>(&mut self, value: T)  {
     value.eventDispatcher(self);
    // return 1;
  }
}

pub trait QThread_eventDispatcher {
  fn eventDispatcher(self, rsthis: &mut QThread) ;
}

// proto:  QAbstractEventDispatcher * QThread::eventDispatcher();
impl<'a> /*trait*/ QThread_eventDispatcher for () {
  fn eventDispatcher(self, rsthis: &mut QThread)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QThread15eventDispatcherEv()};
     unsafe {_ZNK7QThread15eventDispatcherEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QThread {
  pub fn setStackSize<T: QThread_setStackSize>(&mut self, value: T)  {
     value.setStackSize(self);
    // return 1;
  }
}

pub trait QThread_setStackSize {
  fn setStackSize(self, rsthis: &mut QThread) ;
}

// proto:  void QThread::setStackSize(uint stackSize);
impl<'a> /*trait*/ QThread_setStackSize for (u32) {
  fn setStackSize(self, rsthis: &mut QThread)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThread12setStackSizeEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN7QThread12setStackSizeEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QThread {
  pub fn isFinished<T: QThread_isFinished>(&mut self, value: T) -> i8 {
    return value.isFinished(self);
    // return 1;
  }
}

pub trait QThread_isFinished {
  fn isFinished(self, rsthis: &mut QThread) -> i8;
}

// proto:  bool QThread::isFinished();
impl<'a> /*trait*/ QThread_isFinished for () {
  fn isFinished(self, rsthis: &mut QThread) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QThread10isFinishedEv()};
    let mut ret = unsafe {_ZNK7QThread10isFinishedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QThread {
  pub fn sleep<T: QThread_sleep>(&mut self, value: T)  {
     value.sleep(self);
    // return 1;
  }
}

pub trait QThread_sleep {
  fn sleep(self, rsthis: &mut QThread) ;
}

// proto: static void QThread::sleep(unsigned long );
impl<'a> /*trait*/ QThread_sleep for (u32) {
  fn sleep(self, rsthis: &mut QThread)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThread5sleepEm()};
    let arg0 = self  as c_ulong;
     unsafe {_ZN7QThread5sleepEm(arg0)};
    // return 1;
  }
}

impl /*struct*/ QThread {
  pub fn usleep<T: QThread_usleep>(&mut self, value: T)  {
     value.usleep(self);
    // return 1;
  }
}

pub trait QThread_usleep {
  fn usleep(self, rsthis: &mut QThread) ;
}

// proto: static void QThread::usleep(unsigned long );
impl<'a> /*trait*/ QThread_usleep for (u32) {
  fn usleep(self, rsthis: &mut QThread)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThread6usleepEm()};
    let arg0 = self  as c_ulong;
     unsafe {_ZN7QThread6usleepEm(arg0)};
    // return 1;
  }
}

impl /*struct*/ QThread {
  pub fn idealThreadCount<T: QThread_idealThreadCount>(&mut self, value: T) -> i32 {
    return value.idealThreadCount(self);
    // return 1;
  }
}

pub trait QThread_idealThreadCount {
  fn idealThreadCount(self, rsthis: &mut QThread) -> i32;
}

// proto: static int QThread::idealThreadCount();
impl<'a> /*trait*/ QThread_idealThreadCount for () {
  fn idealThreadCount(self, rsthis: &mut QThread) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThread16idealThreadCountEv()};
    let mut ret = unsafe {_ZN7QThread16idealThreadCountEv()};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QThread {
  pub fn wait<T: QThread_wait>(&mut self, value: T) -> i8 {
    return value.wait(self);
    // return 1;
  }
}

pub trait QThread_wait {
  fn wait(self, rsthis: &mut QThread) -> i8;
}

// proto:  bool QThread::wait(unsigned long time);
impl<'a> /*trait*/ QThread_wait for (u32) {
  fn wait(self, rsthis: &mut QThread) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThread4waitEm()};
    let arg0 = self  as c_ulong;
    let mut ret = unsafe {_ZN7QThread4waitEm(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QThread {
  pub fn currentThread<T: QThread_currentThread>(&mut self, value: T) -> QThread {
    return value.currentThread(self);
    // return 1;
  }
}

pub trait QThread_currentThread {
  fn currentThread(self, rsthis: &mut QThread) -> QThread;
}

// proto: static QThread * QThread::currentThread();
impl<'a> /*trait*/ QThread_currentThread for () {
  fn currentThread(self, rsthis: &mut QThread) -> QThread {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThread13currentThreadEv()};
    let mut ret = unsafe {_ZN7QThread13currentThreadEv()};
    let mut ret1 = QThread{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QThread {
  pub fn isRunning<T: QThread_isRunning>(&mut self, value: T) -> i8 {
    return value.isRunning(self);
    // return 1;
  }
}

pub trait QThread_isRunning {
  fn isRunning(self, rsthis: &mut QThread) -> i8;
}

// proto:  bool QThread::isRunning();
impl<'a> /*trait*/ QThread_isRunning for () {
  fn isRunning(self, rsthis: &mut QThread) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QThread9isRunningEv()};
    let mut ret = unsafe {_ZNK7QThread9isRunningEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QThread {
  pub fn terminate<T: QThread_terminate>(&mut self, value: T)  {
     value.terminate(self);
    // return 1;
  }
}

pub trait QThread_terminate {
  fn terminate(self, rsthis: &mut QThread) ;
}

// proto:  void QThread::terminate();
impl<'a> /*trait*/ QThread_terminate for () {
  fn terminate(self, rsthis: &mut QThread)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThread9terminateEv()};
     unsafe {_ZN7QThread9terminateEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QThread {
  pub fn FreeQThread<T: QThread_FreeQThread>(&mut self, value: T)  {
     value.FreeQThread(self);
    // return 1;
  }
}

pub trait QThread_FreeQThread {
  fn FreeQThread(self, rsthis: &mut QThread) ;
}

// proto:  void QThread::FreeQThread();
impl<'a> /*trait*/ QThread_FreeQThread for () {
  fn FreeQThread(self, rsthis: &mut QThread)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThreadD0Ev()};
     unsafe {_ZN7QThreadD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QThread {
  pub fn quit<T: QThread_quit>(&mut self, value: T)  {
     value.quit(self);
    // return 1;
  }
}

pub trait QThread_quit {
  fn quit(self, rsthis: &mut QThread) ;
}

// proto:  void QThread::quit();
impl<'a> /*trait*/ QThread_quit for () {
  fn quit(self, rsthis: &mut QThread)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThread4quitEv()};
     unsafe {_ZN7QThread4quitEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QThread {
  pub fn loopLevel<T: QThread_loopLevel>(&mut self, value: T) -> i32 {
    return value.loopLevel(self);
    // return 1;
  }
}

pub trait QThread_loopLevel {
  fn loopLevel(self, rsthis: &mut QThread) -> i32;
}

// proto:  int QThread::loopLevel();
impl<'a> /*trait*/ QThread_loopLevel for () {
  fn loopLevel(self, rsthis: &mut QThread) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QThread9loopLevelEv()};
    let mut ret = unsafe {_ZNK7QThread9loopLevelEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

