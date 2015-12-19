// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qiodevice::QIODevice;
use super::qbytearray::QByteArray;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  bool QCryptographicHash::addData(QIODevice * device);
  fn _ZN18QCryptographicHash7addDataEP9QIODevice(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QCryptographicHash::FreeQCryptographicHash();
  fn _ZN18QCryptographicHashD0Ev(qthis: *mut c_void) ;
  // proto:  void QCryptographicHash::reset();
  fn _ZN18QCryptographicHash5resetEv(qthis: *mut c_void) ;
  // proto:  void QCryptographicHash::addData(const char * data, int length);
  fn _ZN18QCryptographicHash7addDataEPKci(qthis: *mut c_void, arg0: *const c_char, arg1: c_int) ;
  // proto:  QByteArray QCryptographicHash::result();
  fn _ZNK18QCryptographicHash6resultEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QCryptographicHash::addData(const QByteArray & data);
  fn _ZN18QCryptographicHash7addDataERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QCryptographicHash::NewQCryptographicHash(const QCryptographicHash & );
  fn _ZN18QCryptographicHashC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QCryptographicHash)=8
pub struct QCryptographicHash {
  pub qclsinst: *mut c_void,
}

// proto:  bool QCryptographicHash::addData(QIODevice * device);
impl /*struct*/ QCryptographicHash {
  pub fn addData<RetType, T: QCryptographicHash_addData<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.addData(self);
    // return 1;
  }
}

pub trait QCryptographicHash_addData<RetType> {
  fn addData(self , rsthis: &mut QCryptographicHash) -> RetType;
}

// proto:  bool QCryptographicHash::addData(QIODevice * device);
impl<'a> /*trait*/ QCryptographicHash_addData<i8> for (&'a mut QIODevice) {
  fn addData(self , rsthis: &mut QCryptographicHash) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCryptographicHash7addDataEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN18QCryptographicHash7addDataEP9QIODevice(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QCryptographicHash::FreeQCryptographicHash();
impl /*struct*/ QCryptographicHash {
  pub fn FreeQCryptographicHash<RetType, T: QCryptographicHash_FreeQCryptographicHash<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQCryptographicHash(self);
    // return 1;
  }
}

pub trait QCryptographicHash_FreeQCryptographicHash<RetType> {
  fn FreeQCryptographicHash(self , rsthis: &mut QCryptographicHash) -> RetType;
}

// proto:  void QCryptographicHash::FreeQCryptographicHash();
impl<'a> /*trait*/ QCryptographicHash_FreeQCryptographicHash<()> for () {
  fn FreeQCryptographicHash(self , rsthis: &mut QCryptographicHash) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCryptographicHashD0Ev()};
     unsafe {_ZN18QCryptographicHashD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QCryptographicHash::reset();
impl /*struct*/ QCryptographicHash {
  pub fn reset<RetType, T: QCryptographicHash_reset<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.reset(self);
    // return 1;
  }
}

pub trait QCryptographicHash_reset<RetType> {
  fn reset(self , rsthis: &mut QCryptographicHash) -> RetType;
}

// proto:  void QCryptographicHash::reset();
impl<'a> /*trait*/ QCryptographicHash_reset<()> for () {
  fn reset(self , rsthis: &mut QCryptographicHash) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCryptographicHash5resetEv()};
     unsafe {_ZN18QCryptographicHash5resetEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QCryptographicHash::addData(const char * data, int length);
impl<'a> /*trait*/ QCryptographicHash_addData<()> for (&'a  String, i32) {
  fn addData(self , rsthis: &mut QCryptographicHash) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCryptographicHash7addDataEPKci()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_int;
     unsafe {_ZN18QCryptographicHash7addDataEPKci(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  QByteArray QCryptographicHash::result();
impl /*struct*/ QCryptographicHash {
  pub fn result<RetType, T: QCryptographicHash_result<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.result(self);
    // return 1;
  }
}

pub trait QCryptographicHash_result<RetType> {
  fn result(self , rsthis: &mut QCryptographicHash) -> RetType;
}

// proto:  QByteArray QCryptographicHash::result();
impl<'a> /*trait*/ QCryptographicHash_result<QByteArray> for () {
  fn result(self , rsthis: &mut QCryptographicHash) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QCryptographicHash6resultEv()};
    let mut ret = unsafe {_ZNK18QCryptographicHash6resultEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QCryptographicHash::addData(const QByteArray & data);
impl<'a> /*trait*/ QCryptographicHash_addData<()> for (&'a  QByteArray) {
  fn addData(self , rsthis: &mut QCryptographicHash) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCryptographicHash7addDataERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QCryptographicHash7addDataERK10QByteArray(rsthis.qclsinst, arg0)};
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

