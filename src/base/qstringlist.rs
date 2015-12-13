// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qregularexpression::QRegularExpression;
use super::qregexp::QRegExp;
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZNK11QStringList11lastIndexOfERK18QRegularExpressioni(arg0: *const c_void, arg1: c_int) -> i32;
  fn _ZN11QStringListC1Ev(qthis: *mut c_void) -> i32;
  fn _ZNK11QStringList7indexOfERK7QRegExpi(arg0: *const c_void, arg1: c_int) -> i32;
  fn _ZNK11QStringList7indexOfER7QRegExpi(arg0: *mut c_void, arg1: c_int) -> i32;
  fn _ZNK11QStringList7indexOfERK18QRegularExpressioni(arg0: *const c_void, arg1: c_int) -> i32;
  fn _ZNK11QStringList11lastIndexOfERK7QRegExpi(arg0: *const c_void, arg1: c_int) -> i32;
  fn _ZNK11QStringList11lastIndexOfER7QRegExpi(arg0: *mut c_void, arg1: c_int) -> i32;
  fn _ZN11QStringListC1ERK7QString(qthis: *mut c_void, arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QStringList)=1
pub struct QStringList {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStringList {
  pub fn lastIndexOf<T: QStringList_lastIndexOf>(&mut self, value: T) -> i32 {
    value.lastIndexOf(self);
    return 1;
  }
}

pub trait QStringList_lastIndexOf {
  fn lastIndexOf(self, this: &mut QStringList) -> i32;
}

// proto: int QStringList::lastIndexOf(const QRegularExpression & re, int from);
impl<'a> /*trait*/ QStringList_lastIndexOf for (&'a  QRegularExpression, i32) {
  fn lastIndexOf(self, this: &mut QStringList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStringList11lastIndexOfERK18QRegularExpressioni()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK11QStringList11lastIndexOfERK18QRegularExpressioni(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QStringList {
  pub fn NewQStringList<T: QStringList_NewQStringList>(value: T) -> QStringList {
    let rsthis = value.NewQStringList();
    return rsthis;
    // return 1;
  }
}

pub trait QStringList_NewQStringList {
  fn NewQStringList(self) -> QStringList;
}

// proto: void QStringList::NewQStringList();
impl<'a> /*trait*/ QStringList_NewQStringList for () {
  fn NewQStringList(self) -> QStringList {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStringListC1Ev()};
    unsafe {_ZN11QStringListC1Ev(qthis)};
    let rsthis = QStringList{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStringList {
  pub fn indexOf<T: QStringList_indexOf>(&mut self, value: T) -> i32 {
    value.indexOf(self);
    return 1;
  }
}

pub trait QStringList_indexOf {
  fn indexOf(self, this: &mut QStringList) -> i32;
}

// proto: int QStringList::indexOf(const QRegExp & rx, int from);
impl<'a> /*trait*/ QStringList_indexOf for (&'a  QRegExp, i32) {
  fn indexOf(self, this: &mut QStringList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStringList7indexOfERK7QRegExpi()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK11QStringList7indexOfERK7QRegExpi(arg0, arg1)};
    return 1;
  }
}

// proto: int QStringList::indexOf(QRegExp & rx, int from);
impl<'a> /*trait*/ QStringList_indexOf for (&'a mut QRegExp, i32) {
  fn indexOf(self, this: &mut QStringList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStringList7indexOfER7QRegExpi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK11QStringList7indexOfER7QRegExpi(arg0, arg1)};
    return 1;
  }
}

// proto: int QStringList::indexOf(const QRegularExpression & re, int from);
impl<'a> /*trait*/ QStringList_indexOf for (&'a  QRegularExpression, i32) {
  fn indexOf(self, this: &mut QStringList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStringList7indexOfERK18QRegularExpressioni()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK11QStringList7indexOfERK18QRegularExpressioni(arg0, arg1)};
    return 1;
  }
}

// proto: int QStringList::lastIndexOf(const QRegExp & rx, int from);
impl<'a> /*trait*/ QStringList_lastIndexOf for (&'a  QRegExp, i32) {
  fn lastIndexOf(self, this: &mut QStringList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStringList11lastIndexOfERK7QRegExpi()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK11QStringList11lastIndexOfERK7QRegExpi(arg0, arg1)};
    return 1;
  }
}

// proto: int QStringList::lastIndexOf(QRegExp & rx, int from);
impl<'a> /*trait*/ QStringList_lastIndexOf for (&'a mut QRegExp, i32) {
  fn lastIndexOf(self, this: &mut QStringList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStringList11lastIndexOfER7QRegExpi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK11QStringList11lastIndexOfER7QRegExpi(arg0, arg1)};
    return 1;
  }
}

// proto: void QStringList::NewQStringList(const QString & i);
impl<'a> /*trait*/ QStringList_NewQStringList for (&'a  QString) {
  fn NewQStringList(self) -> QStringList {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStringListC1ERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QStringListC1ERK7QString(qthis, arg0)};
    let rsthis = QStringList{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

