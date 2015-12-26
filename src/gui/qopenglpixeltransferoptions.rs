// auto generated, do not modify.
// created: Sat Dec 26 10:52:38 2015
// src-file: /QtGui/qopenglpixeltransferoptions.h
// dst-file: /src/gui/qopenglpixeltransferoptions.rs
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
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QOpenGLPixelTransferOptions_Class_Size() -> c_int;
  // proto:  void QOpenGLPixelTransferOptions::QOpenGLPixelTransferOptions();
  fn dector_ZN27QOpenGLPixelTransferOptionsC1Ev() -> *mut c_void;
  fn _ZN27QOpenGLPixelTransferOptionsC1Ev(qthis: *mut c_void);
  // proto:  void QOpenGLPixelTransferOptions::~QOpenGLPixelTransferOptions();
  fn _ZN27QOpenGLPixelTransferOptionsD0Ev(qthis: *mut c_void);
  // proto:  bool QOpenGLPixelTransferOptions::isSwapBytesEnabled();
  fn _ZNK27QOpenGLPixelTransferOptions18isSwapBytesEnabledEv(qthis: *mut c_void) -> c_char;
  // proto:  void QOpenGLPixelTransferOptions::swap(QOpenGLPixelTransferOptions & other);
  fn _ZN27QOpenGLPixelTransferOptions4swapERS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QOpenGLPixelTransferOptions::QOpenGLPixelTransferOptions(const QOpenGLPixelTransferOptions & );
  fn dector_ZN27QOpenGLPixelTransferOptionsC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN27QOpenGLPixelTransferOptionsC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QOpenGLPixelTransferOptions::skipImages();
  fn _ZNK27QOpenGLPixelTransferOptions10skipImagesEv(qthis: *mut c_void) -> c_int;
  // proto:  void QOpenGLPixelTransferOptions::setSkipRows(int skipRows);
  fn _ZN27QOpenGLPixelTransferOptions11setSkipRowsEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QOpenGLPixelTransferOptions::skipPixels();
  fn _ZNK27QOpenGLPixelTransferOptions10skipPixelsEv(qthis: *mut c_void) -> c_int;
  // proto:  void QOpenGLPixelTransferOptions::setRowLength(int rowLength);
  fn _ZN27QOpenGLPixelTransferOptions12setRowLengthEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QOpenGLPixelTransferOptions::imageHeight();
  fn _ZNK27QOpenGLPixelTransferOptions11imageHeightEv(qthis: *mut c_void) -> c_int;
  // proto:  void QOpenGLPixelTransferOptions::setImageHeight(int imageHeight);
  fn _ZN27QOpenGLPixelTransferOptions14setImageHeightEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QOpenGLPixelTransferOptions::skipRows();
  fn _ZNK27QOpenGLPixelTransferOptions8skipRowsEv(qthis: *mut c_void) -> c_int;
  // proto:  void QOpenGLPixelTransferOptions::setAlignment(int alignment);
  fn _ZN27QOpenGLPixelTransferOptions12setAlignmentEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QOpenGLPixelTransferOptions::setSkipImages(int skipImages);
  fn _ZN27QOpenGLPixelTransferOptions13setSkipImagesEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QOpenGLPixelTransferOptions::alignment();
  fn _ZNK27QOpenGLPixelTransferOptions9alignmentEv(qthis: *mut c_void) -> c_int;
  // proto:  void QOpenGLPixelTransferOptions::setSkipPixels(int skipPixels);
  fn _ZN27QOpenGLPixelTransferOptions13setSkipPixelsEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QOpenGLPixelTransferOptions::setSwapBytesEnabled(bool swapBytes);
  fn _ZN27QOpenGLPixelTransferOptions19setSwapBytesEnabledEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QOpenGLPixelTransferOptions::setLeastSignificantByteFirst(bool lsbFirst);
  fn _ZN27QOpenGLPixelTransferOptions28setLeastSignificantByteFirstEb(qthis: *mut c_void, arg0: c_char);
  // proto:  bool QOpenGLPixelTransferOptions::isLeastSignificantBitFirst();
  fn _ZNK27QOpenGLPixelTransferOptions26isLeastSignificantBitFirstEv(qthis: *mut c_void) -> c_char;
  // proto:  int QOpenGLPixelTransferOptions::rowLength();
  fn _ZNK27QOpenGLPixelTransferOptions9rowLengthEv(qthis: *mut c_void) -> c_int;
} // <= ext block end

// body block begin =>
// class sizeof(QOpenGLPixelTransferOptions)=1
pub struct QOpenGLPixelTransferOptions {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOpenGLPixelTransferOptions {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLPixelTransferOptions {
    return QOpenGLPixelTransferOptions{qclsinst: qthis};
  }
}
  // proto:  void QOpenGLPixelTransferOptions::QOpenGLPixelTransferOptions();
impl /*struct*/ QOpenGLPixelTransferOptions {
  pub fn New<T: QOpenGLPixelTransferOptions_New>(value: T) -> QOpenGLPixelTransferOptions {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLPixelTransferOptions_New {
  fn New(self) -> QOpenGLPixelTransferOptions;
}

  // proto:  void QOpenGLPixelTransferOptions::QOpenGLPixelTransferOptions();
impl<'a> /*trait*/ QOpenGLPixelTransferOptions_New for () {
  fn New(self) -> QOpenGLPixelTransferOptions {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QOpenGLPixelTransferOptionsC1Ev()};
    let ctysz: c_int = unsafe{QOpenGLPixelTransferOptions_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN27QOpenGLPixelTransferOptionsC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN27QOpenGLPixelTransferOptionsC1Ev()};
    let rsthis = QOpenGLPixelTransferOptions{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QOpenGLPixelTransferOptions::~QOpenGLPixelTransferOptions();
impl /*struct*/ QOpenGLPixelTransferOptions {
  pub fn Free<RetType, T: QOpenGLPixelTransferOptions_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QOpenGLPixelTransferOptions_Free<RetType> {
  fn Free(self , rsthis: & QOpenGLPixelTransferOptions) -> RetType;
}

  // proto:  void QOpenGLPixelTransferOptions::~QOpenGLPixelTransferOptions();
impl<'a> /*trait*/ QOpenGLPixelTransferOptions_Free<()> for () {
  fn Free(self , rsthis: & QOpenGLPixelTransferOptions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QOpenGLPixelTransferOptionsD0Ev()};
     unsafe {_ZN27QOpenGLPixelTransferOptionsD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QOpenGLPixelTransferOptions::isSwapBytesEnabled();
impl /*struct*/ QOpenGLPixelTransferOptions {
  pub fn isSwapBytesEnabled<RetType, T: QOpenGLPixelTransferOptions_isSwapBytesEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSwapBytesEnabled(self);
    // return 1;
  }
}

pub trait QOpenGLPixelTransferOptions_isSwapBytesEnabled<RetType> {
  fn isSwapBytesEnabled(self , rsthis: & QOpenGLPixelTransferOptions) -> RetType;
}

  // proto:  bool QOpenGLPixelTransferOptions::isSwapBytesEnabled();
impl<'a> /*trait*/ QOpenGLPixelTransferOptions_isSwapBytesEnabled<i8> for () {
  fn isSwapBytesEnabled(self , rsthis: & QOpenGLPixelTransferOptions) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QOpenGLPixelTransferOptions18isSwapBytesEnabledEv()};
    let mut ret = unsafe {_ZNK27QOpenGLPixelTransferOptions18isSwapBytesEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QOpenGLPixelTransferOptions::swap(QOpenGLPixelTransferOptions & other);
impl /*struct*/ QOpenGLPixelTransferOptions {
  pub fn swap<RetType, T: QOpenGLPixelTransferOptions_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QOpenGLPixelTransferOptions_swap<RetType> {
  fn swap(self , rsthis: & QOpenGLPixelTransferOptions) -> RetType;
}

  // proto:  void QOpenGLPixelTransferOptions::swap(QOpenGLPixelTransferOptions & other);
impl<'a> /*trait*/ QOpenGLPixelTransferOptions_swap<()> for (&'a QOpenGLPixelTransferOptions) {
  fn swap(self , rsthis: & QOpenGLPixelTransferOptions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QOpenGLPixelTransferOptions4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN27QOpenGLPixelTransferOptions4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLPixelTransferOptions::QOpenGLPixelTransferOptions(const QOpenGLPixelTransferOptions & );
impl<'a> /*trait*/ QOpenGLPixelTransferOptions_New for (&'a QOpenGLPixelTransferOptions) {
  fn New(self) -> QOpenGLPixelTransferOptions {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QOpenGLPixelTransferOptionsC1ERKS_()};
    let ctysz: c_int = unsafe{QOpenGLPixelTransferOptions_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN27QOpenGLPixelTransferOptionsC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN27QOpenGLPixelTransferOptionsC1ERKS_(arg0)};
    let rsthis = QOpenGLPixelTransferOptions{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QOpenGLPixelTransferOptions::skipImages();
impl /*struct*/ QOpenGLPixelTransferOptions {
  pub fn skipImages<RetType, T: QOpenGLPixelTransferOptions_skipImages<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.skipImages(self);
    // return 1;
  }
}

pub trait QOpenGLPixelTransferOptions_skipImages<RetType> {
  fn skipImages(self , rsthis: & QOpenGLPixelTransferOptions) -> RetType;
}

  // proto:  int QOpenGLPixelTransferOptions::skipImages();
impl<'a> /*trait*/ QOpenGLPixelTransferOptions_skipImages<i32> for () {
  fn skipImages(self , rsthis: & QOpenGLPixelTransferOptions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QOpenGLPixelTransferOptions10skipImagesEv()};
    let mut ret = unsafe {_ZNK27QOpenGLPixelTransferOptions10skipImagesEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QOpenGLPixelTransferOptions::setSkipRows(int skipRows);
impl /*struct*/ QOpenGLPixelTransferOptions {
  pub fn setSkipRows<RetType, T: QOpenGLPixelTransferOptions_setSkipRows<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSkipRows(self);
    // return 1;
  }
}

pub trait QOpenGLPixelTransferOptions_setSkipRows<RetType> {
  fn setSkipRows(self , rsthis: & QOpenGLPixelTransferOptions) -> RetType;
}

  // proto:  void QOpenGLPixelTransferOptions::setSkipRows(int skipRows);
impl<'a> /*trait*/ QOpenGLPixelTransferOptions_setSkipRows<()> for (i32) {
  fn setSkipRows(self , rsthis: & QOpenGLPixelTransferOptions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QOpenGLPixelTransferOptions11setSkipRowsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN27QOpenGLPixelTransferOptions11setSkipRowsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QOpenGLPixelTransferOptions::skipPixels();
impl /*struct*/ QOpenGLPixelTransferOptions {
  pub fn skipPixels<RetType, T: QOpenGLPixelTransferOptions_skipPixels<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.skipPixels(self);
    // return 1;
  }
}

pub trait QOpenGLPixelTransferOptions_skipPixels<RetType> {
  fn skipPixels(self , rsthis: & QOpenGLPixelTransferOptions) -> RetType;
}

  // proto:  int QOpenGLPixelTransferOptions::skipPixels();
impl<'a> /*trait*/ QOpenGLPixelTransferOptions_skipPixels<i32> for () {
  fn skipPixels(self , rsthis: & QOpenGLPixelTransferOptions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QOpenGLPixelTransferOptions10skipPixelsEv()};
    let mut ret = unsafe {_ZNK27QOpenGLPixelTransferOptions10skipPixelsEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QOpenGLPixelTransferOptions::setRowLength(int rowLength);
impl /*struct*/ QOpenGLPixelTransferOptions {
  pub fn setRowLength<RetType, T: QOpenGLPixelTransferOptions_setRowLength<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRowLength(self);
    // return 1;
  }
}

pub trait QOpenGLPixelTransferOptions_setRowLength<RetType> {
  fn setRowLength(self , rsthis: & QOpenGLPixelTransferOptions) -> RetType;
}

  // proto:  void QOpenGLPixelTransferOptions::setRowLength(int rowLength);
impl<'a> /*trait*/ QOpenGLPixelTransferOptions_setRowLength<()> for (i32) {
  fn setRowLength(self , rsthis: & QOpenGLPixelTransferOptions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QOpenGLPixelTransferOptions12setRowLengthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN27QOpenGLPixelTransferOptions12setRowLengthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QOpenGLPixelTransferOptions::imageHeight();
impl /*struct*/ QOpenGLPixelTransferOptions {
  pub fn imageHeight<RetType, T: QOpenGLPixelTransferOptions_imageHeight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.imageHeight(self);
    // return 1;
  }
}

pub trait QOpenGLPixelTransferOptions_imageHeight<RetType> {
  fn imageHeight(self , rsthis: & QOpenGLPixelTransferOptions) -> RetType;
}

  // proto:  int QOpenGLPixelTransferOptions::imageHeight();
impl<'a> /*trait*/ QOpenGLPixelTransferOptions_imageHeight<i32> for () {
  fn imageHeight(self , rsthis: & QOpenGLPixelTransferOptions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QOpenGLPixelTransferOptions11imageHeightEv()};
    let mut ret = unsafe {_ZNK27QOpenGLPixelTransferOptions11imageHeightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QOpenGLPixelTransferOptions::setImageHeight(int imageHeight);
impl /*struct*/ QOpenGLPixelTransferOptions {
  pub fn setImageHeight<RetType, T: QOpenGLPixelTransferOptions_setImageHeight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setImageHeight(self);
    // return 1;
  }
}

pub trait QOpenGLPixelTransferOptions_setImageHeight<RetType> {
  fn setImageHeight(self , rsthis: & QOpenGLPixelTransferOptions) -> RetType;
}

  // proto:  void QOpenGLPixelTransferOptions::setImageHeight(int imageHeight);
impl<'a> /*trait*/ QOpenGLPixelTransferOptions_setImageHeight<()> for (i32) {
  fn setImageHeight(self , rsthis: & QOpenGLPixelTransferOptions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QOpenGLPixelTransferOptions14setImageHeightEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN27QOpenGLPixelTransferOptions14setImageHeightEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QOpenGLPixelTransferOptions::skipRows();
impl /*struct*/ QOpenGLPixelTransferOptions {
  pub fn skipRows<RetType, T: QOpenGLPixelTransferOptions_skipRows<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.skipRows(self);
    // return 1;
  }
}

pub trait QOpenGLPixelTransferOptions_skipRows<RetType> {
  fn skipRows(self , rsthis: & QOpenGLPixelTransferOptions) -> RetType;
}

  // proto:  int QOpenGLPixelTransferOptions::skipRows();
impl<'a> /*trait*/ QOpenGLPixelTransferOptions_skipRows<i32> for () {
  fn skipRows(self , rsthis: & QOpenGLPixelTransferOptions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QOpenGLPixelTransferOptions8skipRowsEv()};
    let mut ret = unsafe {_ZNK27QOpenGLPixelTransferOptions8skipRowsEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QOpenGLPixelTransferOptions::setAlignment(int alignment);
impl /*struct*/ QOpenGLPixelTransferOptions {
  pub fn setAlignment<RetType, T: QOpenGLPixelTransferOptions_setAlignment<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAlignment(self);
    // return 1;
  }
}

pub trait QOpenGLPixelTransferOptions_setAlignment<RetType> {
  fn setAlignment(self , rsthis: & QOpenGLPixelTransferOptions) -> RetType;
}

  // proto:  void QOpenGLPixelTransferOptions::setAlignment(int alignment);
impl<'a> /*trait*/ QOpenGLPixelTransferOptions_setAlignment<()> for (i32) {
  fn setAlignment(self , rsthis: & QOpenGLPixelTransferOptions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QOpenGLPixelTransferOptions12setAlignmentEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN27QOpenGLPixelTransferOptions12setAlignmentEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLPixelTransferOptions::setSkipImages(int skipImages);
impl /*struct*/ QOpenGLPixelTransferOptions {
  pub fn setSkipImages<RetType, T: QOpenGLPixelTransferOptions_setSkipImages<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSkipImages(self);
    // return 1;
  }
}

pub trait QOpenGLPixelTransferOptions_setSkipImages<RetType> {
  fn setSkipImages(self , rsthis: & QOpenGLPixelTransferOptions) -> RetType;
}

  // proto:  void QOpenGLPixelTransferOptions::setSkipImages(int skipImages);
impl<'a> /*trait*/ QOpenGLPixelTransferOptions_setSkipImages<()> for (i32) {
  fn setSkipImages(self , rsthis: & QOpenGLPixelTransferOptions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QOpenGLPixelTransferOptions13setSkipImagesEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN27QOpenGLPixelTransferOptions13setSkipImagesEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QOpenGLPixelTransferOptions::alignment();
impl /*struct*/ QOpenGLPixelTransferOptions {
  pub fn alignment<RetType, T: QOpenGLPixelTransferOptions_alignment<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.alignment(self);
    // return 1;
  }
}

pub trait QOpenGLPixelTransferOptions_alignment<RetType> {
  fn alignment(self , rsthis: & QOpenGLPixelTransferOptions) -> RetType;
}

  // proto:  int QOpenGLPixelTransferOptions::alignment();
impl<'a> /*trait*/ QOpenGLPixelTransferOptions_alignment<i32> for () {
  fn alignment(self , rsthis: & QOpenGLPixelTransferOptions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QOpenGLPixelTransferOptions9alignmentEv()};
    let mut ret = unsafe {_ZNK27QOpenGLPixelTransferOptions9alignmentEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QOpenGLPixelTransferOptions::setSkipPixels(int skipPixels);
impl /*struct*/ QOpenGLPixelTransferOptions {
  pub fn setSkipPixels<RetType, T: QOpenGLPixelTransferOptions_setSkipPixels<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSkipPixels(self);
    // return 1;
  }
}

pub trait QOpenGLPixelTransferOptions_setSkipPixels<RetType> {
  fn setSkipPixels(self , rsthis: & QOpenGLPixelTransferOptions) -> RetType;
}

  // proto:  void QOpenGLPixelTransferOptions::setSkipPixels(int skipPixels);
impl<'a> /*trait*/ QOpenGLPixelTransferOptions_setSkipPixels<()> for (i32) {
  fn setSkipPixels(self , rsthis: & QOpenGLPixelTransferOptions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QOpenGLPixelTransferOptions13setSkipPixelsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN27QOpenGLPixelTransferOptions13setSkipPixelsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLPixelTransferOptions::setSwapBytesEnabled(bool swapBytes);
impl /*struct*/ QOpenGLPixelTransferOptions {
  pub fn setSwapBytesEnabled<RetType, T: QOpenGLPixelTransferOptions_setSwapBytesEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSwapBytesEnabled(self);
    // return 1;
  }
}

pub trait QOpenGLPixelTransferOptions_setSwapBytesEnabled<RetType> {
  fn setSwapBytesEnabled(self , rsthis: & QOpenGLPixelTransferOptions) -> RetType;
}

  // proto:  void QOpenGLPixelTransferOptions::setSwapBytesEnabled(bool swapBytes);
impl<'a> /*trait*/ QOpenGLPixelTransferOptions_setSwapBytesEnabled<()> for (i8) {
  fn setSwapBytesEnabled(self , rsthis: & QOpenGLPixelTransferOptions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QOpenGLPixelTransferOptions19setSwapBytesEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN27QOpenGLPixelTransferOptions19setSwapBytesEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLPixelTransferOptions::setLeastSignificantByteFirst(bool lsbFirst);
impl /*struct*/ QOpenGLPixelTransferOptions {
  pub fn setLeastSignificantByteFirst<RetType, T: QOpenGLPixelTransferOptions_setLeastSignificantByteFirst<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLeastSignificantByteFirst(self);
    // return 1;
  }
}

pub trait QOpenGLPixelTransferOptions_setLeastSignificantByteFirst<RetType> {
  fn setLeastSignificantByteFirst(self , rsthis: & QOpenGLPixelTransferOptions) -> RetType;
}

  // proto:  void QOpenGLPixelTransferOptions::setLeastSignificantByteFirst(bool lsbFirst);
impl<'a> /*trait*/ QOpenGLPixelTransferOptions_setLeastSignificantByteFirst<()> for (i8) {
  fn setLeastSignificantByteFirst(self , rsthis: & QOpenGLPixelTransferOptions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QOpenGLPixelTransferOptions28setLeastSignificantByteFirstEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN27QOpenGLPixelTransferOptions28setLeastSignificantByteFirstEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QOpenGLPixelTransferOptions::isLeastSignificantBitFirst();
impl /*struct*/ QOpenGLPixelTransferOptions {
  pub fn isLeastSignificantBitFirst<RetType, T: QOpenGLPixelTransferOptions_isLeastSignificantBitFirst<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isLeastSignificantBitFirst(self);
    // return 1;
  }
}

pub trait QOpenGLPixelTransferOptions_isLeastSignificantBitFirst<RetType> {
  fn isLeastSignificantBitFirst(self , rsthis: & QOpenGLPixelTransferOptions) -> RetType;
}

  // proto:  bool QOpenGLPixelTransferOptions::isLeastSignificantBitFirst();
impl<'a> /*trait*/ QOpenGLPixelTransferOptions_isLeastSignificantBitFirst<i8> for () {
  fn isLeastSignificantBitFirst(self , rsthis: & QOpenGLPixelTransferOptions) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QOpenGLPixelTransferOptions26isLeastSignificantBitFirstEv()};
    let mut ret = unsafe {_ZNK27QOpenGLPixelTransferOptions26isLeastSignificantBitFirstEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QOpenGLPixelTransferOptions::rowLength();
impl /*struct*/ QOpenGLPixelTransferOptions {
  pub fn rowLength<RetType, T: QOpenGLPixelTransferOptions_rowLength<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rowLength(self);
    // return 1;
  }
}

pub trait QOpenGLPixelTransferOptions_rowLength<RetType> {
  fn rowLength(self , rsthis: & QOpenGLPixelTransferOptions) -> RetType;
}

  // proto:  int QOpenGLPixelTransferOptions::rowLength();
impl<'a> /*trait*/ QOpenGLPixelTransferOptions_rowLength<i32> for () {
  fn rowLength(self , rsthis: & QOpenGLPixelTransferOptions) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QOpenGLPixelTransferOptions9rowLengthEv()};
    let mut ret = unsafe {_ZNK27QOpenGLPixelTransferOptions9rowLengthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// <= body block end

