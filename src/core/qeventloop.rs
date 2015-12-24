// auto generated, do not modify.
// created: Thu Dec 24 23:00:39 2015
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
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]

// #[link(name = "QtInline")]

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
    return & self.qbase;
  }
}
impl AsRef<QObject> for QEventLoop {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QEventLoop::exit(int returnCode);
impl /*struct*/ QEventLoop {
  pub fn exit<RetType, T: QEventLoop_exit<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.exit(self);
    // return 1;
  }
}

pub trait QEventLoop_exit<RetType> {
  fn exit(self , rsthis: & QEventLoop) -> RetType;
}

  // proto:  void QEventLoop::exit(int returnCode);
impl<'a> /*trait*/ QEventLoop_exit<()> for (i32) {
  fn exit(self , rsthis: & QEventLoop) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QEventLoop4exitEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QEventLoop4exitEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QEventLoop::quit();
impl /*struct*/ QEventLoop {
  pub fn quit<RetType, T: QEventLoop_quit<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.quit(self);
    // return 1;
  }
}

pub trait QEventLoop_quit<RetType> {
  fn quit(self , rsthis: & QEventLoop) -> RetType;
}

  // proto:  void QEventLoop::quit();
impl<'a> /*trait*/ QEventLoop_quit<()> for () {
  fn quit(self , rsthis: & QEventLoop) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QEventLoop4quitEv()};
     unsafe {_ZN10QEventLoop4quitEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QEventLoop::QEventLoop(QObject * parent);
impl /*struct*/ QEventLoop {
  pub fn New<T: QEventLoop_New>(value: T) -> QEventLoop {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QEventLoop_New {
  fn New(self) -> QEventLoop;
}

  // proto:  void QEventLoop::QEventLoop(QObject * parent);
impl<'a> /*trait*/ QEventLoop_New for (&'a QObject) {
  fn New(self) -> QEventLoop {
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
  pub fn isRunning<RetType, T: QEventLoop_isRunning<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isRunning(self);
    // return 1;
  }
}

pub trait QEventLoop_isRunning<RetType> {
  fn isRunning(self , rsthis: & QEventLoop) -> RetType;
}

  // proto:  bool QEventLoop::isRunning();
impl<'a> /*trait*/ QEventLoop_isRunning<i8> for () {
  fn isRunning(self , rsthis: & QEventLoop) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QEventLoop9isRunningEv()};
    let mut ret = unsafe {_ZNK10QEventLoop9isRunningEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  const QMetaObject * QEventLoop::metaObject();
impl /*struct*/ QEventLoop {
  pub fn metaObject<RetType, T: QEventLoop_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QEventLoop_metaObject<RetType> {
  fn metaObject(self , rsthis: & QEventLoop) -> RetType;
}

  // proto:  const QMetaObject * QEventLoop::metaObject();
impl<'a> /*trait*/ QEventLoop_metaObject<()> for () {
  fn metaObject(self , rsthis: & QEventLoop) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QEventLoop10metaObjectEv()};
     unsafe {_ZNK10QEventLoop10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QEventLoop::wakeUp();
impl /*struct*/ QEventLoop {
  pub fn wakeUp<RetType, T: QEventLoop_wakeUp<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.wakeUp(self);
    // return 1;
  }
}

pub trait QEventLoop_wakeUp<RetType> {
  fn wakeUp(self , rsthis: & QEventLoop) -> RetType;
}

  // proto:  void QEventLoop::wakeUp();
impl<'a> /*trait*/ QEventLoop_wakeUp<()> for () {
  fn wakeUp(self , rsthis: & QEventLoop) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QEventLoop6wakeUpEv()};
     unsafe {_ZN10QEventLoop6wakeUpEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QEventLoop::~QEventLoop();
impl /*struct*/ QEventLoop {
  pub fn Free<RetType, T: QEventLoop_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QEventLoop_Free<RetType> {
  fn Free(self , rsthis: & QEventLoop) -> RetType;
}

  // proto:  void QEventLoop::~QEventLoop();
impl<'a> /*trait*/ QEventLoop_Free<()> for () {
  fn Free(self , rsthis: & QEventLoop) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QEventLoopD0Ev()};
     unsafe {_ZN10QEventLoopD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QEventLoop::event(QEvent * event);
impl /*struct*/ QEventLoop {
  pub fn event<RetType, T: QEventLoop_event<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.event(self);
    // return 1;
  }
}

pub trait QEventLoop_event<RetType> {
  fn event(self , rsthis: & QEventLoop) -> RetType;
}

  // proto:  bool QEventLoop::event(QEvent * event);
impl<'a> /*trait*/ QEventLoop_event<i8> for (&'a QEvent) {
  fn event(self , rsthis: & QEventLoop) -> i8 {
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
  pub fn New<T: QEventLoopLocker_New>(value: T) -> QEventLoopLocker {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QEventLoopLocker_New {
  fn New(self) -> QEventLoopLocker;
}

  // proto:  void QEventLoopLocker::QEventLoopLocker(QThread * thread);
impl<'a> /*trait*/ QEventLoopLocker_New for (&'a QThread) {
  fn New(self) -> QEventLoopLocker {
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
impl<'a> /*trait*/ QEventLoopLocker_New for (&'a QEventLoop) {
  fn New(self) -> QEventLoopLocker {
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
impl<'a> /*trait*/ QEventLoopLocker_New for () {
  fn New(self) -> QEventLoopLocker {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QEventLoopLockerC1Ev()};
    unsafe {_ZN16QEventLoopLockerC1Ev(qthis)};
    let rsthis = QEventLoopLocker{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QEventLoopLocker::QEventLoopLocker(const QEventLoopLocker & );
impl<'a> /*trait*/ QEventLoopLocker_New for (&'a QEventLoopLocker) {
  fn New(self) -> QEventLoopLocker {
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
  pub fn Free<RetType, T: QEventLoopLocker_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QEventLoopLocker_Free<RetType> {
  fn Free(self , rsthis: & QEventLoopLocker) -> RetType;
}

  // proto:  void QEventLoopLocker::~QEventLoopLocker();
impl<'a> /*trait*/ QEventLoopLocker_Free<()> for () {
  fn Free(self , rsthis: & QEventLoopLocker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QEventLoopLockerD0Ev()};
     unsafe {_ZN16QEventLoopLockerD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

