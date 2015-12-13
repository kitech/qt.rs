// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QStyle * QStyleFactory::create(const QString & );
  fn _ZN13QStyleFactory6createERK7QString(arg0: *const c_void) -> i32;
  // proto: QStringList QStyleFactory::keys();
  fn _ZN13QStyleFactory4keysEv() -> i32;
}

// body block begin
// class sizeof(QStyleFactory)=1
pub struct QStyleFactory {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStyleFactory {
  pub fn create<T: QStyleFactory_create>(&mut self, value: T) -> i32 {
    value.create(self);
    return 1;
  }
}

pub trait QStyleFactory_create {
  fn create(self, this: &mut QStyleFactory) -> i32;
}

// proto: QStyle * QStyleFactory::create(const QString & );
impl<'a> /*trait*/ QStyleFactory_create for (&'a  QString) {
  fn create(self, this: &mut QStyleFactory) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStyleFactory6createERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QStyleFactory6createERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QStyleFactory {
  pub fn keys<T: QStyleFactory_keys>(&mut self, value: T) -> i32 {
    value.keys(self);
    return 1;
  }
}

pub trait QStyleFactory_keys {
  fn keys(self, this: &mut QStyleFactory) -> i32;
}

// proto: QStringList QStyleFactory::keys();
impl<'a> /*trait*/ QStyleFactory_keys for () {
  fn keys(self, this: &mut QStyleFactory) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStyleFactory4keysEv()};
    unsafe {_ZN13QStyleFactory4keysEv()};
    return 1;
  }
}

