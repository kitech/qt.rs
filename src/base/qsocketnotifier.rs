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
  // proto:  void QSocketNotifier::NewQSocketNotifier(const QSocketNotifier & );
  fn _ZN15QSocketNotifierC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  qptrdiff QSocketNotifier::socket();
  fn _ZNK15QSocketNotifier6socketEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QSocketNotifier::isEnabled();
  fn _ZNK15QSocketNotifier9isEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QSocketNotifier::setEnabled(bool );
  fn _ZN15QSocketNotifier10setEnabledEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  const QMetaObject * QSocketNotifier::metaObject();
  fn _ZNK15QSocketNotifier10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QSocketNotifier::FreeQSocketNotifier();
  fn _ZN15QSocketNotifierD0Ev(qthis: *mut c_void) ;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QSocketNotifierC1ERKS_(qthis, arg0)};
    let rsthis = QSocketNotifier{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSocketNotifier {
  pub fn socket<RetType, T: QSocketNotifier_socket<RetType>>(&mut self, value: T) -> RetType {
    return value.socket(self);
    // return 1;
  }
}

pub trait QSocketNotifier_socket<RetType> {
  fn socket(self, rsthis: &mut QSocketNotifier) -> RetType;
}

// proto:  qptrdiff QSocketNotifier::socket();
impl<'a> /*trait*/ QSocketNotifier_socket<i32> for () {
  fn socket(self, rsthis: &mut QSocketNotifier) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSocketNotifier6socketEv()};
    let mut ret = unsafe {_ZNK15QSocketNotifier6socketEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QSocketNotifier {
  pub fn isEnabled<RetType, T: QSocketNotifier_isEnabled<RetType>>(&mut self, value: T) -> RetType {
    return value.isEnabled(self);
    // return 1;
  }
}

pub trait QSocketNotifier_isEnabled<RetType> {
  fn isEnabled(self, rsthis: &mut QSocketNotifier) -> RetType;
}

// proto:  bool QSocketNotifier::isEnabled();
impl<'a> /*trait*/ QSocketNotifier_isEnabled<i8> for () {
  fn isEnabled(self, rsthis: &mut QSocketNotifier) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSocketNotifier9isEnabledEv()};
    let mut ret = unsafe {_ZNK15QSocketNotifier9isEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSocketNotifier {
  pub fn setEnabled<RetType, T: QSocketNotifier_setEnabled<RetType>>(&mut self, value: T) -> RetType {
    return value.setEnabled(self);
    // return 1;
  }
}

pub trait QSocketNotifier_setEnabled<RetType> {
  fn setEnabled(self, rsthis: &mut QSocketNotifier) -> RetType;
}

// proto:  void QSocketNotifier::setEnabled(bool );
impl<'a> /*trait*/ QSocketNotifier_setEnabled<()> for (i8) {
  fn setEnabled(self, rsthis: &mut QSocketNotifier) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSocketNotifier10setEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN15QSocketNotifier10setEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSocketNotifier {
  pub fn metaObject<RetType, T: QSocketNotifier_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QSocketNotifier_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QSocketNotifier) -> RetType;
}

// proto:  const QMetaObject * QSocketNotifier::metaObject();
impl<'a> /*trait*/ QSocketNotifier_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QSocketNotifier) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSocketNotifier10metaObjectEv()};
     unsafe {_ZNK15QSocketNotifier10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSocketNotifier {
  pub fn FreeQSocketNotifier<RetType, T: QSocketNotifier_FreeQSocketNotifier<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQSocketNotifier(self);
    // return 1;
  }
}

pub trait QSocketNotifier_FreeQSocketNotifier<RetType> {
  fn FreeQSocketNotifier(self, rsthis: &mut QSocketNotifier) -> RetType;
}

// proto:  void QSocketNotifier::FreeQSocketNotifier();
impl<'a> /*trait*/ QSocketNotifier_FreeQSocketNotifier<()> for () {
  fn FreeQSocketNotifier(self, rsthis: &mut QSocketNotifier) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSocketNotifierD0Ev()};
     unsafe {_ZN15QSocketNotifierD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

