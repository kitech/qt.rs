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

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZNK13QStateMachine17defaultAnimationsEv() -> i32;
  fn _ZN13QStateMachine16postDelayedEventEP6QEventi(arg0: *mut c_void, arg1: c_int) -> i32;
  fn _ZNK13QStateMachine13configurationEv() -> i32;
  fn _ZN13QStateMachine10setRunningEb(arg0: int8_t) -> i32;
  fn _ZN13QStateMachine11setAnimatedEb(arg0: int8_t) -> i32;
  fn _ZN13QStateMachineC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZNK13QStateMachine11errorStringEv() -> i32;
  fn _ZNK13QStateMachine9isRunningEv() -> i32;
  fn _ZN13QStateMachine18cancelDelayedEventEi(arg0: c_int) -> i32;
  fn _ZN13QStateMachineD0Ev() -> i32;
  fn _ZNK13QStateMachine10metaObjectEv() -> i32;
  fn _ZN13QStateMachine14runningChangedEb(arg0: int8_t) -> i32;
  fn _ZN13QStateMachine10clearErrorEv() -> i32;
  fn _ZN13QStateMachine4stopEv() -> i32;
  fn _ZNK13QStateMachine10isAnimatedEv() -> i32;
  fn _ZN13QStateMachine5startEv() -> i32;
  fn _ZN13QStateMachine11eventFilterEP7QObjectP6QEvent(arg0: *mut c_void, arg1: *mut c_void) -> i32;
  fn _ZN13QStateMachineC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QStateMachine)=1
pub struct QStateMachine {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStateMachine {
  pub fn defaultAnimations<T: QStateMachine_defaultAnimations>(&mut self, value: T) -> i32 {
    value.defaultAnimations(self);
    return 1;
  }
}

pub trait QStateMachine_defaultAnimations {
  fn defaultAnimations(self, this: &mut QStateMachine) -> i32;
}

// proto: QList<QAbstractAnimation *> QStateMachine::defaultAnimations();
impl<'a> /*trait*/ QStateMachine_defaultAnimations for () {
  fn defaultAnimations(self, this: &mut QStateMachine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStateMachine17defaultAnimationsEv()};
    unsafe {_ZNK13QStateMachine17defaultAnimationsEv()};
    return 1;
  }
}

impl /*struct*/ QStateMachine {
  pub fn postDelayedEvent<T: QStateMachine_postDelayedEvent>(&mut self, value: T) -> i32 {
    value.postDelayedEvent(self);
    return 1;
  }
}

pub trait QStateMachine_postDelayedEvent {
  fn postDelayedEvent(self, this: &mut QStateMachine) -> i32;
}

// proto: int QStateMachine::postDelayedEvent(QEvent * event, int delay);
impl<'a> /*trait*/ QStateMachine_postDelayedEvent for (&'a mut QEvent, i32) {
  fn postDelayedEvent(self, this: &mut QStateMachine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine16postDelayedEventEP6QEventi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN13QStateMachine16postDelayedEventEP6QEventi(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QStateMachine {
  pub fn configuration<T: QStateMachine_configuration>(&mut self, value: T) -> i32 {
    value.configuration(self);
    return 1;
  }
}

pub trait QStateMachine_configuration {
  fn configuration(self, this: &mut QStateMachine) -> i32;
}

// proto: QSet<QAbstractState *> QStateMachine::configuration();
impl<'a> /*trait*/ QStateMachine_configuration for () {
  fn configuration(self, this: &mut QStateMachine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStateMachine13configurationEv()};
    unsafe {_ZNK13QStateMachine13configurationEv()};
    return 1;
  }
}

impl /*struct*/ QStateMachine {
  pub fn setRunning<T: QStateMachine_setRunning>(&mut self, value: T) -> i32 {
    value.setRunning(self);
    return 1;
  }
}

pub trait QStateMachine_setRunning {
  fn setRunning(self, this: &mut QStateMachine) -> i32;
}

// proto: void QStateMachine::setRunning(bool running);
impl<'a> /*trait*/ QStateMachine_setRunning for (i8) {
  fn setRunning(self, this: &mut QStateMachine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine10setRunningEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN13QStateMachine10setRunningEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QStateMachine {
  pub fn setAnimated<T: QStateMachine_setAnimated>(&mut self, value: T) -> i32 {
    value.setAnimated(self);
    return 1;
  }
}

pub trait QStateMachine_setAnimated {
  fn setAnimated(self, this: &mut QStateMachine) -> i32;
}

// proto: void QStateMachine::setAnimated(bool enabled);
impl<'a> /*trait*/ QStateMachine_setAnimated for (i8) {
  fn setAnimated(self, this: &mut QStateMachine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine11setAnimatedEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN13QStateMachine11setAnimatedEb(arg0)};
    return 1;
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
  pub fn errorString<T: QStateMachine_errorString>(&mut self, value: T) -> i32 {
    value.errorString(self);
    return 1;
  }
}

pub trait QStateMachine_errorString {
  fn errorString(self, this: &mut QStateMachine) -> i32;
}

// proto: QString QStateMachine::errorString();
impl<'a> /*trait*/ QStateMachine_errorString for () {
  fn errorString(self, this: &mut QStateMachine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStateMachine11errorStringEv()};
    unsafe {_ZNK13QStateMachine11errorStringEv()};
    return 1;
  }
}

impl /*struct*/ QStateMachine {
  pub fn isRunning<T: QStateMachine_isRunning>(&mut self, value: T) -> i32 {
    value.isRunning(self);
    return 1;
  }
}

pub trait QStateMachine_isRunning {
  fn isRunning(self, this: &mut QStateMachine) -> i32;
}

// proto: bool QStateMachine::isRunning();
impl<'a> /*trait*/ QStateMachine_isRunning for () {
  fn isRunning(self, this: &mut QStateMachine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStateMachine9isRunningEv()};
    unsafe {_ZNK13QStateMachine9isRunningEv()};
    return 1;
  }
}

impl /*struct*/ QStateMachine {
  pub fn cancelDelayedEvent<T: QStateMachine_cancelDelayedEvent>(&mut self, value: T) -> i32 {
    value.cancelDelayedEvent(self);
    return 1;
  }
}

pub trait QStateMachine_cancelDelayedEvent {
  fn cancelDelayedEvent(self, this: &mut QStateMachine) -> i32;
}

// proto: bool QStateMachine::cancelDelayedEvent(int id);
impl<'a> /*trait*/ QStateMachine_cancelDelayedEvent for (i32) {
  fn cancelDelayedEvent(self, this: &mut QStateMachine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine18cancelDelayedEventEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN13QStateMachine18cancelDelayedEventEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QStateMachine {
  pub fn FreeQStateMachine<T: QStateMachine_FreeQStateMachine>(&mut self, value: T) -> i32 {
    value.FreeQStateMachine(self);
    return 1;
  }
}

pub trait QStateMachine_FreeQStateMachine {
  fn FreeQStateMachine(self, this: &mut QStateMachine) -> i32;
}

// proto: void QStateMachine::FreeQStateMachine();
impl<'a> /*trait*/ QStateMachine_FreeQStateMachine for () {
  fn FreeQStateMachine(self, this: &mut QStateMachine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachineD0Ev()};
    unsafe {_ZN13QStateMachineD0Ev()};
    return 1;
  }
}

impl /*struct*/ QStateMachine {
  pub fn metaObject<T: QStateMachine_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QStateMachine_metaObject {
  fn metaObject(self, this: &mut QStateMachine) -> i32;
}

// proto: const QMetaObject * QStateMachine::metaObject();
impl<'a> /*trait*/ QStateMachine_metaObject for () {
  fn metaObject(self, this: &mut QStateMachine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStateMachine10metaObjectEv()};
    unsafe {_ZNK13QStateMachine10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QStateMachine {
  pub fn runningChanged<T: QStateMachine_runningChanged>(&mut self, value: T) -> i32 {
    value.runningChanged(self);
    return 1;
  }
}

pub trait QStateMachine_runningChanged {
  fn runningChanged(self, this: &mut QStateMachine) -> i32;
}

// proto: void QStateMachine::runningChanged(bool running);
impl<'a> /*trait*/ QStateMachine_runningChanged for (i8) {
  fn runningChanged(self, this: &mut QStateMachine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine14runningChangedEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN13QStateMachine14runningChangedEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QStateMachine {
  pub fn clearError<T: QStateMachine_clearError>(&mut self, value: T) -> i32 {
    value.clearError(self);
    return 1;
  }
}

pub trait QStateMachine_clearError {
  fn clearError(self, this: &mut QStateMachine) -> i32;
}

// proto: void QStateMachine::clearError();
impl<'a> /*trait*/ QStateMachine_clearError for () {
  fn clearError(self, this: &mut QStateMachine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine10clearErrorEv()};
    unsafe {_ZN13QStateMachine10clearErrorEv()};
    return 1;
  }
}

impl /*struct*/ QStateMachine {
  pub fn stop<T: QStateMachine_stop>(&mut self, value: T) -> i32 {
    value.stop(self);
    return 1;
  }
}

pub trait QStateMachine_stop {
  fn stop(self, this: &mut QStateMachine) -> i32;
}

// proto: void QStateMachine::stop();
impl<'a> /*trait*/ QStateMachine_stop for () {
  fn stop(self, this: &mut QStateMachine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine4stopEv()};
    unsafe {_ZN13QStateMachine4stopEv()};
    return 1;
  }
}

impl /*struct*/ QStateMachine {
  pub fn isAnimated<T: QStateMachine_isAnimated>(&mut self, value: T) -> i32 {
    value.isAnimated(self);
    return 1;
  }
}

pub trait QStateMachine_isAnimated {
  fn isAnimated(self, this: &mut QStateMachine) -> i32;
}

// proto: bool QStateMachine::isAnimated();
impl<'a> /*trait*/ QStateMachine_isAnimated for () {
  fn isAnimated(self, this: &mut QStateMachine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStateMachine10isAnimatedEv()};
    unsafe {_ZNK13QStateMachine10isAnimatedEv()};
    return 1;
  }
}

impl /*struct*/ QStateMachine {
  pub fn start<T: QStateMachine_start>(&mut self, value: T) -> i32 {
    value.start(self);
    return 1;
  }
}

pub trait QStateMachine_start {
  fn start(self, this: &mut QStateMachine) -> i32;
}

// proto: void QStateMachine::start();
impl<'a> /*trait*/ QStateMachine_start for () {
  fn start(self, this: &mut QStateMachine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine5startEv()};
    unsafe {_ZN13QStateMachine5startEv()};
    return 1;
  }
}

impl /*struct*/ QStateMachine {
  pub fn eventFilter<T: QStateMachine_eventFilter>(&mut self, value: T) -> i32 {
    value.eventFilter(self);
    return 1;
  }
}

pub trait QStateMachine_eventFilter {
  fn eventFilter(self, this: &mut QStateMachine) -> i32;
}

// proto: bool QStateMachine::eventFilter(QObject * watched, QEvent * event);
impl<'a> /*trait*/ QStateMachine_eventFilter for (&'a mut QObject, &'a mut QEvent) {
  fn eventFilter(self, this: &mut QStateMachine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachine11eventFilterEP7QObjectP6QEvent()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN13QStateMachine11eventFilterEP7QObjectP6QEvent(arg0, arg1)};
    return 1;
  }
}

// proto: void QStateMachine::NewQStateMachine(const QStateMachine & );
impl<'a> /*trait*/ QStateMachine_NewQStateMachine for (&'a  QStateMachine) {
  fn NewQStateMachine(self) -> QStateMachine {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStateMachineC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QStateMachineC1ERKS_(qthis, arg0)};
    let rsthis = QStateMachine{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

