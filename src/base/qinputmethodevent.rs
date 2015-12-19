// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  const QString & QInputMethodEvent::preeditString();
  fn _ZNK17QInputMethodEvent13preeditStringEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QInputMethodEvent::NewQInputMethodEvent();
  fn _ZN17QInputMethodEventC1Ev(qthis: *mut c_void) ;
  // proto:  int QInputMethodEvent::replacementStart();
  fn _ZNK17QInputMethodEvent16replacementStartEv(qthis: *mut c_void) -> c_int;
  // proto:  void QInputMethodEvent::NewQInputMethodEvent(const QInputMethodEvent & other);
  fn _ZN17QInputMethodEventC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QString & QInputMethodEvent::commitString();
  fn _ZNK17QInputMethodEvent12commitStringEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QInputMethodEvent::setCommitString(const QString & commitString, int replaceFrom, int replaceLength);
  fn _ZN17QInputMethodEvent15setCommitStringERK7QStringii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int) ;
  // proto:  int QInputMethodEvent::replacementLength();
  fn _ZNK17QInputMethodEvent17replacementLengthEv(qthis: *mut c_void) -> c_int;
}

// body block begin
// class sizeof(QInputMethodEvent)=1
pub struct QInputMethodEvent {
  pub qclsinst: *mut c_void,
}

// proto:  const QString & QInputMethodEvent::preeditString();
impl /*struct*/ QInputMethodEvent {
  pub fn preeditString<RetType, T: QInputMethodEvent_preeditString<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.preeditString(self);
    // return 1;
  }
}

pub trait QInputMethodEvent_preeditString<RetType> {
  fn preeditString(self , rsthis: &mut QInputMethodEvent) -> RetType;
}

// proto:  const QString & QInputMethodEvent::preeditString();
impl<'a> /*trait*/ QInputMethodEvent_preeditString<QString> for () {
  fn preeditString(self , rsthis: &mut QInputMethodEvent) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QInputMethodEvent13preeditStringEv()};
    let mut ret = unsafe {_ZNK17QInputMethodEvent13preeditStringEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QInputMethodEvent {
  pub fn NewQInputMethodEvent<T: QInputMethodEvent_NewQInputMethodEvent>(value: T) -> QInputMethodEvent {
    let rsthis = value.NewQInputMethodEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QInputMethodEvent_NewQInputMethodEvent {
  fn NewQInputMethodEvent(self) -> QInputMethodEvent;
}

// proto: void QInputMethodEvent::NewQInputMethodEvent();
impl<'a> /*trait*/ QInputMethodEvent_NewQInputMethodEvent for () {
  fn NewQInputMethodEvent(self) -> QInputMethodEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QInputMethodEventC1Ev()};
    unsafe {_ZN17QInputMethodEventC1Ev(qthis)};
    let rsthis = QInputMethodEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  int QInputMethodEvent::replacementStart();
impl /*struct*/ QInputMethodEvent {
  pub fn replacementStart<RetType, T: QInputMethodEvent_replacementStart<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.replacementStart(self);
    // return 1;
  }
}

pub trait QInputMethodEvent_replacementStart<RetType> {
  fn replacementStart(self , rsthis: &mut QInputMethodEvent) -> RetType;
}

// proto:  int QInputMethodEvent::replacementStart();
impl<'a> /*trait*/ QInputMethodEvent_replacementStart<i32> for () {
  fn replacementStart(self , rsthis: &mut QInputMethodEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QInputMethodEvent16replacementStartEv()};
    let mut ret = unsafe {_ZNK17QInputMethodEvent16replacementStartEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto: void QInputMethodEvent::NewQInputMethodEvent(const QInputMethodEvent & other);
impl<'a> /*trait*/ QInputMethodEvent_NewQInputMethodEvent for (&'a  QInputMethodEvent) {
  fn NewQInputMethodEvent(self) -> QInputMethodEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QInputMethodEventC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QInputMethodEventC1ERKS_(qthis, arg0)};
    let rsthis = QInputMethodEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  const QString & QInputMethodEvent::commitString();
impl /*struct*/ QInputMethodEvent {
  pub fn commitString<RetType, T: QInputMethodEvent_commitString<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.commitString(self);
    // return 1;
  }
}

pub trait QInputMethodEvent_commitString<RetType> {
  fn commitString(self , rsthis: &mut QInputMethodEvent) -> RetType;
}

// proto:  const QString & QInputMethodEvent::commitString();
impl<'a> /*trait*/ QInputMethodEvent_commitString<QString> for () {
  fn commitString(self , rsthis: &mut QInputMethodEvent) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QInputMethodEvent12commitStringEv()};
    let mut ret = unsafe {_ZNK17QInputMethodEvent12commitStringEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QInputMethodEvent::setCommitString(const QString & commitString, int replaceFrom, int replaceLength);
impl /*struct*/ QInputMethodEvent {
  pub fn setCommitString<RetType, T: QInputMethodEvent_setCommitString<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setCommitString(self);
    // return 1;
  }
}

pub trait QInputMethodEvent_setCommitString<RetType> {
  fn setCommitString(self , rsthis: &mut QInputMethodEvent) -> RetType;
}

// proto:  void QInputMethodEvent::setCommitString(const QString & commitString, int replaceFrom, int replaceLength);
impl<'a> /*trait*/ QInputMethodEvent_setCommitString<()> for (&'a  QString, i32, i32) {
  fn setCommitString(self , rsthis: &mut QInputMethodEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QInputMethodEvent15setCommitStringERK7QStringii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN17QInputMethodEvent15setCommitStringERK7QStringii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

// proto:  int QInputMethodEvent::replacementLength();
impl /*struct*/ QInputMethodEvent {
  pub fn replacementLength<RetType, T: QInputMethodEvent_replacementLength<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.replacementLength(self);
    // return 1;
  }
}

pub trait QInputMethodEvent_replacementLength<RetType> {
  fn replacementLength(self , rsthis: &mut QInputMethodEvent) -> RetType;
}

// proto:  int QInputMethodEvent::replacementLength();
impl<'a> /*trait*/ QInputMethodEvent_replacementLength<i32> for () {
  fn replacementLength(self , rsthis: &mut QInputMethodEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QInputMethodEvent17replacementLengthEv()};
    let mut ret = unsafe {_ZNK17QInputMethodEvent17replacementLengthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

