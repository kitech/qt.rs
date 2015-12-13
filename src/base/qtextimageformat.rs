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
  // proto: void QTextImageFormat::NewQTextImageFormat();
  fn _ZN16QTextImageFormatC1Ev(qthis: *mut c_void) -> i32;
  // proto: bool QTextImageFormat::isValid();
  fn _ZNK16QTextImageFormat7isValidEv() -> i32;
  // proto: double QTextImageFormat::width();
  fn _ZNK16QTextImageFormat5widthEv() -> i32;
  // proto: void QTextImageFormat::NewQTextImageFormat(const QTextFormat & format);
  fn _ZN16QTextImageFormatC1ERK11QTextFormat(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QTextImageFormat::setHeight(qreal height);
  fn _ZN16QTextImageFormat9setHeightEd(arg0: c_double) -> i32;
  // proto: void QTextImageFormat::setWidth(qreal width);
  fn _ZN16QTextImageFormat8setWidthEd(arg0: c_double) -> i32;
  // proto: void QTextImageFormat::setName(const QString & name);
  fn _ZN16QTextImageFormat7setNameERK7QString(arg0: *const c_void) -> i32;
  // proto: QString QTextImageFormat::name();
  fn _ZNK16QTextImageFormat4nameEv() -> i32;
  // proto: double QTextImageFormat::height();
  fn _ZNK16QTextImageFormat6heightEv() -> i32;
}

// body block begin
// class sizeof(QTextImageFormat)=1
pub struct QTextImageFormat {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextImageFormat {
  pub fn NewQTextImageFormat<T: QTextImageFormat_NewQTextImageFormat>(value: T) -> QTextImageFormat {
    let rsthis = value.NewQTextImageFormat();
    return rsthis;
    // return 1;
  }
}

pub trait QTextImageFormat_NewQTextImageFormat {
  fn NewQTextImageFormat(self) -> QTextImageFormat;
}

// proto: void QTextImageFormat::NewQTextImageFormat();
impl<'a> /*trait*/ QTextImageFormat_NewQTextImageFormat for () {
  fn NewQTextImageFormat(self) -> QTextImageFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextImageFormatC1Ev()};
    unsafe {_ZN16QTextImageFormatC1Ev(qthis)};
    let rsthis = QTextImageFormat{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextImageFormat {
  pub fn isValid<T: QTextImageFormat_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QTextImageFormat_isValid {
  fn isValid(self, this: &mut QTextImageFormat) -> i32;
}

// proto: bool QTextImageFormat::isValid();
impl<'a> /*trait*/ QTextImageFormat_isValid for () {
  fn isValid(self, this: &mut QTextImageFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextImageFormat7isValidEv()};
    unsafe {_ZNK16QTextImageFormat7isValidEv()};
    return 1;
  }
}

impl /*struct*/ QTextImageFormat {
  pub fn width<T: QTextImageFormat_width>(&mut self, value: T) -> i32 {
    value.width(self);
    return 1;
  }
}

pub trait QTextImageFormat_width {
  fn width(self, this: &mut QTextImageFormat) -> i32;
}

// proto: double QTextImageFormat::width();
impl<'a> /*trait*/ QTextImageFormat_width for () {
  fn width(self, this: &mut QTextImageFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextImageFormat5widthEv()};
    unsafe {_ZNK16QTextImageFormat5widthEv()};
    return 1;
  }
}

// proto: void QTextImageFormat::NewQTextImageFormat(const QTextFormat & format);
impl<'a> /*trait*/ QTextImageFormat_NewQTextImageFormat for (&'a  QTextFormat) {
  fn NewQTextImageFormat(self) -> QTextImageFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextImageFormatC1ERK11QTextFormat()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QTextImageFormatC1ERK11QTextFormat(qthis, arg0)};
    let rsthis = QTextImageFormat{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextImageFormat {
  pub fn setHeight<T: QTextImageFormat_setHeight>(&mut self, value: T) -> i32 {
    value.setHeight(self);
    return 1;
  }
}

pub trait QTextImageFormat_setHeight {
  fn setHeight(self, this: &mut QTextImageFormat) -> i32;
}

// proto: void QTextImageFormat::setHeight(qreal height);
impl<'a> /*trait*/ QTextImageFormat_setHeight for (f64) {
  fn setHeight(self, this: &mut QTextImageFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextImageFormat9setHeightEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN16QTextImageFormat9setHeightEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextImageFormat {
  pub fn setWidth<T: QTextImageFormat_setWidth>(&mut self, value: T) -> i32 {
    value.setWidth(self);
    return 1;
  }
}

pub trait QTextImageFormat_setWidth {
  fn setWidth(self, this: &mut QTextImageFormat) -> i32;
}

// proto: void QTextImageFormat::setWidth(qreal width);
impl<'a> /*trait*/ QTextImageFormat_setWidth for (f64) {
  fn setWidth(self, this: &mut QTextImageFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextImageFormat8setWidthEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN16QTextImageFormat8setWidthEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextImageFormat {
  pub fn setName<T: QTextImageFormat_setName>(&mut self, value: T) -> i32 {
    value.setName(self);
    return 1;
  }
}

pub trait QTextImageFormat_setName {
  fn setName(self, this: &mut QTextImageFormat) -> i32;
}

// proto: void QTextImageFormat::setName(const QString & name);
impl<'a> /*trait*/ QTextImageFormat_setName for (&'a  QString) {
  fn setName(self, this: &mut QTextImageFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextImageFormat7setNameERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QTextImageFormat7setNameERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextImageFormat {
  pub fn name<T: QTextImageFormat_name>(&mut self, value: T) -> i32 {
    value.name(self);
    return 1;
  }
}

pub trait QTextImageFormat_name {
  fn name(self, this: &mut QTextImageFormat) -> i32;
}

// proto: QString QTextImageFormat::name();
impl<'a> /*trait*/ QTextImageFormat_name for () {
  fn name(self, this: &mut QTextImageFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextImageFormat4nameEv()};
    unsafe {_ZNK16QTextImageFormat4nameEv()};
    return 1;
  }
}

impl /*struct*/ QTextImageFormat {
  pub fn height<T: QTextImageFormat_height>(&mut self, value: T) -> i32 {
    value.height(self);
    return 1;
  }
}

pub trait QTextImageFormat_height {
  fn height(self, this: &mut QTextImageFormat) -> i32;
}

// proto: double QTextImageFormat::height();
impl<'a> /*trait*/ QTextImageFormat_height for () {
  fn height(self, this: &mut QTextImageFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextImageFormat6heightEv()};
    unsafe {_ZNK16QTextImageFormat6heightEv()};
    return 1;
  }
}

