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
  // proto:  QString QXmlStreamEntityResolver::resolveEntity(const QString & publicId, const QString & systemId);
  fn _ZN24QXmlStreamEntityResolver13resolveEntityERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QString QXmlStreamEntityResolver::resolveUndeclaredEntity(const QString & name);
  fn _ZN24QXmlStreamEntityResolver23resolveUndeclaredEntityERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QXmlStreamEntityResolver::FreeQXmlStreamEntityResolver();
  fn _ZN24QXmlStreamEntityResolverD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QXmlStreamEntityResolver)=8
pub struct QXmlStreamEntityResolver {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QXmlStreamEntityResolver {
  pub fn resolveEntity<T: QXmlStreamEntityResolver_resolveEntity>(&mut self, value: T) -> QString {
    return value.resolveEntity(self);
    // return 1;
  }
}

pub trait QXmlStreamEntityResolver_resolveEntity {
  fn resolveEntity(self, rsthis: &mut QXmlStreamEntityResolver) -> QString;
}

// proto:  QString QXmlStreamEntityResolver::resolveEntity(const QString & publicId, const QString & systemId);
impl<'a> /*trait*/ QXmlStreamEntityResolver_resolveEntity for (&'a  QString, &'a  QString) {
  fn resolveEntity(self, rsthis: &mut QXmlStreamEntityResolver) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QXmlStreamEntityResolver13resolveEntityERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN24QXmlStreamEntityResolver13resolveEntityERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamEntityResolver {
  pub fn resolveUndeclaredEntity<T: QXmlStreamEntityResolver_resolveUndeclaredEntity>(&mut self, value: T) -> QString {
    return value.resolveUndeclaredEntity(self);
    // return 1;
  }
}

pub trait QXmlStreamEntityResolver_resolveUndeclaredEntity {
  fn resolveUndeclaredEntity(self, rsthis: &mut QXmlStreamEntityResolver) -> QString;
}

// proto:  QString QXmlStreamEntityResolver::resolveUndeclaredEntity(const QString & name);
impl<'a> /*trait*/ QXmlStreamEntityResolver_resolveUndeclaredEntity for (&'a  QString) {
  fn resolveUndeclaredEntity(self, rsthis: &mut QXmlStreamEntityResolver) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QXmlStreamEntityResolver23resolveUndeclaredEntityERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN24QXmlStreamEntityResolver23resolveUndeclaredEntityERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamEntityResolver {
  pub fn FreeQXmlStreamEntityResolver<T: QXmlStreamEntityResolver_FreeQXmlStreamEntityResolver>(&mut self, value: T)  {
     value.FreeQXmlStreamEntityResolver(self);
    // return 1;
  }
}

pub trait QXmlStreamEntityResolver_FreeQXmlStreamEntityResolver {
  fn FreeQXmlStreamEntityResolver(self, rsthis: &mut QXmlStreamEntityResolver) ;
}

// proto:  void QXmlStreamEntityResolver::FreeQXmlStreamEntityResolver();
impl<'a> /*trait*/ QXmlStreamEntityResolver_FreeQXmlStreamEntityResolver for () {
  fn FreeQXmlStreamEntityResolver(self, rsthis: &mut QXmlStreamEntityResolver)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QXmlStreamEntityResolverD0Ev()};
     unsafe {_ZN24QXmlStreamEntityResolverD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

