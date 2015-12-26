// auto generated, do not modify.
// created: Sat Dec 26 10:52:38 2015
// src-file: /QtCore/qsavefile.h
// dst-file: /src/core/qsavefile.rs
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
use super::qfiledevice::QFileDevice; // 773
use std::ops::Deref;
use super::qobject::QObject; // 773
use super::qstring::QString; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSaveFile_Class_Size() -> c_int;
  // proto:  void QSaveFile::cancelWriting();
  fn _ZN9QSaveFile13cancelWritingEv(qthis: *mut c_void);
  // proto:  void QSaveFile::QSaveFile(QObject * parent);
  fn dector_ZN9QSaveFileC1EP7QObject(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QSaveFileC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QSaveFile::QSaveFile(const QSaveFile & );
  fn dector_ZN9QSaveFileC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QSaveFileC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QSaveFile::QSaveFile(const QString & name, QObject * parent);
  fn dector_ZN9QSaveFileC1ERK7QStringP7QObject(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  fn _ZN9QSaveFileC1ERK7QStringP7QObject(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QString QSaveFile::fileName();
  fn _ZNK9QSaveFile8fileNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QSaveFile::QSaveFile(const QString & name);
  fn dector_ZN9QSaveFileC1ERK7QString(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QSaveFileC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QSaveFile::metaObject();
  fn _ZNK9QSaveFile10metaObjectEv(qthis: *mut c_void);
  // proto:  bool QSaveFile::commit();
  fn _ZN9QSaveFile6commitEv(qthis: *mut c_void) -> c_char;
  // proto:  void QSaveFile::~QSaveFile();
  fn _ZN9QSaveFileD0Ev(qthis: *mut c_void);
  // proto:  void QSaveFile::setFileName(const QString & name);
  fn _ZN9QSaveFile11setFileNameERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QSaveFile::directWriteFallback();
  fn _ZNK9QSaveFile19directWriteFallbackEv(qthis: *mut c_void) -> c_char;
  // proto:  void QSaveFile::setDirectWriteFallback(bool enabled);
  fn _ZN9QSaveFile22setDirectWriteFallbackEb(qthis: *mut c_void, arg0: c_char);
} // <= ext block end

// body block begin =>
// class sizeof(QSaveFile)=1
pub struct QSaveFile {
  qbase: QFileDevice,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSaveFile {
  pub fn inheritFrom(qthis: *mut c_void) -> QSaveFile {
    return QSaveFile{qbase: QFileDevice::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QSaveFile {
  type Target = QFileDevice;

  fn deref(&self) -> &QFileDevice {
    return & self.qbase;
  }
}
impl AsRef<QFileDevice> for QSaveFile {
  fn as_ref(& self) -> & QFileDevice {
    return & self.qbase;
  }
}
  // proto:  void QSaveFile::cancelWriting();
impl /*struct*/ QSaveFile {
  pub fn cancelWriting<RetType, T: QSaveFile_cancelWriting<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cancelWriting(self);
    // return 1;
  }
}

pub trait QSaveFile_cancelWriting<RetType> {
  fn cancelWriting(self , rsthis: & QSaveFile) -> RetType;
}

  // proto:  void QSaveFile::cancelWriting();
impl<'a> /*trait*/ QSaveFile_cancelWriting<()> for () {
  fn cancelWriting(self , rsthis: & QSaveFile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSaveFile13cancelWritingEv()};
     unsafe {_ZN9QSaveFile13cancelWritingEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSaveFile::QSaveFile(QObject * parent);
impl /*struct*/ QSaveFile {
  pub fn New<T: QSaveFile_New>(value: T) -> QSaveFile {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QSaveFile_New {
  fn New(self) -> QSaveFile;
}

  // proto:  void QSaveFile::QSaveFile(QObject * parent);
impl<'a> /*trait*/ QSaveFile_New for (&'a QObject) {
  fn New(self) -> QSaveFile {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSaveFileC1EP7QObject()};
    let ctysz: c_int = unsafe{QSaveFile_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QSaveFileC1EP7QObject(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN9QSaveFileC1EP7QObject(arg0)};
    let rsthis = QSaveFile{/**/qbase: QFileDevice::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSaveFile::QSaveFile(const QSaveFile & );
impl<'a> /*trait*/ QSaveFile_New for (&'a QSaveFile) {
  fn New(self) -> QSaveFile {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSaveFileC1ERKS_()};
    let ctysz: c_int = unsafe{QSaveFile_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QSaveFileC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN9QSaveFileC1ERKS_(arg0)};
    let rsthis = QSaveFile{/**/qbase: QFileDevice::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSaveFile::QSaveFile(const QString & name, QObject * parent);
impl<'a> /*trait*/ QSaveFile_New for (&'a QString, &'a QObject) {
  fn New(self) -> QSaveFile {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSaveFileC1ERK7QStringP7QObject()};
    let ctysz: c_int = unsafe{QSaveFile_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    // unsafe {_ZN9QSaveFileC1ERK7QStringP7QObject(qthis, arg0, arg1)};
    let qthis: *mut c_void = unsafe {dector_ZN9QSaveFileC1ERK7QStringP7QObject(arg0, arg1)};
    let rsthis = QSaveFile{/**/qbase: QFileDevice::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QSaveFile::fileName();
impl /*struct*/ QSaveFile {
  pub fn fileName<RetType, T: QSaveFile_fileName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fileName(self);
    // return 1;
  }
}

pub trait QSaveFile_fileName<RetType> {
  fn fileName(self , rsthis: & QSaveFile) -> RetType;
}

  // proto:  QString QSaveFile::fileName();
impl<'a> /*trait*/ QSaveFile_fileName<QString> for () {
  fn fileName(self , rsthis: & QSaveFile) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSaveFile8fileNameEv()};
    let mut ret = unsafe {_ZNK9QSaveFile8fileNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSaveFile::QSaveFile(const QString & name);
impl<'a> /*trait*/ QSaveFile_New for (&'a QString) {
  fn New(self) -> QSaveFile {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSaveFileC1ERK7QString()};
    let ctysz: c_int = unsafe{QSaveFile_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QSaveFileC1ERK7QString(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN9QSaveFileC1ERK7QString(arg0)};
    let rsthis = QSaveFile{/**/qbase: QFileDevice::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QSaveFile::metaObject();
impl /*struct*/ QSaveFile {
  pub fn metaObject<RetType, T: QSaveFile_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QSaveFile_metaObject<RetType> {
  fn metaObject(self , rsthis: & QSaveFile) -> RetType;
}

  // proto:  const QMetaObject * QSaveFile::metaObject();
impl<'a> /*trait*/ QSaveFile_metaObject<()> for () {
  fn metaObject(self , rsthis: & QSaveFile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSaveFile10metaObjectEv()};
     unsafe {_ZNK9QSaveFile10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QSaveFile::commit();
impl /*struct*/ QSaveFile {
  pub fn commit<RetType, T: QSaveFile_commit<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.commit(self);
    // return 1;
  }
}

pub trait QSaveFile_commit<RetType> {
  fn commit(self , rsthis: & QSaveFile) -> RetType;
}

  // proto:  bool QSaveFile::commit();
impl<'a> /*trait*/ QSaveFile_commit<i8> for () {
  fn commit(self , rsthis: & QSaveFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSaveFile6commitEv()};
    let mut ret = unsafe {_ZN9QSaveFile6commitEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QSaveFile::~QSaveFile();
impl /*struct*/ QSaveFile {
  pub fn Free<RetType, T: QSaveFile_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QSaveFile_Free<RetType> {
  fn Free(self , rsthis: & QSaveFile) -> RetType;
}

  // proto:  void QSaveFile::~QSaveFile();
impl<'a> /*trait*/ QSaveFile_Free<()> for () {
  fn Free(self , rsthis: & QSaveFile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSaveFileD0Ev()};
     unsafe {_ZN9QSaveFileD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSaveFile::setFileName(const QString & name);
impl /*struct*/ QSaveFile {
  pub fn setFileName<RetType, T: QSaveFile_setFileName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFileName(self);
    // return 1;
  }
}

pub trait QSaveFile_setFileName<RetType> {
  fn setFileName(self , rsthis: & QSaveFile) -> RetType;
}

  // proto:  void QSaveFile::setFileName(const QString & name);
impl<'a> /*trait*/ QSaveFile_setFileName<()> for (&'a QString) {
  fn setFileName(self , rsthis: & QSaveFile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSaveFile11setFileNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QSaveFile11setFileNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QSaveFile::directWriteFallback();
impl /*struct*/ QSaveFile {
  pub fn directWriteFallback<RetType, T: QSaveFile_directWriteFallback<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.directWriteFallback(self);
    // return 1;
  }
}

pub trait QSaveFile_directWriteFallback<RetType> {
  fn directWriteFallback(self , rsthis: & QSaveFile) -> RetType;
}

  // proto:  bool QSaveFile::directWriteFallback();
impl<'a> /*trait*/ QSaveFile_directWriteFallback<i8> for () {
  fn directWriteFallback(self , rsthis: & QSaveFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSaveFile19directWriteFallbackEv()};
    let mut ret = unsafe {_ZNK9QSaveFile19directWriteFallbackEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QSaveFile::setDirectWriteFallback(bool enabled);
impl /*struct*/ QSaveFile {
  pub fn setDirectWriteFallback<RetType, T: QSaveFile_setDirectWriteFallback<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDirectWriteFallback(self);
    // return 1;
  }
}

pub trait QSaveFile_setDirectWriteFallback<RetType> {
  fn setDirectWriteFallback(self , rsthis: & QSaveFile) -> RetType;
}

  // proto:  void QSaveFile::setDirectWriteFallback(bool enabled);
impl<'a> /*trait*/ QSaveFile_setDirectWriteFallback<()> for (i8) {
  fn setDirectWriteFallback(self , rsthis: & QSaveFile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSaveFile22setDirectWriteFallbackEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QSaveFile22setDirectWriteFallbackEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

