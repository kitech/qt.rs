// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstringlist::QStringList;
use super::qiodevice::QIODevice;
use super::qstring::QString;
use super::qrect::QRect;
use super::qpainter::QPainter;
use super::qpaintengine::QPaintEngine;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  const char * QPicture::data();
  fn _ZNK8QPicture4dataEv(qthis: *mut c_void) -> *const c_char;
  // proto: static QStringList QPicture::inputFormatList();
  fn _ZN8QPicture15inputFormatListEv() -> *mut c_void;
  // proto:  void QPicture::swap(QPicture & other);
  fn _ZN8QPicture4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  unsigned int QPicture::size();
  fn _ZNK8QPicture4sizeEv(qthis: *mut c_void) -> c_uint;
  // proto:  bool QPicture::isNull();
  fn _ZNK8QPicture6isNullEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QPicture::save(QIODevice * dev, const char * format);
  fn _ZN8QPicture4saveEP9QIODevicePKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *const c_char) -> int8_t;
  // proto:  void QPicture::detach();
  fn _ZN8QPicture6detachEv(qthis: *mut c_void) ;
  // proto: static QList<QByteArray> QPicture::inputFormats();
  fn _ZN8QPicture12inputFormatsEv() ;
  // proto:  void QPicture::NewQPicture(int formatVersion);
  fn _ZN8QPictureC1Ei(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QPicture::NewQPicture(const QPicture & );
  fn _ZN8QPictureC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QPicture::isDetached();
  fn _ZNK8QPicture10isDetachedEv(qthis: *mut c_void) -> int8_t;
  // proto: static QStringList QPicture::outputFormatList();
  fn _ZN8QPicture16outputFormatListEv() -> *mut c_void;
  // proto:  void QPicture::setData(const char * data, uint size);
  fn _ZN8QPicture7setDataEPKcj(qthis: *mut c_void, arg0: *const c_char, arg1: c_uint) ;
  // proto: static QList<QByteArray> QPicture::outputFormats();
  fn _ZN8QPicture13outputFormatsEv() ;
  // proto:  int QPicture::devType();
  fn _ZNK8QPicture7devTypeEv(qthis: *mut c_void) -> c_int;
  // proto: static const char * QPicture::pictureFormat(const QString & fileName);
  fn _ZN8QPicture13pictureFormatERK7QString(arg0: *mut c_void) -> *const c_char;
  // proto:  bool QPicture::save(const QString & fileName, const char * format);
  fn _ZN8QPicture4saveERK7QStringPKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *const c_char) -> int8_t;
  // proto:  bool QPicture::load(const QString & fileName, const char * format);
  fn _ZN8QPicture4loadERK7QStringPKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *const c_char) -> int8_t;
  // proto:  void QPicture::FreeQPicture();
  fn _ZN8QPictureD0Ev(qthis: *mut c_void) ;
  // proto:  void QPicture::setBoundingRect(const QRect & r);
  fn _ZN8QPicture15setBoundingRectERK5QRect(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QPicture::load(QIODevice * dev, const char * format);
  fn _ZN8QPicture4loadEP9QIODevicePKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *const c_char) -> int8_t;
  // proto:  bool QPicture::play(QPainter * p);
  fn _ZN8QPicture4playEP8QPainter(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QPaintEngine * QPicture::paintEngine();
  fn _ZNK8QPicture11paintEngineEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QPicture)=1
pub struct QPicture {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPicture {
  pub fn data<T: QPicture_data>(&mut self, value: T) -> String {
    return value.data(self);
    // return 1;
  }
}

pub trait QPicture_data {
  fn data(self, rsthis: &mut QPicture) -> String;
}

// proto:  const char * QPicture::data();
impl<'a> /*trait*/ QPicture_data for () {
  fn data(self, rsthis: &mut QPicture) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPicture4dataEv()};
    let mut ret = unsafe {_ZNK8QPicture4dataEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

impl /*struct*/ QPicture {
  pub fn inputFormatList<T: QPicture_inputFormatList>(&mut self, value: T) -> QStringList {
    return value.inputFormatList(self);
    // return 1;
  }
}

pub trait QPicture_inputFormatList {
  fn inputFormatList(self, rsthis: &mut QPicture) -> QStringList;
}

// proto: static QStringList QPicture::inputFormatList();
impl<'a> /*trait*/ QPicture_inputFormatList for () {
  fn inputFormatList(self, rsthis: &mut QPicture) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture15inputFormatListEv()};
    let mut ret = unsafe {_ZN8QPicture15inputFormatListEv()};
    let mut ret1 = QStringList{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPicture {
  pub fn swap<T: QPicture_swap>(&mut self, value: T)  {
     value.swap(self);
    // return 1;
  }
}

pub trait QPicture_swap {
  fn swap(self, rsthis: &mut QPicture) ;
}

// proto:  void QPicture::swap(QPicture & other);
impl<'a> /*trait*/ QPicture_swap for (&'a mut QPicture) {
  fn swap(self, rsthis: &mut QPicture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPicture4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPicture {
  pub fn size<T: QPicture_size>(&mut self, value: T) -> u32 {
    return value.size(self);
    // return 1;
  }
}

pub trait QPicture_size {
  fn size(self, rsthis: &mut QPicture) -> u32;
}

// proto:  unsigned int QPicture::size();
impl<'a> /*trait*/ QPicture_size for () {
  fn size(self, rsthis: &mut QPicture) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPicture4sizeEv()};
    let mut ret = unsafe {_ZNK8QPicture4sizeEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

impl /*struct*/ QPicture {
  pub fn isNull<T: QPicture_isNull>(&mut self, value: T) -> i8 {
    return value.isNull(self);
    // return 1;
  }
}

pub trait QPicture_isNull {
  fn isNull(self, rsthis: &mut QPicture) -> i8;
}

// proto:  bool QPicture::isNull();
impl<'a> /*trait*/ QPicture_isNull for () {
  fn isNull(self, rsthis: &mut QPicture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPicture6isNullEv()};
    let mut ret = unsafe {_ZNK8QPicture6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPicture {
  pub fn save<T: QPicture_save>(&mut self, value: T) -> i8 {
    return value.save(self);
    // return 1;
  }
}

pub trait QPicture_save {
  fn save(self, rsthis: &mut QPicture) -> i8;
}

// proto:  bool QPicture::save(QIODevice * dev, const char * format);
impl<'a> /*trait*/ QPicture_save for (&'a mut QIODevice, &'a  String) {
  fn save(self, rsthis: &mut QPicture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture4saveEP9QIODevicePKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    let mut ret = unsafe {_ZN8QPicture4saveEP9QIODevicePKc(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPicture {
  pub fn detach<T: QPicture_detach>(&mut self, value: T)  {
     value.detach(self);
    // return 1;
  }
}

pub trait QPicture_detach {
  fn detach(self, rsthis: &mut QPicture) ;
}

// proto:  void QPicture::detach();
impl<'a> /*trait*/ QPicture_detach for () {
  fn detach(self, rsthis: &mut QPicture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture6detachEv()};
     unsafe {_ZN8QPicture6detachEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPicture {
  pub fn inputFormats<T: QPicture_inputFormats>(&mut self, value: T)  {
     value.inputFormats(self);
    // return 1;
  }
}

pub trait QPicture_inputFormats {
  fn inputFormats(self, rsthis: &mut QPicture) ;
}

// proto: static QList<QByteArray> QPicture::inputFormats();
impl<'a> /*trait*/ QPicture_inputFormats for () {
  fn inputFormats(self, rsthis: &mut QPicture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture12inputFormatsEv()};
     unsafe {_ZN8QPicture12inputFormatsEv()};
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QPictureC1ERKS_(qthis, arg0)};
    let rsthis = QPicture{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPicture {
  pub fn isDetached<T: QPicture_isDetached>(&mut self, value: T) -> i8 {
    return value.isDetached(self);
    // return 1;
  }
}

pub trait QPicture_isDetached {
  fn isDetached(self, rsthis: &mut QPicture) -> i8;
}

// proto:  bool QPicture::isDetached();
impl<'a> /*trait*/ QPicture_isDetached for () {
  fn isDetached(self, rsthis: &mut QPicture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPicture10isDetachedEv()};
    let mut ret = unsafe {_ZNK8QPicture10isDetachedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPicture {
  pub fn outputFormatList<T: QPicture_outputFormatList>(&mut self, value: T) -> QStringList {
    return value.outputFormatList(self);
    // return 1;
  }
}

pub trait QPicture_outputFormatList {
  fn outputFormatList(self, rsthis: &mut QPicture) -> QStringList;
}

// proto: static QStringList QPicture::outputFormatList();
impl<'a> /*trait*/ QPicture_outputFormatList for () {
  fn outputFormatList(self, rsthis: &mut QPicture) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture16outputFormatListEv()};
    let mut ret = unsafe {_ZN8QPicture16outputFormatListEv()};
    let mut ret1 = QStringList{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPicture {
  pub fn setData<T: QPicture_setData>(&mut self, value: T)  {
     value.setData(self);
    // return 1;
  }
}

pub trait QPicture_setData {
  fn setData(self, rsthis: &mut QPicture) ;
}

// proto:  void QPicture::setData(const char * data, uint size);
impl<'a> /*trait*/ QPicture_setData for (&'a  String, u32) {
  fn setData(self, rsthis: &mut QPicture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture7setDataEPKcj()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_uint;
     unsafe {_ZN8QPicture7setDataEPKcj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QPicture {
  pub fn outputFormats<T: QPicture_outputFormats>(&mut self, value: T)  {
     value.outputFormats(self);
    // return 1;
  }
}

pub trait QPicture_outputFormats {
  fn outputFormats(self, rsthis: &mut QPicture) ;
}

// proto: static QList<QByteArray> QPicture::outputFormats();
impl<'a> /*trait*/ QPicture_outputFormats for () {
  fn outputFormats(self, rsthis: &mut QPicture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture13outputFormatsEv()};
     unsafe {_ZN8QPicture13outputFormatsEv()};
    // return 1;
  }
}

impl /*struct*/ QPicture {
  pub fn devType<T: QPicture_devType>(&mut self, value: T) -> i32 {
    return value.devType(self);
    // return 1;
  }
}

pub trait QPicture_devType {
  fn devType(self, rsthis: &mut QPicture) -> i32;
}

// proto:  int QPicture::devType();
impl<'a> /*trait*/ QPicture_devType for () {
  fn devType(self, rsthis: &mut QPicture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPicture7devTypeEv()};
    let mut ret = unsafe {_ZNK8QPicture7devTypeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QPicture {
  pub fn pictureFormat<T: QPicture_pictureFormat>(&mut self, value: T) -> String {
    return value.pictureFormat(self);
    // return 1;
  }
}

pub trait QPicture_pictureFormat {
  fn pictureFormat(self, rsthis: &mut QPicture) -> String;
}

// proto: static const char * QPicture::pictureFormat(const QString & fileName);
impl<'a> /*trait*/ QPicture_pictureFormat for (&'a  QString) {
  fn pictureFormat(self, rsthis: &mut QPicture) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture13pictureFormatERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QPicture13pictureFormatERK7QString(arg0)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

// proto:  bool QPicture::save(const QString & fileName, const char * format);
impl<'a> /*trait*/ QPicture_save for (&'a  QString, &'a  String) {
  fn save(self, rsthis: &mut QPicture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture4saveERK7QStringPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    let mut ret = unsafe {_ZN8QPicture4saveERK7QStringPKc(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPicture {
  pub fn load<T: QPicture_load>(&mut self, value: T) -> i8 {
    return value.load(self);
    // return 1;
  }
}

pub trait QPicture_load {
  fn load(self, rsthis: &mut QPicture) -> i8;
}

// proto:  bool QPicture::load(const QString & fileName, const char * format);
impl<'a> /*trait*/ QPicture_load for (&'a  QString, &'a  String) {
  fn load(self, rsthis: &mut QPicture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture4loadERK7QStringPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    let mut ret = unsafe {_ZN8QPicture4loadERK7QStringPKc(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPicture {
  pub fn FreeQPicture<T: QPicture_FreeQPicture>(&mut self, value: T)  {
     value.FreeQPicture(self);
    // return 1;
  }
}

pub trait QPicture_FreeQPicture {
  fn FreeQPicture(self, rsthis: &mut QPicture) ;
}

// proto:  void QPicture::FreeQPicture();
impl<'a> /*trait*/ QPicture_FreeQPicture for () {
  fn FreeQPicture(self, rsthis: &mut QPicture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPictureD0Ev()};
     unsafe {_ZN8QPictureD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPicture {
  pub fn setBoundingRect<T: QPicture_setBoundingRect>(&mut self, value: T)  {
     value.setBoundingRect(self);
    // return 1;
  }
}

pub trait QPicture_setBoundingRect {
  fn setBoundingRect(self, rsthis: &mut QPicture) ;
}

// proto:  void QPicture::setBoundingRect(const QRect & r);
impl<'a> /*trait*/ QPicture_setBoundingRect for (&'a  QRect) {
  fn setBoundingRect(self, rsthis: &mut QPicture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture15setBoundingRectERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPicture15setBoundingRectERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  bool QPicture::load(QIODevice * dev, const char * format);
impl<'a> /*trait*/ QPicture_load for (&'a mut QIODevice, &'a  String) {
  fn load(self, rsthis: &mut QPicture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture4loadEP9QIODevicePKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    let mut ret = unsafe {_ZN8QPicture4loadEP9QIODevicePKc(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPicture {
  pub fn play<T: QPicture_play>(&mut self, value: T) -> i8 {
    return value.play(self);
    // return 1;
  }
}

pub trait QPicture_play {
  fn play(self, rsthis: &mut QPicture) -> i8;
}

// proto:  bool QPicture::play(QPainter * p);
impl<'a> /*trait*/ QPicture_play for (&'a mut QPainter) {
  fn play(self, rsthis: &mut QPicture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture4playEP8QPainter()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QPicture4playEP8QPainter(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPicture {
  pub fn paintEngine<T: QPicture_paintEngine>(&mut self, value: T) -> QPaintEngine {
    return value.paintEngine(self);
    // return 1;
  }
}

pub trait QPicture_paintEngine {
  fn paintEngine(self, rsthis: &mut QPicture) -> QPaintEngine;
}

// proto:  QPaintEngine * QPicture::paintEngine();
impl<'a> /*trait*/ QPicture_paintEngine for () {
  fn paintEngine(self, rsthis: &mut QPicture) -> QPaintEngine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPicture11paintEngineEv()};
    let mut ret = unsafe {_ZNK8QPicture11paintEngineEv(rsthis.qclsinst)};
    let mut ret1 = QPaintEngine{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

