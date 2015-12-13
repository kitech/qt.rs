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
  fn _ZN9QResourceC1ERK7QStringRK7QLocale(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZNK9QResource6localeEv() -> i32;
  fn _ZN9QResource9setLocaleERK7QLocale(arg0: *const c_void) -> i32;
  fn _ZN9QResource16registerResourceEPKhRK7QString(arg0: *const c_uchar, arg1: *const c_void) -> i32;
  fn _ZNK9QResource4dataEv() -> i32;
  fn _ZN9QResource11searchPathsEv() -> i32;
  fn _ZNK9QResource8fileNameEv() -> i32;
  fn _ZNK9QResource16absoluteFilePathEv() -> i32;
  fn _ZN9QResource18unregisterResourceEPKhRK7QString(arg0: *const c_uchar, arg1: *const c_void) -> i32;
  fn _ZN9QResource16registerResourceERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZN9QResource13addSearchPathERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK9QResource4sizeEv() -> i32;
  fn _ZN9QResourceD0Ev() -> i32;
  fn _ZNK9QResource7isValidEv() -> i32;
  fn _ZN9QResource11setFileNameERK7QString(arg0: *const c_void) -> i32;
  fn _ZN9QResource18unregisterResourceERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZNK9QResource12isCompressedEv() -> i32;
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
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN9QResourceC1ERK7QStringRK7QLocale(qthis, arg0, arg1)};
    let rsthis = QResource{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QResource {
  pub fn locale<T: QResource_locale>(&mut self, value: T) -> i32 {
    value.locale(self);
    return 1;
  }
}

pub trait QResource_locale {
  fn locale(self, this: &mut QResource) -> i32;
}

// proto: QLocale QResource::locale();
impl<'a> /*trait*/ QResource_locale for () {
  fn locale(self, this: &mut QResource) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QResource6localeEv()};
    unsafe {_ZNK9QResource6localeEv()};
    return 1;
  }
}

impl /*struct*/ QResource {
  pub fn setLocale<T: QResource_setLocale>(&mut self, value: T) -> i32 {
    value.setLocale(self);
    return 1;
  }
}

pub trait QResource_setLocale {
  fn setLocale(self, this: &mut QResource) -> i32;
}

// proto: void QResource::setLocale(const QLocale & locale);
impl<'a> /*trait*/ QResource_setLocale for (&'a  QLocale) {
  fn setLocale(self, this: &mut QResource) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QResource9setLocaleERK7QLocale()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QResource9setLocaleERK7QLocale(arg0)};
    return 1;
  }
}

impl /*struct*/ QResource {
  pub fn registerResource<T: QResource_registerResource>(&mut self, value: T) -> i32 {
    value.registerResource(self);
    return 1;
  }
}

pub trait QResource_registerResource {
  fn registerResource(self, this: &mut QResource) -> i32;
}

// proto: bool QResource::registerResource(const uchar * rccData, const QString & resourceRoot);
impl<'a> /*trait*/ QResource_registerResource for (&'a  String, &'a  QString) {
  fn registerResource(self, this: &mut QResource) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QResource16registerResourceEPKhRK7QString()};
    let arg0 = self.0.as_ptr()  as *const c_uchar;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN9QResource16registerResourceEPKhRK7QString(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QResource {
  pub fn data<T: QResource_data>(&mut self, value: T) -> i32 {
    value.data(self);
    return 1;
  }
}

pub trait QResource_data {
  fn data(self, this: &mut QResource) -> i32;
}

// proto: const uchar * QResource::data();
impl<'a> /*trait*/ QResource_data for () {
  fn data(self, this: &mut QResource) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QResource4dataEv()};
    unsafe {_ZNK9QResource4dataEv()};
    return 1;
  }
}

impl /*struct*/ QResource {
  pub fn searchPaths<T: QResource_searchPaths>(&mut self, value: T) -> i32 {
    value.searchPaths(self);
    return 1;
  }
}

pub trait QResource_searchPaths {
  fn searchPaths(self, this: &mut QResource) -> i32;
}

// proto: QStringList QResource::searchPaths();
impl<'a> /*trait*/ QResource_searchPaths for () {
  fn searchPaths(self, this: &mut QResource) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QResource11searchPathsEv()};
    unsafe {_ZN9QResource11searchPathsEv()};
    return 1;
  }
}

impl /*struct*/ QResource {
  pub fn fileName<T: QResource_fileName>(&mut self, value: T) -> i32 {
    value.fileName(self);
    return 1;
  }
}

pub trait QResource_fileName {
  fn fileName(self, this: &mut QResource) -> i32;
}

// proto: QString QResource::fileName();
impl<'a> /*trait*/ QResource_fileName for () {
  fn fileName(self, this: &mut QResource) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QResource8fileNameEv()};
    unsafe {_ZNK9QResource8fileNameEv()};
    return 1;
  }
}

impl /*struct*/ QResource {
  pub fn absoluteFilePath<T: QResource_absoluteFilePath>(&mut self, value: T) -> i32 {
    value.absoluteFilePath(self);
    return 1;
  }
}

pub trait QResource_absoluteFilePath {
  fn absoluteFilePath(self, this: &mut QResource) -> i32;
}

// proto: QString QResource::absoluteFilePath();
impl<'a> /*trait*/ QResource_absoluteFilePath for () {
  fn absoluteFilePath(self, this: &mut QResource) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QResource16absoluteFilePathEv()};
    unsafe {_ZNK9QResource16absoluteFilePathEv()};
    return 1;
  }
}

impl /*struct*/ QResource {
  pub fn unregisterResource<T: QResource_unregisterResource>(&mut self, value: T) -> i32 {
    value.unregisterResource(self);
    return 1;
  }
}

pub trait QResource_unregisterResource {
  fn unregisterResource(self, this: &mut QResource) -> i32;
}

// proto: bool QResource::unregisterResource(const uchar * rccData, const QString & resourceRoot);
impl<'a> /*trait*/ QResource_unregisterResource for (&'a  String, &'a  QString) {
  fn unregisterResource(self, this: &mut QResource) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QResource18unregisterResourceEPKhRK7QString()};
    let arg0 = self.0.as_ptr()  as *const c_uchar;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN9QResource18unregisterResourceEPKhRK7QString(arg0, arg1)};
    return 1;
  }
}

// proto: bool QResource::registerResource(const QString & rccFilename, const QString & resourceRoot);
impl<'a> /*trait*/ QResource_registerResource for (&'a  QString, &'a  QString) {
  fn registerResource(self, this: &mut QResource) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QResource16registerResourceERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN9QResource16registerResourceERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QResource {
  pub fn addSearchPath<T: QResource_addSearchPath>(&mut self, value: T) -> i32 {
    value.addSearchPath(self);
    return 1;
  }
}

pub trait QResource_addSearchPath {
  fn addSearchPath(self, this: &mut QResource) -> i32;
}

// proto: void QResource::addSearchPath(const QString & path);
impl<'a> /*trait*/ QResource_addSearchPath for (&'a  QString) {
  fn addSearchPath(self, this: &mut QResource) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QResource13addSearchPathERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QResource13addSearchPathERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QResource {
  pub fn size<T: QResource_size>(&mut self, value: T) -> i32 {
    value.size(self);
    return 1;
  }
}

pub trait QResource_size {
  fn size(self, this: &mut QResource) -> i32;
}

// proto: long long QResource::size();
impl<'a> /*trait*/ QResource_size for () {
  fn size(self, this: &mut QResource) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QResource4sizeEv()};
    unsafe {_ZNK9QResource4sizeEv()};
    return 1;
  }
}

impl /*struct*/ QResource {
  pub fn FreeQResource<T: QResource_FreeQResource>(&mut self, value: T) -> i32 {
    value.FreeQResource(self);
    return 1;
  }
}

pub trait QResource_FreeQResource {
  fn FreeQResource(self, this: &mut QResource) -> i32;
}

// proto: void QResource::FreeQResource();
impl<'a> /*trait*/ QResource_FreeQResource for () {
  fn FreeQResource(self, this: &mut QResource) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QResourceD0Ev()};
    unsafe {_ZN9QResourceD0Ev()};
    return 1;
  }
}

impl /*struct*/ QResource {
  pub fn isValid<T: QResource_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QResource_isValid {
  fn isValid(self, this: &mut QResource) -> i32;
}

// proto: bool QResource::isValid();
impl<'a> /*trait*/ QResource_isValid for () {
  fn isValid(self, this: &mut QResource) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QResource7isValidEv()};
    unsafe {_ZNK9QResource7isValidEv()};
    return 1;
  }
}

impl /*struct*/ QResource {
  pub fn setFileName<T: QResource_setFileName>(&mut self, value: T) -> i32 {
    value.setFileName(self);
    return 1;
  }
}

pub trait QResource_setFileName {
  fn setFileName(self, this: &mut QResource) -> i32;
}

// proto: void QResource::setFileName(const QString & file);
impl<'a> /*trait*/ QResource_setFileName for (&'a  QString) {
  fn setFileName(self, this: &mut QResource) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QResource11setFileNameERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QResource11setFileNameERK7QString(arg0)};
    return 1;
  }
}

// proto: bool QResource::unregisterResource(const QString & rccFilename, const QString & resourceRoot);
impl<'a> /*trait*/ QResource_unregisterResource for (&'a  QString, &'a  QString) {
  fn unregisterResource(self, this: &mut QResource) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QResource18unregisterResourceERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN9QResource18unregisterResourceERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QResource {
  pub fn isCompressed<T: QResource_isCompressed>(&mut self, value: T) -> i32 {
    value.isCompressed(self);
    return 1;
  }
}

pub trait QResource_isCompressed {
  fn isCompressed(self, this: &mut QResource) -> i32;
}

// proto: bool QResource::isCompressed();
impl<'a> /*trait*/ QResource_isCompressed for () {
  fn isCompressed(self, this: &mut QResource) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QResource12isCompressedEv()};
    unsafe {_ZNK9QResource12isCompressedEv()};
    return 1;
  }
}

