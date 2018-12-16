

// mod ::gui::QAccessibleImageInterface
// package qtgui
// /usr/include/qt/QtGui/qaccessible.h
// #include <qaccessible.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 18
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QAccessibleImageInterface)=8
pub struct QAccessibleImageInterface {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAccessibleImageInterface_ITF interface {
//    QAccessibleImageInterface_PTR() *QAccessibleImageInterface
//}
//func (ptr *QAccessibleImageInterface) QAccessibleImageInterface_PTR() *QAccessibleImageInterface { return ptr }

impl /*struct*/ QAccessibleImageInterface {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAccessibleImageInterface {
    return QAccessibleImageInterface{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAccessibleImageInterface {
//  type Target = QAccessibleImageInterfaceBASE;
//
//  fn deref(&self) -> &QAccessibleImageInterfaceBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAccessibleImageInterfaceBASE> for QAccessibleImageInterface {
//  fn as_ref(& self) -> & QAccessibleImageInterfaceBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qaccessible.h:655
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QAccessibleImageInterface()

/*

*/
pub fn DeleteQAccessibleImageInterface(this :*mut QAccessibleImageInterface) {
    // let rv = qtrt::InvokeQtFunc6("_ZN25QAccessibleImageInterfaceD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qaccessible.h:657
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [8] QString imageDescription() const

/*

*/
impl /*struct*/ QAccessibleImageInterface {
  pub fn imageDescription_0<RetType, T: QAccessibleImageInterface_imageDescription_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.imageDescription_0(self);
    // return 1;
  }
}
pub trait QAccessibleImageInterface_imageDescription_0<RetType> {
  fn imageDescription_0(self , rsthis: & QAccessibleImageInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleImageInterface_imageDescription_0<usize> for () {
  fn imageDescription_0(self , rsthis: & QAccessibleImageInterface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK25QAccessibleImageInterface16imageDescriptionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:658
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [8] QSize imageSize() const

/*

*/
impl /*struct*/ QAccessibleImageInterface {
  pub fn imageSize_0<RetType, T: QAccessibleImageInterface_imageSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.imageSize_0(self);
    // return 1;
  }
}
pub trait QAccessibleImageInterface_imageSize_0<RetType> {
  fn imageSize_0(self , rsthis: & QAccessibleImageInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleImageInterface_imageSize_0<usize> for () {
  fn imageSize_0(self , rsthis: & QAccessibleImageInterface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK25QAccessibleImageInterface9imageSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:659
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [8] QPoint imagePosition() const

/*

*/
impl /*struct*/ QAccessibleImageInterface {
  pub fn imagePosition_0<RetType, T: QAccessibleImageInterface_imagePosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.imagePosition_0(self);
    // return 1;
  }
}
pub trait QAccessibleImageInterface_imagePosition_0<RetType> {
  fn imagePosition_0(self , rsthis: & QAccessibleImageInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleImageInterface_imagePosition_0<usize> for () {
  fn imagePosition_0(self , rsthis: & QAccessibleImageInterface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK25QAccessibleImageInterface13imagePositionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
