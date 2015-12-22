// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
// src-file: /QtCore/qeventloop.h
// dst-file: /src/core/qeventloop.rs
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
use super::qthread::QThread; // 773
// use super::qeventloop::QEventLoop; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QEventLoop::exit(int returnCode);
  fn _ZN10QEventLoop4exitEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QEventLoop::quit();
  fn _ZN10QEventLoop4quitEv(qthis: *mut c_void);
  // proto:  void QEventLoop::QEventLoop(QObject * parent);
  fn _ZN10QEventLoopC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QEventLoop::isRunning();
  fn _ZNK10QEventLoop9isRunningEv(qthis: *mut c_void) -> c_char;
  // proto:  const QMetaObject * QEventLoop::metaObject();
  fn _ZNK10QEventLoop10metaObjectEv(qthis: *mut c_void);
  // proto:  void QEventLoop::wakeUp();
  fn _ZN10QEventLoop6wakeUpEv(qthis: *mut c_void);
  // proto:  void QEventLoop::~QEventLoop();
  fn _ZN10QEventLoopD0Ev(qthis: *mut c_void);
  // proto:  bool QEventLoop::event(QEvent * event);
  fn _ZN10QEventLoop5eventEP6QEvent(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  void QEventLoopLocker::QEventLoopLocker(QThread * thread);
  fn _ZN16QEventLoopLockerC1EP7QThread(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QEventLoopLocker::QEventLoopLocker(QEventLoop * loop);
  fn _ZN16QEventLoopLockerC1EP10QEventLoop(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QEventLoopLocker::QEventLoopLocker();
  fn _ZN16QEventLoopLockerC1Ev(qthis: *mut c_void);
  // proto:  void QEventLoopLocker::QEventLoopLocker(const QEventLoopLocker & );
  fn _ZN16QEventLoopLockerC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QEventLoopLocker::~QEventLoopLocker();
  fn _ZN16QEventLoopLockerD0Ev(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QEventLoop)=1
pub struct QEventLoop {
  qbase: QObject,
  pub qclsinst: *mut c_void,
}

// class sizeof(QEventLoopLocker)=8
pub struct QEventLoopLocker {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QEventLoop {
  pub fn inheritFrom(qthis: *mut c_void) -> QEventLoop {
    return QEventLoop{qbase: QObject::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QEventLoop {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return &self.qbase;
  }
}
impl AsRef<QObject> for QEventLoop {
  fn as_ref(&self) -> &QObject {
    return &self.qbase;
  }
}
  // proto:  void QEventLoop::exit(int returnCode);
impl /*struct*/ QEventLoop {
  pub fn exit<RetType, T: QEventLoop_exit<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.exit(self);
    // return 1;
  }
}

pub trait QEventLoop_exit<RetType> {
  fn exit(self , rsthis: &mut QEventLoop) -> RetType;
}

  // proto:  void QEventLoop::exit(int returnCode);
impl<'a> /*trait*/ QEventLoop_exit<()> for (i32) {
  fn exit(self , rsthis: &mut QEventLoop) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QEventLoop4exitEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QEventLoop4exitEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QEventLoop::quit();
impl /*struct*/ QEventLoop {
  pub fn quit<RetType, T: QEventLoop_quit<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.quit(self);
    // return 1;
  }
}

pub trait QEventLoop_quit<RetType> {
  fn quit(self , rsthis: &mut QEventLoop) -> RetType;
}

  // proto:  void QEventLoop::quit();
impl<'a> /*trait*/ QEventLoop_quit<()> for () {
  fn quit(self , rsthis: &mut QEventLoop) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QEventLoop4quitEv()};
     unsafe {_ZN10QEventLoop4quitEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QEventLoop::QEventLoop(QObject * parent);
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

  // proto:  void QEventLoop::QEventLoop(QObject * parent);
impl<'a> /*trait*/ QEventLoop_NewQEventLoop for (QObject) {
  fn NewQEventLoop(self) -> QEventLoop {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QEventLoopC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QEventLoopC1EP7QObject(qthis, arg0)};
    let rsthis = QEventLoop{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QEventLoop::isRunning();
impl /*struct*/ QEventLoop {
  pub fn isRunning<RetType, T: QEventLoop_isRunning<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isRunning(self);
    // return 1;
  }
}

pub trait QEventLoop_isRunning<RetType> {
  fn isRunning(self , rsthis: &mut QEventLoop) -> RetType;
}

  // proto:  bool QEventLoop::isRunning();
impl<'a> /*trait*/ QEventLoop_isRunning<i8> for () {
  fn isRunning(self , rsthis: &mut QEventLoop) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QEventLoop9isRunningEv()};
    let mut ret = unsafe {_ZNK10QEventLoop9isRunningEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  const QMetaObject * QEventLoop::metaObject();
impl /*struct*/ QEventLoop {
  pub fn metaObject<RetType, T: QEventLoop_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QEventLoop_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QEventLoop) -> RetType;
}

  // proto:  const QMetaObject * QEventLoop::metaObject();
impl<'a> /*trait*/ QEventLoop_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QEventLoop) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QEventLoop10metaObjectEv()};
     unsafe {_ZNK10QEventLoop10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QEventLoop::wakeUp();
impl /*struct*/ QEventLoop {
  pub fn wakeUp<RetType, T: QEventLoop_wakeUp<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.wakeUp(self);
    // return 1;
  }
}

pub trait QEventLoop_wakeUp<RetType> {
  fn wakeUp(self , rsthis: &mut QEventLoop) -> RetType;
}

  // proto:  void QEventLoop::wakeUp();
impl<'a> /*trait*/ QEventLoop_wakeUp<()> for () {
  fn wakeUp(self , rsthis: &mut QEventLoop) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QEventLoop6wakeUpEv()};
     unsafe {_ZN10QEventLoop6wakeUpEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QEventLoop::~QEventLoop();
impl /*struct*/ QEventLoop {
  pub fn FreeQEventLoop<RetType, T: QEventLoop_FreeQEventLoop<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQEventLoop(self);
    // return 1;
  }
}

pub trait QEventLoop_FreeQEventLoop<RetType> {
  fn FreeQEventLoop(self , rsthis: &mut QEventLoop) -> RetType;
}

  // proto:  void QEventLoop::~QEventLoop();
impl<'a> /*trait*/ QEventLoop_FreeQEventLoop<()> for () {
  fn FreeQEventLoop(self , rsthis: &mut QEventLoop) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QEventLoopD0Ev()};
     unsafe {_ZN10QEventLoopD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QEventLoop::event(QEvent * event);
impl /*struct*/ QEventLoop {
  pub fn event<RetType, T: QEventLoop_event<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.event(self);
    // return 1;
  }
}

pub trait QEventLoop_event<RetType> {
  fn event(self , rsthis: &mut QEventLoop) -> RetType;
}

  // proto:  bool QEventLoop::event(QEvent * event);
impl<'a> /*trait*/ QEventLoop_event<i8> for (QEvent) {
  fn event(self , rsthis: &mut QEventLoop) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QEventLoop5eventEP6QEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QEventLoop5eventEP6QEvent(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QEventLoopLocker {
  pub fn inheritFrom(qthis: *mut c_void) -> QEventLoopLocker {
    return QEventLoopLocker{qclsinst: qthis};
  }
}
  // proto:  void QEventLoopLocker::QEventLoopLocker(QThread * thread);
impl /*struct*/ QEventLoopLocker {
  pub fn NewQEventLoopLocker<T: QEventLoopLocker_NewQEventLoopLocker>(value: T) -> QEventLoopLocker {
    let rsthis = value.NewQEventLoopLocker();
    return rsthis;
    // return 1;
  }
}

pub trait QEventLoopLocker_NewQEventLoopLocker {
  fn NewQEventLoopLocker(self) -> QEventLoopLocker;
}

  // proto:  void QEventLoopLocker::QEventLoopLocker(QThread * thread);
impl<'a> /*trait*/ QEventLoopLocker_NewQEventLoopLocker for (QThread) {
  fn NewQEventLoopLocker(self) -> QEventLoopLocker {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QEventLoopLockerC1EP7QThread()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QEventLoopLockerC1EP7QThread(qthis, arg0)};
    let rsthis = QEventLoopLocker{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QEventLoopLocker::QEventLoopLocker(QEventLoop * loop);
impl<'a> /*trait*/ QEventLoopLocker_NewQEventLoopLocker for (QEventLoop) {
  fn NewQEventLoopLocker(self) -> QEventLoopLocker {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QEventLoopLockerC1EP10QEventLoop()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QEventLoopLockerC1EP10QEventLoop(qthis, arg0)};
    let rsthis = QEventLoopLocker{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QEventLoopLocker::QEventLoopLocker();
impl<'a> /*trait*/ QEventLoopLocker_NewQEventLoopLocker for () {
  fn NewQEventLoopLocker(self) -> QEventLoopLocker {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QEventLoopLockerC1Ev()};
    unsafe {_ZN16QEventLoopLockerC1Ev(qthis)};
    let rsthis = QEventLoopLocker{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QEventLoopLocker::QEventLoopLocker(const QEventLoopLocker & );
impl<'a> /*trait*/ QEventLoopLocker_NewQEventLoopLocker for (QEventLoopLocker) {
  fn NewQEventLoopLocker(self) -> QEventLoopLocker {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QEventLoopLockerC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QEventLoopLockerC1ERKS_(qthis, arg0)};
    let rsthis = QEventLoopLocker{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QEventLoopLocker::~QEventLoopLocker();
impl /*struct*/ QEventLoopLocker {
  pub fn FreeQEventLoopLocker<RetType, T: QEventLoopLocker_FreeQEventLoopLocker<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQEventLoopLocker(self);
    // return 1;
  }
}

pub trait QEventLoopLocker_FreeQEventLoopLocker<RetType> {
  fn FreeQEventLoopLocker(self , rsthis: &mut QEventLoopLocker) -> RetType;
}

  // proto:  void QEventLoopLocker::~QEventLoopLocker();
impl<'a> /*trait*/ QEventLoopLocker_FreeQEventLoopLocker<()> for () {
  fn FreeQEventLoopLocker(self , rsthis: &mut QEventLoopLocker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QEventLoopLockerD0Ev()};
     unsafe {_ZN16QEventLoopLockerD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

