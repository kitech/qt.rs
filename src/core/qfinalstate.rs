// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
// src-file: /QtCore/qfinalstate.h
// dst-file: /src/core/qfinalstate.rs
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
use super::qabstractstate::QAbstractState; // 773
use std::ops::Deref;
use super::qstate::QState; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QFinalState::QFinalState(QState * parent);
  fn _ZN11QFinalStateC1EP6QState(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QFinalState::QFinalState(const QFinalState & );
  fn _ZN11QFinalStateC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QFinalState::~QFinalState();
  fn _ZN11QFinalStateD0Ev(qthis: *mut c_void);
  // proto:  const QMetaObject * QFinalState::metaObject();
  fn _ZNK11QFinalState10metaObjectEv(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QFinalState)=1
pub struct QFinalState {
  qbase: QAbstractState,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFinalState {
  pub fn inheritFrom(qthis: *mut c_void) -> QFinalState {
    return QFinalState{qbase: QAbstractState::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QFinalState {
  type Target = QAbstractState;

  fn deref(&self) -> &QAbstractState {
    return &self.qbase;
  }
}
impl AsRef<QAbstractState> for QFinalState {
  fn as_ref(&self) -> &QAbstractState {
    return &self.qbase;
  }
}
  // proto:  void QFinalState::QFinalState(QState * parent);
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

  // proto:  void QFinalState::QFinalState(QState * parent);
impl<'a> /*trait*/ QFinalState_NewQFinalState for (QState) {
  fn NewQFinalState(self) -> QFinalState {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFinalStateC1EP6QState()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QFinalStateC1EP6QState(qthis, arg0)};
    let rsthis = QFinalState{/**/qbase: QAbstractState::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QFinalState::QFinalState(const QFinalState & );
impl<'a> /*trait*/ QFinalState_NewQFinalState for (QFinalState) {
  fn NewQFinalState(self) -> QFinalState {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFinalStateC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QFinalStateC1ERKS_(qthis, arg0)};
    let rsthis = QFinalState{/**/qbase: QAbstractState::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QFinalState::~QFinalState();
impl /*struct*/ QFinalState {
  pub fn FreeQFinalState<RetType, T: QFinalState_FreeQFinalState<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQFinalState(self);
    // return 1;
  }
}

pub trait QFinalState_FreeQFinalState<RetType> {
  fn FreeQFinalState(self , rsthis: &mut QFinalState) -> RetType;
}

  // proto:  void QFinalState::~QFinalState();
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
  pub fn metaObject<RetType, T: QFinalState_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
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

// <= body block end

