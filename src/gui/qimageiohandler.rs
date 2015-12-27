// auto generated, do not modify.
// created: Sun Dec 27 22:52:02 2015
// src-file: /QtGui/qimageiohandler.h
// dst-file: /src/gui/qimageiohandler.rs
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
use super::super::core::qrect::QRect; // 771
use super::super::core::qbytearray::QByteArray; // 771
use super::qimage::QImage; // 773
use super::super::core::qvariant::QVariant; // 771
use super::super::core::qiodevice::QIODevice; // 771
use super::super::core::qobject::QObject; // 771
// use super::qimageiohandler::QImageIOHandler; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QImageIOHandler_Class_Size() -> c_int;
  // proto:  void QImageIOHandler::QImageIOHandler(const QImageIOHandler & );
  fn dector_ZN15QImageIOHandlerC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN15QImageIOHandlerC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QImageIOHandler::imageCount();
  fn _ZNK15QImageIOHandler10imageCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QRect QImageIOHandler::currentImageRect();
  fn _ZNK15QImageIOHandler16currentImageRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QImageIOHandler::jumpToImage(int imageNumber);
  fn _ZN15QImageIOHandler11jumpToImageEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  int QImageIOHandler::currentImageNumber();
  fn _ZNK15QImageIOHandler18currentImageNumberEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QImageIOHandler::setFormat(const QByteArray & format);
  fn _ZN15QImageIOHandler9setFormatERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QImageIOHandler::jumpToNextImage();
  fn _ZN15QImageIOHandler15jumpToNextImageEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QImageIOHandler::~QImageIOHandler();
  fn _ZN15QImageIOHandlerD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  int QImageIOHandler::loopCount();
  fn _ZNK15QImageIOHandler9loopCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QImageIOHandler::QImageIOHandler();
  fn dector_ZN15QImageIOHandlerC1Ev() -> *mut c_void;
  fn _ZN15QImageIOHandlerC1Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QImageIOHandler::read(QImage * image);
  fn _ZN15QImageIOHandler4readEP6QImage(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QByteArray QImageIOHandler::name();
  fn _ZNK15QImageIOHandler4nameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QByteArray QImageIOHandler::format();
  fn _ZNK15QImageIOHandler6formatEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QImageIOHandler::nextImageDelay();
  fn _ZNK15QImageIOHandler14nextImageDelayEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QImageIOHandler::setDevice(QIODevice * device);
  fn _ZN15QImageIOHandler9setDeviceEP9QIODevice(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QImageIOHandler::canRead();
  fn _ZNK15QImageIOHandler7canReadEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QIODevice * QImageIOHandler::device();
  fn _ZNK15QImageIOHandler6deviceEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QImageIOHandler::write(const QImage & image);
  fn _ZN15QImageIOHandler5writeERK6QImage(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  fn QImageIOPlugin_Class_Size() -> c_int;
  // proto:  const QMetaObject * QImageIOPlugin::metaObject();
  fn _ZNK14QImageIOPlugin10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QImageIOPlugin::~QImageIOPlugin();
  fn _ZN14QImageIOPluginD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QImageIOHandler * QImageIOPlugin::create(QIODevice * device, const QByteArray & format);
  fn _ZNK14QImageIOPlugin6createEP9QIODeviceRK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QImageIOPlugin::QImageIOPlugin(QObject * parent);
  fn dector_ZN14QImageIOPluginC1EP7QObject(arg0: *mut c_void) -> *mut c_void;
  fn _ZN14QImageIOPluginC1EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QImageIOHandler)=1
#[derive(Default)]
pub struct QImageIOHandler {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QImageIOPlugin)=1
#[derive(Default)]
pub struct QImageIOPlugin {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QImageIOHandler {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QImageIOHandler {
    return QImageIOHandler{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QImageIOHandler::QImageIOHandler(const QImageIOHandler & );
impl /*struct*/ QImageIOHandler {
  pub fn New<T: QImageIOHandler_New>(value: T) -> QImageIOHandler {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QImageIOHandler_New {
  fn New(self) -> QImageIOHandler;
}

  // proto:  void QImageIOHandler::QImageIOHandler(const QImageIOHandler & );
impl<'a> /*trait*/ QImageIOHandler_New for (&'a QImageIOHandler) {
  fn New(self) -> QImageIOHandler {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QImageIOHandlerC1ERKS_()};
    let ctysz: c_int = unsafe{QImageIOHandler_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN15QImageIOHandlerC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN15QImageIOHandlerC1ERKS_(arg0)} as u64;
    let rsthis = QImageIOHandler{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QImageIOHandler::imageCount();
impl /*struct*/ QImageIOHandler {
  pub fn imageCount<RetType, T: QImageIOHandler_imageCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.imageCount(self);
    // return 1;
  }
}

pub trait QImageIOHandler_imageCount<RetType> {
  fn imageCount(self , rsthis: & QImageIOHandler) -> RetType;
}

  // proto:  int QImageIOHandler::imageCount();
impl<'a> /*trait*/ QImageIOHandler_imageCount<i32> for () {
  fn imageCount(self , rsthis: & QImageIOHandler) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QImageIOHandler10imageCountEv()};
    let mut ret = unsafe {_ZNK15QImageIOHandler10imageCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QRect QImageIOHandler::currentImageRect();
impl /*struct*/ QImageIOHandler {
  pub fn currentImageRect<RetType, T: QImageIOHandler_currentImageRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentImageRect(self);
    // return 1;
  }
}

pub trait QImageIOHandler_currentImageRect<RetType> {
  fn currentImageRect(self , rsthis: & QImageIOHandler) -> RetType;
}

  // proto:  QRect QImageIOHandler::currentImageRect();
impl<'a> /*trait*/ QImageIOHandler_currentImageRect<QRect> for () {
  fn currentImageRect(self , rsthis: & QImageIOHandler) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QImageIOHandler16currentImageRectEv()};
    let mut ret = unsafe {_ZNK15QImageIOHandler16currentImageRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QImageIOHandler::jumpToImage(int imageNumber);
impl /*struct*/ QImageIOHandler {
  pub fn jumpToImage<RetType, T: QImageIOHandler_jumpToImage<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.jumpToImage(self);
    // return 1;
  }
}

pub trait QImageIOHandler_jumpToImage<RetType> {
  fn jumpToImage(self , rsthis: & QImageIOHandler) -> RetType;
}

  // proto:  bool QImageIOHandler::jumpToImage(int imageNumber);
impl<'a> /*trait*/ QImageIOHandler_jumpToImage<i8> for (i32) {
  fn jumpToImage(self , rsthis: & QImageIOHandler) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QImageIOHandler11jumpToImageEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN15QImageIOHandler11jumpToImageEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QImageIOHandler::currentImageNumber();
impl /*struct*/ QImageIOHandler {
  pub fn currentImageNumber<RetType, T: QImageIOHandler_currentImageNumber<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentImageNumber(self);
    // return 1;
  }
}

pub trait QImageIOHandler_currentImageNumber<RetType> {
  fn currentImageNumber(self , rsthis: & QImageIOHandler) -> RetType;
}

  // proto:  int QImageIOHandler::currentImageNumber();
impl<'a> /*trait*/ QImageIOHandler_currentImageNumber<i32> for () {
  fn currentImageNumber(self , rsthis: & QImageIOHandler) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QImageIOHandler18currentImageNumberEv()};
    let mut ret = unsafe {_ZNK15QImageIOHandler18currentImageNumberEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QImageIOHandler::setFormat(const QByteArray & format);
impl /*struct*/ QImageIOHandler {
  pub fn setFormat<RetType, T: QImageIOHandler_setFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFormat(self);
    // return 1;
  }
}

pub trait QImageIOHandler_setFormat<RetType> {
  fn setFormat(self , rsthis: & QImageIOHandler) -> RetType;
}

  // proto:  void QImageIOHandler::setFormat(const QByteArray & format);
impl<'a> /*trait*/ QImageIOHandler_setFormat<()> for (&'a QByteArray) {
  fn setFormat(self , rsthis: & QImageIOHandler) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QImageIOHandler9setFormatERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QImageIOHandler9setFormatERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QImageIOHandler::jumpToNextImage();
impl /*struct*/ QImageIOHandler {
  pub fn jumpToNextImage<RetType, T: QImageIOHandler_jumpToNextImage<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.jumpToNextImage(self);
    // return 1;
  }
}

pub trait QImageIOHandler_jumpToNextImage<RetType> {
  fn jumpToNextImage(self , rsthis: & QImageIOHandler) -> RetType;
}

  // proto:  bool QImageIOHandler::jumpToNextImage();
impl<'a> /*trait*/ QImageIOHandler_jumpToNextImage<i8> for () {
  fn jumpToNextImage(self , rsthis: & QImageIOHandler) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QImageIOHandler15jumpToNextImageEv()};
    let mut ret = unsafe {_ZN15QImageIOHandler15jumpToNextImageEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QImageIOHandler::~QImageIOHandler();
impl /*struct*/ QImageIOHandler {
  pub fn Free<RetType, T: QImageIOHandler_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QImageIOHandler_Free<RetType> {
  fn Free(self , rsthis: & QImageIOHandler) -> RetType;
}

  // proto:  void QImageIOHandler::~QImageIOHandler();
impl<'a> /*trait*/ QImageIOHandler_Free<()> for () {
  fn Free(self , rsthis: & QImageIOHandler) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QImageIOHandlerD0Ev()};
     unsafe {_ZN15QImageIOHandlerD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QImageIOHandler::loopCount();
impl /*struct*/ QImageIOHandler {
  pub fn loopCount<RetType, T: QImageIOHandler_loopCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.loopCount(self);
    // return 1;
  }
}

pub trait QImageIOHandler_loopCount<RetType> {
  fn loopCount(self , rsthis: & QImageIOHandler) -> RetType;
}

  // proto:  int QImageIOHandler::loopCount();
impl<'a> /*trait*/ QImageIOHandler_loopCount<i32> for () {
  fn loopCount(self , rsthis: & QImageIOHandler) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QImageIOHandler9loopCountEv()};
    let mut ret = unsafe {_ZNK15QImageIOHandler9loopCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QImageIOHandler::QImageIOHandler();
impl<'a> /*trait*/ QImageIOHandler_New for () {
  fn New(self) -> QImageIOHandler {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QImageIOHandlerC1Ev()};
    let ctysz: c_int = unsafe{QImageIOHandler_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN15QImageIOHandlerC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN15QImageIOHandlerC1Ev()} as u64;
    let rsthis = QImageIOHandler{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QImageIOHandler::read(QImage * image);
impl /*struct*/ QImageIOHandler {
  pub fn read<RetType, T: QImageIOHandler_read<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.read(self);
    // return 1;
  }
}

pub trait QImageIOHandler_read<RetType> {
  fn read(self , rsthis: & QImageIOHandler) -> RetType;
}

  // proto:  bool QImageIOHandler::read(QImage * image);
impl<'a> /*trait*/ QImageIOHandler_read<i8> for (&'a QImage) {
  fn read(self , rsthis: & QImageIOHandler) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QImageIOHandler4readEP6QImage()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN15QImageIOHandler4readEP6QImage(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QByteArray QImageIOHandler::name();
impl /*struct*/ QImageIOHandler {
  pub fn name<RetType, T: QImageIOHandler_name<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QImageIOHandler_name<RetType> {
  fn name(self , rsthis: & QImageIOHandler) -> RetType;
}

  // proto:  QByteArray QImageIOHandler::name();
impl<'a> /*trait*/ QImageIOHandler_name<QByteArray> for () {
  fn name(self , rsthis: & QImageIOHandler) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QImageIOHandler4nameEv()};
    let mut ret = unsafe {_ZNK15QImageIOHandler4nameEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray QImageIOHandler::format();
impl /*struct*/ QImageIOHandler {
  pub fn format<RetType, T: QImageIOHandler_format<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.format(self);
    // return 1;
  }
}

pub trait QImageIOHandler_format<RetType> {
  fn format(self , rsthis: & QImageIOHandler) -> RetType;
}

  // proto:  QByteArray QImageIOHandler::format();
impl<'a> /*trait*/ QImageIOHandler_format<QByteArray> for () {
  fn format(self , rsthis: & QImageIOHandler) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QImageIOHandler6formatEv()};
    let mut ret = unsafe {_ZNK15QImageIOHandler6formatEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QImageIOHandler::nextImageDelay();
impl /*struct*/ QImageIOHandler {
  pub fn nextImageDelay<RetType, T: QImageIOHandler_nextImageDelay<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.nextImageDelay(self);
    // return 1;
  }
}

pub trait QImageIOHandler_nextImageDelay<RetType> {
  fn nextImageDelay(self , rsthis: & QImageIOHandler) -> RetType;
}

  // proto:  int QImageIOHandler::nextImageDelay();
impl<'a> /*trait*/ QImageIOHandler_nextImageDelay<i32> for () {
  fn nextImageDelay(self , rsthis: & QImageIOHandler) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QImageIOHandler14nextImageDelayEv()};
    let mut ret = unsafe {_ZNK15QImageIOHandler14nextImageDelayEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QImageIOHandler::setDevice(QIODevice * device);
impl /*struct*/ QImageIOHandler {
  pub fn setDevice<RetType, T: QImageIOHandler_setDevice<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDevice(self);
    // return 1;
  }
}

pub trait QImageIOHandler_setDevice<RetType> {
  fn setDevice(self , rsthis: & QImageIOHandler) -> RetType;
}

  // proto:  void QImageIOHandler::setDevice(QIODevice * device);
impl<'a> /*trait*/ QImageIOHandler_setDevice<()> for (&'a QIODevice) {
  fn setDevice(self , rsthis: & QImageIOHandler) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QImageIOHandler9setDeviceEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QImageIOHandler9setDeviceEP9QIODevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QImageIOHandler::canRead();
impl /*struct*/ QImageIOHandler {
  pub fn canRead<RetType, T: QImageIOHandler_canRead<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.canRead(self);
    // return 1;
  }
}

pub trait QImageIOHandler_canRead<RetType> {
  fn canRead(self , rsthis: & QImageIOHandler) -> RetType;
}

  // proto:  bool QImageIOHandler::canRead();
impl<'a> /*trait*/ QImageIOHandler_canRead<i8> for () {
  fn canRead(self , rsthis: & QImageIOHandler) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QImageIOHandler7canReadEv()};
    let mut ret = unsafe {_ZNK15QImageIOHandler7canReadEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QIODevice * QImageIOHandler::device();
impl /*struct*/ QImageIOHandler {
  pub fn device<RetType, T: QImageIOHandler_device<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.device(self);
    // return 1;
  }
}

pub trait QImageIOHandler_device<RetType> {
  fn device(self , rsthis: & QImageIOHandler) -> RetType;
}

  // proto:  QIODevice * QImageIOHandler::device();
impl<'a> /*trait*/ QImageIOHandler_device<QIODevice> for () {
  fn device(self , rsthis: & QImageIOHandler) -> QIODevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QImageIOHandler6deviceEv()};
    let mut ret = unsafe {_ZNK15QImageIOHandler6deviceEv(rsthis.qclsinst)};
    let mut ret1 = QIODevice::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QImageIOHandler::write(const QImage & image);
impl /*struct*/ QImageIOHandler {
  pub fn write<RetType, T: QImageIOHandler_write<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.write(self);
    // return 1;
  }
}

pub trait QImageIOHandler_write<RetType> {
  fn write(self , rsthis: & QImageIOHandler) -> RetType;
}

  // proto:  bool QImageIOHandler::write(const QImage & image);
impl<'a> /*trait*/ QImageIOHandler_write<i8> for (&'a QImage) {
  fn write(self , rsthis: & QImageIOHandler) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QImageIOHandler5writeERK6QImage()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN15QImageIOHandler5writeERK6QImage(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QImageIOPlugin {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QImageIOPlugin {
    return QImageIOPlugin{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QImageIOPlugin {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QImageIOPlugin {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  const QMetaObject * QImageIOPlugin::metaObject();
impl /*struct*/ QImageIOPlugin {
  pub fn metaObject<RetType, T: QImageIOPlugin_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QImageIOPlugin_metaObject<RetType> {
  fn metaObject(self , rsthis: & QImageIOPlugin) -> RetType;
}

  // proto:  const QMetaObject * QImageIOPlugin::metaObject();
impl<'a> /*trait*/ QImageIOPlugin_metaObject<()> for () {
  fn metaObject(self , rsthis: & QImageIOPlugin) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QImageIOPlugin10metaObjectEv()};
     unsafe {_ZNK14QImageIOPlugin10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QImageIOPlugin::~QImageIOPlugin();
impl /*struct*/ QImageIOPlugin {
  pub fn Free<RetType, T: QImageIOPlugin_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QImageIOPlugin_Free<RetType> {
  fn Free(self , rsthis: & QImageIOPlugin) -> RetType;
}

  // proto:  void QImageIOPlugin::~QImageIOPlugin();
impl<'a> /*trait*/ QImageIOPlugin_Free<()> for () {
  fn Free(self , rsthis: & QImageIOPlugin) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QImageIOPluginD0Ev()};
     unsafe {_ZN14QImageIOPluginD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QImageIOHandler * QImageIOPlugin::create(QIODevice * device, const QByteArray & format);
impl /*struct*/ QImageIOPlugin {
  pub fn create<RetType, T: QImageIOPlugin_create<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.create(self);
    // return 1;
  }
}

pub trait QImageIOPlugin_create<RetType> {
  fn create(self , rsthis: & QImageIOPlugin) -> RetType;
}

  // proto:  QImageIOHandler * QImageIOPlugin::create(QIODevice * device, const QByteArray & format);
impl<'a> /*trait*/ QImageIOPlugin_create<QImageIOHandler> for (&'a QIODevice, &'a QByteArray) {
  fn create(self , rsthis: & QImageIOPlugin) -> QImageIOHandler {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QImageIOPlugin6createEP9QIODeviceRK10QByteArray()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK14QImageIOPlugin6createEP9QIODeviceRK10QByteArray(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QImageIOHandler::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QImageIOPlugin::QImageIOPlugin(QObject * parent);
impl /*struct*/ QImageIOPlugin {
  pub fn New<T: QImageIOPlugin_New>(value: T) -> QImageIOPlugin {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QImageIOPlugin_New {
  fn New(self) -> QImageIOPlugin;
}

  // proto:  void QImageIOPlugin::QImageIOPlugin(QObject * parent);
impl<'a> /*trait*/ QImageIOPlugin_New for (&'a QObject) {
  fn New(self) -> QImageIOPlugin {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QImageIOPluginC1EP7QObject()};
    let ctysz: c_int = unsafe{QImageIOPlugin_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN14QImageIOPluginC1EP7QObject(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN14QImageIOPluginC1EP7QObject(arg0)} as u64;
    let rsthis = QImageIOPlugin{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

