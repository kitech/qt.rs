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
  // proto: void QOpenGLPixelTransferOptions::NewQOpenGLPixelTransferOptions();
  fn _ZN27QOpenGLPixelTransferOptionsC1Ev(qthis: *mut c_void) -> i32;
  // proto: void QOpenGLPixelTransferOptions::FreeQOpenGLPixelTransferOptions();
  fn _ZN27QOpenGLPixelTransferOptionsD0Ev() -> i32;
  // proto: bool QOpenGLPixelTransferOptions::isSwapBytesEnabled();
  fn _ZNK27QOpenGLPixelTransferOptions18isSwapBytesEnabledEv() -> i32;
  // proto: void QOpenGLPixelTransferOptions::swap(QOpenGLPixelTransferOptions & other);
  fn _ZN27QOpenGLPixelTransferOptions4swapERS_(arg0: *mut c_void) -> i32;
  // proto: void QOpenGLPixelTransferOptions::NewQOpenGLPixelTransferOptions(const QOpenGLPixelTransferOptions & );
  fn _ZN27QOpenGLPixelTransferOptionsC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: int QOpenGLPixelTransferOptions::skipImages();
  fn _ZNK27QOpenGLPixelTransferOptions10skipImagesEv() -> i32;
  // proto: void QOpenGLPixelTransferOptions::setSkipRows(int skipRows);
  fn _ZN27QOpenGLPixelTransferOptions11setSkipRowsEi(arg0: c_int) -> i32;
  // proto: int QOpenGLPixelTransferOptions::skipPixels();
  fn _ZNK27QOpenGLPixelTransferOptions10skipPixelsEv() -> i32;
  // proto: void QOpenGLPixelTransferOptions::setRowLength(int rowLength);
  fn _ZN27QOpenGLPixelTransferOptions12setRowLengthEi(arg0: c_int) -> i32;
  // proto: int QOpenGLPixelTransferOptions::imageHeight();
  fn _ZNK27QOpenGLPixelTransferOptions11imageHeightEv() -> i32;
  // proto: void QOpenGLPixelTransferOptions::setImageHeight(int imageHeight);
  fn _ZN27QOpenGLPixelTransferOptions14setImageHeightEi(arg0: c_int) -> i32;
  // proto: int QOpenGLPixelTransferOptions::skipRows();
  fn _ZNK27QOpenGLPixelTransferOptions8skipRowsEv() -> i32;
  // proto: void QOpenGLPixelTransferOptions::setAlignment(int alignment);
  fn _ZN27QOpenGLPixelTransferOptions12setAlignmentEi(arg0: c_int) -> i32;
  // proto: void QOpenGLPixelTransferOptions::setSkipImages(int skipImages);
  fn _ZN27QOpenGLPixelTransferOptions13setSkipImagesEi(arg0: c_int) -> i32;
  // proto: int QOpenGLPixelTransferOptions::alignment();
  fn _ZNK27QOpenGLPixelTransferOptions9alignmentEv() -> i32;
  // proto: void QOpenGLPixelTransferOptions::setSkipPixels(int skipPixels);
  fn _ZN27QOpenGLPixelTransferOptions13setSkipPixelsEi(arg0: c_int) -> i32;
  // proto: void QOpenGLPixelTransferOptions::setSwapBytesEnabled(bool swapBytes);
  fn _ZN27QOpenGLPixelTransferOptions19setSwapBytesEnabledEb(arg0: int8_t) -> i32;
  // proto: void QOpenGLPixelTransferOptions::setLeastSignificantByteFirst(bool lsbFirst);
  fn _ZN27QOpenGLPixelTransferOptions28setLeastSignificantByteFirstEb(arg0: int8_t) -> i32;
  // proto: bool QOpenGLPixelTransferOptions::isLeastSignificantBitFirst();
  fn _ZNK27QOpenGLPixelTransferOptions26isLeastSignificantBitFirstEv() -> i32;
  // proto: int QOpenGLPixelTransferOptions::rowLength();
  fn _ZNK27QOpenGLPixelTransferOptions9rowLengthEv() -> i32;
}

// body block begin
// class sizeof(QOpenGLPixelTransferOptions)=1
pub struct QOpenGLPixelTransferOptions {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOpenGLPixelTransferOptions {
  pub fn NewQOpenGLPixelTransferOptions<T: QOpenGLPixelTransferOptions_NewQOpenGLPixelTransferOptions>(value: T) -> QOpenGLPixelTransferOptions {
    let rsthis = value.NewQOpenGLPixelTransferOptions();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLPixelTransferOptions_NewQOpenGLPixelTransferOptions {
  fn NewQOpenGLPixelTransferOptions(self) -> QOpenGLPixelTransferOptions;
}

// proto: void QOpenGLPixelTransferOptions::NewQOpenGLPixelTransferOptions();
impl<'a> /*trait*/ QOpenGLPixelTransferOptions_NewQOpenGLPixelTransferOptions for () {
  fn NewQOpenGLPixelTransferOptions(self) -> QOpenGLPixelTransferOptions {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QOpenGLPixelTransferOptionsC1Ev()};
    unsafe {_ZN27QOpenGLPixelTransferOptionsC1Ev(qthis)};
    let rsthis = QOpenGLPixelTransferOptions{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLPixelTransferOptions {
  pub fn FreeQOpenGLPixelTransferOptions<T: QOpenGLPixelTransferOptions_FreeQOpenGLPixelTransferOptions>(&mut self, value: T) -> i32 {
    value.FreeQOpenGLPixelTransferOptions(self);
    return 1;
  }
}

pub trait QOpenGLPixelTransferOptions_FreeQOpenGLPixelTransferOptions {
  fn FreeQOpenGLPixelTransferOptions(self, this: &mut QOpenGLPixelTransferOptions) -> i32;
}

// proto: void QOpenGLPixelTransferOptions::FreeQOpenGLPixelTransferOptions();
impl<'a> /*trait*/ QOpenGLPixelTransferOptions_FreeQOpenGLPixelTransferOptions for () {
  fn FreeQOpenGLPixelTransferOptions(self, this: &mut QOpenGLPixelTransferOptions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QOpenGLPixelTransferOptionsD0Ev()};
    unsafe {_ZN27QOpenGLPixelTransferOptionsD0Ev()};
    return 1;
  }
}

impl /*struct*/ QOpenGLPixelTransferOptions {
  pub fn isSwapBytesEnabled<T: QOpenGLPixelTransferOptions_isSwapBytesEnabled>(&mut self, value: T) -> i32 {
    value.isSwapBytesEnabled(self);
    return 1;
  }
}

pub trait QOpenGLPixelTransferOptions_isSwapBytesEnabled {
  fn isSwapBytesEnabled(self, this: &mut QOpenGLPixelTransferOptions) -> i32;
}

// proto: bool QOpenGLPixelTransferOptions::isSwapBytesEnabled();
impl<'a> /*trait*/ QOpenGLPixelTransferOptions_isSwapBytesEnabled for () {
  fn isSwapBytesEnabled(self, this: &mut QOpenGLPixelTransferOptions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QOpenGLPixelTransferOptions18isSwapBytesEnabledEv()};
    unsafe {_ZNK27QOpenGLPixelTransferOptions18isSwapBytesEnabledEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLPixelTransferOptions {
  pub fn swap<T: QOpenGLPixelTransferOptions_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QOpenGLPixelTransferOptions_swap {
  fn swap(self, this: &mut QOpenGLPixelTransferOptions) -> i32;
}

// proto: void QOpenGLPixelTransferOptions::swap(QOpenGLPixelTransferOptions & other);
impl<'a> /*trait*/ QOpenGLPixelTransferOptions_swap for (&'a mut QOpenGLPixelTransferOptions) {
  fn swap(self, this: &mut QOpenGLPixelTransferOptions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QOpenGLPixelTransferOptions4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN27QOpenGLPixelTransferOptions4swapERS_(arg0)};
    return 1;
  }
}

// proto: void QOpenGLPixelTransferOptions::NewQOpenGLPixelTransferOptions(const QOpenGLPixelTransferOptions & );
impl<'a> /*trait*/ QOpenGLPixelTransferOptions_NewQOpenGLPixelTransferOptions for (&'a  QOpenGLPixelTransferOptions) {
  fn NewQOpenGLPixelTransferOptions(self) -> QOpenGLPixelTransferOptions {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QOpenGLPixelTransferOptionsC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN27QOpenGLPixelTransferOptionsC1ERKS_(qthis, arg0)};
    let rsthis = QOpenGLPixelTransferOptions{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLPixelTransferOptions {
  pub fn skipImages<T: QOpenGLPixelTransferOptions_skipImages>(&mut self, value: T) -> i32 {
    value.skipImages(self);
    return 1;
  }
}

pub trait QOpenGLPixelTransferOptions_skipImages {
  fn skipImages(self, this: &mut QOpenGLPixelTransferOptions) -> i32;
}

// proto: int QOpenGLPixelTransferOptions::skipImages();
impl<'a> /*trait*/ QOpenGLPixelTransferOptions_skipImages for () {
  fn skipImages(self, this: &mut QOpenGLPixelTransferOptions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QOpenGLPixelTransferOptions10skipImagesEv()};
    unsafe {_ZNK27QOpenGLPixelTransferOptions10skipImagesEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLPixelTransferOptions {
  pub fn setSkipRows<T: QOpenGLPixelTransferOptions_setSkipRows>(&mut self, value: T) -> i32 {
    value.setSkipRows(self);
    return 1;
  }
}

pub trait QOpenGLPixelTransferOptions_setSkipRows {
  fn setSkipRows(self, this: &mut QOpenGLPixelTransferOptions) -> i32;
}

// proto: void QOpenGLPixelTransferOptions::setSkipRows(int skipRows);
impl<'a> /*trait*/ QOpenGLPixelTransferOptions_setSkipRows for (i32) {
  fn setSkipRows(self, this: &mut QOpenGLPixelTransferOptions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QOpenGLPixelTransferOptions11setSkipRowsEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN27QOpenGLPixelTransferOptions11setSkipRowsEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLPixelTransferOptions {
  pub fn skipPixels<T: QOpenGLPixelTransferOptions_skipPixels>(&mut self, value: T) -> i32 {
    value.skipPixels(self);
    return 1;
  }
}

pub trait QOpenGLPixelTransferOptions_skipPixels {
  fn skipPixels(self, this: &mut QOpenGLPixelTransferOptions) -> i32;
}

// proto: int QOpenGLPixelTransferOptions::skipPixels();
impl<'a> /*trait*/ QOpenGLPixelTransferOptions_skipPixels for () {
  fn skipPixels(self, this: &mut QOpenGLPixelTransferOptions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QOpenGLPixelTransferOptions10skipPixelsEv()};
    unsafe {_ZNK27QOpenGLPixelTransferOptions10skipPixelsEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLPixelTransferOptions {
  pub fn setRowLength<T: QOpenGLPixelTransferOptions_setRowLength>(&mut self, value: T) -> i32 {
    value.setRowLength(self);
    return 1;
  }
}

pub trait QOpenGLPixelTransferOptions_setRowLength {
  fn setRowLength(self, this: &mut QOpenGLPixelTransferOptions) -> i32;
}

// proto: void QOpenGLPixelTransferOptions::setRowLength(int rowLength);
impl<'a> /*trait*/ QOpenGLPixelTransferOptions_setRowLength for (i32) {
  fn setRowLength(self, this: &mut QOpenGLPixelTransferOptions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QOpenGLPixelTransferOptions12setRowLengthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN27QOpenGLPixelTransferOptions12setRowLengthEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLPixelTransferOptions {
  pub fn imageHeight<T: QOpenGLPixelTransferOptions_imageHeight>(&mut self, value: T) -> i32 {
    value.imageHeight(self);
    return 1;
  }
}

pub trait QOpenGLPixelTransferOptions_imageHeight {
  fn imageHeight(self, this: &mut QOpenGLPixelTransferOptions) -> i32;
}

// proto: int QOpenGLPixelTransferOptions::imageHeight();
impl<'a> /*trait*/ QOpenGLPixelTransferOptions_imageHeight for () {
  fn imageHeight(self, this: &mut QOpenGLPixelTransferOptions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QOpenGLPixelTransferOptions11imageHeightEv()};
    unsafe {_ZNK27QOpenGLPixelTransferOptions11imageHeightEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLPixelTransferOptions {
  pub fn setImageHeight<T: QOpenGLPixelTransferOptions_setImageHeight>(&mut self, value: T) -> i32 {
    value.setImageHeight(self);
    return 1;
  }
}

pub trait QOpenGLPixelTransferOptions_setImageHeight {
  fn setImageHeight(self, this: &mut QOpenGLPixelTransferOptions) -> i32;
}

// proto: void QOpenGLPixelTransferOptions::setImageHeight(int imageHeight);
impl<'a> /*trait*/ QOpenGLPixelTransferOptions_setImageHeight for (i32) {
  fn setImageHeight(self, this: &mut QOpenGLPixelTransferOptions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QOpenGLPixelTransferOptions14setImageHeightEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN27QOpenGLPixelTransferOptions14setImageHeightEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLPixelTransferOptions {
  pub fn skipRows<T: QOpenGLPixelTransferOptions_skipRows>(&mut self, value: T) -> i32 {
    value.skipRows(self);
    return 1;
  }
}

pub trait QOpenGLPixelTransferOptions_skipRows {
  fn skipRows(self, this: &mut QOpenGLPixelTransferOptions) -> i32;
}

// proto: int QOpenGLPixelTransferOptions::skipRows();
impl<'a> /*trait*/ QOpenGLPixelTransferOptions_skipRows for () {
  fn skipRows(self, this: &mut QOpenGLPixelTransferOptions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QOpenGLPixelTransferOptions8skipRowsEv()};
    unsafe {_ZNK27QOpenGLPixelTransferOptions8skipRowsEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLPixelTransferOptions {
  pub fn setAlignment<T: QOpenGLPixelTransferOptions_setAlignment>(&mut self, value: T) -> i32 {
    value.setAlignment(self);
    return 1;
  }
}

pub trait QOpenGLPixelTransferOptions_setAlignment {
  fn setAlignment(self, this: &mut QOpenGLPixelTransferOptions) -> i32;
}

// proto: void QOpenGLPixelTransferOptions::setAlignment(int alignment);
impl<'a> /*trait*/ QOpenGLPixelTransferOptions_setAlignment for (i32) {
  fn setAlignment(self, this: &mut QOpenGLPixelTransferOptions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QOpenGLPixelTransferOptions12setAlignmentEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN27QOpenGLPixelTransferOptions12setAlignmentEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLPixelTransferOptions {
  pub fn setSkipImages<T: QOpenGLPixelTransferOptions_setSkipImages>(&mut self, value: T) -> i32 {
    value.setSkipImages(self);
    return 1;
  }
}

pub trait QOpenGLPixelTransferOptions_setSkipImages {
  fn setSkipImages(self, this: &mut QOpenGLPixelTransferOptions) -> i32;
}

// proto: void QOpenGLPixelTransferOptions::setSkipImages(int skipImages);
impl<'a> /*trait*/ QOpenGLPixelTransferOptions_setSkipImages for (i32) {
  fn setSkipImages(self, this: &mut QOpenGLPixelTransferOptions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QOpenGLPixelTransferOptions13setSkipImagesEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN27QOpenGLPixelTransferOptions13setSkipImagesEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLPixelTransferOptions {
  pub fn alignment<T: QOpenGLPixelTransferOptions_alignment>(&mut self, value: T) -> i32 {
    value.alignment(self);
    return 1;
  }
}

pub trait QOpenGLPixelTransferOptions_alignment {
  fn alignment(self, this: &mut QOpenGLPixelTransferOptions) -> i32;
}

// proto: int QOpenGLPixelTransferOptions::alignment();
impl<'a> /*trait*/ QOpenGLPixelTransferOptions_alignment for () {
  fn alignment(self, this: &mut QOpenGLPixelTransferOptions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QOpenGLPixelTransferOptions9alignmentEv()};
    unsafe {_ZNK27QOpenGLPixelTransferOptions9alignmentEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLPixelTransferOptions {
  pub fn setSkipPixels<T: QOpenGLPixelTransferOptions_setSkipPixels>(&mut self, value: T) -> i32 {
    value.setSkipPixels(self);
    return 1;
  }
}

pub trait QOpenGLPixelTransferOptions_setSkipPixels {
  fn setSkipPixels(self, this: &mut QOpenGLPixelTransferOptions) -> i32;
}

// proto: void QOpenGLPixelTransferOptions::setSkipPixels(int skipPixels);
impl<'a> /*trait*/ QOpenGLPixelTransferOptions_setSkipPixels for (i32) {
  fn setSkipPixels(self, this: &mut QOpenGLPixelTransferOptions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QOpenGLPixelTransferOptions13setSkipPixelsEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN27QOpenGLPixelTransferOptions13setSkipPixelsEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLPixelTransferOptions {
  pub fn setSwapBytesEnabled<T: QOpenGLPixelTransferOptions_setSwapBytesEnabled>(&mut self, value: T) -> i32 {
    value.setSwapBytesEnabled(self);
    return 1;
  }
}

pub trait QOpenGLPixelTransferOptions_setSwapBytesEnabled {
  fn setSwapBytesEnabled(self, this: &mut QOpenGLPixelTransferOptions) -> i32;
}

// proto: void QOpenGLPixelTransferOptions::setSwapBytesEnabled(bool swapBytes);
impl<'a> /*trait*/ QOpenGLPixelTransferOptions_setSwapBytesEnabled for (i8) {
  fn setSwapBytesEnabled(self, this: &mut QOpenGLPixelTransferOptions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QOpenGLPixelTransferOptions19setSwapBytesEnabledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN27QOpenGLPixelTransferOptions19setSwapBytesEnabledEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLPixelTransferOptions {
  pub fn setLeastSignificantByteFirst<T: QOpenGLPixelTransferOptions_setLeastSignificantByteFirst>(&mut self, value: T) -> i32 {
    value.setLeastSignificantByteFirst(self);
    return 1;
  }
}

pub trait QOpenGLPixelTransferOptions_setLeastSignificantByteFirst {
  fn setLeastSignificantByteFirst(self, this: &mut QOpenGLPixelTransferOptions) -> i32;
}

// proto: void QOpenGLPixelTransferOptions::setLeastSignificantByteFirst(bool lsbFirst);
impl<'a> /*trait*/ QOpenGLPixelTransferOptions_setLeastSignificantByteFirst for (i8) {
  fn setLeastSignificantByteFirst(self, this: &mut QOpenGLPixelTransferOptions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QOpenGLPixelTransferOptions28setLeastSignificantByteFirstEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN27QOpenGLPixelTransferOptions28setLeastSignificantByteFirstEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLPixelTransferOptions {
  pub fn isLeastSignificantBitFirst<T: QOpenGLPixelTransferOptions_isLeastSignificantBitFirst>(&mut self, value: T) -> i32 {
    value.isLeastSignificantBitFirst(self);
    return 1;
  }
}

pub trait QOpenGLPixelTransferOptions_isLeastSignificantBitFirst {
  fn isLeastSignificantBitFirst(self, this: &mut QOpenGLPixelTransferOptions) -> i32;
}

// proto: bool QOpenGLPixelTransferOptions::isLeastSignificantBitFirst();
impl<'a> /*trait*/ QOpenGLPixelTransferOptions_isLeastSignificantBitFirst for () {
  fn isLeastSignificantBitFirst(self, this: &mut QOpenGLPixelTransferOptions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QOpenGLPixelTransferOptions26isLeastSignificantBitFirstEv()};
    unsafe {_ZNK27QOpenGLPixelTransferOptions26isLeastSignificantBitFirstEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLPixelTransferOptions {
  pub fn rowLength<T: QOpenGLPixelTransferOptions_rowLength>(&mut self, value: T) -> i32 {
    value.rowLength(self);
    return 1;
  }
}

pub trait QOpenGLPixelTransferOptions_rowLength {
  fn rowLength(self, this: &mut QOpenGLPixelTransferOptions) -> i32;
}

// proto: int QOpenGLPixelTransferOptions::rowLength();
impl<'a> /*trait*/ QOpenGLPixelTransferOptions_rowLength for () {
  fn rowLength(self, this: &mut QOpenGLPixelTransferOptions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QOpenGLPixelTransferOptions9rowLengthEv()};
    unsafe {_ZNK27QOpenGLPixelTransferOptions9rowLengthEv()};
    return 1;
  }
}

