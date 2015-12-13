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
  // proto: QString QAccessibleImageInterface::imageDescription();
  fn _ZNK25QAccessibleImageInterface16imageDescriptionEv() -> i32;
  // proto: QPoint QAccessibleImageInterface::imagePosition();
  fn _ZNK25QAccessibleImageInterface13imagePositionEv() -> i32;
  // proto: void QAccessibleImageInterface::FreeQAccessibleImageInterface();
  fn _ZN25QAccessibleImageInterfaceD0Ev() -> i32;
  // proto: QSize QAccessibleImageInterface::imageSize();
  fn _ZNK25QAccessibleImageInterface9imageSizeEv() -> i32;
}

// body block begin
// class sizeof(QAccessibleImageInterface)=8
pub struct QAccessibleImageInterface {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAccessibleImageInterface {
  pub fn imageDescription<T: QAccessibleImageInterface_imageDescription>(&mut self, value: T) -> i32 {
    value.imageDescription(self);
    return 1;
  }
}

pub trait QAccessibleImageInterface_imageDescription {
  fn imageDescription(self, this: &mut QAccessibleImageInterface) -> i32;
}

// proto: QString QAccessibleImageInterface::imageDescription();
impl<'a> /*trait*/ QAccessibleImageInterface_imageDescription for () {
  fn imageDescription(self, this: &mut QAccessibleImageInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleImageInterface16imageDescriptionEv()};
    unsafe {_ZNK25QAccessibleImageInterface16imageDescriptionEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleImageInterface {
  pub fn imagePosition<T: QAccessibleImageInterface_imagePosition>(&mut self, value: T) -> i32 {
    value.imagePosition(self);
    return 1;
  }
}

pub trait QAccessibleImageInterface_imagePosition {
  fn imagePosition(self, this: &mut QAccessibleImageInterface) -> i32;
}

// proto: QPoint QAccessibleImageInterface::imagePosition();
impl<'a> /*trait*/ QAccessibleImageInterface_imagePosition for () {
  fn imagePosition(self, this: &mut QAccessibleImageInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleImageInterface13imagePositionEv()};
    unsafe {_ZNK25QAccessibleImageInterface13imagePositionEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleImageInterface {
  pub fn FreeQAccessibleImageInterface<T: QAccessibleImageInterface_FreeQAccessibleImageInterface>(&mut self, value: T) -> i32 {
    value.FreeQAccessibleImageInterface(self);
    return 1;
  }
}

pub trait QAccessibleImageInterface_FreeQAccessibleImageInterface {
  fn FreeQAccessibleImageInterface(self, this: &mut QAccessibleImageInterface) -> i32;
}

// proto: void QAccessibleImageInterface::FreeQAccessibleImageInterface();
impl<'a> /*trait*/ QAccessibleImageInterface_FreeQAccessibleImageInterface for () {
  fn FreeQAccessibleImageInterface(self, this: &mut QAccessibleImageInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QAccessibleImageInterfaceD0Ev()};
    unsafe {_ZN25QAccessibleImageInterfaceD0Ev()};
    return 1;
  }
}

impl /*struct*/ QAccessibleImageInterface {
  pub fn imageSize<T: QAccessibleImageInterface_imageSize>(&mut self, value: T) -> i32 {
    value.imageSize(self);
    return 1;
  }
}

pub trait QAccessibleImageInterface_imageSize {
  fn imageSize(self, this: &mut QAccessibleImageInterface) -> i32;
}

// proto: QSize QAccessibleImageInterface::imageSize();
impl<'a> /*trait*/ QAccessibleImageInterface_imageSize for () {
  fn imageSize(self, this: &mut QAccessibleImageInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleImageInterface9imageSizeEv()};
    unsafe {_ZNK25QAccessibleImageInterface9imageSizeEv()};
    return 1;
  }
}

