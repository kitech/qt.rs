// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: static QSurfaceFormat QSurfaceFormat::defaultFormat();
  fn _ZN14QSurfaceFormat13defaultFormatEv() -> *mut c_void;
  // proto:  void QSurfaceFormat::setAlphaBufferSize(int size);
  fn _ZN14QSurfaceFormat18setAlphaBufferSizeEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QSurfaceFormat::setMinorVersion(int minorVersion);
  fn _ZN14QSurfaceFormat15setMinorVersionEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QSurfaceFormat::stencilBufferSize();
  fn _ZNK14QSurfaceFormat17stencilBufferSizeEv(qthis: *mut c_void) -> c_int;
  // proto:  void QSurfaceFormat::setRedBufferSize(int size);
  fn _ZN14QSurfaceFormat16setRedBufferSizeEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QSurfaceFormat::setDepthBufferSize(int size);
  fn _ZN14QSurfaceFormat18setDepthBufferSizeEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QSurfaceFormat::majorVersion();
  fn _ZNK14QSurfaceFormat12majorVersionEv(qthis: *mut c_void) -> c_int;
  // proto:  void QSurfaceFormat::setSamples(int numSamples);
  fn _ZN14QSurfaceFormat10setSamplesEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QSurfaceFormat::setMajorVersion(int majorVersion);
  fn _ZN14QSurfaceFormat15setMajorVersionEi(qthis: *mut c_void, arg0: c_int);
  // proto: static void QSurfaceFormat::setDefaultFormat(const QSurfaceFormat & format);
  fn _ZN14QSurfaceFormat16setDefaultFormatERKS_(arg0: *mut c_void);
  // proto:  int QSurfaceFormat::greenBufferSize();
  fn _ZNK14QSurfaceFormat15greenBufferSizeEv(qthis: *mut c_void) -> c_int;
  // proto:  int QSurfaceFormat::minorVersion();
  fn _ZNK14QSurfaceFormat12minorVersionEv(qthis: *mut c_void) -> c_int;
  // proto:  void QSurfaceFormat::setStencilBufferSize(int size);
  fn _ZN14QSurfaceFormat20setStencilBufferSizeEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QSurfaceFormat::swapInterval();
  fn _ZNK14QSurfaceFormat12swapIntervalEv(qthis: *mut c_void) -> c_int;
  // proto:  void QSurfaceFormat::setVersion(int major, int minor);
  fn _ZN14QSurfaceFormat10setVersionEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  bool QSurfaceFormat::hasAlpha();
  fn _ZNK14QSurfaceFormat8hasAlphaEv(qthis: *mut c_void) -> c_char;
  // proto:  void QSurfaceFormat::QSurfaceFormat(const QSurfaceFormat & other);
  fn _ZN14QSurfaceFormatC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QPair<int, int> QSurfaceFormat::version();
  fn _ZNK14QSurfaceFormat7versionEv(qthis: *mut c_void);
  // proto:  int QSurfaceFormat::blueBufferSize();
  fn _ZNK14QSurfaceFormat14blueBufferSizeEv(qthis: *mut c_void) -> c_int;
  // proto:  void QSurfaceFormat::QSurfaceFormat();
  fn _ZN14QSurfaceFormatC1Ev(qthis: *mut c_void);
  // proto:  int QSurfaceFormat::redBufferSize();
  fn _ZNK14QSurfaceFormat13redBufferSizeEv(qthis: *mut c_void) -> c_int;
  // proto:  void QSurfaceFormat::~QSurfaceFormat();
  fn _ZN14QSurfaceFormatD0Ev(qthis: *mut c_void);
  // proto:  void QSurfaceFormat::setGreenBufferSize(int size);
  fn _ZN14QSurfaceFormat18setGreenBufferSizeEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QSurfaceFormat::samples();
  fn _ZNK14QSurfaceFormat7samplesEv(qthis: *mut c_void) -> c_int;
  // proto:  int QSurfaceFormat::depthBufferSize();
  fn _ZNK14QSurfaceFormat15depthBufferSizeEv(qthis: *mut c_void) -> c_int;
  // proto:  void QSurfaceFormat::setBlueBufferSize(int size);
  fn _ZN14QSurfaceFormat17setBlueBufferSizeEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QSurfaceFormat::alphaBufferSize();
  fn _ZNK14QSurfaceFormat15alphaBufferSizeEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QSurfaceFormat::stereo();
  fn _ZNK14QSurfaceFormat6stereoEv(qthis: *mut c_void) -> c_char;
  // proto:  void QSurfaceFormat::setSwapInterval(int interval);
  fn _ZN14QSurfaceFormat15setSwapIntervalEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QSurfaceFormat::setStereo(bool enable);
  fn _ZN14QSurfaceFormat9setStereoEb(qthis: *mut c_void, arg0: c_char);
}

// body block begin
// class sizeof(QSurfaceFormat)=8
pub struct QSurfaceFormat {
  pub qclsinst: *mut c_void,
}

  // proto: static QSurfaceFormat QSurfaceFormat::defaultFormat();
impl /*struct*/ QSurfaceFormat {
  pub fn defaultFormat_s<RetType, T: QSurfaceFormat_defaultFormat_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.defaultFormat_s();
    // return 1;
  }
}

pub trait QSurfaceFormat_defaultFormat_s<RetType> {
  fn defaultFormat_s(self ) -> RetType;
}

  // proto: static QSurfaceFormat QSurfaceFormat::defaultFormat();
impl<'a> /*trait*/ QSurfaceFormat_defaultFormat_s<QSurfaceFormat> for () {
  fn defaultFormat_s(self ) -> QSurfaceFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat13defaultFormatEv()};
    let mut ret = unsafe {_ZN14QSurfaceFormat13defaultFormatEv()};
    let mut ret1 = QSurfaceFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QSurfaceFormat::setAlphaBufferSize(int size);
impl /*struct*/ QSurfaceFormat {
  pub fn setAlphaBufferSize<RetType, T: QSurfaceFormat_setAlphaBufferSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setAlphaBufferSize(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_setAlphaBufferSize<RetType> {
  fn setAlphaBufferSize(self , rsthis: &mut QSurfaceFormat) -> RetType;
}

  // proto:  void QSurfaceFormat::setAlphaBufferSize(int size);
impl<'a> /*trait*/ QSurfaceFormat_setAlphaBufferSize<()> for (i32) {
  fn setAlphaBufferSize(self , rsthis: &mut QSurfaceFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat18setAlphaBufferSizeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QSurfaceFormat18setAlphaBufferSizeEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSurfaceFormat::setMinorVersion(int minorVersion);
impl /*struct*/ QSurfaceFormat {
  pub fn setMinorVersion<RetType, T: QSurfaceFormat_setMinorVersion<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setMinorVersion(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_setMinorVersion<RetType> {
  fn setMinorVersion(self , rsthis: &mut QSurfaceFormat) -> RetType;
}

  // proto:  void QSurfaceFormat::setMinorVersion(int minorVersion);
impl<'a> /*trait*/ QSurfaceFormat_setMinorVersion<()> for (i32) {
  fn setMinorVersion(self , rsthis: &mut QSurfaceFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat15setMinorVersionEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QSurfaceFormat15setMinorVersionEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QSurfaceFormat::stencilBufferSize();
impl /*struct*/ QSurfaceFormat {
  pub fn stencilBufferSize<RetType, T: QSurfaceFormat_stencilBufferSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.stencilBufferSize(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_stencilBufferSize<RetType> {
  fn stencilBufferSize(self , rsthis: &mut QSurfaceFormat) -> RetType;
}

  // proto:  int QSurfaceFormat::stencilBufferSize();
impl<'a> /*trait*/ QSurfaceFormat_stencilBufferSize<i32> for () {
  fn stencilBufferSize(self , rsthis: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSurfaceFormat17stencilBufferSizeEv()};
    let mut ret = unsafe {_ZNK14QSurfaceFormat17stencilBufferSizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QSurfaceFormat::setRedBufferSize(int size);
impl /*struct*/ QSurfaceFormat {
  pub fn setRedBufferSize<RetType, T: QSurfaceFormat_setRedBufferSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setRedBufferSize(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_setRedBufferSize<RetType> {
  fn setRedBufferSize(self , rsthis: &mut QSurfaceFormat) -> RetType;
}

  // proto:  void QSurfaceFormat::setRedBufferSize(int size);
impl<'a> /*trait*/ QSurfaceFormat_setRedBufferSize<()> for (i32) {
  fn setRedBufferSize(self , rsthis: &mut QSurfaceFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat16setRedBufferSizeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QSurfaceFormat16setRedBufferSizeEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSurfaceFormat::setDepthBufferSize(int size);
impl /*struct*/ QSurfaceFormat {
  pub fn setDepthBufferSize<RetType, T: QSurfaceFormat_setDepthBufferSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDepthBufferSize(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_setDepthBufferSize<RetType> {
  fn setDepthBufferSize(self , rsthis: &mut QSurfaceFormat) -> RetType;
}

  // proto:  void QSurfaceFormat::setDepthBufferSize(int size);
impl<'a> /*trait*/ QSurfaceFormat_setDepthBufferSize<()> for (i32) {
  fn setDepthBufferSize(self , rsthis: &mut QSurfaceFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat18setDepthBufferSizeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QSurfaceFormat18setDepthBufferSizeEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QSurfaceFormat::majorVersion();
impl /*struct*/ QSurfaceFormat {
  pub fn majorVersion<RetType, T: QSurfaceFormat_majorVersion<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.majorVersion(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_majorVersion<RetType> {
  fn majorVersion(self , rsthis: &mut QSurfaceFormat) -> RetType;
}

  // proto:  int QSurfaceFormat::majorVersion();
impl<'a> /*trait*/ QSurfaceFormat_majorVersion<i32> for () {
  fn majorVersion(self , rsthis: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSurfaceFormat12majorVersionEv()};
    let mut ret = unsafe {_ZNK14QSurfaceFormat12majorVersionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QSurfaceFormat::setSamples(int numSamples);
impl /*struct*/ QSurfaceFormat {
  pub fn setSamples<RetType, T: QSurfaceFormat_setSamples<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setSamples(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_setSamples<RetType> {
  fn setSamples(self , rsthis: &mut QSurfaceFormat) -> RetType;
}

  // proto:  void QSurfaceFormat::setSamples(int numSamples);
impl<'a> /*trait*/ QSurfaceFormat_setSamples<()> for (i32) {
  fn setSamples(self , rsthis: &mut QSurfaceFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat10setSamplesEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QSurfaceFormat10setSamplesEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSurfaceFormat::setMajorVersion(int majorVersion);
impl /*struct*/ QSurfaceFormat {
  pub fn setMajorVersion<RetType, T: QSurfaceFormat_setMajorVersion<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setMajorVersion(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_setMajorVersion<RetType> {
  fn setMajorVersion(self , rsthis: &mut QSurfaceFormat) -> RetType;
}

  // proto:  void QSurfaceFormat::setMajorVersion(int majorVersion);
impl<'a> /*trait*/ QSurfaceFormat_setMajorVersion<()> for (i32) {
  fn setMajorVersion(self , rsthis: &mut QSurfaceFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat15setMajorVersionEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QSurfaceFormat15setMajorVersionEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static void QSurfaceFormat::setDefaultFormat(const QSurfaceFormat & format);
impl /*struct*/ QSurfaceFormat {
  pub fn setDefaultFormat_s<RetType, T: QSurfaceFormat_setDefaultFormat_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setDefaultFormat_s();
    // return 1;
  }
}

pub trait QSurfaceFormat_setDefaultFormat_s<RetType> {
  fn setDefaultFormat_s(self ) -> RetType;
}

  // proto: static void QSurfaceFormat::setDefaultFormat(const QSurfaceFormat & format);
impl<'a> /*trait*/ QSurfaceFormat_setDefaultFormat_s<()> for (QSurfaceFormat) {
  fn setDefaultFormat_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat16setDefaultFormatERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QSurfaceFormat16setDefaultFormatERKS_(arg0)};
    // return 1;
  }
}

  // proto:  int QSurfaceFormat::greenBufferSize();
impl /*struct*/ QSurfaceFormat {
  pub fn greenBufferSize<RetType, T: QSurfaceFormat_greenBufferSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.greenBufferSize(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_greenBufferSize<RetType> {
  fn greenBufferSize(self , rsthis: &mut QSurfaceFormat) -> RetType;
}

  // proto:  int QSurfaceFormat::greenBufferSize();
impl<'a> /*trait*/ QSurfaceFormat_greenBufferSize<i32> for () {
  fn greenBufferSize(self , rsthis: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSurfaceFormat15greenBufferSizeEv()};
    let mut ret = unsafe {_ZNK14QSurfaceFormat15greenBufferSizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QSurfaceFormat::minorVersion();
impl /*struct*/ QSurfaceFormat {
  pub fn minorVersion<RetType, T: QSurfaceFormat_minorVersion<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.minorVersion(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_minorVersion<RetType> {
  fn minorVersion(self , rsthis: &mut QSurfaceFormat) -> RetType;
}

  // proto:  int QSurfaceFormat::minorVersion();
impl<'a> /*trait*/ QSurfaceFormat_minorVersion<i32> for () {
  fn minorVersion(self , rsthis: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSurfaceFormat12minorVersionEv()};
    let mut ret = unsafe {_ZNK14QSurfaceFormat12minorVersionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QSurfaceFormat::setStencilBufferSize(int size);
impl /*struct*/ QSurfaceFormat {
  pub fn setStencilBufferSize<RetType, T: QSurfaceFormat_setStencilBufferSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setStencilBufferSize(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_setStencilBufferSize<RetType> {
  fn setStencilBufferSize(self , rsthis: &mut QSurfaceFormat) -> RetType;
}

  // proto:  void QSurfaceFormat::setStencilBufferSize(int size);
impl<'a> /*trait*/ QSurfaceFormat_setStencilBufferSize<()> for (i32) {
  fn setStencilBufferSize(self , rsthis: &mut QSurfaceFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat20setStencilBufferSizeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QSurfaceFormat20setStencilBufferSizeEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QSurfaceFormat::swapInterval();
impl /*struct*/ QSurfaceFormat {
  pub fn swapInterval<RetType, T: QSurfaceFormat_swapInterval<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.swapInterval(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_swapInterval<RetType> {
  fn swapInterval(self , rsthis: &mut QSurfaceFormat) -> RetType;
}

  // proto:  int QSurfaceFormat::swapInterval();
impl<'a> /*trait*/ QSurfaceFormat_swapInterval<i32> for () {
  fn swapInterval(self , rsthis: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSurfaceFormat12swapIntervalEv()};
    let mut ret = unsafe {_ZNK14QSurfaceFormat12swapIntervalEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QSurfaceFormat::setVersion(int major, int minor);
impl /*struct*/ QSurfaceFormat {
  pub fn setVersion<RetType, T: QSurfaceFormat_setVersion<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setVersion(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_setVersion<RetType> {
  fn setVersion(self , rsthis: &mut QSurfaceFormat) -> RetType;
}

  // proto:  void QSurfaceFormat::setVersion(int major, int minor);
impl<'a> /*trait*/ QSurfaceFormat_setVersion<()> for (i32, i32) {
  fn setVersion(self , rsthis: &mut QSurfaceFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat10setVersionEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN14QSurfaceFormat10setVersionEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QSurfaceFormat::hasAlpha();
impl /*struct*/ QSurfaceFormat {
  pub fn hasAlpha<RetType, T: QSurfaceFormat_hasAlpha<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.hasAlpha(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_hasAlpha<RetType> {
  fn hasAlpha(self , rsthis: &mut QSurfaceFormat) -> RetType;
}

  // proto:  bool QSurfaceFormat::hasAlpha();
impl<'a> /*trait*/ QSurfaceFormat_hasAlpha<i8> for () {
  fn hasAlpha(self , rsthis: &mut QSurfaceFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSurfaceFormat8hasAlphaEv()};
    let mut ret = unsafe {_ZNK14QSurfaceFormat8hasAlphaEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QSurfaceFormat::QSurfaceFormat(const QSurfaceFormat & other);
impl /*struct*/ QSurfaceFormat {
  pub fn NewQSurfaceFormat<T: QSurfaceFormat_NewQSurfaceFormat>(value: T) -> QSurfaceFormat {
    let rsthis = value.NewQSurfaceFormat();
    return rsthis;
    // return 1;
  }
}

pub trait QSurfaceFormat_NewQSurfaceFormat {
  fn NewQSurfaceFormat(self) -> QSurfaceFormat;
}

  // proto:  void QSurfaceFormat::QSurfaceFormat(const QSurfaceFormat & other);
impl<'a> /*trait*/ QSurfaceFormat_NewQSurfaceFormat for (QSurfaceFormat) {
  fn NewQSurfaceFormat(self) -> QSurfaceFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormatC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QSurfaceFormatC1ERKS_(qthis, arg0)};
    let rsthis = QSurfaceFormat{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPair<int, int> QSurfaceFormat::version();
impl /*struct*/ QSurfaceFormat {
  pub fn version<RetType, T: QSurfaceFormat_version<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.version(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_version<RetType> {
  fn version(self , rsthis: &mut QSurfaceFormat) -> RetType;
}

  // proto:  QPair<int, int> QSurfaceFormat::version();
impl<'a> /*trait*/ QSurfaceFormat_version<()> for () {
  fn version(self , rsthis: &mut QSurfaceFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSurfaceFormat7versionEv()};
     unsafe {_ZNK14QSurfaceFormat7versionEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QSurfaceFormat::blueBufferSize();
impl /*struct*/ QSurfaceFormat {
  pub fn blueBufferSize<RetType, T: QSurfaceFormat_blueBufferSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.blueBufferSize(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_blueBufferSize<RetType> {
  fn blueBufferSize(self , rsthis: &mut QSurfaceFormat) -> RetType;
}

  // proto:  int QSurfaceFormat::blueBufferSize();
impl<'a> /*trait*/ QSurfaceFormat_blueBufferSize<i32> for () {
  fn blueBufferSize(self , rsthis: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSurfaceFormat14blueBufferSizeEv()};
    let mut ret = unsafe {_ZNK14QSurfaceFormat14blueBufferSizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QSurfaceFormat::QSurfaceFormat();
impl<'a> /*trait*/ QSurfaceFormat_NewQSurfaceFormat for () {
  fn NewQSurfaceFormat(self) -> QSurfaceFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormatC1Ev()};
    unsafe {_ZN14QSurfaceFormatC1Ev(qthis)};
    let rsthis = QSurfaceFormat{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QSurfaceFormat::redBufferSize();
impl /*struct*/ QSurfaceFormat {
  pub fn redBufferSize<RetType, T: QSurfaceFormat_redBufferSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.redBufferSize(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_redBufferSize<RetType> {
  fn redBufferSize(self , rsthis: &mut QSurfaceFormat) -> RetType;
}

  // proto:  int QSurfaceFormat::redBufferSize();
impl<'a> /*trait*/ QSurfaceFormat_redBufferSize<i32> for () {
  fn redBufferSize(self , rsthis: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSurfaceFormat13redBufferSizeEv()};
    let mut ret = unsafe {_ZNK14QSurfaceFormat13redBufferSizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QSurfaceFormat::~QSurfaceFormat();
impl /*struct*/ QSurfaceFormat {
  pub fn FreeQSurfaceFormat<RetType, T: QSurfaceFormat_FreeQSurfaceFormat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQSurfaceFormat(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_FreeQSurfaceFormat<RetType> {
  fn FreeQSurfaceFormat(self , rsthis: &mut QSurfaceFormat) -> RetType;
}

  // proto:  void QSurfaceFormat::~QSurfaceFormat();
impl<'a> /*trait*/ QSurfaceFormat_FreeQSurfaceFormat<()> for () {
  fn FreeQSurfaceFormat(self , rsthis: &mut QSurfaceFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormatD0Ev()};
     unsafe {_ZN14QSurfaceFormatD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSurfaceFormat::setGreenBufferSize(int size);
impl /*struct*/ QSurfaceFormat {
  pub fn setGreenBufferSize<RetType, T: QSurfaceFormat_setGreenBufferSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setGreenBufferSize(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_setGreenBufferSize<RetType> {
  fn setGreenBufferSize(self , rsthis: &mut QSurfaceFormat) -> RetType;
}

  // proto:  void QSurfaceFormat::setGreenBufferSize(int size);
impl<'a> /*trait*/ QSurfaceFormat_setGreenBufferSize<()> for (i32) {
  fn setGreenBufferSize(self , rsthis: &mut QSurfaceFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat18setGreenBufferSizeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QSurfaceFormat18setGreenBufferSizeEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QSurfaceFormat::samples();
impl /*struct*/ QSurfaceFormat {
  pub fn samples<RetType, T: QSurfaceFormat_samples<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.samples(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_samples<RetType> {
  fn samples(self , rsthis: &mut QSurfaceFormat) -> RetType;
}

  // proto:  int QSurfaceFormat::samples();
impl<'a> /*trait*/ QSurfaceFormat_samples<i32> for () {
  fn samples(self , rsthis: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSurfaceFormat7samplesEv()};
    let mut ret = unsafe {_ZNK14QSurfaceFormat7samplesEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QSurfaceFormat::depthBufferSize();
impl /*struct*/ QSurfaceFormat {
  pub fn depthBufferSize<RetType, T: QSurfaceFormat_depthBufferSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.depthBufferSize(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_depthBufferSize<RetType> {
  fn depthBufferSize(self , rsthis: &mut QSurfaceFormat) -> RetType;
}

  // proto:  int QSurfaceFormat::depthBufferSize();
impl<'a> /*trait*/ QSurfaceFormat_depthBufferSize<i32> for () {
  fn depthBufferSize(self , rsthis: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSurfaceFormat15depthBufferSizeEv()};
    let mut ret = unsafe {_ZNK14QSurfaceFormat15depthBufferSizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QSurfaceFormat::setBlueBufferSize(int size);
impl /*struct*/ QSurfaceFormat {
  pub fn setBlueBufferSize<RetType, T: QSurfaceFormat_setBlueBufferSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setBlueBufferSize(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_setBlueBufferSize<RetType> {
  fn setBlueBufferSize(self , rsthis: &mut QSurfaceFormat) -> RetType;
}

  // proto:  void QSurfaceFormat::setBlueBufferSize(int size);
impl<'a> /*trait*/ QSurfaceFormat_setBlueBufferSize<()> for (i32) {
  fn setBlueBufferSize(self , rsthis: &mut QSurfaceFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat17setBlueBufferSizeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QSurfaceFormat17setBlueBufferSizeEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QSurfaceFormat::alphaBufferSize();
impl /*struct*/ QSurfaceFormat {
  pub fn alphaBufferSize<RetType, T: QSurfaceFormat_alphaBufferSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.alphaBufferSize(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_alphaBufferSize<RetType> {
  fn alphaBufferSize(self , rsthis: &mut QSurfaceFormat) -> RetType;
}

  // proto:  int QSurfaceFormat::alphaBufferSize();
impl<'a> /*trait*/ QSurfaceFormat_alphaBufferSize<i32> for () {
  fn alphaBufferSize(self , rsthis: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSurfaceFormat15alphaBufferSizeEv()};
    let mut ret = unsafe {_ZNK14QSurfaceFormat15alphaBufferSizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QSurfaceFormat::stereo();
impl /*struct*/ QSurfaceFormat {
  pub fn stereo<RetType, T: QSurfaceFormat_stereo<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.stereo(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_stereo<RetType> {
  fn stereo(self , rsthis: &mut QSurfaceFormat) -> RetType;
}

  // proto:  bool QSurfaceFormat::stereo();
impl<'a> /*trait*/ QSurfaceFormat_stereo<i8> for () {
  fn stereo(self , rsthis: &mut QSurfaceFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSurfaceFormat6stereoEv()};
    let mut ret = unsafe {_ZNK14QSurfaceFormat6stereoEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QSurfaceFormat::setSwapInterval(int interval);
impl /*struct*/ QSurfaceFormat {
  pub fn setSwapInterval<RetType, T: QSurfaceFormat_setSwapInterval<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setSwapInterval(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_setSwapInterval<RetType> {
  fn setSwapInterval(self , rsthis: &mut QSurfaceFormat) -> RetType;
}

  // proto:  void QSurfaceFormat::setSwapInterval(int interval);
impl<'a> /*trait*/ QSurfaceFormat_setSwapInterval<()> for (i32) {
  fn setSwapInterval(self , rsthis: &mut QSurfaceFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat15setSwapIntervalEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QSurfaceFormat15setSwapIntervalEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSurfaceFormat::setStereo(bool enable);
impl /*struct*/ QSurfaceFormat {
  pub fn setStereo<RetType, T: QSurfaceFormat_setStereo<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setStereo(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_setStereo<RetType> {
  fn setStereo(self , rsthis: &mut QSurfaceFormat) -> RetType;
}

  // proto:  void QSurfaceFormat::setStereo(bool enable);
impl<'a> /*trait*/ QSurfaceFormat_setStereo<()> for (i8) {
  fn setStereo(self , rsthis: &mut QSurfaceFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat9setStereoEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN14QSurfaceFormat9setStereoEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

