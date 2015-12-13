// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qaccessibleinterface::QAccessibleInterface;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: int QAccessibleTextSelectionEvent::selectionEnd();
  fn _ZNK29QAccessibleTextSelectionEvent12selectionEndEv() -> i32;
  // proto: void QAccessibleTextSelectionEvent::NewQAccessibleTextSelectionEvent(QAccessibleInterface * iface, int start, int end);
  fn _ZN29QAccessibleTextSelectionEventC1EP20QAccessibleInterfaceii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int) -> i32;
  // proto: int QAccessibleTextSelectionEvent::selectionStart();
  fn _ZNK29QAccessibleTextSelectionEvent14selectionStartEv() -> i32;
  // proto: void QAccessibleTextSelectionEvent::NewQAccessibleTextSelectionEvent(QObject * obj, int start, int end);
  fn _ZN29QAccessibleTextSelectionEventC1EP7QObjectii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int) -> i32;
  // proto: void QAccessibleTextSelectionEvent::setSelection(int start, int end);
  fn _ZN29QAccessibleTextSelectionEvent12setSelectionEii(arg0: c_int, arg1: c_int) -> i32;
}

// body block begin
// class sizeof(QAccessibleTextSelectionEvent)=40
pub struct QAccessibleTextSelectionEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAccessibleTextSelectionEvent {
  pub fn selectionEnd<T: QAccessibleTextSelectionEvent_selectionEnd>(&mut self, value: T) -> i32 {
    value.selectionEnd(self);
    return 1;
  }
}

pub trait QAccessibleTextSelectionEvent_selectionEnd {
  fn selectionEnd(self, this: &mut QAccessibleTextSelectionEvent) -> i32;
}

// proto: int QAccessibleTextSelectionEvent::selectionEnd();
impl<'a> /*trait*/ QAccessibleTextSelectionEvent_selectionEnd for () {
  fn selectionEnd(self, this: &mut QAccessibleTextSelectionEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK29QAccessibleTextSelectionEvent12selectionEndEv()};
    unsafe {_ZNK29QAccessibleTextSelectionEvent12selectionEndEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleTextSelectionEvent {
  pub fn NewQAccessibleTextSelectionEvent<T: QAccessibleTextSelectionEvent_NewQAccessibleTextSelectionEvent>(value: T) -> QAccessibleTextSelectionEvent {
    let rsthis = value.NewQAccessibleTextSelectionEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleTextSelectionEvent_NewQAccessibleTextSelectionEvent {
  fn NewQAccessibleTextSelectionEvent(self) -> QAccessibleTextSelectionEvent;
}

// proto: void QAccessibleTextSelectionEvent::NewQAccessibleTextSelectionEvent(QAccessibleInterface * iface, int start, int end);
impl<'a> /*trait*/ QAccessibleTextSelectionEvent_NewQAccessibleTextSelectionEvent for (&'a mut QAccessibleInterface, i32, i32) {
  fn NewQAccessibleTextSelectionEvent(self) -> QAccessibleTextSelectionEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN29QAccessibleTextSelectionEventC1EP20QAccessibleInterfaceii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZN29QAccessibleTextSelectionEventC1EP20QAccessibleInterfaceii(qthis, arg0, arg1, arg2)};
    let rsthis = QAccessibleTextSelectionEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTextSelectionEvent {
  pub fn selectionStart<T: QAccessibleTextSelectionEvent_selectionStart>(&mut self, value: T) -> i32 {
    value.selectionStart(self);
    return 1;
  }
}

pub trait QAccessibleTextSelectionEvent_selectionStart {
  fn selectionStart(self, this: &mut QAccessibleTextSelectionEvent) -> i32;
}

// proto: int QAccessibleTextSelectionEvent::selectionStart();
impl<'a> /*trait*/ QAccessibleTextSelectionEvent_selectionStart for () {
  fn selectionStart(self, this: &mut QAccessibleTextSelectionEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK29QAccessibleTextSelectionEvent14selectionStartEv()};
    unsafe {_ZNK29QAccessibleTextSelectionEvent14selectionStartEv()};
    return 1;
  }
}

// proto: void QAccessibleTextSelectionEvent::NewQAccessibleTextSelectionEvent(QObject * obj, int start, int end);
impl<'a> /*trait*/ QAccessibleTextSelectionEvent_NewQAccessibleTextSelectionEvent for (&'a mut QObject, i32, i32) {
  fn NewQAccessibleTextSelectionEvent(self) -> QAccessibleTextSelectionEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN29QAccessibleTextSelectionEventC1EP7QObjectii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZN29QAccessibleTextSelectionEventC1EP7QObjectii(qthis, arg0, arg1, arg2)};
    let rsthis = QAccessibleTextSelectionEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTextSelectionEvent {
  pub fn setSelection<T: QAccessibleTextSelectionEvent_setSelection>(&mut self, value: T) -> i32 {
    value.setSelection(self);
    return 1;
  }
}

pub trait QAccessibleTextSelectionEvent_setSelection {
  fn setSelection(self, this: &mut QAccessibleTextSelectionEvent) -> i32;
}

// proto: void QAccessibleTextSelectionEvent::setSelection(int start, int end);
impl<'a> /*trait*/ QAccessibleTextSelectionEvent_setSelection for (i32, i32) {
  fn setSelection(self, this: &mut QAccessibleTextSelectionEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN29QAccessibleTextSelectionEvent12setSelectionEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN29QAccessibleTextSelectionEvent12setSelectionEii(arg0, arg1)};
    return 1;
  }
}

