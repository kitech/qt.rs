// auto generated, do not modify.
// created: Sat Dec 26 10:16:52 2015
// src-file: /QtCore/qloggingcategory.h
// dst-file: /src/core/qloggingcategory.rs
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
use super::qstring::QString; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QLoggingCategory_Class_Size() -> c_int;
  // proto:  void QLoggingCategory::QLoggingCategory(const char * category, QtMsgType severityLevel);
  fn dector_ZN16QLoggingCategoryC1EPKc9QtMsgType(arg0: *mut c_char, arg1: c_int) -> *mut c_void;
  fn _ZN16QLoggingCategoryC1EPKc9QtMsgType(qthis: *mut c_void, arg0: *mut c_char, arg1: c_int);
  // proto:  void QLoggingCategory::QLoggingCategory(const QLoggingCategory & );
  fn dector_ZN16QLoggingCategoryC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN16QLoggingCategoryC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QLoggingCategory::~QLoggingCategory();
  fn _ZN16QLoggingCategoryD0Ev(qthis: *mut c_void);
  // proto:  void QLoggingCategory::QLoggingCategory(const char * category);
  fn dector_ZN16QLoggingCategoryC1EPKc(arg0: *mut c_char) -> *mut c_void;
  fn _ZN16QLoggingCategoryC1EPKc(qthis: *mut c_void, arg0: *mut c_char);
  // proto:  void QLoggingCategory::setEnabled(QtMsgType type, bool enable);
  fn _ZN16QLoggingCategory10setEnabledE9QtMsgTypeb(qthis: *mut c_void, arg0: c_int, arg1: c_char);
  // proto:  bool QLoggingCategory::isEnabled(QtMsgType type);
  fn _ZNK16QLoggingCategory9isEnabledE9QtMsgType(qthis: *mut c_void, arg0: c_int) -> c_char;
  // proto: static QLoggingCategory * QLoggingCategory::defaultCategory();
  fn _ZN16QLoggingCategory15defaultCategoryEv() -> *mut c_void;
  // proto: static void QLoggingCategory::setFilterRules(const QString & rules);
  fn _ZN16QLoggingCategory14setFilterRulesERK7QString(arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QLoggingCategory)=24
pub struct QLoggingCategory {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QLoggingCategory {
  pub fn inheritFrom(qthis: *mut c_void) -> QLoggingCategory {
    return QLoggingCategory{qclsinst: qthis};
  }
}
  // proto:  void QLoggingCategory::QLoggingCategory(const char * category, QtMsgType severityLevel);
impl /*struct*/ QLoggingCategory {
  pub fn New<T: QLoggingCategory_New>(value: T) -> QLoggingCategory {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QLoggingCategory_New {
  fn New(self) -> QLoggingCategory;
}

  // proto:  void QLoggingCategory::QLoggingCategory(const char * category, QtMsgType severityLevel);
impl<'a> /*trait*/ QLoggingCategory_New for (&'a  String, i32) {
  fn New(self) -> QLoggingCategory {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QLoggingCategoryC1EPKc9QtMsgType()};
    let ctysz: c_int = unsafe{QLoggingCategory_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    // unsafe {_ZN16QLoggingCategoryC1EPKc9QtMsgType(qthis, arg0, arg1)};
    let qthis: *mut c_void = unsafe {dector_ZN16QLoggingCategoryC1EPKc9QtMsgType(arg0, arg1)};
    let rsthis = QLoggingCategory{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QLoggingCategory::QLoggingCategory(const QLoggingCategory & );
impl<'a> /*trait*/ QLoggingCategory_New for (&'a QLoggingCategory) {
  fn New(self) -> QLoggingCategory {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QLoggingCategoryC1ERKS_()};
    let ctysz: c_int = unsafe{QLoggingCategory_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN16QLoggingCategoryC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN16QLoggingCategoryC1ERKS_(arg0)};
    let rsthis = QLoggingCategory{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QLoggingCategory::~QLoggingCategory();
impl /*struct*/ QLoggingCategory {
  pub fn Free<RetType, T: QLoggingCategory_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QLoggingCategory_Free<RetType> {
  fn Free(self , rsthis: & QLoggingCategory) -> RetType;
}

  // proto:  void QLoggingCategory::~QLoggingCategory();
impl<'a> /*trait*/ QLoggingCategory_Free<()> for () {
  fn Free(self , rsthis: & QLoggingCategory) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QLoggingCategoryD0Ev()};
     unsafe {_ZN16QLoggingCategoryD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QLoggingCategory::QLoggingCategory(const char * category);
impl<'a> /*trait*/ QLoggingCategory_New for (&'a  String) {
  fn New(self) -> QLoggingCategory {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QLoggingCategoryC1EPKc()};
    let ctysz: c_int = unsafe{QLoggingCategory_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.as_ptr()  as *mut c_char;
    // unsafe {_ZN16QLoggingCategoryC1EPKc(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN16QLoggingCategoryC1EPKc(arg0)};
    let rsthis = QLoggingCategory{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QLoggingCategory::setEnabled(QtMsgType type, bool enable);
impl /*struct*/ QLoggingCategory {
  pub fn setEnabled<RetType, T: QLoggingCategory_setEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setEnabled(self);
    // return 1;
  }
}

pub trait QLoggingCategory_setEnabled<RetType> {
  fn setEnabled(self , rsthis: & QLoggingCategory) -> RetType;
}

  // proto:  void QLoggingCategory::setEnabled(QtMsgType type, bool enable);
impl<'a> /*trait*/ QLoggingCategory_setEnabled<()> for (i32, i8) {
  fn setEnabled(self , rsthis: & QLoggingCategory) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QLoggingCategory10setEnabledE9QtMsgTypeb()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_char;
     unsafe {_ZN16QLoggingCategory10setEnabledE9QtMsgTypeb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QLoggingCategory::isEnabled(QtMsgType type);
impl /*struct*/ QLoggingCategory {
  pub fn isEnabled<RetType, T: QLoggingCategory_isEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEnabled(self);
    // return 1;
  }
}

pub trait QLoggingCategory_isEnabled<RetType> {
  fn isEnabled(self , rsthis: & QLoggingCategory) -> RetType;
}

  // proto:  bool QLoggingCategory::isEnabled(QtMsgType type);
impl<'a> /*trait*/ QLoggingCategory_isEnabled<i8> for (i32) {
  fn isEnabled(self , rsthis: & QLoggingCategory) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QLoggingCategory9isEnabledE9QtMsgType()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK16QLoggingCategory9isEnabledE9QtMsgType(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static QLoggingCategory * QLoggingCategory::defaultCategory();
impl /*struct*/ QLoggingCategory {
  pub fn defaultCategory_s<RetType, T: QLoggingCategory_defaultCategory_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.defaultCategory_s();
    // return 1;
  }
}

pub trait QLoggingCategory_defaultCategory_s<RetType> {
  fn defaultCategory_s(self ) -> RetType;
}

  // proto: static QLoggingCategory * QLoggingCategory::defaultCategory();
impl<'a> /*trait*/ QLoggingCategory_defaultCategory_s<QLoggingCategory> for () {
  fn defaultCategory_s(self ) -> QLoggingCategory {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QLoggingCategory15defaultCategoryEv()};
    let mut ret = unsafe {_ZN16QLoggingCategory15defaultCategoryEv()};
    let mut ret1 = QLoggingCategory::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static void QLoggingCategory::setFilterRules(const QString & rules);
impl /*struct*/ QLoggingCategory {
  pub fn setFilterRules_s<RetType, T: QLoggingCategory_setFilterRules_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setFilterRules_s();
    // return 1;
  }
}

pub trait QLoggingCategory_setFilterRules_s<RetType> {
  fn setFilterRules_s(self ) -> RetType;
}

  // proto: static void QLoggingCategory::setFilterRules(const QString & rules);
impl<'a> /*trait*/ QLoggingCategory_setFilterRules_s<()> for (&'a QString) {
  fn setFilterRules_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QLoggingCategory14setFilterRulesERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QLoggingCategory14setFilterRulesERK7QString(arg0)};
    // return 1;
  }
}

// <= body block end

