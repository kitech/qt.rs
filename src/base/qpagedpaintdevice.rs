// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qmarginsf::QMarginsF;
use super::qpagesize::QPageSize;
use super::qsizef::QSizeF;
use super::qpagelayout::QPageLayout;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QSizeF QPagedPaintDevice::pageSizeMM();
  fn _ZNK17QPagedPaintDevice10pageSizeMMEv() -> i32;
  // proto: void QPagedPaintDevice::FreeQPagedPaintDevice();
  fn _ZN17QPagedPaintDeviceD0Ev() -> i32;
  // proto: bool QPagedPaintDevice::setPageMargins(const QMarginsF & margins);
  fn _ZN17QPagedPaintDevice14setPageMarginsERK9QMarginsF(arg0: *const c_void) -> i32;
  // proto: QPageLayout QPagedPaintDevice::pageLayout();
  fn _ZNK17QPagedPaintDevice10pageLayoutEv() -> i32;
  // proto: bool QPagedPaintDevice::setPageSize(const QPageSize & pageSize);
  fn _ZN17QPagedPaintDevice11setPageSizeERK9QPageSize(arg0: *const c_void) -> i32;
  // proto: void QPagedPaintDevice::NewQPagedPaintDevice();
  fn _ZN17QPagedPaintDeviceC1Ev(qthis: *mut c_void) -> i32;
  // proto: void QPagedPaintDevice::setPageSizeMM(const QSizeF & size);
  fn _ZN17QPagedPaintDevice13setPageSizeMMERK6QSizeF(arg0: *const c_void) -> i32;
  // proto: bool QPagedPaintDevice::setPageLayout(const QPageLayout & pageLayout);
  fn _ZN17QPagedPaintDevice13setPageLayoutERK11QPageLayout(arg0: *const c_void) -> i32;
  // proto: bool QPagedPaintDevice::newPage();
  fn _ZN17QPagedPaintDevice7newPageEv() -> i32;
}

// body block begin
// class sizeof(QPagedPaintDevice)=32
pub struct QPagedPaintDevice {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPagedPaintDevice {
  pub fn pageSizeMM<T: QPagedPaintDevice_pageSizeMM>(&mut self, value: T) -> i32 {
    value.pageSizeMM(self);
    return 1;
  }
}

pub trait QPagedPaintDevice_pageSizeMM {
  fn pageSizeMM(self, this: &mut QPagedPaintDevice) -> i32;
}

// proto: QSizeF QPagedPaintDevice::pageSizeMM();
impl<'a> /*trait*/ QPagedPaintDevice_pageSizeMM for () {
  fn pageSizeMM(self, this: &mut QPagedPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPagedPaintDevice10pageSizeMMEv()};
    unsafe {_ZNK17QPagedPaintDevice10pageSizeMMEv()};
    return 1;
  }
}

impl /*struct*/ QPagedPaintDevice {
  pub fn FreeQPagedPaintDevice<T: QPagedPaintDevice_FreeQPagedPaintDevice>(&mut self, value: T) -> i32 {
    value.FreeQPagedPaintDevice(self);
    return 1;
  }
}

pub trait QPagedPaintDevice_FreeQPagedPaintDevice {
  fn FreeQPagedPaintDevice(self, this: &mut QPagedPaintDevice) -> i32;
}

// proto: void QPagedPaintDevice::FreeQPagedPaintDevice();
impl<'a> /*trait*/ QPagedPaintDevice_FreeQPagedPaintDevice for () {
  fn FreeQPagedPaintDevice(self, this: &mut QPagedPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QPagedPaintDeviceD0Ev()};
    unsafe {_ZN17QPagedPaintDeviceD0Ev()};
    return 1;
  }
}

impl /*struct*/ QPagedPaintDevice {
  pub fn setPageMargins<T: QPagedPaintDevice_setPageMargins>(&mut self, value: T) -> i32 {
    value.setPageMargins(self);
    return 1;
  }
}

pub trait QPagedPaintDevice_setPageMargins {
  fn setPageMargins(self, this: &mut QPagedPaintDevice) -> i32;
}

// proto: bool QPagedPaintDevice::setPageMargins(const QMarginsF & margins);
impl<'a> /*trait*/ QPagedPaintDevice_setPageMargins for (&'a  QMarginsF) {
  fn setPageMargins(self, this: &mut QPagedPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QPagedPaintDevice14setPageMarginsERK9QMarginsF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN17QPagedPaintDevice14setPageMarginsERK9QMarginsF(arg0)};
    return 1;
  }
}

impl /*struct*/ QPagedPaintDevice {
  pub fn pageLayout<T: QPagedPaintDevice_pageLayout>(&mut self, value: T) -> i32 {
    value.pageLayout(self);
    return 1;
  }
}

pub trait QPagedPaintDevice_pageLayout {
  fn pageLayout(self, this: &mut QPagedPaintDevice) -> i32;
}

// proto: QPageLayout QPagedPaintDevice::pageLayout();
impl<'a> /*trait*/ QPagedPaintDevice_pageLayout for () {
  fn pageLayout(self, this: &mut QPagedPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPagedPaintDevice10pageLayoutEv()};
    unsafe {_ZNK17QPagedPaintDevice10pageLayoutEv()};
    return 1;
  }
}

impl /*struct*/ QPagedPaintDevice {
  pub fn setPageSize<T: QPagedPaintDevice_setPageSize>(&mut self, value: T) -> i32 {
    value.setPageSize(self);
    return 1;
  }
}

pub trait QPagedPaintDevice_setPageSize {
  fn setPageSize(self, this: &mut QPagedPaintDevice) -> i32;
}

// proto: bool QPagedPaintDevice::setPageSize(const QPageSize & pageSize);
impl<'a> /*trait*/ QPagedPaintDevice_setPageSize for (&'a  QPageSize) {
  fn setPageSize(self, this: &mut QPagedPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QPagedPaintDevice11setPageSizeERK9QPageSize()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN17QPagedPaintDevice11setPageSizeERK9QPageSize(arg0)};
    return 1;
  }
}

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

// proto: void QPagedPaintDevice::NewQPagedPaintDevice();
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

impl /*struct*/ QPagedPaintDevice {
  pub fn setPageSizeMM<T: QPagedPaintDevice_setPageSizeMM>(&mut self, value: T) -> i32 {
    value.setPageSizeMM(self);
    return 1;
  }
}

pub trait QPagedPaintDevice_setPageSizeMM {
  fn setPageSizeMM(self, this: &mut QPagedPaintDevice) -> i32;
}

// proto: void QPagedPaintDevice::setPageSizeMM(const QSizeF & size);
impl<'a> /*trait*/ QPagedPaintDevice_setPageSizeMM for (&'a  QSizeF) {
  fn setPageSizeMM(self, this: &mut QPagedPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QPagedPaintDevice13setPageSizeMMERK6QSizeF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN17QPagedPaintDevice13setPageSizeMMERK6QSizeF(arg0)};
    return 1;
  }
}

impl /*struct*/ QPagedPaintDevice {
  pub fn setPageLayout<T: QPagedPaintDevice_setPageLayout>(&mut self, value: T) -> i32 {
    value.setPageLayout(self);
    return 1;
  }
}

pub trait QPagedPaintDevice_setPageLayout {
  fn setPageLayout(self, this: &mut QPagedPaintDevice) -> i32;
}

// proto: bool QPagedPaintDevice::setPageLayout(const QPageLayout & pageLayout);
impl<'a> /*trait*/ QPagedPaintDevice_setPageLayout for (&'a  QPageLayout) {
  fn setPageLayout(self, this: &mut QPagedPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QPagedPaintDevice13setPageLayoutERK11QPageLayout()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN17QPagedPaintDevice13setPageLayoutERK11QPageLayout(arg0)};
    return 1;
  }
}

impl /*struct*/ QPagedPaintDevice {
  pub fn newPage<T: QPagedPaintDevice_newPage>(&mut self, value: T) -> i32 {
    value.newPage(self);
    return 1;
  }
}

pub trait QPagedPaintDevice_newPage {
  fn newPage(self, this: &mut QPagedPaintDevice) -> i32;
}

// proto: bool QPagedPaintDevice::newPage();
impl<'a> /*trait*/ QPagedPaintDevice_newPage for () {
  fn newPage(self, this: &mut QPagedPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QPagedPaintDevice7newPageEv()};
    unsafe {_ZN17QPagedPaintDevice7newPageEv()};
    return 1;
  }
}

