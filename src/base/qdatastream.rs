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
  // proto:  QDataStream & QDataStream::readBytes(char *& , uint & len);
  fn _ZN11QDataStream9readBytesERPcRj(qthis: *mut c_void, arg0: *mut c_char, arg1: *mut c_uint) -> *mut c_void;
  // proto:  void QDataStream::unsetDevice();
  fn _ZN11QDataStream11unsetDeviceEv(qthis: *mut c_void) ;
  // proto:  void QDataStream::NewQDataStream(const QDataStream & );
  fn _ZN11QDataStreamC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QDataStream::NewQDataStream(QIODevice * );
  fn _ZN11QDataStreamC1EP9QIODevice(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QDataStream::FreeQDataStream();
  fn _ZN11QDataStreamD0Ev(qthis: *mut c_void) ;
  // proto:  int QDataStream::skipRawData(int len);
  fn _ZN11QDataStream11skipRawDataEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  QDataStream & QDataStream::writeBytes(const char * , uint len);
  fn _ZN11QDataStream10writeBytesEPKcj(qthis: *mut c_void, arg0: *const c_char, arg1: c_uint) -> *mut c_void;
  // proto:  void QDataStream::NewQDataStream();
  fn _ZN11QDataStreamC1Ev(qthis: *mut c_void) ;
  // proto:  void QDataStream::resetStatus();
  fn _ZN11QDataStream11resetStatusEv(qthis: *mut c_void) ;
  // proto:  void QDataStream::NewQDataStream(const QByteArray & );
  fn _ZN11QDataStreamC1ERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QDataStream::version();
  fn _ZNK11QDataStream7versionEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QDataStream::atEnd();
  fn _ZNK11QDataStream5atEndEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QDataStream::setVersion(int );
  fn _ZN11QDataStream10setVersionEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QDataStream::setDevice(QIODevice * );
  fn _ZN11QDataStream9setDeviceEP9QIODevice(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QDataStream::writeRawData(const char * , int len);
  fn _ZN11QDataStream12writeRawDataEPKci(qthis: *mut c_void, arg0: *const c_char, arg1: c_int) -> c_int;
  // proto:  int QDataStream::readRawData(char * , int len);
  fn _ZN11QDataStream11readRawDataEPci(qthis: *mut c_void, arg0: *mut c_char, arg1: c_int) -> c_int;
  // proto:  QIODevice * QDataStream::device();
  fn _ZNK11QDataStream6deviceEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QDataStream)=1
pub struct QDataStream {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDataStream {
  pub fn readBytes<RetType, T: QDataStream_readBytes<RetType>>(&mut self, value: T) -> RetType {
    return value.readBytes(self);
    // return 1;
  }
}

pub trait QDataStream_readBytes<RetType> {
  fn readBytes(self, rsthis: &mut QDataStream) -> RetType;
}

// proto:  QDataStream & QDataStream::readBytes(char *& , uint & len);
impl<'a> /*trait*/ QDataStream_readBytes<QDataStream> for (&'a mut String, &'a mut u32) {
  fn readBytes(self, rsthis: &mut QDataStream) -> QDataStream {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDataStream9readBytesERPcRj()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as *mut c_uint;
    let mut ret = unsafe {_ZN11QDataStream9readBytesERPcRj(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QDataStream{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDataStream {
  pub fn unsetDevice<RetType, T: QDataStream_unsetDevice<RetType>>(&mut self, value: T) -> RetType {
    return value.unsetDevice(self);
    // return 1;
  }
}

pub trait QDataStream_unsetDevice<RetType> {
  fn unsetDevice(self, rsthis: &mut QDataStream) -> RetType;
}

// proto:  void QDataStream::unsetDevice();
impl<'a> /*trait*/ QDataStream_unsetDevice<()> for () {
  fn unsetDevice(self, rsthis: &mut QDataStream) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDataStream11unsetDeviceEv()};
     unsafe {_ZN11QDataStream11unsetDeviceEv(rsthis.qclsinst)};
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
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
  pub fn FreeQDataStream<RetType, T: QDataStream_FreeQDataStream<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQDataStream(self);
    // return 1;
  }
}

pub trait QDataStream_FreeQDataStream<RetType> {
  fn FreeQDataStream(self, rsthis: &mut QDataStream) -> RetType;
}

// proto:  void QDataStream::FreeQDataStream();
impl<'a> /*trait*/ QDataStream_FreeQDataStream<()> for () {
  fn FreeQDataStream(self, rsthis: &mut QDataStream) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDataStreamD0Ev()};
     unsafe {_ZN11QDataStreamD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDataStream {
  pub fn skipRawData<RetType, T: QDataStream_skipRawData<RetType>>(&mut self, value: T) -> RetType {
    return value.skipRawData(self);
    // return 1;
  }
}

pub trait QDataStream_skipRawData<RetType> {
  fn skipRawData(self, rsthis: &mut QDataStream) -> RetType;
}

// proto:  int QDataStream::skipRawData(int len);
impl<'a> /*trait*/ QDataStream_skipRawData<i32> for (i32) {
  fn skipRawData(self, rsthis: &mut QDataStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDataStream11skipRawDataEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN11QDataStream11skipRawDataEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QDataStream {
  pub fn writeBytes<RetType, T: QDataStream_writeBytes<RetType>>(&mut self, value: T) -> RetType {
    return value.writeBytes(self);
    // return 1;
  }
}

pub trait QDataStream_writeBytes<RetType> {
  fn writeBytes(self, rsthis: &mut QDataStream) -> RetType;
}

// proto:  QDataStream & QDataStream::writeBytes(const char * , uint len);
impl<'a> /*trait*/ QDataStream_writeBytes<QDataStream> for (&'a  String, u32) {
  fn writeBytes(self, rsthis: &mut QDataStream) -> QDataStream {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDataStream10writeBytesEPKcj()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_uint;
    let mut ret = unsafe {_ZN11QDataStream10writeBytesEPKcj(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QDataStream{qclsinst: ret};
    return ret1;
    // return 1;
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
  pub fn resetStatus<RetType, T: QDataStream_resetStatus<RetType>>(&mut self, value: T) -> RetType {
    return value.resetStatus(self);
    // return 1;
  }
}

pub trait QDataStream_resetStatus<RetType> {
  fn resetStatus(self, rsthis: &mut QDataStream) -> RetType;
}

// proto:  void QDataStream::resetStatus();
impl<'a> /*trait*/ QDataStream_resetStatus<()> for () {
  fn resetStatus(self, rsthis: &mut QDataStream) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDataStream11resetStatusEv()};
     unsafe {_ZN11QDataStream11resetStatusEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QDataStream::NewQDataStream(const QByteArray & );
impl<'a> /*trait*/ QDataStream_NewQDataStream for (&'a  QByteArray) {
  fn NewQDataStream(self) -> QDataStream {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDataStreamC1ERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QDataStreamC1ERK10QByteArray(qthis, arg0)};
    let rsthis = QDataStream{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDataStream {
  pub fn version<RetType, T: QDataStream_version<RetType>>(&mut self, value: T) -> RetType {
    return value.version(self);
    // return 1;
  }
}

pub trait QDataStream_version<RetType> {
  fn version(self, rsthis: &mut QDataStream) -> RetType;
}

// proto:  int QDataStream::version();
impl<'a> /*trait*/ QDataStream_version<i32> for () {
  fn version(self, rsthis: &mut QDataStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QDataStream7versionEv()};
    let mut ret = unsafe {_ZNK11QDataStream7versionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QDataStream {
  pub fn atEnd<RetType, T: QDataStream_atEnd<RetType>>(&mut self, value: T) -> RetType {
    return value.atEnd(self);
    // return 1;
  }
}

pub trait QDataStream_atEnd<RetType> {
  fn atEnd(self, rsthis: &mut QDataStream) -> RetType;
}

// proto:  bool QDataStream::atEnd();
impl<'a> /*trait*/ QDataStream_atEnd<i8> for () {
  fn atEnd(self, rsthis: &mut QDataStream) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QDataStream5atEndEv()};
    let mut ret = unsafe {_ZNK11QDataStream5atEndEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDataStream {
  pub fn setVersion<RetType, T: QDataStream_setVersion<RetType>>(&mut self, value: T) -> RetType {
    return value.setVersion(self);
    // return 1;
  }
}

pub trait QDataStream_setVersion<RetType> {
  fn setVersion(self, rsthis: &mut QDataStream) -> RetType;
}

// proto:  void QDataStream::setVersion(int );
impl<'a> /*trait*/ QDataStream_setVersion<()> for (i32) {
  fn setVersion(self, rsthis: &mut QDataStream) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDataStream10setVersionEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QDataStream10setVersionEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDataStream {
  pub fn setDevice<RetType, T: QDataStream_setDevice<RetType>>(&mut self, value: T) -> RetType {
    return value.setDevice(self);
    // return 1;
  }
}

pub trait QDataStream_setDevice<RetType> {
  fn setDevice(self, rsthis: &mut QDataStream) -> RetType;
}

// proto:  void QDataStream::setDevice(QIODevice * );
impl<'a> /*trait*/ QDataStream_setDevice<()> for (&'a mut QIODevice) {
  fn setDevice(self, rsthis: &mut QDataStream) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDataStream9setDeviceEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QDataStream9setDeviceEP9QIODevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDataStream {
  pub fn writeRawData<RetType, T: QDataStream_writeRawData<RetType>>(&mut self, value: T) -> RetType {
    return value.writeRawData(self);
    // return 1;
  }
}

pub trait QDataStream_writeRawData<RetType> {
  fn writeRawData(self, rsthis: &mut QDataStream) -> RetType;
}

// proto:  int QDataStream::writeRawData(const char * , int len);
impl<'a> /*trait*/ QDataStream_writeRawData<i32> for (&'a  String, i32) {
  fn writeRawData(self, rsthis: &mut QDataStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDataStream12writeRawDataEPKci()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN11QDataStream12writeRawDataEPKci(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QDataStream {
  pub fn readRawData<RetType, T: QDataStream_readRawData<RetType>>(&mut self, value: T) -> RetType {
    return value.readRawData(self);
    // return 1;
  }
}

pub trait QDataStream_readRawData<RetType> {
  fn readRawData(self, rsthis: &mut QDataStream) -> RetType;
}

// proto:  int QDataStream::readRawData(char * , int len);
impl<'a> /*trait*/ QDataStream_readRawData<i32> for (&'a mut String, i32) {
  fn readRawData(self, rsthis: &mut QDataStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDataStream11readRawDataEPci()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN11QDataStream11readRawDataEPci(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QDataStream {
  pub fn device<RetType, T: QDataStream_device<RetType>>(&mut self, value: T) -> RetType {
    return value.device(self);
    // return 1;
  }
}

pub trait QDataStream_device<RetType> {
  fn device(self, rsthis: &mut QDataStream) -> RetType;
}

// proto:  QIODevice * QDataStream::device();
impl<'a> /*trait*/ QDataStream_device<QIODevice> for () {
  fn device(self, rsthis: &mut QDataStream) -> QIODevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QDataStream6deviceEv()};
    let mut ret = unsafe {_ZNK11QDataStream6deviceEv(rsthis.qclsinst)};
    let mut ret1 = QIODevice{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

