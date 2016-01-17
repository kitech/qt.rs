// auto generated, do not modify.
// created: Sun Jan 17 17:37:11 2016
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
use std::ops::Deref;
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QStaticPlugin_Class_Size() -> c_int;
  // proto:  QJsonObject QStaticPlugin::metaData();
  fn _ZNK13QStaticPlugin8metaDataEv(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QStaticPlugin)=16
#[derive(Default)]
pub struct QStaticPlugin {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QStaticPlugin {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStaticPlugin {
    return QStaticPlugin{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QJsonObject QStaticPlugin::metaData();
impl /*struct*/ QStaticPlugin {
  pub fn metaData<RetType, T: QStaticPlugin_metaData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaData(self);
    // return 1;
  }
}

pub trait QStaticPlugin_metaData<RetType> {
  fn metaData(self , rsthis: & QStaticPlugin) -> RetType;
}

  // proto:  QJsonObject QStaticPlugin::metaData();
impl<'a> /*trait*/ QStaticPlugin_metaData<()> for () {
  fn metaData(self , rsthis: & QStaticPlugin) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStaticPlugin8metaDataEv()};
     unsafe {_ZNK13QStaticPlugin8metaDataEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

