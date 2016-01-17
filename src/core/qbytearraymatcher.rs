// auto generated, do not modify.
// created: Sun Jan 17 17:37:11 2016
// src-file: /QtCore/qbytearraymatcher.h
// dst-file: /src/core/qbytearraymatcher.rs
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
use super::qbytearray::QByteArray; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QByteArrayMatcher_Class_Size() -> c_int;
  // proto:  QByteArray QByteArrayMatcher::pattern();
  fn _ZNK17QByteArrayMatcher7patternEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QByteArrayMatcher::indexIn(const char * str, int len, int from);
  fn _ZNK17QByteArrayMatcher7indexInEPKcii(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_int, arg2: c_int) -> c_int;
  // proto:  void QByteArrayMatcher::setPattern(const QByteArray & pattern);
  fn _ZN17QByteArrayMatcher10setPatternERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QByteArrayMatcher::QByteArrayMatcher();
  fn _ZN17QByteArrayMatcherC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QByteArrayMatcher::QByteArrayMatcher(const char * pattern, int length);
  fn _ZN17QByteArrayMatcherC2EPKci(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_int);
  // proto:  void QByteArrayMatcher::QByteArrayMatcher(const QByteArray & pattern);
  fn _ZN17QByteArrayMatcherC2ERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QByteArrayMatcher::indexIn(const QByteArray & ba, int from);
  fn _ZNK17QByteArrayMatcher7indexInERK10QByteArrayi(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  void QByteArrayMatcher::QByteArrayMatcher(const QByteArrayMatcher & other);
  fn _ZN17QByteArrayMatcherC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QByteArrayMatcher::~QByteArrayMatcher();
  fn _ZN17QByteArrayMatcherD2Ev(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QByteArrayMatcher)=1040
#[derive(Default)]
pub struct QByteArrayMatcher {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QByteArrayMatcher {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QByteArrayMatcher {
    return QByteArrayMatcher{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QByteArray QByteArrayMatcher::pattern();
impl /*struct*/ QByteArrayMatcher {
  pub fn pattern<RetType, T: QByteArrayMatcher_pattern<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pattern(self);
    // return 1;
  }
}

pub trait QByteArrayMatcher_pattern<RetType> {
  fn pattern(self , rsthis: & QByteArrayMatcher) -> RetType;
}

  // proto:  QByteArray QByteArrayMatcher::pattern();
impl<'a> /*trait*/ QByteArrayMatcher_pattern<QByteArray> for () {
  fn pattern(self , rsthis: & QByteArrayMatcher) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 1040)};
    // unsafe{_ZNK17QByteArrayMatcher7patternEv()};
    let mut ret = unsafe {_ZNK17QByteArrayMatcher7patternEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QByteArrayMatcher::indexIn(const char * str, int len, int from);
impl /*struct*/ QByteArrayMatcher {
  pub fn indexIn<RetType, T: QByteArrayMatcher_indexIn<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indexIn(self);
    // return 1;
  }
}

pub trait QByteArrayMatcher_indexIn<RetType> {
  fn indexIn(self , rsthis: & QByteArrayMatcher) -> RetType;
}

  // proto:  int QByteArrayMatcher::indexIn(const char * str, int len, int from);
impl<'a> /*trait*/ QByteArrayMatcher_indexIn<i32> for (&'a  String, i32, i32) {
  fn indexIn(self , rsthis: & QByteArrayMatcher) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 1040)};
    // unsafe{_ZNK17QByteArrayMatcher7indexInEPKcii()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZNK17QByteArrayMatcher7indexInEPKcii(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QByteArrayMatcher::setPattern(const QByteArray & pattern);
impl /*struct*/ QByteArrayMatcher {
  pub fn setPattern<RetType, T: QByteArrayMatcher_setPattern<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPattern(self);
    // return 1;
  }
}

pub trait QByteArrayMatcher_setPattern<RetType> {
  fn setPattern(self , rsthis: & QByteArrayMatcher) -> RetType;
}

  // proto:  void QByteArrayMatcher::setPattern(const QByteArray & pattern);
impl<'a> /*trait*/ QByteArrayMatcher_setPattern<()> for (&'a QByteArray) {
  fn setPattern(self , rsthis: & QByteArrayMatcher) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 1040)};
    // unsafe{_ZN17QByteArrayMatcher10setPatternERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QByteArrayMatcher10setPatternERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QByteArrayMatcher::QByteArrayMatcher();
impl /*struct*/ QByteArrayMatcher {
  pub fn new<T: QByteArrayMatcher_new>(value: T) -> QByteArrayMatcher {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QByteArrayMatcher_new {
  fn new(self) -> QByteArrayMatcher;
}

  // proto:  void QByteArrayMatcher::QByteArrayMatcher();
impl<'a> /*trait*/ QByteArrayMatcher_new for () {
  fn new(self) -> QByteArrayMatcher {
    // let qthis: *mut c_void = unsafe{calloc(1, 1040)};
    // unsafe{_ZN17QByteArrayMatcherC2Ev()};
    let ctysz: c_int = unsafe{QByteArrayMatcher_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN17QByteArrayMatcherC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QByteArrayMatcher{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QByteArrayMatcher::QByteArrayMatcher(const char * pattern, int length);
impl<'a> /*trait*/ QByteArrayMatcher_new for (&'a  String, i32) {
  fn new(self) -> QByteArrayMatcher {
    // let qthis: *mut c_void = unsafe{calloc(1, 1040)};
    // unsafe{_ZN17QByteArrayMatcherC2EPKci()};
    let ctysz: c_int = unsafe{QByteArrayMatcher_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    unsafe {_ZN17QByteArrayMatcherC2EPKci(qthis_ph, arg0, arg1)};
    let qthis: u64 = qthis_ph;
    let rsthis = QByteArrayMatcher{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QByteArrayMatcher::QByteArrayMatcher(const QByteArray & pattern);
impl<'a> /*trait*/ QByteArrayMatcher_new for (&'a QByteArray) {
  fn new(self) -> QByteArrayMatcher {
    // let qthis: *mut c_void = unsafe{calloc(1, 1040)};
    // unsafe{_ZN17QByteArrayMatcherC2ERK10QByteArray()};
    let ctysz: c_int = unsafe{QByteArrayMatcher_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QByteArrayMatcherC2ERK10QByteArray(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QByteArrayMatcher{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QByteArrayMatcher::indexIn(const QByteArray & ba, int from);
impl<'a> /*trait*/ QByteArrayMatcher_indexIn<i32> for (&'a QByteArray, i32) {
  fn indexIn(self , rsthis: & QByteArrayMatcher) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 1040)};
    // unsafe{_ZNK17QByteArrayMatcher7indexInERK10QByteArrayi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK17QByteArrayMatcher7indexInERK10QByteArrayi(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QByteArrayMatcher::QByteArrayMatcher(const QByteArrayMatcher & other);
impl<'a> /*trait*/ QByteArrayMatcher_new for (&'a QByteArrayMatcher) {
  fn new(self) -> QByteArrayMatcher {
    // let qthis: *mut c_void = unsafe{calloc(1, 1040)};
    // unsafe{_ZN17QByteArrayMatcherC2ERKS_()};
    let ctysz: c_int = unsafe{QByteArrayMatcher_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QByteArrayMatcherC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QByteArrayMatcher{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QByteArrayMatcher::~QByteArrayMatcher();
impl /*struct*/ QByteArrayMatcher {
  pub fn free<RetType, T: QByteArrayMatcher_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QByteArrayMatcher_free<RetType> {
  fn free(self , rsthis: & QByteArrayMatcher) -> RetType;
}

  // proto:  void QByteArrayMatcher::~QByteArrayMatcher();
impl<'a> /*trait*/ QByteArrayMatcher_free<()> for () {
  fn free(self , rsthis: & QByteArrayMatcher) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 1040)};
    // unsafe{_ZN17QByteArrayMatcherD2Ev()};
     unsafe {_ZN17QByteArrayMatcherD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

