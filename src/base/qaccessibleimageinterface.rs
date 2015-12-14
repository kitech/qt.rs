// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qpoint::QPoint;
use super::qsize::QSize;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QString QAccessibleImageInterface::imageDescription();
  fn _ZNK25QAccessibleImageInterface16imageDescriptionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPoint QAccessibleImageInterface::imagePosition();
  fn _ZNK25QAccessibleImageInterface13imagePositionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAccessibleImageInterface::FreeQAccessibleImageInterface();
  fn _ZN25QAccessibleImageInterfaceD0Ev(qthis: *mut c_void) ;
  // proto:  QSize QAccessibleImageInterface::imageSize();
  fn _ZNK25QAccessibleImageInterface9imageSizeEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QAccessibleImageInterface)=8
pub struct QAccessibleImageInterface {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAccessibleImageInterface {
  pub fn imageDescription<T: QAccessibleImageInterface_imageDescription>(&mut self, value: T) -> QString {
    return value.imageDescription(self);
    // return 1;
  }
}

pub trait QAccessibleImageInterface_imageDescription {
  fn imageDescription(self, rsthis: &mut QAccessibleImageInterface) -> QString;
}

// proto:  QString QAccessibleImageInterface::imageDescription();
impl<'a> /*trait*/ QAccessibleImageInterface_imageDescription for () {
  fn imageDescription(self, rsthis: &mut QAccessibleImageInterface) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleImageInterface16imageDescriptionEv()};
    let mut ret = unsafe {_ZNK25QAccessibleImageInterface16imageDescriptionEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleImageInterface {
  pub fn imagePosition<T: QAccessibleImageInterface_imagePosition>(&mut self, value: T) -> QPoint {
    return value.imagePosition(self);
    // return 1;
  }
}

pub trait QAccessibleImageInterface_imagePosition {
  fn imagePosition(self, rsthis: &mut QAccessibleImageInterface) -> QPoint;
}

// proto:  QPoint QAccessibleImageInterface::imagePosition();
impl<'a> /*trait*/ QAccessibleImageInterface_imagePosition for () {
  fn imagePosition(self, rsthis: &mut QAccessibleImageInterface) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleImageInterface13imagePositionEv()};
    let mut ret = unsafe {_ZNK25QAccessibleImageInterface13imagePositionEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleImageInterface {
  pub fn FreeQAccessibleImageInterface<T: QAccessibleImageInterface_FreeQAccessibleImageInterface>(&mut self, value: T)  {
     value.FreeQAccessibleImageInterface(self);
    // return 1;
  }
}

pub trait QAccessibleImageInterface_FreeQAccessibleImageInterface {
  fn FreeQAccessibleImageInterface(self, rsthis: &mut QAccessibleImageInterface) ;
}

// proto:  void QAccessibleImageInterface::FreeQAccessibleImageInterface();
impl<'a> /*trait*/ QAccessibleImageInterface_FreeQAccessibleImageInterface for () {
  fn FreeQAccessibleImageInterface(self, rsthis: &mut QAccessibleImageInterface)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QAccessibleImageInterfaceD0Ev()};
     unsafe {_ZN25QAccessibleImageInterfaceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QAccessibleImageInterface {
  pub fn imageSize<T: QAccessibleImageInterface_imageSize>(&mut self, value: T) -> QSize {
    return value.imageSize(self);
    // return 1;
  }
}

pub trait QAccessibleImageInterface_imageSize {
  fn imageSize(self, rsthis: &mut QAccessibleImageInterface) -> QSize;
}

// proto:  QSize QAccessibleImageInterface::imageSize();
impl<'a> /*trait*/ QAccessibleImageInterface_imageSize for () {
  fn imageSize(self, rsthis: &mut QAccessibleImageInterface) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleImageInterface9imageSizeEv()};
    let mut ret = unsafe {_ZNK25QAccessibleImageInterface9imageSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

