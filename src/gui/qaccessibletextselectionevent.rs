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
  // proto:  int QAccessibleTextSelectionEvent::selectionEnd();
  fn _ZNK29QAccessibleTextSelectionEvent12selectionEndEv(qthis: *mut c_void) -> c_int;
  // proto:  void QAccessibleTextSelectionEvent::QAccessibleTextSelectionEvent(QAccessibleInterface * iface, int start, int end);
  fn _ZN29QAccessibleTextSelectionEventC1EP20QAccessibleInterfaceii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int);
  // proto:  int QAccessibleTextSelectionEvent::selectionStart();
  fn _ZNK29QAccessibleTextSelectionEvent14selectionStartEv(qthis: *mut c_void) -> c_int;
  // proto:  void QAccessibleTextSelectionEvent::QAccessibleTextSelectionEvent(QObject * obj, int start, int end);
  fn _ZN29QAccessibleTextSelectionEventC1EP7QObjectii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int);
  // proto:  void QAccessibleTextSelectionEvent::setSelection(int start, int end);
  fn _ZN29QAccessibleTextSelectionEvent12setSelectionEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
}

// body block begin
// class sizeof(QAccessibleTextSelectionEvent)=40
pub struct QAccessibleTextSelectionEvent {
  pub qclsinst: *mut c_void,
}

  // proto:  int QAccessibleTextSelectionEvent::selectionEnd();
impl /*struct*/ QAccessibleTextSelectionEvent {
  pub fn selectionEnd<RetType, T: QAccessibleTextSelectionEvent_selectionEnd<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.selectionEnd(self);
    // return 1;
  }
}

pub trait QAccessibleTextSelectionEvent_selectionEnd<RetType> {
  fn selectionEnd(self , rsthis: &mut QAccessibleTextSelectionEvent) -> RetType;
}

  // proto:  int QAccessibleTextSelectionEvent::selectionEnd();
impl<'a> /*trait*/ QAccessibleTextSelectionEvent_selectionEnd<i32> for () {
  fn selectionEnd(self , rsthis: &mut QAccessibleTextSelectionEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK29QAccessibleTextSelectionEvent12selectionEndEv()};
    let mut ret = unsafe {_ZNK29QAccessibleTextSelectionEvent12selectionEndEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QAccessibleTextSelectionEvent::QAccessibleTextSelectionEvent(QAccessibleInterface * iface, int start, int end);
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

  // proto:  void QAccessibleTextSelectionEvent::QAccessibleTextSelectionEvent(QAccessibleInterface * iface, int start, int end);
impl<'a> /*trait*/ QAccessibleTextSelectionEvent_NewQAccessibleTextSelectionEvent for (QAccessibleInterface, i32, i32) {
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

  // proto:  int QAccessibleTextSelectionEvent::selectionStart();
impl /*struct*/ QAccessibleTextSelectionEvent {
  pub fn selectionStart<RetType, T: QAccessibleTextSelectionEvent_selectionStart<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.selectionStart(self);
    // return 1;
  }
}

pub trait QAccessibleTextSelectionEvent_selectionStart<RetType> {
  fn selectionStart(self , rsthis: &mut QAccessibleTextSelectionEvent) -> RetType;
}

  // proto:  int QAccessibleTextSelectionEvent::selectionStart();
impl<'a> /*trait*/ QAccessibleTextSelectionEvent_selectionStart<i32> for () {
  fn selectionStart(self , rsthis: &mut QAccessibleTextSelectionEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK29QAccessibleTextSelectionEvent14selectionStartEv()};
    let mut ret = unsafe {_ZNK29QAccessibleTextSelectionEvent14selectionStartEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QAccessibleTextSelectionEvent::QAccessibleTextSelectionEvent(QObject * obj, int start, int end);
impl<'a> /*trait*/ QAccessibleTextSelectionEvent_NewQAccessibleTextSelectionEvent for (QObject, i32, i32) {
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

  // proto:  void QAccessibleTextSelectionEvent::setSelection(int start, int end);
impl /*struct*/ QAccessibleTextSelectionEvent {
  pub fn setSelection<RetType, T: QAccessibleTextSelectionEvent_setSelection<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setSelection(self);
    // return 1;
  }
}

pub trait QAccessibleTextSelectionEvent_setSelection<RetType> {
  fn setSelection(self , rsthis: &mut QAccessibleTextSelectionEvent) -> RetType;
}

  // proto:  void QAccessibleTextSelectionEvent::setSelection(int start, int end);
impl<'a> /*trait*/ QAccessibleTextSelectionEvent_setSelection<()> for (i32, i32) {
  fn setSelection(self , rsthis: &mut QAccessibleTextSelectionEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN29QAccessibleTextSelectionEvent12setSelectionEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN29QAccessibleTextSelectionEvent12setSelectionEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

