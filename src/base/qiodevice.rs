// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qbytearray::QByteArray;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QIODevice::ungetChar(char c);
  fn _ZN9QIODevice9ungetCharEc(qthis: *mut c_void, arg0: c_char) ;
  // proto:  QString QIODevice::errorString();
  fn _ZNK9QIODevice11errorStringEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  long long QIODevice::write(const QByteArray & data);
  fn _ZN9QIODevice5writeERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) -> c_longlong;
  // proto:  long long QIODevice::write(const char * data);
  fn _ZN9QIODevice5writeEPKc(qthis: *mut c_void, arg0: *const c_char) -> c_longlong;
  // proto:  bool QIODevice::isReadable();
  fn _ZNK9QIODevice10isReadableEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QIODevice::readyRead();
  fn _ZN9QIODevice9readyReadEv(qthis: *mut c_void) ;
  // proto:  bool QIODevice::waitForReadyRead(int msecs);
  fn _ZN9QIODevice16waitForReadyReadEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  void QIODevice::aboutToClose();
  fn _ZN9QIODevice12aboutToCloseEv(qthis: *mut c_void) ;
  // proto:  long long QIODevice::size();
  fn _ZNK9QIODevice4sizeEv(qthis: *mut c_void) -> c_longlong;
  // proto:  bool QIODevice::getChar(char * c);
  fn _ZN9QIODevice7getCharEPc(qthis: *mut c_void, arg0: *mut c_char) -> int8_t;
  // proto:  bool QIODevice::putChar(char c);
  fn _ZN9QIODevice7putCharEc(qthis: *mut c_void, arg0: c_char) -> int8_t;
  // proto:  bool QIODevice::isTextModeEnabled();
  fn _ZNK9QIODevice17isTextModeEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QIODevice::isSequential();
  fn _ZNK9QIODevice12isSequentialEv(qthis: *mut c_void) -> int8_t;
  // proto:  long long QIODevice::bytesAvailable();
  fn _ZNK9QIODevice14bytesAvailableEv(qthis: *mut c_void) -> c_longlong;
  // proto:  long long QIODevice::write(const char * data, qint64 len);
  fn _ZN9QIODevice5writeEPKcx(qthis: *mut c_void, arg0: *const c_char, arg1: c_longlong) -> c_longlong;
  // proto:  void QIODevice::close();
  fn _ZN9QIODevice5closeEv(qthis: *mut c_void) ;
  // proto:  QByteArray QIODevice::readAll();
  fn _ZN9QIODevice7readAllEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QIODevice::atEnd();
  fn _ZNK9QIODevice5atEndEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QIODevice::seek(qint64 pos);
  fn _ZN9QIODevice4seekEx(qthis: *mut c_void, arg0: c_longlong) -> int8_t;
  // proto:  void QIODevice::NewQIODevice(const QIODevice & );
  fn _ZN9QIODeviceC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  long long QIODevice::pos();
  fn _ZNK9QIODevice3posEv(qthis: *mut c_void) -> c_longlong;
  // proto:  void QIODevice::readChannelFinished();
  fn _ZN9QIODevice19readChannelFinishedEv(qthis: *mut c_void) ;
  // proto:  void QIODevice::bytesWritten(qint64 bytes);
  fn _ZN9QIODevice12bytesWrittenEx(qthis: *mut c_void, arg0: c_longlong) ;
  // proto:  bool QIODevice::waitForBytesWritten(int msecs);
  fn _ZN9QIODevice19waitForBytesWrittenEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  long long QIODevice::bytesToWrite();
  fn _ZNK9QIODevice12bytesToWriteEv(qthis: *mut c_void) -> c_longlong;
  // proto:  bool QIODevice::reset();
  fn _ZN9QIODevice5resetEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QIODevice::isWritable();
  fn _ZNK9QIODevice10isWritableEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QIODevice::NewQIODevice(QObject * parent);
  fn _ZN9QIODeviceC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QMetaObject * QIODevice::metaObject();
  fn _ZNK9QIODevice10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QIODevice::setTextModeEnabled(bool enabled);
  fn _ZN9QIODevice18setTextModeEnabledEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QIODevice::NewQIODevice();
  fn _ZN9QIODeviceC1Ev(qthis: *mut c_void) ;
  // proto:  bool QIODevice::isOpen();
  fn _ZNK9QIODevice6isOpenEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QIODevice::canReadLine();
  fn _ZNK9QIODevice11canReadLineEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QIODevice::FreeQIODevice();
  fn _ZN9QIODeviceD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QIODevice)=1
pub struct QIODevice {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QIODevice {
  pub fn ungetChar<T: QIODevice_ungetChar>(&mut self, value: T)  {
     value.ungetChar(self);
    // return 1;
  }
}

pub trait QIODevice_ungetChar {
  fn ungetChar(self, rsthis: &mut QIODevice) ;
}

// proto:  void QIODevice::ungetChar(char c);
impl<'a> /*trait*/ QIODevice_ungetChar for (i8) {
  fn ungetChar(self, rsthis: &mut QIODevice)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice9ungetCharEc()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QIODevice9ungetCharEc(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn errorString<T: QIODevice_errorString>(&mut self, value: T) -> QString {
    return value.errorString(self);
    // return 1;
  }
}

pub trait QIODevice_errorString {
  fn errorString(self, rsthis: &mut QIODevice) -> QString;
}

// proto:  QString QIODevice::errorString();
impl<'a> /*trait*/ QIODevice_errorString for () {
  fn errorString(self, rsthis: &mut QIODevice) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QIODevice11errorStringEv()};
    let mut ret = unsafe {_ZNK9QIODevice11errorStringEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn write<T: QIODevice_write>(&mut self, value: T) -> i64 {
    return value.write(self);
    // return 1;
  }
}

pub trait QIODevice_write {
  fn write(self, rsthis: &mut QIODevice) -> i64;
}

// proto:  long long QIODevice::write(const QByteArray & data);
impl<'a> /*trait*/ QIODevice_write for (&'a  QByteArray) {
  fn write(self, rsthis: &mut QIODevice) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice5writeERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QIODevice5writeERK10QByteArray(rsthis.qclsinst, arg0)};
    return ret as i64;
    // return 1;
  }
}

// proto:  long long QIODevice::write(const char * data);
impl<'a> /*trait*/ QIODevice_write for (&'a  String) {
  fn write(self, rsthis: &mut QIODevice) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice5writeEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    let mut ret = unsafe {_ZN9QIODevice5writeEPKc(rsthis.qclsinst, arg0)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn isReadable<T: QIODevice_isReadable>(&mut self, value: T) -> i8 {
    return value.isReadable(self);
    // return 1;
  }
}

pub trait QIODevice_isReadable {
  fn isReadable(self, rsthis: &mut QIODevice) -> i8;
}

// proto:  bool QIODevice::isReadable();
impl<'a> /*trait*/ QIODevice_isReadable for () {
  fn isReadable(self, rsthis: &mut QIODevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QIODevice10isReadableEv()};
    let mut ret = unsafe {_ZNK9QIODevice10isReadableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn readyRead<T: QIODevice_readyRead>(&mut self, value: T)  {
     value.readyRead(self);
    // return 1;
  }
}

pub trait QIODevice_readyRead {
  fn readyRead(self, rsthis: &mut QIODevice) ;
}

// proto:  void QIODevice::readyRead();
impl<'a> /*trait*/ QIODevice_readyRead for () {
  fn readyRead(self, rsthis: &mut QIODevice)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice9readyReadEv()};
     unsafe {_ZN9QIODevice9readyReadEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn waitForReadyRead<T: QIODevice_waitForReadyRead>(&mut self, value: T) -> i8 {
    return value.waitForReadyRead(self);
    // return 1;
  }
}

pub trait QIODevice_waitForReadyRead {
  fn waitForReadyRead(self, rsthis: &mut QIODevice) -> i8;
}

// proto:  bool QIODevice::waitForReadyRead(int msecs);
impl<'a> /*trait*/ QIODevice_waitForReadyRead for (i32) {
  fn waitForReadyRead(self, rsthis: &mut QIODevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice16waitForReadyReadEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN9QIODevice16waitForReadyReadEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn aboutToClose<T: QIODevice_aboutToClose>(&mut self, value: T)  {
     value.aboutToClose(self);
    // return 1;
  }
}

pub trait QIODevice_aboutToClose {
  fn aboutToClose(self, rsthis: &mut QIODevice) ;
}

// proto:  void QIODevice::aboutToClose();
impl<'a> /*trait*/ QIODevice_aboutToClose for () {
  fn aboutToClose(self, rsthis: &mut QIODevice)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice12aboutToCloseEv()};
     unsafe {_ZN9QIODevice12aboutToCloseEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn size<T: QIODevice_size>(&mut self, value: T) -> i64 {
    return value.size(self);
    // return 1;
  }
}

pub trait QIODevice_size {
  fn size(self, rsthis: &mut QIODevice) -> i64;
}

// proto:  long long QIODevice::size();
impl<'a> /*trait*/ QIODevice_size for () {
  fn size(self, rsthis: &mut QIODevice) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QIODevice4sizeEv()};
    let mut ret = unsafe {_ZNK9QIODevice4sizeEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn getChar<T: QIODevice_getChar>(&mut self, value: T) -> i8 {
    return value.getChar(self);
    // return 1;
  }
}

pub trait QIODevice_getChar {
  fn getChar(self, rsthis: &mut QIODevice) -> i8;
}

// proto:  bool QIODevice::getChar(char * c);
impl<'a> /*trait*/ QIODevice_getChar for (&'a mut String) {
  fn getChar(self, rsthis: &mut QIODevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice7getCharEPc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN9QIODevice7getCharEPc(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn putChar<T: QIODevice_putChar>(&mut self, value: T) -> i8 {
    return value.putChar(self);
    // return 1;
  }
}

pub trait QIODevice_putChar {
  fn putChar(self, rsthis: &mut QIODevice) -> i8;
}

// proto:  bool QIODevice::putChar(char c);
impl<'a> /*trait*/ QIODevice_putChar for (i8) {
  fn putChar(self, rsthis: &mut QIODevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice7putCharEc()};
    let arg0 = self  as c_char;
    let mut ret = unsafe {_ZN9QIODevice7putCharEc(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn isTextModeEnabled<T: QIODevice_isTextModeEnabled>(&mut self, value: T) -> i8 {
    return value.isTextModeEnabled(self);
    // return 1;
  }
}

pub trait QIODevice_isTextModeEnabled {
  fn isTextModeEnabled(self, rsthis: &mut QIODevice) -> i8;
}

// proto:  bool QIODevice::isTextModeEnabled();
impl<'a> /*trait*/ QIODevice_isTextModeEnabled for () {
  fn isTextModeEnabled(self, rsthis: &mut QIODevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QIODevice17isTextModeEnabledEv()};
    let mut ret = unsafe {_ZNK9QIODevice17isTextModeEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn isSequential<T: QIODevice_isSequential>(&mut self, value: T) -> i8 {
    return value.isSequential(self);
    // return 1;
  }
}

pub trait QIODevice_isSequential {
  fn isSequential(self, rsthis: &mut QIODevice) -> i8;
}

// proto:  bool QIODevice::isSequential();
impl<'a> /*trait*/ QIODevice_isSequential for () {
  fn isSequential(self, rsthis: &mut QIODevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QIODevice12isSequentialEv()};
    let mut ret = unsafe {_ZNK9QIODevice12isSequentialEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn bytesAvailable<T: QIODevice_bytesAvailable>(&mut self, value: T) -> i64 {
    return value.bytesAvailable(self);
    // return 1;
  }
}

pub trait QIODevice_bytesAvailable {
  fn bytesAvailable(self, rsthis: &mut QIODevice) -> i64;
}

// proto:  long long QIODevice::bytesAvailable();
impl<'a> /*trait*/ QIODevice_bytesAvailable for () {
  fn bytesAvailable(self, rsthis: &mut QIODevice) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QIODevice14bytesAvailableEv()};
    let mut ret = unsafe {_ZNK9QIODevice14bytesAvailableEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

// proto:  long long QIODevice::write(const char * data, qint64 len);
impl<'a> /*trait*/ QIODevice_write for (&'a  String, i64) {
  fn write(self, rsthis: &mut QIODevice) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice5writeEPKcx()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_longlong;
    let mut ret = unsafe {_ZN9QIODevice5writeEPKcx(rsthis.qclsinst, arg0, arg1)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn close<T: QIODevice_close>(&mut self, value: T)  {
     value.close(self);
    // return 1;
  }
}

pub trait QIODevice_close {
  fn close(self, rsthis: &mut QIODevice) ;
}

// proto:  void QIODevice::close();
impl<'a> /*trait*/ QIODevice_close for () {
  fn close(self, rsthis: &mut QIODevice)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice5closeEv()};
     unsafe {_ZN9QIODevice5closeEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn readAll<T: QIODevice_readAll>(&mut self, value: T) -> QByteArray {
    return value.readAll(self);
    // return 1;
  }
}

pub trait QIODevice_readAll {
  fn readAll(self, rsthis: &mut QIODevice) -> QByteArray;
}

// proto:  QByteArray QIODevice::readAll();
impl<'a> /*trait*/ QIODevice_readAll for () {
  fn readAll(self, rsthis: &mut QIODevice) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice7readAllEv()};
    let mut ret = unsafe {_ZN9QIODevice7readAllEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn atEnd<T: QIODevice_atEnd>(&mut self, value: T) -> i8 {
    return value.atEnd(self);
    // return 1;
  }
}

pub trait QIODevice_atEnd {
  fn atEnd(self, rsthis: &mut QIODevice) -> i8;
}

// proto:  bool QIODevice::atEnd();
impl<'a> /*trait*/ QIODevice_atEnd for () {
  fn atEnd(self, rsthis: &mut QIODevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QIODevice5atEndEv()};
    let mut ret = unsafe {_ZNK9QIODevice5atEndEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn seek<T: QIODevice_seek>(&mut self, value: T) -> i8 {
    return value.seek(self);
    // return 1;
  }
}

pub trait QIODevice_seek {
  fn seek(self, rsthis: &mut QIODevice) -> i8;
}

// proto:  bool QIODevice::seek(qint64 pos);
impl<'a> /*trait*/ QIODevice_seek for (i64) {
  fn seek(self, rsthis: &mut QIODevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice4seekEx()};
    let arg0 = self  as c_longlong;
    let mut ret = unsafe {_ZN9QIODevice4seekEx(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn NewQIODevice<T: QIODevice_NewQIODevice>(value: T) -> QIODevice {
    let rsthis = value.NewQIODevice();
    return rsthis;
    // return 1;
  }
}

pub trait QIODevice_NewQIODevice {
  fn NewQIODevice(self) -> QIODevice;
}

// proto: void QIODevice::NewQIODevice(const QIODevice & );
impl<'a> /*trait*/ QIODevice_NewQIODevice for (&'a  QIODevice) {
  fn NewQIODevice(self) -> QIODevice {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODeviceC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QIODeviceC1ERKS_(qthis, arg0)};
    let rsthis = QIODevice{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn pos<T: QIODevice_pos>(&mut self, value: T) -> i64 {
    return value.pos(self);
    // return 1;
  }
}

pub trait QIODevice_pos {
  fn pos(self, rsthis: &mut QIODevice) -> i64;
}

// proto:  long long QIODevice::pos();
impl<'a> /*trait*/ QIODevice_pos for () {
  fn pos(self, rsthis: &mut QIODevice) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QIODevice3posEv()};
    let mut ret = unsafe {_ZNK9QIODevice3posEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn readChannelFinished<T: QIODevice_readChannelFinished>(&mut self, value: T)  {
     value.readChannelFinished(self);
    // return 1;
  }
}

pub trait QIODevice_readChannelFinished {
  fn readChannelFinished(self, rsthis: &mut QIODevice) ;
}

// proto:  void QIODevice::readChannelFinished();
impl<'a> /*trait*/ QIODevice_readChannelFinished for () {
  fn readChannelFinished(self, rsthis: &mut QIODevice)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice19readChannelFinishedEv()};
     unsafe {_ZN9QIODevice19readChannelFinishedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn bytesWritten<T: QIODevice_bytesWritten>(&mut self, value: T)  {
     value.bytesWritten(self);
    // return 1;
  }
}

pub trait QIODevice_bytesWritten {
  fn bytesWritten(self, rsthis: &mut QIODevice) ;
}

// proto:  void QIODevice::bytesWritten(qint64 bytes);
impl<'a> /*trait*/ QIODevice_bytesWritten for (i64) {
  fn bytesWritten(self, rsthis: &mut QIODevice)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice12bytesWrittenEx()};
    let arg0 = self  as c_longlong;
     unsafe {_ZN9QIODevice12bytesWrittenEx(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn waitForBytesWritten<T: QIODevice_waitForBytesWritten>(&mut self, value: T) -> i8 {
    return value.waitForBytesWritten(self);
    // return 1;
  }
}

pub trait QIODevice_waitForBytesWritten {
  fn waitForBytesWritten(self, rsthis: &mut QIODevice) -> i8;
}

// proto:  bool QIODevice::waitForBytesWritten(int msecs);
impl<'a> /*trait*/ QIODevice_waitForBytesWritten for (i32) {
  fn waitForBytesWritten(self, rsthis: &mut QIODevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice19waitForBytesWrittenEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN9QIODevice19waitForBytesWrittenEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn bytesToWrite<T: QIODevice_bytesToWrite>(&mut self, value: T) -> i64 {
    return value.bytesToWrite(self);
    // return 1;
  }
}

pub trait QIODevice_bytesToWrite {
  fn bytesToWrite(self, rsthis: &mut QIODevice) -> i64;
}

// proto:  long long QIODevice::bytesToWrite();
impl<'a> /*trait*/ QIODevice_bytesToWrite for () {
  fn bytesToWrite(self, rsthis: &mut QIODevice) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QIODevice12bytesToWriteEv()};
    let mut ret = unsafe {_ZNK9QIODevice12bytesToWriteEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn reset<T: QIODevice_reset>(&mut self, value: T) -> i8 {
    return value.reset(self);
    // return 1;
  }
}

pub trait QIODevice_reset {
  fn reset(self, rsthis: &mut QIODevice) -> i8;
}

// proto:  bool QIODevice::reset();
impl<'a> /*trait*/ QIODevice_reset for () {
  fn reset(self, rsthis: &mut QIODevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice5resetEv()};
    let mut ret = unsafe {_ZN9QIODevice5resetEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn isWritable<T: QIODevice_isWritable>(&mut self, value: T) -> i8 {
    return value.isWritable(self);
    // return 1;
  }
}

pub trait QIODevice_isWritable {
  fn isWritable(self, rsthis: &mut QIODevice) -> i8;
}

// proto:  bool QIODevice::isWritable();
impl<'a> /*trait*/ QIODevice_isWritable for () {
  fn isWritable(self, rsthis: &mut QIODevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QIODevice10isWritableEv()};
    let mut ret = unsafe {_ZNK9QIODevice10isWritableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QIODevice::NewQIODevice(QObject * parent);
impl<'a> /*trait*/ QIODevice_NewQIODevice for (&'a mut QObject) {
  fn NewQIODevice(self) -> QIODevice {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODeviceC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QIODeviceC1EP7QObject(qthis, arg0)};
    let rsthis = QIODevice{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn metaObject<T: QIODevice_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QIODevice_metaObject {
  fn metaObject(self, rsthis: &mut QIODevice) ;
}

// proto:  const QMetaObject * QIODevice::metaObject();
impl<'a> /*trait*/ QIODevice_metaObject for () {
  fn metaObject(self, rsthis: &mut QIODevice)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QIODevice10metaObjectEv()};
     unsafe {_ZNK9QIODevice10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn setTextModeEnabled<T: QIODevice_setTextModeEnabled>(&mut self, value: T)  {
     value.setTextModeEnabled(self);
    // return 1;
  }
}

pub trait QIODevice_setTextModeEnabled {
  fn setTextModeEnabled(self, rsthis: &mut QIODevice) ;
}

// proto:  void QIODevice::setTextModeEnabled(bool enabled);
impl<'a> /*trait*/ QIODevice_setTextModeEnabled for (i8) {
  fn setTextModeEnabled(self, rsthis: &mut QIODevice)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice18setTextModeEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QIODevice18setTextModeEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QIODevice::NewQIODevice();
impl<'a> /*trait*/ QIODevice_NewQIODevice for () {
  fn NewQIODevice(self) -> QIODevice {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODeviceC1Ev()};
    unsafe {_ZN9QIODeviceC1Ev(qthis)};
    let rsthis = QIODevice{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn isOpen<T: QIODevice_isOpen>(&mut self, value: T) -> i8 {
    return value.isOpen(self);
    // return 1;
  }
}

pub trait QIODevice_isOpen {
  fn isOpen(self, rsthis: &mut QIODevice) -> i8;
}

// proto:  bool QIODevice::isOpen();
impl<'a> /*trait*/ QIODevice_isOpen for () {
  fn isOpen(self, rsthis: &mut QIODevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QIODevice6isOpenEv()};
    let mut ret = unsafe {_ZNK9QIODevice6isOpenEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn canReadLine<T: QIODevice_canReadLine>(&mut self, value: T) -> i8 {
    return value.canReadLine(self);
    // return 1;
  }
}

pub trait QIODevice_canReadLine {
  fn canReadLine(self, rsthis: &mut QIODevice) -> i8;
}

// proto:  bool QIODevice::canReadLine();
impl<'a> /*trait*/ QIODevice_canReadLine for () {
  fn canReadLine(self, rsthis: &mut QIODevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QIODevice11canReadLineEv()};
    let mut ret = unsafe {_ZNK9QIODevice11canReadLineEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn FreeQIODevice<T: QIODevice_FreeQIODevice>(&mut self, value: T)  {
     value.FreeQIODevice(self);
    // return 1;
  }
}

pub trait QIODevice_FreeQIODevice {
  fn FreeQIODevice(self, rsthis: &mut QIODevice) ;
}

// proto:  void QIODevice::FreeQIODevice();
impl<'a> /*trait*/ QIODevice_FreeQIODevice for () {
  fn FreeQIODevice(self, rsthis: &mut QIODevice)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODeviceD0Ev()};
     unsafe {_ZN9QIODeviceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

