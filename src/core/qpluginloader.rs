// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtCore/qpluginloader.h
// dst-file: /src/core/qpluginloader.rs
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
use super::qstring::QString; // 773
use super::qobject::QObject; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  bool QPluginLoader::isLoaded();
  fn _ZNK13QPluginLoader8isLoadedEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QPluginLoader::unload();
  fn _ZN13QPluginLoader6unloadEv(qthis: *mut c_void) -> c_char;
  // proto:  void QPluginLoader::QPluginLoader(const QString & fileName, QObject * parent);
  fn _ZN13QPluginLoaderC1ERK7QStringP7QObject(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  bool QPluginLoader::load();
  fn _ZN13QPluginLoader4loadEv(qthis: *mut c_void) -> c_char;
  // proto:  const QMetaObject * QPluginLoader::metaObject();
  fn _ZNK13QPluginLoader10metaObjectEv(qthis: *mut c_void);
  // proto:  QObject * QPluginLoader::instance();
  fn _ZN13QPluginLoader8instanceEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static QVector<QStaticPlugin> QPluginLoader::staticPlugins();
  fn _ZN13QPluginLoader13staticPluginsEv();
  // proto: static QObjectList QPluginLoader::staticInstances();
  fn _ZN13QPluginLoader15staticInstancesEv();
  // proto:  QJsonObject QPluginLoader::metaData();
  fn _ZNK13QPluginLoader8metaDataEv(qthis: *mut c_void);
  // proto:  QString QPluginLoader::errorString();
  fn _ZNK13QPluginLoader11errorStringEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPluginLoader::QPluginLoader(const QPluginLoader & );
  fn _ZN13QPluginLoaderC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QString QPluginLoader::fileName();
  fn _ZNK13QPluginLoader8fileNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPluginLoader::setFileName(const QString & fileName);
  fn _ZN13QPluginLoader11setFileNameERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPluginLoader::QPluginLoader(QObject * parent);
  fn _ZN13QPluginLoaderC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPluginLoader::~QPluginLoader();
  fn _ZN13QPluginLoaderD0Ev(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QPluginLoader)=1
pub struct QPluginLoader {
  pub qclsinst: *mut c_void,
}

  // proto:  bool QPluginLoader::isLoaded();
impl /*struct*/ QPluginLoader {
  pub fn isLoaded<RetType, T: QPluginLoader_isLoaded<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isLoaded(self);
    // return 1;
  }
}

pub trait QPluginLoader_isLoaded<RetType> {
  fn isLoaded(self , rsthis: &mut QPluginLoader) -> RetType;
}

  // proto:  bool QPluginLoader::isLoaded();
impl<'a> /*trait*/ QPluginLoader_isLoaded<i8> for () {
  fn isLoaded(self , rsthis: &mut QPluginLoader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPluginLoader8isLoadedEv()};
    let mut ret = unsafe {_ZNK13QPluginLoader8isLoadedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QPluginLoader::unload();
impl /*struct*/ QPluginLoader {
  pub fn unload<RetType, T: QPluginLoader_unload<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.unload(self);
    // return 1;
  }
}

pub trait QPluginLoader_unload<RetType> {
  fn unload(self , rsthis: &mut QPluginLoader) -> RetType;
}

  // proto:  bool QPluginLoader::unload();
impl<'a> /*trait*/ QPluginLoader_unload<i8> for () {
  fn unload(self , rsthis: &mut QPluginLoader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPluginLoader6unloadEv()};
    let mut ret = unsafe {_ZN13QPluginLoader6unloadEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPluginLoader::QPluginLoader(const QString & fileName, QObject * parent);
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

  // proto:  void QPluginLoader::QPluginLoader(const QString & fileName, QObject * parent);
impl<'a> /*trait*/ QPluginLoader_NewQPluginLoader for (QString, QObject) {
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

  // proto:  bool QPluginLoader::load();
impl /*struct*/ QPluginLoader {
  pub fn load<RetType, T: QPluginLoader_load<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.load(self);
    // return 1;
  }
}

pub trait QPluginLoader_load<RetType> {
  fn load(self , rsthis: &mut QPluginLoader) -> RetType;
}

  // proto:  bool QPluginLoader::load();
impl<'a> /*trait*/ QPluginLoader_load<i8> for () {
  fn load(self , rsthis: &mut QPluginLoader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPluginLoader4loadEv()};
    let mut ret = unsafe {_ZN13QPluginLoader4loadEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  const QMetaObject * QPluginLoader::metaObject();
impl /*struct*/ QPluginLoader {
  pub fn metaObject<RetType, T: QPluginLoader_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QPluginLoader_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QPluginLoader) -> RetType;
}

  // proto:  const QMetaObject * QPluginLoader::metaObject();
impl<'a> /*trait*/ QPluginLoader_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QPluginLoader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPluginLoader10metaObjectEv()};
     unsafe {_ZNK13QPluginLoader10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QObject * QPluginLoader::instance();
impl /*struct*/ QPluginLoader {
  pub fn instance<RetType, T: QPluginLoader_instance<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.instance(self);
    // return 1;
  }
}

pub trait QPluginLoader_instance<RetType> {
  fn instance(self , rsthis: &mut QPluginLoader) -> RetType;
}

  // proto:  QObject * QPluginLoader::instance();
impl<'a> /*trait*/ QPluginLoader_instance<QObject> for () {
  fn instance(self , rsthis: &mut QPluginLoader) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPluginLoader8instanceEv()};
    let mut ret = unsafe {_ZN13QPluginLoader8instanceEv(rsthis.qclsinst)};
    let mut ret1 = QObject{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto: static QVector<QStaticPlugin> QPluginLoader::staticPlugins();
impl /*struct*/ QPluginLoader {
  pub fn staticPlugins_s<RetType, T: QPluginLoader_staticPlugins_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.staticPlugins_s();
    // return 1;
  }
}

pub trait QPluginLoader_staticPlugins_s<RetType> {
  fn staticPlugins_s(self ) -> RetType;
}

  // proto: static QVector<QStaticPlugin> QPluginLoader::staticPlugins();
impl<'a> /*trait*/ QPluginLoader_staticPlugins_s<()> for () {
  fn staticPlugins_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPluginLoader13staticPluginsEv()};
     unsafe {_ZN13QPluginLoader13staticPluginsEv()};
    // return 1;
  }
}

  // proto: static QObjectList QPluginLoader::staticInstances();
impl /*struct*/ QPluginLoader {
  pub fn staticInstances_s<RetType, T: QPluginLoader_staticInstances_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.staticInstances_s();
    // return 1;
  }
}

pub trait QPluginLoader_staticInstances_s<RetType> {
  fn staticInstances_s(self ) -> RetType;
}

  // proto: static QObjectList QPluginLoader::staticInstances();
impl<'a> /*trait*/ QPluginLoader_staticInstances_s<()> for () {
  fn staticInstances_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPluginLoader15staticInstancesEv()};
     unsafe {_ZN13QPluginLoader15staticInstancesEv()};
    // return 1;
  }
}

  // proto:  QJsonObject QPluginLoader::metaData();
impl /*struct*/ QPluginLoader {
  pub fn metaData<RetType, T: QPluginLoader_metaData<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaData(self);
    // return 1;
  }
}

pub trait QPluginLoader_metaData<RetType> {
  fn metaData(self , rsthis: &mut QPluginLoader) -> RetType;
}

  // proto:  QJsonObject QPluginLoader::metaData();
impl<'a> /*trait*/ QPluginLoader_metaData<()> for () {
  fn metaData(self , rsthis: &mut QPluginLoader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPluginLoader8metaDataEv()};
     unsafe {_ZNK13QPluginLoader8metaDataEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QPluginLoader::errorString();
impl /*struct*/ QPluginLoader {
  pub fn errorString<RetType, T: QPluginLoader_errorString<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.errorString(self);
    // return 1;
  }
}

pub trait QPluginLoader_errorString<RetType> {
  fn errorString(self , rsthis: &mut QPluginLoader) -> RetType;
}

  // proto:  QString QPluginLoader::errorString();
impl<'a> /*trait*/ QPluginLoader_errorString<QString> for () {
  fn errorString(self , rsthis: &mut QPluginLoader) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPluginLoader11errorStringEv()};
    let mut ret = unsafe {_ZNK13QPluginLoader11errorStringEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPluginLoader::QPluginLoader(const QPluginLoader & );
impl<'a> /*trait*/ QPluginLoader_NewQPluginLoader for (QPluginLoader) {
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

  // proto:  QString QPluginLoader::fileName();
impl /*struct*/ QPluginLoader {
  pub fn fileName<RetType, T: QPluginLoader_fileName<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.fileName(self);
    // return 1;
  }
}

pub trait QPluginLoader_fileName<RetType> {
  fn fileName(self , rsthis: &mut QPluginLoader) -> RetType;
}

  // proto:  QString QPluginLoader::fileName();
impl<'a> /*trait*/ QPluginLoader_fileName<QString> for () {
  fn fileName(self , rsthis: &mut QPluginLoader) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPluginLoader8fileNameEv()};
    let mut ret = unsafe {_ZNK13QPluginLoader8fileNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPluginLoader::setFileName(const QString & fileName);
impl /*struct*/ QPluginLoader {
  pub fn setFileName<RetType, T: QPluginLoader_setFileName<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFileName(self);
    // return 1;
  }
}

pub trait QPluginLoader_setFileName<RetType> {
  fn setFileName(self , rsthis: &mut QPluginLoader) -> RetType;
}

  // proto:  void QPluginLoader::setFileName(const QString & fileName);
impl<'a> /*trait*/ QPluginLoader_setFileName<()> for (QString) {
  fn setFileName(self , rsthis: &mut QPluginLoader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPluginLoader11setFileNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QPluginLoader11setFileNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPluginLoader::QPluginLoader(QObject * parent);
impl<'a> /*trait*/ QPluginLoader_NewQPluginLoader for (QObject) {
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

  // proto:  void QPluginLoader::~QPluginLoader();
impl /*struct*/ QPluginLoader {
  pub fn FreeQPluginLoader<RetType, T: QPluginLoader_FreeQPluginLoader<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQPluginLoader(self);
    // return 1;
  }
}

pub trait QPluginLoader_FreeQPluginLoader<RetType> {
  fn FreeQPluginLoader(self , rsthis: &mut QPluginLoader) -> RetType;
}

  // proto:  void QPluginLoader::~QPluginLoader();
impl<'a> /*trait*/ QPluginLoader_FreeQPluginLoader<()> for () {
  fn FreeQPluginLoader(self , rsthis: &mut QPluginLoader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPluginLoaderD0Ev()};
     unsafe {_ZN13QPluginLoaderD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

