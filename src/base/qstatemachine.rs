// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qevent::QEvent;
use super::qobject::QObject;
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QList<QAbstractAnimation *> QStateMachine::defaultAnimations();
  fn _ZNK13QStateMachine17defaultAnimationsEv(qthis: *mut c_void) ;
  // proto:  int QStateMachine::postDelayedEvent(QEvent * event, int delay);
  fn _ZN13QStateMachine16postDelayedEventEP6QEventi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  QSet<QAbstractState *> QStateMachine::configuration();
  fn _ZNK13QStateMachine13configurationEv(qthis: *mut c_void) ;
  // proto:  void QStateMachine::setRunning(bool running);
  fn _ZN13QStateMachine10setRunningEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QStateMachine::setAnimated(bool enabled);
  fn _ZN13QStateMachine11setAnimatedEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QStateMachine::NewQStateMachine(QObject * parent);
  fn _ZN13QStateMachineC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QStateMachine::errorString();
  fn _ZNK13QStateMachine11errorStringEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QStateMachine::isRunning();
  fn _ZNK13QStateMachine9isRunningEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QStateMachine::cancelDelayedEvent(int id);
  fn _ZN13QStateMachine18cancelDelayedEventEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  void QStateMachine::FreeQStateMachine();
  fn _ZN13QStateMachineD0Ev(qthis: *mut c_void) ;
  // proto:  const QMetaObject * QStateMachine::metaObject();
  fn _ZNK13QStateMachine10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QStateMachine::runningChanged(bool running);
  fn _ZN13QStateMachine14runningChangedEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QStateMachine::clearError();
  fn _ZN13QStateMachine10clearErrorEv(qthis: *mut c_void) ;
  // proto:  void QStateMachine::stop();
  fn _ZN13QStateMachine4stopEv(qthis: *mut c_void) ;
  // proto:  bool QStateMachine::isAnimated();
  fn _ZNK13QStateMachine10isAnimatedEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QStateMachine::start();
  fn _ZN13QStateMachine5startEv(qthis: *mut c_void) ;
  // proto:  bool QStateMachine::eventFilter(QObject * watched, QEvent * event);
  fn _ZN13QStateMachine11eventFilterEP7QObjectP6QEvent(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> int8_t;
  // proto:  void QStateMachine::NewQStateMachine(const QStateMachine & );
  fn _ZN13QStateMachineC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QStateMachine)=1
pub struct QStateMachine {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStateMachine {
  pub fn defaultAnimations<T: QStateMachine_defaultAnimations>(&mut self, value: T)  {
     value.defaultAnimations(self);
    // return 1;
  }
}

pub trait QStateMachine_defaultAnimations {
  fn defaultAnimations(self, rsthis: &mut QStateMachine) ;
}

// proto:  QList<QAbstractAnimation *> QStateMachine::defaultAnimations();
impl<'a> /*trait*/ QStateMachine_defaultAnimations for () {
  fn defaultAnimations(self, rsthis: &mut QStateMachine)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStateMachine17defaultAnimationsEv()};
     unsafe {_ZNK13QStateMachine17defaultAnimationsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QStateMachine {
  pub fn postDelayedEvent<T: QStateMachine_postDelayedEvent>(&mut self, value: T) -> i32 {
    return value.postDelayedEvent(self);
    // return 1;
  }
}

pub trait QStateMachine_postDelayedEvent {
  fn postDelayedEvent(self, rsthis: &mut QStateMachine) -> i32;
}

// proto:  int QStateMachine::postDelayedEvent(QEvent * event, int delay);
impl<'a> /*trait*/ QStateMachine_postDelayedEvent for (&'a mut QEvent, i32) {
  fn postDelayedEvent(self, rsthis: &mut QStateMachine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine16postDelayedEventEP6QEventi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN13QStateMachine16postDelayedEventEP6QEventi(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QStateMachine {
  pub fn configuration<T: QStateMachine_configuration>(&mut self, value: T)  {
     value.configuration(self);
    // return 1;
  }
}

pub trait QStateMachine_configuration {
  fn configuration(self, rsthis: &mut QStateMachine) ;
}

// proto:  QSet<QAbstractState *> QStateMachine::configuration();
impl<'a> /*trait*/ QStateMachine_configuration for () {
  fn configuration(self, rsthis: &mut QStateMachine)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStateMachine13configurationEv()};
     unsafe {_ZNK13QStateMachine13configurationEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QStateMachine {
  pub fn setRunning<T: QStateMachine_setRunning>(&mut self, value: T)  {
     value.setRunning(self);
    // return 1;
  }
}

pub trait QStateMachine_setRunning {
  fn setRunning(self, rsthis: &mut QStateMachine) ;
}

// proto:  void QStateMachine::setRunning(bool running);
impl<'a> /*trait*/ QStateMachine_setRunning for (i8) {
  fn setRunning(self, rsthis: &mut QStateMachine)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine10setRunningEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN13QStateMachine10setRunningEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStateMachine {
  pub fn setAnimated<T: QStateMachine_setAnimated>(&mut self, value: T)  {
     value.setAnimated(self);
    // return 1;
  }
}

pub trait QStateMachine_setAnimated {
  fn setAnimated(self, rsthis: &mut QStateMachine) ;
}

// proto:  void QStateMachine::setAnimated(bool enabled);
impl<'a> /*trait*/ QStateMachine_setAnimated for (i8) {
  fn setAnimated(self, rsthis: &mut QStateMachine)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine11setAnimatedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN13QStateMachine11setAnimatedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStateMachine {
  pub fn NewQStateMachine<T: QStateMachine_NewQStateMachine>(value: T) -> QStateMachine {
    let rsthis = value.NewQStateMachine();
    return rsthis;
    // return 1;
  }
}

pub trait QStateMachine_NewQStateMachine {
  fn NewQStateMachine(self) -> QStateMachine;
}

// proto: void QStateMachine::NewQStateMachine(QObject * parent);
impl<'a> /*trait*/ QStateMachine_NewQStateMachine for (&'a mut QObject) {
  fn NewQStateMachine(self) -> QStateMachine {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachineC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QStateMachineC1EP7QObject(qthis, arg0)};
    let rsthis = QStateMachine{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStateMachine {
  pub fn errorString<T: QStateMachine_errorString>(&mut self, value: T) -> QString {
    return value.errorString(self);
    // return 1;
  }
}

pub trait QStateMachine_errorString {
  fn errorString(self, rsthis: &mut QStateMachine) -> QString;
}

// proto:  QString QStateMachine::errorString();
impl<'a> /*trait*/ QStateMachine_errorString for () {
  fn errorString(self, rsthis: &mut QStateMachine) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStateMachine11errorStringEv()};
    let mut ret = unsafe {_ZNK13QStateMachine11errorStringEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStateMachine {
  pub fn isRunning<T: QStateMachine_isRunning>(&mut self, value: T) -> i8 {
    return value.isRunning(self);
    // return 1;
  }
}

pub trait QStateMachine_isRunning {
  fn isRunning(self, rsthis: &mut QStateMachine) -> i8;
}

// proto:  bool QStateMachine::isRunning();
impl<'a> /*trait*/ QStateMachine_isRunning for () {
  fn isRunning(self, rsthis: &mut QStateMachine) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStateMachine9isRunningEv()};
    let mut ret = unsafe {_ZNK13QStateMachine9isRunningEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QStateMachine {
  pub fn cancelDelayedEvent<T: QStateMachine_cancelDelayedEvent>(&mut self, value: T) -> i8 {
    return value.cancelDelayedEvent(self);
    // return 1;
  }
}

pub trait QStateMachine_cancelDelayedEvent {
  fn cancelDelayedEvent(self, rsthis: &mut QStateMachine) -> i8;
}

// proto:  bool QStateMachine::cancelDelayedEvent(int id);
impl<'a> /*trait*/ QStateMachine_cancelDelayedEvent for (i32) {
  fn cancelDelayedEvent(self, rsthis: &mut QStateMachine) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine18cancelDelayedEventEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN13QStateMachine18cancelDelayedEventEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QStateMachine {
  pub fn FreeQStateMachine<T: QStateMachine_FreeQStateMachine>(&mut self, value: T)  {
     value.FreeQStateMachine(self);
    // return 1;
  }
}

pub trait QStateMachine_FreeQStateMachine {
  fn FreeQStateMachine(self, rsthis: &mut QStateMachine) ;
}

// proto:  void QStateMachine::FreeQStateMachine();
impl<'a> /*trait*/ QStateMachine_FreeQStateMachine for () {
  fn FreeQStateMachine(self, rsthis: &mut QStateMachine)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachineD0Ev()};
     unsafe {_ZN13QStateMachineD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QStateMachine {
  pub fn metaObject<T: QStateMachine_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QStateMachine_metaObject {
  fn metaObject(self, rsthis: &mut QStateMachine) ;
}

// proto:  const QMetaObject * QStateMachine::metaObject();
impl<'a> /*trait*/ QStateMachine_metaObject for () {
  fn metaObject(self, rsthis: &mut QStateMachine)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStateMachine10metaObjectEv()};
     unsafe {_ZNK13QStateMachine10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QStateMachine {
  pub fn runningChanged<T: QStateMachine_runningChanged>(&mut self, value: T)  {
     value.runningChanged(self);
    // return 1;
  }
}

pub trait QStateMachine_runningChanged {
  fn runningChanged(self, rsthis: &mut QStateMachine) ;
}

// proto:  void QStateMachine::runningChanged(bool running);
impl<'a> /*trait*/ QStateMachine_runningChanged for (i8) {
  fn runningChanged(self, rsthis: &mut QStateMachine)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine14runningChangedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN13QStateMachine14runningChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStateMachine {
  pub fn clearError<T: QStateMachine_clearError>(&mut self, value: T)  {
     value.clearError(self);
    // return 1;
  }
}

pub trait QStateMachine_clearError {
  fn clearError(self, rsthis: &mut QStateMachine) ;
}

// proto:  void QStateMachine::clearError();
impl<'a> /*trait*/ QStateMachine_clearError for () {
  fn clearError(self, rsthis: &mut QStateMachine)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine10clearErrorEv()};
     unsafe {_ZN13QStateMachine10clearErrorEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QStateMachine {
  pub fn stop<T: QStateMachine_stop>(&mut self, value: T)  {
     value.stop(self);
    // return 1;
  }
}

pub trait QStateMachine_stop {
  fn stop(self, rsthis: &mut QStateMachine) ;
}

// proto:  void QStateMachine::stop();
impl<'a> /*trait*/ QStateMachine_stop for () {
  fn stop(self, rsthis: &mut QStateMachine)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine4stopEv()};
     unsafe {_ZN13QStateMachine4stopEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QStateMachine {
  pub fn isAnimated<T: QStateMachine_isAnimated>(&mut self, value: T) -> i8 {
    return value.isAnimated(self);
    // return 1;
  }
}

pub trait QStateMachine_isAnimated {
  fn isAnimated(self, rsthis: &mut QStateMachine) -> i8;
}

// proto:  bool QStateMachine::isAnimated();
impl<'a> /*trait*/ QStateMachine_isAnimated for () {
  fn isAnimated(self, rsthis: &mut QStateMachine) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStateMachine10isAnimatedEv()};
    let mut ret = unsafe {_ZNK13QStateMachine10isAnimatedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QStateMachine {
  pub fn start<T: QStateMachine_start>(&mut self, value: T)  {
     value.start(self);
    // return 1;
  }
}

pub trait QStateMachine_start {
  fn start(self, rsthis: &mut QStateMachine) ;
}

// proto:  void QStateMachine::start();
impl<'a> /*trait*/ QStateMachine_start for () {
  fn start(self, rsthis: &mut QStateMachine)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine5startEv()};
     unsafe {_ZN13QStateMachine5startEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QStateMachine {
  pub fn eventFilter<T: QStateMachine_eventFilter>(&mut self, value: T) -> i8 {
    return value.eventFilter(self);
    // return 1;
  }
}

pub trait QStateMachine_eventFilter {
  fn eventFilter(self, rsthis: &mut QStateMachine) -> i8;
}

// proto:  bool QStateMachine::eventFilter(QObject * watched, QEvent * event);
impl<'a> /*trait*/ QStateMachine_eventFilter for (&'a mut QObject, &'a mut QEvent) {
  fn eventFilter(self, rsthis: &mut QStateMachine) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine11eventFilterEP7QObjectP6QEvent()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN13QStateMachine11eventFilterEP7QObjectP6QEvent(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QStateMachine::NewQStateMachine(const QStateMachine & );
impl<'a> /*trait*/ QStateMachine_NewQStateMachine for (&'a  QStateMachine) {
  fn NewQStateMachine(self) -> QStateMachine {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachineC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QStateMachineC1ERKS_(qthis, arg0)};
    let rsthis = QStateMachine{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

