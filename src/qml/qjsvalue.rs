// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQml/qjsvalue.h
// dst-file: /src/qml/qjsvalue.rs
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
use super::super::core::qstring::QString; // 771
use super::super::core::qdatetime::QDateTime; // 771
use super::super::core::qobject::QObject; // 771
use super::super::core::qvariant::QVariant; // 771
use super::qjsengine::QJSEngine; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QJSValue_Class_Size() -> c_int;
  // proto:  QJSValue QJSValue::prototype();
  fn _ZNK8QJSValue9prototypeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QJSValue::setProperty(const QString & name, const QJSValue & value);
  fn _ZN8QJSValue11setPropertyERK7QStringRKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QJSValue::setProperty(quint32 arrayIndex, const QJSValue & value);
  fn _ZN8QJSValue11setPropertyEjRKS_(qthis: u64 /* *mut c_void*/, arg0: c_uint, arg1: *mut c_void);
  // proto:  bool QJSValue::isError();
  fn _ZNK8QJSValue7isErrorEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QJSValue::isBool();
  fn _ZNK8QJSValue6isBoolEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QJSValue::QJSValue(const QString & value);
  fn _ZN8QJSValueC2ERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QDateTime QJSValue::toDateTime();
  fn _ZNK8QJSValue10toDateTimeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QJSValue::isCallable();
  fn _ZNK8QJSValue10isCallableEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QJSValue::isString();
  fn _ZNK8QJSValue8isStringEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  qint32 QJSValue::toInt();
  fn _ZNK8QJSValue5toIntEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QJSValue::isVariant();
  fn _ZNK8QJSValue9isVariantEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QJSValue::isUndefined();
  fn _ZNK8QJSValue11isUndefinedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QJSValue::setPrototype(const QJSValue & prototype);
  fn _ZN8QJSValue12setPrototypeERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QJSValue::isArray();
  fn _ZNK8QJSValue7isArrayEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QJSValue QJSValue::property(const QString & name);
  fn _ZNK8QJSValue8propertyERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QJSValue::isObject();
  fn _ZNK8QJSValue8isObjectEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QJSValue QJSValue::property(quint32 arrayIndex);
  fn _ZNK8QJSValue8propertyEj(qthis: u64 /* *mut c_void*/, arg0: c_uint) -> *mut c_void;
  // proto:  QObject * QJSValue::toQObject();
  fn _ZNK8QJSValue9toQObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QJSValue::QJSValue(void * );
  fn _ZN8QJSValueC2EPv(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QJSValue::isNumber();
  fn _ZNK8QJSValue8isNumberEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QJSValue::hasOwnProperty(const QString & name);
  fn _ZNK8QJSValue14hasOwnPropertyERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  bool QJSValue::hasProperty(const QString & name);
  fn _ZNK8QJSValue11hasPropertyERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  bool QJSValue::deleteProperty(const QString & name);
  fn _ZN8QJSValue14deletePropertyERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QVariant QJSValue::toVariant();
  fn _ZNK8QJSValue9toVariantEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QJSEngine * QJSValue::engine();
  fn _ZNK8QJSValue6engineEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QJSValue::isQObject();
  fn _ZNK8QJSValue9isQObjectEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QJSValue::isNull();
  fn _ZNK8QJSValue6isNullEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QJSValue::isDate();
  fn _ZNK8QJSValue6isDateEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QJSValue::QJSValue(const QJSValue & other);
  fn _ZN8QJSValueC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  double QJSValue::toNumber();
  fn _ZNK8QJSValue8toNumberEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QJSValue::~QJSValue();
  fn _ZN8QJSValueD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QJSValue::strictlyEquals(const QJSValue & other);
  fn _ZNK8QJSValue14strictlyEqualsERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  bool QJSValue::equals(const QJSValue & other);
  fn _ZNK8QJSValue6equalsERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  bool QJSValue::toBool();
  fn _ZNK8QJSValue6toBoolEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  quint32 QJSValue::toUInt();
  fn _ZNK8QJSValue6toUIntEv(qthis: u64 /* *mut c_void*/) -> c_uint;
  // proto:  void QJSValue::QJSValue(int value);
  fn _ZN8QJSValueC2Ei(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QJSValue::QJSValue(const char * str);
  fn _ZN8QJSValueC2EPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_char);
  // proto:  void QJSValue::QJSValue(uint value);
  fn _ZN8QJSValueC2Ej(qthis: u64 /* *mut c_void*/, arg0: c_uint);
  // proto:  bool QJSValue::isRegExp();
  fn _ZNK8QJSValue8isRegExpEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QJSValue::QJSValue(bool value);
  fn _ZN8QJSValueC2Eb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QJSValue::QJSValue(double value);
  fn _ZN8QJSValueC2Ed(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  QString QJSValue::toString();
  fn _ZNK8QJSValue8toStringEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QJSValue)=4
#[derive(Default)]
pub struct QJSValue {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QJSValue {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QJSValue {
    return QJSValue{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QJSValue QJSValue::prototype();
impl /*struct*/ QJSValue {
  pub fn prototype<RetType, T: QJSValue_prototype<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.prototype(self);
    // return 1;
  }
}

pub trait QJSValue_prototype<RetType> {
  fn prototype(self , rsthis: & QJSValue) -> RetType;
}

  // proto:  QJSValue QJSValue::prototype();
impl<'a> /*trait*/ QJSValue_prototype<QJSValue> for () {
  fn prototype(self , rsthis: & QJSValue) -> QJSValue {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QJSValue9prototypeEv()};
    let mut ret = unsafe {_ZNK8QJSValue9prototypeEv(rsthis.qclsinst)};
    let mut ret1 = QJSValue::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QJSValue::setProperty(const QString & name, const QJSValue & value);
impl /*struct*/ QJSValue {
  pub fn setProperty<RetType, T: QJSValue_setProperty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setProperty(self);
    // return 1;
  }
}

pub trait QJSValue_setProperty<RetType> {
  fn setProperty(self , rsthis: & QJSValue) -> RetType;
}

  // proto:  void QJSValue::setProperty(const QString & name, const QJSValue & value);
impl<'a> /*trait*/ QJSValue_setProperty<()> for (&'a QString, &'a QJSValue) {
  fn setProperty(self , rsthis: & QJSValue) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QJSValue11setPropertyERK7QStringRKS_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QJSValue11setPropertyERK7QStringRKS_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QJSValue::setProperty(quint32 arrayIndex, const QJSValue & value);
impl<'a> /*trait*/ QJSValue_setProperty<()> for (u32, &'a QJSValue) {
  fn setProperty(self , rsthis: & QJSValue) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QJSValue11setPropertyEjRKS_()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QJSValue11setPropertyEjRKS_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QJSValue::isError();
impl /*struct*/ QJSValue {
  pub fn isError<RetType, T: QJSValue_isError<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isError(self);
    // return 1;
  }
}

pub trait QJSValue_isError<RetType> {
  fn isError(self , rsthis: & QJSValue) -> RetType;
}

  // proto:  bool QJSValue::isError();
impl<'a> /*trait*/ QJSValue_isError<i8> for () {
  fn isError(self , rsthis: & QJSValue) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QJSValue7isErrorEv()};
    let mut ret = unsafe {_ZNK8QJSValue7isErrorEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QJSValue::isBool();
impl /*struct*/ QJSValue {
  pub fn isBool<RetType, T: QJSValue_isBool<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isBool(self);
    // return 1;
  }
}

pub trait QJSValue_isBool<RetType> {
  fn isBool(self , rsthis: & QJSValue) -> RetType;
}

  // proto:  bool QJSValue::isBool();
impl<'a> /*trait*/ QJSValue_isBool<i8> for () {
  fn isBool(self , rsthis: & QJSValue) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QJSValue6isBoolEv()};
    let mut ret = unsafe {_ZNK8QJSValue6isBoolEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QJSValue::QJSValue(const QString & value);
impl /*struct*/ QJSValue {
  pub fn new<T: QJSValue_new>(value: T) -> QJSValue {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QJSValue_new {
  fn new(self) -> QJSValue;
}

  // proto:  void QJSValue::QJSValue(const QString & value);
impl<'a> /*trait*/ QJSValue_new for (&'a QString) {
  fn new(self) -> QJSValue {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QJSValueC2ERK7QString()};
    let ctysz: c_int = unsafe{QJSValue_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QJSValueC2ERK7QString(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QJSValue{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QDateTime QJSValue::toDateTime();
impl /*struct*/ QJSValue {
  pub fn toDateTime<RetType, T: QJSValue_toDateTime<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toDateTime(self);
    // return 1;
  }
}

pub trait QJSValue_toDateTime<RetType> {
  fn toDateTime(self , rsthis: & QJSValue) -> RetType;
}

  // proto:  QDateTime QJSValue::toDateTime();
impl<'a> /*trait*/ QJSValue_toDateTime<QDateTime> for () {
  fn toDateTime(self , rsthis: & QJSValue) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QJSValue10toDateTimeEv()};
    let mut ret = unsafe {_ZNK8QJSValue10toDateTimeEv(rsthis.qclsinst)};
    let mut ret1 = QDateTime::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QJSValue::isCallable();
impl /*struct*/ QJSValue {
  pub fn isCallable<RetType, T: QJSValue_isCallable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isCallable(self);
    // return 1;
  }
}

pub trait QJSValue_isCallable<RetType> {
  fn isCallable(self , rsthis: & QJSValue) -> RetType;
}

  // proto:  bool QJSValue::isCallable();
impl<'a> /*trait*/ QJSValue_isCallable<i8> for () {
  fn isCallable(self , rsthis: & QJSValue) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QJSValue10isCallableEv()};
    let mut ret = unsafe {_ZNK8QJSValue10isCallableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QJSValue::isString();
impl /*struct*/ QJSValue {
  pub fn isString<RetType, T: QJSValue_isString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isString(self);
    // return 1;
  }
}

pub trait QJSValue_isString<RetType> {
  fn isString(self , rsthis: & QJSValue) -> RetType;
}

  // proto:  bool QJSValue::isString();
impl<'a> /*trait*/ QJSValue_isString<i8> for () {
  fn isString(self , rsthis: & QJSValue) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QJSValue8isStringEv()};
    let mut ret = unsafe {_ZNK8QJSValue8isStringEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qint32 QJSValue::toInt();
impl /*struct*/ QJSValue {
  pub fn toInt<RetType, T: QJSValue_toInt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toInt(self);
    // return 1;
  }
}

pub trait QJSValue_toInt<RetType> {
  fn toInt(self , rsthis: & QJSValue) -> RetType;
}

  // proto:  qint32 QJSValue::toInt();
impl<'a> /*trait*/ QJSValue_toInt<i32> for () {
  fn toInt(self , rsthis: & QJSValue) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QJSValue5toIntEv()};
    let mut ret = unsafe {_ZNK8QJSValue5toIntEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QJSValue::isVariant();
impl /*struct*/ QJSValue {
  pub fn isVariant<RetType, T: QJSValue_isVariant<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isVariant(self);
    // return 1;
  }
}

pub trait QJSValue_isVariant<RetType> {
  fn isVariant(self , rsthis: & QJSValue) -> RetType;
}

  // proto:  bool QJSValue::isVariant();
impl<'a> /*trait*/ QJSValue_isVariant<i8> for () {
  fn isVariant(self , rsthis: & QJSValue) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QJSValue9isVariantEv()};
    let mut ret = unsafe {_ZNK8QJSValue9isVariantEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QJSValue::isUndefined();
impl /*struct*/ QJSValue {
  pub fn isUndefined<RetType, T: QJSValue_isUndefined<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isUndefined(self);
    // return 1;
  }
}

pub trait QJSValue_isUndefined<RetType> {
  fn isUndefined(self , rsthis: & QJSValue) -> RetType;
}

  // proto:  bool QJSValue::isUndefined();
impl<'a> /*trait*/ QJSValue_isUndefined<i8> for () {
  fn isUndefined(self , rsthis: & QJSValue) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QJSValue11isUndefinedEv()};
    let mut ret = unsafe {_ZNK8QJSValue11isUndefinedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QJSValue::setPrototype(const QJSValue & prototype);
impl /*struct*/ QJSValue {
  pub fn setPrototype<RetType, T: QJSValue_setPrototype<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPrototype(self);
    // return 1;
  }
}

pub trait QJSValue_setPrototype<RetType> {
  fn setPrototype(self , rsthis: & QJSValue) -> RetType;
}

  // proto:  void QJSValue::setPrototype(const QJSValue & prototype);
impl<'a> /*trait*/ QJSValue_setPrototype<()> for (&'a QJSValue) {
  fn setPrototype(self , rsthis: & QJSValue) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QJSValue12setPrototypeERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QJSValue12setPrototypeERKS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QJSValue::isArray();
impl /*struct*/ QJSValue {
  pub fn isArray<RetType, T: QJSValue_isArray<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isArray(self);
    // return 1;
  }
}

pub trait QJSValue_isArray<RetType> {
  fn isArray(self , rsthis: & QJSValue) -> RetType;
}

  // proto:  bool QJSValue::isArray();
impl<'a> /*trait*/ QJSValue_isArray<i8> for () {
  fn isArray(self , rsthis: & QJSValue) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QJSValue7isArrayEv()};
    let mut ret = unsafe {_ZNK8QJSValue7isArrayEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QJSValue QJSValue::property(const QString & name);
impl /*struct*/ QJSValue {
  pub fn property<RetType, T: QJSValue_property<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.property(self);
    // return 1;
  }
}

pub trait QJSValue_property<RetType> {
  fn property(self , rsthis: & QJSValue) -> RetType;
}

  // proto:  QJSValue QJSValue::property(const QString & name);
impl<'a> /*trait*/ QJSValue_property<QJSValue> for (&'a QString) {
  fn property(self , rsthis: & QJSValue) -> QJSValue {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QJSValue8propertyERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK8QJSValue8propertyERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QJSValue::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QJSValue::isObject();
impl /*struct*/ QJSValue {
  pub fn isObject<RetType, T: QJSValue_isObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isObject(self);
    // return 1;
  }
}

pub trait QJSValue_isObject<RetType> {
  fn isObject(self , rsthis: & QJSValue) -> RetType;
}

  // proto:  bool QJSValue::isObject();
impl<'a> /*trait*/ QJSValue_isObject<i8> for () {
  fn isObject(self , rsthis: & QJSValue) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QJSValue8isObjectEv()};
    let mut ret = unsafe {_ZNK8QJSValue8isObjectEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QJSValue QJSValue::property(quint32 arrayIndex);
impl<'a> /*trait*/ QJSValue_property<QJSValue> for (u32) {
  fn property(self , rsthis: & QJSValue) -> QJSValue {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QJSValue8propertyEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZNK8QJSValue8propertyEj(rsthis.qclsinst, arg0)};
    let mut ret1 = QJSValue::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QObject * QJSValue::toQObject();
impl /*struct*/ QJSValue {
  pub fn toQObject<RetType, T: QJSValue_toQObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toQObject(self);
    // return 1;
  }
}

pub trait QJSValue_toQObject<RetType> {
  fn toQObject(self , rsthis: & QJSValue) -> RetType;
}

  // proto:  QObject * QJSValue::toQObject();
impl<'a> /*trait*/ QJSValue_toQObject<QObject> for () {
  fn toQObject(self , rsthis: & QJSValue) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QJSValue9toQObjectEv()};
    let mut ret = unsafe {_ZNK8QJSValue9toQObjectEv(rsthis.qclsinst)};
    let mut ret1 = QObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QJSValue::QJSValue(void * );
impl<'a> /*trait*/ QJSValue_new for (*mut c_void) {
  fn new(self) -> QJSValue {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QJSValueC2EPv()};
    let ctysz: c_int = unsafe{QJSValue_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as *mut c_void;
    unsafe {_ZN8QJSValueC2EPv(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QJSValue{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QJSValue::isNumber();
impl /*struct*/ QJSValue {
  pub fn isNumber<RetType, T: QJSValue_isNumber<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNumber(self);
    // return 1;
  }
}

pub trait QJSValue_isNumber<RetType> {
  fn isNumber(self , rsthis: & QJSValue) -> RetType;
}

  // proto:  bool QJSValue::isNumber();
impl<'a> /*trait*/ QJSValue_isNumber<i8> for () {
  fn isNumber(self , rsthis: & QJSValue) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QJSValue8isNumberEv()};
    let mut ret = unsafe {_ZNK8QJSValue8isNumberEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QJSValue::hasOwnProperty(const QString & name);
impl /*struct*/ QJSValue {
  pub fn hasOwnProperty<RetType, T: QJSValue_hasOwnProperty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasOwnProperty(self);
    // return 1;
  }
}

pub trait QJSValue_hasOwnProperty<RetType> {
  fn hasOwnProperty(self , rsthis: & QJSValue) -> RetType;
}

  // proto:  bool QJSValue::hasOwnProperty(const QString & name);
impl<'a> /*trait*/ QJSValue_hasOwnProperty<i8> for (&'a QString) {
  fn hasOwnProperty(self , rsthis: & QJSValue) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QJSValue14hasOwnPropertyERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK8QJSValue14hasOwnPropertyERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QJSValue::hasProperty(const QString & name);
impl /*struct*/ QJSValue {
  pub fn hasProperty<RetType, T: QJSValue_hasProperty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasProperty(self);
    // return 1;
  }
}

pub trait QJSValue_hasProperty<RetType> {
  fn hasProperty(self , rsthis: & QJSValue) -> RetType;
}

  // proto:  bool QJSValue::hasProperty(const QString & name);
impl<'a> /*trait*/ QJSValue_hasProperty<i8> for (&'a QString) {
  fn hasProperty(self , rsthis: & QJSValue) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QJSValue11hasPropertyERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK8QJSValue11hasPropertyERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QJSValue::deleteProperty(const QString & name);
impl /*struct*/ QJSValue {
  pub fn deleteProperty<RetType, T: QJSValue_deleteProperty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.deleteProperty(self);
    // return 1;
  }
}

pub trait QJSValue_deleteProperty<RetType> {
  fn deleteProperty(self , rsthis: & QJSValue) -> RetType;
}

  // proto:  bool QJSValue::deleteProperty(const QString & name);
impl<'a> /*trait*/ QJSValue_deleteProperty<i8> for (&'a QString) {
  fn deleteProperty(self , rsthis: & QJSValue) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QJSValue14deletePropertyERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QJSValue14deletePropertyERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QVariant QJSValue::toVariant();
impl /*struct*/ QJSValue {
  pub fn toVariant<RetType, T: QJSValue_toVariant<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toVariant(self);
    // return 1;
  }
}

pub trait QJSValue_toVariant<RetType> {
  fn toVariant(self , rsthis: & QJSValue) -> RetType;
}

  // proto:  QVariant QJSValue::toVariant();
impl<'a> /*trait*/ QJSValue_toVariant<QVariant> for () {
  fn toVariant(self , rsthis: & QJSValue) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QJSValue9toVariantEv()};
    let mut ret = unsafe {_ZNK8QJSValue9toVariantEv(rsthis.qclsinst)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QJSEngine * QJSValue::engine();
impl /*struct*/ QJSValue {
  pub fn engine<RetType, T: QJSValue_engine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.engine(self);
    // return 1;
  }
}

pub trait QJSValue_engine<RetType> {
  fn engine(self , rsthis: & QJSValue) -> RetType;
}

  // proto:  QJSEngine * QJSValue::engine();
impl<'a> /*trait*/ QJSValue_engine<QJSEngine> for () {
  fn engine(self , rsthis: & QJSValue) -> QJSEngine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QJSValue6engineEv()};
    let mut ret = unsafe {_ZNK8QJSValue6engineEv(rsthis.qclsinst)};
    let mut ret1 = QJSEngine::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QJSValue::isQObject();
impl /*struct*/ QJSValue {
  pub fn isQObject<RetType, T: QJSValue_isQObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isQObject(self);
    // return 1;
  }
}

pub trait QJSValue_isQObject<RetType> {
  fn isQObject(self , rsthis: & QJSValue) -> RetType;
}

  // proto:  bool QJSValue::isQObject();
impl<'a> /*trait*/ QJSValue_isQObject<i8> for () {
  fn isQObject(self , rsthis: & QJSValue) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QJSValue9isQObjectEv()};
    let mut ret = unsafe {_ZNK8QJSValue9isQObjectEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QJSValue::isNull();
impl /*struct*/ QJSValue {
  pub fn isNull<RetType, T: QJSValue_isNull<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QJSValue_isNull<RetType> {
  fn isNull(self , rsthis: & QJSValue) -> RetType;
}

  // proto:  bool QJSValue::isNull();
impl<'a> /*trait*/ QJSValue_isNull<i8> for () {
  fn isNull(self , rsthis: & QJSValue) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QJSValue6isNullEv()};
    let mut ret = unsafe {_ZNK8QJSValue6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QJSValue::isDate();
impl /*struct*/ QJSValue {
  pub fn isDate<RetType, T: QJSValue_isDate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isDate(self);
    // return 1;
  }
}

pub trait QJSValue_isDate<RetType> {
  fn isDate(self , rsthis: & QJSValue) -> RetType;
}

  // proto:  bool QJSValue::isDate();
impl<'a> /*trait*/ QJSValue_isDate<i8> for () {
  fn isDate(self , rsthis: & QJSValue) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QJSValue6isDateEv()};
    let mut ret = unsafe {_ZNK8QJSValue6isDateEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QJSValue::QJSValue(const QJSValue & other);
impl<'a> /*trait*/ QJSValue_new for (&'a QJSValue) {
  fn new(self) -> QJSValue {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QJSValueC2ERKS_()};
    let ctysz: c_int = unsafe{QJSValue_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QJSValueC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QJSValue{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  double QJSValue::toNumber();
impl /*struct*/ QJSValue {
  pub fn toNumber<RetType, T: QJSValue_toNumber<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toNumber(self);
    // return 1;
  }
}

pub trait QJSValue_toNumber<RetType> {
  fn toNumber(self , rsthis: & QJSValue) -> RetType;
}

  // proto:  double QJSValue::toNumber();
impl<'a> /*trait*/ QJSValue_toNumber<f64> for () {
  fn toNumber(self , rsthis: & QJSValue) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QJSValue8toNumberEv()};
    let mut ret = unsafe {_ZNK8QJSValue8toNumberEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QJSValue::~QJSValue();
impl /*struct*/ QJSValue {
  pub fn free<RetType, T: QJSValue_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QJSValue_free<RetType> {
  fn free(self , rsthis: & QJSValue) -> RetType;
}

  // proto:  void QJSValue::~QJSValue();
impl<'a> /*trait*/ QJSValue_free<()> for () {
  fn free(self , rsthis: & QJSValue) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QJSValueD2Ev()};
     unsafe {_ZN8QJSValueD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QJSValue::strictlyEquals(const QJSValue & other);
impl /*struct*/ QJSValue {
  pub fn strictlyEquals<RetType, T: QJSValue_strictlyEquals<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.strictlyEquals(self);
    // return 1;
  }
}

pub trait QJSValue_strictlyEquals<RetType> {
  fn strictlyEquals(self , rsthis: & QJSValue) -> RetType;
}

  // proto:  bool QJSValue::strictlyEquals(const QJSValue & other);
impl<'a> /*trait*/ QJSValue_strictlyEquals<i8> for (&'a QJSValue) {
  fn strictlyEquals(self , rsthis: & QJSValue) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QJSValue14strictlyEqualsERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK8QJSValue14strictlyEqualsERKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QJSValue::equals(const QJSValue & other);
impl /*struct*/ QJSValue {
  pub fn equals<RetType, T: QJSValue_equals<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.equals(self);
    // return 1;
  }
}

pub trait QJSValue_equals<RetType> {
  fn equals(self , rsthis: & QJSValue) -> RetType;
}

  // proto:  bool QJSValue::equals(const QJSValue & other);
impl<'a> /*trait*/ QJSValue_equals<i8> for (&'a QJSValue) {
  fn equals(self , rsthis: & QJSValue) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QJSValue6equalsERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK8QJSValue6equalsERKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QJSValue::toBool();
impl /*struct*/ QJSValue {
  pub fn toBool<RetType, T: QJSValue_toBool<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toBool(self);
    // return 1;
  }
}

pub trait QJSValue_toBool<RetType> {
  fn toBool(self , rsthis: & QJSValue) -> RetType;
}

  // proto:  bool QJSValue::toBool();
impl<'a> /*trait*/ QJSValue_toBool<i8> for () {
  fn toBool(self , rsthis: & QJSValue) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QJSValue6toBoolEv()};
    let mut ret = unsafe {_ZNK8QJSValue6toBoolEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  quint32 QJSValue::toUInt();
impl /*struct*/ QJSValue {
  pub fn toUInt<RetType, T: QJSValue_toUInt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toUInt(self);
    // return 1;
  }
}

pub trait QJSValue_toUInt<RetType> {
  fn toUInt(self , rsthis: & QJSValue) -> RetType;
}

  // proto:  quint32 QJSValue::toUInt();
impl<'a> /*trait*/ QJSValue_toUInt<u32> for () {
  fn toUInt(self , rsthis: & QJSValue) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QJSValue6toUIntEv()};
    let mut ret = unsafe {_ZNK8QJSValue6toUIntEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

  // proto:  void QJSValue::QJSValue(int value);
impl<'a> /*trait*/ QJSValue_new for (i32) {
  fn new(self) -> QJSValue {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QJSValueC2Ei()};
    let ctysz: c_int = unsafe{QJSValue_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_int;
    unsafe {_ZN8QJSValueC2Ei(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QJSValue{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QJSValue::QJSValue(const char * str);
impl<'a> /*trait*/ QJSValue_new for (&'a  String) {
  fn new(self) -> QJSValue {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QJSValueC2EPKc()};
    let ctysz: c_int = unsafe{QJSValue_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.as_ptr()  as *mut c_char;
    unsafe {_ZN8QJSValueC2EPKc(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QJSValue{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QJSValue::QJSValue(uint value);
impl<'a> /*trait*/ QJSValue_new for (u32) {
  fn new(self) -> QJSValue {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QJSValueC2Ej()};
    let ctysz: c_int = unsafe{QJSValue_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_uint;
    unsafe {_ZN8QJSValueC2Ej(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QJSValue{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QJSValue::isRegExp();
impl /*struct*/ QJSValue {
  pub fn isRegExp<RetType, T: QJSValue_isRegExp<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isRegExp(self);
    // return 1;
  }
}

pub trait QJSValue_isRegExp<RetType> {
  fn isRegExp(self , rsthis: & QJSValue) -> RetType;
}

  // proto:  bool QJSValue::isRegExp();
impl<'a> /*trait*/ QJSValue_isRegExp<i8> for () {
  fn isRegExp(self , rsthis: & QJSValue) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QJSValue8isRegExpEv()};
    let mut ret = unsafe {_ZNK8QJSValue8isRegExpEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QJSValue::QJSValue(bool value);
impl<'a> /*trait*/ QJSValue_new for (i8) {
  fn new(self) -> QJSValue {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QJSValueC2Eb()};
    let ctysz: c_int = unsafe{QJSValue_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_char;
    unsafe {_ZN8QJSValueC2Eb(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QJSValue{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QJSValue::QJSValue(double value);
impl<'a> /*trait*/ QJSValue_new for (f64) {
  fn new(self) -> QJSValue {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QJSValueC2Ed()};
    let ctysz: c_int = unsafe{QJSValue_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_double;
    unsafe {_ZN8QJSValueC2Ed(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QJSValue{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QJSValue::toString();
impl /*struct*/ QJSValue {
  pub fn toString<RetType, T: QJSValue_toString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toString(self);
    // return 1;
  }
}

pub trait QJSValue_toString<RetType> {
  fn toString(self , rsthis: & QJSValue) -> RetType;
}

  // proto:  QString QJSValue::toString();
impl<'a> /*trait*/ QJSValue_toString<QString> for () {
  fn toString(self , rsthis: & QJSValue) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QJSValue8toStringEv()};
    let mut ret = unsafe {_ZNK8QJSValue8toStringEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

