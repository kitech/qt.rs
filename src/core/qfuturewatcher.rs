// auto generated, do not modify.
// created: Wed Dec 23 22:29:56 2015
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
use super::qobject::QObject; // 773
use std::ops::Deref;
use super::qcoreevent::QEvent; // 773
use super::qstring::QString; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QFutureWatcherBase::finished();
  fn _ZN18QFutureWatcherBase8finishedEv(qthis: *mut c_void);
  // proto:  bool QFutureWatcherBase::isRunning();
  fn _ZNK18QFutureWatcherBase9isRunningEv(qthis: *mut c_void) -> c_char;
  // proto:  void QFutureWatcherBase::setPaused(bool paused);
  fn _ZN18QFutureWatcherBase9setPausedEb(qthis: *mut c_void, arg0: c_char);
  // proto:  int QFutureWatcherBase::progressMinimum();
  fn _ZNK18QFutureWatcherBase15progressMinimumEv(qthis: *mut c_void) -> c_int;
  // proto:  void QFutureWatcherBase::resume();
  fn _ZN18QFutureWatcherBase6resumeEv(qthis: *mut c_void);
  // proto:  const QMetaObject * QFutureWatcherBase::metaObject();
  fn _ZNK18QFutureWatcherBase10metaObjectEv(qthis: *mut c_void);
  // proto:  void QFutureWatcherBase::resultsReadyAt(int beginIndex, int endIndex);
  fn _ZN18QFutureWatcherBase14resultsReadyAtEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  bool QFutureWatcherBase::isFinished();
  fn _ZNK18QFutureWatcherBase10isFinishedEv(qthis: *mut c_void) -> c_char;
  // proto:  void QFutureWatcherBase::progressValueChanged(int progressValue);
  fn _ZN18QFutureWatcherBase20progressValueChangedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QFutureWatcherBase::progressMaximum();
  fn _ZNK18QFutureWatcherBase15progressMaximumEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QFutureWatcherBase::event(QEvent * event);
  fn _ZN18QFutureWatcherBase5eventEP6QEvent(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  void QFutureWatcherBase::progressTextChanged(const QString & progressText);
  fn _ZN18QFutureWatcherBase19progressTextChangedERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QFutureWatcherBase::isCanceled();
  fn _ZNK18QFutureWatcherBase10isCanceledEv(qthis: *mut c_void) -> c_char;
  // proto:  void QFutureWatcherBase::resultReadyAt(int resultIndex);
  fn _ZN18QFutureWatcherBase13resultReadyAtEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QFutureWatcherBase::progressValue();
  fn _ZNK18QFutureWatcherBase13progressValueEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QFutureWatcherBase::isStarted();
  fn _ZNK18QFutureWatcherBase9isStartedEv(qthis: *mut c_void) -> c_char;
  // proto:  void QFutureWatcherBase::paused();
  fn _ZN18QFutureWatcherBase6pausedEv(qthis: *mut c_void);
  // proto:  void QFutureWatcherBase::started();
  fn _ZN18QFutureWatcherBase7startedEv(qthis: *mut c_void);
  // proto:  void QFutureWatcherBase::setPendingResultsLimit(int limit);
  fn _ZN18QFutureWatcherBase22setPendingResultsLimitEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QFutureWatcherBase::resumed();
  fn _ZN18QFutureWatcherBase7resumedEv(qthis: *mut c_void);
  // proto:  void QFutureWatcherBase::cancel();
  fn _ZN18QFutureWatcherBase6cancelEv(qthis: *mut c_void);
  // proto:  void QFutureWatcherBase::progressRangeChanged(int minimum, int maximum);
  fn _ZN18QFutureWatcherBase20progressRangeChangedEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  void QFutureWatcherBase::canceled();
  fn _ZN18QFutureWatcherBase8canceledEv(qthis: *mut c_void);
  // proto:  bool QFutureWatcherBase::isPaused();
  fn _ZNK18QFutureWatcherBase8isPausedEv(qthis: *mut c_void) -> c_char;
  // proto:  void QFutureWatcherBase::pause();
  fn _ZN18QFutureWatcherBase5pauseEv(qthis: *mut c_void);
  // proto:  QString QFutureWatcherBase::progressText();
  fn _ZNK18QFutureWatcherBase12progressTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QFutureWatcherBase::QFutureWatcherBase(QObject * parent);
  fn _ZN18QFutureWatcherBaseC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QFutureWatcherBase::togglePaused();
  fn _ZN18QFutureWatcherBase12togglePausedEv(qthis: *mut c_void);
  // proto:  void QFutureWatcherBase::waitForFinished();
  fn _ZN18QFutureWatcherBase15waitForFinishedEv(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QFutureWatcherBase)=1
pub struct QFutureWatcherBase {
  qbase: QObject,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFutureWatcherBase {
  pub fn inheritFrom(qthis: *mut c_void) -> QFutureWatcherBase {
    return QFutureWatcherBase{qbase: QObject::inheritFrom(qthis), qclsinst: qthis};
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
  // proto:  void QFutureWatcherBase::finished();
impl /*struct*/ QFutureWatcherBase {
  pub fn finished<RetType, T: QFutureWatcherBase_finished<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.finished(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_finished<RetType> {
  fn finished(self , rsthis: & QFutureWatcherBase) -> RetType;
}

  // proto:  void QFutureWatcherBase::finished();
impl<'a> /*trait*/ QFutureWatcherBase_finished<()> for () {
  fn finished(self , rsthis: & QFutureWatcherBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase8finishedEv()};
     unsafe {_ZN18QFutureWatcherBase8finishedEv(rsthis.qclsinst)};
    // return 1;
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
    let mut ret = unsafe {_ZNK18QFutureWatcherBase9isRunningEv(rsthis.qclsinst)};
    return ret as i8;
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
     unsafe {_ZN18QFutureWatcherBase9setPausedEb(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {_ZNK18QFutureWatcherBase15progressMinimumEv(rsthis.qclsinst)};
    return ret as i32;
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
     unsafe {_ZN18QFutureWatcherBase6resumeEv(rsthis.qclsinst)};
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
impl<'a> /*trait*/ QFutureWatcherBase_metaObject<()> for () {
  fn metaObject(self , rsthis: & QFutureWatcherBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFutureWatcherBase10metaObjectEv()};
     unsafe {_ZNK18QFutureWatcherBase10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QFutureWatcherBase::resultsReadyAt(int beginIndex, int endIndex);
impl /*struct*/ QFutureWatcherBase {
  pub fn resultsReadyAt<RetType, T: QFutureWatcherBase_resultsReadyAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resultsReadyAt(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_resultsReadyAt<RetType> {
  fn resultsReadyAt(self , rsthis: & QFutureWatcherBase) -> RetType;
}

  // proto:  void QFutureWatcherBase::resultsReadyAt(int beginIndex, int endIndex);
impl<'a> /*trait*/ QFutureWatcherBase_resultsReadyAt<()> for (i32, i32) {
  fn resultsReadyAt(self , rsthis: & QFutureWatcherBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase14resultsReadyAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN18QFutureWatcherBase14resultsReadyAtEii(rsthis.qclsinst, arg0, arg1)};
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
    let mut ret = unsafe {_ZNK18QFutureWatcherBase10isFinishedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QFutureWatcherBase::progressValueChanged(int progressValue);
impl /*struct*/ QFutureWatcherBase {
  pub fn progressValueChanged<RetType, T: QFutureWatcherBase_progressValueChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.progressValueChanged(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_progressValueChanged<RetType> {
  fn progressValueChanged(self , rsthis: & QFutureWatcherBase) -> RetType;
}

  // proto:  void QFutureWatcherBase::progressValueChanged(int progressValue);
impl<'a> /*trait*/ QFutureWatcherBase_progressValueChanged<()> for (i32) {
  fn progressValueChanged(self , rsthis: & QFutureWatcherBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase20progressValueChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN18QFutureWatcherBase20progressValueChangedEi(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {_ZNK18QFutureWatcherBase15progressMaximumEv(rsthis.qclsinst)};
    return ret as i32;
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
    let mut ret = unsafe {_ZN18QFutureWatcherBase5eventEP6QEvent(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QFutureWatcherBase::progressTextChanged(const QString & progressText);
impl /*struct*/ QFutureWatcherBase {
  pub fn progressTextChanged<RetType, T: QFutureWatcherBase_progressTextChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.progressTextChanged(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_progressTextChanged<RetType> {
  fn progressTextChanged(self , rsthis: & QFutureWatcherBase) -> RetType;
}

  // proto:  void QFutureWatcherBase::progressTextChanged(const QString & progressText);
impl<'a> /*trait*/ QFutureWatcherBase_progressTextChanged<()> for (&'a QString) {
  fn progressTextChanged(self , rsthis: & QFutureWatcherBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase19progressTextChangedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QFutureWatcherBase19progressTextChangedERK7QString(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {_ZNK18QFutureWatcherBase10isCanceledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QFutureWatcherBase::resultReadyAt(int resultIndex);
impl /*struct*/ QFutureWatcherBase {
  pub fn resultReadyAt<RetType, T: QFutureWatcherBase_resultReadyAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resultReadyAt(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_resultReadyAt<RetType> {
  fn resultReadyAt(self , rsthis: & QFutureWatcherBase) -> RetType;
}

  // proto:  void QFutureWatcherBase::resultReadyAt(int resultIndex);
impl<'a> /*trait*/ QFutureWatcherBase_resultReadyAt<()> for (i32) {
  fn resultReadyAt(self , rsthis: & QFutureWatcherBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase13resultReadyAtEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN18QFutureWatcherBase13resultReadyAtEi(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {_ZNK18QFutureWatcherBase13progressValueEv(rsthis.qclsinst)};
    return ret as i32;
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
    let mut ret = unsafe {_ZNK18QFutureWatcherBase9isStartedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QFutureWatcherBase::paused();
impl /*struct*/ QFutureWatcherBase {
  pub fn paused<RetType, T: QFutureWatcherBase_paused<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.paused(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_paused<RetType> {
  fn paused(self , rsthis: & QFutureWatcherBase) -> RetType;
}

  // proto:  void QFutureWatcherBase::paused();
impl<'a> /*trait*/ QFutureWatcherBase_paused<()> for () {
  fn paused(self , rsthis: & QFutureWatcherBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase6pausedEv()};
     unsafe {_ZN18QFutureWatcherBase6pausedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QFutureWatcherBase::started();
impl /*struct*/ QFutureWatcherBase {
  pub fn started<RetType, T: QFutureWatcherBase_started<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.started(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_started<RetType> {
  fn started(self , rsthis: & QFutureWatcherBase) -> RetType;
}

  // proto:  void QFutureWatcherBase::started();
impl<'a> /*trait*/ QFutureWatcherBase_started<()> for () {
  fn started(self , rsthis: & QFutureWatcherBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase7startedEv()};
     unsafe {_ZN18QFutureWatcherBase7startedEv(rsthis.qclsinst)};
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
     unsafe {_ZN18QFutureWatcherBase22setPendingResultsLimitEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QFutureWatcherBase::resumed();
impl /*struct*/ QFutureWatcherBase {
  pub fn resumed<RetType, T: QFutureWatcherBase_resumed<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resumed(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_resumed<RetType> {
  fn resumed(self , rsthis: & QFutureWatcherBase) -> RetType;
}

  // proto:  void QFutureWatcherBase::resumed();
impl<'a> /*trait*/ QFutureWatcherBase_resumed<()> for () {
  fn resumed(self , rsthis: & QFutureWatcherBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase7resumedEv()};
     unsafe {_ZN18QFutureWatcherBase7resumedEv(rsthis.qclsinst)};
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
     unsafe {_ZN18QFutureWatcherBase6cancelEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QFutureWatcherBase::progressRangeChanged(int minimum, int maximum);
impl /*struct*/ QFutureWatcherBase {
  pub fn progressRangeChanged<RetType, T: QFutureWatcherBase_progressRangeChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.progressRangeChanged(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_progressRangeChanged<RetType> {
  fn progressRangeChanged(self , rsthis: & QFutureWatcherBase) -> RetType;
}

  // proto:  void QFutureWatcherBase::progressRangeChanged(int minimum, int maximum);
impl<'a> /*trait*/ QFutureWatcherBase_progressRangeChanged<()> for (i32, i32) {
  fn progressRangeChanged(self , rsthis: & QFutureWatcherBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase20progressRangeChangedEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN18QFutureWatcherBase20progressRangeChangedEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QFutureWatcherBase::canceled();
impl /*struct*/ QFutureWatcherBase {
  pub fn canceled<RetType, T: QFutureWatcherBase_canceled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.canceled(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_canceled<RetType> {
  fn canceled(self , rsthis: & QFutureWatcherBase) -> RetType;
}

  // proto:  void QFutureWatcherBase::canceled();
impl<'a> /*trait*/ QFutureWatcherBase_canceled<()> for () {
  fn canceled(self , rsthis: & QFutureWatcherBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase8canceledEv()};
     unsafe {_ZN18QFutureWatcherBase8canceledEv(rsthis.qclsinst)};
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
    let mut ret = unsafe {_ZNK18QFutureWatcherBase8isPausedEv(rsthis.qclsinst)};
    return ret as i8;
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
     unsafe {_ZN18QFutureWatcherBase5pauseEv(rsthis.qclsinst)};
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
    let mut ret = unsafe {_ZNK18QFutureWatcherBase12progressTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QFutureWatcherBase::QFutureWatcherBase(QObject * parent);
impl /*struct*/ QFutureWatcherBase {
  pub fn New<T: QFutureWatcherBase_New>(value: T) -> QFutureWatcherBase {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QFutureWatcherBase_New {
  fn New(self) -> QFutureWatcherBase;
}

  // proto:  void QFutureWatcherBase::QFutureWatcherBase(QObject * parent);
impl<'a> /*trait*/ QFutureWatcherBase_New for (&'a QObject) {
  fn New(self) -> QFutureWatcherBase {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBaseC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QFutureWatcherBaseC1EP7QObject(qthis, arg0)};
    let rsthis = QFutureWatcherBase{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
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
     unsafe {_ZN18QFutureWatcherBase12togglePausedEv(rsthis.qclsinst)};
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
     unsafe {_ZN18QFutureWatcherBase15waitForFinishedEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

