// auto generated, do not modify.
// created: Sat Dec 26 10:52:38 2015
// src-file: /QtCore/qfiledevice.h
// dst-file: /src/core/qfiledevice.rs
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
use super::qiodevice::QIODevice; // 773
use std::ops::Deref;
use super::qstring::QString; // 773
use super::qobject::QObject; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QFileDevice_Class_Size() -> c_int;
  // proto:  qint64 QFileDevice::size();
  fn _ZNK11QFileDevice4sizeEv(qthis: *mut c_void) -> c_longlong;
  // proto:  bool QFileDevice::seek(qint64 offset);
  fn _ZN11QFileDevice4seekEx(qthis: *mut c_void, arg0: c_longlong) -> c_char;
  // proto:  bool QFileDevice::unmap(uchar * address);
  fn _ZN11QFileDevice5unmapEPh(qthis: *mut c_void, arg0: *mut c_uchar) -> c_char;
  // proto:  void QFileDevice::close();
  fn _ZN11QFileDevice5closeEv(qthis: *mut c_void);
  // proto:  qint64 QFileDevice::pos();
  fn _ZNK11QFileDevice3posEv(qthis: *mut c_void) -> c_longlong;
  // proto:  int QFileDevice::handle();
  fn _ZNK11QFileDevice6handleEv(qthis: *mut c_void) -> c_int;
  // proto:  QString QFileDevice::fileName();
  fn _ZNK11QFileDevice8fileNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QFileDevice::QFileDevice(QObject * parent);
  fn dector_ZN11QFileDeviceC1EP7QObject(arg0: *mut c_void) -> *mut c_void;
  fn _ZN11QFileDeviceC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QFileDevice::~QFileDevice();
  fn _ZN11QFileDeviceD0Ev(qthis: *mut c_void);
  // proto:  bool QFileDevice::atEnd();
  fn _ZNK11QFileDevice5atEndEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QFileDevice::isSequential();
  fn _ZNK11QFileDevice12isSequentialEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QFileDevice::flush();
  fn _ZN11QFileDevice5flushEv(qthis: *mut c_void) -> c_char;
  // proto:  void QFileDevice::QFileDevice();
  fn dector_ZN11QFileDeviceC1Ev() -> *mut c_void;
  fn _ZN11QFileDeviceC1Ev(qthis: *mut c_void);
  // proto:  void QFileDevice::unsetError();
  fn _ZN11QFileDevice10unsetErrorEv(qthis: *mut c_void);
  // proto:  void QFileDevice::QFileDevice(const QFileDevice & );
  fn dector_ZN11QFileDeviceC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN11QFileDeviceC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QFileDevice::metaObject();
  fn _ZNK11QFileDevice10metaObjectEv(qthis: *mut c_void);
  // proto:  bool QFileDevice::resize(qint64 sz);
  fn _ZN11QFileDevice6resizeEx(qthis: *mut c_void, arg0: c_longlong) -> c_char;
} // <= ext block end

// body block begin =>
// class sizeof(QFileDevice)=1
pub struct QFileDevice {
  qbase: QIODevice,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFileDevice {
  pub fn inheritFrom(qthis: *mut c_void) -> QFileDevice {
    return QFileDevice{qbase: QIODevice::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QFileDevice {
  type Target = QIODevice;

  fn deref(&self) -> &QIODevice {
    return & self.qbase;
  }
}
impl AsRef<QIODevice> for QFileDevice {
  fn as_ref(& self) -> & QIODevice {
    return & self.qbase;
  }
}
  // proto:  qint64 QFileDevice::size();
impl /*struct*/ QFileDevice {
  pub fn size<RetType, T: QFileDevice_size<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QFileDevice_size<RetType> {
  fn size(self , rsthis: & QFileDevice) -> RetType;
}

  // proto:  qint64 QFileDevice::size();
impl<'a> /*trait*/ QFileDevice_size<i64> for () {
  fn size(self , rsthis: & QFileDevice) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDevice4sizeEv()};
    let mut ret = unsafe {_ZNK11QFileDevice4sizeEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  bool QFileDevice::seek(qint64 offset);
impl /*struct*/ QFileDevice {
  pub fn seek<RetType, T: QFileDevice_seek<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.seek(self);
    // return 1;
  }
}

pub trait QFileDevice_seek<RetType> {
  fn seek(self , rsthis: & QFileDevice) -> RetType;
}

  // proto:  bool QFileDevice::seek(qint64 offset);
impl<'a> /*trait*/ QFileDevice_seek<i8> for (i64) {
  fn seek(self , rsthis: & QFileDevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDevice4seekEx()};
    let arg0 = self  as c_longlong;
    let mut ret = unsafe {_ZN11QFileDevice4seekEx(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QFileDevice::unmap(uchar * address);
impl /*struct*/ QFileDevice {
  pub fn unmap<RetType, T: QFileDevice_unmap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.unmap(self);
    // return 1;
  }
}

pub trait QFileDevice_unmap<RetType> {
  fn unmap(self , rsthis: & QFileDevice) -> RetType;
}

  // proto:  bool QFileDevice::unmap(uchar * address);
impl<'a> /*trait*/ QFileDevice_unmap<i8> for (&'a mut String) {
  fn unmap(self , rsthis: & QFileDevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDevice5unmapEPh()};
    let arg0 = self.as_ptr()  as *mut c_uchar;
    let mut ret = unsafe {_ZN11QFileDevice5unmapEPh(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QFileDevice::close();
impl /*struct*/ QFileDevice {
  pub fn close<RetType, T: QFileDevice_close<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.close(self);
    // return 1;
  }
}

pub trait QFileDevice_close<RetType> {
  fn close(self , rsthis: & QFileDevice) -> RetType;
}

  // proto:  void QFileDevice::close();
impl<'a> /*trait*/ QFileDevice_close<()> for () {
  fn close(self , rsthis: & QFileDevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDevice5closeEv()};
     unsafe {_ZN11QFileDevice5closeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qint64 QFileDevice::pos();
impl /*struct*/ QFileDevice {
  pub fn pos<RetType, T: QFileDevice_pos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QFileDevice_pos<RetType> {
  fn pos(self , rsthis: & QFileDevice) -> RetType;
}

  // proto:  qint64 QFileDevice::pos();
impl<'a> /*trait*/ QFileDevice_pos<i64> for () {
  fn pos(self , rsthis: & QFileDevice) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDevice3posEv()};
    let mut ret = unsafe {_ZNK11QFileDevice3posEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  int QFileDevice::handle();
impl /*struct*/ QFileDevice {
  pub fn handle<RetType, T: QFileDevice_handle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.handle(self);
    // return 1;
  }
}

pub trait QFileDevice_handle<RetType> {
  fn handle(self , rsthis: & QFileDevice) -> RetType;
}

  // proto:  int QFileDevice::handle();
impl<'a> /*trait*/ QFileDevice_handle<i32> for () {
  fn handle(self , rsthis: & QFileDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDevice6handleEv()};
    let mut ret = unsafe {_ZNK11QFileDevice6handleEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QString QFileDevice::fileName();
impl /*struct*/ QFileDevice {
  pub fn fileName<RetType, T: QFileDevice_fileName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fileName(self);
    // return 1;
  }
}

pub trait QFileDevice_fileName<RetType> {
  fn fileName(self , rsthis: & QFileDevice) -> RetType;
}

  // proto:  QString QFileDevice::fileName();
impl<'a> /*trait*/ QFileDevice_fileName<QString> for () {
  fn fileName(self , rsthis: & QFileDevice) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDevice8fileNameEv()};
    let mut ret = unsafe {_ZNK11QFileDevice8fileNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QFileDevice::QFileDevice(QObject * parent);
impl /*struct*/ QFileDevice {
  pub fn New<T: QFileDevice_New>(value: T) -> QFileDevice {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QFileDevice_New {
  fn New(self) -> QFileDevice;
}

  // proto:  void QFileDevice::QFileDevice(QObject * parent);
impl<'a> /*trait*/ QFileDevice_New for (&'a QObject) {
  fn New(self) -> QFileDevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDeviceC1EP7QObject()};
    let ctysz: c_int = unsafe{QFileDevice_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN11QFileDeviceC1EP7QObject(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN11QFileDeviceC1EP7QObject(arg0)};
    let rsthis = QFileDevice{/**/qbase: QIODevice::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QFileDevice::~QFileDevice();
impl /*struct*/ QFileDevice {
  pub fn Free<RetType, T: QFileDevice_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QFileDevice_Free<RetType> {
  fn Free(self , rsthis: & QFileDevice) -> RetType;
}

  // proto:  void QFileDevice::~QFileDevice();
impl<'a> /*trait*/ QFileDevice_Free<()> for () {
  fn Free(self , rsthis: & QFileDevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDeviceD0Ev()};
     unsafe {_ZN11QFileDeviceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QFileDevice::atEnd();
impl /*struct*/ QFileDevice {
  pub fn atEnd<RetType, T: QFileDevice_atEnd<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.atEnd(self);
    // return 1;
  }
}

pub trait QFileDevice_atEnd<RetType> {
  fn atEnd(self , rsthis: & QFileDevice) -> RetType;
}

  // proto:  bool QFileDevice::atEnd();
impl<'a> /*trait*/ QFileDevice_atEnd<i8> for () {
  fn atEnd(self , rsthis: & QFileDevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDevice5atEndEv()};
    let mut ret = unsafe {_ZNK11QFileDevice5atEndEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QFileDevice::isSequential();
impl /*struct*/ QFileDevice {
  pub fn isSequential<RetType, T: QFileDevice_isSequential<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSequential(self);
    // return 1;
  }
}

pub trait QFileDevice_isSequential<RetType> {
  fn isSequential(self , rsthis: & QFileDevice) -> RetType;
}

  // proto:  bool QFileDevice::isSequential();
impl<'a> /*trait*/ QFileDevice_isSequential<i8> for () {
  fn isSequential(self , rsthis: & QFileDevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDevice12isSequentialEv()};
    let mut ret = unsafe {_ZNK11QFileDevice12isSequentialEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QFileDevice::flush();
impl /*struct*/ QFileDevice {
  pub fn flush<RetType, T: QFileDevice_flush<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.flush(self);
    // return 1;
  }
}

pub trait QFileDevice_flush<RetType> {
  fn flush(self , rsthis: & QFileDevice) -> RetType;
}

  // proto:  bool QFileDevice::flush();
impl<'a> /*trait*/ QFileDevice_flush<i8> for () {
  fn flush(self , rsthis: & QFileDevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDevice5flushEv()};
    let mut ret = unsafe {_ZN11QFileDevice5flushEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QFileDevice::QFileDevice();
impl<'a> /*trait*/ QFileDevice_New for () {
  fn New(self) -> QFileDevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDeviceC1Ev()};
    let ctysz: c_int = unsafe{QFileDevice_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN11QFileDeviceC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN11QFileDeviceC1Ev()};
    let rsthis = QFileDevice{/**/qbase: QIODevice::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QFileDevice::unsetError();
impl /*struct*/ QFileDevice {
  pub fn unsetError<RetType, T: QFileDevice_unsetError<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.unsetError(self);
    // return 1;
  }
}

pub trait QFileDevice_unsetError<RetType> {
  fn unsetError(self , rsthis: & QFileDevice) -> RetType;
}

  // proto:  void QFileDevice::unsetError();
impl<'a> /*trait*/ QFileDevice_unsetError<()> for () {
  fn unsetError(self , rsthis: & QFileDevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDevice10unsetErrorEv()};
     unsafe {_ZN11QFileDevice10unsetErrorEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QFileDevice::QFileDevice(const QFileDevice & );
impl<'a> /*trait*/ QFileDevice_New for (&'a QFileDevice) {
  fn New(self) -> QFileDevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDeviceC1ERKS_()};
    let ctysz: c_int = unsafe{QFileDevice_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN11QFileDeviceC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN11QFileDeviceC1ERKS_(arg0)};
    let rsthis = QFileDevice{/**/qbase: QIODevice::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QFileDevice::metaObject();
impl /*struct*/ QFileDevice {
  pub fn metaObject<RetType, T: QFileDevice_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QFileDevice_metaObject<RetType> {
  fn metaObject(self , rsthis: & QFileDevice) -> RetType;
}

  // proto:  const QMetaObject * QFileDevice::metaObject();
impl<'a> /*trait*/ QFileDevice_metaObject<()> for () {
  fn metaObject(self , rsthis: & QFileDevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDevice10metaObjectEv()};
     unsafe {_ZNK11QFileDevice10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QFileDevice::resize(qint64 sz);
impl /*struct*/ QFileDevice {
  pub fn resize<RetType, T: QFileDevice_resize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resize(self);
    // return 1;
  }
}

pub trait QFileDevice_resize<RetType> {
  fn resize(self , rsthis: & QFileDevice) -> RetType;
}

  // proto:  bool QFileDevice::resize(qint64 sz);
impl<'a> /*trait*/ QFileDevice_resize<i8> for (i64) {
  fn resize(self , rsthis: & QFileDevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDevice6resizeEx()};
    let arg0 = self  as c_longlong;
    let mut ret = unsafe {_ZN11QFileDevice6resizeEx(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// <= body block end

