// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
// src-file: /QtCore/qjsonvalue.h
// dst-file: /src/core/qjsonvalue.rs
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
use super::qjsonobject::*; // 773
use super::qstring::*; // 773
use super::qjsonarray::*; // 773
use super::qvariant::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QJsonValueRefPtr_Class_Size() -> c_int;
  fn QJsonValuePtr_Class_Size() -> c_int;
  fn QJsonValue_Class_Size() -> c_int;
  // proto:  QJsonObject QJsonValue::toObject();
  fn C_ZNK10QJsonValue8toObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QJsonValue::isDouble();
  fn C_ZNK10QJsonValue8isDoubleEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QJsonValue::QJsonValue(const QString & s);
  fn C_ZN10QJsonValueC2ERK7QString(arg0: *mut c_void) -> u64;
  // proto:  int QJsonValue::toInt(int defaultValue);
  fn C_ZNK10QJsonValue5toIntEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_int;
  // proto:  QJsonArray QJsonValue::toArray();
  fn C_ZNK10QJsonValue7toArrayEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QJsonValue::isArray();
  fn C_ZNK10QJsonValue7isArrayEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QJsonValue::QJsonValue(const char * s);
  fn C_ZN10QJsonValueC2EPKc(arg0: *mut c_char) -> u64;
  // proto:  QString QJsonValue::toString(const QString & defaultValue);
  fn C_ZNK10QJsonValue8toStringERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  double QJsonValue::toDouble(double defaultValue);
  fn C_ZNK10QJsonValue8toDoubleEd(qthis: u64 /* *mut c_void*/, arg0: c_double) -> c_double;
  // proto:  void QJsonValue::~QJsonValue();
  fn C_ZN10QJsonValueD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QVariant QJsonValue::toVariant();
  fn C_ZNK10QJsonValue9toVariantEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QJsonValue::isObject();
  fn C_ZNK10QJsonValue8isObjectEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto: static QJsonValue QJsonValue::fromVariant(const QVariant & variant);
  fn C_ZN10QJsonValue11fromVariantERK8QVariant(arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QJsonValue::toBool(bool defaultValue);
  fn C_ZNK10QJsonValue6toBoolEb(qthis: u64 /* *mut c_void*/, arg0: c_char) -> c_char;
  // proto:  void QJsonValue::QJsonValue(double n);
  fn C_ZN10QJsonValueC2Ed(arg0: c_double) -> u64;
  // proto:  bool QJsonValue::isBool();
  fn C_ZNK10QJsonValue6isBoolEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QJsonValue::QJsonValue(bool b);
  fn C_ZN10QJsonValueC2Eb(arg0: c_char) -> u64;
  // proto:  bool QJsonValue::isUndefined();
  fn C_ZNK10QJsonValue11isUndefinedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QJsonValue::isNull();
  fn C_ZNK10QJsonValue6isNullEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QJsonValue::isString();
  fn C_ZNK10QJsonValue8isStringEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QJsonValue::QJsonValue(int n);
  fn C_ZN10QJsonValueC2Ei(arg0: c_int) -> u64;
  // proto:  void QJsonValue::QJsonValue(qint64 n);
  fn C_ZN10QJsonValueC2Ex(arg0: c_longlong) -> u64;
  fn QJsonValueRef_Class_Size() -> c_int;
  // proto:  QJsonArray QJsonValueRef::toArray();
  fn C_ZNK13QJsonValueRef7toArrayEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QJsonObject QJsonValueRef::toObject();
  fn C_ZNK13QJsonValueRef8toObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QJsonValueRef::isBool();
  fn C_ZNK13QJsonValueRef6isBoolEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QJsonValueRef::isDouble();
  fn C_ZNK13QJsonValueRef8isDoubleEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  double QJsonValueRef::toDouble();
  fn C_ZNK13QJsonValueRef8toDoubleEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  bool QJsonValueRef::toBool(bool defaultValue);
  fn C_ZNK13QJsonValueRef6toBoolEb(qthis: u64 /* *mut c_void*/, arg0: c_char) -> c_char;
  // proto:  double QJsonValueRef::toDouble(double defaultValue);
  fn C_ZNK13QJsonValueRef8toDoubleEd(qthis: u64 /* *mut c_void*/, arg0: c_double) -> c_double;
  // proto:  bool QJsonValueRef::toBool();
  fn C_ZNK13QJsonValueRef6toBoolEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QVariant QJsonValueRef::toVariant();
  fn C_ZNK13QJsonValueRef9toVariantEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QJsonValueRef::toString(const QString & defaultValue);
  fn C_ZNK13QJsonValueRef8toStringERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QJsonValueRef::isObject();
  fn C_ZNK13QJsonValueRef8isObjectEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QJsonValueRef::isString();
  fn C_ZNK13QJsonValueRef8isStringEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QString QJsonValueRef::toString();
  fn C_ZNK13QJsonValueRef8toStringEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QJsonValueRef::toInt(int defaultValue);
  fn C_ZNK13QJsonValueRef5toIntEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_int;
  // proto:  bool QJsonValueRef::isArray();
  fn C_ZNK13QJsonValueRef7isArrayEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QJsonValueRef::isNull();
  fn C_ZNK13QJsonValueRef6isNullEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QJsonValueRef::toInt();
  fn C_ZNK13QJsonValueRef5toIntEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QJsonValueRef::isUndefined();
  fn C_ZNK13QJsonValueRef11isUndefinedEv(qthis: u64 /* *mut c_void*/) -> c_char;
} // <= ext block end

// body block begin =>
// class sizeof(QJsonValueRefPtr)=16
#[derive(Default)]
pub struct QJsonValueRefPtr {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QJsonValuePtr)=24
#[derive(Default)]
pub struct QJsonValuePtr {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QJsonValue)=24
#[derive(Default)]
pub struct QJsonValue {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QJsonValueRef)=16
#[derive(Default)]
pub struct QJsonValueRef {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QJsonValueRefPtr {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QJsonValueRefPtr {
    return QJsonValueRefPtr{qclsinst: qthis, ..Default::default()};
  }
}
impl /*struct*/ QJsonValuePtr {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QJsonValuePtr {
    return QJsonValuePtr{qclsinst: qthis, ..Default::default()};
  }
}
impl /*struct*/ QJsonValue {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QJsonValue {
    return QJsonValue{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QJsonObject QJsonValue::toObject();
impl /*struct*/ QJsonValue {
  pub fn toObject<RetType, T: QJsonValue_toObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toObject(self);
    // return 1;
  }
}

pub trait QJsonValue_toObject<RetType> {
  fn toObject(self , rsthis: & QJsonValue) -> RetType;
}

  // proto:  QJsonObject QJsonValue::toObject();
impl<'a> /*trait*/ QJsonValue_toObject<QJsonObject> for () {
  fn toObject(self , rsthis: & QJsonValue) -> QJsonObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QJsonValue8toObjectEv()};
    let mut ret = unsafe {C_ZNK10QJsonValue8toObjectEv(rsthis.qclsinst)};
    let mut ret1 = QJsonObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QJsonValue::isDouble();
impl /*struct*/ QJsonValue {
  pub fn isDouble<RetType, T: QJsonValue_isDouble<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isDouble(self);
    // return 1;
  }
}

pub trait QJsonValue_isDouble<RetType> {
  fn isDouble(self , rsthis: & QJsonValue) -> RetType;
}

  // proto:  bool QJsonValue::isDouble();
impl<'a> /*trait*/ QJsonValue_isDouble<i8> for () {
  fn isDouble(self , rsthis: & QJsonValue) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QJsonValue8isDoubleEv()};
    let mut ret = unsafe {C_ZNK10QJsonValue8isDoubleEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QJsonValue::QJsonValue(const QString & s);
impl /*struct*/ QJsonValue {
  pub fn new<T: QJsonValue_new>(value: T) -> QJsonValue {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QJsonValue_new {
  fn new(self) -> QJsonValue;
}

  // proto:  void QJsonValue::QJsonValue(const QString & s);
impl<'a> /*trait*/ QJsonValue_new for (&'a QString) {
  fn new(self) -> QJsonValue {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QJsonValueC2ERK7QString()};
    let ctysz: c_int = unsafe{QJsonValue_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN10QJsonValueC2ERK7QString(arg0)};
    let rsthis = QJsonValue{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QJsonValue::toInt(int defaultValue);
impl /*struct*/ QJsonValue {
  pub fn toInt<RetType, T: QJsonValue_toInt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toInt(self);
    // return 1;
  }
}

pub trait QJsonValue_toInt<RetType> {
  fn toInt(self , rsthis: & QJsonValue) -> RetType;
}

  // proto:  int QJsonValue::toInt(int defaultValue);
impl<'a> /*trait*/ QJsonValue_toInt<i32> for (i32) {
  fn toInt(self , rsthis: & QJsonValue) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QJsonValue5toIntEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK10QJsonValue5toIntEi(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QJsonArray QJsonValue::toArray();
impl /*struct*/ QJsonValue {
  pub fn toArray<RetType, T: QJsonValue_toArray<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toArray(self);
    // return 1;
  }
}

pub trait QJsonValue_toArray<RetType> {
  fn toArray(self , rsthis: & QJsonValue) -> RetType;
}

  // proto:  QJsonArray QJsonValue::toArray();
impl<'a> /*trait*/ QJsonValue_toArray<QJsonArray> for () {
  fn toArray(self , rsthis: & QJsonValue) -> QJsonArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QJsonValue7toArrayEv()};
    let mut ret = unsafe {C_ZNK10QJsonValue7toArrayEv(rsthis.qclsinst)};
    let mut ret1 = QJsonArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QJsonValue::isArray();
impl /*struct*/ QJsonValue {
  pub fn isArray<RetType, T: QJsonValue_isArray<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isArray(self);
    // return 1;
  }
}

pub trait QJsonValue_isArray<RetType> {
  fn isArray(self , rsthis: & QJsonValue) -> RetType;
}

  // proto:  bool QJsonValue::isArray();
impl<'a> /*trait*/ QJsonValue_isArray<i8> for () {
  fn isArray(self , rsthis: & QJsonValue) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QJsonValue7isArrayEv()};
    let mut ret = unsafe {C_ZNK10QJsonValue7isArrayEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QJsonValue::QJsonValue(const char * s);
impl<'a> /*trait*/ QJsonValue_new for (&'a  String) {
  fn new(self) -> QJsonValue {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QJsonValueC2EPKc()};
    let ctysz: c_int = unsafe{QJsonValue_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.as_ptr()  as *mut c_char;
    let qthis: u64 = unsafe {C_ZN10QJsonValueC2EPKc(arg0)};
    let rsthis = QJsonValue{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QJsonValue::toString(const QString & defaultValue);
impl /*struct*/ QJsonValue {
  pub fn toString<RetType, T: QJsonValue_toString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toString(self);
    // return 1;
  }
}

pub trait QJsonValue_toString<RetType> {
  fn toString(self , rsthis: & QJsonValue) -> RetType;
}

  // proto:  QString QJsonValue::toString(const QString & defaultValue);
impl<'a> /*trait*/ QJsonValue_toString<QString> for (&'a QString) {
  fn toString(self , rsthis: & QJsonValue) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QJsonValue8toStringERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK10QJsonValue8toStringERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  double QJsonValue::toDouble(double defaultValue);
impl /*struct*/ QJsonValue {
  pub fn toDouble<RetType, T: QJsonValue_toDouble<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toDouble(self);
    // return 1;
  }
}

pub trait QJsonValue_toDouble<RetType> {
  fn toDouble(self , rsthis: & QJsonValue) -> RetType;
}

  // proto:  double QJsonValue::toDouble(double defaultValue);
impl<'a> /*trait*/ QJsonValue_toDouble<f64> for (f64) {
  fn toDouble(self , rsthis: & QJsonValue) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QJsonValue8toDoubleEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {C_ZNK10QJsonValue8toDoubleEd(rsthis.qclsinst, arg0)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  void QJsonValue::~QJsonValue();
impl /*struct*/ QJsonValue {
  pub fn free<RetType, T: QJsonValue_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QJsonValue_free<RetType> {
  fn free(self , rsthis: & QJsonValue) -> RetType;
}

  // proto:  void QJsonValue::~QJsonValue();
impl<'a> /*trait*/ QJsonValue_free<()> for () {
  fn free(self , rsthis: & QJsonValue) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QJsonValueD2Ev()};
     unsafe {C_ZN10QJsonValueD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QVariant QJsonValue::toVariant();
impl /*struct*/ QJsonValue {
  pub fn toVariant<RetType, T: QJsonValue_toVariant<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toVariant(self);
    // return 1;
  }
}

pub trait QJsonValue_toVariant<RetType> {
  fn toVariant(self , rsthis: & QJsonValue) -> RetType;
}

  // proto:  QVariant QJsonValue::toVariant();
impl<'a> /*trait*/ QJsonValue_toVariant<QVariant> for () {
  fn toVariant(self , rsthis: & QJsonValue) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QJsonValue9toVariantEv()};
    let mut ret = unsafe {C_ZNK10QJsonValue9toVariantEv(rsthis.qclsinst)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QJsonValue::isObject();
impl /*struct*/ QJsonValue {
  pub fn isObject<RetType, T: QJsonValue_isObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isObject(self);
    // return 1;
  }
}

pub trait QJsonValue_isObject<RetType> {
  fn isObject(self , rsthis: & QJsonValue) -> RetType;
}

  // proto:  bool QJsonValue::isObject();
impl<'a> /*trait*/ QJsonValue_isObject<i8> for () {
  fn isObject(self , rsthis: & QJsonValue) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QJsonValue8isObjectEv()};
    let mut ret = unsafe {C_ZNK10QJsonValue8isObjectEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto: static QJsonValue QJsonValue::fromVariant(const QVariant & variant);
impl /*struct*/ QJsonValue {
  pub fn fromVariant_s<RetType, T: QJsonValue_fromVariant_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromVariant_s();
    // return 1;
  }
}

pub trait QJsonValue_fromVariant_s<RetType> {
  fn fromVariant_s(self ) -> RetType;
}

  // proto: static QJsonValue QJsonValue::fromVariant(const QVariant & variant);
impl<'a> /*trait*/ QJsonValue_fromVariant_s<QJsonValue> for (&'a QVariant) {
  fn fromVariant_s(self ) -> QJsonValue {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QJsonValue11fromVariantERK8QVariant()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN10QJsonValue11fromVariantERK8QVariant(arg0)};
    let mut ret1 = QJsonValue::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QJsonValue::toBool(bool defaultValue);
impl /*struct*/ QJsonValue {
  pub fn toBool<RetType, T: QJsonValue_toBool<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toBool(self);
    // return 1;
  }
}

pub trait QJsonValue_toBool<RetType> {
  fn toBool(self , rsthis: & QJsonValue) -> RetType;
}

  // proto:  bool QJsonValue::toBool(bool defaultValue);
impl<'a> /*trait*/ QJsonValue_toBool<i8> for (i8) {
  fn toBool(self , rsthis: & QJsonValue) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QJsonValue6toBoolEb()};
    let arg0 = self  as c_char;
    let mut ret = unsafe {C_ZNK10QJsonValue6toBoolEb(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QJsonValue::QJsonValue(double n);
impl<'a> /*trait*/ QJsonValue_new for (f64) {
  fn new(self) -> QJsonValue {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QJsonValueC2Ed()};
    let ctysz: c_int = unsafe{QJsonValue_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_double;
    let qthis: u64 = unsafe {C_ZN10QJsonValueC2Ed(arg0)};
    let rsthis = QJsonValue{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QJsonValue::isBool();
impl /*struct*/ QJsonValue {
  pub fn isBool<RetType, T: QJsonValue_isBool<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isBool(self);
    // return 1;
  }
}

pub trait QJsonValue_isBool<RetType> {
  fn isBool(self , rsthis: & QJsonValue) -> RetType;
}

  // proto:  bool QJsonValue::isBool();
impl<'a> /*trait*/ QJsonValue_isBool<i8> for () {
  fn isBool(self , rsthis: & QJsonValue) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QJsonValue6isBoolEv()};
    let mut ret = unsafe {C_ZNK10QJsonValue6isBoolEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QJsonValue::QJsonValue(bool b);
impl<'a> /*trait*/ QJsonValue_new for (i8) {
  fn new(self) -> QJsonValue {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QJsonValueC2Eb()};
    let ctysz: c_int = unsafe{QJsonValue_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_char;
    let qthis: u64 = unsafe {C_ZN10QJsonValueC2Eb(arg0)};
    let rsthis = QJsonValue{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QJsonValue::isUndefined();
impl /*struct*/ QJsonValue {
  pub fn isUndefined<RetType, T: QJsonValue_isUndefined<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isUndefined(self);
    // return 1;
  }
}

pub trait QJsonValue_isUndefined<RetType> {
  fn isUndefined(self , rsthis: & QJsonValue) -> RetType;
}

  // proto:  bool QJsonValue::isUndefined();
impl<'a> /*trait*/ QJsonValue_isUndefined<i8> for () {
  fn isUndefined(self , rsthis: & QJsonValue) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QJsonValue11isUndefinedEv()};
    let mut ret = unsafe {C_ZNK10QJsonValue11isUndefinedEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QJsonValue::isNull();
impl /*struct*/ QJsonValue {
  pub fn isNull<RetType, T: QJsonValue_isNull<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QJsonValue_isNull<RetType> {
  fn isNull(self , rsthis: & QJsonValue) -> RetType;
}

  // proto:  bool QJsonValue::isNull();
impl<'a> /*trait*/ QJsonValue_isNull<i8> for () {
  fn isNull(self , rsthis: & QJsonValue) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QJsonValue6isNullEv()};
    let mut ret = unsafe {C_ZNK10QJsonValue6isNullEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QJsonValue::isString();
impl /*struct*/ QJsonValue {
  pub fn isString<RetType, T: QJsonValue_isString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isString(self);
    // return 1;
  }
}

pub trait QJsonValue_isString<RetType> {
  fn isString(self , rsthis: & QJsonValue) -> RetType;
}

  // proto:  bool QJsonValue::isString();
impl<'a> /*trait*/ QJsonValue_isString<i8> for () {
  fn isString(self , rsthis: & QJsonValue) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QJsonValue8isStringEv()};
    let mut ret = unsafe {C_ZNK10QJsonValue8isStringEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QJsonValue::QJsonValue(int n);
impl<'a> /*trait*/ QJsonValue_new for (i32) {
  fn new(self) -> QJsonValue {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QJsonValueC2Ei()};
    let ctysz: c_int = unsafe{QJsonValue_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_int;
    let qthis: u64 = unsafe {C_ZN10QJsonValueC2Ei(arg0)};
    let rsthis = QJsonValue{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QJsonValue::QJsonValue(qint64 n);
impl<'a> /*trait*/ QJsonValue_new for (i64) {
  fn new(self) -> QJsonValue {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QJsonValueC2Ex()};
    let ctysz: c_int = unsafe{QJsonValue_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_longlong;
    let qthis: u64 = unsafe {C_ZN10QJsonValueC2Ex(arg0)};
    let rsthis = QJsonValue{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QJsonValueRef {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QJsonValueRef {
    return QJsonValueRef{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QJsonArray QJsonValueRef::toArray();
impl /*struct*/ QJsonValueRef {
  pub fn toArray<RetType, T: QJsonValueRef_toArray<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toArray(self);
    // return 1;
  }
}

pub trait QJsonValueRef_toArray<RetType> {
  fn toArray(self , rsthis: & QJsonValueRef) -> RetType;
}

  // proto:  QJsonArray QJsonValueRef::toArray();
impl<'a> /*trait*/ QJsonValueRef_toArray<QJsonArray> for () {
  fn toArray(self , rsthis: & QJsonValueRef) -> QJsonArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QJsonValueRef7toArrayEv()};
    let mut ret = unsafe {C_ZNK13QJsonValueRef7toArrayEv(rsthis.qclsinst)};
    let mut ret1 = QJsonArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QJsonObject QJsonValueRef::toObject();
impl /*struct*/ QJsonValueRef {
  pub fn toObject<RetType, T: QJsonValueRef_toObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toObject(self);
    // return 1;
  }
}

pub trait QJsonValueRef_toObject<RetType> {
  fn toObject(self , rsthis: & QJsonValueRef) -> RetType;
}

  // proto:  QJsonObject QJsonValueRef::toObject();
impl<'a> /*trait*/ QJsonValueRef_toObject<QJsonObject> for () {
  fn toObject(self , rsthis: & QJsonValueRef) -> QJsonObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QJsonValueRef8toObjectEv()};
    let mut ret = unsafe {C_ZNK13QJsonValueRef8toObjectEv(rsthis.qclsinst)};
    let mut ret1 = QJsonObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QJsonValueRef::isBool();
impl /*struct*/ QJsonValueRef {
  pub fn isBool<RetType, T: QJsonValueRef_isBool<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isBool(self);
    // return 1;
  }
}

pub trait QJsonValueRef_isBool<RetType> {
  fn isBool(self , rsthis: & QJsonValueRef) -> RetType;
}

  // proto:  bool QJsonValueRef::isBool();
impl<'a> /*trait*/ QJsonValueRef_isBool<i8> for () {
  fn isBool(self , rsthis: & QJsonValueRef) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QJsonValueRef6isBoolEv()};
    let mut ret = unsafe {C_ZNK13QJsonValueRef6isBoolEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QJsonValueRef::isDouble();
impl /*struct*/ QJsonValueRef {
  pub fn isDouble<RetType, T: QJsonValueRef_isDouble<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isDouble(self);
    // return 1;
  }
}

pub trait QJsonValueRef_isDouble<RetType> {
  fn isDouble(self , rsthis: & QJsonValueRef) -> RetType;
}

  // proto:  bool QJsonValueRef::isDouble();
impl<'a> /*trait*/ QJsonValueRef_isDouble<i8> for () {
  fn isDouble(self , rsthis: & QJsonValueRef) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QJsonValueRef8isDoubleEv()};
    let mut ret = unsafe {C_ZNK13QJsonValueRef8isDoubleEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  double QJsonValueRef::toDouble();
impl /*struct*/ QJsonValueRef {
  pub fn toDouble<RetType, T: QJsonValueRef_toDouble<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toDouble(self);
    // return 1;
  }
}

pub trait QJsonValueRef_toDouble<RetType> {
  fn toDouble(self , rsthis: & QJsonValueRef) -> RetType;
}

  // proto:  double QJsonValueRef::toDouble();
impl<'a> /*trait*/ QJsonValueRef_toDouble<f64> for () {
  fn toDouble(self , rsthis: & QJsonValueRef) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QJsonValueRef8toDoubleEv()};
    let mut ret = unsafe {C_ZNK13QJsonValueRef8toDoubleEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  bool QJsonValueRef::toBool(bool defaultValue);
impl /*struct*/ QJsonValueRef {
  pub fn toBool<RetType, T: QJsonValueRef_toBool<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toBool(self);
    // return 1;
  }
}

pub trait QJsonValueRef_toBool<RetType> {
  fn toBool(self , rsthis: & QJsonValueRef) -> RetType;
}

  // proto:  bool QJsonValueRef::toBool(bool defaultValue);
impl<'a> /*trait*/ QJsonValueRef_toBool<i8> for (i8) {
  fn toBool(self , rsthis: & QJsonValueRef) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QJsonValueRef6toBoolEb()};
    let arg0 = self  as c_char;
    let mut ret = unsafe {C_ZNK13QJsonValueRef6toBoolEb(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  double QJsonValueRef::toDouble(double defaultValue);
impl<'a> /*trait*/ QJsonValueRef_toDouble<f64> for (f64) {
  fn toDouble(self , rsthis: & QJsonValueRef) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QJsonValueRef8toDoubleEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {C_ZNK13QJsonValueRef8toDoubleEd(rsthis.qclsinst, arg0)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  bool QJsonValueRef::toBool();
impl<'a> /*trait*/ QJsonValueRef_toBool<i8> for () {
  fn toBool(self , rsthis: & QJsonValueRef) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QJsonValueRef6toBoolEv()};
    let mut ret = unsafe {C_ZNK13QJsonValueRef6toBoolEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QVariant QJsonValueRef::toVariant();
impl /*struct*/ QJsonValueRef {
  pub fn toVariant<RetType, T: QJsonValueRef_toVariant<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toVariant(self);
    // return 1;
  }
}

pub trait QJsonValueRef_toVariant<RetType> {
  fn toVariant(self , rsthis: & QJsonValueRef) -> RetType;
}

  // proto:  QVariant QJsonValueRef::toVariant();
impl<'a> /*trait*/ QJsonValueRef_toVariant<QVariant> for () {
  fn toVariant(self , rsthis: & QJsonValueRef) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QJsonValueRef9toVariantEv()};
    let mut ret = unsafe {C_ZNK13QJsonValueRef9toVariantEv(rsthis.qclsinst)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QJsonValueRef::toString(const QString & defaultValue);
impl /*struct*/ QJsonValueRef {
  pub fn toString<RetType, T: QJsonValueRef_toString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toString(self);
    // return 1;
  }
}

pub trait QJsonValueRef_toString<RetType> {
  fn toString(self , rsthis: & QJsonValueRef) -> RetType;
}

  // proto:  QString QJsonValueRef::toString(const QString & defaultValue);
impl<'a> /*trait*/ QJsonValueRef_toString<QString> for (&'a QString) {
  fn toString(self , rsthis: & QJsonValueRef) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QJsonValueRef8toStringERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QJsonValueRef8toStringERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QJsonValueRef::isObject();
impl /*struct*/ QJsonValueRef {
  pub fn isObject<RetType, T: QJsonValueRef_isObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isObject(self);
    // return 1;
  }
}

pub trait QJsonValueRef_isObject<RetType> {
  fn isObject(self , rsthis: & QJsonValueRef) -> RetType;
}

  // proto:  bool QJsonValueRef::isObject();
impl<'a> /*trait*/ QJsonValueRef_isObject<i8> for () {
  fn isObject(self , rsthis: & QJsonValueRef) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QJsonValueRef8isObjectEv()};
    let mut ret = unsafe {C_ZNK13QJsonValueRef8isObjectEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QJsonValueRef::isString();
impl /*struct*/ QJsonValueRef {
  pub fn isString<RetType, T: QJsonValueRef_isString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isString(self);
    // return 1;
  }
}

pub trait QJsonValueRef_isString<RetType> {
  fn isString(self , rsthis: & QJsonValueRef) -> RetType;
}

  // proto:  bool QJsonValueRef::isString();
impl<'a> /*trait*/ QJsonValueRef_isString<i8> for () {
  fn isString(self , rsthis: & QJsonValueRef) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QJsonValueRef8isStringEv()};
    let mut ret = unsafe {C_ZNK13QJsonValueRef8isStringEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QString QJsonValueRef::toString();
impl<'a> /*trait*/ QJsonValueRef_toString<QString> for () {
  fn toString(self , rsthis: & QJsonValueRef) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QJsonValueRef8toStringEv()};
    let mut ret = unsafe {C_ZNK13QJsonValueRef8toStringEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QJsonValueRef::toInt(int defaultValue);
impl /*struct*/ QJsonValueRef {
  pub fn toInt<RetType, T: QJsonValueRef_toInt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toInt(self);
    // return 1;
  }
}

pub trait QJsonValueRef_toInt<RetType> {
  fn toInt(self , rsthis: & QJsonValueRef) -> RetType;
}

  // proto:  int QJsonValueRef::toInt(int defaultValue);
impl<'a> /*trait*/ QJsonValueRef_toInt<i32> for (i32) {
  fn toInt(self , rsthis: & QJsonValueRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QJsonValueRef5toIntEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK13QJsonValueRef5toIntEi(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  bool QJsonValueRef::isArray();
impl /*struct*/ QJsonValueRef {
  pub fn isArray<RetType, T: QJsonValueRef_isArray<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isArray(self);
    // return 1;
  }
}

pub trait QJsonValueRef_isArray<RetType> {
  fn isArray(self , rsthis: & QJsonValueRef) -> RetType;
}

  // proto:  bool QJsonValueRef::isArray();
impl<'a> /*trait*/ QJsonValueRef_isArray<i8> for () {
  fn isArray(self , rsthis: & QJsonValueRef) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QJsonValueRef7isArrayEv()};
    let mut ret = unsafe {C_ZNK13QJsonValueRef7isArrayEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QJsonValueRef::isNull();
impl /*struct*/ QJsonValueRef {
  pub fn isNull<RetType, T: QJsonValueRef_isNull<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QJsonValueRef_isNull<RetType> {
  fn isNull(self , rsthis: & QJsonValueRef) -> RetType;
}

  // proto:  bool QJsonValueRef::isNull();
impl<'a> /*trait*/ QJsonValueRef_isNull<i8> for () {
  fn isNull(self , rsthis: & QJsonValueRef) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QJsonValueRef6isNullEv()};
    let mut ret = unsafe {C_ZNK13QJsonValueRef6isNullEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  int QJsonValueRef::toInt();
impl<'a> /*trait*/ QJsonValueRef_toInt<i32> for () {
  fn toInt(self , rsthis: & QJsonValueRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QJsonValueRef5toIntEv()};
    let mut ret = unsafe {C_ZNK13QJsonValueRef5toIntEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  bool QJsonValueRef::isUndefined();
impl /*struct*/ QJsonValueRef {
  pub fn isUndefined<RetType, T: QJsonValueRef_isUndefined<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isUndefined(self);
    // return 1;
  }
}

pub trait QJsonValueRef_isUndefined<RetType> {
  fn isUndefined(self , rsthis: & QJsonValueRef) -> RetType;
}

  // proto:  bool QJsonValueRef::isUndefined();
impl<'a> /*trait*/ QJsonValueRef_isUndefined<i8> for () {
  fn isUndefined(self , rsthis: & QJsonValueRef) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QJsonValueRef11isUndefinedEv()};
    let mut ret = unsafe {C_ZNK13QJsonValueRef11isUndefinedEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

// <= body block end

