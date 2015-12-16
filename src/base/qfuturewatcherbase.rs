// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qevent::QEvent;
use super::qstring::QString;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QFutureWatcherBase::finished();
  fn _ZN18QFutureWatcherBase8finishedEv(qthis: *mut c_void) ;
  // proto:  bool QFutureWatcherBase::isRunning();
  fn _ZNK18QFutureWatcherBase9isRunningEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QFutureWatcherBase::setPaused(bool paused);
  fn _ZN18QFutureWatcherBase9setPausedEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  int QFutureWatcherBase::progressMinimum();
  fn _ZNK18QFutureWatcherBase15progressMinimumEv(qthis: *mut c_void) -> c_int;
  // proto:  void QFutureWatcherBase::resume();
  fn _ZN18QFutureWatcherBase6resumeEv(qthis: *mut c_void) ;
  // proto:  const QMetaObject * QFutureWatcherBase::metaObject();
  fn _ZNK18QFutureWatcherBase10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QFutureWatcherBase::resultsReadyAt(int beginIndex, int endIndex);
  fn _ZN18QFutureWatcherBase14resultsReadyAtEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  bool QFutureWatcherBase::isFinished();
  fn _ZNK18QFutureWatcherBase10isFinishedEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QFutureWatcherBase::progressValueChanged(int progressValue);
  fn _ZN18QFutureWatcherBase20progressValueChangedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QFutureWatcherBase::progressMaximum();
  fn _ZNK18QFutureWatcherBase15progressMaximumEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QFutureWatcherBase::event(QEvent * event);
  fn _ZN18QFutureWatcherBase5eventEP6QEvent(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QFutureWatcherBase::progressTextChanged(const QString & progressText);
  fn _ZN18QFutureWatcherBase19progressTextChangedERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QFutureWatcherBase::isCanceled();
  fn _ZNK18QFutureWatcherBase10isCanceledEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QFutureWatcherBase::resultReadyAt(int resultIndex);
  fn _ZN18QFutureWatcherBase13resultReadyAtEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QFutureWatcherBase::progressValue();
  fn _ZNK18QFutureWatcherBase13progressValueEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QFutureWatcherBase::isStarted();
  fn _ZNK18QFutureWatcherBase9isStartedEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QFutureWatcherBase::paused();
  fn _ZN18QFutureWatcherBase6pausedEv(qthis: *mut c_void) ;
  // proto:  void QFutureWatcherBase::started();
  fn _ZN18QFutureWatcherBase7startedEv(qthis: *mut c_void) ;
  // proto:  void QFutureWatcherBase::setPendingResultsLimit(int limit);
  fn _ZN18QFutureWatcherBase22setPendingResultsLimitEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QFutureWatcherBase::resumed();
  fn _ZN18QFutureWatcherBase7resumedEv(qthis: *mut c_void) ;
  // proto:  void QFutureWatcherBase::cancel();
  fn _ZN18QFutureWatcherBase6cancelEv(qthis: *mut c_void) ;
  // proto:  void QFutureWatcherBase::progressRangeChanged(int minimum, int maximum);
  fn _ZN18QFutureWatcherBase20progressRangeChangedEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QFutureWatcherBase::canceled();
  fn _ZN18QFutureWatcherBase8canceledEv(qthis: *mut c_void) ;
  // proto:  bool QFutureWatcherBase::isPaused();
  fn _ZNK18QFutureWatcherBase8isPausedEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QFutureWatcherBase::pause();
  fn _ZN18QFutureWatcherBase5pauseEv(qthis: *mut c_void) ;
  // proto:  QString QFutureWatcherBase::progressText();
  fn _ZNK18QFutureWatcherBase12progressTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QFutureWatcherBase::NewQFutureWatcherBase(QObject * parent);
  fn _ZN18QFutureWatcherBaseC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QFutureWatcherBase::togglePaused();
  fn _ZN18QFutureWatcherBase12togglePausedEv(qthis: *mut c_void) ;
  // proto:  void QFutureWatcherBase::waitForFinished();
  fn _ZN18QFutureWatcherBase15waitForFinishedEv(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QFutureWatcherBase)=1
pub struct QFutureWatcherBase {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFutureWatcherBase {
  pub fn finished<T: QFutureWatcherBase_finished>(&mut self, value: T)  {
     value.finished(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_finished {
  fn finished(self, rsthis: &mut QFutureWatcherBase) ;
}

// proto:  void QFutureWatcherBase::finished();
impl<'a> /*trait*/ QFutureWatcherBase_finished for () {
  fn finished(self, rsthis: &mut QFutureWatcherBase)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase8finishedEv()};
     unsafe {_ZN18QFutureWatcherBase8finishedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn isRunning<T: QFutureWatcherBase_isRunning>(&mut self, value: T) -> i8 {
    return value.isRunning(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_isRunning {
  fn isRunning(self, rsthis: &mut QFutureWatcherBase) -> i8;
}

// proto:  bool QFutureWatcherBase::isRunning();
impl<'a> /*trait*/ QFutureWatcherBase_isRunning for () {
  fn isRunning(self, rsthis: &mut QFutureWatcherBase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFutureWatcherBase9isRunningEv()};
    let mut ret = unsafe {_ZNK18QFutureWatcherBase9isRunningEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn setPaused<T: QFutureWatcherBase_setPaused>(&mut self, value: T)  {
     value.setPaused(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_setPaused {
  fn setPaused(self, rsthis: &mut QFutureWatcherBase) ;
}

// proto:  void QFutureWatcherBase::setPaused(bool paused);
impl<'a> /*trait*/ QFutureWatcherBase_setPaused for (i8) {
  fn setPaused(self, rsthis: &mut QFutureWatcherBase)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase9setPausedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN18QFutureWatcherBase9setPausedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn progressMinimum<T: QFutureWatcherBase_progressMinimum>(&mut self, value: T) -> i32 {
    return value.progressMinimum(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_progressMinimum {
  fn progressMinimum(self, rsthis: &mut QFutureWatcherBase) -> i32;
}

// proto:  int QFutureWatcherBase::progressMinimum();
impl<'a> /*trait*/ QFutureWatcherBase_progressMinimum for () {
  fn progressMinimum(self, rsthis: &mut QFutureWatcherBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFutureWatcherBase15progressMinimumEv()};
    let mut ret = unsafe {_ZNK18QFutureWatcherBase15progressMinimumEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn resume<T: QFutureWatcherBase_resume>(&mut self, value: T)  {
     value.resume(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_resume {
  fn resume(self, rsthis: &mut QFutureWatcherBase) ;
}

// proto:  void QFutureWatcherBase::resume();
impl<'a> /*trait*/ QFutureWatcherBase_resume for () {
  fn resume(self, rsthis: &mut QFutureWatcherBase)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase6resumeEv()};
     unsafe {_ZN18QFutureWatcherBase6resumeEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn metaObject<T: QFutureWatcherBase_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_metaObject {
  fn metaObject(self, rsthis: &mut QFutureWatcherBase) ;
}

// proto:  const QMetaObject * QFutureWatcherBase::metaObject();
impl<'a> /*trait*/ QFutureWatcherBase_metaObject for () {
  fn metaObject(self, rsthis: &mut QFutureWatcherBase)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFutureWatcherBase10metaObjectEv()};
     unsafe {_ZNK18QFutureWatcherBase10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn resultsReadyAt<T: QFutureWatcherBase_resultsReadyAt>(&mut self, value: T)  {
     value.resultsReadyAt(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_resultsReadyAt {
  fn resultsReadyAt(self, rsthis: &mut QFutureWatcherBase) ;
}

// proto:  void QFutureWatcherBase::resultsReadyAt(int beginIndex, int endIndex);
impl<'a> /*trait*/ QFutureWatcherBase_resultsReadyAt for (i32, i32) {
  fn resultsReadyAt(self, rsthis: &mut QFutureWatcherBase)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase14resultsReadyAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN18QFutureWatcherBase14resultsReadyAtEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn isFinished<T: QFutureWatcherBase_isFinished>(&mut self, value: T) -> i8 {
    return value.isFinished(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_isFinished {
  fn isFinished(self, rsthis: &mut QFutureWatcherBase) -> i8;
}

// proto:  bool QFutureWatcherBase::isFinished();
impl<'a> /*trait*/ QFutureWatcherBase_isFinished for () {
  fn isFinished(self, rsthis: &mut QFutureWatcherBase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFutureWatcherBase10isFinishedEv()};
    let mut ret = unsafe {_ZNK18QFutureWatcherBase10isFinishedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn progressValueChanged<T: QFutureWatcherBase_progressValueChanged>(&mut self, value: T)  {
     value.progressValueChanged(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_progressValueChanged {
  fn progressValueChanged(self, rsthis: &mut QFutureWatcherBase) ;
}

// proto:  void QFutureWatcherBase::progressValueChanged(int progressValue);
impl<'a> /*trait*/ QFutureWatcherBase_progressValueChanged for (i32) {
  fn progressValueChanged(self, rsthis: &mut QFutureWatcherBase)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase20progressValueChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN18QFutureWatcherBase20progressValueChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn progressMaximum<T: QFutureWatcherBase_progressMaximum>(&mut self, value: T) -> i32 {
    return value.progressMaximum(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_progressMaximum {
  fn progressMaximum(self, rsthis: &mut QFutureWatcherBase) -> i32;
}

// proto:  int QFutureWatcherBase::progressMaximum();
impl<'a> /*trait*/ QFutureWatcherBase_progressMaximum for () {
  fn progressMaximum(self, rsthis: &mut QFutureWatcherBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFutureWatcherBase15progressMaximumEv()};
    let mut ret = unsafe {_ZNK18QFutureWatcherBase15progressMaximumEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn event<T: QFutureWatcherBase_event>(&mut self, value: T) -> i8 {
    return value.event(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_event {
  fn event(self, rsthis: &mut QFutureWatcherBase) -> i8;
}

// proto:  bool QFutureWatcherBase::event(QEvent * event);
impl<'a> /*trait*/ QFutureWatcherBase_event for (&'a mut QEvent) {
  fn event(self, rsthis: &mut QFutureWatcherBase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase5eventEP6QEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN18QFutureWatcherBase5eventEP6QEvent(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn progressTextChanged<T: QFutureWatcherBase_progressTextChanged>(&mut self, value: T)  {
     value.progressTextChanged(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_progressTextChanged {
  fn progressTextChanged(self, rsthis: &mut QFutureWatcherBase) ;
}

// proto:  void QFutureWatcherBase::progressTextChanged(const QString & progressText);
impl<'a> /*trait*/ QFutureWatcherBase_progressTextChanged for (&'a  QString) {
  fn progressTextChanged(self, rsthis: &mut QFutureWatcherBase)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase19progressTextChangedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QFutureWatcherBase19progressTextChangedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn isCanceled<T: QFutureWatcherBase_isCanceled>(&mut self, value: T) -> i8 {
    return value.isCanceled(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_isCanceled {
  fn isCanceled(self, rsthis: &mut QFutureWatcherBase) -> i8;
}

// proto:  bool QFutureWatcherBase::isCanceled();
impl<'a> /*trait*/ QFutureWatcherBase_isCanceled for () {
  fn isCanceled(self, rsthis: &mut QFutureWatcherBase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFutureWatcherBase10isCanceledEv()};
    let mut ret = unsafe {_ZNK18QFutureWatcherBase10isCanceledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn resultReadyAt<T: QFutureWatcherBase_resultReadyAt>(&mut self, value: T)  {
     value.resultReadyAt(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_resultReadyAt {
  fn resultReadyAt(self, rsthis: &mut QFutureWatcherBase) ;
}

// proto:  void QFutureWatcherBase::resultReadyAt(int resultIndex);
impl<'a> /*trait*/ QFutureWatcherBase_resultReadyAt for (i32) {
  fn resultReadyAt(self, rsthis: &mut QFutureWatcherBase)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase13resultReadyAtEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN18QFutureWatcherBase13resultReadyAtEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn progressValue<T: QFutureWatcherBase_progressValue>(&mut self, value: T) -> i32 {
    return value.progressValue(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_progressValue {
  fn progressValue(self, rsthis: &mut QFutureWatcherBase) -> i32;
}

// proto:  int QFutureWatcherBase::progressValue();
impl<'a> /*trait*/ QFutureWatcherBase_progressValue for () {
  fn progressValue(self, rsthis: &mut QFutureWatcherBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFutureWatcherBase13progressValueEv()};
    let mut ret = unsafe {_ZNK18QFutureWatcherBase13progressValueEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn isStarted<T: QFutureWatcherBase_isStarted>(&mut self, value: T) -> i8 {
    return value.isStarted(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_isStarted {
  fn isStarted(self, rsthis: &mut QFutureWatcherBase) -> i8;
}

// proto:  bool QFutureWatcherBase::isStarted();
impl<'a> /*trait*/ QFutureWatcherBase_isStarted for () {
  fn isStarted(self, rsthis: &mut QFutureWatcherBase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFutureWatcherBase9isStartedEv()};
    let mut ret = unsafe {_ZNK18QFutureWatcherBase9isStartedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn paused<T: QFutureWatcherBase_paused>(&mut self, value: T)  {
     value.paused(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_paused {
  fn paused(self, rsthis: &mut QFutureWatcherBase) ;
}

// proto:  void QFutureWatcherBase::paused();
impl<'a> /*trait*/ QFutureWatcherBase_paused for () {
  fn paused(self, rsthis: &mut QFutureWatcherBase)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase6pausedEv()};
     unsafe {_ZN18QFutureWatcherBase6pausedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn started<T: QFutureWatcherBase_started>(&mut self, value: T)  {
     value.started(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_started {
  fn started(self, rsthis: &mut QFutureWatcherBase) ;
}

// proto:  void QFutureWatcherBase::started();
impl<'a> /*trait*/ QFutureWatcherBase_started for () {
  fn started(self, rsthis: &mut QFutureWatcherBase)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase7startedEv()};
     unsafe {_ZN18QFutureWatcherBase7startedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn setPendingResultsLimit<T: QFutureWatcherBase_setPendingResultsLimit>(&mut self, value: T)  {
     value.setPendingResultsLimit(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_setPendingResultsLimit {
  fn setPendingResultsLimit(self, rsthis: &mut QFutureWatcherBase) ;
}

// proto:  void QFutureWatcherBase::setPendingResultsLimit(int limit);
impl<'a> /*trait*/ QFutureWatcherBase_setPendingResultsLimit for (i32) {
  fn setPendingResultsLimit(self, rsthis: &mut QFutureWatcherBase)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase22setPendingResultsLimitEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN18QFutureWatcherBase22setPendingResultsLimitEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn resumed<T: QFutureWatcherBase_resumed>(&mut self, value: T)  {
     value.resumed(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_resumed {
  fn resumed(self, rsthis: &mut QFutureWatcherBase) ;
}

// proto:  void QFutureWatcherBase::resumed();
impl<'a> /*trait*/ QFutureWatcherBase_resumed for () {
  fn resumed(self, rsthis: &mut QFutureWatcherBase)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase7resumedEv()};
     unsafe {_ZN18QFutureWatcherBase7resumedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn cancel<T: QFutureWatcherBase_cancel>(&mut self, value: T)  {
     value.cancel(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_cancel {
  fn cancel(self, rsthis: &mut QFutureWatcherBase) ;
}

// proto:  void QFutureWatcherBase::cancel();
impl<'a> /*trait*/ QFutureWatcherBase_cancel for () {
  fn cancel(self, rsthis: &mut QFutureWatcherBase)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase6cancelEv()};
     unsafe {_ZN18QFutureWatcherBase6cancelEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn progressRangeChanged<T: QFutureWatcherBase_progressRangeChanged>(&mut self, value: T)  {
     value.progressRangeChanged(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_progressRangeChanged {
  fn progressRangeChanged(self, rsthis: &mut QFutureWatcherBase) ;
}

// proto:  void QFutureWatcherBase::progressRangeChanged(int minimum, int maximum);
impl<'a> /*trait*/ QFutureWatcherBase_progressRangeChanged for (i32, i32) {
  fn progressRangeChanged(self, rsthis: &mut QFutureWatcherBase)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase20progressRangeChangedEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN18QFutureWatcherBase20progressRangeChangedEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn canceled<T: QFutureWatcherBase_canceled>(&mut self, value: T)  {
     value.canceled(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_canceled {
  fn canceled(self, rsthis: &mut QFutureWatcherBase) ;
}

// proto:  void QFutureWatcherBase::canceled();
impl<'a> /*trait*/ QFutureWatcherBase_canceled for () {
  fn canceled(self, rsthis: &mut QFutureWatcherBase)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase8canceledEv()};
     unsafe {_ZN18QFutureWatcherBase8canceledEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn isPaused<T: QFutureWatcherBase_isPaused>(&mut self, value: T) -> i8 {
    return value.isPaused(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_isPaused {
  fn isPaused(self, rsthis: &mut QFutureWatcherBase) -> i8;
}

// proto:  bool QFutureWatcherBase::isPaused();
impl<'a> /*trait*/ QFutureWatcherBase_isPaused for () {
  fn isPaused(self, rsthis: &mut QFutureWatcherBase) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFutureWatcherBase8isPausedEv()};
    let mut ret = unsafe {_ZNK18QFutureWatcherBase8isPausedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn pause<T: QFutureWatcherBase_pause>(&mut self, value: T)  {
     value.pause(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_pause {
  fn pause(self, rsthis: &mut QFutureWatcherBase) ;
}

// proto:  void QFutureWatcherBase::pause();
impl<'a> /*trait*/ QFutureWatcherBase_pause for () {
  fn pause(self, rsthis: &mut QFutureWatcherBase)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase5pauseEv()};
     unsafe {_ZN18QFutureWatcherBase5pauseEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn progressText<T: QFutureWatcherBase_progressText>(&mut self, value: T) -> QString {
    return value.progressText(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_progressText {
  fn progressText(self, rsthis: &mut QFutureWatcherBase) -> QString;
}

// proto:  QString QFutureWatcherBase::progressText();
impl<'a> /*trait*/ QFutureWatcherBase_progressText for () {
  fn progressText(self, rsthis: &mut QFutureWatcherBase) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFutureWatcherBase12progressTextEv()};
    let mut ret = unsafe {_ZNK18QFutureWatcherBase12progressTextEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn NewQFutureWatcherBase<T: QFutureWatcherBase_NewQFutureWatcherBase>(value: T) -> QFutureWatcherBase {
    let rsthis = value.NewQFutureWatcherBase();
    return rsthis;
    // return 1;
  }
}

pub trait QFutureWatcherBase_NewQFutureWatcherBase {
  fn NewQFutureWatcherBase(self) -> QFutureWatcherBase;
}

// proto: void QFutureWatcherBase::NewQFutureWatcherBase(QObject * parent);
impl<'a> /*trait*/ QFutureWatcherBase_NewQFutureWatcherBase for (&'a mut QObject) {
  fn NewQFutureWatcherBase(self) -> QFutureWatcherBase {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBaseC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QFutureWatcherBaseC1EP7QObject(qthis, arg0)};
    let rsthis = QFutureWatcherBase{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn togglePaused<T: QFutureWatcherBase_togglePaused>(&mut self, value: T)  {
     value.togglePaused(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_togglePaused {
  fn togglePaused(self, rsthis: &mut QFutureWatcherBase) ;
}

// proto:  void QFutureWatcherBase::togglePaused();
impl<'a> /*trait*/ QFutureWatcherBase_togglePaused for () {
  fn togglePaused(self, rsthis: &mut QFutureWatcherBase)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase12togglePausedEv()};
     unsafe {_ZN18QFutureWatcherBase12togglePausedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn waitForFinished<T: QFutureWatcherBase_waitForFinished>(&mut self, value: T)  {
     value.waitForFinished(self);
    // return 1;
  }
}

pub trait QFutureWatcherBase_waitForFinished {
  fn waitForFinished(self, rsthis: &mut QFutureWatcherBase) ;
}

// proto:  void QFutureWatcherBase::waitForFinished();
impl<'a> /*trait*/ QFutureWatcherBase_waitForFinished for () {
  fn waitForFinished(self, rsthis: &mut QFutureWatcherBase)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase15waitForFinishedEv()};
     unsafe {_ZN18QFutureWatcherBase15waitForFinishedEv(rsthis.qclsinst)};
    // return 1;
  }
}

