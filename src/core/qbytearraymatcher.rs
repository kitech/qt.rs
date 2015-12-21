// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qbytearray::QByteArray;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QByteArray QByteArrayMatcher::pattern();
  fn _ZNK17QByteArrayMatcher7patternEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QByteArrayMatcher::indexIn(const char * str, int len, int from);
  fn _ZNK17QByteArrayMatcher7indexInEPKcii(qthis: *mut c_void, arg0: *mut c_char, arg1: c_int, arg2: c_int) -> c_int;
  // proto:  void QByteArrayMatcher::setPattern(const QByteArray & pattern);
  fn _ZN17QByteArrayMatcher10setPatternERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QByteArrayMatcher::QByteArrayMatcher();
  fn _ZN17QByteArrayMatcherC1Ev(qthis: *mut c_void);
  // proto:  void QByteArrayMatcher::QByteArrayMatcher(const char * pattern, int length);
  fn _ZN17QByteArrayMatcherC1EPKci(qthis: *mut c_void, arg0: *mut c_char, arg1: c_int);
  // proto:  void QByteArrayMatcher::QByteArrayMatcher(const QByteArray & pattern);
  fn _ZN17QByteArrayMatcherC1ERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QByteArrayMatcher::indexIn(const QByteArray & ba, int from);
  fn _ZNK17QByteArrayMatcher7indexInERK10QByteArrayi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  void QByteArrayMatcher::QByteArrayMatcher(const QByteArrayMatcher & other);
  fn _ZN17QByteArrayMatcherC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QByteArrayMatcher::~QByteArrayMatcher();
  fn _ZN17QByteArrayMatcherD0Ev(qthis: *mut c_void);
}

// body block begin
// class sizeof(QByteArrayMatcher)=1040
pub struct QByteArrayMatcher {
  pub qclsinst: *mut c_void,
}

  // proto:  QByteArray QByteArrayMatcher::pattern();
impl /*struct*/ QByteArrayMatcher {
  pub fn pattern<RetType, T: QByteArrayMatcher_pattern<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.pattern(self);
    // return 1;
  }
}

pub trait QByteArrayMatcher_pattern<RetType> {
  fn pattern(self , rsthis: &mut QByteArrayMatcher) -> RetType;
}

  // proto:  QByteArray QByteArrayMatcher::pattern();
impl<'a> /*trait*/ QByteArrayMatcher_pattern<QByteArray> for () {
  fn pattern(self , rsthis: &mut QByteArrayMatcher) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 1040)};
    // unsafe{_ZNK17QByteArrayMatcher7patternEv()};
    let mut ret = unsafe {_ZNK17QByteArrayMatcher7patternEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QByteArrayMatcher::indexIn(const char * str, int len, int from);
impl /*struct*/ QByteArrayMatcher {
  pub fn indexIn<RetType, T: QByteArrayMatcher_indexIn<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.indexIn(self);
    // return 1;
  }
}

pub trait QByteArrayMatcher_indexIn<RetType> {
  fn indexIn(self , rsthis: &mut QByteArrayMatcher) -> RetType;
}

  // proto:  int QByteArrayMatcher::indexIn(const char * str, int len, int from);
impl<'a> /*trait*/ QByteArrayMatcher_indexIn<i32> for (&'a  String, i32, i32) {
  fn indexIn(self , rsthis: &mut QByteArrayMatcher) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 1040)};
    // unsafe{_ZNK17QByteArrayMatcher7indexInEPKcii()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZNK17QByteArrayMatcher7indexInEPKcii(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QByteArrayMatcher::setPattern(const QByteArray & pattern);
impl /*struct*/ QByteArrayMatcher {
  pub fn setPattern<RetType, T: QByteArrayMatcher_setPattern<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setPattern(self);
    // return 1;
  }
}

pub trait QByteArrayMatcher_setPattern<RetType> {
  fn setPattern(self , rsthis: &mut QByteArrayMatcher) -> RetType;
}

  // proto:  void QByteArrayMatcher::setPattern(const QByteArray & pattern);
impl<'a> /*trait*/ QByteArrayMatcher_setPattern<()> for (QByteArray) {
  fn setPattern(self , rsthis: &mut QByteArrayMatcher) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 1040)};
    // unsafe{_ZN17QByteArrayMatcher10setPatternERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QByteArrayMatcher10setPatternERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QByteArrayMatcher::QByteArrayMatcher();
impl /*struct*/ QByteArrayMatcher {
  pub fn NewQByteArrayMatcher<T: QByteArrayMatcher_NewQByteArrayMatcher>(value: T) -> QByteArrayMatcher {
    let rsthis = value.NewQByteArrayMatcher();
    return rsthis;
    // return 1;
  }
}

pub trait QByteArrayMatcher_NewQByteArrayMatcher {
  fn NewQByteArrayMatcher(self) -> QByteArrayMatcher;
}

  // proto:  void QByteArrayMatcher::QByteArrayMatcher();
impl<'a> /*trait*/ QByteArrayMatcher_NewQByteArrayMatcher for () {
  fn NewQByteArrayMatcher(self) -> QByteArrayMatcher {
    let qthis: *mut c_void = unsafe{calloc(1, 1040)};
    // unsafe{_ZN17QByteArrayMatcherC1Ev()};
    unsafe {_ZN17QByteArrayMatcherC1Ev(qthis)};
    let rsthis = QByteArrayMatcher{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QByteArrayMatcher::QByteArrayMatcher(const char * pattern, int length);
impl<'a> /*trait*/ QByteArrayMatcher_NewQByteArrayMatcher for (&'a  String, i32) {
  fn NewQByteArrayMatcher(self) -> QByteArrayMatcher {
    let qthis: *mut c_void = unsafe{calloc(1, 1040)};
    // unsafe{_ZN17QByteArrayMatcherC1EPKci()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    unsafe {_ZN17QByteArrayMatcherC1EPKci(qthis, arg0, arg1)};
    let rsthis = QByteArrayMatcher{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QByteArrayMatcher::QByteArrayMatcher(const QByteArray & pattern);
impl<'a> /*trait*/ QByteArrayMatcher_NewQByteArrayMatcher for (QByteArray) {
  fn NewQByteArrayMatcher(self) -> QByteArrayMatcher {
    let qthis: *mut c_void = unsafe{calloc(1, 1040)};
    // unsafe{_ZN17QByteArrayMatcherC1ERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QByteArrayMatcherC1ERK10QByteArray(qthis, arg0)};
    let rsthis = QByteArrayMatcher{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QByteArrayMatcher::indexIn(const QByteArray & ba, int from);
impl<'a> /*trait*/ QByteArrayMatcher_indexIn<i32> for (QByteArray, i32) {
  fn indexIn(self , rsthis: &mut QByteArrayMatcher) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 1040)};
    // unsafe{_ZNK17QByteArrayMatcher7indexInERK10QByteArrayi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK17QByteArrayMatcher7indexInERK10QByteArrayi(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QByteArrayMatcher::QByteArrayMatcher(const QByteArrayMatcher & other);
impl<'a> /*trait*/ QByteArrayMatcher_NewQByteArrayMatcher for (QByteArrayMatcher) {
  fn NewQByteArrayMatcher(self) -> QByteArrayMatcher {
    let qthis: *mut c_void = unsafe{calloc(1, 1040)};
    // unsafe{_ZN17QByteArrayMatcherC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QByteArrayMatcherC1ERKS_(qthis, arg0)};
    let rsthis = QByteArrayMatcher{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QByteArrayMatcher::~QByteArrayMatcher();
impl /*struct*/ QByteArrayMatcher {
  pub fn FreeQByteArrayMatcher<RetType, T: QByteArrayMatcher_FreeQByteArrayMatcher<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQByteArrayMatcher(self);
    // return 1;
  }
}

pub trait QByteArrayMatcher_FreeQByteArrayMatcher<RetType> {
  fn FreeQByteArrayMatcher(self , rsthis: &mut QByteArrayMatcher) -> RetType;
}

  // proto:  void QByteArrayMatcher::~QByteArrayMatcher();
impl<'a> /*trait*/ QByteArrayMatcher_FreeQByteArrayMatcher<()> for () {
  fn FreeQByteArrayMatcher(self , rsthis: &mut QByteArrayMatcher) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 1040)};
    // unsafe{_ZN17QByteArrayMatcherD0Ev()};
     unsafe {_ZN17QByteArrayMatcherD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

