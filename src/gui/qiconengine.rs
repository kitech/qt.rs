// auto generated, do not modify.
// created: Fri Jan  1 15:54:32 2016
// src-file: /QtGui/qiconengine.h
// dst-file: /src/gui/qiconengine.rs
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
use super::super::core::qstring::QString; // 771
use super::super::core::qsize::QSize; // 771
use super::super::core::qdatastream::QDataStream; // 771
use super::qpainter::QPainter; // 773
use super::super::core::qrect::QRect; // 771
use super::qpixmap::QPixmap; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QIconEngine_Class_Size() -> c_int;
  // proto:  bool QIconEngine::read(QDataStream & in);
  fn _ZN11QIconEngine4readER11QDataStream(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QString QIconEngine::iconName();
  fn _ZNK11QIconEngine8iconNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QIconEngine::write(QDataStream & out);
  fn _ZNK11QIconEngine5writeER11QDataStream(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  void QIconEngine::virtual_hook(int id, void * data);
  fn _ZN11QIconEngine12virtual_hookEiPv(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  QIconEngine * QIconEngine::clone();
  fn _ZNK11QIconEngine5cloneEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QIconEngine::key();
  fn _ZNK11QIconEngine3keyEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QIconEngine::~QIconEngine();
  fn _ZN11QIconEngineD0Ev(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QIconEngine)=8
#[derive(Default)]
pub struct QIconEngine {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QIconEngine {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QIconEngine {
    return QIconEngine{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  bool QIconEngine::read(QDataStream & in);
impl /*struct*/ QIconEngine {
  pub fn read<RetType, T: QIconEngine_read<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.read(self);
    // return 1;
  }
}

pub trait QIconEngine_read<RetType> {
  fn read(self , rsthis: & QIconEngine) -> RetType;
}

  // proto:  bool QIconEngine::read(QDataStream & in);
impl<'a> /*trait*/ QIconEngine_read<i8> for (&'a QDataStream) {
  fn read(self , rsthis: & QIconEngine) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QIconEngine4readER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN11QIconEngine4readER11QDataStream(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QIconEngine::iconName();
impl /*struct*/ QIconEngine {
  pub fn iconName<RetType, T: QIconEngine_iconName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.iconName(self);
    // return 1;
  }
}

pub trait QIconEngine_iconName<RetType> {
  fn iconName(self , rsthis: & QIconEngine) -> RetType;
}

  // proto:  QString QIconEngine::iconName();
impl<'a> /*trait*/ QIconEngine_iconName<QString> for () {
  fn iconName(self , rsthis: & QIconEngine) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QIconEngine8iconNameEv()};
    let mut ret = unsafe {_ZNK11QIconEngine8iconNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QIconEngine::write(QDataStream & out);
impl /*struct*/ QIconEngine {
  pub fn write<RetType, T: QIconEngine_write<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.write(self);
    // return 1;
  }
}

pub trait QIconEngine_write<RetType> {
  fn write(self , rsthis: & QIconEngine) -> RetType;
}

  // proto:  bool QIconEngine::write(QDataStream & out);
impl<'a> /*trait*/ QIconEngine_write<i8> for (&'a QDataStream) {
  fn write(self , rsthis: & QIconEngine) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QIconEngine5writeER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QIconEngine5writeER11QDataStream(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QIconEngine::virtual_hook(int id, void * data);
impl /*struct*/ QIconEngine {
  pub fn virtual_hook<RetType, T: QIconEngine_virtual_hook<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.virtual_hook(self);
    // return 1;
  }
}

pub trait QIconEngine_virtual_hook<RetType> {
  fn virtual_hook(self , rsthis: & QIconEngine) -> RetType;
}

  // proto:  void QIconEngine::virtual_hook(int id, void * data);
impl<'a> /*trait*/ QIconEngine_virtual_hook<()> for (i32, *mut c_void) {
  fn virtual_hook(self , rsthis: & QIconEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QIconEngine12virtual_hookEiPv()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *mut c_void;
     unsafe {_ZN11QIconEngine12virtual_hookEiPv(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QIconEngine * QIconEngine::clone();
impl /*struct*/ QIconEngine {
  pub fn clone<RetType, T: QIconEngine_clone<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clone(self);
    // return 1;
  }
}

pub trait QIconEngine_clone<RetType> {
  fn clone(self , rsthis: & QIconEngine) -> RetType;
}

  // proto:  QIconEngine * QIconEngine::clone();
impl<'a> /*trait*/ QIconEngine_clone<QIconEngine> for () {
  fn clone(self , rsthis: & QIconEngine) -> QIconEngine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QIconEngine5cloneEv()};
    let mut ret = unsafe {_ZNK11QIconEngine5cloneEv(rsthis.qclsinst)};
    let mut ret1 = QIconEngine::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QIconEngine::key();
impl /*struct*/ QIconEngine {
  pub fn key<RetType, T: QIconEngine_key<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.key(self);
    // return 1;
  }
}

pub trait QIconEngine_key<RetType> {
  fn key(self , rsthis: & QIconEngine) -> RetType;
}

  // proto:  QString QIconEngine::key();
impl<'a> /*trait*/ QIconEngine_key<QString> for () {
  fn key(self , rsthis: & QIconEngine) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QIconEngine3keyEv()};
    let mut ret = unsafe {_ZNK11QIconEngine3keyEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QIconEngine::~QIconEngine();
impl /*struct*/ QIconEngine {
  pub fn free<RetType, T: QIconEngine_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QIconEngine_free<RetType> {
  fn free(self , rsthis: & QIconEngine) -> RetType;
}

  // proto:  void QIconEngine::~QIconEngine();
impl<'a> /*trait*/ QIconEngine_free<()> for () {
  fn free(self , rsthis: & QIconEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QIconEngineD0Ev()};
     unsafe {_ZN11QIconEngineD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

