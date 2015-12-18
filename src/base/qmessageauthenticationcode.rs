// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qbytearray::QByteArray;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QByteArray QMessageAuthenticationCode::result();
  fn _ZNK26QMessageAuthenticationCode6resultEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMessageAuthenticationCode::NewQMessageAuthenticationCode(const QMessageAuthenticationCode & );
  fn _ZN26QMessageAuthenticationCodeC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QMessageAuthenticationCode::FreeQMessageAuthenticationCode();
  fn _ZN26QMessageAuthenticationCodeD0Ev(qthis: *mut c_void) ;
  // proto:  void QMessageAuthenticationCode::reset();
  fn _ZN26QMessageAuthenticationCode5resetEv(qthis: *mut c_void) ;
  // proto:  void QMessageAuthenticationCode::setKey(const QByteArray & key);
  fn _ZN26QMessageAuthenticationCode6setKeyERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QMessageAuthenticationCode)=8
pub struct QMessageAuthenticationCode {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMessageAuthenticationCode {
  pub fn result<RetType, T: QMessageAuthenticationCode_result<RetType>>(&mut self, value: T) -> RetType {
    return value.result(self);
    // return 1;
  }
}

pub trait QMessageAuthenticationCode_result<RetType> {
  fn result(self, rsthis: &mut QMessageAuthenticationCode) -> RetType;
}

// proto:  QByteArray QMessageAuthenticationCode::result();
impl<'a> /*trait*/ QMessageAuthenticationCode_result<QByteArray> for () {
  fn result(self, rsthis: &mut QMessageAuthenticationCode) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK26QMessageAuthenticationCode6resultEv()};
    let mut ret = unsafe {_ZNK26QMessageAuthenticationCode6resultEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMessageAuthenticationCode {
  pub fn NewQMessageAuthenticationCode<T: QMessageAuthenticationCode_NewQMessageAuthenticationCode>(value: T) -> QMessageAuthenticationCode {
    let rsthis = value.NewQMessageAuthenticationCode();
    return rsthis;
    // return 1;
  }
}

pub trait QMessageAuthenticationCode_NewQMessageAuthenticationCode {
  fn NewQMessageAuthenticationCode(self) -> QMessageAuthenticationCode;
}

// proto: void QMessageAuthenticationCode::NewQMessageAuthenticationCode(const QMessageAuthenticationCode & );
impl<'a> /*trait*/ QMessageAuthenticationCode_NewQMessageAuthenticationCode for (&'a  QMessageAuthenticationCode) {
  fn NewQMessageAuthenticationCode(self) -> QMessageAuthenticationCode {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QMessageAuthenticationCodeC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN26QMessageAuthenticationCodeC1ERKS_(qthis, arg0)};
    let rsthis = QMessageAuthenticationCode{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMessageAuthenticationCode {
  pub fn FreeQMessageAuthenticationCode<RetType, T: QMessageAuthenticationCode_FreeQMessageAuthenticationCode<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQMessageAuthenticationCode(self);
    // return 1;
  }
}

pub trait QMessageAuthenticationCode_FreeQMessageAuthenticationCode<RetType> {
  fn FreeQMessageAuthenticationCode(self, rsthis: &mut QMessageAuthenticationCode) -> RetType;
}

// proto:  void QMessageAuthenticationCode::FreeQMessageAuthenticationCode();
impl<'a> /*trait*/ QMessageAuthenticationCode_FreeQMessageAuthenticationCode<()> for () {
  fn FreeQMessageAuthenticationCode(self, rsthis: &mut QMessageAuthenticationCode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QMessageAuthenticationCodeD0Ev()};
     unsafe {_ZN26QMessageAuthenticationCodeD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMessageAuthenticationCode {
  pub fn reset<RetType, T: QMessageAuthenticationCode_reset<RetType>>(&mut self, value: T) -> RetType {
    return value.reset(self);
    // return 1;
  }
}

pub trait QMessageAuthenticationCode_reset<RetType> {
  fn reset(self, rsthis: &mut QMessageAuthenticationCode) -> RetType;
}

// proto:  void QMessageAuthenticationCode::reset();
impl<'a> /*trait*/ QMessageAuthenticationCode_reset<()> for () {
  fn reset(self, rsthis: &mut QMessageAuthenticationCode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QMessageAuthenticationCode5resetEv()};
     unsafe {_ZN26QMessageAuthenticationCode5resetEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMessageAuthenticationCode {
  pub fn setKey<RetType, T: QMessageAuthenticationCode_setKey<RetType>>(&mut self, value: T) -> RetType {
    return value.setKey(self);
    // return 1;
  }
}

pub trait QMessageAuthenticationCode_setKey<RetType> {
  fn setKey(self, rsthis: &mut QMessageAuthenticationCode) -> RetType;
}

// proto:  void QMessageAuthenticationCode::setKey(const QByteArray & key);
impl<'a> /*trait*/ QMessageAuthenticationCode_setKey<()> for (&'a  QByteArray) {
  fn setKey(self, rsthis: &mut QMessageAuthenticationCode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QMessageAuthenticationCode6setKeyERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN26QMessageAuthenticationCode6setKeyERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

