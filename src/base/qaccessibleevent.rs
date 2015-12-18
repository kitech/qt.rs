// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;
use super::qaccessibleinterface::QAccessibleInterface;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QObject * QAccessibleEvent::object();
  fn _ZNK16QAccessibleEvent6objectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAccessibleEvent::setChild(int chld);
  fn _ZN16QAccessibleEvent8setChildEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QAccessibleInterface * QAccessibleEvent::accessibleInterface();
  fn _ZNK16QAccessibleEvent19accessibleInterfaceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAccessibleEvent::NewQAccessibleEvent(const QAccessibleEvent & );
  fn _ZN16QAccessibleEventC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QAccessibleEvent::child();
  fn _ZNK16QAccessibleEvent5childEv(qthis: *mut c_void) -> c_int;
  // proto:  void QAccessibleEvent::FreeQAccessibleEvent();
  fn _ZN16QAccessibleEventD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QAccessibleEvent)=32
pub struct QAccessibleEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAccessibleEvent {
  pub fn object<RetType, T: QAccessibleEvent_object<RetType>>(&mut self, value: T) -> RetType {
    return value.object(self);
    // return 1;
  }
}

pub trait QAccessibleEvent_object<RetType> {
  fn object(self, rsthis: &mut QAccessibleEvent) -> RetType;
}

// proto:  QObject * QAccessibleEvent::object();
impl<'a> /*trait*/ QAccessibleEvent_object<QObject> for () {
  fn object(self, rsthis: &mut QAccessibleEvent) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QAccessibleEvent6objectEv()};
    let mut ret = unsafe {_ZNK16QAccessibleEvent6objectEv(rsthis.qclsinst)};
    let mut ret1 = QObject{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleEvent {
  pub fn setChild<RetType, T: QAccessibleEvent_setChild<RetType>>(&mut self, value: T) -> RetType {
    return value.setChild(self);
    // return 1;
  }
}

pub trait QAccessibleEvent_setChild<RetType> {
  fn setChild(self, rsthis: &mut QAccessibleEvent) -> RetType;
}

// proto:  void QAccessibleEvent::setChild(int chld);
impl<'a> /*trait*/ QAccessibleEvent_setChild<()> for (i32) {
  fn setChild(self, rsthis: &mut QAccessibleEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAccessibleEvent8setChildEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN16QAccessibleEvent8setChildEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QAccessibleEvent {
  pub fn accessibleInterface<RetType, T: QAccessibleEvent_accessibleInterface<RetType>>(&mut self, value: T) -> RetType {
    return value.accessibleInterface(self);
    // return 1;
  }
}

pub trait QAccessibleEvent_accessibleInterface<RetType> {
  fn accessibleInterface(self, rsthis: &mut QAccessibleEvent) -> RetType;
}

// proto:  QAccessibleInterface * QAccessibleEvent::accessibleInterface();
impl<'a> /*trait*/ QAccessibleEvent_accessibleInterface<QAccessibleInterface> for () {
  fn accessibleInterface(self, rsthis: &mut QAccessibleEvent) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QAccessibleEvent19accessibleInterfaceEv()};
    let mut ret = unsafe {_ZNK16QAccessibleEvent19accessibleInterfaceEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleEvent {
  pub fn NewQAccessibleEvent<T: QAccessibleEvent_NewQAccessibleEvent>(value: T) -> QAccessibleEvent {
    let rsthis = value.NewQAccessibleEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleEvent_NewQAccessibleEvent {
  fn NewQAccessibleEvent(self) -> QAccessibleEvent;
}

// proto: void QAccessibleEvent::NewQAccessibleEvent(const QAccessibleEvent & );
impl<'a> /*trait*/ QAccessibleEvent_NewQAccessibleEvent for (&'a  QAccessibleEvent) {
  fn NewQAccessibleEvent(self) -> QAccessibleEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAccessibleEventC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QAccessibleEventC1ERKS_(qthis, arg0)};
    let rsthis = QAccessibleEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QAccessibleEvent {
  pub fn child<RetType, T: QAccessibleEvent_child<RetType>>(&mut self, value: T) -> RetType {
    return value.child(self);
    // return 1;
  }
}

pub trait QAccessibleEvent_child<RetType> {
  fn child(self, rsthis: &mut QAccessibleEvent) -> RetType;
}

// proto:  int QAccessibleEvent::child();
impl<'a> /*trait*/ QAccessibleEvent_child<i32> for () {
  fn child(self, rsthis: &mut QAccessibleEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QAccessibleEvent5childEv()};
    let mut ret = unsafe {_ZNK16QAccessibleEvent5childEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QAccessibleEvent {
  pub fn FreeQAccessibleEvent<RetType, T: QAccessibleEvent_FreeQAccessibleEvent<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQAccessibleEvent(self);
    // return 1;
  }
}

pub trait QAccessibleEvent_FreeQAccessibleEvent<RetType> {
  fn FreeQAccessibleEvent(self, rsthis: &mut QAccessibleEvent) -> RetType;
}

// proto:  void QAccessibleEvent::FreeQAccessibleEvent();
impl<'a> /*trait*/ QAccessibleEvent_FreeQAccessibleEvent<()> for () {
  fn FreeQAccessibleEvent(self, rsthis: &mut QAccessibleEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAccessibleEventD0Ev()};
     unsafe {_ZN16QAccessibleEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

