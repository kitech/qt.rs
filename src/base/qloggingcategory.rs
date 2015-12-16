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
  // proto:  void QLoggingCategory::NewQLoggingCategory(const char * category, QtMsgType severityLevel);
  fn _ZN16QLoggingCategoryC1EPKc9QtMsgType(qthis: *mut c_void, arg0: *const c_char, arg1: c_int) ;
  // proto:  void QLoggingCategory::NewQLoggingCategory(const QLoggingCategory & );
  fn _ZN16QLoggingCategoryC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QLoggingCategory::isDebugEnabled();
  fn _ZNK16QLoggingCategory14isDebugEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QLoggingCategory::FreeQLoggingCategory();
  fn _ZN16QLoggingCategoryD0Ev(qthis: *mut c_void) ;
  // proto:  void QLoggingCategory::NewQLoggingCategory(const char * category);
  fn _ZN16QLoggingCategoryC1EPKc(qthis: *mut c_void, arg0: *const c_char) ;
  // proto:  void QLoggingCategory::setEnabled(QtMsgType type, bool enable);
  fn _ZN16QLoggingCategory10setEnabledE9QtMsgTypeb(qthis: *mut c_void, arg0: c_int, arg1: int8_t) ;
  // proto:  bool QLoggingCategory::isEnabled(QtMsgType type);
  fn _ZNK16QLoggingCategory9isEnabledE9QtMsgType(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  bool QLoggingCategory::isWarningEnabled();
  fn _ZNK16QLoggingCategory16isWarningEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QLoggingCategory::isInfoEnabled();
  fn _ZNK16QLoggingCategory13isInfoEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  const char * QLoggingCategory::categoryName();
  fn _ZNK16QLoggingCategory12categoryNameEv(qthis: *mut c_void) -> *const c_char;
  // proto:  bool QLoggingCategory::isCriticalEnabled();
  fn _ZNK16QLoggingCategory17isCriticalEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto: static QLoggingCategory * QLoggingCategory::defaultCategory();
  fn _ZN16QLoggingCategory15defaultCategoryEv() -> *mut c_void;
  // proto: static void QLoggingCategory::setFilterRules(const QString & rules);
  fn _ZN16QLoggingCategory14setFilterRulesERK7QString(arg0: *mut c_void) ;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QLoggingCategoryC1ERKS_(qthis, arg0)};
    let rsthis = QLoggingCategory{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLoggingCategory {
  pub fn isDebugEnabled<T: QLoggingCategory_isDebugEnabled>(&mut self, value: T) -> i8 {
    return value.isDebugEnabled(self);
    // return 1;
  }
}

pub trait QLoggingCategory_isDebugEnabled {
  fn isDebugEnabled(self, rsthis: &mut QLoggingCategory) -> i8;
}

// proto:  bool QLoggingCategory::isDebugEnabled();
impl<'a> /*trait*/ QLoggingCategory_isDebugEnabled for () {
  fn isDebugEnabled(self, rsthis: &mut QLoggingCategory) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QLoggingCategory14isDebugEnabledEv()};
    let mut ret = unsafe {_ZNK16QLoggingCategory14isDebugEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QLoggingCategory {
  pub fn FreeQLoggingCategory<T: QLoggingCategory_FreeQLoggingCategory>(&mut self, value: T)  {
     value.FreeQLoggingCategory(self);
    // return 1;
  }
}

pub trait QLoggingCategory_FreeQLoggingCategory {
  fn FreeQLoggingCategory(self, rsthis: &mut QLoggingCategory) ;
}

// proto:  void QLoggingCategory::FreeQLoggingCategory();
impl<'a> /*trait*/ QLoggingCategory_FreeQLoggingCategory for () {
  fn FreeQLoggingCategory(self, rsthis: &mut QLoggingCategory)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QLoggingCategoryD0Ev()};
     unsafe {_ZN16QLoggingCategoryD0Ev(rsthis.qclsinst)};
    // return 1;
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
  pub fn setEnabled<T: QLoggingCategory_setEnabled>(&mut self, value: T)  {
     value.setEnabled(self);
    // return 1;
  }
}

pub trait QLoggingCategory_setEnabled {
  fn setEnabled(self, rsthis: &mut QLoggingCategory) ;
}

// proto:  void QLoggingCategory::setEnabled(QtMsgType type, bool enable);
impl<'a> /*trait*/ QLoggingCategory_setEnabled for (i32, i8) {
  fn setEnabled(self, rsthis: &mut QLoggingCategory)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QLoggingCategory10setEnabledE9QtMsgTypeb()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as int8_t;
     unsafe {_ZN16QLoggingCategory10setEnabledE9QtMsgTypeb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QLoggingCategory {
  pub fn isEnabled<T: QLoggingCategory_isEnabled>(&mut self, value: T) -> i8 {
    return value.isEnabled(self);
    // return 1;
  }
}

pub trait QLoggingCategory_isEnabled {
  fn isEnabled(self, rsthis: &mut QLoggingCategory) -> i8;
}

// proto:  bool QLoggingCategory::isEnabled(QtMsgType type);
impl<'a> /*trait*/ QLoggingCategory_isEnabled for (i32) {
  fn isEnabled(self, rsthis: &mut QLoggingCategory) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QLoggingCategory9isEnabledE9QtMsgType()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK16QLoggingCategory9isEnabledE9QtMsgType(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QLoggingCategory {
  pub fn isWarningEnabled<T: QLoggingCategory_isWarningEnabled>(&mut self, value: T) -> i8 {
    return value.isWarningEnabled(self);
    // return 1;
  }
}

pub trait QLoggingCategory_isWarningEnabled {
  fn isWarningEnabled(self, rsthis: &mut QLoggingCategory) -> i8;
}

// proto:  bool QLoggingCategory::isWarningEnabled();
impl<'a> /*trait*/ QLoggingCategory_isWarningEnabled for () {
  fn isWarningEnabled(self, rsthis: &mut QLoggingCategory) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QLoggingCategory16isWarningEnabledEv()};
    let mut ret = unsafe {_ZNK16QLoggingCategory16isWarningEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QLoggingCategory {
  pub fn isInfoEnabled<T: QLoggingCategory_isInfoEnabled>(&mut self, value: T) -> i8 {
    return value.isInfoEnabled(self);
    // return 1;
  }
}

pub trait QLoggingCategory_isInfoEnabled {
  fn isInfoEnabled(self, rsthis: &mut QLoggingCategory) -> i8;
}

// proto:  bool QLoggingCategory::isInfoEnabled();
impl<'a> /*trait*/ QLoggingCategory_isInfoEnabled for () {
  fn isInfoEnabled(self, rsthis: &mut QLoggingCategory) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QLoggingCategory13isInfoEnabledEv()};
    let mut ret = unsafe {_ZNK16QLoggingCategory13isInfoEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QLoggingCategory {
  pub fn categoryName<T: QLoggingCategory_categoryName>(&mut self, value: T) -> String {
    return value.categoryName(self);
    // return 1;
  }
}

pub trait QLoggingCategory_categoryName {
  fn categoryName(self, rsthis: &mut QLoggingCategory) -> String;
}

// proto:  const char * QLoggingCategory::categoryName();
impl<'a> /*trait*/ QLoggingCategory_categoryName for () {
  fn categoryName(self, rsthis: &mut QLoggingCategory) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QLoggingCategory12categoryNameEv()};
    let mut ret = unsafe {_ZNK16QLoggingCategory12categoryNameEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

impl /*struct*/ QLoggingCategory {
  pub fn isCriticalEnabled<T: QLoggingCategory_isCriticalEnabled>(&mut self, value: T) -> i8 {
    return value.isCriticalEnabled(self);
    // return 1;
  }
}

pub trait QLoggingCategory_isCriticalEnabled {
  fn isCriticalEnabled(self, rsthis: &mut QLoggingCategory) -> i8;
}

// proto:  bool QLoggingCategory::isCriticalEnabled();
impl<'a> /*trait*/ QLoggingCategory_isCriticalEnabled for () {
  fn isCriticalEnabled(self, rsthis: &mut QLoggingCategory) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QLoggingCategory17isCriticalEnabledEv()};
    let mut ret = unsafe {_ZNK16QLoggingCategory17isCriticalEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QLoggingCategory {
  pub fn defaultCategory<T: QLoggingCategory_defaultCategory>(&mut self, value: T) -> QLoggingCategory {
    return value.defaultCategory(self);
    // return 1;
  }
}

pub trait QLoggingCategory_defaultCategory {
  fn defaultCategory(self, rsthis: &mut QLoggingCategory) -> QLoggingCategory;
}

// proto: static QLoggingCategory * QLoggingCategory::defaultCategory();
impl<'a> /*trait*/ QLoggingCategory_defaultCategory for () {
  fn defaultCategory(self, rsthis: &mut QLoggingCategory) -> QLoggingCategory {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QLoggingCategory15defaultCategoryEv()};
    let mut ret = unsafe {_ZN16QLoggingCategory15defaultCategoryEv()};
    let mut ret1 = QLoggingCategory{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLoggingCategory {
  pub fn setFilterRules<T: QLoggingCategory_setFilterRules>(&mut self, value: T)  {
     value.setFilterRules(self);
    // return 1;
  }
}

pub trait QLoggingCategory_setFilterRules {
  fn setFilterRules(self, rsthis: &mut QLoggingCategory) ;
}

// proto: static void QLoggingCategory::setFilterRules(const QString & rules);
impl<'a> /*trait*/ QLoggingCategory_setFilterRules for (&'a  QString) {
  fn setFilterRules(self, rsthis: &mut QLoggingCategory)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QLoggingCategory14setFilterRulesERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QLoggingCategory14setFilterRulesERK7QString(arg0)};
    // return 1;
  }
}

