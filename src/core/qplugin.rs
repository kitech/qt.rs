// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtCore/qplugin.h
// dst-file: /src/core/qplugin.rs
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
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  QJsonObject QStaticPlugin::metaData();
  fn _ZNK13QStaticPlugin8metaDataEv(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QStaticPlugin)=16
pub struct QStaticPlugin {
  pub qclsinst: *mut c_void,
}

  // proto:  QJsonObject QStaticPlugin::metaData();
impl /*struct*/ QStaticPlugin {
  pub fn metaData<RetType, T: QStaticPlugin_metaData<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaData(self);
    // return 1;
  }
}

pub trait QStaticPlugin_metaData<RetType> {
  fn metaData(self , rsthis: &mut QStaticPlugin) -> RetType;
}

  // proto:  QJsonObject QStaticPlugin::metaData();
impl<'a> /*trait*/ QStaticPlugin_metaData<()> for () {
  fn metaData(self , rsthis: &mut QStaticPlugin) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStaticPlugin8metaDataEv()};
     unsafe {_ZNK13QStaticPlugin8metaDataEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

