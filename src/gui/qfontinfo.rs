// auto generated, do not modify.
// created: Sat Dec 26 10:52:38 2015
// src-file: /QtGui/qfontinfo.h
// dst-file: /src/gui/qfontinfo.rs
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
use super::qfont::QFont; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QFontInfo_Class_Size() -> c_int;
  // proto:  bool QFontInfo::rawMode();
  fn _ZNK9QFontInfo7rawModeEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QFontInfo::exactMatch();
  fn _ZNK9QFontInfo10exactMatchEv(qthis: *mut c_void) -> c_char;
  // proto:  int QFontInfo::pointSize();
  fn _ZNK9QFontInfo9pointSizeEv(qthis: *mut c_void) -> c_int;
  // proto:  void QFontInfo::QFontInfo(const QFontInfo & );
  fn dector_ZN9QFontInfoC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QFontInfoC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QString QFontInfo::family();
  fn _ZNK9QFontInfo6familyEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QFontInfo::bold();
  fn _ZNK9QFontInfo4boldEv(qthis: *mut c_void) -> c_char;
  // proto:  qreal QFontInfo::pointSizeF();
  fn _ZNK9QFontInfo10pointSizeFEv(qthis: *mut c_void) -> c_double;
  // proto:  bool QFontInfo::fixedPitch();
  fn _ZNK9QFontInfo10fixedPitchEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QFontInfo::overline();
  fn _ZNK9QFontInfo8overlineEv(qthis: *mut c_void) -> c_char;
  // proto:  void QFontInfo::swap(QFontInfo & other);
  fn _ZN9QFontInfo4swapERS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QFontInfo::QFontInfo(const QFont & );
  fn dector_ZN9QFontInfoC1ERK5QFont(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QFontInfoC1ERK5QFont(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QFontInfo::pixelSize();
  fn _ZNK9QFontInfo9pixelSizeEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QFontInfo::strikeOut();
  fn _ZNK9QFontInfo9strikeOutEv(qthis: *mut c_void) -> c_char;
  // proto:  void QFontInfo::~QFontInfo();
  fn _ZN9QFontInfoD0Ev(qthis: *mut c_void);
  // proto:  bool QFontInfo::italic();
  fn _ZNK9QFontInfo6italicEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QFontInfo::underline();
  fn _ZNK9QFontInfo9underlineEv(qthis: *mut c_void) -> c_char;
  // proto:  QString QFontInfo::styleName();
  fn _ZNK9QFontInfo9styleNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QFontInfo::weight();
  fn _ZNK9QFontInfo6weightEv(qthis: *mut c_void) -> c_int;
} // <= ext block end

// body block begin =>
// class sizeof(QFontInfo)=1
pub struct QFontInfo {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFontInfo {
  pub fn inheritFrom(qthis: *mut c_void) -> QFontInfo {
    return QFontInfo{qclsinst: qthis};
  }
}
  // proto:  bool QFontInfo::rawMode();
impl /*struct*/ QFontInfo {
  pub fn rawMode<RetType, T: QFontInfo_rawMode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rawMode(self);
    // return 1;
  }
}

pub trait QFontInfo_rawMode<RetType> {
  fn rawMode(self , rsthis: & QFontInfo) -> RetType;
}

  // proto:  bool QFontInfo::rawMode();
impl<'a> /*trait*/ QFontInfo_rawMode<i8> for () {
  fn rawMode(self , rsthis: & QFontInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo7rawModeEv()};
    let mut ret = unsafe {_ZNK9QFontInfo7rawModeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QFontInfo::exactMatch();
impl /*struct*/ QFontInfo {
  pub fn exactMatch<RetType, T: QFontInfo_exactMatch<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.exactMatch(self);
    // return 1;
  }
}

pub trait QFontInfo_exactMatch<RetType> {
  fn exactMatch(self , rsthis: & QFontInfo) -> RetType;
}

  // proto:  bool QFontInfo::exactMatch();
impl<'a> /*trait*/ QFontInfo_exactMatch<i8> for () {
  fn exactMatch(self , rsthis: & QFontInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo10exactMatchEv()};
    let mut ret = unsafe {_ZNK9QFontInfo10exactMatchEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QFontInfo::pointSize();
impl /*struct*/ QFontInfo {
  pub fn pointSize<RetType, T: QFontInfo_pointSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pointSize(self);
    // return 1;
  }
}

pub trait QFontInfo_pointSize<RetType> {
  fn pointSize(self , rsthis: & QFontInfo) -> RetType;
}

  // proto:  int QFontInfo::pointSize();
impl<'a> /*trait*/ QFontInfo_pointSize<i32> for () {
  fn pointSize(self , rsthis: & QFontInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo9pointSizeEv()};
    let mut ret = unsafe {_ZNK9QFontInfo9pointSizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QFontInfo::QFontInfo(const QFontInfo & );
impl /*struct*/ QFontInfo {
  pub fn New<T: QFontInfo_New>(value: T) -> QFontInfo {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QFontInfo_New {
  fn New(self) -> QFontInfo;
}

  // proto:  void QFontInfo::QFontInfo(const QFontInfo & );
impl<'a> /*trait*/ QFontInfo_New for (&'a QFontInfo) {
  fn New(self) -> QFontInfo {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFontInfoC1ERKS_()};
    let ctysz: c_int = unsafe{QFontInfo_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QFontInfoC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN9QFontInfoC1ERKS_(arg0)};
    let rsthis = QFontInfo{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QFontInfo::family();
impl /*struct*/ QFontInfo {
  pub fn family<RetType, T: QFontInfo_family<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.family(self);
    // return 1;
  }
}

pub trait QFontInfo_family<RetType> {
  fn family(self , rsthis: & QFontInfo) -> RetType;
}

  // proto:  QString QFontInfo::family();
impl<'a> /*trait*/ QFontInfo_family<QString> for () {
  fn family(self , rsthis: & QFontInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo6familyEv()};
    let mut ret = unsafe {_ZNK9QFontInfo6familyEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QFontInfo::bold();
impl /*struct*/ QFontInfo {
  pub fn bold<RetType, T: QFontInfo_bold<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bold(self);
    // return 1;
  }
}

pub trait QFontInfo_bold<RetType> {
  fn bold(self , rsthis: & QFontInfo) -> RetType;
}

  // proto:  bool QFontInfo::bold();
impl<'a> /*trait*/ QFontInfo_bold<i8> for () {
  fn bold(self , rsthis: & QFontInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo4boldEv()};
    let mut ret = unsafe {_ZNK9QFontInfo4boldEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qreal QFontInfo::pointSizeF();
impl /*struct*/ QFontInfo {
  pub fn pointSizeF<RetType, T: QFontInfo_pointSizeF<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pointSizeF(self);
    // return 1;
  }
}

pub trait QFontInfo_pointSizeF<RetType> {
  fn pointSizeF(self , rsthis: & QFontInfo) -> RetType;
}

  // proto:  qreal QFontInfo::pointSizeF();
impl<'a> /*trait*/ QFontInfo_pointSizeF<f64> for () {
  fn pointSizeF(self , rsthis: & QFontInfo) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo10pointSizeFEv()};
    let mut ret = unsafe {_ZNK9QFontInfo10pointSizeFEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  bool QFontInfo::fixedPitch();
impl /*struct*/ QFontInfo {
  pub fn fixedPitch<RetType, T: QFontInfo_fixedPitch<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fixedPitch(self);
    // return 1;
  }
}

pub trait QFontInfo_fixedPitch<RetType> {
  fn fixedPitch(self , rsthis: & QFontInfo) -> RetType;
}

  // proto:  bool QFontInfo::fixedPitch();
impl<'a> /*trait*/ QFontInfo_fixedPitch<i8> for () {
  fn fixedPitch(self , rsthis: & QFontInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo10fixedPitchEv()};
    let mut ret = unsafe {_ZNK9QFontInfo10fixedPitchEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QFontInfo::overline();
impl /*struct*/ QFontInfo {
  pub fn overline<RetType, T: QFontInfo_overline<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.overline(self);
    // return 1;
  }
}

pub trait QFontInfo_overline<RetType> {
  fn overline(self , rsthis: & QFontInfo) -> RetType;
}

  // proto:  bool QFontInfo::overline();
impl<'a> /*trait*/ QFontInfo_overline<i8> for () {
  fn overline(self , rsthis: & QFontInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo8overlineEv()};
    let mut ret = unsafe {_ZNK9QFontInfo8overlineEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QFontInfo::swap(QFontInfo & other);
impl /*struct*/ QFontInfo {
  pub fn swap<RetType, T: QFontInfo_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QFontInfo_swap<RetType> {
  fn swap(self , rsthis: & QFontInfo) -> RetType;
}

  // proto:  void QFontInfo::swap(QFontInfo & other);
impl<'a> /*trait*/ QFontInfo_swap<()> for (&'a QFontInfo) {
  fn swap(self , rsthis: & QFontInfo) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFontInfo4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QFontInfo4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QFontInfo::QFontInfo(const QFont & );
impl<'a> /*trait*/ QFontInfo_New for (&'a QFont) {
  fn New(self) -> QFontInfo {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFontInfoC1ERK5QFont()};
    let ctysz: c_int = unsafe{QFontInfo_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QFontInfoC1ERK5QFont(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN9QFontInfoC1ERK5QFont(arg0)};
    let rsthis = QFontInfo{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QFontInfo::pixelSize();
impl /*struct*/ QFontInfo {
  pub fn pixelSize<RetType, T: QFontInfo_pixelSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pixelSize(self);
    // return 1;
  }
}

pub trait QFontInfo_pixelSize<RetType> {
  fn pixelSize(self , rsthis: & QFontInfo) -> RetType;
}

  // proto:  int QFontInfo::pixelSize();
impl<'a> /*trait*/ QFontInfo_pixelSize<i32> for () {
  fn pixelSize(self , rsthis: & QFontInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo9pixelSizeEv()};
    let mut ret = unsafe {_ZNK9QFontInfo9pixelSizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QFontInfo::strikeOut();
impl /*struct*/ QFontInfo {
  pub fn strikeOut<RetType, T: QFontInfo_strikeOut<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.strikeOut(self);
    // return 1;
  }
}

pub trait QFontInfo_strikeOut<RetType> {
  fn strikeOut(self , rsthis: & QFontInfo) -> RetType;
}

  // proto:  bool QFontInfo::strikeOut();
impl<'a> /*trait*/ QFontInfo_strikeOut<i8> for () {
  fn strikeOut(self , rsthis: & QFontInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo9strikeOutEv()};
    let mut ret = unsafe {_ZNK9QFontInfo9strikeOutEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QFontInfo::~QFontInfo();
impl /*struct*/ QFontInfo {
  pub fn Free<RetType, T: QFontInfo_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QFontInfo_Free<RetType> {
  fn Free(self , rsthis: & QFontInfo) -> RetType;
}

  // proto:  void QFontInfo::~QFontInfo();
impl<'a> /*trait*/ QFontInfo_Free<()> for () {
  fn Free(self , rsthis: & QFontInfo) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFontInfoD0Ev()};
     unsafe {_ZN9QFontInfoD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QFontInfo::italic();
impl /*struct*/ QFontInfo {
  pub fn italic<RetType, T: QFontInfo_italic<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.italic(self);
    // return 1;
  }
}

pub trait QFontInfo_italic<RetType> {
  fn italic(self , rsthis: & QFontInfo) -> RetType;
}

  // proto:  bool QFontInfo::italic();
impl<'a> /*trait*/ QFontInfo_italic<i8> for () {
  fn italic(self , rsthis: & QFontInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo6italicEv()};
    let mut ret = unsafe {_ZNK9QFontInfo6italicEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QFontInfo::underline();
impl /*struct*/ QFontInfo {
  pub fn underline<RetType, T: QFontInfo_underline<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.underline(self);
    // return 1;
  }
}

pub trait QFontInfo_underline<RetType> {
  fn underline(self , rsthis: & QFontInfo) -> RetType;
}

  // proto:  bool QFontInfo::underline();
impl<'a> /*trait*/ QFontInfo_underline<i8> for () {
  fn underline(self , rsthis: & QFontInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo9underlineEv()};
    let mut ret = unsafe {_ZNK9QFontInfo9underlineEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QFontInfo::styleName();
impl /*struct*/ QFontInfo {
  pub fn styleName<RetType, T: QFontInfo_styleName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.styleName(self);
    // return 1;
  }
}

pub trait QFontInfo_styleName<RetType> {
  fn styleName(self , rsthis: & QFontInfo) -> RetType;
}

  // proto:  QString QFontInfo::styleName();
impl<'a> /*trait*/ QFontInfo_styleName<QString> for () {
  fn styleName(self , rsthis: & QFontInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo9styleNameEv()};
    let mut ret = unsafe {_ZNK9QFontInfo9styleNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QFontInfo::weight();
impl /*struct*/ QFontInfo {
  pub fn weight<RetType, T: QFontInfo_weight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.weight(self);
    // return 1;
  }
}

pub trait QFontInfo_weight<RetType> {
  fn weight(self , rsthis: & QFontInfo) -> RetType;
}

  // proto:  int QFontInfo::weight();
impl<'a> /*trait*/ QFontInfo_weight<i32> for () {
  fn weight(self , rsthis: & QFontInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFontInfo6weightEv()};
    let mut ret = unsafe {_ZNK9QFontInfo6weightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// <= body block end

