// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qstyle::QStyle;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QStyle * QStylePlugin::create(const QString & key);
  fn _ZN12QStylePlugin6createERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QStylePlugin::metaObject();
  fn _ZNK12QStylePlugin10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QStylePlugin::NewQStylePlugin(QObject * parent);
  fn _ZN12QStylePluginC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QStylePlugin::FreeQStylePlugin();
  fn _ZN12QStylePluginD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QStylePlugin)=1
pub struct QStylePlugin {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStylePlugin {
  pub fn create<RetType, T: QStylePlugin_create<RetType>>(&mut self, value: T) -> RetType {
    return value.create(self);
    // return 1;
  }
}

pub trait QStylePlugin_create<RetType> {
  fn create(self, rsthis: &mut QStylePlugin) -> RetType;
}

// proto:  QStyle * QStylePlugin::create(const QString & key);
impl<'a> /*trait*/ QStylePlugin_create<QStyle> for (&'a  QString) {
  fn create(self, rsthis: &mut QStylePlugin) -> QStyle {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStylePlugin6createERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QStylePlugin6createERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QStyle{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStylePlugin {
  pub fn metaObject<RetType, T: QStylePlugin_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QStylePlugin_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QStylePlugin) -> RetType;
}

// proto:  const QMetaObject * QStylePlugin::metaObject();
impl<'a> /*trait*/ QStylePlugin_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QStylePlugin) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QStylePlugin10metaObjectEv()};
     unsafe {_ZNK12QStylePlugin10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QStylePlugin {
  pub fn NewQStylePlugin<T: QStylePlugin_NewQStylePlugin>(value: T) -> QStylePlugin {
    let rsthis = value.NewQStylePlugin();
    return rsthis;
    // return 1;
  }
}

pub trait QStylePlugin_NewQStylePlugin {
  fn NewQStylePlugin(self) -> QStylePlugin;
}

// proto: void QStylePlugin::NewQStylePlugin(QObject * parent);
impl<'a> /*trait*/ QStylePlugin_NewQStylePlugin for (&'a mut QObject) {
  fn NewQStylePlugin(self) -> QStylePlugin {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStylePluginC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QStylePluginC1EP7QObject(qthis, arg0)};
    let rsthis = QStylePlugin{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStylePlugin {
  pub fn FreeQStylePlugin<RetType, T: QStylePlugin_FreeQStylePlugin<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQStylePlugin(self);
    // return 1;
  }
}

pub trait QStylePlugin_FreeQStylePlugin<RetType> {
  fn FreeQStylePlugin(self, rsthis: &mut QStylePlugin) -> RetType;
}

// proto:  void QStylePlugin::FreeQStylePlugin();
impl<'a> /*trait*/ QStylePlugin_FreeQStylePlugin<()> for () {
  fn FreeQStylePlugin(self, rsthis: &mut QStylePlugin) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStylePluginD0Ev()};
     unsafe {_ZN12QStylePluginD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

