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
  pub fn size<RetType, T: QStaticText_size<RetType>>(&mut self, value: T) -> RetType {
    return value.size(self);
    // return 1;
  }
}

pub trait QStaticText_size<RetType> {
  fn size(self, rsthis: &mut QStaticText) -> RetType;
}

// proto:  QSizeF QStaticText::size();
impl<'a> /*trait*/ QStaticText_size<QSizeF> for () {
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
  pub fn text<RetType, T: QStaticText_text<RetType>>(&mut self, value: T) -> RetType {
    return value.text(self);
    // return 1;
  }
}

pub trait QStaticText_text<RetType> {
  fn text(self, rsthis: &mut QStaticText) -> RetType;
}

// proto:  QString QStaticText::text();
impl<'a> /*trait*/ QStaticText_text<QString> for () {
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
  pub fn FreeQStaticText<RetType, T: QStaticText_FreeQStaticText<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQStaticText(self);
    // return 1;
  }
}

pub trait QStaticText_FreeQStaticText<RetType> {
  fn FreeQStaticText(self, rsthis: &mut QStaticText) -> RetType;
}

// proto:  void QStaticText::FreeQStaticText();
impl<'a> /*trait*/ QStaticText_FreeQStaticText<()> for () {
  fn FreeQStaticText(self, rsthis: &mut QStaticText) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticTextD0Ev()};
     unsafe {_ZN11QStaticTextD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QStaticText {
  pub fn setText<RetType, T: QStaticText_setText<RetType>>(&mut self, value: T) -> RetType {
    return value.setText(self);
    // return 1;
  }
}

pub trait QStaticText_setText<RetType> {
  fn setText(self, rsthis: &mut QStaticText) -> RetType;
}

// proto:  void QStaticText::setText(const QString & text);
impl<'a> /*trait*/ QStaticText_setText<()> for (&'a  QString) {
  fn setText(self, rsthis: &mut QStaticText) -> () {
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
  pub fn prepare<RetType, T: QStaticText_prepare<RetType>>(&mut self, value: T) -> RetType {
    return value.prepare(self);
    // return 1;
  }
}

pub trait QStaticText_prepare<RetType> {
  fn prepare(self, rsthis: &mut QStaticText) -> RetType;
}

// proto:  void QStaticText::prepare(const QTransform & matrix, const QFont & font);
impl<'a> /*trait*/ QStaticText_prepare<()> for (&'a  QTransform, &'a  QFont) {
  fn prepare(self, rsthis: &mut QStaticText) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticText7prepareERK10QTransformRK5QFont()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QStaticText7prepareERK10QTransformRK5QFont(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QStaticText {
  pub fn setTextOption<RetType, T: QStaticText_setTextOption<RetType>>(&mut self, value: T) -> RetType {
    return value.setTextOption(self);
    // return 1;
  }
}

pub trait QStaticText_setTextOption<RetType> {
  fn setTextOption(self, rsthis: &mut QStaticText) -> RetType;
}

// proto:  void QStaticText::setTextOption(const QTextOption & textOption);
impl<'a> /*trait*/ QStaticText_setTextOption<()> for (&'a  QTextOption) {
  fn setTextOption(self, rsthis: &mut QStaticText) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticText13setTextOptionERK11QTextOption()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QStaticText13setTextOptionERK11QTextOption(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStaticText {
  pub fn setTextWidth<RetType, T: QStaticText_setTextWidth<RetType>>(&mut self, value: T) -> RetType {
    return value.setTextWidth(self);
    // return 1;
  }
}

pub trait QStaticText_setTextWidth<RetType> {
  fn setTextWidth(self, rsthis: &mut QStaticText) -> RetType;
}

// proto:  void QStaticText::setTextWidth(qreal textWidth);
impl<'a> /*trait*/ QStaticText_setTextWidth<()> for (f64) {
  fn setTextWidth(self, rsthis: &mut QStaticText) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticText12setTextWidthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN11QStaticText12setTextWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStaticText {
  pub fn textWidth<RetType, T: QStaticText_textWidth<RetType>>(&mut self, value: T) -> RetType {
    return value.textWidth(self);
    // return 1;
  }
}

pub trait QStaticText_textWidth<RetType> {
  fn textWidth(self, rsthis: &mut QStaticText) -> RetType;
}

// proto:  double QStaticText::textWidth();
impl<'a> /*trait*/ QStaticText_textWidth<f64> for () {
  fn textWidth(self, rsthis: &mut QStaticText) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStaticText9textWidthEv()};
    let mut ret = unsafe {_ZNK11QStaticText9textWidthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QStaticText {
  pub fn swap<RetType, T: QStaticText_swap<RetType>>(&mut self, value: T) -> RetType {
    return value.swap(self);
    // return 1;
  }
}

pub trait QStaticText_swap<RetType> {
  fn swap(self, rsthis: &mut QStaticText) -> RetType;
}

// proto:  void QStaticText::swap(QStaticText & other);
impl<'a> /*trait*/ QStaticText_swap<()> for (&'a mut QStaticText) {
  fn swap(self, rsthis: &mut QStaticText) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticText4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QStaticText4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStaticText {
  pub fn textOption<RetType, T: QStaticText_textOption<RetType>>(&mut self, value: T) -> RetType {
    return value.textOption(self);
    // return 1;
  }
}

pub trait QStaticText_textOption<RetType> {
  fn textOption(self, rsthis: &mut QStaticText) -> RetType;
}

// proto:  QTextOption QStaticText::textOption();
impl<'a> /*trait*/ QStaticText_textOption<QTextOption> for () {
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

