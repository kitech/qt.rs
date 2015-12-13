// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;
use super::qstate::QState;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN16QEventTransitionC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN16QEventTransitionD0Ev() -> i32;
  fn _ZN16QEventTransition14setEventSourceEP7QObject(arg0: *mut c_void) -> i32;
  fn _ZN16QEventTransitionC1EP6QState(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZNK16QEventTransition10metaObjectEv() -> i32;
  fn _ZNK16QEventTransition11eventSourceEv() -> i32;
}

// body block begin
// class sizeof(QEventTransition)=1
pub struct QEventTransition {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QEventTransition {
  pub fn NewQEventTransition<T: QEventTransition_NewQEventTransition>(value: T) -> QEventTransition {
    let rsthis = value.NewQEventTransition();
    return rsthis;
    // return 1;
  }
}

pub trait QEventTransition_NewQEventTransition {
  fn NewQEventTransition(self) -> QEventTransition;
}

// proto: void QEventTransition::NewQEventTransition(const QEventTransition & );
impl<'a> /*trait*/ QEventTransition_NewQEventTransition for (&'a  QEventTransition) {
  fn NewQEventTransition(self) -> QEventTransition {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QEventTransitionC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QEventTransitionC1ERKS_(qthis, arg0)};
    let rsthis = QEventTransition{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QEventTransition {
  pub fn FreeQEventTransition<T: QEventTransition_FreeQEventTransition>(&mut self, value: T) -> i32 {
    value.FreeQEventTransition(self);
    return 1;
  }
}

pub trait QEventTransition_FreeQEventTransition {
  fn FreeQEventTransition(self, this: &mut QEventTransition) -> i32;
}

// proto: void QEventTransition::FreeQEventTransition();
impl<'a> /*trait*/ QEventTransition_FreeQEventTransition for () {
  fn FreeQEventTransition(self, this: &mut QEventTransition) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QEventTransitionD0Ev()};
    unsafe {_ZN16QEventTransitionD0Ev()};
    return 1;
  }
}

impl /*struct*/ QEventTransition {
  pub fn setEventSource<T: QEventTransition_setEventSource>(&mut self, value: T) -> i32 {
    value.setEventSource(self);
    return 1;
  }
}

pub trait QEventTransition_setEventSource {
  fn setEventSource(self, this: &mut QEventTransition) -> i32;
}

// proto: void QEventTransition::setEventSource(QObject * object);
impl<'a> /*trait*/ QEventTransition_setEventSource for (&'a mut QObject) {
  fn setEventSource(self, this: &mut QEventTransition) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QEventTransition14setEventSourceEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QEventTransition14setEventSourceEP7QObject(arg0)};
    return 1;
  }
}

// proto: void QEventTransition::NewQEventTransition(QState * sourceState);
impl<'a> /*trait*/ QEventTransition_NewQEventTransition for (&'a mut QState) {
  fn NewQEventTransition(self) -> QEventTransition {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QEventTransitionC1EP6QState()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QEventTransitionC1EP6QState(qthis, arg0)};
    let rsthis = QEventTransition{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QEventTransition {
  pub fn metaObject<T: QEventTransition_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QEventTransition_metaObject {
  fn metaObject(self, this: &mut QEventTransition) -> i32;
}

// proto: const QMetaObject * QEventTransition::metaObject();
impl<'a> /*trait*/ QEventTransition_metaObject for () {
  fn metaObject(self, this: &mut QEventTransition) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QEventTransition10metaObjectEv()};
    unsafe {_ZNK16QEventTransition10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QEventTransition {
  pub fn eventSource<T: QEventTransition_eventSource>(&mut self, value: T) -> i32 {
    value.eventSource(self);
    return 1;
  }
}

pub trait QEventTransition_eventSource {
  fn eventSource(self, this: &mut QEventTransition) -> i32;
}

// proto: QObject * QEventTransition::eventSource();
impl<'a> /*trait*/ QEventTransition_eventSource for () {
  fn eventSource(self, this: &mut QEventTransition) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QEventTransition11eventSourceEv()};
    unsafe {_ZNK16QEventTransition11eventSourceEv()};
    return 1;
  }
}

