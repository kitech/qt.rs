// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qrect::QRect;
use super::qbytearray::QByteArray;
use super::qimage::QImage;
use super::qiodevice::QIODevice;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QImageIOHandler::NewQImageIOHandler(const QImageIOHandler & );
  fn _ZN15QImageIOHandlerC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QImageIOHandler::imageCount();
  fn _ZNK15QImageIOHandler10imageCountEv(qthis: *mut c_void) -> c_int;
  // proto:  QRect QImageIOHandler::currentImageRect();
  fn _ZNK15QImageIOHandler16currentImageRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QImageIOHandler::jumpToImage(int imageNumber);
  fn _ZN15QImageIOHandler11jumpToImageEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  int QImageIOHandler::currentImageNumber();
  fn _ZNK15QImageIOHandler18currentImageNumberEv(qthis: *mut c_void) -> c_int;
  // proto:  void QImageIOHandler::setFormat(const QByteArray & format);
  fn _ZN15QImageIOHandler9setFormatERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QImageIOHandler::jumpToNextImage();
  fn _ZN15QImageIOHandler15jumpToNextImageEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QImageIOHandler::FreeQImageIOHandler();
  fn _ZN15QImageIOHandlerD0Ev(qthis: *mut c_void) ;
  // proto:  int QImageIOHandler::loopCount();
  fn _ZNK15QImageIOHandler9loopCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QImageIOHandler::NewQImageIOHandler();
  fn _ZN15QImageIOHandlerC1Ev(qthis: *mut c_void) ;
  // proto:  bool QImageIOHandler::read(QImage * image);
  fn _ZN15QImageIOHandler4readEP6QImage(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QByteArray QImageIOHandler::name();
  fn _ZNK15QImageIOHandler4nameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QByteArray QImageIOHandler::format();
  fn _ZNK15QImageIOHandler6formatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QImageIOHandler::nextImageDelay();
  fn _ZNK15QImageIOHandler14nextImageDelayEv(qthis: *mut c_void) -> c_int;
  // proto:  void QImageIOHandler::setDevice(QIODevice * device);
  fn _ZN15QImageIOHandler9setDeviceEP9QIODevice(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QImageIOHandler::canRead();
  fn _ZNK15QImageIOHandler7canReadEv(qthis: *mut c_void) -> int8_t;
  // proto:  QIODevice * QImageIOHandler::device();
  fn _ZNK15QImageIOHandler6deviceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QImageIOHandler::write(const QImage & image);
  fn _ZN15QImageIOHandler5writeERK6QImage(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
}

// body block begin
// class sizeof(QImageIOHandler)=1
pub struct QImageIOHandler {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QImageIOHandler {
  pub fn NewQImageIOHandler<T: QImageIOHandler_NewQImageIOHandler>(value: T) -> QImageIOHandler {
    let rsthis = value.NewQImageIOHandler();
    return rsthis;
    // return 1;
  }
}

pub trait QImageIOHandler_NewQImageIOHandler {
  fn NewQImageIOHandler(self) -> QImageIOHandler;
}

// proto: void QImageIOHandler::NewQImageIOHandler(const QImageIOHandler & );
impl<'a> /*trait*/ QImageIOHandler_NewQImageIOHandler for (&'a  QImageIOHandler) {
  fn NewQImageIOHandler(self) -> QImageIOHandler {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QImageIOHandlerC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QImageIOHandlerC1ERKS_(qthis, arg0)};
    let rsthis = QImageIOHandler{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QImageIOHandler {
  pub fn imageCount<RetType, T: QImageIOHandler_imageCount<RetType>>(&mut self, value: T) -> RetType {
    return value.imageCount(self);
    // return 1;
  }
}

pub trait QImageIOHandler_imageCount<RetType> {
  fn imageCount(self, rsthis: &mut QImageIOHandler) -> RetType;
}

// proto:  int QImageIOHandler::imageCount();
impl<'a> /*trait*/ QImageIOHandler_imageCount<i32> for () {
  fn imageCount(self, rsthis: &mut QImageIOHandler) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QImageIOHandler10imageCountEv()};
    let mut ret = unsafe {_ZNK15QImageIOHandler10imageCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QImageIOHandler {
  pub fn currentImageRect<RetType, T: QImageIOHandler_currentImageRect<RetType>>(&mut self, value: T) -> RetType {
    return value.currentImageRect(self);
    // return 1;
  }
}

pub trait QImageIOHandler_currentImageRect<RetType> {
  fn currentImageRect(self, rsthis: &mut QImageIOHandler) -> RetType;
}

// proto:  QRect QImageIOHandler::currentImageRect();
impl<'a> /*trait*/ QImageIOHandler_currentImageRect<QRect> for () {
  fn currentImageRect(self, rsthis: &mut QImageIOHandler) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QImageIOHandler16currentImageRectEv()};
    let mut ret = unsafe {_ZNK15QImageIOHandler16currentImageRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImageIOHandler {
  pub fn jumpToImage<RetType, T: QImageIOHandler_jumpToImage<RetType>>(&mut self, value: T) -> RetType {
    return value.jumpToImage(self);
    // return 1;
  }
}

pub trait QImageIOHandler_jumpToImage<RetType> {
  fn jumpToImage(self, rsthis: &mut QImageIOHandler) -> RetType;
}

// proto:  bool QImageIOHandler::jumpToImage(int imageNumber);
impl<'a> /*trait*/ QImageIOHandler_jumpToImage<i8> for (i32) {
  fn jumpToImage(self, rsthis: &mut QImageIOHandler) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QImageIOHandler11jumpToImageEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN15QImageIOHandler11jumpToImageEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QImageIOHandler {
  pub fn currentImageNumber<RetType, T: QImageIOHandler_currentImageNumber<RetType>>(&mut self, value: T) -> RetType {
    return value.currentImageNumber(self);
    // return 1;
  }
}

pub trait QImageIOHandler_currentImageNumber<RetType> {
  fn currentImageNumber(self, rsthis: &mut QImageIOHandler) -> RetType;
}

// proto:  int QImageIOHandler::currentImageNumber();
impl<'a> /*trait*/ QImageIOHandler_currentImageNumber<i32> for () {
  fn currentImageNumber(self, rsthis: &mut QImageIOHandler) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QImageIOHandler18currentImageNumberEv()};
    let mut ret = unsafe {_ZNK15QImageIOHandler18currentImageNumberEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QImageIOHandler {
  pub fn setFormat<RetType, T: QImageIOHandler_setFormat<RetType>>(&mut self, value: T) -> RetType {
    return value.setFormat(self);
    // return 1;
  }
}

pub trait QImageIOHandler_setFormat<RetType> {
  fn setFormat(self, rsthis: &mut QImageIOHandler) -> RetType;
}

// proto:  void QImageIOHandler::setFormat(const QByteArray & format);
impl<'a> /*trait*/ QImageIOHandler_setFormat<()> for (&'a  QByteArray) {
  fn setFormat(self, rsthis: &mut QImageIOHandler) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QImageIOHandler9setFormatERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QImageIOHandler9setFormatERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QImageIOHandler {
  pub fn jumpToNextImage<RetType, T: QImageIOHandler_jumpToNextImage<RetType>>(&mut self, value: T) -> RetType {
    return value.jumpToNextImage(self);
    // return 1;
  }
}

pub trait QImageIOHandler_jumpToNextImage<RetType> {
  fn jumpToNextImage(self, rsthis: &mut QImageIOHandler) -> RetType;
}

// proto:  bool QImageIOHandler::jumpToNextImage();
impl<'a> /*trait*/ QImageIOHandler_jumpToNextImage<i8> for () {
  fn jumpToNextImage(self, rsthis: &mut QImageIOHandler) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QImageIOHandler15jumpToNextImageEv()};
    let mut ret = unsafe {_ZN15QImageIOHandler15jumpToNextImageEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QImageIOHandler {
  pub fn FreeQImageIOHandler<RetType, T: QImageIOHandler_FreeQImageIOHandler<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQImageIOHandler(self);
    // return 1;
  }
}

pub trait QImageIOHandler_FreeQImageIOHandler<RetType> {
  fn FreeQImageIOHandler(self, rsthis: &mut QImageIOHandler) -> RetType;
}

// proto:  void QImageIOHandler::FreeQImageIOHandler();
impl<'a> /*trait*/ QImageIOHandler_FreeQImageIOHandler<()> for () {
  fn FreeQImageIOHandler(self, rsthis: &mut QImageIOHandler) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QImageIOHandlerD0Ev()};
     unsafe {_ZN15QImageIOHandlerD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QImageIOHandler {
  pub fn loopCount<RetType, T: QImageIOHandler_loopCount<RetType>>(&mut self, value: T) -> RetType {
    return value.loopCount(self);
    // return 1;
  }
}

pub trait QImageIOHandler_loopCount<RetType> {
  fn loopCount(self, rsthis: &mut QImageIOHandler) -> RetType;
}

// proto:  int QImageIOHandler::loopCount();
impl<'a> /*trait*/ QImageIOHandler_loopCount<i32> for () {
  fn loopCount(self, rsthis: &mut QImageIOHandler) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QImageIOHandler9loopCountEv()};
    let mut ret = unsafe {_ZNK15QImageIOHandler9loopCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto: void QImageIOHandler::NewQImageIOHandler();
impl<'a> /*trait*/ QImageIOHandler_NewQImageIOHandler for () {
  fn NewQImageIOHandler(self) -> QImageIOHandler {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QImageIOHandlerC1Ev()};
    unsafe {_ZN15QImageIOHandlerC1Ev(qthis)};
    let rsthis = QImageIOHandler{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QImageIOHandler {
  pub fn read<RetType, T: QImageIOHandler_read<RetType>>(&mut self, value: T) -> RetType {
    return value.read(self);
    // return 1;
  }
}

pub trait QImageIOHandler_read<RetType> {
  fn read(self, rsthis: &mut QImageIOHandler) -> RetType;
}

// proto:  bool QImageIOHandler::read(QImage * image);
impl<'a> /*trait*/ QImageIOHandler_read<i8> for (&'a mut QImage) {
  fn read(self, rsthis: &mut QImageIOHandler) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QImageIOHandler4readEP6QImage()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN15QImageIOHandler4readEP6QImage(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QImageIOHandler {
  pub fn name<RetType, T: QImageIOHandler_name<RetType>>(&mut self, value: T) -> RetType {
    return value.name(self);
    // return 1;
  }
}

pub trait QImageIOHandler_name<RetType> {
  fn name(self, rsthis: &mut QImageIOHandler) -> RetType;
}

// proto:  QByteArray QImageIOHandler::name();
impl<'a> /*trait*/ QImageIOHandler_name<QByteArray> for () {
  fn name(self, rsthis: &mut QImageIOHandler) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QImageIOHandler4nameEv()};
    let mut ret = unsafe {_ZNK15QImageIOHandler4nameEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImageIOHandler {
  pub fn format<RetType, T: QImageIOHandler_format<RetType>>(&mut self, value: T) -> RetType {
    return value.format(self);
    // return 1;
  }
}

pub trait QImageIOHandler_format<RetType> {
  fn format(self, rsthis: &mut QImageIOHandler) -> RetType;
}

// proto:  QByteArray QImageIOHandler::format();
impl<'a> /*trait*/ QImageIOHandler_format<QByteArray> for () {
  fn format(self, rsthis: &mut QImageIOHandler) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QImageIOHandler6formatEv()};
    let mut ret = unsafe {_ZNK15QImageIOHandler6formatEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImageIOHandler {
  pub fn nextImageDelay<RetType, T: QImageIOHandler_nextImageDelay<RetType>>(&mut self, value: T) -> RetType {
    return value.nextImageDelay(self);
    // return 1;
  }
}

pub trait QImageIOHandler_nextImageDelay<RetType> {
  fn nextImageDelay(self, rsthis: &mut QImageIOHandler) -> RetType;
}

// proto:  int QImageIOHandler::nextImageDelay();
impl<'a> /*trait*/ QImageIOHandler_nextImageDelay<i32> for () {
  fn nextImageDelay(self, rsthis: &mut QImageIOHandler) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QImageIOHandler14nextImageDelayEv()};
    let mut ret = unsafe {_ZNK15QImageIOHandler14nextImageDelayEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QImageIOHandler {
  pub fn setDevice<RetType, T: QImageIOHandler_setDevice<RetType>>(&mut self, value: T) -> RetType {
    return value.setDevice(self);
    // return 1;
  }
}

pub trait QImageIOHandler_setDevice<RetType> {
  fn setDevice(self, rsthis: &mut QImageIOHandler) -> RetType;
}

// proto:  void QImageIOHandler::setDevice(QIODevice * device);
impl<'a> /*trait*/ QImageIOHandler_setDevice<()> for (&'a mut QIODevice) {
  fn setDevice(self, rsthis: &mut QImageIOHandler) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QImageIOHandler9setDeviceEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QImageIOHandler9setDeviceEP9QIODevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QImageIOHandler {
  pub fn canRead<RetType, T: QImageIOHandler_canRead<RetType>>(&mut self, value: T) -> RetType {
    return value.canRead(self);
    // return 1;
  }
}

pub trait QImageIOHandler_canRead<RetType> {
  fn canRead(self, rsthis: &mut QImageIOHandler) -> RetType;
}

// proto:  bool QImageIOHandler::canRead();
impl<'a> /*trait*/ QImageIOHandler_canRead<i8> for () {
  fn canRead(self, rsthis: &mut QImageIOHandler) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QImageIOHandler7canReadEv()};
    let mut ret = unsafe {_ZNK15QImageIOHandler7canReadEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QImageIOHandler {
  pub fn device<RetType, T: QImageIOHandler_device<RetType>>(&mut self, value: T) -> RetType {
    return value.device(self);
    // return 1;
  }
}

pub trait QImageIOHandler_device<RetType> {
  fn device(self, rsthis: &mut QImageIOHandler) -> RetType;
}

// proto:  QIODevice * QImageIOHandler::device();
impl<'a> /*trait*/ QImageIOHandler_device<QIODevice> for () {
  fn device(self, rsthis: &mut QImageIOHandler) -> QIODevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QImageIOHandler6deviceEv()};
    let mut ret = unsafe {_ZNK15QImageIOHandler6deviceEv(rsthis.qclsinst)};
    let mut ret1 = QIODevice{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImageIOHandler {
  pub fn write<RetType, T: QImageIOHandler_write<RetType>>(&mut self, value: T) -> RetType {
    return value.write(self);
    // return 1;
  }
}

pub trait QImageIOHandler_write<RetType> {
  fn write(self, rsthis: &mut QImageIOHandler) -> RetType;
}

// proto:  bool QImageIOHandler::write(const QImage & image);
impl<'a> /*trait*/ QImageIOHandler_write<i8> for (&'a  QImage) {
  fn write(self, rsthis: &mut QImageIOHandler) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QImageIOHandler5writeERK6QImage()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN15QImageIOHandler5writeERK6QImage(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

