// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
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
use super::super::core::qstring::QString; // 771
use super::super::core::qsize::QSize; // 771
use super::super::core::qdatastream::QDataStream; // 771
use super::qpainter::QPainter; // 773
use super::super::core::qrect::QRect; // 771
use super::qpixmap::QPixmap; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  bool QIconEngine::read(QDataStream & in);
  fn _ZN11QIconEngine4readER11QDataStream(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  QString QIconEngine::iconName();
  fn _ZNK11QIconEngine8iconNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QIconEngine::write(QDataStream & out);
  fn _ZNK11QIconEngine5writeER11QDataStream(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  void QIconEngine::virtual_hook(int id, void * data);
  fn _ZN11QIconEngine12virtual_hookEiPv(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  QIconEngine * QIconEngine::clone();
  fn _ZNK11QIconEngine5cloneEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QIconEngine::key();
  fn _ZNK11QIconEngine3keyEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QIconEngine::~QIconEngine();
  fn _ZN11QIconEngineD0Ev(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QIconEngine)=8
pub struct QIconEngine {
  pub qclsinst: *mut c_void,
}

  // proto:  bool QIconEngine::read(QDataStream & in);
impl /*struct*/ QIconEngine {
  pub fn read<RetType, T: QIconEngine_read<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.read(self);
    // return 1;
  }
}

pub trait QIconEngine_read<RetType> {
  fn read(self , rsthis: &mut QIconEngine) -> RetType;
}

  // proto:  bool QIconEngine::read(QDataStream & in);
impl<'a> /*trait*/ QIconEngine_read<i8> for (QDataStream) {
  fn read(self , rsthis: &mut QIconEngine) -> i8 {
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
  pub fn iconName<RetType, T: QIconEngine_iconName<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.iconName(self);
    // return 1;
  }
}

pub trait QIconEngine_iconName<RetType> {
  fn iconName(self , rsthis: &mut QIconEngine) -> RetType;
}

  // proto:  QString QIconEngine::iconName();
impl<'a> /*trait*/ QIconEngine_iconName<QString> for () {
  fn iconName(self , rsthis: &mut QIconEngine) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QIconEngine8iconNameEv()};
    let mut ret = unsafe {_ZNK11QIconEngine8iconNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QIconEngine::write(QDataStream & out);
impl /*struct*/ QIconEngine {
  pub fn write<RetType, T: QIconEngine_write<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.write(self);
    // return 1;
  }
}

pub trait QIconEngine_write<RetType> {
  fn write(self , rsthis: &mut QIconEngine) -> RetType;
}

  // proto:  bool QIconEngine::write(QDataStream & out);
impl<'a> /*trait*/ QIconEngine_write<i8> for (QDataStream) {
  fn write(self , rsthis: &mut QIconEngine) -> i8 {
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
  pub fn virtual_hook<RetType, T: QIconEngine_virtual_hook<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.virtual_hook(self);
    // return 1;
  }
}

pub trait QIconEngine_virtual_hook<RetType> {
  fn virtual_hook(self , rsthis: &mut QIconEngine) -> RetType;
}

  // proto:  void QIconEngine::virtual_hook(int id, void * data);
impl<'a> /*trait*/ QIconEngine_virtual_hook<()> for (i32, *mut c_void) {
  fn virtual_hook(self , rsthis: &mut QIconEngine) -> () {
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
  pub fn clone<RetType, T: QIconEngine_clone<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.clone(self);
    // return 1;
  }
}

pub trait QIconEngine_clone<RetType> {
  fn clone(self , rsthis: &mut QIconEngine) -> RetType;
}

  // proto:  QIconEngine * QIconEngine::clone();
impl<'a> /*trait*/ QIconEngine_clone<QIconEngine> for () {
  fn clone(self , rsthis: &mut QIconEngine) -> QIconEngine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QIconEngine5cloneEv()};
    let mut ret = unsafe {_ZNK11QIconEngine5cloneEv(rsthis.qclsinst)};
    let mut ret1 = QIconEngine{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QIconEngine::key();
impl /*struct*/ QIconEngine {
  pub fn key<RetType, T: QIconEngine_key<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.key(self);
    // return 1;
  }
}

pub trait QIconEngine_key<RetType> {
  fn key(self , rsthis: &mut QIconEngine) -> RetType;
}

  // proto:  QString QIconEngine::key();
impl<'a> /*trait*/ QIconEngine_key<QString> for () {
  fn key(self , rsthis: &mut QIconEngine) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QIconEngine3keyEv()};
    let mut ret = unsafe {_ZNK11QIconEngine3keyEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QIconEngine::~QIconEngine();
impl /*struct*/ QIconEngine {
  pub fn FreeQIconEngine<RetType, T: QIconEngine_FreeQIconEngine<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQIconEngine(self);
    // return 1;
  }
}

pub trait QIconEngine_FreeQIconEngine<RetType> {
  fn FreeQIconEngine(self , rsthis: &mut QIconEngine) -> RetType;
}

  // proto:  void QIconEngine::~QIconEngine();
impl<'a> /*trait*/ QIconEngine_FreeQIconEngine<()> for () {
  fn FreeQIconEngine(self , rsthis: &mut QIconEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QIconEngineD0Ev()};
     unsafe {_ZN11QIconEngineD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

