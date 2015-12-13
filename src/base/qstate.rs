// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;
use super::qvariant::QVariant;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZNK6QState10errorStateEv() -> i32;
  fn _ZNK6QState12initialStateEv() -> i32;
  fn _ZN6QStateD0Ev() -> i32;
  fn _ZN6QState14assignPropertyEP7QObjectPKcRK8QVariant(arg0: *mut c_void, arg1: *const c_char, arg2: *const c_void) -> i32;
  fn _ZN6QStateC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN6QStateC1EPS_(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZNK6QState10metaObjectEv() -> i32;
  fn _ZNK6QState11transitionsEv() -> i32;
}

// body block begin
// class sizeof(QState)=1
pub struct QState {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QState {
  pub fn errorState<T: QState_errorState>(&mut self, value: T) -> i32 {
    value.errorState(self);
    return 1;
  }
}

pub trait QState_errorState {
  fn errorState(self, this: &mut QState) -> i32;
}

// proto: QAbstractState * QState::errorState();
impl<'a> /*trait*/ QState_errorState for () {
  fn errorState(self, this: &mut QState) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QState10errorStateEv()};
    unsafe {_ZNK6QState10errorStateEv()};
    return 1;
  }
}

impl /*struct*/ QState {
  pub fn initialState<T: QState_initialState>(&mut self, value: T) -> i32 {
    value.initialState(self);
    return 1;
  }
}

pub trait QState_initialState {
  fn initialState(self, this: &mut QState) -> i32;
}

// proto: QAbstractState * QState::initialState();
impl<'a> /*trait*/ QState_initialState for () {
  fn initialState(self, this: &mut QState) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QState12initialStateEv()};
    unsafe {_ZNK6QState12initialStateEv()};
    return 1;
  }
}

impl /*struct*/ QState {
  pub fn FreeQState<T: QState_FreeQState>(&mut self, value: T) -> i32 {
    value.FreeQState(self);
    return 1;
  }
}

pub trait QState_FreeQState {
  fn FreeQState(self, this: &mut QState) -> i32;
}

// proto: void QState::FreeQState();
impl<'a> /*trait*/ QState_FreeQState for () {
  fn FreeQState(self, this: &mut QState) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QStateD0Ev()};
    unsafe {_ZN6QStateD0Ev()};
    return 1;
  }
}

impl /*struct*/ QState {
  pub fn assignProperty<T: QState_assignProperty>(&mut self, value: T) -> i32 {
    value.assignProperty(self);
    return 1;
  }
}

pub trait QState_assignProperty {
  fn assignProperty(self, this: &mut QState) -> i32;
}

// proto: void QState::assignProperty(QObject * object, const char * name, const QVariant & value);
impl<'a> /*trait*/ QState_assignProperty for (&'a mut QObject, &'a  String, &'a  QVariant) {
  fn assignProperty(self, this: &mut QState) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QState14assignPropertyEP7QObjectPKcRK8QVariant()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN6QState14assignPropertyEP7QObjectPKcRK8QVariant(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QState {
  pub fn NewQState<T: QState_NewQState>(value: T) -> QState {
    let rsthis = value.NewQState();
    return rsthis;
    // return 1;
  }
}

pub trait QState_NewQState {
  fn NewQState(self) -> QState;
}

// proto: void QState::NewQState(const QState & );
impl<'a> /*trait*/ QState_NewQState for (&'a  QState) {
  fn NewQState(self) -> QState {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QStateC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QStateC1ERKS_(qthis, arg0)};
    let rsthis = QState{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QState::NewQState(QState * parent);
impl<'a> /*trait*/ QState_NewQState for (&'a mut QState) {
  fn NewQState(self) -> QState {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QStateC1EPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QStateC1EPS_(qthis, arg0)};
    let rsthis = QState{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QState {
  pub fn metaObject<T: QState_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QState_metaObject {
  fn metaObject(self, this: &mut QState) -> i32;
}

// proto: const QMetaObject * QState::metaObject();
impl<'a> /*trait*/ QState_metaObject for () {
  fn metaObject(self, this: &mut QState) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QState10metaObjectEv()};
    unsafe {_ZNK6QState10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QState {
  pub fn transitions<T: QState_transitions>(&mut self, value: T) -> i32 {
    value.transitions(self);
    return 1;
  }
}

pub trait QState_transitions {
  fn transitions(self, this: &mut QState) -> i32;
}

// proto: QList<QAbstractTransition *> QState::transitions();
impl<'a> /*trait*/ QState_transitions for () {
  fn transitions(self, this: &mut QState) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QState11transitionsEv()};
    unsafe {_ZNK6QState11transitionsEv()};
    return 1;
  }
}

