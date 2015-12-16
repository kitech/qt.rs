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
  // proto:  int QRegularExpression::patternErrorOffset();
  fn _ZNK18QRegularExpression18patternErrorOffsetEv(qthis: *mut c_void) -> c_int;
  // proto:  QString QRegularExpression::pattern();
  fn _ZNK18QRegularExpression7patternEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRegularExpression::FreeQRegularExpression();
  fn _ZN18QRegularExpressionD0Ev(qthis: *mut c_void) ;
  // proto:  void QRegularExpression::optimize();
  fn _ZNK18QRegularExpression8optimizeEv(qthis: *mut c_void) ;
  // proto: static QString QRegularExpression::escape(const QString & str);
  fn _ZN18QRegularExpression6escapeERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto:  void QRegularExpression::NewQRegularExpression();
  fn _ZN18QRegularExpressionC1Ev(qthis: *mut c_void) ;
  // proto:  void QRegularExpression::swap(QRegularExpression & other);
  fn _ZN18QRegularExpression4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QRegularExpression::errorString();
  fn _ZNK18QRegularExpression11errorStringEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QRegularExpression::isValid();
  fn _ZNK18QRegularExpression7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QRegularExpression::NewQRegularExpression(const QRegularExpression & re);
  fn _ZN18QRegularExpressionC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QStringList QRegularExpression::namedCaptureGroups();
  fn _ZNK18QRegularExpression18namedCaptureGroupsEv(qthis: *mut c_void) ;
  // proto:  int QRegularExpression::captureCount();
  fn _ZNK18QRegularExpression12captureCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QRegularExpression::setPattern(const QString & pattern);
  fn _ZN18QRegularExpression10setPatternERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QRegularExpression)=1
pub struct QRegularExpression {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QRegularExpression {
  pub fn patternErrorOffset<T: QRegularExpression_patternErrorOffset>(&mut self, value: T) -> i32 {
    return value.patternErrorOffset(self);
    // return 1;
  }
}

pub trait QRegularExpression_patternErrorOffset {
  fn patternErrorOffset(self, rsthis: &mut QRegularExpression) -> i32;
}

// proto:  int QRegularExpression::patternErrorOffset();
impl<'a> /*trait*/ QRegularExpression_patternErrorOffset for () {
  fn patternErrorOffset(self, rsthis: &mut QRegularExpression) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QRegularExpression18patternErrorOffsetEv()};
    let mut ret = unsafe {_ZNK18QRegularExpression18patternErrorOffsetEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QRegularExpression {
  pub fn pattern<T: QRegularExpression_pattern>(&mut self, value: T) -> QString {
    return value.pattern(self);
    // return 1;
  }
}

pub trait QRegularExpression_pattern {
  fn pattern(self, rsthis: &mut QRegularExpression) -> QString;
}

// proto:  QString QRegularExpression::pattern();
impl<'a> /*trait*/ QRegularExpression_pattern for () {
  fn pattern(self, rsthis: &mut QRegularExpression) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QRegularExpression7patternEv()};
    let mut ret = unsafe {_ZNK18QRegularExpression7patternEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QRegularExpression {
  pub fn FreeQRegularExpression<T: QRegularExpression_FreeQRegularExpression>(&mut self, value: T)  {
     value.FreeQRegularExpression(self);
    // return 1;
  }
}

pub trait QRegularExpression_FreeQRegularExpression {
  fn FreeQRegularExpression(self, rsthis: &mut QRegularExpression) ;
}

// proto:  void QRegularExpression::FreeQRegularExpression();
impl<'a> /*trait*/ QRegularExpression_FreeQRegularExpression for () {
  fn FreeQRegularExpression(self, rsthis: &mut QRegularExpression)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QRegularExpressionD0Ev()};
     unsafe {_ZN18QRegularExpressionD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QRegularExpression {
  pub fn optimize<T: QRegularExpression_optimize>(&mut self, value: T)  {
     value.optimize(self);
    // return 1;
  }
}

pub trait QRegularExpression_optimize {
  fn optimize(self, rsthis: &mut QRegularExpression) ;
}

// proto:  void QRegularExpression::optimize();
impl<'a> /*trait*/ QRegularExpression_optimize for () {
  fn optimize(self, rsthis: &mut QRegularExpression)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QRegularExpression8optimizeEv()};
     unsafe {_ZNK18QRegularExpression8optimizeEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QRegularExpression {
  pub fn escape<T: QRegularExpression_escape>(&mut self, value: T) -> QString {
    return value.escape(self);
    // return 1;
  }
}

pub trait QRegularExpression_escape {
  fn escape(self, rsthis: &mut QRegularExpression) -> QString;
}

// proto: static QString QRegularExpression::escape(const QString & str);
impl<'a> /*trait*/ QRegularExpression_escape for (&'a  QString) {
  fn escape(self, rsthis: &mut QRegularExpression) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QRegularExpression6escapeERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN18QRegularExpression6escapeERK7QString(arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
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
  pub fn swap<T: QRegularExpression_swap>(&mut self, value: T)  {
     value.swap(self);
    // return 1;
  }
}

pub trait QRegularExpression_swap {
  fn swap(self, rsthis: &mut QRegularExpression) ;
}

// proto:  void QRegularExpression::swap(QRegularExpression & other);
impl<'a> /*trait*/ QRegularExpression_swap for (&'a mut QRegularExpression) {
  fn swap(self, rsthis: &mut QRegularExpression)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QRegularExpression4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QRegularExpression4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRegularExpression {
  pub fn errorString<T: QRegularExpression_errorString>(&mut self, value: T) -> QString {
    return value.errorString(self);
    // return 1;
  }
}

pub trait QRegularExpression_errorString {
  fn errorString(self, rsthis: &mut QRegularExpression) -> QString;
}

// proto:  QString QRegularExpression::errorString();
impl<'a> /*trait*/ QRegularExpression_errorString for () {
  fn errorString(self, rsthis: &mut QRegularExpression) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QRegularExpression11errorStringEv()};
    let mut ret = unsafe {_ZNK18QRegularExpression11errorStringEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QRegularExpression {
  pub fn isValid<T: QRegularExpression_isValid>(&mut self, value: T) -> i8 {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QRegularExpression_isValid {
  fn isValid(self, rsthis: &mut QRegularExpression) -> i8;
}

// proto:  bool QRegularExpression::isValid();
impl<'a> /*trait*/ QRegularExpression_isValid for () {
  fn isValid(self, rsthis: &mut QRegularExpression) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QRegularExpression7isValidEv()};
    let mut ret = unsafe {_ZNK18QRegularExpression7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QRegularExpression::NewQRegularExpression(const QRegularExpression & re);
impl<'a> /*trait*/ QRegularExpression_NewQRegularExpression for (&'a  QRegularExpression) {
  fn NewQRegularExpression(self) -> QRegularExpression {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QRegularExpressionC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QRegularExpressionC1ERKS_(qthis, arg0)};
    let rsthis = QRegularExpression{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QRegularExpression {
  pub fn namedCaptureGroups<T: QRegularExpression_namedCaptureGroups>(&mut self, value: T)  {
     value.namedCaptureGroups(self);
    // return 1;
  }
}

pub trait QRegularExpression_namedCaptureGroups {
  fn namedCaptureGroups(self, rsthis: &mut QRegularExpression) ;
}

// proto:  QStringList QRegularExpression::namedCaptureGroups();
impl<'a> /*trait*/ QRegularExpression_namedCaptureGroups for () {
  fn namedCaptureGroups(self, rsthis: &mut QRegularExpression)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QRegularExpression18namedCaptureGroupsEv()};
     unsafe {_ZNK18QRegularExpression18namedCaptureGroupsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QRegularExpression {
  pub fn captureCount<T: QRegularExpression_captureCount>(&mut self, value: T) -> i32 {
    return value.captureCount(self);
    // return 1;
  }
}

pub trait QRegularExpression_captureCount {
  fn captureCount(self, rsthis: &mut QRegularExpression) -> i32;
}

// proto:  int QRegularExpression::captureCount();
impl<'a> /*trait*/ QRegularExpression_captureCount for () {
  fn captureCount(self, rsthis: &mut QRegularExpression) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QRegularExpression12captureCountEv()};
    let mut ret = unsafe {_ZNK18QRegularExpression12captureCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QRegularExpression {
  pub fn setPattern<T: QRegularExpression_setPattern>(&mut self, value: T)  {
     value.setPattern(self);
    // return 1;
  }
}

pub trait QRegularExpression_setPattern {
  fn setPattern(self, rsthis: &mut QRegularExpression) ;
}

// proto:  void QRegularExpression::setPattern(const QString & pattern);
impl<'a> /*trait*/ QRegularExpression_setPattern for (&'a  QString) {
  fn setPattern(self, rsthis: &mut QRegularExpression)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QRegularExpression10setPatternERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QRegularExpression10setPatternERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

