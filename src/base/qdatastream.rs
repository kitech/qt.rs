// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qiodevice::QIODevice;
use super::qbytearray::QByteArray;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN11QDataStream9readBytesERPcRj(arg0: *mut c_char, arg1: *mut c_uint) -> i32;
  fn _ZN11QDataStream11unsetDeviceEv() -> i32;
  fn _ZN11QDataStreamC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN11QDataStreamC1EP9QIODevice(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZN11QDataStreamD0Ev() -> i32;
  fn _ZN11QDataStream11skipRawDataEi(arg0: c_int) -> i32;
  fn _ZN11QDataStream10writeBytesEPKcj(arg0: *const c_char, arg1: c_uint) -> i32;
  fn _ZN11QDataStreamC1Ev(qthis: *mut c_void) -> i32;
  fn _ZN11QDataStream11resetStatusEv() -> i32;
  fn _ZN11QDataStreamC1ERK10QByteArray(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK11QDataStream7versionEv() -> i32;
  fn _ZNK11QDataStream5atEndEv() -> i32;
  fn _ZN11QDataStream10setVersionEi(arg0: c_int) -> i32;
  fn _ZN11QDataStream9setDeviceEP9QIODevice(arg0: *mut c_void) -> i32;
  fn _ZN11QDataStream12writeRawDataEPKci(arg0: *const c_char, arg1: c_int) -> i32;
  fn _ZN11QDataStream11readRawDataEPci(arg0: *mut c_char, arg1: c_int) -> i32;
  fn _ZNK11QDataStream6deviceEv() -> i32;
}

// body block begin
// class sizeof(QDataStream)=1
pub struct QDataStream {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDataStream {
  pub fn readBytes<T: QDataStream_readBytes>(&mut self, value: T) -> i32 {
    value.readBytes(self);
    return 1;
  }
}

pub trait QDataStream_readBytes {
  fn readBytes(self, this: &mut QDataStream) -> i32;
}

// proto: QDataStream & QDataStream::readBytes(char *& , uint & len);
impl<'a> /*trait*/ QDataStream_readBytes for (&'a mut String, &'a mut u32) {
  fn readBytes(self, this: &mut QDataStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDataStream9readBytesERPcRj()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as *mut c_uint;
    unsafe {_ZN11QDataStream9readBytesERPcRj(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QDataStream {
  pub fn unsetDevice<T: QDataStream_unsetDevice>(&mut self, value: T) -> i32 {
    value.unsetDevice(self);
    return 1;
  }
}

pub trait QDataStream_unsetDevice {
  fn unsetDevice(self, this: &mut QDataStream) -> i32;
}

// proto: void QDataStream::unsetDevice();
impl<'a> /*trait*/ QDataStream_unsetDevice for () {
  fn unsetDevice(self, this: &mut QDataStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDataStream11unsetDeviceEv()};
    unsafe {_ZN11QDataStream11unsetDeviceEv()};
    return 1;
  }
}

impl /*struct*/ QDataStream {
  pub fn NewQDataStream<T: QDataStream_NewQDataStream>(value: T) -> QDataStream {
    let rsthis = value.NewQDataStream();
    return rsthis;
    // return 1;
  }
}

pub trait QDataStream_NewQDataStream {
  fn NewQDataStream(self) -> QDataStream;
}

// proto: void QDataStream::NewQDataStream(const QDataStream & );
impl<'a> /*trait*/ QDataStream_NewQDataStream for (&'a  QDataStream) {
  fn NewQDataStream(self) -> QDataStream {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDataStreamC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QDataStreamC1ERKS_(qthis, arg0)};
    let rsthis = QDataStream{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QDataStream::NewQDataStream(QIODevice * );
impl<'a> /*trait*/ QDataStream_NewQDataStream for (&'a mut QIODevice) {
  fn NewQDataStream(self) -> QDataStream {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDataStreamC1EP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QDataStreamC1EP9QIODevice(qthis, arg0)};
    let rsthis = QDataStream{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDataStream {
  pub fn FreeQDataStream<T: QDataStream_FreeQDataStream>(&mut self, value: T) -> i32 {
    value.FreeQDataStream(self);
    return 1;
  }
}

pub trait QDataStream_FreeQDataStream {
  fn FreeQDataStream(self, this: &mut QDataStream) -> i32;
}

// proto: void QDataStream::FreeQDataStream();
impl<'a> /*trait*/ QDataStream_FreeQDataStream for () {
  fn FreeQDataStream(self, this: &mut QDataStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDataStreamD0Ev()};
    unsafe {_ZN11QDataStreamD0Ev()};
    return 1;
  }
}

impl /*struct*/ QDataStream {
  pub fn skipRawData<T: QDataStream_skipRawData>(&mut self, value: T) -> i32 {
    value.skipRawData(self);
    return 1;
  }
}

pub trait QDataStream_skipRawData {
  fn skipRawData(self, this: &mut QDataStream) -> i32;
}

// proto: int QDataStream::skipRawData(int len);
impl<'a> /*trait*/ QDataStream_skipRawData for (i32) {
  fn skipRawData(self, this: &mut QDataStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDataStream11skipRawDataEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QDataStream11skipRawDataEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QDataStream {
  pub fn writeBytes<T: QDataStream_writeBytes>(&mut self, value: T) -> i32 {
    value.writeBytes(self);
    return 1;
  }
}

pub trait QDataStream_writeBytes {
  fn writeBytes(self, this: &mut QDataStream) -> i32;
}

// proto: QDataStream & QDataStream::writeBytes(const char * , uint len);
impl<'a> /*trait*/ QDataStream_writeBytes for (&'a  String, u32) {
  fn writeBytes(self, this: &mut QDataStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDataStream10writeBytesEPKcj()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_uint;
    unsafe {_ZN11QDataStream10writeBytesEPKcj(arg0, arg1)};
    return 1;
  }
}

// proto: void QDataStream::NewQDataStream();
impl<'a> /*trait*/ QDataStream_NewQDataStream for () {
  fn NewQDataStream(self) -> QDataStream {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDataStreamC1Ev()};
    unsafe {_ZN11QDataStreamC1Ev(qthis)};
    let rsthis = QDataStream{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDataStream {
  pub fn resetStatus<T: QDataStream_resetStatus>(&mut self, value: T) -> i32 {
    value.resetStatus(self);
    return 1;
  }
}

pub trait QDataStream_resetStatus {
  fn resetStatus(self, this: &mut QDataStream) -> i32;
}

// proto: void QDataStream::resetStatus();
impl<'a> /*trait*/ QDataStream_resetStatus for () {
  fn resetStatus(self, this: &mut QDataStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDataStream11resetStatusEv()};
    unsafe {_ZN11QDataStream11resetStatusEv()};
    return 1;
  }
}

// proto: void QDataStream::NewQDataStream(const QByteArray & );
impl<'a> /*trait*/ QDataStream_NewQDataStream for (&'a  QByteArray) {
  fn NewQDataStream(self) -> QDataStream {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDataStreamC1ERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QDataStreamC1ERK10QByteArray(qthis, arg0)};
    let rsthis = QDataStream{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDataStream {
  pub fn version<T: QDataStream_version>(&mut self, value: T) -> i32 {
    value.version(self);
    return 1;
  }
}

pub trait QDataStream_version {
  fn version(self, this: &mut QDataStream) -> i32;
}

// proto: int QDataStream::version();
impl<'a> /*trait*/ QDataStream_version for () {
  fn version(self, this: &mut QDataStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QDataStream7versionEv()};
    unsafe {_ZNK11QDataStream7versionEv()};
    return 1;
  }
}

impl /*struct*/ QDataStream {
  pub fn atEnd<T: QDataStream_atEnd>(&mut self, value: T) -> i32 {
    value.atEnd(self);
    return 1;
  }
}

pub trait QDataStream_atEnd {
  fn atEnd(self, this: &mut QDataStream) -> i32;
}

// proto: bool QDataStream::atEnd();
impl<'a> /*trait*/ QDataStream_atEnd for () {
  fn atEnd(self, this: &mut QDataStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QDataStream5atEndEv()};
    unsafe {_ZNK11QDataStream5atEndEv()};
    return 1;
  }
}

impl /*struct*/ QDataStream {
  pub fn setVersion<T: QDataStream_setVersion>(&mut self, value: T) -> i32 {
    value.setVersion(self);
    return 1;
  }
}

pub trait QDataStream_setVersion {
  fn setVersion(self, this: &mut QDataStream) -> i32;
}

// proto: void QDataStream::setVersion(int );
impl<'a> /*trait*/ QDataStream_setVersion for (i32) {
  fn setVersion(self, this: &mut QDataStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDataStream10setVersionEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QDataStream10setVersionEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QDataStream {
  pub fn setDevice<T: QDataStream_setDevice>(&mut self, value: T) -> i32 {
    value.setDevice(self);
    return 1;
  }
}

pub trait QDataStream_setDevice {
  fn setDevice(self, this: &mut QDataStream) -> i32;
}

// proto: void QDataStream::setDevice(QIODevice * );
impl<'a> /*trait*/ QDataStream_setDevice for (&'a mut QIODevice) {
  fn setDevice(self, this: &mut QDataStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDataStream9setDeviceEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QDataStream9setDeviceEP9QIODevice(arg0)};
    return 1;
  }
}

impl /*struct*/ QDataStream {
  pub fn writeRawData<T: QDataStream_writeRawData>(&mut self, value: T) -> i32 {
    value.writeRawData(self);
    return 1;
  }
}

pub trait QDataStream_writeRawData {
  fn writeRawData(self, this: &mut QDataStream) -> i32;
}

// proto: int QDataStream::writeRawData(const char * , int len);
impl<'a> /*trait*/ QDataStream_writeRawData for (&'a  String, i32) {
  fn writeRawData(self, this: &mut QDataStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDataStream12writeRawDataEPKci()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_int;
    unsafe {_ZN11QDataStream12writeRawDataEPKci(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QDataStream {
  pub fn readRawData<T: QDataStream_readRawData>(&mut self, value: T) -> i32 {
    value.readRawData(self);
    return 1;
  }
}

pub trait QDataStream_readRawData {
  fn readRawData(self, this: &mut QDataStream) -> i32;
}

// proto: int QDataStream::readRawData(char * , int len);
impl<'a> /*trait*/ QDataStream_readRawData for (&'a mut String, i32) {
  fn readRawData(self, this: &mut QDataStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDataStream11readRawDataEPci()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    unsafe {_ZN11QDataStream11readRawDataEPci(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QDataStream {
  pub fn device<T: QDataStream_device>(&mut self, value: T) -> i32 {
    value.device(self);
    return 1;
  }
}

pub trait QDataStream_device {
  fn device(self, this: &mut QDataStream) -> i32;
}

// proto: QIODevice * QDataStream::device();
impl<'a> /*trait*/ QDataStream_device for () {
  fn device(self, this: &mut QDataStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QDataStream6deviceEv()};
    unsafe {_ZNK11QDataStream6deviceEv()};
    return 1;
  }
}

