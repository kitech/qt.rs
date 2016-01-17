// auto generated, do not modify.
// created: Sun Jan 17 17:37:11 2016
// src-file: /QtCore/qjsondocument.h
// dst-file: /src/core/qjsondocument.rs
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
use super::qvariant::QVariant; // 773
use super::qbytearray::QByteArray; // 773
use super::qstring::QString; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QJsonDocument_Class_Size() -> c_int;
  // proto:  QJsonObject QJsonDocument::object();
  fn _ZNK13QJsonDocument6objectEv(qthis: u64 /* *mut c_void*/);
  // proto: static QJsonDocument QJsonDocument::fromVariant(const QVariant & variant);
  fn _ZN13QJsonDocument11fromVariantERK8QVariant(arg0: *mut c_void);
  // proto:  QJsonArray QJsonDocument::array();
  fn _ZNK13QJsonDocument5arrayEv(qthis: u64 /* *mut c_void*/);
  // proto:  QByteArray QJsonDocument::toJson();
  fn _ZNK13QJsonDocument6toJsonEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QJsonDocument::isNull();
  fn _ZNK13QJsonDocument6isNullEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QJsonDocument::QJsonDocument();
  fn _ZN13QJsonDocumentC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QVariant QJsonDocument::toVariant();
  fn _ZNK13QJsonDocument9toVariantEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QJsonDocument::isEmpty();
  fn _ZNK13QJsonDocument7isEmptyEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  const char * QJsonDocument::rawData(int * size);
  fn _ZNK13QJsonDocument7rawDataEPi(qthis: u64 /* *mut c_void*/, arg0: *mut c_int) -> *mut c_char;
  // proto:  bool QJsonDocument::isObject();
  fn _ZNK13QJsonDocument8isObjectEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QJsonDocument::~QJsonDocument();
  fn _ZN13QJsonDocumentD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QJsonDocument::isArray();
  fn _ZNK13QJsonDocument7isArrayEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QByteArray QJsonDocument::toBinaryData();
  fn _ZNK13QJsonDocument12toBinaryDataEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QJsonParseError_Class_Size() -> c_int;
  // proto:  QString QJsonParseError::errorString();
  fn _ZNK15QJsonParseError11errorStringEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QJsonDocument)=8
#[derive(Default)]
pub struct QJsonDocument {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QJsonParseError)=8
#[derive(Default)]
pub struct QJsonParseError {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QJsonDocument {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QJsonDocument {
    return QJsonDocument{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QJsonObject QJsonDocument::object();
impl /*struct*/ QJsonDocument {
  pub fn object<RetType, T: QJsonDocument_object<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.object(self);
    // return 1;
  }
}

pub trait QJsonDocument_object<RetType> {
  fn object(self , rsthis: & QJsonDocument) -> RetType;
}

  // proto:  QJsonObject QJsonDocument::object();
impl<'a> /*trait*/ QJsonDocument_object<()> for () {
  fn object(self , rsthis: & QJsonDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QJsonDocument6objectEv()};
     unsafe {_ZNK13QJsonDocument6objectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static QJsonDocument QJsonDocument::fromVariant(const QVariant & variant);
impl /*struct*/ QJsonDocument {
  pub fn fromVariant_s<RetType, T: QJsonDocument_fromVariant_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromVariant_s();
    // return 1;
  }
}

pub trait QJsonDocument_fromVariant_s<RetType> {
  fn fromVariant_s(self ) -> RetType;
}

  // proto: static QJsonDocument QJsonDocument::fromVariant(const QVariant & variant);
impl<'a> /*trait*/ QJsonDocument_fromVariant_s<()> for (&'a QVariant) {
  fn fromVariant_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QJsonDocument11fromVariantERK8QVariant()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QJsonDocument11fromVariantERK8QVariant(arg0)};
    // return 1;
  }
}

  // proto:  QJsonArray QJsonDocument::array();
impl /*struct*/ QJsonDocument {
  pub fn array<RetType, T: QJsonDocument_array<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.array(self);
    // return 1;
  }
}

pub trait QJsonDocument_array<RetType> {
  fn array(self , rsthis: & QJsonDocument) -> RetType;
}

  // proto:  QJsonArray QJsonDocument::array();
impl<'a> /*trait*/ QJsonDocument_array<()> for () {
  fn array(self , rsthis: & QJsonDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QJsonDocument5arrayEv()};
     unsafe {_ZNK13QJsonDocument5arrayEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QByteArray QJsonDocument::toJson();
impl /*struct*/ QJsonDocument {
  pub fn toJson<RetType, T: QJsonDocument_toJson<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toJson(self);
    // return 1;
  }
}

pub trait QJsonDocument_toJson<RetType> {
  fn toJson(self , rsthis: & QJsonDocument) -> RetType;
}

  // proto:  QByteArray QJsonDocument::toJson();
impl<'a> /*trait*/ QJsonDocument_toJson<QByteArray> for () {
  fn toJson(self , rsthis: & QJsonDocument) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QJsonDocument6toJsonEv()};
    let mut ret = unsafe {_ZNK13QJsonDocument6toJsonEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QJsonDocument::isNull();
impl /*struct*/ QJsonDocument {
  pub fn isNull<RetType, T: QJsonDocument_isNull<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QJsonDocument_isNull<RetType> {
  fn isNull(self , rsthis: & QJsonDocument) -> RetType;
}

  // proto:  bool QJsonDocument::isNull();
impl<'a> /*trait*/ QJsonDocument_isNull<i8> for () {
  fn isNull(self , rsthis: & QJsonDocument) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QJsonDocument6isNullEv()};
    let mut ret = unsafe {_ZNK13QJsonDocument6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QJsonDocument::QJsonDocument();
impl /*struct*/ QJsonDocument {
  pub fn new<T: QJsonDocument_new>(value: T) -> QJsonDocument {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QJsonDocument_new {
  fn new(self) -> QJsonDocument;
}

  // proto:  void QJsonDocument::QJsonDocument();
impl<'a> /*trait*/ QJsonDocument_new for () {
  fn new(self) -> QJsonDocument {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QJsonDocumentC2Ev()};
    let ctysz: c_int = unsafe{QJsonDocument_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN13QJsonDocumentC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QJsonDocument{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QVariant QJsonDocument::toVariant();
impl /*struct*/ QJsonDocument {
  pub fn toVariant<RetType, T: QJsonDocument_toVariant<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toVariant(self);
    // return 1;
  }
}

pub trait QJsonDocument_toVariant<RetType> {
  fn toVariant(self , rsthis: & QJsonDocument) -> RetType;
}

  // proto:  QVariant QJsonDocument::toVariant();
impl<'a> /*trait*/ QJsonDocument_toVariant<QVariant> for () {
  fn toVariant(self , rsthis: & QJsonDocument) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QJsonDocument9toVariantEv()};
    let mut ret = unsafe {_ZNK13QJsonDocument9toVariantEv(rsthis.qclsinst)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QJsonDocument::isEmpty();
impl /*struct*/ QJsonDocument {
  pub fn isEmpty<RetType, T: QJsonDocument_isEmpty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QJsonDocument_isEmpty<RetType> {
  fn isEmpty(self , rsthis: & QJsonDocument) -> RetType;
}

  // proto:  bool QJsonDocument::isEmpty();
impl<'a> /*trait*/ QJsonDocument_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: & QJsonDocument) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QJsonDocument7isEmptyEv()};
    let mut ret = unsafe {_ZNK13QJsonDocument7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  const char * QJsonDocument::rawData(int * size);
impl /*struct*/ QJsonDocument {
  pub fn rawData<RetType, T: QJsonDocument_rawData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rawData(self);
    // return 1;
  }
}

pub trait QJsonDocument_rawData<RetType> {
  fn rawData(self , rsthis: & QJsonDocument) -> RetType;
}

  // proto:  const char * QJsonDocument::rawData(int * size);
impl<'a> /*trait*/ QJsonDocument_rawData<String> for (&'a mut Vec<i32>) {
  fn rawData(self , rsthis: & QJsonDocument) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QJsonDocument7rawDataEPi()};
    let arg0 = self.as_ptr()  as *mut c_int;
    let mut ret = unsafe {_ZNK13QJsonDocument7rawDataEPi(rsthis.qclsinst, arg0)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto:  bool QJsonDocument::isObject();
impl /*struct*/ QJsonDocument {
  pub fn isObject<RetType, T: QJsonDocument_isObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isObject(self);
    // return 1;
  }
}

pub trait QJsonDocument_isObject<RetType> {
  fn isObject(self , rsthis: & QJsonDocument) -> RetType;
}

  // proto:  bool QJsonDocument::isObject();
impl<'a> /*trait*/ QJsonDocument_isObject<i8> for () {
  fn isObject(self , rsthis: & QJsonDocument) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QJsonDocument8isObjectEv()};
    let mut ret = unsafe {_ZNK13QJsonDocument8isObjectEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QJsonDocument::~QJsonDocument();
impl /*struct*/ QJsonDocument {
  pub fn free<RetType, T: QJsonDocument_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QJsonDocument_free<RetType> {
  fn free(self , rsthis: & QJsonDocument) -> RetType;
}

  // proto:  void QJsonDocument::~QJsonDocument();
impl<'a> /*trait*/ QJsonDocument_free<()> for () {
  fn free(self , rsthis: & QJsonDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QJsonDocumentD2Ev()};
     unsafe {_ZN13QJsonDocumentD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QJsonDocument::isArray();
impl /*struct*/ QJsonDocument {
  pub fn isArray<RetType, T: QJsonDocument_isArray<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isArray(self);
    // return 1;
  }
}

pub trait QJsonDocument_isArray<RetType> {
  fn isArray(self , rsthis: & QJsonDocument) -> RetType;
}

  // proto:  bool QJsonDocument::isArray();
impl<'a> /*trait*/ QJsonDocument_isArray<i8> for () {
  fn isArray(self , rsthis: & QJsonDocument) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QJsonDocument7isArrayEv()};
    let mut ret = unsafe {_ZNK13QJsonDocument7isArrayEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QByteArray QJsonDocument::toBinaryData();
impl /*struct*/ QJsonDocument {
  pub fn toBinaryData<RetType, T: QJsonDocument_toBinaryData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toBinaryData(self);
    // return 1;
  }
}

pub trait QJsonDocument_toBinaryData<RetType> {
  fn toBinaryData(self , rsthis: & QJsonDocument) -> RetType;
}

  // proto:  QByteArray QJsonDocument::toBinaryData();
impl<'a> /*trait*/ QJsonDocument_toBinaryData<QByteArray> for () {
  fn toBinaryData(self , rsthis: & QJsonDocument) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QJsonDocument12toBinaryDataEv()};
    let mut ret = unsafe {_ZNK13QJsonDocument12toBinaryDataEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QJsonParseError {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QJsonParseError {
    return QJsonParseError{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QString QJsonParseError::errorString();
impl /*struct*/ QJsonParseError {
  pub fn errorString<RetType, T: QJsonParseError_errorString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.errorString(self);
    // return 1;
  }
}

pub trait QJsonParseError_errorString<RetType> {
  fn errorString(self , rsthis: & QJsonParseError) -> RetType;
}

  // proto:  QString QJsonParseError::errorString();
impl<'a> /*trait*/ QJsonParseError_errorString<QString> for () {
  fn errorString(self , rsthis: & QJsonParseError) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QJsonParseError11errorStringEv()};
    let mut ret = unsafe {_ZNK15QJsonParseError11errorStringEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

