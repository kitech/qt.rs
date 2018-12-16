

// mod ::gui::QAccessibleTableCellInterface
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
// extern C begin: 6
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
#[derive(Default)] // class sizeof(QAccessibleTableCellInterface)=8
pub struct QAccessibleTableCellInterface {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAccessibleTableCellInterface_ITF interface {
//    QAccessibleTableCellInterface_PTR() *QAccessibleTableCellInterface
//}
//func (ptr *QAccessibleTableCellInterface) QAccessibleTableCellInterface_PTR() *QAccessibleTableCellInterface { return ptr }

impl /*struct*/ QAccessibleTableCellInterface {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAccessibleTableCellInterface {
    return QAccessibleTableCellInterface{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAccessibleTableCellInterface {
//  type Target = QAccessibleTableCellInterfaceBASE;
//
//  fn deref(&self) -> &QAccessibleTableCellInterfaceBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAccessibleTableCellInterfaceBASE> for QAccessibleTableCellInterface {
//  fn as_ref(& self) -> & QAccessibleTableCellInterfaceBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qaccessible.h:578
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QAccessibleTableCellInterface()

/*

*/
pub fn DeleteQAccessibleTableCellInterface(this :*mut QAccessibleTableCellInterface) {
    // let rv = qtrt::InvokeQtFunc6("_ZN29QAccessibleTableCellInterfaceD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qaccessible.h:580
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [1] bool isSelected() const

/*

*/
impl /*struct*/ QAccessibleTableCellInterface {
  pub fn isSelected_0<RetType, T: QAccessibleTableCellInterface_isSelected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isSelected_0(self);
    // return 1;
  }
}
pub trait QAccessibleTableCellInterface_isSelected_0<RetType> {
  fn isSelected_0(self , rsthis: & QAccessibleTableCellInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTableCellInterface_isSelected_0<bool> for () {
  fn isSelected_0(self , rsthis: & QAccessibleTableCellInterface) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK29QAccessibleTableCellInterface10isSelectedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:584
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [4] int columnIndex() const

/*

*/
impl /*struct*/ QAccessibleTableCellInterface {
  pub fn columnIndex_0<RetType, T: QAccessibleTableCellInterface_columnIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnIndex_0(self);
    // return 1;
  }
}
pub trait QAccessibleTableCellInterface_columnIndex_0<RetType> {
  fn columnIndex_0(self , rsthis: & QAccessibleTableCellInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTableCellInterface_columnIndex_0<i32> for () {
  fn columnIndex_0(self , rsthis: & QAccessibleTableCellInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK29QAccessibleTableCellInterface11columnIndexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:585
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [4] int rowIndex() const

/*

*/
impl /*struct*/ QAccessibleTableCellInterface {
  pub fn rowIndex_0<RetType, T: QAccessibleTableCellInterface_rowIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowIndex_0(self);
    // return 1;
  }
}
pub trait QAccessibleTableCellInterface_rowIndex_0<RetType> {
  fn rowIndex_0(self , rsthis: & QAccessibleTableCellInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTableCellInterface_rowIndex_0<i32> for () {
  fn rowIndex_0(self , rsthis: & QAccessibleTableCellInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK29QAccessibleTableCellInterface8rowIndexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:586
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [4] int columnExtent() const

/*

*/
impl /*struct*/ QAccessibleTableCellInterface {
  pub fn columnExtent_0<RetType, T: QAccessibleTableCellInterface_columnExtent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnExtent_0(self);
    // return 1;
  }
}
pub trait QAccessibleTableCellInterface_columnExtent_0<RetType> {
  fn columnExtent_0(self , rsthis: & QAccessibleTableCellInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTableCellInterface_columnExtent_0<i32> for () {
  fn columnExtent_0(self , rsthis: & QAccessibleTableCellInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK29QAccessibleTableCellInterface12columnExtentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:587
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [4] int rowExtent() const

/*

*/
impl /*struct*/ QAccessibleTableCellInterface {
  pub fn rowExtent_0<RetType, T: QAccessibleTableCellInterface_rowExtent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowExtent_0(self);
    // return 1;
  }
}
pub trait QAccessibleTableCellInterface_rowExtent_0<RetType> {
  fn rowExtent_0(self , rsthis: & QAccessibleTableCellInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTableCellInterface_rowExtent_0<i32> for () {
  fn rowExtent_0(self , rsthis: & QAccessibleTableCellInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK29QAccessibleTableCellInterface9rowExtentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:589
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [8] QAccessibleInterface * table() const

/*

*/
impl /*struct*/ QAccessibleTableCellInterface {
  pub fn table_0<RetType, T: QAccessibleTableCellInterface_table_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.table_0(self);
    // return 1;
  }
}
pub trait QAccessibleTableCellInterface_table_0<RetType> {
  fn table_0(self , rsthis: & QAccessibleTableCellInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTableCellInterface_table_0<usize> for () {
  fn table_0(self , rsthis: & QAccessibleTableCellInterface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK29QAccessibleTableCellInterface5tableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
