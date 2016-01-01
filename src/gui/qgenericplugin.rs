// auto generated, do not modify.
// created: Fri Jan  1 12:13:41 2016
// src-file: /QtGui/qgenericplugin.h
// dst-file: /src/gui/qgenericplugin.rs
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
use super::super::core::qobject::QObject; // 771
use std::ops::Deref;
use super::super::core::qstring::QString; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QGenericPlugin_Class_Size() -> c_int;
  // proto:  void QGenericPlugin::QGenericPlugin(QObject * parent);
  fn dector_ZN14QGenericPluginC1EP7QObject(arg0: *mut c_void) -> *mut c_void;
  fn _ZN14QGenericPluginC1EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QObject * QGenericPlugin::create(const QString & name, const QString & spec);
  fn _ZN14QGenericPlugin6createERK7QStringS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QGenericPlugin::~QGenericPlugin();
  fn _ZN14QGenericPluginD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  const QMetaObject * QGenericPlugin::metaObject();
  fn _ZNK14QGenericPlugin10metaObjectEv(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QGenericPlugin)=1
#[derive(Default)]
pub struct QGenericPlugin {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QGenericPlugin {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGenericPlugin {
    return QGenericPlugin{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QGenericPlugin {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QGenericPlugin {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QGenericPlugin::QGenericPlugin(QObject * parent);
impl /*struct*/ QGenericPlugin {
  pub fn new<T: QGenericPlugin_new>(value: T) -> QGenericPlugin {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QGenericPlugin_new {
  fn new(self) -> QGenericPlugin;
}

  // proto:  void QGenericPlugin::QGenericPlugin(QObject * parent);
impl<'a> /*trait*/ QGenericPlugin_new for (&'a QObject) {
  fn new(self) -> QGenericPlugin {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGenericPluginC1EP7QObject()};
    let ctysz: c_int = unsafe{QGenericPlugin_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN14QGenericPluginC1EP7QObject(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN14QGenericPluginC1EP7QObject(arg0)} as u64;
    let rsthis = QGenericPlugin{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QObject * QGenericPlugin::create(const QString & name, const QString & spec);
impl /*struct*/ QGenericPlugin {
  pub fn create<RetType, T: QGenericPlugin_create<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.create(self);
    // return 1;
  }
}

pub trait QGenericPlugin_create<RetType> {
  fn create(self , rsthis: & QGenericPlugin) -> RetType;
}

  // proto:  QObject * QGenericPlugin::create(const QString & name, const QString & spec);
impl<'a> /*trait*/ QGenericPlugin_create<QObject> for (&'a QString, &'a QString) {
  fn create(self , rsthis: & QGenericPlugin) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGenericPlugin6createERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN14QGenericPlugin6createERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGenericPlugin::~QGenericPlugin();
impl /*struct*/ QGenericPlugin {
  pub fn free<RetType, T: QGenericPlugin_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QGenericPlugin_free<RetType> {
  fn free(self , rsthis: & QGenericPlugin) -> RetType;
}

  // proto:  void QGenericPlugin::~QGenericPlugin();
impl<'a> /*trait*/ QGenericPlugin_free<()> for () {
  fn free(self , rsthis: & QGenericPlugin) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGenericPluginD0Ev()};
     unsafe {_ZN14QGenericPluginD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QGenericPlugin::metaObject();
impl /*struct*/ QGenericPlugin {
  pub fn metaObject<RetType, T: QGenericPlugin_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QGenericPlugin_metaObject<RetType> {
  fn metaObject(self , rsthis: & QGenericPlugin) -> RetType;
}

  // proto:  const QMetaObject * QGenericPlugin::metaObject();
impl<'a> /*trait*/ QGenericPlugin_metaObject<()> for () {
  fn metaObject(self , rsthis: & QGenericPlugin) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGenericPlugin10metaObjectEv()};
     unsafe {_ZNK14QGenericPlugin10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

