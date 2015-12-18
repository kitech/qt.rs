// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qlocale::QLocale;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QResource::NewQResource(const QString & file, const QLocale & locale);
  fn _ZN9QResourceC1ERK7QStringRK7QLocale(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  QLocale QResource::locale();
  fn _ZNK9QResource6localeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QResource::setLocale(const QLocale & locale);
  fn _ZN9QResource9setLocaleERK7QLocale(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static bool QResource::registerResource(const uchar * rccData, const QString & resourceRoot);
  fn _ZN9QResource16registerResourceEPKhRK7QString(arg0: *const c_uchar, arg1: *mut c_void) -> int8_t;
  // proto:  const uchar * QResource::data();
  fn _ZNK9QResource4dataEv(qthis: *mut c_void) -> *const c_uchar;
  // proto: static QStringList QResource::searchPaths();
  fn _ZN9QResource11searchPathsEv() ;
  // proto:  QString QResource::fileName();
  fn _ZNK9QResource8fileNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QResource::absoluteFilePath();
  fn _ZNK9QResource16absoluteFilePathEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static bool QResource::unregisterResource(const uchar * rccData, const QString & resourceRoot);
  fn _ZN9QResource18unregisterResourceEPKhRK7QString(arg0: *const c_uchar, arg1: *mut c_void) -> int8_t;
  // proto: static bool QResource::registerResource(const QString & rccFilename, const QString & resourceRoot);
  fn _ZN9QResource16registerResourceERK7QStringS2_(arg0: *mut c_void, arg1: *mut c_void) -> int8_t;
  // proto: static void QResource::addSearchPath(const QString & path);
  fn _ZN9QResource13addSearchPathERK7QString(arg0: *mut c_void) ;
  // proto:  long long QResource::size();
  fn _ZNK9QResource4sizeEv(qthis: *mut c_void) -> c_longlong;
  // proto:  void QResource::FreeQResource();
  fn _ZN9QResourceD0Ev(qthis: *mut c_void) ;
  // proto:  bool QResource::isValid();
  fn _ZNK9QResource7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QResource::setFileName(const QString & file);
  fn _ZN9QResource11setFileNameERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static bool QResource::unregisterResource(const QString & rccFilename, const QString & resourceRoot);
  fn _ZN9QResource18unregisterResourceERK7QStringS2_(arg0: *mut c_void, arg1: *mut c_void) -> int8_t;
  // proto:  bool QResource::isCompressed();
  fn _ZNK9QResource12isCompressedEv(qthis: *mut c_void) -> int8_t;
}

// body block begin
// class sizeof(QResource)=1
pub struct QResource {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QResource {
  pub fn NewQResource<T: QResource_NewQResource>(value: T) -> QResource {
    let rsthis = value.NewQResource();
    return rsthis;
    // return 1;
  }
}

pub trait QResource_NewQResource {
  fn NewQResource(self) -> QResource;
}

// proto: void QResource::NewQResource(const QString & file, const QLocale & locale);
impl<'a> /*trait*/ QResource_NewQResource for (&'a  QString, &'a  QLocale) {
  fn NewQResource(self) -> QResource {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QResourceC1ERK7QStringRK7QLocale()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN9QResourceC1ERK7QStringRK7QLocale(qthis, arg0, arg1)};
    let rsthis = QResource{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QResource {
  pub fn locale<RetType, T: QResource_locale<RetType>>(&mut self, value: T) -> RetType {
    return value.locale(self);
    // return 1;
  }
}

pub trait QResource_locale<RetType> {
  fn locale(self, rsthis: &mut QResource) -> RetType;
}

// proto:  QLocale QResource::locale();
impl<'a> /*trait*/ QResource_locale<QLocale> for () {
  fn locale(self, rsthis: &mut QResource) -> QLocale {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QResource6localeEv()};
    let mut ret = unsafe {_ZNK9QResource6localeEv(rsthis.qclsinst)};
    let mut ret1 = QLocale{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QResource {
  pub fn setLocale<RetType, T: QResource_setLocale<RetType>>(&mut self, value: T) -> RetType {
    return value.setLocale(self);
    // return 1;
  }
}

pub trait QResource_setLocale<RetType> {
  fn setLocale(self, rsthis: &mut QResource) -> RetType;
}

// proto:  void QResource::setLocale(const QLocale & locale);
impl<'a> /*trait*/ QResource_setLocale<()> for (&'a  QLocale) {
  fn setLocale(self, rsthis: &mut QResource) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QResource9setLocaleERK7QLocale()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QResource9setLocaleERK7QLocale(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QResource {
  pub fn registerResource<RetType, T: QResource_registerResource<RetType>>(&mut self, value: T) -> RetType {
    return value.registerResource(self);
    // return 1;
  }
}

pub trait QResource_registerResource<RetType> {
  fn registerResource(self, rsthis: &mut QResource) -> RetType;
}

// proto: static bool QResource::registerResource(const uchar * rccData, const QString & resourceRoot);
impl<'a> /*trait*/ QResource_registerResource<i8> for (&'a  String, &'a  QString) {
  fn registerResource(self, rsthis: &mut QResource) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QResource16registerResourceEPKhRK7QString()};
    let arg0 = self.0.as_ptr()  as *const c_uchar;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QResource16registerResourceEPKhRK7QString(arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QResource {
  pub fn data<RetType, T: QResource_data<RetType>>(&mut self, value: T) -> RetType {
    return value.data(self);
    // return 1;
  }
}

pub trait QResource_data<RetType> {
  fn data(self, rsthis: &mut QResource) -> RetType;
}

// proto:  const uchar * QResource::data();
impl<'a> /*trait*/ QResource_data<String> for () {
  fn data(self, rsthis: &mut QResource) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QResource4dataEv()};
    let mut ret = unsafe {_ZNK9QResource4dataEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

impl /*struct*/ QResource {
  pub fn searchPaths<RetType, T: QResource_searchPaths<RetType>>(&mut self, value: T) -> RetType {
    return value.searchPaths(self);
    // return 1;
  }
}

pub trait QResource_searchPaths<RetType> {
  fn searchPaths(self, rsthis: &mut QResource) -> RetType;
}

// proto: static QStringList QResource::searchPaths();
impl<'a> /*trait*/ QResource_searchPaths<()> for () {
  fn searchPaths(self, rsthis: &mut QResource) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QResource11searchPathsEv()};
     unsafe {_ZN9QResource11searchPathsEv()};
    // return 1;
  }
}

impl /*struct*/ QResource {
  pub fn fileName<RetType, T: QResource_fileName<RetType>>(&mut self, value: T) -> RetType {
    return value.fileName(self);
    // return 1;
  }
}

pub trait QResource_fileName<RetType> {
  fn fileName(self, rsthis: &mut QResource) -> RetType;
}

// proto:  QString QResource::fileName();
impl<'a> /*trait*/ QResource_fileName<QString> for () {
  fn fileName(self, rsthis: &mut QResource) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QResource8fileNameEv()};
    let mut ret = unsafe {_ZNK9QResource8fileNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QResource {
  pub fn absoluteFilePath<RetType, T: QResource_absoluteFilePath<RetType>>(&mut self, value: T) -> RetType {
    return value.absoluteFilePath(self);
    // return 1;
  }
}

pub trait QResource_absoluteFilePath<RetType> {
  fn absoluteFilePath(self, rsthis: &mut QResource) -> RetType;
}

// proto:  QString QResource::absoluteFilePath();
impl<'a> /*trait*/ QResource_absoluteFilePath<QString> for () {
  fn absoluteFilePath(self, rsthis: &mut QResource) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QResource16absoluteFilePathEv()};
    let mut ret = unsafe {_ZNK9QResource16absoluteFilePathEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QResource {
  pub fn unregisterResource<RetType, T: QResource_unregisterResource<RetType>>(&mut self, value: T) -> RetType {
    return value.unregisterResource(self);
    // return 1;
  }
}

pub trait QResource_unregisterResource<RetType> {
  fn unregisterResource(self, rsthis: &mut QResource) -> RetType;
}

// proto: static bool QResource::unregisterResource(const uchar * rccData, const QString & resourceRoot);
impl<'a> /*trait*/ QResource_unregisterResource<i8> for (&'a  String, &'a  QString) {
  fn unregisterResource(self, rsthis: &mut QResource) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QResource18unregisterResourceEPKhRK7QString()};
    let arg0 = self.0.as_ptr()  as *const c_uchar;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QResource18unregisterResourceEPKhRK7QString(arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

// proto: static bool QResource::registerResource(const QString & rccFilename, const QString & resourceRoot);
impl<'a> /*trait*/ QResource_registerResource<i8> for (&'a  QString, &'a  QString) {
  fn registerResource(self, rsthis: &mut QResource) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QResource16registerResourceERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QResource16registerResourceERK7QStringS2_(arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QResource {
  pub fn addSearchPath<RetType, T: QResource_addSearchPath<RetType>>(&mut self, value: T) -> RetType {
    return value.addSearchPath(self);
    // return 1;
  }
}

pub trait QResource_addSearchPath<RetType> {
  fn addSearchPath(self, rsthis: &mut QResource) -> RetType;
}

// proto: static void QResource::addSearchPath(const QString & path);
impl<'a> /*trait*/ QResource_addSearchPath<()> for (&'a  QString) {
  fn addSearchPath(self, rsthis: &mut QResource) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QResource13addSearchPathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QResource13addSearchPathERK7QString(arg0)};
    // return 1;
  }
}

impl /*struct*/ QResource {
  pub fn size<RetType, T: QResource_size<RetType>>(&mut self, value: T) -> RetType {
    return value.size(self);
    // return 1;
  }
}

pub trait QResource_size<RetType> {
  fn size(self, rsthis: &mut QResource) -> RetType;
}

// proto:  long long QResource::size();
impl<'a> /*trait*/ QResource_size<i64> for () {
  fn size(self, rsthis: &mut QResource) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QResource4sizeEv()};
    let mut ret = unsafe {_ZNK9QResource4sizeEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QResource {
  pub fn FreeQResource<RetType, T: QResource_FreeQResource<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQResource(self);
    // return 1;
  }
}

pub trait QResource_FreeQResource<RetType> {
  fn FreeQResource(self, rsthis: &mut QResource) -> RetType;
}

// proto:  void QResource::FreeQResource();
impl<'a> /*trait*/ QResource_FreeQResource<()> for () {
  fn FreeQResource(self, rsthis: &mut QResource) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QResourceD0Ev()};
     unsafe {_ZN9QResourceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QResource {
  pub fn isValid<RetType, T: QResource_isValid<RetType>>(&mut self, value: T) -> RetType {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QResource_isValid<RetType> {
  fn isValid(self, rsthis: &mut QResource) -> RetType;
}

// proto:  bool QResource::isValid();
impl<'a> /*trait*/ QResource_isValid<i8> for () {
  fn isValid(self, rsthis: &mut QResource) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QResource7isValidEv()};
    let mut ret = unsafe {_ZNK9QResource7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QResource {
  pub fn setFileName<RetType, T: QResource_setFileName<RetType>>(&mut self, value: T) -> RetType {
    return value.setFileName(self);
    // return 1;
  }
}

pub trait QResource_setFileName<RetType> {
  fn setFileName(self, rsthis: &mut QResource) -> RetType;
}

// proto:  void QResource::setFileName(const QString & file);
impl<'a> /*trait*/ QResource_setFileName<()> for (&'a  QString) {
  fn setFileName(self, rsthis: &mut QResource) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QResource11setFileNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QResource11setFileNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: static bool QResource::unregisterResource(const QString & rccFilename, const QString & resourceRoot);
impl<'a> /*trait*/ QResource_unregisterResource<i8> for (&'a  QString, &'a  QString) {
  fn unregisterResource(self, rsthis: &mut QResource) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QResource18unregisterResourceERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QResource18unregisterResourceERK7QStringS2_(arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QResource {
  pub fn isCompressed<RetType, T: QResource_isCompressed<RetType>>(&mut self, value: T) -> RetType {
    return value.isCompressed(self);
    // return 1;
  }
}

pub trait QResource_isCompressed<RetType> {
  fn isCompressed(self, rsthis: &mut QResource) -> RetType;
}

// proto:  bool QResource::isCompressed();
impl<'a> /*trait*/ QResource_isCompressed<i8> for () {
  fn isCompressed(self, rsthis: &mut QResource) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QResource12isCompressedEv()};
    let mut ret = unsafe {_ZNK9QResource12isCompressedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

