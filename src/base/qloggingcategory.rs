// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN16QLoggingCategoryC1EPKc9QtMsgType(qthis: *mut c_void, arg0: *const c_char, arg1: c_int) -> i32;
  fn _ZN16QLoggingCategoryC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK16QLoggingCategory14isDebugEnabledEv() -> i32;
  fn _ZN16QLoggingCategoryD0Ev() -> i32;
  fn _ZN16QLoggingCategoryC1EPKc(qthis: *mut c_void, arg0: *const c_char) -> i32;
  fn _ZN16QLoggingCategory10setEnabledE9QtMsgTypeb(arg0: c_int, arg1: int8_t) -> i32;
  fn _ZNK16QLoggingCategory9isEnabledE9QtMsgType(arg0: c_int) -> i32;
  fn _ZNK16QLoggingCategory16isWarningEnabledEv() -> i32;
  fn _ZNK16QLoggingCategory13isInfoEnabledEv() -> i32;
  fn _ZNK16QLoggingCategory12categoryNameEv() -> i32;
  fn _ZNK16QLoggingCategory17isCriticalEnabledEv() -> i32;
  fn _ZN16QLoggingCategory15defaultCategoryEv() -> i32;
  fn _ZN16QLoggingCategory14setFilterRulesERK7QString(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QLoggingCategory)=24
pub struct QLoggingCategory {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QLoggingCategory {
  pub fn NewQLoggingCategory<T: QLoggingCategory_NewQLoggingCategory>(value: T) -> QLoggingCategory {
    let rsthis = value.NewQLoggingCategory();
    return rsthis;
    // return 1;
  }
}

pub trait QLoggingCategory_NewQLoggingCategory {
  fn NewQLoggingCategory(self) -> QLoggingCategory;
}

// proto: void QLoggingCategory::NewQLoggingCategory(const char * category, QtMsgType severityLevel);
impl<'a> /*trait*/ QLoggingCategory_NewQLoggingCategory for (&'a  String, i32) {
  fn NewQLoggingCategory(self) -> QLoggingCategory {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QLoggingCategoryC1EPKc9QtMsgType()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_int;
    unsafe {_ZN16QLoggingCategoryC1EPKc9QtMsgType(qthis, arg0, arg1)};
    let rsthis = QLoggingCategory{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QLoggingCategory::NewQLoggingCategory(const QLoggingCategory & );
impl<'a> /*trait*/ QLoggingCategory_NewQLoggingCategory for (&'a  QLoggingCategory) {
  fn NewQLoggingCategory(self) -> QLoggingCategory {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QLoggingCategoryC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QLoggingCategoryC1ERKS_(qthis, arg0)};
    let rsthis = QLoggingCategory{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLoggingCategory {
  pub fn isDebugEnabled<T: QLoggingCategory_isDebugEnabled>(&mut self, value: T) -> i32 {
    value.isDebugEnabled(self);
    return 1;
  }
}

pub trait QLoggingCategory_isDebugEnabled {
  fn isDebugEnabled(self, this: &mut QLoggingCategory) -> i32;
}

// proto: bool QLoggingCategory::isDebugEnabled();
impl<'a> /*trait*/ QLoggingCategory_isDebugEnabled for () {
  fn isDebugEnabled(self, this: &mut QLoggingCategory) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QLoggingCategory14isDebugEnabledEv()};
    unsafe {_ZNK16QLoggingCategory14isDebugEnabledEv()};
    return 1;
  }
}

impl /*struct*/ QLoggingCategory {
  pub fn FreeQLoggingCategory<T: QLoggingCategory_FreeQLoggingCategory>(&mut self, value: T) -> i32 {
    value.FreeQLoggingCategory(self);
    return 1;
  }
}

pub trait QLoggingCategory_FreeQLoggingCategory {
  fn FreeQLoggingCategory(self, this: &mut QLoggingCategory) -> i32;
}

// proto: void QLoggingCategory::FreeQLoggingCategory();
impl<'a> /*trait*/ QLoggingCategory_FreeQLoggingCategory for () {
  fn FreeQLoggingCategory(self, this: &mut QLoggingCategory) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QLoggingCategoryD0Ev()};
    unsafe {_ZN16QLoggingCategoryD0Ev()};
    return 1;
  }
}

// proto: void QLoggingCategory::NewQLoggingCategory(const char * category);
impl<'a> /*trait*/ QLoggingCategory_NewQLoggingCategory for (&'a  String) {
  fn NewQLoggingCategory(self) -> QLoggingCategory {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QLoggingCategoryC1EPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    unsafe {_ZN16QLoggingCategoryC1EPKc(qthis, arg0)};
    let rsthis = QLoggingCategory{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLoggingCategory {
  pub fn setEnabled<T: QLoggingCategory_setEnabled>(&mut self, value: T) -> i32 {
    value.setEnabled(self);
    return 1;
  }
}

pub trait QLoggingCategory_setEnabled {
  fn setEnabled(self, this: &mut QLoggingCategory) -> i32;
}

// proto: void QLoggingCategory::setEnabled(QtMsgType type, bool enable);
impl<'a> /*trait*/ QLoggingCategory_setEnabled for (i32, i8) {
  fn setEnabled(self, this: &mut QLoggingCategory) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QLoggingCategory10setEnabledE9QtMsgTypeb()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as int8_t;
    unsafe {_ZN16QLoggingCategory10setEnabledE9QtMsgTypeb(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QLoggingCategory {
  pub fn isEnabled<T: QLoggingCategory_isEnabled>(&mut self, value: T) -> i32 {
    value.isEnabled(self);
    return 1;
  }
}

pub trait QLoggingCategory_isEnabled {
  fn isEnabled(self, this: &mut QLoggingCategory) -> i32;
}

// proto: bool QLoggingCategory::isEnabled(QtMsgType type);
impl<'a> /*trait*/ QLoggingCategory_isEnabled for (i32) {
  fn isEnabled(self, this: &mut QLoggingCategory) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QLoggingCategory9isEnabledE9QtMsgType()};
    let arg0 = self  as c_int;
    unsafe {_ZNK16QLoggingCategory9isEnabledE9QtMsgType(arg0)};
    return 1;
  }
}

impl /*struct*/ QLoggingCategory {
  pub fn isWarningEnabled<T: QLoggingCategory_isWarningEnabled>(&mut self, value: T) -> i32 {
    value.isWarningEnabled(self);
    return 1;
  }
}

pub trait QLoggingCategory_isWarningEnabled {
  fn isWarningEnabled(self, this: &mut QLoggingCategory) -> i32;
}

// proto: bool QLoggingCategory::isWarningEnabled();
impl<'a> /*trait*/ QLoggingCategory_isWarningEnabled for () {
  fn isWarningEnabled(self, this: &mut QLoggingCategory) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QLoggingCategory16isWarningEnabledEv()};
    unsafe {_ZNK16QLoggingCategory16isWarningEnabledEv()};
    return 1;
  }
}

impl /*struct*/ QLoggingCategory {
  pub fn isInfoEnabled<T: QLoggingCategory_isInfoEnabled>(&mut self, value: T) -> i32 {
    value.isInfoEnabled(self);
    return 1;
  }
}

pub trait QLoggingCategory_isInfoEnabled {
  fn isInfoEnabled(self, this: &mut QLoggingCategory) -> i32;
}

// proto: bool QLoggingCategory::isInfoEnabled();
impl<'a> /*trait*/ QLoggingCategory_isInfoEnabled for () {
  fn isInfoEnabled(self, this: &mut QLoggingCategory) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QLoggingCategory13isInfoEnabledEv()};
    unsafe {_ZNK16QLoggingCategory13isInfoEnabledEv()};
    return 1;
  }
}

impl /*struct*/ QLoggingCategory {
  pub fn categoryName<T: QLoggingCategory_categoryName>(&mut self, value: T) -> i32 {
    value.categoryName(self);
    return 1;
  }
}

pub trait QLoggingCategory_categoryName {
  fn categoryName(self, this: &mut QLoggingCategory) -> i32;
}

// proto: const char * QLoggingCategory::categoryName();
impl<'a> /*trait*/ QLoggingCategory_categoryName for () {
  fn categoryName(self, this: &mut QLoggingCategory) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QLoggingCategory12categoryNameEv()};
    unsafe {_ZNK16QLoggingCategory12categoryNameEv()};
    return 1;
  }
}

impl /*struct*/ QLoggingCategory {
  pub fn isCriticalEnabled<T: QLoggingCategory_isCriticalEnabled>(&mut self, value: T) -> i32 {
    value.isCriticalEnabled(self);
    return 1;
  }
}

pub trait QLoggingCategory_isCriticalEnabled {
  fn isCriticalEnabled(self, this: &mut QLoggingCategory) -> i32;
}

// proto: bool QLoggingCategory::isCriticalEnabled();
impl<'a> /*trait*/ QLoggingCategory_isCriticalEnabled for () {
  fn isCriticalEnabled(self, this: &mut QLoggingCategory) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QLoggingCategory17isCriticalEnabledEv()};
    unsafe {_ZNK16QLoggingCategory17isCriticalEnabledEv()};
    return 1;
  }
}

impl /*struct*/ QLoggingCategory {
  pub fn defaultCategory<T: QLoggingCategory_defaultCategory>(&mut self, value: T) -> i32 {
    value.defaultCategory(self);
    return 1;
  }
}

pub trait QLoggingCategory_defaultCategory {
  fn defaultCategory(self, this: &mut QLoggingCategory) -> i32;
}

// proto: QLoggingCategory * QLoggingCategory::defaultCategory();
impl<'a> /*trait*/ QLoggingCategory_defaultCategory for () {
  fn defaultCategory(self, this: &mut QLoggingCategory) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QLoggingCategory15defaultCategoryEv()};
    unsafe {_ZN16QLoggingCategory15defaultCategoryEv()};
    return 1;
  }
}

impl /*struct*/ QLoggingCategory {
  pub fn setFilterRules<T: QLoggingCategory_setFilterRules>(&mut self, value: T) -> i32 {
    value.setFilterRules(self);
    return 1;
  }
}

pub trait QLoggingCategory_setFilterRules {
  fn setFilterRules(self, this: &mut QLoggingCategory) -> i32;
}

// proto: void QLoggingCategory::setFilterRules(const QString & rules);
impl<'a> /*trait*/ QLoggingCategory_setFilterRules for (&'a  QString) {
  fn setFilterRules(self, this: &mut QLoggingCategory) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QLoggingCategory14setFilterRulesERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QLoggingCategory14setFilterRulesERK7QString(arg0)};
    return 1;
  }
}

