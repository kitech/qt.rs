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
  // proto:  QStringRef QXmlStreamNamespaceDeclaration::namespaceUri();
  fn _ZNK30QXmlStreamNamespaceDeclaration12namespaceUriEv(qthis: *mut c_void) ;
  // proto:  void QXmlStreamNamespaceDeclaration::NewQXmlStreamNamespaceDeclaration(const QXmlStreamNamespaceDeclaration & );
  fn _ZN30QXmlStreamNamespaceDeclarationC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QXmlStreamNamespaceDeclaration::NewQXmlStreamNamespaceDeclaration();
  fn _ZN30QXmlStreamNamespaceDeclarationC1Ev(qthis: *mut c_void) ;
  // proto:  QStringRef QXmlStreamNamespaceDeclaration::prefix();
  fn _ZNK30QXmlStreamNamespaceDeclaration6prefixEv(qthis: *mut c_void) ;
  // proto:  void QXmlStreamNamespaceDeclaration::NewQXmlStreamNamespaceDeclaration(const QString & prefix, const QString & namespaceUri);
  fn _ZN30QXmlStreamNamespaceDeclarationC1ERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QXmlStreamNamespaceDeclaration::FreeQXmlStreamNamespaceDeclaration();
  fn _ZN30QXmlStreamNamespaceDeclarationD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QXmlStreamNamespaceDeclaration)=40
pub struct QXmlStreamNamespaceDeclaration {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QXmlStreamNamespaceDeclaration {
  pub fn namespaceUri<RetType, T: QXmlStreamNamespaceDeclaration_namespaceUri<RetType>>(&mut self, value: T) -> RetType {
    return value.namespaceUri(self);
    // return 1;
  }
}

pub trait QXmlStreamNamespaceDeclaration_namespaceUri<RetType> {
  fn namespaceUri(self, rsthis: &mut QXmlStreamNamespaceDeclaration) -> RetType;
}

// proto:  QStringRef QXmlStreamNamespaceDeclaration::namespaceUri();
impl<'a> /*trait*/ QXmlStreamNamespaceDeclaration_namespaceUri<()> for () {
  fn namespaceUri(self, rsthis: &mut QXmlStreamNamespaceDeclaration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK30QXmlStreamNamespaceDeclaration12namespaceUriEv()};
     unsafe {_ZNK30QXmlStreamNamespaceDeclaration12namespaceUriEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QXmlStreamNamespaceDeclaration {
  pub fn NewQXmlStreamNamespaceDeclaration<T: QXmlStreamNamespaceDeclaration_NewQXmlStreamNamespaceDeclaration>(value: T) -> QXmlStreamNamespaceDeclaration {
    let rsthis = value.NewQXmlStreamNamespaceDeclaration();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamNamespaceDeclaration_NewQXmlStreamNamespaceDeclaration {
  fn NewQXmlStreamNamespaceDeclaration(self) -> QXmlStreamNamespaceDeclaration;
}

// proto: void QXmlStreamNamespaceDeclaration::NewQXmlStreamNamespaceDeclaration(const QXmlStreamNamespaceDeclaration & );
impl<'a> /*trait*/ QXmlStreamNamespaceDeclaration_NewQXmlStreamNamespaceDeclaration for (&'a  QXmlStreamNamespaceDeclaration) {
  fn NewQXmlStreamNamespaceDeclaration(self) -> QXmlStreamNamespaceDeclaration {
    let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN30QXmlStreamNamespaceDeclarationC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN30QXmlStreamNamespaceDeclarationC1ERKS_(qthis, arg0)};
    let rsthis = QXmlStreamNamespaceDeclaration{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QXmlStreamNamespaceDeclaration::NewQXmlStreamNamespaceDeclaration();
impl<'a> /*trait*/ QXmlStreamNamespaceDeclaration_NewQXmlStreamNamespaceDeclaration for () {
  fn NewQXmlStreamNamespaceDeclaration(self) -> QXmlStreamNamespaceDeclaration {
    let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN30QXmlStreamNamespaceDeclarationC1Ev()};
    unsafe {_ZN30QXmlStreamNamespaceDeclarationC1Ev(qthis)};
    let rsthis = QXmlStreamNamespaceDeclaration{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamNamespaceDeclaration {
  pub fn prefix<RetType, T: QXmlStreamNamespaceDeclaration_prefix<RetType>>(&mut self, value: T) -> RetType {
    return value.prefix(self);
    // return 1;
  }
}

pub trait QXmlStreamNamespaceDeclaration_prefix<RetType> {
  fn prefix(self, rsthis: &mut QXmlStreamNamespaceDeclaration) -> RetType;
}

// proto:  QStringRef QXmlStreamNamespaceDeclaration::prefix();
impl<'a> /*trait*/ QXmlStreamNamespaceDeclaration_prefix<()> for () {
  fn prefix(self, rsthis: &mut QXmlStreamNamespaceDeclaration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK30QXmlStreamNamespaceDeclaration6prefixEv()};
     unsafe {_ZNK30QXmlStreamNamespaceDeclaration6prefixEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QXmlStreamNamespaceDeclaration::NewQXmlStreamNamespaceDeclaration(const QString & prefix, const QString & namespaceUri);
impl<'a> /*trait*/ QXmlStreamNamespaceDeclaration_NewQXmlStreamNamespaceDeclaration for (&'a  QString, &'a  QString) {
  fn NewQXmlStreamNamespaceDeclaration(self) -> QXmlStreamNamespaceDeclaration {
    let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN30QXmlStreamNamespaceDeclarationC1ERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN30QXmlStreamNamespaceDeclarationC1ERK7QStringS2_(qthis, arg0, arg1)};
    let rsthis = QXmlStreamNamespaceDeclaration{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamNamespaceDeclaration {
  pub fn FreeQXmlStreamNamespaceDeclaration<RetType, T: QXmlStreamNamespaceDeclaration_FreeQXmlStreamNamespaceDeclaration<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQXmlStreamNamespaceDeclaration(self);
    // return 1;
  }
}

pub trait QXmlStreamNamespaceDeclaration_FreeQXmlStreamNamespaceDeclaration<RetType> {
  fn FreeQXmlStreamNamespaceDeclaration(self, rsthis: &mut QXmlStreamNamespaceDeclaration) -> RetType;
}

// proto:  void QXmlStreamNamespaceDeclaration::FreeQXmlStreamNamespaceDeclaration();
impl<'a> /*trait*/ QXmlStreamNamespaceDeclaration_FreeQXmlStreamNamespaceDeclaration<()> for () {
  fn FreeQXmlStreamNamespaceDeclaration(self, rsthis: &mut QXmlStreamNamespaceDeclaration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN30QXmlStreamNamespaceDeclarationD0Ev()};
     unsafe {_ZN30QXmlStreamNamespaceDeclarationD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

