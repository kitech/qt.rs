// auto generated, do not modify.
// created: Sat Dec 26 10:16:52 2015
// src-file: /QtGui/qstatictext.h
// dst-file: /src/gui/qstatictext.rs
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
use super::super::core::qstring::QString; // 771
use super::super::core::qsize::QSizeF; // 771
use super::qtextoption::QTextOption; // 773
use super::qtransform::QTransform; // 773
use super::qfont::QFont; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QStaticText_Class_Size() -> c_int;
  // proto:  void QStaticText::QStaticText(const QString & text);
  fn dector_ZN11QStaticTextC1ERK7QString(arg0: *mut c_void) -> *mut c_void;
  fn _ZN11QStaticTextC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QSizeF QStaticText::size();
  fn _ZNK11QStaticText4sizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QStaticText::text();
  fn _ZNK11QStaticText4textEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QStaticText::~QStaticText();
  fn _ZN11QStaticTextD0Ev(qthis: *mut c_void);
  // proto:  void QStaticText::setText(const QString & text);
  fn _ZN11QStaticText7setTextERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStaticText::QStaticText();
  fn dector_ZN11QStaticTextC1Ev() -> *mut c_void;
  fn _ZN11QStaticTextC1Ev(qthis: *mut c_void);
  // proto:  void QStaticText::setTextOption(const QTextOption & textOption);
  fn _ZN11QStaticText13setTextOptionERK11QTextOption(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStaticText::setTextWidth(qreal textWidth);
  fn _ZN11QStaticText12setTextWidthEd(qthis: *mut c_void, arg0: c_double);
  // proto:  qreal QStaticText::textWidth();
  fn _ZNK11QStaticText9textWidthEv(qthis: *mut c_void) -> c_double;
  // proto:  void QStaticText::prepare(const QTransform & matrix, const QFont & font);
  fn _ZN11QStaticText7prepareERK10QTransformRK5QFont(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QTextOption QStaticText::textOption();
  fn _ZNK11QStaticText10textOptionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QStaticText::QStaticText(const QStaticText & other);
  fn dector_ZN11QStaticTextC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN11QStaticTextC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QStaticText)=1
pub struct QStaticText {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStaticText {
  pub fn inheritFrom(qthis: *mut c_void) -> QStaticText {
    return QStaticText{qclsinst: qthis};
  }
}
  // proto:  void QStaticText::QStaticText(const QString & text);
impl /*struct*/ QStaticText {
  pub fn New<T: QStaticText_New>(value: T) -> QStaticText {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QStaticText_New {
  fn New(self) -> QStaticText;
}

  // proto:  void QStaticText::QStaticText(const QString & text);
impl<'a> /*trait*/ QStaticText_New for (&'a QString) {
  fn New(self) -> QStaticText {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticTextC1ERK7QString()};
    let ctysz: c_int = unsafe{QStaticText_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN11QStaticTextC1ERK7QString(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN11QStaticTextC1ERK7QString(arg0)};
    let rsthis = QStaticText{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QSizeF QStaticText::size();
impl /*struct*/ QStaticText {
  pub fn size<RetType, T: QStaticText_size<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QStaticText_size<RetType> {
  fn size(self , rsthis: & QStaticText) -> RetType;
}

  // proto:  QSizeF QStaticText::size();
impl<'a> /*trait*/ QStaticText_size<QSizeF> for () {
  fn size(self , rsthis: & QStaticText) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStaticText4sizeEv()};
    let mut ret = unsafe {_ZNK11QStaticText4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QStaticText::text();
impl /*struct*/ QStaticText {
  pub fn text<RetType, T: QStaticText_text<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.text(self);
    // return 1;
  }
}

pub trait QStaticText_text<RetType> {
  fn text(self , rsthis: & QStaticText) -> RetType;
}

  // proto:  QString QStaticText::text();
impl<'a> /*trait*/ QStaticText_text<QString> for () {
  fn text(self , rsthis: & QStaticText) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStaticText4textEv()};
    let mut ret = unsafe {_ZNK11QStaticText4textEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QStaticText::~QStaticText();
impl /*struct*/ QStaticText {
  pub fn Free<RetType, T: QStaticText_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QStaticText_Free<RetType> {
  fn Free(self , rsthis: & QStaticText) -> RetType;
}

  // proto:  void QStaticText::~QStaticText();
impl<'a> /*trait*/ QStaticText_Free<()> for () {
  fn Free(self , rsthis: & QStaticText) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticTextD0Ev()};
     unsafe {_ZN11QStaticTextD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QStaticText::setText(const QString & text);
impl /*struct*/ QStaticText {
  pub fn setText<RetType, T: QStaticText_setText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setText(self);
    // return 1;
  }
}

pub trait QStaticText_setText<RetType> {
  fn setText(self , rsthis: & QStaticText) -> RetType;
}

  // proto:  void QStaticText::setText(const QString & text);
impl<'a> /*trait*/ QStaticText_setText<()> for (&'a QString) {
  fn setText(self , rsthis: & QStaticText) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticText7setTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QStaticText7setTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStaticText::QStaticText();
impl<'a> /*trait*/ QStaticText_New for () {
  fn New(self) -> QStaticText {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticTextC1Ev()};
    let ctysz: c_int = unsafe{QStaticText_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN11QStaticTextC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN11QStaticTextC1Ev()};
    let rsthis = QStaticText{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStaticText::setTextOption(const QTextOption & textOption);
impl /*struct*/ QStaticText {
  pub fn setTextOption<RetType, T: QStaticText_setTextOption<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTextOption(self);
    // return 1;
  }
}

pub trait QStaticText_setTextOption<RetType> {
  fn setTextOption(self , rsthis: & QStaticText) -> RetType;
}

  // proto:  void QStaticText::setTextOption(const QTextOption & textOption);
impl<'a> /*trait*/ QStaticText_setTextOption<()> for (&'a QTextOption) {
  fn setTextOption(self , rsthis: & QStaticText) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticText13setTextOptionERK11QTextOption()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QStaticText13setTextOptionERK11QTextOption(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStaticText::setTextWidth(qreal textWidth);
impl /*struct*/ QStaticText {
  pub fn setTextWidth<RetType, T: QStaticText_setTextWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTextWidth(self);
    // return 1;
  }
}

pub trait QStaticText_setTextWidth<RetType> {
  fn setTextWidth(self , rsthis: & QStaticText) -> RetType;
}

  // proto:  void QStaticText::setTextWidth(qreal textWidth);
impl<'a> /*trait*/ QStaticText_setTextWidth<()> for (f64) {
  fn setTextWidth(self , rsthis: & QStaticText) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticText12setTextWidthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN11QStaticText12setTextWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QStaticText::textWidth();
impl /*struct*/ QStaticText {
  pub fn textWidth<RetType, T: QStaticText_textWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textWidth(self);
    // return 1;
  }
}

pub trait QStaticText_textWidth<RetType> {
  fn textWidth(self , rsthis: & QStaticText) -> RetType;
}

  // proto:  qreal QStaticText::textWidth();
impl<'a> /*trait*/ QStaticText_textWidth<f64> for () {
  fn textWidth(self , rsthis: & QStaticText) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStaticText9textWidthEv()};
    let mut ret = unsafe {_ZNK11QStaticText9textWidthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QStaticText::prepare(const QTransform & matrix, const QFont & font);
impl /*struct*/ QStaticText {
  pub fn prepare<RetType, T: QStaticText_prepare<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.prepare(self);
    // return 1;
  }
}

pub trait QStaticText_prepare<RetType> {
  fn prepare(self , rsthis: & QStaticText) -> RetType;
}

  // proto:  void QStaticText::prepare(const QTransform & matrix, const QFont & font);
impl<'a> /*trait*/ QStaticText_prepare<()> for (&'a QTransform, &'a QFont) {
  fn prepare(self , rsthis: & QStaticText) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticText7prepareERK10QTransformRK5QFont()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QStaticText7prepareERK10QTransformRK5QFont(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QTextOption QStaticText::textOption();
impl /*struct*/ QStaticText {
  pub fn textOption<RetType, T: QStaticText_textOption<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textOption(self);
    // return 1;
  }
}

pub trait QStaticText_textOption<RetType> {
  fn textOption(self , rsthis: & QStaticText) -> RetType;
}

  // proto:  QTextOption QStaticText::textOption();
impl<'a> /*trait*/ QStaticText_textOption<QTextOption> for () {
  fn textOption(self , rsthis: & QStaticText) -> QTextOption {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStaticText10textOptionEv()};
    let mut ret = unsafe {_ZNK11QStaticText10textOptionEv(rsthis.qclsinst)};
    let mut ret1 = QTextOption::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QStaticText::QStaticText(const QStaticText & other);
impl<'a> /*trait*/ QStaticText_New for (&'a QStaticText) {
  fn New(self) -> QStaticText {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticTextC1ERKS_()};
    let ctysz: c_int = unsafe{QStaticText_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN11QStaticTextC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN11QStaticTextC1ERKS_(arg0)};
    let rsthis = QStaticText{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// <= body block end

