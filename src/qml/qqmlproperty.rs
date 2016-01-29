// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQml/qqmlproperty.h
// dst-file: /src/qml/qqmlproperty.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use std::ops::Deref;
use super::super::core::qobject::QObject; // 771
use super::qqmlcontext::QQmlContext; // 773
use super::super::core::qstring::QString; // 771
use super::super::core::qmetaobject::QMetaMethod; // 771
use super::qqmlengine::QQmlEngine; // 773
use super::super::core::qvariant::QVariant; // 771
use super::super::core::qmetaobject::QMetaProperty; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QQmlProperty_Class_Size() -> c_int;
  // proto:  void QQmlProperty::QQmlProperty(QObject * );
  fn _ZN12QQmlPropertyC2EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QQmlProperty::connectNotifySignal(QObject * dest, const char * slot);
  fn _ZNK12QQmlProperty19connectNotifySignalEP7QObjectPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_char) -> c_char;
  // proto:  void QQmlProperty::QQmlProperty(const QQmlProperty & );
  fn _ZN12QQmlPropertyC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQmlProperty::QQmlProperty(QObject * , QQmlContext * );
  fn _ZN12QQmlPropertyC2EP7QObjectP11QQmlContext(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QString QQmlProperty::name();
  fn _ZNK12QQmlProperty4nameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QQmlProperty::index();
  fn _ZNK12QQmlProperty5indexEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QQmlProperty::~QQmlProperty();
  fn _ZN12QQmlPropertyD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QQmlProperty::QQmlProperty(QObject * , const QString & );
  fn _ZN12QQmlPropertyC2EP7QObjectRK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  bool QQmlProperty::isWritable();
  fn _ZNK12QQmlProperty10isWritableEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QMetaMethod QQmlProperty::method();
  fn _ZNK12QQmlProperty6methodEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QQmlProperty::QQmlProperty(QObject * , const QString & , QQmlEngine * );
  fn _ZN12QQmlPropertyC2EP7QObjectRK7QStringP10QQmlEngine(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto: static bool QQmlProperty::write(QObject * , const QString & , const QVariant & , QQmlEngine * );
  fn _ZN12QQmlProperty5writeEP7QObjectRK7QStringRK8QVariantP10QQmlEngine(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void) -> c_char;
  // proto:  const char * QQmlProperty::propertyTypeName();
  fn _ZNK12QQmlProperty16propertyTypeNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_char;
  // proto:  bool QQmlProperty::write(const QVariant & );
  fn _ZNK12QQmlProperty5writeERK8QVariant(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  bool QQmlProperty::isValid();
  fn _ZNK12QQmlProperty7isValidEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QQmlProperty::isProperty();
  fn _ZNK12QQmlProperty10isPropertyEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QQmlProperty::needsNotifySignal();
  fn _ZNK12QQmlProperty17needsNotifySignalEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QQmlProperty::QQmlProperty();
  fn _ZN12QQmlPropertyC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QVariant QQmlProperty::read();
  fn _ZNK12QQmlProperty4readEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static QVariant QQmlProperty::read(const QObject * , const QString & , QQmlContext * );
  fn _ZN12QQmlProperty4readEPK7QObjectRK7QStringP11QQmlContext(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;
  // proto: static bool QQmlProperty::write(QObject * , const QString & , const QVariant & );
  fn _ZN12QQmlProperty5writeEP7QObjectRK7QStringRK8QVariant(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> c_char;
  // proto:  int QQmlProperty::propertyType();
  fn _ZNK12QQmlProperty12propertyTypeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QQmlProperty::QQmlProperty(QObject * , const QString & , QQmlContext * );
  fn _ZN12QQmlPropertyC2EP7QObjectRK7QStringP11QQmlContext(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto: static QVariant QQmlProperty::read(const QObject * , const QString & , QQmlEngine * );
  fn _ZN12QQmlProperty4readEPK7QObjectRK7QStringP10QQmlEngine(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;
  // proto:  QMetaProperty QQmlProperty::property();
  fn _ZNK12QQmlProperty8propertyEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static QVariant QQmlProperty::read(const QObject * , const QString & );
  fn _ZN12QQmlProperty4readEPK7QObjectRK7QString(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  bool QQmlProperty::isSignalProperty();
  fn _ZNK12QQmlProperty16isSignalPropertyEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QQmlProperty::reset();
  fn _ZNK12QQmlProperty5resetEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QQmlProperty::hasNotifySignal();
  fn _ZNK12QQmlProperty15hasNotifySignalEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto: static bool QQmlProperty::write(QObject * , const QString & , const QVariant & , QQmlContext * );
  fn _ZN12QQmlProperty5writeEP7QObjectRK7QStringRK8QVariantP11QQmlContext(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void) -> c_char;
  // proto:  void QQmlProperty::QQmlProperty(QObject * , QQmlEngine * );
  fn _ZN12QQmlPropertyC2EP7QObjectP10QQmlEngine(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  bool QQmlProperty::isDesignable();
  fn _ZNK12QQmlProperty12isDesignableEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QQmlProperty::connectNotifySignal(QObject * dest, int method);
  fn _ZNK12QQmlProperty19connectNotifySignalEP7QObjecti(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int) -> c_char;
  // proto:  bool QQmlProperty::isResettable();
  fn _ZNK12QQmlProperty12isResettableEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QObject * QQmlProperty::object();
  fn _ZNK12QQmlProperty6objectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QQmlProperty)=8
#[derive(Default)]
pub struct QQmlProperty {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QQmlProperty {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QQmlProperty {
    return QQmlProperty{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QQmlProperty::QQmlProperty(QObject * );
impl /*struct*/ QQmlProperty {
  pub fn new<T: QQmlProperty_new>(value: T) -> QQmlProperty {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QQmlProperty_new {
  fn new(self) -> QQmlProperty;
}

  // proto:  void QQmlProperty::QQmlProperty(QObject * );
impl<'a> /*trait*/ QQmlProperty_new for (&'a QObject) {
  fn new(self) -> QQmlProperty {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QQmlPropertyC2EP7QObject()};
    let ctysz: c_int = unsafe{QQmlProperty_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QQmlPropertyC2EP7QObject(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlProperty{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QQmlProperty::connectNotifySignal(QObject * dest, const char * slot);
impl /*struct*/ QQmlProperty {
  pub fn connectNotifySignal<RetType, T: QQmlProperty_connectNotifySignal<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.connectNotifySignal(self);
    // return 1;
  }
}

pub trait QQmlProperty_connectNotifySignal<RetType> {
  fn connectNotifySignal(self , rsthis: & QQmlProperty) -> RetType;
}

  // proto:  bool QQmlProperty::connectNotifySignal(QObject * dest, const char * slot);
impl<'a> /*trait*/ QQmlProperty_connectNotifySignal<i8> for (&'a QObject, &'a  String) {
  fn connectNotifySignal(self , rsthis: & QQmlProperty) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QQmlProperty19connectNotifySignalEP7QObjectPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZNK12QQmlProperty19connectNotifySignalEP7QObjectPKc(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QQmlProperty::QQmlProperty(const QQmlProperty & );
impl<'a> /*trait*/ QQmlProperty_new for (&'a QQmlProperty) {
  fn new(self) -> QQmlProperty {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QQmlPropertyC2ERKS_()};
    let ctysz: c_int = unsafe{QQmlProperty_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QQmlPropertyC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlProperty{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QQmlProperty::QQmlProperty(QObject * , QQmlContext * );
impl<'a> /*trait*/ QQmlProperty_new for (&'a QObject, &'a QQmlContext) {
  fn new(self) -> QQmlProperty {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QQmlPropertyC2EP7QObjectP11QQmlContext()};
    let ctysz: c_int = unsafe{QQmlProperty_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN12QQmlPropertyC2EP7QObjectP11QQmlContext(qthis_ph, arg0, arg1)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlProperty{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QQmlProperty::name();
impl /*struct*/ QQmlProperty {
  pub fn name<RetType, T: QQmlProperty_name<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QQmlProperty_name<RetType> {
  fn name(self , rsthis: & QQmlProperty) -> RetType;
}

  // proto:  QString QQmlProperty::name();
impl<'a> /*trait*/ QQmlProperty_name<QString> for () {
  fn name(self , rsthis: & QQmlProperty) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QQmlProperty4nameEv()};
    let mut ret = unsafe {_ZNK12QQmlProperty4nameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QQmlProperty::index();
impl /*struct*/ QQmlProperty {
  pub fn index<RetType, T: QQmlProperty_index<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.index(self);
    // return 1;
  }
}

pub trait QQmlProperty_index<RetType> {
  fn index(self , rsthis: & QQmlProperty) -> RetType;
}

  // proto:  int QQmlProperty::index();
impl<'a> /*trait*/ QQmlProperty_index<i32> for () {
  fn index(self , rsthis: & QQmlProperty) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QQmlProperty5indexEv()};
    let mut ret = unsafe {_ZNK12QQmlProperty5indexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QQmlProperty::~QQmlProperty();
impl /*struct*/ QQmlProperty {
  pub fn free<RetType, T: QQmlProperty_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QQmlProperty_free<RetType> {
  fn free(self , rsthis: & QQmlProperty) -> RetType;
}

  // proto:  void QQmlProperty::~QQmlProperty();
impl<'a> /*trait*/ QQmlProperty_free<()> for () {
  fn free(self , rsthis: & QQmlProperty) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QQmlPropertyD2Ev()};
     unsafe {_ZN12QQmlPropertyD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QQmlProperty::QQmlProperty(QObject * , const QString & );
impl<'a> /*trait*/ QQmlProperty_new for (&'a QObject, &'a QString) {
  fn new(self) -> QQmlProperty {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QQmlPropertyC2EP7QObjectRK7QString()};
    let ctysz: c_int = unsafe{QQmlProperty_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN12QQmlPropertyC2EP7QObjectRK7QString(qthis_ph, arg0, arg1)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlProperty{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QQmlProperty::isWritable();
impl /*struct*/ QQmlProperty {
  pub fn isWritable<RetType, T: QQmlProperty_isWritable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isWritable(self);
    // return 1;
  }
}

pub trait QQmlProperty_isWritable<RetType> {
  fn isWritable(self , rsthis: & QQmlProperty) -> RetType;
}

  // proto:  bool QQmlProperty::isWritable();
impl<'a> /*trait*/ QQmlProperty_isWritable<i8> for () {
  fn isWritable(self , rsthis: & QQmlProperty) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QQmlProperty10isWritableEv()};
    let mut ret = unsafe {_ZNK12QQmlProperty10isWritableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QMetaMethod QQmlProperty::method();
impl /*struct*/ QQmlProperty {
  pub fn method<RetType, T: QQmlProperty_method<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.method(self);
    // return 1;
  }
}

pub trait QQmlProperty_method<RetType> {
  fn method(self , rsthis: & QQmlProperty) -> RetType;
}

  // proto:  QMetaMethod QQmlProperty::method();
impl<'a> /*trait*/ QQmlProperty_method<QMetaMethod> for () {
  fn method(self , rsthis: & QQmlProperty) -> QMetaMethod {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QQmlProperty6methodEv()};
    let mut ret = unsafe {_ZNK12QQmlProperty6methodEv(rsthis.qclsinst)};
    let mut ret1 = QMetaMethod::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQmlProperty::QQmlProperty(QObject * , const QString & , QQmlEngine * );
impl<'a> /*trait*/ QQmlProperty_new for (&'a QObject, &'a QString, &'a QQmlEngine) {
  fn new(self) -> QQmlProperty {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QQmlPropertyC2EP7QObjectRK7QStringP10QQmlEngine()};
    let ctysz: c_int = unsafe{QQmlProperty_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN12QQmlPropertyC2EP7QObjectRK7QStringP10QQmlEngine(qthis_ph, arg0, arg1, arg2)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlProperty{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto: static bool QQmlProperty::write(QObject * , const QString & , const QVariant & , QQmlEngine * );
impl /*struct*/ QQmlProperty {
  pub fn write_s<RetType, T: QQmlProperty_write_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.write_s();
    // return 1;
  }
}

pub trait QQmlProperty_write_s<RetType> {
  fn write_s(self ) -> RetType;
}

  // proto: static bool QQmlProperty::write(QObject * , const QString & , const QVariant & , QQmlEngine * );
impl<'a> /*trait*/ QQmlProperty_write_s<i8> for (&'a QObject, &'a QString, &'a QVariant, &'a QQmlEngine) {
  fn write_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QQmlProperty5writeEP7QObjectRK7QStringRK8QVariantP10QQmlEngine()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QQmlProperty5writeEP7QObjectRK7QStringRK8QVariantP10QQmlEngine(arg0, arg1, arg2, arg3)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  const char * QQmlProperty::propertyTypeName();
impl /*struct*/ QQmlProperty {
  pub fn propertyTypeName<RetType, T: QQmlProperty_propertyTypeName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.propertyTypeName(self);
    // return 1;
  }
}

pub trait QQmlProperty_propertyTypeName<RetType> {
  fn propertyTypeName(self , rsthis: & QQmlProperty) -> RetType;
}

  // proto:  const char * QQmlProperty::propertyTypeName();
impl<'a> /*trait*/ QQmlProperty_propertyTypeName<String> for () {
  fn propertyTypeName(self , rsthis: & QQmlProperty) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QQmlProperty16propertyTypeNameEv()};
    let mut ret = unsafe {_ZNK12QQmlProperty16propertyTypeNameEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto:  bool QQmlProperty::write(const QVariant & );
impl /*struct*/ QQmlProperty {
  pub fn write<RetType, T: QQmlProperty_write<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.write(self);
    // return 1;
  }
}

pub trait QQmlProperty_write<RetType> {
  fn write(self , rsthis: & QQmlProperty) -> RetType;
}

  // proto:  bool QQmlProperty::write(const QVariant & );
impl<'a> /*trait*/ QQmlProperty_write<i8> for (&'a QVariant) {
  fn write(self , rsthis: & QQmlProperty) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QQmlProperty5writeERK8QVariant()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QQmlProperty5writeERK8QVariant(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QQmlProperty::isValid();
impl /*struct*/ QQmlProperty {
  pub fn isValid<RetType, T: QQmlProperty_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QQmlProperty_isValid<RetType> {
  fn isValid(self , rsthis: & QQmlProperty) -> RetType;
}

  // proto:  bool QQmlProperty::isValid();
impl<'a> /*trait*/ QQmlProperty_isValid<i8> for () {
  fn isValid(self , rsthis: & QQmlProperty) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QQmlProperty7isValidEv()};
    let mut ret = unsafe {_ZNK12QQmlProperty7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QQmlProperty::isProperty();
impl /*struct*/ QQmlProperty {
  pub fn isProperty<RetType, T: QQmlProperty_isProperty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isProperty(self);
    // return 1;
  }
}

pub trait QQmlProperty_isProperty<RetType> {
  fn isProperty(self , rsthis: & QQmlProperty) -> RetType;
}

  // proto:  bool QQmlProperty::isProperty();
impl<'a> /*trait*/ QQmlProperty_isProperty<i8> for () {
  fn isProperty(self , rsthis: & QQmlProperty) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QQmlProperty10isPropertyEv()};
    let mut ret = unsafe {_ZNK12QQmlProperty10isPropertyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QQmlProperty::needsNotifySignal();
impl /*struct*/ QQmlProperty {
  pub fn needsNotifySignal<RetType, T: QQmlProperty_needsNotifySignal<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.needsNotifySignal(self);
    // return 1;
  }
}

pub trait QQmlProperty_needsNotifySignal<RetType> {
  fn needsNotifySignal(self , rsthis: & QQmlProperty) -> RetType;
}

  // proto:  bool QQmlProperty::needsNotifySignal();
impl<'a> /*trait*/ QQmlProperty_needsNotifySignal<i8> for () {
  fn needsNotifySignal(self , rsthis: & QQmlProperty) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QQmlProperty17needsNotifySignalEv()};
    let mut ret = unsafe {_ZNK12QQmlProperty17needsNotifySignalEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QQmlProperty::QQmlProperty();
impl<'a> /*trait*/ QQmlProperty_new for () {
  fn new(self) -> QQmlProperty {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QQmlPropertyC2Ev()};
    let ctysz: c_int = unsafe{QQmlProperty_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN12QQmlPropertyC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlProperty{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QVariant QQmlProperty::read();
impl /*struct*/ QQmlProperty {
  pub fn read<RetType, T: QQmlProperty_read<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.read(self);
    // return 1;
  }
}

pub trait QQmlProperty_read<RetType> {
  fn read(self , rsthis: & QQmlProperty) -> RetType;
}

  // proto:  QVariant QQmlProperty::read();
impl<'a> /*trait*/ QQmlProperty_read<QVariant> for () {
  fn read(self , rsthis: & QQmlProperty) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QQmlProperty4readEv()};
    let mut ret = unsafe {_ZNK12QQmlProperty4readEv(rsthis.qclsinst)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QVariant QQmlProperty::read(const QObject * , const QString & , QQmlContext * );
impl /*struct*/ QQmlProperty {
  pub fn read_s<RetType, T: QQmlProperty_read_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.read_s();
    // return 1;
  }
}

pub trait QQmlProperty_read_s<RetType> {
  fn read_s(self ) -> RetType;
}

  // proto: static QVariant QQmlProperty::read(const QObject * , const QString & , QQmlContext * );
impl<'a> /*trait*/ QQmlProperty_read_s<QVariant> for (&'a QObject, &'a QString, &'a QQmlContext) {
  fn read_s(self ) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QQmlProperty4readEPK7QObjectRK7QStringP11QQmlContext()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QQmlProperty4readEPK7QObjectRK7QStringP11QQmlContext(arg0, arg1, arg2)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static bool QQmlProperty::write(QObject * , const QString & , const QVariant & );
impl<'a> /*trait*/ QQmlProperty_write_s<i8> for (&'a QObject, &'a QString, &'a QVariant) {
  fn write_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QQmlProperty5writeEP7QObjectRK7QStringRK8QVariant()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QQmlProperty5writeEP7QObjectRK7QStringRK8QVariant(arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QQmlProperty::propertyType();
impl /*struct*/ QQmlProperty {
  pub fn propertyType<RetType, T: QQmlProperty_propertyType<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.propertyType(self);
    // return 1;
  }
}

pub trait QQmlProperty_propertyType<RetType> {
  fn propertyType(self , rsthis: & QQmlProperty) -> RetType;
}

  // proto:  int QQmlProperty::propertyType();
impl<'a> /*trait*/ QQmlProperty_propertyType<i32> for () {
  fn propertyType(self , rsthis: & QQmlProperty) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QQmlProperty12propertyTypeEv()};
    let mut ret = unsafe {_ZNK12QQmlProperty12propertyTypeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QQmlProperty::QQmlProperty(QObject * , const QString & , QQmlContext * );
impl<'a> /*trait*/ QQmlProperty_new for (&'a QObject, &'a QString, &'a QQmlContext) {
  fn new(self) -> QQmlProperty {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QQmlPropertyC2EP7QObjectRK7QStringP11QQmlContext()};
    let ctysz: c_int = unsafe{QQmlProperty_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN12QQmlPropertyC2EP7QObjectRK7QStringP11QQmlContext(qthis_ph, arg0, arg1, arg2)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlProperty{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto: static QVariant QQmlProperty::read(const QObject * , const QString & , QQmlEngine * );
impl<'a> /*trait*/ QQmlProperty_read_s<QVariant> for (&'a QObject, &'a QString, &'a QQmlEngine) {
  fn read_s(self ) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QQmlProperty4readEPK7QObjectRK7QStringP10QQmlEngine()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QQmlProperty4readEPK7QObjectRK7QStringP10QQmlEngine(arg0, arg1, arg2)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QMetaProperty QQmlProperty::property();
impl /*struct*/ QQmlProperty {
  pub fn property<RetType, T: QQmlProperty_property<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.property(self);
    // return 1;
  }
}

pub trait QQmlProperty_property<RetType> {
  fn property(self , rsthis: & QQmlProperty) -> RetType;
}

  // proto:  QMetaProperty QQmlProperty::property();
impl<'a> /*trait*/ QQmlProperty_property<QMetaProperty> for () {
  fn property(self , rsthis: & QQmlProperty) -> QMetaProperty {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QQmlProperty8propertyEv()};
    let mut ret = unsafe {_ZNK12QQmlProperty8propertyEv(rsthis.qclsinst)};
    let mut ret1 = QMetaProperty::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QVariant QQmlProperty::read(const QObject * , const QString & );
impl<'a> /*trait*/ QQmlProperty_read_s<QVariant> for (&'a QObject, &'a QString) {
  fn read_s(self ) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QQmlProperty4readEPK7QObjectRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QQmlProperty4readEPK7QObjectRK7QString(arg0, arg1)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QQmlProperty::isSignalProperty();
impl /*struct*/ QQmlProperty {
  pub fn isSignalProperty<RetType, T: QQmlProperty_isSignalProperty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSignalProperty(self);
    // return 1;
  }
}

pub trait QQmlProperty_isSignalProperty<RetType> {
  fn isSignalProperty(self , rsthis: & QQmlProperty) -> RetType;
}

  // proto:  bool QQmlProperty::isSignalProperty();
impl<'a> /*trait*/ QQmlProperty_isSignalProperty<i8> for () {
  fn isSignalProperty(self , rsthis: & QQmlProperty) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QQmlProperty16isSignalPropertyEv()};
    let mut ret = unsafe {_ZNK12QQmlProperty16isSignalPropertyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QQmlProperty::reset();
impl /*struct*/ QQmlProperty {
  pub fn reset<RetType, T: QQmlProperty_reset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.reset(self);
    // return 1;
  }
}

pub trait QQmlProperty_reset<RetType> {
  fn reset(self , rsthis: & QQmlProperty) -> RetType;
}

  // proto:  bool QQmlProperty::reset();
impl<'a> /*trait*/ QQmlProperty_reset<i8> for () {
  fn reset(self , rsthis: & QQmlProperty) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QQmlProperty5resetEv()};
    let mut ret = unsafe {_ZNK12QQmlProperty5resetEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QQmlProperty::hasNotifySignal();
impl /*struct*/ QQmlProperty {
  pub fn hasNotifySignal<RetType, T: QQmlProperty_hasNotifySignal<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasNotifySignal(self);
    // return 1;
  }
}

pub trait QQmlProperty_hasNotifySignal<RetType> {
  fn hasNotifySignal(self , rsthis: & QQmlProperty) -> RetType;
}

  // proto:  bool QQmlProperty::hasNotifySignal();
impl<'a> /*trait*/ QQmlProperty_hasNotifySignal<i8> for () {
  fn hasNotifySignal(self , rsthis: & QQmlProperty) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QQmlProperty15hasNotifySignalEv()};
    let mut ret = unsafe {_ZNK12QQmlProperty15hasNotifySignalEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static bool QQmlProperty::write(QObject * , const QString & , const QVariant & , QQmlContext * );
impl<'a> /*trait*/ QQmlProperty_write_s<i8> for (&'a QObject, &'a QString, &'a QVariant, &'a QQmlContext) {
  fn write_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QQmlProperty5writeEP7QObjectRK7QStringRK8QVariantP11QQmlContext()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QQmlProperty5writeEP7QObjectRK7QStringRK8QVariantP11QQmlContext(arg0, arg1, arg2, arg3)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QQmlProperty::QQmlProperty(QObject * , QQmlEngine * );
impl<'a> /*trait*/ QQmlProperty_new for (&'a QObject, &'a QQmlEngine) {
  fn new(self) -> QQmlProperty {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QQmlPropertyC2EP7QObjectP10QQmlEngine()};
    let ctysz: c_int = unsafe{QQmlProperty_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN12QQmlPropertyC2EP7QObjectP10QQmlEngine(qthis_ph, arg0, arg1)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlProperty{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QQmlProperty::isDesignable();
impl /*struct*/ QQmlProperty {
  pub fn isDesignable<RetType, T: QQmlProperty_isDesignable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isDesignable(self);
    // return 1;
  }
}

pub trait QQmlProperty_isDesignable<RetType> {
  fn isDesignable(self , rsthis: & QQmlProperty) -> RetType;
}

  // proto:  bool QQmlProperty::isDesignable();
impl<'a> /*trait*/ QQmlProperty_isDesignable<i8> for () {
  fn isDesignable(self , rsthis: & QQmlProperty) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QQmlProperty12isDesignableEv()};
    let mut ret = unsafe {_ZNK12QQmlProperty12isDesignableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QQmlProperty::connectNotifySignal(QObject * dest, int method);
impl<'a> /*trait*/ QQmlProperty_connectNotifySignal<i8> for (&'a QObject, i32) {
  fn connectNotifySignal(self , rsthis: & QQmlProperty) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QQmlProperty19connectNotifySignalEP7QObjecti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK12QQmlProperty19connectNotifySignalEP7QObjecti(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QQmlProperty::isResettable();
impl /*struct*/ QQmlProperty {
  pub fn isResettable<RetType, T: QQmlProperty_isResettable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isResettable(self);
    // return 1;
  }
}

pub trait QQmlProperty_isResettable<RetType> {
  fn isResettable(self , rsthis: & QQmlProperty) -> RetType;
}

  // proto:  bool QQmlProperty::isResettable();
impl<'a> /*trait*/ QQmlProperty_isResettable<i8> for () {
  fn isResettable(self , rsthis: & QQmlProperty) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QQmlProperty12isResettableEv()};
    let mut ret = unsafe {_ZNK12QQmlProperty12isResettableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QObject * QQmlProperty::object();
impl /*struct*/ QQmlProperty {
  pub fn object<RetType, T: QQmlProperty_object<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.object(self);
    // return 1;
  }
}

pub trait QQmlProperty_object<RetType> {
  fn object(self , rsthis: & QQmlProperty) -> RetType;
}

  // proto:  QObject * QQmlProperty::object();
impl<'a> /*trait*/ QQmlProperty_object<QObject> for () {
  fn object(self , rsthis: & QQmlProperty) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QQmlProperty6objectEv()};
    let mut ret = unsafe {_ZNK12QQmlProperty6objectEv(rsthis.qclsinst)};
    let mut ret1 = QObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

