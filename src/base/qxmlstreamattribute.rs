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
  // proto:  void QXmlStreamAttribute::NewQXmlStreamAttribute(const QString & qualifiedName, const QString & value);
  fn _ZN19QXmlStreamAttributeC1ERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  QStringRef QXmlStreamAttribute::qualifiedName();
  fn _ZNK19QXmlStreamAttribute13qualifiedNameEv(qthis: *mut c_void) ;
  // proto:  void QXmlStreamAttribute::FreeQXmlStreamAttribute();
  fn _ZN19QXmlStreamAttributeD0Ev(qthis: *mut c_void) ;
  // proto:  QStringRef QXmlStreamAttribute::value();
  fn _ZNK19QXmlStreamAttribute5valueEv(qthis: *mut c_void) ;
  // proto:  QStringRef QXmlStreamAttribute::namespaceUri();
  fn _ZNK19QXmlStreamAttribute12namespaceUriEv(qthis: *mut c_void) ;
  // proto:  void QXmlStreamAttribute::NewQXmlStreamAttribute();
  fn _ZN19QXmlStreamAttributeC1Ev(qthis: *mut c_void) ;
  // proto:  void QXmlStreamAttribute::NewQXmlStreamAttribute(const QXmlStreamAttribute & );
  fn _ZN19QXmlStreamAttributeC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QXmlStreamAttribute::NewQXmlStreamAttribute(const QString & namespaceUri, const QString & name, const QString & value);
  fn _ZN19QXmlStreamAttributeC1ERK7QStringS2_S2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  bool QXmlStreamAttribute::isDefault();
  fn _ZNK19QXmlStreamAttribute9isDefaultEv(qthis: *mut c_void) -> int8_t;
  // proto:  QStringRef QXmlStreamAttribute::prefix();
  fn _ZNK19QXmlStreamAttribute6prefixEv(qthis: *mut c_void) ;
  // proto:  QStringRef QXmlStreamAttribute::name();
  fn _ZNK19QXmlStreamAttribute4nameEv(qthis: *mut c_void) ;
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
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN19QXmlStreamAttributeC1ERK7QStringS2_(qthis, arg0, arg1)};
    let rsthis = QXmlStreamAttribute{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamAttribute {
  pub fn qualifiedName<T: QXmlStreamAttribute_qualifiedName>(&mut self, value: T)  {
     value.qualifiedName(self);
    // return 1;
  }
}

pub trait QXmlStreamAttribute_qualifiedName {
  fn qualifiedName(self, rsthis: &mut QXmlStreamAttribute) ;
}

// proto:  QStringRef QXmlStreamAttribute::qualifiedName();
impl<'a> /*trait*/ QXmlStreamAttribute_qualifiedName for () {
  fn qualifiedName(self, rsthis: &mut QXmlStreamAttribute)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZNK19QXmlStreamAttribute13qualifiedNameEv()};
     unsafe {_ZNK19QXmlStreamAttribute13qualifiedNameEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QXmlStreamAttribute {
  pub fn FreeQXmlStreamAttribute<T: QXmlStreamAttribute_FreeQXmlStreamAttribute>(&mut self, value: T)  {
     value.FreeQXmlStreamAttribute(self);
    // return 1;
  }
}

pub trait QXmlStreamAttribute_FreeQXmlStreamAttribute {
  fn FreeQXmlStreamAttribute(self, rsthis: &mut QXmlStreamAttribute) ;
}

// proto:  void QXmlStreamAttribute::FreeQXmlStreamAttribute();
impl<'a> /*trait*/ QXmlStreamAttribute_FreeQXmlStreamAttribute for () {
  fn FreeQXmlStreamAttribute(self, rsthis: &mut QXmlStreamAttribute)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN19QXmlStreamAttributeD0Ev()};
     unsafe {_ZN19QXmlStreamAttributeD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QXmlStreamAttribute {
  pub fn value<T: QXmlStreamAttribute_value>(&mut self, value: T)  {
     value.value(self);
    // return 1;
  }
}

pub trait QXmlStreamAttribute_value {
  fn value(self, rsthis: &mut QXmlStreamAttribute) ;
}

// proto:  QStringRef QXmlStreamAttribute::value();
impl<'a> /*trait*/ QXmlStreamAttribute_value for () {
  fn value(self, rsthis: &mut QXmlStreamAttribute)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZNK19QXmlStreamAttribute5valueEv()};
     unsafe {_ZNK19QXmlStreamAttribute5valueEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QXmlStreamAttribute {
  pub fn namespaceUri<T: QXmlStreamAttribute_namespaceUri>(&mut self, value: T)  {
     value.namespaceUri(self);
    // return 1;
  }
}

pub trait QXmlStreamAttribute_namespaceUri {
  fn namespaceUri(self, rsthis: &mut QXmlStreamAttribute) ;
}

// proto:  QStringRef QXmlStreamAttribute::namespaceUri();
impl<'a> /*trait*/ QXmlStreamAttribute_namespaceUri for () {
  fn namespaceUri(self, rsthis: &mut QXmlStreamAttribute)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZNK19QXmlStreamAttribute12namespaceUriEv()};
     unsafe {_ZNK19QXmlStreamAttribute12namespaceUriEv(rsthis.qclsinst)};
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
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
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN19QXmlStreamAttributeC1ERK7QStringS2_S2_(qthis, arg0, arg1, arg2)};
    let rsthis = QXmlStreamAttribute{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamAttribute {
  pub fn isDefault<T: QXmlStreamAttribute_isDefault>(&mut self, value: T) -> i8 {
    return value.isDefault(self);
    // return 1;
  }
}

pub trait QXmlStreamAttribute_isDefault {
  fn isDefault(self, rsthis: &mut QXmlStreamAttribute) -> i8;
}

// proto:  bool QXmlStreamAttribute::isDefault();
impl<'a> /*trait*/ QXmlStreamAttribute_isDefault for () {
  fn isDefault(self, rsthis: &mut QXmlStreamAttribute) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZNK19QXmlStreamAttribute9isDefaultEv()};
    let mut ret = unsafe {_ZNK19QXmlStreamAttribute9isDefaultEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamAttribute {
  pub fn prefix<T: QXmlStreamAttribute_prefix>(&mut self, value: T)  {
     value.prefix(self);
    // return 1;
  }
}

pub trait QXmlStreamAttribute_prefix {
  fn prefix(self, rsthis: &mut QXmlStreamAttribute) ;
}

// proto:  QStringRef QXmlStreamAttribute::prefix();
impl<'a> /*trait*/ QXmlStreamAttribute_prefix for () {
  fn prefix(self, rsthis: &mut QXmlStreamAttribute)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZNK19QXmlStreamAttribute6prefixEv()};
     unsafe {_ZNK19QXmlStreamAttribute6prefixEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QXmlStreamAttribute {
  pub fn name<T: QXmlStreamAttribute_name>(&mut self, value: T)  {
     value.name(self);
    // return 1;
  }
}

pub trait QXmlStreamAttribute_name {
  fn name(self, rsthis: &mut QXmlStreamAttribute) ;
}

// proto:  QStringRef QXmlStreamAttribute::name();
impl<'a> /*trait*/ QXmlStreamAttribute_name for () {
  fn name(self, rsthis: &mut QXmlStreamAttribute)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZNK19QXmlStreamAttribute4nameEv()};
     unsafe {_ZNK19QXmlStreamAttribute4nameEv(rsthis.qclsinst)};
    // return 1;
  }
}

