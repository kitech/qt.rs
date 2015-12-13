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
  fn _ZN7QRegExpC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN7QRegExp13capturedTextsEv() -> i32;
  fn _ZNK7QRegExp12captureCountEv() -> i32;
  fn _ZN7QRegExp6escapeERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK7QRegExp7isEmptyEv() -> i32;
  fn _ZNK7QRegExp9isMinimalEv() -> i32;
  fn _ZNK7QRegExp13matchedLengthEv() -> i32;
  fn _ZNK7QRegExp7patternEv() -> i32;
  fn _ZN7QRegExp10setPatternERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK7QRegExp7isValidEv() -> i32;
  fn _ZN7QRegExpD0Ev() -> i32;
  fn _ZNK7QRegExp10exactMatchERK7QString(arg0: *const c_void) -> i32;
  fn _ZN7QRegExp4swapERS_(arg0: *mut c_void) -> i32;
  fn _ZN7QRegExp3posEi(arg0: c_int) -> i32;
  fn _ZN7QRegExpC1Ev(qthis: *mut c_void) -> i32;
  fn _ZN7QRegExp3capEi(arg0: c_int) -> i32;
  fn _ZN7QRegExp11errorStringEv() -> i32;
  fn _ZN7QRegExp10setMinimalEb(arg0: int8_t) -> i32;
}

// body block begin
// class sizeof(QRegExp)=8
pub struct QRegExp {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QRegExp {
  pub fn NewQRegExp<T: QRegExp_NewQRegExp>(value: T) -> QRegExp {
    let rsthis = value.NewQRegExp();
    return rsthis;
    // return 1;
  }
}

pub trait QRegExp_NewQRegExp {
  fn NewQRegExp(self) -> QRegExp;
}

// proto: void QRegExp::NewQRegExp(const QRegExp & rx);
impl<'a> /*trait*/ QRegExp_NewQRegExp for (&'a  QRegExp) {
  fn NewQRegExp(self) -> QRegExp {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegExpC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QRegExpC1ERKS_(qthis, arg0)};
    let rsthis = QRegExp{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QRegExp {
  pub fn capturedTexts<T: QRegExp_capturedTexts>(&mut self, value: T) -> i32 {
    value.capturedTexts(self);
    return 1;
  }
}

pub trait QRegExp_capturedTexts {
  fn capturedTexts(self, this: &mut QRegExp) -> i32;
}

// proto: QStringList QRegExp::capturedTexts();
impl<'a> /*trait*/ QRegExp_capturedTexts for () {
  fn capturedTexts(self, this: &mut QRegExp) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegExp13capturedTextsEv()};
    unsafe {_ZN7QRegExp13capturedTextsEv()};
    return 1;
  }
}

impl /*struct*/ QRegExp {
  pub fn captureCount<T: QRegExp_captureCount>(&mut self, value: T) -> i32 {
    value.captureCount(self);
    return 1;
  }
}

pub trait QRegExp_captureCount {
  fn captureCount(self, this: &mut QRegExp) -> i32;
}

// proto: int QRegExp::captureCount();
impl<'a> /*trait*/ QRegExp_captureCount for () {
  fn captureCount(self, this: &mut QRegExp) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegExp12captureCountEv()};
    unsafe {_ZNK7QRegExp12captureCountEv()};
    return 1;
  }
}

impl /*struct*/ QRegExp {
  pub fn escape<T: QRegExp_escape>(&mut self, value: T) -> i32 {
    value.escape(self);
    return 1;
  }
}

pub trait QRegExp_escape {
  fn escape(self, this: &mut QRegExp) -> i32;
}

// proto: QString QRegExp::escape(const QString & str);
impl<'a> /*trait*/ QRegExp_escape for (&'a  QString) {
  fn escape(self, this: &mut QRegExp) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegExp6escapeERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QRegExp6escapeERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QRegExp {
  pub fn isEmpty<T: QRegExp_isEmpty>(&mut self, value: T) -> i32 {
    value.isEmpty(self);
    return 1;
  }
}

pub trait QRegExp_isEmpty {
  fn isEmpty(self, this: &mut QRegExp) -> i32;
}

// proto: bool QRegExp::isEmpty();
impl<'a> /*trait*/ QRegExp_isEmpty for () {
  fn isEmpty(self, this: &mut QRegExp) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegExp7isEmptyEv()};
    unsafe {_ZNK7QRegExp7isEmptyEv()};
    return 1;
  }
}

impl /*struct*/ QRegExp {
  pub fn isMinimal<T: QRegExp_isMinimal>(&mut self, value: T) -> i32 {
    value.isMinimal(self);
    return 1;
  }
}

pub trait QRegExp_isMinimal {
  fn isMinimal(self, this: &mut QRegExp) -> i32;
}

// proto: bool QRegExp::isMinimal();
impl<'a> /*trait*/ QRegExp_isMinimal for () {
  fn isMinimal(self, this: &mut QRegExp) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegExp9isMinimalEv()};
    unsafe {_ZNK7QRegExp9isMinimalEv()};
    return 1;
  }
}

impl /*struct*/ QRegExp {
  pub fn matchedLength<T: QRegExp_matchedLength>(&mut self, value: T) -> i32 {
    value.matchedLength(self);
    return 1;
  }
}

pub trait QRegExp_matchedLength {
  fn matchedLength(self, this: &mut QRegExp) -> i32;
}

// proto: int QRegExp::matchedLength();
impl<'a> /*trait*/ QRegExp_matchedLength for () {
  fn matchedLength(self, this: &mut QRegExp) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegExp13matchedLengthEv()};
    unsafe {_ZNK7QRegExp13matchedLengthEv()};
    return 1;
  }
}

impl /*struct*/ QRegExp {
  pub fn pattern<T: QRegExp_pattern>(&mut self, value: T) -> i32 {
    value.pattern(self);
    return 1;
  }
}

pub trait QRegExp_pattern {
  fn pattern(self, this: &mut QRegExp) -> i32;
}

// proto: QString QRegExp::pattern();
impl<'a> /*trait*/ QRegExp_pattern for () {
  fn pattern(self, this: &mut QRegExp) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegExp7patternEv()};
    unsafe {_ZNK7QRegExp7patternEv()};
    return 1;
  }
}

impl /*struct*/ QRegExp {
  pub fn setPattern<T: QRegExp_setPattern>(&mut self, value: T) -> i32 {
    value.setPattern(self);
    return 1;
  }
}

pub trait QRegExp_setPattern {
  fn setPattern(self, this: &mut QRegExp) -> i32;
}

// proto: void QRegExp::setPattern(const QString & pattern);
impl<'a> /*trait*/ QRegExp_setPattern for (&'a  QString) {
  fn setPattern(self, this: &mut QRegExp) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegExp10setPatternERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QRegExp10setPatternERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QRegExp {
  pub fn isValid<T: QRegExp_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QRegExp_isValid {
  fn isValid(self, this: &mut QRegExp) -> i32;
}

// proto: bool QRegExp::isValid();
impl<'a> /*trait*/ QRegExp_isValid for () {
  fn isValid(self, this: &mut QRegExp) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegExp7isValidEv()};
    unsafe {_ZNK7QRegExp7isValidEv()};
    return 1;
  }
}

impl /*struct*/ QRegExp {
  pub fn FreeQRegExp<T: QRegExp_FreeQRegExp>(&mut self, value: T) -> i32 {
    value.FreeQRegExp(self);
    return 1;
  }
}

pub trait QRegExp_FreeQRegExp {
  fn FreeQRegExp(self, this: &mut QRegExp) -> i32;
}

// proto: void QRegExp::FreeQRegExp();
impl<'a> /*trait*/ QRegExp_FreeQRegExp for () {
  fn FreeQRegExp(self, this: &mut QRegExp) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegExpD0Ev()};
    unsafe {_ZN7QRegExpD0Ev()};
    return 1;
  }
}

impl /*struct*/ QRegExp {
  pub fn exactMatch<T: QRegExp_exactMatch>(&mut self, value: T) -> i32 {
    value.exactMatch(self);
    return 1;
  }
}

pub trait QRegExp_exactMatch {
  fn exactMatch(self, this: &mut QRegExp) -> i32;
}

// proto: bool QRegExp::exactMatch(const QString & str);
impl<'a> /*trait*/ QRegExp_exactMatch for (&'a  QString) {
  fn exactMatch(self, this: &mut QRegExp) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegExp10exactMatchERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QRegExp10exactMatchERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QRegExp {
  pub fn swap<T: QRegExp_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QRegExp_swap {
  fn swap(self, this: &mut QRegExp) -> i32;
}

// proto: void QRegExp::swap(QRegExp & other);
impl<'a> /*trait*/ QRegExp_swap for (&'a mut QRegExp) {
  fn swap(self, this: &mut QRegExp) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegExp4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QRegExp4swapERS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QRegExp {
  pub fn pos<T: QRegExp_pos>(&mut self, value: T) -> i32 {
    value.pos(self);
    return 1;
  }
}

pub trait QRegExp_pos {
  fn pos(self, this: &mut QRegExp) -> i32;
}

// proto: int QRegExp::pos(int nth);
impl<'a> /*trait*/ QRegExp_pos for (i32) {
  fn pos(self, this: &mut QRegExp) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegExp3posEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QRegExp3posEi(arg0)};
    return 1;
  }
}

// proto: void QRegExp::NewQRegExp();
impl<'a> /*trait*/ QRegExp_NewQRegExp for () {
  fn NewQRegExp(self) -> QRegExp {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegExpC1Ev()};
    unsafe {_ZN7QRegExpC1Ev(qthis)};
    let rsthis = QRegExp{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QRegExp {
  pub fn cap<T: QRegExp_cap>(&mut self, value: T) -> i32 {
    value.cap(self);
    return 1;
  }
}

pub trait QRegExp_cap {
  fn cap(self, this: &mut QRegExp) -> i32;
}

// proto: QString QRegExp::cap(int nth);
impl<'a> /*trait*/ QRegExp_cap for (i32) {
  fn cap(self, this: &mut QRegExp) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegExp3capEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QRegExp3capEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QRegExp {
  pub fn errorString<T: QRegExp_errorString>(&mut self, value: T) -> i32 {
    value.errorString(self);
    return 1;
  }
}

pub trait QRegExp_errorString {
  fn errorString(self, this: &mut QRegExp) -> i32;
}

// proto: QString QRegExp::errorString();
impl<'a> /*trait*/ QRegExp_errorString for () {
  fn errorString(self, this: &mut QRegExp) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegExp11errorStringEv()};
    unsafe {_ZN7QRegExp11errorStringEv()};
    return 1;
  }
}

impl /*struct*/ QRegExp {
  pub fn setMinimal<T: QRegExp_setMinimal>(&mut self, value: T) -> i32 {
    value.setMinimal(self);
    return 1;
  }
}

pub trait QRegExp_setMinimal {
  fn setMinimal(self, this: &mut QRegExp) -> i32;
}

// proto: void QRegExp::setMinimal(bool minimal);
impl<'a> /*trait*/ QRegExp_setMinimal for (i8) {
  fn setMinimal(self, this: &mut QRegExp) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegExp10setMinimalEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QRegExp10setMinimalEb(arg0)};
    return 1;
  }
}

