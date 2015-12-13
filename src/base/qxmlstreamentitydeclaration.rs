// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN27QXmlStreamEntityDeclarationD0Ev() -> i32;
  fn _ZNK27QXmlStreamEntityDeclaration8publicIdEv() -> i32;
  fn _ZNK27QXmlStreamEntityDeclaration4nameEv() -> i32;
  fn _ZN27QXmlStreamEntityDeclarationC1Ev(qthis: *mut c_void) -> i32;
  fn _ZNK27QXmlStreamEntityDeclaration5valueEv() -> i32;
  fn _ZNK27QXmlStreamEntityDeclaration12notationNameEv() -> i32;
  fn _ZN27QXmlStreamEntityDeclarationC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK27QXmlStreamEntityDeclaration8systemIdEv() -> i32;
}

// body block begin
// class sizeof(QXmlStreamEntityDeclaration)=88
pub struct QXmlStreamEntityDeclaration {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn FreeQXmlStreamEntityDeclaration<T: QXmlStreamEntityDeclaration_FreeQXmlStreamEntityDeclaration>(&mut self, value: T) -> i32 {
    value.FreeQXmlStreamEntityDeclaration(self);
    return 1;
  }
}

pub trait QXmlStreamEntityDeclaration_FreeQXmlStreamEntityDeclaration {
  fn FreeQXmlStreamEntityDeclaration(self, this: &mut QXmlStreamEntityDeclaration) -> i32;
}

// proto: void QXmlStreamEntityDeclaration::FreeQXmlStreamEntityDeclaration();
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_FreeQXmlStreamEntityDeclaration for () {
  fn FreeQXmlStreamEntityDeclaration(self, this: &mut QXmlStreamEntityDeclaration) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN27QXmlStreamEntityDeclarationD0Ev()};
    unsafe {_ZN27QXmlStreamEntityDeclarationD0Ev()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn publicId<T: QXmlStreamEntityDeclaration_publicId>(&mut self, value: T) -> i32 {
    value.publicId(self);
    return 1;
  }
}

pub trait QXmlStreamEntityDeclaration_publicId {
  fn publicId(self, this: &mut QXmlStreamEntityDeclaration) -> i32;
}

// proto: QStringRef QXmlStreamEntityDeclaration::publicId();
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_publicId for () {
  fn publicId(self, this: &mut QXmlStreamEntityDeclaration) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK27QXmlStreamEntityDeclaration8publicIdEv()};
    unsafe {_ZNK27QXmlStreamEntityDeclaration8publicIdEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn name<T: QXmlStreamEntityDeclaration_name>(&mut self, value: T) -> i32 {
    value.name(self);
    return 1;
  }
}

pub trait QXmlStreamEntityDeclaration_name {
  fn name(self, this: &mut QXmlStreamEntityDeclaration) -> i32;
}

// proto: QStringRef QXmlStreamEntityDeclaration::name();
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_name for () {
  fn name(self, this: &mut QXmlStreamEntityDeclaration) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK27QXmlStreamEntityDeclaration4nameEv()};
    unsafe {_ZNK27QXmlStreamEntityDeclaration4nameEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn NewQXmlStreamEntityDeclaration<T: QXmlStreamEntityDeclaration_NewQXmlStreamEntityDeclaration>(value: T) -> QXmlStreamEntityDeclaration {
    let rsthis = value.NewQXmlStreamEntityDeclaration();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamEntityDeclaration_NewQXmlStreamEntityDeclaration {
  fn NewQXmlStreamEntityDeclaration(self) -> QXmlStreamEntityDeclaration;
}

// proto: void QXmlStreamEntityDeclaration::NewQXmlStreamEntityDeclaration();
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_NewQXmlStreamEntityDeclaration for () {
  fn NewQXmlStreamEntityDeclaration(self) -> QXmlStreamEntityDeclaration {
    let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN27QXmlStreamEntityDeclarationC1Ev()};
    unsafe {_ZN27QXmlStreamEntityDeclarationC1Ev(qthis)};
    let rsthis = QXmlStreamEntityDeclaration{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn value<T: QXmlStreamEntityDeclaration_value>(&mut self, value: T) -> i32 {
    value.value(self);
    return 1;
  }
}

pub trait QXmlStreamEntityDeclaration_value {
  fn value(self, this: &mut QXmlStreamEntityDeclaration) -> i32;
}

// proto: QStringRef QXmlStreamEntityDeclaration::value();
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_value for () {
  fn value(self, this: &mut QXmlStreamEntityDeclaration) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK27QXmlStreamEntityDeclaration5valueEv()};
    unsafe {_ZNK27QXmlStreamEntityDeclaration5valueEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn notationName<T: QXmlStreamEntityDeclaration_notationName>(&mut self, value: T) -> i32 {
    value.notationName(self);
    return 1;
  }
}

pub trait QXmlStreamEntityDeclaration_notationName {
  fn notationName(self, this: &mut QXmlStreamEntityDeclaration) -> i32;
}

// proto: QStringRef QXmlStreamEntityDeclaration::notationName();
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_notationName for () {
  fn notationName(self, this: &mut QXmlStreamEntityDeclaration) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK27QXmlStreamEntityDeclaration12notationNameEv()};
    unsafe {_ZNK27QXmlStreamEntityDeclaration12notationNameEv()};
    return 1;
  }
}

// proto: void QXmlStreamEntityDeclaration::NewQXmlStreamEntityDeclaration(const QXmlStreamEntityDeclaration & );
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_NewQXmlStreamEntityDeclaration for (&'a  QXmlStreamEntityDeclaration) {
  fn NewQXmlStreamEntityDeclaration(self) -> QXmlStreamEntityDeclaration {
    let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN27QXmlStreamEntityDeclarationC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN27QXmlStreamEntityDeclarationC1ERKS_(qthis, arg0)};
    let rsthis = QXmlStreamEntityDeclaration{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn systemId<T: QXmlStreamEntityDeclaration_systemId>(&mut self, value: T) -> i32 {
    value.systemId(self);
    return 1;
  }
}

pub trait QXmlStreamEntityDeclaration_systemId {
  fn systemId(self, this: &mut QXmlStreamEntityDeclaration) -> i32;
}

// proto: QStringRef QXmlStreamEntityDeclaration::systemId();
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_systemId for () {
  fn systemId(self, this: &mut QXmlStreamEntityDeclaration) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK27QXmlStreamEntityDeclaration8systemIdEv()};
    unsafe {_ZNK27QXmlStreamEntityDeclaration8systemIdEv()};
    return 1;
  }
}

