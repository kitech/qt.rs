// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
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
  pub qclsinst: *mut c_void,
}

  // proto:  void QAccessiblePlugin::QAccessiblePlugin(QObject * parent);
impl /*struct*/ QAccessiblePlugin {
  pub fn NewQAccessiblePlugin<T: QAccessiblePlugin_NewQAccessiblePlugin>(value: T) -> QAccessiblePlugin {
    let rsthis = value.NewQAccessiblePlugin();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessiblePlugin_NewQAccessiblePlugin {
  fn NewQAccessiblePlugin(self) -> QAccessiblePlugin;
}

  // proto:  void QAccessiblePlugin::QAccessiblePlugin(QObject * parent);
impl<'a> /*trait*/ QAccessiblePlugin_NewQAccessiblePlugin for (QObject) {
  fn NewQAccessiblePlugin(self) -> QAccessiblePlugin {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAccessiblePluginC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QAccessiblePluginC1EP7QObject(qthis, arg0)};
    let rsthis = QAccessiblePlugin{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QAccessiblePlugin::metaObject();
impl /*struct*/ QAccessiblePlugin {
  pub fn metaObject<RetType, T: QAccessiblePlugin_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QAccessiblePlugin_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QAccessiblePlugin) -> RetType;
}

  // proto:  const QMetaObject * QAccessiblePlugin::metaObject();
impl<'a> /*trait*/ QAccessiblePlugin_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QAccessiblePlugin) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessiblePlugin10metaObjectEv()};
     unsafe {_ZNK17QAccessiblePlugin10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAccessiblePlugin::~QAccessiblePlugin();
impl /*struct*/ QAccessiblePlugin {
  pub fn FreeQAccessiblePlugin<RetType, T: QAccessiblePlugin_FreeQAccessiblePlugin<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQAccessiblePlugin(self);
    // return 1;
  }
}

pub trait QAccessiblePlugin_FreeQAccessiblePlugin<RetType> {
  fn FreeQAccessiblePlugin(self , rsthis: &mut QAccessiblePlugin) -> RetType;
}

  // proto:  void QAccessiblePlugin::~QAccessiblePlugin();
impl<'a> /*trait*/ QAccessiblePlugin_FreeQAccessiblePlugin<()> for () {
  fn FreeQAccessiblePlugin(self , rsthis: &mut QAccessiblePlugin) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAccessiblePluginD0Ev()};
     unsafe {_ZN17QAccessiblePluginD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QAccessibleInterface * QAccessiblePlugin::create(const QString & key, QObject * object);
impl /*struct*/ QAccessiblePlugin {
  pub fn create<RetType, T: QAccessiblePlugin_create<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.create(self);
    // return 1;
  }
}

pub trait QAccessiblePlugin_create<RetType> {
  fn create(self , rsthis: &mut QAccessiblePlugin) -> RetType;
}

  // proto:  QAccessibleInterface * QAccessiblePlugin::create(const QString & key, QObject * object);
impl<'a> /*trait*/ QAccessiblePlugin_create<QAccessibleInterface> for (QString, QObject) {
  fn create(self , rsthis: &mut QAccessiblePlugin) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAccessiblePlugin6createERK7QStringP7QObject()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN17QAccessiblePlugin6createERK7QStringP7QObject(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QAccessibleInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// <= body block end

