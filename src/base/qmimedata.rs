// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qbytearray::QByteArray;
use super::qvariant::QVariant;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QMimeData::setData(const QString & mimetype, const QByteArray & data);
  fn _ZN9QMimeData7setDataERK7QStringRK10QByteArray(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  QVariant QMimeData::colorData();
  fn _ZNK9QMimeData9colorDataEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMimeData::FreeQMimeData();
  fn _ZN9QMimeDataD0Ev(qthis: *mut c_void) ;
  // proto:  bool QMimeData::hasHtml();
  fn _ZNK9QMimeData7hasHtmlEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QMimeData::NewQMimeData(const QMimeData & );
  fn _ZN9QMimeDataC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QVariant QMimeData::imageData();
  fn _ZNK9QMimeData9imageDataEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QMimeData::hasFormat(const QString & mimetype);
  fn _ZNK9QMimeData9hasFormatERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QMimeData::setText(const QString & text);
  fn _ZN9QMimeData7setTextERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QMimeData::clear();
  fn _ZN9QMimeData5clearEv(qthis: *mut c_void) ;
  // proto:  QString QMimeData::text();
  fn _ZNK9QMimeData4textEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMimeData::setHtml(const QString & html);
  fn _ZN9QMimeData7setHtmlERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QMimeData::setImageData(const QVariant & image);
  fn _ZN9QMimeData12setImageDataERK8QVariant(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QMimeData::hasUrls();
  fn _ZNK9QMimeData7hasUrlsEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QMimeData::hasColor();
  fn _ZNK9QMimeData8hasColorEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QMimeData::removeFormat(const QString & mimetype);
  fn _ZN9QMimeData12removeFormatERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QMimeData::html();
  fn _ZNK9QMimeData4htmlEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMimeData::NewQMimeData();
  fn _ZN9QMimeDataC1Ev(qthis: *mut c_void) ;
  // proto:  QList<QUrl> QMimeData::urls();
  fn _ZNK9QMimeData4urlsEv(qthis: *mut c_void) ;
  // proto:  void QMimeData::setColorData(const QVariant & color);
  fn _ZN9QMimeData12setColorDataERK8QVariant(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QMimeData::hasText();
  fn _ZNK9QMimeData7hasTextEv(qthis: *mut c_void) -> int8_t;
  // proto:  const QMetaObject * QMimeData::metaObject();
  fn _ZNK9QMimeData10metaObjectEv(qthis: *mut c_void) ;
  // proto:  QByteArray QMimeData::data(const QString & mimetype);
  fn _ZNK9QMimeData4dataERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QStringList QMimeData::formats();
  fn _ZNK9QMimeData7formatsEv(qthis: *mut c_void) ;
  // proto:  bool QMimeData::hasImage();
  fn _ZNK9QMimeData8hasImageEv(qthis: *mut c_void) -> int8_t;
}

// body block begin
// class sizeof(QMimeData)=1
pub struct QMimeData {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMimeData {
  pub fn setData<T: QMimeData_setData>(&mut self, value: T)  {
     value.setData(self);
    // return 1;
  }
}

pub trait QMimeData_setData {
  fn setData(self, rsthis: &mut QMimeData) ;
}

// proto:  void QMimeData::setData(const QString & mimetype, const QByteArray & data);
impl<'a> /*trait*/ QMimeData_setData for (&'a  QString, &'a  QByteArray) {
  fn setData(self, rsthis: &mut QMimeData)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeData7setDataERK7QStringRK10QByteArray()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN9QMimeData7setDataERK7QStringRK10QByteArray(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn colorData<T: QMimeData_colorData>(&mut self, value: T) -> QVariant {
    return value.colorData(self);
    // return 1;
  }
}

pub trait QMimeData_colorData {
  fn colorData(self, rsthis: &mut QMimeData) -> QVariant;
}

// proto:  QVariant QMimeData::colorData();
impl<'a> /*trait*/ QMimeData_colorData for () {
  fn colorData(self, rsthis: &mut QMimeData) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData9colorDataEv()};
    let mut ret = unsafe {_ZNK9QMimeData9colorDataEv(rsthis.qclsinst)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn FreeQMimeData<T: QMimeData_FreeQMimeData>(&mut self, value: T)  {
     value.FreeQMimeData(self);
    // return 1;
  }
}

pub trait QMimeData_FreeQMimeData {
  fn FreeQMimeData(self, rsthis: &mut QMimeData) ;
}

// proto:  void QMimeData::FreeQMimeData();
impl<'a> /*trait*/ QMimeData_FreeQMimeData for () {
  fn FreeQMimeData(self, rsthis: &mut QMimeData)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeDataD0Ev()};
     unsafe {_ZN9QMimeDataD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn hasHtml<T: QMimeData_hasHtml>(&mut self, value: T) -> i8 {
    return value.hasHtml(self);
    // return 1;
  }
}

pub trait QMimeData_hasHtml {
  fn hasHtml(self, rsthis: &mut QMimeData) -> i8;
}

// proto:  bool QMimeData::hasHtml();
impl<'a> /*trait*/ QMimeData_hasHtml for () {
  fn hasHtml(self, rsthis: &mut QMimeData) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData7hasHtmlEv()};
    let mut ret = unsafe {_ZNK9QMimeData7hasHtmlEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn NewQMimeData<T: QMimeData_NewQMimeData>(value: T) -> QMimeData {
    let rsthis = value.NewQMimeData();
    return rsthis;
    // return 1;
  }
}

pub trait QMimeData_NewQMimeData {
  fn NewQMimeData(self) -> QMimeData;
}

// proto: void QMimeData::NewQMimeData(const QMimeData & );
impl<'a> /*trait*/ QMimeData_NewQMimeData for (&'a  QMimeData) {
  fn NewQMimeData(self) -> QMimeData {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeDataC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QMimeDataC1ERKS_(qthis, arg0)};
    let rsthis = QMimeData{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn imageData<T: QMimeData_imageData>(&mut self, value: T) -> QVariant {
    return value.imageData(self);
    // return 1;
  }
}

pub trait QMimeData_imageData {
  fn imageData(self, rsthis: &mut QMimeData) -> QVariant;
}

// proto:  QVariant QMimeData::imageData();
impl<'a> /*trait*/ QMimeData_imageData for () {
  fn imageData(self, rsthis: &mut QMimeData) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData9imageDataEv()};
    let mut ret = unsafe {_ZNK9QMimeData9imageDataEv(rsthis.qclsinst)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn hasFormat<T: QMimeData_hasFormat>(&mut self, value: T) -> i8 {
    return value.hasFormat(self);
    // return 1;
  }
}

pub trait QMimeData_hasFormat {
  fn hasFormat(self, rsthis: &mut QMimeData) -> i8;
}

// proto:  bool QMimeData::hasFormat(const QString & mimetype);
impl<'a> /*trait*/ QMimeData_hasFormat for (&'a  QString) {
  fn hasFormat(self, rsthis: &mut QMimeData) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData9hasFormatERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QMimeData9hasFormatERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn setText<T: QMimeData_setText>(&mut self, value: T)  {
     value.setText(self);
    // return 1;
  }
}

pub trait QMimeData_setText {
  fn setText(self, rsthis: &mut QMimeData) ;
}

// proto:  void QMimeData::setText(const QString & text);
impl<'a> /*trait*/ QMimeData_setText for (&'a  QString) {
  fn setText(self, rsthis: &mut QMimeData)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeData7setTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QMimeData7setTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn clear<T: QMimeData_clear>(&mut self, value: T)  {
     value.clear(self);
    // return 1;
  }
}

pub trait QMimeData_clear {
  fn clear(self, rsthis: &mut QMimeData) ;
}

// proto:  void QMimeData::clear();
impl<'a> /*trait*/ QMimeData_clear for () {
  fn clear(self, rsthis: &mut QMimeData)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeData5clearEv()};
     unsafe {_ZN9QMimeData5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn text<T: QMimeData_text>(&mut self, value: T) -> QString {
    return value.text(self);
    // return 1;
  }
}

pub trait QMimeData_text {
  fn text(self, rsthis: &mut QMimeData) -> QString;
}

// proto:  QString QMimeData::text();
impl<'a> /*trait*/ QMimeData_text for () {
  fn text(self, rsthis: &mut QMimeData) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData4textEv()};
    let mut ret = unsafe {_ZNK9QMimeData4textEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn setHtml<T: QMimeData_setHtml>(&mut self, value: T)  {
     value.setHtml(self);
    // return 1;
  }
}

pub trait QMimeData_setHtml {
  fn setHtml(self, rsthis: &mut QMimeData) ;
}

// proto:  void QMimeData::setHtml(const QString & html);
impl<'a> /*trait*/ QMimeData_setHtml for (&'a  QString) {
  fn setHtml(self, rsthis: &mut QMimeData)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeData7setHtmlERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QMimeData7setHtmlERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn setImageData<T: QMimeData_setImageData>(&mut self, value: T)  {
     value.setImageData(self);
    // return 1;
  }
}

pub trait QMimeData_setImageData {
  fn setImageData(self, rsthis: &mut QMimeData) ;
}

// proto:  void QMimeData::setImageData(const QVariant & image);
impl<'a> /*trait*/ QMimeData_setImageData for (&'a  QVariant) {
  fn setImageData(self, rsthis: &mut QMimeData)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeData12setImageDataERK8QVariant()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QMimeData12setImageDataERK8QVariant(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn hasUrls<T: QMimeData_hasUrls>(&mut self, value: T) -> i8 {
    return value.hasUrls(self);
    // return 1;
  }
}

pub trait QMimeData_hasUrls {
  fn hasUrls(self, rsthis: &mut QMimeData) -> i8;
}

// proto:  bool QMimeData::hasUrls();
impl<'a> /*trait*/ QMimeData_hasUrls for () {
  fn hasUrls(self, rsthis: &mut QMimeData) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData7hasUrlsEv()};
    let mut ret = unsafe {_ZNK9QMimeData7hasUrlsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn hasColor<T: QMimeData_hasColor>(&mut self, value: T) -> i8 {
    return value.hasColor(self);
    // return 1;
  }
}

pub trait QMimeData_hasColor {
  fn hasColor(self, rsthis: &mut QMimeData) -> i8;
}

// proto:  bool QMimeData::hasColor();
impl<'a> /*trait*/ QMimeData_hasColor for () {
  fn hasColor(self, rsthis: &mut QMimeData) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData8hasColorEv()};
    let mut ret = unsafe {_ZNK9QMimeData8hasColorEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn removeFormat<T: QMimeData_removeFormat>(&mut self, value: T)  {
     value.removeFormat(self);
    // return 1;
  }
}

pub trait QMimeData_removeFormat {
  fn removeFormat(self, rsthis: &mut QMimeData) ;
}

// proto:  void QMimeData::removeFormat(const QString & mimetype);
impl<'a> /*trait*/ QMimeData_removeFormat for (&'a  QString) {
  fn removeFormat(self, rsthis: &mut QMimeData)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeData12removeFormatERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QMimeData12removeFormatERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn html<T: QMimeData_html>(&mut self, value: T) -> QString {
    return value.html(self);
    // return 1;
  }
}

pub trait QMimeData_html {
  fn html(self, rsthis: &mut QMimeData) -> QString;
}

// proto:  QString QMimeData::html();
impl<'a> /*trait*/ QMimeData_html for () {
  fn html(self, rsthis: &mut QMimeData) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData4htmlEv()};
    let mut ret = unsafe {_ZNK9QMimeData4htmlEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QMimeData::NewQMimeData();
impl<'a> /*trait*/ QMimeData_NewQMimeData for () {
  fn NewQMimeData(self) -> QMimeData {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeDataC1Ev()};
    unsafe {_ZN9QMimeDataC1Ev(qthis)};
    let rsthis = QMimeData{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn urls<T: QMimeData_urls>(&mut self, value: T)  {
     value.urls(self);
    // return 1;
  }
}

pub trait QMimeData_urls {
  fn urls(self, rsthis: &mut QMimeData) ;
}

// proto:  QList<QUrl> QMimeData::urls();
impl<'a> /*trait*/ QMimeData_urls for () {
  fn urls(self, rsthis: &mut QMimeData)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData4urlsEv()};
     unsafe {_ZNK9QMimeData4urlsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn setColorData<T: QMimeData_setColorData>(&mut self, value: T)  {
     value.setColorData(self);
    // return 1;
  }
}

pub trait QMimeData_setColorData {
  fn setColorData(self, rsthis: &mut QMimeData) ;
}

// proto:  void QMimeData::setColorData(const QVariant & color);
impl<'a> /*trait*/ QMimeData_setColorData for (&'a  QVariant) {
  fn setColorData(self, rsthis: &mut QMimeData)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeData12setColorDataERK8QVariant()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QMimeData12setColorDataERK8QVariant(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn hasText<T: QMimeData_hasText>(&mut self, value: T) -> i8 {
    return value.hasText(self);
    // return 1;
  }
}

pub trait QMimeData_hasText {
  fn hasText(self, rsthis: &mut QMimeData) -> i8;
}

// proto:  bool QMimeData::hasText();
impl<'a> /*trait*/ QMimeData_hasText for () {
  fn hasText(self, rsthis: &mut QMimeData) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData7hasTextEv()};
    let mut ret = unsafe {_ZNK9QMimeData7hasTextEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn metaObject<T: QMimeData_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QMimeData_metaObject {
  fn metaObject(self, rsthis: &mut QMimeData) ;
}

// proto:  const QMetaObject * QMimeData::metaObject();
impl<'a> /*trait*/ QMimeData_metaObject for () {
  fn metaObject(self, rsthis: &mut QMimeData)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData10metaObjectEv()};
     unsafe {_ZNK9QMimeData10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn data<T: QMimeData_data>(&mut self, value: T) -> QByteArray {
    return value.data(self);
    // return 1;
  }
}

pub trait QMimeData_data {
  fn data(self, rsthis: &mut QMimeData) -> QByteArray;
}

// proto:  QByteArray QMimeData::data(const QString & mimetype);
impl<'a> /*trait*/ QMimeData_data for (&'a  QString) {
  fn data(self, rsthis: &mut QMimeData) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData4dataERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QMimeData4dataERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn formats<T: QMimeData_formats>(&mut self, value: T)  {
     value.formats(self);
    // return 1;
  }
}

pub trait QMimeData_formats {
  fn formats(self, rsthis: &mut QMimeData) ;
}

// proto:  QStringList QMimeData::formats();
impl<'a> /*trait*/ QMimeData_formats for () {
  fn formats(self, rsthis: &mut QMimeData)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData7formatsEv()};
     unsafe {_ZNK9QMimeData7formatsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn hasImage<T: QMimeData_hasImage>(&mut self, value: T) -> i8 {
    return value.hasImage(self);
    // return 1;
  }
}

pub trait QMimeData_hasImage {
  fn hasImage(self, rsthis: &mut QMimeData) -> i8;
}

// proto:  bool QMimeData::hasImage();
impl<'a> /*trait*/ QMimeData_hasImage for () {
  fn hasImage(self, rsthis: &mut QMimeData) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData8hasImageEv()};
    let mut ret = unsafe {_ZNK9QMimeData8hasImageEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

