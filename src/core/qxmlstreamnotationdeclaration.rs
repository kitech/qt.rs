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
  // proto:  void QXmlStreamNotationDeclaration::QXmlStreamNotationDeclaration(const QXmlStreamNotationDeclaration & );
  fn _ZN29QXmlStreamNotationDeclarationC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QStringRef QXmlStreamNotationDeclaration::publicId();
  fn _ZNK29QXmlStreamNotationDeclaration8publicIdEv(qthis: *mut c_void);
  // proto:  QStringRef QXmlStreamNotationDeclaration::name();
  fn _ZNK29QXmlStreamNotationDeclaration4nameEv(qthis: *mut c_void);
  // proto:  void QXmlStreamNotationDeclaration::~QXmlStreamNotationDeclaration();
  fn _ZN29QXmlStreamNotationDeclarationD0Ev(qthis: *mut c_void);
  // proto:  QStringRef QXmlStreamNotationDeclaration::systemId();
  fn _ZNK29QXmlStreamNotationDeclaration8systemIdEv(qthis: *mut c_void);
  // proto:  void QXmlStreamNotationDeclaration::QXmlStreamNotationDeclaration();
  fn _ZN29QXmlStreamNotationDeclarationC1Ev(qthis: *mut c_void);
}

// body block begin
// class sizeof(QXmlStreamNotationDeclaration)=56
pub struct QXmlStreamNotationDeclaration {
  pub qclsinst: *mut c_void,
}

  // proto:  void QXmlStreamNotationDeclaration::QXmlStreamNotationDeclaration(const QXmlStreamNotationDeclaration & );
impl /*struct*/ QXmlStreamNotationDeclaration {
  pub fn NewQXmlStreamNotationDeclaration<T: QXmlStreamNotationDeclaration_NewQXmlStreamNotationDeclaration>(value: T) -> QXmlStreamNotationDeclaration {
    let rsthis = value.NewQXmlStreamNotationDeclaration();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamNotationDeclaration_NewQXmlStreamNotationDeclaration {
  fn NewQXmlStreamNotationDeclaration(self) -> QXmlStreamNotationDeclaration;
}

  // proto:  void QXmlStreamNotationDeclaration::QXmlStreamNotationDeclaration(const QXmlStreamNotationDeclaration & );
impl<'a> /*trait*/ QXmlStreamNotationDeclaration_NewQXmlStreamNotationDeclaration for (QXmlStreamNotationDeclaration) {
  fn NewQXmlStreamNotationDeclaration(self) -> QXmlStreamNotationDeclaration {
    let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZN29QXmlStreamNotationDeclarationC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN29QXmlStreamNotationDeclarationC1ERKS_(qthis, arg0)};
    let rsthis = QXmlStreamNotationDeclaration{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamNotationDeclaration::publicId();
impl /*struct*/ QXmlStreamNotationDeclaration {
  pub fn publicId<RetType, T: QXmlStreamNotationDeclaration_publicId<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.publicId(self);
    // return 1;
  }
}

pub trait QXmlStreamNotationDeclaration_publicId<RetType> {
  fn publicId(self , rsthis: &mut QXmlStreamNotationDeclaration) -> RetType;
}

  // proto:  QStringRef QXmlStreamNotationDeclaration::publicId();
impl<'a> /*trait*/ QXmlStreamNotationDeclaration_publicId<()> for () {
  fn publicId(self , rsthis: &mut QXmlStreamNotationDeclaration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZNK29QXmlStreamNotationDeclaration8publicIdEv()};
     unsafe {_ZNK29QXmlStreamNotationDeclaration8publicIdEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamNotationDeclaration::name();
impl /*struct*/ QXmlStreamNotationDeclaration {
  pub fn name<RetType, T: QXmlStreamNotationDeclaration_name<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QXmlStreamNotationDeclaration_name<RetType> {
  fn name(self , rsthis: &mut QXmlStreamNotationDeclaration) -> RetType;
}

  // proto:  QStringRef QXmlStreamNotationDeclaration::name();
impl<'a> /*trait*/ QXmlStreamNotationDeclaration_name<()> for () {
  fn name(self , rsthis: &mut QXmlStreamNotationDeclaration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZNK29QXmlStreamNotationDeclaration4nameEv()};
     unsafe {_ZNK29QXmlStreamNotationDeclaration4nameEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QXmlStreamNotationDeclaration::~QXmlStreamNotationDeclaration();
impl /*struct*/ QXmlStreamNotationDeclaration {
  pub fn FreeQXmlStreamNotationDeclaration<RetType, T: QXmlStreamNotationDeclaration_FreeQXmlStreamNotationDeclaration<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQXmlStreamNotationDeclaration(self);
    // return 1;
  }
}

pub trait QXmlStreamNotationDeclaration_FreeQXmlStreamNotationDeclaration<RetType> {
  fn FreeQXmlStreamNotationDeclaration(self , rsthis: &mut QXmlStreamNotationDeclaration) -> RetType;
}

  // proto:  void QXmlStreamNotationDeclaration::~QXmlStreamNotationDeclaration();
impl<'a> /*trait*/ QXmlStreamNotationDeclaration_FreeQXmlStreamNotationDeclaration<()> for () {
  fn FreeQXmlStreamNotationDeclaration(self , rsthis: &mut QXmlStreamNotationDeclaration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZN29QXmlStreamNotationDeclarationD0Ev()};
     unsafe {_ZN29QXmlStreamNotationDeclarationD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamNotationDeclaration::systemId();
impl /*struct*/ QXmlStreamNotationDeclaration {
  pub fn systemId<RetType, T: QXmlStreamNotationDeclaration_systemId<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.systemId(self);
    // return 1;
  }
}

pub trait QXmlStreamNotationDeclaration_systemId<RetType> {
  fn systemId(self , rsthis: &mut QXmlStreamNotationDeclaration) -> RetType;
}

  // proto:  QStringRef QXmlStreamNotationDeclaration::systemId();
impl<'a> /*trait*/ QXmlStreamNotationDeclaration_systemId<()> for () {
  fn systemId(self , rsthis: &mut QXmlStreamNotationDeclaration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZNK29QXmlStreamNotationDeclaration8systemIdEv()};
     unsafe {_ZNK29QXmlStreamNotationDeclaration8systemIdEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QXmlStreamNotationDeclaration::QXmlStreamNotationDeclaration();
impl<'a> /*trait*/ QXmlStreamNotationDeclaration_NewQXmlStreamNotationDeclaration for () {
  fn NewQXmlStreamNotationDeclaration(self) -> QXmlStreamNotationDeclaration {
    let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZN29QXmlStreamNotationDeclarationC1Ev()};
    unsafe {_ZN29QXmlStreamNotationDeclarationC1Ev(qthis)};
    let rsthis = QXmlStreamNotationDeclaration{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

