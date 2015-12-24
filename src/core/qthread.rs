// auto generated, do not modify.
// created: Thu Dec 24 23:00:39 2015
// src-file: /QtCore/qthread.h
// dst-file: /src/core/qthread.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use super::qobject::QObject; // 773
use std::ops::Deref;
use super::qcoreevent::QEvent; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]

// #[link(name = "QtInline")]

extern {
  // proto:  void QThread::QThread(QObject * parent);
  fn _ZN7QThreadC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QThread::metaObject();
  fn _ZNK7QThread10metaObjectEv(qthis: *mut c_void);
  // proto: static void QThread::yieldCurrentThread();
  fn _ZN7QThread18yieldCurrentThreadEv();
  // proto:  bool QThread::isInterruptionRequested();
  fn _ZNK7QThread23isInterruptionRequestedEv(qthis: *mut c_void) -> c_char;
  // proto: static void QThread::msleep(unsigned long );
  fn _ZN7QThread6msleepEm(arg0: c_ulong);
  // proto:  void QThread::requestInterruption();
  fn _ZN7QThread19requestInterruptionEv(qthis: *mut c_void);
  // proto:  void QThread::exit(int retcode);
  fn _ZN7QThread4exitEi(qthis: *mut c_void, arg0: c_int);
  // proto:  bool QThread::event(QEvent * event);
  fn _ZN7QThread5eventEP6QEvent(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  uint QThread::stackSize();
  fn _ZNK7QThread9stackSizeEv(qthis: *mut c_void) -> c_uint;
  // proto:  QAbstractEventDispatcher * QThread::eventDispatcher();
  fn _ZNK7QThread15eventDispatcherEv(qthis: *mut c_void);
  // proto:  void QThread::setStackSize(uint stackSize);
  fn _ZN7QThread12setStackSizeEj(qthis: *mut c_void, arg0: c_uint);
  // proto:  bool QThread::isFinished();
  fn _ZNK7QThread10isFinishedEv(qthis: *mut c_void) -> c_char;
  // proto: static void QThread::sleep(unsigned long );
  fn _ZN7QThread5sleepEm(arg0: c_ulong);
  // proto: static void QThread::usleep(unsigned long );
  fn _ZN7QThread6usleepEm(arg0: c_ulong);
  // proto: static int QThread::idealThreadCount();
  fn _ZN7QThread16idealThreadCountEv() -> c_int;
  // proto:  bool QThread::wait(unsigned long time);
  fn _ZN7QThread4waitEm(qthis: *mut c_void, arg0: c_ulong) -> c_char;
  // proto: static QThread * QThread::currentThread();
  fn _ZN7QThread13currentThreadEv() -> *mut c_void;
  // proto:  bool QThread::isRunning();
  fn _ZNK7QThread9isRunningEv(qthis: *mut c_void) -> c_char;
  // proto:  void QThread::terminate();
  fn _ZN7QThread9terminateEv(qthis: *mut c_void);
  // proto:  void QThread::~QThread();
  fn _ZN7QThreadD0Ev(qthis: *mut c_void);
  // proto:  void QThread::quit();
  fn _ZN7QThread4quitEv(qthis: *mut c_void);
  // proto:  int QThread::loopLevel();
  fn _ZNK7QThread9loopLevelEv(qthis: *mut c_void) -> c_int;
} // <= ext block end

// body block begin =>
// class sizeof(QThread)=1
pub struct QThread {
  qbase: QObject,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QThread {
  pub fn inheritFrom(qthis: *mut c_void) -> QThread {
    return QThread{qbase: QObject::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QThread {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QThread {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QThread::QThread(QObject * parent);
impl /*struct*/ QThread {
  pub fn New<T: QThread_New>(value: T) -> QThread {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QThread_New {
  fn New(self) -> QThread;
}

  // proto:  void QThread::QThread(QObject * parent);
impl<'a> /*trait*/ QThread_New for (&'a QObject) {
  fn New(self) -> QThread {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThreadC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QThreadC1EP7QObject(qthis, arg0)};
    let rsthis = QThread{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QThread::metaObject();
impl /*struct*/ QThread {
  pub fn metaObject<RetType, T: QThread_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QThread_metaObject<RetType> {
  fn metaObject(self , rsthis: & QThread) -> RetType;
}

  // proto:  const QMetaObject * QThread::metaObject();
impl<'a> /*trait*/ QThread_metaObject<()> for () {
  fn metaObject(self , rsthis: & QThread) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QThread10metaObjectEv()};
     unsafe {_ZNK7QThread10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static void QThread::yieldCurrentThread();
impl /*struct*/ QThread {
  pub fn yieldCurrentThread_s<RetType, T: QThread_yieldCurrentThread_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.yieldCurrentThread_s();
    // return 1;
  }
}

pub trait QThread_yieldCurrentThread_s<RetType> {
  fn yieldCurrentThread_s(self ) -> RetType;
}

  // proto: static void QThread::yieldCurrentThread();
impl<'a> /*trait*/ QThread_yieldCurrentThread_s<()> for () {
  fn yieldCurrentThread_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThread18yieldCurrentThreadEv()};
     unsafe {_ZN7QThread18yieldCurrentThreadEv()};
    // return 1;
  }
}

  // proto:  bool QThread::isInterruptionRequested();
impl /*struct*/ QThread {
  pub fn isInterruptionRequested<RetType, T: QThread_isInterruptionRequested<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isInterruptionRequested(self);
    // return 1;
  }
}

pub trait QThread_isInterruptionRequested<RetType> {
  fn isInterruptionRequested(self , rsthis: & QThread) -> RetType;
}

  // proto:  bool QThread::isInterruptionRequested();
impl<'a> /*trait*/ QThread_isInterruptionRequested<i8> for () {
  fn isInterruptionRequested(self , rsthis: & QThread) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QThread23isInterruptionRequestedEv()};
    let mut ret = unsafe {_ZNK7QThread23isInterruptionRequestedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static void QThread::msleep(unsigned long );
impl /*struct*/ QThread {
  pub fn msleep_s<RetType, T: QThread_msleep_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.msleep_s();
    // return 1;
  }
}

pub trait QThread_msleep_s<RetType> {
  fn msleep_s(self ) -> RetType;
}

  // proto: static void QThread::msleep(unsigned long );
impl<'a> /*trait*/ QThread_msleep_s<()> for (u64) {
  fn msleep_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThread6msleepEm()};
    let arg0 = self  as c_ulong;
     unsafe {_ZN7QThread6msleepEm(arg0)};
    // return 1;
  }
}

  // proto:  void QThread::requestInterruption();
impl /*struct*/ QThread {
  pub fn requestInterruption<RetType, T: QThread_requestInterruption<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.requestInterruption(self);
    // return 1;
  }
}

pub trait QThread_requestInterruption<RetType> {
  fn requestInterruption(self , rsthis: & QThread) -> RetType;
}

  // proto:  void QThread::requestInterruption();
impl<'a> /*trait*/ QThread_requestInterruption<()> for () {
  fn requestInterruption(self , rsthis: & QThread) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThread19requestInterruptionEv()};
     unsafe {_ZN7QThread19requestInterruptionEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QThread::exit(int retcode);
impl /*struct*/ QThread {
  pub fn exit<RetType, T: QThread_exit<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.exit(self);
    // return 1;
  }
}

pub trait QThread_exit<RetType> {
  fn exit(self , rsthis: & QThread) -> RetType;
}

  // proto:  void QThread::exit(int retcode);
impl<'a> /*trait*/ QThread_exit<()> for (i32) {
  fn exit(self , rsthis: & QThread) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThread4exitEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QThread4exitEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QThread::event(QEvent * event);
impl /*struct*/ QThread {
  pub fn event<RetType, T: QThread_event<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.event(self);
    // return 1;
  }
}

pub trait QThread_event<RetType> {
  fn event(self , rsthis: & QThread) -> RetType;
}

  // proto:  bool QThread::event(QEvent * event);
impl<'a> /*trait*/ QThread_event<i8> for (&'a QEvent) {
  fn event(self , rsthis: & QThread) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThread5eventEP6QEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QThread5eventEP6QEvent(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  uint QThread::stackSize();
impl /*struct*/ QThread {
  pub fn stackSize<RetType, T: QThread_stackSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.stackSize(self);
    // return 1;
  }
}

pub trait QThread_stackSize<RetType> {
  fn stackSize(self , rsthis: & QThread) -> RetType;
}

  // proto:  uint QThread::stackSize();
impl<'a> /*trait*/ QThread_stackSize<u32> for () {
  fn stackSize(self , rsthis: & QThread) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QThread9stackSizeEv()};
    let mut ret = unsafe {_ZNK7QThread9stackSizeEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

  // proto:  QAbstractEventDispatcher * QThread::eventDispatcher();
impl /*struct*/ QThread {
  pub fn eventDispatcher<RetType, T: QThread_eventDispatcher<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.eventDispatcher(self);
    // return 1;
  }
}

pub trait QThread_eventDispatcher<RetType> {
  fn eventDispatcher(self , rsthis: & QThread) -> RetType;
}

  // proto:  QAbstractEventDispatcher * QThread::eventDispatcher();
impl<'a> /*trait*/ QThread_eventDispatcher<()> for () {
  fn eventDispatcher(self , rsthis: & QThread) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QThread15eventDispatcherEv()};
     unsafe {_ZNK7QThread15eventDispatcherEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QThread::setStackSize(uint stackSize);
impl /*struct*/ QThread {
  pub fn setStackSize<RetType, T: QThread_setStackSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setStackSize(self);
    // return 1;
  }
}

pub trait QThread_setStackSize<RetType> {
  fn setStackSize(self , rsthis: & QThread) -> RetType;
}

  // proto:  void QThread::setStackSize(uint stackSize);
impl<'a> /*trait*/ QThread_setStackSize<()> for (u32) {
  fn setStackSize(self , rsthis: & QThread) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThread12setStackSizeEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN7QThread12setStackSizeEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QThread::isFinished();
impl /*struct*/ QThread {
  pub fn isFinished<RetType, T: QThread_isFinished<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isFinished(self);
    // return 1;
  }
}

pub trait QThread_isFinished<RetType> {
  fn isFinished(self , rsthis: & QThread) -> RetType;
}

  // proto:  bool QThread::isFinished();
impl<'a> /*trait*/ QThread_isFinished<i8> for () {
  fn isFinished(self , rsthis: & QThread) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QThread10isFinishedEv()};
    let mut ret = unsafe {_ZNK7QThread10isFinishedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static void QThread::sleep(unsigned long );
impl /*struct*/ QThread {
  pub fn sleep_s<RetType, T: QThread_sleep_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.sleep_s();
    // return 1;
  }
}

pub trait QThread_sleep_s<RetType> {
  fn sleep_s(self ) -> RetType;
}

  // proto: static void QThread::sleep(unsigned long );
impl<'a> /*trait*/ QThread_sleep_s<()> for (u64) {
  fn sleep_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThread5sleepEm()};
    let arg0 = self  as c_ulong;
     unsafe {_ZN7QThread5sleepEm(arg0)};
    // return 1;
  }
}

  // proto: static void QThread::usleep(unsigned long );
impl /*struct*/ QThread {
  pub fn usleep_s<RetType, T: QThread_usleep_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.usleep_s();
    // return 1;
  }
}

pub trait QThread_usleep_s<RetType> {
  fn usleep_s(self ) -> RetType;
}

  // proto: static void QThread::usleep(unsigned long );
impl<'a> /*trait*/ QThread_usleep_s<()> for (u64) {
  fn usleep_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThread6usleepEm()};
    let arg0 = self  as c_ulong;
     unsafe {_ZN7QThread6usleepEm(arg0)};
    // return 1;
  }
}

  // proto: static int QThread::idealThreadCount();
impl /*struct*/ QThread {
  pub fn idealThreadCount_s<RetType, T: QThread_idealThreadCount_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.idealThreadCount_s();
    // return 1;
  }
}

pub trait QThread_idealThreadCount_s<RetType> {
  fn idealThreadCount_s(self ) -> RetType;
}

  // proto: static int QThread::idealThreadCount();
impl<'a> /*trait*/ QThread_idealThreadCount_s<i32> for () {
  fn idealThreadCount_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThread16idealThreadCountEv()};
    let mut ret = unsafe {_ZN7QThread16idealThreadCountEv()};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QThread::wait(unsigned long time);
impl /*struct*/ QThread {
  pub fn wait<RetType, T: QThread_wait<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.wait(self);
    // return 1;
  }
}

pub trait QThread_wait<RetType> {
  fn wait(self , rsthis: & QThread) -> RetType;
}

  // proto:  bool QThread::wait(unsigned long time);
impl<'a> /*trait*/ QThread_wait<i8> for (u64) {
  fn wait(self , rsthis: & QThread) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThread4waitEm()};
    let arg0 = self  as c_ulong;
    let mut ret = unsafe {_ZN7QThread4waitEm(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static QThread * QThread::currentThread();
impl /*struct*/ QThread {
  pub fn currentThread_s<RetType, T: QThread_currentThread_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.currentThread_s();
    // return 1;
  }
}

pub trait QThread_currentThread_s<RetType> {
  fn currentThread_s(self ) -> RetType;
}

  // proto: static QThread * QThread::currentThread();
impl<'a> /*trait*/ QThread_currentThread_s<QThread> for () {
  fn currentThread_s(self ) -> QThread {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThread13currentThreadEv()};
    let mut ret = unsafe {_ZN7QThread13currentThreadEv()};
    let mut ret1 = QThread::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QThread::isRunning();
impl /*struct*/ QThread {
  pub fn isRunning<RetType, T: QThread_isRunning<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isRunning(self);
    // return 1;
  }
}

pub trait QThread_isRunning<RetType> {
  fn isRunning(self , rsthis: & QThread) -> RetType;
}

  // proto:  bool QThread::isRunning();
impl<'a> /*trait*/ QThread_isRunning<i8> for () {
  fn isRunning(self , rsthis: & QThread) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QThread9isRunningEv()};
    let mut ret = unsafe {_ZNK7QThread9isRunningEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QThread::terminate();
impl /*struct*/ QThread {
  pub fn terminate<RetType, T: QThread_terminate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.terminate(self);
    // return 1;
  }
}

pub trait QThread_terminate<RetType> {
  fn terminate(self , rsthis: & QThread) -> RetType;
}

  // proto:  void QThread::terminate();
impl<'a> /*trait*/ QThread_terminate<()> for () {
  fn terminate(self , rsthis: & QThread) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThread9terminateEv()};
     unsafe {_ZN7QThread9terminateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QThread::~QThread();
impl /*struct*/ QThread {
  pub fn Free<RetType, T: QThread_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QThread_Free<RetType> {
  fn Free(self , rsthis: & QThread) -> RetType;
}

  // proto:  void QThread::~QThread();
impl<'a> /*trait*/ QThread_Free<()> for () {
  fn Free(self , rsthis: & QThread) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThreadD0Ev()};
     unsafe {_ZN7QThreadD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QThread::quit();
impl /*struct*/ QThread {
  pub fn quit<RetType, T: QThread_quit<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.quit(self);
    // return 1;
  }
}

pub trait QThread_quit<RetType> {
  fn quit(self , rsthis: & QThread) -> RetType;
}

  // proto:  void QThread::quit();
impl<'a> /*trait*/ QThread_quit<()> for () {
  fn quit(self , rsthis: & QThread) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QThread4quitEv()};
     unsafe {_ZN7QThread4quitEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QThread::loopLevel();
impl /*struct*/ QThread {
  pub fn loopLevel<RetType, T: QThread_loopLevel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.loopLevel(self);
    // return 1;
  }
}

pub trait QThread_loopLevel<RetType> {
  fn loopLevel(self , rsthis: & QThread) -> RetType;
}

  // proto:  int QThread::loopLevel();
impl<'a> /*trait*/ QThread_loopLevel<i32> for () {
  fn loopLevel(self , rsthis: & QThread) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QThread9loopLevelEv()};
    let mut ret = unsafe {_ZNK7QThread9loopLevelEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// <= body block end

