// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qtextformat::QTextFormat;
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: int QTextListFormat::indent();
  fn _ZNK15QTextListFormat6indentEv() -> i32;
  // proto: void QTextListFormat::NewQTextListFormat(const QTextFormat & fmt);
  fn _ZN15QTextListFormatC1ERK11QTextFormat(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QTextListFormat::setIndent(int indent);
  fn _ZN15QTextListFormat9setIndentEi(arg0: c_int) -> i32;
  // proto: QString QTextListFormat::numberSuffix();
  fn _ZNK15QTextListFormat12numberSuffixEv() -> i32;
  // proto: void QTextListFormat::NewQTextListFormat();
  fn _ZN15QTextListFormatC1Ev(qthis: *mut c_void) -> i32;
  // proto: QString QTextListFormat::numberPrefix();
  fn _ZNK15QTextListFormat12numberPrefixEv() -> i32;
  // proto: bool QTextListFormat::isValid();
  fn _ZNK15QTextListFormat7isValidEv() -> i32;
  // proto: void QTextListFormat::setNumberSuffix(const QString & numberSuffix);
  fn _ZN15QTextListFormat15setNumberSuffixERK7QString(arg0: *const c_void) -> i32;
  // proto: void QTextListFormat::setNumberPrefix(const QString & numberPrefix);
  fn _ZN15QTextListFormat15setNumberPrefixERK7QString(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QTextListFormat)=1
pub struct QTextListFormat {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextListFormat {
  pub fn indent<T: QTextListFormat_indent>(&mut self, value: T) -> i32 {
    value.indent(self);
    return 1;
  }
}

pub trait QTextListFormat_indent {
  fn indent(self, this: &mut QTextListFormat) -> i32;
}

// proto: int QTextListFormat::indent();
impl<'a> /*trait*/ QTextListFormat_indent for () {
  fn indent(self, this: &mut QTextListFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextListFormat6indentEv()};
    unsafe {_ZNK15QTextListFormat6indentEv()};
    return 1;
  }
}

impl /*struct*/ QTextListFormat {
  pub fn NewQTextListFormat<T: QTextListFormat_NewQTextListFormat>(value: T) -> QTextListFormat {
    let rsthis = value.NewQTextListFormat();
    return rsthis;
    // return 1;
  }
}

pub trait QTextListFormat_NewQTextListFormat {
  fn NewQTextListFormat(self) -> QTextListFormat;
}

// proto: void QTextListFormat::NewQTextListFormat(const QTextFormat & fmt);
impl<'a> /*trait*/ QTextListFormat_NewQTextListFormat for (&'a  QTextFormat) {
  fn NewQTextListFormat(self) -> QTextListFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextListFormatC1ERK11QTextFormat()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QTextListFormatC1ERK11QTextFormat(qthis, arg0)};
    let rsthis = QTextListFormat{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextListFormat {
  pub fn setIndent<T: QTextListFormat_setIndent>(&mut self, value: T) -> i32 {
    value.setIndent(self);
    return 1;
  }
}

pub trait QTextListFormat_setIndent {
  fn setIndent(self, this: &mut QTextListFormat) -> i32;
}

// proto: void QTextListFormat::setIndent(int indent);
impl<'a> /*trait*/ QTextListFormat_setIndent for (i32) {
  fn setIndent(self, this: &mut QTextListFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextListFormat9setIndentEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN15QTextListFormat9setIndentEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextListFormat {
  pub fn numberSuffix<T: QTextListFormat_numberSuffix>(&mut self, value: T) -> i32 {
    value.numberSuffix(self);
    return 1;
  }
}

pub trait QTextListFormat_numberSuffix {
  fn numberSuffix(self, this: &mut QTextListFormat) -> i32;
}

// proto: QString QTextListFormat::numberSuffix();
impl<'a> /*trait*/ QTextListFormat_numberSuffix for () {
  fn numberSuffix(self, this: &mut QTextListFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextListFormat12numberSuffixEv()};
    unsafe {_ZNK15QTextListFormat12numberSuffixEv()};
    return 1;
  }
}

// proto: void QTextListFormat::NewQTextListFormat();
impl<'a> /*trait*/ QTextListFormat_NewQTextListFormat for () {
  fn NewQTextListFormat(self) -> QTextListFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextListFormatC1Ev()};
    unsafe {_ZN15QTextListFormatC1Ev(qthis)};
    let rsthis = QTextListFormat{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextListFormat {
  pub fn numberPrefix<T: QTextListFormat_numberPrefix>(&mut self, value: T) -> i32 {
    value.numberPrefix(self);
    return 1;
  }
}

pub trait QTextListFormat_numberPrefix {
  fn numberPrefix(self, this: &mut QTextListFormat) -> i32;
}

// proto: QString QTextListFormat::numberPrefix();
impl<'a> /*trait*/ QTextListFormat_numberPrefix for () {
  fn numberPrefix(self, this: &mut QTextListFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextListFormat12numberPrefixEv()};
    unsafe {_ZNK15QTextListFormat12numberPrefixEv()};
    return 1;
  }
}

impl /*struct*/ QTextListFormat {
  pub fn isValid<T: QTextListFormat_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QTextListFormat_isValid {
  fn isValid(self, this: &mut QTextListFormat) -> i32;
}

// proto: bool QTextListFormat::isValid();
impl<'a> /*trait*/ QTextListFormat_isValid for () {
  fn isValid(self, this: &mut QTextListFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextListFormat7isValidEv()};
    unsafe {_ZNK15QTextListFormat7isValidEv()};
    return 1;
  }
}

impl /*struct*/ QTextListFormat {
  pub fn setNumberSuffix<T: QTextListFormat_setNumberSuffix>(&mut self, value: T) -> i32 {
    value.setNumberSuffix(self);
    return 1;
  }
}

pub trait QTextListFormat_setNumberSuffix {
  fn setNumberSuffix(self, this: &mut QTextListFormat) -> i32;
}

// proto: void QTextListFormat::setNumberSuffix(const QString & numberSuffix);
impl<'a> /*trait*/ QTextListFormat_setNumberSuffix for (&'a  QString) {
  fn setNumberSuffix(self, this: &mut QTextListFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextListFormat15setNumberSuffixERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QTextListFormat15setNumberSuffixERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextListFormat {
  pub fn setNumberPrefix<T: QTextListFormat_setNumberPrefix>(&mut self, value: T) -> i32 {
    value.setNumberPrefix(self);
    return 1;
  }
}

pub trait QTextListFormat_setNumberPrefix {
  fn setNumberPrefix(self, this: &mut QTextListFormat) -> i32;
}

// proto: void QTextListFormat::setNumberPrefix(const QString & numberPrefix);
impl<'a> /*trait*/ QTextListFormat_setNumberPrefix for (&'a  QString) {
  fn setNumberPrefix(self, this: &mut QTextListFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextListFormat15setNumberPrefixERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QTextListFormat15setNumberPrefixERK7QString(arg0)};
    return 1;
  }
}

