// auto generated, do not modify.
// created: Sat Dec 26 10:16:52 2015
// src-file: /QtCore/qstringlist.h
// dst-file: /src/core/qstringlist.rs
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
use super::qregexp::QRegExp; // 773
use super::qregularexpression::QRegularExpression; // 773
use super::qstring::QString; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QStringList_Class_Size() -> c_int;
  // proto:  int QStringList::indexOf(QRegExp & rx, int from);
  fn _ZNK11QStringList7indexOfER7QRegExpi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  int QStringList::lastIndexOf(const QRegExp & rx, int from);
  fn _ZNK11QStringList11lastIndexOfERK7QRegExpi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  int QStringList::indexOf(const QRegularExpression & re, int from);
  fn _ZNK11QStringList7indexOfERK18QRegularExpressioni(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  int QStringList::lastIndexOf(const QRegularExpression & re, int from);
  fn _ZNK11QStringList11lastIndexOfERK18QRegularExpressioni(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  int QStringList::indexOf(const QRegExp & rx, int from);
  fn _ZNK11QStringList7indexOfERK7QRegExpi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  int QStringList::lastIndexOf(QRegExp & rx, int from);
  fn _ZNK11QStringList11lastIndexOfER7QRegExpi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> c_int;
} // <= ext block end

// body block begin =>
// class sizeof(QStringList)=1
pub struct QStringList {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStringList {
  pub fn inheritFrom(qthis: *mut c_void) -> QStringList {
    return QStringList{qclsinst: qthis};
  }
}
  // proto:  int QStringList::indexOf(QRegExp & rx, int from);
impl /*struct*/ QStringList {
  pub fn indexOf<RetType, T: QStringList_indexOf<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indexOf(self);
    // return 1;
  }
}

pub trait QStringList_indexOf<RetType> {
  fn indexOf(self , rsthis: & QStringList) -> RetType;
}

  // proto:  int QStringList::indexOf(QRegExp & rx, int from);
impl<'a> /*trait*/ QStringList_indexOf<i32> for (&'a QRegExp, i32) {
  fn indexOf(self , rsthis: & QStringList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStringList7indexOfER7QRegExpi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK11QStringList7indexOfER7QRegExpi(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QStringList::lastIndexOf(const QRegExp & rx, int from);
impl /*struct*/ QStringList {
  pub fn lastIndexOf<RetType, T: QStringList_lastIndexOf<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lastIndexOf(self);
    // return 1;
  }
}

pub trait QStringList_lastIndexOf<RetType> {
  fn lastIndexOf(self , rsthis: & QStringList) -> RetType;
}

  // proto:  int QStringList::lastIndexOf(const QRegExp & rx, int from);
impl<'a> /*trait*/ QStringList_lastIndexOf<i32> for (&'a QRegExp, i32) {
  fn lastIndexOf(self , rsthis: & QStringList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStringList11lastIndexOfERK7QRegExpi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK11QStringList11lastIndexOfERK7QRegExpi(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QStringList::indexOf(const QRegularExpression & re, int from);
impl<'a> /*trait*/ QStringList_indexOf<i32> for (&'a QRegularExpression, i32) {
  fn indexOf(self , rsthis: & QStringList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStringList7indexOfERK18QRegularExpressioni()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK11QStringList7indexOfERK18QRegularExpressioni(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QStringList::lastIndexOf(const QRegularExpression & re, int from);
impl<'a> /*trait*/ QStringList_lastIndexOf<i32> for (&'a QRegularExpression, i32) {
  fn lastIndexOf(self , rsthis: & QStringList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStringList11lastIndexOfERK18QRegularExpressioni()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK11QStringList11lastIndexOfERK18QRegularExpressioni(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

// <= body block end

