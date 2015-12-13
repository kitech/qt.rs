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
  fn _ZN18QFutureWatcherBase8finishedEv() -> i32;
  fn _ZNK18QFutureWatcherBase9isRunningEv() -> i32;
  fn _ZN18QFutureWatcherBase9setPausedEb(arg0: int8_t) -> i32;
  fn _ZNK18QFutureWatcherBase15progressMinimumEv() -> i32;
  fn _ZN18QFutureWatcherBase6resumeEv() -> i32;
  fn _ZNK18QFutureWatcherBase10metaObjectEv() -> i32;
  fn _ZN18QFutureWatcherBase14resultsReadyAtEii(arg0: c_int, arg1: c_int) -> i32;
  fn _ZNK18QFutureWatcherBase10isFinishedEv() -> i32;
  fn _ZN18QFutureWatcherBase20progressValueChangedEi(arg0: c_int) -> i32;
  fn _ZNK18QFutureWatcherBase15progressMaximumEv() -> i32;
  fn _ZN18QFutureWatcherBase5eventEP6QEvent(arg0: *mut c_void) -> i32;
  fn _ZN18QFutureWatcherBase19progressTextChangedERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK18QFutureWatcherBase10isCanceledEv() -> i32;
  fn _ZN18QFutureWatcherBase13resultReadyAtEi(arg0: c_int) -> i32;
  fn _ZNK18QFutureWatcherBase13progressValueEv() -> i32;
  fn _ZNK18QFutureWatcherBase9isStartedEv() -> i32;
  fn _ZN18QFutureWatcherBase6pausedEv() -> i32;
  fn _ZN18QFutureWatcherBase7startedEv() -> i32;
  fn _ZN18QFutureWatcherBase22setPendingResultsLimitEi(arg0: c_int) -> i32;
  fn _ZN18QFutureWatcherBase7resumedEv() -> i32;
  fn _ZN18QFutureWatcherBase6cancelEv() -> i32;
  fn _ZN18QFutureWatcherBase20progressRangeChangedEii(arg0: c_int, arg1: c_int) -> i32;
  fn _ZN18QFutureWatcherBase8canceledEv() -> i32;
  fn _ZNK18QFutureWatcherBase8isPausedEv() -> i32;
  fn _ZN18QFutureWatcherBase5pauseEv() -> i32;
  fn _ZNK18QFutureWatcherBase12progressTextEv() -> i32;
  fn _ZN18QFutureWatcherBaseC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZN18QFutureWatcherBase12togglePausedEv() -> i32;
  fn _ZN18QFutureWatcherBase15waitForFinishedEv() -> i32;
}

// body block begin
// class sizeof(QFutureWatcherBase)=1
pub struct QFutureWatcherBase {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFutureWatcherBase {
  pub fn finished<T: QFutureWatcherBase_finished>(&mut self, value: T) -> i32 {
    value.finished(self);
    return 1;
  }
}

pub trait QFutureWatcherBase_finished {
  fn finished(self, this: &mut QFutureWatcherBase) -> i32;
}

// proto: void QFutureWatcherBase::finished();
impl<'a> /*trait*/ QFutureWatcherBase_finished for () {
  fn finished(self, this: &mut QFutureWatcherBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase8finishedEv()};
    unsafe {_ZN18QFutureWatcherBase8finishedEv()};
    return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn isRunning<T: QFutureWatcherBase_isRunning>(&mut self, value: T) -> i32 {
    value.isRunning(self);
    return 1;
  }
}

pub trait QFutureWatcherBase_isRunning {
  fn isRunning(self, this: &mut QFutureWatcherBase) -> i32;
}

// proto: bool QFutureWatcherBase::isRunning();
impl<'a> /*trait*/ QFutureWatcherBase_isRunning for () {
  fn isRunning(self, this: &mut QFutureWatcherBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFutureWatcherBase9isRunningEv()};
    unsafe {_ZNK18QFutureWatcherBase9isRunningEv()};
    return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn setPaused<T: QFutureWatcherBase_setPaused>(&mut self, value: T) -> i32 {
    value.setPaused(self);
    return 1;
  }
}

pub trait QFutureWatcherBase_setPaused {
  fn setPaused(self, this: &mut QFutureWatcherBase) -> i32;
}

// proto: void QFutureWatcherBase::setPaused(bool paused);
impl<'a> /*trait*/ QFutureWatcherBase_setPaused for (i8) {
  fn setPaused(self, this: &mut QFutureWatcherBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase9setPausedEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN18QFutureWatcherBase9setPausedEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn progressMinimum<T: QFutureWatcherBase_progressMinimum>(&mut self, value: T) -> i32 {
    value.progressMinimum(self);
    return 1;
  }
}

pub trait QFutureWatcherBase_progressMinimum {
  fn progressMinimum(self, this: &mut QFutureWatcherBase) -> i32;
}

// proto: int QFutureWatcherBase::progressMinimum();
impl<'a> /*trait*/ QFutureWatcherBase_progressMinimum for () {
  fn progressMinimum(self, this: &mut QFutureWatcherBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFutureWatcherBase15progressMinimumEv()};
    unsafe {_ZNK18QFutureWatcherBase15progressMinimumEv()};
    return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn resume<T: QFutureWatcherBase_resume>(&mut self, value: T) -> i32 {
    value.resume(self);
    return 1;
  }
}

pub trait QFutureWatcherBase_resume {
  fn resume(self, this: &mut QFutureWatcherBase) -> i32;
}

// proto: void QFutureWatcherBase::resume();
impl<'a> /*trait*/ QFutureWatcherBase_resume for () {
  fn resume(self, this: &mut QFutureWatcherBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase6resumeEv()};
    unsafe {_ZN18QFutureWatcherBase6resumeEv()};
    return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn metaObject<T: QFutureWatcherBase_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QFutureWatcherBase_metaObject {
  fn metaObject(self, this: &mut QFutureWatcherBase) -> i32;
}

// proto: const QMetaObject * QFutureWatcherBase::metaObject();
impl<'a> /*trait*/ QFutureWatcherBase_metaObject for () {
  fn metaObject(self, this: &mut QFutureWatcherBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFutureWatcherBase10metaObjectEv()};
    unsafe {_ZNK18QFutureWatcherBase10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn resultsReadyAt<T: QFutureWatcherBase_resultsReadyAt>(&mut self, value: T) -> i32 {
    value.resultsReadyAt(self);
    return 1;
  }
}

pub trait QFutureWatcherBase_resultsReadyAt {
  fn resultsReadyAt(self, this: &mut QFutureWatcherBase) -> i32;
}

// proto: void QFutureWatcherBase::resultsReadyAt(int beginIndex, int endIndex);
impl<'a> /*trait*/ QFutureWatcherBase_resultsReadyAt for (i32, i32) {
  fn resultsReadyAt(self, this: &mut QFutureWatcherBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase14resultsReadyAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN18QFutureWatcherBase14resultsReadyAtEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn isFinished<T: QFutureWatcherBase_isFinished>(&mut self, value: T) -> i32 {
    value.isFinished(self);
    return 1;
  }
}

pub trait QFutureWatcherBase_isFinished {
  fn isFinished(self, this: &mut QFutureWatcherBase) -> i32;
}

// proto: bool QFutureWatcherBase::isFinished();
impl<'a> /*trait*/ QFutureWatcherBase_isFinished for () {
  fn isFinished(self, this: &mut QFutureWatcherBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFutureWatcherBase10isFinishedEv()};
    unsafe {_ZNK18QFutureWatcherBase10isFinishedEv()};
    return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn progressValueChanged<T: QFutureWatcherBase_progressValueChanged>(&mut self, value: T) -> i32 {
    value.progressValueChanged(self);
    return 1;
  }
}

pub trait QFutureWatcherBase_progressValueChanged {
  fn progressValueChanged(self, this: &mut QFutureWatcherBase) -> i32;
}

// proto: void QFutureWatcherBase::progressValueChanged(int progressValue);
impl<'a> /*trait*/ QFutureWatcherBase_progressValueChanged for (i32) {
  fn progressValueChanged(self, this: &mut QFutureWatcherBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase20progressValueChangedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN18QFutureWatcherBase20progressValueChangedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn progressMaximum<T: QFutureWatcherBase_progressMaximum>(&mut self, value: T) -> i32 {
    value.progressMaximum(self);
    return 1;
  }
}

pub trait QFutureWatcherBase_progressMaximum {
  fn progressMaximum(self, this: &mut QFutureWatcherBase) -> i32;
}

// proto: int QFutureWatcherBase::progressMaximum();
impl<'a> /*trait*/ QFutureWatcherBase_progressMaximum for () {
  fn progressMaximum(self, this: &mut QFutureWatcherBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFutureWatcherBase15progressMaximumEv()};
    unsafe {_ZNK18QFutureWatcherBase15progressMaximumEv()};
    return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn event<T: QFutureWatcherBase_event>(&mut self, value: T) -> i32 {
    value.event(self);
    return 1;
  }
}

pub trait QFutureWatcherBase_event {
  fn event(self, this: &mut QFutureWatcherBase) -> i32;
}

// proto: bool QFutureWatcherBase::event(QEvent * event);
impl<'a> /*trait*/ QFutureWatcherBase_event for (&'a mut QEvent) {
  fn event(self, this: &mut QFutureWatcherBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase5eventEP6QEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QFutureWatcherBase5eventEP6QEvent(arg0)};
    return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn progressTextChanged<T: QFutureWatcherBase_progressTextChanged>(&mut self, value: T) -> i32 {
    value.progressTextChanged(self);
    return 1;
  }
}

pub trait QFutureWatcherBase_progressTextChanged {
  fn progressTextChanged(self, this: &mut QFutureWatcherBase) -> i32;
}

// proto: void QFutureWatcherBase::progressTextChanged(const QString & progressText);
impl<'a> /*trait*/ QFutureWatcherBase_progressTextChanged for (&'a  QString) {
  fn progressTextChanged(self, this: &mut QFutureWatcherBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase19progressTextChangedERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN18QFutureWatcherBase19progressTextChangedERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn isCanceled<T: QFutureWatcherBase_isCanceled>(&mut self, value: T) -> i32 {
    value.isCanceled(self);
    return 1;
  }
}

pub trait QFutureWatcherBase_isCanceled {
  fn isCanceled(self, this: &mut QFutureWatcherBase) -> i32;
}

// proto: bool QFutureWatcherBase::isCanceled();
impl<'a> /*trait*/ QFutureWatcherBase_isCanceled for () {
  fn isCanceled(self, this: &mut QFutureWatcherBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFutureWatcherBase10isCanceledEv()};
    unsafe {_ZNK18QFutureWatcherBase10isCanceledEv()};
    return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn resultReadyAt<T: QFutureWatcherBase_resultReadyAt>(&mut self, value: T) -> i32 {
    value.resultReadyAt(self);
    return 1;
  }
}

pub trait QFutureWatcherBase_resultReadyAt {
  fn resultReadyAt(self, this: &mut QFutureWatcherBase) -> i32;
}

// proto: void QFutureWatcherBase::resultReadyAt(int resultIndex);
impl<'a> /*trait*/ QFutureWatcherBase_resultReadyAt for (i32) {
  fn resultReadyAt(self, this: &mut QFutureWatcherBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase13resultReadyAtEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN18QFutureWatcherBase13resultReadyAtEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn progressValue<T: QFutureWatcherBase_progressValue>(&mut self, value: T) -> i32 {
    value.progressValue(self);
    return 1;
  }
}

pub trait QFutureWatcherBase_progressValue {
  fn progressValue(self, this: &mut QFutureWatcherBase) -> i32;
}

// proto: int QFutureWatcherBase::progressValue();
impl<'a> /*trait*/ QFutureWatcherBase_progressValue for () {
  fn progressValue(self, this: &mut QFutureWatcherBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFutureWatcherBase13progressValueEv()};
    unsafe {_ZNK18QFutureWatcherBase13progressValueEv()};
    return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn isStarted<T: QFutureWatcherBase_isStarted>(&mut self, value: T) -> i32 {
    value.isStarted(self);
    return 1;
  }
}

pub trait QFutureWatcherBase_isStarted {
  fn isStarted(self, this: &mut QFutureWatcherBase) -> i32;
}

// proto: bool QFutureWatcherBase::isStarted();
impl<'a> /*trait*/ QFutureWatcherBase_isStarted for () {
  fn isStarted(self, this: &mut QFutureWatcherBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFutureWatcherBase9isStartedEv()};
    unsafe {_ZNK18QFutureWatcherBase9isStartedEv()};
    return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn paused<T: QFutureWatcherBase_paused>(&mut self, value: T) -> i32 {
    value.paused(self);
    return 1;
  }
}

pub trait QFutureWatcherBase_paused {
  fn paused(self, this: &mut QFutureWatcherBase) -> i32;
}

// proto: void QFutureWatcherBase::paused();
impl<'a> /*trait*/ QFutureWatcherBase_paused for () {
  fn paused(self, this: &mut QFutureWatcherBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase6pausedEv()};
    unsafe {_ZN18QFutureWatcherBase6pausedEv()};
    return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn started<T: QFutureWatcherBase_started>(&mut self, value: T) -> i32 {
    value.started(self);
    return 1;
  }
}

pub trait QFutureWatcherBase_started {
  fn started(self, this: &mut QFutureWatcherBase) -> i32;
}

// proto: void QFutureWatcherBase::started();
impl<'a> /*trait*/ QFutureWatcherBase_started for () {
  fn started(self, this: &mut QFutureWatcherBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase7startedEv()};
    unsafe {_ZN18QFutureWatcherBase7startedEv()};
    return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn setPendingResultsLimit<T: QFutureWatcherBase_setPendingResultsLimit>(&mut self, value: T) -> i32 {
    value.setPendingResultsLimit(self);
    return 1;
  }
}

pub trait QFutureWatcherBase_setPendingResultsLimit {
  fn setPendingResultsLimit(self, this: &mut QFutureWatcherBase) -> i32;
}

// proto: void QFutureWatcherBase::setPendingResultsLimit(int limit);
impl<'a> /*trait*/ QFutureWatcherBase_setPendingResultsLimit for (i32) {
  fn setPendingResultsLimit(self, this: &mut QFutureWatcherBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase22setPendingResultsLimitEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN18QFutureWatcherBase22setPendingResultsLimitEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn resumed<T: QFutureWatcherBase_resumed>(&mut self, value: T) -> i32 {
    value.resumed(self);
    return 1;
  }
}

pub trait QFutureWatcherBase_resumed {
  fn resumed(self, this: &mut QFutureWatcherBase) -> i32;
}

// proto: void QFutureWatcherBase::resumed();
impl<'a> /*trait*/ QFutureWatcherBase_resumed for () {
  fn resumed(self, this: &mut QFutureWatcherBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase7resumedEv()};
    unsafe {_ZN18QFutureWatcherBase7resumedEv()};
    return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn cancel<T: QFutureWatcherBase_cancel>(&mut self, value: T) -> i32 {
    value.cancel(self);
    return 1;
  }
}

pub trait QFutureWatcherBase_cancel {
  fn cancel(self, this: &mut QFutureWatcherBase) -> i32;
}

// proto: void QFutureWatcherBase::cancel();
impl<'a> /*trait*/ QFutureWatcherBase_cancel for () {
  fn cancel(self, this: &mut QFutureWatcherBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase6cancelEv()};
    unsafe {_ZN18QFutureWatcherBase6cancelEv()};
    return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn progressRangeChanged<T: QFutureWatcherBase_progressRangeChanged>(&mut self, value: T) -> i32 {
    value.progressRangeChanged(self);
    return 1;
  }
}

pub trait QFutureWatcherBase_progressRangeChanged {
  fn progressRangeChanged(self, this: &mut QFutureWatcherBase) -> i32;
}

// proto: void QFutureWatcherBase::progressRangeChanged(int minimum, int maximum);
impl<'a> /*trait*/ QFutureWatcherBase_progressRangeChanged for (i32, i32) {
  fn progressRangeChanged(self, this: &mut QFutureWatcherBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase20progressRangeChangedEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN18QFutureWatcherBase20progressRangeChangedEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn canceled<T: QFutureWatcherBase_canceled>(&mut self, value: T) -> i32 {
    value.canceled(self);
    return 1;
  }
}

pub trait QFutureWatcherBase_canceled {
  fn canceled(self, this: &mut QFutureWatcherBase) -> i32;
}

// proto: void QFutureWatcherBase::canceled();
impl<'a> /*trait*/ QFutureWatcherBase_canceled for () {
  fn canceled(self, this: &mut QFutureWatcherBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase8canceledEv()};
    unsafe {_ZN18QFutureWatcherBase8canceledEv()};
    return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn isPaused<T: QFutureWatcherBase_isPaused>(&mut self, value: T) -> i32 {
    value.isPaused(self);
    return 1;
  }
}

pub trait QFutureWatcherBase_isPaused {
  fn isPaused(self, this: &mut QFutureWatcherBase) -> i32;
}

// proto: bool QFutureWatcherBase::isPaused();
impl<'a> /*trait*/ QFutureWatcherBase_isPaused for () {
  fn isPaused(self, this: &mut QFutureWatcherBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFutureWatcherBase8isPausedEv()};
    unsafe {_ZNK18QFutureWatcherBase8isPausedEv()};
    return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn pause<T: QFutureWatcherBase_pause>(&mut self, value: T) -> i32 {
    value.pause(self);
    return 1;
  }
}

pub trait QFutureWatcherBase_pause {
  fn pause(self, this: &mut QFutureWatcherBase) -> i32;
}

// proto: void QFutureWatcherBase::pause();
impl<'a> /*trait*/ QFutureWatcherBase_pause for () {
  fn pause(self, this: &mut QFutureWatcherBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase5pauseEv()};
    unsafe {_ZN18QFutureWatcherBase5pauseEv()};
    return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn progressText<T: QFutureWatcherBase_progressText>(&mut self, value: T) -> i32 {
    value.progressText(self);
    return 1;
  }
}

pub trait QFutureWatcherBase_progressText {
  fn progressText(self, this: &mut QFutureWatcherBase) -> i32;
}

// proto: QString QFutureWatcherBase::progressText();
impl<'a> /*trait*/ QFutureWatcherBase_progressText for () {
  fn progressText(self, this: &mut QFutureWatcherBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFutureWatcherBase12progressTextEv()};
    unsafe {_ZNK18QFutureWatcherBase12progressTextEv()};
    return 1;
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
  pub fn togglePaused<T: QFutureWatcherBase_togglePaused>(&mut self, value: T) -> i32 {
    value.togglePaused(self);
    return 1;
  }
}

pub trait QFutureWatcherBase_togglePaused {
  fn togglePaused(self, this: &mut QFutureWatcherBase) -> i32;
}

// proto: void QFutureWatcherBase::togglePaused();
impl<'a> /*trait*/ QFutureWatcherBase_togglePaused for () {
  fn togglePaused(self, this: &mut QFutureWatcherBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase12togglePausedEv()};
    unsafe {_ZN18QFutureWatcherBase12togglePausedEv()};
    return 1;
  }
}

impl /*struct*/ QFutureWatcherBase {
  pub fn waitForFinished<T: QFutureWatcherBase_waitForFinished>(&mut self, value: T) -> i32 {
    value.waitForFinished(self);
    return 1;
  }
}

pub trait QFutureWatcherBase_waitForFinished {
  fn waitForFinished(self, this: &mut QFutureWatcherBase) -> i32;
}

// proto: void QFutureWatcherBase::waitForFinished();
impl<'a> /*trait*/ QFutureWatcherBase_waitForFinished for () {
  fn waitForFinished(self, this: &mut QFutureWatcherBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFutureWatcherBase15waitForFinishedEv()};
    unsafe {_ZN18QFutureWatcherBase15waitForFinishedEv()};
    return 1;
  }
}

