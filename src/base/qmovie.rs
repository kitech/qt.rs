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
use super::qstring::QString;
use super::qbytearray::QByteArray;
use super::qcolor::QColor;
use super::qrect::QRect;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QMovie::NewQMovie(QObject * parent);
  fn _ZN6QMovieC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: int QMovie::speed();
  fn _ZNK6QMovie5speedEv() -> i32;
  // proto: bool QMovie::jumpToNextFrame();
  fn _ZN6QMovie15jumpToNextFrameEv() -> i32;
  // proto: void QMovie::frameChanged(int frameNumber);
  fn _ZN6QMovie12frameChangedEi(arg0: c_int) -> i32;
  // proto: int QMovie::frameCount();
  fn _ZNK6QMovie10frameCountEv() -> i32;
  // proto: void QMovie::setScaledSize(const QSize & size);
  fn _ZN6QMovie13setScaledSizeERK5QSize(arg0: *const c_void) -> i32;
  // proto: void QMovie::setDevice(QIODevice * device);
  fn _ZN6QMovie9setDeviceEP9QIODevice(arg0: *mut c_void) -> i32;
  // proto: QImage QMovie::currentImage();
  fn _ZNK6QMovie12currentImageEv() -> i32;
  // proto: bool QMovie::jumpToFrame(int frameNumber);
  fn _ZN6QMovie11jumpToFrameEi(arg0: c_int) -> i32;
  // proto: void QMovie::started();
  fn _ZN6QMovie7startedEv() -> i32;
  // proto: void QMovie::NewQMovie(const QString & fileName, const QByteArray & format, QObject * parent);
  fn _ZN6QMovieC1ERK7QStringRK10QByteArrayP7QObject(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_void, arg2: *mut c_void) -> i32;
  // proto: void QMovie::finished();
  fn _ZN6QMovie8finishedEv() -> i32;
  // proto: const QMetaObject * QMovie::metaObject();
  fn _ZNK6QMovie10metaObjectEv() -> i32;
  // proto: void QMovie::FreeQMovie();
  fn _ZN6QMovieD0Ev() -> i32;
  // proto: void QMovie::start();
  fn _ZN6QMovie5startEv() -> i32;
  // proto: int QMovie::loopCount();
  fn _ZNK6QMovie9loopCountEv() -> i32;
  // proto: void QMovie::NewQMovie(QIODevice * device, const QByteArray & format, QObject * parent);
  fn _ZN6QMovieC1EP9QIODeviceRK10QByteArrayP7QObject(qthis: *mut c_void, arg0: *mut c_void, arg1: *const c_void, arg2: *mut c_void) -> i32;
  // proto: void QMovie::setFormat(const QByteArray & format);
  fn _ZN6QMovie9setFormatERK10QByteArray(arg0: *const c_void) -> i32;
  // proto: void QMovie::resized(const QSize & size);
  fn _ZN6QMovie7resizedERK5QSize(arg0: *const c_void) -> i32;
  // proto: QList<QByteArray> QMovie::supportedFormats();
  fn _ZN6QMovie16supportedFormatsEv() -> i32;
  // proto: QRect QMovie::frameRect();
  fn _ZNK6QMovie9frameRectEv() -> i32;
  // proto: void QMovie::setPaused(bool paused);
  fn _ZN6QMovie9setPausedEb(arg0: int8_t) -> i32;
  // proto: QSize QMovie::scaledSize();
  fn _ZN6QMovie10scaledSizeEv() -> i32;
  // proto: QIODevice * QMovie::device();
  fn _ZNK6QMovie6deviceEv() -> i32;
  // proto: void QMovie::setBackgroundColor(const QColor & color);
  fn _ZN6QMovie18setBackgroundColorERK6QColor(arg0: *const c_void) -> i32;
  // proto: bool QMovie::isValid();
  fn _ZNK6QMovie7isValidEv() -> i32;
  // proto: void QMovie::setSpeed(int percentSpeed);
  fn _ZN6QMovie8setSpeedEi(arg0: c_int) -> i32;
  // proto: void QMovie::NewQMovie(const QMovie & );
  fn _ZN6QMovieC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QMovie::stop();
  fn _ZN6QMovie4stopEv() -> i32;
  // proto: int QMovie::currentFrameNumber();
  fn _ZNK6QMovie18currentFrameNumberEv() -> i32;
  // proto: int QMovie::nextFrameDelay();
  fn _ZNK6QMovie14nextFrameDelayEv() -> i32;
  // proto: QPixmap QMovie::currentPixmap();
  fn _ZNK6QMovie13currentPixmapEv() -> i32;
  // proto: QByteArray QMovie::format();
  fn _ZNK6QMovie6formatEv() -> i32;
  // proto: QString QMovie::fileName();
  fn _ZNK6QMovie8fileNameEv() -> i32;
  // proto: void QMovie::updated(const QRect & rect);
  fn _ZN6QMovie7updatedERK5QRect(arg0: *const c_void) -> i32;
  // proto: QColor QMovie::backgroundColor();
  fn _ZNK6QMovie15backgroundColorEv() -> i32;
  // proto: void QMovie::setFileName(const QString & fileName);
  fn _ZN6QMovie11setFileNameERK7QString(arg0: *const c_void) -> i32;
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
    value.speed(self);
    return 1;
  }
}

pub trait QMovie_speed {
  fn speed(self, this: &mut QMovie) -> i32;
}

// proto: int QMovie::speed();
impl<'a> /*trait*/ QMovie_speed for () {
  fn speed(self, this: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie5speedEv()};
    unsafe {_ZNK6QMovie5speedEv()};
    return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn jumpToNextFrame<T: QMovie_jumpToNextFrame>(&mut self, value: T) -> i32 {
    value.jumpToNextFrame(self);
    return 1;
  }
}

pub trait QMovie_jumpToNextFrame {
  fn jumpToNextFrame(self, this: &mut QMovie) -> i32;
}

// proto: bool QMovie::jumpToNextFrame();
impl<'a> /*trait*/ QMovie_jumpToNextFrame for () {
  fn jumpToNextFrame(self, this: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie15jumpToNextFrameEv()};
    unsafe {_ZN6QMovie15jumpToNextFrameEv()};
    return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn frameChanged<T: QMovie_frameChanged>(&mut self, value: T) -> i32 {
    value.frameChanged(self);
    return 1;
  }
}

pub trait QMovie_frameChanged {
  fn frameChanged(self, this: &mut QMovie) -> i32;
}

// proto: void QMovie::frameChanged(int frameNumber);
impl<'a> /*trait*/ QMovie_frameChanged for (i32) {
  fn frameChanged(self, this: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie12frameChangedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN6QMovie12frameChangedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn frameCount<T: QMovie_frameCount>(&mut self, value: T) -> i32 {
    value.frameCount(self);
    return 1;
  }
}

pub trait QMovie_frameCount {
  fn frameCount(self, this: &mut QMovie) -> i32;
}

// proto: int QMovie::frameCount();
impl<'a> /*trait*/ QMovie_frameCount for () {
  fn frameCount(self, this: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie10frameCountEv()};
    unsafe {_ZNK6QMovie10frameCountEv()};
    return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn setScaledSize<T: QMovie_setScaledSize>(&mut self, value: T) -> i32 {
    value.setScaledSize(self);
    return 1;
  }
}

pub trait QMovie_setScaledSize {
  fn setScaledSize(self, this: &mut QMovie) -> i32;
}

// proto: void QMovie::setScaledSize(const QSize & size);
impl<'a> /*trait*/ QMovie_setScaledSize for (&'a  QSize) {
  fn setScaledSize(self, this: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie13setScaledSizeERK5QSize()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QMovie13setScaledSizeERK5QSize(arg0)};
    return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn setDevice<T: QMovie_setDevice>(&mut self, value: T) -> i32 {
    value.setDevice(self);
    return 1;
  }
}

pub trait QMovie_setDevice {
  fn setDevice(self, this: &mut QMovie) -> i32;
}

// proto: void QMovie::setDevice(QIODevice * device);
impl<'a> /*trait*/ QMovie_setDevice for (&'a mut QIODevice) {
  fn setDevice(self, this: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie9setDeviceEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QMovie9setDeviceEP9QIODevice(arg0)};
    return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn currentImage<T: QMovie_currentImage>(&mut self, value: T) -> i32 {
    value.currentImage(self);
    return 1;
  }
}

pub trait QMovie_currentImage {
  fn currentImage(self, this: &mut QMovie) -> i32;
}

// proto: QImage QMovie::currentImage();
impl<'a> /*trait*/ QMovie_currentImage for () {
  fn currentImage(self, this: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie12currentImageEv()};
    unsafe {_ZNK6QMovie12currentImageEv()};
    return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn jumpToFrame<T: QMovie_jumpToFrame>(&mut self, value: T) -> i32 {
    value.jumpToFrame(self);
    return 1;
  }
}

pub trait QMovie_jumpToFrame {
  fn jumpToFrame(self, this: &mut QMovie) -> i32;
}

// proto: bool QMovie::jumpToFrame(int frameNumber);
impl<'a> /*trait*/ QMovie_jumpToFrame for (i32) {
  fn jumpToFrame(self, this: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie11jumpToFrameEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN6QMovie11jumpToFrameEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn started<T: QMovie_started>(&mut self, value: T) -> i32 {
    value.started(self);
    return 1;
  }
}

pub trait QMovie_started {
  fn started(self, this: &mut QMovie) -> i32;
}

// proto: void QMovie::started();
impl<'a> /*trait*/ QMovie_started for () {
  fn started(self, this: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie7startedEv()};
    unsafe {_ZN6QMovie7startedEv()};
    return 1;
  }
}

// proto: void QMovie::NewQMovie(const QString & fileName, const QByteArray & format, QObject * parent);
impl<'a> /*trait*/ QMovie_NewQMovie for (&'a  QString, &'a  QByteArray, &'a mut QObject) {
  fn NewQMovie(self) -> QMovie {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovieC1ERK7QStringRK10QByteArrayP7QObject()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN6QMovieC1ERK7QStringRK10QByteArrayP7QObject(qthis, arg0, arg1, arg2)};
    let rsthis = QMovie{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn finished<T: QMovie_finished>(&mut self, value: T) -> i32 {
    value.finished(self);
    return 1;
  }
}

pub trait QMovie_finished {
  fn finished(self, this: &mut QMovie) -> i32;
}

// proto: void QMovie::finished();
impl<'a> /*trait*/ QMovie_finished for () {
  fn finished(self, this: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie8finishedEv()};
    unsafe {_ZN6QMovie8finishedEv()};
    return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn metaObject<T: QMovie_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QMovie_metaObject {
  fn metaObject(self, this: &mut QMovie) -> i32;
}

// proto: const QMetaObject * QMovie::metaObject();
impl<'a> /*trait*/ QMovie_metaObject for () {
  fn metaObject(self, this: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie10metaObjectEv()};
    unsafe {_ZNK6QMovie10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn FreeQMovie<T: QMovie_FreeQMovie>(&mut self, value: T) -> i32 {
    value.FreeQMovie(self);
    return 1;
  }
}

pub trait QMovie_FreeQMovie {
  fn FreeQMovie(self, this: &mut QMovie) -> i32;
}

// proto: void QMovie::FreeQMovie();
impl<'a> /*trait*/ QMovie_FreeQMovie for () {
  fn FreeQMovie(self, this: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovieD0Ev()};
    unsafe {_ZN6QMovieD0Ev()};
    return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn start<T: QMovie_start>(&mut self, value: T) -> i32 {
    value.start(self);
    return 1;
  }
}

pub trait QMovie_start {
  fn start(self, this: &mut QMovie) -> i32;
}

// proto: void QMovie::start();
impl<'a> /*trait*/ QMovie_start for () {
  fn start(self, this: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie5startEv()};
    unsafe {_ZN6QMovie5startEv()};
    return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn loopCount<T: QMovie_loopCount>(&mut self, value: T) -> i32 {
    value.loopCount(self);
    return 1;
  }
}

pub trait QMovie_loopCount {
  fn loopCount(self, this: &mut QMovie) -> i32;
}

// proto: int QMovie::loopCount();
impl<'a> /*trait*/ QMovie_loopCount for () {
  fn loopCount(self, this: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie9loopCountEv()};
    unsafe {_ZNK6QMovie9loopCountEv()};
    return 1;
  }
}

// proto: void QMovie::NewQMovie(QIODevice * device, const QByteArray & format, QObject * parent);
impl<'a> /*trait*/ QMovie_NewQMovie for (&'a mut QIODevice, &'a  QByteArray, &'a mut QObject) {
  fn NewQMovie(self) -> QMovie {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovieC1EP9QIODeviceRK10QByteArrayP7QObject()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN6QMovieC1EP9QIODeviceRK10QByteArrayP7QObject(qthis, arg0, arg1, arg2)};
    let rsthis = QMovie{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn setFormat<T: QMovie_setFormat>(&mut self, value: T) -> i32 {
    value.setFormat(self);
    return 1;
  }
}

pub trait QMovie_setFormat {
  fn setFormat(self, this: &mut QMovie) -> i32;
}

// proto: void QMovie::setFormat(const QByteArray & format);
impl<'a> /*trait*/ QMovie_setFormat for (&'a  QByteArray) {
  fn setFormat(self, this: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie9setFormatERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QMovie9setFormatERK10QByteArray(arg0)};
    return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn resized<T: QMovie_resized>(&mut self, value: T) -> i32 {
    value.resized(self);
    return 1;
  }
}

pub trait QMovie_resized {
  fn resized(self, this: &mut QMovie) -> i32;
}

// proto: void QMovie::resized(const QSize & size);
impl<'a> /*trait*/ QMovie_resized for (&'a  QSize) {
  fn resized(self, this: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie7resizedERK5QSize()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QMovie7resizedERK5QSize(arg0)};
    return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn supportedFormats<T: QMovie_supportedFormats>(&mut self, value: T) -> i32 {
    value.supportedFormats(self);
    return 1;
  }
}

pub trait QMovie_supportedFormats {
  fn supportedFormats(self, this: &mut QMovie) -> i32;
}

// proto: QList<QByteArray> QMovie::supportedFormats();
impl<'a> /*trait*/ QMovie_supportedFormats for () {
  fn supportedFormats(self, this: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie16supportedFormatsEv()};
    unsafe {_ZN6QMovie16supportedFormatsEv()};
    return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn frameRect<T: QMovie_frameRect>(&mut self, value: T) -> i32 {
    value.frameRect(self);
    return 1;
  }
}

pub trait QMovie_frameRect {
  fn frameRect(self, this: &mut QMovie) -> i32;
}

// proto: QRect QMovie::frameRect();
impl<'a> /*trait*/ QMovie_frameRect for () {
  fn frameRect(self, this: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie9frameRectEv()};
    unsafe {_ZNK6QMovie9frameRectEv()};
    return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn setPaused<T: QMovie_setPaused>(&mut self, value: T) -> i32 {
    value.setPaused(self);
    return 1;
  }
}

pub trait QMovie_setPaused {
  fn setPaused(self, this: &mut QMovie) -> i32;
}

// proto: void QMovie::setPaused(bool paused);
impl<'a> /*trait*/ QMovie_setPaused for (i8) {
  fn setPaused(self, this: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie9setPausedEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN6QMovie9setPausedEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn scaledSize<T: QMovie_scaledSize>(&mut self, value: T) -> i32 {
    value.scaledSize(self);
    return 1;
  }
}

pub trait QMovie_scaledSize {
  fn scaledSize(self, this: &mut QMovie) -> i32;
}

// proto: QSize QMovie::scaledSize();
impl<'a> /*trait*/ QMovie_scaledSize for () {
  fn scaledSize(self, this: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie10scaledSizeEv()};
    unsafe {_ZN6QMovie10scaledSizeEv()};
    return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn device<T: QMovie_device>(&mut self, value: T) -> i32 {
    value.device(self);
    return 1;
  }
}

pub trait QMovie_device {
  fn device(self, this: &mut QMovie) -> i32;
}

// proto: QIODevice * QMovie::device();
impl<'a> /*trait*/ QMovie_device for () {
  fn device(self, this: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie6deviceEv()};
    unsafe {_ZNK6QMovie6deviceEv()};
    return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn setBackgroundColor<T: QMovie_setBackgroundColor>(&mut self, value: T) -> i32 {
    value.setBackgroundColor(self);
    return 1;
  }
}

pub trait QMovie_setBackgroundColor {
  fn setBackgroundColor(self, this: &mut QMovie) -> i32;
}

// proto: void QMovie::setBackgroundColor(const QColor & color);
impl<'a> /*trait*/ QMovie_setBackgroundColor for (&'a  QColor) {
  fn setBackgroundColor(self, this: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie18setBackgroundColorERK6QColor()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QMovie18setBackgroundColorERK6QColor(arg0)};
    return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn isValid<T: QMovie_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QMovie_isValid {
  fn isValid(self, this: &mut QMovie) -> i32;
}

// proto: bool QMovie::isValid();
impl<'a> /*trait*/ QMovie_isValid for () {
  fn isValid(self, this: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie7isValidEv()};
    unsafe {_ZNK6QMovie7isValidEv()};
    return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn setSpeed<T: QMovie_setSpeed>(&mut self, value: T) -> i32 {
    value.setSpeed(self);
    return 1;
  }
}

pub trait QMovie_setSpeed {
  fn setSpeed(self, this: &mut QMovie) -> i32;
}

// proto: void QMovie::setSpeed(int percentSpeed);
impl<'a> /*trait*/ QMovie_setSpeed for (i32) {
  fn setSpeed(self, this: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie8setSpeedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN6QMovie8setSpeedEi(arg0)};
    return 1;
  }
}

// proto: void QMovie::NewQMovie(const QMovie & );
impl<'a> /*trait*/ QMovie_NewQMovie for (&'a  QMovie) {
  fn NewQMovie(self) -> QMovie {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovieC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QMovieC1ERKS_(qthis, arg0)};
    let rsthis = QMovie{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn stop<T: QMovie_stop>(&mut self, value: T) -> i32 {
    value.stop(self);
    return 1;
  }
}

pub trait QMovie_stop {
  fn stop(self, this: &mut QMovie) -> i32;
}

// proto: void QMovie::stop();
impl<'a> /*trait*/ QMovie_stop for () {
  fn stop(self, this: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie4stopEv()};
    unsafe {_ZN6QMovie4stopEv()};
    return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn currentFrameNumber<T: QMovie_currentFrameNumber>(&mut self, value: T) -> i32 {
    value.currentFrameNumber(self);
    return 1;
  }
}

pub trait QMovie_currentFrameNumber {
  fn currentFrameNumber(self, this: &mut QMovie) -> i32;
}

// proto: int QMovie::currentFrameNumber();
impl<'a> /*trait*/ QMovie_currentFrameNumber for () {
  fn currentFrameNumber(self, this: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie18currentFrameNumberEv()};
    unsafe {_ZNK6QMovie18currentFrameNumberEv()};
    return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn nextFrameDelay<T: QMovie_nextFrameDelay>(&mut self, value: T) -> i32 {
    value.nextFrameDelay(self);
    return 1;
  }
}

pub trait QMovie_nextFrameDelay {
  fn nextFrameDelay(self, this: &mut QMovie) -> i32;
}

// proto: int QMovie::nextFrameDelay();
impl<'a> /*trait*/ QMovie_nextFrameDelay for () {
  fn nextFrameDelay(self, this: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie14nextFrameDelayEv()};
    unsafe {_ZNK6QMovie14nextFrameDelayEv()};
    return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn currentPixmap<T: QMovie_currentPixmap>(&mut self, value: T) -> i32 {
    value.currentPixmap(self);
    return 1;
  }
}

pub trait QMovie_currentPixmap {
  fn currentPixmap(self, this: &mut QMovie) -> i32;
}

// proto: QPixmap QMovie::currentPixmap();
impl<'a> /*trait*/ QMovie_currentPixmap for () {
  fn currentPixmap(self, this: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie13currentPixmapEv()};
    unsafe {_ZNK6QMovie13currentPixmapEv()};
    return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn format<T: QMovie_format>(&mut self, value: T) -> i32 {
    value.format(self);
    return 1;
  }
}

pub trait QMovie_format {
  fn format(self, this: &mut QMovie) -> i32;
}

// proto: QByteArray QMovie::format();
impl<'a> /*trait*/ QMovie_format for () {
  fn format(self, this: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie6formatEv()};
    unsafe {_ZNK6QMovie6formatEv()};
    return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn fileName<T: QMovie_fileName>(&mut self, value: T) -> i32 {
    value.fileName(self);
    return 1;
  }
}

pub trait QMovie_fileName {
  fn fileName(self, this: &mut QMovie) -> i32;
}

// proto: QString QMovie::fileName();
impl<'a> /*trait*/ QMovie_fileName for () {
  fn fileName(self, this: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie8fileNameEv()};
    unsafe {_ZNK6QMovie8fileNameEv()};
    return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn updated<T: QMovie_updated>(&mut self, value: T) -> i32 {
    value.updated(self);
    return 1;
  }
}

pub trait QMovie_updated {
  fn updated(self, this: &mut QMovie) -> i32;
}

// proto: void QMovie::updated(const QRect & rect);
impl<'a> /*trait*/ QMovie_updated for (&'a  QRect) {
  fn updated(self, this: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie7updatedERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QMovie7updatedERK5QRect(arg0)};
    return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn backgroundColor<T: QMovie_backgroundColor>(&mut self, value: T) -> i32 {
    value.backgroundColor(self);
    return 1;
  }
}

pub trait QMovie_backgroundColor {
  fn backgroundColor(self, this: &mut QMovie) -> i32;
}

// proto: QColor QMovie::backgroundColor();
impl<'a> /*trait*/ QMovie_backgroundColor for () {
  fn backgroundColor(self, this: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie15backgroundColorEv()};
    unsafe {_ZNK6QMovie15backgroundColorEv()};
    return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn setFileName<T: QMovie_setFileName>(&mut self, value: T) -> i32 {
    value.setFileName(self);
    return 1;
  }
}

pub trait QMovie_setFileName {
  fn setFileName(self, this: &mut QMovie) -> i32;
}

// proto: void QMovie::setFileName(const QString & fileName);
impl<'a> /*trait*/ QMovie_setFileName for (&'a  QString) {
  fn setFileName(self, this: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie11setFileNameERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QMovie11setFileNameERK7QString(arg0)};
    return 1;
  }
}

