// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qregularexpression::QRegularExpression;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  int QRegularExpressionMatch::lastCapturedIndex();
  fn _ZNK23QRegularExpressionMatch17lastCapturedIndexEv(qthis: *mut c_void) -> c_int;
  // proto:  void QRegularExpressionMatch::NewQRegularExpressionMatch();
  fn _ZN23QRegularExpressionMatchC1Ev(qthis: *mut c_void) ;
  // proto:  bool QRegularExpressionMatch::isValid();
  fn _ZNK23QRegularExpressionMatch7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QRegularExpressionMatch::capturedLength(int nth);
  fn _ZNK23QRegularExpressionMatch14capturedLengthEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  int QRegularExpressionMatch::capturedLength(const QString & name);
  fn _ZNK23QRegularExpressionMatch14capturedLengthERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  QStringRef QRegularExpressionMatch::capturedRef(int nth);
  fn _ZNK23QRegularExpressionMatch11capturedRefEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QRegularExpressionMatch::capturedEnd(const QString & name);
  fn _ZNK23QRegularExpressionMatch11capturedEndERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  QString QRegularExpressionMatch::captured(const QString & name);
  fn _ZNK23QRegularExpressionMatch8capturedERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QStringList QRegularExpressionMatch::capturedTexts();
  fn _ZNK23QRegularExpressionMatch13capturedTextsEv(qthis: *mut c_void) ;
  // proto:  void QRegularExpressionMatch::NewQRegularExpressionMatch(const QRegularExpressionMatch & match);
  fn _ZN23QRegularExpressionMatchC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QRegularExpressionMatch::swap(QRegularExpressionMatch & other);
  fn _ZN23QRegularExpressionMatch4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QRegularExpressionMatch::FreeQRegularExpressionMatch();
  fn _ZN23QRegularExpressionMatchD0Ev(qthis: *mut c_void) ;
  // proto:  int QRegularExpressionMatch::capturedEnd(int nth);
  fn _ZNK23QRegularExpressionMatch11capturedEndEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  QStringRef QRegularExpressionMatch::capturedRef(const QString & name);
  fn _ZNK23QRegularExpressionMatch11capturedRefERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QRegularExpressionMatch::hasMatch();
  fn _ZNK23QRegularExpressionMatch8hasMatchEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QRegularExpressionMatch::capturedStart(const QString & name);
  fn _ZNK23QRegularExpressionMatch13capturedStartERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  QRegularExpression QRegularExpressionMatch::regularExpression();
  fn _ZNK23QRegularExpressionMatch17regularExpressionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QRegularExpressionMatch::captured(int nth);
  fn _ZNK23QRegularExpressionMatch8capturedEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  int QRegularExpressionMatch::capturedStart(int nth);
  fn _ZNK23QRegularExpressionMatch13capturedStartEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  bool QRegularExpressionMatch::hasPartialMatch();
  fn _ZNK23QRegularExpressionMatch15hasPartialMatchEv(qthis: *mut c_void) -> int8_t;
}

// body block begin
// class sizeof(QRegularExpressionMatch)=1
pub struct QRegularExpressionMatch {
  pub qclsinst: *mut c_void,
}

// proto:  int QRegularExpressionMatch::lastCapturedIndex();
impl /*struct*/ QRegularExpressionMatch {
  pub fn lastCapturedIndex<RetType, T: QRegularExpressionMatch_lastCapturedIndex<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.lastCapturedIndex(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatch_lastCapturedIndex<RetType> {
  fn lastCapturedIndex(self , rsthis: &mut QRegularExpressionMatch) -> RetType;
}

// proto:  int QRegularExpressionMatch::lastCapturedIndex();
impl<'a> /*trait*/ QRegularExpressionMatch_lastCapturedIndex<i32> for () {
  fn lastCapturedIndex(self , rsthis: &mut QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch17lastCapturedIndexEv()};
    let mut ret = unsafe {_ZNK23QRegularExpressionMatch17lastCapturedIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
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

// proto:  bool QRegularExpressionMatch::isValid();
impl /*struct*/ QRegularExpressionMatch {
  pub fn isValid<RetType, T: QRegularExpressionMatch_isValid<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatch_isValid<RetType> {
  fn isValid(self , rsthis: &mut QRegularExpressionMatch) -> RetType;
}

// proto:  bool QRegularExpressionMatch::isValid();
impl<'a> /*trait*/ QRegularExpressionMatch_isValid<i8> for () {
  fn isValid(self , rsthis: &mut QRegularExpressionMatch) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch7isValidEv()};
    let mut ret = unsafe {_ZNK23QRegularExpressionMatch7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  int QRegularExpressionMatch::capturedLength(int nth);
impl /*struct*/ QRegularExpressionMatch {
  pub fn capturedLength<RetType, T: QRegularExpressionMatch_capturedLength<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.capturedLength(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatch_capturedLength<RetType> {
  fn capturedLength(self , rsthis: &mut QRegularExpressionMatch) -> RetType;
}

// proto:  int QRegularExpressionMatch::capturedLength(int nth);
impl<'a> /*trait*/ QRegularExpressionMatch_capturedLength<i32> for (i32) {
  fn capturedLength(self , rsthis: &mut QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch14capturedLengthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK23QRegularExpressionMatch14capturedLengthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

// proto:  int QRegularExpressionMatch::capturedLength(const QString & name);
impl<'a> /*trait*/ QRegularExpressionMatch_capturedLength<i32> for (&'a  QString) {
  fn capturedLength(self , rsthis: &mut QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch14capturedLengthERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK23QRegularExpressionMatch14capturedLengthERK7QString(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

// proto:  QStringRef QRegularExpressionMatch::capturedRef(int nth);
impl /*struct*/ QRegularExpressionMatch {
  pub fn capturedRef<RetType, T: QRegularExpressionMatch_capturedRef<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.capturedRef(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatch_capturedRef<RetType> {
  fn capturedRef(self , rsthis: &mut QRegularExpressionMatch) -> RetType;
}

// proto:  QStringRef QRegularExpressionMatch::capturedRef(int nth);
impl<'a> /*trait*/ QRegularExpressionMatch_capturedRef<()> for (i32) {
  fn capturedRef(self , rsthis: &mut QRegularExpressionMatch) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch11capturedRefEi()};
    let arg0 = self  as c_int;
     unsafe {_ZNK23QRegularExpressionMatch11capturedRefEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  int QRegularExpressionMatch::capturedEnd(const QString & name);
impl /*struct*/ QRegularExpressionMatch {
  pub fn capturedEnd<RetType, T: QRegularExpressionMatch_capturedEnd<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.capturedEnd(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatch_capturedEnd<RetType> {
  fn capturedEnd(self , rsthis: &mut QRegularExpressionMatch) -> RetType;
}

// proto:  int QRegularExpressionMatch::capturedEnd(const QString & name);
impl<'a> /*trait*/ QRegularExpressionMatch_capturedEnd<i32> for (&'a  QString) {
  fn capturedEnd(self , rsthis: &mut QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch11capturedEndERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK23QRegularExpressionMatch11capturedEndERK7QString(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

// proto:  QString QRegularExpressionMatch::captured(const QString & name);
impl /*struct*/ QRegularExpressionMatch {
  pub fn captured<RetType, T: QRegularExpressionMatch_captured<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.captured(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatch_captured<RetType> {
  fn captured(self , rsthis: &mut QRegularExpressionMatch) -> RetType;
}

// proto:  QString QRegularExpressionMatch::captured(const QString & name);
impl<'a> /*trait*/ QRegularExpressionMatch_captured<QString> for (&'a  QString) {
  fn captured(self , rsthis: &mut QRegularExpressionMatch) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch8capturedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK23QRegularExpressionMatch8capturedERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QStringList QRegularExpressionMatch::capturedTexts();
impl /*struct*/ QRegularExpressionMatch {
  pub fn capturedTexts<RetType, T: QRegularExpressionMatch_capturedTexts<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.capturedTexts(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatch_capturedTexts<RetType> {
  fn capturedTexts(self , rsthis: &mut QRegularExpressionMatch) -> RetType;
}

// proto:  QStringList QRegularExpressionMatch::capturedTexts();
impl<'a> /*trait*/ QRegularExpressionMatch_capturedTexts<()> for () {
  fn capturedTexts(self , rsthis: &mut QRegularExpressionMatch) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch13capturedTextsEv()};
     unsafe {_ZNK23QRegularExpressionMatch13capturedTextsEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QRegularExpressionMatch::NewQRegularExpressionMatch(const QRegularExpressionMatch & match);
impl<'a> /*trait*/ QRegularExpressionMatch_NewQRegularExpressionMatch for (&'a  QRegularExpressionMatch) {
  fn NewQRegularExpressionMatch(self) -> QRegularExpressionMatch {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QRegularExpressionMatchC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN23QRegularExpressionMatchC1ERKS_(qthis, arg0)};
    let rsthis = QRegularExpressionMatch{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QRegularExpressionMatch::swap(QRegularExpressionMatch & other);
impl /*struct*/ QRegularExpressionMatch {
  pub fn swap<RetType, T: QRegularExpressionMatch_swap<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatch_swap<RetType> {
  fn swap(self , rsthis: &mut QRegularExpressionMatch) -> RetType;
}

// proto:  void QRegularExpressionMatch::swap(QRegularExpressionMatch & other);
impl<'a> /*trait*/ QRegularExpressionMatch_swap<()> for (&'a mut QRegularExpressionMatch) {
  fn swap(self , rsthis: &mut QRegularExpressionMatch) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QRegularExpressionMatch4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN23QRegularExpressionMatch4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QRegularExpressionMatch::FreeQRegularExpressionMatch();
impl /*struct*/ QRegularExpressionMatch {
  pub fn FreeQRegularExpressionMatch<RetType, T: QRegularExpressionMatch_FreeQRegularExpressionMatch<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQRegularExpressionMatch(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatch_FreeQRegularExpressionMatch<RetType> {
  fn FreeQRegularExpressionMatch(self , rsthis: &mut QRegularExpressionMatch) -> RetType;
}

// proto:  void QRegularExpressionMatch::FreeQRegularExpressionMatch();
impl<'a> /*trait*/ QRegularExpressionMatch_FreeQRegularExpressionMatch<()> for () {
  fn FreeQRegularExpressionMatch(self , rsthis: &mut QRegularExpressionMatch) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QRegularExpressionMatchD0Ev()};
     unsafe {_ZN23QRegularExpressionMatchD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  int QRegularExpressionMatch::capturedEnd(int nth);
impl<'a> /*trait*/ QRegularExpressionMatch_capturedEnd<i32> for (i32) {
  fn capturedEnd(self , rsthis: &mut QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch11capturedEndEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK23QRegularExpressionMatch11capturedEndEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

// proto:  QStringRef QRegularExpressionMatch::capturedRef(const QString & name);
impl<'a> /*trait*/ QRegularExpressionMatch_capturedRef<()> for (&'a  QString) {
  fn capturedRef(self , rsthis: &mut QRegularExpressionMatch) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch11capturedRefERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK23QRegularExpressionMatch11capturedRefERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  bool QRegularExpressionMatch::hasMatch();
impl /*struct*/ QRegularExpressionMatch {
  pub fn hasMatch<RetType, T: QRegularExpressionMatch_hasMatch<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.hasMatch(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatch_hasMatch<RetType> {
  fn hasMatch(self , rsthis: &mut QRegularExpressionMatch) -> RetType;
}

// proto:  bool QRegularExpressionMatch::hasMatch();
impl<'a> /*trait*/ QRegularExpressionMatch_hasMatch<i8> for () {
  fn hasMatch(self , rsthis: &mut QRegularExpressionMatch) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch8hasMatchEv()};
    let mut ret = unsafe {_ZNK23QRegularExpressionMatch8hasMatchEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  int QRegularExpressionMatch::capturedStart(const QString & name);
impl /*struct*/ QRegularExpressionMatch {
  pub fn capturedStart<RetType, T: QRegularExpressionMatch_capturedStart<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.capturedStart(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatch_capturedStart<RetType> {
  fn capturedStart(self , rsthis: &mut QRegularExpressionMatch) -> RetType;
}

// proto:  int QRegularExpressionMatch::capturedStart(const QString & name);
impl<'a> /*trait*/ QRegularExpressionMatch_capturedStart<i32> for (&'a  QString) {
  fn capturedStart(self , rsthis: &mut QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch13capturedStartERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK23QRegularExpressionMatch13capturedStartERK7QString(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

// proto:  QRegularExpression QRegularExpressionMatch::regularExpression();
impl /*struct*/ QRegularExpressionMatch {
  pub fn regularExpression<RetType, T: QRegularExpressionMatch_regularExpression<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.regularExpression(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatch_regularExpression<RetType> {
  fn regularExpression(self , rsthis: &mut QRegularExpressionMatch) -> RetType;
}

// proto:  QRegularExpression QRegularExpressionMatch::regularExpression();
impl<'a> /*trait*/ QRegularExpressionMatch_regularExpression<QRegularExpression> for () {
  fn regularExpression(self , rsthis: &mut QRegularExpressionMatch) -> QRegularExpression {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch17regularExpressionEv()};
    let mut ret = unsafe {_ZNK23QRegularExpressionMatch17regularExpressionEv(rsthis.qclsinst)};
    let mut ret1 = QRegularExpression{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QString QRegularExpressionMatch::captured(int nth);
impl<'a> /*trait*/ QRegularExpressionMatch_captured<QString> for (i32) {
  fn captured(self , rsthis: &mut QRegularExpressionMatch) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch8capturedEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK23QRegularExpressionMatch8capturedEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  int QRegularExpressionMatch::capturedStart(int nth);
impl<'a> /*trait*/ QRegularExpressionMatch_capturedStart<i32> for (i32) {
  fn capturedStart(self , rsthis: &mut QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch13capturedStartEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK23QRegularExpressionMatch13capturedStartEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

// proto:  bool QRegularExpressionMatch::hasPartialMatch();
impl /*struct*/ QRegularExpressionMatch {
  pub fn hasPartialMatch<RetType, T: QRegularExpressionMatch_hasPartialMatch<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.hasPartialMatch(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatch_hasPartialMatch<RetType> {
  fn hasPartialMatch(self , rsthis: &mut QRegularExpressionMatch) -> RetType;
}

// proto:  bool QRegularExpressionMatch::hasPartialMatch();
impl<'a> /*trait*/ QRegularExpressionMatch_hasPartialMatch<i8> for () {
  fn hasPartialMatch(self , rsthis: &mut QRegularExpressionMatch) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch15hasPartialMatchEv()};
    let mut ret = unsafe {_ZNK23QRegularExpressionMatch15hasPartialMatchEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

