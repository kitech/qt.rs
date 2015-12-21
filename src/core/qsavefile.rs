// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
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
use super::qobject::QObject; // 773
use super::qstring::QString; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QSaveFile::cancelWriting();
  fn _ZN9QSaveFile13cancelWritingEv(qthis: *mut c_void);
  // proto:  void QSaveFile::QSaveFile(QObject * parent);
  fn _ZN9QSaveFileC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QSaveFile::QSaveFile(const QSaveFile & );
  fn _ZN9QSaveFileC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QSaveFile::QSaveFile(const QString & name, QObject * parent);
  fn _ZN9QSaveFileC1ERK7QStringP7QObject(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QString QSaveFile::fileName();
  fn _ZNK9QSaveFile8fileNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QSaveFile::QSaveFile(const QString & name);
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
  pub qclsinst: *mut c_void,
}

  // proto:  void QSaveFile::cancelWriting();
impl /*struct*/ QSaveFile {
  pub fn cancelWriting<RetType, T: QSaveFile_cancelWriting<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.cancelWriting(self);
    // return 1;
  }
}

pub trait QSaveFile_cancelWriting<RetType> {
  fn cancelWriting(self , rsthis: &mut QSaveFile) -> RetType;
}

  // proto:  void QSaveFile::cancelWriting();
impl<'a> /*trait*/ QSaveFile_cancelWriting<()> for () {
  fn cancelWriting(self , rsthis: &mut QSaveFile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSaveFile13cancelWritingEv()};
     unsafe {_ZN9QSaveFile13cancelWritingEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSaveFile::QSaveFile(QObject * parent);
impl /*struct*/ QSaveFile {
  pub fn NewQSaveFile<T: QSaveFile_NewQSaveFile>(value: T) -> QSaveFile {
    let rsthis = value.NewQSaveFile();
    return rsthis;
    // return 1;
  }
}

pub trait QSaveFile_NewQSaveFile {
  fn NewQSaveFile(self) -> QSaveFile;
}

  // proto:  void QSaveFile::QSaveFile(QObject * parent);
impl<'a> /*trait*/ QSaveFile_NewQSaveFile for (QObject) {
  fn NewQSaveFile(self) -> QSaveFile {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSaveFileC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QSaveFileC1EP7QObject(qthis, arg0)};
    let rsthis = QSaveFile{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSaveFile::QSaveFile(const QSaveFile & );
impl<'a> /*trait*/ QSaveFile_NewQSaveFile for (QSaveFile) {
  fn NewQSaveFile(self) -> QSaveFile {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSaveFileC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QSaveFileC1ERKS_(qthis, arg0)};
    let rsthis = QSaveFile{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSaveFile::QSaveFile(const QString & name, QObject * parent);
impl<'a> /*trait*/ QSaveFile_NewQSaveFile for (QString, QObject) {
  fn NewQSaveFile(self) -> QSaveFile {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSaveFileC1ERK7QStringP7QObject()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN9QSaveFileC1ERK7QStringP7QObject(qthis, arg0, arg1)};
    let rsthis = QSaveFile{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QSaveFile::fileName();
impl /*struct*/ QSaveFile {
  pub fn fileName<RetType, T: QSaveFile_fileName<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.fileName(self);
    // return 1;
  }
}

pub trait QSaveFile_fileName<RetType> {
  fn fileName(self , rsthis: &mut QSaveFile) -> RetType;
}

  // proto:  QString QSaveFile::fileName();
impl<'a> /*trait*/ QSaveFile_fileName<QString> for () {
  fn fileName(self , rsthis: &mut QSaveFile) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSaveFile8fileNameEv()};
    let mut ret = unsafe {_ZNK9QSaveFile8fileNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QSaveFile::QSaveFile(const QString & name);
impl<'a> /*trait*/ QSaveFile_NewQSaveFile for (QString) {
  fn NewQSaveFile(self) -> QSaveFile {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSaveFileC1ERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QSaveFileC1ERK7QString(qthis, arg0)};
    let rsthis = QSaveFile{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QSaveFile::metaObject();
impl /*struct*/ QSaveFile {
  pub fn metaObject<RetType, T: QSaveFile_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QSaveFile_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QSaveFile) -> RetType;
}

  // proto:  const QMetaObject * QSaveFile::metaObject();
impl<'a> /*trait*/ QSaveFile_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QSaveFile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSaveFile10metaObjectEv()};
     unsafe {_ZNK9QSaveFile10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QSaveFile::commit();
impl /*struct*/ QSaveFile {
  pub fn commit<RetType, T: QSaveFile_commit<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.commit(self);
    // return 1;
  }
}

pub trait QSaveFile_commit<RetType> {
  fn commit(self , rsthis: &mut QSaveFile) -> RetType;
}

  // proto:  bool QSaveFile::commit();
impl<'a> /*trait*/ QSaveFile_commit<i8> for () {
  fn commit(self , rsthis: &mut QSaveFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSaveFile6commitEv()};
    let mut ret = unsafe {_ZN9QSaveFile6commitEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QSaveFile::~QSaveFile();
impl /*struct*/ QSaveFile {
  pub fn FreeQSaveFile<RetType, T: QSaveFile_FreeQSaveFile<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQSaveFile(self);
    // return 1;
  }
}

pub trait QSaveFile_FreeQSaveFile<RetType> {
  fn FreeQSaveFile(self , rsthis: &mut QSaveFile) -> RetType;
}

  // proto:  void QSaveFile::~QSaveFile();
impl<'a> /*trait*/ QSaveFile_FreeQSaveFile<()> for () {
  fn FreeQSaveFile(self , rsthis: &mut QSaveFile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSaveFileD0Ev()};
     unsafe {_ZN9QSaveFileD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSaveFile::setFileName(const QString & name);
impl /*struct*/ QSaveFile {
  pub fn setFileName<RetType, T: QSaveFile_setFileName<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFileName(self);
    // return 1;
  }
}

pub trait QSaveFile_setFileName<RetType> {
  fn setFileName(self , rsthis: &mut QSaveFile) -> RetType;
}

  // proto:  void QSaveFile::setFileName(const QString & name);
impl<'a> /*trait*/ QSaveFile_setFileName<()> for (QString) {
  fn setFileName(self , rsthis: &mut QSaveFile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSaveFile11setFileNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QSaveFile11setFileNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QSaveFile::directWriteFallback();
impl /*struct*/ QSaveFile {
  pub fn directWriteFallback<RetType, T: QSaveFile_directWriteFallback<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.directWriteFallback(self);
    // return 1;
  }
}

pub trait QSaveFile_directWriteFallback<RetType> {
  fn directWriteFallback(self , rsthis: &mut QSaveFile) -> RetType;
}

  // proto:  bool QSaveFile::directWriteFallback();
impl<'a> /*trait*/ QSaveFile_directWriteFallback<i8> for () {
  fn directWriteFallback(self , rsthis: &mut QSaveFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSaveFile19directWriteFallbackEv()};
    let mut ret = unsafe {_ZNK9QSaveFile19directWriteFallbackEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QSaveFile::setDirectWriteFallback(bool enabled);
impl /*struct*/ QSaveFile {
  pub fn setDirectWriteFallback<RetType, T: QSaveFile_setDirectWriteFallback<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDirectWriteFallback(self);
    // return 1;
  }
}

pub trait QSaveFile_setDirectWriteFallback<RetType> {
  fn setDirectWriteFallback(self , rsthis: &mut QSaveFile) -> RetType;
}

  // proto:  void QSaveFile::setDirectWriteFallback(bool enabled);
impl<'a> /*trait*/ QSaveFile_setDirectWriteFallback<()> for (i8) {
  fn setDirectWriteFallback(self , rsthis: &mut QSaveFile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSaveFile22setDirectWriteFallbackEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QSaveFile22setDirectWriteFallbackEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

