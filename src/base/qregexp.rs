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
  // proto:  void QRegExp::NewQRegExp(const QRegExp & rx);
  fn _ZN7QRegExpC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QStringList QRegExp::capturedTexts();
  fn _ZN7QRegExp13capturedTextsEv(qthis: *mut c_void) ;
  // proto:  int QRegExp::captureCount();
  fn _ZNK7QRegExp12captureCountEv(qthis: *mut c_void) -> c_int;
  // proto: static QString QRegExp::escape(const QString & str);
  fn _ZN7QRegExp6escapeERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QRegExp::isEmpty();
  fn _ZNK7QRegExp7isEmptyEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QRegExp::isMinimal();
  fn _ZNK7QRegExp9isMinimalEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QRegExp::matchedLength();
  fn _ZNK7QRegExp13matchedLengthEv(qthis: *mut c_void) -> c_int;
  // proto:  QString QRegExp::pattern();
  fn _ZNK7QRegExp7patternEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRegExp::setPattern(const QString & pattern);
  fn _ZN7QRegExp10setPatternERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QRegExp::isValid();
  fn _ZNK7QRegExp7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QRegExp::FreeQRegExp();
  fn _ZN7QRegExpD0Ev(qthis: *mut c_void) ;
  // proto:  bool QRegExp::exactMatch(const QString & str);
  fn _ZNK7QRegExp10exactMatchERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QRegExp::swap(QRegExp & other);
  fn _ZN7QRegExp4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QRegExp::pos(int nth);
  fn _ZN7QRegExp3posEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  void QRegExp::NewQRegExp();
  fn _ZN7QRegExpC1Ev(qthis: *mut c_void) ;
  // proto:  QString QRegExp::cap(int nth);
  fn _ZN7QRegExp3capEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QString QRegExp::errorString();
  fn _ZN7QRegExp11errorStringEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRegExp::setMinimal(bool minimal);
  fn _ZN7QRegExp10setMinimalEb(qthis: *mut c_void, arg0: int8_t) ;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QRegExpC1ERKS_(qthis, arg0)};
    let rsthis = QRegExp{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QRegExp {
  pub fn capturedTexts<T: QRegExp_capturedTexts>(&mut self, value: T)  {
     value.capturedTexts(self);
    // return 1;
  }
}

pub trait QRegExp_capturedTexts {
  fn capturedTexts(self, rsthis: &mut QRegExp) ;
}

// proto:  QStringList QRegExp::capturedTexts();
impl<'a> /*trait*/ QRegExp_capturedTexts for () {
  fn capturedTexts(self, rsthis: &mut QRegExp)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegExp13capturedTextsEv()};
     unsafe {_ZN7QRegExp13capturedTextsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QRegExp {
  pub fn captureCount<T: QRegExp_captureCount>(&mut self, value: T) -> i32 {
    return value.captureCount(self);
    // return 1;
  }
}

pub trait QRegExp_captureCount {
  fn captureCount(self, rsthis: &mut QRegExp) -> i32;
}

// proto:  int QRegExp::captureCount();
impl<'a> /*trait*/ QRegExp_captureCount for () {
  fn captureCount(self, rsthis: &mut QRegExp) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegExp12captureCountEv()};
    let mut ret = unsafe {_ZNK7QRegExp12captureCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QRegExp {
  pub fn escape<T: QRegExp_escape>(&mut self, value: T) -> QString {
    return value.escape(self);
    // return 1;
  }
}

pub trait QRegExp_escape {
  fn escape(self, rsthis: &mut QRegExp) -> QString;
}

// proto: static QString QRegExp::escape(const QString & str);
impl<'a> /*trait*/ QRegExp_escape for (&'a  QString) {
  fn escape(self, rsthis: &mut QRegExp) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegExp6escapeERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QRegExp6escapeERK7QString(arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QRegExp {
  pub fn isEmpty<T: QRegExp_isEmpty>(&mut self, value: T) -> i8 {
    return value.isEmpty(self);
    // return 1;
  }
}

pub trait QRegExp_isEmpty {
  fn isEmpty(self, rsthis: &mut QRegExp) -> i8;
}

// proto:  bool QRegExp::isEmpty();
impl<'a> /*trait*/ QRegExp_isEmpty for () {
  fn isEmpty(self, rsthis: &mut QRegExp) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegExp7isEmptyEv()};
    let mut ret = unsafe {_ZNK7QRegExp7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QRegExp {
  pub fn isMinimal<T: QRegExp_isMinimal>(&mut self, value: T) -> i8 {
    return value.isMinimal(self);
    // return 1;
  }
}

pub trait QRegExp_isMinimal {
  fn isMinimal(self, rsthis: &mut QRegExp) -> i8;
}

// proto:  bool QRegExp::isMinimal();
impl<'a> /*trait*/ QRegExp_isMinimal for () {
  fn isMinimal(self, rsthis: &mut QRegExp) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegExp9isMinimalEv()};
    let mut ret = unsafe {_ZNK7QRegExp9isMinimalEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QRegExp {
  pub fn matchedLength<T: QRegExp_matchedLength>(&mut self, value: T) -> i32 {
    return value.matchedLength(self);
    // return 1;
  }
}

pub trait QRegExp_matchedLength {
  fn matchedLength(self, rsthis: &mut QRegExp) -> i32;
}

// proto:  int QRegExp::matchedLength();
impl<'a> /*trait*/ QRegExp_matchedLength for () {
  fn matchedLength(self, rsthis: &mut QRegExp) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegExp13matchedLengthEv()};
    let mut ret = unsafe {_ZNK7QRegExp13matchedLengthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QRegExp {
  pub fn pattern<T: QRegExp_pattern>(&mut self, value: T) -> QString {
    return value.pattern(self);
    // return 1;
  }
}

pub trait QRegExp_pattern {
  fn pattern(self, rsthis: &mut QRegExp) -> QString;
}

// proto:  QString QRegExp::pattern();
impl<'a> /*trait*/ QRegExp_pattern for () {
  fn pattern(self, rsthis: &mut QRegExp) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegExp7patternEv()};
    let mut ret = unsafe {_ZNK7QRegExp7patternEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QRegExp {
  pub fn setPattern<T: QRegExp_setPattern>(&mut self, value: T)  {
     value.setPattern(self);
    // return 1;
  }
}

pub trait QRegExp_setPattern {
  fn setPattern(self, rsthis: &mut QRegExp) ;
}

// proto:  void QRegExp::setPattern(const QString & pattern);
impl<'a> /*trait*/ QRegExp_setPattern for (&'a  QString) {
  fn setPattern(self, rsthis: &mut QRegExp)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegExp10setPatternERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QRegExp10setPatternERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRegExp {
  pub fn isValid<T: QRegExp_isValid>(&mut self, value: T) -> i8 {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QRegExp_isValid {
  fn isValid(self, rsthis: &mut QRegExp) -> i8;
}

// proto:  bool QRegExp::isValid();
impl<'a> /*trait*/ QRegExp_isValid for () {
  fn isValid(self, rsthis: &mut QRegExp) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegExp7isValidEv()};
    let mut ret = unsafe {_ZNK7QRegExp7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QRegExp {
  pub fn FreeQRegExp<T: QRegExp_FreeQRegExp>(&mut self, value: T)  {
     value.FreeQRegExp(self);
    // return 1;
  }
}

pub trait QRegExp_FreeQRegExp {
  fn FreeQRegExp(self, rsthis: &mut QRegExp) ;
}

// proto:  void QRegExp::FreeQRegExp();
impl<'a> /*trait*/ QRegExp_FreeQRegExp for () {
  fn FreeQRegExp(self, rsthis: &mut QRegExp)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegExpD0Ev()};
     unsafe {_ZN7QRegExpD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QRegExp {
  pub fn exactMatch<T: QRegExp_exactMatch>(&mut self, value: T) -> i8 {
    return value.exactMatch(self);
    // return 1;
  }
}

pub trait QRegExp_exactMatch {
  fn exactMatch(self, rsthis: &mut QRegExp) -> i8;
}

// proto:  bool QRegExp::exactMatch(const QString & str);
impl<'a> /*trait*/ QRegExp_exactMatch for (&'a  QString) {
  fn exactMatch(self, rsthis: &mut QRegExp) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegExp10exactMatchERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QRegExp10exactMatchERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QRegExp {
  pub fn swap<T: QRegExp_swap>(&mut self, value: T)  {
     value.swap(self);
    // return 1;
  }
}

pub trait QRegExp_swap {
  fn swap(self, rsthis: &mut QRegExp) ;
}

// proto:  void QRegExp::swap(QRegExp & other);
impl<'a> /*trait*/ QRegExp_swap for (&'a mut QRegExp) {
  fn swap(self, rsthis: &mut QRegExp)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegExp4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QRegExp4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRegExp {
  pub fn pos<T: QRegExp_pos>(&mut self, value: T) -> i32 {
    return value.pos(self);
    // return 1;
  }
}

pub trait QRegExp_pos {
  fn pos(self, rsthis: &mut QRegExp) -> i32;
}

// proto:  int QRegExp::pos(int nth);
impl<'a> /*trait*/ QRegExp_pos for (i32) {
  fn pos(self, rsthis: &mut QRegExp) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegExp3posEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN7QRegExp3posEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
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
  pub fn cap<T: QRegExp_cap>(&mut self, value: T) -> QString {
    return value.cap(self);
    // return 1;
  }
}

pub trait QRegExp_cap {
  fn cap(self, rsthis: &mut QRegExp) -> QString;
}

// proto:  QString QRegExp::cap(int nth);
impl<'a> /*trait*/ QRegExp_cap for (i32) {
  fn cap(self, rsthis: &mut QRegExp) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegExp3capEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN7QRegExp3capEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QRegExp {
  pub fn errorString<T: QRegExp_errorString>(&mut self, value: T) -> QString {
    return value.errorString(self);
    // return 1;
  }
}

pub trait QRegExp_errorString {
  fn errorString(self, rsthis: &mut QRegExp) -> QString;
}

// proto:  QString QRegExp::errorString();
impl<'a> /*trait*/ QRegExp_errorString for () {
  fn errorString(self, rsthis: &mut QRegExp) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegExp11errorStringEv()};
    let mut ret = unsafe {_ZN7QRegExp11errorStringEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QRegExp {
  pub fn setMinimal<T: QRegExp_setMinimal>(&mut self, value: T)  {
     value.setMinimal(self);
    // return 1;
  }
}

pub trait QRegExp_setMinimal {
  fn setMinimal(self, rsthis: &mut QRegExp) ;
}

// proto:  void QRegExp::setMinimal(bool minimal);
impl<'a> /*trait*/ QRegExp_setMinimal for (i8) {
  fn setMinimal(self, rsthis: &mut QRegExp)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegExp10setMinimalEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN7QRegExp10setMinimalEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

