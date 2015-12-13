// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstate::QState;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZNK13QHistoryState10metaObjectEv() -> i32;
  fn _ZN13QHistoryStateC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN13QHistoryStateC1EP6QState(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZNK13QHistoryState12defaultStateEv() -> i32;
  fn _ZN13QHistoryStateD0Ev() -> i32;
}

// body block begin
// class sizeof(QHistoryState)=1
pub struct QHistoryState {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QHistoryState {
  pub fn metaObject<T: QHistoryState_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QHistoryState_metaObject {
  fn metaObject(self, this: &mut QHistoryState) -> i32;
}

// proto: const QMetaObject * QHistoryState::metaObject();
impl<'a> /*trait*/ QHistoryState_metaObject for () {
  fn metaObject(self, this: &mut QHistoryState) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QHistoryState10metaObjectEv()};
    unsafe {_ZNK13QHistoryState10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QHistoryState {
  pub fn NewQHistoryState<T: QHistoryState_NewQHistoryState>(value: T) -> QHistoryState {
    let rsthis = value.NewQHistoryState();
    return rsthis;
    // return 1;
  }
}

pub trait QHistoryState_NewQHistoryState {
  fn NewQHistoryState(self) -> QHistoryState;
}

// proto: void QHistoryState::NewQHistoryState(const QHistoryState & );
impl<'a> /*trait*/ QHistoryState_NewQHistoryState for (&'a  QHistoryState) {
  fn NewQHistoryState(self) -> QHistoryState {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QHistoryStateC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QHistoryStateC1ERKS_(qthis, arg0)};
    let rsthis = QHistoryState{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QHistoryState::NewQHistoryState(QState * parent);
impl<'a> /*trait*/ QHistoryState_NewQHistoryState for (&'a mut QState) {
  fn NewQHistoryState(self) -> QHistoryState {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QHistoryStateC1EP6QState()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QHistoryStateC1EP6QState(qthis, arg0)};
    let rsthis = QHistoryState{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QHistoryState {
  pub fn defaultState<T: QHistoryState_defaultState>(&mut self, value: T) -> i32 {
    value.defaultState(self);
    return 1;
  }
}

pub trait QHistoryState_defaultState {
  fn defaultState(self, this: &mut QHistoryState) -> i32;
}

// proto: QAbstractState * QHistoryState::defaultState();
impl<'a> /*trait*/ QHistoryState_defaultState for () {
  fn defaultState(self, this: &mut QHistoryState) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QHistoryState12defaultStateEv()};
    unsafe {_ZNK13QHistoryState12defaultStateEv()};
    return 1;
  }
}

impl /*struct*/ QHistoryState {
  pub fn FreeQHistoryState<T: QHistoryState_FreeQHistoryState>(&mut self, value: T) -> i32 {
    value.FreeQHistoryState(self);
    return 1;
  }
}

pub trait QHistoryState_FreeQHistoryState {
  fn FreeQHistoryState(self, this: &mut QHistoryState) -> i32;
}

// proto: void QHistoryState::FreeQHistoryState();
impl<'a> /*trait*/ QHistoryState_FreeQHistoryState for () {
  fn FreeQHistoryState(self, this: &mut QHistoryState) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QHistoryStateD0Ev()};
    unsafe {_ZN13QHistoryStateD0Ev()};
    return 1;
  }
}

