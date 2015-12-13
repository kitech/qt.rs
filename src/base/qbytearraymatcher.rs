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
  fn _ZNK17QByteArrayMatcher7patternEv() -> i32;
  fn _ZNK17QByteArrayMatcher7indexInEPKcii(arg0: *const c_char, arg1: c_int, arg2: c_int) -> i32;
  fn _ZN17QByteArrayMatcher10setPatternERK10QByteArray(arg0: *const c_void) -> i32;
  fn _ZN17QByteArrayMatcherC1Ev(qthis: *mut c_void) -> i32;
  fn _ZN17QByteArrayMatcherC1EPKci(qthis: *mut c_void, arg0: *const c_char, arg1: c_int) -> i32;
  fn _ZN17QByteArrayMatcherC1ERK10QByteArray(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK17QByteArrayMatcher7indexInERK10QByteArrayi(arg0: *const c_void, arg1: c_int) -> i32;
  fn _ZN17QByteArrayMatcherC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN17QByteArrayMatcherD0Ev() -> i32;
}

// body block begin
// class sizeof(QByteArrayMatcher)=1040
pub struct QByteArrayMatcher {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QByteArrayMatcher {
  pub fn pattern<T: QByteArrayMatcher_pattern>(&mut self, value: T) -> i32 {
    value.pattern(self);
    return 1;
  }
}

pub trait QByteArrayMatcher_pattern {
  fn pattern(self, this: &mut QByteArrayMatcher) -> i32;
}

// proto: QByteArray QByteArrayMatcher::pattern();
impl<'a> /*trait*/ QByteArrayMatcher_pattern for () {
  fn pattern(self, this: &mut QByteArrayMatcher) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 1040)};
    // unsafe{_ZNK17QByteArrayMatcher7patternEv()};
    unsafe {_ZNK17QByteArrayMatcher7patternEv()};
    return 1;
  }
}

impl /*struct*/ QByteArrayMatcher {
  pub fn indexIn<T: QByteArrayMatcher_indexIn>(&mut self, value: T) -> i32 {
    value.indexIn(self);
    return 1;
  }
}

pub trait QByteArrayMatcher_indexIn {
  fn indexIn(self, this: &mut QByteArrayMatcher) -> i32;
}

// proto: int QByteArrayMatcher::indexIn(const char * str, int len, int from);
impl<'a> /*trait*/ QByteArrayMatcher_indexIn for (&'a  String, i32, i32) {
  fn indexIn(self, this: &mut QByteArrayMatcher) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 1040)};
    // unsafe{_ZNK17QByteArrayMatcher7indexInEPKcii()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZNK17QByteArrayMatcher7indexInEPKcii(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QByteArrayMatcher {
  pub fn setPattern<T: QByteArrayMatcher_setPattern>(&mut self, value: T) -> i32 {
    value.setPattern(self);
    return 1;
  }
}

pub trait QByteArrayMatcher_setPattern {
  fn setPattern(self, this: &mut QByteArrayMatcher) -> i32;
}

// proto: void QByteArrayMatcher::setPattern(const QByteArray & pattern);
impl<'a> /*trait*/ QByteArrayMatcher_setPattern for (&'a  QByteArray) {
  fn setPattern(self, this: &mut QByteArrayMatcher) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 1040)};
    // unsafe{_ZN17QByteArrayMatcher10setPatternERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN17QByteArrayMatcher10setPatternERK10QByteArray(arg0)};
    return 1;
  }
}

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

// proto: void QByteArrayMatcher::NewQByteArrayMatcher();
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

// proto: void QByteArrayMatcher::NewQByteArrayMatcher(const char * pattern, int length);
impl<'a> /*trait*/ QByteArrayMatcher_NewQByteArrayMatcher for (&'a  String, i32) {
  fn NewQByteArrayMatcher(self) -> QByteArrayMatcher {
    let qthis: *mut c_void = unsafe{calloc(1, 1040)};
    // unsafe{_ZN17QByteArrayMatcherC1EPKci()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_int;
    unsafe {_ZN17QByteArrayMatcherC1EPKci(qthis, arg0, arg1)};
    let rsthis = QByteArrayMatcher{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QByteArrayMatcher::NewQByteArrayMatcher(const QByteArray & pattern);
impl<'a> /*trait*/ QByteArrayMatcher_NewQByteArrayMatcher for (&'a  QByteArray) {
  fn NewQByteArrayMatcher(self) -> QByteArrayMatcher {
    let qthis: *mut c_void = unsafe{calloc(1, 1040)};
    // unsafe{_ZN17QByteArrayMatcherC1ERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN17QByteArrayMatcherC1ERK10QByteArray(qthis, arg0)};
    let rsthis = QByteArrayMatcher{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: int QByteArrayMatcher::indexIn(const QByteArray & ba, int from);
impl<'a> /*trait*/ QByteArrayMatcher_indexIn for (&'a  QByteArray, i32) {
  fn indexIn(self, this: &mut QByteArrayMatcher) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 1040)};
    // unsafe{_ZNK17QByteArrayMatcher7indexInERK10QByteArrayi()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK17QByteArrayMatcher7indexInERK10QByteArrayi(arg0, arg1)};
    return 1;
  }
}

// proto: void QByteArrayMatcher::NewQByteArrayMatcher(const QByteArrayMatcher & other);
impl<'a> /*trait*/ QByteArrayMatcher_NewQByteArrayMatcher for (&'a  QByteArrayMatcher) {
  fn NewQByteArrayMatcher(self) -> QByteArrayMatcher {
    let qthis: *mut c_void = unsafe{calloc(1, 1040)};
    // unsafe{_ZN17QByteArrayMatcherC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN17QByteArrayMatcherC1ERKS_(qthis, arg0)};
    let rsthis = QByteArrayMatcher{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QByteArrayMatcher {
  pub fn FreeQByteArrayMatcher<T: QByteArrayMatcher_FreeQByteArrayMatcher>(&mut self, value: T) -> i32 {
    value.FreeQByteArrayMatcher(self);
    return 1;
  }
}

pub trait QByteArrayMatcher_FreeQByteArrayMatcher {
  fn FreeQByteArrayMatcher(self, this: &mut QByteArrayMatcher) -> i32;
}

// proto: void QByteArrayMatcher::FreeQByteArrayMatcher();
impl<'a> /*trait*/ QByteArrayMatcher_FreeQByteArrayMatcher for () {
  fn FreeQByteArrayMatcher(self, this: &mut QByteArrayMatcher) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 1040)};
    // unsafe{_ZN17QByteArrayMatcherD0Ev()};
    unsafe {_ZN17QByteArrayMatcherD0Ev()};
    return 1;
  }
}

