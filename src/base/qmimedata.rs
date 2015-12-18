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
  pub fn setData<RetType, T: QMimeData_setData<RetType>>(&mut self, value: T) -> RetType {
    return value.setData(self);
    // return 1;
  }
}

pub trait QMimeData_setData<RetType> {
  fn setData(self, rsthis: &mut QMimeData) -> RetType;
}

// proto:  void QMimeData::setData(const QString & mimetype, const QByteArray & data);
impl<'a> /*trait*/ QMimeData_setData<()> for (&'a  QString, &'a  QByteArray) {
  fn setData(self, rsthis: &mut QMimeData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeData7setDataERK7QStringRK10QByteArray()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN9QMimeData7setDataERK7QStringRK10QByteArray(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn colorData<RetType, T: QMimeData_colorData<RetType>>(&mut self, value: T) -> RetType {
    return value.colorData(self);
    // return 1;
  }
}

pub trait QMimeData_colorData<RetType> {
  fn colorData(self, rsthis: &mut QMimeData) -> RetType;
}

// proto:  QVariant QMimeData::colorData();
impl<'a> /*trait*/ QMimeData_colorData<QVariant> for () {
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
  pub fn FreeQMimeData<RetType, T: QMimeData_FreeQMimeData<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQMimeData(self);
    // return 1;
  }
}

pub trait QMimeData_FreeQMimeData<RetType> {
  fn FreeQMimeData(self, rsthis: &mut QMimeData) -> RetType;
}

// proto:  void QMimeData::FreeQMimeData();
impl<'a> /*trait*/ QMimeData_FreeQMimeData<()> for () {
  fn FreeQMimeData(self, rsthis: &mut QMimeData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeDataD0Ev()};
     unsafe {_ZN9QMimeDataD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn hasHtml<RetType, T: QMimeData_hasHtml<RetType>>(&mut self, value: T) -> RetType {
    return value.hasHtml(self);
    // return 1;
  }
}

pub trait QMimeData_hasHtml<RetType> {
  fn hasHtml(self, rsthis: &mut QMimeData) -> RetType;
}

// proto:  bool QMimeData::hasHtml();
impl<'a> /*trait*/ QMimeData_hasHtml<i8> for () {
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
  pub fn imageData<RetType, T: QMimeData_imageData<RetType>>(&mut self, value: T) -> RetType {
    return value.imageData(self);
    // return 1;
  }
}

pub trait QMimeData_imageData<RetType> {
  fn imageData(self, rsthis: &mut QMimeData) -> RetType;
}

// proto:  QVariant QMimeData::imageData();
impl<'a> /*trait*/ QMimeData_imageData<QVariant> for () {
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
  pub fn hasFormat<RetType, T: QMimeData_hasFormat<RetType>>(&mut self, value: T) -> RetType {
    return value.hasFormat(self);
    // return 1;
  }
}

pub trait QMimeData_hasFormat<RetType> {
  fn hasFormat(self, rsthis: &mut QMimeData) -> RetType;
}

// proto:  bool QMimeData::hasFormat(const QString & mimetype);
impl<'a> /*trait*/ QMimeData_hasFormat<i8> for (&'a  QString) {
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
  pub fn setText<RetType, T: QMimeData_setText<RetType>>(&mut self, value: T) -> RetType {
    return value.setText(self);
    // return 1;
  }
}

pub trait QMimeData_setText<RetType> {
  fn setText(self, rsthis: &mut QMimeData) -> RetType;
}

// proto:  void QMimeData::setText(const QString & text);
impl<'a> /*trait*/ QMimeData_setText<()> for (&'a  QString) {
  fn setText(self, rsthis: &mut QMimeData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeData7setTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QMimeData7setTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn clear<RetType, T: QMimeData_clear<RetType>>(&mut self, value: T) -> RetType {
    return value.clear(self);
    // return 1;
  }
}

pub trait QMimeData_clear<RetType> {
  fn clear(self, rsthis: &mut QMimeData) -> RetType;
}

// proto:  void QMimeData::clear();
impl<'a> /*trait*/ QMimeData_clear<()> for () {
  fn clear(self, rsthis: &mut QMimeData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeData5clearEv()};
     unsafe {_ZN9QMimeData5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn text<RetType, T: QMimeData_text<RetType>>(&mut self, value: T) -> RetType {
    return value.text(self);
    // return 1;
  }
}

pub trait QMimeData_text<RetType> {
  fn text(self, rsthis: &mut QMimeData) -> RetType;
}

// proto:  QString QMimeData::text();
impl<'a> /*trait*/ QMimeData_text<QString> for () {
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
  pub fn setHtml<RetType, T: QMimeData_setHtml<RetType>>(&mut self, value: T) -> RetType {
    return value.setHtml(self);
    // return 1;
  }
}

pub trait QMimeData_setHtml<RetType> {
  fn setHtml(self, rsthis: &mut QMimeData) -> RetType;
}

// proto:  void QMimeData::setHtml(const QString & html);
impl<'a> /*trait*/ QMimeData_setHtml<()> for (&'a  QString) {
  fn setHtml(self, rsthis: &mut QMimeData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeData7setHtmlERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QMimeData7setHtmlERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn setImageData<RetType, T: QMimeData_setImageData<RetType>>(&mut self, value: T) -> RetType {
    return value.setImageData(self);
    // return 1;
  }
}

pub trait QMimeData_setImageData<RetType> {
  fn setImageData(self, rsthis: &mut QMimeData) -> RetType;
}

// proto:  void QMimeData::setImageData(const QVariant & image);
impl<'a> /*trait*/ QMimeData_setImageData<()> for (&'a  QVariant) {
  fn setImageData(self, rsthis: &mut QMimeData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeData12setImageDataERK8QVariant()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QMimeData12setImageDataERK8QVariant(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn hasUrls<RetType, T: QMimeData_hasUrls<RetType>>(&mut self, value: T) -> RetType {
    return value.hasUrls(self);
    // return 1;
  }
}

pub trait QMimeData_hasUrls<RetType> {
  fn hasUrls(self, rsthis: &mut QMimeData) -> RetType;
}

// proto:  bool QMimeData::hasUrls();
impl<'a> /*trait*/ QMimeData_hasUrls<i8> for () {
  fn hasUrls(self, rsthis: &mut QMimeData) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData7hasUrlsEv()};
    let mut ret = unsafe {_ZNK9QMimeData7hasUrlsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn hasColor<RetType, T: QMimeData_hasColor<RetType>>(&mut self, value: T) -> RetType {
    return value.hasColor(self);
    // return 1;
  }
}

pub trait QMimeData_hasColor<RetType> {
  fn hasColor(self, rsthis: &mut QMimeData) -> RetType;
}

// proto:  bool QMimeData::hasColor();
impl<'a> /*trait*/ QMimeData_hasColor<i8> for () {
  fn hasColor(self, rsthis: &mut QMimeData) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData8hasColorEv()};
    let mut ret = unsafe {_ZNK9QMimeData8hasColorEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn removeFormat<RetType, T: QMimeData_removeFormat<RetType>>(&mut self, value: T) -> RetType {
    return value.removeFormat(self);
    // return 1;
  }
}

pub trait QMimeData_removeFormat<RetType> {
  fn removeFormat(self, rsthis: &mut QMimeData) -> RetType;
}

// proto:  void QMimeData::removeFormat(const QString & mimetype);
impl<'a> /*trait*/ QMimeData_removeFormat<()> for (&'a  QString) {
  fn removeFormat(self, rsthis: &mut QMimeData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeData12removeFormatERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QMimeData12removeFormatERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn html<RetType, T: QMimeData_html<RetType>>(&mut self, value: T) -> RetType {
    return value.html(self);
    // return 1;
  }
}

pub trait QMimeData_html<RetType> {
  fn html(self, rsthis: &mut QMimeData) -> RetType;
}

// proto:  QString QMimeData::html();
impl<'a> /*trait*/ QMimeData_html<QString> for () {
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
  pub fn urls<RetType, T: QMimeData_urls<RetType>>(&mut self, value: T) -> RetType {
    return value.urls(self);
    // return 1;
  }
}

pub trait QMimeData_urls<RetType> {
  fn urls(self, rsthis: &mut QMimeData) -> RetType;
}

// proto:  QList<QUrl> QMimeData::urls();
impl<'a> /*trait*/ QMimeData_urls<()> for () {
  fn urls(self, rsthis: &mut QMimeData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData4urlsEv()};
     unsafe {_ZNK9QMimeData4urlsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn setColorData<RetType, T: QMimeData_setColorData<RetType>>(&mut self, value: T) -> RetType {
    return value.setColorData(self);
    // return 1;
  }
}

pub trait QMimeData_setColorData<RetType> {
  fn setColorData(self, rsthis: &mut QMimeData) -> RetType;
}

// proto:  void QMimeData::setColorData(const QVariant & color);
impl<'a> /*trait*/ QMimeData_setColorData<()> for (&'a  QVariant) {
  fn setColorData(self, rsthis: &mut QMimeData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeData12setColorDataERK8QVariant()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QMimeData12setColorDataERK8QVariant(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn hasText<RetType, T: QMimeData_hasText<RetType>>(&mut self, value: T) -> RetType {
    return value.hasText(self);
    // return 1;
  }
}

pub trait QMimeData_hasText<RetType> {
  fn hasText(self, rsthis: &mut QMimeData) -> RetType;
}

// proto:  bool QMimeData::hasText();
impl<'a> /*trait*/ QMimeData_hasText<i8> for () {
  fn hasText(self, rsthis: &mut QMimeData) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData7hasTextEv()};
    let mut ret = unsafe {_ZNK9QMimeData7hasTextEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn metaObject<RetType, T: QMimeData_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QMimeData_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QMimeData) -> RetType;
}

// proto:  const QMetaObject * QMimeData::metaObject();
impl<'a> /*trait*/ QMimeData_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QMimeData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData10metaObjectEv()};
     unsafe {_ZNK9QMimeData10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn data<RetType, T: QMimeData_data<RetType>>(&mut self, value: T) -> RetType {
    return value.data(self);
    // return 1;
  }
}

pub trait QMimeData_data<RetType> {
  fn data(self, rsthis: &mut QMimeData) -> RetType;
}

// proto:  QByteArray QMimeData::data(const QString & mimetype);
impl<'a> /*trait*/ QMimeData_data<QByteArray> for (&'a  QString) {
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
  pub fn formats<RetType, T: QMimeData_formats<RetType>>(&mut self, value: T) -> RetType {
    return value.formats(self);
    // return 1;
  }
}

pub trait QMimeData_formats<RetType> {
  fn formats(self, rsthis: &mut QMimeData) -> RetType;
}

// proto:  QStringList QMimeData::formats();
impl<'a> /*trait*/ QMimeData_formats<()> for () {
  fn formats(self, rsthis: &mut QMimeData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData7formatsEv()};
     unsafe {_ZNK9QMimeData7formatsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMimeData {
  pub fn hasImage<RetType, T: QMimeData_hasImage<RetType>>(&mut self, value: T) -> RetType {
    return value.hasImage(self);
    // return 1;
  }
}

pub trait QMimeData_hasImage<RetType> {
  fn hasImage(self, rsthis: &mut QMimeData) -> RetType;
}

// proto:  bool QMimeData::hasImage();
impl<'a> /*trait*/ QMimeData_hasImage<i8> for () {
  fn hasImage(self, rsthis: &mut QMimeData) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeData8hasImageEv()};
    let mut ret = unsafe {_ZNK9QMimeData8hasImageEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

