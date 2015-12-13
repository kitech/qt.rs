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
  fn _ZNK23QRegularExpressionMatch17lastCapturedIndexEv() -> i32;
  fn _ZN23QRegularExpressionMatchC1Ev(qthis: *mut c_void) -> i32;
  fn _ZNK23QRegularExpressionMatch7isValidEv() -> i32;
  fn _ZNK23QRegularExpressionMatch14capturedLengthEi(arg0: c_int) -> i32;
  fn _ZNK23QRegularExpressionMatch14capturedLengthERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK23QRegularExpressionMatch11capturedRefEi(arg0: c_int) -> i32;
  fn _ZNK23QRegularExpressionMatch11capturedEndERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK23QRegularExpressionMatch8capturedERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK23QRegularExpressionMatch13capturedTextsEv() -> i32;
  fn _ZN23QRegularExpressionMatchC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN23QRegularExpressionMatch4swapERS_(arg0: *mut c_void) -> i32;
  fn _ZN23QRegularExpressionMatchD0Ev() -> i32;
  fn _ZNK23QRegularExpressionMatch11capturedEndEi(arg0: c_int) -> i32;
  fn _ZNK23QRegularExpressionMatch11capturedRefERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK23QRegularExpressionMatch8hasMatchEv() -> i32;
  fn _ZNK23QRegularExpressionMatch13capturedStartERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK23QRegularExpressionMatch17regularExpressionEv() -> i32;
  fn _ZNK23QRegularExpressionMatch8capturedEi(arg0: c_int) -> i32;
  fn _ZNK23QRegularExpressionMatch13capturedStartEi(arg0: c_int) -> i32;
  fn _ZNK23QRegularExpressionMatch15hasPartialMatchEv() -> i32;
}

// body block begin
// class sizeof(QRegularExpressionMatch)=1
pub struct QRegularExpressionMatch {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QRegularExpressionMatch {
  pub fn lastCapturedIndex<T: QRegularExpressionMatch_lastCapturedIndex>(&mut self, value: T) -> i32 {
    value.lastCapturedIndex(self);
    return 1;
  }
}

pub trait QRegularExpressionMatch_lastCapturedIndex {
  fn lastCapturedIndex(self, this: &mut QRegularExpressionMatch) -> i32;
}

// proto: int QRegularExpressionMatch::lastCapturedIndex();
impl<'a> /*trait*/ QRegularExpressionMatch_lastCapturedIndex for () {
  fn lastCapturedIndex(self, this: &mut QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch17lastCapturedIndexEv()};
    unsafe {_ZNK23QRegularExpressionMatch17lastCapturedIndexEv()};
    return 1;
  }
}

impl /*struct*/ QRegularExpressionMatch {
  pub fn NewQRegularExpressionMatch<T: QRegularExpressionMatch_NewQRegularExpressionMatch>(value: T) -> QRegularExpressionMatch {
    let rsthis = value.NewQRegularExpressionMatch();
    return rsthis;
    // return 1;
  }
}

pub trait QRegularExpressionMatch_NewQRegularExpressionMatch {
  fn NewQRegularExpressionMatch(self) -> QRegularExpressionMatch;
}

// proto: void QRegularExpressionMatch::NewQRegularExpressionMatch();
impl<'a> /*trait*/ QRegularExpressionMatch_NewQRegularExpressionMatch for () {
  fn NewQRegularExpressionMatch(self) -> QRegularExpressionMatch {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QRegularExpressionMatchC1Ev()};
    unsafe {_ZN23QRegularExpressionMatchC1Ev(qthis)};
    let rsthis = QRegularExpressionMatch{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QRegularExpressionMatch {
  pub fn isValid<T: QRegularExpressionMatch_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QRegularExpressionMatch_isValid {
  fn isValid(self, this: &mut QRegularExpressionMatch) -> i32;
}

// proto: bool QRegularExpressionMatch::isValid();
impl<'a> /*trait*/ QRegularExpressionMatch_isValid for () {
  fn isValid(self, this: &mut QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch7isValidEv()};
    unsafe {_ZNK23QRegularExpressionMatch7isValidEv()};
    return 1;
  }
}

impl /*struct*/ QRegularExpressionMatch {
  pub fn capturedLength<T: QRegularExpressionMatch_capturedLength>(&mut self, value: T) -> i32 {
    value.capturedLength(self);
    return 1;
  }
}

pub trait QRegularExpressionMatch_capturedLength {
  fn capturedLength(self, this: &mut QRegularExpressionMatch) -> i32;
}

// proto: int QRegularExpressionMatch::capturedLength(int nth);
impl<'a> /*trait*/ QRegularExpressionMatch_capturedLength for (i32) {
  fn capturedLength(self, this: &mut QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch14capturedLengthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK23QRegularExpressionMatch14capturedLengthEi(arg0)};
    return 1;
  }
}

// proto: int QRegularExpressionMatch::capturedLength(const QString & name);
impl<'a> /*trait*/ QRegularExpressionMatch_capturedLength for (&'a  QString) {
  fn capturedLength(self, this: &mut QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch14capturedLengthERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK23QRegularExpressionMatch14capturedLengthERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QRegularExpressionMatch {
  pub fn capturedRef<T: QRegularExpressionMatch_capturedRef>(&mut self, value: T) -> i32 {
    value.capturedRef(self);
    return 1;
  }
}

pub trait QRegularExpressionMatch_capturedRef {
  fn capturedRef(self, this: &mut QRegularExpressionMatch) -> i32;
}

// proto: QStringRef QRegularExpressionMatch::capturedRef(int nth);
impl<'a> /*trait*/ QRegularExpressionMatch_capturedRef for (i32) {
  fn capturedRef(self, this: &mut QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch11capturedRefEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK23QRegularExpressionMatch11capturedRefEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QRegularExpressionMatch {
  pub fn capturedEnd<T: QRegularExpressionMatch_capturedEnd>(&mut self, value: T) -> i32 {
    value.capturedEnd(self);
    return 1;
  }
}

pub trait QRegularExpressionMatch_capturedEnd {
  fn capturedEnd(self, this: &mut QRegularExpressionMatch) -> i32;
}

// proto: int QRegularExpressionMatch::capturedEnd(const QString & name);
impl<'a> /*trait*/ QRegularExpressionMatch_capturedEnd for (&'a  QString) {
  fn capturedEnd(self, this: &mut QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch11capturedEndERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK23QRegularExpressionMatch11capturedEndERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QRegularExpressionMatch {
  pub fn captured<T: QRegularExpressionMatch_captured>(&mut self, value: T) -> i32 {
    value.captured(self);
    return 1;
  }
}

pub trait QRegularExpressionMatch_captured {
  fn captured(self, this: &mut QRegularExpressionMatch) -> i32;
}

// proto: QString QRegularExpressionMatch::captured(const QString & name);
impl<'a> /*trait*/ QRegularExpressionMatch_captured for (&'a  QString) {
  fn captured(self, this: &mut QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch8capturedERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK23QRegularExpressionMatch8capturedERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QRegularExpressionMatch {
  pub fn capturedTexts<T: QRegularExpressionMatch_capturedTexts>(&mut self, value: T) -> i32 {
    value.capturedTexts(self);
    return 1;
  }
}

pub trait QRegularExpressionMatch_capturedTexts {
  fn capturedTexts(self, this: &mut QRegularExpressionMatch) -> i32;
}

// proto: QStringList QRegularExpressionMatch::capturedTexts();
impl<'a> /*trait*/ QRegularExpressionMatch_capturedTexts for () {
  fn capturedTexts(self, this: &mut QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch13capturedTextsEv()};
    unsafe {_ZNK23QRegularExpressionMatch13capturedTextsEv()};
    return 1;
  }
}

// proto: void QRegularExpressionMatch::NewQRegularExpressionMatch(const QRegularExpressionMatch & match);
impl<'a> /*trait*/ QRegularExpressionMatch_NewQRegularExpressionMatch for (&'a  QRegularExpressionMatch) {
  fn NewQRegularExpressionMatch(self) -> QRegularExpressionMatch {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QRegularExpressionMatchC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN23QRegularExpressionMatchC1ERKS_(qthis, arg0)};
    let rsthis = QRegularExpressionMatch{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QRegularExpressionMatch {
  pub fn swap<T: QRegularExpressionMatch_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QRegularExpressionMatch_swap {
  fn swap(self, this: &mut QRegularExpressionMatch) -> i32;
}

// proto: void QRegularExpressionMatch::swap(QRegularExpressionMatch & other);
impl<'a> /*trait*/ QRegularExpressionMatch_swap for (&'a mut QRegularExpressionMatch) {
  fn swap(self, this: &mut QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QRegularExpressionMatch4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN23QRegularExpressionMatch4swapERS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QRegularExpressionMatch {
  pub fn FreeQRegularExpressionMatch<T: QRegularExpressionMatch_FreeQRegularExpressionMatch>(&mut self, value: T) -> i32 {
    value.FreeQRegularExpressionMatch(self);
    return 1;
  }
}

pub trait QRegularExpressionMatch_FreeQRegularExpressionMatch {
  fn FreeQRegularExpressionMatch(self, this: &mut QRegularExpressionMatch) -> i32;
}

// proto: void QRegularExpressionMatch::FreeQRegularExpressionMatch();
impl<'a> /*trait*/ QRegularExpressionMatch_FreeQRegularExpressionMatch for () {
  fn FreeQRegularExpressionMatch(self, this: &mut QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QRegularExpressionMatchD0Ev()};
    unsafe {_ZN23QRegularExpressionMatchD0Ev()};
    return 1;
  }
}

// proto: int QRegularExpressionMatch::capturedEnd(int nth);
impl<'a> /*trait*/ QRegularExpressionMatch_capturedEnd for (i32) {
  fn capturedEnd(self, this: &mut QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch11capturedEndEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK23QRegularExpressionMatch11capturedEndEi(arg0)};
    return 1;
  }
}

// proto: QStringRef QRegularExpressionMatch::capturedRef(const QString & name);
impl<'a> /*trait*/ QRegularExpressionMatch_capturedRef for (&'a  QString) {
  fn capturedRef(self, this: &mut QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch11capturedRefERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK23QRegularExpressionMatch11capturedRefERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QRegularExpressionMatch {
  pub fn hasMatch<T: QRegularExpressionMatch_hasMatch>(&mut self, value: T) -> i32 {
    value.hasMatch(self);
    return 1;
  }
}

pub trait QRegularExpressionMatch_hasMatch {
  fn hasMatch(self, this: &mut QRegularExpressionMatch) -> i32;
}

// proto: bool QRegularExpressionMatch::hasMatch();
impl<'a> /*trait*/ QRegularExpressionMatch_hasMatch for () {
  fn hasMatch(self, this: &mut QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch8hasMatchEv()};
    unsafe {_ZNK23QRegularExpressionMatch8hasMatchEv()};
    return 1;
  }
}

impl /*struct*/ QRegularExpressionMatch {
  pub fn capturedStart<T: QRegularExpressionMatch_capturedStart>(&mut self, value: T) -> i32 {
    value.capturedStart(self);
    return 1;
  }
}

pub trait QRegularExpressionMatch_capturedStart {
  fn capturedStart(self, this: &mut QRegularExpressionMatch) -> i32;
}

// proto: int QRegularExpressionMatch::capturedStart(const QString & name);
impl<'a> /*trait*/ QRegularExpressionMatch_capturedStart for (&'a  QString) {
  fn capturedStart(self, this: &mut QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch13capturedStartERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK23QRegularExpressionMatch13capturedStartERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QRegularExpressionMatch {
  pub fn regularExpression<T: QRegularExpressionMatch_regularExpression>(&mut self, value: T) -> i32 {
    value.regularExpression(self);
    return 1;
  }
}

pub trait QRegularExpressionMatch_regularExpression {
  fn regularExpression(self, this: &mut QRegularExpressionMatch) -> i32;
}

// proto: QRegularExpression QRegularExpressionMatch::regularExpression();
impl<'a> /*trait*/ QRegularExpressionMatch_regularExpression for () {
  fn regularExpression(self, this: &mut QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch17regularExpressionEv()};
    unsafe {_ZNK23QRegularExpressionMatch17regularExpressionEv()};
    return 1;
  }
}

// proto: QString QRegularExpressionMatch::captured(int nth);
impl<'a> /*trait*/ QRegularExpressionMatch_captured for (i32) {
  fn captured(self, this: &mut QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch8capturedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK23QRegularExpressionMatch8capturedEi(arg0)};
    return 1;
  }
}

// proto: int QRegularExpressionMatch::capturedStart(int nth);
impl<'a> /*trait*/ QRegularExpressionMatch_capturedStart for (i32) {
  fn capturedStart(self, this: &mut QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch13capturedStartEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK23QRegularExpressionMatch13capturedStartEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QRegularExpressionMatch {
  pub fn hasPartialMatch<T: QRegularExpressionMatch_hasPartialMatch>(&mut self, value: T) -> i32 {
    value.hasPartialMatch(self);
    return 1;
  }
}

pub trait QRegularExpressionMatch_hasPartialMatch {
  fn hasPartialMatch(self, this: &mut QRegularExpressionMatch) -> i32;
}

// proto: bool QRegularExpressionMatch::hasPartialMatch();
impl<'a> /*trait*/ QRegularExpressionMatch_hasPartialMatch for () {
  fn hasPartialMatch(self, this: &mut QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch15hasPartialMatchEv()};
    unsafe {_ZNK23QRegularExpressionMatch15hasPartialMatchEv()};
    return 1;
  }
}

