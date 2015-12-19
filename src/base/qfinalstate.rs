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
  // proto:  void QFinalState::NewQFinalState(QState * parent);
  fn _ZN11QFinalStateC1EP6QState(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QFinalState::NewQFinalState(const QFinalState & );
  fn _ZN11QFinalStateC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QFinalState::FreeQFinalState();
  fn _ZN11QFinalStateD0Ev(qthis: *mut c_void) ;
  // proto:  const QMetaObject * QFinalState::metaObject();
  fn _ZNK11QFinalState10metaObjectEv(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QFinalState)=1
pub struct QFinalState {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFinalState {
  pub fn NewQFinalState<T: QFinalState_NewQFinalState>(value: T) -> QFinalState {
    let rsthis = value.NewQFinalState();
    return rsthis;
    // return 1;
  }
}

pub trait QFinalState_NewQFinalState {
  fn NewQFinalState(self) -> QFinalState;
}

// proto: void QFinalState::NewQFinalState(QState * parent);
impl<'a> /*trait*/ QFinalState_NewQFinalState for (&'a mut QState) {
  fn NewQFinalState(self) -> QFinalState {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFinalStateC1EP6QState()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QFinalStateC1EP6QState(qthis, arg0)};
    let rsthis = QFinalState{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QFinalState::NewQFinalState(const QFinalState & );
impl<'a> /*trait*/ QFinalState_NewQFinalState for (&'a  QFinalState) {
  fn NewQFinalState(self) -> QFinalState {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFinalStateC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QFinalStateC1ERKS_(qthis, arg0)};
    let rsthis = QFinalState{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QFinalState::FreeQFinalState();
impl /*struct*/ QFinalState {
  pub fn FreeQFinalState<RetType, T: QFinalState_FreeQFinalState<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQFinalState(self);
    // return 1;
  }
}

pub trait QFinalState_FreeQFinalState<RetType> {
  fn FreeQFinalState(self , rsthis: &mut QFinalState) -> RetType;
}

// proto:  void QFinalState::FreeQFinalState();
impl<'a> /*trait*/ QFinalState_FreeQFinalState<()> for () {
  fn FreeQFinalState(self , rsthis: &mut QFinalState) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFinalStateD0Ev()};
     unsafe {_ZN11QFinalStateD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  const QMetaObject * QFinalState::metaObject();
impl /*struct*/ QFinalState {
  pub fn metaObject<RetType, T: QFinalState_metaObject<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QFinalState_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QFinalState) -> RetType;
}

// proto:  const QMetaObject * QFinalState::metaObject();
impl<'a> /*trait*/ QFinalState_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QFinalState) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFinalState10metaObjectEv()};
     unsafe {_ZNK11QFinalState10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

