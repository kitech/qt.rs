// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qtextcodec::QTextCodec;
use super::qiodevice::QIODevice;
use super::qlocale::QLocale;
use super::qstring::QString;
use super::qchar::QChar;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QTextCodec * QTextStream::codec();
  fn _ZNK11QTextStream5codecEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextStream::NewQTextStream(QIODevice * device);
  fn _ZN11QTextStreamC1EP9QIODevice(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextStream::setLocale(const QLocale & locale);
  fn _ZN11QTextStream9setLocaleERK7QLocale(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextStream::NewQTextStream();
  fn _ZN11QTextStreamC1Ev(qthis: *mut c_void) ;
  // proto:  bool QTextStream::atEnd();
  fn _ZNK11QTextStream5atEndEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QTextStream::readLineInto(QString * line, qint64 maxlen);
  fn _ZN11QTextStream12readLineIntoEP7QStringx(qthis: *mut c_void, arg0: *mut c_void, arg1: c_longlong) -> int8_t;
  // proto:  void QTextStream::setRealNumberPrecision(int precision);
  fn _ZN11QTextStream22setRealNumberPrecisionEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTextStream::setDevice(QIODevice * device);
  fn _ZN11QTextStream9setDeviceEP9QIODevice(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextStream::reset();
  fn _ZN11QTextStream5resetEv(qthis: *mut c_void) ;
  // proto:  bool QTextStream::seek(qint64 pos);
  fn _ZN11QTextStream4seekEx(qthis: *mut c_void, arg0: c_longlong) -> int8_t;
  // proto:  QString * QTextStream::string();
  fn _ZNK11QTextStream6stringEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextStream::setAutoDetectUnicode(bool enabled);
  fn _ZN11QTextStream20setAutoDetectUnicodeEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QChar QTextStream::padChar();
  fn _ZNK11QTextStream7padCharEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QIODevice * QTextStream::device();
  fn _ZNK11QTextStream6deviceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextStream::resetStatus();
  fn _ZN11QTextStream11resetStatusEv(qthis: *mut c_void) ;
  // proto:  bool QTextStream::autoDetectUnicode();
  fn _ZNK11QTextStream17autoDetectUnicodeEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QTextStream::fieldWidth();
  fn _ZNK11QTextStream10fieldWidthEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QTextStream::generateByteOrderMark();
  fn _ZNK11QTextStream21generateByteOrderMarkEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTextStream::setGenerateByteOrderMark(bool generate);
  fn _ZN11QTextStream24setGenerateByteOrderMarkEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTextStream::setCodec(QTextCodec * codec);
  fn _ZN11QTextStream8setCodecEP10QTextCodec(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextStream::flush();
  fn _ZN11QTextStream5flushEv(qthis: *mut c_void) ;
  // proto:  void QTextStream::setIntegerBase(int base);
  fn _ZN11QTextStream14setIntegerBaseEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTextStream::FreeQTextStream();
  fn _ZN11QTextStreamD0Ev(qthis: *mut c_void) ;
  // proto:  QLocale QTextStream::locale();
  fn _ZNK11QTextStream6localeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QTextStream::read(qint64 maxlen);
  fn _ZN11QTextStream4readEx(qthis: *mut c_void, arg0: c_longlong) -> *mut c_void;
  // proto:  void QTextStream::setPadChar(QChar ch);
  fn _ZN11QTextStream10setPadCharE5QChar(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QTextStream::realNumberPrecision();
  fn _ZNK11QTextStream19realNumberPrecisionEv(qthis: *mut c_void) -> c_int;
  // proto:  long long QTextStream::pos();
  fn _ZNK11QTextStream3posEv(qthis: *mut c_void) -> c_longlong;
  // proto:  QString QTextStream::readAll();
  fn _ZN11QTextStream7readAllEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextStream::skipWhiteSpace();
  fn _ZN11QTextStream14skipWhiteSpaceEv(qthis: *mut c_void) ;
  // proto:  void QTextStream::setFieldWidth(int width);
  fn _ZN11QTextStream13setFieldWidthEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTextStream::setCodec(const char * codecName);
  fn _ZN11QTextStream8setCodecEPKc(qthis: *mut c_void, arg0: *const c_char) ;
  // proto:  int QTextStream::integerBase();
  fn _ZNK11QTextStream11integerBaseEv(qthis: *mut c_void) -> c_int;
  // proto:  QString QTextStream::readLine(qint64 maxlen);
  fn _ZN11QTextStream8readLineEx(qthis: *mut c_void, arg0: c_longlong) -> *mut c_void;
  // proto:  void QTextStream::NewQTextStream(const QTextStream & );
  fn _ZN11QTextStreamC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QTextStream)=1
pub struct QTextStream {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextStream {
  pub fn codec<RetType, T: QTextStream_codec<RetType>>(&mut self, value: T) -> RetType {
    return value.codec(self);
    // return 1;
  }
}

pub trait QTextStream_codec<RetType> {
  fn codec(self, rsthis: &mut QTextStream) -> RetType;
}

// proto:  QTextCodec * QTextStream::codec();
impl<'a> /*trait*/ QTextStream_codec<QTextCodec> for () {
  fn codec(self, rsthis: &mut QTextStream) -> QTextCodec {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextStream5codecEv()};
    let mut ret = unsafe {_ZNK11QTextStream5codecEv(rsthis.qclsinst)};
    let mut ret1 = QTextCodec{qclsinst: ret};
    return ret1;
    // return 1;
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
  pub fn setLocale<RetType, T: QTextStream_setLocale<RetType>>(&mut self, value: T) -> RetType {
    return value.setLocale(self);
    // return 1;
  }
}

pub trait QTextStream_setLocale<RetType> {
  fn setLocale(self, rsthis: &mut QTextStream) -> RetType;
}

// proto:  void QTextStream::setLocale(const QLocale & locale);
impl<'a> /*trait*/ QTextStream_setLocale<()> for (&'a  QLocale) {
  fn setLocale(self, rsthis: &mut QTextStream) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream9setLocaleERK7QLocale()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextStream9setLocaleERK7QLocale(rsthis.qclsinst, arg0)};
    // return 1;
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
  pub fn atEnd<RetType, T: QTextStream_atEnd<RetType>>(&mut self, value: T) -> RetType {
    return value.atEnd(self);
    // return 1;
  }
}

pub trait QTextStream_atEnd<RetType> {
  fn atEnd(self, rsthis: &mut QTextStream) -> RetType;
}

// proto:  bool QTextStream::atEnd();
impl<'a> /*trait*/ QTextStream_atEnd<i8> for () {
  fn atEnd(self, rsthis: &mut QTextStream) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextStream5atEndEv()};
    let mut ret = unsafe {_ZNK11QTextStream5atEndEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn readLineInto<RetType, T: QTextStream_readLineInto<RetType>>(&mut self, value: T) -> RetType {
    return value.readLineInto(self);
    // return 1;
  }
}

pub trait QTextStream_readLineInto<RetType> {
  fn readLineInto(self, rsthis: &mut QTextStream) -> RetType;
}

// proto:  bool QTextStream::readLineInto(QString * line, qint64 maxlen);
impl<'a> /*trait*/ QTextStream_readLineInto<i8> for (&'a mut QString, i64) {
  fn readLineInto(self, rsthis: &mut QTextStream) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream12readLineIntoEP7QStringx()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_longlong;
    let mut ret = unsafe {_ZN11QTextStream12readLineIntoEP7QStringx(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn setRealNumberPrecision<RetType, T: QTextStream_setRealNumberPrecision<RetType>>(&mut self, value: T) -> RetType {
    return value.setRealNumberPrecision(self);
    // return 1;
  }
}

pub trait QTextStream_setRealNumberPrecision<RetType> {
  fn setRealNumberPrecision(self, rsthis: &mut QTextStream) -> RetType;
}

// proto:  void QTextStream::setRealNumberPrecision(int precision);
impl<'a> /*trait*/ QTextStream_setRealNumberPrecision<()> for (i32) {
  fn setRealNumberPrecision(self, rsthis: &mut QTextStream) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream22setRealNumberPrecisionEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QTextStream22setRealNumberPrecisionEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn setDevice<RetType, T: QTextStream_setDevice<RetType>>(&mut self, value: T) -> RetType {
    return value.setDevice(self);
    // return 1;
  }
}

pub trait QTextStream_setDevice<RetType> {
  fn setDevice(self, rsthis: &mut QTextStream) -> RetType;
}

// proto:  void QTextStream::setDevice(QIODevice * device);
impl<'a> /*trait*/ QTextStream_setDevice<()> for (&'a mut QIODevice) {
  fn setDevice(self, rsthis: &mut QTextStream) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream9setDeviceEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextStream9setDeviceEP9QIODevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn reset<RetType, T: QTextStream_reset<RetType>>(&mut self, value: T) -> RetType {
    return value.reset(self);
    // return 1;
  }
}

pub trait QTextStream_reset<RetType> {
  fn reset(self, rsthis: &mut QTextStream) -> RetType;
}

// proto:  void QTextStream::reset();
impl<'a> /*trait*/ QTextStream_reset<()> for () {
  fn reset(self, rsthis: &mut QTextStream) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream5resetEv()};
     unsafe {_ZN11QTextStream5resetEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn seek<RetType, T: QTextStream_seek<RetType>>(&mut self, value: T) -> RetType {
    return value.seek(self);
    // return 1;
  }
}

pub trait QTextStream_seek<RetType> {
  fn seek(self, rsthis: &mut QTextStream) -> RetType;
}

// proto:  bool QTextStream::seek(qint64 pos);
impl<'a> /*trait*/ QTextStream_seek<i8> for (i64) {
  fn seek(self, rsthis: &mut QTextStream) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream4seekEx()};
    let arg0 = self  as c_longlong;
    let mut ret = unsafe {_ZN11QTextStream4seekEx(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn string<RetType, T: QTextStream_string<RetType>>(&mut self, value: T) -> RetType {
    return value.string(self);
    // return 1;
  }
}

pub trait QTextStream_string<RetType> {
  fn string(self, rsthis: &mut QTextStream) -> RetType;
}

// proto:  QString * QTextStream::string();
impl<'a> /*trait*/ QTextStream_string<QString> for () {
  fn string(self, rsthis: &mut QTextStream) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextStream6stringEv()};
    let mut ret = unsafe {_ZNK11QTextStream6stringEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn setAutoDetectUnicode<RetType, T: QTextStream_setAutoDetectUnicode<RetType>>(&mut self, value: T) -> RetType {
    return value.setAutoDetectUnicode(self);
    // return 1;
  }
}

pub trait QTextStream_setAutoDetectUnicode<RetType> {
  fn setAutoDetectUnicode(self, rsthis: &mut QTextStream) -> RetType;
}

// proto:  void QTextStream::setAutoDetectUnicode(bool enabled);
impl<'a> /*trait*/ QTextStream_setAutoDetectUnicode<()> for (i8) {
  fn setAutoDetectUnicode(self, rsthis: &mut QTextStream) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream20setAutoDetectUnicodeEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QTextStream20setAutoDetectUnicodeEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn padChar<RetType, T: QTextStream_padChar<RetType>>(&mut self, value: T) -> RetType {
    return value.padChar(self);
    // return 1;
  }
}

pub trait QTextStream_padChar<RetType> {
  fn padChar(self, rsthis: &mut QTextStream) -> RetType;
}

// proto:  QChar QTextStream::padChar();
impl<'a> /*trait*/ QTextStream_padChar<QChar> for () {
  fn padChar(self, rsthis: &mut QTextStream) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextStream7padCharEv()};
    let mut ret = unsafe {_ZNK11QTextStream7padCharEv(rsthis.qclsinst)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn device<RetType, T: QTextStream_device<RetType>>(&mut self, value: T) -> RetType {
    return value.device(self);
    // return 1;
  }
}

pub trait QTextStream_device<RetType> {
  fn device(self, rsthis: &mut QTextStream) -> RetType;
}

// proto:  QIODevice * QTextStream::device();
impl<'a> /*trait*/ QTextStream_device<QIODevice> for () {
  fn device(self, rsthis: &mut QTextStream) -> QIODevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextStream6deviceEv()};
    let mut ret = unsafe {_ZNK11QTextStream6deviceEv(rsthis.qclsinst)};
    let mut ret1 = QIODevice{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn resetStatus<RetType, T: QTextStream_resetStatus<RetType>>(&mut self, value: T) -> RetType {
    return value.resetStatus(self);
    // return 1;
  }
}

pub trait QTextStream_resetStatus<RetType> {
  fn resetStatus(self, rsthis: &mut QTextStream) -> RetType;
}

// proto:  void QTextStream::resetStatus();
impl<'a> /*trait*/ QTextStream_resetStatus<()> for () {
  fn resetStatus(self, rsthis: &mut QTextStream) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream11resetStatusEv()};
     unsafe {_ZN11QTextStream11resetStatusEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn autoDetectUnicode<RetType, T: QTextStream_autoDetectUnicode<RetType>>(&mut self, value: T) -> RetType {
    return value.autoDetectUnicode(self);
    // return 1;
  }
}

pub trait QTextStream_autoDetectUnicode<RetType> {
  fn autoDetectUnicode(self, rsthis: &mut QTextStream) -> RetType;
}

// proto:  bool QTextStream::autoDetectUnicode();
impl<'a> /*trait*/ QTextStream_autoDetectUnicode<i8> for () {
  fn autoDetectUnicode(self, rsthis: &mut QTextStream) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextStream17autoDetectUnicodeEv()};
    let mut ret = unsafe {_ZNK11QTextStream17autoDetectUnicodeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn fieldWidth<RetType, T: QTextStream_fieldWidth<RetType>>(&mut self, value: T) -> RetType {
    return value.fieldWidth(self);
    // return 1;
  }
}

pub trait QTextStream_fieldWidth<RetType> {
  fn fieldWidth(self, rsthis: &mut QTextStream) -> RetType;
}

// proto:  int QTextStream::fieldWidth();
impl<'a> /*trait*/ QTextStream_fieldWidth<i32> for () {
  fn fieldWidth(self, rsthis: &mut QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextStream10fieldWidthEv()};
    let mut ret = unsafe {_ZNK11QTextStream10fieldWidthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn generateByteOrderMark<RetType, T: QTextStream_generateByteOrderMark<RetType>>(&mut self, value: T) -> RetType {
    return value.generateByteOrderMark(self);
    // return 1;
  }
}

pub trait QTextStream_generateByteOrderMark<RetType> {
  fn generateByteOrderMark(self, rsthis: &mut QTextStream) -> RetType;
}

// proto:  bool QTextStream::generateByteOrderMark();
impl<'a> /*trait*/ QTextStream_generateByteOrderMark<i8> for () {
  fn generateByteOrderMark(self, rsthis: &mut QTextStream) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextStream21generateByteOrderMarkEv()};
    let mut ret = unsafe {_ZNK11QTextStream21generateByteOrderMarkEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn setGenerateByteOrderMark<RetType, T: QTextStream_setGenerateByteOrderMark<RetType>>(&mut self, value: T) -> RetType {
    return value.setGenerateByteOrderMark(self);
    // return 1;
  }
}

pub trait QTextStream_setGenerateByteOrderMark<RetType> {
  fn setGenerateByteOrderMark(self, rsthis: &mut QTextStream) -> RetType;
}

// proto:  void QTextStream::setGenerateByteOrderMark(bool generate);
impl<'a> /*trait*/ QTextStream_setGenerateByteOrderMark<()> for (i8) {
  fn setGenerateByteOrderMark(self, rsthis: &mut QTextStream) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream24setGenerateByteOrderMarkEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QTextStream24setGenerateByteOrderMarkEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn setCodec<RetType, T: QTextStream_setCodec<RetType>>(&mut self, value: T) -> RetType {
    return value.setCodec(self);
    // return 1;
  }
}

pub trait QTextStream_setCodec<RetType> {
  fn setCodec(self, rsthis: &mut QTextStream) -> RetType;
}

// proto:  void QTextStream::setCodec(QTextCodec * codec);
impl<'a> /*trait*/ QTextStream_setCodec<()> for (&'a mut QTextCodec) {
  fn setCodec(self, rsthis: &mut QTextStream) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream8setCodecEP10QTextCodec()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextStream8setCodecEP10QTextCodec(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn flush<RetType, T: QTextStream_flush<RetType>>(&mut self, value: T) -> RetType {
    return value.flush(self);
    // return 1;
  }
}

pub trait QTextStream_flush<RetType> {
  fn flush(self, rsthis: &mut QTextStream) -> RetType;
}

// proto:  void QTextStream::flush();
impl<'a> /*trait*/ QTextStream_flush<()> for () {
  fn flush(self, rsthis: &mut QTextStream) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream5flushEv()};
     unsafe {_ZN11QTextStream5flushEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn setIntegerBase<RetType, T: QTextStream_setIntegerBase<RetType>>(&mut self, value: T) -> RetType {
    return value.setIntegerBase(self);
    // return 1;
  }
}

pub trait QTextStream_setIntegerBase<RetType> {
  fn setIntegerBase(self, rsthis: &mut QTextStream) -> RetType;
}

// proto:  void QTextStream::setIntegerBase(int base);
impl<'a> /*trait*/ QTextStream_setIntegerBase<()> for (i32) {
  fn setIntegerBase(self, rsthis: &mut QTextStream) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream14setIntegerBaseEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QTextStream14setIntegerBaseEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn FreeQTextStream<RetType, T: QTextStream_FreeQTextStream<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQTextStream(self);
    // return 1;
  }
}

pub trait QTextStream_FreeQTextStream<RetType> {
  fn FreeQTextStream(self, rsthis: &mut QTextStream) -> RetType;
}

// proto:  void QTextStream::FreeQTextStream();
impl<'a> /*trait*/ QTextStream_FreeQTextStream<()> for () {
  fn FreeQTextStream(self, rsthis: &mut QTextStream) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStreamD0Ev()};
     unsafe {_ZN11QTextStreamD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn locale<RetType, T: QTextStream_locale<RetType>>(&mut self, value: T) -> RetType {
    return value.locale(self);
    // return 1;
  }
}

pub trait QTextStream_locale<RetType> {
  fn locale(self, rsthis: &mut QTextStream) -> RetType;
}

// proto:  QLocale QTextStream::locale();
impl<'a> /*trait*/ QTextStream_locale<QLocale> for () {
  fn locale(self, rsthis: &mut QTextStream) -> QLocale {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextStream6localeEv()};
    let mut ret = unsafe {_ZNK11QTextStream6localeEv(rsthis.qclsinst)};
    let mut ret1 = QLocale{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn read<RetType, T: QTextStream_read<RetType>>(&mut self, value: T) -> RetType {
    return value.read(self);
    // return 1;
  }
}

pub trait QTextStream_read<RetType> {
  fn read(self, rsthis: &mut QTextStream) -> RetType;
}

// proto:  QString QTextStream::read(qint64 maxlen);
impl<'a> /*trait*/ QTextStream_read<QString> for (i64) {
  fn read(self, rsthis: &mut QTextStream) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream4readEx()};
    let arg0 = self  as c_longlong;
    let mut ret = unsafe {_ZN11QTextStream4readEx(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn setPadChar<RetType, T: QTextStream_setPadChar<RetType>>(&mut self, value: T) -> RetType {
    return value.setPadChar(self);
    // return 1;
  }
}

pub trait QTextStream_setPadChar<RetType> {
  fn setPadChar(self, rsthis: &mut QTextStream) -> RetType;
}

// proto:  void QTextStream::setPadChar(QChar ch);
impl<'a> /*trait*/ QTextStream_setPadChar<()> for (QChar) {
  fn setPadChar(self, rsthis: &mut QTextStream) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream10setPadCharE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextStream10setPadCharE5QChar(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn realNumberPrecision<RetType, T: QTextStream_realNumberPrecision<RetType>>(&mut self, value: T) -> RetType {
    return value.realNumberPrecision(self);
    // return 1;
  }
}

pub trait QTextStream_realNumberPrecision<RetType> {
  fn realNumberPrecision(self, rsthis: &mut QTextStream) -> RetType;
}

// proto:  int QTextStream::realNumberPrecision();
impl<'a> /*trait*/ QTextStream_realNumberPrecision<i32> for () {
  fn realNumberPrecision(self, rsthis: &mut QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextStream19realNumberPrecisionEv()};
    let mut ret = unsafe {_ZNK11QTextStream19realNumberPrecisionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn pos<RetType, T: QTextStream_pos<RetType>>(&mut self, value: T) -> RetType {
    return value.pos(self);
    // return 1;
  }
}

pub trait QTextStream_pos<RetType> {
  fn pos(self, rsthis: &mut QTextStream) -> RetType;
}

// proto:  long long QTextStream::pos();
impl<'a> /*trait*/ QTextStream_pos<i64> for () {
  fn pos(self, rsthis: &mut QTextStream) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextStream3posEv()};
    let mut ret = unsafe {_ZNK11QTextStream3posEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn readAll<RetType, T: QTextStream_readAll<RetType>>(&mut self, value: T) -> RetType {
    return value.readAll(self);
    // return 1;
  }
}

pub trait QTextStream_readAll<RetType> {
  fn readAll(self, rsthis: &mut QTextStream) -> RetType;
}

// proto:  QString QTextStream::readAll();
impl<'a> /*trait*/ QTextStream_readAll<QString> for () {
  fn readAll(self, rsthis: &mut QTextStream) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream7readAllEv()};
    let mut ret = unsafe {_ZN11QTextStream7readAllEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn skipWhiteSpace<RetType, T: QTextStream_skipWhiteSpace<RetType>>(&mut self, value: T) -> RetType {
    return value.skipWhiteSpace(self);
    // return 1;
  }
}

pub trait QTextStream_skipWhiteSpace<RetType> {
  fn skipWhiteSpace(self, rsthis: &mut QTextStream) -> RetType;
}

// proto:  void QTextStream::skipWhiteSpace();
impl<'a> /*trait*/ QTextStream_skipWhiteSpace<()> for () {
  fn skipWhiteSpace(self, rsthis: &mut QTextStream) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream14skipWhiteSpaceEv()};
     unsafe {_ZN11QTextStream14skipWhiteSpaceEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn setFieldWidth<RetType, T: QTextStream_setFieldWidth<RetType>>(&mut self, value: T) -> RetType {
    return value.setFieldWidth(self);
    // return 1;
  }
}

pub trait QTextStream_setFieldWidth<RetType> {
  fn setFieldWidth(self, rsthis: &mut QTextStream) -> RetType;
}

// proto:  void QTextStream::setFieldWidth(int width);
impl<'a> /*trait*/ QTextStream_setFieldWidth<()> for (i32) {
  fn setFieldWidth(self, rsthis: &mut QTextStream) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream13setFieldWidthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QTextStream13setFieldWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QTextStream::setCodec(const char * codecName);
impl<'a> /*trait*/ QTextStream_setCodec<()> for (&'a  String) {
  fn setCodec(self, rsthis: &mut QTextStream) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream8setCodecEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
     unsafe {_ZN11QTextStream8setCodecEPKc(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn integerBase<RetType, T: QTextStream_integerBase<RetType>>(&mut self, value: T) -> RetType {
    return value.integerBase(self);
    // return 1;
  }
}

pub trait QTextStream_integerBase<RetType> {
  fn integerBase(self, rsthis: &mut QTextStream) -> RetType;
}

// proto:  int QTextStream::integerBase();
impl<'a> /*trait*/ QTextStream_integerBase<i32> for () {
  fn integerBase(self, rsthis: &mut QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextStream11integerBaseEv()};
    let mut ret = unsafe {_ZNK11QTextStream11integerBaseEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn readLine<RetType, T: QTextStream_readLine<RetType>>(&mut self, value: T) -> RetType {
    return value.readLine(self);
    // return 1;
  }
}

pub trait QTextStream_readLine<RetType> {
  fn readLine(self, rsthis: &mut QTextStream) -> RetType;
}

// proto:  QString QTextStream::readLine(qint64 maxlen);
impl<'a> /*trait*/ QTextStream_readLine<QString> for (i64) {
  fn readLine(self, rsthis: &mut QTextStream) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream8readLineEx()};
    let arg0 = self  as c_longlong;
    let mut ret = unsafe {_ZN11QTextStream8readLineEx(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QTextStream::NewQTextStream(const QTextStream & );
impl<'a> /*trait*/ QTextStream_NewQTextStream for (&'a  QTextStream) {
  fn NewQTextStream(self) -> QTextStream {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStreamC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QTextStreamC1ERKS_(qthis, arg0)};
    let rsthis = QTextStream{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

