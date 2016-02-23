// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
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
use super::super::core::qstring::*; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QKeySequence_Class_Size() -> c_int;
  // proto:  bool QKeySequence::isDetached();
  fn C_ZNK12QKeySequence10isDetachedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QKeySequence::isEmpty();
  fn C_ZNK12QKeySequence7isEmptyEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QKeySequence::QKeySequence(const QKeySequence & ks);
  fn C_ZN12QKeySequenceC2ERKS_(arg0: *mut c_void) -> u64;
  // proto:  void QKeySequence::QKeySequence(int k1, int k2, int k3, int k4);
  fn C_ZN12QKeySequenceC2Eiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> u64;
  // proto:  int QKeySequence::count();
  fn C_ZNK12QKeySequence5countEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto: static QKeySequence QKeySequence::mnemonic(const QString & text);
  fn C_ZN12QKeySequence8mnemonicERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto:  void QKeySequence::QKeySequence();
  fn C_ZN12QKeySequenceC2Ev() -> u64;
  // proto:  void QKeySequence::~QKeySequence();
  fn C_ZN12QKeySequenceD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QKeySequence::swap(QKeySequence & other);
  fn C_ZN12QKeySequence4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QKeySequence)=8
#[derive(Default)]
pub struct QKeySequence {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QKeySequence {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QKeySequence {
    return QKeySequence{qclsinst: qthis, ..Default::default()};
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
    let mut ret = unsafe {C_ZNK12QKeySequence10isDetachedEv(rsthis.qclsinst)};
    return ret as i8; // 1
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
    let mut ret = unsafe {C_ZNK12QKeySequence7isEmptyEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QKeySequence::QKeySequence(const QKeySequence & ks);
impl /*struct*/ QKeySequence {
  pub fn new<T: QKeySequence_new>(value: T) -> QKeySequence {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QKeySequence_new {
  fn new(self) -> QKeySequence;
}

  // proto:  void QKeySequence::QKeySequence(const QKeySequence & ks);
impl<'a> /*trait*/ QKeySequence_new for (&'a QKeySequence) {
  fn new(self) -> QKeySequence {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QKeySequenceC2ERKS_()};
    let ctysz: c_int = unsafe{QKeySequence_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN12QKeySequenceC2ERKS_(arg0)};
    let rsthis = QKeySequence{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QKeySequence::QKeySequence(int k1, int k2, int k3, int k4);
impl<'a> /*trait*/ QKeySequence_new for (i32, Option<i32>, Option<i32>, Option<i32>) {
  fn new(self) -> QKeySequence {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QKeySequenceC2Eiiii()};
    let ctysz: c_int = unsafe{QKeySequence_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_int;
    let arg1 = (if self.1.is_none() {0} else {self.1.unwrap()})  as c_int;
    let arg2 = (if self.2.is_none() {0} else {self.2.unwrap()})  as c_int;
    let arg3 = (if self.3.is_none() {0} else {self.3.unwrap()})  as c_int;
    let qthis: u64 = unsafe {C_ZN12QKeySequenceC2Eiiii(arg0, arg1, arg2, arg3)};
    let rsthis = QKeySequence{qclsinst: qthis, ..Default::default()};
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
    let mut ret = unsafe {C_ZNK12QKeySequence5countEv(rsthis.qclsinst)};
    return ret as i32; // 1
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
    let mut ret = unsafe {C_ZN12QKeySequence8mnemonicERK7QString(arg0)};
    let mut ret1 = QKeySequence::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QKeySequence::QKeySequence();
impl<'a> /*trait*/ QKeySequence_new for () {
  fn new(self) -> QKeySequence {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QKeySequenceC2Ev()};
    let ctysz: c_int = unsafe{QKeySequence_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN12QKeySequenceC2Ev()};
    let rsthis = QKeySequence{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QKeySequence::~QKeySequence();
impl /*struct*/ QKeySequence {
  pub fn free<RetType, T: QKeySequence_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QKeySequence_free<RetType> {
  fn free(self , rsthis: & QKeySequence) -> RetType;
}

  // proto:  void QKeySequence::~QKeySequence();
impl<'a> /*trait*/ QKeySequence_free<()> for () {
  fn free(self , rsthis: & QKeySequence) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QKeySequenceD2Ev()};
     unsafe {C_ZN12QKeySequenceD2Ev(rsthis.qclsinst)};
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
     unsafe {C_ZN12QKeySequence4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

