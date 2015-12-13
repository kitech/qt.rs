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

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZNK13QPluginLoader8isLoadedEv() -> i32;
  fn _ZN13QPluginLoader6unloadEv() -> i32;
  fn _ZN13QPluginLoaderC1ERK7QStringP7QObject(qthis: *mut c_void, arg0: *const c_void, arg1: *mut c_void) -> i32;
  fn _ZN13QPluginLoader4loadEv() -> i32;
  fn _ZNK13QPluginLoader10metaObjectEv() -> i32;
  fn _ZN13QPluginLoader8instanceEv() -> i32;
  fn _ZN13QPluginLoader13staticPluginsEv() -> i32;
  fn _ZN13QPluginLoader15staticInstancesEv() -> i32;
  fn _ZNK13QPluginLoader8metaDataEv() -> i32;
  fn _ZNK13QPluginLoader11errorStringEv() -> i32;
  fn _ZN13QPluginLoaderC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK13QPluginLoader8fileNameEv() -> i32;
  fn _ZN13QPluginLoader11setFileNameERK7QString(arg0: *const c_void) -> i32;
  fn _ZN13QPluginLoaderC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZN13QPluginLoaderD0Ev() -> i32;
}

// body block begin
// class sizeof(QPluginLoader)=1
pub struct QPluginLoader {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPluginLoader {
  pub fn isLoaded<T: QPluginLoader_isLoaded>(&mut self, value: T) -> i32 {
    value.isLoaded(self);
    return 1;
  }
}

pub trait QPluginLoader_isLoaded {
  fn isLoaded(self, this: &mut QPluginLoader) -> i32;
}

// proto: bool QPluginLoader::isLoaded();
impl<'a> /*trait*/ QPluginLoader_isLoaded for () {
  fn isLoaded(self, this: &mut QPluginLoader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPluginLoader8isLoadedEv()};
    unsafe {_ZNK13QPluginLoader8isLoadedEv()};
    return 1;
  }
}

impl /*struct*/ QPluginLoader {
  pub fn unload<T: QPluginLoader_unload>(&mut self, value: T) -> i32 {
    value.unload(self);
    return 1;
  }
}

pub trait QPluginLoader_unload {
  fn unload(self, this: &mut QPluginLoader) -> i32;
}

// proto: bool QPluginLoader::unload();
impl<'a> /*trait*/ QPluginLoader_unload for () {
  fn unload(self, this: &mut QPluginLoader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPluginLoader6unloadEv()};
    unsafe {_ZN13QPluginLoader6unloadEv()};
    return 1;
  }
}

impl /*struct*/ QPluginLoader {
  pub fn NewQPluginLoader<T: QPluginLoader_NewQPluginLoader>(value: T) -> QPluginLoader {
    let rsthis = value.NewQPluginLoader();
    return rsthis;
    // return 1;
  }
}

pub trait QPluginLoader_NewQPluginLoader {
  fn NewQPluginLoader(self) -> QPluginLoader;
}

// proto: void QPluginLoader::NewQPluginLoader(const QString & fileName, QObject * parent);
impl<'a> /*trait*/ QPluginLoader_NewQPluginLoader for (&'a  QString, &'a mut QObject) {
  fn NewQPluginLoader(self) -> QPluginLoader {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPluginLoaderC1ERK7QStringP7QObject()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN13QPluginLoaderC1ERK7QStringP7QObject(qthis, arg0, arg1)};
    let rsthis = QPluginLoader{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPluginLoader {
  pub fn load<T: QPluginLoader_load>(&mut self, value: T) -> i32 {
    value.load(self);
    return 1;
  }
}

pub trait QPluginLoader_load {
  fn load(self, this: &mut QPluginLoader) -> i32;
}

// proto: bool QPluginLoader::load();
impl<'a> /*trait*/ QPluginLoader_load for () {
  fn load(self, this: &mut QPluginLoader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPluginLoader4loadEv()};
    unsafe {_ZN13QPluginLoader4loadEv()};
    return 1;
  }
}

impl /*struct*/ QPluginLoader {
  pub fn metaObject<T: QPluginLoader_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QPluginLoader_metaObject {
  fn metaObject(self, this: &mut QPluginLoader) -> i32;
}

// proto: const QMetaObject * QPluginLoader::metaObject();
impl<'a> /*trait*/ QPluginLoader_metaObject for () {
  fn metaObject(self, this: &mut QPluginLoader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPluginLoader10metaObjectEv()};
    unsafe {_ZNK13QPluginLoader10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QPluginLoader {
  pub fn instance<T: QPluginLoader_instance>(&mut self, value: T) -> i32 {
    value.instance(self);
    return 1;
  }
}

pub trait QPluginLoader_instance {
  fn instance(self, this: &mut QPluginLoader) -> i32;
}

// proto: QObject * QPluginLoader::instance();
impl<'a> /*trait*/ QPluginLoader_instance for () {
  fn instance(self, this: &mut QPluginLoader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPluginLoader8instanceEv()};
    unsafe {_ZN13QPluginLoader8instanceEv()};
    return 1;
  }
}

impl /*struct*/ QPluginLoader {
  pub fn staticPlugins<T: QPluginLoader_staticPlugins>(&mut self, value: T) -> i32 {
    value.staticPlugins(self);
    return 1;
  }
}

pub trait QPluginLoader_staticPlugins {
  fn staticPlugins(self, this: &mut QPluginLoader) -> i32;
}

// proto: QVector<QStaticPlugin> QPluginLoader::staticPlugins();
impl<'a> /*trait*/ QPluginLoader_staticPlugins for () {
  fn staticPlugins(self, this: &mut QPluginLoader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPluginLoader13staticPluginsEv()};
    unsafe {_ZN13QPluginLoader13staticPluginsEv()};
    return 1;
  }
}

impl /*struct*/ QPluginLoader {
  pub fn staticInstances<T: QPluginLoader_staticInstances>(&mut self, value: T) -> i32 {
    value.staticInstances(self);
    return 1;
  }
}

pub trait QPluginLoader_staticInstances {
  fn staticInstances(self, this: &mut QPluginLoader) -> i32;
}

// proto: QList<QObject *> QPluginLoader::staticInstances();
impl<'a> /*trait*/ QPluginLoader_staticInstances for () {
  fn staticInstances(self, this: &mut QPluginLoader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPluginLoader15staticInstancesEv()};
    unsafe {_ZN13QPluginLoader15staticInstancesEv()};
    return 1;
  }
}

impl /*struct*/ QPluginLoader {
  pub fn metaData<T: QPluginLoader_metaData>(&mut self, value: T) -> i32 {
    value.metaData(self);
    return 1;
  }
}

pub trait QPluginLoader_metaData {
  fn metaData(self, this: &mut QPluginLoader) -> i32;
}

// proto: QJsonObject QPluginLoader::metaData();
impl<'a> /*trait*/ QPluginLoader_metaData for () {
  fn metaData(self, this: &mut QPluginLoader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPluginLoader8metaDataEv()};
    unsafe {_ZNK13QPluginLoader8metaDataEv()};
    return 1;
  }
}

impl /*struct*/ QPluginLoader {
  pub fn errorString<T: QPluginLoader_errorString>(&mut self, value: T) -> i32 {
    value.errorString(self);
    return 1;
  }
}

pub trait QPluginLoader_errorString {
  fn errorString(self, this: &mut QPluginLoader) -> i32;
}

// proto: QString QPluginLoader::errorString();
impl<'a> /*trait*/ QPluginLoader_errorString for () {
  fn errorString(self, this: &mut QPluginLoader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPluginLoader11errorStringEv()};
    unsafe {_ZNK13QPluginLoader11errorStringEv()};
    return 1;
  }
}

// proto: void QPluginLoader::NewQPluginLoader(const QPluginLoader & );
impl<'a> /*trait*/ QPluginLoader_NewQPluginLoader for (&'a  QPluginLoader) {
  fn NewQPluginLoader(self) -> QPluginLoader {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPluginLoaderC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QPluginLoaderC1ERKS_(qthis, arg0)};
    let rsthis = QPluginLoader{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPluginLoader {
  pub fn fileName<T: QPluginLoader_fileName>(&mut self, value: T) -> i32 {
    value.fileName(self);
    return 1;
  }
}

pub trait QPluginLoader_fileName {
  fn fileName(self, this: &mut QPluginLoader) -> i32;
}

// proto: QString QPluginLoader::fileName();
impl<'a> /*trait*/ QPluginLoader_fileName for () {
  fn fileName(self, this: &mut QPluginLoader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPluginLoader8fileNameEv()};
    unsafe {_ZNK13QPluginLoader8fileNameEv()};
    return 1;
  }
}

impl /*struct*/ QPluginLoader {
  pub fn setFileName<T: QPluginLoader_setFileName>(&mut self, value: T) -> i32 {
    value.setFileName(self);
    return 1;
  }
}

pub trait QPluginLoader_setFileName {
  fn setFileName(self, this: &mut QPluginLoader) -> i32;
}

// proto: void QPluginLoader::setFileName(const QString & fileName);
impl<'a> /*trait*/ QPluginLoader_setFileName for (&'a  QString) {
  fn setFileName(self, this: &mut QPluginLoader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPluginLoader11setFileNameERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QPluginLoader11setFileNameERK7QString(arg0)};
    return 1;
  }
}

// proto: void QPluginLoader::NewQPluginLoader(QObject * parent);
impl<'a> /*trait*/ QPluginLoader_NewQPluginLoader for (&'a mut QObject) {
  fn NewQPluginLoader(self) -> QPluginLoader {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPluginLoaderC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QPluginLoaderC1EP7QObject(qthis, arg0)};
    let rsthis = QPluginLoader{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPluginLoader {
  pub fn FreeQPluginLoader<T: QPluginLoader_FreeQPluginLoader>(&mut self, value: T) -> i32 {
    value.FreeQPluginLoader(self);
    return 1;
  }
}

pub trait QPluginLoader_FreeQPluginLoader {
  fn FreeQPluginLoader(self, this: &mut QPluginLoader) -> i32;
}

// proto: void QPluginLoader::FreeQPluginLoader();
impl<'a> /*trait*/ QPluginLoader_FreeQPluginLoader for () {
  fn FreeQPluginLoader(self, this: &mut QPluginLoader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPluginLoaderD0Ev()};
    unsafe {_ZN13QPluginLoaderD0Ev()};
    return 1;
  }
}

