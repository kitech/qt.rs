// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qpicture::QPicture;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  bool QPictureFormatPlugin::loadPicture(const QString & format, const QString & filename, QPicture * pic);
  fn _ZN20QPictureFormatPlugin11loadPictureERK7QStringS2_P8QPicture(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> int8_t;
  // proto:  bool QPictureFormatPlugin::savePicture(const QString & format, const QString & filename, const QPicture & pic);
  fn _ZN20QPictureFormatPlugin11savePictureERK7QStringS2_RK8QPicture(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> int8_t;
  // proto:  void QPictureFormatPlugin::FreeQPictureFormatPlugin();
  fn _ZN20QPictureFormatPluginD0Ev(qthis: *mut c_void) ;
  // proto:  void QPictureFormatPlugin::NewQPictureFormatPlugin(QObject * parent);
  fn _ZN20QPictureFormatPluginC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QPictureFormatPlugin::installIOHandler(const QString & format);
  fn _ZN20QPictureFormatPlugin16installIOHandlerERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  const QMetaObject * QPictureFormatPlugin::metaObject();
  fn _ZNK20QPictureFormatPlugin10metaObjectEv(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QPictureFormatPlugin)=1
pub struct QPictureFormatPlugin {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPictureFormatPlugin {
  pub fn loadPicture<T: QPictureFormatPlugin_loadPicture>(&mut self, value: T) -> i8 {
    return value.loadPicture(self);
    // return 1;
  }
}

pub trait QPictureFormatPlugin_loadPicture {
  fn loadPicture(self, rsthis: &mut QPictureFormatPlugin) -> i8;
}

// proto:  bool QPictureFormatPlugin::loadPicture(const QString & format, const QString & filename, QPicture * pic);
impl<'a> /*trait*/ QPictureFormatPlugin_loadPicture for (&'a  QString, &'a  QString, &'a mut QPicture) {
  fn loadPicture(self, rsthis: &mut QPictureFormatPlugin) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QPictureFormatPlugin11loadPictureERK7QStringS2_P8QPicture()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN20QPictureFormatPlugin11loadPictureERK7QStringS2_P8QPicture(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPictureFormatPlugin {
  pub fn savePicture<T: QPictureFormatPlugin_savePicture>(&mut self, value: T) -> i8 {
    return value.savePicture(self);
    // return 1;
  }
}

pub trait QPictureFormatPlugin_savePicture {
  fn savePicture(self, rsthis: &mut QPictureFormatPlugin) -> i8;
}

// proto:  bool QPictureFormatPlugin::savePicture(const QString & format, const QString & filename, const QPicture & pic);
impl<'a> /*trait*/ QPictureFormatPlugin_savePicture for (&'a  QString, &'a  QString, &'a  QPicture) {
  fn savePicture(self, rsthis: &mut QPictureFormatPlugin) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QPictureFormatPlugin11savePictureERK7QStringS2_RK8QPicture()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN20QPictureFormatPlugin11savePictureERK7QStringS2_RK8QPicture(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPictureFormatPlugin {
  pub fn FreeQPictureFormatPlugin<T: QPictureFormatPlugin_FreeQPictureFormatPlugin>(&mut self, value: T)  {
     value.FreeQPictureFormatPlugin(self);
    // return 1;
  }
}

pub trait QPictureFormatPlugin_FreeQPictureFormatPlugin {
  fn FreeQPictureFormatPlugin(self, rsthis: &mut QPictureFormatPlugin) ;
}

// proto:  void QPictureFormatPlugin::FreeQPictureFormatPlugin();
impl<'a> /*trait*/ QPictureFormatPlugin_FreeQPictureFormatPlugin for () {
  fn FreeQPictureFormatPlugin(self, rsthis: &mut QPictureFormatPlugin)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QPictureFormatPluginD0Ev()};
     unsafe {_ZN20QPictureFormatPluginD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPictureFormatPlugin {
  pub fn NewQPictureFormatPlugin<T: QPictureFormatPlugin_NewQPictureFormatPlugin>(value: T) -> QPictureFormatPlugin {
    let rsthis = value.NewQPictureFormatPlugin();
    return rsthis;
    // return 1;
  }
}

pub trait QPictureFormatPlugin_NewQPictureFormatPlugin {
  fn NewQPictureFormatPlugin(self) -> QPictureFormatPlugin;
}

// proto: void QPictureFormatPlugin::NewQPictureFormatPlugin(QObject * parent);
impl<'a> /*trait*/ QPictureFormatPlugin_NewQPictureFormatPlugin for (&'a mut QObject) {
  fn NewQPictureFormatPlugin(self) -> QPictureFormatPlugin {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QPictureFormatPluginC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN20QPictureFormatPluginC1EP7QObject(qthis, arg0)};
    let rsthis = QPictureFormatPlugin{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPictureFormatPlugin {
  pub fn installIOHandler<T: QPictureFormatPlugin_installIOHandler>(&mut self, value: T) -> i8 {
    return value.installIOHandler(self);
    // return 1;
  }
}

pub trait QPictureFormatPlugin_installIOHandler {
  fn installIOHandler(self, rsthis: &mut QPictureFormatPlugin) -> i8;
}

// proto:  bool QPictureFormatPlugin::installIOHandler(const QString & format);
impl<'a> /*trait*/ QPictureFormatPlugin_installIOHandler for (&'a  QString) {
  fn installIOHandler(self, rsthis: &mut QPictureFormatPlugin) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QPictureFormatPlugin16installIOHandlerERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN20QPictureFormatPlugin16installIOHandlerERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPictureFormatPlugin {
  pub fn metaObject<T: QPictureFormatPlugin_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QPictureFormatPlugin_metaObject {
  fn metaObject(self, rsthis: &mut QPictureFormatPlugin) ;
}

// proto:  const QMetaObject * QPictureFormatPlugin::metaObject();
impl<'a> /*trait*/ QPictureFormatPlugin_metaObject for () {
  fn metaObject(self, rsthis: &mut QPictureFormatPlugin)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QPictureFormatPlugin10metaObjectEv()};
     unsafe {_ZNK20QPictureFormatPlugin10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

