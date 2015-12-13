// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN15QSocketNotifierC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK15QSocketNotifier6socketEv() -> i32;
  fn _ZNK15QSocketNotifier9isEnabledEv() -> i32;
  fn _ZN15QSocketNotifier10setEnabledEb(arg0: int8_t) -> i32;
  fn _ZNK15QSocketNotifier10metaObjectEv() -> i32;
  fn _ZN15QSocketNotifierD0Ev() -> i32;
}

// body block begin
// class sizeof(QSocketNotifier)=1
pub struct QSocketNotifier {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSocketNotifier {
  pub fn NewQSocketNotifier<T: QSocketNotifier_NewQSocketNotifier>(value: T) -> QSocketNotifier {
    let rsthis = value.NewQSocketNotifier();
    return rsthis;
    // return 1;
  }
}

pub trait QSocketNotifier_NewQSocketNotifier {
  fn NewQSocketNotifier(self) -> QSocketNotifier;
}

// proto: void QSocketNotifier::NewQSocketNotifier(const QSocketNotifier & );
impl<'a> /*trait*/ QSocketNotifier_NewQSocketNotifier for (&'a  QSocketNotifier) {
  fn NewQSocketNotifier(self) -> QSocketNotifier {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSocketNotifierC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QSocketNotifierC1ERKS_(qthis, arg0)};
    let rsthis = QSocketNotifier{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSocketNotifier {
  pub fn socket<T: QSocketNotifier_socket>(&mut self, value: T) -> i32 {
    value.socket(self);
    return 1;
  }
}

pub trait QSocketNotifier_socket {
  fn socket(self, this: &mut QSocketNotifier) -> i32;
}

// proto: qptrdiff QSocketNotifier::socket();
impl<'a> /*trait*/ QSocketNotifier_socket for () {
  fn socket(self, this: &mut QSocketNotifier) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSocketNotifier6socketEv()};
    unsafe {_ZNK15QSocketNotifier6socketEv()};
    return 1;
  }
}

impl /*struct*/ QSocketNotifier {
  pub fn isEnabled<T: QSocketNotifier_isEnabled>(&mut self, value: T) -> i32 {
    value.isEnabled(self);
    return 1;
  }
}

pub trait QSocketNotifier_isEnabled {
  fn isEnabled(self, this: &mut QSocketNotifier) -> i32;
}

// proto: bool QSocketNotifier::isEnabled();
impl<'a> /*trait*/ QSocketNotifier_isEnabled for () {
  fn isEnabled(self, this: &mut QSocketNotifier) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSocketNotifier9isEnabledEv()};
    unsafe {_ZNK15QSocketNotifier9isEnabledEv()};
    return 1;
  }
}

impl /*struct*/ QSocketNotifier {
  pub fn setEnabled<T: QSocketNotifier_setEnabled>(&mut self, value: T) -> i32 {
    value.setEnabled(self);
    return 1;
  }
}

pub trait QSocketNotifier_setEnabled {
  fn setEnabled(self, this: &mut QSocketNotifier) -> i32;
}

// proto: void QSocketNotifier::setEnabled(bool );
impl<'a> /*trait*/ QSocketNotifier_setEnabled for (i8) {
  fn setEnabled(self, this: &mut QSocketNotifier) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSocketNotifier10setEnabledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN15QSocketNotifier10setEnabledEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QSocketNotifier {
  pub fn metaObject<T: QSocketNotifier_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QSocketNotifier_metaObject {
  fn metaObject(self, this: &mut QSocketNotifier) -> i32;
}

// proto: const QMetaObject * QSocketNotifier::metaObject();
impl<'a> /*trait*/ QSocketNotifier_metaObject for () {
  fn metaObject(self, this: &mut QSocketNotifier) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSocketNotifier10metaObjectEv()};
    unsafe {_ZNK15QSocketNotifier10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QSocketNotifier {
  pub fn FreeQSocketNotifier<T: QSocketNotifier_FreeQSocketNotifier>(&mut self, value: T) -> i32 {
    value.FreeQSocketNotifier(self);
    return 1;
  }
}

pub trait QSocketNotifier_FreeQSocketNotifier {
  fn FreeQSocketNotifier(self, this: &mut QSocketNotifier) -> i32;
}

// proto: void QSocketNotifier::FreeQSocketNotifier();
impl<'a> /*trait*/ QSocketNotifier_FreeQSocketNotifier for () {
  fn FreeQSocketNotifier(self, this: &mut QSocketNotifier) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSocketNotifierD0Ev()};
    unsafe {_ZN15QSocketNotifierD0Ev()};
    return 1;
  }
}

