// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
// src-file: /QtCore/qtextstream.h
// dst-file: /src/core/qtextstream.rs
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
// use super::qtextstream::QTextStream; // 773
use super::qchar::*; // 773
use super::qtextcodec::*; // 773
use super::qiodevice::*; // 773
use super::qlocale::*; // 773
use super::qstring::*; // 773
use super::qbytearray::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QTextStreamManipulator_Class_Size() -> c_int;
  // proto:  void QTextStreamManipulator::exec(QTextStream & s);
  fn C_ZN22QTextStreamManipulator4execER11QTextStream(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTextStreamManipulator::QTextStreamManipulator(QTSMFI m, int a);
  fn C_ZN22QTextStreamManipulatorC2EM11QTextStreamFviEi(arg0: *mut c_void, arg1: c_int) -> u64;
  // proto:  void QTextStreamManipulator::QTextStreamManipulator(QTSMFC m, QChar c);
  fn C_ZN22QTextStreamManipulatorC2EM11QTextStreamFv5QCharES1_(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  fn QTextStream_Class_Size() -> c_int;
  // proto:  QTextCodec * QTextStream::codec();
  fn C_ZNK11QTextStream5codecEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTextStream::QTextStream(QIODevice * device);
  fn C_ZN11QTextStreamC2EP9QIODevice(arg0: *mut c_void) -> u64;
  // proto:  void QTextStream::setLocale(const QLocale & locale);
  fn C_ZN11QTextStream9setLocaleERK7QLocale(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTextStream::QTextStream();
  fn C_ZN11QTextStreamC2Ev() -> u64;
  // proto:  bool QTextStream::atEnd();
  fn C_ZNK11QTextStream5atEndEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QTextStream::readLineInto(QString * line, qint64 maxlen);
  fn C_ZN11QTextStream12readLineIntoEP7QStringx(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_longlong) -> c_char;
  // proto:  void QTextStream::setRealNumberPrecision(int precision);
  fn C_ZN11QTextStream22setRealNumberPrecisionEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QTextStream::setDevice(QIODevice * device);
  fn C_ZN11QTextStream9setDeviceEP9QIODevice(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTextStream::reset();
  fn C_ZN11QTextStream5resetEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QTextStream::seek(qint64 pos);
  fn C_ZN11QTextStream4seekEx(qthis: u64 /* *mut c_void*/, arg0: c_longlong) -> c_char;
  // proto:  QString * QTextStream::string();
  fn C_ZNK11QTextStream6stringEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTextStream::setAutoDetectUnicode(bool enabled);
  fn C_ZN11QTextStream20setAutoDetectUnicodeEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QChar QTextStream::padChar();
  fn C_ZNK11QTextStream7padCharEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QIODevice * QTextStream::device();
  fn C_ZNK11QTextStream6deviceEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTextStream::resetStatus();
  fn C_ZN11QTextStream11resetStatusEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QTextStream::autoDetectUnicode();
  fn C_ZNK11QTextStream17autoDetectUnicodeEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QTextStream::fieldWidth();
  fn C_ZNK11QTextStream10fieldWidthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QTextStream::generateByteOrderMark();
  fn C_ZNK11QTextStream21generateByteOrderMarkEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QTextStream::setGenerateByteOrderMark(bool generate);
  fn C_ZN11QTextStream24setGenerateByteOrderMarkEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QTextStream::setCodec(QTextCodec * codec);
  fn C_ZN11QTextStream8setCodecEP10QTextCodec(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTextStream::flush();
  fn C_ZN11QTextStream5flushEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QTextStream::setIntegerBase(int base);
  fn C_ZN11QTextStream14setIntegerBaseEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QTextStream::~QTextStream();
  fn C_ZN11QTextStreamD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QLocale QTextStream::locale();
  fn C_ZNK11QTextStream6localeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QTextStream::read(qint64 maxlen);
  fn C_ZN11QTextStream4readEx(qthis: u64 /* *mut c_void*/, arg0: c_longlong) -> *mut c_void;
  // proto:  void QTextStream::setPadChar(QChar ch);
  fn C_ZN11QTextStream10setPadCharE5QChar(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QTextStream::realNumberPrecision();
  fn C_ZNK11QTextStream19realNumberPrecisionEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  qint64 QTextStream::pos();
  fn C_ZNK11QTextStream3posEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  QString QTextStream::readAll();
  fn C_ZN11QTextStream7readAllEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTextStream::skipWhiteSpace();
  fn C_ZN11QTextStream14skipWhiteSpaceEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QTextStream::setFieldWidth(int width);
  fn C_ZN11QTextStream13setFieldWidthEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QTextStream::setCodec(const char * codecName);
  fn C_ZN11QTextStream8setCodecEPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_char);
  // proto:  int QTextStream::integerBase();
  fn C_ZNK11QTextStream11integerBaseEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QString QTextStream::readLine(qint64 maxlen);
  fn C_ZN11QTextStream8readLineEx(qthis: u64 /* *mut c_void*/, arg0: c_longlong) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QTextStreamManipulator)=40
#[derive(Default)]
pub struct QTextStreamManipulator {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QTextStream)=1
#[derive(Default)]
pub struct QTextStream {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QTextStreamManipulator {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTextStreamManipulator {
    return QTextStreamManipulator{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QTextStreamManipulator::exec(QTextStream & s);
impl /*struct*/ QTextStreamManipulator {
  pub fn exec<RetType, T: QTextStreamManipulator_exec<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.exec(self);
    // return 1;
  }
}

pub trait QTextStreamManipulator_exec<RetType> {
  fn exec(self , rsthis: & QTextStreamManipulator) -> RetType;
}

  // proto:  void QTextStreamManipulator::exec(QTextStream & s);
impl<'a> /*trait*/ QTextStreamManipulator_exec<()> for (&'a QTextStream) {
  fn exec(self , rsthis: & QTextStreamManipulator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN22QTextStreamManipulator4execER11QTextStream()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN22QTextStreamManipulator4execER11QTextStream(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextStreamManipulator::QTextStreamManipulator(QTSMFI m, int a);
impl /*struct*/ QTextStreamManipulator {
  pub fn new<T: QTextStreamManipulator_new>(value: T) -> QTextStreamManipulator {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QTextStreamManipulator_new {
  fn new(self) -> QTextStreamManipulator;
}

  // proto:  void QTextStreamManipulator::QTextStreamManipulator(QTSMFI m, int a);
impl<'a> /*trait*/ QTextStreamManipulator_new for (*mut u64, i32) {
  fn new(self) -> QTextStreamManipulator {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN22QTextStreamManipulatorC2EM11QTextStreamFviEi()};
    let ctysz: c_int = unsafe{QTextStreamManipulator_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as *mut c_void;
    let arg1 = self.1  as c_int;
    let qthis: u64 = unsafe {C_ZN22QTextStreamManipulatorC2EM11QTextStreamFviEi(arg0, arg1)};
    let rsthis = QTextStreamManipulator{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextStreamManipulator::QTextStreamManipulator(QTSMFC m, QChar c);
impl<'a> /*trait*/ QTextStreamManipulator_new for (*mut u64, QChar) {
  fn new(self) -> QTextStreamManipulator {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN22QTextStreamManipulatorC2EM11QTextStreamFv5QCharES1_()};
    let ctysz: c_int = unsafe{QTextStreamManipulator_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN22QTextStreamManipulatorC2EM11QTextStreamFv5QCharES1_(arg0, arg1)};
    let rsthis = QTextStreamManipulator{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextStream {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTextStream {
    return QTextStream{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QTextCodec * QTextStream::codec();
impl /*struct*/ QTextStream {
  pub fn codec<RetType, T: QTextStream_codec<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.codec(self);
    // return 1;
  }
}

pub trait QTextStream_codec<RetType> {
  fn codec(self , rsthis: & QTextStream) -> RetType;
}

  // proto:  QTextCodec * QTextStream::codec();
impl<'a> /*trait*/ QTextStream_codec<QTextCodec> for () {
  fn codec(self , rsthis: & QTextStream) -> QTextCodec {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextStream5codecEv()};
    let mut ret = unsafe {C_ZNK11QTextStream5codecEv(rsthis.qclsinst)};
    let mut ret1 = QTextCodec::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextStream::QTextStream(QIODevice * device);
impl /*struct*/ QTextStream {
  pub fn new<T: QTextStream_new>(value: T) -> QTextStream {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QTextStream_new {
  fn new(self) -> QTextStream;
}

  // proto:  void QTextStream::QTextStream(QIODevice * device);
impl<'a> /*trait*/ QTextStream_new for (&'a QIODevice) {
  fn new(self) -> QTextStream {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStreamC2EP9QIODevice()};
    let ctysz: c_int = unsafe{QTextStream_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN11QTextStreamC2EP9QIODevice(arg0)};
    let rsthis = QTextStream{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextStream::setLocale(const QLocale & locale);
impl /*struct*/ QTextStream {
  pub fn setLocale<RetType, T: QTextStream_setLocale<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLocale(self);
    // return 1;
  }
}

pub trait QTextStream_setLocale<RetType> {
  fn setLocale(self , rsthis: & QTextStream) -> RetType;
}

  // proto:  void QTextStream::setLocale(const QLocale & locale);
impl<'a> /*trait*/ QTextStream_setLocale<()> for (&'a QLocale) {
  fn setLocale(self , rsthis: & QTextStream) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream9setLocaleERK7QLocale()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QTextStream9setLocaleERK7QLocale(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextStream::QTextStream();
impl<'a> /*trait*/ QTextStream_new for () {
  fn new(self) -> QTextStream {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStreamC2Ev()};
    let ctysz: c_int = unsafe{QTextStream_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN11QTextStreamC2Ev()};
    let rsthis = QTextStream{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QTextStream::atEnd();
impl /*struct*/ QTextStream {
  pub fn atEnd<RetType, T: QTextStream_atEnd<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.atEnd(self);
    // return 1;
  }
}

pub trait QTextStream_atEnd<RetType> {
  fn atEnd(self , rsthis: & QTextStream) -> RetType;
}

  // proto:  bool QTextStream::atEnd();
impl<'a> /*trait*/ QTextStream_atEnd<i8> for () {
  fn atEnd(self , rsthis: & QTextStream) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextStream5atEndEv()};
    let mut ret = unsafe {C_ZNK11QTextStream5atEndEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QTextStream::readLineInto(QString * line, qint64 maxlen);
impl /*struct*/ QTextStream {
  pub fn readLineInto<RetType, T: QTextStream_readLineInto<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.readLineInto(self);
    // return 1;
  }
}

pub trait QTextStream_readLineInto<RetType> {
  fn readLineInto(self , rsthis: & QTextStream) -> RetType;
}

  // proto:  bool QTextStream::readLineInto(QString * line, qint64 maxlen);
impl<'a> /*trait*/ QTextStream_readLineInto<i8> for (&'a QString, i64) {
  fn readLineInto(self , rsthis: & QTextStream) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream12readLineIntoEP7QStringx()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_longlong;
    let mut ret = unsafe {C_ZN11QTextStream12readLineIntoEP7QStringx(rsthis.qclsinst, arg0, arg1)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QTextStream::setRealNumberPrecision(int precision);
impl /*struct*/ QTextStream {
  pub fn setRealNumberPrecision<RetType, T: QTextStream_setRealNumberPrecision<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRealNumberPrecision(self);
    // return 1;
  }
}

pub trait QTextStream_setRealNumberPrecision<RetType> {
  fn setRealNumberPrecision(self , rsthis: & QTextStream) -> RetType;
}

  // proto:  void QTextStream::setRealNumberPrecision(int precision);
impl<'a> /*trait*/ QTextStream_setRealNumberPrecision<()> for (i32) {
  fn setRealNumberPrecision(self , rsthis: & QTextStream) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream22setRealNumberPrecisionEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN11QTextStream22setRealNumberPrecisionEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextStream::setDevice(QIODevice * device);
impl /*struct*/ QTextStream {
  pub fn setDevice<RetType, T: QTextStream_setDevice<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDevice(self);
    // return 1;
  }
}

pub trait QTextStream_setDevice<RetType> {
  fn setDevice(self , rsthis: & QTextStream) -> RetType;
}

  // proto:  void QTextStream::setDevice(QIODevice * device);
impl<'a> /*trait*/ QTextStream_setDevice<()> for (&'a QIODevice) {
  fn setDevice(self , rsthis: & QTextStream) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream9setDeviceEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QTextStream9setDeviceEP9QIODevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextStream::reset();
impl /*struct*/ QTextStream {
  pub fn reset<RetType, T: QTextStream_reset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.reset(self);
    // return 1;
  }
}

pub trait QTextStream_reset<RetType> {
  fn reset(self , rsthis: & QTextStream) -> RetType;
}

  // proto:  void QTextStream::reset();
impl<'a> /*trait*/ QTextStream_reset<()> for () {
  fn reset(self , rsthis: & QTextStream) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream5resetEv()};
     unsafe {C_ZN11QTextStream5resetEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QTextStream::seek(qint64 pos);
impl /*struct*/ QTextStream {
  pub fn seek<RetType, T: QTextStream_seek<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.seek(self);
    // return 1;
  }
}

pub trait QTextStream_seek<RetType> {
  fn seek(self , rsthis: & QTextStream) -> RetType;
}

  // proto:  bool QTextStream::seek(qint64 pos);
impl<'a> /*trait*/ QTextStream_seek<i8> for (i64) {
  fn seek(self , rsthis: & QTextStream) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream4seekEx()};
    let arg0 = self  as c_longlong;
    let mut ret = unsafe {C_ZN11QTextStream4seekEx(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QString * QTextStream::string();
impl /*struct*/ QTextStream {
  pub fn string<RetType, T: QTextStream_string<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.string(self);
    // return 1;
  }
}

pub trait QTextStream_string<RetType> {
  fn string(self , rsthis: & QTextStream) -> RetType;
}

  // proto:  QString * QTextStream::string();
impl<'a> /*trait*/ QTextStream_string<QString> for () {
  fn string(self , rsthis: & QTextStream) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextStream6stringEv()};
    let mut ret = unsafe {C_ZNK11QTextStream6stringEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextStream::setAutoDetectUnicode(bool enabled);
impl /*struct*/ QTextStream {
  pub fn setAutoDetectUnicode<RetType, T: QTextStream_setAutoDetectUnicode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAutoDetectUnicode(self);
    // return 1;
  }
}

pub trait QTextStream_setAutoDetectUnicode<RetType> {
  fn setAutoDetectUnicode(self , rsthis: & QTextStream) -> RetType;
}

  // proto:  void QTextStream::setAutoDetectUnicode(bool enabled);
impl<'a> /*trait*/ QTextStream_setAutoDetectUnicode<()> for (i8) {
  fn setAutoDetectUnicode(self , rsthis: & QTextStream) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream20setAutoDetectUnicodeEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN11QTextStream20setAutoDetectUnicodeEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QChar QTextStream::padChar();
impl /*struct*/ QTextStream {
  pub fn padChar<RetType, T: QTextStream_padChar<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.padChar(self);
    // return 1;
  }
}

pub trait QTextStream_padChar<RetType> {
  fn padChar(self , rsthis: & QTextStream) -> RetType;
}

  // proto:  QChar QTextStream::padChar();
impl<'a> /*trait*/ QTextStream_padChar<QChar> for () {
  fn padChar(self , rsthis: & QTextStream) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextStream7padCharEv()};
    let mut ret = unsafe {C_ZNK11QTextStream7padCharEv(rsthis.qclsinst)};
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QIODevice * QTextStream::device();
impl /*struct*/ QTextStream {
  pub fn device<RetType, T: QTextStream_device<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.device(self);
    // return 1;
  }
}

pub trait QTextStream_device<RetType> {
  fn device(self , rsthis: & QTextStream) -> RetType;
}

  // proto:  QIODevice * QTextStream::device();
impl<'a> /*trait*/ QTextStream_device<QIODevice> for () {
  fn device(self , rsthis: & QTextStream) -> QIODevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextStream6deviceEv()};
    let mut ret = unsafe {C_ZNK11QTextStream6deviceEv(rsthis.qclsinst)};
    let mut ret1 = QIODevice::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextStream::resetStatus();
impl /*struct*/ QTextStream {
  pub fn resetStatus<RetType, T: QTextStream_resetStatus<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resetStatus(self);
    // return 1;
  }
}

pub trait QTextStream_resetStatus<RetType> {
  fn resetStatus(self , rsthis: & QTextStream) -> RetType;
}

  // proto:  void QTextStream::resetStatus();
impl<'a> /*trait*/ QTextStream_resetStatus<()> for () {
  fn resetStatus(self , rsthis: & QTextStream) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream11resetStatusEv()};
     unsafe {C_ZN11QTextStream11resetStatusEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QTextStream::autoDetectUnicode();
impl /*struct*/ QTextStream {
  pub fn autoDetectUnicode<RetType, T: QTextStream_autoDetectUnicode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.autoDetectUnicode(self);
    // return 1;
  }
}

pub trait QTextStream_autoDetectUnicode<RetType> {
  fn autoDetectUnicode(self , rsthis: & QTextStream) -> RetType;
}

  // proto:  bool QTextStream::autoDetectUnicode();
impl<'a> /*trait*/ QTextStream_autoDetectUnicode<i8> for () {
  fn autoDetectUnicode(self , rsthis: & QTextStream) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextStream17autoDetectUnicodeEv()};
    let mut ret = unsafe {C_ZNK11QTextStream17autoDetectUnicodeEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  int QTextStream::fieldWidth();
impl /*struct*/ QTextStream {
  pub fn fieldWidth<RetType, T: QTextStream_fieldWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fieldWidth(self);
    // return 1;
  }
}

pub trait QTextStream_fieldWidth<RetType> {
  fn fieldWidth(self , rsthis: & QTextStream) -> RetType;
}

  // proto:  int QTextStream::fieldWidth();
impl<'a> /*trait*/ QTextStream_fieldWidth<i32> for () {
  fn fieldWidth(self , rsthis: & QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextStream10fieldWidthEv()};
    let mut ret = unsafe {C_ZNK11QTextStream10fieldWidthEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  bool QTextStream::generateByteOrderMark();
impl /*struct*/ QTextStream {
  pub fn generateByteOrderMark<RetType, T: QTextStream_generateByteOrderMark<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.generateByteOrderMark(self);
    // return 1;
  }
}

pub trait QTextStream_generateByteOrderMark<RetType> {
  fn generateByteOrderMark(self , rsthis: & QTextStream) -> RetType;
}

  // proto:  bool QTextStream::generateByteOrderMark();
impl<'a> /*trait*/ QTextStream_generateByteOrderMark<i8> for () {
  fn generateByteOrderMark(self , rsthis: & QTextStream) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextStream21generateByteOrderMarkEv()};
    let mut ret = unsafe {C_ZNK11QTextStream21generateByteOrderMarkEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QTextStream::setGenerateByteOrderMark(bool generate);
impl /*struct*/ QTextStream {
  pub fn setGenerateByteOrderMark<RetType, T: QTextStream_setGenerateByteOrderMark<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setGenerateByteOrderMark(self);
    // return 1;
  }
}

pub trait QTextStream_setGenerateByteOrderMark<RetType> {
  fn setGenerateByteOrderMark(self , rsthis: & QTextStream) -> RetType;
}

  // proto:  void QTextStream::setGenerateByteOrderMark(bool generate);
impl<'a> /*trait*/ QTextStream_setGenerateByteOrderMark<()> for (i8) {
  fn setGenerateByteOrderMark(self , rsthis: & QTextStream) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream24setGenerateByteOrderMarkEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN11QTextStream24setGenerateByteOrderMarkEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextStream::setCodec(QTextCodec * codec);
impl /*struct*/ QTextStream {
  pub fn setCodec<RetType, T: QTextStream_setCodec<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCodec(self);
    // return 1;
  }
}

pub trait QTextStream_setCodec<RetType> {
  fn setCodec(self , rsthis: & QTextStream) -> RetType;
}

  // proto:  void QTextStream::setCodec(QTextCodec * codec);
impl<'a> /*trait*/ QTextStream_setCodec<()> for (&'a QTextCodec) {
  fn setCodec(self , rsthis: & QTextStream) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream8setCodecEP10QTextCodec()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QTextStream8setCodecEP10QTextCodec(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextStream::flush();
impl /*struct*/ QTextStream {
  pub fn flush<RetType, T: QTextStream_flush<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.flush(self);
    // return 1;
  }
}

pub trait QTextStream_flush<RetType> {
  fn flush(self , rsthis: & QTextStream) -> RetType;
}

  // proto:  void QTextStream::flush();
impl<'a> /*trait*/ QTextStream_flush<()> for () {
  fn flush(self , rsthis: & QTextStream) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream5flushEv()};
     unsafe {C_ZN11QTextStream5flushEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextStream::setIntegerBase(int base);
impl /*struct*/ QTextStream {
  pub fn setIntegerBase<RetType, T: QTextStream_setIntegerBase<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIntegerBase(self);
    // return 1;
  }
}

pub trait QTextStream_setIntegerBase<RetType> {
  fn setIntegerBase(self , rsthis: & QTextStream) -> RetType;
}

  // proto:  void QTextStream::setIntegerBase(int base);
impl<'a> /*trait*/ QTextStream_setIntegerBase<()> for (i32) {
  fn setIntegerBase(self , rsthis: & QTextStream) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream14setIntegerBaseEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN11QTextStream14setIntegerBaseEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextStream::~QTextStream();
impl /*struct*/ QTextStream {
  pub fn free<RetType, T: QTextStream_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QTextStream_free<RetType> {
  fn free(self , rsthis: & QTextStream) -> RetType;
}

  // proto:  void QTextStream::~QTextStream();
impl<'a> /*trait*/ QTextStream_free<()> for () {
  fn free(self , rsthis: & QTextStream) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStreamD2Ev()};
     unsafe {C_ZN11QTextStreamD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QLocale QTextStream::locale();
impl /*struct*/ QTextStream {
  pub fn locale<RetType, T: QTextStream_locale<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.locale(self);
    // return 1;
  }
}

pub trait QTextStream_locale<RetType> {
  fn locale(self , rsthis: & QTextStream) -> RetType;
}

  // proto:  QLocale QTextStream::locale();
impl<'a> /*trait*/ QTextStream_locale<QLocale> for () {
  fn locale(self , rsthis: & QTextStream) -> QLocale {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextStream6localeEv()};
    let mut ret = unsafe {C_ZNK11QTextStream6localeEv(rsthis.qclsinst)};
    let mut ret1 = QLocale::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QTextStream::read(qint64 maxlen);
impl /*struct*/ QTextStream {
  pub fn read<RetType, T: QTextStream_read<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.read(self);
    // return 1;
  }
}

pub trait QTextStream_read<RetType> {
  fn read(self , rsthis: & QTextStream) -> RetType;
}

  // proto:  QString QTextStream::read(qint64 maxlen);
impl<'a> /*trait*/ QTextStream_read<QString> for (i64) {
  fn read(self , rsthis: & QTextStream) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream4readEx()};
    let arg0 = self  as c_longlong;
    let mut ret = unsafe {C_ZN11QTextStream4readEx(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextStream::setPadChar(QChar ch);
impl /*struct*/ QTextStream {
  pub fn setPadChar<RetType, T: QTextStream_setPadChar<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPadChar(self);
    // return 1;
  }
}

pub trait QTextStream_setPadChar<RetType> {
  fn setPadChar(self , rsthis: & QTextStream) -> RetType;
}

  // proto:  void QTextStream::setPadChar(QChar ch);
impl<'a> /*trait*/ QTextStream_setPadChar<()> for (QChar) {
  fn setPadChar(self , rsthis: & QTextStream) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream10setPadCharE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QTextStream10setPadCharE5QChar(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTextStream::realNumberPrecision();
impl /*struct*/ QTextStream {
  pub fn realNumberPrecision<RetType, T: QTextStream_realNumberPrecision<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.realNumberPrecision(self);
    // return 1;
  }
}

pub trait QTextStream_realNumberPrecision<RetType> {
  fn realNumberPrecision(self , rsthis: & QTextStream) -> RetType;
}

  // proto:  int QTextStream::realNumberPrecision();
impl<'a> /*trait*/ QTextStream_realNumberPrecision<i32> for () {
  fn realNumberPrecision(self , rsthis: & QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextStream19realNumberPrecisionEv()};
    let mut ret = unsafe {C_ZNK11QTextStream19realNumberPrecisionEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  qint64 QTextStream::pos();
impl /*struct*/ QTextStream {
  pub fn pos<RetType, T: QTextStream_pos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QTextStream_pos<RetType> {
  fn pos(self , rsthis: & QTextStream) -> RetType;
}

  // proto:  qint64 QTextStream::pos();
impl<'a> /*trait*/ QTextStream_pos<i64> for () {
  fn pos(self , rsthis: & QTextStream) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextStream3posEv()};
    let mut ret = unsafe {C_ZNK11QTextStream3posEv(rsthis.qclsinst)};
    return ret as i64; // 1
    // return 1;
  }
}

  // proto:  QString QTextStream::readAll();
impl /*struct*/ QTextStream {
  pub fn readAll<RetType, T: QTextStream_readAll<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.readAll(self);
    // return 1;
  }
}

pub trait QTextStream_readAll<RetType> {
  fn readAll(self , rsthis: & QTextStream) -> RetType;
}

  // proto:  QString QTextStream::readAll();
impl<'a> /*trait*/ QTextStream_readAll<QString> for () {
  fn readAll(self , rsthis: & QTextStream) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream7readAllEv()};
    let mut ret = unsafe {C_ZN11QTextStream7readAllEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextStream::skipWhiteSpace();
impl /*struct*/ QTextStream {
  pub fn skipWhiteSpace<RetType, T: QTextStream_skipWhiteSpace<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.skipWhiteSpace(self);
    // return 1;
  }
}

pub trait QTextStream_skipWhiteSpace<RetType> {
  fn skipWhiteSpace(self , rsthis: & QTextStream) -> RetType;
}

  // proto:  void QTextStream::skipWhiteSpace();
impl<'a> /*trait*/ QTextStream_skipWhiteSpace<()> for () {
  fn skipWhiteSpace(self , rsthis: & QTextStream) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream14skipWhiteSpaceEv()};
     unsafe {C_ZN11QTextStream14skipWhiteSpaceEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextStream::setFieldWidth(int width);
impl /*struct*/ QTextStream {
  pub fn setFieldWidth<RetType, T: QTextStream_setFieldWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFieldWidth(self);
    // return 1;
  }
}

pub trait QTextStream_setFieldWidth<RetType> {
  fn setFieldWidth(self , rsthis: & QTextStream) -> RetType;
}

  // proto:  void QTextStream::setFieldWidth(int width);
impl<'a> /*trait*/ QTextStream_setFieldWidth<()> for (i32) {
  fn setFieldWidth(self , rsthis: & QTextStream) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream13setFieldWidthEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN11QTextStream13setFieldWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextStream::setCodec(const char * codecName);
impl<'a> /*trait*/ QTextStream_setCodec<()> for (&'a  String) {
  fn setCodec(self , rsthis: & QTextStream) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream8setCodecEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
     unsafe {C_ZN11QTextStream8setCodecEPKc(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTextStream::integerBase();
impl /*struct*/ QTextStream {
  pub fn integerBase<RetType, T: QTextStream_integerBase<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.integerBase(self);
    // return 1;
  }
}

pub trait QTextStream_integerBase<RetType> {
  fn integerBase(self , rsthis: & QTextStream) -> RetType;
}

  // proto:  int QTextStream::integerBase();
impl<'a> /*trait*/ QTextStream_integerBase<i32> for () {
  fn integerBase(self , rsthis: & QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextStream11integerBaseEv()};
    let mut ret = unsafe {C_ZNK11QTextStream11integerBaseEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QString QTextStream::readLine(qint64 maxlen);
impl /*struct*/ QTextStream {
  pub fn readLine<RetType, T: QTextStream_readLine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.readLine(self);
    // return 1;
  }
}

pub trait QTextStream_readLine<RetType> {
  fn readLine(self , rsthis: & QTextStream) -> RetType;
}

  // proto:  QString QTextStream::readLine(qint64 maxlen);
impl<'a> /*trait*/ QTextStream_readLine<QString> for (i64) {
  fn readLine(self , rsthis: & QTextStream) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextStream8readLineEx()};
    let arg0 = self  as c_longlong;
    let mut ret = unsafe {C_ZN11QTextStream8readLineEx(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

