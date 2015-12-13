// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qbytearray::QByteArray;
use super::qstring::QString;
use super::qchar::QChar;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZNK10QTextCodec4nameEv() -> i32;
  fn _ZNK10QTextCodec9toUnicodeERK10QByteArray(arg0: *const c_void) -> i32;
  fn _ZNK10QTextCodec11fromUnicodeERK7QString(arg0: *const c_void) -> i32;
  fn _ZN10QTextCodec14codecForLocaleEv() -> i32;
  fn _ZNK10QTextCodec11makeDecoderE6QFlagsINS_14ConversionFlagEE(arg0: c_int) -> i32;
  fn _ZN10QTextCodec13availableMibsEv() -> i32;
  fn _ZN10QTextCodec12codecForHtmlERK10QByteArray(arg0: *const c_void) -> i32;
  fn _ZN10QTextCodecC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN10QTextCodec17setCodecForLocaleEPS_(arg0: *mut c_void) -> i32;
  fn _ZN10QTextCodec15codecForUtfTextERK10QByteArray(arg0: *const c_void) -> i32;
  fn _ZNK10QTextCodec9toUnicodeEPKc(arg0: *const c_char) -> i32;
  fn _ZNK10QTextCodec7mibEnumEv() -> i32;
  fn _ZN10QTextCodec12codecForNameEPKc(arg0: *const c_char) -> i32;
  fn _ZNK10QTextCodec9canEncodeERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK10QTextCodec7aliasesEv() -> i32;
  fn _ZN10QTextCodec12codecForNameERK10QByteArray(arg0: *const c_void) -> i32;
  fn _ZN10QTextCodec15availableCodecsEv() -> i32;
  fn _ZN10QTextCodec12codecForHtmlERK10QByteArrayPS_(arg0: *const c_void, arg1: *mut c_void) -> i32;
  fn _ZN10QTextCodecD0Ev() -> i32;
  fn _ZNK10QTextCodec11makeEncoderE6QFlagsINS_14ConversionFlagEE(arg0: c_int) -> i32;
  fn _ZN10QTextCodec11codecForMibEi(arg0: c_int) -> i32;
  fn _ZN10QTextCodecC1Ev(qthis: *mut c_void) -> i32;
  fn _ZN10QTextCodec15codecForUtfTextERK10QByteArrayPS_(arg0: *const c_void, arg1: *mut c_void) -> i32;
  fn _ZNK10QTextCodec9canEncodeE5QChar(arg0: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QTextCodec)=8
pub struct QTextCodec {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextCodec {
  pub fn name<T: QTextCodec_name>(&mut self, value: T) -> i32 {
    value.name(self);
    return 1;
  }
}

pub trait QTextCodec_name {
  fn name(self, this: &mut QTextCodec) -> i32;
}

// proto: QByteArray QTextCodec::name();
impl<'a> /*trait*/ QTextCodec_name for () {
  fn name(self, this: &mut QTextCodec) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextCodec4nameEv()};
    unsafe {_ZNK10QTextCodec4nameEv()};
    return 1;
  }
}

impl /*struct*/ QTextCodec {
  pub fn toUnicode<T: QTextCodec_toUnicode>(&mut self, value: T) -> i32 {
    value.toUnicode(self);
    return 1;
  }
}

pub trait QTextCodec_toUnicode {
  fn toUnicode(self, this: &mut QTextCodec) -> i32;
}

// proto: QString QTextCodec::toUnicode(const QByteArray & );
impl<'a> /*trait*/ QTextCodec_toUnicode for (&'a  QByteArray) {
  fn toUnicode(self, this: &mut QTextCodec) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextCodec9toUnicodeERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK10QTextCodec9toUnicodeERK10QByteArray(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextCodec {
  pub fn fromUnicode<T: QTextCodec_fromUnicode>(&mut self, value: T) -> i32 {
    value.fromUnicode(self);
    return 1;
  }
}

pub trait QTextCodec_fromUnicode {
  fn fromUnicode(self, this: &mut QTextCodec) -> i32;
}

// proto: QByteArray QTextCodec::fromUnicode(const QString & uc);
impl<'a> /*trait*/ QTextCodec_fromUnicode for (&'a  QString) {
  fn fromUnicode(self, this: &mut QTextCodec) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextCodec11fromUnicodeERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK10QTextCodec11fromUnicodeERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextCodec {
  pub fn codecForLocale<T: QTextCodec_codecForLocale>(&mut self, value: T) -> i32 {
    value.codecForLocale(self);
    return 1;
  }
}

pub trait QTextCodec_codecForLocale {
  fn codecForLocale(self, this: &mut QTextCodec) -> i32;
}

// proto: QTextCodec * QTextCodec::codecForLocale();
impl<'a> /*trait*/ QTextCodec_codecForLocale for () {
  fn codecForLocale(self, this: &mut QTextCodec) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec14codecForLocaleEv()};
    unsafe {_ZN10QTextCodec14codecForLocaleEv()};
    return 1;
  }
}

impl /*struct*/ QTextCodec {
  pub fn makeDecoder<T: QTextCodec_makeDecoder>(&mut self, value: T) -> i32 {
    value.makeDecoder(self);
    return 1;
  }
}

pub trait QTextCodec_makeDecoder {
  fn makeDecoder(self, this: &mut QTextCodec) -> i32;
}

// proto: QTextDecoder * QTextCodec::makeDecoder(ConversionFlags flags);
impl<'a> /*trait*/ QTextCodec_makeDecoder for (i32) {
  fn makeDecoder(self, this: &mut QTextCodec) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextCodec11makeDecoderE6QFlagsINS_14ConversionFlagEE()};
    let arg0 = self  as c_int;
    unsafe {_ZNK10QTextCodec11makeDecoderE6QFlagsINS_14ConversionFlagEE(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextCodec {
  pub fn availableMibs<T: QTextCodec_availableMibs>(&mut self, value: T) -> i32 {
    value.availableMibs(self);
    return 1;
  }
}

pub trait QTextCodec_availableMibs {
  fn availableMibs(self, this: &mut QTextCodec) -> i32;
}

// proto: QList<int> QTextCodec::availableMibs();
impl<'a> /*trait*/ QTextCodec_availableMibs for () {
  fn availableMibs(self, this: &mut QTextCodec) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec13availableMibsEv()};
    unsafe {_ZN10QTextCodec13availableMibsEv()};
    return 1;
  }
}

impl /*struct*/ QTextCodec {
  pub fn codecForHtml<T: QTextCodec_codecForHtml>(&mut self, value: T) -> i32 {
    value.codecForHtml(self);
    return 1;
  }
}

pub trait QTextCodec_codecForHtml {
  fn codecForHtml(self, this: &mut QTextCodec) -> i32;
}

// proto: QTextCodec * QTextCodec::codecForHtml(const QByteArray & ba);
impl<'a> /*trait*/ QTextCodec_codecForHtml for (&'a  QByteArray) {
  fn codecForHtml(self, this: &mut QTextCodec) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec12codecForHtmlERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QTextCodec12codecForHtmlERK10QByteArray(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextCodec {
  pub fn NewQTextCodec<T: QTextCodec_NewQTextCodec>(value: T) -> QTextCodec {
    let rsthis = value.NewQTextCodec();
    return rsthis;
    // return 1;
  }
}

pub trait QTextCodec_NewQTextCodec {
  fn NewQTextCodec(self) -> QTextCodec;
}

// proto: void QTextCodec::NewQTextCodec(const QTextCodec & );
impl<'a> /*trait*/ QTextCodec_NewQTextCodec for (&'a  QTextCodec) {
  fn NewQTextCodec(self) -> QTextCodec {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodecC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QTextCodecC1ERKS_(qthis, arg0)};
    let rsthis = QTextCodec{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextCodec {
  pub fn setCodecForLocale<T: QTextCodec_setCodecForLocale>(&mut self, value: T) -> i32 {
    value.setCodecForLocale(self);
    return 1;
  }
}

pub trait QTextCodec_setCodecForLocale {
  fn setCodecForLocale(self, this: &mut QTextCodec) -> i32;
}

// proto: void QTextCodec::setCodecForLocale(QTextCodec * c);
impl<'a> /*trait*/ QTextCodec_setCodecForLocale for (&'a mut QTextCodec) {
  fn setCodecForLocale(self, this: &mut QTextCodec) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec17setCodecForLocaleEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QTextCodec17setCodecForLocaleEPS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextCodec {
  pub fn codecForUtfText<T: QTextCodec_codecForUtfText>(&mut self, value: T) -> i32 {
    value.codecForUtfText(self);
    return 1;
  }
}

pub trait QTextCodec_codecForUtfText {
  fn codecForUtfText(self, this: &mut QTextCodec) -> i32;
}

// proto: QTextCodec * QTextCodec::codecForUtfText(const QByteArray & ba);
impl<'a> /*trait*/ QTextCodec_codecForUtfText for (&'a  QByteArray) {
  fn codecForUtfText(self, this: &mut QTextCodec) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec15codecForUtfTextERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QTextCodec15codecForUtfTextERK10QByteArray(arg0)};
    return 1;
  }
}

// proto: QString QTextCodec::toUnicode(const char * chars);
impl<'a> /*trait*/ QTextCodec_toUnicode for (&'a  String) {
  fn toUnicode(self, this: &mut QTextCodec) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextCodec9toUnicodeEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    unsafe {_ZNK10QTextCodec9toUnicodeEPKc(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextCodec {
  pub fn mibEnum<T: QTextCodec_mibEnum>(&mut self, value: T) -> i32 {
    value.mibEnum(self);
    return 1;
  }
}

pub trait QTextCodec_mibEnum {
  fn mibEnum(self, this: &mut QTextCodec) -> i32;
}

// proto: int QTextCodec::mibEnum();
impl<'a> /*trait*/ QTextCodec_mibEnum for () {
  fn mibEnum(self, this: &mut QTextCodec) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextCodec7mibEnumEv()};
    unsafe {_ZNK10QTextCodec7mibEnumEv()};
    return 1;
  }
}

impl /*struct*/ QTextCodec {
  pub fn codecForName<T: QTextCodec_codecForName>(&mut self, value: T) -> i32 {
    value.codecForName(self);
    return 1;
  }
}

pub trait QTextCodec_codecForName {
  fn codecForName(self, this: &mut QTextCodec) -> i32;
}

// proto: QTextCodec * QTextCodec::codecForName(const char * name);
impl<'a> /*trait*/ QTextCodec_codecForName for (&'a  String) {
  fn codecForName(self, this: &mut QTextCodec) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec12codecForNameEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    unsafe {_ZN10QTextCodec12codecForNameEPKc(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextCodec {
  pub fn canEncode<T: QTextCodec_canEncode>(&mut self, value: T) -> i32 {
    value.canEncode(self);
    return 1;
  }
}

pub trait QTextCodec_canEncode {
  fn canEncode(self, this: &mut QTextCodec) -> i32;
}

// proto: bool QTextCodec::canEncode(const QString & );
impl<'a> /*trait*/ QTextCodec_canEncode for (&'a  QString) {
  fn canEncode(self, this: &mut QTextCodec) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextCodec9canEncodeERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK10QTextCodec9canEncodeERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextCodec {
  pub fn aliases<T: QTextCodec_aliases>(&mut self, value: T) -> i32 {
    value.aliases(self);
    return 1;
  }
}

pub trait QTextCodec_aliases {
  fn aliases(self, this: &mut QTextCodec) -> i32;
}

// proto: QList<QByteArray> QTextCodec::aliases();
impl<'a> /*trait*/ QTextCodec_aliases for () {
  fn aliases(self, this: &mut QTextCodec) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextCodec7aliasesEv()};
    unsafe {_ZNK10QTextCodec7aliasesEv()};
    return 1;
  }
}

// proto: QTextCodec * QTextCodec::codecForName(const QByteArray & name);
impl<'a> /*trait*/ QTextCodec_codecForName for (&'a  QByteArray) {
  fn codecForName(self, this: &mut QTextCodec) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec12codecForNameERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QTextCodec12codecForNameERK10QByteArray(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextCodec {
  pub fn availableCodecs<T: QTextCodec_availableCodecs>(&mut self, value: T) -> i32 {
    value.availableCodecs(self);
    return 1;
  }
}

pub trait QTextCodec_availableCodecs {
  fn availableCodecs(self, this: &mut QTextCodec) -> i32;
}

// proto: QList<QByteArray> QTextCodec::availableCodecs();
impl<'a> /*trait*/ QTextCodec_availableCodecs for () {
  fn availableCodecs(self, this: &mut QTextCodec) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec15availableCodecsEv()};
    unsafe {_ZN10QTextCodec15availableCodecsEv()};
    return 1;
  }
}

// proto: QTextCodec * QTextCodec::codecForHtml(const QByteArray & ba, QTextCodec * defaultCodec);
impl<'a> /*trait*/ QTextCodec_codecForHtml for (&'a  QByteArray, &'a mut QTextCodec) {
  fn codecForHtml(self, this: &mut QTextCodec) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec12codecForHtmlERK10QByteArrayPS_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN10QTextCodec12codecForHtmlERK10QByteArrayPS_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTextCodec {
  pub fn FreeQTextCodec<T: QTextCodec_FreeQTextCodec>(&mut self, value: T) -> i32 {
    value.FreeQTextCodec(self);
    return 1;
  }
}

pub trait QTextCodec_FreeQTextCodec {
  fn FreeQTextCodec(self, this: &mut QTextCodec) -> i32;
}

// proto: void QTextCodec::FreeQTextCodec();
impl<'a> /*trait*/ QTextCodec_FreeQTextCodec for () {
  fn FreeQTextCodec(self, this: &mut QTextCodec) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodecD0Ev()};
    unsafe {_ZN10QTextCodecD0Ev()};
    return 1;
  }
}

impl /*struct*/ QTextCodec {
  pub fn makeEncoder<T: QTextCodec_makeEncoder>(&mut self, value: T) -> i32 {
    value.makeEncoder(self);
    return 1;
  }
}

pub trait QTextCodec_makeEncoder {
  fn makeEncoder(self, this: &mut QTextCodec) -> i32;
}

// proto: QTextEncoder * QTextCodec::makeEncoder(ConversionFlags flags);
impl<'a> /*trait*/ QTextCodec_makeEncoder for (i32) {
  fn makeEncoder(self, this: &mut QTextCodec) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextCodec11makeEncoderE6QFlagsINS_14ConversionFlagEE()};
    let arg0 = self  as c_int;
    unsafe {_ZNK10QTextCodec11makeEncoderE6QFlagsINS_14ConversionFlagEE(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextCodec {
  pub fn codecForMib<T: QTextCodec_codecForMib>(&mut self, value: T) -> i32 {
    value.codecForMib(self);
    return 1;
  }
}

pub trait QTextCodec_codecForMib {
  fn codecForMib(self, this: &mut QTextCodec) -> i32;
}

// proto: QTextCodec * QTextCodec::codecForMib(int mib);
impl<'a> /*trait*/ QTextCodec_codecForMib for (i32) {
  fn codecForMib(self, this: &mut QTextCodec) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec11codecForMibEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QTextCodec11codecForMibEi(arg0)};
    return 1;
  }
}

// proto: void QTextCodec::NewQTextCodec();
impl<'a> /*trait*/ QTextCodec_NewQTextCodec for () {
  fn NewQTextCodec(self) -> QTextCodec {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodecC1Ev()};
    unsafe {_ZN10QTextCodecC1Ev(qthis)};
    let rsthis = QTextCodec{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: QTextCodec * QTextCodec::codecForUtfText(const QByteArray & ba, QTextCodec * defaultCodec);
impl<'a> /*trait*/ QTextCodec_codecForUtfText for (&'a  QByteArray, &'a mut QTextCodec) {
  fn codecForUtfText(self, this: &mut QTextCodec) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec15codecForUtfTextERK10QByteArrayPS_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN10QTextCodec15codecForUtfTextERK10QByteArrayPS_(arg0, arg1)};
    return 1;
  }
}

// proto: bool QTextCodec::canEncode(QChar );
impl<'a> /*trait*/ QTextCodec_canEncode for (QChar) {
  fn canEncode(self, this: &mut QTextCodec) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextCodec9canEncodeE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK10QTextCodec9canEncodeE5QChar(arg0)};
    return 1;
  }
}

