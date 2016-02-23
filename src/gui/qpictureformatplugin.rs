// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtGui/qpictureformatplugin.h
// dst-file: /src/gui/qpictureformatplugin.rs
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
use super::super::core::qobject::*; // 771
use std::ops::Deref;
use super::super::core::qstring::*; // 771
use super::qpicture::*; // 773
use super::super::core::qobjectdefs::*; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QPictureFormatPlugin_Class_Size() -> c_int;
  // proto:  bool QPictureFormatPlugin::loadPicture(const QString & format, const QString & filename, QPicture * pic);
  fn C_ZN20QPictureFormatPlugin11loadPictureERK7QStringS2_P8QPicture(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> c_char;
  // proto:  bool QPictureFormatPlugin::savePicture(const QString & format, const QString & filename, const QPicture & pic);
  fn C_ZN20QPictureFormatPlugin11savePictureERK7QStringS2_RK8QPicture(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> c_char;
  // proto:  void QPictureFormatPlugin::~QPictureFormatPlugin();
  fn C_ZN20QPictureFormatPluginD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QPictureFormatPlugin::QPictureFormatPlugin(QObject * parent);
  fn C_ZN20QPictureFormatPluginC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  bool QPictureFormatPlugin::installIOHandler(const QString & format);
  fn C_ZN20QPictureFormatPlugin16installIOHandlerERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  const QMetaObject * QPictureFormatPlugin::metaObject();
  fn C_ZNK20QPictureFormatPlugin10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QPictureFormatPlugin)=1
#[derive(Default)]
pub struct QPictureFormatPlugin {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QPictureFormatPlugin {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QPictureFormatPlugin {
    return QPictureFormatPlugin{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QPictureFormatPlugin {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QPictureFormatPlugin {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  bool QPictureFormatPlugin::loadPicture(const QString & format, const QString & filename, QPicture * pic);
impl /*struct*/ QPictureFormatPlugin {
  pub fn loadPicture<RetType, T: QPictureFormatPlugin_loadPicture<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.loadPicture(self);
    // return 1;
  }
}

pub trait QPictureFormatPlugin_loadPicture<RetType> {
  fn loadPicture(self , rsthis: & QPictureFormatPlugin) -> RetType;
}

  // proto:  bool QPictureFormatPlugin::loadPicture(const QString & format, const QString & filename, QPicture * pic);
impl<'a> /*trait*/ QPictureFormatPlugin_loadPicture<i8> for (&'a QString, &'a QString, &'a QPicture) {
  fn loadPicture(self , rsthis: & QPictureFormatPlugin) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QPictureFormatPlugin11loadPictureERK7QStringS2_P8QPicture()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN20QPictureFormatPlugin11loadPictureERK7QStringS2_P8QPicture(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QPictureFormatPlugin::savePicture(const QString & format, const QString & filename, const QPicture & pic);
impl /*struct*/ QPictureFormatPlugin {
  pub fn savePicture<RetType, T: QPictureFormatPlugin_savePicture<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.savePicture(self);
    // return 1;
  }
}

pub trait QPictureFormatPlugin_savePicture<RetType> {
  fn savePicture(self , rsthis: & QPictureFormatPlugin) -> RetType;
}

  // proto:  bool QPictureFormatPlugin::savePicture(const QString & format, const QString & filename, const QPicture & pic);
impl<'a> /*trait*/ QPictureFormatPlugin_savePicture<i8> for (&'a QString, &'a QString, &'a QPicture) {
  fn savePicture(self , rsthis: & QPictureFormatPlugin) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QPictureFormatPlugin11savePictureERK7QStringS2_RK8QPicture()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN20QPictureFormatPlugin11savePictureERK7QStringS2_RK8QPicture(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QPictureFormatPlugin::~QPictureFormatPlugin();
impl /*struct*/ QPictureFormatPlugin {
  pub fn free<RetType, T: QPictureFormatPlugin_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QPictureFormatPlugin_free<RetType> {
  fn free(self , rsthis: & QPictureFormatPlugin) -> RetType;
}

  // proto:  void QPictureFormatPlugin::~QPictureFormatPlugin();
impl<'a> /*trait*/ QPictureFormatPlugin_free<()> for () {
  fn free(self , rsthis: & QPictureFormatPlugin) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QPictureFormatPluginD2Ev()};
     unsafe {C_ZN20QPictureFormatPluginD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPictureFormatPlugin::QPictureFormatPlugin(QObject * parent);
impl /*struct*/ QPictureFormatPlugin {
  pub fn new<T: QPictureFormatPlugin_new>(value: T) -> QPictureFormatPlugin {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QPictureFormatPlugin_new {
  fn new(self) -> QPictureFormatPlugin;
}

  // proto:  void QPictureFormatPlugin::QPictureFormatPlugin(QObject * parent);
impl<'a> /*trait*/ QPictureFormatPlugin_new for (Option<&'a QObject>) {
  fn new(self) -> QPictureFormatPlugin {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QPictureFormatPluginC2EP7QObject()};
    let ctysz: c_int = unsafe{QPictureFormatPlugin_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = (if self.is_none() {0} else {self.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN20QPictureFormatPluginC2EP7QObject(arg0)};
    let rsthis = QPictureFormatPlugin{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QPictureFormatPlugin::installIOHandler(const QString & format);
impl /*struct*/ QPictureFormatPlugin {
  pub fn installIOHandler<RetType, T: QPictureFormatPlugin_installIOHandler<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.installIOHandler(self);
    // return 1;
  }
}

pub trait QPictureFormatPlugin_installIOHandler<RetType> {
  fn installIOHandler(self , rsthis: & QPictureFormatPlugin) -> RetType;
}

  // proto:  bool QPictureFormatPlugin::installIOHandler(const QString & format);
impl<'a> /*trait*/ QPictureFormatPlugin_installIOHandler<i8> for (&'a QString) {
  fn installIOHandler(self , rsthis: & QPictureFormatPlugin) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QPictureFormatPlugin16installIOHandlerERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN20QPictureFormatPlugin16installIOHandlerERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  const QMetaObject * QPictureFormatPlugin::metaObject();
impl /*struct*/ QPictureFormatPlugin {
  pub fn metaObject<RetType, T: QPictureFormatPlugin_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QPictureFormatPlugin_metaObject<RetType> {
  fn metaObject(self , rsthis: & QPictureFormatPlugin) -> RetType;
}

  // proto:  const QMetaObject * QPictureFormatPlugin::metaObject();
impl<'a> /*trait*/ QPictureFormatPlugin_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QPictureFormatPlugin) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QPictureFormatPlugin10metaObjectEv()};
    let mut ret = unsafe {C_ZNK20QPictureFormatPlugin10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

