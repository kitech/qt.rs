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
  fn _ZN24QXmlStreamEntityResolver13resolveEntityERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZN24QXmlStreamEntityResolver23resolveUndeclaredEntityERK7QString(arg0: *const c_void) -> i32;
  fn _ZN24QXmlStreamEntityResolverD0Ev() -> i32;
}

// body block begin
// class sizeof(QXmlStreamEntityResolver)=8
pub struct QXmlStreamEntityResolver {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QXmlStreamEntityResolver {
  pub fn resolveEntity<T: QXmlStreamEntityResolver_resolveEntity>(&mut self, value: T) -> i32 {
    value.resolveEntity(self);
    return 1;
  }
}

pub trait QXmlStreamEntityResolver_resolveEntity {
  fn resolveEntity(self, this: &mut QXmlStreamEntityResolver) -> i32;
}

// proto: QString QXmlStreamEntityResolver::resolveEntity(const QString & publicId, const QString & systemId);
impl<'a> /*trait*/ QXmlStreamEntityResolver_resolveEntity for (&'a  QString, &'a  QString) {
  fn resolveEntity(self, this: &mut QXmlStreamEntityResolver) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QXmlStreamEntityResolver13resolveEntityERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN24QXmlStreamEntityResolver13resolveEntityERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QXmlStreamEntityResolver {
  pub fn resolveUndeclaredEntity<T: QXmlStreamEntityResolver_resolveUndeclaredEntity>(&mut self, value: T) -> i32 {
    value.resolveUndeclaredEntity(self);
    return 1;
  }
}

pub trait QXmlStreamEntityResolver_resolveUndeclaredEntity {
  fn resolveUndeclaredEntity(self, this: &mut QXmlStreamEntityResolver) -> i32;
}

// proto: QString QXmlStreamEntityResolver::resolveUndeclaredEntity(const QString & name);
impl<'a> /*trait*/ QXmlStreamEntityResolver_resolveUndeclaredEntity for (&'a  QString) {
  fn resolveUndeclaredEntity(self, this: &mut QXmlStreamEntityResolver) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QXmlStreamEntityResolver23resolveUndeclaredEntityERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN24QXmlStreamEntityResolver23resolveUndeclaredEntityERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QXmlStreamEntityResolver {
  pub fn FreeQXmlStreamEntityResolver<T: QXmlStreamEntityResolver_FreeQXmlStreamEntityResolver>(&mut self, value: T) -> i32 {
    value.FreeQXmlStreamEntityResolver(self);
    return 1;
  }
}

pub trait QXmlStreamEntityResolver_FreeQXmlStreamEntityResolver {
  fn FreeQXmlStreamEntityResolver(self, this: &mut QXmlStreamEntityResolver) -> i32;
}

// proto: void QXmlStreamEntityResolver::FreeQXmlStreamEntityResolver();
impl<'a> /*trait*/ QXmlStreamEntityResolver_FreeQXmlStreamEntityResolver for () {
  fn FreeQXmlStreamEntityResolver(self, this: &mut QXmlStreamEntityResolver) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QXmlStreamEntityResolverD0Ev()};
    unsafe {_ZN24QXmlStreamEntityResolverD0Ev()};
    return 1;
  }
}

