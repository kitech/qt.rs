// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;
use super::qstring::QString;
use super::qaccessiblebridge::QAccessibleBridge;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QAccessibleBridgePlugin::QAccessibleBridgePlugin(QObject * parent);
  fn _ZN23QAccessibleBridgePluginC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QAccessibleBridge * QAccessibleBridgePlugin::create(const QString & key);
  fn _ZN23QAccessibleBridgePlugin6createERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QAccessibleBridgePlugin::~QAccessibleBridgePlugin();
  fn _ZN23QAccessibleBridgePluginD0Ev(qthis: *mut c_void);
  // proto:  const QMetaObject * QAccessibleBridgePlugin::metaObject();
  fn _ZNK23QAccessibleBridgePlugin10metaObjectEv(qthis: *mut c_void);
}

// body block begin
// class sizeof(QAccessibleBridgePlugin)=1
pub struct QAccessibleBridgePlugin {
  pub qclsinst: *mut c_void,
}

  // proto:  void QAccessibleBridgePlugin::QAccessibleBridgePlugin(QObject * parent);
impl /*struct*/ QAccessibleBridgePlugin {
  pub fn NewQAccessibleBridgePlugin<T: QAccessibleBridgePlugin_NewQAccessibleBridgePlugin>(value: T) -> QAccessibleBridgePlugin {
    let rsthis = value.NewQAccessibleBridgePlugin();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleBridgePlugin_NewQAccessibleBridgePlugin {
  fn NewQAccessibleBridgePlugin(self) -> QAccessibleBridgePlugin;
}

  // proto:  void QAccessibleBridgePlugin::QAccessibleBridgePlugin(QObject * parent);
impl<'a> /*trait*/ QAccessibleBridgePlugin_NewQAccessibleBridgePlugin for (QObject) {
  fn NewQAccessibleBridgePlugin(self) -> QAccessibleBridgePlugin {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QAccessibleBridgePluginC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN23QAccessibleBridgePluginC1EP7QObject(qthis, arg0)};
    let rsthis = QAccessibleBridgePlugin{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QAccessibleBridge * QAccessibleBridgePlugin::create(const QString & key);
impl /*struct*/ QAccessibleBridgePlugin {
  pub fn create<RetType, T: QAccessibleBridgePlugin_create<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.create(self);
    // return 1;
  }
}

pub trait QAccessibleBridgePlugin_create<RetType> {
  fn create(self , rsthis: &mut QAccessibleBridgePlugin) -> RetType;
}

  // proto:  QAccessibleBridge * QAccessibleBridgePlugin::create(const QString & key);
impl<'a> /*trait*/ QAccessibleBridgePlugin_create<QAccessibleBridge> for (QString) {
  fn create(self , rsthis: &mut QAccessibleBridgePlugin) -> QAccessibleBridge {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QAccessibleBridgePlugin6createERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN23QAccessibleBridgePlugin6createERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QAccessibleBridge{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QAccessibleBridgePlugin::~QAccessibleBridgePlugin();
impl /*struct*/ QAccessibleBridgePlugin {
  pub fn FreeQAccessibleBridgePlugin<RetType, T: QAccessibleBridgePlugin_FreeQAccessibleBridgePlugin<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQAccessibleBridgePlugin(self);
    // return 1;
  }
}

pub trait QAccessibleBridgePlugin_FreeQAccessibleBridgePlugin<RetType> {
  fn FreeQAccessibleBridgePlugin(self , rsthis: &mut QAccessibleBridgePlugin) -> RetType;
}

  // proto:  void QAccessibleBridgePlugin::~QAccessibleBridgePlugin();
impl<'a> /*trait*/ QAccessibleBridgePlugin_FreeQAccessibleBridgePlugin<()> for () {
  fn FreeQAccessibleBridgePlugin(self , rsthis: &mut QAccessibleBridgePlugin) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QAccessibleBridgePluginD0Ev()};
     unsafe {_ZN23QAccessibleBridgePluginD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QAccessibleBridgePlugin::metaObject();
impl /*struct*/ QAccessibleBridgePlugin {
  pub fn metaObject<RetType, T: QAccessibleBridgePlugin_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QAccessibleBridgePlugin_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QAccessibleBridgePlugin) -> RetType;
}

  // proto:  const QMetaObject * QAccessibleBridgePlugin::metaObject();
impl<'a> /*trait*/ QAccessibleBridgePlugin_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QAccessibleBridgePlugin) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QAccessibleBridgePlugin10metaObjectEv()};
     unsafe {_ZNK23QAccessibleBridgePlugin10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

