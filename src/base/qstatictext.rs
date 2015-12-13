// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qtransform::QTransform;
use super::qfont::QFont;
use super::qtextoption::QTextOption;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QStaticText::NewQStaticText(const QString & text);
  fn _ZN11QStaticTextC1ERK7QString(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: QSizeF QStaticText::size();
  fn _ZNK11QStaticText4sizeEv() -> i32;
  // proto: QString QStaticText::text();
  fn _ZNK11QStaticText4textEv() -> i32;
  // proto: void QStaticText::FreeQStaticText();
  fn _ZN11QStaticTextD0Ev() -> i32;
  // proto: void QStaticText::setText(const QString & text);
  fn _ZN11QStaticText7setTextERK7QString(arg0: *const c_void) -> i32;
  // proto: void QStaticText::NewQStaticText();
  fn _ZN11QStaticTextC1Ev(qthis: *mut c_void) -> i32;
  // proto: void QStaticText::prepare(const QTransform & matrix, const QFont & font);
  fn _ZN11QStaticText7prepareERK10QTransformRK5QFont(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QStaticText::setTextOption(const QTextOption & textOption);
  fn _ZN11QStaticText13setTextOptionERK11QTextOption(arg0: *const c_void) -> i32;
  // proto: void QStaticText::setTextWidth(qreal textWidth);
  fn _ZN11QStaticText12setTextWidthEd(arg0: c_double) -> i32;
  // proto: double QStaticText::textWidth();
  fn _ZNK11QStaticText9textWidthEv() -> i32;
  // proto: void QStaticText::swap(QStaticText & other);
  fn _ZN11QStaticText4swapERS_(arg0: *mut c_void) -> i32;
  // proto: QTextOption QStaticText::textOption();
  fn _ZNK11QStaticText10textOptionEv() -> i32;
  // proto: void QStaticText::NewQStaticText(const QStaticText & other);
  fn _ZN11QStaticTextC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QStaticText)=1
pub struct QStaticText {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStaticText {
  pub fn NewQStaticText<T: QStaticText_NewQStaticText>(value: T) -> QStaticText {
    let rsthis = value.NewQStaticText();
    return rsthis;
    // return 1;
  }
}

pub trait QStaticText_NewQStaticText {
  fn NewQStaticText(self) -> QStaticText;
}

// proto: void QStaticText::NewQStaticText(const QString & text);
impl<'a> /*trait*/ QStaticText_NewQStaticText for (&'a  QString) {
  fn NewQStaticText(self) -> QStaticText {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticTextC1ERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QStaticTextC1ERK7QString(qthis, arg0)};
    let rsthis = QStaticText{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStaticText {
  pub fn size<T: QStaticText_size>(&mut self, value: T) -> i32 {
    value.size(self);
    return 1;
  }
}

pub trait QStaticText_size {
  fn size(self, this: &mut QStaticText) -> i32;
}

// proto: QSizeF QStaticText::size();
impl<'a> /*trait*/ QStaticText_size for () {
  fn size(self, this: &mut QStaticText) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStaticText4sizeEv()};
    unsafe {_ZNK11QStaticText4sizeEv()};
    return 1;
  }
}

impl /*struct*/ QStaticText {
  pub fn text<T: QStaticText_text>(&mut self, value: T) -> i32 {
    value.text(self);
    return 1;
  }
}

pub trait QStaticText_text {
  fn text(self, this: &mut QStaticText) -> i32;
}

// proto: QString QStaticText::text();
impl<'a> /*trait*/ QStaticText_text for () {
  fn text(self, this: &mut QStaticText) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStaticText4textEv()};
    unsafe {_ZNK11QStaticText4textEv()};
    return 1;
  }
}

impl /*struct*/ QStaticText {
  pub fn FreeQStaticText<T: QStaticText_FreeQStaticText>(&mut self, value: T) -> i32 {
    value.FreeQStaticText(self);
    return 1;
  }
}

pub trait QStaticText_FreeQStaticText {
  fn FreeQStaticText(self, this: &mut QStaticText) -> i32;
}

// proto: void QStaticText::FreeQStaticText();
impl<'a> /*trait*/ QStaticText_FreeQStaticText for () {
  fn FreeQStaticText(self, this: &mut QStaticText) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticTextD0Ev()};
    unsafe {_ZN11QStaticTextD0Ev()};
    return 1;
  }
}

impl /*struct*/ QStaticText {
  pub fn setText<T: QStaticText_setText>(&mut self, value: T) -> i32 {
    value.setText(self);
    return 1;
  }
}

pub trait QStaticText_setText {
  fn setText(self, this: &mut QStaticText) -> i32;
}

// proto: void QStaticText::setText(const QString & text);
impl<'a> /*trait*/ QStaticText_setText for (&'a  QString) {
  fn setText(self, this: &mut QStaticText) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticText7setTextERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QStaticText7setTextERK7QString(arg0)};
    return 1;
  }
}

// proto: void QStaticText::NewQStaticText();
impl<'a> /*trait*/ QStaticText_NewQStaticText for () {
  fn NewQStaticText(self) -> QStaticText {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticTextC1Ev()};
    unsafe {_ZN11QStaticTextC1Ev(qthis)};
    let rsthis = QStaticText{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStaticText {
  pub fn prepare<T: QStaticText_prepare>(&mut self, value: T) -> i32 {
    value.prepare(self);
    return 1;
  }
}

pub trait QStaticText_prepare {
  fn prepare(self, this: &mut QStaticText) -> i32;
}

// proto: void QStaticText::prepare(const QTransform & matrix, const QFont & font);
impl<'a> /*trait*/ QStaticText_prepare for (&'a  QTransform, &'a  QFont) {
  fn prepare(self, this: &mut QStaticText) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticText7prepareERK10QTransformRK5QFont()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN11QStaticText7prepareERK10QTransformRK5QFont(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QStaticText {
  pub fn setTextOption<T: QStaticText_setTextOption>(&mut self, value: T) -> i32 {
    value.setTextOption(self);
    return 1;
  }
}

pub trait QStaticText_setTextOption {
  fn setTextOption(self, this: &mut QStaticText) -> i32;
}

// proto: void QStaticText::setTextOption(const QTextOption & textOption);
impl<'a> /*trait*/ QStaticText_setTextOption for (&'a  QTextOption) {
  fn setTextOption(self, this: &mut QStaticText) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticText13setTextOptionERK11QTextOption()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QStaticText13setTextOptionERK11QTextOption(arg0)};
    return 1;
  }
}

impl /*struct*/ QStaticText {
  pub fn setTextWidth<T: QStaticText_setTextWidth>(&mut self, value: T) -> i32 {
    value.setTextWidth(self);
    return 1;
  }
}

pub trait QStaticText_setTextWidth {
  fn setTextWidth(self, this: &mut QStaticText) -> i32;
}

// proto: void QStaticText::setTextWidth(qreal textWidth);
impl<'a> /*trait*/ QStaticText_setTextWidth for (f64) {
  fn setTextWidth(self, this: &mut QStaticText) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticText12setTextWidthEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN11QStaticText12setTextWidthEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QStaticText {
  pub fn textWidth<T: QStaticText_textWidth>(&mut self, value: T) -> i32 {
    value.textWidth(self);
    return 1;
  }
}

pub trait QStaticText_textWidth {
  fn textWidth(self, this: &mut QStaticText) -> i32;
}

// proto: double QStaticText::textWidth();
impl<'a> /*trait*/ QStaticText_textWidth for () {
  fn textWidth(self, this: &mut QStaticText) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStaticText9textWidthEv()};
    unsafe {_ZNK11QStaticText9textWidthEv()};
    return 1;
  }
}

impl /*struct*/ QStaticText {
  pub fn swap<T: QStaticText_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QStaticText_swap {
  fn swap(self, this: &mut QStaticText) -> i32;
}

// proto: void QStaticText::swap(QStaticText & other);
impl<'a> /*trait*/ QStaticText_swap for (&'a mut QStaticText) {
  fn swap(self, this: &mut QStaticText) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticText4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QStaticText4swapERS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QStaticText {
  pub fn textOption<T: QStaticText_textOption>(&mut self, value: T) -> i32 {
    value.textOption(self);
    return 1;
  }
}

pub trait QStaticText_textOption {
  fn textOption(self, this: &mut QStaticText) -> i32;
}

// proto: QTextOption QStaticText::textOption();
impl<'a> /*trait*/ QStaticText_textOption for () {
  fn textOption(self, this: &mut QStaticText) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStaticText10textOptionEv()};
    unsafe {_ZNK11QStaticText10textOptionEv()};
    return 1;
  }
}

// proto: void QStaticText::NewQStaticText(const QStaticText & other);
impl<'a> /*trait*/ QStaticText_NewQStaticText for (&'a  QStaticText) {
  fn NewQStaticText(self) -> QStaticText {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticTextC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QStaticTextC1ERKS_(qthis, arg0)};
    let rsthis = QStaticText{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

