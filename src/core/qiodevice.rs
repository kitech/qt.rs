// auto generated, do not modify.
// created: Sun Jan 17 17:37:11 2016
// src-file: /QtCore/qiodevice.h
// dst-file: /src/core/qiodevice.rs
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
use super::qobject::QObject; // 773
use std::ops::Deref;
use super::qstring::QString; // 773
use super::qbytearray::QByteArray; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QIODevice_Class_Size() -> c_int;
  // proto:  void QIODevice::ungetChar(char c);
  fn _ZN9QIODevice9ungetCharEc(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QString QIODevice::errorString();
  fn _ZNK9QIODevice11errorStringEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  qint64 QIODevice::write(const QByteArray & data);
  fn _ZN9QIODevice5writeERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_longlong;
  // proto:  qint64 QIODevice::write(const char * data);
  fn _ZN9QIODevice5writeEPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_char) -> c_longlong;
  // proto:  bool QIODevice::isReadable();
  fn _ZNK9QIODevice10isReadableEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  qint64 QIODevice::readLine(char * data, qint64 maxlen);
  fn _ZN9QIODevice8readLineEPcx(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_longlong) -> c_longlong;
  // proto:  QByteArray QIODevice::readLine(qint64 maxlen);
  fn _ZN9QIODevice8readLineEx(qthis: u64 /* *mut c_void*/, arg0: c_longlong) -> *mut c_void;
  // proto:  bool QIODevice::waitForReadyRead(int msecs);
  fn _ZN9QIODevice16waitForReadyReadEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  qint64 QIODevice::size();
  fn _ZNK9QIODevice4sizeEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  bool QIODevice::getChar(char * c);
  fn _ZN9QIODevice7getCharEPc(qthis: u64 /* *mut c_void*/, arg0: *mut c_char) -> c_char;
  // proto:  bool QIODevice::putChar(char c);
  fn _ZN9QIODevice7putCharEc(qthis: u64 /* *mut c_void*/, arg0: c_char) -> c_char;
  // proto:  bool QIODevice::isTextModeEnabled();
  fn _ZNK9QIODevice17isTextModeEnabledEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QIODevice::isSequential();
  fn _ZNK9QIODevice12isSequentialEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  qint64 QIODevice::bytesAvailable();
  fn _ZNK9QIODevice14bytesAvailableEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  qint64 QIODevice::write(const char * data, qint64 len);
  fn _ZN9QIODevice5writeEPKcx(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_longlong) -> c_longlong;
  // proto:  void QIODevice::close();
  fn _ZN9QIODevice5closeEv(qthis: u64 /* *mut c_void*/);
  // proto:  QByteArray QIODevice::readAll();
  fn _ZN9QIODevice7readAllEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QIODevice::atEnd();
  fn _ZNK9QIODevice5atEndEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QIODevice::seek(qint64 pos);
  fn _ZN9QIODevice4seekEx(qthis: u64 /* *mut c_void*/, arg0: c_longlong) -> c_char;
  // proto:  void QIODevice::QIODevice(const QIODevice & );
  fn _ZN9QIODeviceC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  qint64 QIODevice::pos();
  fn _ZNK9QIODevice3posEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  QByteArray QIODevice::read(qint64 maxlen);
  fn _ZN9QIODevice4readEx(qthis: u64 /* *mut c_void*/, arg0: c_longlong) -> *mut c_void;
  // proto:  qint64 QIODevice::peek(char * data, qint64 maxlen);
  fn _ZN9QIODevice4peekEPcx(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_longlong) -> c_longlong;
  // proto:  qint64 QIODevice::read(char * data, qint64 maxlen);
  fn _ZN9QIODevice4readEPcx(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_longlong) -> c_longlong;
  // proto:  bool QIODevice::waitForBytesWritten(int msecs);
  fn _ZN9QIODevice19waitForBytesWrittenEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  qint64 QIODevice::bytesToWrite();
  fn _ZNK9QIODevice12bytesToWriteEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  bool QIODevice::reset();
  fn _ZN9QIODevice5resetEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QIODevice::isWritable();
  fn _ZNK9QIODevice10isWritableEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QByteArray QIODevice::peek(qint64 maxlen);
  fn _ZN9QIODevice4peekEx(qthis: u64 /* *mut c_void*/, arg0: c_longlong) -> *mut c_void;
  // proto:  void QIODevice::QIODevice(QObject * parent);
  fn _ZN9QIODeviceC2EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QMetaObject * QIODevice::metaObject();
  fn _ZNK9QIODevice10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QIODevice::setTextModeEnabled(bool enabled);
  fn _ZN9QIODevice18setTextModeEnabledEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QIODevice::QIODevice();
  fn _ZN9QIODeviceC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QIODevice::isOpen();
  fn _ZNK9QIODevice6isOpenEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QIODevice::canReadLine();
  fn _ZNK9QIODevice11canReadLineEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QIODevice::~QIODevice();
  fn _ZN9QIODeviceD2Ev(qthis: u64 /* *mut c_void*/);
  fn QIODevice_SlotProxy_connect__ZN9QIODevice12bytesWrittenEx(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QIODevice_SlotProxy_connect__ZN9QIODevice12aboutToCloseEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QIODevice_SlotProxy_connect__ZN9QIODevice9readyReadEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QIODevice_SlotProxy_connect__ZN9QIODevice19readChannelFinishedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QIODevice)=1
#[derive(Default)]
pub struct QIODevice {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _readyRead: QIODevice_readyRead_signal,
  pub _readChannelFinished: QIODevice_readChannelFinished_signal,
  pub _aboutToClose: QIODevice_aboutToClose_signal,
  pub _bytesWritten: QIODevice_bytesWritten_signal,
}

impl /*struct*/ QIODevice {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QIODevice {
    return QIODevice{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QIODevice {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QIODevice {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QIODevice::ungetChar(char c);
impl /*struct*/ QIODevice {
  pub fn ungetChar<RetType, T: QIODevice_ungetChar<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ungetChar(self);
    // return 1;
  }
}

pub trait QIODevice_ungetChar<RetType> {
  fn ungetChar(self , rsthis: & QIODevice) -> RetType;
}

  // proto:  void QIODevice::ungetChar(char c);
impl<'a> /*trait*/ QIODevice_ungetChar<()> for (i8) {
  fn ungetChar(self , rsthis: & QIODevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice9ungetCharEc()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QIODevice9ungetCharEc(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QIODevice::errorString();
impl /*struct*/ QIODevice {
  pub fn errorString<RetType, T: QIODevice_errorString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.errorString(self);
    // return 1;
  }
}

pub trait QIODevice_errorString<RetType> {
  fn errorString(self , rsthis: & QIODevice) -> RetType;
}

  // proto:  QString QIODevice::errorString();
impl<'a> /*trait*/ QIODevice_errorString<QString> for () {
  fn errorString(self , rsthis: & QIODevice) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QIODevice11errorStringEv()};
    let mut ret = unsafe {_ZNK9QIODevice11errorStringEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  qint64 QIODevice::write(const QByteArray & data);
impl /*struct*/ QIODevice {
  pub fn write<RetType, T: QIODevice_write<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.write(self);
    // return 1;
  }
}

pub trait QIODevice_write<RetType> {
  fn write(self , rsthis: & QIODevice) -> RetType;
}

  // proto:  qint64 QIODevice::write(const QByteArray & data);
impl<'a> /*trait*/ QIODevice_write<i64> for (&'a QByteArray) {
  fn write(self , rsthis: & QIODevice) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice5writeERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QIODevice5writeERK10QByteArray(rsthis.qclsinst, arg0)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  qint64 QIODevice::write(const char * data);
impl<'a> /*trait*/ QIODevice_write<i64> for (&'a  String) {
  fn write(self , rsthis: & QIODevice) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice5writeEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN9QIODevice5writeEPKc(rsthis.qclsinst, arg0)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  bool QIODevice::isReadable();
impl /*struct*/ QIODevice {
  pub fn isReadable<RetType, T: QIODevice_isReadable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isReadable(self);
    // return 1;
  }
}

pub trait QIODevice_isReadable<RetType> {
  fn isReadable(self , rsthis: & QIODevice) -> RetType;
}

  // proto:  bool QIODevice::isReadable();
impl<'a> /*trait*/ QIODevice_isReadable<i8> for () {
  fn isReadable(self , rsthis: & QIODevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QIODevice10isReadableEv()};
    let mut ret = unsafe {_ZNK9QIODevice10isReadableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qint64 QIODevice::readLine(char * data, qint64 maxlen);
impl /*struct*/ QIODevice {
  pub fn readLine<RetType, T: QIODevice_readLine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.readLine(self);
    // return 1;
  }
}

pub trait QIODevice_readLine<RetType> {
  fn readLine(self , rsthis: & QIODevice) -> RetType;
}

  // proto:  qint64 QIODevice::readLine(char * data, qint64 maxlen);
impl<'a> /*trait*/ QIODevice_readLine<i64> for (&'a mut String, i64) {
  fn readLine(self , rsthis: & QIODevice) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice8readLineEPcx()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_longlong;
    let mut ret = unsafe {_ZN9QIODevice8readLineEPcx(rsthis.qclsinst, arg0, arg1)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  QByteArray QIODevice::readLine(qint64 maxlen);
impl<'a> /*trait*/ QIODevice_readLine<QByteArray> for (i64) {
  fn readLine(self , rsthis: & QIODevice) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice8readLineEx()};
    let arg0 = self  as c_longlong;
    let mut ret = unsafe {_ZN9QIODevice8readLineEx(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QIODevice::waitForReadyRead(int msecs);
impl /*struct*/ QIODevice {
  pub fn waitForReadyRead<RetType, T: QIODevice_waitForReadyRead<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.waitForReadyRead(self);
    // return 1;
  }
}

pub trait QIODevice_waitForReadyRead<RetType> {
  fn waitForReadyRead(self , rsthis: & QIODevice) -> RetType;
}

  // proto:  bool QIODevice::waitForReadyRead(int msecs);
impl<'a> /*trait*/ QIODevice_waitForReadyRead<i8> for (i32) {
  fn waitForReadyRead(self , rsthis: & QIODevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice16waitForReadyReadEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN9QIODevice16waitForReadyReadEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qint64 QIODevice::size();
impl /*struct*/ QIODevice {
  pub fn size<RetType, T: QIODevice_size<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QIODevice_size<RetType> {
  fn size(self , rsthis: & QIODevice) -> RetType;
}

  // proto:  qint64 QIODevice::size();
impl<'a> /*trait*/ QIODevice_size<i64> for () {
  fn size(self , rsthis: & QIODevice) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QIODevice4sizeEv()};
    let mut ret = unsafe {_ZNK9QIODevice4sizeEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  bool QIODevice::getChar(char * c);
impl /*struct*/ QIODevice {
  pub fn getChar<RetType, T: QIODevice_getChar<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.getChar(self);
    // return 1;
  }
}

pub trait QIODevice_getChar<RetType> {
  fn getChar(self , rsthis: & QIODevice) -> RetType;
}

  // proto:  bool QIODevice::getChar(char * c);
impl<'a> /*trait*/ QIODevice_getChar<i8> for (&'a mut String) {
  fn getChar(self , rsthis: & QIODevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice7getCharEPc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN9QIODevice7getCharEPc(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QIODevice::putChar(char c);
impl /*struct*/ QIODevice {
  pub fn putChar<RetType, T: QIODevice_putChar<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.putChar(self);
    // return 1;
  }
}

pub trait QIODevice_putChar<RetType> {
  fn putChar(self , rsthis: & QIODevice) -> RetType;
}

  // proto:  bool QIODevice::putChar(char c);
impl<'a> /*trait*/ QIODevice_putChar<i8> for (i8) {
  fn putChar(self , rsthis: & QIODevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice7putCharEc()};
    let arg0 = self  as c_char;
    let mut ret = unsafe {_ZN9QIODevice7putCharEc(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QIODevice::isTextModeEnabled();
impl /*struct*/ QIODevice {
  pub fn isTextModeEnabled<RetType, T: QIODevice_isTextModeEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isTextModeEnabled(self);
    // return 1;
  }
}

pub trait QIODevice_isTextModeEnabled<RetType> {
  fn isTextModeEnabled(self , rsthis: & QIODevice) -> RetType;
}

  // proto:  bool QIODevice::isTextModeEnabled();
impl<'a> /*trait*/ QIODevice_isTextModeEnabled<i8> for () {
  fn isTextModeEnabled(self , rsthis: & QIODevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QIODevice17isTextModeEnabledEv()};
    let mut ret = unsafe {_ZNK9QIODevice17isTextModeEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QIODevice::isSequential();
impl /*struct*/ QIODevice {
  pub fn isSequential<RetType, T: QIODevice_isSequential<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSequential(self);
    // return 1;
  }
}

pub trait QIODevice_isSequential<RetType> {
  fn isSequential(self , rsthis: & QIODevice) -> RetType;
}

  // proto:  bool QIODevice::isSequential();
impl<'a> /*trait*/ QIODevice_isSequential<i8> for () {
  fn isSequential(self , rsthis: & QIODevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QIODevice12isSequentialEv()};
    let mut ret = unsafe {_ZNK9QIODevice12isSequentialEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qint64 QIODevice::bytesAvailable();
impl /*struct*/ QIODevice {
  pub fn bytesAvailable<RetType, T: QIODevice_bytesAvailable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bytesAvailable(self);
    // return 1;
  }
}

pub trait QIODevice_bytesAvailable<RetType> {
  fn bytesAvailable(self , rsthis: & QIODevice) -> RetType;
}

  // proto:  qint64 QIODevice::bytesAvailable();
impl<'a> /*trait*/ QIODevice_bytesAvailable<i64> for () {
  fn bytesAvailable(self , rsthis: & QIODevice) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QIODevice14bytesAvailableEv()};
    let mut ret = unsafe {_ZNK9QIODevice14bytesAvailableEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  qint64 QIODevice::write(const char * data, qint64 len);
impl<'a> /*trait*/ QIODevice_write<i64> for (&'a  String, i64) {
  fn write(self , rsthis: & QIODevice) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice5writeEPKcx()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_longlong;
    let mut ret = unsafe {_ZN9QIODevice5writeEPKcx(rsthis.qclsinst, arg0, arg1)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  void QIODevice::close();
impl /*struct*/ QIODevice {
  pub fn close<RetType, T: QIODevice_close<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.close(self);
    // return 1;
  }
}

pub trait QIODevice_close<RetType> {
  fn close(self , rsthis: & QIODevice) -> RetType;
}

  // proto:  void QIODevice::close();
impl<'a> /*trait*/ QIODevice_close<()> for () {
  fn close(self , rsthis: & QIODevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice5closeEv()};
     unsafe {_ZN9QIODevice5closeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QByteArray QIODevice::readAll();
impl /*struct*/ QIODevice {
  pub fn readAll<RetType, T: QIODevice_readAll<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.readAll(self);
    // return 1;
  }
}

pub trait QIODevice_readAll<RetType> {
  fn readAll(self , rsthis: & QIODevice) -> RetType;
}

  // proto:  QByteArray QIODevice::readAll();
impl<'a> /*trait*/ QIODevice_readAll<QByteArray> for () {
  fn readAll(self , rsthis: & QIODevice) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice7readAllEv()};
    let mut ret = unsafe {_ZN9QIODevice7readAllEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QIODevice::atEnd();
impl /*struct*/ QIODevice {
  pub fn atEnd<RetType, T: QIODevice_atEnd<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.atEnd(self);
    // return 1;
  }
}

pub trait QIODevice_atEnd<RetType> {
  fn atEnd(self , rsthis: & QIODevice) -> RetType;
}

  // proto:  bool QIODevice::atEnd();
impl<'a> /*trait*/ QIODevice_atEnd<i8> for () {
  fn atEnd(self , rsthis: & QIODevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QIODevice5atEndEv()};
    let mut ret = unsafe {_ZNK9QIODevice5atEndEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QIODevice::seek(qint64 pos);
impl /*struct*/ QIODevice {
  pub fn seek<RetType, T: QIODevice_seek<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.seek(self);
    // return 1;
  }
}

pub trait QIODevice_seek<RetType> {
  fn seek(self , rsthis: & QIODevice) -> RetType;
}

  // proto:  bool QIODevice::seek(qint64 pos);
impl<'a> /*trait*/ QIODevice_seek<i8> for (i64) {
  fn seek(self , rsthis: & QIODevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice4seekEx()};
    let arg0 = self  as c_longlong;
    let mut ret = unsafe {_ZN9QIODevice4seekEx(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QIODevice::QIODevice(const QIODevice & );
impl /*struct*/ QIODevice {
  pub fn new<T: QIODevice_new>(value: T) -> QIODevice {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QIODevice_new {
  fn new(self) -> QIODevice;
}

  // proto:  void QIODevice::QIODevice(const QIODevice & );
impl<'a> /*trait*/ QIODevice_new for (&'a QIODevice) {
  fn new(self) -> QIODevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODeviceC2ERKS_()};
    let ctysz: c_int = unsafe{QIODevice_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QIODeviceC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QIODevice{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  qint64 QIODevice::pos();
impl /*struct*/ QIODevice {
  pub fn pos<RetType, T: QIODevice_pos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QIODevice_pos<RetType> {
  fn pos(self , rsthis: & QIODevice) -> RetType;
}

  // proto:  qint64 QIODevice::pos();
impl<'a> /*trait*/ QIODevice_pos<i64> for () {
  fn pos(self , rsthis: & QIODevice) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QIODevice3posEv()};
    let mut ret = unsafe {_ZNK9QIODevice3posEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  QByteArray QIODevice::read(qint64 maxlen);
impl /*struct*/ QIODevice {
  pub fn read<RetType, T: QIODevice_read<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.read(self);
    // return 1;
  }
}

pub trait QIODevice_read<RetType> {
  fn read(self , rsthis: & QIODevice) -> RetType;
}

  // proto:  QByteArray QIODevice::read(qint64 maxlen);
impl<'a> /*trait*/ QIODevice_read<QByteArray> for (i64) {
  fn read(self , rsthis: & QIODevice) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice4readEx()};
    let arg0 = self  as c_longlong;
    let mut ret = unsafe {_ZN9QIODevice4readEx(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  qint64 QIODevice::peek(char * data, qint64 maxlen);
impl /*struct*/ QIODevice {
  pub fn peek<RetType, T: QIODevice_peek<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.peek(self);
    // return 1;
  }
}

pub trait QIODevice_peek<RetType> {
  fn peek(self , rsthis: & QIODevice) -> RetType;
}

  // proto:  qint64 QIODevice::peek(char * data, qint64 maxlen);
impl<'a> /*trait*/ QIODevice_peek<i64> for (&'a mut String, i64) {
  fn peek(self , rsthis: & QIODevice) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice4peekEPcx()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_longlong;
    let mut ret = unsafe {_ZN9QIODevice4peekEPcx(rsthis.qclsinst, arg0, arg1)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  qint64 QIODevice::read(char * data, qint64 maxlen);
impl<'a> /*trait*/ QIODevice_read<i64> for (&'a mut String, i64) {
  fn read(self , rsthis: & QIODevice) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice4readEPcx()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_longlong;
    let mut ret = unsafe {_ZN9QIODevice4readEPcx(rsthis.qclsinst, arg0, arg1)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  bool QIODevice::waitForBytesWritten(int msecs);
impl /*struct*/ QIODevice {
  pub fn waitForBytesWritten<RetType, T: QIODevice_waitForBytesWritten<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.waitForBytesWritten(self);
    // return 1;
  }
}

pub trait QIODevice_waitForBytesWritten<RetType> {
  fn waitForBytesWritten(self , rsthis: & QIODevice) -> RetType;
}

  // proto:  bool QIODevice::waitForBytesWritten(int msecs);
impl<'a> /*trait*/ QIODevice_waitForBytesWritten<i8> for (i32) {
  fn waitForBytesWritten(self , rsthis: & QIODevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice19waitForBytesWrittenEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN9QIODevice19waitForBytesWrittenEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qint64 QIODevice::bytesToWrite();
impl /*struct*/ QIODevice {
  pub fn bytesToWrite<RetType, T: QIODevice_bytesToWrite<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bytesToWrite(self);
    // return 1;
  }
}

pub trait QIODevice_bytesToWrite<RetType> {
  fn bytesToWrite(self , rsthis: & QIODevice) -> RetType;
}

  // proto:  qint64 QIODevice::bytesToWrite();
impl<'a> /*trait*/ QIODevice_bytesToWrite<i64> for () {
  fn bytesToWrite(self , rsthis: & QIODevice) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QIODevice12bytesToWriteEv()};
    let mut ret = unsafe {_ZNK9QIODevice12bytesToWriteEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  bool QIODevice::reset();
impl /*struct*/ QIODevice {
  pub fn reset<RetType, T: QIODevice_reset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.reset(self);
    // return 1;
  }
}

pub trait QIODevice_reset<RetType> {
  fn reset(self , rsthis: & QIODevice) -> RetType;
}

  // proto:  bool QIODevice::reset();
impl<'a> /*trait*/ QIODevice_reset<i8> for () {
  fn reset(self , rsthis: & QIODevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice5resetEv()};
    let mut ret = unsafe {_ZN9QIODevice5resetEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QIODevice::isWritable();
impl /*struct*/ QIODevice {
  pub fn isWritable<RetType, T: QIODevice_isWritable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isWritable(self);
    // return 1;
  }
}

pub trait QIODevice_isWritable<RetType> {
  fn isWritable(self , rsthis: & QIODevice) -> RetType;
}

  // proto:  bool QIODevice::isWritable();
impl<'a> /*trait*/ QIODevice_isWritable<i8> for () {
  fn isWritable(self , rsthis: & QIODevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QIODevice10isWritableEv()};
    let mut ret = unsafe {_ZNK9QIODevice10isWritableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QByteArray QIODevice::peek(qint64 maxlen);
impl<'a> /*trait*/ QIODevice_peek<QByteArray> for (i64) {
  fn peek(self , rsthis: & QIODevice) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice4peekEx()};
    let arg0 = self  as c_longlong;
    let mut ret = unsafe {_ZN9QIODevice4peekEx(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QIODevice::QIODevice(QObject * parent);
impl<'a> /*trait*/ QIODevice_new for (&'a QObject) {
  fn new(self) -> QIODevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODeviceC2EP7QObject()};
    let ctysz: c_int = unsafe{QIODevice_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QIODeviceC2EP7QObject(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QIODevice{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QIODevice::metaObject();
impl /*struct*/ QIODevice {
  pub fn metaObject<RetType, T: QIODevice_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QIODevice_metaObject<RetType> {
  fn metaObject(self , rsthis: & QIODevice) -> RetType;
}

  // proto:  const QMetaObject * QIODevice::metaObject();
impl<'a> /*trait*/ QIODevice_metaObject<()> for () {
  fn metaObject(self , rsthis: & QIODevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QIODevice10metaObjectEv()};
     unsafe {_ZNK9QIODevice10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QIODevice::setTextModeEnabled(bool enabled);
impl /*struct*/ QIODevice {
  pub fn setTextModeEnabled<RetType, T: QIODevice_setTextModeEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTextModeEnabled(self);
    // return 1;
  }
}

pub trait QIODevice_setTextModeEnabled<RetType> {
  fn setTextModeEnabled(self , rsthis: & QIODevice) -> RetType;
}

  // proto:  void QIODevice::setTextModeEnabled(bool enabled);
impl<'a> /*trait*/ QIODevice_setTextModeEnabled<()> for (i8) {
  fn setTextModeEnabled(self , rsthis: & QIODevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice18setTextModeEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QIODevice18setTextModeEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QIODevice::QIODevice();
impl<'a> /*trait*/ QIODevice_new for () {
  fn new(self) -> QIODevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODeviceC2Ev()};
    let ctysz: c_int = unsafe{QIODevice_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN9QIODeviceC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QIODevice{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QIODevice::isOpen();
impl /*struct*/ QIODevice {
  pub fn isOpen<RetType, T: QIODevice_isOpen<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isOpen(self);
    // return 1;
  }
}

pub trait QIODevice_isOpen<RetType> {
  fn isOpen(self , rsthis: & QIODevice) -> RetType;
}

  // proto:  bool QIODevice::isOpen();
impl<'a> /*trait*/ QIODevice_isOpen<i8> for () {
  fn isOpen(self , rsthis: & QIODevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QIODevice6isOpenEv()};
    let mut ret = unsafe {_ZNK9QIODevice6isOpenEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QIODevice::canReadLine();
impl /*struct*/ QIODevice {
  pub fn canReadLine<RetType, T: QIODevice_canReadLine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.canReadLine(self);
    // return 1;
  }
}

pub trait QIODevice_canReadLine<RetType> {
  fn canReadLine(self , rsthis: & QIODevice) -> RetType;
}

  // proto:  bool QIODevice::canReadLine();
impl<'a> /*trait*/ QIODevice_canReadLine<i8> for () {
  fn canReadLine(self , rsthis: & QIODevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QIODevice11canReadLineEv()};
    let mut ret = unsafe {_ZNK9QIODevice11canReadLineEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QIODevice::~QIODevice();
impl /*struct*/ QIODevice {
  pub fn free<RetType, T: QIODevice_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QIODevice_free<RetType> {
  fn free(self , rsthis: & QIODevice) -> RetType;
}

  // proto:  void QIODevice::~QIODevice();
impl<'a> /*trait*/ QIODevice_free<()> for () {
  fn free(self , rsthis: & QIODevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODeviceD2Ev()};
     unsafe {_ZN9QIODeviceD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

#[derive(Default)] // for QIODevice_readyRead
pub struct QIODevice_readyRead_signal{poi:u64}
impl /* struct */ QIODevice {
  pub fn readyRead(&self) -> QIODevice_readyRead_signal {
     return QIODevice_readyRead_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QIODevice_readyRead_signal {
  pub fn connect<T: QIODevice_readyRead_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QIODevice_readyRead_signal_connect {
  fn connect(self, sigthis: QIODevice_readyRead_signal);
}

#[derive(Default)] // for QIODevice_readChannelFinished
pub struct QIODevice_readChannelFinished_signal{poi:u64}
impl /* struct */ QIODevice {
  pub fn readChannelFinished(&self) -> QIODevice_readChannelFinished_signal {
     return QIODevice_readChannelFinished_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QIODevice_readChannelFinished_signal {
  pub fn connect<T: QIODevice_readChannelFinished_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QIODevice_readChannelFinished_signal_connect {
  fn connect(self, sigthis: QIODevice_readChannelFinished_signal);
}

#[derive(Default)] // for QIODevice_aboutToClose
pub struct QIODevice_aboutToClose_signal{poi:u64}
impl /* struct */ QIODevice {
  pub fn aboutToClose(&self) -> QIODevice_aboutToClose_signal {
     return QIODevice_aboutToClose_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QIODevice_aboutToClose_signal {
  pub fn connect<T: QIODevice_aboutToClose_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QIODevice_aboutToClose_signal_connect {
  fn connect(self, sigthis: QIODevice_aboutToClose_signal);
}

#[derive(Default)] // for QIODevice_bytesWritten
pub struct QIODevice_bytesWritten_signal{poi:u64}
impl /* struct */ QIODevice {
  pub fn bytesWritten(&self) -> QIODevice_bytesWritten_signal {
     return QIODevice_bytesWritten_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QIODevice_bytesWritten_signal {
  pub fn connect<T: QIODevice_bytesWritten_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QIODevice_bytesWritten_signal_connect {
  fn connect(self, sigthis: QIODevice_bytesWritten_signal);
}

// bytesWritten(qint64)
extern fn QIODevice_bytesWritten_signal_connect_cb_0(rsfptr:fn(i64), arg0: c_longlong) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i64;
  rsfptr(rsarg0);
}
extern fn QIODevice_bytesWritten_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(i64)>, arg0: c_longlong) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i64;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QIODevice_bytesWritten_signal_connect for fn(i64) {
  fn connect(self, sigthis: QIODevice_bytesWritten_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QIODevice_bytesWritten_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QIODevice_SlotProxy_connect__ZN9QIODevice12bytesWrittenEx(arg0, arg1, arg2)};
  }
}
impl /* trait */ QIODevice_bytesWritten_signal_connect for Box<Fn(i64)> {
  fn connect(self, sigthis: QIODevice_bytesWritten_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QIODevice_bytesWritten_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QIODevice_SlotProxy_connect__ZN9QIODevice12bytesWrittenEx(arg0, arg1, arg2)};
  }
}
// aboutToClose()
extern fn QIODevice_aboutToClose_signal_connect_cb_1(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QIODevice_aboutToClose_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QIODevice_aboutToClose_signal_connect for fn() {
  fn connect(self, sigthis: QIODevice_aboutToClose_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QIODevice_aboutToClose_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QIODevice_SlotProxy_connect__ZN9QIODevice12aboutToCloseEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QIODevice_aboutToClose_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QIODevice_aboutToClose_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QIODevice_aboutToClose_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QIODevice_SlotProxy_connect__ZN9QIODevice12aboutToCloseEv(arg0, arg1, arg2)};
  }
}
// readyRead()
extern fn QIODevice_readyRead_signal_connect_cb_2(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QIODevice_readyRead_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QIODevice_readyRead_signal_connect for fn() {
  fn connect(self, sigthis: QIODevice_readyRead_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QIODevice_readyRead_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QIODevice_SlotProxy_connect__ZN9QIODevice9readyReadEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QIODevice_readyRead_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QIODevice_readyRead_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QIODevice_readyRead_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QIODevice_SlotProxy_connect__ZN9QIODevice9readyReadEv(arg0, arg1, arg2)};
  }
}
// readChannelFinished()
extern fn QIODevice_readChannelFinished_signal_connect_cb_3(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QIODevice_readChannelFinished_signal_connect_cb_box_3(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QIODevice_readChannelFinished_signal_connect for fn() {
  fn connect(self, sigthis: QIODevice_readChannelFinished_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QIODevice_readChannelFinished_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QIODevice_SlotProxy_connect__ZN9QIODevice19readChannelFinishedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QIODevice_readChannelFinished_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QIODevice_readChannelFinished_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QIODevice_readChannelFinished_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QIODevice_SlotProxy_connect__ZN9QIODevice19readChannelFinishedEv(arg0, arg1, arg2)};
  }
}
// <= body block end

