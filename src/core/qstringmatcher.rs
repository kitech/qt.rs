// auto generated, do not modify.
// created: Wed Dec 30 23:22:52 2015
// src-file: /QtCore/qstringmatcher.h
// dst-file: /src/core/qstringmatcher.rs
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
use super::qchar::QChar; // 773
use super::qstring::QString; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QStringMatcher_Class_Size() -> c_int;
  // proto:  void QStringMatcher::QStringMatcher();
  fn dector_ZN14QStringMatcherC1Ev() -> *mut c_void;
  fn _ZN14QStringMatcherC1Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QStringMatcher::QStringMatcher(const QStringMatcher & other);
  fn dector_ZN14QStringMatcherC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN14QStringMatcherC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QStringMatcher::indexIn(const QChar * str, int length, int from);
  fn _ZNK14QStringMatcher7indexInEPK5QCharii(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int, arg2: c_int) -> c_int;
  // proto:  void QStringMatcher::setPattern(const QString & pattern);
  fn _ZN14QStringMatcher10setPatternERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QStringMatcher::pattern();
  fn _ZNK14QStringMatcher7patternEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QStringMatcher::~QStringMatcher();
  fn _ZN14QStringMatcherD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  int QStringMatcher::indexIn(const QString & str, int from);
  fn _ZNK14QStringMatcher7indexInERK7QStringi(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int) -> c_int;
} // <= ext block end

// body block begin =>
// class sizeof(QStringMatcher)=1048
#[derive(Default)]
pub struct QStringMatcher {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QStringMatcher {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStringMatcher {
    return QStringMatcher{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QStringMatcher::QStringMatcher();
impl /*struct*/ QStringMatcher {
  pub fn New<T: QStringMatcher_New>(value: T) -> QStringMatcher {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QStringMatcher_New {
  fn New(self) -> QStringMatcher;
}

  // proto:  void QStringMatcher::QStringMatcher();
impl<'a> /*trait*/ QStringMatcher_New for () {
  fn New(self) -> QStringMatcher {
    // let qthis: *mut c_void = unsafe{calloc(1, 1048)};
    // unsafe{_ZN14QStringMatcherC1Ev()};
    let ctysz: c_int = unsafe{QStringMatcher_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN14QStringMatcherC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN14QStringMatcherC1Ev()} as u64;
    let rsthis = QStringMatcher{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStringMatcher::QStringMatcher(const QStringMatcher & other);
impl<'a> /*trait*/ QStringMatcher_New for (&'a QStringMatcher) {
  fn New(self) -> QStringMatcher {
    // let qthis: *mut c_void = unsafe{calloc(1, 1048)};
    // unsafe{_ZN14QStringMatcherC1ERKS_()};
    let ctysz: c_int = unsafe{QStringMatcher_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN14QStringMatcherC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN14QStringMatcherC1ERKS_(arg0)} as u64;
    let rsthis = QStringMatcher{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QStringMatcher::indexIn(const QChar * str, int length, int from);
impl /*struct*/ QStringMatcher {
  pub fn indexIn<RetType, T: QStringMatcher_indexIn<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indexIn(self);
    // return 1;
  }
}

pub trait QStringMatcher_indexIn<RetType> {
  fn indexIn(self , rsthis: & QStringMatcher) -> RetType;
}

  // proto:  int QStringMatcher::indexIn(const QChar * str, int length, int from);
impl<'a> /*trait*/ QStringMatcher_indexIn<i32> for (&'a QChar, i32, i32) {
  fn indexIn(self , rsthis: & QStringMatcher) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 1048)};
    // unsafe{_ZNK14QStringMatcher7indexInEPK5QCharii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZNK14QStringMatcher7indexInEPK5QCharii(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QStringMatcher::setPattern(const QString & pattern);
impl /*struct*/ QStringMatcher {
  pub fn setPattern<RetType, T: QStringMatcher_setPattern<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPattern(self);
    // return 1;
  }
}

pub trait QStringMatcher_setPattern<RetType> {
  fn setPattern(self , rsthis: & QStringMatcher) -> RetType;
}

  // proto:  void QStringMatcher::setPattern(const QString & pattern);
impl<'a> /*trait*/ QStringMatcher_setPattern<()> for (&'a QString) {
  fn setPattern(self , rsthis: & QStringMatcher) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 1048)};
    // unsafe{_ZN14QStringMatcher10setPatternERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QStringMatcher10setPatternERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QStringMatcher::pattern();
impl /*struct*/ QStringMatcher {
  pub fn pattern<RetType, T: QStringMatcher_pattern<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pattern(self);
    // return 1;
  }
}

pub trait QStringMatcher_pattern<RetType> {
  fn pattern(self , rsthis: & QStringMatcher) -> RetType;
}

  // proto:  QString QStringMatcher::pattern();
impl<'a> /*trait*/ QStringMatcher_pattern<QString> for () {
  fn pattern(self , rsthis: & QStringMatcher) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 1048)};
    // unsafe{_ZNK14QStringMatcher7patternEv()};
    let mut ret = unsafe {_ZNK14QStringMatcher7patternEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QStringMatcher::~QStringMatcher();
impl /*struct*/ QStringMatcher {
  pub fn Free<RetType, T: QStringMatcher_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QStringMatcher_Free<RetType> {
  fn Free(self , rsthis: & QStringMatcher) -> RetType;
}

  // proto:  void QStringMatcher::~QStringMatcher();
impl<'a> /*trait*/ QStringMatcher_Free<()> for () {
  fn Free(self , rsthis: & QStringMatcher) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 1048)};
    // unsafe{_ZN14QStringMatcherD0Ev()};
     unsafe {_ZN14QStringMatcherD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QStringMatcher::indexIn(const QString & str, int from);
impl<'a> /*trait*/ QStringMatcher_indexIn<i32> for (&'a QString, i32) {
  fn indexIn(self , rsthis: & QStringMatcher) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 1048)};
    // unsafe{_ZNK14QStringMatcher7indexInERK7QStringi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK14QStringMatcher7indexInERK7QStringi(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

// <= body block end

