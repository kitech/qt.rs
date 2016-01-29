// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
// src-file: /QtCore/qfuturewatcher.h
// dst-file: /src/core/qfuturewatcher.rs
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
use super::qobject::*; // 773
use std::ops::Deref;
use super::qobjectdefs::*; // 773
use super::qcoreevent::*; // 773
use super::qstring::*; // 773
// use super::qfuturewatcher::QFutureWatcherBase; // 773
// use super::qfuture::*; // 775
use super::qfuture::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QFutureWatcherBase_Class_Size() -> c_int;
  // proto:  bool QFutureWatcherBase::isRunning();
  fn C_ZNK18QFutureWatcherBase9isRunningEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QFutureWatcherBase::setPaused(bool paused);
  fn C_ZN18QFutureWatcherBase9setPausedEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  int QFutureWatcherBase::progressMinimum();
  fn C_ZNK18QFutureWatcherBase15progressMinimumEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QFutureWatcherBase::resume();
  fn C_ZN18QFutureWatcherBase6resumeEv(qthis: u64 /* *mut c_void*/);
  // proto:  const QMetaObject * QFutureWatcherBase::metaObject();
  fn C_ZNK18QFutureWatcherBase10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QFutureWatcherBase::isFinished();
  fn C_ZNK18QFutureWatcherBase10isFinishedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QFutureWatcherBase::progressMaximum();
  fn C_ZNK18QFutureWatcherBase15progressMaximumEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QFutureWatcherBase::event(QEvent * event);
  fn C_ZN18QFutureWatcherBase5eventEP6QEvent(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  bool QFutureWatcherBase::isCanceled();
  fn C_ZNK18QFutureWatcherBase10isCanceledEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QFutureWatcherBase::progressValue();
  fn C_ZNK18QFutureWatcherBase13progressValueEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QFutureWatcherBase::isStarted();
  fn C_ZNK18QFutureWatcherBase9isStartedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QFutureWatcherBase::setPendingResultsLimit(int limit);
  fn C_ZN18QFutureWatcherBase22setPendingResultsLimitEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QFutureWatcherBase::cancel();
  fn C_ZN18QFutureWatcherBase6cancelEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QFutureWatcherBase::isPaused();
  fn C_ZNK18QFutureWatcherBase8isPausedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QFutureWatcherBase::pause();
  fn C_ZN18QFutureWatcherBase5pauseEv(qthis: u64 /* *mut c_void*/);
  // proto:  QString QFutureWatcherBase::progressText();
  fn C_ZNK18QFutureWatcherBase12progressTextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QFutureWatcherBase::QFutureWatcherBase(QObject * parent);
  fn C_ZN18QFutureWatcherBaseC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  void QFutureWatcherBase::togglePaused();
  fn C_ZN18QFutureWatcherBase12togglePausedEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QFutureWatcherBase::waitForFinished();
  fn C_ZN18QFutureWatcherBase15waitForFinishedEv(qthis: u64 /* *mut c_void*/);
  fn QFutureWatcherLvoidG_Class_Size() -> c_int;
  // proto:  void QFutureWatcher<void>::QFutureWatcher(QObject * _parent);
  fn C_ZN14QFutureWatcherIvEC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  QFuture<void> QFutureWatcher<void>::future();
  fn C_ZNK14QFutureWatcherIvE6futureEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QFutureWatcher<void>::~QFutureWatcher();
  fn C_ZN14QFutureWatcherIvED2Ev(qthis: u64 /* *mut c_void*/);
  fn QFutureWatcherBase_SlotProxy_connect__ZN18QFutureWatcherBase8finishedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QFutureWatcherBase_SlotProxy_connect__ZN18QFutureWatcherBase19progressTextChangedERK7QString(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QFutureWatcherBase_SlotProxy_connect__ZN18QFutureWatcherBase6pausedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QFutureWatcherBase_SlotProxy_connect__ZN18QFutureWatcherBase7startedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QFutureWatcherBase_SlotProxy_connect__ZN18QFutureWatcherBase7resumedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QFutureWatcherBase_SlotProxy_connect__ZN18QFutureWatcherBase20progressRangeChangedEii(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QFutureWatcherBase_SlotProxy_connect__ZN18QFutureWatcherBase8canceledEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QFutureWatcherBase_SlotProxy_connect__ZN18QFutureWatcherBase20progressValueChangedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QFutureWatcherBase_SlotProxy_connect__ZN18QFutureWatcherBase13resultReadyAtEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QFutureWatcherBase_SlotProxy_connect__ZN18QFutureWatcherBase14resultsReadyAtEii(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QFutureWatcherBase)=1
#[derive(Default)]
pub struct QFutureWatcherBase {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _progressRangeChanged: QFutureWatcherBase_progressRangeChanged_signal,
  pub _resumed: QFutureWatcherBase_resumed_signal,
  pub _progressValueChanged: QFutureWatcherBase_progressValueChanged_signal,
  pub _started: QFutureWatcherBase_started_signal,
  pub _resultReadyAt: QFutureWatcherBase_resultReadyAt_signal,
  pub _resultsReadyAt: QFutureWatcherBase_resultsReadyAt_signal,
  pub _paused: QFutureWatcherBase_paused_signal,
  pub _canceled: QFutureWatcherBase_canceled_signal,
  pub _finished: QFutureWatcherBase_finished_signal,
  pub _progressTextChanged: QFutureWatcherBase_progressTextChanged_signal,
}

// class sizeof(QFutureWatcherLvoidG)=1
#[derive(Default)]
pub struct QFutureWatcherLvoidG {
  qbase: QFutureWatcherBase,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QFutureWatcherBase {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QFutureWatcherBase {
    return QFutureWatcherBase{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QFutureWatcherBase {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QFutureWatcherBase {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  bool QFutureWatcherBase::isRunning();
impl /*struct*/ QFutureWatcherBase {
  pub fn isRunning<RetType, T: QFutureWatcherBase_isRunning<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isRunning(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_isRunning<RetType> {
  fn isRunning(self , rsthis: & QFutureWatcherBase) -> RetType;
}

  // proto:  bool QFutureWatcherBase::isRunning();
impl<'a> /*trait*/ QFutureWatcherBase_isRunning<i8> for () {
  fn isRunning(self , rsthis: & QFutureWatcherBase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFutureWatcherBase9isRunningEv()};
    let mut ret = unsafe {C_ZNK18QFutureWatcherBase9isRunningEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QFutureWatcherBase::setPaused(bool paused);
impl /*struct*/ QFutureWatcherBase {
  pub fn setPaused<RetType, T: QFutureWatcherBase_setPaused<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPaused(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_setPaused<RetType> {
  fn setPaused(self , rsthis: & QFutureWatcherBase) -> RetType;
}

  // proto:  void QFutureWatcherBase::setPaused(bool paused);
impl<'a> /*trait*/ QFutureWatcherBase_setPaused<()> for (i8) {
  fn setPaused(self , rsthis: & QFutureWatcherBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase9setPausedEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN18QFutureWatcherBase9setPausedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QFutureWatcherBase::progressMinimum();
impl /*struct*/ QFutureWatcherBase {
  pub fn progressMinimum<RetType, T: QFutureWatcherBase_progressMinimum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.progressMinimum(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_progressMinimum<RetType> {
  fn progressMinimum(self , rsthis: & QFutureWatcherBase) -> RetType;
}

  // proto:  int QFutureWatcherBase::progressMinimum();
impl<'a> /*trait*/ QFutureWatcherBase_progressMinimum<i32> for () {
  fn progressMinimum(self , rsthis: & QFutureWatcherBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFutureWatcherBase15progressMinimumEv()};
    let mut ret = unsafe {C_ZNK18QFutureWatcherBase15progressMinimumEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QFutureWatcherBase::resume();
impl /*struct*/ QFutureWatcherBase {
  pub fn resume<RetType, T: QFutureWatcherBase_resume<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resume(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_resume<RetType> {
  fn resume(self , rsthis: & QFutureWatcherBase) -> RetType;
}

  // proto:  void QFutureWatcherBase::resume();
impl<'a> /*trait*/ QFutureWatcherBase_resume<()> for () {
  fn resume(self , rsthis: & QFutureWatcherBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase6resumeEv()};
     unsafe {C_ZN18QFutureWatcherBase6resumeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QFutureWatcherBase::metaObject();
impl /*struct*/ QFutureWatcherBase {
  pub fn metaObject<RetType, T: QFutureWatcherBase_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_metaObject<RetType> {
  fn metaObject(self , rsthis: & QFutureWatcherBase) -> RetType;
}

  // proto:  const QMetaObject * QFutureWatcherBase::metaObject();
impl<'a> /*trait*/ QFutureWatcherBase_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QFutureWatcherBase) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFutureWatcherBase10metaObjectEv()};
    let mut ret = unsafe {C_ZNK18QFutureWatcherBase10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QFutureWatcherBase::isFinished();
impl /*struct*/ QFutureWatcherBase {
  pub fn isFinished<RetType, T: QFutureWatcherBase_isFinished<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isFinished(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_isFinished<RetType> {
  fn isFinished(self , rsthis: & QFutureWatcherBase) -> RetType;
}

  // proto:  bool QFutureWatcherBase::isFinished();
impl<'a> /*trait*/ QFutureWatcherBase_isFinished<i8> for () {
  fn isFinished(self , rsthis: & QFutureWatcherBase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFutureWatcherBase10isFinishedEv()};
    let mut ret = unsafe {C_ZNK18QFutureWatcherBase10isFinishedEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  int QFutureWatcherBase::progressMaximum();
impl /*struct*/ QFutureWatcherBase {
  pub fn progressMaximum<RetType, T: QFutureWatcherBase_progressMaximum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.progressMaximum(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_progressMaximum<RetType> {
  fn progressMaximum(self , rsthis: & QFutureWatcherBase) -> RetType;
}

  // proto:  int QFutureWatcherBase::progressMaximum();
impl<'a> /*trait*/ QFutureWatcherBase_progressMaximum<i32> for () {
  fn progressMaximum(self , rsthis: & QFutureWatcherBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFutureWatcherBase15progressMaximumEv()};
    let mut ret = unsafe {C_ZNK18QFutureWatcherBase15progressMaximumEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  bool QFutureWatcherBase::event(QEvent * event);
impl /*struct*/ QFutureWatcherBase {
  pub fn event<RetType, T: QFutureWatcherBase_event<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.event(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_event<RetType> {
  fn event(self , rsthis: & QFutureWatcherBase) -> RetType;
}

  // proto:  bool QFutureWatcherBase::event(QEvent * event);
impl<'a> /*trait*/ QFutureWatcherBase_event<i8> for (&'a QEvent) {
  fn event(self , rsthis: & QFutureWatcherBase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase5eventEP6QEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN18QFutureWatcherBase5eventEP6QEvent(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QFutureWatcherBase::isCanceled();
impl /*struct*/ QFutureWatcherBase {
  pub fn isCanceled<RetType, T: QFutureWatcherBase_isCanceled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isCanceled(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_isCanceled<RetType> {
  fn isCanceled(self , rsthis: & QFutureWatcherBase) -> RetType;
}

  // proto:  bool QFutureWatcherBase::isCanceled();
impl<'a> /*trait*/ QFutureWatcherBase_isCanceled<i8> for () {
  fn isCanceled(self , rsthis: & QFutureWatcherBase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFutureWatcherBase10isCanceledEv()};
    let mut ret = unsafe {C_ZNK18QFutureWatcherBase10isCanceledEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  int QFutureWatcherBase::progressValue();
impl /*struct*/ QFutureWatcherBase {
  pub fn progressValue<RetType, T: QFutureWatcherBase_progressValue<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.progressValue(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_progressValue<RetType> {
  fn progressValue(self , rsthis: & QFutureWatcherBase) -> RetType;
}

  // proto:  int QFutureWatcherBase::progressValue();
impl<'a> /*trait*/ QFutureWatcherBase_progressValue<i32> for () {
  fn progressValue(self , rsthis: & QFutureWatcherBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFutureWatcherBase13progressValueEv()};
    let mut ret = unsafe {C_ZNK18QFutureWatcherBase13progressValueEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  bool QFutureWatcherBase::isStarted();
impl /*struct*/ QFutureWatcherBase {
  pub fn isStarted<RetType, T: QFutureWatcherBase_isStarted<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isStarted(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_isStarted<RetType> {
  fn isStarted(self , rsthis: & QFutureWatcherBase) -> RetType;
}

  // proto:  bool QFutureWatcherBase::isStarted();
impl<'a> /*trait*/ QFutureWatcherBase_isStarted<i8> for () {
  fn isStarted(self , rsthis: & QFutureWatcherBase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFutureWatcherBase9isStartedEv()};
    let mut ret = unsafe {C_ZNK18QFutureWatcherBase9isStartedEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QFutureWatcherBase::setPendingResultsLimit(int limit);
impl /*struct*/ QFutureWatcherBase {
  pub fn setPendingResultsLimit<RetType, T: QFutureWatcherBase_setPendingResultsLimit<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPendingResultsLimit(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_setPendingResultsLimit<RetType> {
  fn setPendingResultsLimit(self , rsthis: & QFutureWatcherBase) -> RetType;
}

  // proto:  void QFutureWatcherBase::setPendingResultsLimit(int limit);
impl<'a> /*trait*/ QFutureWatcherBase_setPendingResultsLimit<()> for (i32) {
  fn setPendingResultsLimit(self , rsthis: & QFutureWatcherBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase22setPendingResultsLimitEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN18QFutureWatcherBase22setPendingResultsLimitEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QFutureWatcherBase::cancel();
impl /*struct*/ QFutureWatcherBase {
  pub fn cancel<RetType, T: QFutureWatcherBase_cancel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cancel(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_cancel<RetType> {
  fn cancel(self , rsthis: & QFutureWatcherBase) -> RetType;
}

  // proto:  void QFutureWatcherBase::cancel();
impl<'a> /*trait*/ QFutureWatcherBase_cancel<()> for () {
  fn cancel(self , rsthis: & QFutureWatcherBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase6cancelEv()};
     unsafe {C_ZN18QFutureWatcherBase6cancelEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QFutureWatcherBase::isPaused();
impl /*struct*/ QFutureWatcherBase {
  pub fn isPaused<RetType, T: QFutureWatcherBase_isPaused<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isPaused(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_isPaused<RetType> {
  fn isPaused(self , rsthis: & QFutureWatcherBase) -> RetType;
}

  // proto:  bool QFutureWatcherBase::isPaused();
impl<'a> /*trait*/ QFutureWatcherBase_isPaused<i8> for () {
  fn isPaused(self , rsthis: & QFutureWatcherBase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFutureWatcherBase8isPausedEv()};
    let mut ret = unsafe {C_ZNK18QFutureWatcherBase8isPausedEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QFutureWatcherBase::pause();
impl /*struct*/ QFutureWatcherBase {
  pub fn pause<RetType, T: QFutureWatcherBase_pause<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pause(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_pause<RetType> {
  fn pause(self , rsthis: & QFutureWatcherBase) -> RetType;
}

  // proto:  void QFutureWatcherBase::pause();
impl<'a> /*trait*/ QFutureWatcherBase_pause<()> for () {
  fn pause(self , rsthis: & QFutureWatcherBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase5pauseEv()};
     unsafe {C_ZN18QFutureWatcherBase5pauseEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QFutureWatcherBase::progressText();
impl /*struct*/ QFutureWatcherBase {
  pub fn progressText<RetType, T: QFutureWatcherBase_progressText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.progressText(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_progressText<RetType> {
  fn progressText(self , rsthis: & QFutureWatcherBase) -> RetType;
}

  // proto:  QString QFutureWatcherBase::progressText();
impl<'a> /*trait*/ QFutureWatcherBase_progressText<QString> for () {
  fn progressText(self , rsthis: & QFutureWatcherBase) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFutureWatcherBase12progressTextEv()};
    let mut ret = unsafe {C_ZNK18QFutureWatcherBase12progressTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QFutureWatcherBase::QFutureWatcherBase(QObject * parent);
impl /*struct*/ QFutureWatcherBase {
  pub fn new<T: QFutureWatcherBase_new>(value: T) -> QFutureWatcherBase {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QFutureWatcherBase_new {
  fn new(self) -> QFutureWatcherBase;
}

  // proto:  void QFutureWatcherBase::QFutureWatcherBase(QObject * parent);
impl<'a> /*trait*/ QFutureWatcherBase_new for (&'a QObject) {
  fn new(self) -> QFutureWatcherBase {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBaseC2EP7QObject()};
    let ctysz: c_int = unsafe{QFutureWatcherBase_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN18QFutureWatcherBaseC2EP7QObject(arg0)};
    let rsthis = QFutureWatcherBase{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QFutureWatcherBase::togglePaused();
impl /*struct*/ QFutureWatcherBase {
  pub fn togglePaused<RetType, T: QFutureWatcherBase_togglePaused<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.togglePaused(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_togglePaused<RetType> {
  fn togglePaused(self , rsthis: & QFutureWatcherBase) -> RetType;
}

  // proto:  void QFutureWatcherBase::togglePaused();
impl<'a> /*trait*/ QFutureWatcherBase_togglePaused<()> for () {
  fn togglePaused(self , rsthis: & QFutureWatcherBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase12togglePausedEv()};
     unsafe {C_ZN18QFutureWatcherBase12togglePausedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QFutureWatcherBase::waitForFinished();
impl /*struct*/ QFutureWatcherBase {
  pub fn waitForFinished<RetType, T: QFutureWatcherBase_waitForFinished<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.waitForFinished(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_waitForFinished<RetType> {
  fn waitForFinished(self , rsthis: & QFutureWatcherBase) -> RetType;
}

  // proto:  void QFutureWatcherBase::waitForFinished();
impl<'a> /*trait*/ QFutureWatcherBase_waitForFinished<()> for () {
  fn waitForFinished(self , rsthis: & QFutureWatcherBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase15waitForFinishedEv()};
     unsafe {C_ZN18QFutureWatcherBase15waitForFinishedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFutureWatcherLvoidG {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QFutureWatcherLvoidG {
    return QFutureWatcherLvoidG{qbase: QFutureWatcherBase::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QFutureWatcherLvoidG {
  type Target = QFutureWatcherBase;

  fn deref(&self) -> &QFutureWatcherBase {
    return & self.qbase;
  }
}
impl AsRef<QFutureWatcherBase> for QFutureWatcherLvoidG {
  fn as_ref(& self) -> & QFutureWatcherBase {
    return & self.qbase;
  }
}
  // proto:  void QFutureWatcher<void>::QFutureWatcher(QObject * _parent);
impl /*struct*/ QFutureWatcherLvoidG {
  pub fn new<T: QFutureWatcherLvoidG_new>(value: T) -> QFutureWatcherLvoidG {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QFutureWatcherLvoidG_new {
  fn new(self) -> QFutureWatcherLvoidG;
}

  // proto:  void QFutureWatcher<void>::QFutureWatcher(QObject * _parent);
impl<'a> /*trait*/ QFutureWatcherLvoidG_new for (&'a QObject) {
  fn new(self) -> QFutureWatcherLvoidG {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QFutureWatcherIvEC2EP7QObject()};
    let ctysz: c_int = unsafe{QFutureWatcherLvoidG_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN14QFutureWatcherIvEC2EP7QObject(arg0)};
    let rsthis = QFutureWatcherLvoidG{qbase: QFutureWatcherBase::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QFuture<void> QFutureWatcher<void>::future();
impl /*struct*/ QFutureWatcherLvoidG {
  pub fn future<RetType, T: QFutureWatcherLvoidG_future<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.future(self);
    // return 1;
  }
}

pub trait QFutureWatcherLvoidG_future<RetType> {
  fn future(self , rsthis: & QFutureWatcherLvoidG) -> RetType;
}

  // proto:  QFuture<void> QFutureWatcher<void>::future();
impl<'a> /*trait*/ QFutureWatcherLvoidG_future<QFutureLvoidG> for () {
  fn future(self , rsthis: & QFutureWatcherLvoidG) -> QFutureLvoidG {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QFutureWatcherIvE6futureEv()};
    let mut ret = unsafe {C_ZNK14QFutureWatcherIvE6futureEv(rsthis.qclsinst)};
    let mut ret1 = QFutureLvoidG::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QFutureWatcher<void>::~QFutureWatcher();
impl /*struct*/ QFutureWatcherLvoidG {
  pub fn free<RetType, T: QFutureWatcherLvoidG_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QFutureWatcherLvoidG_free<RetType> {
  fn free(self , rsthis: & QFutureWatcherLvoidG) -> RetType;
}

  // proto:  void QFutureWatcher<void>::~QFutureWatcher();
impl<'a> /*trait*/ QFutureWatcherLvoidG_free<()> for () {
  fn free(self , rsthis: & QFutureWatcherLvoidG) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QFutureWatcherIvED2Ev()};
     unsafe {C_ZN14QFutureWatcherIvED2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

#[derive(Default)] // for QFutureWatcherBase_progressRangeChanged
pub struct QFutureWatcherBase_progressRangeChanged_signal{poi:u64}
impl /* struct */ QFutureWatcherBase {
  pub fn progressRangeChanged(&self) -> QFutureWatcherBase_progressRangeChanged_signal {
     return QFutureWatcherBase_progressRangeChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QFutureWatcherBase_progressRangeChanged_signal {
  pub fn connect<T: QFutureWatcherBase_progressRangeChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QFutureWatcherBase_progressRangeChanged_signal_connect {
  fn connect(self, sigthis: QFutureWatcherBase_progressRangeChanged_signal);
}

#[derive(Default)] // for QFutureWatcherBase_resumed
pub struct QFutureWatcherBase_resumed_signal{poi:u64}
impl /* struct */ QFutureWatcherBase {
  pub fn resumed(&self) -> QFutureWatcherBase_resumed_signal {
     return QFutureWatcherBase_resumed_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QFutureWatcherBase_resumed_signal {
  pub fn connect<T: QFutureWatcherBase_resumed_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QFutureWatcherBase_resumed_signal_connect {
  fn connect(self, sigthis: QFutureWatcherBase_resumed_signal);
}

#[derive(Default)] // for QFutureWatcherBase_progressValueChanged
pub struct QFutureWatcherBase_progressValueChanged_signal{poi:u64}
impl /* struct */ QFutureWatcherBase {
  pub fn progressValueChanged(&self) -> QFutureWatcherBase_progressValueChanged_signal {
     return QFutureWatcherBase_progressValueChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QFutureWatcherBase_progressValueChanged_signal {
  pub fn connect<T: QFutureWatcherBase_progressValueChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QFutureWatcherBase_progressValueChanged_signal_connect {
  fn connect(self, sigthis: QFutureWatcherBase_progressValueChanged_signal);
}

#[derive(Default)] // for QFutureWatcherBase_started
pub struct QFutureWatcherBase_started_signal{poi:u64}
impl /* struct */ QFutureWatcherBase {
  pub fn started(&self) -> QFutureWatcherBase_started_signal {
     return QFutureWatcherBase_started_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QFutureWatcherBase_started_signal {
  pub fn connect<T: QFutureWatcherBase_started_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QFutureWatcherBase_started_signal_connect {
  fn connect(self, sigthis: QFutureWatcherBase_started_signal);
}

#[derive(Default)] // for QFutureWatcherBase_resultReadyAt
pub struct QFutureWatcherBase_resultReadyAt_signal{poi:u64}
impl /* struct */ QFutureWatcherBase {
  pub fn resultReadyAt(&self) -> QFutureWatcherBase_resultReadyAt_signal {
     return QFutureWatcherBase_resultReadyAt_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QFutureWatcherBase_resultReadyAt_signal {
  pub fn connect<T: QFutureWatcherBase_resultReadyAt_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QFutureWatcherBase_resultReadyAt_signal_connect {
  fn connect(self, sigthis: QFutureWatcherBase_resultReadyAt_signal);
}

#[derive(Default)] // for QFutureWatcherBase_resultsReadyAt
pub struct QFutureWatcherBase_resultsReadyAt_signal{poi:u64}
impl /* struct */ QFutureWatcherBase {
  pub fn resultsReadyAt(&self) -> QFutureWatcherBase_resultsReadyAt_signal {
     return QFutureWatcherBase_resultsReadyAt_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QFutureWatcherBase_resultsReadyAt_signal {
  pub fn connect<T: QFutureWatcherBase_resultsReadyAt_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QFutureWatcherBase_resultsReadyAt_signal_connect {
  fn connect(self, sigthis: QFutureWatcherBase_resultsReadyAt_signal);
}

#[derive(Default)] // for QFutureWatcherBase_paused
pub struct QFutureWatcherBase_paused_signal{poi:u64}
impl /* struct */ QFutureWatcherBase {
  pub fn paused(&self) -> QFutureWatcherBase_paused_signal {
     return QFutureWatcherBase_paused_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QFutureWatcherBase_paused_signal {
  pub fn connect<T: QFutureWatcherBase_paused_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QFutureWatcherBase_paused_signal_connect {
  fn connect(self, sigthis: QFutureWatcherBase_paused_signal);
}

#[derive(Default)] // for QFutureWatcherBase_canceled
pub struct QFutureWatcherBase_canceled_signal{poi:u64}
impl /* struct */ QFutureWatcherBase {
  pub fn canceled(&self) -> QFutureWatcherBase_canceled_signal {
     return QFutureWatcherBase_canceled_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QFutureWatcherBase_canceled_signal {
  pub fn connect<T: QFutureWatcherBase_canceled_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QFutureWatcherBase_canceled_signal_connect {
  fn connect(self, sigthis: QFutureWatcherBase_canceled_signal);
}

#[derive(Default)] // for QFutureWatcherBase_finished
pub struct QFutureWatcherBase_finished_signal{poi:u64}
impl /* struct */ QFutureWatcherBase {
  pub fn finished(&self) -> QFutureWatcherBase_finished_signal {
     return QFutureWatcherBase_finished_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QFutureWatcherBase_finished_signal {
  pub fn connect<T: QFutureWatcherBase_finished_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QFutureWatcherBase_finished_signal_connect {
  fn connect(self, sigthis: QFutureWatcherBase_finished_signal);
}

#[derive(Default)] // for QFutureWatcherBase_progressTextChanged
pub struct QFutureWatcherBase_progressTextChanged_signal{poi:u64}
impl /* struct */ QFutureWatcherBase {
  pub fn progressTextChanged(&self) -> QFutureWatcherBase_progressTextChanged_signal {
     return QFutureWatcherBase_progressTextChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QFutureWatcherBase_progressTextChanged_signal {
  pub fn connect<T: QFutureWatcherBase_progressTextChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QFutureWatcherBase_progressTextChanged_signal_connect {
  fn connect(self, sigthis: QFutureWatcherBase_progressTextChanged_signal);
}

// finished()
extern fn QFutureWatcherBase_finished_signal_connect_cb_0(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QFutureWatcherBase_finished_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QFutureWatcherBase_finished_signal_connect for fn() {
  fn connect(self, sigthis: QFutureWatcherBase_finished_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QFutureWatcherBase_finished_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QFutureWatcherBase_SlotProxy_connect__ZN18QFutureWatcherBase8finishedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QFutureWatcherBase_finished_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QFutureWatcherBase_finished_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QFutureWatcherBase_finished_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QFutureWatcherBase_SlotProxy_connect__ZN18QFutureWatcherBase8finishedEv(arg0, arg1, arg2)};
  }
}
// progressTextChanged(const class QString &)
extern fn QFutureWatcherBase_progressTextChanged_signal_connect_cb_1(rsfptr:fn(QString), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QFutureWatcherBase_progressTextChanged_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn(QString)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QFutureWatcherBase_progressTextChanged_signal_connect for fn(QString) {
  fn connect(self, sigthis: QFutureWatcherBase_progressTextChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QFutureWatcherBase_progressTextChanged_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QFutureWatcherBase_SlotProxy_connect__ZN18QFutureWatcherBase19progressTextChangedERK7QString(arg0, arg1, arg2)};
  }
}
impl /* trait */ QFutureWatcherBase_progressTextChanged_signal_connect for Box<Fn(QString)> {
  fn connect(self, sigthis: QFutureWatcherBase_progressTextChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QFutureWatcherBase_progressTextChanged_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QFutureWatcherBase_SlotProxy_connect__ZN18QFutureWatcherBase19progressTextChangedERK7QString(arg0, arg1, arg2)};
  }
}
// paused()
extern fn QFutureWatcherBase_paused_signal_connect_cb_2(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QFutureWatcherBase_paused_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QFutureWatcherBase_paused_signal_connect for fn() {
  fn connect(self, sigthis: QFutureWatcherBase_paused_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QFutureWatcherBase_paused_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QFutureWatcherBase_SlotProxy_connect__ZN18QFutureWatcherBase6pausedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QFutureWatcherBase_paused_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QFutureWatcherBase_paused_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QFutureWatcherBase_paused_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QFutureWatcherBase_SlotProxy_connect__ZN18QFutureWatcherBase6pausedEv(arg0, arg1, arg2)};
  }
}
// started()
extern fn QFutureWatcherBase_started_signal_connect_cb_3(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QFutureWatcherBase_started_signal_connect_cb_box_3(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QFutureWatcherBase_started_signal_connect for fn() {
  fn connect(self, sigthis: QFutureWatcherBase_started_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QFutureWatcherBase_started_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QFutureWatcherBase_SlotProxy_connect__ZN18QFutureWatcherBase7startedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QFutureWatcherBase_started_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QFutureWatcherBase_started_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QFutureWatcherBase_started_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QFutureWatcherBase_SlotProxy_connect__ZN18QFutureWatcherBase7startedEv(arg0, arg1, arg2)};
  }
}
// resumed()
extern fn QFutureWatcherBase_resumed_signal_connect_cb_4(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QFutureWatcherBase_resumed_signal_connect_cb_box_4(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QFutureWatcherBase_resumed_signal_connect for fn() {
  fn connect(self, sigthis: QFutureWatcherBase_resumed_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QFutureWatcherBase_resumed_signal_connect_cb_4 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QFutureWatcherBase_SlotProxy_connect__ZN18QFutureWatcherBase7resumedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QFutureWatcherBase_resumed_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QFutureWatcherBase_resumed_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QFutureWatcherBase_resumed_signal_connect_cb_box_4 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QFutureWatcherBase_SlotProxy_connect__ZN18QFutureWatcherBase7resumedEv(arg0, arg1, arg2)};
  }
}
// progressRangeChanged(int, int)
extern fn QFutureWatcherBase_progressRangeChanged_signal_connect_cb_5(rsfptr:fn(i32, i32), arg0: c_int, arg1: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  let rsarg1 = arg1 as i32;
  rsfptr(rsarg0,rsarg1);
}
extern fn QFutureWatcherBase_progressRangeChanged_signal_connect_cb_box_5(rsfptr_raw:*mut Box<Fn(i32, i32)>, arg0: c_int, arg1: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  let rsarg1 = arg1 as i32;
  // rsfptr(rsarg0,rsarg1);
  unsafe{(*rsfptr_raw)(rsarg0,rsarg1)};
}
impl /* trait */ QFutureWatcherBase_progressRangeChanged_signal_connect for fn(i32, i32) {
  fn connect(self, sigthis: QFutureWatcherBase_progressRangeChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QFutureWatcherBase_progressRangeChanged_signal_connect_cb_5 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QFutureWatcherBase_SlotProxy_connect__ZN18QFutureWatcherBase20progressRangeChangedEii(arg0, arg1, arg2)};
  }
}
impl /* trait */ QFutureWatcherBase_progressRangeChanged_signal_connect for Box<Fn(i32, i32)> {
  fn connect(self, sigthis: QFutureWatcherBase_progressRangeChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QFutureWatcherBase_progressRangeChanged_signal_connect_cb_box_5 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QFutureWatcherBase_SlotProxy_connect__ZN18QFutureWatcherBase20progressRangeChangedEii(arg0, arg1, arg2)};
  }
}
// canceled()
extern fn QFutureWatcherBase_canceled_signal_connect_cb_6(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QFutureWatcherBase_canceled_signal_connect_cb_box_6(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QFutureWatcherBase_canceled_signal_connect for fn() {
  fn connect(self, sigthis: QFutureWatcherBase_canceled_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QFutureWatcherBase_canceled_signal_connect_cb_6 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QFutureWatcherBase_SlotProxy_connect__ZN18QFutureWatcherBase8canceledEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QFutureWatcherBase_canceled_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QFutureWatcherBase_canceled_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QFutureWatcherBase_canceled_signal_connect_cb_box_6 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QFutureWatcherBase_SlotProxy_connect__ZN18QFutureWatcherBase8canceledEv(arg0, arg1, arg2)};
  }
}
// progressValueChanged(int)
extern fn QFutureWatcherBase_progressValueChanged_signal_connect_cb_7(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QFutureWatcherBase_progressValueChanged_signal_connect_cb_box_7(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QFutureWatcherBase_progressValueChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QFutureWatcherBase_progressValueChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QFutureWatcherBase_progressValueChanged_signal_connect_cb_7 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QFutureWatcherBase_SlotProxy_connect__ZN18QFutureWatcherBase20progressValueChangedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QFutureWatcherBase_progressValueChanged_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QFutureWatcherBase_progressValueChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QFutureWatcherBase_progressValueChanged_signal_connect_cb_box_7 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QFutureWatcherBase_SlotProxy_connect__ZN18QFutureWatcherBase20progressValueChangedEi(arg0, arg1, arg2)};
  }
}
// resultReadyAt(int)
extern fn QFutureWatcherBase_resultReadyAt_signal_connect_cb_8(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QFutureWatcherBase_resultReadyAt_signal_connect_cb_box_8(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QFutureWatcherBase_resultReadyAt_signal_connect for fn(i32) {
  fn connect(self, sigthis: QFutureWatcherBase_resultReadyAt_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QFutureWatcherBase_resultReadyAt_signal_connect_cb_8 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QFutureWatcherBase_SlotProxy_connect__ZN18QFutureWatcherBase13resultReadyAtEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QFutureWatcherBase_resultReadyAt_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QFutureWatcherBase_resultReadyAt_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QFutureWatcherBase_resultReadyAt_signal_connect_cb_box_8 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QFutureWatcherBase_SlotProxy_connect__ZN18QFutureWatcherBase13resultReadyAtEi(arg0, arg1, arg2)};
  }
}
// resultsReadyAt(int, int)
extern fn QFutureWatcherBase_resultsReadyAt_signal_connect_cb_9(rsfptr:fn(i32, i32), arg0: c_int, arg1: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  let rsarg1 = arg1 as i32;
  rsfptr(rsarg0,rsarg1);
}
extern fn QFutureWatcherBase_resultsReadyAt_signal_connect_cb_box_9(rsfptr_raw:*mut Box<Fn(i32, i32)>, arg0: c_int, arg1: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  let rsarg1 = arg1 as i32;
  // rsfptr(rsarg0,rsarg1);
  unsafe{(*rsfptr_raw)(rsarg0,rsarg1)};
}
impl /* trait */ QFutureWatcherBase_resultsReadyAt_signal_connect for fn(i32, i32) {
  fn connect(self, sigthis: QFutureWatcherBase_resultsReadyAt_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QFutureWatcherBase_resultsReadyAt_signal_connect_cb_9 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QFutureWatcherBase_SlotProxy_connect__ZN18QFutureWatcherBase14resultsReadyAtEii(arg0, arg1, arg2)};
  }
}
impl /* trait */ QFutureWatcherBase_resultsReadyAt_signal_connect for Box<Fn(i32, i32)> {
  fn connect(self, sigthis: QFutureWatcherBase_resultsReadyAt_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QFutureWatcherBase_resultsReadyAt_signal_connect_cb_box_9 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QFutureWatcherBase_SlotProxy_connect__ZN18QFutureWatcherBase14resultsReadyAtEii(arg0, arg1, arg2)};
  }
}
// <= body block end

