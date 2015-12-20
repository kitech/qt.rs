// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qaction::QAction;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QActionEvent::QActionEvent(int type, QAction * action, QAction * before);
  fn _ZN12QActionEventC1EiP7QActionS1_(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  QAction * QActionEvent::before();
  fn _ZNK12QActionEvent6beforeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QAction * QActionEvent::action();
  fn _ZNK12QActionEvent6actionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QActionEvent::~QActionEvent();
  fn _ZN12QActionEventD0Ev(qthis: *mut c_void);
}

// body block begin
// class sizeof(QActionEvent)=40
pub struct QActionEvent {
  pub qclsinst: *mut c_void,
}

  // proto:  void QActionEvent::QActionEvent(int type, QAction * action, QAction * before);
impl /*struct*/ QActionEvent {
  pub fn NewQActionEvent<T: QActionEvent_NewQActionEvent>(value: T) -> QActionEvent {
    let rsthis = value.NewQActionEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QActionEvent_NewQActionEvent {
  fn NewQActionEvent(self) -> QActionEvent;
}

  // proto:  void QActionEvent::QActionEvent(int type, QAction * action, QAction * before);
impl<'a> /*trait*/ QActionEvent_NewQActionEvent for (i32, QAction, QAction) {
  fn NewQActionEvent(self) -> QActionEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN12QActionEventC1EiP7QActionS1_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN12QActionEventC1EiP7QActionS1_(qthis, arg0, arg1, arg2)};
    let rsthis = QActionEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QAction * QActionEvent::before();
impl /*struct*/ QActionEvent {
  pub fn before<RetType, T: QActionEvent_before<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.before(self);
    // return 1;
  }
}

pub trait QActionEvent_before<RetType> {
  fn before(self , rsthis: &mut QActionEvent) -> RetType;
}

  // proto:  QAction * QActionEvent::before();
impl<'a> /*trait*/ QActionEvent_before<QAction> for () {
  fn before(self , rsthis: &mut QActionEvent) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK12QActionEvent6beforeEv()};
    let mut ret = unsafe {_ZNK12QActionEvent6beforeEv(rsthis.qclsinst)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QAction * QActionEvent::action();
impl /*struct*/ QActionEvent {
  pub fn action<RetType, T: QActionEvent_action<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.action(self);
    // return 1;
  }
}

pub trait QActionEvent_action<RetType> {
  fn action(self , rsthis: &mut QActionEvent) -> RetType;
}

  // proto:  QAction * QActionEvent::action();
impl<'a> /*trait*/ QActionEvent_action<QAction> for () {
  fn action(self , rsthis: &mut QActionEvent) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK12QActionEvent6actionEv()};
    let mut ret = unsafe {_ZNK12QActionEvent6actionEv(rsthis.qclsinst)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QActionEvent::~QActionEvent();
impl /*struct*/ QActionEvent {
  pub fn FreeQActionEvent<RetType, T: QActionEvent_FreeQActionEvent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQActionEvent(self);
    // return 1;
  }
}

pub trait QActionEvent_FreeQActionEvent<RetType> {
  fn FreeQActionEvent(self , rsthis: &mut QActionEvent) -> RetType;
}

  // proto:  void QActionEvent::~QActionEvent();
impl<'a> /*trait*/ QActionEvent_FreeQActionEvent<()> for () {
  fn FreeQActionEvent(self , rsthis: &mut QActionEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN12QActionEventD0Ev()};
     unsafe {_ZN12QActionEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

