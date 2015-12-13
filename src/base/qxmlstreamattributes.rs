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
  fn _ZNK20QXmlStreamAttributes5valueERK7QString(arg0: *const c_void) -> i32;
  fn _ZN20QXmlStreamAttributesC1Ev(qthis: *mut c_void) -> i32;
  fn _ZNK20QXmlStreamAttributes12hasAttributeERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK20QXmlStreamAttributes12hasAttributeERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZN20QXmlStreamAttributes6appendERK7QStringS2_S2_(arg0: *const c_void, arg1: *const c_void, arg2: *const c_void) -> i32;
  fn _ZN20QXmlStreamAttributes6appendERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZNK20QXmlStreamAttributes5valueERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
}

// body block begin
// class sizeof(QXmlStreamAttributes)=1
pub struct QXmlStreamAttributes {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QXmlStreamAttributes {
  pub fn value<T: QXmlStreamAttributes_value>(&mut self, value: T) -> i32 {
    value.value(self);
    return 1;
  }
}

pub trait QXmlStreamAttributes_value {
  fn value(self, this: &mut QXmlStreamAttributes) -> i32;
}

// proto: QStringRef QXmlStreamAttributes::value(const QString & qualifiedName);
impl<'a> /*trait*/ QXmlStreamAttributes_value for (&'a  QString) {
  fn value(self, this: &mut QXmlStreamAttributes) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QXmlStreamAttributes5valueERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK20QXmlStreamAttributes5valueERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QXmlStreamAttributes {
  pub fn NewQXmlStreamAttributes<T: QXmlStreamAttributes_NewQXmlStreamAttributes>(value: T) -> QXmlStreamAttributes {
    let rsthis = value.NewQXmlStreamAttributes();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamAttributes_NewQXmlStreamAttributes {
  fn NewQXmlStreamAttributes(self) -> QXmlStreamAttributes;
}

// proto: void QXmlStreamAttributes::NewQXmlStreamAttributes();
impl<'a> /*trait*/ QXmlStreamAttributes_NewQXmlStreamAttributes for () {
  fn NewQXmlStreamAttributes(self) -> QXmlStreamAttributes {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QXmlStreamAttributesC1Ev()};
    unsafe {_ZN20QXmlStreamAttributesC1Ev(qthis)};
    let rsthis = QXmlStreamAttributes{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamAttributes {
  pub fn hasAttribute<T: QXmlStreamAttributes_hasAttribute>(&mut self, value: T) -> i32 {
    value.hasAttribute(self);
    return 1;
  }
}

pub trait QXmlStreamAttributes_hasAttribute {
  fn hasAttribute(self, this: &mut QXmlStreamAttributes) -> i32;
}

// proto: bool QXmlStreamAttributes::hasAttribute(const QString & qualifiedName);
impl<'a> /*trait*/ QXmlStreamAttributes_hasAttribute for (&'a  QString) {
  fn hasAttribute(self, this: &mut QXmlStreamAttributes) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QXmlStreamAttributes12hasAttributeERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK20QXmlStreamAttributes12hasAttributeERK7QString(arg0)};
    return 1;
  }
}

// proto: bool QXmlStreamAttributes::hasAttribute(const QString & namespaceUri, const QString & name);
impl<'a> /*trait*/ QXmlStreamAttributes_hasAttribute for (&'a  QString, &'a  QString) {
  fn hasAttribute(self, this: &mut QXmlStreamAttributes) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QXmlStreamAttributes12hasAttributeERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK20QXmlStreamAttributes12hasAttributeERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QXmlStreamAttributes {
  pub fn append<T: QXmlStreamAttributes_append>(&mut self, value: T) -> i32 {
    value.append(self);
    return 1;
  }
}

pub trait QXmlStreamAttributes_append {
  fn append(self, this: &mut QXmlStreamAttributes) -> i32;
}

// proto: void QXmlStreamAttributes::append(const QString & namespaceUri, const QString & name, const QString & value);
impl<'a> /*trait*/ QXmlStreamAttributes_append for (&'a  QString, &'a  QString, &'a  QString) {
  fn append(self, this: &mut QXmlStreamAttributes) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QXmlStreamAttributes6appendERK7QStringS2_S2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN20QXmlStreamAttributes6appendERK7QStringS2_S2_(arg0, arg1, arg2)};
    return 1;
  }
}

// proto: void QXmlStreamAttributes::append(const QString & qualifiedName, const QString & value);
impl<'a> /*trait*/ QXmlStreamAttributes_append for (&'a  QString, &'a  QString) {
  fn append(self, this: &mut QXmlStreamAttributes) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QXmlStreamAttributes6appendERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN20QXmlStreamAttributes6appendERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

// proto: QStringRef QXmlStreamAttributes::value(const QString & namespaceUri, const QString & name);
impl<'a> /*trait*/ QXmlStreamAttributes_value for (&'a  QString, &'a  QString) {
  fn value(self, this: &mut QXmlStreamAttributes) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QXmlStreamAttributes5valueERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK20QXmlStreamAttributes5valueERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

