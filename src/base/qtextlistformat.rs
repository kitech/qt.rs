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
  // proto:  int QTextListFormat::indent();
  fn _ZNK15QTextListFormat6indentEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextListFormat::NewQTextListFormat(const QTextFormat & fmt);
  fn _ZN15QTextListFormatC1ERK11QTextFormat(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextListFormat::setIndent(int indent);
  fn _ZN15QTextListFormat9setIndentEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QString QTextListFormat::numberSuffix();
  fn _ZNK15QTextListFormat12numberSuffixEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextListFormat::NewQTextListFormat();
  fn _ZN15QTextListFormatC1Ev(qthis: *mut c_void) ;
  // proto:  QString QTextListFormat::numberPrefix();
  fn _ZNK15QTextListFormat12numberPrefixEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QTextListFormat::isValid();
  fn _ZNK15QTextListFormat7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTextListFormat::setNumberSuffix(const QString & numberSuffix);
  fn _ZN15QTextListFormat15setNumberSuffixERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextListFormat::setNumberPrefix(const QString & numberPrefix);
  fn _ZN15QTextListFormat15setNumberPrefixERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QTextListFormat)=1
pub struct QTextListFormat {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextListFormat {
  pub fn indent<RetType, T: QTextListFormat_indent<RetType>>(&mut self, value: T) -> RetType {
    return value.indent(self);
    // return 1;
  }
}

pub trait QTextListFormat_indent<RetType> {
  fn indent(self, rsthis: &mut QTextListFormat) -> RetType;
}

// proto:  int QTextListFormat::indent();
impl<'a> /*trait*/ QTextListFormat_indent<i32> for () {
  fn indent(self, rsthis: &mut QTextListFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextListFormat6indentEv()};
    let mut ret = unsafe {_ZNK15QTextListFormat6indentEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QTextListFormatC1ERK11QTextFormat(qthis, arg0)};
    let rsthis = QTextListFormat{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextListFormat {
  pub fn setIndent<RetType, T: QTextListFormat_setIndent<RetType>>(&mut self, value: T) -> RetType {
    return value.setIndent(self);
    // return 1;
  }
}

pub trait QTextListFormat_setIndent<RetType> {
  fn setIndent(self, rsthis: &mut QTextListFormat) -> RetType;
}

// proto:  void QTextListFormat::setIndent(int indent);
impl<'a> /*trait*/ QTextListFormat_setIndent<()> for (i32) {
  fn setIndent(self, rsthis: &mut QTextListFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextListFormat9setIndentEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QTextListFormat9setIndentEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextListFormat {
  pub fn numberSuffix<RetType, T: QTextListFormat_numberSuffix<RetType>>(&mut self, value: T) -> RetType {
    return value.numberSuffix(self);
    // return 1;
  }
}

pub trait QTextListFormat_numberSuffix<RetType> {
  fn numberSuffix(self, rsthis: &mut QTextListFormat) -> RetType;
}

// proto:  QString QTextListFormat::numberSuffix();
impl<'a> /*trait*/ QTextListFormat_numberSuffix<QString> for () {
  fn numberSuffix(self, rsthis: &mut QTextListFormat) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextListFormat12numberSuffixEv()};
    let mut ret = unsafe {_ZNK15QTextListFormat12numberSuffixEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
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
  pub fn numberPrefix<RetType, T: QTextListFormat_numberPrefix<RetType>>(&mut self, value: T) -> RetType {
    return value.numberPrefix(self);
    // return 1;
  }
}

pub trait QTextListFormat_numberPrefix<RetType> {
  fn numberPrefix(self, rsthis: &mut QTextListFormat) -> RetType;
}

// proto:  QString QTextListFormat::numberPrefix();
impl<'a> /*trait*/ QTextListFormat_numberPrefix<QString> for () {
  fn numberPrefix(self, rsthis: &mut QTextListFormat) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextListFormat12numberPrefixEv()};
    let mut ret = unsafe {_ZNK15QTextListFormat12numberPrefixEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextListFormat {
  pub fn isValid<RetType, T: QTextListFormat_isValid<RetType>>(&mut self, value: T) -> RetType {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QTextListFormat_isValid<RetType> {
  fn isValid(self, rsthis: &mut QTextListFormat) -> RetType;
}

// proto:  bool QTextListFormat::isValid();
impl<'a> /*trait*/ QTextListFormat_isValid<i8> for () {
  fn isValid(self, rsthis: &mut QTextListFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextListFormat7isValidEv()};
    let mut ret = unsafe {_ZNK15QTextListFormat7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextListFormat {
  pub fn setNumberSuffix<RetType, T: QTextListFormat_setNumberSuffix<RetType>>(&mut self, value: T) -> RetType {
    return value.setNumberSuffix(self);
    // return 1;
  }
}

pub trait QTextListFormat_setNumberSuffix<RetType> {
  fn setNumberSuffix(self, rsthis: &mut QTextListFormat) -> RetType;
}

// proto:  void QTextListFormat::setNumberSuffix(const QString & numberSuffix);
impl<'a> /*trait*/ QTextListFormat_setNumberSuffix<()> for (&'a  QString) {
  fn setNumberSuffix(self, rsthis: &mut QTextListFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextListFormat15setNumberSuffixERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QTextListFormat15setNumberSuffixERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextListFormat {
  pub fn setNumberPrefix<RetType, T: QTextListFormat_setNumberPrefix<RetType>>(&mut self, value: T) -> RetType {
    return value.setNumberPrefix(self);
    // return 1;
  }
}

pub trait QTextListFormat_setNumberPrefix<RetType> {
  fn setNumberPrefix(self, rsthis: &mut QTextListFormat) -> RetType;
}

// proto:  void QTextListFormat::setNumberPrefix(const QString & numberPrefix);
impl<'a> /*trait*/ QTextListFormat_setNumberPrefix<()> for (&'a  QString) {
  fn setNumberPrefix(self, rsthis: &mut QTextListFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextListFormat15setNumberPrefixERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QTextListFormat15setNumberPrefixERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

