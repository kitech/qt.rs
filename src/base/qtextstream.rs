// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qiodevice::QIODevice;
use super::qlocale::QLocale;
use super::qstring::QString;
use super::qtextcodec::QTextCodec;
use super::qchar::QChar;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZNK11QTextStream5codecEv() -> i32;
  fn _ZN11QTextStreamC1EP9QIODevice(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZN11QTextStream9setLocaleERK7QLocale(arg0: *const c_void) -> i32;
  fn _ZN11QTextStreamC1Ev(qthis: *mut c_void) -> i32;
  fn _ZNK11QTextStream5atEndEv() -> i32;
  fn _ZN11QTextStream12readLineIntoEP7QStringx(arg0: *mut c_void, arg1: c_longlong) -> i32;
  fn _ZN11QTextStream22setRealNumberPrecisionEi(arg0: c_int) -> i32;
  fn _ZN11QTextStream9setDeviceEP9QIODevice(arg0: *mut c_void) -> i32;
  fn _ZN11QTextStream5resetEv() -> i32;
  fn _ZN11QTextStream4seekEx(arg0: c_longlong) -> i32;
  fn _ZNK11QTextStream6stringEv() -> i32;
  fn _ZN11QTextStream20setAutoDetectUnicodeEb(arg0: int8_t) -> i32;
  fn _ZNK11QTextStream7padCharEv() -> i32;
  fn _ZNK11QTextStream6deviceEv() -> i32;
  fn _ZN11QTextStream11resetStatusEv() -> i32;
  fn _ZNK11QTextStream17autoDetectUnicodeEv() -> i32;
  fn _ZNK11QTextStream10fieldWidthEv() -> i32;
  fn _ZNK11QTextStream21generateByteOrderMarkEv() -> i32;
  fn _ZN11QTextStream24setGenerateByteOrderMarkEb(arg0: int8_t) -> i32;
  fn _ZN11QTextStream8setCodecEP10QTextCodec(arg0: *mut c_void) -> i32;
  fn _ZN11QTextStream5flushEv() -> i32;
  fn _ZN11QTextStream14setIntegerBaseEi(arg0: c_int) -> i32;
  fn _ZN11QTextStreamD0Ev() -> i32;
  fn _ZNK11QTextStream6localeEv() -> i32;
  fn _ZN11QTextStream4readEx(arg0: c_longlong) -> i32;
  fn _ZN11QTextStream10setPadCharE5QChar(arg0: *mut c_void) -> i32;
  fn _ZNK11QTextStream19realNumberPrecisionEv() -> i32;
  fn _ZNK11QTextStream3posEv() -> i32;
  fn _ZN11QTextStream7readAllEv() -> i32;
  fn _ZN11QTextStream14skipWhiteSpaceEv() -> i32;
  fn _ZN11QTextStream13setFieldWidthEi(arg0: c_int) -> i32;
  fn _ZN11QTextStream8setCodecEPKc(arg0: *const c_char) -> i32;
  fn _ZNK11QTextStream11integerBaseEv() -> i32;
  fn _ZN11QTextStream14setNumberFlagsE6QFlagsINS_10NumberFlagEE(arg0: c_int) -> i32;
  fn _ZN11QTextStream8readLineEx(arg0: c_longlong) -> i32;
  fn _ZN11QTextStreamC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QTextStream)=1
pub struct QTextStream {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextStream {
  pub fn codec<T: QTextStream_codec>(&mut self, value: T) -> i32 {
    value.codec(self);
    return 1;
  }
}

pub trait QTextStream_codec {
  fn codec(self, this: &mut QTextStream) -> i32;
}

// proto: QTextCodec * QTextStream::codec();
impl<'a> /*trait*/ QTextStream_codec for () {
  fn codec(self, this: &mut QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextStream5codecEv()};
    unsafe {_ZNK11QTextStream5codecEv()};
    return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn NewQTextStream<T: QTextStream_NewQTextStream>(value: T) -> QTextStream {
    let rsthis = value.NewQTextStream();
    return rsthis;
    // return 1;
  }
}

pub trait QTextStream_NewQTextStream {
  fn NewQTextStream(self) -> QTextStream;
}

// proto: void QTextStream::NewQTextStream(QIODevice * device);
impl<'a> /*trait*/ QTextStream_NewQTextStream for (&'a mut QIODevice) {
  fn NewQTextStream(self) -> QTextStream {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStreamC1EP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QTextStreamC1EP9QIODevice(qthis, arg0)};
    let rsthis = QTextStream{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn setLocale<T: QTextStream_setLocale>(&mut self, value: T) -> i32 {
    value.setLocale(self);
    return 1;
  }
}

pub trait QTextStream_setLocale {
  fn setLocale(self, this: &mut QTextStream) -> i32;
}

// proto: void QTextStream::setLocale(const QLocale & locale);
impl<'a> /*trait*/ QTextStream_setLocale for (&'a  QLocale) {
  fn setLocale(self, this: &mut QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream9setLocaleERK7QLocale()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QTextStream9setLocaleERK7QLocale(arg0)};
    return 1;
  }
}

// proto: void QTextStream::NewQTextStream();
impl<'a> /*trait*/ QTextStream_NewQTextStream for () {
  fn NewQTextStream(self) -> QTextStream {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStreamC1Ev()};
    unsafe {_ZN11QTextStreamC1Ev(qthis)};
    let rsthis = QTextStream{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn atEnd<T: QTextStream_atEnd>(&mut self, value: T) -> i32 {
    value.atEnd(self);
    return 1;
  }
}

pub trait QTextStream_atEnd {
  fn atEnd(self, this: &mut QTextStream) -> i32;
}

// proto: bool QTextStream::atEnd();
impl<'a> /*trait*/ QTextStream_atEnd for () {
  fn atEnd(self, this: &mut QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextStream5atEndEv()};
    unsafe {_ZNK11QTextStream5atEndEv()};
    return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn readLineInto<T: QTextStream_readLineInto>(&mut self, value: T) -> i32 {
    value.readLineInto(self);
    return 1;
  }
}

pub trait QTextStream_readLineInto {
  fn readLineInto(self, this: &mut QTextStream) -> i32;
}

// proto: bool QTextStream::readLineInto(QString * line, qint64 maxlen);
impl<'a> /*trait*/ QTextStream_readLineInto for (&'a mut QString, i64) {
  fn readLineInto(self, this: &mut QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream12readLineIntoEP7QStringx()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_longlong;
    unsafe {_ZN11QTextStream12readLineIntoEP7QStringx(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn setRealNumberPrecision<T: QTextStream_setRealNumberPrecision>(&mut self, value: T) -> i32 {
    value.setRealNumberPrecision(self);
    return 1;
  }
}

pub trait QTextStream_setRealNumberPrecision {
  fn setRealNumberPrecision(self, this: &mut QTextStream) -> i32;
}

// proto: void QTextStream::setRealNumberPrecision(int precision);
impl<'a> /*trait*/ QTextStream_setRealNumberPrecision for (i32) {
  fn setRealNumberPrecision(self, this: &mut QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream22setRealNumberPrecisionEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QTextStream22setRealNumberPrecisionEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn setDevice<T: QTextStream_setDevice>(&mut self, value: T) -> i32 {
    value.setDevice(self);
    return 1;
  }
}

pub trait QTextStream_setDevice {
  fn setDevice(self, this: &mut QTextStream) -> i32;
}

// proto: void QTextStream::setDevice(QIODevice * device);
impl<'a> /*trait*/ QTextStream_setDevice for (&'a mut QIODevice) {
  fn setDevice(self, this: &mut QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream9setDeviceEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QTextStream9setDeviceEP9QIODevice(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn reset<T: QTextStream_reset>(&mut self, value: T) -> i32 {
    value.reset(self);
    return 1;
  }
}

pub trait QTextStream_reset {
  fn reset(self, this: &mut QTextStream) -> i32;
}

// proto: void QTextStream::reset();
impl<'a> /*trait*/ QTextStream_reset for () {
  fn reset(self, this: &mut QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream5resetEv()};
    unsafe {_ZN11QTextStream5resetEv()};
    return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn seek<T: QTextStream_seek>(&mut self, value: T) -> i32 {
    value.seek(self);
    return 1;
  }
}

pub trait QTextStream_seek {
  fn seek(self, this: &mut QTextStream) -> i32;
}

// proto: bool QTextStream::seek(qint64 pos);
impl<'a> /*trait*/ QTextStream_seek for (i64) {
  fn seek(self, this: &mut QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream4seekEx()};
    let arg0 = self  as c_longlong;
    unsafe {_ZN11QTextStream4seekEx(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn string<T: QTextStream_string>(&mut self, value: T) -> i32 {
    value.string(self);
    return 1;
  }
}

pub trait QTextStream_string {
  fn string(self, this: &mut QTextStream) -> i32;
}

// proto: QString * QTextStream::string();
impl<'a> /*trait*/ QTextStream_string for () {
  fn string(self, this: &mut QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextStream6stringEv()};
    unsafe {_ZNK11QTextStream6stringEv()};
    return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn setAutoDetectUnicode<T: QTextStream_setAutoDetectUnicode>(&mut self, value: T) -> i32 {
    value.setAutoDetectUnicode(self);
    return 1;
  }
}

pub trait QTextStream_setAutoDetectUnicode {
  fn setAutoDetectUnicode(self, this: &mut QTextStream) -> i32;
}

// proto: void QTextStream::setAutoDetectUnicode(bool enabled);
impl<'a> /*trait*/ QTextStream_setAutoDetectUnicode for (i8) {
  fn setAutoDetectUnicode(self, this: &mut QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream20setAutoDetectUnicodeEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN11QTextStream20setAutoDetectUnicodeEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn padChar<T: QTextStream_padChar>(&mut self, value: T) -> i32 {
    value.padChar(self);
    return 1;
  }
}

pub trait QTextStream_padChar {
  fn padChar(self, this: &mut QTextStream) -> i32;
}

// proto: QChar QTextStream::padChar();
impl<'a> /*trait*/ QTextStream_padChar for () {
  fn padChar(self, this: &mut QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextStream7padCharEv()};
    unsafe {_ZNK11QTextStream7padCharEv()};
    return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn device<T: QTextStream_device>(&mut self, value: T) -> i32 {
    value.device(self);
    return 1;
  }
}

pub trait QTextStream_device {
  fn device(self, this: &mut QTextStream) -> i32;
}

// proto: QIODevice * QTextStream::device();
impl<'a> /*trait*/ QTextStream_device for () {
  fn device(self, this: &mut QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextStream6deviceEv()};
    unsafe {_ZNK11QTextStream6deviceEv()};
    return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn resetStatus<T: QTextStream_resetStatus>(&mut self, value: T) -> i32 {
    value.resetStatus(self);
    return 1;
  }
}

pub trait QTextStream_resetStatus {
  fn resetStatus(self, this: &mut QTextStream) -> i32;
}

// proto: void QTextStream::resetStatus();
impl<'a> /*trait*/ QTextStream_resetStatus for () {
  fn resetStatus(self, this: &mut QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream11resetStatusEv()};
    unsafe {_ZN11QTextStream11resetStatusEv()};
    return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn autoDetectUnicode<T: QTextStream_autoDetectUnicode>(&mut self, value: T) -> i32 {
    value.autoDetectUnicode(self);
    return 1;
  }
}

pub trait QTextStream_autoDetectUnicode {
  fn autoDetectUnicode(self, this: &mut QTextStream) -> i32;
}

// proto: bool QTextStream::autoDetectUnicode();
impl<'a> /*trait*/ QTextStream_autoDetectUnicode for () {
  fn autoDetectUnicode(self, this: &mut QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextStream17autoDetectUnicodeEv()};
    unsafe {_ZNK11QTextStream17autoDetectUnicodeEv()};
    return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn fieldWidth<T: QTextStream_fieldWidth>(&mut self, value: T) -> i32 {
    value.fieldWidth(self);
    return 1;
  }
}

pub trait QTextStream_fieldWidth {
  fn fieldWidth(self, this: &mut QTextStream) -> i32;
}

// proto: int QTextStream::fieldWidth();
impl<'a> /*trait*/ QTextStream_fieldWidth for () {
  fn fieldWidth(self, this: &mut QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextStream10fieldWidthEv()};
    unsafe {_ZNK11QTextStream10fieldWidthEv()};
    return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn generateByteOrderMark<T: QTextStream_generateByteOrderMark>(&mut self, value: T) -> i32 {
    value.generateByteOrderMark(self);
    return 1;
  }
}

pub trait QTextStream_generateByteOrderMark {
  fn generateByteOrderMark(self, this: &mut QTextStream) -> i32;
}

// proto: bool QTextStream::generateByteOrderMark();
impl<'a> /*trait*/ QTextStream_generateByteOrderMark for () {
  fn generateByteOrderMark(self, this: &mut QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextStream21generateByteOrderMarkEv()};
    unsafe {_ZNK11QTextStream21generateByteOrderMarkEv()};
    return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn setGenerateByteOrderMark<T: QTextStream_setGenerateByteOrderMark>(&mut self, value: T) -> i32 {
    value.setGenerateByteOrderMark(self);
    return 1;
  }
}

pub trait QTextStream_setGenerateByteOrderMark {
  fn setGenerateByteOrderMark(self, this: &mut QTextStream) -> i32;
}

// proto: void QTextStream::setGenerateByteOrderMark(bool generate);
impl<'a> /*trait*/ QTextStream_setGenerateByteOrderMark for (i8) {
  fn setGenerateByteOrderMark(self, this: &mut QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream24setGenerateByteOrderMarkEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN11QTextStream24setGenerateByteOrderMarkEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn setCodec<T: QTextStream_setCodec>(&mut self, value: T) -> i32 {
    value.setCodec(self);
    return 1;
  }
}

pub trait QTextStream_setCodec {
  fn setCodec(self, this: &mut QTextStream) -> i32;
}

// proto: void QTextStream::setCodec(QTextCodec * codec);
impl<'a> /*trait*/ QTextStream_setCodec for (&'a mut QTextCodec) {
  fn setCodec(self, this: &mut QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream8setCodecEP10QTextCodec()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QTextStream8setCodecEP10QTextCodec(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn flush<T: QTextStream_flush>(&mut self, value: T) -> i32 {
    value.flush(self);
    return 1;
  }
}

pub trait QTextStream_flush {
  fn flush(self, this: &mut QTextStream) -> i32;
}

// proto: void QTextStream::flush();
impl<'a> /*trait*/ QTextStream_flush for () {
  fn flush(self, this: &mut QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream5flushEv()};
    unsafe {_ZN11QTextStream5flushEv()};
    return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn setIntegerBase<T: QTextStream_setIntegerBase>(&mut self, value: T) -> i32 {
    value.setIntegerBase(self);
    return 1;
  }
}

pub trait QTextStream_setIntegerBase {
  fn setIntegerBase(self, this: &mut QTextStream) -> i32;
}

// proto: void QTextStream::setIntegerBase(int base);
impl<'a> /*trait*/ QTextStream_setIntegerBase for (i32) {
  fn setIntegerBase(self, this: &mut QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream14setIntegerBaseEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QTextStream14setIntegerBaseEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn FreeQTextStream<T: QTextStream_FreeQTextStream>(&mut self, value: T) -> i32 {
    value.FreeQTextStream(self);
    return 1;
  }
}

pub trait QTextStream_FreeQTextStream {
  fn FreeQTextStream(self, this: &mut QTextStream) -> i32;
}

// proto: void QTextStream::FreeQTextStream();
impl<'a> /*trait*/ QTextStream_FreeQTextStream for () {
  fn FreeQTextStream(self, this: &mut QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStreamD0Ev()};
    unsafe {_ZN11QTextStreamD0Ev()};
    return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn locale<T: QTextStream_locale>(&mut self, value: T) -> i32 {
    value.locale(self);
    return 1;
  }
}

pub trait QTextStream_locale {
  fn locale(self, this: &mut QTextStream) -> i32;
}

// proto: QLocale QTextStream::locale();
impl<'a> /*trait*/ QTextStream_locale for () {
  fn locale(self, this: &mut QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextStream6localeEv()};
    unsafe {_ZNK11QTextStream6localeEv()};
    return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn read<T: QTextStream_read>(&mut self, value: T) -> i32 {
    value.read(self);
    return 1;
  }
}

pub trait QTextStream_read {
  fn read(self, this: &mut QTextStream) -> i32;
}

// proto: QString QTextStream::read(qint64 maxlen);
impl<'a> /*trait*/ QTextStream_read for (i64) {
  fn read(self, this: &mut QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream4readEx()};
    let arg0 = self  as c_longlong;
    unsafe {_ZN11QTextStream4readEx(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn setPadChar<T: QTextStream_setPadChar>(&mut self, value: T) -> i32 {
    value.setPadChar(self);
    return 1;
  }
}

pub trait QTextStream_setPadChar {
  fn setPadChar(self, this: &mut QTextStream) -> i32;
}

// proto: void QTextStream::setPadChar(QChar ch);
impl<'a> /*trait*/ QTextStream_setPadChar for (QChar) {
  fn setPadChar(self, this: &mut QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream10setPadCharE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QTextStream10setPadCharE5QChar(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn realNumberPrecision<T: QTextStream_realNumberPrecision>(&mut self, value: T) -> i32 {
    value.realNumberPrecision(self);
    return 1;
  }
}

pub trait QTextStream_realNumberPrecision {
  fn realNumberPrecision(self, this: &mut QTextStream) -> i32;
}

// proto: int QTextStream::realNumberPrecision();
impl<'a> /*trait*/ QTextStream_realNumberPrecision for () {
  fn realNumberPrecision(self, this: &mut QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextStream19realNumberPrecisionEv()};
    unsafe {_ZNK11QTextStream19realNumberPrecisionEv()};
    return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn pos<T: QTextStream_pos>(&mut self, value: T) -> i32 {
    value.pos(self);
    return 1;
  }
}

pub trait QTextStream_pos {
  fn pos(self, this: &mut QTextStream) -> i32;
}

// proto: long long QTextStream::pos();
impl<'a> /*trait*/ QTextStream_pos for () {
  fn pos(self, this: &mut QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextStream3posEv()};
    unsafe {_ZNK11QTextStream3posEv()};
    return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn readAll<T: QTextStream_readAll>(&mut self, value: T) -> i32 {
    value.readAll(self);
    return 1;
  }
}

pub trait QTextStream_readAll {
  fn readAll(self, this: &mut QTextStream) -> i32;
}

// proto: QString QTextStream::readAll();
impl<'a> /*trait*/ QTextStream_readAll for () {
  fn readAll(self, this: &mut QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream7readAllEv()};
    unsafe {_ZN11QTextStream7readAllEv()};
    return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn skipWhiteSpace<T: QTextStream_skipWhiteSpace>(&mut self, value: T) -> i32 {
    value.skipWhiteSpace(self);
    return 1;
  }
}

pub trait QTextStream_skipWhiteSpace {
  fn skipWhiteSpace(self, this: &mut QTextStream) -> i32;
}

// proto: void QTextStream::skipWhiteSpace();
impl<'a> /*trait*/ QTextStream_skipWhiteSpace for () {
  fn skipWhiteSpace(self, this: &mut QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream14skipWhiteSpaceEv()};
    unsafe {_ZN11QTextStream14skipWhiteSpaceEv()};
    return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn setFieldWidth<T: QTextStream_setFieldWidth>(&mut self, value: T) -> i32 {
    value.setFieldWidth(self);
    return 1;
  }
}

pub trait QTextStream_setFieldWidth {
  fn setFieldWidth(self, this: &mut QTextStream) -> i32;
}

// proto: void QTextStream::setFieldWidth(int width);
impl<'a> /*trait*/ QTextStream_setFieldWidth for (i32) {
  fn setFieldWidth(self, this: &mut QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream13setFieldWidthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QTextStream13setFieldWidthEi(arg0)};
    return 1;
  }
}

// proto: void QTextStream::setCodec(const char * codecName);
impl<'a> /*trait*/ QTextStream_setCodec for (&'a  String) {
  fn setCodec(self, this: &mut QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream8setCodecEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    unsafe {_ZN11QTextStream8setCodecEPKc(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn integerBase<T: QTextStream_integerBase>(&mut self, value: T) -> i32 {
    value.integerBase(self);
    return 1;
  }
}

pub trait QTextStream_integerBase {
  fn integerBase(self, this: &mut QTextStream) -> i32;
}

// proto: int QTextStream::integerBase();
impl<'a> /*trait*/ QTextStream_integerBase for () {
  fn integerBase(self, this: &mut QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextStream11integerBaseEv()};
    unsafe {_ZNK11QTextStream11integerBaseEv()};
    return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn setNumberFlags<T: QTextStream_setNumberFlags>(&mut self, value: T) -> i32 {
    value.setNumberFlags(self);
    return 1;
  }
}

pub trait QTextStream_setNumberFlags {
  fn setNumberFlags(self, this: &mut QTextStream) -> i32;
}

// proto: void QTextStream::setNumberFlags(NumberFlags flags);
impl<'a> /*trait*/ QTextStream_setNumberFlags for (i32) {
  fn setNumberFlags(self, this: &mut QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream14setNumberFlagsE6QFlagsINS_10NumberFlagEE()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QTextStream14setNumberFlagsE6QFlagsINS_10NumberFlagEE(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn readLine<T: QTextStream_readLine>(&mut self, value: T) -> i32 {
    value.readLine(self);
    return 1;
  }
}

pub trait QTextStream_readLine {
  fn readLine(self, this: &mut QTextStream) -> i32;
}

// proto: QString QTextStream::readLine(qint64 maxlen);
impl<'a> /*trait*/ QTextStream_readLine for (i64) {
  fn readLine(self, this: &mut QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream8readLineEx()};
    let arg0 = self  as c_longlong;
    unsafe {_ZN11QTextStream8readLineEx(arg0)};
    return 1;
  }
}

// proto: void QTextStream::NewQTextStream(const QTextStream & );
impl<'a> /*trait*/ QTextStream_NewQTextStream for (&'a  QTextStream) {
  fn NewQTextStream(self) -> QTextStream {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStreamC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QTextStreamC1ERKS_(qthis, arg0)};
    let rsthis = QTextStream{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

