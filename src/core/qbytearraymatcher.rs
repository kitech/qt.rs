// auto generated, do not modify.
// created: Sat Dec 26 10:16:52 2015
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
  // proto:  int QByteArrayMatcher::indexIn(const char * str, int len, int from);
  fn _ZNK17QByteArrayMatcher7indexInEPKcii(qthis: *mut c_void, arg0: *mut c_char, arg1: c_int, arg2: c_int) -> c_int;
  // proto:  void QByteArrayMatcher::setPattern(const QByteArray & pattern);
  fn _ZN17QByteArrayMatcher10setPatternERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QByteArrayMatcher::QByteArrayMatcher();
  fn dector_ZN17QByteArrayMatcherC1Ev() -> *mut c_void;
  fn _ZN17QByteArrayMatcherC1Ev(qthis: *mut c_void);
  // proto:  void QByteArrayMatcher::QByteArrayMatcher(const char * pattern, int length);
  fn dector_ZN17QByteArrayMatcherC1EPKci(arg0: *mut c_char, arg1: c_int) -> *mut c_void;
  fn _ZN17QByteArrayMatcherC1EPKci(qthis: *mut c_void, arg0: *mut c_char, arg1: c_int);
  // proto:  void QByteArrayMatcher::QByteArrayMatcher(const QByteArray & pattern);
  fn dector_ZN17QByteArrayMatcherC1ERK10QByteArray(arg0: *mut c_void) -> *mut c_void;
  fn _ZN17QByteArrayMatcherC1ERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QByteArrayMatcher::indexIn(const QByteArray & ba, int from);
  fn _ZNK17QByteArrayMatcher7indexInERK10QByteArrayi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  void QByteArrayMatcher::QByteArrayMatcher(const QByteArrayMatcher & other);
  fn dector_ZN17QByteArrayMatcherC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN17QByteArrayMatcherC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QByteArrayMatcher::~QByteArrayMatcher();
  fn _ZN17QByteArrayMatcherD0Ev(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QByteArrayMatcher)=1040
pub struct QByteArrayMatcher {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QByteArrayMatcher {
  pub fn inheritFrom(qthis: *mut c_void) -> QByteArrayMatcher {
    return QByteArrayMatcher{qclsinst: qthis};
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
  pub fn New<T: QByteArrayMatcher_New>(value: T) -> QByteArrayMatcher {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QByteArrayMatcher_New {
  fn New(self) -> QByteArrayMatcher;
}

  // proto:  void QByteArrayMatcher::QByteArrayMatcher();
impl<'a> /*trait*/ QByteArrayMatcher_New for () {
  fn New(self) -> QByteArrayMatcher {
    // let qthis: *mut c_void = unsafe{calloc(1, 1040)};
    // unsafe{_ZN17QByteArrayMatcherC1Ev()};
    let ctysz: c_int = unsafe{QByteArrayMatcher_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN17QByteArrayMatcherC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN17QByteArrayMatcherC1Ev()};
    let rsthis = QByteArrayMatcher{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QByteArrayMatcher::QByteArrayMatcher(const char * pattern, int length);
impl<'a> /*trait*/ QByteArrayMatcher_New for (&'a  String, i32) {
  fn New(self) -> QByteArrayMatcher {
    // let qthis: *mut c_void = unsafe{calloc(1, 1040)};
    // unsafe{_ZN17QByteArrayMatcherC1EPKci()};
    let ctysz: c_int = unsafe{QByteArrayMatcher_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    // unsafe {_ZN17QByteArrayMatcherC1EPKci(qthis, arg0, arg1)};
    let qthis: *mut c_void = unsafe {dector_ZN17QByteArrayMatcherC1EPKci(arg0, arg1)};
    let rsthis = QByteArrayMatcher{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QByteArrayMatcher::QByteArrayMatcher(const QByteArray & pattern);
impl<'a> /*trait*/ QByteArrayMatcher_New for (&'a QByteArray) {
  fn New(self) -> QByteArrayMatcher {
    // let qthis: *mut c_void = unsafe{calloc(1, 1040)};
    // unsafe{_ZN17QByteArrayMatcherC1ERK10QByteArray()};
    let ctysz: c_int = unsafe{QByteArrayMatcher_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN17QByteArrayMatcherC1ERK10QByteArray(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN17QByteArrayMatcherC1ERK10QByteArray(arg0)};
    let rsthis = QByteArrayMatcher{qclsinst: qthis};
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
impl<'a> /*trait*/ QByteArrayMatcher_New for (&'a QByteArrayMatcher) {
  fn New(self) -> QByteArrayMatcher {
    // let qthis: *mut c_void = unsafe{calloc(1, 1040)};
    // unsafe{_ZN17QByteArrayMatcherC1ERKS_()};
    let ctysz: c_int = unsafe{QByteArrayMatcher_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN17QByteArrayMatcherC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN17QByteArrayMatcherC1ERKS_(arg0)};
    let rsthis = QByteArrayMatcher{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QByteArrayMatcher::~QByteArrayMatcher();
impl /*struct*/ QByteArrayMatcher {
  pub fn Free<RetType, T: QByteArrayMatcher_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QByteArrayMatcher_Free<RetType> {
  fn Free(self , rsthis: & QByteArrayMatcher) -> RetType;
}

  // proto:  void QByteArrayMatcher::~QByteArrayMatcher();
impl<'a> /*trait*/ QByteArrayMatcher_Free<()> for () {
  fn Free(self , rsthis: & QByteArrayMatcher) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 1040)};
    // unsafe{_ZN17QByteArrayMatcherD0Ev()};
     unsafe {_ZN17QByteArrayMatcherD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

