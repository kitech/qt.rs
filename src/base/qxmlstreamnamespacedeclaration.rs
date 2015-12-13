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
  fn _ZNK30QXmlStreamNamespaceDeclaration12namespaceUriEv() -> i32;
  fn _ZN30QXmlStreamNamespaceDeclarationC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN30QXmlStreamNamespaceDeclarationC1Ev(qthis: *mut c_void) -> i32;
  fn _ZNK30QXmlStreamNamespaceDeclaration6prefixEv() -> i32;
  fn _ZN30QXmlStreamNamespaceDeclarationC1ERK7QStringS2_(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZN30QXmlStreamNamespaceDeclarationD0Ev() -> i32;
}

// body block begin
// class sizeof(QXmlStreamNamespaceDeclaration)=40
pub struct QXmlStreamNamespaceDeclaration {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QXmlStreamNamespaceDeclaration {
  pub fn namespaceUri<T: QXmlStreamNamespaceDeclaration_namespaceUri>(&mut self, value: T) -> i32 {
    value.namespaceUri(self);
    return 1;
  }
}

pub trait QXmlStreamNamespaceDeclaration_namespaceUri {
  fn namespaceUri(self, this: &mut QXmlStreamNamespaceDeclaration) -> i32;
}

// proto: QStringRef QXmlStreamNamespaceDeclaration::namespaceUri();
impl<'a> /*trait*/ QXmlStreamNamespaceDeclaration_namespaceUri for () {
  fn namespaceUri(self, this: &mut QXmlStreamNamespaceDeclaration) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK30QXmlStreamNamespaceDeclaration12namespaceUriEv()};
    unsafe {_ZNK30QXmlStreamNamespaceDeclaration12namespaceUriEv()};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
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
  pub fn prefix<T: QXmlStreamNamespaceDeclaration_prefix>(&mut self, value: T) -> i32 {
    value.prefix(self);
    return 1;
  }
}

pub trait QXmlStreamNamespaceDeclaration_prefix {
  fn prefix(self, this: &mut QXmlStreamNamespaceDeclaration) -> i32;
}

// proto: QStringRef QXmlStreamNamespaceDeclaration::prefix();
impl<'a> /*trait*/ QXmlStreamNamespaceDeclaration_prefix for () {
  fn prefix(self, this: &mut QXmlStreamNamespaceDeclaration) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK30QXmlStreamNamespaceDeclaration6prefixEv()};
    unsafe {_ZNK30QXmlStreamNamespaceDeclaration6prefixEv()};
    return 1;
  }
}

// proto: void QXmlStreamNamespaceDeclaration::NewQXmlStreamNamespaceDeclaration(const QString & prefix, const QString & namespaceUri);
impl<'a> /*trait*/ QXmlStreamNamespaceDeclaration_NewQXmlStreamNamespaceDeclaration for (&'a  QString, &'a  QString) {
  fn NewQXmlStreamNamespaceDeclaration(self) -> QXmlStreamNamespaceDeclaration {
    let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN30QXmlStreamNamespaceDeclarationC1ERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN30QXmlStreamNamespaceDeclarationC1ERK7QStringS2_(qthis, arg0, arg1)};
    let rsthis = QXmlStreamNamespaceDeclaration{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamNamespaceDeclaration {
  pub fn FreeQXmlStreamNamespaceDeclaration<T: QXmlStreamNamespaceDeclaration_FreeQXmlStreamNamespaceDeclaration>(&mut self, value: T) -> i32 {
    value.FreeQXmlStreamNamespaceDeclaration(self);
    return 1;
  }
}

pub trait QXmlStreamNamespaceDeclaration_FreeQXmlStreamNamespaceDeclaration {
  fn FreeQXmlStreamNamespaceDeclaration(self, this: &mut QXmlStreamNamespaceDeclaration) -> i32;
}

// proto: void QXmlStreamNamespaceDeclaration::FreeQXmlStreamNamespaceDeclaration();
impl<'a> /*trait*/ QXmlStreamNamespaceDeclaration_FreeQXmlStreamNamespaceDeclaration for () {
  fn FreeQXmlStreamNamespaceDeclaration(self, this: &mut QXmlStreamNamespaceDeclaration) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN30QXmlStreamNamespaceDeclarationD0Ev()};
    unsafe {_ZN30QXmlStreamNamespaceDeclarationD0Ev()};
    return 1;
  }
}

