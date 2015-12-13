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
use super::qbytearray::QByteArray;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN17QSignalTransition15setSenderObjectEPK7QObject(arg0: *const c_void) -> i32;
  fn _ZNK17QSignalTransition6signalEv() -> i32;
  fn _ZN17QSignalTransitionD0Ev() -> i32;
  fn _ZN17QSignalTransitionC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN17QSignalTransitionC1EPK7QObjectPKcP6QState(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_char, arg2: *mut c_void) -> i32;
  fn _ZNK17QSignalTransition12senderObjectEv() -> i32;
  fn _ZN17QSignalTransitionC1EP6QState(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZN17QSignalTransition9setSignalERK10QByteArray(arg0: *const c_void) -> i32;
  fn _ZNK17QSignalTransition10metaObjectEv() -> i32;
}

// body block begin
// class sizeof(QSignalTransition)=1
pub struct QSignalTransition {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSignalTransition {
  pub fn setSenderObject<T: QSignalTransition_setSenderObject>(&mut self, value: T) -> i32 {
    value.setSenderObject(self);
    return 1;
  }
}

pub trait QSignalTransition_setSenderObject {
  fn setSenderObject(self, this: &mut QSignalTransition) -> i32;
}

// proto: void QSignalTransition::setSenderObject(const QObject * sender);
impl<'a> /*trait*/ QSignalTransition_setSenderObject for (&'a  QObject) {
  fn setSenderObject(self, this: &mut QSignalTransition) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSignalTransition15setSenderObjectEPK7QObject()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN17QSignalTransition15setSenderObjectEPK7QObject(arg0)};
    return 1;
  }
}

impl /*struct*/ QSignalTransition {
  pub fn signal<T: QSignalTransition_signal>(&mut self, value: T) -> i32 {
    value.signal(self);
    return 1;
  }
}

pub trait QSignalTransition_signal {
  fn signal(self, this: &mut QSignalTransition) -> i32;
}

// proto: QByteArray QSignalTransition::signal();
impl<'a> /*trait*/ QSignalTransition_signal for () {
  fn signal(self, this: &mut QSignalTransition) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QSignalTransition6signalEv()};
    unsafe {_ZNK17QSignalTransition6signalEv()};
    return 1;
  }
}

impl /*struct*/ QSignalTransition {
  pub fn FreeQSignalTransition<T: QSignalTransition_FreeQSignalTransition>(&mut self, value: T) -> i32 {
    value.FreeQSignalTransition(self);
    return 1;
  }
}

pub trait QSignalTransition_FreeQSignalTransition {
  fn FreeQSignalTransition(self, this: &mut QSignalTransition) -> i32;
}

// proto: void QSignalTransition::FreeQSignalTransition();
impl<'a> /*trait*/ QSignalTransition_FreeQSignalTransition for () {
  fn FreeQSignalTransition(self, this: &mut QSignalTransition) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSignalTransitionD0Ev()};
    unsafe {_ZN17QSignalTransitionD0Ev()};
    return 1;
  }
}

impl /*struct*/ QSignalTransition {
  pub fn NewQSignalTransition<T: QSignalTransition_NewQSignalTransition>(value: T) -> QSignalTransition {
    let rsthis = value.NewQSignalTransition();
    return rsthis;
    // return 1;
  }
}

pub trait QSignalTransition_NewQSignalTransition {
  fn NewQSignalTransition(self) -> QSignalTransition;
}

// proto: void QSignalTransition::NewQSignalTransition(const QSignalTransition & );
impl<'a> /*trait*/ QSignalTransition_NewQSignalTransition for (&'a  QSignalTransition) {
  fn NewQSignalTransition(self) -> QSignalTransition {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSignalTransitionC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN17QSignalTransitionC1ERKS_(qthis, arg0)};
    let rsthis = QSignalTransition{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QSignalTransition::NewQSignalTransition(const QObject * sender, const char * signal, QState * sourceState);
impl<'a> /*trait*/ QSignalTransition_NewQSignalTransition for (&'a  QObject, &'a  String, &'a mut QState) {
  fn NewQSignalTransition(self) -> QSignalTransition {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSignalTransitionC1EPK7QObjectPKcP6QState()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN17QSignalTransitionC1EPK7QObjectPKcP6QState(qthis, arg0, arg1, arg2)};
    let rsthis = QSignalTransition{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSignalTransition {
  pub fn senderObject<T: QSignalTransition_senderObject>(&mut self, value: T) -> i32 {
    value.senderObject(self);
    return 1;
  }
}

pub trait QSignalTransition_senderObject {
  fn senderObject(self, this: &mut QSignalTransition) -> i32;
}

// proto: QObject * QSignalTransition::senderObject();
impl<'a> /*trait*/ QSignalTransition_senderObject for () {
  fn senderObject(self, this: &mut QSignalTransition) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QSignalTransition12senderObjectEv()};
    unsafe {_ZNK17QSignalTransition12senderObjectEv()};
    return 1;
  }
}

// proto: void QSignalTransition::NewQSignalTransition(QState * sourceState);
impl<'a> /*trait*/ QSignalTransition_NewQSignalTransition for (&'a mut QState) {
  fn NewQSignalTransition(self) -> QSignalTransition {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSignalTransitionC1EP6QState()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QSignalTransitionC1EP6QState(qthis, arg0)};
    let rsthis = QSignalTransition{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSignalTransition {
  pub fn setSignal<T: QSignalTransition_setSignal>(&mut self, value: T) -> i32 {
    value.setSignal(self);
    return 1;
  }
}

pub trait QSignalTransition_setSignal {
  fn setSignal(self, this: &mut QSignalTransition) -> i32;
}

// proto: void QSignalTransition::setSignal(const QByteArray & signal);
impl<'a> /*trait*/ QSignalTransition_setSignal for (&'a  QByteArray) {
  fn setSignal(self, this: &mut QSignalTransition) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSignalTransition9setSignalERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN17QSignalTransition9setSignalERK10QByteArray(arg0)};
    return 1;
  }
}

impl /*struct*/ QSignalTransition {
  pub fn metaObject<T: QSignalTransition_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QSignalTransition_metaObject {
  fn metaObject(self, this: &mut QSignalTransition) -> i32;
}

// proto: const QMetaObject * QSignalTransition::metaObject();
impl<'a> /*trait*/ QSignalTransition_metaObject for () {
  fn metaObject(self, this: &mut QSignalTransition) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QSignalTransition10metaObjectEv()};
    unsafe {_ZNK17QSignalTransition10metaObjectEv()};
    return 1;
  }
}

