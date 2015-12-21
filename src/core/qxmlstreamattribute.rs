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
  // proto:  void QXmlStreamAttribute::QXmlStreamAttribute(const QString & qualifiedName, const QString & value);
  fn _ZN19QXmlStreamAttributeC1ERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QStringRef QXmlStreamAttribute::qualifiedName();
  fn _ZNK19QXmlStreamAttribute13qualifiedNameEv(qthis: *mut c_void);
  // proto:  void QXmlStreamAttribute::~QXmlStreamAttribute();
  fn _ZN19QXmlStreamAttributeD0Ev(qthis: *mut c_void);
  // proto:  QStringRef QXmlStreamAttribute::value();
  fn _ZNK19QXmlStreamAttribute5valueEv(qthis: *mut c_void);
  // proto:  QStringRef QXmlStreamAttribute::namespaceUri();
  fn _ZNK19QXmlStreamAttribute12namespaceUriEv(qthis: *mut c_void);
  // proto:  void QXmlStreamAttribute::QXmlStreamAttribute();
  fn _ZN19QXmlStreamAttributeC1Ev(qthis: *mut c_void);
  // proto:  void QXmlStreamAttribute::QXmlStreamAttribute(const QXmlStreamAttribute & );
  fn _ZN19QXmlStreamAttributeC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QXmlStreamAttribute::QXmlStreamAttribute(const QString & namespaceUri, const QString & name, const QString & value);
  fn _ZN19QXmlStreamAttributeC1ERK7QStringS2_S2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  bool QXmlStreamAttribute::isDefault();
  fn _ZNK19QXmlStreamAttribute9isDefaultEv(qthis: *mut c_void) -> c_char;
  // proto:  QStringRef QXmlStreamAttribute::prefix();
  fn _ZNK19QXmlStreamAttribute6prefixEv(qthis: *mut c_void);
  // proto:  QStringRef QXmlStreamAttribute::name();
  fn _ZNK19QXmlStreamAttribute4nameEv(qthis: *mut c_void);
}

// body block begin
// class sizeof(QXmlStreamAttribute)=80
pub struct QXmlStreamAttribute {
  pub qclsinst: *mut c_void,
}

  // proto:  void QXmlStreamAttribute::QXmlStreamAttribute(const QString & qualifiedName, const QString & value);
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

  // proto:  void QXmlStreamAttribute::QXmlStreamAttribute(const QString & qualifiedName, const QString & value);
impl<'a> /*trait*/ QXmlStreamAttribute_NewQXmlStreamAttribute for (QString, QString) {
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

  // proto:  QStringRef QXmlStreamAttribute::qualifiedName();
impl /*struct*/ QXmlStreamAttribute {
  pub fn qualifiedName<RetType, T: QXmlStreamAttribute_qualifiedName<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.qualifiedName(self);
    // return 1;
  }
}

pub trait QXmlStreamAttribute_qualifiedName<RetType> {
  fn qualifiedName(self , rsthis: &mut QXmlStreamAttribute) -> RetType;
}

  // proto:  QStringRef QXmlStreamAttribute::qualifiedName();
impl<'a> /*trait*/ QXmlStreamAttribute_qualifiedName<()> for () {
  fn qualifiedName(self , rsthis: &mut QXmlStreamAttribute) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZNK19QXmlStreamAttribute13qualifiedNameEv()};
     unsafe {_ZNK19QXmlStreamAttribute13qualifiedNameEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QXmlStreamAttribute::~QXmlStreamAttribute();
impl /*struct*/ QXmlStreamAttribute {
  pub fn FreeQXmlStreamAttribute<RetType, T: QXmlStreamAttribute_FreeQXmlStreamAttribute<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQXmlStreamAttribute(self);
    // return 1;
  }
}

pub trait QXmlStreamAttribute_FreeQXmlStreamAttribute<RetType> {
  fn FreeQXmlStreamAttribute(self , rsthis: &mut QXmlStreamAttribute) -> RetType;
}

  // proto:  void QXmlStreamAttribute::~QXmlStreamAttribute();
impl<'a> /*trait*/ QXmlStreamAttribute_FreeQXmlStreamAttribute<()> for () {
  fn FreeQXmlStreamAttribute(self , rsthis: &mut QXmlStreamAttribute) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN19QXmlStreamAttributeD0Ev()};
     unsafe {_ZN19QXmlStreamAttributeD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamAttribute::value();
impl /*struct*/ QXmlStreamAttribute {
  pub fn value<RetType, T: QXmlStreamAttribute_value<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QXmlStreamAttribute_value<RetType> {
  fn value(self , rsthis: &mut QXmlStreamAttribute) -> RetType;
}

  // proto:  QStringRef QXmlStreamAttribute::value();
impl<'a> /*trait*/ QXmlStreamAttribute_value<()> for () {
  fn value(self , rsthis: &mut QXmlStreamAttribute) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZNK19QXmlStreamAttribute5valueEv()};
     unsafe {_ZNK19QXmlStreamAttribute5valueEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamAttribute::namespaceUri();
impl /*struct*/ QXmlStreamAttribute {
  pub fn namespaceUri<RetType, T: QXmlStreamAttribute_namespaceUri<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.namespaceUri(self);
    // return 1;
  }
}

pub trait QXmlStreamAttribute_namespaceUri<RetType> {
  fn namespaceUri(self , rsthis: &mut QXmlStreamAttribute) -> RetType;
}

  // proto:  QStringRef QXmlStreamAttribute::namespaceUri();
impl<'a> /*trait*/ QXmlStreamAttribute_namespaceUri<()> for () {
  fn namespaceUri(self , rsthis: &mut QXmlStreamAttribute) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZNK19QXmlStreamAttribute12namespaceUriEv()};
     unsafe {_ZNK19QXmlStreamAttribute12namespaceUriEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QXmlStreamAttribute::QXmlStreamAttribute();
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

  // proto:  void QXmlStreamAttribute::QXmlStreamAttribute(const QXmlStreamAttribute & );
impl<'a> /*trait*/ QXmlStreamAttribute_NewQXmlStreamAttribute for (QXmlStreamAttribute) {
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

  // proto:  void QXmlStreamAttribute::QXmlStreamAttribute(const QString & namespaceUri, const QString & name, const QString & value);
impl<'a> /*trait*/ QXmlStreamAttribute_NewQXmlStreamAttribute for (QString, QString, QString) {
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

  // proto:  bool QXmlStreamAttribute::isDefault();
impl /*struct*/ QXmlStreamAttribute {
  pub fn isDefault<RetType, T: QXmlStreamAttribute_isDefault<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isDefault(self);
    // return 1;
  }
}

pub trait QXmlStreamAttribute_isDefault<RetType> {
  fn isDefault(self , rsthis: &mut QXmlStreamAttribute) -> RetType;
}

  // proto:  bool QXmlStreamAttribute::isDefault();
impl<'a> /*trait*/ QXmlStreamAttribute_isDefault<i8> for () {
  fn isDefault(self , rsthis: &mut QXmlStreamAttribute) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZNK19QXmlStreamAttribute9isDefaultEv()};
    let mut ret = unsafe {_ZNK19QXmlStreamAttribute9isDefaultEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamAttribute::prefix();
impl /*struct*/ QXmlStreamAttribute {
  pub fn prefix<RetType, T: QXmlStreamAttribute_prefix<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.prefix(self);
    // return 1;
  }
}

pub trait QXmlStreamAttribute_prefix<RetType> {
  fn prefix(self , rsthis: &mut QXmlStreamAttribute) -> RetType;
}

  // proto:  QStringRef QXmlStreamAttribute::prefix();
impl<'a> /*trait*/ QXmlStreamAttribute_prefix<()> for () {
  fn prefix(self , rsthis: &mut QXmlStreamAttribute) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZNK19QXmlStreamAttribute6prefixEv()};
     unsafe {_ZNK19QXmlStreamAttribute6prefixEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamAttribute::name();
impl /*struct*/ QXmlStreamAttribute {
  pub fn name<RetType, T: QXmlStreamAttribute_name<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QXmlStreamAttribute_name<RetType> {
  fn name(self , rsthis: &mut QXmlStreamAttribute) -> RetType;
}

  // proto:  QStringRef QXmlStreamAttribute::name();
impl<'a> /*trait*/ QXmlStreamAttribute_name<()> for () {
  fn name(self , rsthis: &mut QXmlStreamAttribute) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZNK19QXmlStreamAttribute4nameEv()};
     unsafe {_ZNK19QXmlStreamAttribute4nameEv(rsthis.qclsinst)};
    // return 1;
  }
}

