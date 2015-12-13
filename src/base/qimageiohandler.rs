// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qbytearray::QByteArray;
use super::qimage::QImage;
use super::qiodevice::QIODevice;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QImageIOHandler::NewQImageIOHandler(const QImageIOHandler & );
  fn _ZN15QImageIOHandlerC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: int QImageIOHandler::imageCount();
  fn _ZNK15QImageIOHandler10imageCountEv() -> i32;
  // proto: QRect QImageIOHandler::currentImageRect();
  fn _ZNK15QImageIOHandler16currentImageRectEv() -> i32;
  // proto: bool QImageIOHandler::jumpToImage(int imageNumber);
  fn _ZN15QImageIOHandler11jumpToImageEi(arg0: c_int) -> i32;
  // proto: int QImageIOHandler::currentImageNumber();
  fn _ZNK15QImageIOHandler18currentImageNumberEv() -> i32;
  // proto: void QImageIOHandler::setFormat(const QByteArray & format);
  fn _ZN15QImageIOHandler9setFormatERK10QByteArray(arg0: *const c_void) -> i32;
  // proto: bool QImageIOHandler::jumpToNextImage();
  fn _ZN15QImageIOHandler15jumpToNextImageEv() -> i32;
  // proto: void QImageIOHandler::FreeQImageIOHandler();
  fn _ZN15QImageIOHandlerD0Ev() -> i32;
  // proto: int QImageIOHandler::loopCount();
  fn _ZNK15QImageIOHandler9loopCountEv() -> i32;
  // proto: void QImageIOHandler::NewQImageIOHandler();
  fn _ZN15QImageIOHandlerC1Ev(qthis: *mut c_void) -> i32;
  // proto: bool QImageIOHandler::read(QImage * image);
  fn _ZN15QImageIOHandler4readEP6QImage(arg0: *mut c_void) -> i32;
  // proto: QByteArray QImageIOHandler::name();
  fn _ZNK15QImageIOHandler4nameEv() -> i32;
  // proto: QByteArray QImageIOHandler::format();
  fn _ZNK15QImageIOHandler6formatEv() -> i32;
  // proto: int QImageIOHandler::nextImageDelay();
  fn _ZNK15QImageIOHandler14nextImageDelayEv() -> i32;
  // proto: void QImageIOHandler::setDevice(QIODevice * device);
  fn _ZN15QImageIOHandler9setDeviceEP9QIODevice(arg0: *mut c_void) -> i32;
  // proto: bool QImageIOHandler::canRead();
  fn _ZNK15QImageIOHandler7canReadEv() -> i32;
  // proto: QIODevice * QImageIOHandler::device();
  fn _ZNK15QImageIOHandler6deviceEv() -> i32;
  // proto: bool QImageIOHandler::write(const QImage & image);
  fn _ZN15QImageIOHandler5writeERK6QImage(arg0: *const c_void) -> i32;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QImageIOHandlerC1ERKS_(qthis, arg0)};
    let rsthis = QImageIOHandler{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QImageIOHandler {
  pub fn imageCount<T: QImageIOHandler_imageCount>(&mut self, value: T) -> i32 {
    value.imageCount(self);
    return 1;
  }
}

pub trait QImageIOHandler_imageCount {
  fn imageCount(self, this: &mut QImageIOHandler) -> i32;
}

// proto: int QImageIOHandler::imageCount();
impl<'a> /*trait*/ QImageIOHandler_imageCount for () {
  fn imageCount(self, this: &mut QImageIOHandler) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QImageIOHandler10imageCountEv()};
    unsafe {_ZNK15QImageIOHandler10imageCountEv()};
    return 1;
  }
}

impl /*struct*/ QImageIOHandler {
  pub fn currentImageRect<T: QImageIOHandler_currentImageRect>(&mut self, value: T) -> i32 {
    value.currentImageRect(self);
    return 1;
  }
}

pub trait QImageIOHandler_currentImageRect {
  fn currentImageRect(self, this: &mut QImageIOHandler) -> i32;
}

// proto: QRect QImageIOHandler::currentImageRect();
impl<'a> /*trait*/ QImageIOHandler_currentImageRect for () {
  fn currentImageRect(self, this: &mut QImageIOHandler) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QImageIOHandler16currentImageRectEv()};
    unsafe {_ZNK15QImageIOHandler16currentImageRectEv()};
    return 1;
  }
}

impl /*struct*/ QImageIOHandler {
  pub fn jumpToImage<T: QImageIOHandler_jumpToImage>(&mut self, value: T) -> i32 {
    value.jumpToImage(self);
    return 1;
  }
}

pub trait QImageIOHandler_jumpToImage {
  fn jumpToImage(self, this: &mut QImageIOHandler) -> i32;
}

// proto: bool QImageIOHandler::jumpToImage(int imageNumber);
impl<'a> /*trait*/ QImageIOHandler_jumpToImage for (i32) {
  fn jumpToImage(self, this: &mut QImageIOHandler) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QImageIOHandler11jumpToImageEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN15QImageIOHandler11jumpToImageEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QImageIOHandler {
  pub fn currentImageNumber<T: QImageIOHandler_currentImageNumber>(&mut self, value: T) -> i32 {
    value.currentImageNumber(self);
    return 1;
  }
}

pub trait QImageIOHandler_currentImageNumber {
  fn currentImageNumber(self, this: &mut QImageIOHandler) -> i32;
}

// proto: int QImageIOHandler::currentImageNumber();
impl<'a> /*trait*/ QImageIOHandler_currentImageNumber for () {
  fn currentImageNumber(self, this: &mut QImageIOHandler) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QImageIOHandler18currentImageNumberEv()};
    unsafe {_ZNK15QImageIOHandler18currentImageNumberEv()};
    return 1;
  }
}

impl /*struct*/ QImageIOHandler {
  pub fn setFormat<T: QImageIOHandler_setFormat>(&mut self, value: T) -> i32 {
    value.setFormat(self);
    return 1;
  }
}

pub trait QImageIOHandler_setFormat {
  fn setFormat(self, this: &mut QImageIOHandler) -> i32;
}

// proto: void QImageIOHandler::setFormat(const QByteArray & format);
impl<'a> /*trait*/ QImageIOHandler_setFormat for (&'a  QByteArray) {
  fn setFormat(self, this: &mut QImageIOHandler) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QImageIOHandler9setFormatERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QImageIOHandler9setFormatERK10QByteArray(arg0)};
    return 1;
  }
}

impl /*struct*/ QImageIOHandler {
  pub fn jumpToNextImage<T: QImageIOHandler_jumpToNextImage>(&mut self, value: T) -> i32 {
    value.jumpToNextImage(self);
    return 1;
  }
}

pub trait QImageIOHandler_jumpToNextImage {
  fn jumpToNextImage(self, this: &mut QImageIOHandler) -> i32;
}

// proto: bool QImageIOHandler::jumpToNextImage();
impl<'a> /*trait*/ QImageIOHandler_jumpToNextImage for () {
  fn jumpToNextImage(self, this: &mut QImageIOHandler) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QImageIOHandler15jumpToNextImageEv()};
    unsafe {_ZN15QImageIOHandler15jumpToNextImageEv()};
    return 1;
  }
}

impl /*struct*/ QImageIOHandler {
  pub fn FreeQImageIOHandler<T: QImageIOHandler_FreeQImageIOHandler>(&mut self, value: T) -> i32 {
    value.FreeQImageIOHandler(self);
    return 1;
  }
}

pub trait QImageIOHandler_FreeQImageIOHandler {
  fn FreeQImageIOHandler(self, this: &mut QImageIOHandler) -> i32;
}

// proto: void QImageIOHandler::FreeQImageIOHandler();
impl<'a> /*trait*/ QImageIOHandler_FreeQImageIOHandler for () {
  fn FreeQImageIOHandler(self, this: &mut QImageIOHandler) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QImageIOHandlerD0Ev()};
    unsafe {_ZN15QImageIOHandlerD0Ev()};
    return 1;
  }
}

impl /*struct*/ QImageIOHandler {
  pub fn loopCount<T: QImageIOHandler_loopCount>(&mut self, value: T) -> i32 {
    value.loopCount(self);
    return 1;
  }
}

pub trait QImageIOHandler_loopCount {
  fn loopCount(self, this: &mut QImageIOHandler) -> i32;
}

// proto: int QImageIOHandler::loopCount();
impl<'a> /*trait*/ QImageIOHandler_loopCount for () {
  fn loopCount(self, this: &mut QImageIOHandler) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QImageIOHandler9loopCountEv()};
    unsafe {_ZNK15QImageIOHandler9loopCountEv()};
    return 1;
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
  pub fn read<T: QImageIOHandler_read>(&mut self, value: T) -> i32 {
    value.read(self);
    return 1;
  }
}

pub trait QImageIOHandler_read {
  fn read(self, this: &mut QImageIOHandler) -> i32;
}

// proto: bool QImageIOHandler::read(QImage * image);
impl<'a> /*trait*/ QImageIOHandler_read for (&'a mut QImage) {
  fn read(self, this: &mut QImageIOHandler) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QImageIOHandler4readEP6QImage()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QImageIOHandler4readEP6QImage(arg0)};
    return 1;
  }
}

impl /*struct*/ QImageIOHandler {
  pub fn name<T: QImageIOHandler_name>(&mut self, value: T) -> i32 {
    value.name(self);
    return 1;
  }
}

pub trait QImageIOHandler_name {
  fn name(self, this: &mut QImageIOHandler) -> i32;
}

// proto: QByteArray QImageIOHandler::name();
impl<'a> /*trait*/ QImageIOHandler_name for () {
  fn name(self, this: &mut QImageIOHandler) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QImageIOHandler4nameEv()};
    unsafe {_ZNK15QImageIOHandler4nameEv()};
    return 1;
  }
}

impl /*struct*/ QImageIOHandler {
  pub fn format<T: QImageIOHandler_format>(&mut self, value: T) -> i32 {
    value.format(self);
    return 1;
  }
}

pub trait QImageIOHandler_format {
  fn format(self, this: &mut QImageIOHandler) -> i32;
}

// proto: QByteArray QImageIOHandler::format();
impl<'a> /*trait*/ QImageIOHandler_format for () {
  fn format(self, this: &mut QImageIOHandler) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QImageIOHandler6formatEv()};
    unsafe {_ZNK15QImageIOHandler6formatEv()};
    return 1;
  }
}

impl /*struct*/ QImageIOHandler {
  pub fn nextImageDelay<T: QImageIOHandler_nextImageDelay>(&mut self, value: T) -> i32 {
    value.nextImageDelay(self);
    return 1;
  }
}

pub trait QImageIOHandler_nextImageDelay {
  fn nextImageDelay(self, this: &mut QImageIOHandler) -> i32;
}

// proto: int QImageIOHandler::nextImageDelay();
impl<'a> /*trait*/ QImageIOHandler_nextImageDelay for () {
  fn nextImageDelay(self, this: &mut QImageIOHandler) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QImageIOHandler14nextImageDelayEv()};
    unsafe {_ZNK15QImageIOHandler14nextImageDelayEv()};
    return 1;
  }
}

impl /*struct*/ QImageIOHandler {
  pub fn setDevice<T: QImageIOHandler_setDevice>(&mut self, value: T) -> i32 {
    value.setDevice(self);
    return 1;
  }
}

pub trait QImageIOHandler_setDevice {
  fn setDevice(self, this: &mut QImageIOHandler) -> i32;
}

// proto: void QImageIOHandler::setDevice(QIODevice * device);
impl<'a> /*trait*/ QImageIOHandler_setDevice for (&'a mut QIODevice) {
  fn setDevice(self, this: &mut QImageIOHandler) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QImageIOHandler9setDeviceEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QImageIOHandler9setDeviceEP9QIODevice(arg0)};
    return 1;
  }
}

impl /*struct*/ QImageIOHandler {
  pub fn canRead<T: QImageIOHandler_canRead>(&mut self, value: T) -> i32 {
    value.canRead(self);
    return 1;
  }
}

pub trait QImageIOHandler_canRead {
  fn canRead(self, this: &mut QImageIOHandler) -> i32;
}

// proto: bool QImageIOHandler::canRead();
impl<'a> /*trait*/ QImageIOHandler_canRead for () {
  fn canRead(self, this: &mut QImageIOHandler) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QImageIOHandler7canReadEv()};
    unsafe {_ZNK15QImageIOHandler7canReadEv()};
    return 1;
  }
}

impl /*struct*/ QImageIOHandler {
  pub fn device<T: QImageIOHandler_device>(&mut self, value: T) -> i32 {
    value.device(self);
    return 1;
  }
}

pub trait QImageIOHandler_device {
  fn device(self, this: &mut QImageIOHandler) -> i32;
}

// proto: QIODevice * QImageIOHandler::device();
impl<'a> /*trait*/ QImageIOHandler_device for () {
  fn device(self, this: &mut QImageIOHandler) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QImageIOHandler6deviceEv()};
    unsafe {_ZNK15QImageIOHandler6deviceEv()};
    return 1;
  }
}

impl /*struct*/ QImageIOHandler {
  pub fn write<T: QImageIOHandler_write>(&mut self, value: T) -> i32 {
    value.write(self);
    return 1;
  }
}

pub trait QImageIOHandler_write {
  fn write(self, this: &mut QImageIOHandler) -> i32;
}

// proto: bool QImageIOHandler::write(const QImage & image);
impl<'a> /*trait*/ QImageIOHandler_write for (&'a  QImage) {
  fn write(self, this: &mut QImageIOHandler) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QImageIOHandler5writeERK6QImage()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QImageIOHandler5writeERK6QImage(arg0)};
    return 1;
  }
}

