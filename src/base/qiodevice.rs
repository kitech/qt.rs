// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qbytearray::QByteArray;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN9QIODevice9ungetCharEc(arg0: c_char) -> i32;
  fn _ZNK9QIODevice11errorStringEv() -> i32;
  fn _ZN9QIODevice5writeERK10QByteArray(arg0: *const c_void) -> i32;
  fn _ZN9QIODevice5writeEPKc(arg0: *const c_char) -> i32;
  fn _ZNK9QIODevice10isReadableEv() -> i32;
  fn _ZN9QIODevice9readyReadEv() -> i32;
  fn _ZN9QIODevice8readLineEPcx(arg0: *mut c_char, arg1: c_longlong) -> i32;
  fn _ZN9QIODevice8readLineEx(arg0: c_longlong) -> i32;
  fn _ZN9QIODevice16waitForReadyReadEi(arg0: c_int) -> i32;
  fn _ZN9QIODevice12aboutToCloseEv() -> i32;
  fn _ZNK9QIODevice4sizeEv() -> i32;
  fn _ZN9QIODevice7getCharEPc(arg0: *mut c_char) -> i32;
  fn _ZN9QIODevice7putCharEc(arg0: c_char) -> i32;
  fn _ZNK9QIODevice17isTextModeEnabledEv() -> i32;
  fn _ZNK9QIODevice12isSequentialEv() -> i32;
  fn _ZNK9QIODevice14bytesAvailableEv() -> i32;
  fn _ZN9QIODevice5writeEPKcx(arg0: *const c_char, arg1: c_longlong) -> i32;
  fn _ZN9QIODevice5closeEv() -> i32;
  fn _ZN9QIODevice7readAllEv() -> i32;
  fn _ZNK9QIODevice5atEndEv() -> i32;
  fn _ZN9QIODevice4seekEx(arg0: c_longlong) -> i32;
  fn _ZN9QIODeviceC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK9QIODevice3posEv() -> i32;
  fn _ZN9QIODevice19readChannelFinishedEv() -> i32;
  fn _ZN9QIODevice4readEx(arg0: c_longlong) -> i32;
  fn _ZN9QIODevice4openE6QFlagsINS_12OpenModeFlagEE(arg0: c_int) -> i32;
  fn _ZN9QIODevice4peekEPcx(arg0: *mut c_char, arg1: c_longlong) -> i32;
  fn _ZN9QIODevice4readEPcx(arg0: *mut c_char, arg1: c_longlong) -> i32;
  fn _ZN9QIODevice12bytesWrittenEx(arg0: c_longlong) -> i32;
  fn _ZN9QIODevice19waitForBytesWrittenEi(arg0: c_int) -> i32;
  fn _ZNK9QIODevice12bytesToWriteEv() -> i32;
  fn _ZN9QIODevice5resetEv() -> i32;
  fn _ZNK9QIODevice10isWritableEv() -> i32;
  fn _ZN9QIODevice4peekEx(arg0: c_longlong) -> i32;
  fn _ZN9QIODeviceC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZNK9QIODevice10metaObjectEv() -> i32;
  fn _ZN9QIODevice18setTextModeEnabledEb(arg0: int8_t) -> i32;
  fn _ZN9QIODeviceC1Ev(qthis: *mut c_void) -> i32;
  fn _ZNK9QIODevice6isOpenEv() -> i32;
  fn _ZNK9QIODevice11canReadLineEv() -> i32;
  fn _ZN9QIODeviceD0Ev() -> i32;
}

// body block begin
// class sizeof(QIODevice)=1
pub struct QIODevice {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QIODevice {
  pub fn ungetChar<T: QIODevice_ungetChar>(&mut self, value: T) -> i32 {
    value.ungetChar(self);
    return 1;
  }
}

pub trait QIODevice_ungetChar {
  fn ungetChar(self, this: &mut QIODevice) -> i32;
}

// proto: void QIODevice::ungetChar(char c);
impl<'a> /*trait*/ QIODevice_ungetChar for (i8) {
  fn ungetChar(self, this: &mut QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice9ungetCharEc()};
    let arg0 = self  as c_char;
    unsafe {_ZN9QIODevice9ungetCharEc(arg0)};
    return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn errorString<T: QIODevice_errorString>(&mut self, value: T) -> i32 {
    value.errorString(self);
    return 1;
  }
}

pub trait QIODevice_errorString {
  fn errorString(self, this: &mut QIODevice) -> i32;
}

// proto: QString QIODevice::errorString();
impl<'a> /*trait*/ QIODevice_errorString for () {
  fn errorString(self, this: &mut QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QIODevice11errorStringEv()};
    unsafe {_ZNK9QIODevice11errorStringEv()};
    return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn write<T: QIODevice_write>(&mut self, value: T) -> i32 {
    value.write(self);
    return 1;
  }
}

pub trait QIODevice_write {
  fn write(self, this: &mut QIODevice) -> i32;
}

// proto: long long QIODevice::write(const QByteArray & data);
impl<'a> /*trait*/ QIODevice_write for (&'a  QByteArray) {
  fn write(self, this: &mut QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice5writeERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QIODevice5writeERK10QByteArray(arg0)};
    return 1;
  }
}

// proto: long long QIODevice::write(const char * data);
impl<'a> /*trait*/ QIODevice_write for (&'a  String) {
  fn write(self, this: &mut QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice5writeEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    unsafe {_ZN9QIODevice5writeEPKc(arg0)};
    return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn isReadable<T: QIODevice_isReadable>(&mut self, value: T) -> i32 {
    value.isReadable(self);
    return 1;
  }
}

pub trait QIODevice_isReadable {
  fn isReadable(self, this: &mut QIODevice) -> i32;
}

// proto: bool QIODevice::isReadable();
impl<'a> /*trait*/ QIODevice_isReadable for () {
  fn isReadable(self, this: &mut QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QIODevice10isReadableEv()};
    unsafe {_ZNK9QIODevice10isReadableEv()};
    return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn readyRead<T: QIODevice_readyRead>(&mut self, value: T) -> i32 {
    value.readyRead(self);
    return 1;
  }
}

pub trait QIODevice_readyRead {
  fn readyRead(self, this: &mut QIODevice) -> i32;
}

// proto: void QIODevice::readyRead();
impl<'a> /*trait*/ QIODevice_readyRead for () {
  fn readyRead(self, this: &mut QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice9readyReadEv()};
    unsafe {_ZN9QIODevice9readyReadEv()};
    return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn readLine<T: QIODevice_readLine>(&mut self, value: T) -> i32 {
    value.readLine(self);
    return 1;
  }
}

pub trait QIODevice_readLine {
  fn readLine(self, this: &mut QIODevice) -> i32;
}

// proto: long long QIODevice::readLine(char * data, qint64 maxlen);
impl<'a> /*trait*/ QIODevice_readLine for (&'a mut String, i64) {
  fn readLine(self, this: &mut QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice8readLineEPcx()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_longlong;
    unsafe {_ZN9QIODevice8readLineEPcx(arg0, arg1)};
    return 1;
  }
}

// proto: QByteArray QIODevice::readLine(qint64 maxlen);
impl<'a> /*trait*/ QIODevice_readLine for (i64) {
  fn readLine(self, this: &mut QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice8readLineEx()};
    let arg0 = self  as c_longlong;
    unsafe {_ZN9QIODevice8readLineEx(arg0)};
    return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn waitForReadyRead<T: QIODevice_waitForReadyRead>(&mut self, value: T) -> i32 {
    value.waitForReadyRead(self);
    return 1;
  }
}

pub trait QIODevice_waitForReadyRead {
  fn waitForReadyRead(self, this: &mut QIODevice) -> i32;
}

// proto: bool QIODevice::waitForReadyRead(int msecs);
impl<'a> /*trait*/ QIODevice_waitForReadyRead for (i32) {
  fn waitForReadyRead(self, this: &mut QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice16waitForReadyReadEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QIODevice16waitForReadyReadEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn aboutToClose<T: QIODevice_aboutToClose>(&mut self, value: T) -> i32 {
    value.aboutToClose(self);
    return 1;
  }
}

pub trait QIODevice_aboutToClose {
  fn aboutToClose(self, this: &mut QIODevice) -> i32;
}

// proto: void QIODevice::aboutToClose();
impl<'a> /*trait*/ QIODevice_aboutToClose for () {
  fn aboutToClose(self, this: &mut QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice12aboutToCloseEv()};
    unsafe {_ZN9QIODevice12aboutToCloseEv()};
    return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn size<T: QIODevice_size>(&mut self, value: T) -> i32 {
    value.size(self);
    return 1;
  }
}

pub trait QIODevice_size {
  fn size(self, this: &mut QIODevice) -> i32;
}

// proto: long long QIODevice::size();
impl<'a> /*trait*/ QIODevice_size for () {
  fn size(self, this: &mut QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QIODevice4sizeEv()};
    unsafe {_ZNK9QIODevice4sizeEv()};
    return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn getChar<T: QIODevice_getChar>(&mut self, value: T) -> i32 {
    value.getChar(self);
    return 1;
  }
}

pub trait QIODevice_getChar {
  fn getChar(self, this: &mut QIODevice) -> i32;
}

// proto: bool QIODevice::getChar(char * c);
impl<'a> /*trait*/ QIODevice_getChar for (&'a mut String) {
  fn getChar(self, this: &mut QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice7getCharEPc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    unsafe {_ZN9QIODevice7getCharEPc(arg0)};
    return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn putChar<T: QIODevice_putChar>(&mut self, value: T) -> i32 {
    value.putChar(self);
    return 1;
  }
}

pub trait QIODevice_putChar {
  fn putChar(self, this: &mut QIODevice) -> i32;
}

// proto: bool QIODevice::putChar(char c);
impl<'a> /*trait*/ QIODevice_putChar for (i8) {
  fn putChar(self, this: &mut QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice7putCharEc()};
    let arg0 = self  as c_char;
    unsafe {_ZN9QIODevice7putCharEc(arg0)};
    return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn isTextModeEnabled<T: QIODevice_isTextModeEnabled>(&mut self, value: T) -> i32 {
    value.isTextModeEnabled(self);
    return 1;
  }
}

pub trait QIODevice_isTextModeEnabled {
  fn isTextModeEnabled(self, this: &mut QIODevice) -> i32;
}

// proto: bool QIODevice::isTextModeEnabled();
impl<'a> /*trait*/ QIODevice_isTextModeEnabled for () {
  fn isTextModeEnabled(self, this: &mut QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QIODevice17isTextModeEnabledEv()};
    unsafe {_ZNK9QIODevice17isTextModeEnabledEv()};
    return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn isSequential<T: QIODevice_isSequential>(&mut self, value: T) -> i32 {
    value.isSequential(self);
    return 1;
  }
}

pub trait QIODevice_isSequential {
  fn isSequential(self, this: &mut QIODevice) -> i32;
}

// proto: bool QIODevice::isSequential();
impl<'a> /*trait*/ QIODevice_isSequential for () {
  fn isSequential(self, this: &mut QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QIODevice12isSequentialEv()};
    unsafe {_ZNK9QIODevice12isSequentialEv()};
    return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn bytesAvailable<T: QIODevice_bytesAvailable>(&mut self, value: T) -> i32 {
    value.bytesAvailable(self);
    return 1;
  }
}

pub trait QIODevice_bytesAvailable {
  fn bytesAvailable(self, this: &mut QIODevice) -> i32;
}

// proto: long long QIODevice::bytesAvailable();
impl<'a> /*trait*/ QIODevice_bytesAvailable for () {
  fn bytesAvailable(self, this: &mut QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QIODevice14bytesAvailableEv()};
    unsafe {_ZNK9QIODevice14bytesAvailableEv()};
    return 1;
  }
}

// proto: long long QIODevice::write(const char * data, qint64 len);
impl<'a> /*trait*/ QIODevice_write for (&'a  String, i64) {
  fn write(self, this: &mut QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice5writeEPKcx()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_longlong;
    unsafe {_ZN9QIODevice5writeEPKcx(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn close<T: QIODevice_close>(&mut self, value: T) -> i32 {
    value.close(self);
    return 1;
  }
}

pub trait QIODevice_close {
  fn close(self, this: &mut QIODevice) -> i32;
}

// proto: void QIODevice::close();
impl<'a> /*trait*/ QIODevice_close for () {
  fn close(self, this: &mut QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice5closeEv()};
    unsafe {_ZN9QIODevice5closeEv()};
    return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn readAll<T: QIODevice_readAll>(&mut self, value: T) -> i32 {
    value.readAll(self);
    return 1;
  }
}

pub trait QIODevice_readAll {
  fn readAll(self, this: &mut QIODevice) -> i32;
}

// proto: QByteArray QIODevice::readAll();
impl<'a> /*trait*/ QIODevice_readAll for () {
  fn readAll(self, this: &mut QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice7readAllEv()};
    unsafe {_ZN9QIODevice7readAllEv()};
    return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn atEnd<T: QIODevice_atEnd>(&mut self, value: T) -> i32 {
    value.atEnd(self);
    return 1;
  }
}

pub trait QIODevice_atEnd {
  fn atEnd(self, this: &mut QIODevice) -> i32;
}

// proto: bool QIODevice::atEnd();
impl<'a> /*trait*/ QIODevice_atEnd for () {
  fn atEnd(self, this: &mut QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QIODevice5atEndEv()};
    unsafe {_ZNK9QIODevice5atEndEv()};
    return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn seek<T: QIODevice_seek>(&mut self, value: T) -> i32 {
    value.seek(self);
    return 1;
  }
}

pub trait QIODevice_seek {
  fn seek(self, this: &mut QIODevice) -> i32;
}

// proto: bool QIODevice::seek(qint64 pos);
impl<'a> /*trait*/ QIODevice_seek for (i64) {
  fn seek(self, this: &mut QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice4seekEx()};
    let arg0 = self  as c_longlong;
    unsafe {_ZN9QIODevice4seekEx(arg0)};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QIODeviceC1ERKS_(qthis, arg0)};
    let rsthis = QIODevice{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn pos<T: QIODevice_pos>(&mut self, value: T) -> i32 {
    value.pos(self);
    return 1;
  }
}

pub trait QIODevice_pos {
  fn pos(self, this: &mut QIODevice) -> i32;
}

// proto: long long QIODevice::pos();
impl<'a> /*trait*/ QIODevice_pos for () {
  fn pos(self, this: &mut QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QIODevice3posEv()};
    unsafe {_ZNK9QIODevice3posEv()};
    return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn readChannelFinished<T: QIODevice_readChannelFinished>(&mut self, value: T) -> i32 {
    value.readChannelFinished(self);
    return 1;
  }
}

pub trait QIODevice_readChannelFinished {
  fn readChannelFinished(self, this: &mut QIODevice) -> i32;
}

// proto: void QIODevice::readChannelFinished();
impl<'a> /*trait*/ QIODevice_readChannelFinished for () {
  fn readChannelFinished(self, this: &mut QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice19readChannelFinishedEv()};
    unsafe {_ZN9QIODevice19readChannelFinishedEv()};
    return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn read<T: QIODevice_read>(&mut self, value: T) -> i32 {
    value.read(self);
    return 1;
  }
}

pub trait QIODevice_read {
  fn read(self, this: &mut QIODevice) -> i32;
}

// proto: QByteArray QIODevice::read(qint64 maxlen);
impl<'a> /*trait*/ QIODevice_read for (i64) {
  fn read(self, this: &mut QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice4readEx()};
    let arg0 = self  as c_longlong;
    unsafe {_ZN9QIODevice4readEx(arg0)};
    return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn open<T: QIODevice_open>(&mut self, value: T) -> i32 {
    value.open(self);
    return 1;
  }
}

pub trait QIODevice_open {
  fn open(self, this: &mut QIODevice) -> i32;
}

// proto: bool QIODevice::open(OpenMode mode);
impl<'a> /*trait*/ QIODevice_open for (i32) {
  fn open(self, this: &mut QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice4openE6QFlagsINS_12OpenModeFlagEE()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QIODevice4openE6QFlagsINS_12OpenModeFlagEE(arg0)};
    return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn peek<T: QIODevice_peek>(&mut self, value: T) -> i32 {
    value.peek(self);
    return 1;
  }
}

pub trait QIODevice_peek {
  fn peek(self, this: &mut QIODevice) -> i32;
}

// proto: long long QIODevice::peek(char * data, qint64 maxlen);
impl<'a> /*trait*/ QIODevice_peek for (&'a mut String, i64) {
  fn peek(self, this: &mut QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice4peekEPcx()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_longlong;
    unsafe {_ZN9QIODevice4peekEPcx(arg0, arg1)};
    return 1;
  }
}

// proto: long long QIODevice::read(char * data, qint64 maxlen);
impl<'a> /*trait*/ QIODevice_read for (&'a mut String, i64) {
  fn read(self, this: &mut QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice4readEPcx()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_longlong;
    unsafe {_ZN9QIODevice4readEPcx(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn bytesWritten<T: QIODevice_bytesWritten>(&mut self, value: T) -> i32 {
    value.bytesWritten(self);
    return 1;
  }
}

pub trait QIODevice_bytesWritten {
  fn bytesWritten(self, this: &mut QIODevice) -> i32;
}

// proto: void QIODevice::bytesWritten(qint64 bytes);
impl<'a> /*trait*/ QIODevice_bytesWritten for (i64) {
  fn bytesWritten(self, this: &mut QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice12bytesWrittenEx()};
    let arg0 = self  as c_longlong;
    unsafe {_ZN9QIODevice12bytesWrittenEx(arg0)};
    return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn waitForBytesWritten<T: QIODevice_waitForBytesWritten>(&mut self, value: T) -> i32 {
    value.waitForBytesWritten(self);
    return 1;
  }
}

pub trait QIODevice_waitForBytesWritten {
  fn waitForBytesWritten(self, this: &mut QIODevice) -> i32;
}

// proto: bool QIODevice::waitForBytesWritten(int msecs);
impl<'a> /*trait*/ QIODevice_waitForBytesWritten for (i32) {
  fn waitForBytesWritten(self, this: &mut QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice19waitForBytesWrittenEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QIODevice19waitForBytesWrittenEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn bytesToWrite<T: QIODevice_bytesToWrite>(&mut self, value: T) -> i32 {
    value.bytesToWrite(self);
    return 1;
  }
}

pub trait QIODevice_bytesToWrite {
  fn bytesToWrite(self, this: &mut QIODevice) -> i32;
}

// proto: long long QIODevice::bytesToWrite();
impl<'a> /*trait*/ QIODevice_bytesToWrite for () {
  fn bytesToWrite(self, this: &mut QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QIODevice12bytesToWriteEv()};
    unsafe {_ZNK9QIODevice12bytesToWriteEv()};
    return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn reset<T: QIODevice_reset>(&mut self, value: T) -> i32 {
    value.reset(self);
    return 1;
  }
}

pub trait QIODevice_reset {
  fn reset(self, this: &mut QIODevice) -> i32;
}

// proto: bool QIODevice::reset();
impl<'a> /*trait*/ QIODevice_reset for () {
  fn reset(self, this: &mut QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice5resetEv()};
    unsafe {_ZN9QIODevice5resetEv()};
    return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn isWritable<T: QIODevice_isWritable>(&mut self, value: T) -> i32 {
    value.isWritable(self);
    return 1;
  }
}

pub trait QIODevice_isWritable {
  fn isWritable(self, this: &mut QIODevice) -> i32;
}

// proto: bool QIODevice::isWritable();
impl<'a> /*trait*/ QIODevice_isWritable for () {
  fn isWritable(self, this: &mut QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QIODevice10isWritableEv()};
    unsafe {_ZNK9QIODevice10isWritableEv()};
    return 1;
  }
}

// proto: QByteArray QIODevice::peek(qint64 maxlen);
impl<'a> /*trait*/ QIODevice_peek for (i64) {
  fn peek(self, this: &mut QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice4peekEx()};
    let arg0 = self  as c_longlong;
    unsafe {_ZN9QIODevice4peekEx(arg0)};
    return 1;
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
  pub fn metaObject<T: QIODevice_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QIODevice_metaObject {
  fn metaObject(self, this: &mut QIODevice) -> i32;
}

// proto: const QMetaObject * QIODevice::metaObject();
impl<'a> /*trait*/ QIODevice_metaObject for () {
  fn metaObject(self, this: &mut QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QIODevice10metaObjectEv()};
    unsafe {_ZNK9QIODevice10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn setTextModeEnabled<T: QIODevice_setTextModeEnabled>(&mut self, value: T) -> i32 {
    value.setTextModeEnabled(self);
    return 1;
  }
}

pub trait QIODevice_setTextModeEnabled {
  fn setTextModeEnabled(self, this: &mut QIODevice) -> i32;
}

// proto: void QIODevice::setTextModeEnabled(bool enabled);
impl<'a> /*trait*/ QIODevice_setTextModeEnabled for (i8) {
  fn setTextModeEnabled(self, this: &mut QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODevice18setTextModeEnabledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QIODevice18setTextModeEnabledEb(arg0)};
    return 1;
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
  pub fn isOpen<T: QIODevice_isOpen>(&mut self, value: T) -> i32 {
    value.isOpen(self);
    return 1;
  }
}

pub trait QIODevice_isOpen {
  fn isOpen(self, this: &mut QIODevice) -> i32;
}

// proto: bool QIODevice::isOpen();
impl<'a> /*trait*/ QIODevice_isOpen for () {
  fn isOpen(self, this: &mut QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QIODevice6isOpenEv()};
    unsafe {_ZNK9QIODevice6isOpenEv()};
    return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn canReadLine<T: QIODevice_canReadLine>(&mut self, value: T) -> i32 {
    value.canReadLine(self);
    return 1;
  }
}

pub trait QIODevice_canReadLine {
  fn canReadLine(self, this: &mut QIODevice) -> i32;
}

// proto: bool QIODevice::canReadLine();
impl<'a> /*trait*/ QIODevice_canReadLine for () {
  fn canReadLine(self, this: &mut QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QIODevice11canReadLineEv()};
    unsafe {_ZNK9QIODevice11canReadLineEv()};
    return 1;
  }
}

impl /*struct*/ QIODevice {
  pub fn FreeQIODevice<T: QIODevice_FreeQIODevice>(&mut self, value: T) -> i32 {
    value.FreeQIODevice(self);
    return 1;
  }
}

pub trait QIODevice_FreeQIODevice {
  fn FreeQIODevice(self, this: &mut QIODevice) -> i32;
}

// proto: void QIODevice::FreeQIODevice();
impl<'a> /*trait*/ QIODevice_FreeQIODevice for () {
  fn FreeQIODevice(self, this: &mut QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QIODeviceD0Ev()};
    unsafe {_ZN9QIODeviceD0Ev()};
    return 1;
  }
}

