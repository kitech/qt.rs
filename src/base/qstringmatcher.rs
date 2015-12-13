// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qchar::QChar;
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN14QStringMatcherC1Ev(qthis: *mut c_void) -> i32;
  fn _ZN14QStringMatcherC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK14QStringMatcher7indexInEPK5QCharii(arg0: *const c_void, arg1: c_int, arg2: c_int) -> i32;
  fn _ZN14QStringMatcher10setPatternERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK14QStringMatcher7patternEv() -> i32;
  fn _ZN14QStringMatcherD0Ev() -> i32;
  fn _ZNK14QStringMatcher7indexInERK7QStringi(arg0: *const c_void, arg1: c_int) -> i32;
}

// body block begin
// class sizeof(QStringMatcher)=1048
pub struct QStringMatcher {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStringMatcher {
  pub fn NewQStringMatcher<T: QStringMatcher_NewQStringMatcher>(value: T) -> QStringMatcher {
    let rsthis = value.NewQStringMatcher();
    return rsthis;
    // return 1;
  }
}

pub trait QStringMatcher_NewQStringMatcher {
  fn NewQStringMatcher(self) -> QStringMatcher;
}

// proto: void QStringMatcher::NewQStringMatcher();
impl<'a> /*trait*/ QStringMatcher_NewQStringMatcher for () {
  fn NewQStringMatcher(self) -> QStringMatcher {
    let qthis: *mut c_void = unsafe{calloc(1, 1048)};
    // unsafe{_ZN14QStringMatcherC1Ev()};
    unsafe {_ZN14QStringMatcherC1Ev(qthis)};
    let rsthis = QStringMatcher{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QStringMatcher::NewQStringMatcher(const QStringMatcher & other);
impl<'a> /*trait*/ QStringMatcher_NewQStringMatcher for (&'a  QStringMatcher) {
  fn NewQStringMatcher(self) -> QStringMatcher {
    let qthis: *mut c_void = unsafe{calloc(1, 1048)};
    // unsafe{_ZN14QStringMatcherC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QStringMatcherC1ERKS_(qthis, arg0)};
    let rsthis = QStringMatcher{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStringMatcher {
  pub fn indexIn<T: QStringMatcher_indexIn>(&mut self, value: T) -> i32 {
    value.indexIn(self);
    return 1;
  }
}

pub trait QStringMatcher_indexIn {
  fn indexIn(self, this: &mut QStringMatcher) -> i32;
}

// proto: int QStringMatcher::indexIn(const QChar * str, int length, int from);
impl<'a> /*trait*/ QStringMatcher_indexIn for (&'a  QChar, i32, i32) {
  fn indexIn(self, this: &mut QStringMatcher) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 1048)};
    // unsafe{_ZNK14QStringMatcher7indexInEPK5QCharii()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZNK14QStringMatcher7indexInEPK5QCharii(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QStringMatcher {
  pub fn setPattern<T: QStringMatcher_setPattern>(&mut self, value: T) -> i32 {
    value.setPattern(self);
    return 1;
  }
}

pub trait QStringMatcher_setPattern {
  fn setPattern(self, this: &mut QStringMatcher) -> i32;
}

// proto: void QStringMatcher::setPattern(const QString & pattern);
impl<'a> /*trait*/ QStringMatcher_setPattern for (&'a  QString) {
  fn setPattern(self, this: &mut QStringMatcher) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 1048)};
    // unsafe{_ZN14QStringMatcher10setPatternERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QStringMatcher10setPatternERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QStringMatcher {
  pub fn pattern<T: QStringMatcher_pattern>(&mut self, value: T) -> i32 {
    value.pattern(self);
    return 1;
  }
}

pub trait QStringMatcher_pattern {
  fn pattern(self, this: &mut QStringMatcher) -> i32;
}

// proto: QString QStringMatcher::pattern();
impl<'a> /*trait*/ QStringMatcher_pattern for () {
  fn pattern(self, this: &mut QStringMatcher) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 1048)};
    // unsafe{_ZNK14QStringMatcher7patternEv()};
    unsafe {_ZNK14QStringMatcher7patternEv()};
    return 1;
  }
}

impl /*struct*/ QStringMatcher {
  pub fn FreeQStringMatcher<T: QStringMatcher_FreeQStringMatcher>(&mut self, value: T) -> i32 {
    value.FreeQStringMatcher(self);
    return 1;
  }
}

pub trait QStringMatcher_FreeQStringMatcher {
  fn FreeQStringMatcher(self, this: &mut QStringMatcher) -> i32;
}

// proto: void QStringMatcher::FreeQStringMatcher();
impl<'a> /*trait*/ QStringMatcher_FreeQStringMatcher for () {
  fn FreeQStringMatcher(self, this: &mut QStringMatcher) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 1048)};
    // unsafe{_ZN14QStringMatcherD0Ev()};
    unsafe {_ZN14QStringMatcherD0Ev()};
    return 1;
  }
}

// proto: int QStringMatcher::indexIn(const QString & str, int from);
impl<'a> /*trait*/ QStringMatcher_indexIn for (&'a  QString, i32) {
  fn indexIn(self, this: &mut QStringMatcher) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 1048)};
    // unsafe{_ZNK14QStringMatcher7indexInERK7QStringi()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK14QStringMatcher7indexInERK7QStringi(arg0, arg1)};
    return 1;
  }
}

