// auto generated, do not modify.
// created: Fri Jan  1 15:54:32 2016
// src-file: /QtCore/qabstractstate.h
// dst-file: /src/core/qabstractstate.rs
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
use super::qobject::QObject; // 773
use std::ops::Deref;
use super::qstate::QState; // 773
use super::qstatemachine::QStateMachine; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QAbstractState_Class_Size() -> c_int;
  // proto:  void QAbstractState::~QAbstractState();
  fn _ZN14QAbstractStateD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QAbstractState::QAbstractState(const QAbstractState & );
  fn dector_ZN14QAbstractStateC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN14QAbstractStateC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QAbstractState::QAbstractState(QState * parent);
  fn dector_ZN14QAbstractStateC1EP6QState(arg0: *mut c_void) -> *mut c_void;
  fn _ZN14QAbstractStateC1EP6QState(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QMetaObject * QAbstractState::metaObject();
  fn _ZNK14QAbstractState10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  QState * QAbstractState::parentState();
  fn _ZNK14QAbstractState11parentStateEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QStateMachine * QAbstractState::machine();
  fn _ZNK14QAbstractState7machineEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QAbstractState::active();
  fn _ZNK14QAbstractState6activeEv(qthis: u64 /* *mut c_void*/) -> c_char;
  fn QAbstractState_SlotProxy_connect__ZN14QAbstractState13activeChangedEb(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QAbstractState)=1
#[derive(Default)]
pub struct QAbstractState {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _entered: QAbstractState_entered_signal,
  pub _exited: QAbstractState_exited_signal,
  pub _activeChanged: QAbstractState_activeChanged_signal,
}

impl /*struct*/ QAbstractState {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QAbstractState {
    return QAbstractState{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QAbstractState {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QAbstractState {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QAbstractState::~QAbstractState();
impl /*struct*/ QAbstractState {
  pub fn free<RetType, T: QAbstractState_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QAbstractState_free<RetType> {
  fn free(self , rsthis: & QAbstractState) -> RetType;
}

  // proto:  void QAbstractState::~QAbstractState();
impl<'a> /*trait*/ QAbstractState_free<()> for () {
  fn free(self , rsthis: & QAbstractState) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QAbstractStateD0Ev()};
     unsafe {_ZN14QAbstractStateD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractState::QAbstractState(const QAbstractState & );
impl /*struct*/ QAbstractState {
  pub fn new<T: QAbstractState_new>(value: T) -> QAbstractState {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractState_new {
  fn new(self) -> QAbstractState;
}

  // proto:  void QAbstractState::QAbstractState(const QAbstractState & );
impl<'a> /*trait*/ QAbstractState_new for (&'a QAbstractState) {
  fn new(self) -> QAbstractState {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QAbstractStateC1ERKS_()};
    let ctysz: c_int = unsafe{QAbstractState_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN14QAbstractStateC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN14QAbstractStateC1ERKS_(arg0)} as u64;
    let rsthis = QAbstractState{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QAbstractState::QAbstractState(QState * parent);
impl<'a> /*trait*/ QAbstractState_new for (&'a QState) {
  fn new(self) -> QAbstractState {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QAbstractStateC1EP6QState()};
    let ctysz: c_int = unsafe{QAbstractState_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN14QAbstractStateC1EP6QState(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN14QAbstractStateC1EP6QState(arg0)} as u64;
    let rsthis = QAbstractState{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QAbstractState::metaObject();
impl /*struct*/ QAbstractState {
  pub fn metaObject<RetType, T: QAbstractState_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QAbstractState_metaObject<RetType> {
  fn metaObject(self , rsthis: & QAbstractState) -> RetType;
}

  // proto:  const QMetaObject * QAbstractState::metaObject();
impl<'a> /*trait*/ QAbstractState_metaObject<()> for () {
  fn metaObject(self , rsthis: & QAbstractState) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QAbstractState10metaObjectEv()};
     unsafe {_ZNK14QAbstractState10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QState * QAbstractState::parentState();
impl /*struct*/ QAbstractState {
  pub fn parentState<RetType, T: QAbstractState_parentState<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.parentState(self);
    // return 1;
  }
}

pub trait QAbstractState_parentState<RetType> {
  fn parentState(self , rsthis: & QAbstractState) -> RetType;
}

  // proto:  QState * QAbstractState::parentState();
impl<'a> /*trait*/ QAbstractState_parentState<QState> for () {
  fn parentState(self , rsthis: & QAbstractState) -> QState {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QAbstractState11parentStateEv()};
    let mut ret = unsafe {_ZNK14QAbstractState11parentStateEv(rsthis.qclsinst)};
    let mut ret1 = QState::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QStateMachine * QAbstractState::machine();
impl /*struct*/ QAbstractState {
  pub fn machine<RetType, T: QAbstractState_machine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.machine(self);
    // return 1;
  }
}

pub trait QAbstractState_machine<RetType> {
  fn machine(self , rsthis: & QAbstractState) -> RetType;
}

  // proto:  QStateMachine * QAbstractState::machine();
impl<'a> /*trait*/ QAbstractState_machine<QStateMachine> for () {
  fn machine(self , rsthis: & QAbstractState) -> QStateMachine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QAbstractState7machineEv()};
    let mut ret = unsafe {_ZNK14QAbstractState7machineEv(rsthis.qclsinst)};
    let mut ret1 = QStateMachine::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QAbstractState::active();
impl /*struct*/ QAbstractState {
  pub fn active<RetType, T: QAbstractState_active<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.active(self);
    // return 1;
  }
}

pub trait QAbstractState_active<RetType> {
  fn active(self , rsthis: & QAbstractState) -> RetType;
}

  // proto:  bool QAbstractState::active();
impl<'a> /*trait*/ QAbstractState_active<i8> for () {
  fn active(self , rsthis: & QAbstractState) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QAbstractState6activeEv()};
    let mut ret = unsafe {_ZNK14QAbstractState6activeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

#[derive(Default)] // for QAbstractState_entered
pub struct QAbstractState_entered_signal{poi:u64}
impl /* struct */ QAbstractState {
  pub fn entered(&self) -> QAbstractState_entered_signal {
     return QAbstractState_entered_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractState_entered_signal {
  pub fn connect<T: QAbstractState_entered_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractState_entered_signal_connect {
  fn connect(self, sigthis: QAbstractState_entered_signal);
}

#[derive(Default)] // for QAbstractState_exited
pub struct QAbstractState_exited_signal{poi:u64}
impl /* struct */ QAbstractState {
  pub fn exited(&self) -> QAbstractState_exited_signal {
     return QAbstractState_exited_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractState_exited_signal {
  pub fn connect<T: QAbstractState_exited_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractState_exited_signal_connect {
  fn connect(self, sigthis: QAbstractState_exited_signal);
}

#[derive(Default)] // for QAbstractState_activeChanged
pub struct QAbstractState_activeChanged_signal{poi:u64}
impl /* struct */ QAbstractState {
  pub fn activeChanged(&self) -> QAbstractState_activeChanged_signal {
     return QAbstractState_activeChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractState_activeChanged_signal {
  pub fn connect<T: QAbstractState_activeChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractState_activeChanged_signal_connect {
  fn connect(self, sigthis: QAbstractState_activeChanged_signal);
}

// activeChanged(_Bool)
extern fn QAbstractState_activeChanged_signal_connect_cb_0(rsfptr:fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
extern fn QAbstractState_activeChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(i8)>, arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i8;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QAbstractState_activeChanged_signal_connect for fn(i8) {
  fn connect(self, sigthis: QAbstractState_activeChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractState_activeChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QAbstractState_SlotProxy_connect__ZN14QAbstractState13activeChangedEb(arg0, arg1, arg2)};
  }
}
impl /* trait */ QAbstractState_activeChanged_signal_connect for Box<Fn(i8)> {
  fn connect(self, sigthis: QAbstractState_activeChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractState_activeChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QAbstractState_SlotProxy_connect__ZN14QAbstractState13activeChangedEb(arg0, arg1, arg2)};
  }
}
// <= body block end

