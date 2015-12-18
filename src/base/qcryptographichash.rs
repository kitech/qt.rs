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
  // proto:  void QCryptographicHash::FreeQCryptographicHash();
  fn _ZN18QCryptographicHashD0Ev(qthis: *mut c_void) ;
  // proto:  void QCryptographicHash::reset();
  fn _ZN18QCryptographicHash5resetEv(qthis: *mut c_void) ;
  // proto:  QByteArray QCryptographicHash::result();
  fn _ZNK18QCryptographicHash6resultEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QCryptographicHash::NewQCryptographicHash(const QCryptographicHash & );
  fn _ZN18QCryptographicHashC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QCryptographicHash)=8
pub struct QCryptographicHash {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QCryptographicHash {
  pub fn FreeQCryptographicHash<RetType, T: QCryptographicHash_FreeQCryptographicHash<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQCryptographicHash(self);
    // return 1;
  }
}

pub trait QCryptographicHash_FreeQCryptographicHash<RetType> {
  fn FreeQCryptographicHash(self, rsthis: &mut QCryptographicHash) -> RetType;
}

// proto:  void QCryptographicHash::FreeQCryptographicHash();
impl<'a> /*trait*/ QCryptographicHash_FreeQCryptographicHash<()> for () {
  fn FreeQCryptographicHash(self, rsthis: &mut QCryptographicHash) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCryptographicHashD0Ev()};
     unsafe {_ZN18QCryptographicHashD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QCryptographicHash {
  pub fn reset<RetType, T: QCryptographicHash_reset<RetType>>(&mut self, value: T) -> RetType {
    return value.reset(self);
    // return 1;
  }
}

pub trait QCryptographicHash_reset<RetType> {
  fn reset(self, rsthis: &mut QCryptographicHash) -> RetType;
}

// proto:  void QCryptographicHash::reset();
impl<'a> /*trait*/ QCryptographicHash_reset<()> for () {
  fn reset(self, rsthis: &mut QCryptographicHash) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCryptographicHash5resetEv()};
     unsafe {_ZN18QCryptographicHash5resetEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QCryptographicHash {
  pub fn result<RetType, T: QCryptographicHash_result<RetType>>(&mut self, value: T) -> RetType {
    return value.result(self);
    // return 1;
  }
}

pub trait QCryptographicHash_result<RetType> {
  fn result(self, rsthis: &mut QCryptographicHash) -> RetType;
}

// proto:  QByteArray QCryptographicHash::result();
impl<'a> /*trait*/ QCryptographicHash_result<QByteArray> for () {
  fn result(self, rsthis: &mut QCryptographicHash) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QCryptographicHash6resultEv()};
    let mut ret = unsafe {_ZNK18QCryptographicHash6resultEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QCryptographicHash {
  pub fn NewQCryptographicHash<T: QCryptographicHash_NewQCryptographicHash>(value: T) -> QCryptographicHash {
    let rsthis = value.NewQCryptographicHash();
    return rsthis;
    // return 1;
  }
}

pub trait QCryptographicHash_NewQCryptographicHash {
  fn NewQCryptographicHash(self) -> QCryptographicHash;
}

// proto: void QCryptographicHash::NewQCryptographicHash(const QCryptographicHash & );
impl<'a> /*trait*/ QCryptographicHash_NewQCryptographicHash for (&'a  QCryptographicHash) {
  fn NewQCryptographicHash(self) -> QCryptographicHash {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCryptographicHashC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QCryptographicHashC1ERKS_(qthis, arg0)};
    let rsthis = QCryptographicHash{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

