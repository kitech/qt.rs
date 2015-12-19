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

// proto:  QString QAccessibleImageInterface::imageDescription();
impl /*struct*/ QAccessibleImageInterface {
  pub fn imageDescription<RetType, T: QAccessibleImageInterface_imageDescription<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.imageDescription(self);
    // return 1;
  }
}

pub trait QAccessibleImageInterface_imageDescription<RetType> {
  fn imageDescription(self , rsthis: &mut QAccessibleImageInterface) -> RetType;
}

// proto:  QString QAccessibleImageInterface::imageDescription();
impl<'a> /*trait*/ QAccessibleImageInterface_imageDescription<QString> for () {
  fn imageDescription(self , rsthis: &mut QAccessibleImageInterface) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleImageInterface16imageDescriptionEv()};
    let mut ret = unsafe {_ZNK25QAccessibleImageInterface16imageDescriptionEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QPoint QAccessibleImageInterface::imagePosition();
impl /*struct*/ QAccessibleImageInterface {
  pub fn imagePosition<RetType, T: QAccessibleImageInterface_imagePosition<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.imagePosition(self);
    // return 1;
  }
}

pub trait QAccessibleImageInterface_imagePosition<RetType> {
  fn imagePosition(self , rsthis: &mut QAccessibleImageInterface) -> RetType;
}

// proto:  QPoint QAccessibleImageInterface::imagePosition();
impl<'a> /*trait*/ QAccessibleImageInterface_imagePosition<QPoint> for () {
  fn imagePosition(self , rsthis: &mut QAccessibleImageInterface) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleImageInterface13imagePositionEv()};
    let mut ret = unsafe {_ZNK25QAccessibleImageInterface13imagePositionEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QAccessibleImageInterface::FreeQAccessibleImageInterface();
impl /*struct*/ QAccessibleImageInterface {
  pub fn FreeQAccessibleImageInterface<RetType, T: QAccessibleImageInterface_FreeQAccessibleImageInterface<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQAccessibleImageInterface(self);
    // return 1;
  }
}

pub trait QAccessibleImageInterface_FreeQAccessibleImageInterface<RetType> {
  fn FreeQAccessibleImageInterface(self , rsthis: &mut QAccessibleImageInterface) -> RetType;
}

// proto:  void QAccessibleImageInterface::FreeQAccessibleImageInterface();
impl<'a> /*trait*/ QAccessibleImageInterface_FreeQAccessibleImageInterface<()> for () {
  fn FreeQAccessibleImageInterface(self , rsthis: &mut QAccessibleImageInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QAccessibleImageInterfaceD0Ev()};
     unsafe {_ZN25QAccessibleImageInterfaceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QSize QAccessibleImageInterface::imageSize();
impl /*struct*/ QAccessibleImageInterface {
  pub fn imageSize<RetType, T: QAccessibleImageInterface_imageSize<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.imageSize(self);
    // return 1;
  }
}

pub trait QAccessibleImageInterface_imageSize<RetType> {
  fn imageSize(self , rsthis: &mut QAccessibleImageInterface) -> RetType;
}

// proto:  QSize QAccessibleImageInterface::imageSize();
impl<'a> /*trait*/ QAccessibleImageInterface_imageSize<QSize> for () {
  fn imageSize(self , rsthis: &mut QAccessibleImageInterface) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleImageInterface9imageSizeEv()};
    let mut ret = unsafe {_ZNK25QAccessibleImageInterface9imageSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

