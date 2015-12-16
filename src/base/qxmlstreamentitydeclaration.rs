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
  // proto:  void QXmlStreamEntityDeclaration::FreeQXmlStreamEntityDeclaration();
  fn _ZN27QXmlStreamEntityDeclarationD0Ev(qthis: *mut c_void) ;
  // proto:  QStringRef QXmlStreamEntityDeclaration::publicId();
  fn _ZNK27QXmlStreamEntityDeclaration8publicIdEv(qthis: *mut c_void) ;
  // proto:  QStringRef QXmlStreamEntityDeclaration::name();
  fn _ZNK27QXmlStreamEntityDeclaration4nameEv(qthis: *mut c_void) ;
  // proto:  void QXmlStreamEntityDeclaration::NewQXmlStreamEntityDeclaration();
  fn _ZN27QXmlStreamEntityDeclarationC1Ev(qthis: *mut c_void) ;
  // proto:  QStringRef QXmlStreamEntityDeclaration::value();
  fn _ZNK27QXmlStreamEntityDeclaration5valueEv(qthis: *mut c_void) ;
  // proto:  QStringRef QXmlStreamEntityDeclaration::notationName();
  fn _ZNK27QXmlStreamEntityDeclaration12notationNameEv(qthis: *mut c_void) ;
  // proto:  void QXmlStreamEntityDeclaration::NewQXmlStreamEntityDeclaration(const QXmlStreamEntityDeclaration & );
  fn _ZN27QXmlStreamEntityDeclarationC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QStringRef QXmlStreamEntityDeclaration::systemId();
  fn _ZNK27QXmlStreamEntityDeclaration8systemIdEv(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QXmlStreamEntityDeclaration)=88
pub struct QXmlStreamEntityDeclaration {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn FreeQXmlStreamEntityDeclaration<T: QXmlStreamEntityDeclaration_FreeQXmlStreamEntityDeclaration>(&mut self, value: T)  {
     value.FreeQXmlStreamEntityDeclaration(self);
    // return 1;
  }
}

pub trait QXmlStreamEntityDeclaration_FreeQXmlStreamEntityDeclaration {
  fn FreeQXmlStreamEntityDeclaration(self, rsthis: &mut QXmlStreamEntityDeclaration) ;
}

// proto:  void QXmlStreamEntityDeclaration::FreeQXmlStreamEntityDeclaration();
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_FreeQXmlStreamEntityDeclaration for () {
  fn FreeQXmlStreamEntityDeclaration(self, rsthis: &mut QXmlStreamEntityDeclaration)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN27QXmlStreamEntityDeclarationD0Ev()};
     unsafe {_ZN27QXmlStreamEntityDeclarationD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn publicId<T: QXmlStreamEntityDeclaration_publicId>(&mut self, value: T)  {
     value.publicId(self);
    // return 1;
  }
}

pub trait QXmlStreamEntityDeclaration_publicId {
  fn publicId(self, rsthis: &mut QXmlStreamEntityDeclaration) ;
}

// proto:  QStringRef QXmlStreamEntityDeclaration::publicId();
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_publicId for () {
  fn publicId(self, rsthis: &mut QXmlStreamEntityDeclaration)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK27QXmlStreamEntityDeclaration8publicIdEv()};
     unsafe {_ZNK27QXmlStreamEntityDeclaration8publicIdEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn name<T: QXmlStreamEntityDeclaration_name>(&mut self, value: T)  {
     value.name(self);
    // return 1;
  }
}

pub trait QXmlStreamEntityDeclaration_name {
  fn name(self, rsthis: &mut QXmlStreamEntityDeclaration) ;
}

// proto:  QStringRef QXmlStreamEntityDeclaration::name();
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_name for () {
  fn name(self, rsthis: &mut QXmlStreamEntityDeclaration)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK27QXmlStreamEntityDeclaration4nameEv()};
     unsafe {_ZNK27QXmlStreamEntityDeclaration4nameEv(rsthis.qclsinst)};
    // return 1;
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
  pub fn value<T: QXmlStreamEntityDeclaration_value>(&mut self, value: T)  {
     value.value(self);
    // return 1;
  }
}

pub trait QXmlStreamEntityDeclaration_value {
  fn value(self, rsthis: &mut QXmlStreamEntityDeclaration) ;
}

// proto:  QStringRef QXmlStreamEntityDeclaration::value();
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_value for () {
  fn value(self, rsthis: &mut QXmlStreamEntityDeclaration)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK27QXmlStreamEntityDeclaration5valueEv()};
     unsafe {_ZNK27QXmlStreamEntityDeclaration5valueEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn notationName<T: QXmlStreamEntityDeclaration_notationName>(&mut self, value: T)  {
     value.notationName(self);
    // return 1;
  }
}

pub trait QXmlStreamEntityDeclaration_notationName {
  fn notationName(self, rsthis: &mut QXmlStreamEntityDeclaration) ;
}

// proto:  QStringRef QXmlStreamEntityDeclaration::notationName();
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_notationName for () {
  fn notationName(self, rsthis: &mut QXmlStreamEntityDeclaration)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK27QXmlStreamEntityDeclaration12notationNameEv()};
     unsafe {_ZNK27QXmlStreamEntityDeclaration12notationNameEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QXmlStreamEntityDeclaration::NewQXmlStreamEntityDeclaration(const QXmlStreamEntityDeclaration & );
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_NewQXmlStreamEntityDeclaration for (&'a  QXmlStreamEntityDeclaration) {
  fn NewQXmlStreamEntityDeclaration(self) -> QXmlStreamEntityDeclaration {
    let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN27QXmlStreamEntityDeclarationC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN27QXmlStreamEntityDeclarationC1ERKS_(qthis, arg0)};
    let rsthis = QXmlStreamEntityDeclaration{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn systemId<T: QXmlStreamEntityDeclaration_systemId>(&mut self, value: T)  {
     value.systemId(self);
    // return 1;
  }
}

pub trait QXmlStreamEntityDeclaration_systemId {
  fn systemId(self, rsthis: &mut QXmlStreamEntityDeclaration) ;
}

// proto:  QStringRef QXmlStreamEntityDeclaration::systemId();
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_systemId for () {
  fn systemId(self, rsthis: &mut QXmlStreamEntityDeclaration)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK27QXmlStreamEntityDeclaration8systemIdEv()};
     unsafe {_ZNK27QXmlStreamEntityDeclaration8systemIdEv(rsthis.qclsinst)};
    // return 1;
  }
}

