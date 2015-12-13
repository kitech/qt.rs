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
  fn _ZN19QXmlStreamAttributeC1ERK7QStringS2_(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZNK19QXmlStreamAttribute13qualifiedNameEv() -> i32;
  fn _ZN19QXmlStreamAttributeD0Ev() -> i32;
  fn _ZNK19QXmlStreamAttribute5valueEv() -> i32;
  fn _ZNK19QXmlStreamAttribute12namespaceUriEv() -> i32;
  fn _ZN19QXmlStreamAttributeC1Ev(qthis: *mut c_void) -> i32;
  fn _ZN19QXmlStreamAttributeC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN19QXmlStreamAttributeC1ERK7QStringS2_S2_(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_void, arg2: *const c_void) -> i32;
  fn _ZNK19QXmlStreamAttribute9isDefaultEv() -> i32;
  fn _ZNK19QXmlStreamAttribute6prefixEv() -> i32;
  fn _ZNK19QXmlStreamAttribute4nameEv() -> i32;
}

// body block begin
// class sizeof(QXmlStreamAttribute)=80
pub struct QXmlStreamAttribute {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QXmlStreamAttribute {
  pub fn NewQXmlStreamAttribute<T: QXmlStreamAttribute_NewQXmlStreamAttribute>(value: T) -> QXmlStreamAttribute {
    let rsthis = value.NewQXmlStreamAttribute();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamAttribute_NewQXmlStreamAttribute {
  fn NewQXmlStreamAttribute(self) -> QXmlStreamAttribute;
}

// proto: void QXmlStreamAttribute::NewQXmlStreamAttribute(const QString & qualifiedName, const QString & value);
impl<'a> /*trait*/ QXmlStreamAttribute_NewQXmlStreamAttribute for (&'a  QString, &'a  QString) {
  fn NewQXmlStreamAttribute(self) -> QXmlStreamAttribute {
    let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN19QXmlStreamAttributeC1ERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN19QXmlStreamAttributeC1ERK7QStringS2_(qthis, arg0, arg1)};
    let rsthis = QXmlStreamAttribute{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamAttribute {
  pub fn qualifiedName<T: QXmlStreamAttribute_qualifiedName>(&mut self, value: T) -> i32 {
    value.qualifiedName(self);
    return 1;
  }
}

pub trait QXmlStreamAttribute_qualifiedName {
  fn qualifiedName(self, this: &mut QXmlStreamAttribute) -> i32;
}

// proto: QStringRef QXmlStreamAttribute::qualifiedName();
impl<'a> /*trait*/ QXmlStreamAttribute_qualifiedName for () {
  fn qualifiedName(self, this: &mut QXmlStreamAttribute) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZNK19QXmlStreamAttribute13qualifiedNameEv()};
    unsafe {_ZNK19QXmlStreamAttribute13qualifiedNameEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamAttribute {
  pub fn FreeQXmlStreamAttribute<T: QXmlStreamAttribute_FreeQXmlStreamAttribute>(&mut self, value: T) -> i32 {
    value.FreeQXmlStreamAttribute(self);
    return 1;
  }
}

pub trait QXmlStreamAttribute_FreeQXmlStreamAttribute {
  fn FreeQXmlStreamAttribute(self, this: &mut QXmlStreamAttribute) -> i32;
}

// proto: void QXmlStreamAttribute::FreeQXmlStreamAttribute();
impl<'a> /*trait*/ QXmlStreamAttribute_FreeQXmlStreamAttribute for () {
  fn FreeQXmlStreamAttribute(self, this: &mut QXmlStreamAttribute) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN19QXmlStreamAttributeD0Ev()};
    unsafe {_ZN19QXmlStreamAttributeD0Ev()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamAttribute {
  pub fn value<T: QXmlStreamAttribute_value>(&mut self, value: T) -> i32 {
    value.value(self);
    return 1;
  }
}

pub trait QXmlStreamAttribute_value {
  fn value(self, this: &mut QXmlStreamAttribute) -> i32;
}

// proto: QStringRef QXmlStreamAttribute::value();
impl<'a> /*trait*/ QXmlStreamAttribute_value for () {
  fn value(self, this: &mut QXmlStreamAttribute) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZNK19QXmlStreamAttribute5valueEv()};
    unsafe {_ZNK19QXmlStreamAttribute5valueEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamAttribute {
  pub fn namespaceUri<T: QXmlStreamAttribute_namespaceUri>(&mut self, value: T) -> i32 {
    value.namespaceUri(self);
    return 1;
  }
}

pub trait QXmlStreamAttribute_namespaceUri {
  fn namespaceUri(self, this: &mut QXmlStreamAttribute) -> i32;
}

// proto: QStringRef QXmlStreamAttribute::namespaceUri();
impl<'a> /*trait*/ QXmlStreamAttribute_namespaceUri for () {
  fn namespaceUri(self, this: &mut QXmlStreamAttribute) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZNK19QXmlStreamAttribute12namespaceUriEv()};
    unsafe {_ZNK19QXmlStreamAttribute12namespaceUriEv()};
    return 1;
  }
}

// proto: void QXmlStreamAttribute::NewQXmlStreamAttribute();
impl<'a> /*trait*/ QXmlStreamAttribute_NewQXmlStreamAttribute for () {
  fn NewQXmlStreamAttribute(self) -> QXmlStreamAttribute {
    let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN19QXmlStreamAttributeC1Ev()};
    unsafe {_ZN19QXmlStreamAttributeC1Ev(qthis)};
    let rsthis = QXmlStreamAttribute{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QXmlStreamAttribute::NewQXmlStreamAttribute(const QXmlStreamAttribute & );
impl<'a> /*trait*/ QXmlStreamAttribute_NewQXmlStreamAttribute for (&'a  QXmlStreamAttribute) {
  fn NewQXmlStreamAttribute(self) -> QXmlStreamAttribute {
    let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN19QXmlStreamAttributeC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN19QXmlStreamAttributeC1ERKS_(qthis, arg0)};
    let rsthis = QXmlStreamAttribute{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QXmlStreamAttribute::NewQXmlStreamAttribute(const QString & namespaceUri, const QString & name, const QString & value);
impl<'a> /*trait*/ QXmlStreamAttribute_NewQXmlStreamAttribute for (&'a  QString, &'a  QString, &'a  QString) {
  fn NewQXmlStreamAttribute(self) -> QXmlStreamAttribute {
    let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN19QXmlStreamAttributeC1ERK7QStringS2_S2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN19QXmlStreamAttributeC1ERK7QStringS2_S2_(qthis, arg0, arg1, arg2)};
    let rsthis = QXmlStreamAttribute{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamAttribute {
  pub fn isDefault<T: QXmlStreamAttribute_isDefault>(&mut self, value: T) -> i32 {
    value.isDefault(self);
    return 1;
  }
}

pub trait QXmlStreamAttribute_isDefault {
  fn isDefault(self, this: &mut QXmlStreamAttribute) -> i32;
}

// proto: bool QXmlStreamAttribute::isDefault();
impl<'a> /*trait*/ QXmlStreamAttribute_isDefault for () {
  fn isDefault(self, this: &mut QXmlStreamAttribute) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZNK19QXmlStreamAttribute9isDefaultEv()};
    unsafe {_ZNK19QXmlStreamAttribute9isDefaultEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamAttribute {
  pub fn prefix<T: QXmlStreamAttribute_prefix>(&mut self, value: T) -> i32 {
    value.prefix(self);
    return 1;
  }
}

pub trait QXmlStreamAttribute_prefix {
  fn prefix(self, this: &mut QXmlStreamAttribute) -> i32;
}

// proto: QStringRef QXmlStreamAttribute::prefix();
impl<'a> /*trait*/ QXmlStreamAttribute_prefix for () {
  fn prefix(self, this: &mut QXmlStreamAttribute) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZNK19QXmlStreamAttribute6prefixEv()};
    unsafe {_ZNK19QXmlStreamAttribute6prefixEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamAttribute {
  pub fn name<T: QXmlStreamAttribute_name>(&mut self, value: T) -> i32 {
    value.name(self);
    return 1;
  }
}

pub trait QXmlStreamAttribute_name {
  fn name(self, this: &mut QXmlStreamAttribute) -> i32;
}

// proto: QStringRef QXmlStreamAttribute::name();
impl<'a> /*trait*/ QXmlStreamAttribute_name for () {
  fn name(self, this: &mut QXmlStreamAttribute) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZNK19QXmlStreamAttribute4nameEv()};
    unsafe {_ZNK19QXmlStreamAttribute4nameEv()};
    return 1;
  }
}

