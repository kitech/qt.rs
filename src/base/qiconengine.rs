// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qdatastream::QDataStream;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: bool QIconEngine::read(QDataStream & in);
  fn _ZN11QIconEngine4readER11QDataStream(arg0: *mut c_void) -> i32;
  // proto: QString QIconEngine::iconName();
  fn _ZNK11QIconEngine8iconNameEv() -> i32;
  // proto: bool QIconEngine::write(QDataStream & out);
  fn _ZNK11QIconEngine5writeER11QDataStream(arg0: *mut c_void) -> i32;
  // proto: void QIconEngine::virtual_hook(int id, void * data);
  fn _ZN11QIconEngine12virtual_hookEiPv(arg0: c_int, arg1: *mut uint8_t) -> i32;
  // proto: QIconEngine * QIconEngine::clone();
  fn _ZNK11QIconEngine5cloneEv() -> i32;
  // proto: QString QIconEngine::key();
  fn _ZNK11QIconEngine3keyEv() -> i32;
  // proto: void QIconEngine::FreeQIconEngine();
  fn _ZN11QIconEngineD0Ev() -> i32;
}

// body block begin
// class sizeof(QIconEngine)=8
pub struct QIconEngine {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QIconEngine {
  pub fn read<T: QIconEngine_read>(&mut self, value: T) -> i32 {
    value.read(self);
    return 1;
  }
}

pub trait QIconEngine_read {
  fn read(self, this: &mut QIconEngine) -> i32;
}

// proto: bool QIconEngine::read(QDataStream & in);
impl<'a> /*trait*/ QIconEngine_read for (&'a mut QDataStream) {
  fn read(self, this: &mut QIconEngine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QIconEngine4readER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QIconEngine4readER11QDataStream(arg0)};
    return 1;
  }
}

impl /*struct*/ QIconEngine {
  pub fn iconName<T: QIconEngine_iconName>(&mut self, value: T) -> i32 {
    value.iconName(self);
    return 1;
  }
}

pub trait QIconEngine_iconName {
  fn iconName(self, this: &mut QIconEngine) -> i32;
}

// proto: QString QIconEngine::iconName();
impl<'a> /*trait*/ QIconEngine_iconName for () {
  fn iconName(self, this: &mut QIconEngine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QIconEngine8iconNameEv()};
    unsafe {_ZNK11QIconEngine8iconNameEv()};
    return 1;
  }
}

impl /*struct*/ QIconEngine {
  pub fn write<T: QIconEngine_write>(&mut self, value: T) -> i32 {
    value.write(self);
    return 1;
  }
}

pub trait QIconEngine_write {
  fn write(self, this: &mut QIconEngine) -> i32;
}

// proto: bool QIconEngine::write(QDataStream & out);
impl<'a> /*trait*/ QIconEngine_write for (&'a mut QDataStream) {
  fn write(self, this: &mut QIconEngine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QIconEngine5writeER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK11QIconEngine5writeER11QDataStream(arg0)};
    return 1;
  }
}

impl /*struct*/ QIconEngine {
  pub fn virtual_hook<T: QIconEngine_virtual_hook>(&mut self, value: T) -> i32 {
    value.virtual_hook(self);
    return 1;
  }
}

pub trait QIconEngine_virtual_hook {
  fn virtual_hook(self, this: &mut QIconEngine) -> i32;
}

// proto: void QIconEngine::virtual_hook(int id, void * data);
impl<'a> /*trait*/ QIconEngine_virtual_hook for (i32, &'a mut u8) {
  fn virtual_hook(self, this: &mut QIconEngine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QIconEngine12virtual_hookEiPv()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *mut uint8_t;
    unsafe {_ZN11QIconEngine12virtual_hookEiPv(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QIconEngine {
  pub fn clone<T: QIconEngine_clone>(&mut self, value: T) -> i32 {
    value.clone(self);
    return 1;
  }
}

pub trait QIconEngine_clone {
  fn clone(self, this: &mut QIconEngine) -> i32;
}

// proto: QIconEngine * QIconEngine::clone();
impl<'a> /*trait*/ QIconEngine_clone for () {
  fn clone(self, this: &mut QIconEngine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QIconEngine5cloneEv()};
    unsafe {_ZNK11QIconEngine5cloneEv()};
    return 1;
  }
}

impl /*struct*/ QIconEngine {
  pub fn key<T: QIconEngine_key>(&mut self, value: T) -> i32 {
    value.key(self);
    return 1;
  }
}

pub trait QIconEngine_key {
  fn key(self, this: &mut QIconEngine) -> i32;
}

// proto: QString QIconEngine::key();
impl<'a> /*trait*/ QIconEngine_key for () {
  fn key(self, this: &mut QIconEngine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QIconEngine3keyEv()};
    unsafe {_ZNK11QIconEngine3keyEv()};
    return 1;
  }
}

impl /*struct*/ QIconEngine {
  pub fn FreeQIconEngine<T: QIconEngine_FreeQIconEngine>(&mut self, value: T) -> i32 {
    value.FreeQIconEngine(self);
    return 1;
  }
}

pub trait QIconEngine_FreeQIconEngine {
  fn FreeQIconEngine(self, this: &mut QIconEngine) -> i32;
}

// proto: void QIconEngine::FreeQIconEngine();
impl<'a> /*trait*/ QIconEngine_FreeQIconEngine for () {
  fn FreeQIconEngine(self, this: &mut QIconEngine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QIconEngineD0Ev()};
    unsafe {_ZN11QIconEngineD0Ev()};
    return 1;
  }
}

