// auto generated, do not modify.
// created: Wed Dec 30 23:22:52 2015
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
  // proto:  ushort QLatin1Char::unicode();
  fn _ZNK11QLatin1Char7unicodeEv(qthis: u64 /* *mut c_void*/) -> c_ushort;
  // proto:  void QLatin1Char::QLatin1Char(char c);
  fn dector_ZN11QLatin1CharC1Ec(arg0: c_char) -> *mut c_void;
  fn _ZN11QLatin1CharC1Ec(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  char QLatin1Char::toLatin1();
  fn _ZNK11QLatin1Char8toLatin1Ev(qthis: u64 /* *mut c_void*/) -> c_char;
  fn QChar_Class_Size() -> c_int;
  // proto: static uint QChar::toUpper(uint ucs4);
  fn _ZN5QChar7toUpperEj(arg0: c_uint) -> c_uint;
  // proto:  bool QChar::hasMirrored();
  fn demth_ZNK5QChar11hasMirroredEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto: static ushort QChar::lowSurrogate(uint ucs4);
  fn demth_ZN5QChar12lowSurrogateEj(arg0: c_uint) -> c_ushort;
  // proto: static bool QChar::isSymbol(uint ucs4);
  fn _ZN5QChar8isSymbolEj(arg0: c_uint) -> c_char;
  // proto:  uchar QChar::cell();
  fn _ZNK5QChar4cellEv(qthis: u64 /* *mut c_void*/) -> c_uchar;
  // proto: static uint QChar::surrogateToUcs4(QChar high, QChar low);
  fn demth_ZN5QChar15surrogateToUcs4ES_S_(arg0: *mut c_void, arg1: *mut c_void) -> c_uint;
  // proto: static bool QChar::isTitleCase(uint ucs4);
  fn demth_ZN5QChar11isTitleCaseEj(arg0: c_uint) -> c_char;
  // proto:  bool QChar::isNull();
  fn _ZNK5QChar6isNullEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QChar::digitValue();
  fn demth_ZNK5QChar10digitValueEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QChar QChar::toTitleCase();
  fn demth_ZNK5QChar11toTitleCaseEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static bool QChar::isLower(uint ucs4);
  fn demth_ZN5QChar7isLowerEj(arg0: c_uint) -> c_char;
  // proto:  bool QChar::isLowSurrogate();
  fn _ZNK5QChar14isLowSurrogateEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto: static bool QChar::isPrint(uint ucs4);
  fn _ZN5QChar7isPrintEj(arg0: c_uint) -> c_char;
  // proto:  bool QChar::isSymbol();
  fn demth_ZNK5QChar8isSymbolEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QChar::isLower();
  fn _ZNK5QChar7isLowerEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QChar QChar::mirroredChar();
  fn demth_ZNK5QChar12mirroredCharEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  uchar QChar::row();
  fn _ZNK5QChar3rowEv(qthis: u64 /* *mut c_void*/) -> c_uchar;
  // proto:  bool QChar::isDigit();
  fn _ZNK5QChar7isDigitEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto: static uint QChar::toTitleCase(uint ucs4);
  fn _ZN5QChar11toTitleCaseEj(arg0: c_uint) -> c_uint;
  // proto:  bool QChar::isSurrogate();
  fn _ZNK5QChar11isSurrogateEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto: static bool QChar::hasMirrored(uint ucs4);
  fn _ZN5QChar11hasMirroredEj(arg0: c_uint) -> c_char;
  // proto:  bool QChar::isNumber();
  fn _ZNK5QChar8isNumberEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto: static bool QChar::isHighSurrogate(uint ucs4);
  fn demth_ZN5QChar15isHighSurrogateEj(arg0: c_uint) -> c_char;
  // proto: static uint QChar::toCaseFolded(uint ucs4);
  fn _ZN5QChar12toCaseFoldedEj(arg0: c_uint) -> c_uint;
  // proto:  bool QChar::isMark();
  fn demth_ZNK5QChar6isMarkEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto: static uint QChar::surrogateToUcs4(ushort high, ushort low);
  fn demth_ZN5QChar15surrogateToUcs4Ett(arg0: c_ushort, arg1: c_ushort) -> c_uint;
  // proto: static uint QChar::toLower(uint ucs4);
  fn _ZN5QChar7toLowerEj(arg0: c_uint) -> c_uint;
  // proto: static uint QChar::mirroredChar(uint ucs4);
  fn _ZN5QChar12mirroredCharEj(arg0: c_uint) -> c_uint;
  // proto:  void QChar::setRow(uchar row);
  fn demth_ZN5QChar6setRowEh(qthis: u64 /* *mut c_void*/, arg0: c_uchar);
  // proto: static QString QChar::decomposition(uint ucs4);
  fn _ZN5QChar13decompositionEj(arg0: c_uint) -> *mut c_void;
  // proto: static int QChar::digitValue(uint ucs4);
  fn _ZN5QChar10digitValueEj(arg0: c_uint) -> c_int;
  // proto:  void QChar::setCell(uchar cell);
  fn demth_ZN5QChar7setCellEh(qthis: u64 /* *mut c_void*/, arg0: c_uchar);
  // proto: static bool QChar::isUpper(uint ucs4);
  fn demth_ZN5QChar7isUpperEj(arg0: c_uint) -> c_char;
  // proto:  void QChar::QChar(uchar c, uchar r);
  fn dector_ZN5QCharC1Ehh(arg0: c_uchar, arg1: c_uchar) -> *mut c_void;
  fn _ZN5QCharC1Ehh(qthis: u64 /* *mut c_void*/, arg0: c_uchar, arg1: c_uchar);
  // proto:  QChar QChar::toCaseFolded();
  fn demth_ZNK5QChar12toCaseFoldedEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QChar::isPrint();
  fn demth_ZNK5QChar7isPrintEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QChar::QChar(char c);
  fn dector_ZN5QCharC1Ec(arg0: c_char) -> *mut c_void;
  fn _ZN5QCharC1Ec(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QChar::isPunct();
  fn demth_ZNK5QChar7isPunctEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QString QChar::decomposition();
  fn _ZNK5QChar13decompositionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QChar::QChar(uint rc);
  fn dector_ZN5QCharC1Ej(arg0: c_uint) -> *mut c_void;
  fn _ZN5QCharC1Ej(qthis: u64 /* *mut c_void*/, arg0: c_uint);
  // proto:  void QChar::QChar(int rc);
  fn dector_ZN5QCharC1Ei(arg0: c_int) -> *mut c_void;
  fn _ZN5QCharC1Ei(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  bool QChar::isSpace();
  fn _ZNK5QChar7isSpaceEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QChar::QChar(short rc);
  fn dector_ZN5QCharC1Es(arg0: c_short) -> *mut c_void;
  fn _ZN5QCharC1Es(qthis: u64 /* *mut c_void*/, arg0: c_short);
  // proto:  void QChar::QChar();
  fn dector_ZN5QCharC1Ev() -> *mut c_void;
  fn _ZN5QCharC1Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QChar::QChar(ushort rc);
  fn dector_ZN5QCharC1Et(arg0: c_ushort) -> *mut c_void;
  fn _ZN5QCharC1Et(qthis: u64 /* *mut c_void*/, arg0: c_ushort);
  // proto:  bool QChar::isUpper();
  fn _ZNK5QChar7isUpperEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto: static unsigned char QChar::combiningClass(uint ucs4);
  fn _ZN5QChar14combiningClassEj(arg0: c_uint) -> c_uchar;
  // proto:  bool QChar::isNonCharacter();
  fn _ZNK5QChar14isNonCharacterEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto: static bool QChar::isLetterOrNumber(uint ucs4);
  fn demth_ZN5QChar16isLetterOrNumberEj(arg0: c_uint) -> c_char;
  // proto: static bool QChar::isDigit(uint ucs4);
  fn demth_ZN5QChar7isDigitEj(arg0: c_uint) -> c_char;
  // proto: static bool QChar::isPunct(uint ucs4);
  fn _ZN5QChar7isPunctEj(arg0: c_uint) -> c_char;
  // proto:  bool QChar::isTitleCase();
  fn _ZNK5QChar11isTitleCaseEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QChar::isLetter();
  fn _ZNK5QChar8isLetterEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  unsigned char QChar::combiningClass();
  fn demth_ZNK5QChar14combiningClassEv(qthis: u64 /* *mut c_void*/) -> c_uchar;
  // proto:  bool QChar::isHighSurrogate();
  fn _ZNK5QChar15isHighSurrogateEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto: static ushort QChar::highSurrogate(uint ucs4);
  fn demth_ZN5QChar13highSurrogateEj(arg0: c_uint) -> c_ushort;
  // proto: static bool QChar::requiresSurrogates(uint ucs4);
  fn demth_ZN5QChar18requiresSurrogatesEj(arg0: c_uint) -> c_char;
  // proto:  bool QChar::isLetterOrNumber();
  fn _ZNK5QChar16isLetterOrNumberEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  ushort & QChar::unicode();
  fn demth_ZN5QChar7unicodeEv(qthis: u64 /* *mut c_void*/);
  // proto: static bool QChar::isLowSurrogate(uint ucs4);
  fn demth_ZN5QChar14isLowSurrogateEj(arg0: c_uint) -> c_char;
  // proto: static bool QChar::isNumber(uint ucs4);
  fn demth_ZN5QChar8isNumberEj(arg0: c_uint) -> c_char;
  // proto:  QChar QChar::toLower();
  fn demth_ZNK5QChar7toLowerEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QChar::QChar(uchar c);
  fn dector_ZN5QCharC1Eh(arg0: c_uchar) -> *mut c_void;
  fn _ZN5QCharC1Eh(qthis: u64 /* *mut c_void*/, arg0: c_uchar);
  // proto: static bool QChar::isLetter(uint ucs4);
  fn demth_ZN5QChar8isLetterEj(arg0: c_uint) -> c_char;
  // proto:  QChar QChar::toUpper();
  fn demth_ZNK5QChar7toUpperEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static bool QChar::isSpace(uint ucs4);
  fn demth_ZN5QChar7isSpaceEj(arg0: c_uint) -> c_char;
  // proto: static QChar QChar::fromLatin1(char c);
  fn _ZN5QChar10fromLatin1Ec(arg0: c_char) -> *mut c_void;
  // proto: static bool QChar::isSurrogate(uint ucs4);
  fn demth_ZN5QChar11isSurrogateEj(arg0: c_uint) -> c_char;
  // proto: static bool QChar::isMark(uint ucs4);
  fn _ZN5QChar6isMarkEj(arg0: c_uint) -> c_char;
  // proto: static bool QChar::isNonCharacter(uint ucs4);
  fn demth_ZN5QChar14isNonCharacterEj(arg0: c_uint) -> c_char;
  // proto:  char QChar::toLatin1();
  fn _ZNK5QChar8toLatin1Ev(qthis: u64 /* *mut c_void*/) -> c_char;
} // <= ext block end

// body block begin =>
// class sizeof(QLatin1Char)=1
#[derive(Default)]
pub struct QLatin1Char {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QChar)=2
#[derive(Default)]
pub struct QChar {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QLatin1Char {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QLatin1Char {
    return QLatin1Char{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  ushort QLatin1Char::unicode();
impl /*struct*/ QLatin1Char {
  pub fn unicode<RetType, T: QLatin1Char_unicode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.unicode(self);
    // return 1;
  }
}

pub trait QLatin1Char_unicode<RetType> {
  fn unicode(self , rsthis: & QLatin1Char) -> RetType;
}

  // proto:  ushort QLatin1Char::unicode();
impl<'a> /*trait*/ QLatin1Char_unicode<u16> for () {
  fn unicode(self , rsthis: & QLatin1Char) -> u16 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLatin1Char7unicodeEv()};
    let mut ret = unsafe {_ZNK11QLatin1Char7unicodeEv(rsthis.qclsinst)};
    return ret as u16;
    // return 1;
  }
}

  // proto:  void QLatin1Char::QLatin1Char(char c);
impl /*struct*/ QLatin1Char {
  pub fn New<T: QLatin1Char_New>(value: T) -> QLatin1Char {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QLatin1Char_New {
  fn New(self) -> QLatin1Char;
}

  // proto:  void QLatin1Char::QLatin1Char(char c);
impl<'a> /*trait*/ QLatin1Char_New for (i8) {
  fn New(self) -> QLatin1Char {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QLatin1CharC1Ec()};
    let ctysz: c_int = unsafe{QLatin1Char_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_char;
    // unsafe {_ZN11QLatin1CharC1Ec(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN11QLatin1CharC1Ec(arg0)} as u64;
    let rsthis = QLatin1Char{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  char QLatin1Char::toLatin1();
impl /*struct*/ QLatin1Char {
  pub fn toLatin1<RetType, T: QLatin1Char_toLatin1<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toLatin1(self);
    // return 1;
  }
}

pub trait QLatin1Char_toLatin1<RetType> {
  fn toLatin1(self , rsthis: & QLatin1Char) -> RetType;
}

  // proto:  char QLatin1Char::toLatin1();
impl<'a> /*trait*/ QLatin1Char_toLatin1<i8> for () {
  fn toLatin1(self , rsthis: & QLatin1Char) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLatin1Char8toLatin1Ev()};
    let mut ret = unsafe {_ZNK11QLatin1Char8toLatin1Ev(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QChar {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QChar {
    return QChar{qclsinst: qthis, ..Default::default()};
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

  // proto:  bool QChar::hasMirrored();
impl /*struct*/ QChar {
  pub fn hasMirrored<RetType, T: QChar_hasMirrored<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasMirrored(self);
    // return 1;
  }
}

pub trait QChar_hasMirrored<RetType> {
  fn hasMirrored(self , rsthis: & QChar) -> RetType;
}

  // proto:  bool QChar::hasMirrored();
impl<'a> /*trait*/ QChar_hasMirrored<i8> for () {
  fn hasMirrored(self , rsthis: & QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar11hasMirroredEv()};
    let mut ret = unsafe {demth_ZNK5QChar11hasMirroredEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static ushort QChar::lowSurrogate(uint ucs4);
impl /*struct*/ QChar {
  pub fn lowSurrogate_s<RetType, T: QChar_lowSurrogate_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.lowSurrogate_s();
    // return 1;
  }
}

pub trait QChar_lowSurrogate_s<RetType> {
  fn lowSurrogate_s(self ) -> RetType;
}

  // proto: static ushort QChar::lowSurrogate(uint ucs4);
impl<'a> /*trait*/ QChar_lowSurrogate_s<u16> for (u32) {
  fn lowSurrogate_s(self ) -> u16 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar12lowSurrogateEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {demth_ZN5QChar12lowSurrogateEj(arg0)};
    return ret as u16;
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

  // proto:  uchar QChar::cell();
impl /*struct*/ QChar {
  pub fn cell<RetType, T: QChar_cell<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cell(self);
    // return 1;
  }
}

pub trait QChar_cell<RetType> {
  fn cell(self , rsthis: & QChar) -> RetType;
}

  // proto:  uchar QChar::cell();
impl<'a> /*trait*/ QChar_cell<u8> for () {
  fn cell(self , rsthis: & QChar) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar4cellEv()};
    let mut ret = unsafe {_ZNK5QChar4cellEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

  // proto: static uint QChar::surrogateToUcs4(QChar high, QChar low);
impl /*struct*/ QChar {
  pub fn surrogateToUcs4_s<RetType, T: QChar_surrogateToUcs4_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.surrogateToUcs4_s();
    // return 1;
  }
}

pub trait QChar_surrogateToUcs4_s<RetType> {
  fn surrogateToUcs4_s(self ) -> RetType;
}

  // proto: static uint QChar::surrogateToUcs4(QChar high, QChar low);
impl<'a> /*trait*/ QChar_surrogateToUcs4_s<u32> for (QChar, QChar) {
  fn surrogateToUcs4_s(self ) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar15surrogateToUcs4ES_S_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {demth_ZN5QChar15surrogateToUcs4ES_S_(arg0, arg1)};
    return ret as u32;
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
    let mut ret = unsafe {demth_ZN5QChar11isTitleCaseEj(arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QChar::isNull();
impl /*struct*/ QChar {
  pub fn isNull<RetType, T: QChar_isNull<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QChar_isNull<RetType> {
  fn isNull(self , rsthis: & QChar) -> RetType;
}

  // proto:  bool QChar::isNull();
impl<'a> /*trait*/ QChar_isNull<i8> for () {
  fn isNull(self , rsthis: & QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar6isNullEv()};
    let mut ret = unsafe {_ZNK5QChar6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QChar::digitValue();
impl /*struct*/ QChar {
  pub fn digitValue<RetType, T: QChar_digitValue<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.digitValue(self);
    // return 1;
  }
}

pub trait QChar_digitValue<RetType> {
  fn digitValue(self , rsthis: & QChar) -> RetType;
}

  // proto:  int QChar::digitValue();
impl<'a> /*trait*/ QChar_digitValue<i32> for () {
  fn digitValue(self , rsthis: & QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar10digitValueEv()};
    let mut ret = unsafe {demth_ZNK5QChar10digitValueEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QChar QChar::toTitleCase();
impl /*struct*/ QChar {
  pub fn toTitleCase<RetType, T: QChar_toTitleCase<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toTitleCase(self);
    // return 1;
  }
}

pub trait QChar_toTitleCase<RetType> {
  fn toTitleCase(self , rsthis: & QChar) -> RetType;
}

  // proto:  QChar QChar::toTitleCase();
impl<'a> /*trait*/ QChar_toTitleCase<QChar> for () {
  fn toTitleCase(self , rsthis: & QChar) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar11toTitleCaseEv()};
    let mut ret = unsafe {demth_ZNK5QChar11toTitleCaseEv(rsthis.qclsinst)};
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
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
    let mut ret = unsafe {demth_ZN5QChar7isLowerEj(arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QChar::isLowSurrogate();
impl /*struct*/ QChar {
  pub fn isLowSurrogate<RetType, T: QChar_isLowSurrogate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isLowSurrogate(self);
    // return 1;
  }
}

pub trait QChar_isLowSurrogate<RetType> {
  fn isLowSurrogate(self , rsthis: & QChar) -> RetType;
}

  // proto:  bool QChar::isLowSurrogate();
impl<'a> /*trait*/ QChar_isLowSurrogate<i8> for () {
  fn isLowSurrogate(self , rsthis: & QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar14isLowSurrogateEv()};
    let mut ret = unsafe {_ZNK5QChar14isLowSurrogateEv(rsthis.qclsinst)};
    return ret as i8;
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

  // proto:  bool QChar::isSymbol();
impl /*struct*/ QChar {
  pub fn isSymbol<RetType, T: QChar_isSymbol<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSymbol(self);
    // return 1;
  }
}

pub trait QChar_isSymbol<RetType> {
  fn isSymbol(self , rsthis: & QChar) -> RetType;
}

  // proto:  bool QChar::isSymbol();
impl<'a> /*trait*/ QChar_isSymbol<i8> for () {
  fn isSymbol(self , rsthis: & QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar8isSymbolEv()};
    let mut ret = unsafe {demth_ZNK5QChar8isSymbolEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QChar::isLower();
impl /*struct*/ QChar {
  pub fn isLower<RetType, T: QChar_isLower<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isLower(self);
    // return 1;
  }
}

pub trait QChar_isLower<RetType> {
  fn isLower(self , rsthis: & QChar) -> RetType;
}

  // proto:  bool QChar::isLower();
impl<'a> /*trait*/ QChar_isLower<i8> for () {
  fn isLower(self , rsthis: & QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar7isLowerEv()};
    let mut ret = unsafe {_ZNK5QChar7isLowerEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QChar QChar::mirroredChar();
impl /*struct*/ QChar {
  pub fn mirroredChar<RetType, T: QChar_mirroredChar<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mirroredChar(self);
    // return 1;
  }
}

pub trait QChar_mirroredChar<RetType> {
  fn mirroredChar(self , rsthis: & QChar) -> RetType;
}

  // proto:  QChar QChar::mirroredChar();
impl<'a> /*trait*/ QChar_mirroredChar<QChar> for () {
  fn mirroredChar(self , rsthis: & QChar) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar12mirroredCharEv()};
    let mut ret = unsafe {demth_ZNK5QChar12mirroredCharEv(rsthis.qclsinst)};
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  uchar QChar::row();
impl /*struct*/ QChar {
  pub fn row<RetType, T: QChar_row<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.row(self);
    // return 1;
  }
}

pub trait QChar_row<RetType> {
  fn row(self , rsthis: & QChar) -> RetType;
}

  // proto:  uchar QChar::row();
impl<'a> /*trait*/ QChar_row<u8> for () {
  fn row(self , rsthis: & QChar) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar3rowEv()};
    let mut ret = unsafe {_ZNK5QChar3rowEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

  // proto:  bool QChar::isDigit();
impl /*struct*/ QChar {
  pub fn isDigit<RetType, T: QChar_isDigit<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isDigit(self);
    // return 1;
  }
}

pub trait QChar_isDigit<RetType> {
  fn isDigit(self , rsthis: & QChar) -> RetType;
}

  // proto:  bool QChar::isDigit();
impl<'a> /*trait*/ QChar_isDigit<i8> for () {
  fn isDigit(self , rsthis: & QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar7isDigitEv()};
    let mut ret = unsafe {_ZNK5QChar7isDigitEv(rsthis.qclsinst)};
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

  // proto:  bool QChar::isSurrogate();
impl /*struct*/ QChar {
  pub fn isSurrogate<RetType, T: QChar_isSurrogate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSurrogate(self);
    // return 1;
  }
}

pub trait QChar_isSurrogate<RetType> {
  fn isSurrogate(self , rsthis: & QChar) -> RetType;
}

  // proto:  bool QChar::isSurrogate();
impl<'a> /*trait*/ QChar_isSurrogate<i8> for () {
  fn isSurrogate(self , rsthis: & QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar11isSurrogateEv()};
    let mut ret = unsafe {_ZNK5QChar11isSurrogateEv(rsthis.qclsinst)};
    return ret as i8;
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

  // proto:  bool QChar::isNumber();
impl /*struct*/ QChar {
  pub fn isNumber<RetType, T: QChar_isNumber<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNumber(self);
    // return 1;
  }
}

pub trait QChar_isNumber<RetType> {
  fn isNumber(self , rsthis: & QChar) -> RetType;
}

  // proto:  bool QChar::isNumber();
impl<'a> /*trait*/ QChar_isNumber<i8> for () {
  fn isNumber(self , rsthis: & QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar8isNumberEv()};
    let mut ret = unsafe {_ZNK5QChar8isNumberEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static bool QChar::isHighSurrogate(uint ucs4);
impl /*struct*/ QChar {
  pub fn isHighSurrogate_s<RetType, T: QChar_isHighSurrogate_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.isHighSurrogate_s();
    // return 1;
  }
}

pub trait QChar_isHighSurrogate_s<RetType> {
  fn isHighSurrogate_s(self ) -> RetType;
}

  // proto: static bool QChar::isHighSurrogate(uint ucs4);
impl<'a> /*trait*/ QChar_isHighSurrogate_s<i8> for (u32) {
  fn isHighSurrogate_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar15isHighSurrogateEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {demth_ZN5QChar15isHighSurrogateEj(arg0)};
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

  // proto:  bool QChar::isMark();
impl /*struct*/ QChar {
  pub fn isMark<RetType, T: QChar_isMark<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isMark(self);
    // return 1;
  }
}

pub trait QChar_isMark<RetType> {
  fn isMark(self , rsthis: & QChar) -> RetType;
}

  // proto:  bool QChar::isMark();
impl<'a> /*trait*/ QChar_isMark<i8> for () {
  fn isMark(self , rsthis: & QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar6isMarkEv()};
    let mut ret = unsafe {demth_ZNK5QChar6isMarkEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static uint QChar::surrogateToUcs4(ushort high, ushort low);
impl<'a> /*trait*/ QChar_surrogateToUcs4_s<u32> for (u16, u16) {
  fn surrogateToUcs4_s(self ) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar15surrogateToUcs4Ett()};
    let arg0 = self.0  as c_ushort;
    let arg1 = self.1  as c_ushort;
    let mut ret = unsafe {demth_ZN5QChar15surrogateToUcs4Ett(arg0, arg1)};
    return ret as u32;
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
     unsafe {demth_ZN5QChar6setRowEh(rsthis.qclsinst, arg0)};
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
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
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
     unsafe {demth_ZN5QChar7setCellEh(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {demth_ZN5QChar7isUpperEj(arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QChar::QChar(uchar c, uchar r);
impl /*struct*/ QChar {
  pub fn New<T: QChar_New>(value: T) -> QChar {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QChar_New {
  fn New(self) -> QChar;
}

  // proto:  void QChar::QChar(uchar c, uchar r);
impl<'a> /*trait*/ QChar_New for (u8, u8) {
  fn New(self) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QCharC1Ehh()};
    let ctysz: c_int = unsafe{QChar_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_uchar;
    let arg1 = self.1  as c_uchar;
    // unsafe {_ZN5QCharC1Ehh(qthis, arg0, arg1)};
    let qthis: u64 = unsafe {dector_ZN5QCharC1Ehh(arg0, arg1)} as u64;
    let rsthis = QChar{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QChar QChar::toCaseFolded();
impl /*struct*/ QChar {
  pub fn toCaseFolded<RetType, T: QChar_toCaseFolded<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toCaseFolded(self);
    // return 1;
  }
}

pub trait QChar_toCaseFolded<RetType> {
  fn toCaseFolded(self , rsthis: & QChar) -> RetType;
}

  // proto:  QChar QChar::toCaseFolded();
impl<'a> /*trait*/ QChar_toCaseFolded<QChar> for () {
  fn toCaseFolded(self , rsthis: & QChar) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar12toCaseFoldedEv()};
    let mut ret = unsafe {demth_ZNK5QChar12toCaseFoldedEv(rsthis.qclsinst)};
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QChar::isPrint();
impl /*struct*/ QChar {
  pub fn isPrint<RetType, T: QChar_isPrint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isPrint(self);
    // return 1;
  }
}

pub trait QChar_isPrint<RetType> {
  fn isPrint(self , rsthis: & QChar) -> RetType;
}

  // proto:  bool QChar::isPrint();
impl<'a> /*trait*/ QChar_isPrint<i8> for () {
  fn isPrint(self , rsthis: & QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar7isPrintEv()};
    let mut ret = unsafe {demth_ZNK5QChar7isPrintEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QChar::QChar(char c);
impl<'a> /*trait*/ QChar_New for (i8) {
  fn New(self) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QCharC1Ec()};
    let ctysz: c_int = unsafe{QChar_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_char;
    // unsafe {_ZN5QCharC1Ec(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN5QCharC1Ec(arg0)} as u64;
    let rsthis = QChar{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QChar::isPunct();
impl /*struct*/ QChar {
  pub fn isPunct<RetType, T: QChar_isPunct<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isPunct(self);
    // return 1;
  }
}

pub trait QChar_isPunct<RetType> {
  fn isPunct(self , rsthis: & QChar) -> RetType;
}

  // proto:  bool QChar::isPunct();
impl<'a> /*trait*/ QChar_isPunct<i8> for () {
  fn isPunct(self , rsthis: & QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar7isPunctEv()};
    let mut ret = unsafe {demth_ZNK5QChar7isPunctEv(rsthis.qclsinst)};
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
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QChar::QChar(uint rc);
impl<'a> /*trait*/ QChar_New for (u32) {
  fn New(self) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QCharC1Ej()};
    let ctysz: c_int = unsafe{QChar_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_uint;
    // unsafe {_ZN5QCharC1Ej(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN5QCharC1Ej(arg0)} as u64;
    let rsthis = QChar{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QChar::QChar(int rc);
impl<'a> /*trait*/ QChar_New for (i32) {
  fn New(self) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QCharC1Ei()};
    let ctysz: c_int = unsafe{QChar_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_int;
    // unsafe {_ZN5QCharC1Ei(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN5QCharC1Ei(arg0)} as u64;
    let rsthis = QChar{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QChar::isSpace();
impl /*struct*/ QChar {
  pub fn isSpace<RetType, T: QChar_isSpace<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSpace(self);
    // return 1;
  }
}

pub trait QChar_isSpace<RetType> {
  fn isSpace(self , rsthis: & QChar) -> RetType;
}

  // proto:  bool QChar::isSpace();
impl<'a> /*trait*/ QChar_isSpace<i8> for () {
  fn isSpace(self , rsthis: & QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar7isSpaceEv()};
    let mut ret = unsafe {_ZNK5QChar7isSpaceEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QChar::QChar(short rc);
impl<'a> /*trait*/ QChar_New for (i16) {
  fn New(self) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QCharC1Es()};
    let ctysz: c_int = unsafe{QChar_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_short;
    // unsafe {_ZN5QCharC1Es(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN5QCharC1Es(arg0)} as u64;
    let rsthis = QChar{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QChar::QChar();
impl<'a> /*trait*/ QChar_New for () {
  fn New(self) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QCharC1Ev()};
    let ctysz: c_int = unsafe{QChar_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN5QCharC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN5QCharC1Ev()} as u64;
    let rsthis = QChar{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QChar::QChar(ushort rc);
impl<'a> /*trait*/ QChar_New for (u16) {
  fn New(self) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QCharC1Et()};
    let ctysz: c_int = unsafe{QChar_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_ushort;
    // unsafe {_ZN5QCharC1Et(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN5QCharC1Et(arg0)} as u64;
    let rsthis = QChar{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QChar::isUpper();
impl /*struct*/ QChar {
  pub fn isUpper<RetType, T: QChar_isUpper<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isUpper(self);
    // return 1;
  }
}

pub trait QChar_isUpper<RetType> {
  fn isUpper(self , rsthis: & QChar) -> RetType;
}

  // proto:  bool QChar::isUpper();
impl<'a> /*trait*/ QChar_isUpper<i8> for () {
  fn isUpper(self , rsthis: & QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar7isUpperEv()};
    let mut ret = unsafe {_ZNK5QChar7isUpperEv(rsthis.qclsinst)};
    return ret as i8;
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

  // proto:  bool QChar::isNonCharacter();
impl /*struct*/ QChar {
  pub fn isNonCharacter<RetType, T: QChar_isNonCharacter<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNonCharacter(self);
    // return 1;
  }
}

pub trait QChar_isNonCharacter<RetType> {
  fn isNonCharacter(self , rsthis: & QChar) -> RetType;
}

  // proto:  bool QChar::isNonCharacter();
impl<'a> /*trait*/ QChar_isNonCharacter<i8> for () {
  fn isNonCharacter(self , rsthis: & QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar14isNonCharacterEv()};
    let mut ret = unsafe {_ZNK5QChar14isNonCharacterEv(rsthis.qclsinst)};
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
    let mut ret = unsafe {demth_ZN5QChar16isLetterOrNumberEj(arg0)};
    return ret as i8;
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
    let mut ret = unsafe {demth_ZN5QChar7isDigitEj(arg0)};
    return ret as i8;
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

  // proto:  bool QChar::isTitleCase();
impl /*struct*/ QChar {
  pub fn isTitleCase<RetType, T: QChar_isTitleCase<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isTitleCase(self);
    // return 1;
  }
}

pub trait QChar_isTitleCase<RetType> {
  fn isTitleCase(self , rsthis: & QChar) -> RetType;
}

  // proto:  bool QChar::isTitleCase();
impl<'a> /*trait*/ QChar_isTitleCase<i8> for () {
  fn isTitleCase(self , rsthis: & QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar11isTitleCaseEv()};
    let mut ret = unsafe {_ZNK5QChar11isTitleCaseEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QChar::isLetter();
impl /*struct*/ QChar {
  pub fn isLetter<RetType, T: QChar_isLetter<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isLetter(self);
    // return 1;
  }
}

pub trait QChar_isLetter<RetType> {
  fn isLetter(self , rsthis: & QChar) -> RetType;
}

  // proto:  bool QChar::isLetter();
impl<'a> /*trait*/ QChar_isLetter<i8> for () {
  fn isLetter(self , rsthis: & QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar8isLetterEv()};
    let mut ret = unsafe {_ZNK5QChar8isLetterEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  unsigned char QChar::combiningClass();
impl /*struct*/ QChar {
  pub fn combiningClass<RetType, T: QChar_combiningClass<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.combiningClass(self);
    // return 1;
  }
}

pub trait QChar_combiningClass<RetType> {
  fn combiningClass(self , rsthis: & QChar) -> RetType;
}

  // proto:  unsigned char QChar::combiningClass();
impl<'a> /*trait*/ QChar_combiningClass<u8> for () {
  fn combiningClass(self , rsthis: & QChar) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar14combiningClassEv()};
    let mut ret = unsafe {demth_ZNK5QChar14combiningClassEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

  // proto:  bool QChar::isHighSurrogate();
impl /*struct*/ QChar {
  pub fn isHighSurrogate<RetType, T: QChar_isHighSurrogate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isHighSurrogate(self);
    // return 1;
  }
}

pub trait QChar_isHighSurrogate<RetType> {
  fn isHighSurrogate(self , rsthis: & QChar) -> RetType;
}

  // proto:  bool QChar::isHighSurrogate();
impl<'a> /*trait*/ QChar_isHighSurrogate<i8> for () {
  fn isHighSurrogate(self , rsthis: & QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar15isHighSurrogateEv()};
    let mut ret = unsafe {_ZNK5QChar15isHighSurrogateEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static ushort QChar::highSurrogate(uint ucs4);
impl /*struct*/ QChar {
  pub fn highSurrogate_s<RetType, T: QChar_highSurrogate_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.highSurrogate_s();
    // return 1;
  }
}

pub trait QChar_highSurrogate_s<RetType> {
  fn highSurrogate_s(self ) -> RetType;
}

  // proto: static ushort QChar::highSurrogate(uint ucs4);
impl<'a> /*trait*/ QChar_highSurrogate_s<u16> for (u32) {
  fn highSurrogate_s(self ) -> u16 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar13highSurrogateEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {demth_ZN5QChar13highSurrogateEj(arg0)};
    return ret as u16;
    // return 1;
  }
}

  // proto: static bool QChar::requiresSurrogates(uint ucs4);
impl /*struct*/ QChar {
  pub fn requiresSurrogates_s<RetType, T: QChar_requiresSurrogates_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.requiresSurrogates_s();
    // return 1;
  }
}

pub trait QChar_requiresSurrogates_s<RetType> {
  fn requiresSurrogates_s(self ) -> RetType;
}

  // proto: static bool QChar::requiresSurrogates(uint ucs4);
impl<'a> /*trait*/ QChar_requiresSurrogates_s<i8> for (u32) {
  fn requiresSurrogates_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar18requiresSurrogatesEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {demth_ZN5QChar18requiresSurrogatesEj(arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QChar::isLetterOrNumber();
impl /*struct*/ QChar {
  pub fn isLetterOrNumber<RetType, T: QChar_isLetterOrNumber<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isLetterOrNumber(self);
    // return 1;
  }
}

pub trait QChar_isLetterOrNumber<RetType> {
  fn isLetterOrNumber(self , rsthis: & QChar) -> RetType;
}

  // proto:  bool QChar::isLetterOrNumber();
impl<'a> /*trait*/ QChar_isLetterOrNumber<i8> for () {
  fn isLetterOrNumber(self , rsthis: & QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar16isLetterOrNumberEv()};
    let mut ret = unsafe {_ZNK5QChar16isLetterOrNumberEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  ushort & QChar::unicode();
impl /*struct*/ QChar {
  pub fn unicode<RetType, T: QChar_unicode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.unicode(self);
    // return 1;
  }
}

pub trait QChar_unicode<RetType> {
  fn unicode(self , rsthis: & QChar) -> RetType;
}

  // proto:  ushort & QChar::unicode();
impl<'a> /*trait*/ QChar_unicode<()> for () {
  fn unicode(self , rsthis: & QChar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar7unicodeEv()};
     unsafe {demth_ZN5QChar7unicodeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static bool QChar::isLowSurrogate(uint ucs4);
impl /*struct*/ QChar {
  pub fn isLowSurrogate_s<RetType, T: QChar_isLowSurrogate_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.isLowSurrogate_s();
    // return 1;
  }
}

pub trait QChar_isLowSurrogate_s<RetType> {
  fn isLowSurrogate_s(self ) -> RetType;
}

  // proto: static bool QChar::isLowSurrogate(uint ucs4);
impl<'a> /*trait*/ QChar_isLowSurrogate_s<i8> for (u32) {
  fn isLowSurrogate_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar14isLowSurrogateEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {demth_ZN5QChar14isLowSurrogateEj(arg0)};
    return ret as i8;
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
    let mut ret = unsafe {demth_ZN5QChar8isNumberEj(arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QChar QChar::toLower();
impl /*struct*/ QChar {
  pub fn toLower<RetType, T: QChar_toLower<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toLower(self);
    // return 1;
  }
}

pub trait QChar_toLower<RetType> {
  fn toLower(self , rsthis: & QChar) -> RetType;
}

  // proto:  QChar QChar::toLower();
impl<'a> /*trait*/ QChar_toLower<QChar> for () {
  fn toLower(self , rsthis: & QChar) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar7toLowerEv()};
    let mut ret = unsafe {demth_ZNK5QChar7toLowerEv(rsthis.qclsinst)};
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QChar::QChar(uchar c);
impl<'a> /*trait*/ QChar_New for (u8) {
  fn New(self) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QCharC1Eh()};
    let ctysz: c_int = unsafe{QChar_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_uchar;
    // unsafe {_ZN5QCharC1Eh(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN5QCharC1Eh(arg0)} as u64;
    let rsthis = QChar{qclsinst: qthis, ..Default::default()};
    return rsthis;
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
    let mut ret = unsafe {demth_ZN5QChar8isLetterEj(arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QChar QChar::toUpper();
impl /*struct*/ QChar {
  pub fn toUpper<RetType, T: QChar_toUpper<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toUpper(self);
    // return 1;
  }
}

pub trait QChar_toUpper<RetType> {
  fn toUpper(self , rsthis: & QChar) -> RetType;
}

  // proto:  QChar QChar::toUpper();
impl<'a> /*trait*/ QChar_toUpper<QChar> for () {
  fn toUpper(self , rsthis: & QChar) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar7toUpperEv()};
    let mut ret = unsafe {demth_ZNK5QChar7toUpperEv(rsthis.qclsinst)};
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
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
    let mut ret = unsafe {demth_ZN5QChar7isSpaceEj(arg0)};
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
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static bool QChar::isSurrogate(uint ucs4);
impl /*struct*/ QChar {
  pub fn isSurrogate_s<RetType, T: QChar_isSurrogate_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.isSurrogate_s();
    // return 1;
  }
}

pub trait QChar_isSurrogate_s<RetType> {
  fn isSurrogate_s(self ) -> RetType;
}

  // proto: static bool QChar::isSurrogate(uint ucs4);
impl<'a> /*trait*/ QChar_isSurrogate_s<i8> for (u32) {
  fn isSurrogate_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar11isSurrogateEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {demth_ZN5QChar11isSurrogateEj(arg0)};
    return ret as i8;
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

  // proto: static bool QChar::isNonCharacter(uint ucs4);
impl /*struct*/ QChar {
  pub fn isNonCharacter_s<RetType, T: QChar_isNonCharacter_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.isNonCharacter_s();
    // return 1;
  }
}

pub trait QChar_isNonCharacter_s<RetType> {
  fn isNonCharacter_s(self ) -> RetType;
}

  // proto: static bool QChar::isNonCharacter(uint ucs4);
impl<'a> /*trait*/ QChar_isNonCharacter_s<i8> for (u32) {
  fn isNonCharacter_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar14isNonCharacterEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {demth_ZN5QChar14isNonCharacterEj(arg0)};
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

