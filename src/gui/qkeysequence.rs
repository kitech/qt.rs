// auto generated, do not modify.
// created: Wed Dec 23 22:29:56 2015
// src-file: /QtGui/qkeysequence.h
// dst-file: /src/gui/qkeysequence.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use std::ops::Deref;
use super::super::core::qstring::QString; // 771
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  bool QKeySequence::isDetached();
  fn _ZNK12QKeySequence10isDetachedEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QKeySequence::isEmpty();
  fn _ZNK12QKeySequence7isEmptyEv(qthis: *mut c_void) -> c_char;
  // proto:  void QKeySequence::QKeySequence(const QKeySequence & ks);
  fn _ZN12QKeySequenceC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QKeySequence::QKeySequence(int k1, int k2, int k3, int k4);
  fn _ZN12QKeySequenceC1Eiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int);
  // proto:  int QKeySequence::count();
  fn _ZNK12QKeySequence5countEv(qthis: *mut c_void) -> c_int;
  // proto: static QKeySequence QKeySequence::mnemonic(const QString & text);
  fn _ZN12QKeySequence8mnemonicERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto:  void QKeySequence::QKeySequence();
  fn _ZN12QKeySequenceC1Ev(qthis: *mut c_void);
  // proto:  void QKeySequence::~QKeySequence();
  fn _ZN12QKeySequenceD0Ev(qthis: *mut c_void);
  // proto:  void QKeySequence::swap(QKeySequence & other);
  fn _ZN12QKeySequence4swapERS_(qthis: *mut c_void, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QKeySequence)=8
pub struct QKeySequence {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QKeySequence {
  pub fn inheritFrom(qthis: *mut c_void) -> QKeySequence {
    return QKeySequence{qclsinst: qthis};
  }
}
  // proto:  bool QKeySequence::isDetached();
impl /*struct*/ QKeySequence {
  pub fn isDetached<RetType, T: QKeySequence_isDetached<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isDetached(self);
    // return 1;
  }
}

pub trait QKeySequence_isDetached<RetType> {
  fn isDetached(self , rsthis: & QKeySequence) -> RetType;
}

  // proto:  bool QKeySequence::isDetached();
impl<'a> /*trait*/ QKeySequence_isDetached<i8> for () {
  fn isDetached(self , rsthis: & QKeySequence) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QKeySequence10isDetachedEv()};
    let mut ret = unsafe {_ZNK12QKeySequence10isDetachedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QKeySequence::isEmpty();
impl /*struct*/ QKeySequence {
  pub fn isEmpty<RetType, T: QKeySequence_isEmpty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QKeySequence_isEmpty<RetType> {
  fn isEmpty(self , rsthis: & QKeySequence) -> RetType;
}

  // proto:  bool QKeySequence::isEmpty();
impl<'a> /*trait*/ QKeySequence_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: & QKeySequence) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QKeySequence7isEmptyEv()};
    let mut ret = unsafe {_ZNK12QKeySequence7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QKeySequence::QKeySequence(const QKeySequence & ks);
impl /*struct*/ QKeySequence {
  pub fn New<T: QKeySequence_New>(value: T) -> QKeySequence {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QKeySequence_New {
  fn New(self) -> QKeySequence;
}

  // proto:  void QKeySequence::QKeySequence(const QKeySequence & ks);
impl<'a> /*trait*/ QKeySequence_New for (&'a QKeySequence) {
  fn New(self) -> QKeySequence {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QKeySequenceC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QKeySequenceC1ERKS_(qthis, arg0)};
    let rsthis = QKeySequence{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QKeySequence::QKeySequence(int k1, int k2, int k3, int k4);
impl<'a> /*trait*/ QKeySequence_New for (i32, i32, i32, i32) {
  fn New(self) -> QKeySequence {
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

  // proto:  int QKeySequence::count();
impl /*struct*/ QKeySequence {
  pub fn count<RetType, T: QKeySequence_count<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QKeySequence_count<RetType> {
  fn count(self , rsthis: & QKeySequence) -> RetType;
}

  // proto:  int QKeySequence::count();
impl<'a> /*trait*/ QKeySequence_count<i32> for () {
  fn count(self , rsthis: & QKeySequence) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QKeySequence5countEv()};
    let mut ret = unsafe {_ZNK12QKeySequence5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto: static QKeySequence QKeySequence::mnemonic(const QString & text);
impl /*struct*/ QKeySequence {
  pub fn mnemonic_s<RetType, T: QKeySequence_mnemonic_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.mnemonic_s();
    // return 1;
  }
}

pub trait QKeySequence_mnemonic_s<RetType> {
  fn mnemonic_s(self ) -> RetType;
}

  // proto: static QKeySequence QKeySequence::mnemonic(const QString & text);
impl<'a> /*trait*/ QKeySequence_mnemonic_s<QKeySequence> for (&'a QString) {
  fn mnemonic_s(self ) -> QKeySequence {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QKeySequence8mnemonicERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QKeySequence8mnemonicERK7QString(arg0)};
    let mut ret1 = QKeySequence::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QKeySequence::QKeySequence();
impl<'a> /*trait*/ QKeySequence_New for () {
  fn New(self) -> QKeySequence {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QKeySequenceC1Ev()};
    unsafe {_ZN12QKeySequenceC1Ev(qthis)};
    let rsthis = QKeySequence{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QKeySequence::~QKeySequence();
impl /*struct*/ QKeySequence {
  pub fn Free<RetType, T: QKeySequence_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QKeySequence_Free<RetType> {
  fn Free(self , rsthis: & QKeySequence) -> RetType;
}

  // proto:  void QKeySequence::~QKeySequence();
impl<'a> /*trait*/ QKeySequence_Free<()> for () {
  fn Free(self , rsthis: & QKeySequence) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QKeySequenceD0Ev()};
     unsafe {_ZN12QKeySequenceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QKeySequence::swap(QKeySequence & other);
impl /*struct*/ QKeySequence {
  pub fn swap<RetType, T: QKeySequence_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QKeySequence_swap<RetType> {
  fn swap(self , rsthis: & QKeySequence) -> RetType;
}

  // proto:  void QKeySequence::swap(QKeySequence & other);
impl<'a> /*trait*/ QKeySequence_swap<()> for (&'a QKeySequence) {
  fn swap(self , rsthis: & QKeySequence) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QKeySequence4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QKeySequence4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

