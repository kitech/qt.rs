// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
// src-file: /QtCore/qfuture.h
// dst-file: /src/core/qfuture.rs
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
use std::ops::Deref;
use super::qstring::QString; // 773
use super::qfutureinterface::QFutureInterfaceBase; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QFuture_void__Class_Size() -> c_int;
  // proto:  void QFuture<void>::resume();
  fn C_ZN7QFutureIvE6resumeEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QFuture<void>::waitForFinished();
  fn C_ZN7QFutureIvE15waitForFinishedEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QFuture<void>::isPaused();
  fn C_ZNK7QFutureIvE8isPausedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QFuture<void>::isFinished();
  fn C_ZNK7QFutureIvE10isFinishedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QFuture<void>::isRunning();
  fn C_ZNK7QFutureIvE9isRunningEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QFuture<void>::togglePaused();
  fn C_ZN7QFutureIvE12togglePausedEv(qthis: u64 /* *mut c_void*/);
  // proto:  int QFuture<void>::progressValue();
  fn C_ZNK7QFutureIvE13progressValueEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QString QFuture<void>::progressText();
  fn C_ZNK7QFutureIvE12progressTextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QFuture<void>::resultCount();
  fn C_ZNK7QFutureIvE11resultCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QFuture<void>::progressMaximum();
  fn C_ZNK7QFutureIvE15progressMaximumEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QFuture<void>::isCanceled();
  fn C_ZNK7QFutureIvE10isCanceledEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QFuture<void>::~QFuture();
  fn C_ZN7QFutureIvED2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QFuture<void>::pause();
  fn C_ZN7QFutureIvE5pauseEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QFuture<void>::QFuture(QFutureInterfaceBase * p);
  fn C_ZN7QFutureIvEC2EP20QFutureInterfaceBase(arg0: *mut c_void) -> u64;
  // proto:  void QFuture<void>::QFuture();
  fn C_ZN7QFutureIvEC2Ev() -> u64;
  // proto:  bool QFuture<void>::isStarted();
  fn C_ZNK7QFutureIvE9isStartedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QFuture<void>::cancel();
  fn C_ZN7QFutureIvE6cancelEv(qthis: u64 /* *mut c_void*/);
  // proto:  int QFuture<void>::progressMinimum();
  fn C_ZNK7QFutureIvE15progressMinimumEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QFuture<void>::setPaused(bool paused);
  fn C_ZN7QFutureIvE9setPausedEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
} // <= ext block end

// body block begin =>
// class sizeof(QFuture_void_)=16
#[derive(Default)]
pub struct QFuture_void_ {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QFuture_void_ {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QFuture_void_ {
    return QFuture_void_{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QFuture<void>::resume();
impl /*struct*/ QFuture_void_ {
  pub fn resume<RetType, T: QFuture_void__resume<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resume(self);
    // return 1;
  }
}

pub trait QFuture_void__resume<RetType> {
  fn resume(self , rsthis: & QFuture_void_) -> RetType;
}

  // proto:  void QFuture<void>::resume();
impl<'a> /*trait*/ QFuture_void__resume<()> for () {
  fn resume(self , rsthis: & QFuture_void_) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QFutureIvE6resumeEv()};
     unsafe {C_ZN7QFutureIvE6resumeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QFuture<void>::waitForFinished();
impl /*struct*/ QFuture_void_ {
  pub fn waitForFinished<RetType, T: QFuture_void__waitForFinished<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.waitForFinished(self);
    // return 1;
  }
}

pub trait QFuture_void__waitForFinished<RetType> {
  fn waitForFinished(self , rsthis: & QFuture_void_) -> RetType;
}

  // proto:  void QFuture<void>::waitForFinished();
impl<'a> /*trait*/ QFuture_void__waitForFinished<()> for () {
  fn waitForFinished(self , rsthis: & QFuture_void_) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QFutureIvE15waitForFinishedEv()};
     unsafe {C_ZN7QFutureIvE15waitForFinishedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QFuture<void>::isPaused();
impl /*struct*/ QFuture_void_ {
  pub fn isPaused<RetType, T: QFuture_void__isPaused<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isPaused(self);
    // return 1;
  }
}

pub trait QFuture_void__isPaused<RetType> {
  fn isPaused(self , rsthis: & QFuture_void_) -> RetType;
}

  // proto:  bool QFuture<void>::isPaused();
impl<'a> /*trait*/ QFuture_void__isPaused<i8> for () {
  fn isPaused(self , rsthis: & QFuture_void_) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QFutureIvE8isPausedEv()};
    let mut ret = unsafe {C_ZNK7QFutureIvE8isPausedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QFuture<void>::isFinished();
impl /*struct*/ QFuture_void_ {
  pub fn isFinished<RetType, T: QFuture_void__isFinished<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isFinished(self);
    // return 1;
  }
}

pub trait QFuture_void__isFinished<RetType> {
  fn isFinished(self , rsthis: & QFuture_void_) -> RetType;
}

  // proto:  bool QFuture<void>::isFinished();
impl<'a> /*trait*/ QFuture_void__isFinished<i8> for () {
  fn isFinished(self , rsthis: & QFuture_void_) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QFutureIvE10isFinishedEv()};
    let mut ret = unsafe {C_ZNK7QFutureIvE10isFinishedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QFuture<void>::isRunning();
impl /*struct*/ QFuture_void_ {
  pub fn isRunning<RetType, T: QFuture_void__isRunning<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isRunning(self);
    // return 1;
  }
}

pub trait QFuture_void__isRunning<RetType> {
  fn isRunning(self , rsthis: & QFuture_void_) -> RetType;
}

  // proto:  bool QFuture<void>::isRunning();
impl<'a> /*trait*/ QFuture_void__isRunning<i8> for () {
  fn isRunning(self , rsthis: & QFuture_void_) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QFutureIvE9isRunningEv()};
    let mut ret = unsafe {C_ZNK7QFutureIvE9isRunningEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QFuture<void>::togglePaused();
impl /*struct*/ QFuture_void_ {
  pub fn togglePaused<RetType, T: QFuture_void__togglePaused<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.togglePaused(self);
    // return 1;
  }
}

pub trait QFuture_void__togglePaused<RetType> {
  fn togglePaused(self , rsthis: & QFuture_void_) -> RetType;
}

  // proto:  void QFuture<void>::togglePaused();
impl<'a> /*trait*/ QFuture_void__togglePaused<()> for () {
  fn togglePaused(self , rsthis: & QFuture_void_) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QFutureIvE12togglePausedEv()};
     unsafe {C_ZN7QFutureIvE12togglePausedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QFuture<void>::progressValue();
impl /*struct*/ QFuture_void_ {
  pub fn progressValue<RetType, T: QFuture_void__progressValue<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.progressValue(self);
    // return 1;
  }
}

pub trait QFuture_void__progressValue<RetType> {
  fn progressValue(self , rsthis: & QFuture_void_) -> RetType;
}

  // proto:  int QFuture<void>::progressValue();
impl<'a> /*trait*/ QFuture_void__progressValue<i32> for () {
  fn progressValue(self , rsthis: & QFuture_void_) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QFutureIvE13progressValueEv()};
    let mut ret = unsafe {C_ZNK7QFutureIvE13progressValueEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QString QFuture<void>::progressText();
impl /*struct*/ QFuture_void_ {
  pub fn progressText<RetType, T: QFuture_void__progressText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.progressText(self);
    // return 1;
  }
}

pub trait QFuture_void__progressText<RetType> {
  fn progressText(self , rsthis: & QFuture_void_) -> RetType;
}

  // proto:  QString QFuture<void>::progressText();
impl<'a> /*trait*/ QFuture_void__progressText<QString> for () {
  fn progressText(self , rsthis: & QFuture_void_) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QFutureIvE12progressTextEv()};
    let mut ret = unsafe {C_ZNK7QFutureIvE12progressTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QFuture<void>::resultCount();
impl /*struct*/ QFuture_void_ {
  pub fn resultCount<RetType, T: QFuture_void__resultCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resultCount(self);
    // return 1;
  }
}

pub trait QFuture_void__resultCount<RetType> {
  fn resultCount(self , rsthis: & QFuture_void_) -> RetType;
}

  // proto:  int QFuture<void>::resultCount();
impl<'a> /*trait*/ QFuture_void__resultCount<i32> for () {
  fn resultCount(self , rsthis: & QFuture_void_) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QFutureIvE11resultCountEv()};
    let mut ret = unsafe {C_ZNK7QFutureIvE11resultCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QFuture<void>::progressMaximum();
impl /*struct*/ QFuture_void_ {
  pub fn progressMaximum<RetType, T: QFuture_void__progressMaximum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.progressMaximum(self);
    // return 1;
  }
}

pub trait QFuture_void__progressMaximum<RetType> {
  fn progressMaximum(self , rsthis: & QFuture_void_) -> RetType;
}

  // proto:  int QFuture<void>::progressMaximum();
impl<'a> /*trait*/ QFuture_void__progressMaximum<i32> for () {
  fn progressMaximum(self , rsthis: & QFuture_void_) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QFutureIvE15progressMaximumEv()};
    let mut ret = unsafe {C_ZNK7QFutureIvE15progressMaximumEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QFuture<void>::isCanceled();
impl /*struct*/ QFuture_void_ {
  pub fn isCanceled<RetType, T: QFuture_void__isCanceled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isCanceled(self);
    // return 1;
  }
}

pub trait QFuture_void__isCanceled<RetType> {
  fn isCanceled(self , rsthis: & QFuture_void_) -> RetType;
}

  // proto:  bool QFuture<void>::isCanceled();
impl<'a> /*trait*/ QFuture_void__isCanceled<i8> for () {
  fn isCanceled(self , rsthis: & QFuture_void_) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QFutureIvE10isCanceledEv()};
    let mut ret = unsafe {C_ZNK7QFutureIvE10isCanceledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QFuture<void>::~QFuture();
impl /*struct*/ QFuture_void_ {
  pub fn free<RetType, T: QFuture_void__free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QFuture_void__free<RetType> {
  fn free(self , rsthis: & QFuture_void_) -> RetType;
}

  // proto:  void QFuture<void>::~QFuture();
impl<'a> /*trait*/ QFuture_void__free<()> for () {
  fn free(self , rsthis: & QFuture_void_) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QFutureIvED2Ev()};
     unsafe {C_ZN7QFutureIvED2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QFuture<void>::pause();
impl /*struct*/ QFuture_void_ {
  pub fn pause<RetType, T: QFuture_void__pause<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pause(self);
    // return 1;
  }
}

pub trait QFuture_void__pause<RetType> {
  fn pause(self , rsthis: & QFuture_void_) -> RetType;
}

  // proto:  void QFuture<void>::pause();
impl<'a> /*trait*/ QFuture_void__pause<()> for () {
  fn pause(self , rsthis: & QFuture_void_) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QFutureIvE5pauseEv()};
     unsafe {C_ZN7QFutureIvE5pauseEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QFuture<void>::QFuture(QFutureInterfaceBase * p);
impl /*struct*/ QFuture_void_ {
  pub fn new<T: QFuture_void__new>(value: T) -> QFuture_void_ {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QFuture_void__new {
  fn new(self) -> QFuture_void_;
}

  // proto:  void QFuture<void>::QFuture(QFutureInterfaceBase * p);
impl<'a> /*trait*/ QFuture_void__new for (&'a QFutureInterfaceBase) {
  fn new(self) -> QFuture_void_ {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QFutureIvEC2EP20QFutureInterfaceBase()};
    let ctysz: c_int = unsafe{QFuture_void__Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN7QFutureIvEC2EP20QFutureInterfaceBase(arg0)};
    let rsthis = QFuture_void_{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QFuture<void>::QFuture();
impl<'a> /*trait*/ QFuture_void__new for () {
  fn new(self) -> QFuture_void_ {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QFutureIvEC2Ev()};
    let ctysz: c_int = unsafe{QFuture_void__Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN7QFutureIvEC2Ev()};
    let rsthis = QFuture_void_{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QFuture<void>::isStarted();
impl /*struct*/ QFuture_void_ {
  pub fn isStarted<RetType, T: QFuture_void__isStarted<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isStarted(self);
    // return 1;
  }
}

pub trait QFuture_void__isStarted<RetType> {
  fn isStarted(self , rsthis: & QFuture_void_) -> RetType;
}

  // proto:  bool QFuture<void>::isStarted();
impl<'a> /*trait*/ QFuture_void__isStarted<i8> for () {
  fn isStarted(self , rsthis: & QFuture_void_) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QFutureIvE9isStartedEv()};
    let mut ret = unsafe {C_ZNK7QFutureIvE9isStartedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QFuture<void>::cancel();
impl /*struct*/ QFuture_void_ {
  pub fn cancel<RetType, T: QFuture_void__cancel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cancel(self);
    // return 1;
  }
}

pub trait QFuture_void__cancel<RetType> {
  fn cancel(self , rsthis: & QFuture_void_) -> RetType;
}

  // proto:  void QFuture<void>::cancel();
impl<'a> /*trait*/ QFuture_void__cancel<()> for () {
  fn cancel(self , rsthis: & QFuture_void_) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QFutureIvE6cancelEv()};
     unsafe {C_ZN7QFutureIvE6cancelEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QFuture<void>::progressMinimum();
impl /*struct*/ QFuture_void_ {
  pub fn progressMinimum<RetType, T: QFuture_void__progressMinimum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.progressMinimum(self);
    // return 1;
  }
}

pub trait QFuture_void__progressMinimum<RetType> {
  fn progressMinimum(self , rsthis: & QFuture_void_) -> RetType;
}

  // proto:  int QFuture<void>::progressMinimum();
impl<'a> /*trait*/ QFuture_void__progressMinimum<i32> for () {
  fn progressMinimum(self , rsthis: & QFuture_void_) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QFutureIvE15progressMinimumEv()};
    let mut ret = unsafe {C_ZNK7QFutureIvE15progressMinimumEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QFuture<void>::setPaused(bool paused);
impl /*struct*/ QFuture_void_ {
  pub fn setPaused<RetType, T: QFuture_void__setPaused<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPaused(self);
    // return 1;
  }
}

pub trait QFuture_void__setPaused<RetType> {
  fn setPaused(self , rsthis: & QFuture_void_) -> RetType;
}

  // proto:  void QFuture<void>::setPaused(bool paused);
impl<'a> /*trait*/ QFuture_void__setPaused<()> for (i8) {
  fn setPaused(self , rsthis: & QFuture_void_) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QFutureIvE9setPausedEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN7QFutureIvE9setPausedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

