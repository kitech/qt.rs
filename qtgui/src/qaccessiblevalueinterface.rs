

// mod ::gui::QAccessibleValueInterface
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
// extern C begin: 4
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
#[derive(Default)] // class sizeof(QAccessibleValueInterface)=8
pub struct QAccessibleValueInterface {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAccessibleValueInterface_ITF interface {
//    QAccessibleValueInterface_PTR() *QAccessibleValueInterface
//}
//func (ptr *QAccessibleValueInterface) QAccessibleValueInterface_PTR() *QAccessibleValueInterface { return ptr }

impl /*struct*/ QAccessibleValueInterface {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAccessibleValueInterface {
    return QAccessibleValueInterface{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAccessibleValueInterface {
//  type Target = QAccessibleValueInterfaceBASE;
//
//  fn deref(&self) -> &QAccessibleValueInterfaceBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAccessibleValueInterfaceBASE> for QAccessibleValueInterface {
//  fn as_ref(& self) -> & QAccessibleValueInterfaceBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qaccessible.h:566
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QAccessibleValueInterface()

/*

*/
pub fn DeleteQAccessibleValueInterface(this :*mut QAccessibleValueInterface) {
    // let rv = qtrt::InvokeQtFunc6("_ZN25QAccessibleValueInterfaceD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qaccessible.h:568
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [16] QVariant currentValue() const

/*

*/
impl /*struct*/ QAccessibleValueInterface {
  pub fn currentValue_0<RetType, T: QAccessibleValueInterface_currentValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentValue_0(self);
    // return 1;
  }
}
pub trait QAccessibleValueInterface_currentValue_0<RetType> {
  fn currentValue_0(self , rsthis: & QAccessibleValueInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleValueInterface_currentValue_0<usize> for () {
  fn currentValue_0(self , rsthis: & QAccessibleValueInterface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK25QAccessibleValueInterface12currentValueEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:569
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [-2] void setCurrentValue(const QVariant &)

/*

*/
impl /*struct*/ QAccessibleValueInterface {
  pub fn setCurrentValue_0<RetType, T: QAccessibleValueInterface_setCurrentValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentValue_0(self);
    // return 1;
  }
}
pub trait QAccessibleValueInterface_setCurrentValue_0<RetType> {
  fn setCurrentValue_0(self , rsthis: & QAccessibleValueInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleValueInterface_setCurrentValue_0<(/*void*/)> for (usize) {
  fn setCurrentValue_0(self , rsthis: & QAccessibleValueInterface) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN25QAccessibleValueInterface15setCurrentValueERK8QVariant", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:570
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [16] QVariant maximumValue() const

/*

*/
impl /*struct*/ QAccessibleValueInterface {
  pub fn maximumValue_0<RetType, T: QAccessibleValueInterface_maximumValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maximumValue_0(self);
    // return 1;
  }
}
pub trait QAccessibleValueInterface_maximumValue_0<RetType> {
  fn maximumValue_0(self , rsthis: & QAccessibleValueInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleValueInterface_maximumValue_0<usize> for () {
  fn maximumValue_0(self , rsthis: & QAccessibleValueInterface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK25QAccessibleValueInterface12maximumValueEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:571
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [16] QVariant minimumValue() const

/*

*/
impl /*struct*/ QAccessibleValueInterface {
  pub fn minimumValue_0<RetType, T: QAccessibleValueInterface_minimumValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumValue_0(self);
    // return 1;
  }
}
pub trait QAccessibleValueInterface_minimumValue_0<RetType> {
  fn minimumValue_0(self , rsthis: & QAccessibleValueInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleValueInterface_minimumValue_0<usize> for () {
  fn minimumValue_0(self , rsthis: & QAccessibleValueInterface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK25QAccessibleValueInterface12minimumValueEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:572
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [16] QVariant minimumStepSize() const

/*

*/
impl /*struct*/ QAccessibleValueInterface {
  pub fn minimumStepSize_0<RetType, T: QAccessibleValueInterface_minimumStepSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumStepSize_0(self);
    // return 1;
  }
}
pub trait QAccessibleValueInterface_minimumStepSize_0<RetType> {
  fn minimumStepSize_0(self , rsthis: & QAccessibleValueInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleValueInterface_minimumStepSize_0<usize> for () {
  fn minimumStepSize_0(self , rsthis: & QAccessibleValueInterface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK25QAccessibleValueInterface15minimumStepSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
