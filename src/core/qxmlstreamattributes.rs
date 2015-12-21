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
  // proto:  QStringRef QXmlStreamAttributes::value(const QString & qualifiedName);
  fn _ZNK20QXmlStreamAttributes5valueERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QXmlStreamAttributes::QXmlStreamAttributes();
  fn _ZN20QXmlStreamAttributesC1Ev(qthis: *mut c_void);
  // proto:  bool QXmlStreamAttributes::hasAttribute(const QString & qualifiedName);
  fn _ZNK20QXmlStreamAttributes12hasAttributeERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  bool QXmlStreamAttributes::hasAttribute(const QString & namespaceUri, const QString & name);
  fn _ZNK20QXmlStreamAttributes12hasAttributeERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto:  void QXmlStreamAttributes::append(const QString & namespaceUri, const QString & name, const QString & value);
  fn _ZN20QXmlStreamAttributes6appendERK7QStringS2_S2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QXmlStreamAttributes::append(const QString & qualifiedName, const QString & value);
  fn _ZN20QXmlStreamAttributes6appendERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QStringRef QXmlStreamAttributes::value(const QString & namespaceUri, const QString & name);
  fn _ZNK20QXmlStreamAttributes5valueERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
}

// body block begin
// class sizeof(QXmlStreamAttributes)=1
pub struct QXmlStreamAttributes {
  pub qclsinst: *mut c_void,
}

  // proto:  QStringRef QXmlStreamAttributes::value(const QString & qualifiedName);
impl /*struct*/ QXmlStreamAttributes {
  pub fn value<RetType, T: QXmlStreamAttributes_value<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QXmlStreamAttributes_value<RetType> {
  fn value(self , rsthis: &mut QXmlStreamAttributes) -> RetType;
}

  // proto:  QStringRef QXmlStreamAttributes::value(const QString & qualifiedName);
impl<'a> /*trait*/ QXmlStreamAttributes_value<()> for (QString) {
  fn value(self , rsthis: &mut QXmlStreamAttributes) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QXmlStreamAttributes5valueERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK20QXmlStreamAttributes5valueERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QXmlStreamAttributes::QXmlStreamAttributes();
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

  // proto:  void QXmlStreamAttributes::QXmlStreamAttributes();
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

  // proto:  bool QXmlStreamAttributes::hasAttribute(const QString & qualifiedName);
impl /*struct*/ QXmlStreamAttributes {
  pub fn hasAttribute<RetType, T: QXmlStreamAttributes_hasAttribute<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.hasAttribute(self);
    // return 1;
  }
}

pub trait QXmlStreamAttributes_hasAttribute<RetType> {
  fn hasAttribute(self , rsthis: &mut QXmlStreamAttributes) -> RetType;
}

  // proto:  bool QXmlStreamAttributes::hasAttribute(const QString & qualifiedName);
impl<'a> /*trait*/ QXmlStreamAttributes_hasAttribute<i8> for (QString) {
  fn hasAttribute(self , rsthis: &mut QXmlStreamAttributes) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QXmlStreamAttributes12hasAttributeERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK20QXmlStreamAttributes12hasAttributeERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QXmlStreamAttributes::hasAttribute(const QString & namespaceUri, const QString & name);
impl<'a> /*trait*/ QXmlStreamAttributes_hasAttribute<i8> for (QString, QString) {
  fn hasAttribute(self , rsthis: &mut QXmlStreamAttributes) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QXmlStreamAttributes12hasAttributeERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK20QXmlStreamAttributes12hasAttributeERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QXmlStreamAttributes::append(const QString & namespaceUri, const QString & name, const QString & value);
impl /*struct*/ QXmlStreamAttributes {
  pub fn append<RetType, T: QXmlStreamAttributes_append<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.append(self);
    // return 1;
  }
}

pub trait QXmlStreamAttributes_append<RetType> {
  fn append(self , rsthis: &mut QXmlStreamAttributes) -> RetType;
}

  // proto:  void QXmlStreamAttributes::append(const QString & namespaceUri, const QString & name, const QString & value);
impl<'a> /*trait*/ QXmlStreamAttributes_append<()> for (QString, QString, QString) {
  fn append(self , rsthis: &mut QXmlStreamAttributes) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QXmlStreamAttributes6appendERK7QStringS2_S2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN20QXmlStreamAttributes6appendERK7QStringS2_S2_(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QXmlStreamAttributes::append(const QString & qualifiedName, const QString & value);
impl<'a> /*trait*/ QXmlStreamAttributes_append<()> for (QString, QString) {
  fn append(self , rsthis: &mut QXmlStreamAttributes) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QXmlStreamAttributes6appendERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QXmlStreamAttributes6appendERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamAttributes::value(const QString & namespaceUri, const QString & name);
impl<'a> /*trait*/ QXmlStreamAttributes_value<()> for (QString, QString) {
  fn value(self , rsthis: &mut QXmlStreamAttributes) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QXmlStreamAttributes5valueERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZNK20QXmlStreamAttributes5valueERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

