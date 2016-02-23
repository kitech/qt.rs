// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtNetwork/qhttpmultipart.h
// dst-file: /src/network/qhttpmultipart.rs
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
use super::super::core::qobject::QObject; // 771
use std::ops::Deref;
use super::super::core::qbytearray::QByteArray; // 771
// use super::qhttpmultipart::QHttpPart; // 773
use super::super::core::qiodevice::QIODevice; // 771
use super::super::core::qvariant::QVariant; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QHttpMultiPart_Class_Size() -> c_int;
  // proto:  const QMetaObject * QHttpMultiPart::metaObject();
  fn _ZNK14QHttpMultiPart10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QHttpMultiPart::~QHttpMultiPart();
  fn _ZN14QHttpMultiPartD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QHttpMultiPart::setBoundary(const QByteArray & boundary);
  fn _ZN14QHttpMultiPart11setBoundaryERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QHttpMultiPart::QHttpMultiPart(QObject * parent);
  fn _ZN14QHttpMultiPartC2EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QHttpMultiPart::append(const QHttpPart & httpPart);
  fn _ZN14QHttpMultiPart6appendERK9QHttpPart(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QHttpMultiPart::QHttpMultiPart(const QHttpMultiPart & );
  fn _ZN14QHttpMultiPartC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QByteArray QHttpMultiPart::boundary();
  fn _ZNK14QHttpMultiPart8boundaryEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QHttpPart_Class_Size() -> c_int;
  // proto:  void QHttpPart::setBodyDevice(QIODevice * device);
  fn _ZN9QHttpPart13setBodyDeviceEP9QIODevice(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QHttpPart::QHttpPart(const QHttpPart & other);
  fn _ZN9QHttpPartC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QHttpPart::swap(QHttpPart & other);
  fn _ZN9QHttpPart4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QHttpPart::~QHttpPart();
  fn _ZN9QHttpPartD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QHttpPart::QHttpPart();
  fn _ZN9QHttpPartC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QHttpPart::setBody(const QByteArray & body);
  fn _ZN9QHttpPart7setBodyERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QHttpPart::setRawHeader(const QByteArray & headerName, const QByteArray & headerValue);
  fn _ZN9QHttpPart12setRawHeaderERK10QByteArrayS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QHttpMultiPart)=1
#[derive(Default)]
pub struct QHttpMultiPart {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QHttpPart)=1
#[derive(Default)]
pub struct QHttpPart {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QHttpMultiPart {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QHttpMultiPart {
    return QHttpMultiPart{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QHttpMultiPart {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QHttpMultiPart {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  const QMetaObject * QHttpMultiPart::metaObject();
impl /*struct*/ QHttpMultiPart {
  pub fn metaObject<RetType, T: QHttpMultiPart_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QHttpMultiPart_metaObject<RetType> {
  fn metaObject(self , rsthis: & QHttpMultiPart) -> RetType;
}

  // proto:  const QMetaObject * QHttpMultiPart::metaObject();
impl<'a> /*trait*/ QHttpMultiPart_metaObject<()> for () {
  fn metaObject(self , rsthis: & QHttpMultiPart) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QHttpMultiPart10metaObjectEv()};
     unsafe {_ZNK14QHttpMultiPart10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QHttpMultiPart::~QHttpMultiPart();
impl /*struct*/ QHttpMultiPart {
  pub fn free<RetType, T: QHttpMultiPart_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QHttpMultiPart_free<RetType> {
  fn free(self , rsthis: & QHttpMultiPart) -> RetType;
}

  // proto:  void QHttpMultiPart::~QHttpMultiPart();
impl<'a> /*trait*/ QHttpMultiPart_free<()> for () {
  fn free(self , rsthis: & QHttpMultiPart) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QHttpMultiPartD2Ev()};
     unsafe {_ZN14QHttpMultiPartD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QHttpMultiPart::setBoundary(const QByteArray & boundary);
impl /*struct*/ QHttpMultiPart {
  pub fn setBoundary<RetType, T: QHttpMultiPart_setBoundary<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBoundary(self);
    // return 1;
  }
}

pub trait QHttpMultiPart_setBoundary<RetType> {
  fn setBoundary(self , rsthis: & QHttpMultiPart) -> RetType;
}

  // proto:  void QHttpMultiPart::setBoundary(const QByteArray & boundary);
impl<'a> /*trait*/ QHttpMultiPart_setBoundary<()> for (&'a QByteArray) {
  fn setBoundary(self , rsthis: & QHttpMultiPart) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QHttpMultiPart11setBoundaryERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QHttpMultiPart11setBoundaryERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QHttpMultiPart::QHttpMultiPart(QObject * parent);
impl /*struct*/ QHttpMultiPart {
  pub fn new<T: QHttpMultiPart_new>(value: T) -> QHttpMultiPart {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QHttpMultiPart_new {
  fn new(self) -> QHttpMultiPart;
}

  // proto:  void QHttpMultiPart::QHttpMultiPart(QObject * parent);
impl<'a> /*trait*/ QHttpMultiPart_new for (&'a QObject) {
  fn new(self) -> QHttpMultiPart {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QHttpMultiPartC2EP7QObject()};
    let ctysz: c_int = unsafe{QHttpMultiPart_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QHttpMultiPartC2EP7QObject(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QHttpMultiPart{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QHttpMultiPart::append(const QHttpPart & httpPart);
impl /*struct*/ QHttpMultiPart {
  pub fn append<RetType, T: QHttpMultiPart_append<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.append(self);
    // return 1;
  }
}

pub trait QHttpMultiPart_append<RetType> {
  fn append(self , rsthis: & QHttpMultiPart) -> RetType;
}

  // proto:  void QHttpMultiPart::append(const QHttpPart & httpPart);
impl<'a> /*trait*/ QHttpMultiPart_append<()> for (&'a QHttpPart) {
  fn append(self , rsthis: & QHttpMultiPart) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QHttpMultiPart6appendERK9QHttpPart()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QHttpMultiPart6appendERK9QHttpPart(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QHttpMultiPart::QHttpMultiPart(const QHttpMultiPart & );
impl<'a> /*trait*/ QHttpMultiPart_new for (&'a QHttpMultiPart) {
  fn new(self) -> QHttpMultiPart {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QHttpMultiPartC2ERKS_()};
    let ctysz: c_int = unsafe{QHttpMultiPart_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QHttpMultiPartC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QHttpMultiPart{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QByteArray QHttpMultiPart::boundary();
impl /*struct*/ QHttpMultiPart {
  pub fn boundary<RetType, T: QHttpMultiPart_boundary<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.boundary(self);
    // return 1;
  }
}

pub trait QHttpMultiPart_boundary<RetType> {
  fn boundary(self , rsthis: & QHttpMultiPart) -> RetType;
}

  // proto:  QByteArray QHttpMultiPart::boundary();
impl<'a> /*trait*/ QHttpMultiPart_boundary<QByteArray> for () {
  fn boundary(self , rsthis: & QHttpMultiPart) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QHttpMultiPart8boundaryEv()};
    let mut ret = unsafe {_ZNK14QHttpMultiPart8boundaryEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QHttpPart {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QHttpPart {
    return QHttpPart{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QHttpPart::setBodyDevice(QIODevice * device);
impl /*struct*/ QHttpPart {
  pub fn setBodyDevice<RetType, T: QHttpPart_setBodyDevice<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBodyDevice(self);
    // return 1;
  }
}

pub trait QHttpPart_setBodyDevice<RetType> {
  fn setBodyDevice(self , rsthis: & QHttpPart) -> RetType;
}

  // proto:  void QHttpPart::setBodyDevice(QIODevice * device);
impl<'a> /*trait*/ QHttpPart_setBodyDevice<()> for (&'a QIODevice) {
  fn setBodyDevice(self , rsthis: & QHttpPart) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QHttpPart13setBodyDeviceEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QHttpPart13setBodyDeviceEP9QIODevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QHttpPart::QHttpPart(const QHttpPart & other);
impl /*struct*/ QHttpPart {
  pub fn new<T: QHttpPart_new>(value: T) -> QHttpPart {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QHttpPart_new {
  fn new(self) -> QHttpPart;
}

  // proto:  void QHttpPart::QHttpPart(const QHttpPart & other);
impl<'a> /*trait*/ QHttpPart_new for (&'a QHttpPart) {
  fn new(self) -> QHttpPart {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QHttpPartC2ERKS_()};
    let ctysz: c_int = unsafe{QHttpPart_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QHttpPartC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QHttpPart{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QHttpPart::swap(QHttpPart & other);
impl /*struct*/ QHttpPart {
  pub fn swap<RetType, T: QHttpPart_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QHttpPart_swap<RetType> {
  fn swap(self , rsthis: & QHttpPart) -> RetType;
}

  // proto:  void QHttpPart::swap(QHttpPart & other);
impl<'a> /*trait*/ QHttpPart_swap<()> for (&'a QHttpPart) {
  fn swap(self , rsthis: & QHttpPart) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QHttpPart4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QHttpPart4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QHttpPart::~QHttpPart();
impl /*struct*/ QHttpPart {
  pub fn free<RetType, T: QHttpPart_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QHttpPart_free<RetType> {
  fn free(self , rsthis: & QHttpPart) -> RetType;
}

  // proto:  void QHttpPart::~QHttpPart();
impl<'a> /*trait*/ QHttpPart_free<()> for () {
  fn free(self , rsthis: & QHttpPart) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QHttpPartD2Ev()};
     unsafe {_ZN9QHttpPartD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QHttpPart::QHttpPart();
impl<'a> /*trait*/ QHttpPart_new for () {
  fn new(self) -> QHttpPart {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QHttpPartC2Ev()};
    let ctysz: c_int = unsafe{QHttpPart_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN9QHttpPartC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QHttpPart{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QHttpPart::setBody(const QByteArray & body);
impl /*struct*/ QHttpPart {
  pub fn setBody<RetType, T: QHttpPart_setBody<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBody(self);
    // return 1;
  }
}

pub trait QHttpPart_setBody<RetType> {
  fn setBody(self , rsthis: & QHttpPart) -> RetType;
}

  // proto:  void QHttpPart::setBody(const QByteArray & body);
impl<'a> /*trait*/ QHttpPart_setBody<()> for (&'a QByteArray) {
  fn setBody(self , rsthis: & QHttpPart) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QHttpPart7setBodyERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QHttpPart7setBodyERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QHttpPart::setRawHeader(const QByteArray & headerName, const QByteArray & headerValue);
impl /*struct*/ QHttpPart {
  pub fn setRawHeader<RetType, T: QHttpPart_setRawHeader<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRawHeader(self);
    // return 1;
  }
}

pub trait QHttpPart_setRawHeader<RetType> {
  fn setRawHeader(self , rsthis: & QHttpPart) -> RetType;
}

  // proto:  void QHttpPart::setRawHeader(const QByteArray & headerName, const QByteArray & headerValue);
impl<'a> /*trait*/ QHttpPart_setRawHeader<()> for (&'a QByteArray, &'a QByteArray) {
  fn setRawHeader(self , rsthis: & QHttpPart) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QHttpPart12setRawHeaderERK10QByteArrayS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN9QHttpPart12setRawHeaderERK10QByteArrayS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// <= body block end

