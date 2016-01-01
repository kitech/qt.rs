// auto generated, do not modify.
// created: Fri Jan  1 12:13:41 2016
// src-file: /QtCore/qtemporaryfile.h
// dst-file: /src/core/qtemporaryfile.rs
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
use super::qfile::QFile; // 773
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
  fn QTemporaryFile_Class_Size() -> c_int;
  // proto:  bool QTemporaryFile::autoRemove();
  fn _ZNK14QTemporaryFile10autoRemoveEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto: static QTemporaryFile * QTemporaryFile::createLocalFile(QFile & file);
  fn demth_ZN14QTemporaryFile15createLocalFileER5QFile(arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTemporaryFile::QTemporaryFile(const QString & templateName);
  fn dector_ZN14QTemporaryFileC1ERK7QString(arg0: *mut c_void) -> *mut c_void;
  fn _ZN14QTemporaryFileC1ERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTemporaryFile::QTemporaryFile();
  fn dector_ZN14QTemporaryFileC1Ev() -> *mut c_void;
  fn _ZN14QTemporaryFileC1Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QTemporaryFile::QTemporaryFile(QObject * parent);
  fn dector_ZN14QTemporaryFileC1EP7QObject(arg0: *mut c_void) -> *mut c_void;
  fn _ZN14QTemporaryFileC1EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTemporaryFile::~QTemporaryFile();
  fn _ZN14QTemporaryFileD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  const QMetaObject * QTemporaryFile::metaObject();
  fn _ZNK14QTemporaryFile10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QTemporaryFile::setAutoRemove(bool b);
  fn _ZN14QTemporaryFile13setAutoRemoveEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QString QTemporaryFile::fileName();
  fn _ZNK14QTemporaryFile8fileNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QTemporaryFile::fileTemplate();
  fn _ZNK14QTemporaryFile12fileTemplateEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static QTemporaryFile * QTemporaryFile::createNativeFile(const QString & fileName);
  fn demth_ZN14QTemporaryFile16createNativeFileERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QTemporaryFile::open();
  fn _ZN14QTemporaryFile4openEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto: static QTemporaryFile * QTemporaryFile::createLocalFile(const QString & fileName);
  fn demth_ZN14QTemporaryFile15createLocalFileERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto: static QTemporaryFile * QTemporaryFile::createNativeFile(QFile & file);
  fn _ZN14QTemporaryFile16createNativeFileER5QFile(arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTemporaryFile::setFileTemplate(const QString & name);
  fn _ZN14QTemporaryFile15setFileTemplateERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTemporaryFile::QTemporaryFile(const QTemporaryFile & );
  fn dector_ZN14QTemporaryFileC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN14QTemporaryFileC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTemporaryFile::QTemporaryFile(const QString & templateName, QObject * parent);
  fn dector_ZN14QTemporaryFileC1ERK7QStringP7QObject(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  fn _ZN14QTemporaryFileC1ERK7QStringP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QTemporaryFile)=1
#[derive(Default)]
pub struct QTemporaryFile {
  qbase: QFile,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QTemporaryFile {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTemporaryFile {
    return QTemporaryFile{qbase: QFile::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QTemporaryFile {
  type Target = QFile;

  fn deref(&self) -> &QFile {
    return & self.qbase;
  }
}
impl AsRef<QFile> for QTemporaryFile {
  fn as_ref(& self) -> & QFile {
    return & self.qbase;
  }
}
  // proto:  bool QTemporaryFile::autoRemove();
impl /*struct*/ QTemporaryFile {
  pub fn autoRemove<RetType, T: QTemporaryFile_autoRemove<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.autoRemove(self);
    // return 1;
  }
}

pub trait QTemporaryFile_autoRemove<RetType> {
  fn autoRemove(self , rsthis: & QTemporaryFile) -> RetType;
}

  // proto:  bool QTemporaryFile::autoRemove();
impl<'a> /*trait*/ QTemporaryFile_autoRemove<i8> for () {
  fn autoRemove(self , rsthis: & QTemporaryFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTemporaryFile10autoRemoveEv()};
    let mut ret = unsafe {_ZNK14QTemporaryFile10autoRemoveEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static QTemporaryFile * QTemporaryFile::createLocalFile(QFile & file);
impl /*struct*/ QTemporaryFile {
  pub fn createLocalFile_s<RetType, T: QTemporaryFile_createLocalFile_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.createLocalFile_s();
    // return 1;
  }
}

pub trait QTemporaryFile_createLocalFile_s<RetType> {
  fn createLocalFile_s(self ) -> RetType;
}

  // proto: static QTemporaryFile * QTemporaryFile::createLocalFile(QFile & file);
impl<'a> /*trait*/ QTemporaryFile_createLocalFile_s<QTemporaryFile> for (&'a QFile) {
  fn createLocalFile_s(self ) -> QTemporaryFile {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTemporaryFile15createLocalFileER5QFile()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {demth_ZN14QTemporaryFile15createLocalFileER5QFile(arg0)};
    let mut ret1 = QTemporaryFile::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTemporaryFile::QTemporaryFile(const QString & templateName);
impl /*struct*/ QTemporaryFile {
  pub fn new<T: QTemporaryFile_new>(value: T) -> QTemporaryFile {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QTemporaryFile_new {
  fn new(self) -> QTemporaryFile;
}

  // proto:  void QTemporaryFile::QTemporaryFile(const QString & templateName);
impl<'a> /*trait*/ QTemporaryFile_new for (&'a QString) {
  fn new(self) -> QTemporaryFile {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTemporaryFileC1ERK7QString()};
    let ctysz: c_int = unsafe{QTemporaryFile_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN14QTemporaryFileC1ERK7QString(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN14QTemporaryFileC1ERK7QString(arg0)} as u64;
    let rsthis = QTemporaryFile{qbase: QFile::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTemporaryFile::QTemporaryFile();
impl<'a> /*trait*/ QTemporaryFile_new for () {
  fn new(self) -> QTemporaryFile {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTemporaryFileC1Ev()};
    let ctysz: c_int = unsafe{QTemporaryFile_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN14QTemporaryFileC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN14QTemporaryFileC1Ev()} as u64;
    let rsthis = QTemporaryFile{qbase: QFile::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTemporaryFile::QTemporaryFile(QObject * parent);
impl<'a> /*trait*/ QTemporaryFile_new for (&'a QObject) {
  fn new(self) -> QTemporaryFile {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTemporaryFileC1EP7QObject()};
    let ctysz: c_int = unsafe{QTemporaryFile_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN14QTemporaryFileC1EP7QObject(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN14QTemporaryFileC1EP7QObject(arg0)} as u64;
    let rsthis = QTemporaryFile{qbase: QFile::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTemporaryFile::~QTemporaryFile();
impl /*struct*/ QTemporaryFile {
  pub fn free<RetType, T: QTemporaryFile_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QTemporaryFile_free<RetType> {
  fn free(self , rsthis: & QTemporaryFile) -> RetType;
}

  // proto:  void QTemporaryFile::~QTemporaryFile();
impl<'a> /*trait*/ QTemporaryFile_free<()> for () {
  fn free(self , rsthis: & QTemporaryFile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTemporaryFileD0Ev()};
     unsafe {_ZN14QTemporaryFileD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QTemporaryFile::metaObject();
impl /*struct*/ QTemporaryFile {
  pub fn metaObject<RetType, T: QTemporaryFile_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QTemporaryFile_metaObject<RetType> {
  fn metaObject(self , rsthis: & QTemporaryFile) -> RetType;
}

  // proto:  const QMetaObject * QTemporaryFile::metaObject();
impl<'a> /*trait*/ QTemporaryFile_metaObject<()> for () {
  fn metaObject(self , rsthis: & QTemporaryFile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTemporaryFile10metaObjectEv()};
     unsafe {_ZNK14QTemporaryFile10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTemporaryFile::setAutoRemove(bool b);
impl /*struct*/ QTemporaryFile {
  pub fn setAutoRemove<RetType, T: QTemporaryFile_setAutoRemove<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAutoRemove(self);
    // return 1;
  }
}

pub trait QTemporaryFile_setAutoRemove<RetType> {
  fn setAutoRemove(self , rsthis: & QTemporaryFile) -> RetType;
}

  // proto:  void QTemporaryFile::setAutoRemove(bool b);
impl<'a> /*trait*/ QTemporaryFile_setAutoRemove<()> for (i8) {
  fn setAutoRemove(self , rsthis: & QTemporaryFile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTemporaryFile13setAutoRemoveEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN14QTemporaryFile13setAutoRemoveEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QTemporaryFile::fileName();
impl /*struct*/ QTemporaryFile {
  pub fn fileName<RetType, T: QTemporaryFile_fileName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fileName(self);
    // return 1;
  }
}

pub trait QTemporaryFile_fileName<RetType> {
  fn fileName(self , rsthis: & QTemporaryFile) -> RetType;
}

  // proto:  QString QTemporaryFile::fileName();
impl<'a> /*trait*/ QTemporaryFile_fileName<QString> for () {
  fn fileName(self , rsthis: & QTemporaryFile) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTemporaryFile8fileNameEv()};
    let mut ret = unsafe {_ZNK14QTemporaryFile8fileNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QTemporaryFile::fileTemplate();
impl /*struct*/ QTemporaryFile {
  pub fn fileTemplate<RetType, T: QTemporaryFile_fileTemplate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fileTemplate(self);
    // return 1;
  }
}

pub trait QTemporaryFile_fileTemplate<RetType> {
  fn fileTemplate(self , rsthis: & QTemporaryFile) -> RetType;
}

  // proto:  QString QTemporaryFile::fileTemplate();
impl<'a> /*trait*/ QTemporaryFile_fileTemplate<QString> for () {
  fn fileTemplate(self , rsthis: & QTemporaryFile) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTemporaryFile12fileTemplateEv()};
    let mut ret = unsafe {_ZNK14QTemporaryFile12fileTemplateEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QTemporaryFile * QTemporaryFile::createNativeFile(const QString & fileName);
impl /*struct*/ QTemporaryFile {
  pub fn createNativeFile_s<RetType, T: QTemporaryFile_createNativeFile_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.createNativeFile_s();
    // return 1;
  }
}

pub trait QTemporaryFile_createNativeFile_s<RetType> {
  fn createNativeFile_s(self ) -> RetType;
}

  // proto: static QTemporaryFile * QTemporaryFile::createNativeFile(const QString & fileName);
impl<'a> /*trait*/ QTemporaryFile_createNativeFile_s<QTemporaryFile> for (&'a QString) {
  fn createNativeFile_s(self ) -> QTemporaryFile {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTemporaryFile16createNativeFileERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {demth_ZN14QTemporaryFile16createNativeFileERK7QString(arg0)};
    let mut ret1 = QTemporaryFile::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QTemporaryFile::open();
impl /*struct*/ QTemporaryFile {
  pub fn open<RetType, T: QTemporaryFile_open<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.open(self);
    // return 1;
  }
}

pub trait QTemporaryFile_open<RetType> {
  fn open(self , rsthis: & QTemporaryFile) -> RetType;
}

  // proto:  bool QTemporaryFile::open();
impl<'a> /*trait*/ QTemporaryFile_open<i8> for () {
  fn open(self , rsthis: & QTemporaryFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTemporaryFile4openEv()};
    let mut ret = unsafe {_ZN14QTemporaryFile4openEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static QTemporaryFile * QTemporaryFile::createLocalFile(const QString & fileName);
impl<'a> /*trait*/ QTemporaryFile_createLocalFile_s<QTemporaryFile> for (&'a QString) {
  fn createLocalFile_s(self ) -> QTemporaryFile {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTemporaryFile15createLocalFileERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {demth_ZN14QTemporaryFile15createLocalFileERK7QString(arg0)};
    let mut ret1 = QTemporaryFile::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QTemporaryFile * QTemporaryFile::createNativeFile(QFile & file);
impl<'a> /*trait*/ QTemporaryFile_createNativeFile_s<QTemporaryFile> for (&'a QFile) {
  fn createNativeFile_s(self ) -> QTemporaryFile {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTemporaryFile16createNativeFileER5QFile()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN14QTemporaryFile16createNativeFileER5QFile(arg0)};
    let mut ret1 = QTemporaryFile::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTemporaryFile::setFileTemplate(const QString & name);
impl /*struct*/ QTemporaryFile {
  pub fn setFileTemplate<RetType, T: QTemporaryFile_setFileTemplate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFileTemplate(self);
    // return 1;
  }
}

pub trait QTemporaryFile_setFileTemplate<RetType> {
  fn setFileTemplate(self , rsthis: & QTemporaryFile) -> RetType;
}

  // proto:  void QTemporaryFile::setFileTemplate(const QString & name);
impl<'a> /*trait*/ QTemporaryFile_setFileTemplate<()> for (&'a QString) {
  fn setFileTemplate(self , rsthis: & QTemporaryFile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTemporaryFile15setFileTemplateERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QTemporaryFile15setFileTemplateERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTemporaryFile::QTemporaryFile(const QTemporaryFile & );
impl<'a> /*trait*/ QTemporaryFile_new for (&'a QTemporaryFile) {
  fn new(self) -> QTemporaryFile {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTemporaryFileC1ERKS_()};
    let ctysz: c_int = unsafe{QTemporaryFile_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN14QTemporaryFileC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN14QTemporaryFileC1ERKS_(arg0)} as u64;
    let rsthis = QTemporaryFile{qbase: QFile::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTemporaryFile::QTemporaryFile(const QString & templateName, QObject * parent);
impl<'a> /*trait*/ QTemporaryFile_new for (&'a QString, &'a QObject) {
  fn new(self) -> QTemporaryFile {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTemporaryFileC1ERK7QStringP7QObject()};
    let ctysz: c_int = unsafe{QTemporaryFile_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    // unsafe {_ZN14QTemporaryFileC1ERK7QStringP7QObject(qthis, arg0, arg1)};
    let qthis: u64 = unsafe {dector_ZN14QTemporaryFileC1ERK7QStringP7QObject(arg0, arg1)} as u64;
    let rsthis = QTemporaryFile{qbase: QFile::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

