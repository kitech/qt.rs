// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
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
use super::qregularexpression::*; // 773
use super::qstring::*; // 773
use super::qregexp::*; // 773
// use super::qstringlist::QStringList; // 773
use super::qchar::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QStringList_Class_Size() -> c_int;
  // proto:  int QStringList::lastIndexOf(const QRegularExpression & re, int from);
  fn C_ZNK11QStringList11lastIndexOfERK18QRegularExpressioni(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  void QStringList::QStringList();
  fn C_ZN11QStringListC2Ev() -> u64;
  // proto:  int QStringList::indexOf(const QRegExp & rx, int from);
  fn C_ZNK11QStringList7indexOfERK7QRegExpi(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  int QStringList::indexOf(QRegExp & rx, int from);
  fn C_ZNK11QStringList7indexOfER7QRegExpi(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  int QStringList::indexOf(const QRegularExpression & re, int from);
  fn C_ZNK11QStringList7indexOfERK18QRegularExpressioni(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  int QStringList::lastIndexOf(const QRegExp & rx, int from);
  fn C_ZNK11QStringList11lastIndexOfERK7QRegExpi(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  int QStringList::lastIndexOf(QRegExp & rx, int from);
  fn C_ZNK11QStringList11lastIndexOfER7QRegExpi(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  void QStringList::QStringList(const QString & i);
  fn C_ZN11QStringListC2ERK7QString(arg0: *mut c_void) -> u64;
  fn QListSpecialMethodsLQStringG_Class_Size() -> c_int;
  // proto:  QStringList & QListSpecialMethods<QString>::replaceInStrings(const QRegularExpression & re, const QString & after);
  fn C_ZN19QListSpecialMethodsI7QStringE16replaceInStringsERK18QRegularExpressionRKS0_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QString QListSpecialMethods<QString>::join(const QString & sep);
  fn C_ZNK19QListSpecialMethodsI7QStringE4joinERKS0_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QStringList QListSpecialMethods<QString>::filter(const QRegularExpression & re);
  fn C_ZNK19QListSpecialMethodsI7QStringE6filterERK18QRegularExpression(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QListSpecialMethods<QString>::removeDuplicates();
  fn C_ZN19QListSpecialMethodsI7QStringE16removeDuplicatesEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QStringList & QListSpecialMethods<QString>::replaceInStrings(const QRegExp & rx, const QString & after);
  fn C_ZN19QListSpecialMethodsI7QStringE16replaceInStringsERK7QRegExpRKS0_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QStringList QListSpecialMethods<QString>::filter(const QRegExp & rx);
  fn C_ZNK19QListSpecialMethodsI7QStringE6filterERK7QRegExp(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QString QListSpecialMethods<QString>::join(QChar sep);
  fn C_ZNK19QListSpecialMethodsI7QStringE4joinE5QChar(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QStringList)=1
#[derive(Default)]
pub struct QStringList {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QListSpecialMethodsLQStringG)=1
#[derive(Default)]
pub struct QListSpecialMethodsLQStringG {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QStringList {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStringList {
    return QStringList{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  int QStringList::lastIndexOf(const QRegularExpression & re, int from);
impl /*struct*/ QStringList {
  pub fn lastIndexOf<RetType, T: QStringList_lastIndexOf<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lastIndexOf(self);
    // return 1;
  }
}

pub trait QStringList_lastIndexOf<RetType> {
  fn lastIndexOf(self , rsthis: & QStringList) -> RetType;
}

  // proto:  int QStringList::lastIndexOf(const QRegularExpression & re, int from);
impl<'a> /*trait*/ QStringList_lastIndexOf<i32> for (&'a QRegularExpression, i32) {
  fn lastIndexOf(self , rsthis: & QStringList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStringList11lastIndexOfERK18QRegularExpressioni()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZNK11QStringList11lastIndexOfERK18QRegularExpressioni(rsthis.qclsinst, arg0, arg1)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QStringList::QStringList();
impl /*struct*/ QStringList {
  pub fn new<T: QStringList_new>(value: T) -> QStringList {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStringList_new {
  fn new(self) -> QStringList;
}

  // proto:  void QStringList::QStringList();
impl<'a> /*trait*/ QStringList_new for () {
  fn new(self) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStringListC2Ev()};
    let ctysz: c_int = unsafe{QStringList_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN11QStringListC2Ev()};
    let rsthis = QStringList{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QStringList::indexOf(const QRegExp & rx, int from);
impl /*struct*/ QStringList {
  pub fn indexOf<RetType, T: QStringList_indexOf<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indexOf(self);
    // return 1;
  }
}

pub trait QStringList_indexOf<RetType> {
  fn indexOf(self , rsthis: & QStringList) -> RetType;
}

  // proto:  int QStringList::indexOf(const QRegExp & rx, int from);
impl<'a> /*trait*/ QStringList_indexOf<i32> for (&'a QRegExp, i32) {
  fn indexOf(self , rsthis: & QStringList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStringList7indexOfERK7QRegExpi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZNK11QStringList7indexOfERK7QRegExpi(rsthis.qclsinst, arg0, arg1)};
    return ret as i32; // 1
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
    let mut ret = unsafe {C_ZNK11QStringList7indexOfERK18QRegularExpressioni(rsthis.qclsinst, arg0, arg1)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QStringList::lastIndexOf(const QRegExp & rx, int from);
impl<'a> /*trait*/ QStringList_lastIndexOf<i32> for (&'a QRegExp, i32) {
  fn lastIndexOf(self , rsthis: & QStringList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStringList11lastIndexOfERK7QRegExpi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZNK11QStringList11lastIndexOfERK7QRegExpi(rsthis.qclsinst, arg0, arg1)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QStringList::QStringList(const QString & i);
impl<'a> /*trait*/ QStringList_new for (&'a QString) {
  fn new(self) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStringListC2ERK7QString()};
    let ctysz: c_int = unsafe{QStringList_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN11QStringListC2ERK7QString(arg0)};
    let rsthis = QStringList{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QListSpecialMethodsLQStringG {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QListSpecialMethodsLQStringG {
    return QListSpecialMethodsLQStringG{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QStringList & QListSpecialMethods<QString>::replaceInStrings(const QRegularExpression & re, const QString & after);
impl /*struct*/ QListSpecialMethodsLQStringG {
  pub fn replaceInStrings<RetType, T: QListSpecialMethodsLQStringG_replaceInStrings<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.replaceInStrings(self);
    // return 1;
  }
}

pub trait QListSpecialMethodsLQStringG_replaceInStrings<RetType> {
  fn replaceInStrings(self , rsthis: & QListSpecialMethodsLQStringG) -> RetType;
}

  // proto:  QStringList & QListSpecialMethods<QString>::replaceInStrings(const QRegularExpression & re, const QString & after);
impl<'a> /*trait*/ QListSpecialMethodsLQStringG_replaceInStrings<QStringList> for (&'a QRegularExpression, &'a QString) {
  fn replaceInStrings(self , rsthis: & QListSpecialMethodsLQStringG) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QListSpecialMethodsI7QStringE16replaceInStringsERK18QRegularExpressionRKS0_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN19QListSpecialMethodsI7QStringE16replaceInStringsERK18QRegularExpressionRKS0_(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QStringList::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QListSpecialMethods<QString>::join(const QString & sep);
impl /*struct*/ QListSpecialMethodsLQStringG {
  pub fn join<RetType, T: QListSpecialMethodsLQStringG_join<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.join(self);
    // return 1;
  }
}

pub trait QListSpecialMethodsLQStringG_join<RetType> {
  fn join(self , rsthis: & QListSpecialMethodsLQStringG) -> RetType;
}

  // proto:  QString QListSpecialMethods<QString>::join(const QString & sep);
impl<'a> /*trait*/ QListSpecialMethodsLQStringG_join<QString> for (&'a QString) {
  fn join(self , rsthis: & QListSpecialMethodsLQStringG) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QListSpecialMethodsI7QStringE4joinERKS0_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK19QListSpecialMethodsI7QStringE4joinERKS0_(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QStringList QListSpecialMethods<QString>::filter(const QRegularExpression & re);
impl /*struct*/ QListSpecialMethodsLQStringG {
  pub fn filter<RetType, T: QListSpecialMethodsLQStringG_filter<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.filter(self);
    // return 1;
  }
}

pub trait QListSpecialMethodsLQStringG_filter<RetType> {
  fn filter(self , rsthis: & QListSpecialMethodsLQStringG) -> RetType;
}

  // proto:  QStringList QListSpecialMethods<QString>::filter(const QRegularExpression & re);
impl<'a> /*trait*/ QListSpecialMethodsLQStringG_filter<QStringList> for (&'a QRegularExpression) {
  fn filter(self , rsthis: & QListSpecialMethodsLQStringG) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QListSpecialMethodsI7QStringE6filterERK18QRegularExpression()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK19QListSpecialMethodsI7QStringE6filterERK18QRegularExpression(rsthis.qclsinst, arg0)};
    let mut ret1 = QStringList::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QListSpecialMethods<QString>::removeDuplicates();
impl /*struct*/ QListSpecialMethodsLQStringG {
  pub fn removeDuplicates<RetType, T: QListSpecialMethodsLQStringG_removeDuplicates<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeDuplicates(self);
    // return 1;
  }
}

pub trait QListSpecialMethodsLQStringG_removeDuplicates<RetType> {
  fn removeDuplicates(self , rsthis: & QListSpecialMethodsLQStringG) -> RetType;
}

  // proto:  int QListSpecialMethods<QString>::removeDuplicates();
impl<'a> /*trait*/ QListSpecialMethodsLQStringG_removeDuplicates<i32> for () {
  fn removeDuplicates(self , rsthis: & QListSpecialMethodsLQStringG) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QListSpecialMethodsI7QStringE16removeDuplicatesEv()};
    let mut ret = unsafe {C_ZN19QListSpecialMethodsI7QStringE16removeDuplicatesEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QStringList & QListSpecialMethods<QString>::replaceInStrings(const QRegExp & rx, const QString & after);
impl<'a> /*trait*/ QListSpecialMethodsLQStringG_replaceInStrings<QStringList> for (&'a QRegExp, &'a QString) {
  fn replaceInStrings(self , rsthis: & QListSpecialMethodsLQStringG) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QListSpecialMethodsI7QStringE16replaceInStringsERK7QRegExpRKS0_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN19QListSpecialMethodsI7QStringE16replaceInStringsERK7QRegExpRKS0_(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QStringList::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QStringList QListSpecialMethods<QString>::filter(const QRegExp & rx);
impl<'a> /*trait*/ QListSpecialMethodsLQStringG_filter<QStringList> for (&'a QRegExp) {
  fn filter(self , rsthis: & QListSpecialMethodsLQStringG) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QListSpecialMethodsI7QStringE6filterERK7QRegExp()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK19QListSpecialMethodsI7QStringE6filterERK7QRegExp(rsthis.qclsinst, arg0)};
    let mut ret1 = QStringList::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QListSpecialMethods<QString>::join(QChar sep);
impl<'a> /*trait*/ QListSpecialMethodsLQStringG_join<QString> for (QChar) {
  fn join(self , rsthis: & QListSpecialMethodsLQStringG) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QListSpecialMethodsI7QStringE4joinE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK19QListSpecialMethodsI7QStringE4joinE5QChar(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

