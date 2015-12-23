// auto generated, do not modify.
// created: Wed Dec 23 22:29:56 2015
// src-file: /QtCore/qregularexpression.h
// dst-file: /src/core/qregularexpression.rs
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
// use super::qregularexpression::QRegularExpressionMatch; // 773
// use super::qregularexpression::QRegularExpression; // 773
use super::qstring::QString; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  bool QRegularExpressionMatchIterator::hasNext();
  fn _ZNK31QRegularExpressionMatchIterator7hasNextEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QRegularExpressionMatchIterator::isValid();
  fn _ZNK31QRegularExpressionMatchIterator7isValidEv(qthis: *mut c_void) -> c_char;
  // proto:  QRegularExpressionMatch QRegularExpressionMatchIterator::peekNext();
  fn _ZNK31QRegularExpressionMatchIterator8peekNextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRegularExpressionMatchIterator::QRegularExpressionMatchIterator();
  fn _ZN31QRegularExpressionMatchIteratorC1Ev(qthis: *mut c_void);
  // proto:  QRegularExpression QRegularExpressionMatchIterator::regularExpression();
  fn _ZNK31QRegularExpressionMatchIterator17regularExpressionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRegularExpressionMatchIterator::QRegularExpressionMatchIterator(const QRegularExpressionMatchIterator & iterator);
  fn _ZN31QRegularExpressionMatchIteratorC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QRegularExpressionMatchIterator::~QRegularExpressionMatchIterator();
  fn _ZN31QRegularExpressionMatchIteratorD0Ev(qthis: *mut c_void);
  // proto:  QRegularExpressionMatch QRegularExpressionMatchIterator::next();
  fn _ZN31QRegularExpressionMatchIterator4nextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRegularExpressionMatchIterator::swap(QRegularExpressionMatchIterator & other);
  fn _ZN31QRegularExpressionMatchIterator4swapERS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QRegularExpression::patternErrorOffset();
  fn _ZNK18QRegularExpression18patternErrorOffsetEv(qthis: *mut c_void) -> c_int;
  // proto:  QString QRegularExpression::pattern();
  fn _ZNK18QRegularExpression7patternEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRegularExpression::~QRegularExpression();
  fn _ZN18QRegularExpressionD0Ev(qthis: *mut c_void);
  // proto:  void QRegularExpression::optimize();
  fn _ZNK18QRegularExpression8optimizeEv(qthis: *mut c_void);
  // proto: static QString QRegularExpression::escape(const QString & str);
  fn _ZN18QRegularExpression6escapeERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto:  void QRegularExpression::QRegularExpression();
  fn _ZN18QRegularExpressionC1Ev(qthis: *mut c_void);
  // proto:  void QRegularExpression::swap(QRegularExpression & other);
  fn _ZN18QRegularExpression4swapERS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QString QRegularExpression::errorString();
  fn _ZNK18QRegularExpression11errorStringEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QRegularExpression::isValid();
  fn _ZNK18QRegularExpression7isValidEv(qthis: *mut c_void) -> c_char;
  // proto:  void QRegularExpression::QRegularExpression(const QRegularExpression & re);
  fn _ZN18QRegularExpressionC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QStringList QRegularExpression::namedCaptureGroups();
  fn _ZNK18QRegularExpression18namedCaptureGroupsEv(qthis: *mut c_void);
  // proto:  int QRegularExpression::captureCount();
  fn _ZNK18QRegularExpression12captureCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QRegularExpression::setPattern(const QString & pattern);
  fn _ZN18QRegularExpression10setPatternERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QRegularExpressionMatch::lastCapturedIndex();
  fn _ZNK23QRegularExpressionMatch17lastCapturedIndexEv(qthis: *mut c_void) -> c_int;
  // proto:  void QRegularExpressionMatch::QRegularExpressionMatch();
  fn _ZN23QRegularExpressionMatchC1Ev(qthis: *mut c_void);
  // proto:  bool QRegularExpressionMatch::isValid();
  fn _ZNK23QRegularExpressionMatch7isValidEv(qthis: *mut c_void) -> c_char;
  // proto:  int QRegularExpressionMatch::capturedLength(int nth);
  fn _ZNK23QRegularExpressionMatch14capturedLengthEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  int QRegularExpressionMatch::capturedLength(const QString & name);
  fn _ZNK23QRegularExpressionMatch14capturedLengthERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  QStringRef QRegularExpressionMatch::capturedRef(int nth);
  fn _ZNK23QRegularExpressionMatch11capturedRefEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QRegularExpressionMatch::capturedEnd(const QString & name);
  fn _ZNK23QRegularExpressionMatch11capturedEndERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  QString QRegularExpressionMatch::captured(const QString & name);
  fn _ZNK23QRegularExpressionMatch8capturedERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QStringList QRegularExpressionMatch::capturedTexts();
  fn _ZNK23QRegularExpressionMatch13capturedTextsEv(qthis: *mut c_void);
  // proto:  void QRegularExpressionMatch::QRegularExpressionMatch(const QRegularExpressionMatch & match);
  fn _ZN23QRegularExpressionMatchC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QRegularExpressionMatch::swap(QRegularExpressionMatch & other);
  fn _ZN23QRegularExpressionMatch4swapERS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QRegularExpressionMatch::~QRegularExpressionMatch();
  fn _ZN23QRegularExpressionMatchD0Ev(qthis: *mut c_void);
  // proto:  int QRegularExpressionMatch::capturedEnd(int nth);
  fn _ZNK23QRegularExpressionMatch11capturedEndEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  QStringRef QRegularExpressionMatch::capturedRef(const QString & name);
  fn _ZNK23QRegularExpressionMatch11capturedRefERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QRegularExpressionMatch::hasMatch();
  fn _ZNK23QRegularExpressionMatch8hasMatchEv(qthis: *mut c_void) -> c_char;
  // proto:  int QRegularExpressionMatch::capturedStart(const QString & name);
  fn _ZNK23QRegularExpressionMatch13capturedStartERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  QRegularExpression QRegularExpressionMatch::regularExpression();
  fn _ZNK23QRegularExpressionMatch17regularExpressionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QRegularExpressionMatch::captured(int nth);
  fn _ZNK23QRegularExpressionMatch8capturedEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  int QRegularExpressionMatch::capturedStart(int nth);
  fn _ZNK23QRegularExpressionMatch13capturedStartEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  bool QRegularExpressionMatch::hasPartialMatch();
  fn _ZNK23QRegularExpressionMatch15hasPartialMatchEv(qthis: *mut c_void) -> c_char;
} // <= ext block end

// body block begin =>
// class sizeof(QRegularExpressionMatchIterator)=1
pub struct QRegularExpressionMatchIterator {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QRegularExpression)=1
pub struct QRegularExpression {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QRegularExpressionMatch)=1
pub struct QRegularExpressionMatch {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn inheritFrom(qthis: *mut c_void) -> QRegularExpressionMatchIterator {
    return QRegularExpressionMatchIterator{qclsinst: qthis};
  }
}
  // proto:  bool QRegularExpressionMatchIterator::hasNext();
impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn hasNext<RetType, T: QRegularExpressionMatchIterator_hasNext<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasNext(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatchIterator_hasNext<RetType> {
  fn hasNext(self , rsthis: & QRegularExpressionMatchIterator) -> RetType;
}

  // proto:  bool QRegularExpressionMatchIterator::hasNext();
impl<'a> /*trait*/ QRegularExpressionMatchIterator_hasNext<i8> for () {
  fn hasNext(self , rsthis: & QRegularExpressionMatchIterator) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK31QRegularExpressionMatchIterator7hasNextEv()};
    let mut ret = unsafe {_ZNK31QRegularExpressionMatchIterator7hasNextEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QRegularExpressionMatchIterator::isValid();
impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn isValid<RetType, T: QRegularExpressionMatchIterator_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatchIterator_isValid<RetType> {
  fn isValid(self , rsthis: & QRegularExpressionMatchIterator) -> RetType;
}

  // proto:  bool QRegularExpressionMatchIterator::isValid();
impl<'a> /*trait*/ QRegularExpressionMatchIterator_isValid<i8> for () {
  fn isValid(self , rsthis: & QRegularExpressionMatchIterator) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK31QRegularExpressionMatchIterator7isValidEv()};
    let mut ret = unsafe {_ZNK31QRegularExpressionMatchIterator7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QRegularExpressionMatch QRegularExpressionMatchIterator::peekNext();
impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn peekNext<RetType, T: QRegularExpressionMatchIterator_peekNext<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.peekNext(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatchIterator_peekNext<RetType> {
  fn peekNext(self , rsthis: & QRegularExpressionMatchIterator) -> RetType;
}

  // proto:  QRegularExpressionMatch QRegularExpressionMatchIterator::peekNext();
impl<'a> /*trait*/ QRegularExpressionMatchIterator_peekNext<QRegularExpressionMatch> for () {
  fn peekNext(self , rsthis: & QRegularExpressionMatchIterator) -> QRegularExpressionMatch {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK31QRegularExpressionMatchIterator8peekNextEv()};
    let mut ret = unsafe {_ZNK31QRegularExpressionMatchIterator8peekNextEv(rsthis.qclsinst)};
    let mut ret1 = QRegularExpressionMatch::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRegularExpressionMatchIterator::QRegularExpressionMatchIterator();
impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn New<T: QRegularExpressionMatchIterator_New>(value: T) -> QRegularExpressionMatchIterator {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QRegularExpressionMatchIterator_New {
  fn New(self) -> QRegularExpressionMatchIterator;
}

  // proto:  void QRegularExpressionMatchIterator::QRegularExpressionMatchIterator();
impl<'a> /*trait*/ QRegularExpressionMatchIterator_New for () {
  fn New(self) -> QRegularExpressionMatchIterator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN31QRegularExpressionMatchIteratorC1Ev()};
    unsafe {_ZN31QRegularExpressionMatchIteratorC1Ev(qthis)};
    let rsthis = QRegularExpressionMatchIterator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QRegularExpression QRegularExpressionMatchIterator::regularExpression();
impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn regularExpression<RetType, T: QRegularExpressionMatchIterator_regularExpression<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.regularExpression(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatchIterator_regularExpression<RetType> {
  fn regularExpression(self , rsthis: & QRegularExpressionMatchIterator) -> RetType;
}

  // proto:  QRegularExpression QRegularExpressionMatchIterator::regularExpression();
impl<'a> /*trait*/ QRegularExpressionMatchIterator_regularExpression<QRegularExpression> for () {
  fn regularExpression(self , rsthis: & QRegularExpressionMatchIterator) -> QRegularExpression {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK31QRegularExpressionMatchIterator17regularExpressionEv()};
    let mut ret = unsafe {_ZNK31QRegularExpressionMatchIterator17regularExpressionEv(rsthis.qclsinst)};
    let mut ret1 = QRegularExpression::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRegularExpressionMatchIterator::QRegularExpressionMatchIterator(const QRegularExpressionMatchIterator & iterator);
impl<'a> /*trait*/ QRegularExpressionMatchIterator_New for (&'a QRegularExpressionMatchIterator) {
  fn New(self) -> QRegularExpressionMatchIterator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN31QRegularExpressionMatchIteratorC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN31QRegularExpressionMatchIteratorC1ERKS_(qthis, arg0)};
    let rsthis = QRegularExpressionMatchIterator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QRegularExpressionMatchIterator::~QRegularExpressionMatchIterator();
impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn Free<RetType, T: QRegularExpressionMatchIterator_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatchIterator_Free<RetType> {
  fn Free(self , rsthis: & QRegularExpressionMatchIterator) -> RetType;
}

  // proto:  void QRegularExpressionMatchIterator::~QRegularExpressionMatchIterator();
impl<'a> /*trait*/ QRegularExpressionMatchIterator_Free<()> for () {
  fn Free(self , rsthis: & QRegularExpressionMatchIterator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN31QRegularExpressionMatchIteratorD0Ev()};
     unsafe {_ZN31QRegularExpressionMatchIteratorD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QRegularExpressionMatch QRegularExpressionMatchIterator::next();
impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn next<RetType, T: QRegularExpressionMatchIterator_next<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.next(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatchIterator_next<RetType> {
  fn next(self , rsthis: & QRegularExpressionMatchIterator) -> RetType;
}

  // proto:  QRegularExpressionMatch QRegularExpressionMatchIterator::next();
impl<'a> /*trait*/ QRegularExpressionMatchIterator_next<QRegularExpressionMatch> for () {
  fn next(self , rsthis: & QRegularExpressionMatchIterator) -> QRegularExpressionMatch {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN31QRegularExpressionMatchIterator4nextEv()};
    let mut ret = unsafe {_ZN31QRegularExpressionMatchIterator4nextEv(rsthis.qclsinst)};
    let mut ret1 = QRegularExpressionMatch::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRegularExpressionMatchIterator::swap(QRegularExpressionMatchIterator & other);
impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn swap<RetType, T: QRegularExpressionMatchIterator_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatchIterator_swap<RetType> {
  fn swap(self , rsthis: & QRegularExpressionMatchIterator) -> RetType;
}

  // proto:  void QRegularExpressionMatchIterator::swap(QRegularExpressionMatchIterator & other);
impl<'a> /*trait*/ QRegularExpressionMatchIterator_swap<()> for (&'a QRegularExpressionMatchIterator) {
  fn swap(self , rsthis: & QRegularExpressionMatchIterator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN31QRegularExpressionMatchIterator4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN31QRegularExpressionMatchIterator4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRegularExpression {
  pub fn inheritFrom(qthis: *mut c_void) -> QRegularExpression {
    return QRegularExpression{qclsinst: qthis};
  }
}
  // proto:  int QRegularExpression::patternErrorOffset();
impl /*struct*/ QRegularExpression {
  pub fn patternErrorOffset<RetType, T: QRegularExpression_patternErrorOffset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.patternErrorOffset(self);
    // return 1;
  }
}

pub trait QRegularExpression_patternErrorOffset<RetType> {
  fn patternErrorOffset(self , rsthis: & QRegularExpression) -> RetType;
}

  // proto:  int QRegularExpression::patternErrorOffset();
impl<'a> /*trait*/ QRegularExpression_patternErrorOffset<i32> for () {
  fn patternErrorOffset(self , rsthis: & QRegularExpression) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QRegularExpression18patternErrorOffsetEv()};
    let mut ret = unsafe {_ZNK18QRegularExpression18patternErrorOffsetEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QString QRegularExpression::pattern();
impl /*struct*/ QRegularExpression {
  pub fn pattern<RetType, T: QRegularExpression_pattern<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pattern(self);
    // return 1;
  }
}

pub trait QRegularExpression_pattern<RetType> {
  fn pattern(self , rsthis: & QRegularExpression) -> RetType;
}

  // proto:  QString QRegularExpression::pattern();
impl<'a> /*trait*/ QRegularExpression_pattern<QString> for () {
  fn pattern(self , rsthis: & QRegularExpression) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QRegularExpression7patternEv()};
    let mut ret = unsafe {_ZNK18QRegularExpression7patternEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRegularExpression::~QRegularExpression();
impl /*struct*/ QRegularExpression {
  pub fn Free<RetType, T: QRegularExpression_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QRegularExpression_Free<RetType> {
  fn Free(self , rsthis: & QRegularExpression) -> RetType;
}

  // proto:  void QRegularExpression::~QRegularExpression();
impl<'a> /*trait*/ QRegularExpression_Free<()> for () {
  fn Free(self , rsthis: & QRegularExpression) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QRegularExpressionD0Ev()};
     unsafe {_ZN18QRegularExpressionD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QRegularExpression::optimize();
impl /*struct*/ QRegularExpression {
  pub fn optimize<RetType, T: QRegularExpression_optimize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.optimize(self);
    // return 1;
  }
}

pub trait QRegularExpression_optimize<RetType> {
  fn optimize(self , rsthis: & QRegularExpression) -> RetType;
}

  // proto:  void QRegularExpression::optimize();
impl<'a> /*trait*/ QRegularExpression_optimize<()> for () {
  fn optimize(self , rsthis: & QRegularExpression) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QRegularExpression8optimizeEv()};
     unsafe {_ZNK18QRegularExpression8optimizeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static QString QRegularExpression::escape(const QString & str);
impl /*struct*/ QRegularExpression {
  pub fn escape_s<RetType, T: QRegularExpression_escape_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.escape_s();
    // return 1;
  }
}

pub trait QRegularExpression_escape_s<RetType> {
  fn escape_s(self ) -> RetType;
}

  // proto: static QString QRegularExpression::escape(const QString & str);
impl<'a> /*trait*/ QRegularExpression_escape_s<QString> for (&'a QString) {
  fn escape_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QRegularExpression6escapeERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN18QRegularExpression6escapeERK7QString(arg0)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRegularExpression::QRegularExpression();
impl /*struct*/ QRegularExpression {
  pub fn New<T: QRegularExpression_New>(value: T) -> QRegularExpression {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QRegularExpression_New {
  fn New(self) -> QRegularExpression;
}

  // proto:  void QRegularExpression::QRegularExpression();
impl<'a> /*trait*/ QRegularExpression_New for () {
  fn New(self) -> QRegularExpression {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QRegularExpressionC1Ev()};
    unsafe {_ZN18QRegularExpressionC1Ev(qthis)};
    let rsthis = QRegularExpression{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QRegularExpression::swap(QRegularExpression & other);
impl /*struct*/ QRegularExpression {
  pub fn swap<RetType, T: QRegularExpression_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QRegularExpression_swap<RetType> {
  fn swap(self , rsthis: & QRegularExpression) -> RetType;
}

  // proto:  void QRegularExpression::swap(QRegularExpression & other);
impl<'a> /*trait*/ QRegularExpression_swap<()> for (&'a QRegularExpression) {
  fn swap(self , rsthis: & QRegularExpression) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QRegularExpression4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QRegularExpression4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QRegularExpression::errorString();
impl /*struct*/ QRegularExpression {
  pub fn errorString<RetType, T: QRegularExpression_errorString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.errorString(self);
    // return 1;
  }
}

pub trait QRegularExpression_errorString<RetType> {
  fn errorString(self , rsthis: & QRegularExpression) -> RetType;
}

  // proto:  QString QRegularExpression::errorString();
impl<'a> /*trait*/ QRegularExpression_errorString<QString> for () {
  fn errorString(self , rsthis: & QRegularExpression) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QRegularExpression11errorStringEv()};
    let mut ret = unsafe {_ZNK18QRegularExpression11errorStringEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QRegularExpression::isValid();
impl /*struct*/ QRegularExpression {
  pub fn isValid<RetType, T: QRegularExpression_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QRegularExpression_isValid<RetType> {
  fn isValid(self , rsthis: & QRegularExpression) -> RetType;
}

  // proto:  bool QRegularExpression::isValid();
impl<'a> /*trait*/ QRegularExpression_isValid<i8> for () {
  fn isValid(self , rsthis: & QRegularExpression) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QRegularExpression7isValidEv()};
    let mut ret = unsafe {_ZNK18QRegularExpression7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QRegularExpression::QRegularExpression(const QRegularExpression & re);
impl<'a> /*trait*/ QRegularExpression_New for (&'a QRegularExpression) {
  fn New(self) -> QRegularExpression {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QRegularExpressionC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QRegularExpressionC1ERKS_(qthis, arg0)};
    let rsthis = QRegularExpression{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QStringList QRegularExpression::namedCaptureGroups();
impl /*struct*/ QRegularExpression {
  pub fn namedCaptureGroups<RetType, T: QRegularExpression_namedCaptureGroups<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.namedCaptureGroups(self);
    // return 1;
  }
}

pub trait QRegularExpression_namedCaptureGroups<RetType> {
  fn namedCaptureGroups(self , rsthis: & QRegularExpression) -> RetType;
}

  // proto:  QStringList QRegularExpression::namedCaptureGroups();
impl<'a> /*trait*/ QRegularExpression_namedCaptureGroups<()> for () {
  fn namedCaptureGroups(self , rsthis: & QRegularExpression) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QRegularExpression18namedCaptureGroupsEv()};
     unsafe {_ZNK18QRegularExpression18namedCaptureGroupsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QRegularExpression::captureCount();
impl /*struct*/ QRegularExpression {
  pub fn captureCount<RetType, T: QRegularExpression_captureCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.captureCount(self);
    // return 1;
  }
}

pub trait QRegularExpression_captureCount<RetType> {
  fn captureCount(self , rsthis: & QRegularExpression) -> RetType;
}

  // proto:  int QRegularExpression::captureCount();
impl<'a> /*trait*/ QRegularExpression_captureCount<i32> for () {
  fn captureCount(self , rsthis: & QRegularExpression) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QRegularExpression12captureCountEv()};
    let mut ret = unsafe {_ZNK18QRegularExpression12captureCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QRegularExpression::setPattern(const QString & pattern);
impl /*struct*/ QRegularExpression {
  pub fn setPattern<RetType, T: QRegularExpression_setPattern<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPattern(self);
    // return 1;
  }
}

pub trait QRegularExpression_setPattern<RetType> {
  fn setPattern(self , rsthis: & QRegularExpression) -> RetType;
}

  // proto:  void QRegularExpression::setPattern(const QString & pattern);
impl<'a> /*trait*/ QRegularExpression_setPattern<()> for (&'a QString) {
  fn setPattern(self , rsthis: & QRegularExpression) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QRegularExpression10setPatternERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QRegularExpression10setPatternERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRegularExpressionMatch {
  pub fn inheritFrom(qthis: *mut c_void) -> QRegularExpressionMatch {
    return QRegularExpressionMatch{qclsinst: qthis};
  }
}
  // proto:  int QRegularExpressionMatch::lastCapturedIndex();
impl /*struct*/ QRegularExpressionMatch {
  pub fn lastCapturedIndex<RetType, T: QRegularExpressionMatch_lastCapturedIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lastCapturedIndex(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatch_lastCapturedIndex<RetType> {
  fn lastCapturedIndex(self , rsthis: & QRegularExpressionMatch) -> RetType;
}

  // proto:  int QRegularExpressionMatch::lastCapturedIndex();
impl<'a> /*trait*/ QRegularExpressionMatch_lastCapturedIndex<i32> for () {
  fn lastCapturedIndex(self , rsthis: & QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch17lastCapturedIndexEv()};
    let mut ret = unsafe {_ZNK23QRegularExpressionMatch17lastCapturedIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QRegularExpressionMatch::QRegularExpressionMatch();
impl /*struct*/ QRegularExpressionMatch {
  pub fn New<T: QRegularExpressionMatch_New>(value: T) -> QRegularExpressionMatch {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QRegularExpressionMatch_New {
  fn New(self) -> QRegularExpressionMatch;
}

  // proto:  void QRegularExpressionMatch::QRegularExpressionMatch();
impl<'a> /*trait*/ QRegularExpressionMatch_New for () {
  fn New(self) -> QRegularExpressionMatch {
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
  pub fn isValid<RetType, T: QRegularExpressionMatch_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatch_isValid<RetType> {
  fn isValid(self , rsthis: & QRegularExpressionMatch) -> RetType;
}

  // proto:  bool QRegularExpressionMatch::isValid();
impl<'a> /*trait*/ QRegularExpressionMatch_isValid<i8> for () {
  fn isValid(self , rsthis: & QRegularExpressionMatch) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch7isValidEv()};
    let mut ret = unsafe {_ZNK23QRegularExpressionMatch7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QRegularExpressionMatch::capturedLength(int nth);
impl /*struct*/ QRegularExpressionMatch {
  pub fn capturedLength<RetType, T: QRegularExpressionMatch_capturedLength<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.capturedLength(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatch_capturedLength<RetType> {
  fn capturedLength(self , rsthis: & QRegularExpressionMatch) -> RetType;
}

  // proto:  int QRegularExpressionMatch::capturedLength(int nth);
impl<'a> /*trait*/ QRegularExpressionMatch_capturedLength<i32> for (i32) {
  fn capturedLength(self , rsthis: & QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch14capturedLengthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK23QRegularExpressionMatch14capturedLengthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QRegularExpressionMatch::capturedLength(const QString & name);
impl<'a> /*trait*/ QRegularExpressionMatch_capturedLength<i32> for (&'a QString) {
  fn capturedLength(self , rsthis: & QRegularExpressionMatch) -> i32 {
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
  pub fn capturedRef<RetType, T: QRegularExpressionMatch_capturedRef<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.capturedRef(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatch_capturedRef<RetType> {
  fn capturedRef(self , rsthis: & QRegularExpressionMatch) -> RetType;
}

  // proto:  QStringRef QRegularExpressionMatch::capturedRef(int nth);
impl<'a> /*trait*/ QRegularExpressionMatch_capturedRef<()> for (i32) {
  fn capturedRef(self , rsthis: & QRegularExpressionMatch) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch11capturedRefEi()};
    let arg0 = self  as c_int;
     unsafe {_ZNK23QRegularExpressionMatch11capturedRefEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QRegularExpressionMatch::capturedEnd(const QString & name);
impl /*struct*/ QRegularExpressionMatch {
  pub fn capturedEnd<RetType, T: QRegularExpressionMatch_capturedEnd<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.capturedEnd(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatch_capturedEnd<RetType> {
  fn capturedEnd(self , rsthis: & QRegularExpressionMatch) -> RetType;
}

  // proto:  int QRegularExpressionMatch::capturedEnd(const QString & name);
impl<'a> /*trait*/ QRegularExpressionMatch_capturedEnd<i32> for (&'a QString) {
  fn capturedEnd(self , rsthis: & QRegularExpressionMatch) -> i32 {
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
  pub fn captured<RetType, T: QRegularExpressionMatch_captured<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.captured(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatch_captured<RetType> {
  fn captured(self , rsthis: & QRegularExpressionMatch) -> RetType;
}

  // proto:  QString QRegularExpressionMatch::captured(const QString & name);
impl<'a> /*trait*/ QRegularExpressionMatch_captured<QString> for (&'a QString) {
  fn captured(self , rsthis: & QRegularExpressionMatch) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch8capturedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK23QRegularExpressionMatch8capturedERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QStringList QRegularExpressionMatch::capturedTexts();
impl /*struct*/ QRegularExpressionMatch {
  pub fn capturedTexts<RetType, T: QRegularExpressionMatch_capturedTexts<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.capturedTexts(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatch_capturedTexts<RetType> {
  fn capturedTexts(self , rsthis: & QRegularExpressionMatch) -> RetType;
}

  // proto:  QStringList QRegularExpressionMatch::capturedTexts();
impl<'a> /*trait*/ QRegularExpressionMatch_capturedTexts<()> for () {
  fn capturedTexts(self , rsthis: & QRegularExpressionMatch) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch13capturedTextsEv()};
     unsafe {_ZNK23QRegularExpressionMatch13capturedTextsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QRegularExpressionMatch::QRegularExpressionMatch(const QRegularExpressionMatch & match);
impl<'a> /*trait*/ QRegularExpressionMatch_New for (&'a QRegularExpressionMatch) {
  fn New(self) -> QRegularExpressionMatch {
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
  pub fn swap<RetType, T: QRegularExpressionMatch_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatch_swap<RetType> {
  fn swap(self , rsthis: & QRegularExpressionMatch) -> RetType;
}

  // proto:  void QRegularExpressionMatch::swap(QRegularExpressionMatch & other);
impl<'a> /*trait*/ QRegularExpressionMatch_swap<()> for (&'a QRegularExpressionMatch) {
  fn swap(self , rsthis: & QRegularExpressionMatch) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QRegularExpressionMatch4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN23QRegularExpressionMatch4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRegularExpressionMatch::~QRegularExpressionMatch();
impl /*struct*/ QRegularExpressionMatch {
  pub fn Free<RetType, T: QRegularExpressionMatch_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatch_Free<RetType> {
  fn Free(self , rsthis: & QRegularExpressionMatch) -> RetType;
}

  // proto:  void QRegularExpressionMatch::~QRegularExpressionMatch();
impl<'a> /*trait*/ QRegularExpressionMatch_Free<()> for () {
  fn Free(self , rsthis: & QRegularExpressionMatch) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QRegularExpressionMatchD0Ev()};
     unsafe {_ZN23QRegularExpressionMatchD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QRegularExpressionMatch::capturedEnd(int nth);
impl<'a> /*trait*/ QRegularExpressionMatch_capturedEnd<i32> for (i32) {
  fn capturedEnd(self , rsthis: & QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch11capturedEndEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK23QRegularExpressionMatch11capturedEndEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QStringRef QRegularExpressionMatch::capturedRef(const QString & name);
impl<'a> /*trait*/ QRegularExpressionMatch_capturedRef<()> for (&'a QString) {
  fn capturedRef(self , rsthis: & QRegularExpressionMatch) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch11capturedRefERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK23QRegularExpressionMatch11capturedRefERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QRegularExpressionMatch::hasMatch();
impl /*struct*/ QRegularExpressionMatch {
  pub fn hasMatch<RetType, T: QRegularExpressionMatch_hasMatch<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasMatch(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatch_hasMatch<RetType> {
  fn hasMatch(self , rsthis: & QRegularExpressionMatch) -> RetType;
}

  // proto:  bool QRegularExpressionMatch::hasMatch();
impl<'a> /*trait*/ QRegularExpressionMatch_hasMatch<i8> for () {
  fn hasMatch(self , rsthis: & QRegularExpressionMatch) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch8hasMatchEv()};
    let mut ret = unsafe {_ZNK23QRegularExpressionMatch8hasMatchEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QRegularExpressionMatch::capturedStart(const QString & name);
impl /*struct*/ QRegularExpressionMatch {
  pub fn capturedStart<RetType, T: QRegularExpressionMatch_capturedStart<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.capturedStart(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatch_capturedStart<RetType> {
  fn capturedStart(self , rsthis: & QRegularExpressionMatch) -> RetType;
}

  // proto:  int QRegularExpressionMatch::capturedStart(const QString & name);
impl<'a> /*trait*/ QRegularExpressionMatch_capturedStart<i32> for (&'a QString) {
  fn capturedStart(self , rsthis: & QRegularExpressionMatch) -> i32 {
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
  pub fn regularExpression<RetType, T: QRegularExpressionMatch_regularExpression<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.regularExpression(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatch_regularExpression<RetType> {
  fn regularExpression(self , rsthis: & QRegularExpressionMatch) -> RetType;
}

  // proto:  QRegularExpression QRegularExpressionMatch::regularExpression();
impl<'a> /*trait*/ QRegularExpressionMatch_regularExpression<QRegularExpression> for () {
  fn regularExpression(self , rsthis: & QRegularExpressionMatch) -> QRegularExpression {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch17regularExpressionEv()};
    let mut ret = unsafe {_ZNK23QRegularExpressionMatch17regularExpressionEv(rsthis.qclsinst)};
    let mut ret1 = QRegularExpression::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QRegularExpressionMatch::captured(int nth);
impl<'a> /*trait*/ QRegularExpressionMatch_captured<QString> for (i32) {
  fn captured(self , rsthis: & QRegularExpressionMatch) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch8capturedEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK23QRegularExpressionMatch8capturedEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QRegularExpressionMatch::capturedStart(int nth);
impl<'a> /*trait*/ QRegularExpressionMatch_capturedStart<i32> for (i32) {
  fn capturedStart(self , rsthis: & QRegularExpressionMatch) -> i32 {
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
  pub fn hasPartialMatch<RetType, T: QRegularExpressionMatch_hasPartialMatch<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasPartialMatch(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatch_hasPartialMatch<RetType> {
  fn hasPartialMatch(self , rsthis: & QRegularExpressionMatch) -> RetType;
}

  // proto:  bool QRegularExpressionMatch::hasPartialMatch();
impl<'a> /*trait*/ QRegularExpressionMatch_hasPartialMatch<i8> for () {
  fn hasPartialMatch(self , rsthis: & QRegularExpressionMatch) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QRegularExpressionMatch15hasPartialMatchEv()};
    let mut ret = unsafe {_ZNK23QRegularExpressionMatch15hasPartialMatchEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// <= body block end

