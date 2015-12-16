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
  // proto:  bool QPluginLoader::isLoaded();
  fn _ZNK13QPluginLoader8isLoadedEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QPluginLoader::unload();
  fn _ZN13QPluginLoader6unloadEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QPluginLoader::NewQPluginLoader(const QString & fileName, QObject * parent);
  fn _ZN13QPluginLoaderC1ERK7QStringP7QObject(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  bool QPluginLoader::load();
  fn _ZN13QPluginLoader4loadEv(qthis: *mut c_void) -> int8_t;
  // proto:  const QMetaObject * QPluginLoader::metaObject();
  fn _ZNK13QPluginLoader10metaObjectEv(qthis: *mut c_void) ;
  // proto:  QObject * QPluginLoader::instance();
  fn _ZN13QPluginLoader8instanceEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static QVector<QStaticPlugin> QPluginLoader::staticPlugins();
  fn _ZN13QPluginLoader13staticPluginsEv() ;
  // proto: static QList<QObject *> QPluginLoader::staticInstances();
  fn _ZN13QPluginLoader15staticInstancesEv() ;
  // proto:  QJsonObject QPluginLoader::metaData();
  fn _ZNK13QPluginLoader8metaDataEv(qthis: *mut c_void) ;
  // proto:  QString QPluginLoader::errorString();
  fn _ZNK13QPluginLoader11errorStringEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPluginLoader::NewQPluginLoader(const QPluginLoader & );
  fn _ZN13QPluginLoaderC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QPluginLoader::fileName();
  fn _ZNK13QPluginLoader8fileNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPluginLoader::setFileName(const QString & fileName);
  fn _ZN13QPluginLoader11setFileNameERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPluginLoader::NewQPluginLoader(QObject * parent);
  fn _ZN13QPluginLoaderC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPluginLoader::FreeQPluginLoader();
  fn _ZN13QPluginLoaderD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QPluginLoader)=1
pub struct QPluginLoader {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPluginLoader {
  pub fn isLoaded<T: QPluginLoader_isLoaded>(&mut self, value: T) -> i8 {
    return value.isLoaded(self);
    // return 1;
  }
}

pub trait QPluginLoader_isLoaded {
  fn isLoaded(self, rsthis: &mut QPluginLoader) -> i8;
}

// proto:  bool QPluginLoader::isLoaded();
impl<'a> /*trait*/ QPluginLoader_isLoaded for () {
  fn isLoaded(self, rsthis: &mut QPluginLoader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPluginLoader8isLoadedEv()};
    let mut ret = unsafe {_ZNK13QPluginLoader8isLoadedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPluginLoader {
  pub fn unload<T: QPluginLoader_unload>(&mut self, value: T) -> i8 {
    return value.unload(self);
    // return 1;
  }
}

pub trait QPluginLoader_unload {
  fn unload(self, rsthis: &mut QPluginLoader) -> i8;
}

// proto:  bool QPluginLoader::unload();
impl<'a> /*trait*/ QPluginLoader_unload for () {
  fn unload(self, rsthis: &mut QPluginLoader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPluginLoader6unloadEv()};
    let mut ret = unsafe {_ZN13QPluginLoader6unloadEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
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
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN13QPluginLoaderC1ERK7QStringP7QObject(qthis, arg0, arg1)};
    let rsthis = QPluginLoader{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPluginLoader {
  pub fn load<T: QPluginLoader_load>(&mut self, value: T) -> i8 {
    return value.load(self);
    // return 1;
  }
}

pub trait QPluginLoader_load {
  fn load(self, rsthis: &mut QPluginLoader) -> i8;
}

// proto:  bool QPluginLoader::load();
impl<'a> /*trait*/ QPluginLoader_load for () {
  fn load(self, rsthis: &mut QPluginLoader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPluginLoader4loadEv()};
    let mut ret = unsafe {_ZN13QPluginLoader4loadEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPluginLoader {
  pub fn metaObject<T: QPluginLoader_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QPluginLoader_metaObject {
  fn metaObject(self, rsthis: &mut QPluginLoader) ;
}

// proto:  const QMetaObject * QPluginLoader::metaObject();
impl<'a> /*trait*/ QPluginLoader_metaObject for () {
  fn metaObject(self, rsthis: &mut QPluginLoader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPluginLoader10metaObjectEv()};
     unsafe {_ZNK13QPluginLoader10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPluginLoader {
  pub fn instance<T: QPluginLoader_instance>(&mut self, value: T) -> QObject {
    return value.instance(self);
    // return 1;
  }
}

pub trait QPluginLoader_instance {
  fn instance(self, rsthis: &mut QPluginLoader) -> QObject;
}

// proto:  QObject * QPluginLoader::instance();
impl<'a> /*trait*/ QPluginLoader_instance for () {
  fn instance(self, rsthis: &mut QPluginLoader) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPluginLoader8instanceEv()};
    let mut ret = unsafe {_ZN13QPluginLoader8instanceEv(rsthis.qclsinst)};
    let mut ret1 = QObject{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPluginLoader {
  pub fn staticPlugins<T: QPluginLoader_staticPlugins>(&mut self, value: T)  {
     value.staticPlugins(self);
    // return 1;
  }
}

pub trait QPluginLoader_staticPlugins {
  fn staticPlugins(self, rsthis: &mut QPluginLoader) ;
}

// proto: static QVector<QStaticPlugin> QPluginLoader::staticPlugins();
impl<'a> /*trait*/ QPluginLoader_staticPlugins for () {
  fn staticPlugins(self, rsthis: &mut QPluginLoader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPluginLoader13staticPluginsEv()};
     unsafe {_ZN13QPluginLoader13staticPluginsEv()};
    // return 1;
  }
}

impl /*struct*/ QPluginLoader {
  pub fn staticInstances<T: QPluginLoader_staticInstances>(&mut self, value: T)  {
     value.staticInstances(self);
    // return 1;
  }
}

pub trait QPluginLoader_staticInstances {
  fn staticInstances(self, rsthis: &mut QPluginLoader) ;
}

// proto: static QList<QObject *> QPluginLoader::staticInstances();
impl<'a> /*trait*/ QPluginLoader_staticInstances for () {
  fn staticInstances(self, rsthis: &mut QPluginLoader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPluginLoader15staticInstancesEv()};
     unsafe {_ZN13QPluginLoader15staticInstancesEv()};
    // return 1;
  }
}

impl /*struct*/ QPluginLoader {
  pub fn metaData<T: QPluginLoader_metaData>(&mut self, value: T)  {
     value.metaData(self);
    // return 1;
  }
}

pub trait QPluginLoader_metaData {
  fn metaData(self, rsthis: &mut QPluginLoader) ;
}

// proto:  QJsonObject QPluginLoader::metaData();
impl<'a> /*trait*/ QPluginLoader_metaData for () {
  fn metaData(self, rsthis: &mut QPluginLoader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPluginLoader8metaDataEv()};
     unsafe {_ZNK13QPluginLoader8metaDataEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPluginLoader {
  pub fn errorString<T: QPluginLoader_errorString>(&mut self, value: T) -> QString {
    return value.errorString(self);
    // return 1;
  }
}

pub trait QPluginLoader_errorString {
  fn errorString(self, rsthis: &mut QPluginLoader) -> QString;
}

// proto:  QString QPluginLoader::errorString();
impl<'a> /*trait*/ QPluginLoader_errorString for () {
  fn errorString(self, rsthis: &mut QPluginLoader) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPluginLoader11errorStringEv()};
    let mut ret = unsafe {_ZNK13QPluginLoader11errorStringEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QPluginLoader::NewQPluginLoader(const QPluginLoader & );
impl<'a> /*trait*/ QPluginLoader_NewQPluginLoader for (&'a  QPluginLoader) {
  fn NewQPluginLoader(self) -> QPluginLoader {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPluginLoaderC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QPluginLoaderC1ERKS_(qthis, arg0)};
    let rsthis = QPluginLoader{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPluginLoader {
  pub fn fileName<T: QPluginLoader_fileName>(&mut self, value: T) -> QString {
    return value.fileName(self);
    // return 1;
  }
}

pub trait QPluginLoader_fileName {
  fn fileName(self, rsthis: &mut QPluginLoader) -> QString;
}

// proto:  QString QPluginLoader::fileName();
impl<'a> /*trait*/ QPluginLoader_fileName for () {
  fn fileName(self, rsthis: &mut QPluginLoader) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPluginLoader8fileNameEv()};
    let mut ret = unsafe {_ZNK13QPluginLoader8fileNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPluginLoader {
  pub fn setFileName<T: QPluginLoader_setFileName>(&mut self, value: T)  {
     value.setFileName(self);
    // return 1;
  }
}

pub trait QPluginLoader_setFileName {
  fn setFileName(self, rsthis: &mut QPluginLoader) ;
}

// proto:  void QPluginLoader::setFileName(const QString & fileName);
impl<'a> /*trait*/ QPluginLoader_setFileName for (&'a  QString) {
  fn setFileName(self, rsthis: &mut QPluginLoader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPluginLoader11setFileNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QPluginLoader11setFileNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
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
  pub fn FreeQPluginLoader<T: QPluginLoader_FreeQPluginLoader>(&mut self, value: T)  {
     value.FreeQPluginLoader(self);
    // return 1;
  }
}

pub trait QPluginLoader_FreeQPluginLoader {
  fn FreeQPluginLoader(self, rsthis: &mut QPluginLoader) ;
}

// proto:  void QPluginLoader::FreeQPluginLoader();
impl<'a> /*trait*/ QPluginLoader_FreeQPluginLoader for () {
  fn FreeQPluginLoader(self, rsthis: &mut QPluginLoader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPluginLoaderD0Ev()};
     unsafe {_ZN13QPluginLoaderD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

