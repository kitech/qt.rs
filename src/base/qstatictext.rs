// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qsizef::QSizeF;
use super::qtransform::QTransform;
use super::qfont::QFont;
use super::qtextoption::QTextOption;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QStaticText::NewQStaticText(const QString & text);
  fn _ZN11QStaticTextC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QSizeF QStaticText::size();
  fn _ZNK11QStaticText4sizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QStaticText::text();
  fn _ZNK11QStaticText4textEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QStaticText::FreeQStaticText();
  fn _ZN11QStaticTextD0Ev(qthis: *mut c_void) ;
  // proto:  void QStaticText::setText(const QString & text);
  fn _ZN11QStaticText7setTextERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QStaticText::NewQStaticText();
  fn _ZN11QStaticTextC1Ev(qthis: *mut c_void) ;
  // proto:  void QStaticText::prepare(const QTransform & matrix, const QFont & font);
  fn _ZN11QStaticText7prepareERK10QTransformRK5QFont(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QStaticText::setTextOption(const QTextOption & textOption);
  fn _ZN11QStaticText13setTextOptionERK11QTextOption(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QStaticText::setTextWidth(qreal textWidth);
  fn _ZN11QStaticText12setTextWidthEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  double QStaticText::textWidth();
  fn _ZNK11QStaticText9textWidthEv(qthis: *mut c_void) -> c_double;
  // proto:  void QStaticText::swap(QStaticText & other);
  fn _ZN11QStaticText4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QTextOption QStaticText::textOption();
  fn _ZNK11QStaticText10textOptionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QStaticText::NewQStaticText(const QStaticText & other);
  fn _ZN11QStaticTextC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QStaticTextC1ERK7QString(qthis, arg0)};
    let rsthis = QStaticText{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStaticText {
  pub fn size<T: QStaticText_size>(&mut self, value: T) -> QSizeF {
    return value.size(self);
    // return 1;
  }
}

pub trait QStaticText_size {
  fn size(self, rsthis: &mut QStaticText) -> QSizeF;
}

// proto:  QSizeF QStaticText::size();
impl<'a> /*trait*/ QStaticText_size for () {
  fn size(self, rsthis: &mut QStaticText) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStaticText4sizeEv()};
    let mut ret = unsafe {_ZNK11QStaticText4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStaticText {
  pub fn text<T: QStaticText_text>(&mut self, value: T) -> QString {
    return value.text(self);
    // return 1;
  }
}

pub trait QStaticText_text {
  fn text(self, rsthis: &mut QStaticText) -> QString;
}

// proto:  QString QStaticText::text();
impl<'a> /*trait*/ QStaticText_text for () {
  fn text(self, rsthis: &mut QStaticText) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStaticText4textEv()};
    let mut ret = unsafe {_ZNK11QStaticText4textEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStaticText {
  pub fn FreeQStaticText<T: QStaticText_FreeQStaticText>(&mut self, value: T)  {
     value.FreeQStaticText(self);
    // return 1;
  }
}

pub trait QStaticText_FreeQStaticText {
  fn FreeQStaticText(self, rsthis: &mut QStaticText) ;
}

// proto:  void QStaticText::FreeQStaticText();
impl<'a> /*trait*/ QStaticText_FreeQStaticText for () {
  fn FreeQStaticText(self, rsthis: &mut QStaticText)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticTextD0Ev()};
     unsafe {_ZN11QStaticTextD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QStaticText {
  pub fn setText<T: QStaticText_setText>(&mut self, value: T)  {
     value.setText(self);
    // return 1;
  }
}

pub trait QStaticText_setText {
  fn setText(self, rsthis: &mut QStaticText) ;
}

// proto:  void QStaticText::setText(const QString & text);
impl<'a> /*trait*/ QStaticText_setText for (&'a  QString) {
  fn setText(self, rsthis: &mut QStaticText)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticText7setTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QStaticText7setTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
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
  pub fn prepare<T: QStaticText_prepare>(&mut self, value: T)  {
     value.prepare(self);
    // return 1;
  }
}

pub trait QStaticText_prepare {
  fn prepare(self, rsthis: &mut QStaticText) ;
}

// proto:  void QStaticText::prepare(const QTransform & matrix, const QFont & font);
impl<'a> /*trait*/ QStaticText_prepare for (&'a  QTransform, &'a  QFont) {
  fn prepare(self, rsthis: &mut QStaticText)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticText7prepareERK10QTransformRK5QFont()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QStaticText7prepareERK10QTransformRK5QFont(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QStaticText {
  pub fn setTextOption<T: QStaticText_setTextOption>(&mut self, value: T)  {
     value.setTextOption(self);
    // return 1;
  }
}

pub trait QStaticText_setTextOption {
  fn setTextOption(self, rsthis: &mut QStaticText) ;
}

// proto:  void QStaticText::setTextOption(const QTextOption & textOption);
impl<'a> /*trait*/ QStaticText_setTextOption for (&'a  QTextOption) {
  fn setTextOption(self, rsthis: &mut QStaticText)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticText13setTextOptionERK11QTextOption()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QStaticText13setTextOptionERK11QTextOption(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStaticText {
  pub fn setTextWidth<T: QStaticText_setTextWidth>(&mut self, value: T)  {
     value.setTextWidth(self);
    // return 1;
  }
}

pub trait QStaticText_setTextWidth {
  fn setTextWidth(self, rsthis: &mut QStaticText) ;
}

// proto:  void QStaticText::setTextWidth(qreal textWidth);
impl<'a> /*trait*/ QStaticText_setTextWidth for (f64) {
  fn setTextWidth(self, rsthis: &mut QStaticText)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticText12setTextWidthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN11QStaticText12setTextWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStaticText {
  pub fn textWidth<T: QStaticText_textWidth>(&mut self, value: T) -> f64 {
    return value.textWidth(self);
    // return 1;
  }
}

pub trait QStaticText_textWidth {
  fn textWidth(self, rsthis: &mut QStaticText) -> f64;
}

// proto:  double QStaticText::textWidth();
impl<'a> /*trait*/ QStaticText_textWidth for () {
  fn textWidth(self, rsthis: &mut QStaticText) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStaticText9textWidthEv()};
    let mut ret = unsafe {_ZNK11QStaticText9textWidthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QStaticText {
  pub fn swap<T: QStaticText_swap>(&mut self, value: T)  {
     value.swap(self);
    // return 1;
  }
}

pub trait QStaticText_swap {
  fn swap(self, rsthis: &mut QStaticText) ;
}

// proto:  void QStaticText::swap(QStaticText & other);
impl<'a> /*trait*/ QStaticText_swap for (&'a mut QStaticText) {
  fn swap(self, rsthis: &mut QStaticText)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticText4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QStaticText4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStaticText {
  pub fn textOption<T: QStaticText_textOption>(&mut self, value: T) -> QTextOption {
    return value.textOption(self);
    // return 1;
  }
}

pub trait QStaticText_textOption {
  fn textOption(self, rsthis: &mut QStaticText) -> QTextOption;
}

// proto:  QTextOption QStaticText::textOption();
impl<'a> /*trait*/ QStaticText_textOption for () {
  fn textOption(self, rsthis: &mut QStaticText) -> QTextOption {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStaticText10textOptionEv()};
    let mut ret = unsafe {_ZNK11QStaticText10textOptionEv(rsthis.qclsinst)};
    let mut ret1 = QTextOption{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QStaticText::NewQStaticText(const QStaticText & other);
impl<'a> /*trait*/ QStaticText_NewQStaticText for (&'a  QStaticText) {
  fn NewQStaticText(self) -> QStaticText {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticTextC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QStaticTextC1ERKS_(qthis, arg0)};
    let rsthis = QStaticText{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

