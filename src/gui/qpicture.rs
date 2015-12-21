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
use super::qpaintengine::QPaintEngine;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  const char * QPicture::data();
  fn _ZNK8QPicture4dataEv(qthis: *mut c_void) -> *mut c_char;
  // proto: static QStringList QPicture::inputFormatList();
  fn _ZN8QPicture15inputFormatListEv();
  // proto:  void QPicture::swap(QPicture & other);
  fn _ZN8QPicture4swapERS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  uint QPicture::size();
  fn _ZNK8QPicture4sizeEv(qthis: *mut c_void) -> c_uint;
  // proto:  bool QPicture::isNull();
  fn _ZNK8QPicture6isNullEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QPicture::save(QIODevice * dev, const char * format);
  fn _ZN8QPicture4saveEP9QIODevicePKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_char) -> c_char;
  // proto:  void QPicture::detach();
  fn _ZN8QPicture6detachEv(qthis: *mut c_void);
  // proto: static QList<QByteArray> QPicture::inputFormats();
  fn _ZN8QPicture12inputFormatsEv();
  // proto:  void QPicture::QPicture(int formatVersion);
  fn _ZN8QPictureC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QPicture::QPicture(const QPicture & );
  fn _ZN8QPictureC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QPicture::isDetached();
  fn _ZNK8QPicture10isDetachedEv(qthis: *mut c_void) -> c_char;
  // proto: static QStringList QPicture::outputFormatList();
  fn _ZN8QPicture16outputFormatListEv();
  // proto:  void QPicture::setData(const char * data, uint size);
  fn _ZN8QPicture7setDataEPKcj(qthis: *mut c_void, arg0: *mut c_char, arg1: c_uint);
  // proto: static QList<QByteArray> QPicture::outputFormats();
  fn _ZN8QPicture13outputFormatsEv();
  // proto:  int QPicture::devType();
  fn _ZNK8QPicture7devTypeEv(qthis: *mut c_void) -> c_int;
  // proto: static const char * QPicture::pictureFormat(const QString & fileName);
  fn _ZN8QPicture13pictureFormatERK7QString(arg0: *mut c_void) -> *mut c_char;
  // proto:  bool QPicture::save(const QString & fileName, const char * format);
  fn _ZN8QPicture4saveERK7QStringPKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_char) -> c_char;
  // proto:  bool QPicture::load(const QString & fileName, const char * format);
  fn _ZN8QPicture4loadERK7QStringPKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_char) -> c_char;
  // proto:  void QPicture::~QPicture();
  fn _ZN8QPictureD0Ev(qthis: *mut c_void);
  // proto:  void QPicture::setBoundingRect(const QRect & r);
  fn _ZN8QPicture15setBoundingRectERK5QRect(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QPicture::load(QIODevice * dev, const char * format);
  fn _ZN8QPicture4loadEP9QIODevicePKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_char) -> c_char;
  // proto:  QRect QPicture::boundingRect();
  fn _ZNK8QPicture12boundingRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QPicture::play(QPainter * p);
  fn _ZN8QPicture4playEP8QPainter(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  QPaintEngine * QPicture::paintEngine();
  fn _ZNK8QPicture11paintEngineEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QPicture)=1
pub struct QPicture {
  pub qclsinst: *mut c_void,
}

  // proto:  const char * QPicture::data();
impl /*struct*/ QPicture {
  pub fn data<RetType, T: QPicture_data<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QPicture_data<RetType> {
  fn data(self , rsthis: &mut QPicture) -> RetType;
}

  // proto:  const char * QPicture::data();
impl<'a> /*trait*/ QPicture_data<String> for () {
  fn data(self , rsthis: &mut QPicture) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPicture4dataEv()};
    let mut ret = unsafe {_ZNK8QPicture4dataEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto: static QStringList QPicture::inputFormatList();
impl /*struct*/ QPicture {
  pub fn inputFormatList_s<RetType, T: QPicture_inputFormatList_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.inputFormatList_s();
    // return 1;
  }
}

pub trait QPicture_inputFormatList_s<RetType> {
  fn inputFormatList_s(self ) -> RetType;
}

  // proto: static QStringList QPicture::inputFormatList();
impl<'a> /*trait*/ QPicture_inputFormatList_s<()> for () {
  fn inputFormatList_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture15inputFormatListEv()};
     unsafe {_ZN8QPicture15inputFormatListEv()};
    // return 1;
  }
}

  // proto:  void QPicture::swap(QPicture & other);
impl /*struct*/ QPicture {
  pub fn swap<RetType, T: QPicture_swap<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QPicture_swap<RetType> {
  fn swap(self , rsthis: &mut QPicture) -> RetType;
}

  // proto:  void QPicture::swap(QPicture & other);
impl<'a> /*trait*/ QPicture_swap<()> for (QPicture) {
  fn swap(self , rsthis: &mut QPicture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPicture4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  uint QPicture::size();
impl /*struct*/ QPicture {
  pub fn size<RetType, T: QPicture_size<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QPicture_size<RetType> {
  fn size(self , rsthis: &mut QPicture) -> RetType;
}

  // proto:  uint QPicture::size();
impl<'a> /*trait*/ QPicture_size<u32> for () {
  fn size(self , rsthis: &mut QPicture) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPicture4sizeEv()};
    let mut ret = unsafe {_ZNK8QPicture4sizeEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

  // proto:  bool QPicture::isNull();
impl /*struct*/ QPicture {
  pub fn isNull<RetType, T: QPicture_isNull<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QPicture_isNull<RetType> {
  fn isNull(self , rsthis: &mut QPicture) -> RetType;
}

  // proto:  bool QPicture::isNull();
impl<'a> /*trait*/ QPicture_isNull<i8> for () {
  fn isNull(self , rsthis: &mut QPicture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPicture6isNullEv()};
    let mut ret = unsafe {_ZNK8QPicture6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QPicture::save(QIODevice * dev, const char * format);
impl /*struct*/ QPicture {
  pub fn save<RetType, T: QPicture_save<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.save(self);
    // return 1;
  }
}

pub trait QPicture_save<RetType> {
  fn save(self , rsthis: &mut QPicture) -> RetType;
}

  // proto:  bool QPicture::save(QIODevice * dev, const char * format);
impl<'a> /*trait*/ QPicture_save<i8> for (QIODevice, &'a  String) {
  fn save(self , rsthis: &mut QPicture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture4saveEP9QIODevicePKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN8QPicture4saveEP9QIODevicePKc(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPicture::detach();
impl /*struct*/ QPicture {
  pub fn detach<RetType, T: QPicture_detach<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.detach(self);
    // return 1;
  }
}

pub trait QPicture_detach<RetType> {
  fn detach(self , rsthis: &mut QPicture) -> RetType;
}

  // proto:  void QPicture::detach();
impl<'a> /*trait*/ QPicture_detach<()> for () {
  fn detach(self , rsthis: &mut QPicture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture6detachEv()};
     unsafe {_ZN8QPicture6detachEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static QList<QByteArray> QPicture::inputFormats();
impl /*struct*/ QPicture {
  pub fn inputFormats_s<RetType, T: QPicture_inputFormats_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.inputFormats_s();
    // return 1;
  }
}

pub trait QPicture_inputFormats_s<RetType> {
  fn inputFormats_s(self ) -> RetType;
}

  // proto: static QList<QByteArray> QPicture::inputFormats();
impl<'a> /*trait*/ QPicture_inputFormats_s<()> for () {
  fn inputFormats_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture12inputFormatsEv()};
     unsafe {_ZN8QPicture12inputFormatsEv()};
    // return 1;
  }
}

  // proto:  void QPicture::QPicture(int formatVersion);
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

  // proto:  void QPicture::QPicture(int formatVersion);
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

  // proto:  void QPicture::QPicture(const QPicture & );
impl<'a> /*trait*/ QPicture_NewQPicture for (QPicture) {
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

  // proto:  bool QPicture::isDetached();
impl /*struct*/ QPicture {
  pub fn isDetached<RetType, T: QPicture_isDetached<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isDetached(self);
    // return 1;
  }
}

pub trait QPicture_isDetached<RetType> {
  fn isDetached(self , rsthis: &mut QPicture) -> RetType;
}

  // proto:  bool QPicture::isDetached();
impl<'a> /*trait*/ QPicture_isDetached<i8> for () {
  fn isDetached(self , rsthis: &mut QPicture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPicture10isDetachedEv()};
    let mut ret = unsafe {_ZNK8QPicture10isDetachedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static QStringList QPicture::outputFormatList();
impl /*struct*/ QPicture {
  pub fn outputFormatList_s<RetType, T: QPicture_outputFormatList_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.outputFormatList_s();
    // return 1;
  }
}

pub trait QPicture_outputFormatList_s<RetType> {
  fn outputFormatList_s(self ) -> RetType;
}

  // proto: static QStringList QPicture::outputFormatList();
impl<'a> /*trait*/ QPicture_outputFormatList_s<()> for () {
  fn outputFormatList_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture16outputFormatListEv()};
     unsafe {_ZN8QPicture16outputFormatListEv()};
    // return 1;
  }
}

  // proto:  void QPicture::setData(const char * data, uint size);
impl /*struct*/ QPicture {
  pub fn setData<RetType, T: QPicture_setData<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setData(self);
    // return 1;
  }
}

pub trait QPicture_setData<RetType> {
  fn setData(self , rsthis: &mut QPicture) -> RetType;
}

  // proto:  void QPicture::setData(const char * data, uint size);
impl<'a> /*trait*/ QPicture_setData<()> for (&'a  String, u32) {
  fn setData(self , rsthis: &mut QPicture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture7setDataEPKcj()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_uint;
     unsafe {_ZN8QPicture7setDataEPKcj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto: static QList<QByteArray> QPicture::outputFormats();
impl /*struct*/ QPicture {
  pub fn outputFormats_s<RetType, T: QPicture_outputFormats_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.outputFormats_s();
    // return 1;
  }
}

pub trait QPicture_outputFormats_s<RetType> {
  fn outputFormats_s(self ) -> RetType;
}

  // proto: static QList<QByteArray> QPicture::outputFormats();
impl<'a> /*trait*/ QPicture_outputFormats_s<()> for () {
  fn outputFormats_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture13outputFormatsEv()};
     unsafe {_ZN8QPicture13outputFormatsEv()};
    // return 1;
  }
}

  // proto:  int QPicture::devType();
impl /*struct*/ QPicture {
  pub fn devType<RetType, T: QPicture_devType<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.devType(self);
    // return 1;
  }
}

pub trait QPicture_devType<RetType> {
  fn devType(self , rsthis: &mut QPicture) -> RetType;
}

  // proto:  int QPicture::devType();
impl<'a> /*trait*/ QPicture_devType<i32> for () {
  fn devType(self , rsthis: &mut QPicture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPicture7devTypeEv()};
    let mut ret = unsafe {_ZNK8QPicture7devTypeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto: static const char * QPicture::pictureFormat(const QString & fileName);
impl /*struct*/ QPicture {
  pub fn pictureFormat_s<RetType, T: QPicture_pictureFormat_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.pictureFormat_s();
    // return 1;
  }
}

pub trait QPicture_pictureFormat_s<RetType> {
  fn pictureFormat_s(self ) -> RetType;
}

  // proto: static const char * QPicture::pictureFormat(const QString & fileName);
impl<'a> /*trait*/ QPicture_pictureFormat_s<String> for (QString) {
  fn pictureFormat_s(self ) -> String {
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
impl<'a> /*trait*/ QPicture_save<i8> for (QString, &'a  String) {
  fn save(self , rsthis: &mut QPicture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture4saveERK7QStringPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN8QPicture4saveERK7QStringPKc(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QPicture::load(const QString & fileName, const char * format);
impl /*struct*/ QPicture {
  pub fn load<RetType, T: QPicture_load<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.load(self);
    // return 1;
  }
}

pub trait QPicture_load<RetType> {
  fn load(self , rsthis: &mut QPicture) -> RetType;
}

  // proto:  bool QPicture::load(const QString & fileName, const char * format);
impl<'a> /*trait*/ QPicture_load<i8> for (QString, &'a  String) {
  fn load(self , rsthis: &mut QPicture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture4loadERK7QStringPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN8QPicture4loadERK7QStringPKc(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPicture::~QPicture();
impl /*struct*/ QPicture {
  pub fn FreeQPicture<RetType, T: QPicture_FreeQPicture<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQPicture(self);
    // return 1;
  }
}

pub trait QPicture_FreeQPicture<RetType> {
  fn FreeQPicture(self , rsthis: &mut QPicture) -> RetType;
}

  // proto:  void QPicture::~QPicture();
impl<'a> /*trait*/ QPicture_FreeQPicture<()> for () {
  fn FreeQPicture(self , rsthis: &mut QPicture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPictureD0Ev()};
     unsafe {_ZN8QPictureD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPicture::setBoundingRect(const QRect & r);
impl /*struct*/ QPicture {
  pub fn setBoundingRect<RetType, T: QPicture_setBoundingRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setBoundingRect(self);
    // return 1;
  }
}

pub trait QPicture_setBoundingRect<RetType> {
  fn setBoundingRect(self , rsthis: &mut QPicture) -> RetType;
}

  // proto:  void QPicture::setBoundingRect(const QRect & r);
impl<'a> /*trait*/ QPicture_setBoundingRect<()> for (QRect) {
  fn setBoundingRect(self , rsthis: &mut QPicture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture15setBoundingRectERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPicture15setBoundingRectERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QPicture::load(QIODevice * dev, const char * format);
impl<'a> /*trait*/ QPicture_load<i8> for (QIODevice, &'a  String) {
  fn load(self , rsthis: &mut QPicture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture4loadEP9QIODevicePKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN8QPicture4loadEP9QIODevicePKc(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QRect QPicture::boundingRect();
impl /*struct*/ QPicture {
  pub fn boundingRect<RetType, T: QPicture_boundingRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.boundingRect(self);
    // return 1;
  }
}

pub trait QPicture_boundingRect<RetType> {
  fn boundingRect(self , rsthis: &mut QPicture) -> RetType;
}

  // proto:  QRect QPicture::boundingRect();
impl<'a> /*trait*/ QPicture_boundingRect<QRect> for () {
  fn boundingRect(self , rsthis: &mut QPicture) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPicture12boundingRectEv()};
    let mut ret = unsafe {_ZNK8QPicture12boundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QPicture::play(QPainter * p);
impl /*struct*/ QPicture {
  pub fn play<RetType, T: QPicture_play<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.play(self);
    // return 1;
  }
}

pub trait QPicture_play<RetType> {
  fn play(self , rsthis: &mut QPicture) -> RetType;
}

  // proto:  bool QPicture::play(QPainter * p);
impl<'a> /*trait*/ QPicture_play<i8> for (QPainter) {
  fn play(self , rsthis: &mut QPicture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture4playEP8QPainter()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QPicture4playEP8QPainter(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QPaintEngine * QPicture::paintEngine();
impl /*struct*/ QPicture {
  pub fn paintEngine<RetType, T: QPicture_paintEngine<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.paintEngine(self);
    // return 1;
  }
}

pub trait QPicture_paintEngine<RetType> {
  fn paintEngine(self , rsthis: &mut QPicture) -> RetType;
}

  // proto:  QPaintEngine * QPicture::paintEngine();
impl<'a> /*trait*/ QPicture_paintEngine<QPaintEngine> for () {
  fn paintEngine(self , rsthis: &mut QPicture) -> QPaintEngine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPicture11paintEngineEv()};
    let mut ret = unsafe {_ZNK8QPicture11paintEngineEv(rsthis.qclsinst)};
    let mut ret1 = QPaintEngine{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

