// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtGui/qpagedpaintdevice.h
// dst-file: /src/gui/qpagedpaintdevice.rs
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
use super::super::core::qsize::QSizeF; // 771
use super::super::core::qmargins::QMarginsF; // 771
use super::qpagelayout::QPageLayout; // 773
use super::qpagesize::QPageSize; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  QSizeF QPagedPaintDevice::pageSizeMM();
  fn _ZNK17QPagedPaintDevice10pageSizeMMEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPagedPaintDevice::~QPagedPaintDevice();
  fn _ZN17QPagedPaintDeviceD0Ev(qthis: *mut c_void);
  // proto:  bool QPagedPaintDevice::setPageMargins(const QMarginsF & margins);
  fn _ZN17QPagedPaintDevice14setPageMarginsERK9QMarginsF(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  QPageLayout QPagedPaintDevice::pageLayout();
  fn _ZNK17QPagedPaintDevice10pageLayoutEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QPagedPaintDevice::setPageSize(const QPageSize & pageSize);
  fn _ZN17QPagedPaintDevice11setPageSizeERK9QPageSize(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  void QPagedPaintDevice::QPagedPaintDevice();
  fn _ZN17QPagedPaintDeviceC1Ev(qthis: *mut c_void);
  // proto:  void QPagedPaintDevice::setPageSizeMM(const QSizeF & size);
  fn _ZN17QPagedPaintDevice13setPageSizeMMERK6QSizeF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QPagedPaintDevice::setPageLayout(const QPageLayout & pageLayout);
  fn _ZN17QPagedPaintDevice13setPageLayoutERK11QPageLayout(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  bool QPagedPaintDevice::newPage();
  fn _ZN17QPagedPaintDevice7newPageEv(qthis: *mut c_void) -> c_char;
} // <= ext block end

// body block begin =>
// class sizeof(QPagedPaintDevice)=32
pub struct QPagedPaintDevice {
  pub qclsinst: *mut c_void,
}

  // proto:  QSizeF QPagedPaintDevice::pageSizeMM();
impl /*struct*/ QPagedPaintDevice {
  pub fn pageSizeMM<RetType, T: QPagedPaintDevice_pageSizeMM<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.pageSizeMM(self);
    // return 1;
  }
}

pub trait QPagedPaintDevice_pageSizeMM<RetType> {
  fn pageSizeMM(self , rsthis: &mut QPagedPaintDevice) -> RetType;
}

  // proto:  QSizeF QPagedPaintDevice::pageSizeMM();
impl<'a> /*trait*/ QPagedPaintDevice_pageSizeMM<QSizeF> for () {
  fn pageSizeMM(self , rsthis: &mut QPagedPaintDevice) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPagedPaintDevice10pageSizeMMEv()};
    let mut ret = unsafe {_ZNK17QPagedPaintDevice10pageSizeMMEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPagedPaintDevice::~QPagedPaintDevice();
impl /*struct*/ QPagedPaintDevice {
  pub fn FreeQPagedPaintDevice<RetType, T: QPagedPaintDevice_FreeQPagedPaintDevice<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQPagedPaintDevice(self);
    // return 1;
  }
}

pub trait QPagedPaintDevice_FreeQPagedPaintDevice<RetType> {
  fn FreeQPagedPaintDevice(self , rsthis: &mut QPagedPaintDevice) -> RetType;
}

  // proto:  void QPagedPaintDevice::~QPagedPaintDevice();
impl<'a> /*trait*/ QPagedPaintDevice_FreeQPagedPaintDevice<()> for () {
  fn FreeQPagedPaintDevice(self , rsthis: &mut QPagedPaintDevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QPagedPaintDeviceD0Ev()};
     unsafe {_ZN17QPagedPaintDeviceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QPagedPaintDevice::setPageMargins(const QMarginsF & margins);
impl /*struct*/ QPagedPaintDevice {
  pub fn setPageMargins<RetType, T: QPagedPaintDevice_setPageMargins<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setPageMargins(self);
    // return 1;
  }
}

pub trait QPagedPaintDevice_setPageMargins<RetType> {
  fn setPageMargins(self , rsthis: &mut QPagedPaintDevice) -> RetType;
}

  // proto:  bool QPagedPaintDevice::setPageMargins(const QMarginsF & margins);
impl<'a> /*trait*/ QPagedPaintDevice_setPageMargins<i8> for (QMarginsF) {
  fn setPageMargins(self , rsthis: &mut QPagedPaintDevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QPagedPaintDevice14setPageMarginsERK9QMarginsF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN17QPagedPaintDevice14setPageMarginsERK9QMarginsF(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QPageLayout QPagedPaintDevice::pageLayout();
impl /*struct*/ QPagedPaintDevice {
  pub fn pageLayout<RetType, T: QPagedPaintDevice_pageLayout<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.pageLayout(self);
    // return 1;
  }
}

pub trait QPagedPaintDevice_pageLayout<RetType> {
  fn pageLayout(self , rsthis: &mut QPagedPaintDevice) -> RetType;
}

  // proto:  QPageLayout QPagedPaintDevice::pageLayout();
impl<'a> /*trait*/ QPagedPaintDevice_pageLayout<QPageLayout> for () {
  fn pageLayout(self , rsthis: &mut QPagedPaintDevice) -> QPageLayout {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPagedPaintDevice10pageLayoutEv()};
    let mut ret = unsafe {_ZNK17QPagedPaintDevice10pageLayoutEv(rsthis.qclsinst)};
    let mut ret1 = QPageLayout{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QPagedPaintDevice::setPageSize(const QPageSize & pageSize);
impl /*struct*/ QPagedPaintDevice {
  pub fn setPageSize<RetType, T: QPagedPaintDevice_setPageSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setPageSize(self);
    // return 1;
  }
}

pub trait QPagedPaintDevice_setPageSize<RetType> {
  fn setPageSize(self , rsthis: &mut QPagedPaintDevice) -> RetType;
}

  // proto:  bool QPagedPaintDevice::setPageSize(const QPageSize & pageSize);
impl<'a> /*trait*/ QPagedPaintDevice_setPageSize<i8> for (QPageSize) {
  fn setPageSize(self , rsthis: &mut QPagedPaintDevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QPagedPaintDevice11setPageSizeERK9QPageSize()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN17QPagedPaintDevice11setPageSizeERK9QPageSize(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPagedPaintDevice::QPagedPaintDevice();
impl /*struct*/ QPagedPaintDevice {
  pub fn NewQPagedPaintDevice<T: QPagedPaintDevice_NewQPagedPaintDevice>(value: T) -> QPagedPaintDevice {
    let rsthis = value.NewQPagedPaintDevice();
    return rsthis;
    // return 1;
  }
}

pub trait QPagedPaintDevice_NewQPagedPaintDevice {
  fn NewQPagedPaintDevice(self) -> QPagedPaintDevice;
}

  // proto:  void QPagedPaintDevice::QPagedPaintDevice();
impl<'a> /*trait*/ QPagedPaintDevice_NewQPagedPaintDevice for () {
  fn NewQPagedPaintDevice(self) -> QPagedPaintDevice {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QPagedPaintDeviceC1Ev()};
    unsafe {_ZN17QPagedPaintDeviceC1Ev(qthis)};
    let rsthis = QPagedPaintDevice{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPagedPaintDevice::setPageSizeMM(const QSizeF & size);
impl /*struct*/ QPagedPaintDevice {
  pub fn setPageSizeMM<RetType, T: QPagedPaintDevice_setPageSizeMM<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setPageSizeMM(self);
    // return 1;
  }
}

pub trait QPagedPaintDevice_setPageSizeMM<RetType> {
  fn setPageSizeMM(self , rsthis: &mut QPagedPaintDevice) -> RetType;
}

  // proto:  void QPagedPaintDevice::setPageSizeMM(const QSizeF & size);
impl<'a> /*trait*/ QPagedPaintDevice_setPageSizeMM<()> for (QSizeF) {
  fn setPageSizeMM(self , rsthis: &mut QPagedPaintDevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QPagedPaintDevice13setPageSizeMMERK6QSizeF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QPagedPaintDevice13setPageSizeMMERK6QSizeF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QPagedPaintDevice::setPageLayout(const QPageLayout & pageLayout);
impl /*struct*/ QPagedPaintDevice {
  pub fn setPageLayout<RetType, T: QPagedPaintDevice_setPageLayout<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setPageLayout(self);
    // return 1;
  }
}

pub trait QPagedPaintDevice_setPageLayout<RetType> {
  fn setPageLayout(self , rsthis: &mut QPagedPaintDevice) -> RetType;
}

  // proto:  bool QPagedPaintDevice::setPageLayout(const QPageLayout & pageLayout);
impl<'a> /*trait*/ QPagedPaintDevice_setPageLayout<i8> for (QPageLayout) {
  fn setPageLayout(self , rsthis: &mut QPagedPaintDevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QPagedPaintDevice13setPageLayoutERK11QPageLayout()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN17QPagedPaintDevice13setPageLayoutERK11QPageLayout(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QPagedPaintDevice::newPage();
impl /*struct*/ QPagedPaintDevice {
  pub fn newPage<RetType, T: QPagedPaintDevice_newPage<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.newPage(self);
    // return 1;
  }
}

pub trait QPagedPaintDevice_newPage<RetType> {
  fn newPage(self , rsthis: &mut QPagedPaintDevice) -> RetType;
}

  // proto:  bool QPagedPaintDevice::newPage();
impl<'a> /*trait*/ QPagedPaintDevice_newPage<i8> for () {
  fn newPage(self , rsthis: &mut QPagedPaintDevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QPagedPaintDevice7newPageEv()};
    let mut ret = unsafe {_ZN17QPagedPaintDevice7newPageEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// <= body block end

