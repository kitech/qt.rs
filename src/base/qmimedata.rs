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
  fn _ZN9QMimeData7setDataERK7QStringRK10QByteArray(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZNK9QMimeData9colorDataEv() -> i32;
  fn _ZN9QMimeDataD0Ev() -> i32;
  fn _ZNK9QMimeData7hasHtmlEv() -> i32;
  fn _ZN9QMimeDataC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK9QMimeData9imageDataEv() -> i32;
  fn _ZNK9QMimeData9hasFormatERK7QString(arg0: *const c_void) -> i32;
  fn _ZN9QMimeData7setTextERK7QString(arg0: *const c_void) -> i32;
  fn _ZN9QMimeData5clearEv() -> i32;
  fn _ZNK9QMimeData4textEv() -> i32;
  fn _ZN9QMimeData7setHtmlERK7QString(arg0: *const c_void) -> i32;
  fn _ZN9QMimeData12setImageDataERK8QVariant(arg0: *const c_void) -> i32;
  fn _ZNK9QMimeData7hasUrlsEv() -> i32;
  fn _ZNK9QMimeData8hasColorEv() -> i32;
  fn _ZN9QMimeData12removeFormatERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK9QMimeData4htmlEv() -> i32;
  fn _ZN9QMimeDataC1Ev(qthis: *mut c_void) -> i32;
  fn _ZNK9QMimeData4urlsEv() -> i32;
  fn _ZN9QMimeData12setColorDataERK8QVariant(arg0: *const c_void) -> i32;
  fn _ZNK9QMimeData7hasTextEv() -> i32;
  fn _ZNK9QMimeData10metaObjectEv() -> i32;
  fn _ZNK9QMimeData4dataERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK9QMimeData7formatsEv() -> i32;
  fn _ZNK9QMimeData8hasImageEv() -> i32;
}

// body block begin
// class sizeof(QMimeData)=1
pub struct QMimeData {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMimeData {
  pub fn setData<T: QMimeData_setData>(&mut self, value: T) -> i32 {
    value.setData(self);
    return 1;
  }
}

pub trait QMimeData_setData {
  fn setData(self, this: &mut QMimeData) -> i32;
}

// proto: void QMimeData::setData(const QString & mimetype, const QByteArray & data);
impl<'a> /*trait*/ QMimeData_setData for (&'a  QString, &'a  QByteArray) {
  fn setData(self, this: &mut QMimeData) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeData7setDataERK7QStringRK10QByteArray()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN9QMimeData7setDataERK7QStringRK10QByteArray(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn colorData<T: QMimeData_colorData>(&mut self, value: T) -> i32 {
    value.colorData(self);
    return 1;
  }
}

pub trait QMimeData_colorData {
  fn colorData(self, this: &mut QMimeData) -> i32;
}

// proto: QVariant QMimeData::colorData();
impl<'a> /*trait*/ QMimeData_colorData for () {
  fn colorData(self, this: &mut QMimeData) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData9colorDataEv()};
    unsafe {_ZNK9QMimeData9colorDataEv()};
    return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn FreeQMimeData<T: QMimeData_FreeQMimeData>(&mut self, value: T) -> i32 {
    value.FreeQMimeData(self);
    return 1;
  }
}

pub trait QMimeData_FreeQMimeData {
  fn FreeQMimeData(self, this: &mut QMimeData) -> i32;
}

// proto: void QMimeData::FreeQMimeData();
impl<'a> /*trait*/ QMimeData_FreeQMimeData for () {
  fn FreeQMimeData(self, this: &mut QMimeData) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeDataD0Ev()};
    unsafe {_ZN9QMimeDataD0Ev()};
    return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn hasHtml<T: QMimeData_hasHtml>(&mut self, value: T) -> i32 {
    value.hasHtml(self);
    return 1;
  }
}

pub trait QMimeData_hasHtml {
  fn hasHtml(self, this: &mut QMimeData) -> i32;
}

// proto: bool QMimeData::hasHtml();
impl<'a> /*trait*/ QMimeData_hasHtml for () {
  fn hasHtml(self, this: &mut QMimeData) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData7hasHtmlEv()};
    unsafe {_ZNK9QMimeData7hasHtmlEv()};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QMimeDataC1ERKS_(qthis, arg0)};
    let rsthis = QMimeData{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn imageData<T: QMimeData_imageData>(&mut self, value: T) -> i32 {
    value.imageData(self);
    return 1;
  }
}

pub trait QMimeData_imageData {
  fn imageData(self, this: &mut QMimeData) -> i32;
}

// proto: QVariant QMimeData::imageData();
impl<'a> /*trait*/ QMimeData_imageData for () {
  fn imageData(self, this: &mut QMimeData) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData9imageDataEv()};
    unsafe {_ZNK9QMimeData9imageDataEv()};
    return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn hasFormat<T: QMimeData_hasFormat>(&mut self, value: T) -> i32 {
    value.hasFormat(self);
    return 1;
  }
}

pub trait QMimeData_hasFormat {
  fn hasFormat(self, this: &mut QMimeData) -> i32;
}

// proto: bool QMimeData::hasFormat(const QString & mimetype);
impl<'a> /*trait*/ QMimeData_hasFormat for (&'a  QString) {
  fn hasFormat(self, this: &mut QMimeData) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData9hasFormatERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK9QMimeData9hasFormatERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn setText<T: QMimeData_setText>(&mut self, value: T) -> i32 {
    value.setText(self);
    return 1;
  }
}

pub trait QMimeData_setText {
  fn setText(self, this: &mut QMimeData) -> i32;
}

// proto: void QMimeData::setText(const QString & text);
impl<'a> /*trait*/ QMimeData_setText for (&'a  QString) {
  fn setText(self, this: &mut QMimeData) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeData7setTextERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QMimeData7setTextERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn clear<T: QMimeData_clear>(&mut self, value: T) -> i32 {
    value.clear(self);
    return 1;
  }
}

pub trait QMimeData_clear {
  fn clear(self, this: &mut QMimeData) -> i32;
}

// proto: void QMimeData::clear();
impl<'a> /*trait*/ QMimeData_clear for () {
  fn clear(self, this: &mut QMimeData) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeData5clearEv()};
    unsafe {_ZN9QMimeData5clearEv()};
    return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn text<T: QMimeData_text>(&mut self, value: T) -> i32 {
    value.text(self);
    return 1;
  }
}

pub trait QMimeData_text {
  fn text(self, this: &mut QMimeData) -> i32;
}

// proto: QString QMimeData::text();
impl<'a> /*trait*/ QMimeData_text for () {
  fn text(self, this: &mut QMimeData) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData4textEv()};
    unsafe {_ZNK9QMimeData4textEv()};
    return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn setHtml<T: QMimeData_setHtml>(&mut self, value: T) -> i32 {
    value.setHtml(self);
    return 1;
  }
}

pub trait QMimeData_setHtml {
  fn setHtml(self, this: &mut QMimeData) -> i32;
}

// proto: void QMimeData::setHtml(const QString & html);
impl<'a> /*trait*/ QMimeData_setHtml for (&'a  QString) {
  fn setHtml(self, this: &mut QMimeData) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeData7setHtmlERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QMimeData7setHtmlERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn setImageData<T: QMimeData_setImageData>(&mut self, value: T) -> i32 {
    value.setImageData(self);
    return 1;
  }
}

pub trait QMimeData_setImageData {
  fn setImageData(self, this: &mut QMimeData) -> i32;
}

// proto: void QMimeData::setImageData(const QVariant & image);
impl<'a> /*trait*/ QMimeData_setImageData for (&'a  QVariant) {
  fn setImageData(self, this: &mut QMimeData) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeData12setImageDataERK8QVariant()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QMimeData12setImageDataERK8QVariant(arg0)};
    return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn hasUrls<T: QMimeData_hasUrls>(&mut self, value: T) -> i32 {
    value.hasUrls(self);
    return 1;
  }
}

pub trait QMimeData_hasUrls {
  fn hasUrls(self, this: &mut QMimeData) -> i32;
}

// proto: bool QMimeData::hasUrls();
impl<'a> /*trait*/ QMimeData_hasUrls for () {
  fn hasUrls(self, this: &mut QMimeData) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData7hasUrlsEv()};
    unsafe {_ZNK9QMimeData7hasUrlsEv()};
    return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn hasColor<T: QMimeData_hasColor>(&mut self, value: T) -> i32 {
    value.hasColor(self);
    return 1;
  }
}

pub trait QMimeData_hasColor {
  fn hasColor(self, this: &mut QMimeData) -> i32;
}

// proto: bool QMimeData::hasColor();
impl<'a> /*trait*/ QMimeData_hasColor for () {
  fn hasColor(self, this: &mut QMimeData) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData8hasColorEv()};
    unsafe {_ZNK9QMimeData8hasColorEv()};
    return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn removeFormat<T: QMimeData_removeFormat>(&mut self, value: T) -> i32 {
    value.removeFormat(self);
    return 1;
  }
}

pub trait QMimeData_removeFormat {
  fn removeFormat(self, this: &mut QMimeData) -> i32;
}

// proto: void QMimeData::removeFormat(const QString & mimetype);
impl<'a> /*trait*/ QMimeData_removeFormat for (&'a  QString) {
  fn removeFormat(self, this: &mut QMimeData) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeData12removeFormatERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QMimeData12removeFormatERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn html<T: QMimeData_html>(&mut self, value: T) -> i32 {
    value.html(self);
    return 1;
  }
}

pub trait QMimeData_html {
  fn html(self, this: &mut QMimeData) -> i32;
}

// proto: QString QMimeData::html();
impl<'a> /*trait*/ QMimeData_html for () {
  fn html(self, this: &mut QMimeData) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData4htmlEv()};
    unsafe {_ZNK9QMimeData4htmlEv()};
    return 1;
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
  pub fn urls<T: QMimeData_urls>(&mut self, value: T) -> i32 {
    value.urls(self);
    return 1;
  }
}

pub trait QMimeData_urls {
  fn urls(self, this: &mut QMimeData) -> i32;
}

// proto: QList<QUrl> QMimeData::urls();
impl<'a> /*trait*/ QMimeData_urls for () {
  fn urls(self, this: &mut QMimeData) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData4urlsEv()};
    unsafe {_ZNK9QMimeData4urlsEv()};
    return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn setColorData<T: QMimeData_setColorData>(&mut self, value: T) -> i32 {
    value.setColorData(self);
    return 1;
  }
}

pub trait QMimeData_setColorData {
  fn setColorData(self, this: &mut QMimeData) -> i32;
}

// proto: void QMimeData::setColorData(const QVariant & color);
impl<'a> /*trait*/ QMimeData_setColorData for (&'a  QVariant) {
  fn setColorData(self, this: &mut QMimeData) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeData12setColorDataERK8QVariant()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QMimeData12setColorDataERK8QVariant(arg0)};
    return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn hasText<T: QMimeData_hasText>(&mut self, value: T) -> i32 {
    value.hasText(self);
    return 1;
  }
}

pub trait QMimeData_hasText {
  fn hasText(self, this: &mut QMimeData) -> i32;
}

// proto: bool QMimeData::hasText();
impl<'a> /*trait*/ QMimeData_hasText for () {
  fn hasText(self, this: &mut QMimeData) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData7hasTextEv()};
    unsafe {_ZNK9QMimeData7hasTextEv()};
    return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn metaObject<T: QMimeData_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QMimeData_metaObject {
  fn metaObject(self, this: &mut QMimeData) -> i32;
}

// proto: const QMetaObject * QMimeData::metaObject();
impl<'a> /*trait*/ QMimeData_metaObject for () {
  fn metaObject(self, this: &mut QMimeData) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData10metaObjectEv()};
    unsafe {_ZNK9QMimeData10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn data<T: QMimeData_data>(&mut self, value: T) -> i32 {
    value.data(self);
    return 1;
  }
}

pub trait QMimeData_data {
  fn data(self, this: &mut QMimeData) -> i32;
}

// proto: QByteArray QMimeData::data(const QString & mimetype);
impl<'a> /*trait*/ QMimeData_data for (&'a  QString) {
  fn data(self, this: &mut QMimeData) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData4dataERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK9QMimeData4dataERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn formats<T: QMimeData_formats>(&mut self, value: T) -> i32 {
    value.formats(self);
    return 1;
  }
}

pub trait QMimeData_formats {
  fn formats(self, this: &mut QMimeData) -> i32;
}

// proto: QStringList QMimeData::formats();
impl<'a> /*trait*/ QMimeData_formats for () {
  fn formats(self, this: &mut QMimeData) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData7formatsEv()};
    unsafe {_ZNK9QMimeData7formatsEv()};
    return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn hasImage<T: QMimeData_hasImage>(&mut self, value: T) -> i32 {
    value.hasImage(self);
    return 1;
  }
}

pub trait QMimeData_hasImage {
  fn hasImage(self, this: &mut QMimeData) -> i32;
}

// proto: bool QMimeData::hasImage();
impl<'a> /*trait*/ QMimeData_hasImage for () {
  fn hasImage(self, this: &mut QMimeData) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData8hasImageEv()};
    unsafe {_ZNK9QMimeData8hasImageEv()};
    return 1;
  }
}

