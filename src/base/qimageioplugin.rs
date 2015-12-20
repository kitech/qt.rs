// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qiodevice::QIODevice;
use super::qbytearray::QByteArray;
use super::qimageiohandler::QImageIOHandler;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  const QMetaObject * QImageIOPlugin::metaObject();
  fn _ZNK14QImageIOPlugin10metaObjectEv(qthis: *mut c_void);
  // proto:  void QImageIOPlugin::~QImageIOPlugin();
  fn _ZN14QImageIOPluginD0Ev(qthis: *mut c_void);
  // proto:  QImageIOHandler * QImageIOPlugin::create(QIODevice * device, const QByteArray & format);
  fn _ZNK14QImageIOPlugin6createEP9QIODeviceRK10QByteArray(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QImageIOPlugin::QImageIOPlugin(QObject * parent);
  fn _ZN14QImageIOPluginC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
}

// body block begin
// class sizeof(QImageIOPlugin)=1
pub struct QImageIOPlugin {
  pub qclsinst: *mut c_void,
}

  // proto:  const QMetaObject * QImageIOPlugin::metaObject();
impl /*struct*/ QImageIOPlugin {
  pub fn metaObject<RetType, T: QImageIOPlugin_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QImageIOPlugin_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QImageIOPlugin) -> RetType;
}

  // proto:  const QMetaObject * QImageIOPlugin::metaObject();
impl<'a> /*trait*/ QImageIOPlugin_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QImageIOPlugin) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QImageIOPlugin10metaObjectEv()};
     unsafe {_ZNK14QImageIOPlugin10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QImageIOPlugin::~QImageIOPlugin();
impl /*struct*/ QImageIOPlugin {
  pub fn FreeQImageIOPlugin<RetType, T: QImageIOPlugin_FreeQImageIOPlugin<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQImageIOPlugin(self);
    // return 1;
  }
}

pub trait QImageIOPlugin_FreeQImageIOPlugin<RetType> {
  fn FreeQImageIOPlugin(self , rsthis: &mut QImageIOPlugin) -> RetType;
}

  // proto:  void QImageIOPlugin::~QImageIOPlugin();
impl<'a> /*trait*/ QImageIOPlugin_FreeQImageIOPlugin<()> for () {
  fn FreeQImageIOPlugin(self , rsthis: &mut QImageIOPlugin) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QImageIOPluginD0Ev()};
     unsafe {_ZN14QImageIOPluginD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QImageIOHandler * QImageIOPlugin::create(QIODevice * device, const QByteArray & format);
impl /*struct*/ QImageIOPlugin {
  pub fn create<RetType, T: QImageIOPlugin_create<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.create(self);
    // return 1;
  }
}

pub trait QImageIOPlugin_create<RetType> {
  fn create(self , rsthis: &mut QImageIOPlugin) -> RetType;
}

  // proto:  QImageIOHandler * QImageIOPlugin::create(QIODevice * device, const QByteArray & format);
impl<'a> /*trait*/ QImageIOPlugin_create<QImageIOHandler> for (QIODevice, QByteArray) {
  fn create(self , rsthis: &mut QImageIOPlugin) -> QImageIOHandler {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QImageIOPlugin6createEP9QIODeviceRK10QByteArray()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK14QImageIOPlugin6createEP9QIODeviceRK10QByteArray(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QImageIOHandler{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QImageIOPlugin::QImageIOPlugin(QObject * parent);
impl /*struct*/ QImageIOPlugin {
  pub fn NewQImageIOPlugin<T: QImageIOPlugin_NewQImageIOPlugin>(value: T) -> QImageIOPlugin {
    let rsthis = value.NewQImageIOPlugin();
    return rsthis;
    // return 1;
  }
}

pub trait QImageIOPlugin_NewQImageIOPlugin {
  fn NewQImageIOPlugin(self) -> QImageIOPlugin;
}

  // proto:  void QImageIOPlugin::QImageIOPlugin(QObject * parent);
impl<'a> /*trait*/ QImageIOPlugin_NewQImageIOPlugin for (QObject) {
  fn NewQImageIOPlugin(self) -> QImageIOPlugin {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QImageIOPluginC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QImageIOPluginC1EP7QObject(qthis, arg0)};
    let rsthis = QImageIOPlugin{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

