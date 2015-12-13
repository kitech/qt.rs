// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN5QChar7toUpperEj(arg0: c_uint) -> i32;
  fn _ZNK5QChar11hasMirroredEv() -> i32;
  fn _ZN5QChar12lowSurrogateEj(arg0: c_uint) -> i32;
  fn _ZN5QChar8isSymbolEj(arg0: c_uint) -> i32;
  fn _ZNK5QChar4cellEv() -> i32;
  fn _ZN5QChar15surrogateToUcs4ES_S_(arg0: *mut c_void, arg1: *mut c_void) -> i32;
  fn _ZN5QChar11isTitleCaseEj(arg0: c_uint) -> i32;
  fn _ZNK5QChar6isNullEv() -> i32;
  fn _ZNK5QChar10digitValueEv() -> i32;
  fn _ZNK5QChar11toTitleCaseEv() -> i32;
  fn _ZN5QChar7isLowerEj(arg0: c_uint) -> i32;
  fn _ZNK5QChar14isLowSurrogateEv() -> i32;
  fn _ZN5QChar7isPrintEj(arg0: c_uint) -> i32;
  fn _ZNK5QChar8isSymbolEv() -> i32;
  fn _ZNK5QChar7isLowerEv() -> i32;
  fn _ZNK5QChar12mirroredCharEv() -> i32;
  fn _ZNK5QChar3rowEv() -> i32;
  fn _ZNK5QChar7isDigitEv() -> i32;
  fn _ZN5QChar11toTitleCaseEj(arg0: c_uint) -> i32;
  fn _ZNK5QChar11isSurrogateEv() -> i32;
  fn _ZN5QChar11hasMirroredEj(arg0: c_uint) -> i32;
  fn _ZNK5QChar8isNumberEv() -> i32;
  fn _ZN5QChar15isHighSurrogateEj(arg0: c_uint) -> i32;
  fn _ZN5QChar12toCaseFoldedEj(arg0: c_uint) -> i32;
  fn _ZNK5QChar6isMarkEv() -> i32;
  fn _ZN5QChar15surrogateToUcs4Ett(arg0: c_ushort, arg1: c_ushort) -> i32;
  fn _ZN5QChar7toLowerEj(arg0: c_uint) -> i32;
  fn _ZN5QChar12mirroredCharEj(arg0: c_uint) -> i32;
  fn _ZN5QChar6setRowEh(arg0: c_uchar) -> i32;
  fn _ZN5QChar13decompositionEj(arg0: c_uint) -> i32;
  fn _ZN5QChar10digitValueEj(arg0: c_uint) -> i32;
  fn _ZN5QChar7setCellEh(arg0: c_uchar) -> i32;
  fn _ZN5QChar7isUpperEj(arg0: c_uint) -> i32;
  fn _ZN5QCharC1Ehh(qthis: *mut c_void, arg0: c_uchar, arg1: c_uchar) -> i32;
  fn _ZNK5QChar12toCaseFoldedEv() -> i32;
  fn _ZNK5QChar7isPrintEv() -> i32;
  fn _ZN5QCharC1Ec(qthis: *mut c_void, arg0: c_char) -> i32;
  fn _ZNK5QChar7isPunctEv() -> i32;
  fn _ZNK5QChar13decompositionEv() -> i32;
  fn _ZN5QCharC1Ej(qthis: *mut c_void, arg0: c_uint) -> i32;
  fn _ZN5QCharC1Ei(qthis: *mut c_void, arg0: c_int) -> i32;
  fn _ZNK5QChar7isSpaceEv() -> i32;
  fn _ZN5QCharC1Es(qthis: *mut c_void, arg0: c_short) -> i32;
  fn _ZN5QCharC1Ev(qthis: *mut c_void) -> i32;
  fn _ZN5QCharC1Et(qthis: *mut c_void, arg0: c_ushort) -> i32;
  fn _ZNK5QChar7isUpperEv() -> i32;
  fn _ZN5QChar14combiningClassEj(arg0: c_uint) -> i32;
  fn _ZNK5QChar14isNonCharacterEv() -> i32;
  fn _ZN5QChar16isLetterOrNumberEj(arg0: c_uint) -> i32;
  fn _ZN5QChar7isDigitEj(arg0: c_uint) -> i32;
  fn _ZN5QChar7isPunctEj(arg0: c_uint) -> i32;
  fn _ZNK5QChar11isTitleCaseEv() -> i32;
  fn _ZNK5QChar8isLetterEv() -> i32;
  fn _ZNK5QChar14combiningClassEv() -> i32;
  fn _ZNK5QChar15isHighSurrogateEv() -> i32;
  fn _ZN5QChar13highSurrogateEj(arg0: c_uint) -> i32;
  fn _ZN5QChar18requiresSurrogatesEj(arg0: c_uint) -> i32;
  fn _ZNK5QChar16isLetterOrNumberEv() -> i32;
  fn _ZN5QChar7unicodeEv() -> i32;
  fn _ZN5QChar14isLowSurrogateEj(arg0: c_uint) -> i32;
  fn _ZN5QChar8isNumberEj(arg0: c_uint) -> i32;
  fn _ZNK5QChar7toLowerEv() -> i32;
  fn _ZN5QCharC1Eh(qthis: *mut c_void, arg0: c_uchar) -> i32;
  fn _ZN5QChar8isLetterEj(arg0: c_uint) -> i32;
  fn _ZNK5QChar7toUpperEv() -> i32;
  fn _ZN5QChar7isSpaceEj(arg0: c_uint) -> i32;
  fn _ZN5QChar10fromLatin1Ec(arg0: c_char) -> i32;
  fn _ZN5QChar11isSurrogateEj(arg0: c_uint) -> i32;
  fn _ZN5QChar6isMarkEj(arg0: c_uint) -> i32;
  fn _ZN5QChar14isNonCharacterEj(arg0: c_uint) -> i32;
  fn _ZNK5QChar8toLatin1Ev() -> i32;
}

// body block begin
// class sizeof(QChar)=2
pub struct QChar {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QChar {
  pub fn toUpper<T: QChar_toUpper>(&mut self, value: T) -> i32 {
    value.toUpper(self);
    return 1;
  }
}

pub trait QChar_toUpper {
  fn toUpper(self, this: &mut QChar) -> i32;
}

// proto: unsigned int QChar::toUpper(uint ucs4);
impl<'a> /*trait*/ QChar_toUpper for (u32) {
  fn toUpper(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar7toUpperEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN5QChar7toUpperEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QChar {
  pub fn hasMirrored<T: QChar_hasMirrored>(&mut self, value: T) -> i32 {
    value.hasMirrored(self);
    return 1;
  }
}

pub trait QChar_hasMirrored {
  fn hasMirrored(self, this: &mut QChar) -> i32;
}

// proto: bool QChar::hasMirrored();
impl<'a> /*trait*/ QChar_hasMirrored for () {
  fn hasMirrored(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar11hasMirroredEv()};
    unsafe {_ZNK5QChar11hasMirroredEv()};
    return 1;
  }
}

impl /*struct*/ QChar {
  pub fn lowSurrogate<T: QChar_lowSurrogate>(&mut self, value: T) -> i32 {
    value.lowSurrogate(self);
    return 1;
  }
}

pub trait QChar_lowSurrogate {
  fn lowSurrogate(self, this: &mut QChar) -> i32;
}

// proto: unsigned short QChar::lowSurrogate(uint ucs4);
impl<'a> /*trait*/ QChar_lowSurrogate for (u32) {
  fn lowSurrogate(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar12lowSurrogateEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN5QChar12lowSurrogateEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QChar {
  pub fn isSymbol<T: QChar_isSymbol>(&mut self, value: T) -> i32 {
    value.isSymbol(self);
    return 1;
  }
}

pub trait QChar_isSymbol {
  fn isSymbol(self, this: &mut QChar) -> i32;
}

// proto: bool QChar::isSymbol(uint ucs4);
impl<'a> /*trait*/ QChar_isSymbol for (u32) {
  fn isSymbol(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar8isSymbolEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN5QChar8isSymbolEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QChar {
  pub fn cell<T: QChar_cell>(&mut self, value: T) -> i32 {
    value.cell(self);
    return 1;
  }
}

pub trait QChar_cell {
  fn cell(self, this: &mut QChar) -> i32;
}

// proto: unsigned char QChar::cell();
impl<'a> /*trait*/ QChar_cell for () {
  fn cell(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar4cellEv()};
    unsafe {_ZNK5QChar4cellEv()};
    return 1;
  }
}

impl /*struct*/ QChar {
  pub fn surrogateToUcs4<T: QChar_surrogateToUcs4>(&mut self, value: T) -> i32 {
    value.surrogateToUcs4(self);
    return 1;
  }
}

pub trait QChar_surrogateToUcs4 {
  fn surrogateToUcs4(self, this: &mut QChar) -> i32;
}

// proto: unsigned int QChar::surrogateToUcs4(QChar high, QChar low);
impl<'a> /*trait*/ QChar_surrogateToUcs4 for (QChar, QChar) {
  fn surrogateToUcs4(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar15surrogateToUcs4ES_S_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN5QChar15surrogateToUcs4ES_S_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QChar {
  pub fn isTitleCase<T: QChar_isTitleCase>(&mut self, value: T) -> i32 {
    value.isTitleCase(self);
    return 1;
  }
}

pub trait QChar_isTitleCase {
  fn isTitleCase(self, this: &mut QChar) -> i32;
}

// proto: bool QChar::isTitleCase(uint ucs4);
impl<'a> /*trait*/ QChar_isTitleCase for (u32) {
  fn isTitleCase(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar11isTitleCaseEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN5QChar11isTitleCaseEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QChar {
  pub fn isNull<T: QChar_isNull>(&mut self, value: T) -> i32 {
    value.isNull(self);
    return 1;
  }
}

pub trait QChar_isNull {
  fn isNull(self, this: &mut QChar) -> i32;
}

// proto: bool QChar::isNull();
impl<'a> /*trait*/ QChar_isNull for () {
  fn isNull(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar6isNullEv()};
    unsafe {_ZNK5QChar6isNullEv()};
    return 1;
  }
}

impl /*struct*/ QChar {
  pub fn digitValue<T: QChar_digitValue>(&mut self, value: T) -> i32 {
    value.digitValue(self);
    return 1;
  }
}

pub trait QChar_digitValue {
  fn digitValue(self, this: &mut QChar) -> i32;
}

// proto: int QChar::digitValue();
impl<'a> /*trait*/ QChar_digitValue for () {
  fn digitValue(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar10digitValueEv()};
    unsafe {_ZNK5QChar10digitValueEv()};
    return 1;
  }
}

impl /*struct*/ QChar {
  pub fn toTitleCase<T: QChar_toTitleCase>(&mut self, value: T) -> i32 {
    value.toTitleCase(self);
    return 1;
  }
}

pub trait QChar_toTitleCase {
  fn toTitleCase(self, this: &mut QChar) -> i32;
}

// proto: QChar QChar::toTitleCase();
impl<'a> /*trait*/ QChar_toTitleCase for () {
  fn toTitleCase(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar11toTitleCaseEv()};
    unsafe {_ZNK5QChar11toTitleCaseEv()};
    return 1;
  }
}

impl /*struct*/ QChar {
  pub fn isLower<T: QChar_isLower>(&mut self, value: T) -> i32 {
    value.isLower(self);
    return 1;
  }
}

pub trait QChar_isLower {
  fn isLower(self, this: &mut QChar) -> i32;
}

// proto: bool QChar::isLower(uint ucs4);
impl<'a> /*trait*/ QChar_isLower for (u32) {
  fn isLower(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar7isLowerEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN5QChar7isLowerEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QChar {
  pub fn isLowSurrogate<T: QChar_isLowSurrogate>(&mut self, value: T) -> i32 {
    value.isLowSurrogate(self);
    return 1;
  }
}

pub trait QChar_isLowSurrogate {
  fn isLowSurrogate(self, this: &mut QChar) -> i32;
}

// proto: bool QChar::isLowSurrogate();
impl<'a> /*trait*/ QChar_isLowSurrogate for () {
  fn isLowSurrogate(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar14isLowSurrogateEv()};
    unsafe {_ZNK5QChar14isLowSurrogateEv()};
    return 1;
  }
}

impl /*struct*/ QChar {
  pub fn isPrint<T: QChar_isPrint>(&mut self, value: T) -> i32 {
    value.isPrint(self);
    return 1;
  }
}

pub trait QChar_isPrint {
  fn isPrint(self, this: &mut QChar) -> i32;
}

// proto: bool QChar::isPrint(uint ucs4);
impl<'a> /*trait*/ QChar_isPrint for (u32) {
  fn isPrint(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar7isPrintEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN5QChar7isPrintEj(arg0)};
    return 1;
  }
}

// proto: bool QChar::isSymbol();
impl<'a> /*trait*/ QChar_isSymbol for () {
  fn isSymbol(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar8isSymbolEv()};
    unsafe {_ZNK5QChar8isSymbolEv()};
    return 1;
  }
}

// proto: bool QChar::isLower();
impl<'a> /*trait*/ QChar_isLower for () {
  fn isLower(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar7isLowerEv()};
    unsafe {_ZNK5QChar7isLowerEv()};
    return 1;
  }
}

impl /*struct*/ QChar {
  pub fn mirroredChar<T: QChar_mirroredChar>(&mut self, value: T) -> i32 {
    value.mirroredChar(self);
    return 1;
  }
}

pub trait QChar_mirroredChar {
  fn mirroredChar(self, this: &mut QChar) -> i32;
}

// proto: QChar QChar::mirroredChar();
impl<'a> /*trait*/ QChar_mirroredChar for () {
  fn mirroredChar(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar12mirroredCharEv()};
    unsafe {_ZNK5QChar12mirroredCharEv()};
    return 1;
  }
}

impl /*struct*/ QChar {
  pub fn row<T: QChar_row>(&mut self, value: T) -> i32 {
    value.row(self);
    return 1;
  }
}

pub trait QChar_row {
  fn row(self, this: &mut QChar) -> i32;
}

// proto: unsigned char QChar::row();
impl<'a> /*trait*/ QChar_row for () {
  fn row(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar3rowEv()};
    unsafe {_ZNK5QChar3rowEv()};
    return 1;
  }
}

impl /*struct*/ QChar {
  pub fn isDigit<T: QChar_isDigit>(&mut self, value: T) -> i32 {
    value.isDigit(self);
    return 1;
  }
}

pub trait QChar_isDigit {
  fn isDigit(self, this: &mut QChar) -> i32;
}

// proto: bool QChar::isDigit();
impl<'a> /*trait*/ QChar_isDigit for () {
  fn isDigit(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar7isDigitEv()};
    unsafe {_ZNK5QChar7isDigitEv()};
    return 1;
  }
}

// proto: unsigned int QChar::toTitleCase(uint ucs4);
impl<'a> /*trait*/ QChar_toTitleCase for (u32) {
  fn toTitleCase(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar11toTitleCaseEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN5QChar11toTitleCaseEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QChar {
  pub fn isSurrogate<T: QChar_isSurrogate>(&mut self, value: T) -> i32 {
    value.isSurrogate(self);
    return 1;
  }
}

pub trait QChar_isSurrogate {
  fn isSurrogate(self, this: &mut QChar) -> i32;
}

// proto: bool QChar::isSurrogate();
impl<'a> /*trait*/ QChar_isSurrogate for () {
  fn isSurrogate(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar11isSurrogateEv()};
    unsafe {_ZNK5QChar11isSurrogateEv()};
    return 1;
  }
}

// proto: bool QChar::hasMirrored(uint ucs4);
impl<'a> /*trait*/ QChar_hasMirrored for (u32) {
  fn hasMirrored(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar11hasMirroredEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN5QChar11hasMirroredEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QChar {
  pub fn isNumber<T: QChar_isNumber>(&mut self, value: T) -> i32 {
    value.isNumber(self);
    return 1;
  }
}

pub trait QChar_isNumber {
  fn isNumber(self, this: &mut QChar) -> i32;
}

// proto: bool QChar::isNumber();
impl<'a> /*trait*/ QChar_isNumber for () {
  fn isNumber(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar8isNumberEv()};
    unsafe {_ZNK5QChar8isNumberEv()};
    return 1;
  }
}

impl /*struct*/ QChar {
  pub fn isHighSurrogate<T: QChar_isHighSurrogate>(&mut self, value: T) -> i32 {
    value.isHighSurrogate(self);
    return 1;
  }
}

pub trait QChar_isHighSurrogate {
  fn isHighSurrogate(self, this: &mut QChar) -> i32;
}

// proto: bool QChar::isHighSurrogate(uint ucs4);
impl<'a> /*trait*/ QChar_isHighSurrogate for (u32) {
  fn isHighSurrogate(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar15isHighSurrogateEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN5QChar15isHighSurrogateEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QChar {
  pub fn toCaseFolded<T: QChar_toCaseFolded>(&mut self, value: T) -> i32 {
    value.toCaseFolded(self);
    return 1;
  }
}

pub trait QChar_toCaseFolded {
  fn toCaseFolded(self, this: &mut QChar) -> i32;
}

// proto: unsigned int QChar::toCaseFolded(uint ucs4);
impl<'a> /*trait*/ QChar_toCaseFolded for (u32) {
  fn toCaseFolded(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar12toCaseFoldedEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN5QChar12toCaseFoldedEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QChar {
  pub fn isMark<T: QChar_isMark>(&mut self, value: T) -> i32 {
    value.isMark(self);
    return 1;
  }
}

pub trait QChar_isMark {
  fn isMark(self, this: &mut QChar) -> i32;
}

// proto: bool QChar::isMark();
impl<'a> /*trait*/ QChar_isMark for () {
  fn isMark(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar6isMarkEv()};
    unsafe {_ZNK5QChar6isMarkEv()};
    return 1;
  }
}

// proto: unsigned int QChar::surrogateToUcs4(ushort high, ushort low);
impl<'a> /*trait*/ QChar_surrogateToUcs4 for (u16, u16) {
  fn surrogateToUcs4(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar15surrogateToUcs4Ett()};
    let arg0 = self.0  as c_ushort;
    let arg1 = self.1  as c_ushort;
    unsafe {_ZN5QChar15surrogateToUcs4Ett(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QChar {
  pub fn toLower<T: QChar_toLower>(&mut self, value: T) -> i32 {
    value.toLower(self);
    return 1;
  }
}

pub trait QChar_toLower {
  fn toLower(self, this: &mut QChar) -> i32;
}

// proto: unsigned int QChar::toLower(uint ucs4);
impl<'a> /*trait*/ QChar_toLower for (u32) {
  fn toLower(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar7toLowerEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN5QChar7toLowerEj(arg0)};
    return 1;
  }
}

// proto: unsigned int QChar::mirroredChar(uint ucs4);
impl<'a> /*trait*/ QChar_mirroredChar for (u32) {
  fn mirroredChar(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar12mirroredCharEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN5QChar12mirroredCharEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QChar {
  pub fn setRow<T: QChar_setRow>(&mut self, value: T) -> i32 {
    value.setRow(self);
    return 1;
  }
}

pub trait QChar_setRow {
  fn setRow(self, this: &mut QChar) -> i32;
}

// proto: void QChar::setRow(uchar row);
impl<'a> /*trait*/ QChar_setRow for (u8) {
  fn setRow(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar6setRowEh()};
    let arg0 = self  as c_uchar;
    unsafe {_ZN5QChar6setRowEh(arg0)};
    return 1;
  }
}

impl /*struct*/ QChar {
  pub fn decomposition<T: QChar_decomposition>(&mut self, value: T) -> i32 {
    value.decomposition(self);
    return 1;
  }
}

pub trait QChar_decomposition {
  fn decomposition(self, this: &mut QChar) -> i32;
}

// proto: QString QChar::decomposition(uint ucs4);
impl<'a> /*trait*/ QChar_decomposition for (u32) {
  fn decomposition(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar13decompositionEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN5QChar13decompositionEj(arg0)};
    return 1;
  }
}

// proto: int QChar::digitValue(uint ucs4);
impl<'a> /*trait*/ QChar_digitValue for (u32) {
  fn digitValue(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar10digitValueEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN5QChar10digitValueEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QChar {
  pub fn setCell<T: QChar_setCell>(&mut self, value: T) -> i32 {
    value.setCell(self);
    return 1;
  }
}

pub trait QChar_setCell {
  fn setCell(self, this: &mut QChar) -> i32;
}

// proto: void QChar::setCell(uchar cell);
impl<'a> /*trait*/ QChar_setCell for (u8) {
  fn setCell(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar7setCellEh()};
    let arg0 = self  as c_uchar;
    unsafe {_ZN5QChar7setCellEh(arg0)};
    return 1;
  }
}

impl /*struct*/ QChar {
  pub fn isUpper<T: QChar_isUpper>(&mut self, value: T) -> i32 {
    value.isUpper(self);
    return 1;
  }
}

pub trait QChar_isUpper {
  fn isUpper(self, this: &mut QChar) -> i32;
}

// proto: bool QChar::isUpper(uint ucs4);
impl<'a> /*trait*/ QChar_isUpper for (u32) {
  fn isUpper(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar7isUpperEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN5QChar7isUpperEj(arg0)};
    return 1;
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

// proto: QChar QChar::toCaseFolded();
impl<'a> /*trait*/ QChar_toCaseFolded for () {
  fn toCaseFolded(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar12toCaseFoldedEv()};
    unsafe {_ZNK5QChar12toCaseFoldedEv()};
    return 1;
  }
}

// proto: bool QChar::isPrint();
impl<'a> /*trait*/ QChar_isPrint for () {
  fn isPrint(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar7isPrintEv()};
    unsafe {_ZNK5QChar7isPrintEv()};
    return 1;
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
  pub fn isPunct<T: QChar_isPunct>(&mut self, value: T) -> i32 {
    value.isPunct(self);
    return 1;
  }
}

pub trait QChar_isPunct {
  fn isPunct(self, this: &mut QChar) -> i32;
}

// proto: bool QChar::isPunct();
impl<'a> /*trait*/ QChar_isPunct for () {
  fn isPunct(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar7isPunctEv()};
    unsafe {_ZNK5QChar7isPunctEv()};
    return 1;
  }
}

// proto: QString QChar::decomposition();
impl<'a> /*trait*/ QChar_decomposition for () {
  fn decomposition(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar13decompositionEv()};
    unsafe {_ZNK5QChar13decompositionEv()};
    return 1;
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
  pub fn isSpace<T: QChar_isSpace>(&mut self, value: T) -> i32 {
    value.isSpace(self);
    return 1;
  }
}

pub trait QChar_isSpace {
  fn isSpace(self, this: &mut QChar) -> i32;
}

// proto: bool QChar::isSpace();
impl<'a> /*trait*/ QChar_isSpace for () {
  fn isSpace(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar7isSpaceEv()};
    unsafe {_ZNK5QChar7isSpaceEv()};
    return 1;
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

// proto: bool QChar::isUpper();
impl<'a> /*trait*/ QChar_isUpper for () {
  fn isUpper(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar7isUpperEv()};
    unsafe {_ZNK5QChar7isUpperEv()};
    return 1;
  }
}

impl /*struct*/ QChar {
  pub fn combiningClass<T: QChar_combiningClass>(&mut self, value: T) -> i32 {
    value.combiningClass(self);
    return 1;
  }
}

pub trait QChar_combiningClass {
  fn combiningClass(self, this: &mut QChar) -> i32;
}

// proto: unsigned char QChar::combiningClass(uint ucs4);
impl<'a> /*trait*/ QChar_combiningClass for (u32) {
  fn combiningClass(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar14combiningClassEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN5QChar14combiningClassEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QChar {
  pub fn isNonCharacter<T: QChar_isNonCharacter>(&mut self, value: T) -> i32 {
    value.isNonCharacter(self);
    return 1;
  }
}

pub trait QChar_isNonCharacter {
  fn isNonCharacter(self, this: &mut QChar) -> i32;
}

// proto: bool QChar::isNonCharacter();
impl<'a> /*trait*/ QChar_isNonCharacter for () {
  fn isNonCharacter(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar14isNonCharacterEv()};
    unsafe {_ZNK5QChar14isNonCharacterEv()};
    return 1;
  }
}

impl /*struct*/ QChar {
  pub fn isLetterOrNumber<T: QChar_isLetterOrNumber>(&mut self, value: T) -> i32 {
    value.isLetterOrNumber(self);
    return 1;
  }
}

pub trait QChar_isLetterOrNumber {
  fn isLetterOrNumber(self, this: &mut QChar) -> i32;
}

// proto: bool QChar::isLetterOrNumber(uint ucs4);
impl<'a> /*trait*/ QChar_isLetterOrNumber for (u32) {
  fn isLetterOrNumber(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar16isLetterOrNumberEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN5QChar16isLetterOrNumberEj(arg0)};
    return 1;
  }
}

// proto: bool QChar::isDigit(uint ucs4);
impl<'a> /*trait*/ QChar_isDigit for (u32) {
  fn isDigit(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar7isDigitEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN5QChar7isDigitEj(arg0)};
    return 1;
  }
}

// proto: bool QChar::isPunct(uint ucs4);
impl<'a> /*trait*/ QChar_isPunct for (u32) {
  fn isPunct(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar7isPunctEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN5QChar7isPunctEj(arg0)};
    return 1;
  }
}

// proto: bool QChar::isTitleCase();
impl<'a> /*trait*/ QChar_isTitleCase for () {
  fn isTitleCase(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar11isTitleCaseEv()};
    unsafe {_ZNK5QChar11isTitleCaseEv()};
    return 1;
  }
}

impl /*struct*/ QChar {
  pub fn isLetter<T: QChar_isLetter>(&mut self, value: T) -> i32 {
    value.isLetter(self);
    return 1;
  }
}

pub trait QChar_isLetter {
  fn isLetter(self, this: &mut QChar) -> i32;
}

// proto: bool QChar::isLetter();
impl<'a> /*trait*/ QChar_isLetter for () {
  fn isLetter(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar8isLetterEv()};
    unsafe {_ZNK5QChar8isLetterEv()};
    return 1;
  }
}

// proto: unsigned char QChar::combiningClass();
impl<'a> /*trait*/ QChar_combiningClass for () {
  fn combiningClass(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar14combiningClassEv()};
    unsafe {_ZNK5QChar14combiningClassEv()};
    return 1;
  }
}

// proto: bool QChar::isHighSurrogate();
impl<'a> /*trait*/ QChar_isHighSurrogate for () {
  fn isHighSurrogate(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar15isHighSurrogateEv()};
    unsafe {_ZNK5QChar15isHighSurrogateEv()};
    return 1;
  }
}

impl /*struct*/ QChar {
  pub fn highSurrogate<T: QChar_highSurrogate>(&mut self, value: T) -> i32 {
    value.highSurrogate(self);
    return 1;
  }
}

pub trait QChar_highSurrogate {
  fn highSurrogate(self, this: &mut QChar) -> i32;
}

// proto: unsigned short QChar::highSurrogate(uint ucs4);
impl<'a> /*trait*/ QChar_highSurrogate for (u32) {
  fn highSurrogate(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar13highSurrogateEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN5QChar13highSurrogateEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QChar {
  pub fn requiresSurrogates<T: QChar_requiresSurrogates>(&mut self, value: T) -> i32 {
    value.requiresSurrogates(self);
    return 1;
  }
}

pub trait QChar_requiresSurrogates {
  fn requiresSurrogates(self, this: &mut QChar) -> i32;
}

// proto: bool QChar::requiresSurrogates(uint ucs4);
impl<'a> /*trait*/ QChar_requiresSurrogates for (u32) {
  fn requiresSurrogates(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar18requiresSurrogatesEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN5QChar18requiresSurrogatesEj(arg0)};
    return 1;
  }
}

// proto: bool QChar::isLetterOrNumber();
impl<'a> /*trait*/ QChar_isLetterOrNumber for () {
  fn isLetterOrNumber(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar16isLetterOrNumberEv()};
    unsafe {_ZNK5QChar16isLetterOrNumberEv()};
    return 1;
  }
}

impl /*struct*/ QChar {
  pub fn unicode<T: QChar_unicode>(&mut self, value: T) -> i32 {
    value.unicode(self);
    return 1;
  }
}

pub trait QChar_unicode {
  fn unicode(self, this: &mut QChar) -> i32;
}

// proto: ushort & QChar::unicode();
impl<'a> /*trait*/ QChar_unicode for () {
  fn unicode(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar7unicodeEv()};
    unsafe {_ZN5QChar7unicodeEv()};
    return 1;
  }
}

// proto: bool QChar::isLowSurrogate(uint ucs4);
impl<'a> /*trait*/ QChar_isLowSurrogate for (u32) {
  fn isLowSurrogate(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar14isLowSurrogateEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN5QChar14isLowSurrogateEj(arg0)};
    return 1;
  }
}

// proto: bool QChar::isNumber(uint ucs4);
impl<'a> /*trait*/ QChar_isNumber for (u32) {
  fn isNumber(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar8isNumberEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN5QChar8isNumberEj(arg0)};
    return 1;
  }
}

// proto: QChar QChar::toLower();
impl<'a> /*trait*/ QChar_toLower for () {
  fn toLower(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar7toLowerEv()};
    unsafe {_ZNK5QChar7toLowerEv()};
    return 1;
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

// proto: bool QChar::isLetter(uint ucs4);
impl<'a> /*trait*/ QChar_isLetter for (u32) {
  fn isLetter(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar8isLetterEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN5QChar8isLetterEj(arg0)};
    return 1;
  }
}

// proto: QChar QChar::toUpper();
impl<'a> /*trait*/ QChar_toUpper for () {
  fn toUpper(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar7toUpperEv()};
    unsafe {_ZNK5QChar7toUpperEv()};
    return 1;
  }
}

// proto: bool QChar::isSpace(uint ucs4);
impl<'a> /*trait*/ QChar_isSpace for (u32) {
  fn isSpace(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar7isSpaceEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN5QChar7isSpaceEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QChar {
  pub fn fromLatin1<T: QChar_fromLatin1>(&mut self, value: T) -> i32 {
    value.fromLatin1(self);
    return 1;
  }
}

pub trait QChar_fromLatin1 {
  fn fromLatin1(self, this: &mut QChar) -> i32;
}

// proto: QChar QChar::fromLatin1(char c);
impl<'a> /*trait*/ QChar_fromLatin1 for (i8) {
  fn fromLatin1(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar10fromLatin1Ec()};
    let arg0 = self  as c_char;
    unsafe {_ZN5QChar10fromLatin1Ec(arg0)};
    return 1;
  }
}

// proto: bool QChar::isSurrogate(uint ucs4);
impl<'a> /*trait*/ QChar_isSurrogate for (u32) {
  fn isSurrogate(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar11isSurrogateEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN5QChar11isSurrogateEj(arg0)};
    return 1;
  }
}

// proto: bool QChar::isMark(uint ucs4);
impl<'a> /*trait*/ QChar_isMark for (u32) {
  fn isMark(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar6isMarkEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN5QChar6isMarkEj(arg0)};
    return 1;
  }
}

// proto: bool QChar::isNonCharacter(uint ucs4);
impl<'a> /*trait*/ QChar_isNonCharacter for (u32) {
  fn isNonCharacter(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QChar14isNonCharacterEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN5QChar14isNonCharacterEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QChar {
  pub fn toLatin1<T: QChar_toLatin1>(&mut self, value: T) -> i32 {
    value.toLatin1(self);
    return 1;
  }
}

pub trait QChar_toLatin1 {
  fn toLatin1(self, this: &mut QChar) -> i32;
}

// proto: char QChar::toLatin1();
impl<'a> /*trait*/ QChar_toLatin1 for () {
  fn toLatin1(self, this: &mut QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QChar8toLatin1Ev()};
    unsafe {_ZNK5QChar8toLatin1Ev()};
    return 1;
  }
}

