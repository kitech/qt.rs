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
  // proto: QSurfaceFormat QSurfaceFormat::defaultFormat();
  fn _ZN14QSurfaceFormat13defaultFormatEv() -> i32;
  // proto: void QSurfaceFormat::setAlphaBufferSize(int size);
  fn _ZN14QSurfaceFormat18setAlphaBufferSizeEi(arg0: c_int) -> i32;
  // proto: void QSurfaceFormat::setMinorVersion(int minorVersion);
  fn _ZN14QSurfaceFormat15setMinorVersionEi(arg0: c_int) -> i32;
  // proto: int QSurfaceFormat::stencilBufferSize();
  fn _ZNK14QSurfaceFormat17stencilBufferSizeEv() -> i32;
  // proto: void QSurfaceFormat::setRedBufferSize(int size);
  fn _ZN14QSurfaceFormat16setRedBufferSizeEi(arg0: c_int) -> i32;
  // proto: void QSurfaceFormat::setDepthBufferSize(int size);
  fn _ZN14QSurfaceFormat18setDepthBufferSizeEi(arg0: c_int) -> i32;
  // proto: int QSurfaceFormat::majorVersion();
  fn _ZNK14QSurfaceFormat12majorVersionEv() -> i32;
  // proto: void QSurfaceFormat::setSamples(int numSamples);
  fn _ZN14QSurfaceFormat10setSamplesEi(arg0: c_int) -> i32;
  // proto: void QSurfaceFormat::setMajorVersion(int majorVersion);
  fn _ZN14QSurfaceFormat15setMajorVersionEi(arg0: c_int) -> i32;
  // proto: void QSurfaceFormat::setDefaultFormat(const QSurfaceFormat & format);
  fn _ZN14QSurfaceFormat16setDefaultFormatERKS_(arg0: *const c_void) -> i32;
  // proto: int QSurfaceFormat::greenBufferSize();
  fn _ZNK14QSurfaceFormat15greenBufferSizeEv() -> i32;
  // proto: int QSurfaceFormat::minorVersion();
  fn _ZNK14QSurfaceFormat12minorVersionEv() -> i32;
  // proto: void QSurfaceFormat::setStencilBufferSize(int size);
  fn _ZN14QSurfaceFormat20setStencilBufferSizeEi(arg0: c_int) -> i32;
  // proto: int QSurfaceFormat::swapInterval();
  fn _ZNK14QSurfaceFormat12swapIntervalEv() -> i32;
  // proto: void QSurfaceFormat::setVersion(int major, int minor);
  fn _ZN14QSurfaceFormat10setVersionEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: bool QSurfaceFormat::hasAlpha();
  fn _ZNK14QSurfaceFormat8hasAlphaEv() -> i32;
  // proto: void QSurfaceFormat::NewQSurfaceFormat(const QSurfaceFormat & other);
  fn _ZN14QSurfaceFormatC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: QPair<int, int> QSurfaceFormat::version();
  fn _ZNK14QSurfaceFormat7versionEv() -> i32;
  // proto: int QSurfaceFormat::blueBufferSize();
  fn _ZNK14QSurfaceFormat14blueBufferSizeEv() -> i32;
  // proto: void QSurfaceFormat::NewQSurfaceFormat();
  fn _ZN14QSurfaceFormatC1Ev(qthis: *mut c_void) -> i32;
  // proto: int QSurfaceFormat::redBufferSize();
  fn _ZNK14QSurfaceFormat13redBufferSizeEv() -> i32;
  // proto: void QSurfaceFormat::FreeQSurfaceFormat();
  fn _ZN14QSurfaceFormatD0Ev() -> i32;
  // proto: void QSurfaceFormat::setGreenBufferSize(int size);
  fn _ZN14QSurfaceFormat18setGreenBufferSizeEi(arg0: c_int) -> i32;
  // proto: int QSurfaceFormat::samples();
  fn _ZNK14QSurfaceFormat7samplesEv() -> i32;
  // proto: int QSurfaceFormat::depthBufferSize();
  fn _ZNK14QSurfaceFormat15depthBufferSizeEv() -> i32;
  // proto: void QSurfaceFormat::setBlueBufferSize(int size);
  fn _ZN14QSurfaceFormat17setBlueBufferSizeEi(arg0: c_int) -> i32;
  // proto: int QSurfaceFormat::alphaBufferSize();
  fn _ZNK14QSurfaceFormat15alphaBufferSizeEv() -> i32;
  // proto: bool QSurfaceFormat::stereo();
  fn _ZNK14QSurfaceFormat6stereoEv() -> i32;
  // proto: void QSurfaceFormat::setSwapInterval(int interval);
  fn _ZN14QSurfaceFormat15setSwapIntervalEi(arg0: c_int) -> i32;
  // proto: void QSurfaceFormat::setStereo(bool enable);
  fn _ZN14QSurfaceFormat9setStereoEb(arg0: int8_t) -> i32;
}

// body block begin
// class sizeof(QSurfaceFormat)=8
pub struct QSurfaceFormat {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSurfaceFormat {
  pub fn defaultFormat<T: QSurfaceFormat_defaultFormat>(&mut self, value: T) -> i32 {
    value.defaultFormat(self);
    return 1;
  }
}

pub trait QSurfaceFormat_defaultFormat {
  fn defaultFormat(self, this: &mut QSurfaceFormat) -> i32;
}

// proto: QSurfaceFormat QSurfaceFormat::defaultFormat();
impl<'a> /*trait*/ QSurfaceFormat_defaultFormat for () {
  fn defaultFormat(self, this: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat13defaultFormatEv()};
    unsafe {_ZN14QSurfaceFormat13defaultFormatEv()};
    return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn setAlphaBufferSize<T: QSurfaceFormat_setAlphaBufferSize>(&mut self, value: T) -> i32 {
    value.setAlphaBufferSize(self);
    return 1;
  }
}

pub trait QSurfaceFormat_setAlphaBufferSize {
  fn setAlphaBufferSize(self, this: &mut QSurfaceFormat) -> i32;
}

// proto: void QSurfaceFormat::setAlphaBufferSize(int size);
impl<'a> /*trait*/ QSurfaceFormat_setAlphaBufferSize for (i32) {
  fn setAlphaBufferSize(self, this: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat18setAlphaBufferSizeEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN14QSurfaceFormat18setAlphaBufferSizeEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn setMinorVersion<T: QSurfaceFormat_setMinorVersion>(&mut self, value: T) -> i32 {
    value.setMinorVersion(self);
    return 1;
  }
}

pub trait QSurfaceFormat_setMinorVersion {
  fn setMinorVersion(self, this: &mut QSurfaceFormat) -> i32;
}

// proto: void QSurfaceFormat::setMinorVersion(int minorVersion);
impl<'a> /*trait*/ QSurfaceFormat_setMinorVersion for (i32) {
  fn setMinorVersion(self, this: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat15setMinorVersionEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN14QSurfaceFormat15setMinorVersionEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn stencilBufferSize<T: QSurfaceFormat_stencilBufferSize>(&mut self, value: T) -> i32 {
    value.stencilBufferSize(self);
    return 1;
  }
}

pub trait QSurfaceFormat_stencilBufferSize {
  fn stencilBufferSize(self, this: &mut QSurfaceFormat) -> i32;
}

// proto: int QSurfaceFormat::stencilBufferSize();
impl<'a> /*trait*/ QSurfaceFormat_stencilBufferSize for () {
  fn stencilBufferSize(self, this: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSurfaceFormat17stencilBufferSizeEv()};
    unsafe {_ZNK14QSurfaceFormat17stencilBufferSizeEv()};
    return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn setRedBufferSize<T: QSurfaceFormat_setRedBufferSize>(&mut self, value: T) -> i32 {
    value.setRedBufferSize(self);
    return 1;
  }
}

pub trait QSurfaceFormat_setRedBufferSize {
  fn setRedBufferSize(self, this: &mut QSurfaceFormat) -> i32;
}

// proto: void QSurfaceFormat::setRedBufferSize(int size);
impl<'a> /*trait*/ QSurfaceFormat_setRedBufferSize for (i32) {
  fn setRedBufferSize(self, this: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat16setRedBufferSizeEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN14QSurfaceFormat16setRedBufferSizeEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn setDepthBufferSize<T: QSurfaceFormat_setDepthBufferSize>(&mut self, value: T) -> i32 {
    value.setDepthBufferSize(self);
    return 1;
  }
}

pub trait QSurfaceFormat_setDepthBufferSize {
  fn setDepthBufferSize(self, this: &mut QSurfaceFormat) -> i32;
}

// proto: void QSurfaceFormat::setDepthBufferSize(int size);
impl<'a> /*trait*/ QSurfaceFormat_setDepthBufferSize for (i32) {
  fn setDepthBufferSize(self, this: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat18setDepthBufferSizeEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN14QSurfaceFormat18setDepthBufferSizeEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn majorVersion<T: QSurfaceFormat_majorVersion>(&mut self, value: T) -> i32 {
    value.majorVersion(self);
    return 1;
  }
}

pub trait QSurfaceFormat_majorVersion {
  fn majorVersion(self, this: &mut QSurfaceFormat) -> i32;
}

// proto: int QSurfaceFormat::majorVersion();
impl<'a> /*trait*/ QSurfaceFormat_majorVersion for () {
  fn majorVersion(self, this: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSurfaceFormat12majorVersionEv()};
    unsafe {_ZNK14QSurfaceFormat12majorVersionEv()};
    return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn setSamples<T: QSurfaceFormat_setSamples>(&mut self, value: T) -> i32 {
    value.setSamples(self);
    return 1;
  }
}

pub trait QSurfaceFormat_setSamples {
  fn setSamples(self, this: &mut QSurfaceFormat) -> i32;
}

// proto: void QSurfaceFormat::setSamples(int numSamples);
impl<'a> /*trait*/ QSurfaceFormat_setSamples for (i32) {
  fn setSamples(self, this: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat10setSamplesEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN14QSurfaceFormat10setSamplesEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn setMajorVersion<T: QSurfaceFormat_setMajorVersion>(&mut self, value: T) -> i32 {
    value.setMajorVersion(self);
    return 1;
  }
}

pub trait QSurfaceFormat_setMajorVersion {
  fn setMajorVersion(self, this: &mut QSurfaceFormat) -> i32;
}

// proto: void QSurfaceFormat::setMajorVersion(int majorVersion);
impl<'a> /*trait*/ QSurfaceFormat_setMajorVersion for (i32) {
  fn setMajorVersion(self, this: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat15setMajorVersionEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN14QSurfaceFormat15setMajorVersionEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn setDefaultFormat<T: QSurfaceFormat_setDefaultFormat>(&mut self, value: T) -> i32 {
    value.setDefaultFormat(self);
    return 1;
  }
}

pub trait QSurfaceFormat_setDefaultFormat {
  fn setDefaultFormat(self, this: &mut QSurfaceFormat) -> i32;
}

// proto: void QSurfaceFormat::setDefaultFormat(const QSurfaceFormat & format);
impl<'a> /*trait*/ QSurfaceFormat_setDefaultFormat for (&'a  QSurfaceFormat) {
  fn setDefaultFormat(self, this: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat16setDefaultFormatERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QSurfaceFormat16setDefaultFormatERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn greenBufferSize<T: QSurfaceFormat_greenBufferSize>(&mut self, value: T) -> i32 {
    value.greenBufferSize(self);
    return 1;
  }
}

pub trait QSurfaceFormat_greenBufferSize {
  fn greenBufferSize(self, this: &mut QSurfaceFormat) -> i32;
}

// proto: int QSurfaceFormat::greenBufferSize();
impl<'a> /*trait*/ QSurfaceFormat_greenBufferSize for () {
  fn greenBufferSize(self, this: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSurfaceFormat15greenBufferSizeEv()};
    unsafe {_ZNK14QSurfaceFormat15greenBufferSizeEv()};
    return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn minorVersion<T: QSurfaceFormat_minorVersion>(&mut self, value: T) -> i32 {
    value.minorVersion(self);
    return 1;
  }
}

pub trait QSurfaceFormat_minorVersion {
  fn minorVersion(self, this: &mut QSurfaceFormat) -> i32;
}

// proto: int QSurfaceFormat::minorVersion();
impl<'a> /*trait*/ QSurfaceFormat_minorVersion for () {
  fn minorVersion(self, this: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSurfaceFormat12minorVersionEv()};
    unsafe {_ZNK14QSurfaceFormat12minorVersionEv()};
    return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn setStencilBufferSize<T: QSurfaceFormat_setStencilBufferSize>(&mut self, value: T) -> i32 {
    value.setStencilBufferSize(self);
    return 1;
  }
}

pub trait QSurfaceFormat_setStencilBufferSize {
  fn setStencilBufferSize(self, this: &mut QSurfaceFormat) -> i32;
}

// proto: void QSurfaceFormat::setStencilBufferSize(int size);
impl<'a> /*trait*/ QSurfaceFormat_setStencilBufferSize for (i32) {
  fn setStencilBufferSize(self, this: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat20setStencilBufferSizeEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN14QSurfaceFormat20setStencilBufferSizeEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn swapInterval<T: QSurfaceFormat_swapInterval>(&mut self, value: T) -> i32 {
    value.swapInterval(self);
    return 1;
  }
}

pub trait QSurfaceFormat_swapInterval {
  fn swapInterval(self, this: &mut QSurfaceFormat) -> i32;
}

// proto: int QSurfaceFormat::swapInterval();
impl<'a> /*trait*/ QSurfaceFormat_swapInterval for () {
  fn swapInterval(self, this: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSurfaceFormat12swapIntervalEv()};
    unsafe {_ZNK14QSurfaceFormat12swapIntervalEv()};
    return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn setVersion<T: QSurfaceFormat_setVersion>(&mut self, value: T) -> i32 {
    value.setVersion(self);
    return 1;
  }
}

pub trait QSurfaceFormat_setVersion {
  fn setVersion(self, this: &mut QSurfaceFormat) -> i32;
}

// proto: void QSurfaceFormat::setVersion(int major, int minor);
impl<'a> /*trait*/ QSurfaceFormat_setVersion for (i32, i32) {
  fn setVersion(self, this: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat10setVersionEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN14QSurfaceFormat10setVersionEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn hasAlpha<T: QSurfaceFormat_hasAlpha>(&mut self, value: T) -> i32 {
    value.hasAlpha(self);
    return 1;
  }
}

pub trait QSurfaceFormat_hasAlpha {
  fn hasAlpha(self, this: &mut QSurfaceFormat) -> i32;
}

// proto: bool QSurfaceFormat::hasAlpha();
impl<'a> /*trait*/ QSurfaceFormat_hasAlpha for () {
  fn hasAlpha(self, this: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSurfaceFormat8hasAlphaEv()};
    unsafe {_ZNK14QSurfaceFormat8hasAlphaEv()};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QSurfaceFormatC1ERKS_(qthis, arg0)};
    let rsthis = QSurfaceFormat{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn version<T: QSurfaceFormat_version>(&mut self, value: T) -> i32 {
    value.version(self);
    return 1;
  }
}

pub trait QSurfaceFormat_version {
  fn version(self, this: &mut QSurfaceFormat) -> i32;
}

// proto: QPair<int, int> QSurfaceFormat::version();
impl<'a> /*trait*/ QSurfaceFormat_version for () {
  fn version(self, this: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSurfaceFormat7versionEv()};
    unsafe {_ZNK14QSurfaceFormat7versionEv()};
    return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn blueBufferSize<T: QSurfaceFormat_blueBufferSize>(&mut self, value: T) -> i32 {
    value.blueBufferSize(self);
    return 1;
  }
}

pub trait QSurfaceFormat_blueBufferSize {
  fn blueBufferSize(self, this: &mut QSurfaceFormat) -> i32;
}

// proto: int QSurfaceFormat::blueBufferSize();
impl<'a> /*trait*/ QSurfaceFormat_blueBufferSize for () {
  fn blueBufferSize(self, this: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSurfaceFormat14blueBufferSizeEv()};
    unsafe {_ZNK14QSurfaceFormat14blueBufferSizeEv()};
    return 1;
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
    value.redBufferSize(self);
    return 1;
  }
}

pub trait QSurfaceFormat_redBufferSize {
  fn redBufferSize(self, this: &mut QSurfaceFormat) -> i32;
}

// proto: int QSurfaceFormat::redBufferSize();
impl<'a> /*trait*/ QSurfaceFormat_redBufferSize for () {
  fn redBufferSize(self, this: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSurfaceFormat13redBufferSizeEv()};
    unsafe {_ZNK14QSurfaceFormat13redBufferSizeEv()};
    return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn FreeQSurfaceFormat<T: QSurfaceFormat_FreeQSurfaceFormat>(&mut self, value: T) -> i32 {
    value.FreeQSurfaceFormat(self);
    return 1;
  }
}

pub trait QSurfaceFormat_FreeQSurfaceFormat {
  fn FreeQSurfaceFormat(self, this: &mut QSurfaceFormat) -> i32;
}

// proto: void QSurfaceFormat::FreeQSurfaceFormat();
impl<'a> /*trait*/ QSurfaceFormat_FreeQSurfaceFormat for () {
  fn FreeQSurfaceFormat(self, this: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormatD0Ev()};
    unsafe {_ZN14QSurfaceFormatD0Ev()};
    return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn setGreenBufferSize<T: QSurfaceFormat_setGreenBufferSize>(&mut self, value: T) -> i32 {
    value.setGreenBufferSize(self);
    return 1;
  }
}

pub trait QSurfaceFormat_setGreenBufferSize {
  fn setGreenBufferSize(self, this: &mut QSurfaceFormat) -> i32;
}

// proto: void QSurfaceFormat::setGreenBufferSize(int size);
impl<'a> /*trait*/ QSurfaceFormat_setGreenBufferSize for (i32) {
  fn setGreenBufferSize(self, this: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat18setGreenBufferSizeEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN14QSurfaceFormat18setGreenBufferSizeEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn samples<T: QSurfaceFormat_samples>(&mut self, value: T) -> i32 {
    value.samples(self);
    return 1;
  }
}

pub trait QSurfaceFormat_samples {
  fn samples(self, this: &mut QSurfaceFormat) -> i32;
}

// proto: int QSurfaceFormat::samples();
impl<'a> /*trait*/ QSurfaceFormat_samples for () {
  fn samples(self, this: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSurfaceFormat7samplesEv()};
    unsafe {_ZNK14QSurfaceFormat7samplesEv()};
    return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn depthBufferSize<T: QSurfaceFormat_depthBufferSize>(&mut self, value: T) -> i32 {
    value.depthBufferSize(self);
    return 1;
  }
}

pub trait QSurfaceFormat_depthBufferSize {
  fn depthBufferSize(self, this: &mut QSurfaceFormat) -> i32;
}

// proto: int QSurfaceFormat::depthBufferSize();
impl<'a> /*trait*/ QSurfaceFormat_depthBufferSize for () {
  fn depthBufferSize(self, this: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSurfaceFormat15depthBufferSizeEv()};
    unsafe {_ZNK14QSurfaceFormat15depthBufferSizeEv()};
    return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn setBlueBufferSize<T: QSurfaceFormat_setBlueBufferSize>(&mut self, value: T) -> i32 {
    value.setBlueBufferSize(self);
    return 1;
  }
}

pub trait QSurfaceFormat_setBlueBufferSize {
  fn setBlueBufferSize(self, this: &mut QSurfaceFormat) -> i32;
}

// proto: void QSurfaceFormat::setBlueBufferSize(int size);
impl<'a> /*trait*/ QSurfaceFormat_setBlueBufferSize for (i32) {
  fn setBlueBufferSize(self, this: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat17setBlueBufferSizeEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN14QSurfaceFormat17setBlueBufferSizeEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn alphaBufferSize<T: QSurfaceFormat_alphaBufferSize>(&mut self, value: T) -> i32 {
    value.alphaBufferSize(self);
    return 1;
  }
}

pub trait QSurfaceFormat_alphaBufferSize {
  fn alphaBufferSize(self, this: &mut QSurfaceFormat) -> i32;
}

// proto: int QSurfaceFormat::alphaBufferSize();
impl<'a> /*trait*/ QSurfaceFormat_alphaBufferSize for () {
  fn alphaBufferSize(self, this: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSurfaceFormat15alphaBufferSizeEv()};
    unsafe {_ZNK14QSurfaceFormat15alphaBufferSizeEv()};
    return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn stereo<T: QSurfaceFormat_stereo>(&mut self, value: T) -> i32 {
    value.stereo(self);
    return 1;
  }
}

pub trait QSurfaceFormat_stereo {
  fn stereo(self, this: &mut QSurfaceFormat) -> i32;
}

// proto: bool QSurfaceFormat::stereo();
impl<'a> /*trait*/ QSurfaceFormat_stereo for () {
  fn stereo(self, this: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSurfaceFormat6stereoEv()};
    unsafe {_ZNK14QSurfaceFormat6stereoEv()};
    return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn setSwapInterval<T: QSurfaceFormat_setSwapInterval>(&mut self, value: T) -> i32 {
    value.setSwapInterval(self);
    return 1;
  }
}

pub trait QSurfaceFormat_setSwapInterval {
  fn setSwapInterval(self, this: &mut QSurfaceFormat) -> i32;
}

// proto: void QSurfaceFormat::setSwapInterval(int interval);
impl<'a> /*trait*/ QSurfaceFormat_setSwapInterval for (i32) {
  fn setSwapInterval(self, this: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat15setSwapIntervalEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN14QSurfaceFormat15setSwapIntervalEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QSurfaceFormat {
  pub fn setStereo<T: QSurfaceFormat_setStereo>(&mut self, value: T) -> i32 {
    value.setStereo(self);
    return 1;
  }
}

pub trait QSurfaceFormat_setStereo {
  fn setStereo(self, this: &mut QSurfaceFormat) -> i32;
}

// proto: void QSurfaceFormat::setStereo(bool enable);
impl<'a> /*trait*/ QSurfaceFormat_setStereo for (i8) {
  fn setStereo(self, this: &mut QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSurfaceFormat9setStereoEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN14QSurfaceFormat9setStereoEb(arg0)};
    return 1;
  }
}

