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
  // proto:  bool QKeySequence::isDetached();
  fn _ZNK12QKeySequence10isDetachedEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QKeySequence::isEmpty();
  fn _ZNK12QKeySequence7isEmptyEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QKeySequence::NewQKeySequence(const QKeySequence & ks);
  fn _ZN12QKeySequenceC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QKeySequence::NewQKeySequence(int k1, int k2, int k3, int k4);
  fn _ZN12QKeySequenceC1Eiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) ;
  // proto:  int QKeySequence::count();
  fn _ZNK12QKeySequence5countEv(qthis: *mut c_void) -> c_int;
  // proto: static QKeySequence QKeySequence::mnemonic(const QString & text);
  fn _ZN12QKeySequence8mnemonicERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto:  void QKeySequence::NewQKeySequence();
  fn _ZN12QKeySequenceC1Ev(qthis: *mut c_void) ;
  // proto:  void QKeySequence::FreeQKeySequence();
  fn _ZN12QKeySequenceD0Ev(qthis: *mut c_void) ;
  // proto:  void QKeySequence::swap(QKeySequence & other);
  fn _ZN12QKeySequence4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QKeySequence)=8
pub struct QKeySequence {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QKeySequence {
  pub fn isDetached<RetType, T: QKeySequence_isDetached<RetType>>(&mut self, value: T) -> RetType {
    return value.isDetached(self);
    // return 1;
  }
}

pub trait QKeySequence_isDetached<RetType> {
  fn isDetached(self, rsthis: &mut QKeySequence) -> RetType;
}

// proto:  bool QKeySequence::isDetached();
impl<'a> /*trait*/ QKeySequence_isDetached<i8> for () {
  fn isDetached(self, rsthis: &mut QKeySequence) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QKeySequence10isDetachedEv()};
    let mut ret = unsafe {_ZNK12QKeySequence10isDetachedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QKeySequence {
  pub fn isEmpty<RetType, T: QKeySequence_isEmpty<RetType>>(&mut self, value: T) -> RetType {
    return value.isEmpty(self);
    // return 1;
  }
}

pub trait QKeySequence_isEmpty<RetType> {
  fn isEmpty(self, rsthis: &mut QKeySequence) -> RetType;
}

// proto:  bool QKeySequence::isEmpty();
impl<'a> /*trait*/ QKeySequence_isEmpty<i8> for () {
  fn isEmpty(self, rsthis: &mut QKeySequence) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QKeySequence7isEmptyEv()};
    let mut ret = unsafe {_ZNK12QKeySequence7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QKeySequence {
  pub fn NewQKeySequence<T: QKeySequence_NewQKeySequence>(value: T) -> QKeySequence {
    let rsthis = value.NewQKeySequence();
    return rsthis;
    // return 1;
  }
}

pub trait QKeySequence_NewQKeySequence {
  fn NewQKeySequence(self) -> QKeySequence;
}

// proto: void QKeySequence::NewQKeySequence(const QKeySequence & ks);
impl<'a> /*trait*/ QKeySequence_NewQKeySequence for (&'a  QKeySequence) {
  fn NewQKeySequence(self) -> QKeySequence {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QKeySequenceC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QKeySequenceC1ERKS_(qthis, arg0)};
    let rsthis = QKeySequence{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QKeySequence::NewQKeySequence(int k1, int k2, int k3, int k4);
impl<'a> /*trait*/ QKeySequence_NewQKeySequence for (i32, i32, i32, i32) {
  fn NewQKeySequence(self) -> QKeySequence {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QKeySequenceC1Eiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN12QKeySequenceC1Eiiii(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QKeySequence{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QKeySequence {
  pub fn count<RetType, T: QKeySequence_count<RetType>>(&mut self, value: T) -> RetType {
    return value.count(self);
    // return 1;
  }
}

pub trait QKeySequence_count<RetType> {
  fn count(self, rsthis: &mut QKeySequence) -> RetType;
}

// proto:  int QKeySequence::count();
impl<'a> /*trait*/ QKeySequence_count<i32> for () {
  fn count(self, rsthis: &mut QKeySequence) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QKeySequence5countEv()};
    let mut ret = unsafe {_ZNK12QKeySequence5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QKeySequence {
  pub fn mnemonic<RetType, T: QKeySequence_mnemonic<RetType>>(&mut self, value: T) -> RetType {
    return value.mnemonic(self);
    // return 1;
  }
}

pub trait QKeySequence_mnemonic<RetType> {
  fn mnemonic(self, rsthis: &mut QKeySequence) -> RetType;
}

// proto: static QKeySequence QKeySequence::mnemonic(const QString & text);
impl<'a> /*trait*/ QKeySequence_mnemonic<QKeySequence> for (&'a  QString) {
  fn mnemonic(self, rsthis: &mut QKeySequence) -> QKeySequence {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QKeySequence8mnemonicERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QKeySequence8mnemonicERK7QString(arg0)};
    let mut ret1 = QKeySequence{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QKeySequence::NewQKeySequence();
impl<'a> /*trait*/ QKeySequence_NewQKeySequence for () {
  fn NewQKeySequence(self) -> QKeySequence {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QKeySequenceC1Ev()};
    unsafe {_ZN12QKeySequenceC1Ev(qthis)};
    let rsthis = QKeySequence{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QKeySequence {
  pub fn FreeQKeySequence<RetType, T: QKeySequence_FreeQKeySequence<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQKeySequence(self);
    // return 1;
  }
}

pub trait QKeySequence_FreeQKeySequence<RetType> {
  fn FreeQKeySequence(self, rsthis: &mut QKeySequence) -> RetType;
}

// proto:  void QKeySequence::FreeQKeySequence();
impl<'a> /*trait*/ QKeySequence_FreeQKeySequence<()> for () {
  fn FreeQKeySequence(self, rsthis: &mut QKeySequence) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QKeySequenceD0Ev()};
     unsafe {_ZN12QKeySequenceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QKeySequence {
  pub fn swap<RetType, T: QKeySequence_swap<RetType>>(&mut self, value: T) -> RetType {
    return value.swap(self);
    // return 1;
  }
}

pub trait QKeySequence_swap<RetType> {
  fn swap(self, rsthis: &mut QKeySequence) -> RetType;
}

// proto:  void QKeySequence::swap(QKeySequence & other);
impl<'a> /*trait*/ QKeySequence_swap<()> for (&'a mut QKeySequence) {
  fn swap(self, rsthis: &mut QKeySequence) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QKeySequence4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QKeySequence4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

