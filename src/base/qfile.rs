// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qobject::QObject;
use super::qbytearray::QByteArray;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QString QFile::symLinkTarget();
  fn _ZNK5QFile13symLinkTargetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QFile::NewQFile();
  fn _ZN5QFileC1Ev(qthis: *mut c_void) ;
  // proto:  void QFile::NewQFile(QObject * parent);
  fn _ZN5QFileC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static bool QFile::link(const QString & oldname, const QString & newName);
  fn _ZN5QFile4linkERK7QStringS2_(arg0: *mut c_void, arg1: *mut c_void) -> int8_t;
  // proto: static bool QFile::rename(const QString & oldName, const QString & newName);
  fn _ZN5QFile6renameERK7QStringS2_(arg0: *mut c_void, arg1: *mut c_void) -> int8_t;
  // proto:  bool QFile::link(const QString & newName);
  fn _ZN5QFile4linkERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto: static bool QFile::resize(const QString & filename, qint64 sz);
  fn _ZN5QFile6resizeERK7QStringx(arg0: *mut c_void, arg1: c_longlong) -> int8_t;
  // proto: static bool QFile::exists(const QString & fileName);
  fn _ZN5QFile6existsERK7QString(arg0: *mut c_void) -> int8_t;
  // proto:  void QFile::FreeQFile();
  fn _ZN5QFileD0Ev(qthis: *mut c_void) ;
  // proto: static bool QFile::copy(const QString & fileName, const QString & newName);
  fn _ZN5QFile4copyERK7QStringS2_(arg0: *mut c_void, arg1: *mut c_void) -> int8_t;
  // proto: static QString QFile::readLink(const QString & fileName);
  fn _ZN5QFile8readLinkERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QFile::exists();
  fn _ZNK5QFile6existsEv(qthis: *mut c_void) -> int8_t;
  // proto:  long long QFile::size();
  fn _ZNK5QFile4sizeEv(qthis: *mut c_void) -> c_longlong;
  // proto:  bool QFile::resize(qint64 sz);
  fn _ZN5QFile6resizeEx(qthis: *mut c_void, arg0: c_longlong) -> int8_t;
  // proto:  void QFile::NewQFile(const QFile & );
  fn _ZN5QFileC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QFile::setFileName(const QString & name);
  fn _ZN5QFile11setFileNameERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QFile::remove();
  fn _ZN5QFile6removeEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QFile::copy(const QString & newName);
  fn _ZN5QFile4copyERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto: static QByteArray QFile::encodeName(const QString & fileName);
  fn _ZN5QFile10encodeNameERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto: static QString QFile::decodeName(const QByteArray & localFileName);
  fn _ZN5QFile10decodeNameERK10QByteArray(arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QFile::rename(const QString & newName);
  fn _ZN5QFile6renameERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QString QFile::fileName();
  fn _ZNK5QFile8fileNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static QString QFile::decodeName(const char * localFileName);
  fn _ZN5QFile10decodeNameEPKc(arg0: *const c_char) -> *mut c_void;
  // proto:  const QMetaObject * QFile::metaObject();
  fn _ZNK5QFile10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QFile::NewQFile(const QString & name, QObject * parent);
  fn _ZN5QFileC1ERK7QStringP7QObject(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto: static QString QFile::symLinkTarget(const QString & fileName);
  fn _ZN5QFile13symLinkTargetERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto: static bool QFile::remove(const QString & fileName);
  fn _ZN5QFile6removeERK7QString(arg0: *mut c_void) -> int8_t;
  // proto:  void QFile::NewQFile(const QString & name);
  fn _ZN5QFileC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QFile::readLink();
  fn _ZNK5QFile8readLinkEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QFile)=1
pub struct QFile {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFile {
  pub fn symLinkTarget<RetType, T: QFile_symLinkTarget<RetType>>(&mut self, value: T) -> RetType {
    return value.symLinkTarget(self);
    // return 1;
  }
}

pub trait QFile_symLinkTarget<RetType> {
  fn symLinkTarget(self, rsthis: &mut QFile) -> RetType;
}

// proto:  QString QFile::symLinkTarget();
impl<'a> /*trait*/ QFile_symLinkTarget<QString> for () {
  fn symLinkTarget(self, rsthis: &mut QFile) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFile13symLinkTargetEv()};
    let mut ret = unsafe {_ZNK5QFile13symLinkTargetEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFile {
  pub fn NewQFile<T: QFile_NewQFile>(value: T) -> QFile {
    let rsthis = value.NewQFile();
    return rsthis;
    // return 1;
  }
}

pub trait QFile_NewQFile {
  fn NewQFile(self) -> QFile;
}

// proto: void QFile::NewQFile();
impl<'a> /*trait*/ QFile_NewQFile for () {
  fn NewQFile(self) -> QFile {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFileC1Ev()};
    unsafe {_ZN5QFileC1Ev(qthis)};
    let rsthis = QFile{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QFile::NewQFile(QObject * parent);
impl<'a> /*trait*/ QFile_NewQFile for (&'a mut QObject) {
  fn NewQFile(self) -> QFile {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFileC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN5QFileC1EP7QObject(qthis, arg0)};
    let rsthis = QFile{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFile {
  pub fn link<RetType, T: QFile_link<RetType>>(&mut self, value: T) -> RetType {
    return value.link(self);
    // return 1;
  }
}

pub trait QFile_link<RetType> {
  fn link(self, rsthis: &mut QFile) -> RetType;
}

// proto: static bool QFile::link(const QString & oldname, const QString & newName);
impl<'a> /*trait*/ QFile_link<i8> for (&'a  QString, &'a  QString) {
  fn link(self, rsthis: &mut QFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile4linkERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QFile4linkERK7QStringS2_(arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFile {
  pub fn rename<RetType, T: QFile_rename<RetType>>(&mut self, value: T) -> RetType {
    return value.rename(self);
    // return 1;
  }
}

pub trait QFile_rename<RetType> {
  fn rename(self, rsthis: &mut QFile) -> RetType;
}

// proto: static bool QFile::rename(const QString & oldName, const QString & newName);
impl<'a> /*trait*/ QFile_rename<i8> for (&'a  QString, &'a  QString) {
  fn rename(self, rsthis: &mut QFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile6renameERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QFile6renameERK7QStringS2_(arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

// proto:  bool QFile::link(const QString & newName);
impl<'a> /*trait*/ QFile_link<i8> for (&'a  QString) {
  fn link(self, rsthis: &mut QFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile4linkERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QFile4linkERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFile {
  pub fn resize<RetType, T: QFile_resize<RetType>>(&mut self, value: T) -> RetType {
    return value.resize(self);
    // return 1;
  }
}

pub trait QFile_resize<RetType> {
  fn resize(self, rsthis: &mut QFile) -> RetType;
}

// proto: static bool QFile::resize(const QString & filename, qint64 sz);
impl<'a> /*trait*/ QFile_resize<i8> for (&'a  QString, i64) {
  fn resize(self, rsthis: &mut QFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile6resizeERK7QStringx()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_longlong;
    let mut ret = unsafe {_ZN5QFile6resizeERK7QStringx(arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFile {
  pub fn exists<RetType, T: QFile_exists<RetType>>(&mut self, value: T) -> RetType {
    return value.exists(self);
    // return 1;
  }
}

pub trait QFile_exists<RetType> {
  fn exists(self, rsthis: &mut QFile) -> RetType;
}

// proto: static bool QFile::exists(const QString & fileName);
impl<'a> /*trait*/ QFile_exists<i8> for (&'a  QString) {
  fn exists(self, rsthis: &mut QFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile6existsERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QFile6existsERK7QString(arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFile {
  pub fn FreeQFile<RetType, T: QFile_FreeQFile<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQFile(self);
    // return 1;
  }
}

pub trait QFile_FreeQFile<RetType> {
  fn FreeQFile(self, rsthis: &mut QFile) -> RetType;
}

// proto:  void QFile::FreeQFile();
impl<'a> /*trait*/ QFile_FreeQFile<()> for () {
  fn FreeQFile(self, rsthis: &mut QFile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFileD0Ev()};
     unsafe {_ZN5QFileD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFile {
  pub fn copy<RetType, T: QFile_copy<RetType>>(&mut self, value: T) -> RetType {
    return value.copy(self);
    // return 1;
  }
}

pub trait QFile_copy<RetType> {
  fn copy(self, rsthis: &mut QFile) -> RetType;
}

// proto: static bool QFile::copy(const QString & fileName, const QString & newName);
impl<'a> /*trait*/ QFile_copy<i8> for (&'a  QString, &'a  QString) {
  fn copy(self, rsthis: &mut QFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile4copyERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QFile4copyERK7QStringS2_(arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFile {
  pub fn readLink<RetType, T: QFile_readLink<RetType>>(&mut self, value: T) -> RetType {
    return value.readLink(self);
    // return 1;
  }
}

pub trait QFile_readLink<RetType> {
  fn readLink(self, rsthis: &mut QFile) -> RetType;
}

// proto: static QString QFile::readLink(const QString & fileName);
impl<'a> /*trait*/ QFile_readLink<QString> for (&'a  QString) {
  fn readLink(self, rsthis: &mut QFile) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile8readLinkERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QFile8readLinkERK7QString(arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  bool QFile::exists();
impl<'a> /*trait*/ QFile_exists<i8> for () {
  fn exists(self, rsthis: &mut QFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFile6existsEv()};
    let mut ret = unsafe {_ZNK5QFile6existsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFile {
  pub fn size<RetType, T: QFile_size<RetType>>(&mut self, value: T) -> RetType {
    return value.size(self);
    // return 1;
  }
}

pub trait QFile_size<RetType> {
  fn size(self, rsthis: &mut QFile) -> RetType;
}

// proto:  long long QFile::size();
impl<'a> /*trait*/ QFile_size<i64> for () {
  fn size(self, rsthis: &mut QFile) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFile4sizeEv()};
    let mut ret = unsafe {_ZNK5QFile4sizeEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

// proto:  bool QFile::resize(qint64 sz);
impl<'a> /*trait*/ QFile_resize<i8> for (i64) {
  fn resize(self, rsthis: &mut QFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile6resizeEx()};
    let arg0 = self  as c_longlong;
    let mut ret = unsafe {_ZN5QFile6resizeEx(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QFile::NewQFile(const QFile & );
impl<'a> /*trait*/ QFile_NewQFile for (&'a  QFile) {
  fn NewQFile(self) -> QFile {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFileC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN5QFileC1ERKS_(qthis, arg0)};
    let rsthis = QFile{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFile {
  pub fn setFileName<RetType, T: QFile_setFileName<RetType>>(&mut self, value: T) -> RetType {
    return value.setFileName(self);
    // return 1;
  }
}

pub trait QFile_setFileName<RetType> {
  fn setFileName(self, rsthis: &mut QFile) -> RetType;
}

// proto:  void QFile::setFileName(const QString & name);
impl<'a> /*trait*/ QFile_setFileName<()> for (&'a  QString) {
  fn setFileName(self, rsthis: &mut QFile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile11setFileNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QFile11setFileNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFile {
  pub fn remove<RetType, T: QFile_remove<RetType>>(&mut self, value: T) -> RetType {
    return value.remove(self);
    // return 1;
  }
}

pub trait QFile_remove<RetType> {
  fn remove(self, rsthis: &mut QFile) -> RetType;
}

// proto:  bool QFile::remove();
impl<'a> /*trait*/ QFile_remove<i8> for () {
  fn remove(self, rsthis: &mut QFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile6removeEv()};
    let mut ret = unsafe {_ZN5QFile6removeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  bool QFile::copy(const QString & newName);
impl<'a> /*trait*/ QFile_copy<i8> for (&'a  QString) {
  fn copy(self, rsthis: &mut QFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile4copyERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QFile4copyERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFile {
  pub fn encodeName<RetType, T: QFile_encodeName<RetType>>(&mut self, value: T) -> RetType {
    return value.encodeName(self);
    // return 1;
  }
}

pub trait QFile_encodeName<RetType> {
  fn encodeName(self, rsthis: &mut QFile) -> RetType;
}

// proto: static QByteArray QFile::encodeName(const QString & fileName);
impl<'a> /*trait*/ QFile_encodeName<QByteArray> for (&'a  QString) {
  fn encodeName(self, rsthis: &mut QFile) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile10encodeNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QFile10encodeNameERK7QString(arg0)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFile {
  pub fn decodeName<RetType, T: QFile_decodeName<RetType>>(&mut self, value: T) -> RetType {
    return value.decodeName(self);
    // return 1;
  }
}

pub trait QFile_decodeName<RetType> {
  fn decodeName(self, rsthis: &mut QFile) -> RetType;
}

// proto: static QString QFile::decodeName(const QByteArray & localFileName);
impl<'a> /*trait*/ QFile_decodeName<QString> for (&'a  QByteArray) {
  fn decodeName(self, rsthis: &mut QFile) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile10decodeNameERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QFile10decodeNameERK10QByteArray(arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  bool QFile::rename(const QString & newName);
impl<'a> /*trait*/ QFile_rename<i8> for (&'a  QString) {
  fn rename(self, rsthis: &mut QFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile6renameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QFile6renameERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFile {
  pub fn fileName<RetType, T: QFile_fileName<RetType>>(&mut self, value: T) -> RetType {
    return value.fileName(self);
    // return 1;
  }
}

pub trait QFile_fileName<RetType> {
  fn fileName(self, rsthis: &mut QFile) -> RetType;
}

// proto:  QString QFile::fileName();
impl<'a> /*trait*/ QFile_fileName<QString> for () {
  fn fileName(self, rsthis: &mut QFile) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFile8fileNameEv()};
    let mut ret = unsafe {_ZNK5QFile8fileNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: static QString QFile::decodeName(const char * localFileName);
impl<'a> /*trait*/ QFile_decodeName<QString> for (&'a  String) {
  fn decodeName(self, rsthis: &mut QFile) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile10decodeNameEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    let mut ret = unsafe {_ZN5QFile10decodeNameEPKc(arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFile {
  pub fn metaObject<RetType, T: QFile_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QFile_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QFile) -> RetType;
}

// proto:  const QMetaObject * QFile::metaObject();
impl<'a> /*trait*/ QFile_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QFile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFile10metaObjectEv()};
     unsafe {_ZNK5QFile10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QFile::NewQFile(const QString & name, QObject * parent);
impl<'a> /*trait*/ QFile_NewQFile for (&'a  QString, &'a mut QObject) {
  fn NewQFile(self) -> QFile {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFileC1ERK7QStringP7QObject()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN5QFileC1ERK7QStringP7QObject(qthis, arg0, arg1)};
    let rsthis = QFile{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: static QString QFile::symLinkTarget(const QString & fileName);
impl<'a> /*trait*/ QFile_symLinkTarget<QString> for (&'a  QString) {
  fn symLinkTarget(self, rsthis: &mut QFile) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile13symLinkTargetERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QFile13symLinkTargetERK7QString(arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: static bool QFile::remove(const QString & fileName);
impl<'a> /*trait*/ QFile_remove<i8> for (&'a  QString) {
  fn remove(self, rsthis: &mut QFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFile6removeERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QFile6removeERK7QString(arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QFile::NewQFile(const QString & name);
impl<'a> /*trait*/ QFile_NewQFile for (&'a  QString) {
  fn NewQFile(self) -> QFile {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFileC1ERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN5QFileC1ERK7QString(qthis, arg0)};
    let rsthis = QFile{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  QString QFile::readLink();
impl<'a> /*trait*/ QFile_readLink<QString> for () {
  fn readLink(self, rsthis: &mut QFile) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QFile8readLinkEv()};
    let mut ret = unsafe {_ZNK5QFile8readLinkEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

