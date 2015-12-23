// auto generated, do not modify.
// created: Wed Dec 23 22:29:56 2015
// src-file: /QtGui/qaccessibleplugin.h
// dst-file: /src/gui/qaccessibleplugin.rs
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
use super::qaccessible::QAccessibleInterface; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QAccessiblePlugin::QAccessiblePlugin(QObject * parent);
  fn _ZN17QAccessiblePluginC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QAccessiblePlugin::metaObject();
  fn _ZNK17QAccessiblePlugin10metaObjectEv(qthis: *mut c_void);
  // proto:  void QAccessiblePlugin::~QAccessiblePlugin();
  fn _ZN17QAccessiblePluginD0Ev(qthis: *mut c_void);
  // proto:  QAccessibleInterface * QAccessiblePlugin::create(const QString & key, QObject * object);
  fn _ZN17QAccessiblePlugin6createERK7QStringP7QObject(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QAccessiblePlugin)=1
pub struct QAccessiblePlugin {
  qbase: QObject,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAccessiblePlugin {
  pub fn inheritFrom(qthis: *mut c_void) -> QAccessiblePlugin {
    return QAccessiblePlugin{qbase: QObject::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QAccessiblePlugin {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QAccessiblePlugin {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QAccessiblePlugin::QAccessiblePlugin(QObject * parent);
impl /*struct*/ QAccessiblePlugin {
  pub fn New<T: QAccessiblePlugin_New>(value: T) -> QAccessiblePlugin {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessiblePlugin_New {
  fn New(self) -> QAccessiblePlugin;
}

  // proto:  void QAccessiblePlugin::QAccessiblePlugin(QObject * parent);
impl<'a> /*trait*/ QAccessiblePlugin_New for (&'a QObject) {
  fn New(self) -> QAccessiblePlugin {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAccessiblePluginC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QAccessiblePluginC1EP7QObject(qthis, arg0)};
    let rsthis = QAccessiblePlugin{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QAccessiblePlugin::metaObject();
impl /*struct*/ QAccessiblePlugin {
  pub fn metaObject<RetType, T: QAccessiblePlugin_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QAccessiblePlugin_metaObject<RetType> {
  fn metaObject(self , rsthis: & QAccessiblePlugin) -> RetType;
}

  // proto:  const QMetaObject * QAccessiblePlugin::metaObject();
impl<'a> /*trait*/ QAccessiblePlugin_metaObject<()> for () {
  fn metaObject(self , rsthis: & QAccessiblePlugin) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessiblePlugin10metaObjectEv()};
     unsafe {_ZNK17QAccessiblePlugin10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAccessiblePlugin::~QAccessiblePlugin();
impl /*struct*/ QAccessiblePlugin {
  pub fn Free<RetType, T: QAccessiblePlugin_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QAccessiblePlugin_Free<RetType> {
  fn Free(self , rsthis: & QAccessiblePlugin) -> RetType;
}

  // proto:  void QAccessiblePlugin::~QAccessiblePlugin();
impl<'a> /*trait*/ QAccessiblePlugin_Free<()> for () {
  fn Free(self , rsthis: & QAccessiblePlugin) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAccessiblePluginD0Ev()};
     unsafe {_ZN17QAccessiblePluginD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QAccessibleInterface * QAccessiblePlugin::create(const QString & key, QObject * object);
impl /*struct*/ QAccessiblePlugin {
  pub fn create<RetType, T: QAccessiblePlugin_create<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.create(self);
    // return 1;
  }
}

pub trait QAccessiblePlugin_create<RetType> {
  fn create(self , rsthis: & QAccessiblePlugin) -> RetType;
}

  // proto:  QAccessibleInterface * QAccessiblePlugin::create(const QString & key, QObject * object);
impl<'a> /*trait*/ QAccessiblePlugin_create<QAccessibleInterface> for (&'a QString, &'a QObject) {
  fn create(self , rsthis: & QAccessiblePlugin) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAccessiblePlugin6createERK7QStringP7QObject()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN17QAccessiblePlugin6createERK7QStringP7QObject(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QAccessibleInterface::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

// <= body block end

