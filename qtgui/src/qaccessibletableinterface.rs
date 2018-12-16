

// mod ::gui::QAccessibleTableInterface
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
// extern C begin: 7
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
#[derive(Default)] // class sizeof(QAccessibleTableInterface)=8
pub struct QAccessibleTableInterface {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAccessibleTableInterface_ITF interface {
//    QAccessibleTableInterface_PTR() *QAccessibleTableInterface
//}
//func (ptr *QAccessibleTableInterface) QAccessibleTableInterface_PTR() *QAccessibleTableInterface { return ptr }

impl /*struct*/ QAccessibleTableInterface {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAccessibleTableInterface {
    return QAccessibleTableInterface{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAccessibleTableInterface {
//  type Target = QAccessibleTableInterfaceBASE;
//
//  fn deref(&self) -> &QAccessibleTableInterfaceBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAccessibleTableInterfaceBASE> for QAccessibleTableInterface {
//  fn as_ref(& self) -> & QAccessibleTableInterfaceBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qaccessible.h:595
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QAccessibleTableInterface()

/*

*/
pub fn DeleteQAccessibleTableInterface(this :*mut QAccessibleTableInterface) {
    // let rv = qtrt::InvokeQtFunc6("_ZN25QAccessibleTableInterfaceD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qaccessible.h:597
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [8] QAccessibleInterface * caption() const

/*

*/
impl /*struct*/ QAccessibleTableInterface {
  pub fn caption_0<RetType, T: QAccessibleTableInterface_caption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.caption_0(self);
    // return 1;
  }
}
pub trait QAccessibleTableInterface_caption_0<RetType> {
  fn caption_0(self , rsthis: & QAccessibleTableInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTableInterface_caption_0<usize> for () {
  fn caption_0(self , rsthis: & QAccessibleTableInterface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK25QAccessibleTableInterface7captionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:598
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [8] QAccessibleInterface * summary() const

/*

*/
impl /*struct*/ QAccessibleTableInterface {
  pub fn summary_0<RetType, T: QAccessibleTableInterface_summary_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.summary_0(self);
    // return 1;
  }
}
pub trait QAccessibleTableInterface_summary_0<RetType> {
  fn summary_0(self , rsthis: & QAccessibleTableInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTableInterface_summary_0<usize> for () {
  fn summary_0(self , rsthis: & QAccessibleTableInterface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK25QAccessibleTableInterface7summaryEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:600
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [8] QAccessibleInterface * cellAt(int, int) const

/*

*/
impl /*struct*/ QAccessibleTableInterface {
  pub fn cellAt_0<RetType, T: QAccessibleTableInterface_cellAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cellAt_0(self);
    // return 1;
  }
}
pub trait QAccessibleTableInterface_cellAt_0<RetType> {
  fn cellAt_0(self , rsthis: & QAccessibleTableInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTableInterface_cellAt_0<usize> for (i32,i32) {
  fn cellAt_0(self , rsthis: & QAccessibleTableInterface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK25QAccessibleTableInterface6cellAtEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:601
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [4] int selectedCellCount() const

/*

*/
impl /*struct*/ QAccessibleTableInterface {
  pub fn selectedCellCount_0<RetType, T: QAccessibleTableInterface_selectedCellCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectedCellCount_0(self);
    // return 1;
  }
}
pub trait QAccessibleTableInterface_selectedCellCount_0<RetType> {
  fn selectedCellCount_0(self , rsthis: & QAccessibleTableInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTableInterface_selectedCellCount_0<i32> for () {
  fn selectedCellCount_0(self , rsthis: & QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK25QAccessibleTableInterface17selectedCellCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:604
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [8] QString columnDescription(int) const

/*

*/
impl /*struct*/ QAccessibleTableInterface {
  pub fn columnDescription_0<RetType, T: QAccessibleTableInterface_columnDescription_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnDescription_0(self);
    // return 1;
  }
}
pub trait QAccessibleTableInterface_columnDescription_0<RetType> {
  fn columnDescription_0(self , rsthis: & QAccessibleTableInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTableInterface_columnDescription_0<usize> for (i32) {
  fn columnDescription_0(self , rsthis: & QAccessibleTableInterface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK25QAccessibleTableInterface17columnDescriptionEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:605
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [8] QString rowDescription(int) const

/*

*/
impl /*struct*/ QAccessibleTableInterface {
  pub fn rowDescription_0<RetType, T: QAccessibleTableInterface_rowDescription_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowDescription_0(self);
    // return 1;
  }
}
pub trait QAccessibleTableInterface_rowDescription_0<RetType> {
  fn rowDescription_0(self , rsthis: & QAccessibleTableInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTableInterface_rowDescription_0<usize> for (i32) {
  fn rowDescription_0(self , rsthis: & QAccessibleTableInterface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK25QAccessibleTableInterface14rowDescriptionEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:606
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [4] int selectedColumnCount() const

/*

*/
impl /*struct*/ QAccessibleTableInterface {
  pub fn selectedColumnCount_0<RetType, T: QAccessibleTableInterface_selectedColumnCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectedColumnCount_0(self);
    // return 1;
  }
}
pub trait QAccessibleTableInterface_selectedColumnCount_0<RetType> {
  fn selectedColumnCount_0(self , rsthis: & QAccessibleTableInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTableInterface_selectedColumnCount_0<i32> for () {
  fn selectedColumnCount_0(self , rsthis: & QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK25QAccessibleTableInterface19selectedColumnCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:607
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [4] int selectedRowCount() const

/*

*/
impl /*struct*/ QAccessibleTableInterface {
  pub fn selectedRowCount_0<RetType, T: QAccessibleTableInterface_selectedRowCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectedRowCount_0(self);
    // return 1;
  }
}
pub trait QAccessibleTableInterface_selectedRowCount_0<RetType> {
  fn selectedRowCount_0(self , rsthis: & QAccessibleTableInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTableInterface_selectedRowCount_0<i32> for () {
  fn selectedRowCount_0(self , rsthis: & QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK25QAccessibleTableInterface16selectedRowCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:608
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [4] int columnCount() const

/*

*/
impl /*struct*/ QAccessibleTableInterface {
  pub fn columnCount_0<RetType, T: QAccessibleTableInterface_columnCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnCount_0(self);
    // return 1;
  }
}
pub trait QAccessibleTableInterface_columnCount_0<RetType> {
  fn columnCount_0(self , rsthis: & QAccessibleTableInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTableInterface_columnCount_0<i32> for () {
  fn columnCount_0(self , rsthis: & QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK25QAccessibleTableInterface11columnCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:609
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [4] int rowCount() const

/*

*/
impl /*struct*/ QAccessibleTableInterface {
  pub fn rowCount_0<RetType, T: QAccessibleTableInterface_rowCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowCount_0(self);
    // return 1;
  }
}
pub trait QAccessibleTableInterface_rowCount_0<RetType> {
  fn rowCount_0(self , rsthis: & QAccessibleTableInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTableInterface_rowCount_0<i32> for () {
  fn rowCount_0(self , rsthis: & QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK25QAccessibleTableInterface8rowCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:612
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [1] bool isColumnSelected(int) const

/*

*/
impl /*struct*/ QAccessibleTableInterface {
  pub fn isColumnSelected_0<RetType, T: QAccessibleTableInterface_isColumnSelected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isColumnSelected_0(self);
    // return 1;
  }
}
pub trait QAccessibleTableInterface_isColumnSelected_0<RetType> {
  fn isColumnSelected_0(self , rsthis: & QAccessibleTableInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTableInterface_isColumnSelected_0<bool> for (i32) {
  fn isColumnSelected_0(self , rsthis: & QAccessibleTableInterface) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK25QAccessibleTableInterface16isColumnSelectedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:613
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [1] bool isRowSelected(int) const

/*

*/
impl /*struct*/ QAccessibleTableInterface {
  pub fn isRowSelected_0<RetType, T: QAccessibleTableInterface_isRowSelected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isRowSelected_0(self);
    // return 1;
  }
}
pub trait QAccessibleTableInterface_isRowSelected_0<RetType> {
  fn isRowSelected_0(self , rsthis: & QAccessibleTableInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTableInterface_isRowSelected_0<bool> for (i32) {
  fn isRowSelected_0(self , rsthis: & QAccessibleTableInterface) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK25QAccessibleTableInterface13isRowSelectedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:614
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [1] bool selectRow(int)

/*

*/
impl /*struct*/ QAccessibleTableInterface {
  pub fn selectRow_0<RetType, T: QAccessibleTableInterface_selectRow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectRow_0(self);
    // return 1;
  }
}
pub trait QAccessibleTableInterface_selectRow_0<RetType> {
  fn selectRow_0(self , rsthis: & QAccessibleTableInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTableInterface_selectRow_0<bool> for (i32) {
  fn selectRow_0(self , rsthis: & QAccessibleTableInterface) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN25QAccessibleTableInterface9selectRowEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:615
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [1] bool selectColumn(int)

/*

*/
impl /*struct*/ QAccessibleTableInterface {
  pub fn selectColumn_0<RetType, T: QAccessibleTableInterface_selectColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectColumn_0(self);
    // return 1;
  }
}
pub trait QAccessibleTableInterface_selectColumn_0<RetType> {
  fn selectColumn_0(self , rsthis: & QAccessibleTableInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTableInterface_selectColumn_0<bool> for (i32) {
  fn selectColumn_0(self , rsthis: & QAccessibleTableInterface) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN25QAccessibleTableInterface12selectColumnEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:616
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [1] bool unselectRow(int)

/*

*/
impl /*struct*/ QAccessibleTableInterface {
  pub fn unselectRow_0<RetType, T: QAccessibleTableInterface_unselectRow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unselectRow_0(self);
    // return 1;
  }
}
pub trait QAccessibleTableInterface_unselectRow_0<RetType> {
  fn unselectRow_0(self , rsthis: & QAccessibleTableInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTableInterface_unselectRow_0<bool> for (i32) {
  fn unselectRow_0(self , rsthis: & QAccessibleTableInterface) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN25QAccessibleTableInterface11unselectRowEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:617
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [1] bool unselectColumn(int)

/*

*/
impl /*struct*/ QAccessibleTableInterface {
  pub fn unselectColumn_0<RetType, T: QAccessibleTableInterface_unselectColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unselectColumn_0(self);
    // return 1;
  }
}
pub trait QAccessibleTableInterface_unselectColumn_0<RetType> {
  fn unselectColumn_0(self , rsthis: & QAccessibleTableInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTableInterface_unselectColumn_0<bool> for (i32) {
  fn unselectColumn_0(self , rsthis: & QAccessibleTableInterface) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN25QAccessibleTableInterface14unselectColumnEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:619
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [-2] void modelChange(QAccessibleTableModelChangeEvent *)

/*

*/
impl /*struct*/ QAccessibleTableInterface {
  pub fn modelChange_0<RetType, T: QAccessibleTableInterface_modelChange_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.modelChange_0(self);
    // return 1;
  }
}
pub trait QAccessibleTableInterface_modelChange_0<RetType> {
  fn modelChange_0(self , rsthis: & QAccessibleTableInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTableInterface_modelChange_0<(/*void*/)> for (usize) {
  fn modelChange_0(self , rsthis: & QAccessibleTableInterface) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN25QAccessibleTableInterface11modelChangeEP32QAccessibleTableModelChangeEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
