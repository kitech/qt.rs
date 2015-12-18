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
  pub fn isDebugEnabled<RetType, T: QLoggingCategory_isDebugEnabled<RetType>>(&mut self, value: T) -> RetType {
    return value.isDebugEnabled(self);
    // return 1;
  }
}

pub trait QLoggingCategory_isDebugEnabled<RetType> {
  fn isDebugEnabled(self, rsthis: &mut QLoggingCategory) -> RetType;
}

// proto:  bool QLoggingCategory::isDebugEnabled();
impl<'a> /*trait*/ QLoggingCategory_isDebugEnabled<i8> for () {
  fn isDebugEnabled(self, rsthis: &mut QLoggingCategory) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QLoggingCategory14isDebugEnabledEv()};
    let mut ret = unsafe {_ZNK16QLoggingCategory14isDebugEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QLoggingCategory {
  pub fn FreeQLoggingCategory<RetType, T: QLoggingCategory_FreeQLoggingCategory<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQLoggingCategory(self);
    // return 1;
  }
}

pub trait QLoggingCategory_FreeQLoggingCategory<RetType> {
  fn FreeQLoggingCategory(self, rsthis: &mut QLoggingCategory) -> RetType;
}

// proto:  void QLoggingCategory::FreeQLoggingCategory();
impl<'a> /*trait*/ QLoggingCategory_FreeQLoggingCategory<()> for () {
  fn FreeQLoggingCategory(self, rsthis: &mut QLoggingCategory) -> () {
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
  pub fn setEnabled<RetType, T: QLoggingCategory_setEnabled<RetType>>(&mut self, value: T) -> RetType {
    return value.setEnabled(self);
    // return 1;
  }
}

pub trait QLoggingCategory_setEnabled<RetType> {
  fn setEnabled(self, rsthis: &mut QLoggingCategory) -> RetType;
}

// proto:  void QLoggingCategory::setEnabled(QtMsgType type, bool enable);
impl<'a> /*trait*/ QLoggingCategory_setEnabled<()> for (i32, i8) {
  fn setEnabled(self, rsthis: &mut QLoggingCategory) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QLoggingCategory10setEnabledE9QtMsgTypeb()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as int8_t;
     unsafe {_ZN16QLoggingCategory10setEnabledE9QtMsgTypeb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QLoggingCategory {
  pub fn isEnabled<RetType, T: QLoggingCategory_isEnabled<RetType>>(&mut self, value: T) -> RetType {
    return value.isEnabled(self);
    // return 1;
  }
}

pub trait QLoggingCategory_isEnabled<RetType> {
  fn isEnabled(self, rsthis: &mut QLoggingCategory) -> RetType;
}

// proto:  bool QLoggingCategory::isEnabled(QtMsgType type);
impl<'a> /*trait*/ QLoggingCategory_isEnabled<i8> for (i32) {
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
  pub fn isWarningEnabled<RetType, T: QLoggingCategory_isWarningEnabled<RetType>>(&mut self, value: T) -> RetType {
    return value.isWarningEnabled(self);
    // return 1;
  }
}

pub trait QLoggingCategory_isWarningEnabled<RetType> {
  fn isWarningEnabled(self, rsthis: &mut QLoggingCategory) -> RetType;
}

// proto:  bool QLoggingCategory::isWarningEnabled();
impl<'a> /*trait*/ QLoggingCategory_isWarningEnabled<i8> for () {
  fn isWarningEnabled(self, rsthis: &mut QLoggingCategory) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QLoggingCategory16isWarningEnabledEv()};
    let mut ret = unsafe {_ZNK16QLoggingCategory16isWarningEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QLoggingCategory {
  pub fn isInfoEnabled<RetType, T: QLoggingCategory_isInfoEnabled<RetType>>(&mut self, value: T) -> RetType {
    return value.isInfoEnabled(self);
    // return 1;
  }
}

pub trait QLoggingCategory_isInfoEnabled<RetType> {
  fn isInfoEnabled(self, rsthis: &mut QLoggingCategory) -> RetType;
}

// proto:  bool QLoggingCategory::isInfoEnabled();
impl<'a> /*trait*/ QLoggingCategory_isInfoEnabled<i8> for () {
  fn isInfoEnabled(self, rsthis: &mut QLoggingCategory) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QLoggingCategory13isInfoEnabledEv()};
    let mut ret = unsafe {_ZNK16QLoggingCategory13isInfoEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QLoggingCategory {
  pub fn categoryName<RetType, T: QLoggingCategory_categoryName<RetType>>(&mut self, value: T) -> RetType {
    return value.categoryName(self);
    // return 1;
  }
}

pub trait QLoggingCategory_categoryName<RetType> {
  fn categoryName(self, rsthis: &mut QLoggingCategory) -> RetType;
}

// proto:  const char * QLoggingCategory::categoryName();
impl<'a> /*trait*/ QLoggingCategory_categoryName<String> for () {
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
  pub fn isCriticalEnabled<RetType, T: QLoggingCategory_isCriticalEnabled<RetType>>(&mut self, value: T) -> RetType {
    return value.isCriticalEnabled(self);
    // return 1;
  }
}

pub trait QLoggingCategory_isCriticalEnabled<RetType> {
  fn isCriticalEnabled(self, rsthis: &mut QLoggingCategory) -> RetType;
}

// proto:  bool QLoggingCategory::isCriticalEnabled();
impl<'a> /*trait*/ QLoggingCategory_isCriticalEnabled<i8> for () {
  fn isCriticalEnabled(self, rsthis: &mut QLoggingCategory) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QLoggingCategory17isCriticalEnabledEv()};
    let mut ret = unsafe {_ZNK16QLoggingCategory17isCriticalEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QLoggingCategory {
  pub fn defaultCategory<RetType, T: QLoggingCategory_defaultCategory<RetType>>(&mut self, value: T) -> RetType {
    return value.defaultCategory(self);
    // return 1;
  }
}

pub trait QLoggingCategory_defaultCategory<RetType> {
  fn defaultCategory(self, rsthis: &mut QLoggingCategory) -> RetType;
}

// proto: static QLoggingCategory * QLoggingCategory::defaultCategory();
impl<'a> /*trait*/ QLoggingCategory_defaultCategory<QLoggingCategory> for () {
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
  pub fn setFilterRules<RetType, T: QLoggingCategory_setFilterRules<RetType>>(&mut self, value: T) -> RetType {
    return value.setFilterRules(self);
    // return 1;
  }
}

pub trait QLoggingCategory_setFilterRules<RetType> {
  fn setFilterRules(self, rsthis: &mut QLoggingCategory) -> RetType;
}

// proto: static void QLoggingCategory::setFilterRules(const QString & rules);
impl<'a> /*trait*/ QLoggingCategory_setFilterRules<()> for (&'a  QString) {
  fn setFilterRules(self, rsthis: &mut QLoggingCategory) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QLoggingCategory14setFilterRulesERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QLoggingCategory14setFilterRulesERK7QString(arg0)};
    // return 1;
  }
}

