// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtCore/qmimedata.h
// dst-file: /src/core/qmimedata.rs
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
use super::qobject::*; // 773
use std::ops::Deref;
use super::qstring::*; // 773
use super::qbytearray::*; // 773
use super::qvariant::*; // 773
// use super::qlist::*; // 775
use super::qobjectdefs::*; // 773
use super::qstringlist::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QMimeData_Class_Size() -> c_int;
  // proto:  void QMimeData::setData(const QString & mimetype, const QByteArray & data);
  fn C_ZN9QMimeData7setDataERK7QStringRK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QVariant QMimeData::colorData();
  fn C_ZNK9QMimeData9colorDataEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QMimeData::~QMimeData();
  fn C_ZN9QMimeDataD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QMimeData::hasHtml();
  fn C_ZNK9QMimeData7hasHtmlEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QVariant QMimeData::imageData();
  fn C_ZNK9QMimeData9imageDataEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QMimeData::hasFormat(const QString & mimetype);
  fn C_ZNK9QMimeData9hasFormatERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  void QMimeData::setText(const QString & text);
  fn C_ZN9QMimeData7setTextERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QMimeData::clear();
  fn C_ZN9QMimeData5clearEv(qthis: u64 /* *mut c_void*/);
  // proto:  QString QMimeData::text();
  fn C_ZNK9QMimeData4textEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QMimeData::setHtml(const QString & html);
  fn C_ZN9QMimeData7setHtmlERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QMimeData::setImageData(const QVariant & image);
  fn C_ZN9QMimeData12setImageDataERK8QVariant(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QMimeData::hasUrls();
  fn C_ZNK9QMimeData7hasUrlsEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QMimeData::hasColor();
  fn C_ZNK9QMimeData8hasColorEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QMimeData::removeFormat(const QString & mimetype);
  fn C_ZN9QMimeData12removeFormatERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QMimeData::html();
  fn C_ZNK9QMimeData4htmlEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QMimeData::QMimeData();
  fn C_ZN9QMimeDataC2Ev() -> u64;
  // proto:  QList<QUrl> QMimeData::urls();
  fn C_ZNK9QMimeData4urlsEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QMimeData::setColorData(const QVariant & color);
  fn C_ZN9QMimeData12setColorDataERK8QVariant(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QMimeData::hasText();
  fn C_ZNK9QMimeData7hasTextEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  const QMetaObject * QMimeData::metaObject();
  fn C_ZNK9QMimeData10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QByteArray QMimeData::data(const QString & mimetype);
  fn C_ZNK9QMimeData4dataERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QStringList QMimeData::formats();
  fn C_ZNK9QMimeData7formatsEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QMimeData::hasImage();
  fn C_ZNK9QMimeData8hasImageEv(qthis: u64 /* *mut c_void*/) -> c_char;
} // <= ext block end

// body block begin =>
// class sizeof(QMimeData)=1
#[derive(Default)]
pub struct QMimeData {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QMimeData {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QMimeData {
    return QMimeData{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QMimeData {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QMimeData {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QMimeData::setData(const QString & mimetype, const QByteArray & data);
impl /*struct*/ QMimeData {
  pub fn setData<RetType, T: QMimeData_setData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setData(self);
    // return 1;
  }
}

pub trait QMimeData_setData<RetType> {
  fn setData(self , rsthis: & QMimeData) -> RetType;
}

  // proto:  void QMimeData::setData(const QString & mimetype, const QByteArray & data);
impl<'a> /*trait*/ QMimeData_setData<()> for (&'a QString, &'a QByteArray) {
  fn setData(self , rsthis: & QMimeData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeData7setDataERK7QStringRK10QByteArray()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN9QMimeData7setDataERK7QStringRK10QByteArray(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QVariant QMimeData::colorData();
impl /*struct*/ QMimeData {
  pub fn colorData<RetType, T: QMimeData_colorData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.colorData(self);
    // return 1;
  }
}

pub trait QMimeData_colorData<RetType> {
  fn colorData(self , rsthis: & QMimeData) -> RetType;
}

  // proto:  QVariant QMimeData::colorData();
impl<'a> /*trait*/ QMimeData_colorData<QVariant> for () {
  fn colorData(self , rsthis: & QMimeData) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData9colorDataEv()};
    let mut ret = unsafe {C_ZNK9QMimeData9colorDataEv(rsthis.qclsinst)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMimeData::~QMimeData();
impl /*struct*/ QMimeData {
  pub fn free<RetType, T: QMimeData_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QMimeData_free<RetType> {
  fn free(self , rsthis: & QMimeData) -> RetType;
}

  // proto:  void QMimeData::~QMimeData();
impl<'a> /*trait*/ QMimeData_free<()> for () {
  fn free(self , rsthis: & QMimeData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeDataD2Ev()};
     unsafe {C_ZN9QMimeDataD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QMimeData::hasHtml();
impl /*struct*/ QMimeData {
  pub fn hasHtml<RetType, T: QMimeData_hasHtml<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasHtml(self);
    // return 1;
  }
}

pub trait QMimeData_hasHtml<RetType> {
  fn hasHtml(self , rsthis: & QMimeData) -> RetType;
}

  // proto:  bool QMimeData::hasHtml();
impl<'a> /*trait*/ QMimeData_hasHtml<i8> for () {
  fn hasHtml(self , rsthis: & QMimeData) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData7hasHtmlEv()};
    let mut ret = unsafe {C_ZNK9QMimeData7hasHtmlEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QVariant QMimeData::imageData();
impl /*struct*/ QMimeData {
  pub fn imageData<RetType, T: QMimeData_imageData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.imageData(self);
    // return 1;
  }
}

pub trait QMimeData_imageData<RetType> {
  fn imageData(self , rsthis: & QMimeData) -> RetType;
}

  // proto:  QVariant QMimeData::imageData();
impl<'a> /*trait*/ QMimeData_imageData<QVariant> for () {
  fn imageData(self , rsthis: & QMimeData) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData9imageDataEv()};
    let mut ret = unsafe {C_ZNK9QMimeData9imageDataEv(rsthis.qclsinst)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QMimeData::hasFormat(const QString & mimetype);
impl /*struct*/ QMimeData {
  pub fn hasFormat<RetType, T: QMimeData_hasFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasFormat(self);
    // return 1;
  }
}

pub trait QMimeData_hasFormat<RetType> {
  fn hasFormat(self , rsthis: & QMimeData) -> RetType;
}

  // proto:  bool QMimeData::hasFormat(const QString & mimetype);
impl<'a> /*trait*/ QMimeData_hasFormat<i8> for (&'a QString) {
  fn hasFormat(self , rsthis: & QMimeData) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData9hasFormatERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK9QMimeData9hasFormatERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QMimeData::setText(const QString & text);
impl /*struct*/ QMimeData {
  pub fn setText<RetType, T: QMimeData_setText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setText(self);
    // return 1;
  }
}

pub trait QMimeData_setText<RetType> {
  fn setText(self , rsthis: & QMimeData) -> RetType;
}

  // proto:  void QMimeData::setText(const QString & text);
impl<'a> /*trait*/ QMimeData_setText<()> for (&'a QString) {
  fn setText(self , rsthis: & QMimeData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeData7setTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN9QMimeData7setTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMimeData::clear();
impl /*struct*/ QMimeData {
  pub fn clear<RetType, T: QMimeData_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QMimeData_clear<RetType> {
  fn clear(self , rsthis: & QMimeData) -> RetType;
}

  // proto:  void QMimeData::clear();
impl<'a> /*trait*/ QMimeData_clear<()> for () {
  fn clear(self , rsthis: & QMimeData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeData5clearEv()};
     unsafe {C_ZN9QMimeData5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QMimeData::text();
impl /*struct*/ QMimeData {
  pub fn text<RetType, T: QMimeData_text<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.text(self);
    // return 1;
  }
}

pub trait QMimeData_text<RetType> {
  fn text(self , rsthis: & QMimeData) -> RetType;
}

  // proto:  QString QMimeData::text();
impl<'a> /*trait*/ QMimeData_text<QString> for () {
  fn text(self , rsthis: & QMimeData) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData4textEv()};
    let mut ret = unsafe {C_ZNK9QMimeData4textEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMimeData::setHtml(const QString & html);
impl /*struct*/ QMimeData {
  pub fn setHtml<RetType, T: QMimeData_setHtml<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setHtml(self);
    // return 1;
  }
}

pub trait QMimeData_setHtml<RetType> {
  fn setHtml(self , rsthis: & QMimeData) -> RetType;
}

  // proto:  void QMimeData::setHtml(const QString & html);
impl<'a> /*trait*/ QMimeData_setHtml<()> for (&'a QString) {
  fn setHtml(self , rsthis: & QMimeData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeData7setHtmlERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN9QMimeData7setHtmlERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMimeData::setImageData(const QVariant & image);
impl /*struct*/ QMimeData {
  pub fn setImageData<RetType, T: QMimeData_setImageData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setImageData(self);
    // return 1;
  }
}

pub trait QMimeData_setImageData<RetType> {
  fn setImageData(self , rsthis: & QMimeData) -> RetType;
}

  // proto:  void QMimeData::setImageData(const QVariant & image);
impl<'a> /*trait*/ QMimeData_setImageData<()> for (&'a QVariant) {
  fn setImageData(self , rsthis: & QMimeData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeData12setImageDataERK8QVariant()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN9QMimeData12setImageDataERK8QVariant(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QMimeData::hasUrls();
impl /*struct*/ QMimeData {
  pub fn hasUrls<RetType, T: QMimeData_hasUrls<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasUrls(self);
    // return 1;
  }
}

pub trait QMimeData_hasUrls<RetType> {
  fn hasUrls(self , rsthis: & QMimeData) -> RetType;
}

  // proto:  bool QMimeData::hasUrls();
impl<'a> /*trait*/ QMimeData_hasUrls<i8> for () {
  fn hasUrls(self , rsthis: & QMimeData) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData7hasUrlsEv()};
    let mut ret = unsafe {C_ZNK9QMimeData7hasUrlsEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QMimeData::hasColor();
impl /*struct*/ QMimeData {
  pub fn hasColor<RetType, T: QMimeData_hasColor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasColor(self);
    // return 1;
  }
}

pub trait QMimeData_hasColor<RetType> {
  fn hasColor(self , rsthis: & QMimeData) -> RetType;
}

  // proto:  bool QMimeData::hasColor();
impl<'a> /*trait*/ QMimeData_hasColor<i8> for () {
  fn hasColor(self , rsthis: & QMimeData) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData8hasColorEv()};
    let mut ret = unsafe {C_ZNK9QMimeData8hasColorEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QMimeData::removeFormat(const QString & mimetype);
impl /*struct*/ QMimeData {
  pub fn removeFormat<RetType, T: QMimeData_removeFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeFormat(self);
    // return 1;
  }
}

pub trait QMimeData_removeFormat<RetType> {
  fn removeFormat(self , rsthis: & QMimeData) -> RetType;
}

  // proto:  void QMimeData::removeFormat(const QString & mimetype);
impl<'a> /*trait*/ QMimeData_removeFormat<()> for (&'a QString) {
  fn removeFormat(self , rsthis: & QMimeData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeData12removeFormatERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN9QMimeData12removeFormatERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QMimeData::html();
impl /*struct*/ QMimeData {
  pub fn html<RetType, T: QMimeData_html<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.html(self);
    // return 1;
  }
}

pub trait QMimeData_html<RetType> {
  fn html(self , rsthis: & QMimeData) -> RetType;
}

  // proto:  QString QMimeData::html();
impl<'a> /*trait*/ QMimeData_html<QString> for () {
  fn html(self , rsthis: & QMimeData) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData4htmlEv()};
    let mut ret = unsafe {C_ZNK9QMimeData4htmlEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMimeData::QMimeData();
impl /*struct*/ QMimeData {
  pub fn new<T: QMimeData_new>(value: T) -> QMimeData {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QMimeData_new {
  fn new(self) -> QMimeData;
}

  // proto:  void QMimeData::QMimeData();
impl<'a> /*trait*/ QMimeData_new for () {
  fn new(self) -> QMimeData {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeDataC2Ev()};
    let ctysz: c_int = unsafe{QMimeData_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN9QMimeDataC2Ev()};
    let rsthis = QMimeData{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QList<QUrl> QMimeData::urls();
impl /*struct*/ QMimeData {
  pub fn urls<RetType, T: QMimeData_urls<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.urls(self);
    // return 1;
  }
}

pub trait QMimeData_urls<RetType> {
  fn urls(self , rsthis: & QMimeData) -> RetType;
}

  // proto:  QList<QUrl> QMimeData::urls();
impl<'a> /*trait*/ QMimeData_urls<u64> for () {
  fn urls(self , rsthis: & QMimeData) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData4urlsEv()};
    let mut ret = unsafe {C_ZNK9QMimeData4urlsEv(rsthis.qclsinst)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  void QMimeData::setColorData(const QVariant & color);
impl /*struct*/ QMimeData {
  pub fn setColorData<RetType, T: QMimeData_setColorData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setColorData(self);
    // return 1;
  }
}

pub trait QMimeData_setColorData<RetType> {
  fn setColorData(self , rsthis: & QMimeData) -> RetType;
}

  // proto:  void QMimeData::setColorData(const QVariant & color);
impl<'a> /*trait*/ QMimeData_setColorData<()> for (&'a QVariant) {
  fn setColorData(self , rsthis: & QMimeData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeData12setColorDataERK8QVariant()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN9QMimeData12setColorDataERK8QVariant(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QMimeData::hasText();
impl /*struct*/ QMimeData {
  pub fn hasText<RetType, T: QMimeData_hasText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasText(self);
    // return 1;
  }
}

pub trait QMimeData_hasText<RetType> {
  fn hasText(self , rsthis: & QMimeData) -> RetType;
}

  // proto:  bool QMimeData::hasText();
impl<'a> /*trait*/ QMimeData_hasText<i8> for () {
  fn hasText(self , rsthis: & QMimeData) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData7hasTextEv()};
    let mut ret = unsafe {C_ZNK9QMimeData7hasTextEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  const QMetaObject * QMimeData::metaObject();
impl /*struct*/ QMimeData {
  pub fn metaObject<RetType, T: QMimeData_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QMimeData_metaObject<RetType> {
  fn metaObject(self , rsthis: & QMimeData) -> RetType;
}

  // proto:  const QMetaObject * QMimeData::metaObject();
impl<'a> /*trait*/ QMimeData_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QMimeData) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData10metaObjectEv()};
    let mut ret = unsafe {C_ZNK9QMimeData10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray QMimeData::data(const QString & mimetype);
impl /*struct*/ QMimeData {
  pub fn data<RetType, T: QMimeData_data<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QMimeData_data<RetType> {
  fn data(self , rsthis: & QMimeData) -> RetType;
}

  // proto:  QByteArray QMimeData::data(const QString & mimetype);
impl<'a> /*trait*/ QMimeData_data<QByteArray> for (&'a QString) {
  fn data(self , rsthis: & QMimeData) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData4dataERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK9QMimeData4dataERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QStringList QMimeData::formats();
impl /*struct*/ QMimeData {
  pub fn formats<RetType, T: QMimeData_formats<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.formats(self);
    // return 1;
  }
}

pub trait QMimeData_formats<RetType> {
  fn formats(self , rsthis: & QMimeData) -> RetType;
}

  // proto:  QStringList QMimeData::formats();
impl<'a> /*trait*/ QMimeData_formats<QStringList> for () {
  fn formats(self , rsthis: & QMimeData) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData7formatsEv()};
    let mut ret = unsafe {C_ZNK9QMimeData7formatsEv(rsthis.qclsinst)};
    let mut ret1 = QStringList::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QMimeData::hasImage();
impl /*struct*/ QMimeData {
  pub fn hasImage<RetType, T: QMimeData_hasImage<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasImage(self);
    // return 1;
  }
}

pub trait QMimeData_hasImage<RetType> {
  fn hasImage(self , rsthis: & QMimeData) -> RetType;
}

  // proto:  bool QMimeData::hasImage();
impl<'a> /*trait*/ QMimeData_hasImage<i8> for () {
  fn hasImage(self , rsthis: & QMimeData) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData8hasImageEv()};
    let mut ret = unsafe {C_ZNK9QMimeData8hasImageEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

// <= body block end

