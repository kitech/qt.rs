// auto generated, do not modify.
// created: Sat Dec 26 10:52:38 2015
// src-file: /QtCore/qsharedmemory.h
// dst-file: /src/core/qsharedmemory.rs
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
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSharedMemory_Class_Size() -> c_int;
  // proto:  int QSharedMemory::size();
  fn _ZNK13QSharedMemory4sizeEv(qthis: *mut c_void) -> c_int;
  // proto:  void QSharedMemory::setNativeKey(const QString & key);
  fn _ZN13QSharedMemory12setNativeKeyERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QSharedMemory::QSharedMemory(const QString & key, QObject * parent);
  fn dector_ZN13QSharedMemoryC1ERK7QStringP7QObject(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  fn _ZN13QSharedMemoryC1ERK7QStringP7QObject(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QString QSharedMemory::errorString();
  fn _ZNK13QSharedMemory11errorStringEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QSharedMemory::setKey(const QString & key);
  fn _ZN13QSharedMemory6setKeyERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QString QSharedMemory::key();
  fn _ZNK13QSharedMemory3keyEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const void * QSharedMemory::constData();
  fn _ZNK13QSharedMemory9constDataEv(qthis: *mut c_void);
  // proto:  void * QSharedMemory::data();
  fn _ZN13QSharedMemory4dataEv(qthis: *mut c_void);
  // proto:  void QSharedMemory::QSharedMemory(const QSharedMemory & );
  fn dector_ZN13QSharedMemoryC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN13QSharedMemoryC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QSharedMemory::isAttached();
  fn _ZNK13QSharedMemory10isAttachedEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QSharedMemory::lock();
  fn _ZN13QSharedMemory4lockEv(qthis: *mut c_void) -> c_char;
  // proto:  void QSharedMemory::~QSharedMemory();
  fn _ZN13QSharedMemoryD0Ev(qthis: *mut c_void);
  // proto:  bool QSharedMemory::unlock();
  fn _ZN13QSharedMemory6unlockEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QSharedMemory::detach();
  fn _ZN13QSharedMemory6detachEv(qthis: *mut c_void) -> c_char;
  // proto:  QString QSharedMemory::nativeKey();
  fn _ZNK13QSharedMemory9nativeKeyEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QSharedMemory::metaObject();
  fn _ZNK13QSharedMemory10metaObjectEv(qthis: *mut c_void);
  // proto:  void QSharedMemory::QSharedMemory(QObject * parent);
  fn dector_ZN13QSharedMemoryC1EP7QObject(arg0: *mut c_void) -> *mut c_void;
  fn _ZN13QSharedMemoryC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QSharedMemory)=1
pub struct QSharedMemory {
  qbase: QObject,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSharedMemory {
  pub fn inheritFrom(qthis: *mut c_void) -> QSharedMemory {
    return QSharedMemory{qbase: QObject::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QSharedMemory {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QSharedMemory {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  int QSharedMemory::size();
impl /*struct*/ QSharedMemory {
  pub fn size<RetType, T: QSharedMemory_size<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QSharedMemory_size<RetType> {
  fn size(self , rsthis: & QSharedMemory) -> RetType;
}

  // proto:  int QSharedMemory::size();
impl<'a> /*trait*/ QSharedMemory_size<i32> for () {
  fn size(self , rsthis: & QSharedMemory) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSharedMemory4sizeEv()};
    let mut ret = unsafe {_ZNK13QSharedMemory4sizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QSharedMemory::setNativeKey(const QString & key);
impl /*struct*/ QSharedMemory {
  pub fn setNativeKey<RetType, T: QSharedMemory_setNativeKey<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setNativeKey(self);
    // return 1;
  }
}

pub trait QSharedMemory_setNativeKey<RetType> {
  fn setNativeKey(self , rsthis: & QSharedMemory) -> RetType;
}

  // proto:  void QSharedMemory::setNativeKey(const QString & key);
impl<'a> /*trait*/ QSharedMemory_setNativeKey<()> for (&'a QString) {
  fn setNativeKey(self , rsthis: & QSharedMemory) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemory12setNativeKeyERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QSharedMemory12setNativeKeyERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSharedMemory::QSharedMemory(const QString & key, QObject * parent);
impl /*struct*/ QSharedMemory {
  pub fn New<T: QSharedMemory_New>(value: T) -> QSharedMemory {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QSharedMemory_New {
  fn New(self) -> QSharedMemory;
}

  // proto:  void QSharedMemory::QSharedMemory(const QString & key, QObject * parent);
impl<'a> /*trait*/ QSharedMemory_New for (&'a QString, &'a QObject) {
  fn New(self) -> QSharedMemory {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemoryC1ERK7QStringP7QObject()};
    let ctysz: c_int = unsafe{QSharedMemory_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    // unsafe {_ZN13QSharedMemoryC1ERK7QStringP7QObject(qthis, arg0, arg1)};
    let qthis: *mut c_void = unsafe {dector_ZN13QSharedMemoryC1ERK7QStringP7QObject(arg0, arg1)};
    let rsthis = QSharedMemory{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QSharedMemory::errorString();
impl /*struct*/ QSharedMemory {
  pub fn errorString<RetType, T: QSharedMemory_errorString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.errorString(self);
    // return 1;
  }
}

pub trait QSharedMemory_errorString<RetType> {
  fn errorString(self , rsthis: & QSharedMemory) -> RetType;
}

  // proto:  QString QSharedMemory::errorString();
impl<'a> /*trait*/ QSharedMemory_errorString<QString> for () {
  fn errorString(self , rsthis: & QSharedMemory) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSharedMemory11errorStringEv()};
    let mut ret = unsafe {_ZNK13QSharedMemory11errorStringEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSharedMemory::setKey(const QString & key);
impl /*struct*/ QSharedMemory {
  pub fn setKey<RetType, T: QSharedMemory_setKey<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setKey(self);
    // return 1;
  }
}

pub trait QSharedMemory_setKey<RetType> {
  fn setKey(self , rsthis: & QSharedMemory) -> RetType;
}

  // proto:  void QSharedMemory::setKey(const QString & key);
impl<'a> /*trait*/ QSharedMemory_setKey<()> for (&'a QString) {
  fn setKey(self , rsthis: & QSharedMemory) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemory6setKeyERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QSharedMemory6setKeyERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QSharedMemory::key();
impl /*struct*/ QSharedMemory {
  pub fn key<RetType, T: QSharedMemory_key<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.key(self);
    // return 1;
  }
}

pub trait QSharedMemory_key<RetType> {
  fn key(self , rsthis: & QSharedMemory) -> RetType;
}

  // proto:  QString QSharedMemory::key();
impl<'a> /*trait*/ QSharedMemory_key<QString> for () {
  fn key(self , rsthis: & QSharedMemory) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSharedMemory3keyEv()};
    let mut ret = unsafe {_ZNK13QSharedMemory3keyEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  const void * QSharedMemory::constData();
impl /*struct*/ QSharedMemory {
  pub fn constData<RetType, T: QSharedMemory_constData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.constData(self);
    // return 1;
  }
}

pub trait QSharedMemory_constData<RetType> {
  fn constData(self , rsthis: & QSharedMemory) -> RetType;
}

  // proto:  const void * QSharedMemory::constData();
impl<'a> /*trait*/ QSharedMemory_constData<()> for () {
  fn constData(self , rsthis: & QSharedMemory) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSharedMemory9constDataEv()};
     unsafe {_ZNK13QSharedMemory9constDataEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void * QSharedMemory::data();
impl /*struct*/ QSharedMemory {
  pub fn data<RetType, T: QSharedMemory_data<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QSharedMemory_data<RetType> {
  fn data(self , rsthis: & QSharedMemory) -> RetType;
}

  // proto:  void * QSharedMemory::data();
impl<'a> /*trait*/ QSharedMemory_data<()> for () {
  fn data(self , rsthis: & QSharedMemory) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemory4dataEv()};
     unsafe {_ZN13QSharedMemory4dataEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSharedMemory::QSharedMemory(const QSharedMemory & );
impl<'a> /*trait*/ QSharedMemory_New for (&'a QSharedMemory) {
  fn New(self) -> QSharedMemory {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemoryC1ERKS_()};
    let ctysz: c_int = unsafe{QSharedMemory_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN13QSharedMemoryC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN13QSharedMemoryC1ERKS_(arg0)};
    let rsthis = QSharedMemory{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QSharedMemory::isAttached();
impl /*struct*/ QSharedMemory {
  pub fn isAttached<RetType, T: QSharedMemory_isAttached<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isAttached(self);
    // return 1;
  }
}

pub trait QSharedMemory_isAttached<RetType> {
  fn isAttached(self , rsthis: & QSharedMemory) -> RetType;
}

  // proto:  bool QSharedMemory::isAttached();
impl<'a> /*trait*/ QSharedMemory_isAttached<i8> for () {
  fn isAttached(self , rsthis: & QSharedMemory) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSharedMemory10isAttachedEv()};
    let mut ret = unsafe {_ZNK13QSharedMemory10isAttachedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QSharedMemory::lock();
impl /*struct*/ QSharedMemory {
  pub fn lock<RetType, T: QSharedMemory_lock<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lock(self);
    // return 1;
  }
}

pub trait QSharedMemory_lock<RetType> {
  fn lock(self , rsthis: & QSharedMemory) -> RetType;
}

  // proto:  bool QSharedMemory::lock();
impl<'a> /*trait*/ QSharedMemory_lock<i8> for () {
  fn lock(self , rsthis: & QSharedMemory) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemory4lockEv()};
    let mut ret = unsafe {_ZN13QSharedMemory4lockEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QSharedMemory::~QSharedMemory();
impl /*struct*/ QSharedMemory {
  pub fn Free<RetType, T: QSharedMemory_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QSharedMemory_Free<RetType> {
  fn Free(self , rsthis: & QSharedMemory) -> RetType;
}

  // proto:  void QSharedMemory::~QSharedMemory();
impl<'a> /*trait*/ QSharedMemory_Free<()> for () {
  fn Free(self , rsthis: & QSharedMemory) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemoryD0Ev()};
     unsafe {_ZN13QSharedMemoryD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QSharedMemory::unlock();
impl /*struct*/ QSharedMemory {
  pub fn unlock<RetType, T: QSharedMemory_unlock<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.unlock(self);
    // return 1;
  }
}

pub trait QSharedMemory_unlock<RetType> {
  fn unlock(self , rsthis: & QSharedMemory) -> RetType;
}

  // proto:  bool QSharedMemory::unlock();
impl<'a> /*trait*/ QSharedMemory_unlock<i8> for () {
  fn unlock(self , rsthis: & QSharedMemory) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemory6unlockEv()};
    let mut ret = unsafe {_ZN13QSharedMemory6unlockEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QSharedMemory::detach();
impl /*struct*/ QSharedMemory {
  pub fn detach<RetType, T: QSharedMemory_detach<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.detach(self);
    // return 1;
  }
}

pub trait QSharedMemory_detach<RetType> {
  fn detach(self , rsthis: & QSharedMemory) -> RetType;
}

  // proto:  bool QSharedMemory::detach();
impl<'a> /*trait*/ QSharedMemory_detach<i8> for () {
  fn detach(self , rsthis: & QSharedMemory) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemory6detachEv()};
    let mut ret = unsafe {_ZN13QSharedMemory6detachEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QSharedMemory::nativeKey();
impl /*struct*/ QSharedMemory {
  pub fn nativeKey<RetType, T: QSharedMemory_nativeKey<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.nativeKey(self);
    // return 1;
  }
}

pub trait QSharedMemory_nativeKey<RetType> {
  fn nativeKey(self , rsthis: & QSharedMemory) -> RetType;
}

  // proto:  QString QSharedMemory::nativeKey();
impl<'a> /*trait*/ QSharedMemory_nativeKey<QString> for () {
  fn nativeKey(self , rsthis: & QSharedMemory) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSharedMemory9nativeKeyEv()};
    let mut ret = unsafe {_ZNK13QSharedMemory9nativeKeyEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QSharedMemory::metaObject();
impl /*struct*/ QSharedMemory {
  pub fn metaObject<RetType, T: QSharedMemory_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QSharedMemory_metaObject<RetType> {
  fn metaObject(self , rsthis: & QSharedMemory) -> RetType;
}

  // proto:  const QMetaObject * QSharedMemory::metaObject();
impl<'a> /*trait*/ QSharedMemory_metaObject<()> for () {
  fn metaObject(self , rsthis: & QSharedMemory) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSharedMemory10metaObjectEv()};
     unsafe {_ZNK13QSharedMemory10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSharedMemory::QSharedMemory(QObject * parent);
impl<'a> /*trait*/ QSharedMemory_New for (&'a QObject) {
  fn New(self) -> QSharedMemory {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemoryC1EP7QObject()};
    let ctysz: c_int = unsafe{QSharedMemory_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN13QSharedMemoryC1EP7QObject(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN13QSharedMemoryC1EP7QObject(arg0)};
    let rsthis = QSharedMemory{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// <= body block end

