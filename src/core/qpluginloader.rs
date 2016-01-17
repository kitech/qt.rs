// auto generated, do not modify.
// created: Sun Jan 17 17:37:11 2016
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
use super::qobject::QObject; // 773
use std::ops::Deref;
use super::qstring::QString; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QPluginLoader_Class_Size() -> c_int;
  // proto:  bool QPluginLoader::isLoaded();
  fn _ZNK13QPluginLoader8isLoadedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QPluginLoader::unload();
  fn _ZN13QPluginLoader6unloadEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QPluginLoader::QPluginLoader(const QString & fileName, QObject * parent);
  fn _ZN13QPluginLoaderC2ERK7QStringP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  bool QPluginLoader::load();
  fn _ZN13QPluginLoader4loadEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  const QMetaObject * QPluginLoader::metaObject();
  fn _ZNK13QPluginLoader10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  QObject * QPluginLoader::instance();
  fn _ZN13QPluginLoader8instanceEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static QVector<QStaticPlugin> QPluginLoader::staticPlugins();
  fn _ZN13QPluginLoader13staticPluginsEv();
  // proto: static QObjectList QPluginLoader::staticInstances();
  fn _ZN13QPluginLoader15staticInstancesEv();
  // proto:  QJsonObject QPluginLoader::metaData();
  fn _ZNK13QPluginLoader8metaDataEv(qthis: u64 /* *mut c_void*/);
  // proto:  QString QPluginLoader::errorString();
  fn _ZNK13QPluginLoader11errorStringEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QPluginLoader::QPluginLoader(const QPluginLoader & );
  fn _ZN13QPluginLoaderC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QPluginLoader::fileName();
  fn _ZNK13QPluginLoader8fileNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QPluginLoader::setFileName(const QString & fileName);
  fn _ZN13QPluginLoader11setFileNameERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QPluginLoader::QPluginLoader(QObject * parent);
  fn _ZN13QPluginLoaderC2EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QPluginLoader::~QPluginLoader();
  fn _ZN13QPluginLoaderD2Ev(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QPluginLoader)=1
#[derive(Default)]
pub struct QPluginLoader {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QPluginLoader {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QPluginLoader {
    return QPluginLoader{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QPluginLoader {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QPluginLoader {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  bool QPluginLoader::isLoaded();
impl /*struct*/ QPluginLoader {
  pub fn isLoaded<RetType, T: QPluginLoader_isLoaded<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isLoaded(self);
    // return 1;
  }
}

pub trait QPluginLoader_isLoaded<RetType> {
  fn isLoaded(self , rsthis: & QPluginLoader) -> RetType;
}

  // proto:  bool QPluginLoader::isLoaded();
impl<'a> /*trait*/ QPluginLoader_isLoaded<i8> for () {
  fn isLoaded(self , rsthis: & QPluginLoader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPluginLoader8isLoadedEv()};
    let mut ret = unsafe {_ZNK13QPluginLoader8isLoadedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QPluginLoader::unload();
impl /*struct*/ QPluginLoader {
  pub fn unload<RetType, T: QPluginLoader_unload<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.unload(self);
    // return 1;
  }
}

pub trait QPluginLoader_unload<RetType> {
  fn unload(self , rsthis: & QPluginLoader) -> RetType;
}

  // proto:  bool QPluginLoader::unload();
impl<'a> /*trait*/ QPluginLoader_unload<i8> for () {
  fn unload(self , rsthis: & QPluginLoader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPluginLoader6unloadEv()};
    let mut ret = unsafe {_ZN13QPluginLoader6unloadEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPluginLoader::QPluginLoader(const QString & fileName, QObject * parent);
impl /*struct*/ QPluginLoader {
  pub fn new<T: QPluginLoader_new>(value: T) -> QPluginLoader {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QPluginLoader_new {
  fn new(self) -> QPluginLoader;
}

  // proto:  void QPluginLoader::QPluginLoader(const QString & fileName, QObject * parent);
impl<'a> /*trait*/ QPluginLoader_new for (&'a QString, &'a QObject) {
  fn new(self) -> QPluginLoader {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPluginLoaderC2ERK7QStringP7QObject()};
    let ctysz: c_int = unsafe{QPluginLoader_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN13QPluginLoaderC2ERK7QStringP7QObject(qthis_ph, arg0, arg1)};
    let qthis: u64 = qthis_ph;
    let rsthis = QPluginLoader{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QPluginLoader::load();
impl /*struct*/ QPluginLoader {
  pub fn load<RetType, T: QPluginLoader_load<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.load(self);
    // return 1;
  }
}

pub trait QPluginLoader_load<RetType> {
  fn load(self , rsthis: & QPluginLoader) -> RetType;
}

  // proto:  bool QPluginLoader::load();
impl<'a> /*trait*/ QPluginLoader_load<i8> for () {
  fn load(self , rsthis: & QPluginLoader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPluginLoader4loadEv()};
    let mut ret = unsafe {_ZN13QPluginLoader4loadEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  const QMetaObject * QPluginLoader::metaObject();
impl /*struct*/ QPluginLoader {
  pub fn metaObject<RetType, T: QPluginLoader_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QPluginLoader_metaObject<RetType> {
  fn metaObject(self , rsthis: & QPluginLoader) -> RetType;
}

  // proto:  const QMetaObject * QPluginLoader::metaObject();
impl<'a> /*trait*/ QPluginLoader_metaObject<()> for () {
  fn metaObject(self , rsthis: & QPluginLoader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPluginLoader10metaObjectEv()};
     unsafe {_ZNK13QPluginLoader10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QObject * QPluginLoader::instance();
impl /*struct*/ QPluginLoader {
  pub fn instance<RetType, T: QPluginLoader_instance<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.instance(self);
    // return 1;
  }
}

pub trait QPluginLoader_instance<RetType> {
  fn instance(self , rsthis: & QPluginLoader) -> RetType;
}

  // proto:  QObject * QPluginLoader::instance();
impl<'a> /*trait*/ QPluginLoader_instance<QObject> for () {
  fn instance(self , rsthis: & QPluginLoader) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPluginLoader8instanceEv()};
    let mut ret = unsafe {_ZN13QPluginLoader8instanceEv(rsthis.qclsinst)};
    let mut ret1 = QObject::inheritFrom(ret as u64);
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
  pub fn metaData<RetType, T: QPluginLoader_metaData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaData(self);
    // return 1;
  }
}

pub trait QPluginLoader_metaData<RetType> {
  fn metaData(self , rsthis: & QPluginLoader) -> RetType;
}

  // proto:  QJsonObject QPluginLoader::metaData();
impl<'a> /*trait*/ QPluginLoader_metaData<()> for () {
  fn metaData(self , rsthis: & QPluginLoader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPluginLoader8metaDataEv()};
     unsafe {_ZNK13QPluginLoader8metaDataEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QPluginLoader::errorString();
impl /*struct*/ QPluginLoader {
  pub fn errorString<RetType, T: QPluginLoader_errorString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.errorString(self);
    // return 1;
  }
}

pub trait QPluginLoader_errorString<RetType> {
  fn errorString(self , rsthis: & QPluginLoader) -> RetType;
}

  // proto:  QString QPluginLoader::errorString();
impl<'a> /*trait*/ QPluginLoader_errorString<QString> for () {
  fn errorString(self , rsthis: & QPluginLoader) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPluginLoader11errorStringEv()};
    let mut ret = unsafe {_ZNK13QPluginLoader11errorStringEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPluginLoader::QPluginLoader(const QPluginLoader & );
impl<'a> /*trait*/ QPluginLoader_new for (&'a QPluginLoader) {
  fn new(self) -> QPluginLoader {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPluginLoaderC2ERKS_()};
    let ctysz: c_int = unsafe{QPluginLoader_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QPluginLoaderC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QPluginLoader{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QPluginLoader::fileName();
impl /*struct*/ QPluginLoader {
  pub fn fileName<RetType, T: QPluginLoader_fileName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fileName(self);
    // return 1;
  }
}

pub trait QPluginLoader_fileName<RetType> {
  fn fileName(self , rsthis: & QPluginLoader) -> RetType;
}

  // proto:  QString QPluginLoader::fileName();
impl<'a> /*trait*/ QPluginLoader_fileName<QString> for () {
  fn fileName(self , rsthis: & QPluginLoader) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPluginLoader8fileNameEv()};
    let mut ret = unsafe {_ZNK13QPluginLoader8fileNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPluginLoader::setFileName(const QString & fileName);
impl /*struct*/ QPluginLoader {
  pub fn setFileName<RetType, T: QPluginLoader_setFileName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFileName(self);
    // return 1;
  }
}

pub trait QPluginLoader_setFileName<RetType> {
  fn setFileName(self , rsthis: & QPluginLoader) -> RetType;
}

  // proto:  void QPluginLoader::setFileName(const QString & fileName);
impl<'a> /*trait*/ QPluginLoader_setFileName<()> for (&'a QString) {
  fn setFileName(self , rsthis: & QPluginLoader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPluginLoader11setFileNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QPluginLoader11setFileNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPluginLoader::QPluginLoader(QObject * parent);
impl<'a> /*trait*/ QPluginLoader_new for (&'a QObject) {
  fn new(self) -> QPluginLoader {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPluginLoaderC2EP7QObject()};
    let ctysz: c_int = unsafe{QPluginLoader_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QPluginLoaderC2EP7QObject(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QPluginLoader{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPluginLoader::~QPluginLoader();
impl /*struct*/ QPluginLoader {
  pub fn free<RetType, T: QPluginLoader_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QPluginLoader_free<RetType> {
  fn free(self , rsthis: & QPluginLoader) -> RetType;
}

  // proto:  void QPluginLoader::~QPluginLoader();
impl<'a> /*trait*/ QPluginLoader_free<()> for () {
  fn free(self , rsthis: & QPluginLoader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPluginLoaderD2Ev()};
     unsafe {_ZN13QPluginLoaderD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

