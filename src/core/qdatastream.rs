// auto generated, do not modify.
// created: Sat Dec 26 10:16:52 2015
// src-file: /QtCore/qdatastream.h
// dst-file: /src/core/qdatastream.rs
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
use super::qiodevice::QIODevice; // 773
use super::qbytearray::QByteArray; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QDataStream_Class_Size() -> c_int;
  // proto:  QDataStream & QDataStream::readBytes(char *& , uint & len);
  fn _ZN11QDataStream9readBytesERPcRj(qthis: *mut c_void, arg0: *mut c_char, arg1: *mut c_uint) -> *mut c_void;
  // proto:  void QDataStream::unsetDevice();
  fn _ZN11QDataStream11unsetDeviceEv(qthis: *mut c_void);
  // proto:  void QDataStream::QDataStream(const QDataStream & );
  fn dector_ZN11QDataStreamC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN11QDataStreamC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QDataStream::QDataStream(QIODevice * );
  fn dector_ZN11QDataStreamC1EP9QIODevice(arg0: *mut c_void) -> *mut c_void;
  fn _ZN11QDataStreamC1EP9QIODevice(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QDataStream::~QDataStream();
  fn _ZN11QDataStreamD0Ev(qthis: *mut c_void);
  // proto:  int QDataStream::skipRawData(int len);
  fn _ZN11QDataStream11skipRawDataEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  QDataStream & QDataStream::writeBytes(const char * , uint len);
  fn _ZN11QDataStream10writeBytesEPKcj(qthis: *mut c_void, arg0: *mut c_char, arg1: c_uint) -> *mut c_void;
  // proto:  void QDataStream::QDataStream();
  fn dector_ZN11QDataStreamC1Ev() -> *mut c_void;
  fn _ZN11QDataStreamC1Ev(qthis: *mut c_void);
  // proto:  void QDataStream::resetStatus();
  fn _ZN11QDataStream11resetStatusEv(qthis: *mut c_void);
  // proto:  void QDataStream::QDataStream(const QByteArray & );
  fn dector_ZN11QDataStreamC1ERK10QByteArray(arg0: *mut c_void) -> *mut c_void;
  fn _ZN11QDataStreamC1ERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QDataStream::version();
  fn _ZNK11QDataStream7versionEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QDataStream::atEnd();
  fn _ZNK11QDataStream5atEndEv(qthis: *mut c_void) -> c_char;
  // proto:  void QDataStream::setVersion(int );
  fn _ZN11QDataStream10setVersionEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QDataStream::setDevice(QIODevice * );
  fn _ZN11QDataStream9setDeviceEP9QIODevice(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QDataStream::writeRawData(const char * , int len);
  fn _ZN11QDataStream12writeRawDataEPKci(qthis: *mut c_void, arg0: *mut c_char, arg1: c_int) -> c_int;
  // proto:  int QDataStream::readRawData(char * , int len);
  fn _ZN11QDataStream11readRawDataEPci(qthis: *mut c_void, arg0: *mut c_char, arg1: c_int) -> c_int;
  // proto:  QIODevice * QDataStream::device();
  fn _ZNK11QDataStream6deviceEv(qthis: *mut c_void) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QDataStream)=1
pub struct QDataStream {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDataStream {
  pub fn inheritFrom(qthis: *mut c_void) -> QDataStream {
    return QDataStream{qclsinst: qthis};
  }
}
  // proto:  QDataStream & QDataStream::readBytes(char *& , uint & len);
impl /*struct*/ QDataStream {
  pub fn readBytes<RetType, T: QDataStream_readBytes<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.readBytes(self);
    // return 1;
  }
}

pub trait QDataStream_readBytes<RetType> {
  fn readBytes(self , rsthis: & QDataStream) -> RetType;
}

  // proto:  QDataStream & QDataStream::readBytes(char *& , uint & len);
impl<'a> /*trait*/ QDataStream_readBytes<QDataStream> for (&'a mut String, &'a mut u32) {
  fn readBytes(self , rsthis: & QDataStream) -> QDataStream {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDataStream9readBytesERPcRj()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as *mut c_uint;
    let mut ret = unsafe {_ZN11QDataStream9readBytesERPcRj(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QDataStream::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDataStream::unsetDevice();
impl /*struct*/ QDataStream {
  pub fn unsetDevice<RetType, T: QDataStream_unsetDevice<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.unsetDevice(self);
    // return 1;
  }
}

pub trait QDataStream_unsetDevice<RetType> {
  fn unsetDevice(self , rsthis: & QDataStream) -> RetType;
}

  // proto:  void QDataStream::unsetDevice();
impl<'a> /*trait*/ QDataStream_unsetDevice<()> for () {
  fn unsetDevice(self , rsthis: & QDataStream) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDataStream11unsetDeviceEv()};
     unsafe {_ZN11QDataStream11unsetDeviceEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDataStream::QDataStream(const QDataStream & );
impl /*struct*/ QDataStream {
  pub fn New<T: QDataStream_New>(value: T) -> QDataStream {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QDataStream_New {
  fn New(self) -> QDataStream;
}

  // proto:  void QDataStream::QDataStream(const QDataStream & );
impl<'a> /*trait*/ QDataStream_New for (&'a QDataStream) {
  fn New(self) -> QDataStream {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDataStreamC1ERKS_()};
    let ctysz: c_int = unsafe{QDataStream_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN11QDataStreamC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN11QDataStreamC1ERKS_(arg0)};
    let rsthis = QDataStream{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDataStream::QDataStream(QIODevice * );
impl<'a> /*trait*/ QDataStream_New for (&'a QIODevice) {
  fn New(self) -> QDataStream {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDataStreamC1EP9QIODevice()};
    let ctysz: c_int = unsafe{QDataStream_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN11QDataStreamC1EP9QIODevice(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN11QDataStreamC1EP9QIODevice(arg0)};
    let rsthis = QDataStream{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDataStream::~QDataStream();
impl /*struct*/ QDataStream {
  pub fn Free<RetType, T: QDataStream_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QDataStream_Free<RetType> {
  fn Free(self , rsthis: & QDataStream) -> RetType;
}

  // proto:  void QDataStream::~QDataStream();
impl<'a> /*trait*/ QDataStream_Free<()> for () {
  fn Free(self , rsthis: & QDataStream) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDataStreamD0Ev()};
     unsafe {_ZN11QDataStreamD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QDataStream::skipRawData(int len);
impl /*struct*/ QDataStream {
  pub fn skipRawData<RetType, T: QDataStream_skipRawData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.skipRawData(self);
    // return 1;
  }
}

pub trait QDataStream_skipRawData<RetType> {
  fn skipRawData(self , rsthis: & QDataStream) -> RetType;
}

  // proto:  int QDataStream::skipRawData(int len);
impl<'a> /*trait*/ QDataStream_skipRawData<i32> for (i32) {
  fn skipRawData(self , rsthis: & QDataStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDataStream11skipRawDataEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN11QDataStream11skipRawDataEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QDataStream & QDataStream::writeBytes(const char * , uint len);
impl /*struct*/ QDataStream {
  pub fn writeBytes<RetType, T: QDataStream_writeBytes<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.writeBytes(self);
    // return 1;
  }
}

pub trait QDataStream_writeBytes<RetType> {
  fn writeBytes(self , rsthis: & QDataStream) -> RetType;
}

  // proto:  QDataStream & QDataStream::writeBytes(const char * , uint len);
impl<'a> /*trait*/ QDataStream_writeBytes<QDataStream> for (&'a  String, u32) {
  fn writeBytes(self , rsthis: & QDataStream) -> QDataStream {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDataStream10writeBytesEPKcj()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_uint;
    let mut ret = unsafe {_ZN11QDataStream10writeBytesEPKcj(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QDataStream::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDataStream::QDataStream();
impl<'a> /*trait*/ QDataStream_New for () {
  fn New(self) -> QDataStream {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDataStreamC1Ev()};
    let ctysz: c_int = unsafe{QDataStream_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN11QDataStreamC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN11QDataStreamC1Ev()};
    let rsthis = QDataStream{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDataStream::resetStatus();
impl /*struct*/ QDataStream {
  pub fn resetStatus<RetType, T: QDataStream_resetStatus<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resetStatus(self);
    // return 1;
  }
}

pub trait QDataStream_resetStatus<RetType> {
  fn resetStatus(self , rsthis: & QDataStream) -> RetType;
}

  // proto:  void QDataStream::resetStatus();
impl<'a> /*trait*/ QDataStream_resetStatus<()> for () {
  fn resetStatus(self , rsthis: & QDataStream) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDataStream11resetStatusEv()};
     unsafe {_ZN11QDataStream11resetStatusEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDataStream::QDataStream(const QByteArray & );
impl<'a> /*trait*/ QDataStream_New for (&'a QByteArray) {
  fn New(self) -> QDataStream {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDataStreamC1ERK10QByteArray()};
    let ctysz: c_int = unsafe{QDataStream_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN11QDataStreamC1ERK10QByteArray(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN11QDataStreamC1ERK10QByteArray(arg0)};
    let rsthis = QDataStream{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QDataStream::version();
impl /*struct*/ QDataStream {
  pub fn version<RetType, T: QDataStream_version<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.version(self);
    // return 1;
  }
}

pub trait QDataStream_version<RetType> {
  fn version(self , rsthis: & QDataStream) -> RetType;
}

  // proto:  int QDataStream::version();
impl<'a> /*trait*/ QDataStream_version<i32> for () {
  fn version(self , rsthis: & QDataStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QDataStream7versionEv()};
    let mut ret = unsafe {_ZNK11QDataStream7versionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QDataStream::atEnd();
impl /*struct*/ QDataStream {
  pub fn atEnd<RetType, T: QDataStream_atEnd<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.atEnd(self);
    // return 1;
  }
}

pub trait QDataStream_atEnd<RetType> {
  fn atEnd(self , rsthis: & QDataStream) -> RetType;
}

  // proto:  bool QDataStream::atEnd();
impl<'a> /*trait*/ QDataStream_atEnd<i8> for () {
  fn atEnd(self , rsthis: & QDataStream) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QDataStream5atEndEv()};
    let mut ret = unsafe {_ZNK11QDataStream5atEndEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QDataStream::setVersion(int );
impl /*struct*/ QDataStream {
  pub fn setVersion<RetType, T: QDataStream_setVersion<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setVersion(self);
    // return 1;
  }
}

pub trait QDataStream_setVersion<RetType> {
  fn setVersion(self , rsthis: & QDataStream) -> RetType;
}

  // proto:  void QDataStream::setVersion(int );
impl<'a> /*trait*/ QDataStream_setVersion<()> for (i32) {
  fn setVersion(self , rsthis: & QDataStream) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDataStream10setVersionEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QDataStream10setVersionEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDataStream::setDevice(QIODevice * );
impl /*struct*/ QDataStream {
  pub fn setDevice<RetType, T: QDataStream_setDevice<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDevice(self);
    // return 1;
  }
}

pub trait QDataStream_setDevice<RetType> {
  fn setDevice(self , rsthis: & QDataStream) -> RetType;
}

  // proto:  void QDataStream::setDevice(QIODevice * );
impl<'a> /*trait*/ QDataStream_setDevice<()> for (&'a QIODevice) {
  fn setDevice(self , rsthis: & QDataStream) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDataStream9setDeviceEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QDataStream9setDeviceEP9QIODevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QDataStream::writeRawData(const char * , int len);
impl /*struct*/ QDataStream {
  pub fn writeRawData<RetType, T: QDataStream_writeRawData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.writeRawData(self);
    // return 1;
  }
}

pub trait QDataStream_writeRawData<RetType> {
  fn writeRawData(self , rsthis: & QDataStream) -> RetType;
}

  // proto:  int QDataStream::writeRawData(const char * , int len);
impl<'a> /*trait*/ QDataStream_writeRawData<i32> for (&'a  String, i32) {
  fn writeRawData(self , rsthis: & QDataStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDataStream12writeRawDataEPKci()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN11QDataStream12writeRawDataEPKci(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QDataStream::readRawData(char * , int len);
impl /*struct*/ QDataStream {
  pub fn readRawData<RetType, T: QDataStream_readRawData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.readRawData(self);
    // return 1;
  }
}

pub trait QDataStream_readRawData<RetType> {
  fn readRawData(self , rsthis: & QDataStream) -> RetType;
}

  // proto:  int QDataStream::readRawData(char * , int len);
impl<'a> /*trait*/ QDataStream_readRawData<i32> for (&'a mut String, i32) {
  fn readRawData(self , rsthis: & QDataStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QDataStream11readRawDataEPci()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN11QDataStream11readRawDataEPci(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QIODevice * QDataStream::device();
impl /*struct*/ QDataStream {
  pub fn device<RetType, T: QDataStream_device<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.device(self);
    // return 1;
  }
}

pub trait QDataStream_device<RetType> {
  fn device(self , rsthis: & QDataStream) -> RetType;
}

  // proto:  QIODevice * QDataStream::device();
impl<'a> /*trait*/ QDataStream_device<QIODevice> for () {
  fn device(self , rsthis: & QDataStream) -> QIODevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QDataStream6deviceEv()};
    let mut ret = unsafe {_ZNK11QDataStream6deviceEv(rsthis.qclsinst)};
    let mut ret1 = QIODevice::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

// <= body block end

