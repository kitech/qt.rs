// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtCore/qfutureinterface.h
// dst-file: /src/core/qfutureinterface.rs
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
use super::qmutex::*; // 773
use super::qrunnable::*; // 773
use super::qstring::*; // 773
use super::qthreadpool::*; // 773
// use super::qfutureinterface::QFutureInterfaceBase; // 773
// use super::qfutureinterface::*; // 775
// use super::qfutureinterface::QFutureInterface; // 773
// use super::qfuture::*; // 775
use super::qfuture::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QFutureInterfaceBase_Class_Size() -> c_int;
  // proto:  int QFutureInterfaceBase::progressMinimum();
  fn C_ZNK20QFutureInterfaceBase15progressMinimumEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QFutureInterfaceBase::isStarted();
  fn C_ZNK20QFutureInterfaceBase9isStartedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QMutex * QFutureInterfaceBase::mutex();
  fn C_ZNK20QFutureInterfaceBase5mutexEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QFutureInterfaceBase::isResultReadyAt(int index);
  fn C_ZNK20QFutureInterfaceBase15isResultReadyAtEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  void QFutureInterfaceBase::setPaused(bool paused);
  fn C_ZN20QFutureInterfaceBase9setPausedEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  int QFutureInterfaceBase::expectedResultCount();
  fn C_ZN20QFutureInterfaceBase19expectedResultCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QFutureInterfaceBase::waitForFinished();
  fn C_ZN20QFutureInterfaceBase15waitForFinishedEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QFutureInterfaceBase::isRunning();
  fn C_ZNK20QFutureInterfaceBase9isRunningEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QFutureInterfaceBase::~QFutureInterfaceBase();
  fn C_ZN20QFutureInterfaceBaseD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QFutureInterfaceBase::cancel();
  fn C_ZN20QFutureInterfaceBase6cancelEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QFutureInterfaceBase::reportStarted();
  fn C_ZN20QFutureInterfaceBase13reportStartedEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QFutureInterfaceBase::setRunnable(QRunnable * runnable);
  fn C_ZN20QFutureInterfaceBase11setRunnableEP9QRunnable(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QFutureInterfaceBase::isCanceled();
  fn C_ZNK20QFutureInterfaceBase10isCanceledEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QString QFutureInterfaceBase::progressText();
  fn C_ZNK20QFutureInterfaceBase12progressTextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QFutureInterfaceBase::isProgressUpdateNeeded();
  fn C_ZNK20QFutureInterfaceBase22isProgressUpdateNeededEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QFutureInterfaceBase::setExpectedResultCount(int resultCount);
  fn C_ZN20QFutureInterfaceBase22setExpectedResultCountEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QFutureInterfaceBase::reportResultsReady(int beginIndex, int endIndex);
  fn C_ZN20QFutureInterfaceBase18reportResultsReadyEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  void QFutureInterfaceBase::reportCanceled();
  fn C_ZN20QFutureInterfaceBase14reportCanceledEv(qthis: u64 /* *mut c_void*/);
  // proto:  int QFutureInterfaceBase::resultCount();
  fn C_ZNK20QFutureInterfaceBase11resultCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QFutureInterfaceBase::QFutureInterfaceBase(const QFutureInterfaceBase & other);
  fn C_ZN20QFutureInterfaceBaseC2ERKS_(arg0: *mut c_void) -> u64;
  // proto:  int QFutureInterfaceBase::progressValue();
  fn C_ZNK20QFutureInterfaceBase13progressValueEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QFutureInterfaceBase::isThrottled();
  fn C_ZNK20QFutureInterfaceBase11isThrottledEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QFutureInterfaceBase::setProgressRange(int minimum, int maximum);
  fn C_ZN20QFutureInterfaceBase16setProgressRangeEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  void QFutureInterfaceBase::setThrottled(bool enable);
  fn C_ZN20QFutureInterfaceBase12setThrottledEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QFutureInterfaceBase::setProgressValueAndText(int progressValue, const QString & progressText);
  fn C_ZN20QFutureInterfaceBase23setProgressValueAndTextEiRK7QString(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  void QFutureInterfaceBase::togglePaused();
  fn C_ZN20QFutureInterfaceBase12togglePausedEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QFutureInterfaceBase::waitForResult(int resultIndex);
  fn C_ZN20QFutureInterfaceBase13waitForResultEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  bool QFutureInterfaceBase::isPaused();
  fn C_ZNK20QFutureInterfaceBase8isPausedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QFutureInterfaceBase::waitForNextResult();
  fn C_ZN20QFutureInterfaceBase17waitForNextResultEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QFutureInterfaceBase::reportFinished();
  fn C_ZN20QFutureInterfaceBase14reportFinishedEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QFutureInterfaceBase::setFilterMode(bool enable);
  fn C_ZN20QFutureInterfaceBase13setFilterModeEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  int QFutureInterfaceBase::progressMaximum();
  fn C_ZNK20QFutureInterfaceBase15progressMaximumEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QFutureInterfaceBase::setThreadPool(QThreadPool * pool);
  fn C_ZN20QFutureInterfaceBase13setThreadPoolEP11QThreadPool(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QFutureInterfaceBase::waitForResume();
  fn C_ZN20QFutureInterfaceBase13waitForResumeEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QFutureInterfaceBase::setProgressValue(int progressValue);
  fn C_ZN20QFutureInterfaceBase16setProgressValueEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  bool QFutureInterfaceBase::isFinished();
  fn C_ZNK20QFutureInterfaceBase10isFinishedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  fn QFutureInterfaceLvoidG_Class_Size() -> c_int;
  // proto: static QFutureInterface<void> QFutureInterface<void>::canceledResult();
  fn C_ZN16QFutureInterfaceIvE14canceledResultEv() -> *mut c_void;
  // proto:  QFuture<void> QFutureInterface<void>::future();
  fn C_ZN16QFutureInterfaceIvE6futureEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QFutureInterface<void>::reportResult(const void * , int );
  fn C_ZN16QFutureInterfaceIvE12reportResultEPKvi(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int);
  // proto:  void QFutureInterface<void>::reportFinished(const void * );
  fn C_ZN16QFutureInterfaceIvE14reportFinishedEPKv(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QFutureInterfaceBase)=16
#[derive(Default)]
pub struct QFutureInterfaceBase {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QFutureInterfaceLvoidG)=16
#[derive(Default)]
pub struct QFutureInterfaceLvoidG {
  qbase: QFutureInterfaceBase,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QFutureInterfaceBase {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QFutureInterfaceBase {
    return QFutureInterfaceBase{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  int QFutureInterfaceBase::progressMinimum();
impl /*struct*/ QFutureInterfaceBase {
  pub fn progressMinimum<RetType, T: QFutureInterfaceBase_progressMinimum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.progressMinimum(self);
    // return 1;
  }
}

pub trait QFutureInterfaceBase_progressMinimum<RetType> {
  fn progressMinimum(self , rsthis: & QFutureInterfaceBase) -> RetType;
}

  // proto:  int QFutureInterfaceBase::progressMinimum();
impl<'a> /*trait*/ QFutureInterfaceBase_progressMinimum<i32> for () {
  fn progressMinimum(self , rsthis: & QFutureInterfaceBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QFutureInterfaceBase15progressMinimumEv()};
    let mut ret = unsafe {C_ZNK20QFutureInterfaceBase15progressMinimumEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  bool QFutureInterfaceBase::isStarted();
impl /*struct*/ QFutureInterfaceBase {
  pub fn isStarted<RetType, T: QFutureInterfaceBase_isStarted<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isStarted(self);
    // return 1;
  }
}

pub trait QFutureInterfaceBase_isStarted<RetType> {
  fn isStarted(self , rsthis: & QFutureInterfaceBase) -> RetType;
}

  // proto:  bool QFutureInterfaceBase::isStarted();
impl<'a> /*trait*/ QFutureInterfaceBase_isStarted<i8> for () {
  fn isStarted(self , rsthis: & QFutureInterfaceBase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QFutureInterfaceBase9isStartedEv()};
    let mut ret = unsafe {C_ZNK20QFutureInterfaceBase9isStartedEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QMutex * QFutureInterfaceBase::mutex();
impl /*struct*/ QFutureInterfaceBase {
  pub fn mutex<RetType, T: QFutureInterfaceBase_mutex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mutex(self);
    // return 1;
  }
}

pub trait QFutureInterfaceBase_mutex<RetType> {
  fn mutex(self , rsthis: & QFutureInterfaceBase) -> RetType;
}

  // proto:  QMutex * QFutureInterfaceBase::mutex();
impl<'a> /*trait*/ QFutureInterfaceBase_mutex<QMutex> for () {
  fn mutex(self , rsthis: & QFutureInterfaceBase) -> QMutex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QFutureInterfaceBase5mutexEv()};
    let mut ret = unsafe {C_ZNK20QFutureInterfaceBase5mutexEv(rsthis.qclsinst)};
    let mut ret1 = QMutex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QFutureInterfaceBase::isResultReadyAt(int index);
impl /*struct*/ QFutureInterfaceBase {
  pub fn isResultReadyAt<RetType, T: QFutureInterfaceBase_isResultReadyAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isResultReadyAt(self);
    // return 1;
  }
}

pub trait QFutureInterfaceBase_isResultReadyAt<RetType> {
  fn isResultReadyAt(self , rsthis: & QFutureInterfaceBase) -> RetType;
}

  // proto:  bool QFutureInterfaceBase::isResultReadyAt(int index);
impl<'a> /*trait*/ QFutureInterfaceBase_isResultReadyAt<i8> for (i32) {
  fn isResultReadyAt(self , rsthis: & QFutureInterfaceBase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QFutureInterfaceBase15isResultReadyAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK20QFutureInterfaceBase15isResultReadyAtEi(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QFutureInterfaceBase::setPaused(bool paused);
impl /*struct*/ QFutureInterfaceBase {
  pub fn setPaused<RetType, T: QFutureInterfaceBase_setPaused<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPaused(self);
    // return 1;
  }
}

pub trait QFutureInterfaceBase_setPaused<RetType> {
  fn setPaused(self , rsthis: & QFutureInterfaceBase) -> RetType;
}

  // proto:  void QFutureInterfaceBase::setPaused(bool paused);
impl<'a> /*trait*/ QFutureInterfaceBase_setPaused<()> for (i8) {
  fn setPaused(self , rsthis: & QFutureInterfaceBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QFutureInterfaceBase9setPausedEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN20QFutureInterfaceBase9setPausedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QFutureInterfaceBase::expectedResultCount();
impl /*struct*/ QFutureInterfaceBase {
  pub fn expectedResultCount<RetType, T: QFutureInterfaceBase_expectedResultCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.expectedResultCount(self);
    // return 1;
  }
}

pub trait QFutureInterfaceBase_expectedResultCount<RetType> {
  fn expectedResultCount(self , rsthis: & QFutureInterfaceBase) -> RetType;
}

  // proto:  int QFutureInterfaceBase::expectedResultCount();
impl<'a> /*trait*/ QFutureInterfaceBase_expectedResultCount<i32> for () {
  fn expectedResultCount(self , rsthis: & QFutureInterfaceBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QFutureInterfaceBase19expectedResultCountEv()};
    let mut ret = unsafe {C_ZN20QFutureInterfaceBase19expectedResultCountEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QFutureInterfaceBase::waitForFinished();
impl /*struct*/ QFutureInterfaceBase {
  pub fn waitForFinished<RetType, T: QFutureInterfaceBase_waitForFinished<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.waitForFinished(self);
    // return 1;
  }
}

pub trait QFutureInterfaceBase_waitForFinished<RetType> {
  fn waitForFinished(self , rsthis: & QFutureInterfaceBase) -> RetType;
}

  // proto:  void QFutureInterfaceBase::waitForFinished();
impl<'a> /*trait*/ QFutureInterfaceBase_waitForFinished<()> for () {
  fn waitForFinished(self , rsthis: & QFutureInterfaceBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QFutureInterfaceBase15waitForFinishedEv()};
     unsafe {C_ZN20QFutureInterfaceBase15waitForFinishedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QFutureInterfaceBase::isRunning();
impl /*struct*/ QFutureInterfaceBase {
  pub fn isRunning<RetType, T: QFutureInterfaceBase_isRunning<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isRunning(self);
    // return 1;
  }
}

pub trait QFutureInterfaceBase_isRunning<RetType> {
  fn isRunning(self , rsthis: & QFutureInterfaceBase) -> RetType;
}

  // proto:  bool QFutureInterfaceBase::isRunning();
impl<'a> /*trait*/ QFutureInterfaceBase_isRunning<i8> for () {
  fn isRunning(self , rsthis: & QFutureInterfaceBase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QFutureInterfaceBase9isRunningEv()};
    let mut ret = unsafe {C_ZNK20QFutureInterfaceBase9isRunningEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QFutureInterfaceBase::~QFutureInterfaceBase();
impl /*struct*/ QFutureInterfaceBase {
  pub fn free<RetType, T: QFutureInterfaceBase_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QFutureInterfaceBase_free<RetType> {
  fn free(self , rsthis: & QFutureInterfaceBase) -> RetType;
}

  // proto:  void QFutureInterfaceBase::~QFutureInterfaceBase();
impl<'a> /*trait*/ QFutureInterfaceBase_free<()> for () {
  fn free(self , rsthis: & QFutureInterfaceBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QFutureInterfaceBaseD2Ev()};
     unsafe {C_ZN20QFutureInterfaceBaseD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QFutureInterfaceBase::cancel();
impl /*struct*/ QFutureInterfaceBase {
  pub fn cancel<RetType, T: QFutureInterfaceBase_cancel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cancel(self);
    // return 1;
  }
}

pub trait QFutureInterfaceBase_cancel<RetType> {
  fn cancel(self , rsthis: & QFutureInterfaceBase) -> RetType;
}

  // proto:  void QFutureInterfaceBase::cancel();
impl<'a> /*trait*/ QFutureInterfaceBase_cancel<()> for () {
  fn cancel(self , rsthis: & QFutureInterfaceBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QFutureInterfaceBase6cancelEv()};
     unsafe {C_ZN20QFutureInterfaceBase6cancelEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QFutureInterfaceBase::reportStarted();
impl /*struct*/ QFutureInterfaceBase {
  pub fn reportStarted<RetType, T: QFutureInterfaceBase_reportStarted<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.reportStarted(self);
    // return 1;
  }
}

pub trait QFutureInterfaceBase_reportStarted<RetType> {
  fn reportStarted(self , rsthis: & QFutureInterfaceBase) -> RetType;
}

  // proto:  void QFutureInterfaceBase::reportStarted();
impl<'a> /*trait*/ QFutureInterfaceBase_reportStarted<()> for () {
  fn reportStarted(self , rsthis: & QFutureInterfaceBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QFutureInterfaceBase13reportStartedEv()};
     unsafe {C_ZN20QFutureInterfaceBase13reportStartedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QFutureInterfaceBase::setRunnable(QRunnable * runnable);
impl /*struct*/ QFutureInterfaceBase {
  pub fn setRunnable<RetType, T: QFutureInterfaceBase_setRunnable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRunnable(self);
    // return 1;
  }
}

pub trait QFutureInterfaceBase_setRunnable<RetType> {
  fn setRunnable(self , rsthis: & QFutureInterfaceBase) -> RetType;
}

  // proto:  void QFutureInterfaceBase::setRunnable(QRunnable * runnable);
impl<'a> /*trait*/ QFutureInterfaceBase_setRunnable<()> for (&'a QRunnable) {
  fn setRunnable(self , rsthis: & QFutureInterfaceBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QFutureInterfaceBase11setRunnableEP9QRunnable()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN20QFutureInterfaceBase11setRunnableEP9QRunnable(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QFutureInterfaceBase::isCanceled();
impl /*struct*/ QFutureInterfaceBase {
  pub fn isCanceled<RetType, T: QFutureInterfaceBase_isCanceled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isCanceled(self);
    // return 1;
  }
}

pub trait QFutureInterfaceBase_isCanceled<RetType> {
  fn isCanceled(self , rsthis: & QFutureInterfaceBase) -> RetType;
}

  // proto:  bool QFutureInterfaceBase::isCanceled();
impl<'a> /*trait*/ QFutureInterfaceBase_isCanceled<i8> for () {
  fn isCanceled(self , rsthis: & QFutureInterfaceBase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QFutureInterfaceBase10isCanceledEv()};
    let mut ret = unsafe {C_ZNK20QFutureInterfaceBase10isCanceledEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QString QFutureInterfaceBase::progressText();
impl /*struct*/ QFutureInterfaceBase {
  pub fn progressText<RetType, T: QFutureInterfaceBase_progressText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.progressText(self);
    // return 1;
  }
}

pub trait QFutureInterfaceBase_progressText<RetType> {
  fn progressText(self , rsthis: & QFutureInterfaceBase) -> RetType;
}

  // proto:  QString QFutureInterfaceBase::progressText();
impl<'a> /*trait*/ QFutureInterfaceBase_progressText<QString> for () {
  fn progressText(self , rsthis: & QFutureInterfaceBase) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QFutureInterfaceBase12progressTextEv()};
    let mut ret = unsafe {C_ZNK20QFutureInterfaceBase12progressTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QFutureInterfaceBase::isProgressUpdateNeeded();
impl /*struct*/ QFutureInterfaceBase {
  pub fn isProgressUpdateNeeded<RetType, T: QFutureInterfaceBase_isProgressUpdateNeeded<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isProgressUpdateNeeded(self);
    // return 1;
  }
}

pub trait QFutureInterfaceBase_isProgressUpdateNeeded<RetType> {
  fn isProgressUpdateNeeded(self , rsthis: & QFutureInterfaceBase) -> RetType;
}

  // proto:  bool QFutureInterfaceBase::isProgressUpdateNeeded();
impl<'a> /*trait*/ QFutureInterfaceBase_isProgressUpdateNeeded<i8> for () {
  fn isProgressUpdateNeeded(self , rsthis: & QFutureInterfaceBase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QFutureInterfaceBase22isProgressUpdateNeededEv()};
    let mut ret = unsafe {C_ZNK20QFutureInterfaceBase22isProgressUpdateNeededEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QFutureInterfaceBase::setExpectedResultCount(int resultCount);
impl /*struct*/ QFutureInterfaceBase {
  pub fn setExpectedResultCount<RetType, T: QFutureInterfaceBase_setExpectedResultCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setExpectedResultCount(self);
    // return 1;
  }
}

pub trait QFutureInterfaceBase_setExpectedResultCount<RetType> {
  fn setExpectedResultCount(self , rsthis: & QFutureInterfaceBase) -> RetType;
}

  // proto:  void QFutureInterfaceBase::setExpectedResultCount(int resultCount);
impl<'a> /*trait*/ QFutureInterfaceBase_setExpectedResultCount<()> for (i32) {
  fn setExpectedResultCount(self , rsthis: & QFutureInterfaceBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QFutureInterfaceBase22setExpectedResultCountEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN20QFutureInterfaceBase22setExpectedResultCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QFutureInterfaceBase::reportResultsReady(int beginIndex, int endIndex);
impl /*struct*/ QFutureInterfaceBase {
  pub fn reportResultsReady<RetType, T: QFutureInterfaceBase_reportResultsReady<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.reportResultsReady(self);
    // return 1;
  }
}

pub trait QFutureInterfaceBase_reportResultsReady<RetType> {
  fn reportResultsReady(self , rsthis: & QFutureInterfaceBase) -> RetType;
}

  // proto:  void QFutureInterfaceBase::reportResultsReady(int beginIndex, int endIndex);
impl<'a> /*trait*/ QFutureInterfaceBase_reportResultsReady<()> for (i32, i32) {
  fn reportResultsReady(self , rsthis: & QFutureInterfaceBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QFutureInterfaceBase18reportResultsReadyEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN20QFutureInterfaceBase18reportResultsReadyEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QFutureInterfaceBase::reportCanceled();
impl /*struct*/ QFutureInterfaceBase {
  pub fn reportCanceled<RetType, T: QFutureInterfaceBase_reportCanceled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.reportCanceled(self);
    // return 1;
  }
}

pub trait QFutureInterfaceBase_reportCanceled<RetType> {
  fn reportCanceled(self , rsthis: & QFutureInterfaceBase) -> RetType;
}

  // proto:  void QFutureInterfaceBase::reportCanceled();
impl<'a> /*trait*/ QFutureInterfaceBase_reportCanceled<()> for () {
  fn reportCanceled(self , rsthis: & QFutureInterfaceBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QFutureInterfaceBase14reportCanceledEv()};
     unsafe {C_ZN20QFutureInterfaceBase14reportCanceledEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QFutureInterfaceBase::resultCount();
impl /*struct*/ QFutureInterfaceBase {
  pub fn resultCount<RetType, T: QFutureInterfaceBase_resultCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resultCount(self);
    // return 1;
  }
}

pub trait QFutureInterfaceBase_resultCount<RetType> {
  fn resultCount(self , rsthis: & QFutureInterfaceBase) -> RetType;
}

  // proto:  int QFutureInterfaceBase::resultCount();
impl<'a> /*trait*/ QFutureInterfaceBase_resultCount<i32> for () {
  fn resultCount(self , rsthis: & QFutureInterfaceBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QFutureInterfaceBase11resultCountEv()};
    let mut ret = unsafe {C_ZNK20QFutureInterfaceBase11resultCountEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QFutureInterfaceBase::QFutureInterfaceBase(const QFutureInterfaceBase & other);
impl /*struct*/ QFutureInterfaceBase {
  pub fn new<T: QFutureInterfaceBase_new>(value: T) -> QFutureInterfaceBase {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QFutureInterfaceBase_new {
  fn new(self) -> QFutureInterfaceBase;
}

  // proto:  void QFutureInterfaceBase::QFutureInterfaceBase(const QFutureInterfaceBase & other);
impl<'a> /*trait*/ QFutureInterfaceBase_new for (&'a QFutureInterfaceBase) {
  fn new(self) -> QFutureInterfaceBase {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QFutureInterfaceBaseC2ERKS_()};
    let ctysz: c_int = unsafe{QFutureInterfaceBase_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN20QFutureInterfaceBaseC2ERKS_(arg0)};
    let rsthis = QFutureInterfaceBase{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QFutureInterfaceBase::progressValue();
impl /*struct*/ QFutureInterfaceBase {
  pub fn progressValue<RetType, T: QFutureInterfaceBase_progressValue<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.progressValue(self);
    // return 1;
  }
}

pub trait QFutureInterfaceBase_progressValue<RetType> {
  fn progressValue(self , rsthis: & QFutureInterfaceBase) -> RetType;
}

  // proto:  int QFutureInterfaceBase::progressValue();
impl<'a> /*trait*/ QFutureInterfaceBase_progressValue<i32> for () {
  fn progressValue(self , rsthis: & QFutureInterfaceBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QFutureInterfaceBase13progressValueEv()};
    let mut ret = unsafe {C_ZNK20QFutureInterfaceBase13progressValueEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  bool QFutureInterfaceBase::isThrottled();
impl /*struct*/ QFutureInterfaceBase {
  pub fn isThrottled<RetType, T: QFutureInterfaceBase_isThrottled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isThrottled(self);
    // return 1;
  }
}

pub trait QFutureInterfaceBase_isThrottled<RetType> {
  fn isThrottled(self , rsthis: & QFutureInterfaceBase) -> RetType;
}

  // proto:  bool QFutureInterfaceBase::isThrottled();
impl<'a> /*trait*/ QFutureInterfaceBase_isThrottled<i8> for () {
  fn isThrottled(self , rsthis: & QFutureInterfaceBase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QFutureInterfaceBase11isThrottledEv()};
    let mut ret = unsafe {C_ZNK20QFutureInterfaceBase11isThrottledEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QFutureInterfaceBase::setProgressRange(int minimum, int maximum);
impl /*struct*/ QFutureInterfaceBase {
  pub fn setProgressRange<RetType, T: QFutureInterfaceBase_setProgressRange<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setProgressRange(self);
    // return 1;
  }
}

pub trait QFutureInterfaceBase_setProgressRange<RetType> {
  fn setProgressRange(self , rsthis: & QFutureInterfaceBase) -> RetType;
}

  // proto:  void QFutureInterfaceBase::setProgressRange(int minimum, int maximum);
impl<'a> /*trait*/ QFutureInterfaceBase_setProgressRange<()> for (i32, i32) {
  fn setProgressRange(self , rsthis: & QFutureInterfaceBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QFutureInterfaceBase16setProgressRangeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN20QFutureInterfaceBase16setProgressRangeEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QFutureInterfaceBase::setThrottled(bool enable);
impl /*struct*/ QFutureInterfaceBase {
  pub fn setThrottled<RetType, T: QFutureInterfaceBase_setThrottled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setThrottled(self);
    // return 1;
  }
}

pub trait QFutureInterfaceBase_setThrottled<RetType> {
  fn setThrottled(self , rsthis: & QFutureInterfaceBase) -> RetType;
}

  // proto:  void QFutureInterfaceBase::setThrottled(bool enable);
impl<'a> /*trait*/ QFutureInterfaceBase_setThrottled<()> for (i8) {
  fn setThrottled(self , rsthis: & QFutureInterfaceBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QFutureInterfaceBase12setThrottledEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN20QFutureInterfaceBase12setThrottledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QFutureInterfaceBase::setProgressValueAndText(int progressValue, const QString & progressText);
impl /*struct*/ QFutureInterfaceBase {
  pub fn setProgressValueAndText<RetType, T: QFutureInterfaceBase_setProgressValueAndText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setProgressValueAndText(self);
    // return 1;
  }
}

pub trait QFutureInterfaceBase_setProgressValueAndText<RetType> {
  fn setProgressValueAndText(self , rsthis: & QFutureInterfaceBase) -> RetType;
}

  // proto:  void QFutureInterfaceBase::setProgressValueAndText(int progressValue, const QString & progressText);
impl<'a> /*trait*/ QFutureInterfaceBase_setProgressValueAndText<()> for (i32, &'a QString) {
  fn setProgressValueAndText(self , rsthis: & QFutureInterfaceBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QFutureInterfaceBase23setProgressValueAndTextEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN20QFutureInterfaceBase23setProgressValueAndTextEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QFutureInterfaceBase::togglePaused();
impl /*struct*/ QFutureInterfaceBase {
  pub fn togglePaused<RetType, T: QFutureInterfaceBase_togglePaused<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.togglePaused(self);
    // return 1;
  }
}

pub trait QFutureInterfaceBase_togglePaused<RetType> {
  fn togglePaused(self , rsthis: & QFutureInterfaceBase) -> RetType;
}

  // proto:  void QFutureInterfaceBase::togglePaused();
impl<'a> /*trait*/ QFutureInterfaceBase_togglePaused<()> for () {
  fn togglePaused(self , rsthis: & QFutureInterfaceBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QFutureInterfaceBase12togglePausedEv()};
     unsafe {C_ZN20QFutureInterfaceBase12togglePausedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QFutureInterfaceBase::waitForResult(int resultIndex);
impl /*struct*/ QFutureInterfaceBase {
  pub fn waitForResult<RetType, T: QFutureInterfaceBase_waitForResult<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.waitForResult(self);
    // return 1;
  }
}

pub trait QFutureInterfaceBase_waitForResult<RetType> {
  fn waitForResult(self , rsthis: & QFutureInterfaceBase) -> RetType;
}

  // proto:  void QFutureInterfaceBase::waitForResult(int resultIndex);
impl<'a> /*trait*/ QFutureInterfaceBase_waitForResult<()> for (i32) {
  fn waitForResult(self , rsthis: & QFutureInterfaceBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QFutureInterfaceBase13waitForResultEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN20QFutureInterfaceBase13waitForResultEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QFutureInterfaceBase::isPaused();
impl /*struct*/ QFutureInterfaceBase {
  pub fn isPaused<RetType, T: QFutureInterfaceBase_isPaused<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isPaused(self);
    // return 1;
  }
}

pub trait QFutureInterfaceBase_isPaused<RetType> {
  fn isPaused(self , rsthis: & QFutureInterfaceBase) -> RetType;
}

  // proto:  bool QFutureInterfaceBase::isPaused();
impl<'a> /*trait*/ QFutureInterfaceBase_isPaused<i8> for () {
  fn isPaused(self , rsthis: & QFutureInterfaceBase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QFutureInterfaceBase8isPausedEv()};
    let mut ret = unsafe {C_ZNK20QFutureInterfaceBase8isPausedEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QFutureInterfaceBase::waitForNextResult();
impl /*struct*/ QFutureInterfaceBase {
  pub fn waitForNextResult<RetType, T: QFutureInterfaceBase_waitForNextResult<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.waitForNextResult(self);
    // return 1;
  }
}

pub trait QFutureInterfaceBase_waitForNextResult<RetType> {
  fn waitForNextResult(self , rsthis: & QFutureInterfaceBase) -> RetType;
}

  // proto:  bool QFutureInterfaceBase::waitForNextResult();
impl<'a> /*trait*/ QFutureInterfaceBase_waitForNextResult<i8> for () {
  fn waitForNextResult(self , rsthis: & QFutureInterfaceBase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QFutureInterfaceBase17waitForNextResultEv()};
    let mut ret = unsafe {C_ZN20QFutureInterfaceBase17waitForNextResultEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QFutureInterfaceBase::reportFinished();
impl /*struct*/ QFutureInterfaceBase {
  pub fn reportFinished<RetType, T: QFutureInterfaceBase_reportFinished<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.reportFinished(self);
    // return 1;
  }
}

pub trait QFutureInterfaceBase_reportFinished<RetType> {
  fn reportFinished(self , rsthis: & QFutureInterfaceBase) -> RetType;
}

  // proto:  void QFutureInterfaceBase::reportFinished();
impl<'a> /*trait*/ QFutureInterfaceBase_reportFinished<()> for () {
  fn reportFinished(self , rsthis: & QFutureInterfaceBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QFutureInterfaceBase14reportFinishedEv()};
     unsafe {C_ZN20QFutureInterfaceBase14reportFinishedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QFutureInterfaceBase::setFilterMode(bool enable);
impl /*struct*/ QFutureInterfaceBase {
  pub fn setFilterMode<RetType, T: QFutureInterfaceBase_setFilterMode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFilterMode(self);
    // return 1;
  }
}

pub trait QFutureInterfaceBase_setFilterMode<RetType> {
  fn setFilterMode(self , rsthis: & QFutureInterfaceBase) -> RetType;
}

  // proto:  void QFutureInterfaceBase::setFilterMode(bool enable);
impl<'a> /*trait*/ QFutureInterfaceBase_setFilterMode<()> for (i8) {
  fn setFilterMode(self , rsthis: & QFutureInterfaceBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QFutureInterfaceBase13setFilterModeEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN20QFutureInterfaceBase13setFilterModeEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QFutureInterfaceBase::progressMaximum();
impl /*struct*/ QFutureInterfaceBase {
  pub fn progressMaximum<RetType, T: QFutureInterfaceBase_progressMaximum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.progressMaximum(self);
    // return 1;
  }
}

pub trait QFutureInterfaceBase_progressMaximum<RetType> {
  fn progressMaximum(self , rsthis: & QFutureInterfaceBase) -> RetType;
}

  // proto:  int QFutureInterfaceBase::progressMaximum();
impl<'a> /*trait*/ QFutureInterfaceBase_progressMaximum<i32> for () {
  fn progressMaximum(self , rsthis: & QFutureInterfaceBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QFutureInterfaceBase15progressMaximumEv()};
    let mut ret = unsafe {C_ZNK20QFutureInterfaceBase15progressMaximumEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QFutureInterfaceBase::setThreadPool(QThreadPool * pool);
impl /*struct*/ QFutureInterfaceBase {
  pub fn setThreadPool<RetType, T: QFutureInterfaceBase_setThreadPool<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setThreadPool(self);
    // return 1;
  }
}

pub trait QFutureInterfaceBase_setThreadPool<RetType> {
  fn setThreadPool(self , rsthis: & QFutureInterfaceBase) -> RetType;
}

  // proto:  void QFutureInterfaceBase::setThreadPool(QThreadPool * pool);
impl<'a> /*trait*/ QFutureInterfaceBase_setThreadPool<()> for (&'a QThreadPool) {
  fn setThreadPool(self , rsthis: & QFutureInterfaceBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QFutureInterfaceBase13setThreadPoolEP11QThreadPool()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN20QFutureInterfaceBase13setThreadPoolEP11QThreadPool(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QFutureInterfaceBase::waitForResume();
impl /*struct*/ QFutureInterfaceBase {
  pub fn waitForResume<RetType, T: QFutureInterfaceBase_waitForResume<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.waitForResume(self);
    // return 1;
  }
}

pub trait QFutureInterfaceBase_waitForResume<RetType> {
  fn waitForResume(self , rsthis: & QFutureInterfaceBase) -> RetType;
}

  // proto:  void QFutureInterfaceBase::waitForResume();
impl<'a> /*trait*/ QFutureInterfaceBase_waitForResume<()> for () {
  fn waitForResume(self , rsthis: & QFutureInterfaceBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QFutureInterfaceBase13waitForResumeEv()};
     unsafe {C_ZN20QFutureInterfaceBase13waitForResumeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QFutureInterfaceBase::setProgressValue(int progressValue);
impl /*struct*/ QFutureInterfaceBase {
  pub fn setProgressValue<RetType, T: QFutureInterfaceBase_setProgressValue<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setProgressValue(self);
    // return 1;
  }
}

pub trait QFutureInterfaceBase_setProgressValue<RetType> {
  fn setProgressValue(self , rsthis: & QFutureInterfaceBase) -> RetType;
}

  // proto:  void QFutureInterfaceBase::setProgressValue(int progressValue);
impl<'a> /*trait*/ QFutureInterfaceBase_setProgressValue<()> for (i32) {
  fn setProgressValue(self , rsthis: & QFutureInterfaceBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QFutureInterfaceBase16setProgressValueEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN20QFutureInterfaceBase16setProgressValueEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QFutureInterfaceBase::isFinished();
impl /*struct*/ QFutureInterfaceBase {
  pub fn isFinished<RetType, T: QFutureInterfaceBase_isFinished<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isFinished(self);
    // return 1;
  }
}

pub trait QFutureInterfaceBase_isFinished<RetType> {
  fn isFinished(self , rsthis: & QFutureInterfaceBase) -> RetType;
}

  // proto:  bool QFutureInterfaceBase::isFinished();
impl<'a> /*trait*/ QFutureInterfaceBase_isFinished<i8> for () {
  fn isFinished(self , rsthis: & QFutureInterfaceBase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QFutureInterfaceBase10isFinishedEv()};
    let mut ret = unsafe {C_ZNK20QFutureInterfaceBase10isFinishedEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

impl /*struct*/ QFutureInterfaceLvoidG {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QFutureInterfaceLvoidG {
    return QFutureInterfaceLvoidG{qbase: QFutureInterfaceBase::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QFutureInterfaceLvoidG {
  type Target = QFutureInterfaceBase;

  fn deref(&self) -> &QFutureInterfaceBase {
    return & self.qbase;
  }
}
impl AsRef<QFutureInterfaceBase> for QFutureInterfaceLvoidG {
  fn as_ref(& self) -> & QFutureInterfaceBase {
    return & self.qbase;
  }
}
  // proto: static QFutureInterface<void> QFutureInterface<void>::canceledResult();
impl /*struct*/ QFutureInterfaceLvoidG {
  pub fn canceledResult_s<RetType, T: QFutureInterfaceLvoidG_canceledResult_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.canceledResult_s();
    // return 1;
  }
}

pub trait QFutureInterfaceLvoidG_canceledResult_s<RetType> {
  fn canceledResult_s(self ) -> RetType;
}

  // proto: static QFutureInterface<void> QFutureInterface<void>::canceledResult();
impl<'a> /*trait*/ QFutureInterfaceLvoidG_canceledResult_s<QFutureInterfaceLvoidG> for () {
  fn canceledResult_s(self ) -> QFutureInterfaceLvoidG {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFutureInterfaceIvE14canceledResultEv()};
    let mut ret = unsafe {C_ZN16QFutureInterfaceIvE14canceledResultEv()};
    let mut ret1 = QFutureInterfaceLvoidG::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QFuture<void> QFutureInterface<void>::future();
impl /*struct*/ QFutureInterfaceLvoidG {
  pub fn future<RetType, T: QFutureInterfaceLvoidG_future<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.future(self);
    // return 1;
  }
}

pub trait QFutureInterfaceLvoidG_future<RetType> {
  fn future(self , rsthis: & QFutureInterfaceLvoidG) -> RetType;
}

  // proto:  QFuture<void> QFutureInterface<void>::future();
impl<'a> /*trait*/ QFutureInterfaceLvoidG_future<QFutureLvoidG> for () {
  fn future(self , rsthis: & QFutureInterfaceLvoidG) -> QFutureLvoidG {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFutureInterfaceIvE6futureEv()};
    let mut ret = unsafe {C_ZN16QFutureInterfaceIvE6futureEv(rsthis.qclsinst)};
    let mut ret1 = QFutureLvoidG::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QFutureInterface<void>::reportResult(const void * , int );
impl /*struct*/ QFutureInterfaceLvoidG {
  pub fn reportResult<RetType, T: QFutureInterfaceLvoidG_reportResult<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.reportResult(self);
    // return 1;
  }
}

pub trait QFutureInterfaceLvoidG_reportResult<RetType> {
  fn reportResult(self , rsthis: & QFutureInterfaceLvoidG) -> RetType;
}

  // proto:  void QFutureInterface<void>::reportResult(const void * , int );
impl<'a> /*trait*/ QFutureInterfaceLvoidG_reportResult<()> for (*mut c_void, i32) {
  fn reportResult(self , rsthis: & QFutureInterfaceLvoidG) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFutureInterfaceIvE12reportResultEPKvi()};
    let arg0 = self.0  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN16QFutureInterfaceIvE12reportResultEPKvi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QFutureInterface<void>::reportFinished(const void * );
impl /*struct*/ QFutureInterfaceLvoidG {
  pub fn reportFinished<RetType, T: QFutureInterfaceLvoidG_reportFinished<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.reportFinished(self);
    // return 1;
  }
}

pub trait QFutureInterfaceLvoidG_reportFinished<RetType> {
  fn reportFinished(self , rsthis: & QFutureInterfaceLvoidG) -> RetType;
}

  // proto:  void QFutureInterface<void>::reportFinished(const void * );
impl<'a> /*trait*/ QFutureInterfaceLvoidG_reportFinished<()> for (Option<*mut c_void>) {
  fn reportFinished(self , rsthis: & QFutureInterfaceLvoidG) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFutureInterfaceIvE14reportFinishedEPKv()};
    let arg0 = (if self.is_none() {0 as *mut c_void} else {self.unwrap()})  as *mut c_void;
     unsafe {C_ZN16QFutureInterfaceIvE14reportFinishedEPKv(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

