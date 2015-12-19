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
  // proto:  void QXmlStreamEntityDeclaration::~QXmlStreamEntityDeclaration();
  fn _ZN27QXmlStreamEntityDeclarationD0Ev(qthis: *mut c_void);
  // proto:  QStringRef QXmlStreamEntityDeclaration::publicId();
  fn _ZNK27QXmlStreamEntityDeclaration8publicIdEv(qthis: *mut c_void);
  // proto:  QStringRef QXmlStreamEntityDeclaration::name();
  fn _ZNK27QXmlStreamEntityDeclaration4nameEv(qthis: *mut c_void);
  // proto:  void QXmlStreamEntityDeclaration::QXmlStreamEntityDeclaration();
  fn _ZN27QXmlStreamEntityDeclarationC1Ev(qthis: *mut c_void);
  // proto:  QStringRef QXmlStreamEntityDeclaration::value();
  fn _ZNK27QXmlStreamEntityDeclaration5valueEv(qthis: *mut c_void);
  // proto:  QStringRef QXmlStreamEntityDeclaration::notationName();
  fn _ZNK27QXmlStreamEntityDeclaration12notationNameEv(qthis: *mut c_void);
  // proto:  void QXmlStreamEntityDeclaration::QXmlStreamEntityDeclaration(const QXmlStreamEntityDeclaration & );
  fn _ZN27QXmlStreamEntityDeclarationC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QStringRef QXmlStreamEntityDeclaration::systemId();
  fn _ZNK27QXmlStreamEntityDeclaration8systemIdEv(qthis: *mut c_void);
}

// body block begin
// class sizeof(QXmlStreamEntityDeclaration)=88
pub struct QXmlStreamEntityDeclaration {
  pub qclsinst: *mut c_void,
}

  // proto:  void QXmlStreamEntityDeclaration::~QXmlStreamEntityDeclaration();
impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn FreeQXmlStreamEntityDeclaration<RetType, T: QXmlStreamEntityDeclaration_FreeQXmlStreamEntityDeclaration<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQXmlStreamEntityDeclaration(self);
    // return 1;
  }
}

pub trait QXmlStreamEntityDeclaration_FreeQXmlStreamEntityDeclaration<RetType> {
  fn FreeQXmlStreamEntityDeclaration(self , rsthis: &mut QXmlStreamEntityDeclaration) -> RetType;
}

  // proto:  void QXmlStreamEntityDeclaration::~QXmlStreamEntityDeclaration();
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_FreeQXmlStreamEntityDeclaration<()> for () {
  fn FreeQXmlStreamEntityDeclaration(self , rsthis: &mut QXmlStreamEntityDeclaration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN27QXmlStreamEntityDeclarationD0Ev()};
     unsafe {_ZN27QXmlStreamEntityDeclarationD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamEntityDeclaration::publicId();
impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn publicId<RetType, T: QXmlStreamEntityDeclaration_publicId<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.publicId(self);
    // return 1;
  }
}

pub trait QXmlStreamEntityDeclaration_publicId<RetType> {
  fn publicId(self , rsthis: &mut QXmlStreamEntityDeclaration) -> RetType;
}

  // proto:  QStringRef QXmlStreamEntityDeclaration::publicId();
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_publicId<()> for () {
  fn publicId(self , rsthis: &mut QXmlStreamEntityDeclaration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK27QXmlStreamEntityDeclaration8publicIdEv()};
     unsafe {_ZNK27QXmlStreamEntityDeclaration8publicIdEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamEntityDeclaration::name();
impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn name<RetType, T: QXmlStreamEntityDeclaration_name<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QXmlStreamEntityDeclaration_name<RetType> {
  fn name(self , rsthis: &mut QXmlStreamEntityDeclaration) -> RetType;
}

  // proto:  QStringRef QXmlStreamEntityDeclaration::name();
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_name<()> for () {
  fn name(self , rsthis: &mut QXmlStreamEntityDeclaration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK27QXmlStreamEntityDeclaration4nameEv()};
     unsafe {_ZNK27QXmlStreamEntityDeclaration4nameEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QXmlStreamEntityDeclaration::QXmlStreamEntityDeclaration();
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

  // proto:  void QXmlStreamEntityDeclaration::QXmlStreamEntityDeclaration();
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

  // proto:  QStringRef QXmlStreamEntityDeclaration::value();
impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn value<RetType, T: QXmlStreamEntityDeclaration_value<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QXmlStreamEntityDeclaration_value<RetType> {
  fn value(self , rsthis: &mut QXmlStreamEntityDeclaration) -> RetType;
}

  // proto:  QStringRef QXmlStreamEntityDeclaration::value();
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_value<()> for () {
  fn value(self , rsthis: &mut QXmlStreamEntityDeclaration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK27QXmlStreamEntityDeclaration5valueEv()};
     unsafe {_ZNK27QXmlStreamEntityDeclaration5valueEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamEntityDeclaration::notationName();
impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn notationName<RetType, T: QXmlStreamEntityDeclaration_notationName<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.notationName(self);
    // return 1;
  }
}

pub trait QXmlStreamEntityDeclaration_notationName<RetType> {
  fn notationName(self , rsthis: &mut QXmlStreamEntityDeclaration) -> RetType;
}

  // proto:  QStringRef QXmlStreamEntityDeclaration::notationName();
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_notationName<()> for () {
  fn notationName(self , rsthis: &mut QXmlStreamEntityDeclaration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK27QXmlStreamEntityDeclaration12notationNameEv()};
     unsafe {_ZNK27QXmlStreamEntityDeclaration12notationNameEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QXmlStreamEntityDeclaration::QXmlStreamEntityDeclaration(const QXmlStreamEntityDeclaration & );
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_NewQXmlStreamEntityDeclaration for (QXmlStreamEntityDeclaration) {
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

  // proto:  QStringRef QXmlStreamEntityDeclaration::systemId();
impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn systemId<RetType, T: QXmlStreamEntityDeclaration_systemId<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.systemId(self);
    // return 1;
  }
}

pub trait QXmlStreamEntityDeclaration_systemId<RetType> {
  fn systemId(self , rsthis: &mut QXmlStreamEntityDeclaration) -> RetType;
}

  // proto:  QStringRef QXmlStreamEntityDeclaration::systemId();
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_systemId<()> for () {
  fn systemId(self , rsthis: &mut QXmlStreamEntityDeclaration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK27QXmlStreamEntityDeclaration8systemIdEv()};
     unsafe {_ZNK27QXmlStreamEntityDeclaration8systemIdEv(rsthis.qclsinst)};
    // return 1;
  }
}

