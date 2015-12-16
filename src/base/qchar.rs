// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  bool QChar::hasMirrored();
  fn _ZNK5QChar11hasMirroredEv(qthis: *mut c_void) -> int8_t;
  // proto: static unsigned short QChar::lowSurrogate(uint ucs4);
  fn _ZN5QChar12lowSurrogateEj(arg0: c_uint) -> c_ushort;
  // proto: static bool QChar::isSymbol(uint ucs4);
  fn _ZN5QChar8isSymbolEj(arg0: c_uint) -> int8_t;
  // proto:  unsigned char QChar::cell();
  fn _ZNK5QChar4cellEv(qthis: *mut c_void) -> c_uchar;
  // proto: static unsigned int QChar::surrogateToUcs4(QChar high, QChar low);
  fn _ZN5QChar15surrogateToUcs4ES_S_(arg0: *mut c_void, arg1: *mut c_void) -> c_uint;
  // proto: static bool QChar::isTitleCase(uint ucs4);
  fn _ZN5QChar11isTitleCaseEj(arg0: c_uint) -> int8_t;
  // proto:  bool QChar::isNull();
  fn _ZNK5QChar6isNullEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QChar::digitValue();
  fn _ZNK5QChar10digitValueEv(qthis: *mut c_void) -> c_int;
  // proto: static bool QChar::isLower(uint ucs4);
  fn _ZN5QChar7isLowerEj(arg0: c_uint) -> int8_t;
  // proto:  bool QChar::isLowSurrogate();
  fn _ZNK5QChar14isLowSurrogateEv(qthis: *mut c_void) -> int8_t;
  // proto: static bool QChar::isPrint(uint ucs4);
  fn _ZN5QChar7isPrintEj(arg0: c_uint) -> int8_t;
  // proto:  bool QChar::isSymbol();
  fn _ZNK5QChar8isSymbolEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QChar::isLower();
  fn _ZNK5QChar7isLowerEv(qthis: *mut c_void) -> int8_t;
  // proto:  unsigned char QChar::row();
  fn _ZNK5QChar3rowEv(qthis: *mut c_void) -> c_uchar;
  // proto:  bool QChar::isDigit();
  fn _ZNK5QChar7isDigitEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QChar::isSurrogate();
  fn _ZNK5QChar11isSurrogateEv(qthis: *mut c_void) -> int8_t;
  // proto: static bool QChar::hasMirrored(uint ucs4);
  fn _ZN5QChar11hasMirroredEj(arg0: c_uint) -> int8_t;
  // proto:  bool QChar::isNumber();
  fn _ZNK5QChar8isNumberEv(qthis: *mut c_void) -> int8_t;
  // proto: static bool QChar::isHighSurrogate(uint ucs4);
  fn _ZN5QChar15isHighSurrogateEj(arg0: c_uint) -> int8_t;
  // proto:  bool QChar::isMark();
  fn _ZNK5QChar6isMarkEv(qthis: *mut c_void) -> int8_t;
  // proto: static unsigned int QChar::surrogateToUcs4(ushort high, ushort low);
  fn _ZN5QChar15surrogateToUcs4Ett(arg0: c_ushort, arg1: c_ushort) -> c_uint;
  // proto:  void QChar::setRow(uchar row);
  fn _ZN5QChar6setRowEh(qthis: *mut c_void, arg0: c_uchar) ;
  // proto: static QString QChar::decomposition(uint ucs4);
  fn _ZN5QChar13decompositionEj(arg0: c_uint) -> *mut c_void;
  // proto: static int QChar::digitValue(uint ucs4);
  fn _ZN5QChar10digitValueEj(arg0: c_uint) -> c_int;
  // proto:  void QChar::setCell(uchar cell);
  fn _ZN5QChar7setCellEh(qthis: *mut c_void, arg0: c_uchar) ;
  // proto: static bool QChar::isUpper(uint ucs4);
  fn _ZN5QChar7isUpperEj(arg0: c_uint) -> int8_t;
  // proto:  void QChar::NewQChar(uchar c, uchar r);
  fn _ZN5QCharC1Ehh(qthis: *mut c_void, arg0: c_uchar, arg1: c_uchar) ;
  // proto:  bool QChar::isPrint();
  fn _ZNK5QChar7isPrintEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QChar::NewQChar(char c);
  fn _ZN5QCharC1Ec(qthis: *mut c_void, arg0: c_char) ;
  // proto:  bool QChar::isPunct();
  fn _ZNK5QChar7isPunctEv(qthis: *mut c_void) -> int8_t;
  // proto:  QString QChar::decomposition();
  fn _ZNK5QChar13decompositionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QChar::NewQChar(uint rc);
  fn _ZN5QCharC1Ej(qthis: *mut c_void, arg0: c_uint) ;
  // proto:  void QChar::NewQChar(int rc);
  fn _ZN5QCharC1Ei(qthis: *mut c_void, arg0: c_int) ;
  // proto:  bool QChar::isSpace();
  fn _ZNK5QChar7isSpaceEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QChar::NewQChar(short rc);
  fn _ZN5QCharC1Es(qthis: *mut c_void, arg0: c_short) ;
  // proto:  void QChar::NewQChar();
  fn _ZN5QCharC1Ev(qthis: *mut c_void) ;
  // proto:  void QChar::NewQChar(ushort rc);
  fn _ZN5QCharC1Et(qthis: *mut c_void, arg0: c_ushort) ;
  // proto:  bool QChar::isUpper();
  fn _ZNK5QChar7isUpperEv(qthis: *mut c_void) -> int8_t;
  // proto: static unsigned char QChar::combiningClass(uint ucs4);
  fn _ZN5QChar14combiningClassEj(arg0: c_uint) -> c_uchar;
  // proto:  bool QChar::isNonCharacter();
  fn _ZNK5QChar14isNonCharacterEv(qthis: *mut c_void) -> int8_t;
  // proto: static bool QChar::isLetterOrNumber(uint ucs4);
  fn _ZN5QChar16isLetterOrNumberEj(arg0: c_uint) -> int8_t;
  // proto: static bool QChar::isDigit(uint ucs4);
  fn _ZN5QChar7isDigitEj(arg0: c_uint) -> int8_t;
  // proto: static bool QChar::isPunct(uint ucs4);
  fn _ZN5QChar7isPunctEj(arg0: c_uint) -> int8_t;
  // proto:  bool QChar::isTitleCase();
  fn _ZNK5QChar11isTitleCaseEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QChar::isLetter();
  fn _ZNK5QChar8isLetterEv(qthis: *mut c_void) -> int8_t;
  // proto:  unsigned char QChar::combiningClass();
  fn _ZNK5QChar14combiningClassEv(qthis: *mut c_void) -> c_uchar;
  // proto:  bool QChar::isHighSurrogate();
  fn _ZNK5QChar15isHighSurrogateEv(qthis: *mut c_void) -> int8_t;
  // proto: static unsigned short QChar::highSurrogate(uint ucs4);
  fn _ZN5QChar13highSurrogateEj(arg0: c_uint) -> c_ushort;
  // proto: static bool QChar::requiresSurrogates(uint ucs4);
  fn _ZN5QChar18requiresSurrogatesEj(arg0: c_uint) -> int8_t;
  // proto:  bool QChar::isLetterOrNumber();
  fn _ZNK5QChar16isLetterOrNumberEv(qthis: *mut c_void) -> int8_t;
  // proto:  ushort & QChar::unicode();
  fn _ZN5QChar7unicodeEv(qthis: *mut c_void) ;
  // proto: static bool QChar::isLowSurrogate(uint ucs4);
  fn _ZN5QChar14isLowSurrogateEj(arg0: c_uint) -> int8_t;
  // proto: static bool QChar::isNumber(uint ucs4);
  fn _ZN5QChar8isNumberEj(arg0: c_uint) -> int8_t;
  // proto:  void QChar::NewQChar(uchar c);
  fn _ZN5QCharC1Eh(qthis: *mut c_void, arg0: c_uchar) ;
  // proto: static bool QChar::isLetter(uint ucs4);
  fn _ZN5QChar8isLetterEj(arg0: c_uint) -> int8_t;
  // proto: static bool QChar::isSpace(uint ucs4);
  fn _ZN5QChar7isSpaceEj(arg0: c_uint) -> int8_t;
  // proto: static QChar QChar::fromLatin1(char c);
  fn _ZN5QChar10fromLatin1Ec(arg0: c_char) -> *mut c_void;
  // proto: static bool QChar::isSurrogate(uint ucs4);
  fn _ZN5QChar11isSurrogateEj(arg0: c_uint) -> int8_t;
  // proto: static bool QChar::isMark(uint ucs4);
  fn _ZN5QChar6isMarkEj(arg0: c_uint) -> int8_t;
  // proto: static bool QChar::isNonCharacter(uint ucs4);
  fn _ZN5QChar14isNonCharacterEj(arg0: c_uint) -> int8_t;
  // proto:  char QChar::toLatin1();
  fn _ZNK5QChar8toLatin1Ev(qthis: *mut c_void) -> c_char;
}

// body block begin
// class sizeof(QChar)=2
pub struct QChar {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QChar {
  pub fn hasMirrored<T: QChar_hasMirrored>(&mut self, value: T) -> i8 {
    return value.hasMirrored(self);
    // return 1;
  }
}

pub trait QChar_hasMirrored {
  fn hasMirrored(self, rsthis: &mut QChar) -> i8;
}

// proto:  bool QChar::hasMirrored();
impl<'a> /*trait*/ QChar_hasMirrored for () {
  fn hasMirrored(self, rsthis: &mut QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar11hasMirroredEv()};
    let mut ret = unsafe {_ZNK5QChar11hasMirroredEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QChar {
  pub fn lowSurrogate<T: QChar_lowSurrogate>(&mut self, value: T) -> u16 {
    return value.lowSurrogate(self);
    // return 1;
  }
}

pub trait QChar_lowSurrogate {
  fn lowSurrogate(self, rsthis: &mut QChar) -> u16;
}

// proto: static unsigned short QChar::lowSurrogate(uint ucs4);
impl<'a> /*trait*/ QChar_lowSurrogate for (u32) {
  fn lowSurrogate(self, rsthis: &mut QChar) -> u16 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar12lowSurrogateEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar12lowSurrogateEj(arg0)};
    return ret as u16;
    // return 1;
  }
}

impl /*struct*/ QChar {
  pub fn isSymbol<T: QChar_isSymbol>(&mut self, value: T) -> i8 {
    return value.isSymbol(self);
    // return 1;
  }
}

pub trait QChar_isSymbol {
  fn isSymbol(self, rsthis: &mut QChar) -> i8;
}

// proto: static bool QChar::isSymbol(uint ucs4);
impl<'a> /*trait*/ QChar_isSymbol for (u32) {
  fn isSymbol(self, rsthis: &mut QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar8isSymbolEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar8isSymbolEj(arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QChar {
  pub fn cell<T: QChar_cell>(&mut self, value: T) -> u8 {
    return value.cell(self);
    // return 1;
  }
}

pub trait QChar_cell {
  fn cell(self, rsthis: &mut QChar) -> u8;
}

// proto:  unsigned char QChar::cell();
impl<'a> /*trait*/ QChar_cell for () {
  fn cell(self, rsthis: &mut QChar) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar4cellEv()};
    let mut ret = unsafe {_ZNK5QChar4cellEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

impl /*struct*/ QChar {
  pub fn surrogateToUcs4<T: QChar_surrogateToUcs4>(&mut self, value: T) -> u32 {
    return value.surrogateToUcs4(self);
    // return 1;
  }
}

pub trait QChar_surrogateToUcs4 {
  fn surrogateToUcs4(self, rsthis: &mut QChar) -> u32;
}

// proto: static unsigned int QChar::surrogateToUcs4(QChar high, QChar low);
impl<'a> /*trait*/ QChar_surrogateToUcs4 for (QChar, QChar) {
  fn surrogateToUcs4(self, rsthis: &mut QChar) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar15surrogateToUcs4ES_S_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QChar15surrogateToUcs4ES_S_(arg0, arg1)};
    return ret as u32;
    // return 1;
  }
}

impl /*struct*/ QChar {
  pub fn isTitleCase<T: QChar_isTitleCase>(&mut self, value: T) -> i8 {
    return value.isTitleCase(self);
    // return 1;
  }
}

pub trait QChar_isTitleCase {
  fn isTitleCase(self, rsthis: &mut QChar) -> i8;
}

// proto: static bool QChar::isTitleCase(uint ucs4);
impl<'a> /*trait*/ QChar_isTitleCase for (u32) {
  fn isTitleCase(self, rsthis: &mut QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar11isTitleCaseEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar11isTitleCaseEj(arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QChar {
  pub fn isNull<T: QChar_isNull>(&mut self, value: T) -> i8 {
    return value.isNull(self);
    // return 1;
  }
}

pub trait QChar_isNull {
  fn isNull(self, rsthis: &mut QChar) -> i8;
}

// proto:  bool QChar::isNull();
impl<'a> /*trait*/ QChar_isNull for () {
  fn isNull(self, rsthis: &mut QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar6isNullEv()};
    let mut ret = unsafe {_ZNK5QChar6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QChar {
  pub fn digitValue<T: QChar_digitValue>(&mut self, value: T) -> i32 {
    return value.digitValue(self);
    // return 1;
  }
}

pub trait QChar_digitValue {
  fn digitValue(self, rsthis: &mut QChar) -> i32;
}

// proto:  int QChar::digitValue();
impl<'a> /*trait*/ QChar_digitValue for () {
  fn digitValue(self, rsthis: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar10digitValueEv()};
    let mut ret = unsafe {_ZNK5QChar10digitValueEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QChar {
  pub fn isLower<T: QChar_isLower>(&mut self, value: T) -> i8 {
    return value.isLower(self);
    // return 1;
  }
}

pub trait QChar_isLower {
  fn isLower(self, rsthis: &mut QChar) -> i8;
}

// proto: static bool QChar::isLower(uint ucs4);
impl<'a> /*trait*/ QChar_isLower for (u32) {
  fn isLower(self, rsthis: &mut QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar7isLowerEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar7isLowerEj(arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QChar {
  pub fn isLowSurrogate<T: QChar_isLowSurrogate>(&mut self, value: T) -> i8 {
    return value.isLowSurrogate(self);
    // return 1;
  }
}

pub trait QChar_isLowSurrogate {
  fn isLowSurrogate(self, rsthis: &mut QChar) -> i8;
}

// proto:  bool QChar::isLowSurrogate();
impl<'a> /*trait*/ QChar_isLowSurrogate for () {
  fn isLowSurrogate(self, rsthis: &mut QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar14isLowSurrogateEv()};
    let mut ret = unsafe {_ZNK5QChar14isLowSurrogateEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QChar {
  pub fn isPrint<T: QChar_isPrint>(&mut self, value: T) -> i8 {
    return value.isPrint(self);
    // return 1;
  }
}

pub trait QChar_isPrint {
  fn isPrint(self, rsthis: &mut QChar) -> i8;
}

// proto: static bool QChar::isPrint(uint ucs4);
impl<'a> /*trait*/ QChar_isPrint for (u32) {
  fn isPrint(self, rsthis: &mut QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar7isPrintEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar7isPrintEj(arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto:  bool QChar::isSymbol();
impl<'a> /*trait*/ QChar_isSymbol for () {
  fn isSymbol(self, rsthis: &mut QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar8isSymbolEv()};
    let mut ret = unsafe {_ZNK5QChar8isSymbolEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  bool QChar::isLower();
impl<'a> /*trait*/ QChar_isLower for () {
  fn isLower(self, rsthis: &mut QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar7isLowerEv()};
    let mut ret = unsafe {_ZNK5QChar7isLowerEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QChar {
  pub fn row<T: QChar_row>(&mut self, value: T) -> u8 {
    return value.row(self);
    // return 1;
  }
}

pub trait QChar_row {
  fn row(self, rsthis: &mut QChar) -> u8;
}

// proto:  unsigned char QChar::row();
impl<'a> /*trait*/ QChar_row for () {
  fn row(self, rsthis: &mut QChar) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar3rowEv()};
    let mut ret = unsafe {_ZNK5QChar3rowEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

impl /*struct*/ QChar {
  pub fn isDigit<T: QChar_isDigit>(&mut self, value: T) -> i8 {
    return value.isDigit(self);
    // return 1;
  }
}

pub trait QChar_isDigit {
  fn isDigit(self, rsthis: &mut QChar) -> i8;
}

// proto:  bool QChar::isDigit();
impl<'a> /*trait*/ QChar_isDigit for () {
  fn isDigit(self, rsthis: &mut QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar7isDigitEv()};
    let mut ret = unsafe {_ZNK5QChar7isDigitEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QChar {
  pub fn isSurrogate<T: QChar_isSurrogate>(&mut self, value: T) -> i8 {
    return value.isSurrogate(self);
    // return 1;
  }
}

pub trait QChar_isSurrogate {
  fn isSurrogate(self, rsthis: &mut QChar) -> i8;
}

// proto:  bool QChar::isSurrogate();
impl<'a> /*trait*/ QChar_isSurrogate for () {
  fn isSurrogate(self, rsthis: &mut QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar11isSurrogateEv()};
    let mut ret = unsafe {_ZNK5QChar11isSurrogateEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: static bool QChar::hasMirrored(uint ucs4);
impl<'a> /*trait*/ QChar_hasMirrored for (u32) {
  fn hasMirrored(self, rsthis: &mut QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar11hasMirroredEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar11hasMirroredEj(arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QChar {
  pub fn isNumber<T: QChar_isNumber>(&mut self, value: T) -> i8 {
    return value.isNumber(self);
    // return 1;
  }
}

pub trait QChar_isNumber {
  fn isNumber(self, rsthis: &mut QChar) -> i8;
}

// proto:  bool QChar::isNumber();
impl<'a> /*trait*/ QChar_isNumber for () {
  fn isNumber(self, rsthis: &mut QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar8isNumberEv()};
    let mut ret = unsafe {_ZNK5QChar8isNumberEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QChar {
  pub fn isHighSurrogate<T: QChar_isHighSurrogate>(&mut self, value: T) -> i8 {
    return value.isHighSurrogate(self);
    // return 1;
  }
}

pub trait QChar_isHighSurrogate {
  fn isHighSurrogate(self, rsthis: &mut QChar) -> i8;
}

// proto: static bool QChar::isHighSurrogate(uint ucs4);
impl<'a> /*trait*/ QChar_isHighSurrogate for (u32) {
  fn isHighSurrogate(self, rsthis: &mut QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar15isHighSurrogateEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar15isHighSurrogateEj(arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QChar {
  pub fn isMark<T: QChar_isMark>(&mut self, value: T) -> i8 {
    return value.isMark(self);
    // return 1;
  }
}

pub trait QChar_isMark {
  fn isMark(self, rsthis: &mut QChar) -> i8;
}

// proto:  bool QChar::isMark();
impl<'a> /*trait*/ QChar_isMark for () {
  fn isMark(self, rsthis: &mut QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar6isMarkEv()};
    let mut ret = unsafe {_ZNK5QChar6isMarkEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: static unsigned int QChar::surrogateToUcs4(ushort high, ushort low);
impl<'a> /*trait*/ QChar_surrogateToUcs4 for (u16, u16) {
  fn surrogateToUcs4(self, rsthis: &mut QChar) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar15surrogateToUcs4Ett()};
    let arg0 = self.0  as c_ushort;
    let arg1 = self.1  as c_ushort;
    let mut ret = unsafe {_ZN5QChar15surrogateToUcs4Ett(arg0, arg1)};
    return ret as u32;
    // return 1;
  }
}

impl /*struct*/ QChar {
  pub fn setRow<T: QChar_setRow>(&mut self, value: T)  {
     value.setRow(self);
    // return 1;
  }
}

pub trait QChar_setRow {
  fn setRow(self, rsthis: &mut QChar) ;
}

// proto:  void QChar::setRow(uchar row);
impl<'a> /*trait*/ QChar_setRow for (u8) {
  fn setRow(self, rsthis: &mut QChar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar6setRowEh()};
    let arg0 = self  as c_uchar;
     unsafe {_ZN5QChar6setRowEh(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QChar {
  pub fn decomposition<T: QChar_decomposition>(&mut self, value: T) -> QString {
    return value.decomposition(self);
    // return 1;
  }
}

pub trait QChar_decomposition {
  fn decomposition(self, rsthis: &mut QChar) -> QString;
}

// proto: static QString QChar::decomposition(uint ucs4);
impl<'a> /*trait*/ QChar_decomposition for (u32) {
  fn decomposition(self, rsthis: &mut QChar) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar13decompositionEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar13decompositionEj(arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: static int QChar::digitValue(uint ucs4);
impl<'a> /*trait*/ QChar_digitValue for (u32) {
  fn digitValue(self, rsthis: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar10digitValueEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar10digitValueEj(arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QChar {
  pub fn setCell<T: QChar_setCell>(&mut self, value: T)  {
     value.setCell(self);
    // return 1;
  }
}

pub trait QChar_setCell {
  fn setCell(self, rsthis: &mut QChar) ;
}

// proto:  void QChar::setCell(uchar cell);
impl<'a> /*trait*/ QChar_setCell for (u8) {
  fn setCell(self, rsthis: &mut QChar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar7setCellEh()};
    let arg0 = self  as c_uchar;
     unsafe {_ZN5QChar7setCellEh(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QChar {
  pub fn isUpper<T: QChar_isUpper>(&mut self, value: T) -> i8 {
    return value.isUpper(self);
    // return 1;
  }
}

pub trait QChar_isUpper {
  fn isUpper(self, rsthis: &mut QChar) -> i8;
}

// proto: static bool QChar::isUpper(uint ucs4);
impl<'a> /*trait*/ QChar_isUpper for (u32) {
  fn isUpper(self, rsthis: &mut QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar7isUpperEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar7isUpperEj(arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QChar {
  pub fn NewQChar<T: QChar_NewQChar>(value: T) -> QChar {
    let rsthis = value.NewQChar();
    return rsthis;
    // return 1;
  }
}

pub trait QChar_NewQChar {
  fn NewQChar(self) -> QChar;
}

// proto: void QChar::NewQChar(uchar c, uchar r);
impl<'a> /*trait*/ QChar_NewQChar for (u8, u8) {
  fn NewQChar(self) -> QChar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QCharC1Ehh()};
    let arg0 = self.0  as c_uchar;
    let arg1 = self.1  as c_uchar;
    unsafe {_ZN5QCharC1Ehh(qthis, arg0, arg1)};
    let rsthis = QChar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  bool QChar::isPrint();
impl<'a> /*trait*/ QChar_isPrint for () {
  fn isPrint(self, rsthis: &mut QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar7isPrintEv()};
    let mut ret = unsafe {_ZNK5QChar7isPrintEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QChar::NewQChar(char c);
impl<'a> /*trait*/ QChar_NewQChar for (i8) {
  fn NewQChar(self) -> QChar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QCharC1Ec()};
    let arg0 = self  as c_char;
    unsafe {_ZN5QCharC1Ec(qthis, arg0)};
    let rsthis = QChar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QChar {
  pub fn isPunct<T: QChar_isPunct>(&mut self, value: T) -> i8 {
    return value.isPunct(self);
    // return 1;
  }
}

pub trait QChar_isPunct {
  fn isPunct(self, rsthis: &mut QChar) -> i8;
}

// proto:  bool QChar::isPunct();
impl<'a> /*trait*/ QChar_isPunct for () {
  fn isPunct(self, rsthis: &mut QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar7isPunctEv()};
    let mut ret = unsafe {_ZNK5QChar7isPunctEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  QString QChar::decomposition();
impl<'a> /*trait*/ QChar_decomposition for () {
  fn decomposition(self, rsthis: &mut QChar) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar13decompositionEv()};
    let mut ret = unsafe {_ZNK5QChar13decompositionEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QChar::NewQChar(uint rc);
impl<'a> /*trait*/ QChar_NewQChar for (u32) {
  fn NewQChar(self) -> QChar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QCharC1Ej()};
    let arg0 = self  as c_uint;
    unsafe {_ZN5QCharC1Ej(qthis, arg0)};
    let rsthis = QChar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QChar::NewQChar(int rc);
impl<'a> /*trait*/ QChar_NewQChar for (i32) {
  fn NewQChar(self) -> QChar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QCharC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN5QCharC1Ei(qthis, arg0)};
    let rsthis = QChar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QChar {
  pub fn isSpace<T: QChar_isSpace>(&mut self, value: T) -> i8 {
    return value.isSpace(self);
    // return 1;
  }
}

pub trait QChar_isSpace {
  fn isSpace(self, rsthis: &mut QChar) -> i8;
}

// proto:  bool QChar::isSpace();
impl<'a> /*trait*/ QChar_isSpace for () {
  fn isSpace(self, rsthis: &mut QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar7isSpaceEv()};
    let mut ret = unsafe {_ZNK5QChar7isSpaceEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QChar::NewQChar(short rc);
impl<'a> /*trait*/ QChar_NewQChar for (i16) {
  fn NewQChar(self) -> QChar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QCharC1Es()};
    let arg0 = self  as c_short;
    unsafe {_ZN5QCharC1Es(qthis, arg0)};
    let rsthis = QChar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QChar::NewQChar();
impl<'a> /*trait*/ QChar_NewQChar for () {
  fn NewQChar(self) -> QChar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QCharC1Ev()};
    unsafe {_ZN5QCharC1Ev(qthis)};
    let rsthis = QChar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QChar::NewQChar(ushort rc);
impl<'a> /*trait*/ QChar_NewQChar for (u16) {
  fn NewQChar(self) -> QChar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QCharC1Et()};
    let arg0 = self  as c_ushort;
    unsafe {_ZN5QCharC1Et(qthis, arg0)};
    let rsthis = QChar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  bool QChar::isUpper();
impl<'a> /*trait*/ QChar_isUpper for () {
  fn isUpper(self, rsthis: &mut QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar7isUpperEv()};
    let mut ret = unsafe {_ZNK5QChar7isUpperEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QChar {
  pub fn combiningClass<T: QChar_combiningClass>(&mut self, value: T) -> u8 {
    return value.combiningClass(self);
    // return 1;
  }
}

pub trait QChar_combiningClass {
  fn combiningClass(self, rsthis: &mut QChar) -> u8;
}

// proto: static unsigned char QChar::combiningClass(uint ucs4);
impl<'a> /*trait*/ QChar_combiningClass for (u32) {
  fn combiningClass(self, rsthis: &mut QChar) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar14combiningClassEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar14combiningClassEj(arg0)};
    return ret as u8;
    // return 1;
  }
}

impl /*struct*/ QChar {
  pub fn isNonCharacter<T: QChar_isNonCharacter>(&mut self, value: T) -> i8 {
    return value.isNonCharacter(self);
    // return 1;
  }
}

pub trait QChar_isNonCharacter {
  fn isNonCharacter(self, rsthis: &mut QChar) -> i8;
}

// proto:  bool QChar::isNonCharacter();
impl<'a> /*trait*/ QChar_isNonCharacter for () {
  fn isNonCharacter(self, rsthis: &mut QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar14isNonCharacterEv()};
    let mut ret = unsafe {_ZNK5QChar14isNonCharacterEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QChar {
  pub fn isLetterOrNumber<T: QChar_isLetterOrNumber>(&mut self, value: T) -> i8 {
    return value.isLetterOrNumber(self);
    // return 1;
  }
}

pub trait QChar_isLetterOrNumber {
  fn isLetterOrNumber(self, rsthis: &mut QChar) -> i8;
}

// proto: static bool QChar::isLetterOrNumber(uint ucs4);
impl<'a> /*trait*/ QChar_isLetterOrNumber for (u32) {
  fn isLetterOrNumber(self, rsthis: &mut QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar16isLetterOrNumberEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar16isLetterOrNumberEj(arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto: static bool QChar::isDigit(uint ucs4);
impl<'a> /*trait*/ QChar_isDigit for (u32) {
  fn isDigit(self, rsthis: &mut QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar7isDigitEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar7isDigitEj(arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto: static bool QChar::isPunct(uint ucs4);
impl<'a> /*trait*/ QChar_isPunct for (u32) {
  fn isPunct(self, rsthis: &mut QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar7isPunctEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar7isPunctEj(arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto:  bool QChar::isTitleCase();
impl<'a> /*trait*/ QChar_isTitleCase for () {
  fn isTitleCase(self, rsthis: &mut QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar11isTitleCaseEv()};
    let mut ret = unsafe {_ZNK5QChar11isTitleCaseEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QChar {
  pub fn isLetter<T: QChar_isLetter>(&mut self, value: T) -> i8 {
    return value.isLetter(self);
    // return 1;
  }
}

pub trait QChar_isLetter {
  fn isLetter(self, rsthis: &mut QChar) -> i8;
}

// proto:  bool QChar::isLetter();
impl<'a> /*trait*/ QChar_isLetter for () {
  fn isLetter(self, rsthis: &mut QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar8isLetterEv()};
    let mut ret = unsafe {_ZNK5QChar8isLetterEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  unsigned char QChar::combiningClass();
impl<'a> /*trait*/ QChar_combiningClass for () {
  fn combiningClass(self, rsthis: &mut QChar) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar14combiningClassEv()};
    let mut ret = unsafe {_ZNK5QChar14combiningClassEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

// proto:  bool QChar::isHighSurrogate();
impl<'a> /*trait*/ QChar_isHighSurrogate for () {
  fn isHighSurrogate(self, rsthis: &mut QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar15isHighSurrogateEv()};
    let mut ret = unsafe {_ZNK5QChar15isHighSurrogateEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QChar {
  pub fn highSurrogate<T: QChar_highSurrogate>(&mut self, value: T) -> u16 {
    return value.highSurrogate(self);
    // return 1;
  }
}

pub trait QChar_highSurrogate {
  fn highSurrogate(self, rsthis: &mut QChar) -> u16;
}

// proto: static unsigned short QChar::highSurrogate(uint ucs4);
impl<'a> /*trait*/ QChar_highSurrogate for (u32) {
  fn highSurrogate(self, rsthis: &mut QChar) -> u16 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar13highSurrogateEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar13highSurrogateEj(arg0)};
    return ret as u16;
    // return 1;
  }
}

impl /*struct*/ QChar {
  pub fn requiresSurrogates<T: QChar_requiresSurrogates>(&mut self, value: T) -> i8 {
    return value.requiresSurrogates(self);
    // return 1;
  }
}

pub trait QChar_requiresSurrogates {
  fn requiresSurrogates(self, rsthis: &mut QChar) -> i8;
}

// proto: static bool QChar::requiresSurrogates(uint ucs4);
impl<'a> /*trait*/ QChar_requiresSurrogates for (u32) {
  fn requiresSurrogates(self, rsthis: &mut QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar18requiresSurrogatesEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar18requiresSurrogatesEj(arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto:  bool QChar::isLetterOrNumber();
impl<'a> /*trait*/ QChar_isLetterOrNumber for () {
  fn isLetterOrNumber(self, rsthis: &mut QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar16isLetterOrNumberEv()};
    let mut ret = unsafe {_ZNK5QChar16isLetterOrNumberEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QChar {
  pub fn unicode<T: QChar_unicode>(&mut self, value: T)  {
     value.unicode(self);
    // return 1;
  }
}

pub trait QChar_unicode {
  fn unicode(self, rsthis: &mut QChar) ;
}

// proto:  ushort & QChar::unicode();
impl<'a> /*trait*/ QChar_unicode for () {
  fn unicode(self, rsthis: &mut QChar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar7unicodeEv()};
     unsafe {_ZN5QChar7unicodeEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: static bool QChar::isLowSurrogate(uint ucs4);
impl<'a> /*trait*/ QChar_isLowSurrogate for (u32) {
  fn isLowSurrogate(self, rsthis: &mut QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar14isLowSurrogateEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar14isLowSurrogateEj(arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto: static bool QChar::isNumber(uint ucs4);
impl<'a> /*trait*/ QChar_isNumber for (u32) {
  fn isNumber(self, rsthis: &mut QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar8isNumberEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar8isNumberEj(arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QChar::NewQChar(uchar c);
impl<'a> /*trait*/ QChar_NewQChar for (u8) {
  fn NewQChar(self) -> QChar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QCharC1Eh()};
    let arg0 = self  as c_uchar;
    unsafe {_ZN5QCharC1Eh(qthis, arg0)};
    let rsthis = QChar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: static bool QChar::isLetter(uint ucs4);
impl<'a> /*trait*/ QChar_isLetter for (u32) {
  fn isLetter(self, rsthis: &mut QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar8isLetterEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar8isLetterEj(arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto: static bool QChar::isSpace(uint ucs4);
impl<'a> /*trait*/ QChar_isSpace for (u32) {
  fn isSpace(self, rsthis: &mut QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar7isSpaceEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar7isSpaceEj(arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QChar {
  pub fn fromLatin1<T: QChar_fromLatin1>(&mut self, value: T) -> QChar {
    return value.fromLatin1(self);
    // return 1;
  }
}

pub trait QChar_fromLatin1 {
  fn fromLatin1(self, rsthis: &mut QChar) -> QChar;
}

// proto: static QChar QChar::fromLatin1(char c);
impl<'a> /*trait*/ QChar_fromLatin1 for (i8) {
  fn fromLatin1(self, rsthis: &mut QChar) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar10fromLatin1Ec()};
    let arg0 = self  as c_char;
    let mut ret = unsafe {_ZN5QChar10fromLatin1Ec(arg0)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: static bool QChar::isSurrogate(uint ucs4);
impl<'a> /*trait*/ QChar_isSurrogate for (u32) {
  fn isSurrogate(self, rsthis: &mut QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar11isSurrogateEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar11isSurrogateEj(arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto: static bool QChar::isMark(uint ucs4);
impl<'a> /*trait*/ QChar_isMark for (u32) {
  fn isMark(self, rsthis: &mut QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar6isMarkEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar6isMarkEj(arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto: static bool QChar::isNonCharacter(uint ucs4);
impl<'a> /*trait*/ QChar_isNonCharacter for (u32) {
  fn isNonCharacter(self, rsthis: &mut QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar14isNonCharacterEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN5QChar14isNonCharacterEj(arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QChar {
  pub fn toLatin1<T: QChar_toLatin1>(&mut self, value: T) -> i8 {
    return value.toLatin1(self);
    // return 1;
  }
}

pub trait QChar_toLatin1 {
  fn toLatin1(self, rsthis: &mut QChar) -> i8;
}

// proto:  char QChar::toLatin1();
impl<'a> /*trait*/ QChar_toLatin1 for () {
  fn toLatin1(self, rsthis: &mut QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar8toLatin1Ev()};
    let mut ret = unsafe {_ZNK5QChar8toLatin1Ev(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

