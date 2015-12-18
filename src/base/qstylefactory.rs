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

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: static QStyle * QStyleFactory::create(const QString & );
  fn _ZN13QStyleFactory6createERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto: static QStringList QStyleFactory::keys();
  fn _ZN13QStyleFactory4keysEv() ;
}

// body block begin
// class sizeof(QStyleFactory)=1
pub struct QStyleFactory {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStyleFactory {
  pub fn create<RetType, T: QStyleFactory_create<RetType>>(&mut self, value: T) -> RetType {
    return value.create(self);
    // return 1;
  }
}

pub trait QStyleFactory_create<RetType> {
  fn create(self, rsthis: &mut QStyleFactory) -> RetType;
}

// proto: static QStyle * QStyleFactory::create(const QString & );
impl<'a> /*trait*/ QStyleFactory_create<QStyle> for (&'a  QString) {
  fn create(self, rsthis: &mut QStyleFactory) -> QStyle {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStyleFactory6createERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN13QStyleFactory6createERK7QString(arg0)};
    let mut ret1 = QStyle{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStyleFactory {
  pub fn keys<RetType, T: QStyleFactory_keys<RetType>>(&mut self, value: T) -> RetType {
    return value.keys(self);
    // return 1;
  }
}

pub trait QStyleFactory_keys<RetType> {
  fn keys(self, rsthis: &mut QStyleFactory) -> RetType;
}

// proto: static QStringList QStyleFactory::keys();
impl<'a> /*trait*/ QStyleFactory_keys<()> for () {
  fn keys(self, rsthis: &mut QStyleFactory) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStyleFactory4keysEv()};
     unsafe {_ZN13QStyleFactory4keysEv()};
    // return 1;
  }
}

