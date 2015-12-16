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
  // proto:  int QStringList::lastIndexOf(const QRegularExpression & re, int from);
  fn _ZNK11QStringList11lastIndexOfERK18QRegularExpressioni(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  void QStringList::NewQStringList();
  fn _ZN11QStringListC1Ev(qthis: *mut c_void) ;
  // proto:  int QStringList::indexOf(const QRegExp & rx, int from);
  fn _ZNK11QStringList7indexOfERK7QRegExpi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  int QStringList::indexOf(QRegExp & rx, int from);
  fn _ZNK11QStringList7indexOfER7QRegExpi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  int QStringList::indexOf(const QRegularExpression & re, int from);
  fn _ZNK11QStringList7indexOfERK18QRegularExpressioni(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  int QStringList::lastIndexOf(const QRegExp & rx, int from);
  fn _ZNK11QStringList11lastIndexOfERK7QRegExpi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  int QStringList::lastIndexOf(QRegExp & rx, int from);
  fn _ZNK11QStringList11lastIndexOfER7QRegExpi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  void QStringList::NewQStringList(const QString & i);
  fn _ZN11QStringListC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QStringList)=1
pub struct QStringList {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStringList {
  pub fn lastIndexOf<T: QStringList_lastIndexOf>(&mut self, value: T) -> i32 {
    return value.lastIndexOf(self);
    // return 1;
  }
}

pub trait QStringList_lastIndexOf {
  fn lastIndexOf(self, rsthis: &mut QStringList) -> i32;
}

// proto:  int QStringList::lastIndexOf(const QRegularExpression & re, int from);
impl<'a> /*trait*/ QStringList_lastIndexOf for (&'a  QRegularExpression, i32) {
  fn lastIndexOf(self, rsthis: &mut QStringList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStringList11lastIndexOfERK18QRegularExpressioni()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK11QStringList11lastIndexOfERK18QRegularExpressioni(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
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
    return value.indexOf(self);
    // return 1;
  }
}

pub trait QStringList_indexOf {
  fn indexOf(self, rsthis: &mut QStringList) -> i32;
}

// proto:  int QStringList::indexOf(const QRegExp & rx, int from);
impl<'a> /*trait*/ QStringList_indexOf for (&'a  QRegExp, i32) {
  fn indexOf(self, rsthis: &mut QStringList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStringList7indexOfERK7QRegExpi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK11QStringList7indexOfERK7QRegExpi(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

// proto:  int QStringList::indexOf(QRegExp & rx, int from);
impl<'a> /*trait*/ QStringList_indexOf for (&'a mut QRegExp, i32) {
  fn indexOf(self, rsthis: &mut QStringList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStringList7indexOfER7QRegExpi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK11QStringList7indexOfER7QRegExpi(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

// proto:  int QStringList::indexOf(const QRegularExpression & re, int from);
impl<'a> /*trait*/ QStringList_indexOf for (&'a  QRegularExpression, i32) {
  fn indexOf(self, rsthis: &mut QStringList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStringList7indexOfERK18QRegularExpressioni()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK11QStringList7indexOfERK18QRegularExpressioni(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

// proto:  int QStringList::lastIndexOf(const QRegExp & rx, int from);
impl<'a> /*trait*/ QStringList_lastIndexOf for (&'a  QRegExp, i32) {
  fn lastIndexOf(self, rsthis: &mut QStringList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStringList11lastIndexOfERK7QRegExpi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK11QStringList11lastIndexOfERK7QRegExpi(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

// proto:  int QStringList::lastIndexOf(QRegExp & rx, int from);
impl<'a> /*trait*/ QStringList_lastIndexOf for (&'a mut QRegExp, i32) {
  fn lastIndexOf(self, rsthis: &mut QStringList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStringList11lastIndexOfER7QRegExpi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK11QStringList11lastIndexOfER7QRegExpi(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

// proto: void QStringList::NewQStringList(const QString & i);
impl<'a> /*trait*/ QStringList_NewQStringList for (&'a  QString) {
  fn NewQStringList(self) -> QStringList {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStringListC1ERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QStringListC1ERK7QString(qthis, arg0)};
    let rsthis = QStringList{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

