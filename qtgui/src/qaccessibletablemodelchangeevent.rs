

// mod ::gui::QAccessibleTableModelChangeEvent
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
// extern C begin: 5
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
#[derive(Default)] // class sizeof(QAccessibleTableModelChangeEvent)=48
pub struct QAccessibleTableModelChangeEvent {
  qbase: QAccessibleEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAccessibleTableModelChangeEvent_ITF interface {
//    QAccessibleEvent_ITF
//    QAccessibleTableModelChangeEvent_PTR() *QAccessibleTableModelChangeEvent
//}
//func (ptr *QAccessibleTableModelChangeEvent) QAccessibleTableModelChangeEvent_PTR() *QAccessibleTableModelChangeEvent { return ptr }

impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAccessibleTableModelChangeEvent {
    return QAccessibleTableModelChangeEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAccessibleTableModelChangeEvent {
//  type Target = QAccessibleTableModelChangeEventBASE;
//
//  fn deref(&self) -> &QAccessibleTableModelChangeEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAccessibleTableModelChangeEventBASE> for QAccessibleTableModelChangeEvent {
//  fn as_ref(& self) -> & QAccessibleTableModelChangeEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qaccessible.h:932
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QAccessibleTableModelChangeEvent(QObject *, QAccessibleTableModelChangeEvent::ModelChangeType)

/*

*/
// QAccessibleTableModelChangeEvent(QObject *, QAccessibleTableModelChangeEvent::ModelChangeType) ctx.fn_proto_cpp
impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn QAccessibleTableModelChangeEvent_0<T: QAccessibleTableModelChangeEvent_QAccessibleTableModelChangeEvent_0>(value: T) -> QAccessibleTableModelChangeEvent {
    let rsthis = value.QAccessibleTableModelChangeEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleTableModelChangeEvent_QAccessibleTableModelChangeEvent_0 {
  fn QAccessibleTableModelChangeEvent_0(self) -> QAccessibleTableModelChangeEvent;
}
// QAccessibleTableModelChangeEvent(QObject *, QAccessibleTableModelChangeEvent::ModelChangeType) ctx.fn_proto_cpp
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_QAccessibleTableModelChangeEvent_0 for (usize,i32) {
  fn QAccessibleTableModelChangeEvent_0(self) -> QAccessibleTableModelChangeEvent {
    // unsafe{_ZN32QAccessibleTableModelChangeEventC2EP7QObjectNS_15ModelChangeTypeE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN32QAccessibleTableModelChangeEventC2EP7QObjectNS_15ModelChangeTypeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAccessibleTableModelChangeEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:939
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QAccessibleTableModelChangeEvent(QAccessibleInterface *, QAccessibleTableModelChangeEvent::ModelChangeType)

/*

*/
// QAccessibleTableModelChangeEvent(QAccessibleInterface *, QAccessibleTableModelChangeEvent::ModelChangeType) ctx.fn_proto_cpp
impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn QAccessibleTableModelChangeEvent_1<T: QAccessibleTableModelChangeEvent_QAccessibleTableModelChangeEvent_1>(value: T) -> QAccessibleTableModelChangeEvent {
    let rsthis = value.QAccessibleTableModelChangeEvent_1();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleTableModelChangeEvent_QAccessibleTableModelChangeEvent_1 {
  fn QAccessibleTableModelChangeEvent_1(self) -> QAccessibleTableModelChangeEvent;
}
// QAccessibleTableModelChangeEvent(QAccessibleInterface *, QAccessibleTableModelChangeEvent::ModelChangeType) ctx.fn_proto_cpp
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_QAccessibleTableModelChangeEvent_1 for (usize,i32) {
  fn QAccessibleTableModelChangeEvent_1(self) -> QAccessibleTableModelChangeEvent {
    // unsafe{_ZN32QAccessibleTableModelChangeEventC2EP20QAccessibleInterfaceNS_15ModelChangeTypeE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN32QAccessibleTableModelChangeEventC2EP20QAccessibleInterfaceNS_15ModelChangeTypeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAccessibleTableModelChangeEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:947
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QAccessibleTableModelChangeEvent()

/*

*/
pub fn DeleteQAccessibleTableModelChangeEvent(this :*mut QAccessibleTableModelChangeEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN32QAccessibleTableModelChangeEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qaccessible.h:949
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setModelChangeType(QAccessibleTableModelChangeEvent::ModelChangeType)

/*

*/
impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn setModelChangeType_0<RetType, T: QAccessibleTableModelChangeEvent_setModelChangeType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setModelChangeType_0(self);
    // return 1;
  }
}
pub trait QAccessibleTableModelChangeEvent_setModelChangeType_0<RetType> {
  fn setModelChangeType_0(self , rsthis: & QAccessibleTableModelChangeEvent) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_setModelChangeType_0<(/*void*/)> for (i32) {
  fn setModelChangeType_0(self , rsthis: & QAccessibleTableModelChangeEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN32QAccessibleTableModelChangeEvent18setModelChangeTypeENS_15ModelChangeTypeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:950
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QAccessibleTableModelChangeEvent::ModelChangeType modelChangeType() const

/*

*/
impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn modelChangeType_0<RetType, T: QAccessibleTableModelChangeEvent_modelChangeType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.modelChangeType_0(self);
    // return 1;
  }
}
pub trait QAccessibleTableModelChangeEvent_modelChangeType_0<RetType> {
  fn modelChangeType_0(self , rsthis: & QAccessibleTableModelChangeEvent) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_modelChangeType_0<i32> for () {
  fn modelChangeType_0(self , rsthis: & QAccessibleTableModelChangeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK32QAccessibleTableModelChangeEvent15modelChangeTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:952
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setFirstRow(int)

/*

*/
impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn setFirstRow_0<RetType, T: QAccessibleTableModelChangeEvent_setFirstRow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFirstRow_0(self);
    // return 1;
  }
}
pub trait QAccessibleTableModelChangeEvent_setFirstRow_0<RetType> {
  fn setFirstRow_0(self , rsthis: & QAccessibleTableModelChangeEvent) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_setFirstRow_0<(/*void*/)> for (i32) {
  fn setFirstRow_0(self , rsthis: & QAccessibleTableModelChangeEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN32QAccessibleTableModelChangeEvent11setFirstRowEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:953
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setFirstColumn(int)

/*

*/
impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn setFirstColumn_0<RetType, T: QAccessibleTableModelChangeEvent_setFirstColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFirstColumn_0(self);
    // return 1;
  }
}
pub trait QAccessibleTableModelChangeEvent_setFirstColumn_0<RetType> {
  fn setFirstColumn_0(self , rsthis: & QAccessibleTableModelChangeEvent) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_setFirstColumn_0<(/*void*/)> for (i32) {
  fn setFirstColumn_0(self , rsthis: & QAccessibleTableModelChangeEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN32QAccessibleTableModelChangeEvent14setFirstColumnEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:954
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setLastRow(int)

/*

*/
impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn setLastRow_0<RetType, T: QAccessibleTableModelChangeEvent_setLastRow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLastRow_0(self);
    // return 1;
  }
}
pub trait QAccessibleTableModelChangeEvent_setLastRow_0<RetType> {
  fn setLastRow_0(self , rsthis: & QAccessibleTableModelChangeEvent) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_setLastRow_0<(/*void*/)> for (i32) {
  fn setLastRow_0(self , rsthis: & QAccessibleTableModelChangeEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN32QAccessibleTableModelChangeEvent10setLastRowEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:955
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setLastColumn(int)

/*

*/
impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn setLastColumn_0<RetType, T: QAccessibleTableModelChangeEvent_setLastColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLastColumn_0(self);
    // return 1;
  }
}
pub trait QAccessibleTableModelChangeEvent_setLastColumn_0<RetType> {
  fn setLastColumn_0(self , rsthis: & QAccessibleTableModelChangeEvent) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_setLastColumn_0<(/*void*/)> for (i32) {
  fn setLastColumn_0(self , rsthis: & QAccessibleTableModelChangeEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN32QAccessibleTableModelChangeEvent13setLastColumnEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:956
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int firstRow() const

/*

*/
impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn firstRow_0<RetType, T: QAccessibleTableModelChangeEvent_firstRow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.firstRow_0(self);
    // return 1;
  }
}
pub trait QAccessibleTableModelChangeEvent_firstRow_0<RetType> {
  fn firstRow_0(self , rsthis: & QAccessibleTableModelChangeEvent) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_firstRow_0<i32> for () {
  fn firstRow_0(self , rsthis: & QAccessibleTableModelChangeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK32QAccessibleTableModelChangeEvent8firstRowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:957
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int firstColumn() const

/*

*/
impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn firstColumn_0<RetType, T: QAccessibleTableModelChangeEvent_firstColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.firstColumn_0(self);
    // return 1;
  }
}
pub trait QAccessibleTableModelChangeEvent_firstColumn_0<RetType> {
  fn firstColumn_0(self , rsthis: & QAccessibleTableModelChangeEvent) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_firstColumn_0<i32> for () {
  fn firstColumn_0(self , rsthis: & QAccessibleTableModelChangeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK32QAccessibleTableModelChangeEvent11firstColumnEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:958
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int lastRow() const

/*

*/
impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn lastRow_0<RetType, T: QAccessibleTableModelChangeEvent_lastRow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastRow_0(self);
    // return 1;
  }
}
pub trait QAccessibleTableModelChangeEvent_lastRow_0<RetType> {
  fn lastRow_0(self , rsthis: & QAccessibleTableModelChangeEvent) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_lastRow_0<i32> for () {
  fn lastRow_0(self , rsthis: & QAccessibleTableModelChangeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK32QAccessibleTableModelChangeEvent7lastRowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:959
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int lastColumn() const

/*

*/
impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn lastColumn_0<RetType, T: QAccessibleTableModelChangeEvent_lastColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastColumn_0(self);
    // return 1;
  }
}
pub trait QAccessibleTableModelChangeEvent_lastColumn_0<RetType> {
  fn lastColumn_0(self , rsthis: & QAccessibleTableModelChangeEvent) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_lastColumn_0<i32> for () {
  fn lastColumn_0(self , rsthis: & QAccessibleTableModelChangeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK32QAccessibleTableModelChangeEvent10lastColumnEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}


/*


*/
pub type QAccessibleTableModelChangeEvent__ModelChangeType = i32;
// 
pub const QAccessibleTableModelChangeEvent__ModelReset :QAccessibleTableModelChangeEvent__ModelChangeType = 0;
// 
pub const QAccessibleTableModelChangeEvent__DataChanged :QAccessibleTableModelChangeEvent__ModelChangeType = 1;
// 
pub const QAccessibleTableModelChangeEvent__RowsInserted :QAccessibleTableModelChangeEvent__ModelChangeType = 2;
// 
pub const QAccessibleTableModelChangeEvent__ColumnsInserted :QAccessibleTableModelChangeEvent__ModelChangeType = 3;
// 
pub const QAccessibleTableModelChangeEvent__RowsRemoved :QAccessibleTableModelChangeEvent__ModelChangeType = 4;
// 
pub const QAccessibleTableModelChangeEvent__ColumnsRemoved :QAccessibleTableModelChangeEvent__ModelChangeType = 5;
pub fn QAccessibleTableModelChangeEvent_ModelChangeTypeItemName(val: i32) ->String {
  match val {
     QAccessibleTableModelChangeEvent__ModelReset => // 0
     {return String::from("ModelReset");}
     QAccessibleTableModelChangeEvent__DataChanged => // 1
     {return String::from("DataChanged");}
     QAccessibleTableModelChangeEvent__RowsInserted => // 2
     {return String::from("RowsInserted");}
     QAccessibleTableModelChangeEvent__ColumnsInserted => // 3
     {return String::from("ColumnsInserted");}
     QAccessibleTableModelChangeEvent__RowsRemoved => // 4
     {return String::from("RowsRemoved");}
     QAccessibleTableModelChangeEvent__ColumnsRemoved => // 5
     {return String::from("ColumnsRemoved");}
  _ => {return format!("{}", val);}
}
}
pub fn QAccessibleTableModelChangeEvent_ModelChangeTypeItemName_s(val: i32) ->String {
  //var nilthis *QAccessibleTableModelChangeEvent
  //return nilthis.ModelChangeTypeItemName(val);
  return QAccessibleTableModelChangeEvent_ModelChangeTypeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
