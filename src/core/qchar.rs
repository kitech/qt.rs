// auto generated, do not modify.
// created: Sat Dec 26 10:16:52 2015
// src-file: /QtCore/qchar.h
// dst-file: /src/core/qchar.rs
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
use super::qstring::QString; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QLatin1Char_Class_Size() -> c_int;
  fn QChar_Class_Size() -> c_int;
  // proto: static uint QChar::toUpper(uint ucs4);
  fn _ZN5QChar7toUpperEj(arg0: c_uint) -> c_uint;
  // proto: static bool QChar::isNumber(uint ucs4);
  fn _ZN5QChar8isNumberEj(arg0: c_uint) -> c_char;
  // proto:  void QChar::setCell(uchar cell);
  fn _ZN5QChar7setCellEh(qthis: *mut c_void, arg0: c_uchar);
  // proto:  void QChar::setRow(uchar row);
  fn _ZN5QChar6setRowEh(qthis: *mut c_void, arg0: c_uchar);
  // proto: static uint QChar::mirroredChar(uint ucs4);
  fn _ZN5QChar12mirroredCharEj(arg0: c_uint) -> c_uint;
  // proto: static bool QChar::isPunct(uint ucs4);
  fn _ZN5QChar7isPunctEj(arg0: c_uint) -> c_char;
  // proto: static bool QChar::isSymbol(uint ucs4);
  fn _ZN5QChar8isSymbolEj(arg0: c_uint) -> c_char;
  // proto: static QString QChar::decomposition(uint ucs4);
  fn _ZN5QChar13decompositionEj(arg0: c_uint) -> *mut c_void;
  // proto: static bool QChar::isTitleCase(uint ucs4);
  fn _ZN5QChar11isTitleCaseEj(arg0: c_uint) -> c_char;
  // proto: static int QChar::digitValue(uint ucs4);
  fn _ZN5QChar10digitValueEj(arg0: c_uint) -> c_int;
  // proto: static bool QChar::isUpper(uint ucs4);
  fn _ZN5QChar7isUpperEj(arg0: c_uint) -> c_char;
  // proto: static uint QChar::toTitleCase(uint ucs4);
  fn _ZN5QChar11toTitleCaseEj(arg0: c_uint) -> c_uint;
  // proto: static bool QChar::hasMirrored(uint ucs4);
  fn _ZN5QChar11hasMirroredEj(arg0: c_uint) -> c_char;
  // proto: static bool QChar::isLetterOrNumber(uint ucs4);
  fn _ZN5QChar16isLetterOrNumberEj(arg0: c_uint) -> c_char;
  // proto: static bool QChar::isLower(uint ucs4);
  fn _ZN5QChar7isLowerEj(arg0: c_uint) -> c_char;
  // proto: static bool QChar::isSpace(uint ucs4);
  fn _ZN5QChar7isSpaceEj(arg0: c_uint) -> c_char;
  // proto:  QString QChar::decomposition();
  fn _ZNK5QChar13decompositionEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static bool QChar::isLetter(uint ucs4);
  fn _ZN5QChar8isLetterEj(arg0: c_uint) -> c_char;
  // proto: static uint QChar::toCaseFolded(uint ucs4);
  fn _ZN5QChar12toCaseFoldedEj(arg0: c_uint) -> c_uint;
  // proto: static bool QChar::isPrint(uint ucs4);
  fn _ZN5QChar7isPrintEj(arg0: c_uint) -> c_char;
  // proto: static QChar QChar::fromLatin1(char c);
  fn _ZN5QChar10fromLatin1Ec(arg0: c_char) -> *mut c_void;
  // proto: static unsigned char QChar::combiningClass(uint ucs4);
  fn _ZN5QChar14combiningClassEj(arg0: c_uint) -> c_uchar;
  // proto: static bool QChar::isMark(uint ucs4);
  fn _ZN5QChar6isMarkEj(arg0: c_uint) -> c_char;
  // proto: static uint QChar::toLower(uint ucs4);
  fn _ZN5QChar7toLowerEj(arg0: c_uint) -> c_uint;
  // proto: static bool QChar::isDigit(uint ucs4);
  fn _ZN5QChar7isDigitEj(arg0: c_uint) -> c_char;
  // proto:  char QChar::toLatin1();
  fn _ZNK5QChar8toLatin1Ev(qthis: *mut c_void) -> c_char;
} // <= ext block end

// body block begin =>
// class sizeof(QLatin1Char)=1
pub struct QLatin1Char {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QChar)=2
pub struct QChar {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QLatin1Char {
  pub fn inheritFrom(qthis: *mut c_void) -> QLatin1Char {
    return QLatin1Char{qclsinst: qthis};
  }
}
impl /*struct*/ QChar {
  pub fn inheritFrom(qthis: *mut c_void) -> QChar {
    return QChar{qclsinst: qthis};
  }
}
  // proto: static uint QChar::toUpper(uint ucs4);
impl /*struct*/ QChar {
  pub fn toUpper_s<RetType, T: QChar_toUpper_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.toUpper_s();
    // return 1;
  }
}

pub trait QChar_toUpper_s<RetType> {
  fn toUpper_s(self ) -> RetType;
}

  // proto: static uint QChar::toUpper(uint ucs4);
impl<'a> /*trait*/ QChar_toUpper_s<u32> for (u32) {
  fn toUpper_s(self ) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar7toUpperEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar7toUpperEj(arg0)};
    return ret as u32;
    // return 1;
  }
}

  // proto: static bool QChar::isNumber(uint ucs4);
impl /*struct*/ QChar {
  pub fn isNumber_s<RetType, T: QChar_isNumber_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.isNumber_s();
    // return 1;
  }
}

pub trait QChar_isNumber_s<RetType> {
  fn isNumber_s(self ) -> RetType;
}

  // proto: static bool QChar::isNumber(uint ucs4);
impl<'a> /*trait*/ QChar_isNumber_s<i8> for (u32) {
  fn isNumber_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar8isNumberEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar8isNumberEj(arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QChar::setCell(uchar cell);
impl /*struct*/ QChar {
  pub fn setCell<RetType, T: QChar_setCell<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCell(self);
    // return 1;
  }
}

pub trait QChar_setCell<RetType> {
  fn setCell(self , rsthis: & QChar) -> RetType;
}

  // proto:  void QChar::setCell(uchar cell);
impl<'a> /*trait*/ QChar_setCell<()> for (u8) {
  fn setCell(self , rsthis: & QChar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar7setCellEh()};
    let arg0 = self  as c_uchar;
     unsafe {_ZN5QChar7setCellEh(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QChar::setRow(uchar row);
impl /*struct*/ QChar {
  pub fn setRow<RetType, T: QChar_setRow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRow(self);
    // return 1;
  }
}

pub trait QChar_setRow<RetType> {
  fn setRow(self , rsthis: & QChar) -> RetType;
}

  // proto:  void QChar::setRow(uchar row);
impl<'a> /*trait*/ QChar_setRow<()> for (u8) {
  fn setRow(self , rsthis: & QChar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar6setRowEh()};
    let arg0 = self  as c_uchar;
     unsafe {_ZN5QChar6setRowEh(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static uint QChar::mirroredChar(uint ucs4);
impl /*struct*/ QChar {
  pub fn mirroredChar_s<RetType, T: QChar_mirroredChar_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.mirroredChar_s();
    // return 1;
  }
}

pub trait QChar_mirroredChar_s<RetType> {
  fn mirroredChar_s(self ) -> RetType;
}

  // proto: static uint QChar::mirroredChar(uint ucs4);
impl<'a> /*trait*/ QChar_mirroredChar_s<u32> for (u32) {
  fn mirroredChar_s(self ) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar12mirroredCharEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar12mirroredCharEj(arg0)};
    return ret as u32;
    // return 1;
  }
}

  // proto: static bool QChar::isPunct(uint ucs4);
impl /*struct*/ QChar {
  pub fn isPunct_s<RetType, T: QChar_isPunct_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.isPunct_s();
    // return 1;
  }
}

pub trait QChar_isPunct_s<RetType> {
  fn isPunct_s(self ) -> RetType;
}

  // proto: static bool QChar::isPunct(uint ucs4);
impl<'a> /*trait*/ QChar_isPunct_s<i8> for (u32) {
  fn isPunct_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar7isPunctEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar7isPunctEj(arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static bool QChar::isSymbol(uint ucs4);
impl /*struct*/ QChar {
  pub fn isSymbol_s<RetType, T: QChar_isSymbol_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.isSymbol_s();
    // return 1;
  }
}

pub trait QChar_isSymbol_s<RetType> {
  fn isSymbol_s(self ) -> RetType;
}

  // proto: static bool QChar::isSymbol(uint ucs4);
impl<'a> /*trait*/ QChar_isSymbol_s<i8> for (u32) {
  fn isSymbol_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar8isSymbolEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar8isSymbolEj(arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static QString QChar::decomposition(uint ucs4);
impl /*struct*/ QChar {
  pub fn decomposition_s<RetType, T: QChar_decomposition_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.decomposition_s();
    // return 1;
  }
}

pub trait QChar_decomposition_s<RetType> {
  fn decomposition_s(self ) -> RetType;
}

  // proto: static QString QChar::decomposition(uint ucs4);
impl<'a> /*trait*/ QChar_decomposition_s<QString> for (u32) {
  fn decomposition_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar13decompositionEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar13decompositionEj(arg0)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static bool QChar::isTitleCase(uint ucs4);
impl /*struct*/ QChar {
  pub fn isTitleCase_s<RetType, T: QChar_isTitleCase_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.isTitleCase_s();
    // return 1;
  }
}

pub trait QChar_isTitleCase_s<RetType> {
  fn isTitleCase_s(self ) -> RetType;
}

  // proto: static bool QChar::isTitleCase(uint ucs4);
impl<'a> /*trait*/ QChar_isTitleCase_s<i8> for (u32) {
  fn isTitleCase_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar11isTitleCaseEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar11isTitleCaseEj(arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static int QChar::digitValue(uint ucs4);
impl /*struct*/ QChar {
  pub fn digitValue_s<RetType, T: QChar_digitValue_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.digitValue_s();
    // return 1;
  }
}

pub trait QChar_digitValue_s<RetType> {
  fn digitValue_s(self ) -> RetType;
}

  // proto: static int QChar::digitValue(uint ucs4);
impl<'a> /*trait*/ QChar_digitValue_s<i32> for (u32) {
  fn digitValue_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar10digitValueEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar10digitValueEj(arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto: static bool QChar::isUpper(uint ucs4);
impl /*struct*/ QChar {
  pub fn isUpper_s<RetType, T: QChar_isUpper_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.isUpper_s();
    // return 1;
  }
}

pub trait QChar_isUpper_s<RetType> {
  fn isUpper_s(self ) -> RetType;
}

  // proto: static bool QChar::isUpper(uint ucs4);
impl<'a> /*trait*/ QChar_isUpper_s<i8> for (u32) {
  fn isUpper_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar7isUpperEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar7isUpperEj(arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static uint QChar::toTitleCase(uint ucs4);
impl /*struct*/ QChar {
  pub fn toTitleCase_s<RetType, T: QChar_toTitleCase_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.toTitleCase_s();
    // return 1;
  }
}

pub trait QChar_toTitleCase_s<RetType> {
  fn toTitleCase_s(self ) -> RetType;
}

  // proto: static uint QChar::toTitleCase(uint ucs4);
impl<'a> /*trait*/ QChar_toTitleCase_s<u32> for (u32) {
  fn toTitleCase_s(self ) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar11toTitleCaseEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar11toTitleCaseEj(arg0)};
    return ret as u32;
    // return 1;
  }
}

  // proto: static bool QChar::hasMirrored(uint ucs4);
impl /*struct*/ QChar {
  pub fn hasMirrored_s<RetType, T: QChar_hasMirrored_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.hasMirrored_s();
    // return 1;
  }
}

pub trait QChar_hasMirrored_s<RetType> {
  fn hasMirrored_s(self ) -> RetType;
}

  // proto: static bool QChar::hasMirrored(uint ucs4);
impl<'a> /*trait*/ QChar_hasMirrored_s<i8> for (u32) {
  fn hasMirrored_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar11hasMirroredEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar11hasMirroredEj(arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static bool QChar::isLetterOrNumber(uint ucs4);
impl /*struct*/ QChar {
  pub fn isLetterOrNumber_s<RetType, T: QChar_isLetterOrNumber_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.isLetterOrNumber_s();
    // return 1;
  }
}

pub trait QChar_isLetterOrNumber_s<RetType> {
  fn isLetterOrNumber_s(self ) -> RetType;
}

  // proto: static bool QChar::isLetterOrNumber(uint ucs4);
impl<'a> /*trait*/ QChar_isLetterOrNumber_s<i8> for (u32) {
  fn isLetterOrNumber_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar16isLetterOrNumberEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar16isLetterOrNumberEj(arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static bool QChar::isLower(uint ucs4);
impl /*struct*/ QChar {
  pub fn isLower_s<RetType, T: QChar_isLower_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.isLower_s();
    // return 1;
  }
}

pub trait QChar_isLower_s<RetType> {
  fn isLower_s(self ) -> RetType;
}

  // proto: static bool QChar::isLower(uint ucs4);
impl<'a> /*trait*/ QChar_isLower_s<i8> for (u32) {
  fn isLower_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar7isLowerEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar7isLowerEj(arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static bool QChar::isSpace(uint ucs4);
impl /*struct*/ QChar {
  pub fn isSpace_s<RetType, T: QChar_isSpace_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.isSpace_s();
    // return 1;
  }
}

pub trait QChar_isSpace_s<RetType> {
  fn isSpace_s(self ) -> RetType;
}

  // proto: static bool QChar::isSpace(uint ucs4);
impl<'a> /*trait*/ QChar_isSpace_s<i8> for (u32) {
  fn isSpace_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar7isSpaceEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar7isSpaceEj(arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QChar::decomposition();
impl /*struct*/ QChar {
  pub fn decomposition<RetType, T: QChar_decomposition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.decomposition(self);
    // return 1;
  }
}

pub trait QChar_decomposition<RetType> {
  fn decomposition(self , rsthis: & QChar) -> RetType;
}

  // proto:  QString QChar::decomposition();
impl<'a> /*trait*/ QChar_decomposition<QString> for () {
  fn decomposition(self , rsthis: & QChar) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar13decompositionEv()};
    let mut ret = unsafe {_ZNK5QChar13decompositionEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static bool QChar::isLetter(uint ucs4);
impl /*struct*/ QChar {
  pub fn isLetter_s<RetType, T: QChar_isLetter_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.isLetter_s();
    // return 1;
  }
}

pub trait QChar_isLetter_s<RetType> {
  fn isLetter_s(self ) -> RetType;
}

  // proto: static bool QChar::isLetter(uint ucs4);
impl<'a> /*trait*/ QChar_isLetter_s<i8> for (u32) {
  fn isLetter_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar8isLetterEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar8isLetterEj(arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static uint QChar::toCaseFolded(uint ucs4);
impl /*struct*/ QChar {
  pub fn toCaseFolded_s<RetType, T: QChar_toCaseFolded_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.toCaseFolded_s();
    // return 1;
  }
}

pub trait QChar_toCaseFolded_s<RetType> {
  fn toCaseFolded_s(self ) -> RetType;
}

  // proto: static uint QChar::toCaseFolded(uint ucs4);
impl<'a> /*trait*/ QChar_toCaseFolded_s<u32> for (u32) {
  fn toCaseFolded_s(self ) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar12toCaseFoldedEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar12toCaseFoldedEj(arg0)};
    return ret as u32;
    // return 1;
  }
}

  // proto: static bool QChar::isPrint(uint ucs4);
impl /*struct*/ QChar {
  pub fn isPrint_s<RetType, T: QChar_isPrint_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.isPrint_s();
    // return 1;
  }
}

pub trait QChar_isPrint_s<RetType> {
  fn isPrint_s(self ) -> RetType;
}

  // proto: static bool QChar::isPrint(uint ucs4);
impl<'a> /*trait*/ QChar_isPrint_s<i8> for (u32) {
  fn isPrint_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar7isPrintEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar7isPrintEj(arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static QChar QChar::fromLatin1(char c);
impl /*struct*/ QChar {
  pub fn fromLatin1_s<RetType, T: QChar_fromLatin1_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromLatin1_s();
    // return 1;
  }
}

pub trait QChar_fromLatin1_s<RetType> {
  fn fromLatin1_s(self ) -> RetType;
}

  // proto: static QChar QChar::fromLatin1(char c);
impl<'a> /*trait*/ QChar_fromLatin1_s<QChar> for (i8) {
  fn fromLatin1_s(self ) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar10fromLatin1Ec()};
    let arg0 = self  as c_char;
    let mut ret = unsafe {_ZN5QChar10fromLatin1Ec(arg0)};
    let mut ret1 = QChar::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static unsigned char QChar::combiningClass(uint ucs4);
impl /*struct*/ QChar {
  pub fn combiningClass_s<RetType, T: QChar_combiningClass_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.combiningClass_s();
    // return 1;
  }
}

pub trait QChar_combiningClass_s<RetType> {
  fn combiningClass_s(self ) -> RetType;
}

  // proto: static unsigned char QChar::combiningClass(uint ucs4);
impl<'a> /*trait*/ QChar_combiningClass_s<u8> for (u32) {
  fn combiningClass_s(self ) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar14combiningClassEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar14combiningClassEj(arg0)};
    return ret as u8;
    // return 1;
  }
}

  // proto: static bool QChar::isMark(uint ucs4);
impl /*struct*/ QChar {
  pub fn isMark_s<RetType, T: QChar_isMark_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.isMark_s();
    // return 1;
  }
}

pub trait QChar_isMark_s<RetType> {
  fn isMark_s(self ) -> RetType;
}

  // proto: static bool QChar::isMark(uint ucs4);
impl<'a> /*trait*/ QChar_isMark_s<i8> for (u32) {
  fn isMark_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar6isMarkEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar6isMarkEj(arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static uint QChar::toLower(uint ucs4);
impl /*struct*/ QChar {
  pub fn toLower_s<RetType, T: QChar_toLower_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.toLower_s();
    // return 1;
  }
}

pub trait QChar_toLower_s<RetType> {
  fn toLower_s(self ) -> RetType;
}

  // proto: static uint QChar::toLower(uint ucs4);
impl<'a> /*trait*/ QChar_toLower_s<u32> for (u32) {
  fn toLower_s(self ) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar7toLowerEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar7toLowerEj(arg0)};
    return ret as u32;
    // return 1;
  }
}

  // proto: static bool QChar::isDigit(uint ucs4);
impl /*struct*/ QChar {
  pub fn isDigit_s<RetType, T: QChar_isDigit_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.isDigit_s();
    // return 1;
  }
}

pub trait QChar_isDigit_s<RetType> {
  fn isDigit_s(self ) -> RetType;
}

  // proto: static bool QChar::isDigit(uint ucs4);
impl<'a> /*trait*/ QChar_isDigit_s<i8> for (u32) {
  fn isDigit_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar7isDigitEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar7isDigitEj(arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  char QChar::toLatin1();
impl /*struct*/ QChar {
  pub fn toLatin1<RetType, T: QChar_toLatin1<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toLatin1(self);
    // return 1;
  }
}

pub trait QChar_toLatin1<RetType> {
  fn toLatin1(self , rsthis: & QChar) -> RetType;
}

  // proto:  char QChar::toLatin1();
impl<'a> /*trait*/ QChar_toLatin1<i8> for () {
  fn toLatin1(self , rsthis: & QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar8toLatin1Ev()};
    let mut ret = unsafe {_ZNK5QChar8toLatin1Ev(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// <= body block end

