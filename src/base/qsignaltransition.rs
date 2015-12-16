// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;
use super::qbytearray::QByteArray;
use super::qstate::QState;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QSignalTransition::setSenderObject(const QObject * sender);
  fn _ZN17QSignalTransition15setSenderObjectEPK7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QByteArray QSignalTransition::signal();
  fn _ZNK17QSignalTransition6signalEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QSignalTransition::FreeQSignalTransition();
  fn _ZN17QSignalTransitionD0Ev(qthis: *mut c_void) ;
  // proto:  void QSignalTransition::NewQSignalTransition(const QSignalTransition & );
  fn _ZN17QSignalTransitionC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QSignalTransition::NewQSignalTransition(const QObject * sender, const char * signal, QState * sourceState);
  fn _ZN17QSignalTransitionC1EPK7QObjectPKcP6QState(qthis: *mut c_void, arg0: *mut c_void, arg1: *const c_char, arg2: *mut c_void) ;
  // proto:  QObject * QSignalTransition::senderObject();
  fn _ZNK17QSignalTransition12senderObjectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QSignalTransition::NewQSignalTransition(QState * sourceState);
  fn _ZN17QSignalTransitionC1EP6QState(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QSignalTransition::setSignal(const QByteArray & signal);
  fn _ZN17QSignalTransition9setSignalERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QMetaObject * QSignalTransition::metaObject();
  fn _ZNK17QSignalTransition10metaObjectEv(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QSignalTransition)=1
pub struct QSignalTransition {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSignalTransition {
  pub fn setSenderObject<T: QSignalTransition_setSenderObject>(&mut self, value: T)  {
     value.setSenderObject(self);
    // return 1;
  }
}

pub trait QSignalTransition_setSenderObject {
  fn setSenderObject(self, rsthis: &mut QSignalTransition) ;
}

// proto:  void QSignalTransition::setSenderObject(const QObject * sender);
impl<'a> /*trait*/ QSignalTransition_setSenderObject for (&'a  QObject) {
  fn setSenderObject(self, rsthis: &mut QSignalTransition)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSignalTransition15setSenderObjectEPK7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QSignalTransition15setSenderObjectEPK7QObject(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSignalTransition {
  pub fn signal<T: QSignalTransition_signal>(&mut self, value: T) -> QByteArray {
    return value.signal(self);
    // return 1;
  }
}

pub trait QSignalTransition_signal {
  fn signal(self, rsthis: &mut QSignalTransition) -> QByteArray;
}

// proto:  QByteArray QSignalTransition::signal();
impl<'a> /*trait*/ QSignalTransition_signal for () {
  fn signal(self, rsthis: &mut QSignalTransition) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QSignalTransition6signalEv()};
    let mut ret = unsafe {_ZNK17QSignalTransition6signalEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSignalTransition {
  pub fn FreeQSignalTransition<T: QSignalTransition_FreeQSignalTransition>(&mut self, value: T)  {
     value.FreeQSignalTransition(self);
    // return 1;
  }
}

pub trait QSignalTransition_FreeQSignalTransition {
  fn FreeQSignalTransition(self, rsthis: &mut QSignalTransition) ;
}

// proto:  void QSignalTransition::FreeQSignalTransition();
impl<'a> /*trait*/ QSignalTransition_FreeQSignalTransition for () {
  fn FreeQSignalTransition(self, rsthis: &mut QSignalTransition)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSignalTransitionD0Ev()};
     unsafe {_ZN17QSignalTransitionD0Ev(rsthis.qclsinst)};
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
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
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN17QSignalTransitionC1EPK7QObjectPKcP6QState(qthis, arg0, arg1, arg2)};
    let rsthis = QSignalTransition{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSignalTransition {
  pub fn senderObject<T: QSignalTransition_senderObject>(&mut self, value: T) -> QObject {
    return value.senderObject(self);
    // return 1;
  }
}

pub trait QSignalTransition_senderObject {
  fn senderObject(self, rsthis: &mut QSignalTransition) -> QObject;
}

// proto:  QObject * QSignalTransition::senderObject();
impl<'a> /*trait*/ QSignalTransition_senderObject for () {
  fn senderObject(self, rsthis: &mut QSignalTransition) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QSignalTransition12senderObjectEv()};
    let mut ret = unsafe {_ZNK17QSignalTransition12senderObjectEv(rsthis.qclsinst)};
    let mut ret1 = QObject{qclsinst: ret};
    return ret1;
    // return 1;
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
  pub fn setSignal<T: QSignalTransition_setSignal>(&mut self, value: T)  {
     value.setSignal(self);
    // return 1;
  }
}

pub trait QSignalTransition_setSignal {
  fn setSignal(self, rsthis: &mut QSignalTransition) ;
}

// proto:  void QSignalTransition::setSignal(const QByteArray & signal);
impl<'a> /*trait*/ QSignalTransition_setSignal for (&'a  QByteArray) {
  fn setSignal(self, rsthis: &mut QSignalTransition)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSignalTransition9setSignalERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QSignalTransition9setSignalERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSignalTransition {
  pub fn metaObject<T: QSignalTransition_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QSignalTransition_metaObject {
  fn metaObject(self, rsthis: &mut QSignalTransition) ;
}

// proto:  const QMetaObject * QSignalTransition::metaObject();
impl<'a> /*trait*/ QSignalTransition_metaObject for () {
  fn metaObject(self, rsthis: &mut QSignalTransition)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QSignalTransition10metaObjectEv()};
     unsafe {_ZNK17QSignalTransition10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

