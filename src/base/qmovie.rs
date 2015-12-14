// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;
use super::qsize::QSize;
use super::qiodevice::QIODevice;
use super::qimage::QImage;
use super::qstring::QString;
use super::qbytearray::QByteArray;
use super::qrect::QRect;
use super::qcolor::QColor;
use super::qpixmap::QPixmap;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QMovie::NewQMovie(QObject * parent);
  fn _ZN6QMovieC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QMovie::speed();
  fn _ZNK6QMovie5speedEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QMovie::jumpToNextFrame();
  fn _ZN6QMovie15jumpToNextFrameEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QMovie::frameChanged(int frameNumber);
  fn _ZN6QMovie12frameChangedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QMovie::frameCount();
  fn _ZNK6QMovie10frameCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QMovie::setScaledSize(const QSize & size);
  fn _ZN6QMovie13setScaledSizeERK5QSize(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QMovie::setDevice(QIODevice * device);
  fn _ZN6QMovie9setDeviceEP9QIODevice(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QImage QMovie::currentImage();
  fn _ZNK6QMovie12currentImageEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QMovie::jumpToFrame(int frameNumber);
  fn _ZN6QMovie11jumpToFrameEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  void QMovie::started();
  fn _ZN6QMovie7startedEv(qthis: *mut c_void) ;
  // proto:  void QMovie::NewQMovie(const QString & fileName, const QByteArray & format, QObject * parent);
  fn _ZN6QMovieC1ERK7QStringRK10QByteArrayP7QObject(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  void QMovie::finished();
  fn _ZN6QMovie8finishedEv(qthis: *mut c_void) ;
  // proto:  const QMetaObject * QMovie::metaObject();
  fn _ZNK6QMovie10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QMovie::FreeQMovie();
  fn _ZN6QMovieD0Ev(qthis: *mut c_void) ;
  // proto:  void QMovie::start();
  fn _ZN6QMovie5startEv(qthis: *mut c_void) ;
  // proto:  int QMovie::loopCount();
  fn _ZNK6QMovie9loopCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QMovie::NewQMovie(QIODevice * device, const QByteArray & format, QObject * parent);
  fn _ZN6QMovieC1EP9QIODeviceRK10QByteArrayP7QObject(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  void QMovie::setFormat(const QByteArray & format);
  fn _ZN6QMovie9setFormatERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QMovie::resized(const QSize & size);
  fn _ZN6QMovie7resizedERK5QSize(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static QList<QByteArray> QMovie::supportedFormats();
  fn _ZN6QMovie16supportedFormatsEv() ;
  // proto:  QRect QMovie::frameRect();
  fn _ZNK6QMovie9frameRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMovie::setPaused(bool paused);
  fn _ZN6QMovie9setPausedEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QSize QMovie::scaledSize();
  fn _ZN6QMovie10scaledSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QIODevice * QMovie::device();
  fn _ZNK6QMovie6deviceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMovie::setBackgroundColor(const QColor & color);
  fn _ZN6QMovie18setBackgroundColorERK6QColor(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QMovie::isValid();
  fn _ZNK6QMovie7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QMovie::setSpeed(int percentSpeed);
  fn _ZN6QMovie8setSpeedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QMovie::NewQMovie(const QMovie & );
  fn _ZN6QMovieC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QMovie::stop();
  fn _ZN6QMovie4stopEv(qthis: *mut c_void) ;
  // proto:  int QMovie::currentFrameNumber();
  fn _ZNK6QMovie18currentFrameNumberEv(qthis: *mut c_void) -> c_int;
  // proto:  int QMovie::nextFrameDelay();
  fn _ZNK6QMovie14nextFrameDelayEv(qthis: *mut c_void) -> c_int;
  // proto:  QPixmap QMovie::currentPixmap();
  fn _ZNK6QMovie13currentPixmapEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QByteArray QMovie::format();
  fn _ZNK6QMovie6formatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QMovie::fileName();
  fn _ZNK6QMovie8fileNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMovie::updated(const QRect & rect);
  fn _ZN6QMovie7updatedERK5QRect(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QColor QMovie::backgroundColor();
  fn _ZNK6QMovie15backgroundColorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMovie::setFileName(const QString & fileName);
  fn _ZN6QMovie11setFileNameERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QMovie)=1
pub struct QMovie {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMovie {
  pub fn NewQMovie<T: QMovie_NewQMovie>(value: T) -> QMovie {
    let rsthis = value.NewQMovie();
    return rsthis;
    // return 1;
  }
}

pub trait QMovie_NewQMovie {
  fn NewQMovie(self) -> QMovie;
}

// proto: void QMovie::NewQMovie(QObject * parent);
impl<'a> /*trait*/ QMovie_NewQMovie for (&'a mut QObject) {
  fn NewQMovie(self) -> QMovie {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovieC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QMovieC1EP7QObject(qthis, arg0)};
    let rsthis = QMovie{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn speed<T: QMovie_speed>(&mut self, value: T) -> i32 {
    return value.speed(self);
    // return 1;
  }
}

pub trait QMovie_speed {
  fn speed(self, rsthis: &mut QMovie) -> i32;
}

// proto:  int QMovie::speed();
impl<'a> /*trait*/ QMovie_speed for () {
  fn speed(self, rsthis: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie5speedEv()};
    let mut ret = unsafe {_ZNK6QMovie5speedEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn jumpToNextFrame<T: QMovie_jumpToNextFrame>(&mut self, value: T) -> i8 {
    return value.jumpToNextFrame(self);
    // return 1;
  }
}

pub trait QMovie_jumpToNextFrame {
  fn jumpToNextFrame(self, rsthis: &mut QMovie) -> i8;
}

// proto:  bool QMovie::jumpToNextFrame();
impl<'a> /*trait*/ QMovie_jumpToNextFrame for () {
  fn jumpToNextFrame(self, rsthis: &mut QMovie) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie15jumpToNextFrameEv()};
    let mut ret = unsafe {_ZN6QMovie15jumpToNextFrameEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn frameChanged<T: QMovie_frameChanged>(&mut self, value: T)  {
     value.frameChanged(self);
    // return 1;
  }
}

pub trait QMovie_frameChanged {
  fn frameChanged(self, rsthis: &mut QMovie) ;
}

// proto:  void QMovie::frameChanged(int frameNumber);
impl<'a> /*trait*/ QMovie_frameChanged for (i32) {
  fn frameChanged(self, rsthis: &mut QMovie)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie12frameChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QMovie12frameChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn frameCount<T: QMovie_frameCount>(&mut self, value: T) -> i32 {
    return value.frameCount(self);
    // return 1;
  }
}

pub trait QMovie_frameCount {
  fn frameCount(self, rsthis: &mut QMovie) -> i32;
}

// proto:  int QMovie::frameCount();
impl<'a> /*trait*/ QMovie_frameCount for () {
  fn frameCount(self, rsthis: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie10frameCountEv()};
    let mut ret = unsafe {_ZNK6QMovie10frameCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn setScaledSize<T: QMovie_setScaledSize>(&mut self, value: T)  {
     value.setScaledSize(self);
    // return 1;
  }
}

pub trait QMovie_setScaledSize {
  fn setScaledSize(self, rsthis: &mut QMovie) ;
}

// proto:  void QMovie::setScaledSize(const QSize & size);
impl<'a> /*trait*/ QMovie_setScaledSize for (&'a  QSize) {
  fn setScaledSize(self, rsthis: &mut QMovie)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie13setScaledSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QMovie13setScaledSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn setDevice<T: QMovie_setDevice>(&mut self, value: T)  {
     value.setDevice(self);
    // return 1;
  }
}

pub trait QMovie_setDevice {
  fn setDevice(self, rsthis: &mut QMovie) ;
}

// proto:  void QMovie::setDevice(QIODevice * device);
impl<'a> /*trait*/ QMovie_setDevice for (&'a mut QIODevice) {
  fn setDevice(self, rsthis: &mut QMovie)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie9setDeviceEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QMovie9setDeviceEP9QIODevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn currentImage<T: QMovie_currentImage>(&mut self, value: T) -> QImage {
    return value.currentImage(self);
    // return 1;
  }
}

pub trait QMovie_currentImage {
  fn currentImage(self, rsthis: &mut QMovie) -> QImage;
}

// proto:  QImage QMovie::currentImage();
impl<'a> /*trait*/ QMovie_currentImage for () {
  fn currentImage(self, rsthis: &mut QMovie) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie12currentImageEv()};
    let mut ret = unsafe {_ZNK6QMovie12currentImageEv(rsthis.qclsinst)};
    let mut ret1 = QImage{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn jumpToFrame<T: QMovie_jumpToFrame>(&mut self, value: T) -> i8 {
    return value.jumpToFrame(self);
    // return 1;
  }
}

pub trait QMovie_jumpToFrame {
  fn jumpToFrame(self, rsthis: &mut QMovie) -> i8;
}

// proto:  bool QMovie::jumpToFrame(int frameNumber);
impl<'a> /*trait*/ QMovie_jumpToFrame for (i32) {
  fn jumpToFrame(self, rsthis: &mut QMovie) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie11jumpToFrameEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN6QMovie11jumpToFrameEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn started<T: QMovie_started>(&mut self, value: T)  {
     value.started(self);
    // return 1;
  }
}

pub trait QMovie_started {
  fn started(self, rsthis: &mut QMovie) ;
}

// proto:  void QMovie::started();
impl<'a> /*trait*/ QMovie_started for () {
  fn started(self, rsthis: &mut QMovie)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie7startedEv()};
     unsafe {_ZN6QMovie7startedEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QMovie::NewQMovie(const QString & fileName, const QByteArray & format, QObject * parent);
impl<'a> /*trait*/ QMovie_NewQMovie for (&'a  QString, &'a  QByteArray, &'a mut QObject) {
  fn NewQMovie(self) -> QMovie {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovieC1ERK7QStringRK10QByteArrayP7QObject()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN6QMovieC1ERK7QStringRK10QByteArrayP7QObject(qthis, arg0, arg1, arg2)};
    let rsthis = QMovie{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn finished<T: QMovie_finished>(&mut self, value: T)  {
     value.finished(self);
    // return 1;
  }
}

pub trait QMovie_finished {
  fn finished(self, rsthis: &mut QMovie) ;
}

// proto:  void QMovie::finished();
impl<'a> /*trait*/ QMovie_finished for () {
  fn finished(self, rsthis: &mut QMovie)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie8finishedEv()};
     unsafe {_ZN6QMovie8finishedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn metaObject<T: QMovie_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QMovie_metaObject {
  fn metaObject(self, rsthis: &mut QMovie) ;
}

// proto:  const QMetaObject * QMovie::metaObject();
impl<'a> /*trait*/ QMovie_metaObject for () {
  fn metaObject(self, rsthis: &mut QMovie)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie10metaObjectEv()};
     unsafe {_ZNK6QMovie10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn FreeQMovie<T: QMovie_FreeQMovie>(&mut self, value: T)  {
     value.FreeQMovie(self);
    // return 1;
  }
}

pub trait QMovie_FreeQMovie {
  fn FreeQMovie(self, rsthis: &mut QMovie) ;
}

// proto:  void QMovie::FreeQMovie();
impl<'a> /*trait*/ QMovie_FreeQMovie for () {
  fn FreeQMovie(self, rsthis: &mut QMovie)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovieD0Ev()};
     unsafe {_ZN6QMovieD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn start<T: QMovie_start>(&mut self, value: T)  {
     value.start(self);
    // return 1;
  }
}

pub trait QMovie_start {
  fn start(self, rsthis: &mut QMovie) ;
}

// proto:  void QMovie::start();
impl<'a> /*trait*/ QMovie_start for () {
  fn start(self, rsthis: &mut QMovie)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie5startEv()};
     unsafe {_ZN6QMovie5startEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn loopCount<T: QMovie_loopCount>(&mut self, value: T) -> i32 {
    return value.loopCount(self);
    // return 1;
  }
}

pub trait QMovie_loopCount {
  fn loopCount(self, rsthis: &mut QMovie) -> i32;
}

// proto:  int QMovie::loopCount();
impl<'a> /*trait*/ QMovie_loopCount for () {
  fn loopCount(self, rsthis: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie9loopCountEv()};
    let mut ret = unsafe {_ZNK6QMovie9loopCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto: void QMovie::NewQMovie(QIODevice * device, const QByteArray & format, QObject * parent);
impl<'a> /*trait*/ QMovie_NewQMovie for (&'a mut QIODevice, &'a  QByteArray, &'a mut QObject) {
  fn NewQMovie(self) -> QMovie {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovieC1EP9QIODeviceRK10QByteArrayP7QObject()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN6QMovieC1EP9QIODeviceRK10QByteArrayP7QObject(qthis, arg0, arg1, arg2)};
    let rsthis = QMovie{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn setFormat<T: QMovie_setFormat>(&mut self, value: T)  {
     value.setFormat(self);
    // return 1;
  }
}

pub trait QMovie_setFormat {
  fn setFormat(self, rsthis: &mut QMovie) ;
}

// proto:  void QMovie::setFormat(const QByteArray & format);
impl<'a> /*trait*/ QMovie_setFormat for (&'a  QByteArray) {
  fn setFormat(self, rsthis: &mut QMovie)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie9setFormatERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QMovie9setFormatERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn resized<T: QMovie_resized>(&mut self, value: T)  {
     value.resized(self);
    // return 1;
  }
}

pub trait QMovie_resized {
  fn resized(self, rsthis: &mut QMovie) ;
}

// proto:  void QMovie::resized(const QSize & size);
impl<'a> /*trait*/ QMovie_resized for (&'a  QSize) {
  fn resized(self, rsthis: &mut QMovie)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie7resizedERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QMovie7resizedERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn supportedFormats<T: QMovie_supportedFormats>(&mut self, value: T)  {
     value.supportedFormats(self);
    // return 1;
  }
}

pub trait QMovie_supportedFormats {
  fn supportedFormats(self, rsthis: &mut QMovie) ;
}

// proto: static QList<QByteArray> QMovie::supportedFormats();
impl<'a> /*trait*/ QMovie_supportedFormats for () {
  fn supportedFormats(self, rsthis: &mut QMovie)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie16supportedFormatsEv()};
     unsafe {_ZN6QMovie16supportedFormatsEv()};
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn frameRect<T: QMovie_frameRect>(&mut self, value: T) -> QRect {
    return value.frameRect(self);
    // return 1;
  }
}

pub trait QMovie_frameRect {
  fn frameRect(self, rsthis: &mut QMovie) -> QRect;
}

// proto:  QRect QMovie::frameRect();
impl<'a> /*trait*/ QMovie_frameRect for () {
  fn frameRect(self, rsthis: &mut QMovie) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie9frameRectEv()};
    let mut ret = unsafe {_ZNK6QMovie9frameRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn setPaused<T: QMovie_setPaused>(&mut self, value: T)  {
     value.setPaused(self);
    // return 1;
  }
}

pub trait QMovie_setPaused {
  fn setPaused(self, rsthis: &mut QMovie) ;
}

// proto:  void QMovie::setPaused(bool paused);
impl<'a> /*trait*/ QMovie_setPaused for (i8) {
  fn setPaused(self, rsthis: &mut QMovie)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie9setPausedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN6QMovie9setPausedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn scaledSize<T: QMovie_scaledSize>(&mut self, value: T) -> QSize {
    return value.scaledSize(self);
    // return 1;
  }
}

pub trait QMovie_scaledSize {
  fn scaledSize(self, rsthis: &mut QMovie) -> QSize;
}

// proto:  QSize QMovie::scaledSize();
impl<'a> /*trait*/ QMovie_scaledSize for () {
  fn scaledSize(self, rsthis: &mut QMovie) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie10scaledSizeEv()};
    let mut ret = unsafe {_ZN6QMovie10scaledSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn device<T: QMovie_device>(&mut self, value: T) -> QIODevice {
    return value.device(self);
    // return 1;
  }
}

pub trait QMovie_device {
  fn device(self, rsthis: &mut QMovie) -> QIODevice;
}

// proto:  QIODevice * QMovie::device();
impl<'a> /*trait*/ QMovie_device for () {
  fn device(self, rsthis: &mut QMovie) -> QIODevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie6deviceEv()};
    let mut ret = unsafe {_ZNK6QMovie6deviceEv(rsthis.qclsinst)};
    let mut ret1 = QIODevice{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn setBackgroundColor<T: QMovie_setBackgroundColor>(&mut self, value: T)  {
     value.setBackgroundColor(self);
    // return 1;
  }
}

pub trait QMovie_setBackgroundColor {
  fn setBackgroundColor(self, rsthis: &mut QMovie) ;
}

// proto:  void QMovie::setBackgroundColor(const QColor & color);
impl<'a> /*trait*/ QMovie_setBackgroundColor for (&'a  QColor) {
  fn setBackgroundColor(self, rsthis: &mut QMovie)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie18setBackgroundColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QMovie18setBackgroundColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn isValid<T: QMovie_isValid>(&mut self, value: T) -> i8 {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QMovie_isValid {
  fn isValid(self, rsthis: &mut QMovie) -> i8;
}

// proto:  bool QMovie::isValid();
impl<'a> /*trait*/ QMovie_isValid for () {
  fn isValid(self, rsthis: &mut QMovie) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie7isValidEv()};
    let mut ret = unsafe {_ZNK6QMovie7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn setSpeed<T: QMovie_setSpeed>(&mut self, value: T)  {
     value.setSpeed(self);
    // return 1;
  }
}

pub trait QMovie_setSpeed {
  fn setSpeed(self, rsthis: &mut QMovie) ;
}

// proto:  void QMovie::setSpeed(int percentSpeed);
impl<'a> /*trait*/ QMovie_setSpeed for (i32) {
  fn setSpeed(self, rsthis: &mut QMovie)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie8setSpeedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QMovie8setSpeedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QMovie::NewQMovie(const QMovie & );
impl<'a> /*trait*/ QMovie_NewQMovie for (&'a  QMovie) {
  fn NewQMovie(self) -> QMovie {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovieC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QMovieC1ERKS_(qthis, arg0)};
    let rsthis = QMovie{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn stop<T: QMovie_stop>(&mut self, value: T)  {
     value.stop(self);
    // return 1;
  }
}

pub trait QMovie_stop {
  fn stop(self, rsthis: &mut QMovie) ;
}

// proto:  void QMovie::stop();
impl<'a> /*trait*/ QMovie_stop for () {
  fn stop(self, rsthis: &mut QMovie)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie4stopEv()};
     unsafe {_ZN6QMovie4stopEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn currentFrameNumber<T: QMovie_currentFrameNumber>(&mut self, value: T) -> i32 {
    return value.currentFrameNumber(self);
    // return 1;
  }
}

pub trait QMovie_currentFrameNumber {
  fn currentFrameNumber(self, rsthis: &mut QMovie) -> i32;
}

// proto:  int QMovie::currentFrameNumber();
impl<'a> /*trait*/ QMovie_currentFrameNumber for () {
  fn currentFrameNumber(self, rsthis: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie18currentFrameNumberEv()};
    let mut ret = unsafe {_ZNK6QMovie18currentFrameNumberEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn nextFrameDelay<T: QMovie_nextFrameDelay>(&mut self, value: T) -> i32 {
    return value.nextFrameDelay(self);
    // return 1;
  }
}

pub trait QMovie_nextFrameDelay {
  fn nextFrameDelay(self, rsthis: &mut QMovie) -> i32;
}

// proto:  int QMovie::nextFrameDelay();
impl<'a> /*trait*/ QMovie_nextFrameDelay for () {
  fn nextFrameDelay(self, rsthis: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie14nextFrameDelayEv()};
    let mut ret = unsafe {_ZNK6QMovie14nextFrameDelayEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn currentPixmap<T: QMovie_currentPixmap>(&mut self, value: T) -> QPixmap {
    return value.currentPixmap(self);
    // return 1;
  }
}

pub trait QMovie_currentPixmap {
  fn currentPixmap(self, rsthis: &mut QMovie) -> QPixmap;
}

// proto:  QPixmap QMovie::currentPixmap();
impl<'a> /*trait*/ QMovie_currentPixmap for () {
  fn currentPixmap(self, rsthis: &mut QMovie) -> QPixmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie13currentPixmapEv()};
    let mut ret = unsafe {_ZNK6QMovie13currentPixmapEv(rsthis.qclsinst)};
    let mut ret1 = QPixmap{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn format<T: QMovie_format>(&mut self, value: T) -> QByteArray {
    return value.format(self);
    // return 1;
  }
}

pub trait QMovie_format {
  fn format(self, rsthis: &mut QMovie) -> QByteArray;
}

// proto:  QByteArray QMovie::format();
impl<'a> /*trait*/ QMovie_format for () {
  fn format(self, rsthis: &mut QMovie) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie6formatEv()};
    let mut ret = unsafe {_ZNK6QMovie6formatEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn fileName<T: QMovie_fileName>(&mut self, value: T) -> QString {
    return value.fileName(self);
    // return 1;
  }
}

pub trait QMovie_fileName {
  fn fileName(self, rsthis: &mut QMovie) -> QString;
}

// proto:  QString QMovie::fileName();
impl<'a> /*trait*/ QMovie_fileName for () {
  fn fileName(self, rsthis: &mut QMovie) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie8fileNameEv()};
    let mut ret = unsafe {_ZNK6QMovie8fileNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn updated<T: QMovie_updated>(&mut self, value: T)  {
     value.updated(self);
    // return 1;
  }
}

pub trait QMovie_updated {
  fn updated(self, rsthis: &mut QMovie) ;
}

// proto:  void QMovie::updated(const QRect & rect);
impl<'a> /*trait*/ QMovie_updated for (&'a  QRect) {
  fn updated(self, rsthis: &mut QMovie)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie7updatedERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QMovie7updatedERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn backgroundColor<T: QMovie_backgroundColor>(&mut self, value: T) -> QColor {
    return value.backgroundColor(self);
    // return 1;
  }
}

pub trait QMovie_backgroundColor {
  fn backgroundColor(self, rsthis: &mut QMovie) -> QColor;
}

// proto:  QColor QMovie::backgroundColor();
impl<'a> /*trait*/ QMovie_backgroundColor for () {
  fn backgroundColor(self, rsthis: &mut QMovie) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie15backgroundColorEv()};
    let mut ret = unsafe {_ZNK6QMovie15backgroundColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn setFileName<T: QMovie_setFileName>(&mut self, value: T)  {
     value.setFileName(self);
    // return 1;
  }
}

pub trait QMovie_setFileName {
  fn setFileName(self, rsthis: &mut QMovie) ;
}

// proto:  void QMovie::setFileName(const QString & fileName);
impl<'a> /*trait*/ QMovie_setFileName for (&'a  QString) {
  fn setFileName(self, rsthis: &mut QMovie)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie11setFileNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QMovie11setFileNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

