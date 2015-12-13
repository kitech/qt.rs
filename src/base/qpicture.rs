// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qiodevice::QIODevice;
use super::qstring::QString;
use super::qrect::QRect;
use super::qpainter::QPainter;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: const char * QPicture::data();
  fn _ZNK8QPicture4dataEv() -> i32;
  // proto: QStringList QPicture::inputFormatList();
  fn _ZN8QPicture15inputFormatListEv() -> i32;
  // proto: void QPicture::swap(QPicture & other);
  fn _ZN8QPicture4swapERS_(arg0: *mut c_void) -> i32;
  // proto: unsigned int QPicture::size();
  fn _ZNK8QPicture4sizeEv() -> i32;
  // proto: bool QPicture::isNull();
  fn _ZNK8QPicture6isNullEv() -> i32;
  // proto: bool QPicture::save(QIODevice * dev, const char * format);
  fn _ZN8QPicture4saveEP9QIODevicePKc(arg0: *mut c_void, arg1: *const c_char) -> i32;
  // proto: void QPicture::detach();
  fn _ZN8QPicture6detachEv() -> i32;
  // proto: QList<QByteArray> QPicture::inputFormats();
  fn _ZN8QPicture12inputFormatsEv() -> i32;
  // proto: void QPicture::NewQPicture(int formatVersion);
  fn _ZN8QPictureC1Ei(qthis: *mut c_void, arg0: c_int) -> i32;
  // proto: void QPicture::NewQPicture(const QPicture & );
  fn _ZN8QPictureC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: bool QPicture::isDetached();
  fn _ZNK8QPicture10isDetachedEv() -> i32;
  // proto: QStringList QPicture::outputFormatList();
  fn _ZN8QPicture16outputFormatListEv() -> i32;
  // proto: void QPicture::setData(const char * data, uint size);
  fn _ZN8QPicture7setDataEPKcj(arg0: *const c_char, arg1: c_uint) -> i32;
  // proto: QList<QByteArray> QPicture::outputFormats();
  fn _ZN8QPicture13outputFormatsEv() -> i32;
  // proto: int QPicture::devType();
  fn _ZNK8QPicture7devTypeEv() -> i32;
  // proto: const char * QPicture::pictureFormat(const QString & fileName);
  fn _ZN8QPicture13pictureFormatERK7QString(arg0: *const c_void) -> i32;
  // proto: bool QPicture::save(const QString & fileName, const char * format);
  fn _ZN8QPicture4saveERK7QStringPKc(arg0: *const c_void, arg1: *const c_char) -> i32;
  // proto: bool QPicture::load(const QString & fileName, const char * format);
  fn _ZN8QPicture4loadERK7QStringPKc(arg0: *const c_void, arg1: *const c_char) -> i32;
  // proto: void QPicture::FreeQPicture();
  fn _ZN8QPictureD0Ev() -> i32;
  // proto: void QPicture::setBoundingRect(const QRect & r);
  fn _ZN8QPicture15setBoundingRectERK5QRect(arg0: *const c_void) -> i32;
  // proto: bool QPicture::load(QIODevice * dev, const char * format);
  fn _ZN8QPicture4loadEP9QIODevicePKc(arg0: *mut c_void, arg1: *const c_char) -> i32;
  // proto: QRect QPicture::boundingRect();
  fn _ZNK8QPicture12boundingRectEv() -> i32;
  // proto: bool QPicture::play(QPainter * p);
  fn _ZN8QPicture4playEP8QPainter(arg0: *mut c_void) -> i32;
  // proto: QPaintEngine * QPicture::paintEngine();
  fn _ZNK8QPicture11paintEngineEv() -> i32;
}

// body block begin
// class sizeof(QPicture)=1
pub struct QPicture {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPicture {
  pub fn data<T: QPicture_data>(&mut self, value: T) -> i32 {
    value.data(self);
    return 1;
  }
}

pub trait QPicture_data {
  fn data(self, this: &mut QPicture) -> i32;
}

// proto: const char * QPicture::data();
impl<'a> /*trait*/ QPicture_data for () {
  fn data(self, this: &mut QPicture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPicture4dataEv()};
    unsafe {_ZNK8QPicture4dataEv()};
    return 1;
  }
}

impl /*struct*/ QPicture {
  pub fn inputFormatList<T: QPicture_inputFormatList>(&mut self, value: T) -> i32 {
    value.inputFormatList(self);
    return 1;
  }
}

pub trait QPicture_inputFormatList {
  fn inputFormatList(self, this: &mut QPicture) -> i32;
}

// proto: QStringList QPicture::inputFormatList();
impl<'a> /*trait*/ QPicture_inputFormatList for () {
  fn inputFormatList(self, this: &mut QPicture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture15inputFormatListEv()};
    unsafe {_ZN8QPicture15inputFormatListEv()};
    return 1;
  }
}

impl /*struct*/ QPicture {
  pub fn swap<T: QPicture_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QPicture_swap {
  fn swap(self, this: &mut QPicture) -> i32;
}

// proto: void QPicture::swap(QPicture & other);
impl<'a> /*trait*/ QPicture_swap for (&'a mut QPicture) {
  fn swap(self, this: &mut QPicture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QPicture4swapERS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QPicture {
  pub fn size<T: QPicture_size>(&mut self, value: T) -> i32 {
    value.size(self);
    return 1;
  }
}

pub trait QPicture_size {
  fn size(self, this: &mut QPicture) -> i32;
}

// proto: unsigned int QPicture::size();
impl<'a> /*trait*/ QPicture_size for () {
  fn size(self, this: &mut QPicture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPicture4sizeEv()};
    unsafe {_ZNK8QPicture4sizeEv()};
    return 1;
  }
}

impl /*struct*/ QPicture {
  pub fn isNull<T: QPicture_isNull>(&mut self, value: T) -> i32 {
    value.isNull(self);
    return 1;
  }
}

pub trait QPicture_isNull {
  fn isNull(self, this: &mut QPicture) -> i32;
}

// proto: bool QPicture::isNull();
impl<'a> /*trait*/ QPicture_isNull for () {
  fn isNull(self, this: &mut QPicture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPicture6isNullEv()};
    unsafe {_ZNK8QPicture6isNullEv()};
    return 1;
  }
}

impl /*struct*/ QPicture {
  pub fn save<T: QPicture_save>(&mut self, value: T) -> i32 {
    value.save(self);
    return 1;
  }
}

pub trait QPicture_save {
  fn save(self, this: &mut QPicture) -> i32;
}

// proto: bool QPicture::save(QIODevice * dev, const char * format);
impl<'a> /*trait*/ QPicture_save for (&'a mut QIODevice, &'a  String) {
  fn save(self, this: &mut QPicture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture4saveEP9QIODevicePKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    unsafe {_ZN8QPicture4saveEP9QIODevicePKc(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPicture {
  pub fn detach<T: QPicture_detach>(&mut self, value: T) -> i32 {
    value.detach(self);
    return 1;
  }
}

pub trait QPicture_detach {
  fn detach(self, this: &mut QPicture) -> i32;
}

// proto: void QPicture::detach();
impl<'a> /*trait*/ QPicture_detach for () {
  fn detach(self, this: &mut QPicture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture6detachEv()};
    unsafe {_ZN8QPicture6detachEv()};
    return 1;
  }
}

impl /*struct*/ QPicture {
  pub fn inputFormats<T: QPicture_inputFormats>(&mut self, value: T) -> i32 {
    value.inputFormats(self);
    return 1;
  }
}

pub trait QPicture_inputFormats {
  fn inputFormats(self, this: &mut QPicture) -> i32;
}

// proto: QList<QByteArray> QPicture::inputFormats();
impl<'a> /*trait*/ QPicture_inputFormats for () {
  fn inputFormats(self, this: &mut QPicture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture12inputFormatsEv()};
    unsafe {_ZN8QPicture12inputFormatsEv()};
    return 1;
  }
}

impl /*struct*/ QPicture {
  pub fn NewQPicture<T: QPicture_NewQPicture>(value: T) -> QPicture {
    let rsthis = value.NewQPicture();
    return rsthis;
    // return 1;
  }
}

pub trait QPicture_NewQPicture {
  fn NewQPicture(self) -> QPicture;
}

// proto: void QPicture::NewQPicture(int formatVersion);
impl<'a> /*trait*/ QPicture_NewQPicture for (i32) {
  fn NewQPicture(self) -> QPicture {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPictureC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN8QPictureC1Ei(qthis, arg0)};
    let rsthis = QPicture{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QPicture::NewQPicture(const QPicture & );
impl<'a> /*trait*/ QPicture_NewQPicture for (&'a  QPicture) {
  fn NewQPicture(self) -> QPicture {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPictureC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QPictureC1ERKS_(qthis, arg0)};
    let rsthis = QPicture{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPicture {
  pub fn isDetached<T: QPicture_isDetached>(&mut self, value: T) -> i32 {
    value.isDetached(self);
    return 1;
  }
}

pub trait QPicture_isDetached {
  fn isDetached(self, this: &mut QPicture) -> i32;
}

// proto: bool QPicture::isDetached();
impl<'a> /*trait*/ QPicture_isDetached for () {
  fn isDetached(self, this: &mut QPicture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPicture10isDetachedEv()};
    unsafe {_ZNK8QPicture10isDetachedEv()};
    return 1;
  }
}

impl /*struct*/ QPicture {
  pub fn outputFormatList<T: QPicture_outputFormatList>(&mut self, value: T) -> i32 {
    value.outputFormatList(self);
    return 1;
  }
}

pub trait QPicture_outputFormatList {
  fn outputFormatList(self, this: &mut QPicture) -> i32;
}

// proto: QStringList QPicture::outputFormatList();
impl<'a> /*trait*/ QPicture_outputFormatList for () {
  fn outputFormatList(self, this: &mut QPicture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture16outputFormatListEv()};
    unsafe {_ZN8QPicture16outputFormatListEv()};
    return 1;
  }
}

impl /*struct*/ QPicture {
  pub fn setData<T: QPicture_setData>(&mut self, value: T) -> i32 {
    value.setData(self);
    return 1;
  }
}

pub trait QPicture_setData {
  fn setData(self, this: &mut QPicture) -> i32;
}

// proto: void QPicture::setData(const char * data, uint size);
impl<'a> /*trait*/ QPicture_setData for (&'a  String, u32) {
  fn setData(self, this: &mut QPicture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture7setDataEPKcj()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_uint;
    unsafe {_ZN8QPicture7setDataEPKcj(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPicture {
  pub fn outputFormats<T: QPicture_outputFormats>(&mut self, value: T) -> i32 {
    value.outputFormats(self);
    return 1;
  }
}

pub trait QPicture_outputFormats {
  fn outputFormats(self, this: &mut QPicture) -> i32;
}

// proto: QList<QByteArray> QPicture::outputFormats();
impl<'a> /*trait*/ QPicture_outputFormats for () {
  fn outputFormats(self, this: &mut QPicture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture13outputFormatsEv()};
    unsafe {_ZN8QPicture13outputFormatsEv()};
    return 1;
  }
}

impl /*struct*/ QPicture {
  pub fn devType<T: QPicture_devType>(&mut self, value: T) -> i32 {
    value.devType(self);
    return 1;
  }
}

pub trait QPicture_devType {
  fn devType(self, this: &mut QPicture) -> i32;
}

// proto: int QPicture::devType();
impl<'a> /*trait*/ QPicture_devType for () {
  fn devType(self, this: &mut QPicture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPicture7devTypeEv()};
    unsafe {_ZNK8QPicture7devTypeEv()};
    return 1;
  }
}

impl /*struct*/ QPicture {
  pub fn pictureFormat<T: QPicture_pictureFormat>(&mut self, value: T) -> i32 {
    value.pictureFormat(self);
    return 1;
  }
}

pub trait QPicture_pictureFormat {
  fn pictureFormat(self, this: &mut QPicture) -> i32;
}

// proto: const char * QPicture::pictureFormat(const QString & fileName);
impl<'a> /*trait*/ QPicture_pictureFormat for (&'a  QString) {
  fn pictureFormat(self, this: &mut QPicture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture13pictureFormatERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QPicture13pictureFormatERK7QString(arg0)};
    return 1;
  }
}

// proto: bool QPicture::save(const QString & fileName, const char * format);
impl<'a> /*trait*/ QPicture_save for (&'a  QString, &'a  String) {
  fn save(self, this: &mut QPicture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture4saveERK7QStringPKc()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    unsafe {_ZN8QPicture4saveERK7QStringPKc(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPicture {
  pub fn load<T: QPicture_load>(&mut self, value: T) -> i32 {
    value.load(self);
    return 1;
  }
}

pub trait QPicture_load {
  fn load(self, this: &mut QPicture) -> i32;
}

// proto: bool QPicture::load(const QString & fileName, const char * format);
impl<'a> /*trait*/ QPicture_load for (&'a  QString, &'a  String) {
  fn load(self, this: &mut QPicture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture4loadERK7QStringPKc()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    unsafe {_ZN8QPicture4loadERK7QStringPKc(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPicture {
  pub fn FreeQPicture<T: QPicture_FreeQPicture>(&mut self, value: T) -> i32 {
    value.FreeQPicture(self);
    return 1;
  }
}

pub trait QPicture_FreeQPicture {
  fn FreeQPicture(self, this: &mut QPicture) -> i32;
}

// proto: void QPicture::FreeQPicture();
impl<'a> /*trait*/ QPicture_FreeQPicture for () {
  fn FreeQPicture(self, this: &mut QPicture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPictureD0Ev()};
    unsafe {_ZN8QPictureD0Ev()};
    return 1;
  }
}

impl /*struct*/ QPicture {
  pub fn setBoundingRect<T: QPicture_setBoundingRect>(&mut self, value: T) -> i32 {
    value.setBoundingRect(self);
    return 1;
  }
}

pub trait QPicture_setBoundingRect {
  fn setBoundingRect(self, this: &mut QPicture) -> i32;
}

// proto: void QPicture::setBoundingRect(const QRect & r);
impl<'a> /*trait*/ QPicture_setBoundingRect for (&'a  QRect) {
  fn setBoundingRect(self, this: &mut QPicture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture15setBoundingRectERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QPicture15setBoundingRectERK5QRect(arg0)};
    return 1;
  }
}

// proto: bool QPicture::load(QIODevice * dev, const char * format);
impl<'a> /*trait*/ QPicture_load for (&'a mut QIODevice, &'a  String) {
  fn load(self, this: &mut QPicture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture4loadEP9QIODevicePKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    unsafe {_ZN8QPicture4loadEP9QIODevicePKc(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPicture {
  pub fn boundingRect<T: QPicture_boundingRect>(&mut self, value: T) -> i32 {
    value.boundingRect(self);
    return 1;
  }
}

pub trait QPicture_boundingRect {
  fn boundingRect(self, this: &mut QPicture) -> i32;
}

// proto: QRect QPicture::boundingRect();
impl<'a> /*trait*/ QPicture_boundingRect for () {
  fn boundingRect(self, this: &mut QPicture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPicture12boundingRectEv()};
    unsafe {_ZNK8QPicture12boundingRectEv()};
    return 1;
  }
}

impl /*struct*/ QPicture {
  pub fn play<T: QPicture_play>(&mut self, value: T) -> i32 {
    value.play(self);
    return 1;
  }
}

pub trait QPicture_play {
  fn play(self, this: &mut QPicture) -> i32;
}

// proto: bool QPicture::play(QPainter * p);
impl<'a> /*trait*/ QPicture_play for (&'a mut QPainter) {
  fn play(self, this: &mut QPicture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture4playEP8QPainter()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QPicture4playEP8QPainter(arg0)};
    return 1;
  }
}

impl /*struct*/ QPicture {
  pub fn paintEngine<T: QPicture_paintEngine>(&mut self, value: T) -> i32 {
    value.paintEngine(self);
    return 1;
  }
}

pub trait QPicture_paintEngine {
  fn paintEngine(self, this: &mut QPicture) -> i32;
}

// proto: QPaintEngine * QPicture::paintEngine();
impl<'a> /*trait*/ QPicture_paintEngine for () {
  fn paintEngine(self, this: &mut QPicture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPicture11paintEngineEv()};
    unsafe {_ZNK8QPicture11paintEngineEv()};
    return 1;
  }
}

