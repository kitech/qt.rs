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
  fn _ZNK18QRegularExpression18patternErrorOffsetEv() -> i32;
  fn _ZNK18QRegularExpression7patternEv() -> i32;
  fn _ZN18QRegularExpressionD0Ev() -> i32;
  fn _ZNK18QRegularExpression8optimizeEv() -> i32;
  fn _ZN18QRegularExpression6escapeERK7QString(arg0: *const c_void) -> i32;
  fn _ZN18QRegularExpressionC1Ev(qthis: *mut c_void) -> i32;
  fn _ZN18QRegularExpression4swapERS_(arg0: *mut c_void) -> i32;
  fn _ZN18QRegularExpressionC1ERK7QString6QFlagsINS_13PatternOptionEE(qthis: *mut c_void, arg0: *const c_void, arg1: c_int) -> i32;
  fn _ZNK18QRegularExpression11errorStringEv() -> i32;
  fn _ZNK18QRegularExpression7isValidEv() -> i32;
  fn _ZN18QRegularExpressionC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN18QRegularExpression17setPatternOptionsE6QFlagsINS_13PatternOptionEE(arg0: c_int) -> i32;
  fn _ZNK18QRegularExpression18namedCaptureGroupsEv() -> i32;
  fn _ZNK18QRegularExpression12captureCountEv() -> i32;
  fn _ZN18QRegularExpression10setPatternERK7QString(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QRegularExpression)=1
pub struct QRegularExpression {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QRegularExpression {
  pub fn patternErrorOffset<T: QRegularExpression_patternErrorOffset>(&mut self, value: T) -> i32 {
    value.patternErrorOffset(self);
    return 1;
  }
}

pub trait QRegularExpression_patternErrorOffset {
  fn patternErrorOffset(self, this: &mut QRegularExpression) -> i32;
}

// proto: int QRegularExpression::patternErrorOffset();
impl<'a> /*trait*/ QRegularExpression_patternErrorOffset for () {
  fn patternErrorOffset(self, this: &mut QRegularExpression) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QRegularExpression18patternErrorOffsetEv()};
    unsafe {_ZNK18QRegularExpression18patternErrorOffsetEv()};
    return 1;
  }
}

impl /*struct*/ QRegularExpression {
  pub fn pattern<T: QRegularExpression_pattern>(&mut self, value: T) -> i32 {
    value.pattern(self);
    return 1;
  }
}

pub trait QRegularExpression_pattern {
  fn pattern(self, this: &mut QRegularExpression) -> i32;
}

// proto: QString QRegularExpression::pattern();
impl<'a> /*trait*/ QRegularExpression_pattern for () {
  fn pattern(self, this: &mut QRegularExpression) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QRegularExpression7patternEv()};
    unsafe {_ZNK18QRegularExpression7patternEv()};
    return 1;
  }
}

impl /*struct*/ QRegularExpression {
  pub fn FreeQRegularExpression<T: QRegularExpression_FreeQRegularExpression>(&mut self, value: T) -> i32 {
    value.FreeQRegularExpression(self);
    return 1;
  }
}

pub trait QRegularExpression_FreeQRegularExpression {
  fn FreeQRegularExpression(self, this: &mut QRegularExpression) -> i32;
}

// proto: void QRegularExpression::FreeQRegularExpression();
impl<'a> /*trait*/ QRegularExpression_FreeQRegularExpression for () {
  fn FreeQRegularExpression(self, this: &mut QRegularExpression) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QRegularExpressionD0Ev()};
    unsafe {_ZN18QRegularExpressionD0Ev()};
    return 1;
  }
}

impl /*struct*/ QRegularExpression {
  pub fn optimize<T: QRegularExpression_optimize>(&mut self, value: T) -> i32 {
    value.optimize(self);
    return 1;
  }
}

pub trait QRegularExpression_optimize {
  fn optimize(self, this: &mut QRegularExpression) -> i32;
}

// proto: void QRegularExpression::optimize();
impl<'a> /*trait*/ QRegularExpression_optimize for () {
  fn optimize(self, this: &mut QRegularExpression) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QRegularExpression8optimizeEv()};
    unsafe {_ZNK18QRegularExpression8optimizeEv()};
    return 1;
  }
}

impl /*struct*/ QRegularExpression {
  pub fn escape<T: QRegularExpression_escape>(&mut self, value: T) -> i32 {
    value.escape(self);
    return 1;
  }
}

pub trait QRegularExpression_escape {
  fn escape(self, this: &mut QRegularExpression) -> i32;
}

// proto: QString QRegularExpression::escape(const QString & str);
impl<'a> /*trait*/ QRegularExpression_escape for (&'a  QString) {
  fn escape(self, this: &mut QRegularExpression) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QRegularExpression6escapeERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN18QRegularExpression6escapeERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QRegularExpression {
  pub fn NewQRegularExpression<T: QRegularExpression_NewQRegularExpression>(value: T) -> QRegularExpression {
    let rsthis = value.NewQRegularExpression();
    return rsthis;
    // return 1;
  }
}

pub trait QRegularExpression_NewQRegularExpression {
  fn NewQRegularExpression(self) -> QRegularExpression;
}

// proto: void QRegularExpression::NewQRegularExpression();
impl<'a> /*trait*/ QRegularExpression_NewQRegularExpression for () {
  fn NewQRegularExpression(self) -> QRegularExpression {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QRegularExpressionC1Ev()};
    unsafe {_ZN18QRegularExpressionC1Ev(qthis)};
    let rsthis = QRegularExpression{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QRegularExpression {
  pub fn swap<T: QRegularExpression_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QRegularExpression_swap {
  fn swap(self, this: &mut QRegularExpression) -> i32;
}

// proto: void QRegularExpression::swap(QRegularExpression & other);
impl<'a> /*trait*/ QRegularExpression_swap for (&'a mut QRegularExpression) {
  fn swap(self, this: &mut QRegularExpression) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QRegularExpression4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QRegularExpression4swapERS_(arg0)};
    return 1;
  }
}

// proto: void QRegularExpression::NewQRegularExpression(const QString & pattern, PatternOptions options);
impl<'a> /*trait*/ QRegularExpression_NewQRegularExpression for (&'a  QString, i32) {
  fn NewQRegularExpression(self) -> QRegularExpression {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QRegularExpressionC1ERK7QString6QFlagsINS_13PatternOptionEE()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN18QRegularExpressionC1ERK7QString6QFlagsINS_13PatternOptionEE(qthis, arg0, arg1)};
    let rsthis = QRegularExpression{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QRegularExpression {
  pub fn errorString<T: QRegularExpression_errorString>(&mut self, value: T) -> i32 {
    value.errorString(self);
    return 1;
  }
}

pub trait QRegularExpression_errorString {
  fn errorString(self, this: &mut QRegularExpression) -> i32;
}

// proto: QString QRegularExpression::errorString();
impl<'a> /*trait*/ QRegularExpression_errorString for () {
  fn errorString(self, this: &mut QRegularExpression) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QRegularExpression11errorStringEv()};
    unsafe {_ZNK18QRegularExpression11errorStringEv()};
    return 1;
  }
}

impl /*struct*/ QRegularExpression {
  pub fn isValid<T: QRegularExpression_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QRegularExpression_isValid {
  fn isValid(self, this: &mut QRegularExpression) -> i32;
}

// proto: bool QRegularExpression::isValid();
impl<'a> /*trait*/ QRegularExpression_isValid for () {
  fn isValid(self, this: &mut QRegularExpression) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QRegularExpression7isValidEv()};
    unsafe {_ZNK18QRegularExpression7isValidEv()};
    return 1;
  }
}

// proto: void QRegularExpression::NewQRegularExpression(const QRegularExpression & re);
impl<'a> /*trait*/ QRegularExpression_NewQRegularExpression for (&'a  QRegularExpression) {
  fn NewQRegularExpression(self) -> QRegularExpression {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QRegularExpressionC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN18QRegularExpressionC1ERKS_(qthis, arg0)};
    let rsthis = QRegularExpression{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QRegularExpression {
  pub fn setPatternOptions<T: QRegularExpression_setPatternOptions>(&mut self, value: T) -> i32 {
    value.setPatternOptions(self);
    return 1;
  }
}

pub trait QRegularExpression_setPatternOptions {
  fn setPatternOptions(self, this: &mut QRegularExpression) -> i32;
}

// proto: void QRegularExpression::setPatternOptions(PatternOptions options);
impl<'a> /*trait*/ QRegularExpression_setPatternOptions for (i32) {
  fn setPatternOptions(self, this: &mut QRegularExpression) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QRegularExpression17setPatternOptionsE6QFlagsINS_13PatternOptionEE()};
    let arg0 = self  as c_int;
    unsafe {_ZN18QRegularExpression17setPatternOptionsE6QFlagsINS_13PatternOptionEE(arg0)};
    return 1;
  }
}

impl /*struct*/ QRegularExpression {
  pub fn namedCaptureGroups<T: QRegularExpression_namedCaptureGroups>(&mut self, value: T) -> i32 {
    value.namedCaptureGroups(self);
    return 1;
  }
}

pub trait QRegularExpression_namedCaptureGroups {
  fn namedCaptureGroups(self, this: &mut QRegularExpression) -> i32;
}

// proto: QStringList QRegularExpression::namedCaptureGroups();
impl<'a> /*trait*/ QRegularExpression_namedCaptureGroups for () {
  fn namedCaptureGroups(self, this: &mut QRegularExpression) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QRegularExpression18namedCaptureGroupsEv()};
    unsafe {_ZNK18QRegularExpression18namedCaptureGroupsEv()};
    return 1;
  }
}

impl /*struct*/ QRegularExpression {
  pub fn captureCount<T: QRegularExpression_captureCount>(&mut self, value: T) -> i32 {
    value.captureCount(self);
    return 1;
  }
}

pub trait QRegularExpression_captureCount {
  fn captureCount(self, this: &mut QRegularExpression) -> i32;
}

// proto: int QRegularExpression::captureCount();
impl<'a> /*trait*/ QRegularExpression_captureCount for () {
  fn captureCount(self, this: &mut QRegularExpression) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QRegularExpression12captureCountEv()};
    unsafe {_ZNK18QRegularExpression12captureCountEv()};
    return 1;
  }
}

impl /*struct*/ QRegularExpression {
  pub fn setPattern<T: QRegularExpression_setPattern>(&mut self, value: T) -> i32 {
    value.setPattern(self);
    return 1;
  }
}

pub trait QRegularExpression_setPattern {
  fn setPattern(self, this: &mut QRegularExpression) -> i32;
}

// proto: void QRegularExpression::setPattern(const QString & pattern);
impl<'a> /*trait*/ QRegularExpression_setPattern for (&'a  QString) {
  fn setPattern(self, this: &mut QRegularExpression) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QRegularExpression10setPatternERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN18QRegularExpression10setPatternERK7QString(arg0)};
    return 1;
  }
}

