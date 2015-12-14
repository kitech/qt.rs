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
  fn _ZN14QSurfaceFormat18setAlphaBufferSizeEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QSurfaceFormat::setMinorVersion(int minorVersion);
  fn _ZN14QSurfaceFormat15setMinorVersionEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QSurfaceFormat::stencilBufferSize();
  fn _ZNK14QSurfaceFormat17stencilBufferSizeEv(qthis: *mut c_void) -> c_int;
  // proto:  void QSurfaceFormat::setRedBufferSize(int size);
  fn _ZN14QSurfaceFormat16setRedBufferSizeEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QSurfaceFormat::setDepthBufferSize(int size);
  fn _ZN14QSurfaceFormat18setDepthBufferSizeEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QSurfaceFormat::majorVersion();
  fn _ZNK14QSurfaceFormat12majorVersionEv(qthis: *mut c_void) -> c_int;
  // proto:  void QSurfaceFormat::setSamples(int numSamples);
  fn _ZN14QSurfaceFormat10setSamplesEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QSurfaceFormat::setMajorVersion(int majorVersion);
  fn _ZN14QSurfaceFormat15setMajorVersionEi(qthis: *mut c_void, arg0: c_int) ;
  // proto: static void QSurfaceFormat::setDefaultFormat(const QSurfaceFormat & format);
  fn _ZN14QSurfaceFormat16setDefaultFormatERKS_(arg0: *mut c_void) ;
  // proto:  int QSurfaceFormat::greenBufferSize();
  fn _ZNK14QSurfaceFormat15greenBufferSizeEv(qthis: *mut c_void) -> c_int;
  // proto:  int QSurfaceFormat::minorVersion();
  fn _ZNK14QSurfaceFormat12minorVersionEv(qthis: *mut c_void) -> c_int;
  // proto:  void QSurfaceFormat::setStencilBufferSize(int size);
  fn _ZN14QSurfaceFormat20setStencilBufferSizeEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QSurfaceFormat::swapInterval();
  fn _ZNK14QSurfaceFormat12swapIntervalEv(qthis: *mut c_void) -> c_int;
  // proto:  void QSurfaceFormat::setVersion(int major, int minor);
  fn _ZN14QSurfaceFormat10setVersionEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  bool QSurfaceFormat::hasAlpha();
  fn _ZNK14QSurfaceFormat8hasAlphaEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QSurfaceFormat::NewQSurfaceFormat(const QSurfaceFormat & other);
  fn _ZN14QSurfaceFormatC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QPair<int, int> QSurfaceFormat::version();
  fn _ZNK14QSurfaceFormat7versionEv(qthis: *mut c_void) ;
  // proto:  int QSurfaceFormat::blueBufferSize();
  fn _ZNK14QSurfaceFormat14blueBufferSizeEv(qthis: *mut c_void) -> c_int;
  // proto:  void QSurfaceFormat::NewQSurfaceFormat();
  fn _ZN14QSurfaceFormatC1Ev(qthis: *mut c_void) ;
  // proto:  int QSurfaceFormat::redBufferSize();
  fn _ZNK14QSurfaceFormat13redBufferSizeEv(qthis: *mut c_void) -> c_int;
  // proto:  void QSurfaceFormat::FreeQSurfaceFormat();
  fn _ZN14QSurfaceFormatD0Ev(qthis: *mut c_void) ;
  // proto:  void QSurfaceFormat::setGreenBufferSize(int size);
  fn _ZN14QSurfaceFormat18setGreenBufferSizeEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QSurfaceFormat::samples();
  fn _ZNK14QSurfaceFormat7samplesEv(qthis: *mut c_void) -> c_int;
  // proto:  int QSurfaceFormat::depthBufferSize();
  fn _ZNK14QSurfaceFormat15depthBufferSizeEv(qthis: *mut c_void) -> c_int;
  // proto:  void QSurfaceFormat::setBlueBufferSize(int size);
  fn _ZN14QSurfaceFormat17setBlueBufferSizeEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QSurfaceFormat::alphaBufferSize();
  fn _ZNK14QSurfaceFormat15alphaBufferSizeEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QSurfaceFormat::stereo();
  fn _ZNK14QSurfaceFormat6stereoEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QSurfaceFormat::setSwapInterval(int interval);
  fn _ZN14QSurfaceFormat15setSwapIntervalEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QSurfaceFormat::setStereo(bool enable);
  fn _ZN14QSurfaceFormat9setStereoEb(qthis: *mut c_void, arg0: int8_t) ;
}

// body block begin
// class sizeof(QSurfaceFormat)=8
pub struct QSurfaceFormat {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSurfaceFormat {
  pub fn defaultFormat<T: QSurfaceFormat_defaultFormat>(&mut self, value: T) -> QSurfaceFormat {
    return value.defaultFormat(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_defaultFormat {
  fn defaultFormat(self, rsthis: &mut QSurfaceFormat) -> QSurfaceFormat;
}

// proto: static QSurfaceFormat QSurfaceFormat::defaultFormat();
impl<'a> /*trait*/ QSurfaceFormat_defaultFormat for () {
  fn defaultFormat(self, rsthis: &mut QSurfaceFormat) -> QSurfaceFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat13defaultFormatEv()};
    let mut ret = unsafe {_ZN14QSurfaceFormat13defaultFormatEv()};
    let mut ret1 = QSurfaceFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn setAlphaBufferSize<T: QSurfaceFormat_setAlphaBufferSize>(&mut self, value: T)  {
     value.setAlphaBufferSize(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_setAlphaBufferSize {
  fn setAlphaBufferSize(self, rsthis: &mut QSurfaceFormat) ;
}

// proto:  void QSurfaceFormat::setAlphaBufferSize(int size);
impl<'a> /*trait*/ QSurfaceFormat_setAlphaBufferSize for (i32) {
  fn setAlphaBufferSize(self, rsthis: &mut QSurfaceFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat18setAlphaBufferSizeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QSurfaceFormat18setAlphaBufferSizeEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn setMinorVersion<T: QSurfaceFormat_setMinorVersion>(&mut self, value: T)  {
     value.setMinorVersion(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_setMinorVersion {
  fn setMinorVersion(self, rsthis: &mut QSurfaceFormat) ;
}

// proto:  void QSurfaceFormat::setMinorVersion(int minorVersion);
impl<'a> /*trait*/ QSurfaceFormat_setMinorVersion for (i32) {
  fn setMinorVersion(self, rsthis: &mut QSurfaceFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat15setMinorVersionEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QSurfaceFormat15setMinorVersionEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn stencilBufferSize<T: QSurfaceFormat_stencilBufferSize>(&mut self, value: T) -> i32 {
    return value.stencilBufferSize(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_stencilBufferSize {
  fn stencilBufferSize(self, rsthis: &mut QSurfaceFormat) -> i32;
}

// proto:  int QSurfaceFormat::stencilBufferSize();
impl<'a> /*trait*/ QSurfaceFormat_stencilBufferSize for () {
  fn stencilBufferSize(self, rsthis: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSurfaceFormat17stencilBufferSizeEv()};
    let mut ret = unsafe {_ZNK14QSurfaceFormat17stencilBufferSizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn setRedBufferSize<T: QSurfaceFormat_setRedBufferSize>(&mut self, value: T)  {
     value.setRedBufferSize(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_setRedBufferSize {
  fn setRedBufferSize(self, rsthis: &mut QSurfaceFormat) ;
}

// proto:  void QSurfaceFormat::setRedBufferSize(int size);
impl<'a> /*trait*/ QSurfaceFormat_setRedBufferSize for (i32) {
  fn setRedBufferSize(self, rsthis: &mut QSurfaceFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat16setRedBufferSizeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QSurfaceFormat16setRedBufferSizeEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn setDepthBufferSize<T: QSurfaceFormat_setDepthBufferSize>(&mut self, value: T)  {
     value.setDepthBufferSize(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_setDepthBufferSize {
  fn setDepthBufferSize(self, rsthis: &mut QSurfaceFormat) ;
}

// proto:  void QSurfaceFormat::setDepthBufferSize(int size);
impl<'a> /*trait*/ QSurfaceFormat_setDepthBufferSize for (i32) {
  fn setDepthBufferSize(self, rsthis: &mut QSurfaceFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat18setDepthBufferSizeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QSurfaceFormat18setDepthBufferSizeEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn majorVersion<T: QSurfaceFormat_majorVersion>(&mut self, value: T) -> i32 {
    return value.majorVersion(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_majorVersion {
  fn majorVersion(self, rsthis: &mut QSurfaceFormat) -> i32;
}

// proto:  int QSurfaceFormat::majorVersion();
impl<'a> /*trait*/ QSurfaceFormat_majorVersion for () {
  fn majorVersion(self, rsthis: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSurfaceFormat12majorVersionEv()};
    let mut ret = unsafe {_ZNK14QSurfaceFormat12majorVersionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn setSamples<T: QSurfaceFormat_setSamples>(&mut self, value: T)  {
     value.setSamples(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_setSamples {
  fn setSamples(self, rsthis: &mut QSurfaceFormat) ;
}

// proto:  void QSurfaceFormat::setSamples(int numSamples);
impl<'a> /*trait*/ QSurfaceFormat_setSamples for (i32) {
  fn setSamples(self, rsthis: &mut QSurfaceFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat10setSamplesEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QSurfaceFormat10setSamplesEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn setMajorVersion<T: QSurfaceFormat_setMajorVersion>(&mut self, value: T)  {
     value.setMajorVersion(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_setMajorVersion {
  fn setMajorVersion(self, rsthis: &mut QSurfaceFormat) ;
}

// proto:  void QSurfaceFormat::setMajorVersion(int majorVersion);
impl<'a> /*trait*/ QSurfaceFormat_setMajorVersion for (i32) {
  fn setMajorVersion(self, rsthis: &mut QSurfaceFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat15setMajorVersionEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QSurfaceFormat15setMajorVersionEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn setDefaultFormat<T: QSurfaceFormat_setDefaultFormat>(&mut self, value: T)  {
     value.setDefaultFormat(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_setDefaultFormat {
  fn setDefaultFormat(self, rsthis: &mut QSurfaceFormat) ;
}

// proto: static void QSurfaceFormat::setDefaultFormat(const QSurfaceFormat & format);
impl<'a> /*trait*/ QSurfaceFormat_setDefaultFormat for (&'a  QSurfaceFormat) {
  fn setDefaultFormat(self, rsthis: &mut QSurfaceFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat16setDefaultFormatERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QSurfaceFormat16setDefaultFormatERKS_(arg0)};
    // return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn greenBufferSize<T: QSurfaceFormat_greenBufferSize>(&mut self, value: T) -> i32 {
    return value.greenBufferSize(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_greenBufferSize {
  fn greenBufferSize(self, rsthis: &mut QSurfaceFormat) -> i32;
}

// proto:  int QSurfaceFormat::greenBufferSize();
impl<'a> /*trait*/ QSurfaceFormat_greenBufferSize for () {
  fn greenBufferSize(self, rsthis: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSurfaceFormat15greenBufferSizeEv()};
    let mut ret = unsafe {_ZNK14QSurfaceFormat15greenBufferSizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn minorVersion<T: QSurfaceFormat_minorVersion>(&mut self, value: T) -> i32 {
    return value.minorVersion(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_minorVersion {
  fn minorVersion(self, rsthis: &mut QSurfaceFormat) -> i32;
}

// proto:  int QSurfaceFormat::minorVersion();
impl<'a> /*trait*/ QSurfaceFormat_minorVersion for () {
  fn minorVersion(self, rsthis: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSurfaceFormat12minorVersionEv()};
    let mut ret = unsafe {_ZNK14QSurfaceFormat12minorVersionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn setStencilBufferSize<T: QSurfaceFormat_setStencilBufferSize>(&mut self, value: T)  {
     value.setStencilBufferSize(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_setStencilBufferSize {
  fn setStencilBufferSize(self, rsthis: &mut QSurfaceFormat) ;
}

// proto:  void QSurfaceFormat::setStencilBufferSize(int size);
impl<'a> /*trait*/ QSurfaceFormat_setStencilBufferSize for (i32) {
  fn setStencilBufferSize(self, rsthis: &mut QSurfaceFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat20setStencilBufferSizeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QSurfaceFormat20setStencilBufferSizeEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn swapInterval<T: QSurfaceFormat_swapInterval>(&mut self, value: T) -> i32 {
    return value.swapInterval(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_swapInterval {
  fn swapInterval(self, rsthis: &mut QSurfaceFormat) -> i32;
}

// proto:  int QSurfaceFormat::swapInterval();
impl<'a> /*trait*/ QSurfaceFormat_swapInterval for () {
  fn swapInterval(self, rsthis: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSurfaceFormat12swapIntervalEv()};
    let mut ret = unsafe {_ZNK14QSurfaceFormat12swapIntervalEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn setVersion<T: QSurfaceFormat_setVersion>(&mut self, value: T)  {
     value.setVersion(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_setVersion {
  fn setVersion(self, rsthis: &mut QSurfaceFormat) ;
}

// proto:  void QSurfaceFormat::setVersion(int major, int minor);
impl<'a> /*trait*/ QSurfaceFormat_setVersion for (i32, i32) {
  fn setVersion(self, rsthis: &mut QSurfaceFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat10setVersionEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN14QSurfaceFormat10setVersionEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn hasAlpha<T: QSurfaceFormat_hasAlpha>(&mut self, value: T) -> i8 {
    return value.hasAlpha(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_hasAlpha {
  fn hasAlpha(self, rsthis: &mut QSurfaceFormat) -> i8;
}

// proto:  bool QSurfaceFormat::hasAlpha();
impl<'a> /*trait*/ QSurfaceFormat_hasAlpha for () {
  fn hasAlpha(self, rsthis: &mut QSurfaceFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSurfaceFormat8hasAlphaEv()};
    let mut ret = unsafe {_ZNK14QSurfaceFormat8hasAlphaEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

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

// proto: void QSurfaceFormat::NewQSurfaceFormat(const QSurfaceFormat & other);
impl<'a> /*trait*/ QSurfaceFormat_NewQSurfaceFormat for (&'a  QSurfaceFormat) {
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

impl /*struct*/ QSurfaceFormat {
  pub fn version<T: QSurfaceFormat_version>(&mut self, value: T)  {
     value.version(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_version {
  fn version(self, rsthis: &mut QSurfaceFormat) ;
}

// proto:  QPair<int, int> QSurfaceFormat::version();
impl<'a> /*trait*/ QSurfaceFormat_version for () {
  fn version(self, rsthis: &mut QSurfaceFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSurfaceFormat7versionEv()};
     unsafe {_ZNK14QSurfaceFormat7versionEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn blueBufferSize<T: QSurfaceFormat_blueBufferSize>(&mut self, value: T) -> i32 {
    return value.blueBufferSize(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_blueBufferSize {
  fn blueBufferSize(self, rsthis: &mut QSurfaceFormat) -> i32;
}

// proto:  int QSurfaceFormat::blueBufferSize();
impl<'a> /*trait*/ QSurfaceFormat_blueBufferSize for () {
  fn blueBufferSize(self, rsthis: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSurfaceFormat14blueBufferSizeEv()};
    let mut ret = unsafe {_ZNK14QSurfaceFormat14blueBufferSizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto: void QSurfaceFormat::NewQSurfaceFormat();
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

impl /*struct*/ QSurfaceFormat {
  pub fn redBufferSize<T: QSurfaceFormat_redBufferSize>(&mut self, value: T) -> i32 {
    return value.redBufferSize(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_redBufferSize {
  fn redBufferSize(self, rsthis: &mut QSurfaceFormat) -> i32;
}

// proto:  int QSurfaceFormat::redBufferSize();
impl<'a> /*trait*/ QSurfaceFormat_redBufferSize for () {
  fn redBufferSize(self, rsthis: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSurfaceFormat13redBufferSizeEv()};
    let mut ret = unsafe {_ZNK14QSurfaceFormat13redBufferSizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn FreeQSurfaceFormat<T: QSurfaceFormat_FreeQSurfaceFormat>(&mut self, value: T)  {
     value.FreeQSurfaceFormat(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_FreeQSurfaceFormat {
  fn FreeQSurfaceFormat(self, rsthis: &mut QSurfaceFormat) ;
}

// proto:  void QSurfaceFormat::FreeQSurfaceFormat();
impl<'a> /*trait*/ QSurfaceFormat_FreeQSurfaceFormat for () {
  fn FreeQSurfaceFormat(self, rsthis: &mut QSurfaceFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormatD0Ev()};
     unsafe {_ZN14QSurfaceFormatD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn setGreenBufferSize<T: QSurfaceFormat_setGreenBufferSize>(&mut self, value: T)  {
     value.setGreenBufferSize(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_setGreenBufferSize {
  fn setGreenBufferSize(self, rsthis: &mut QSurfaceFormat) ;
}

// proto:  void QSurfaceFormat::setGreenBufferSize(int size);
impl<'a> /*trait*/ QSurfaceFormat_setGreenBufferSize for (i32) {
  fn setGreenBufferSize(self, rsthis: &mut QSurfaceFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat18setGreenBufferSizeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QSurfaceFormat18setGreenBufferSizeEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn samples<T: QSurfaceFormat_samples>(&mut self, value: T) -> i32 {
    return value.samples(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_samples {
  fn samples(self, rsthis: &mut QSurfaceFormat) -> i32;
}

// proto:  int QSurfaceFormat::samples();
impl<'a> /*trait*/ QSurfaceFormat_samples for () {
  fn samples(self, rsthis: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSurfaceFormat7samplesEv()};
    let mut ret = unsafe {_ZNK14QSurfaceFormat7samplesEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn depthBufferSize<T: QSurfaceFormat_depthBufferSize>(&mut self, value: T) -> i32 {
    return value.depthBufferSize(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_depthBufferSize {
  fn depthBufferSize(self, rsthis: &mut QSurfaceFormat) -> i32;
}

// proto:  int QSurfaceFormat::depthBufferSize();
impl<'a> /*trait*/ QSurfaceFormat_depthBufferSize for () {
  fn depthBufferSize(self, rsthis: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSurfaceFormat15depthBufferSizeEv()};
    let mut ret = unsafe {_ZNK14QSurfaceFormat15depthBufferSizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn setBlueBufferSize<T: QSurfaceFormat_setBlueBufferSize>(&mut self, value: T)  {
     value.setBlueBufferSize(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_setBlueBufferSize {
  fn setBlueBufferSize(self, rsthis: &mut QSurfaceFormat) ;
}

// proto:  void QSurfaceFormat::setBlueBufferSize(int size);
impl<'a> /*trait*/ QSurfaceFormat_setBlueBufferSize for (i32) {
  fn setBlueBufferSize(self, rsthis: &mut QSurfaceFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat17setBlueBufferSizeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QSurfaceFormat17setBlueBufferSizeEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn alphaBufferSize<T: QSurfaceFormat_alphaBufferSize>(&mut self, value: T) -> i32 {
    return value.alphaBufferSize(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_alphaBufferSize {
  fn alphaBufferSize(self, rsthis: &mut QSurfaceFormat) -> i32;
}

// proto:  int QSurfaceFormat::alphaBufferSize();
impl<'a> /*trait*/ QSurfaceFormat_alphaBufferSize for () {
  fn alphaBufferSize(self, rsthis: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSurfaceFormat15alphaBufferSizeEv()};
    let mut ret = unsafe {_ZNK14QSurfaceFormat15alphaBufferSizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn stereo<T: QSurfaceFormat_stereo>(&mut self, value: T) -> i8 {
    return value.stereo(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_stereo {
  fn stereo(self, rsthis: &mut QSurfaceFormat) -> i8;
}

// proto:  bool QSurfaceFormat::stereo();
impl<'a> /*trait*/ QSurfaceFormat_stereo for () {
  fn stereo(self, rsthis: &mut QSurfaceFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSurfaceFormat6stereoEv()};
    let mut ret = unsafe {_ZNK14QSurfaceFormat6stereoEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn setSwapInterval<T: QSurfaceFormat_setSwapInterval>(&mut self, value: T)  {
     value.setSwapInterval(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_setSwapInterval {
  fn setSwapInterval(self, rsthis: &mut QSurfaceFormat) ;
}

// proto:  void QSurfaceFormat::setSwapInterval(int interval);
impl<'a> /*trait*/ QSurfaceFormat_setSwapInterval for (i32) {
  fn setSwapInterval(self, rsthis: &mut QSurfaceFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat15setSwapIntervalEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QSurfaceFormat15setSwapIntervalEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn setStereo<T: QSurfaceFormat_setStereo>(&mut self, value: T)  {
     value.setStereo(self);
    // return 1;
  }
}

pub trait QSurfaceFormat_setStereo {
  fn setStereo(self, rsthis: &mut QSurfaceFormat) ;
}

// proto:  void QSurfaceFormat::setStereo(bool enable);
impl<'a> /*trait*/ QSurfaceFormat_setStereo for (i8) {
  fn setStereo(self, rsthis: &mut QSurfaceFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat9setStereoEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN14QSurfaceFormat9setStereoEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

