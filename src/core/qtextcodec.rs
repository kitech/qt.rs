// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
// src-file: /QtCore/qtextcodec.h
// dst-file: /src/core/qtextcodec.rs
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
use super::qstring::*; // 773
use super::qbytearray::*; // 773
// use super::qtextcodec::QTextCodec; // 773
use super::qchar::*; // 773
// use super::qlist::*; // 775
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QTextEncoder_Class_Size() -> c_int;
  // proto:  void QTextEncoder::~QTextEncoder();
  fn C_ZN12QTextEncoderD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QByteArray QTextEncoder::fromUnicode(const QString & str);
  fn C_ZN12QTextEncoder11fromUnicodeERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QTextEncoder::hasFailure();
  fn C_ZNK12QTextEncoder10hasFailureEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QTextEncoder::QTextEncoder(const QTextCodec * codec);
  fn C_ZN12QTextEncoderC2EPK10QTextCodec(arg0: *mut c_void) -> u64;
  // proto:  QByteArray QTextEncoder::fromUnicode(const QChar * uc, int len);
  fn C_ZN12QTextEncoder11fromUnicodeEPK5QChari(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  fn QTextCodec_Class_Size() -> c_int;
  // proto:  QByteArray QTextCodec::name();
  fn C_ZNK10QTextCodec4nameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QTextCodec::toUnicode(const QByteArray & );
  fn C_ZNK10QTextCodec9toUnicodeERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QByteArray QTextCodec::fromUnicode(const QString & uc);
  fn C_ZNK10QTextCodec11fromUnicodeERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto: static QTextCodec * QTextCodec::codecForLocale();
  fn C_ZN10QTextCodec14codecForLocaleEv() -> *mut c_void;
  // proto: static QList<int> QTextCodec::availableMibs();
  fn C_ZN10QTextCodec13availableMibsEv() -> *mut c_void;
  // proto: static QTextCodec * QTextCodec::codecForHtml(const QByteArray & ba);
  fn C_ZN10QTextCodec12codecForHtmlERK10QByteArray(arg0: *mut c_void) -> *mut c_void;
  // proto: static void QTextCodec::setCodecForLocale(QTextCodec * c);
  fn C_ZN10QTextCodec17setCodecForLocaleEPS_(arg0: *mut c_void);
  // proto: static QTextCodec * QTextCodec::codecForUtfText(const QByteArray & ba);
  fn C_ZN10QTextCodec15codecForUtfTextERK10QByteArray(arg0: *mut c_void) -> *mut c_void;
  // proto:  QString QTextCodec::toUnicode(const char * chars);
  fn C_ZNK10QTextCodec9toUnicodeEPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_char) -> *mut c_void;
  // proto:  int QTextCodec::mibEnum();
  fn C_ZNK10QTextCodec7mibEnumEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto: static QTextCodec * QTextCodec::codecForName(const char * name);
  fn C_ZN10QTextCodec12codecForNameEPKc(arg0: *mut c_char) -> *mut c_void;
  // proto:  bool QTextCodec::canEncode(const QString & );
  fn C_ZNK10QTextCodec9canEncodeERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QList<QByteArray> QTextCodec::aliases();
  fn C_ZNK10QTextCodec7aliasesEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static QTextCodec * QTextCodec::codecForName(const QByteArray & name);
  fn C_ZN10QTextCodec12codecForNameERK10QByteArray(arg0: *mut c_void) -> *mut c_void;
  // proto: static QList<QByteArray> QTextCodec::availableCodecs();
  fn C_ZN10QTextCodec15availableCodecsEv() -> *mut c_void;
  // proto: static QTextCodec * QTextCodec::codecForHtml(const QByteArray & ba, QTextCodec * defaultCodec);
  fn C_ZN10QTextCodec12codecForHtmlERK10QByteArrayPS_(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto: static QTextCodec * QTextCodec::codecForMib(int mib);
  fn C_ZN10QTextCodec11codecForMibEi(arg0: c_int) -> *mut c_void;
  // proto: static QTextCodec * QTextCodec::codecForUtfText(const QByteArray & ba, QTextCodec * defaultCodec);
  fn C_ZN10QTextCodec15codecForUtfTextERK10QByteArrayPS_(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  bool QTextCodec::canEncode(QChar );
  fn C_ZNK10QTextCodec9canEncodeE5QChar(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  fn QTextDecoder_Class_Size() -> c_int;
  // proto:  QString QTextDecoder::toUnicode(const char * chars, int len);
  fn C_ZN12QTextDecoder9toUnicodeEPKci(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_int) -> *mut c_void;
  // proto:  void QTextDecoder::QTextDecoder(const QTextCodec * codec);
  fn C_ZN12QTextDecoderC2EPK10QTextCodec(arg0: *mut c_void) -> u64;
  // proto:  bool QTextDecoder::hasFailure();
  fn C_ZNK12QTextDecoder10hasFailureEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QTextDecoder::~QTextDecoder();
  fn C_ZN12QTextDecoderD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QString QTextDecoder::toUnicode(const QByteArray & ba);
  fn C_ZN12QTextDecoder9toUnicodeERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTextDecoder::toUnicode(QString * target, const char * chars, int len);
  fn C_ZN12QTextDecoder9toUnicodeEP7QStringPKci(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_char, arg2: c_int);
} // <= ext block end

// body block begin =>
// class sizeof(QTextEncoder)=1
#[derive(Default)]
pub struct QTextEncoder {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QTextCodec)=8
#[derive(Default)]
pub struct QTextCodec {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QTextDecoder)=1
#[derive(Default)]
pub struct QTextDecoder {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QTextEncoder {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTextEncoder {
    return QTextEncoder{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QTextEncoder::~QTextEncoder();
impl /*struct*/ QTextEncoder {
  pub fn free<RetType, T: QTextEncoder_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QTextEncoder_free<RetType> {
  fn free(self , rsthis: & QTextEncoder) -> RetType;
}

  // proto:  void QTextEncoder::~QTextEncoder();
impl<'a> /*trait*/ QTextEncoder_free<()> for () {
  fn free(self , rsthis: & QTextEncoder) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextEncoderD2Ev()};
     unsafe {C_ZN12QTextEncoderD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QByteArray QTextEncoder::fromUnicode(const QString & str);
impl /*struct*/ QTextEncoder {
  pub fn fromUnicode<RetType, T: QTextEncoder_fromUnicode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fromUnicode(self);
    // return 1;
  }
}

pub trait QTextEncoder_fromUnicode<RetType> {
  fn fromUnicode(self , rsthis: & QTextEncoder) -> RetType;
}

  // proto:  QByteArray QTextEncoder::fromUnicode(const QString & str);
impl<'a> /*trait*/ QTextEncoder_fromUnicode<QByteArray> for (&'a QString) {
  fn fromUnicode(self , rsthis: & QTextEncoder) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextEncoder11fromUnicodeERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN12QTextEncoder11fromUnicodeERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QTextEncoder::hasFailure();
impl /*struct*/ QTextEncoder {
  pub fn hasFailure<RetType, T: QTextEncoder_hasFailure<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasFailure(self);
    // return 1;
  }
}

pub trait QTextEncoder_hasFailure<RetType> {
  fn hasFailure(self , rsthis: & QTextEncoder) -> RetType;
}

  // proto:  bool QTextEncoder::hasFailure();
impl<'a> /*trait*/ QTextEncoder_hasFailure<i8> for () {
  fn hasFailure(self , rsthis: & QTextEncoder) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextEncoder10hasFailureEv()};
    let mut ret = unsafe {C_ZNK12QTextEncoder10hasFailureEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QTextEncoder::QTextEncoder(const QTextCodec * codec);
impl /*struct*/ QTextEncoder {
  pub fn new<T: QTextEncoder_new>(value: T) -> QTextEncoder {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QTextEncoder_new {
  fn new(self) -> QTextEncoder;
}

  // proto:  void QTextEncoder::QTextEncoder(const QTextCodec * codec);
impl<'a> /*trait*/ QTextEncoder_new for (&'a QTextCodec) {
  fn new(self) -> QTextEncoder {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextEncoderC2EPK10QTextCodec()};
    let ctysz: c_int = unsafe{QTextEncoder_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN12QTextEncoderC2EPK10QTextCodec(arg0)};
    let rsthis = QTextEncoder{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QByteArray QTextEncoder::fromUnicode(const QChar * uc, int len);
impl<'a> /*trait*/ QTextEncoder_fromUnicode<QByteArray> for (&'a QChar, i32) {
  fn fromUnicode(self , rsthis: & QTextEncoder) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextEncoder11fromUnicodeEPK5QChari()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZN12QTextEncoder11fromUnicodeEPK5QChari(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextCodec {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTextCodec {
    return QTextCodec{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QByteArray QTextCodec::name();
impl /*struct*/ QTextCodec {
  pub fn name<RetType, T: QTextCodec_name<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QTextCodec_name<RetType> {
  fn name(self , rsthis: & QTextCodec) -> RetType;
}

  // proto:  QByteArray QTextCodec::name();
impl<'a> /*trait*/ QTextCodec_name<QByteArray> for () {
  fn name(self , rsthis: & QTextCodec) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextCodec4nameEv()};
    let mut ret = unsafe {C_ZNK10QTextCodec4nameEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QTextCodec::toUnicode(const QByteArray & );
impl /*struct*/ QTextCodec {
  pub fn toUnicode<RetType, T: QTextCodec_toUnicode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toUnicode(self);
    // return 1;
  }
}

pub trait QTextCodec_toUnicode<RetType> {
  fn toUnicode(self , rsthis: & QTextCodec) -> RetType;
}

  // proto:  QString QTextCodec::toUnicode(const QByteArray & );
impl<'a> /*trait*/ QTextCodec_toUnicode<QString> for (&'a QByteArray) {
  fn toUnicode(self , rsthis: & QTextCodec) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextCodec9toUnicodeERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK10QTextCodec9toUnicodeERK10QByteArray(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray QTextCodec::fromUnicode(const QString & uc);
impl /*struct*/ QTextCodec {
  pub fn fromUnicode<RetType, T: QTextCodec_fromUnicode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fromUnicode(self);
    // return 1;
  }
}

pub trait QTextCodec_fromUnicode<RetType> {
  fn fromUnicode(self , rsthis: & QTextCodec) -> RetType;
}

  // proto:  QByteArray QTextCodec::fromUnicode(const QString & uc);
impl<'a> /*trait*/ QTextCodec_fromUnicode<QByteArray> for (&'a QString) {
  fn fromUnicode(self , rsthis: & QTextCodec) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextCodec11fromUnicodeERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK10QTextCodec11fromUnicodeERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QTextCodec * QTextCodec::codecForLocale();
impl /*struct*/ QTextCodec {
  pub fn codecForLocale_s<RetType, T: QTextCodec_codecForLocale_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.codecForLocale_s();
    // return 1;
  }
}

pub trait QTextCodec_codecForLocale_s<RetType> {
  fn codecForLocale_s(self ) -> RetType;
}

  // proto: static QTextCodec * QTextCodec::codecForLocale();
impl<'a> /*trait*/ QTextCodec_codecForLocale_s<QTextCodec> for () {
  fn codecForLocale_s(self ) -> QTextCodec {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec14codecForLocaleEv()};
    let mut ret = unsafe {C_ZN10QTextCodec14codecForLocaleEv()};
    let mut ret1 = QTextCodec::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QList<int> QTextCodec::availableMibs();
impl /*struct*/ QTextCodec {
  pub fn availableMibs_s<RetType, T: QTextCodec_availableMibs_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.availableMibs_s();
    // return 1;
  }
}

pub trait QTextCodec_availableMibs_s<RetType> {
  fn availableMibs_s(self ) -> RetType;
}

  // proto: static QList<int> QTextCodec::availableMibs();
impl<'a> /*trait*/ QTextCodec_availableMibs_s<u64> for () {
  fn availableMibs_s(self ) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec13availableMibsEv()};
    let mut ret = unsafe {C_ZN10QTextCodec13availableMibsEv()};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto: static QTextCodec * QTextCodec::codecForHtml(const QByteArray & ba);
impl /*struct*/ QTextCodec {
  pub fn codecForHtml_s<RetType, T: QTextCodec_codecForHtml_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.codecForHtml_s();
    // return 1;
  }
}

pub trait QTextCodec_codecForHtml_s<RetType> {
  fn codecForHtml_s(self ) -> RetType;
}

  // proto: static QTextCodec * QTextCodec::codecForHtml(const QByteArray & ba);
impl<'a> /*trait*/ QTextCodec_codecForHtml_s<QTextCodec> for (&'a QByteArray) {
  fn codecForHtml_s(self ) -> QTextCodec {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec12codecForHtmlERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN10QTextCodec12codecForHtmlERK10QByteArray(arg0)};
    let mut ret1 = QTextCodec::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static void QTextCodec::setCodecForLocale(QTextCodec * c);
impl /*struct*/ QTextCodec {
  pub fn setCodecForLocale_s<RetType, T: QTextCodec_setCodecForLocale_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setCodecForLocale_s();
    // return 1;
  }
}

pub trait QTextCodec_setCodecForLocale_s<RetType> {
  fn setCodecForLocale_s(self ) -> RetType;
}

  // proto: static void QTextCodec::setCodecForLocale(QTextCodec * c);
impl<'a> /*trait*/ QTextCodec_setCodecForLocale_s<()> for (&'a QTextCodec) {
  fn setCodecForLocale_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec17setCodecForLocaleEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN10QTextCodec17setCodecForLocaleEPS_(arg0)};
    // return 1;
  }
}

  // proto: static QTextCodec * QTextCodec::codecForUtfText(const QByteArray & ba);
impl /*struct*/ QTextCodec {
  pub fn codecForUtfText_s<RetType, T: QTextCodec_codecForUtfText_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.codecForUtfText_s();
    // return 1;
  }
}

pub trait QTextCodec_codecForUtfText_s<RetType> {
  fn codecForUtfText_s(self ) -> RetType;
}

  // proto: static QTextCodec * QTextCodec::codecForUtfText(const QByteArray & ba);
impl<'a> /*trait*/ QTextCodec_codecForUtfText_s<QTextCodec> for (&'a QByteArray) {
  fn codecForUtfText_s(self ) -> QTextCodec {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec15codecForUtfTextERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN10QTextCodec15codecForUtfTextERK10QByteArray(arg0)};
    let mut ret1 = QTextCodec::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QTextCodec::toUnicode(const char * chars);
impl<'a> /*trait*/ QTextCodec_toUnicode<QString> for (&'a  String) {
  fn toUnicode(self , rsthis: & QTextCodec) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextCodec9toUnicodeEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {C_ZNK10QTextCodec9toUnicodeEPKc(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QTextCodec::mibEnum();
impl /*struct*/ QTextCodec {
  pub fn mibEnum<RetType, T: QTextCodec_mibEnum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mibEnum(self);
    // return 1;
  }
}

pub trait QTextCodec_mibEnum<RetType> {
  fn mibEnum(self , rsthis: & QTextCodec) -> RetType;
}

  // proto:  int QTextCodec::mibEnum();
impl<'a> /*trait*/ QTextCodec_mibEnum<i32> for () {
  fn mibEnum(self , rsthis: & QTextCodec) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextCodec7mibEnumEv()};
    let mut ret = unsafe {C_ZNK10QTextCodec7mibEnumEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto: static QTextCodec * QTextCodec::codecForName(const char * name);
impl /*struct*/ QTextCodec {
  pub fn codecForName_s<RetType, T: QTextCodec_codecForName_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.codecForName_s();
    // return 1;
  }
}

pub trait QTextCodec_codecForName_s<RetType> {
  fn codecForName_s(self ) -> RetType;
}

  // proto: static QTextCodec * QTextCodec::codecForName(const char * name);
impl<'a> /*trait*/ QTextCodec_codecForName_s<QTextCodec> for (&'a  String) {
  fn codecForName_s(self ) -> QTextCodec {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec12codecForNameEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {C_ZN10QTextCodec12codecForNameEPKc(arg0)};
    let mut ret1 = QTextCodec::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QTextCodec::canEncode(const QString & );
impl /*struct*/ QTextCodec {
  pub fn canEncode<RetType, T: QTextCodec_canEncode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.canEncode(self);
    // return 1;
  }
}

pub trait QTextCodec_canEncode<RetType> {
  fn canEncode(self , rsthis: & QTextCodec) -> RetType;
}

  // proto:  bool QTextCodec::canEncode(const QString & );
impl<'a> /*trait*/ QTextCodec_canEncode<i8> for (&'a QString) {
  fn canEncode(self , rsthis: & QTextCodec) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextCodec9canEncodeERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK10QTextCodec9canEncodeERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QList<QByteArray> QTextCodec::aliases();
impl /*struct*/ QTextCodec {
  pub fn aliases<RetType, T: QTextCodec_aliases<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.aliases(self);
    // return 1;
  }
}

pub trait QTextCodec_aliases<RetType> {
  fn aliases(self , rsthis: & QTextCodec) -> RetType;
}

  // proto:  QList<QByteArray> QTextCodec::aliases();
impl<'a> /*trait*/ QTextCodec_aliases<u64> for () {
  fn aliases(self , rsthis: & QTextCodec) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextCodec7aliasesEv()};
    let mut ret = unsafe {C_ZNK10QTextCodec7aliasesEv(rsthis.qclsinst)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto: static QTextCodec * QTextCodec::codecForName(const QByteArray & name);
impl<'a> /*trait*/ QTextCodec_codecForName_s<QTextCodec> for (&'a QByteArray) {
  fn codecForName_s(self ) -> QTextCodec {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec12codecForNameERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN10QTextCodec12codecForNameERK10QByteArray(arg0)};
    let mut ret1 = QTextCodec::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QList<QByteArray> QTextCodec::availableCodecs();
impl /*struct*/ QTextCodec {
  pub fn availableCodecs_s<RetType, T: QTextCodec_availableCodecs_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.availableCodecs_s();
    // return 1;
  }
}

pub trait QTextCodec_availableCodecs_s<RetType> {
  fn availableCodecs_s(self ) -> RetType;
}

  // proto: static QList<QByteArray> QTextCodec::availableCodecs();
impl<'a> /*trait*/ QTextCodec_availableCodecs_s<u64> for () {
  fn availableCodecs_s(self ) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec15availableCodecsEv()};
    let mut ret = unsafe {C_ZN10QTextCodec15availableCodecsEv()};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto: static QTextCodec * QTextCodec::codecForHtml(const QByteArray & ba, QTextCodec * defaultCodec);
impl<'a> /*trait*/ QTextCodec_codecForHtml_s<QTextCodec> for (&'a QByteArray, &'a QTextCodec) {
  fn codecForHtml_s(self ) -> QTextCodec {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec12codecForHtmlERK10QByteArrayPS_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN10QTextCodec12codecForHtmlERK10QByteArrayPS_(arg0, arg1)};
    let mut ret1 = QTextCodec::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QTextCodec * QTextCodec::codecForMib(int mib);
impl /*struct*/ QTextCodec {
  pub fn codecForMib_s<RetType, T: QTextCodec_codecForMib_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.codecForMib_s();
    // return 1;
  }
}

pub trait QTextCodec_codecForMib_s<RetType> {
  fn codecForMib_s(self ) -> RetType;
}

  // proto: static QTextCodec * QTextCodec::codecForMib(int mib);
impl<'a> /*trait*/ QTextCodec_codecForMib_s<QTextCodec> for (i32) {
  fn codecForMib_s(self ) -> QTextCodec {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec11codecForMibEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZN10QTextCodec11codecForMibEi(arg0)};
    let mut ret1 = QTextCodec::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QTextCodec * QTextCodec::codecForUtfText(const QByteArray & ba, QTextCodec * defaultCodec);
impl<'a> /*trait*/ QTextCodec_codecForUtfText_s<QTextCodec> for (&'a QByteArray, &'a QTextCodec) {
  fn codecForUtfText_s(self ) -> QTextCodec {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextCodec15codecForUtfTextERK10QByteArrayPS_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN10QTextCodec15codecForUtfTextERK10QByteArrayPS_(arg0, arg1)};
    let mut ret1 = QTextCodec::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QTextCodec::canEncode(QChar );
impl<'a> /*trait*/ QTextCodec_canEncode<i8> for (QChar) {
  fn canEncode(self , rsthis: & QTextCodec) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextCodec9canEncodeE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK10QTextCodec9canEncodeE5QChar(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

impl /*struct*/ QTextDecoder {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTextDecoder {
    return QTextDecoder{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QString QTextDecoder::toUnicode(const char * chars, int len);
impl /*struct*/ QTextDecoder {
  pub fn toUnicode<RetType, T: QTextDecoder_toUnicode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toUnicode(self);
    // return 1;
  }
}

pub trait QTextDecoder_toUnicode<RetType> {
  fn toUnicode(self , rsthis: & QTextDecoder) -> RetType;
}

  // proto:  QString QTextDecoder::toUnicode(const char * chars, int len);
impl<'a> /*trait*/ QTextDecoder_toUnicode<QString> for (&'a  String, i32) {
  fn toUnicode(self , rsthis: & QTextDecoder) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextDecoder9toUnicodeEPKci()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZN12QTextDecoder9toUnicodeEPKci(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextDecoder::QTextDecoder(const QTextCodec * codec);
impl /*struct*/ QTextDecoder {
  pub fn new<T: QTextDecoder_new>(value: T) -> QTextDecoder {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QTextDecoder_new {
  fn new(self) -> QTextDecoder;
}

  // proto:  void QTextDecoder::QTextDecoder(const QTextCodec * codec);
impl<'a> /*trait*/ QTextDecoder_new for (&'a QTextCodec) {
  fn new(self) -> QTextDecoder {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextDecoderC2EPK10QTextCodec()};
    let ctysz: c_int = unsafe{QTextDecoder_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN12QTextDecoderC2EPK10QTextCodec(arg0)};
    let rsthis = QTextDecoder{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QTextDecoder::hasFailure();
impl /*struct*/ QTextDecoder {
  pub fn hasFailure<RetType, T: QTextDecoder_hasFailure<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasFailure(self);
    // return 1;
  }
}

pub trait QTextDecoder_hasFailure<RetType> {
  fn hasFailure(self , rsthis: & QTextDecoder) -> RetType;
}

  // proto:  bool QTextDecoder::hasFailure();
impl<'a> /*trait*/ QTextDecoder_hasFailure<i8> for () {
  fn hasFailure(self , rsthis: & QTextDecoder) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextDecoder10hasFailureEv()};
    let mut ret = unsafe {C_ZNK12QTextDecoder10hasFailureEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QTextDecoder::~QTextDecoder();
impl /*struct*/ QTextDecoder {
  pub fn free<RetType, T: QTextDecoder_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QTextDecoder_free<RetType> {
  fn free(self , rsthis: & QTextDecoder) -> RetType;
}

  // proto:  void QTextDecoder::~QTextDecoder();
impl<'a> /*trait*/ QTextDecoder_free<()> for () {
  fn free(self , rsthis: & QTextDecoder) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextDecoderD2Ev()};
     unsafe {C_ZN12QTextDecoderD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QTextDecoder::toUnicode(const QByteArray & ba);
impl<'a> /*trait*/ QTextDecoder_toUnicode<QString> for (&'a QByteArray) {
  fn toUnicode(self , rsthis: & QTextDecoder) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextDecoder9toUnicodeERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN12QTextDecoder9toUnicodeERK10QByteArray(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextDecoder::toUnicode(QString * target, const char * chars, int len);
impl<'a> /*trait*/ QTextDecoder_toUnicode<()> for (&'a QString, &'a  String, i32) {
  fn toUnicode(self , rsthis: & QTextDecoder) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextDecoder9toUnicodeEP7QStringPKci()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let arg2 = self.2  as c_int;
     unsafe {C_ZN12QTextDecoder9toUnicodeEP7QStringPKci(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

// <= body block end

