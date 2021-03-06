// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
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
use super::super::core::qstring::*; // 771
use super::super::core::qsize::*; // 771
use super::qtransform::*; // 773
use super::qfont::*; // 773
use super::qtextoption::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QStaticText_Class_Size() -> c_int;
  // proto:  void QStaticText::QStaticText(const QString & text);
  fn C_ZN11QStaticTextC2ERK7QString(arg0: *mut c_void) -> u64;
  // proto:  QSizeF QStaticText::size();
  fn C_ZNK11QStaticText4sizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QStaticText::text();
  fn C_ZNK11QStaticText4textEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QStaticText::~QStaticText();
  fn C_ZN11QStaticTextD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QStaticText::setText(const QString & text);
  fn C_ZN11QStaticText7setTextERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QStaticText::QStaticText();
  fn C_ZN11QStaticTextC2Ev() -> u64;
  // proto:  void QStaticText::prepare(const QTransform & matrix, const QFont & font);
  fn C_ZN11QStaticText7prepareERK10QTransformRK5QFont(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QStaticText::setTextOption(const QTextOption & textOption);
  fn C_ZN11QStaticText13setTextOptionERK11QTextOption(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QStaticText::setTextWidth(qreal textWidth);
  fn C_ZN11QStaticText12setTextWidthEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  qreal QStaticText::textWidth();
  fn C_ZNK11QStaticText9textWidthEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QStaticText::swap(QStaticText & other);
  fn C_ZN11QStaticText4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QTextOption QStaticText::textOption();
  fn C_ZNK11QStaticText10textOptionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QStaticText::QStaticText(const QStaticText & other);
  fn C_ZN11QStaticTextC2ERKS_(arg0: *mut c_void) -> u64;
} // <= ext block end

// body block begin =>
// class sizeof(QStaticText)=1
#[derive(Default)]
pub struct QStaticText {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QStaticText {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStaticText {
    return QStaticText{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QStaticText::QStaticText(const QString & text);
impl /*struct*/ QStaticText {
  pub fn new<T: QStaticText_new>(value: T) -> QStaticText {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStaticText_new {
  fn new(self) -> QStaticText;
}

  // proto:  void QStaticText::QStaticText(const QString & text);
impl<'a> /*trait*/ QStaticText_new for (&'a QString) {
  fn new(self) -> QStaticText {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticTextC2ERK7QString()};
    let ctysz: c_int = unsafe{QStaticText_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN11QStaticTextC2ERK7QString(arg0)};
    let rsthis = QStaticText{qclsinst: qthis, ..Default::default()};
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
    let mut ret = unsafe {C_ZNK11QStaticText4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF::inheritFrom(ret as u64);
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
    let mut ret = unsafe {C_ZNK11QStaticText4textEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QStaticText::~QStaticText();
impl /*struct*/ QStaticText {
  pub fn free<RetType, T: QStaticText_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QStaticText_free<RetType> {
  fn free(self , rsthis: & QStaticText) -> RetType;
}

  // proto:  void QStaticText::~QStaticText();
impl<'a> /*trait*/ QStaticText_free<()> for () {
  fn free(self , rsthis: & QStaticText) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticTextD2Ev()};
     unsafe {C_ZN11QStaticTextD2Ev(rsthis.qclsinst)};
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
     unsafe {C_ZN11QStaticText7setTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStaticText::QStaticText();
impl<'a> /*trait*/ QStaticText_new for () {
  fn new(self) -> QStaticText {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticTextC2Ev()};
    let ctysz: c_int = unsafe{QStaticText_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN11QStaticTextC2Ev()};
    let rsthis = QStaticText{qclsinst: qthis, ..Default::default()};
    return rsthis;
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
impl<'a> /*trait*/ QStaticText_prepare<()> for (Option<&'a QTransform>, Option<&'a QFont>) {
  fn prepare(self , rsthis: & QStaticText) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticText7prepareERK10QTransformRK5QFont()};
    let arg0 = (if self.0.is_none() {QTransform::new(()).qclsinst} else {self.0.unwrap().qclsinst})  as *mut c_void;
    let arg1 = (if self.1.is_none() {QFont::new(()).qclsinst} else {self.1.unwrap().qclsinst})  as *mut c_void;
     unsafe {C_ZN11QStaticText7prepareERK10QTransformRK5QFont(rsthis.qclsinst, arg0, arg1)};
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
     unsafe {C_ZN11QStaticText13setTextOptionERK11QTextOption(rsthis.qclsinst, arg0)};
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
     unsafe {C_ZN11QStaticText12setTextWidthEd(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {C_ZNK11QStaticText9textWidthEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  void QStaticText::swap(QStaticText & other);
impl /*struct*/ QStaticText {
  pub fn swap<RetType, T: QStaticText_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QStaticText_swap<RetType> {
  fn swap(self , rsthis: & QStaticText) -> RetType;
}

  // proto:  void QStaticText::swap(QStaticText & other);
impl<'a> /*trait*/ QStaticText_swap<()> for (&'a QStaticText) {
  fn swap(self , rsthis: & QStaticText) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticText4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QStaticText4swapERS_(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {C_ZNK11QStaticText10textOptionEv(rsthis.qclsinst)};
    let mut ret1 = QTextOption::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QStaticText::QStaticText(const QStaticText & other);
impl<'a> /*trait*/ QStaticText_new for (&'a QStaticText) {
  fn new(self) -> QStaticText {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticTextC2ERKS_()};
    let ctysz: c_int = unsafe{QStaticText_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN11QStaticTextC2ERKS_(arg0)};
    let rsthis = QStaticText{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

