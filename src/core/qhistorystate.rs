// auto generated, do not modify.
// created: Fri Jan  1 12:13:41 2016
// src-file: /QtCore/qhistorystate.h
// dst-file: /src/core/qhistorystate.rs
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
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QHistoryState_Class_Size() -> c_int;
  // proto:  const QMetaObject * QHistoryState::metaObject();
  fn _ZNK13QHistoryState10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QHistoryState::QHistoryState(const QHistoryState & );
  fn dector_ZN13QHistoryStateC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN13QHistoryStateC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QHistoryState::setDefaultState(QAbstractState * state);
  fn _ZN13QHistoryState15setDefaultStateEP14QAbstractState(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QHistoryState::QHistoryState(QState * parent);
  fn dector_ZN13QHistoryStateC1EP6QState(arg0: *mut c_void) -> *mut c_void;
  fn _ZN13QHistoryStateC1EP6QState(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QAbstractState * QHistoryState::defaultState();
  fn _ZNK13QHistoryState12defaultStateEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QHistoryState::~QHistoryState();
  fn _ZN13QHistoryStateD0Ev(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QHistoryState)=1
#[derive(Default)]
pub struct QHistoryState {
  qbase: QAbstractState,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _defaultStateChanged_1: QHistoryState_defaultStateChanged_signal,
  pub _historyTypeChanged_1: QHistoryState_historyTypeChanged_signal,
}

impl /*struct*/ QHistoryState {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QHistoryState {
    return QHistoryState{qbase: QAbstractState::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QHistoryState {
  type Target = QAbstractState;

  fn deref(&self) -> &QAbstractState {
    return & self.qbase;
  }
}
impl AsRef<QAbstractState> for QHistoryState {
  fn as_ref(& self) -> & QAbstractState {
    return & self.qbase;
  }
}
  // proto:  const QMetaObject * QHistoryState::metaObject();
impl /*struct*/ QHistoryState {
  pub fn metaObject<RetType, T: QHistoryState_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QHistoryState_metaObject<RetType> {
  fn metaObject(self , rsthis: & QHistoryState) -> RetType;
}

  // proto:  const QMetaObject * QHistoryState::metaObject();
impl<'a> /*trait*/ QHistoryState_metaObject<()> for () {
  fn metaObject(self , rsthis: & QHistoryState) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QHistoryState10metaObjectEv()};
     unsafe {_ZNK13QHistoryState10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QHistoryState::QHistoryState(const QHistoryState & );
impl /*struct*/ QHistoryState {
  pub fn new<T: QHistoryState_new>(value: T) -> QHistoryState {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QHistoryState_new {
  fn new(self) -> QHistoryState;
}

  // proto:  void QHistoryState::QHistoryState(const QHistoryState & );
impl<'a> /*trait*/ QHistoryState_new for (&'a QHistoryState) {
  fn new(self) -> QHistoryState {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QHistoryStateC1ERKS_()};
    let ctysz: c_int = unsafe{QHistoryState_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN13QHistoryStateC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN13QHistoryStateC1ERKS_(arg0)} as u64;
    let rsthis = QHistoryState{qbase: QAbstractState::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QHistoryState::setDefaultState(QAbstractState * state);
impl /*struct*/ QHistoryState {
  pub fn setDefaultState<RetType, T: QHistoryState_setDefaultState<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDefaultState(self);
    // return 1;
  }
}

pub trait QHistoryState_setDefaultState<RetType> {
  fn setDefaultState(self , rsthis: & QHistoryState) -> RetType;
}

  // proto:  void QHistoryState::setDefaultState(QAbstractState * state);
impl<'a> /*trait*/ QHistoryState_setDefaultState<()> for (&'a QAbstractState) {
  fn setDefaultState(self , rsthis: & QHistoryState) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QHistoryState15setDefaultStateEP14QAbstractState()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QHistoryState15setDefaultStateEP14QAbstractState(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QHistoryState::QHistoryState(QState * parent);
impl<'a> /*trait*/ QHistoryState_new for (&'a QState) {
  fn new(self) -> QHistoryState {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QHistoryStateC1EP6QState()};
    let ctysz: c_int = unsafe{QHistoryState_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN13QHistoryStateC1EP6QState(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN13QHistoryStateC1EP6QState(arg0)} as u64;
    let rsthis = QHistoryState{qbase: QAbstractState::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QAbstractState * QHistoryState::defaultState();
impl /*struct*/ QHistoryState {
  pub fn defaultState<RetType, T: QHistoryState_defaultState<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.defaultState(self);
    // return 1;
  }
}

pub trait QHistoryState_defaultState<RetType> {
  fn defaultState(self , rsthis: & QHistoryState) -> RetType;
}

  // proto:  QAbstractState * QHistoryState::defaultState();
impl<'a> /*trait*/ QHistoryState_defaultState<()> for () {
  fn defaultState(self , rsthis: & QHistoryState) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QHistoryState12defaultStateEv()};
     unsafe {_ZNK13QHistoryState12defaultStateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QHistoryState::~QHistoryState();
impl /*struct*/ QHistoryState {
  pub fn free<RetType, T: QHistoryState_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QHistoryState_free<RetType> {
  fn free(self , rsthis: & QHistoryState) -> RetType;
}

  // proto:  void QHistoryState::~QHistoryState();
impl<'a> /*trait*/ QHistoryState_free<()> for () {
  fn free(self , rsthis: & QHistoryState) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QHistoryStateD0Ev()};
     unsafe {_ZN13QHistoryStateD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

#[derive(Default)] // for QHistoryState_defaultStateChanged
pub struct QHistoryState_defaultStateChanged_signal{poi:u64}
impl /* struct */ QHistoryState {
  pub fn defaultStateChanged_1(&self) -> QHistoryState_defaultStateChanged_signal {
     return QHistoryState_defaultStateChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QHistoryState_defaultStateChanged_signal {
  pub fn connect<T: QHistoryState_defaultStateChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QHistoryState_defaultStateChanged_signal_connect {
  fn connect(self, sigthis: QHistoryState_defaultStateChanged_signal);
}

#[derive(Default)] // for QHistoryState_historyTypeChanged
pub struct QHistoryState_historyTypeChanged_signal{poi:u64}
impl /* struct */ QHistoryState {
  pub fn historyTypeChanged_1(&self) -> QHistoryState_historyTypeChanged_signal {
     return QHistoryState_historyTypeChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QHistoryState_historyTypeChanged_signal {
  pub fn connect<T: QHistoryState_historyTypeChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QHistoryState_historyTypeChanged_signal_connect {
  fn connect(self, sigthis: QHistoryState_historyTypeChanged_signal);
}

// <= body block end

