// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQml/qqmlfile.h
// dst-file: /src/qml/qqmlfile.rs
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
use super::super::core::qobject::QObject; // 771
use super::qqmlengine::QQmlEngine; // 773
use super::super::core::qurl::QUrl; // 771
use super::super::core::qstring::QString; // 771
use super::super::core::qbytearray::QByteArray; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QQmlFile_Class_Size() -> c_int;
  // proto:  bool QQmlFile::connectDownloadProgress(QObject * , const char * );
  fn _ZN8QQmlFile23connectDownloadProgressEP7QObjectPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_char) -> c_char;
  // proto:  void QQmlFile::clear(QObject * );
  fn _ZN8QQmlFile5clearEP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQmlFile::QQmlFile(QQmlEngine * , const QUrl & );
  fn _ZN8QQmlFileC2EP10QQmlEngineRK4QUrl(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  qint64 QQmlFile::size();
  fn _ZNK8QQmlFile4sizeEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  bool QQmlFile::connectDownloadProgress(QObject * , int );
  fn _ZN8QQmlFile23connectDownloadProgressEP7QObjecti(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int) -> c_char;
  // proto:  bool QQmlFile::isNull();
  fn _ZNK8QQmlFile6isNullEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto: static bool QQmlFile::isLocalFile(const QUrl & url);
  fn _ZN8QQmlFile11isLocalFileERK4QUrl(arg0: *mut c_void) -> c_char;
  // proto:  void QQmlFile::load(QQmlEngine * , const QString & );
  fn _ZN8QQmlFile4loadEP10QQmlEngineRK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QUrl QQmlFile::url();
  fn _ZNK8QQmlFile3urlEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QQmlFile::error();
  fn _ZNK8QQmlFile5errorEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QQmlFile::clear();
  fn _ZN8QQmlFile5clearEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QQmlFile::isError();
  fn _ZNK8QQmlFile7isErrorEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto: static QString QQmlFile::urlToLocalFileOrQrc(const QString & );
  fn _ZN8QQmlFile19urlToLocalFileOrQrcERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto:  QByteArray QQmlFile::dataByteArray();
  fn _ZNK8QQmlFile13dataByteArrayEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static bool QQmlFile::isLocalFile(const QString & url);
  fn _ZN8QQmlFile11isLocalFileERK7QString(arg0: *mut c_void) -> c_char;
  // proto:  bool QQmlFile::isReady();
  fn _ZNK8QQmlFile7isReadyEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QQmlFile::isLoading();
  fn _ZNK8QQmlFile9isLoadingEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QQmlFile::QQmlFile();
  fn _ZN8QQmlFileC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QQmlFile::connectFinished(QObject * , const char * );
  fn _ZN8QQmlFile15connectFinishedEP7QObjectPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_char) -> c_char;
  // proto: static bool QQmlFile::isSynchronous(const QString & url);
  fn _ZN8QQmlFile13isSynchronousERK7QString(arg0: *mut c_void) -> c_char;
  // proto:  void QQmlFile::load(QQmlEngine * , const QUrl & );
  fn _ZN8QQmlFile4loadEP10QQmlEngineRK4QUrl(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto: static bool QQmlFile::isSynchronous(const QUrl & url);
  fn _ZN8QQmlFile13isSynchronousERK4QUrl(arg0: *mut c_void) -> c_char;
  // proto:  bool QQmlFile::connectFinished(QObject * , int );
  fn _ZN8QQmlFile15connectFinishedEP7QObjecti(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int) -> c_char;
  // proto: static QString QQmlFile::urlToLocalFileOrQrc(const QUrl & );
  fn _ZN8QQmlFile19urlToLocalFileOrQrcERK4QUrl(arg0: *mut c_void) -> *mut c_void;
  // proto:  void QQmlFile::QQmlFile(const QQmlFile & );
  fn _ZN8QQmlFileC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQmlFile::QQmlFile(QQmlEngine * , const QString & );
  fn _ZN8QQmlFileC2EP10QQmlEngineRK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  const char * QQmlFile::data();
  fn _ZNK8QQmlFile4dataEv(qthis: u64 /* *mut c_void*/) -> *mut c_char;
  // proto:  void QQmlFile::~QQmlFile();
  fn _ZN8QQmlFileD2Ev(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QQmlFile)=8
#[derive(Default)]
pub struct QQmlFile {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QQmlFile {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QQmlFile {
    return QQmlFile{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  bool QQmlFile::connectDownloadProgress(QObject * , const char * );
impl /*struct*/ QQmlFile {
  pub fn connectDownloadProgress<RetType, T: QQmlFile_connectDownloadProgress<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.connectDownloadProgress(self);
    // return 1;
  }
}

pub trait QQmlFile_connectDownloadProgress<RetType> {
  fn connectDownloadProgress(self , rsthis: & QQmlFile) -> RetType;
}

  // proto:  bool QQmlFile::connectDownloadProgress(QObject * , const char * );
impl<'a> /*trait*/ QQmlFile_connectDownloadProgress<i8> for (&'a QObject, &'a  String) {
  fn connectDownloadProgress(self , rsthis: & QQmlFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QQmlFile23connectDownloadProgressEP7QObjectPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN8QQmlFile23connectDownloadProgressEP7QObjectPKc(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QQmlFile::clear(QObject * );
impl /*struct*/ QQmlFile {
  pub fn clear<RetType, T: QQmlFile_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QQmlFile_clear<RetType> {
  fn clear(self , rsthis: & QQmlFile) -> RetType;
}

  // proto:  void QQmlFile::clear(QObject * );
impl<'a> /*trait*/ QQmlFile_clear<()> for (&'a QObject) {
  fn clear(self , rsthis: & QQmlFile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QQmlFile5clearEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QQmlFile5clearEP7QObject(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQmlFile::QQmlFile(QQmlEngine * , const QUrl & );
impl /*struct*/ QQmlFile {
  pub fn new<T: QQmlFile_new>(value: T) -> QQmlFile {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QQmlFile_new {
  fn new(self) -> QQmlFile;
}

  // proto:  void QQmlFile::QQmlFile(QQmlEngine * , const QUrl & );
impl<'a> /*trait*/ QQmlFile_new for (&'a QQmlEngine, &'a QUrl) {
  fn new(self) -> QQmlFile {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QQmlFileC2EP10QQmlEngineRK4QUrl()};
    let ctysz: c_int = unsafe{QQmlFile_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN8QQmlFileC2EP10QQmlEngineRK4QUrl(qthis_ph, arg0, arg1)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlFile{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  qint64 QQmlFile::size();
impl /*struct*/ QQmlFile {
  pub fn size<RetType, T: QQmlFile_size<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QQmlFile_size<RetType> {
  fn size(self , rsthis: & QQmlFile) -> RetType;
}

  // proto:  qint64 QQmlFile::size();
impl<'a> /*trait*/ QQmlFile_size<i64> for () {
  fn size(self , rsthis: & QQmlFile) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QQmlFile4sizeEv()};
    let mut ret = unsafe {_ZNK8QQmlFile4sizeEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  bool QQmlFile::connectDownloadProgress(QObject * , int );
impl<'a> /*trait*/ QQmlFile_connectDownloadProgress<i8> for (&'a QObject, i32) {
  fn connectDownloadProgress(self , rsthis: & QQmlFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QQmlFile23connectDownloadProgressEP7QObjecti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN8QQmlFile23connectDownloadProgressEP7QObjecti(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QQmlFile::isNull();
impl /*struct*/ QQmlFile {
  pub fn isNull<RetType, T: QQmlFile_isNull<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QQmlFile_isNull<RetType> {
  fn isNull(self , rsthis: & QQmlFile) -> RetType;
}

  // proto:  bool QQmlFile::isNull();
impl<'a> /*trait*/ QQmlFile_isNull<i8> for () {
  fn isNull(self , rsthis: & QQmlFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QQmlFile6isNullEv()};
    let mut ret = unsafe {_ZNK8QQmlFile6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static bool QQmlFile::isLocalFile(const QUrl & url);
impl /*struct*/ QQmlFile {
  pub fn isLocalFile_s<RetType, T: QQmlFile_isLocalFile_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.isLocalFile_s();
    // return 1;
  }
}

pub trait QQmlFile_isLocalFile_s<RetType> {
  fn isLocalFile_s(self ) -> RetType;
}

  // proto: static bool QQmlFile::isLocalFile(const QUrl & url);
impl<'a> /*trait*/ QQmlFile_isLocalFile_s<i8> for (&'a QUrl) {
  fn isLocalFile_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QQmlFile11isLocalFileERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QQmlFile11isLocalFileERK4QUrl(arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QQmlFile::load(QQmlEngine * , const QString & );
impl /*struct*/ QQmlFile {
  pub fn load<RetType, T: QQmlFile_load<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.load(self);
    // return 1;
  }
}

pub trait QQmlFile_load<RetType> {
  fn load(self , rsthis: & QQmlFile) -> RetType;
}

  // proto:  void QQmlFile::load(QQmlEngine * , const QString & );
impl<'a> /*trait*/ QQmlFile_load<()> for (&'a QQmlEngine, &'a QString) {
  fn load(self , rsthis: & QQmlFile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QQmlFile4loadEP10QQmlEngineRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QQmlFile4loadEP10QQmlEngineRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QUrl QQmlFile::url();
impl /*struct*/ QQmlFile {
  pub fn url<RetType, T: QQmlFile_url<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.url(self);
    // return 1;
  }
}

pub trait QQmlFile_url<RetType> {
  fn url(self , rsthis: & QQmlFile) -> RetType;
}

  // proto:  QUrl QQmlFile::url();
impl<'a> /*trait*/ QQmlFile_url<QUrl> for () {
  fn url(self , rsthis: & QQmlFile) -> QUrl {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QQmlFile3urlEv()};
    let mut ret = unsafe {_ZNK8QQmlFile3urlEv(rsthis.qclsinst)};
    let mut ret1 = QUrl::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QQmlFile::error();
impl /*struct*/ QQmlFile {
  pub fn error<RetType, T: QQmlFile_error<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.error(self);
    // return 1;
  }
}

pub trait QQmlFile_error<RetType> {
  fn error(self , rsthis: & QQmlFile) -> RetType;
}

  // proto:  QString QQmlFile::error();
impl<'a> /*trait*/ QQmlFile_error<QString> for () {
  fn error(self , rsthis: & QQmlFile) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QQmlFile5errorEv()};
    let mut ret = unsafe {_ZNK8QQmlFile5errorEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQmlFile::clear();
impl<'a> /*trait*/ QQmlFile_clear<()> for () {
  fn clear(self , rsthis: & QQmlFile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QQmlFile5clearEv()};
     unsafe {_ZN8QQmlFile5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QQmlFile::isError();
impl /*struct*/ QQmlFile {
  pub fn isError<RetType, T: QQmlFile_isError<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isError(self);
    // return 1;
  }
}

pub trait QQmlFile_isError<RetType> {
  fn isError(self , rsthis: & QQmlFile) -> RetType;
}

  // proto:  bool QQmlFile::isError();
impl<'a> /*trait*/ QQmlFile_isError<i8> for () {
  fn isError(self , rsthis: & QQmlFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QQmlFile7isErrorEv()};
    let mut ret = unsafe {_ZNK8QQmlFile7isErrorEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static QString QQmlFile::urlToLocalFileOrQrc(const QString & );
impl /*struct*/ QQmlFile {
  pub fn urlToLocalFileOrQrc_s<RetType, T: QQmlFile_urlToLocalFileOrQrc_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.urlToLocalFileOrQrc_s();
    // return 1;
  }
}

pub trait QQmlFile_urlToLocalFileOrQrc_s<RetType> {
  fn urlToLocalFileOrQrc_s(self ) -> RetType;
}

  // proto: static QString QQmlFile::urlToLocalFileOrQrc(const QString & );
impl<'a> /*trait*/ QQmlFile_urlToLocalFileOrQrc_s<QString> for (&'a QString) {
  fn urlToLocalFileOrQrc_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QQmlFile19urlToLocalFileOrQrcERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QQmlFile19urlToLocalFileOrQrcERK7QString(arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray QQmlFile::dataByteArray();
impl /*struct*/ QQmlFile {
  pub fn dataByteArray<RetType, T: QQmlFile_dataByteArray<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.dataByteArray(self);
    // return 1;
  }
}

pub trait QQmlFile_dataByteArray<RetType> {
  fn dataByteArray(self , rsthis: & QQmlFile) -> RetType;
}

  // proto:  QByteArray QQmlFile::dataByteArray();
impl<'a> /*trait*/ QQmlFile_dataByteArray<QByteArray> for () {
  fn dataByteArray(self , rsthis: & QQmlFile) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QQmlFile13dataByteArrayEv()};
    let mut ret = unsafe {_ZNK8QQmlFile13dataByteArrayEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static bool QQmlFile::isLocalFile(const QString & url);
impl<'a> /*trait*/ QQmlFile_isLocalFile_s<i8> for (&'a QString) {
  fn isLocalFile_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QQmlFile11isLocalFileERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QQmlFile11isLocalFileERK7QString(arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QQmlFile::isReady();
impl /*struct*/ QQmlFile {
  pub fn isReady<RetType, T: QQmlFile_isReady<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isReady(self);
    // return 1;
  }
}

pub trait QQmlFile_isReady<RetType> {
  fn isReady(self , rsthis: & QQmlFile) -> RetType;
}

  // proto:  bool QQmlFile::isReady();
impl<'a> /*trait*/ QQmlFile_isReady<i8> for () {
  fn isReady(self , rsthis: & QQmlFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QQmlFile7isReadyEv()};
    let mut ret = unsafe {_ZNK8QQmlFile7isReadyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QQmlFile::isLoading();
impl /*struct*/ QQmlFile {
  pub fn isLoading<RetType, T: QQmlFile_isLoading<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isLoading(self);
    // return 1;
  }
}

pub trait QQmlFile_isLoading<RetType> {
  fn isLoading(self , rsthis: & QQmlFile) -> RetType;
}

  // proto:  bool QQmlFile::isLoading();
impl<'a> /*trait*/ QQmlFile_isLoading<i8> for () {
  fn isLoading(self , rsthis: & QQmlFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QQmlFile9isLoadingEv()};
    let mut ret = unsafe {_ZNK8QQmlFile9isLoadingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QQmlFile::QQmlFile();
impl<'a> /*trait*/ QQmlFile_new for () {
  fn new(self) -> QQmlFile {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QQmlFileC2Ev()};
    let ctysz: c_int = unsafe{QQmlFile_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN8QQmlFileC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlFile{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QQmlFile::connectFinished(QObject * , const char * );
impl /*struct*/ QQmlFile {
  pub fn connectFinished<RetType, T: QQmlFile_connectFinished<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.connectFinished(self);
    // return 1;
  }
}

pub trait QQmlFile_connectFinished<RetType> {
  fn connectFinished(self , rsthis: & QQmlFile) -> RetType;
}

  // proto:  bool QQmlFile::connectFinished(QObject * , const char * );
impl<'a> /*trait*/ QQmlFile_connectFinished<i8> for (&'a QObject, &'a  String) {
  fn connectFinished(self , rsthis: & QQmlFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QQmlFile15connectFinishedEP7QObjectPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN8QQmlFile15connectFinishedEP7QObjectPKc(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static bool QQmlFile::isSynchronous(const QString & url);
impl /*struct*/ QQmlFile {
  pub fn isSynchronous_s<RetType, T: QQmlFile_isSynchronous_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.isSynchronous_s();
    // return 1;
  }
}

pub trait QQmlFile_isSynchronous_s<RetType> {
  fn isSynchronous_s(self ) -> RetType;
}

  // proto: static bool QQmlFile::isSynchronous(const QString & url);
impl<'a> /*trait*/ QQmlFile_isSynchronous_s<i8> for (&'a QString) {
  fn isSynchronous_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QQmlFile13isSynchronousERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QQmlFile13isSynchronousERK7QString(arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QQmlFile::load(QQmlEngine * , const QUrl & );
impl<'a> /*trait*/ QQmlFile_load<()> for (&'a QQmlEngine, &'a QUrl) {
  fn load(self , rsthis: & QQmlFile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QQmlFile4loadEP10QQmlEngineRK4QUrl()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QQmlFile4loadEP10QQmlEngineRK4QUrl(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto: static bool QQmlFile::isSynchronous(const QUrl & url);
impl<'a> /*trait*/ QQmlFile_isSynchronous_s<i8> for (&'a QUrl) {
  fn isSynchronous_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QQmlFile13isSynchronousERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QQmlFile13isSynchronousERK4QUrl(arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QQmlFile::connectFinished(QObject * , int );
impl<'a> /*trait*/ QQmlFile_connectFinished<i8> for (&'a QObject, i32) {
  fn connectFinished(self , rsthis: & QQmlFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QQmlFile15connectFinishedEP7QObjecti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN8QQmlFile15connectFinishedEP7QObjecti(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static QString QQmlFile::urlToLocalFileOrQrc(const QUrl & );
impl<'a> /*trait*/ QQmlFile_urlToLocalFileOrQrc_s<QString> for (&'a QUrl) {
  fn urlToLocalFileOrQrc_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QQmlFile19urlToLocalFileOrQrcERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QQmlFile19urlToLocalFileOrQrcERK4QUrl(arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQmlFile::QQmlFile(const QQmlFile & );
impl<'a> /*trait*/ QQmlFile_new for (&'a QQmlFile) {
  fn new(self) -> QQmlFile {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QQmlFileC2ERKS_()};
    let ctysz: c_int = unsafe{QQmlFile_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QQmlFileC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlFile{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QQmlFile::QQmlFile(QQmlEngine * , const QString & );
impl<'a> /*trait*/ QQmlFile_new for (&'a QQmlEngine, &'a QString) {
  fn new(self) -> QQmlFile {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QQmlFileC2EP10QQmlEngineRK7QString()};
    let ctysz: c_int = unsafe{QQmlFile_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN8QQmlFileC2EP10QQmlEngineRK7QString(qthis_ph, arg0, arg1)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlFile{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const char * QQmlFile::data();
impl /*struct*/ QQmlFile {
  pub fn data<RetType, T: QQmlFile_data<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QQmlFile_data<RetType> {
  fn data(self , rsthis: & QQmlFile) -> RetType;
}

  // proto:  const char * QQmlFile::data();
impl<'a> /*trait*/ QQmlFile_data<String> for () {
  fn data(self , rsthis: & QQmlFile) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QQmlFile4dataEv()};
    let mut ret = unsafe {_ZNK8QQmlFile4dataEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto:  void QQmlFile::~QQmlFile();
impl /*struct*/ QQmlFile {
  pub fn free<RetType, T: QQmlFile_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QQmlFile_free<RetType> {
  fn free(self , rsthis: & QQmlFile) -> RetType;
}

  // proto:  void QQmlFile::~QQmlFile();
impl<'a> /*trait*/ QQmlFile_free<()> for () {
  fn free(self , rsthis: & QQmlFile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QQmlFileD2Ev()};
     unsafe {_ZN8QQmlFileD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

