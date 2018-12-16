

// mod ::core::QMapNodeBase
// package qtcore
// /usr/include/qt/QtCore/qmap.h
// #include <qmap.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 23
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QMapNodeBase)=24
pub struct QMapNodeBase {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QMapNodeBase_ITF interface {
//    QMapNodeBase_PTR() *QMapNodeBase
//}
//func (ptr *QMapNodeBase) QMapNodeBase_PTR() *QMapNodeBase { return ptr }

impl /*struct*/ QMapNodeBase {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QMapNodeBase {
    return QMapNodeBase{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QMapNodeBase {
//  type Target = QMapNodeBaseBASE;
//
//  fn deref(&self) -> &QMapNodeBaseBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QMapNodeBaseBASE> for QMapNodeBase {
//  fn as_ref(& self) -> & QMapNodeBaseBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qmap.h:91
// index:0
// Public Visibility=Default Availability=Available
// [8] const QMapNodeBase * nextNode() const

/*

*/
impl /*struct*/ QMapNodeBase {
  pub fn nextNode_0<RetType, T: QMapNodeBase_nextNode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.nextNode_0(self);
    // return 1;
  }
}
pub trait QMapNodeBase_nextNode_0<RetType> {
  fn nextNode_0(self , rsthis: & QMapNodeBase) -> RetType;
}
impl<'a> /*trait*/ QMapNodeBase_nextNode_0<usize> for () {
  fn nextNode_0(self , rsthis: & QMapNodeBase) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QMapNodeBase8nextNodeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmap.h:92
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QMapNodeBase * nextNode()

/*

*/
impl /*struct*/ QMapNodeBase {
  pub fn nextNode_1<RetType, T: QMapNodeBase_nextNode_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.nextNode_1(self);
    // return 1;
  }
}
pub trait QMapNodeBase_nextNode_1<RetType> {
  fn nextNode_1(self , rsthis: & QMapNodeBase) -> RetType;
}
impl<'a> /*trait*/ QMapNodeBase_nextNode_1<usize> for () {
  fn nextNode_1(self , rsthis: & QMapNodeBase) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QMapNodeBase8nextNodeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmap.h:93
// index:0
// Public Visibility=Default Availability=Available
// [8] const QMapNodeBase * previousNode() const

/*

*/
impl /*struct*/ QMapNodeBase {
  pub fn previousNode_0<RetType, T: QMapNodeBase_previousNode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.previousNode_0(self);
    // return 1;
  }
}
pub trait QMapNodeBase_previousNode_0<RetType> {
  fn previousNode_0(self , rsthis: & QMapNodeBase) -> RetType;
}
impl<'a> /*trait*/ QMapNodeBase_previousNode_0<usize> for () {
  fn previousNode_0(self , rsthis: & QMapNodeBase) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QMapNodeBase12previousNodeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmap.h:94
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QMapNodeBase * previousNode()

/*

*/
impl /*struct*/ QMapNodeBase {
  pub fn previousNode_1<RetType, T: QMapNodeBase_previousNode_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.previousNode_1(self);
    // return 1;
  }
}
pub trait QMapNodeBase_previousNode_1<RetType> {
  fn previousNode_1(self , rsthis: & QMapNodeBase) -> RetType;
}
impl<'a> /*trait*/ QMapNodeBase_previousNode_1<usize> for () {
  fn previousNode_1(self , rsthis: & QMapNodeBase) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QMapNodeBase12previousNodeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmap.h:96
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QMapNodeBase::Color color() const

/*

*/
impl /*struct*/ QMapNodeBase {
  pub fn color_0<RetType, T: QMapNodeBase_color_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.color_0(self);
    // return 1;
  }
}
pub trait QMapNodeBase_color_0<RetType> {
  fn color_0(self , rsthis: & QMapNodeBase) -> RetType;
}
impl<'a> /*trait*/ QMapNodeBase_color_0<i32> for () {
  fn color_0(self , rsthis: & QMapNodeBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QMapNodeBase5colorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmap.h:97
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setColor(QMapNodeBase::Color)

/*

*/
impl /*struct*/ QMapNodeBase {
  pub fn setColor_0<RetType, T: QMapNodeBase_setColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setColor_0(self);
    // return 1;
  }
}
pub trait QMapNodeBase_setColor_0<RetType> {
  fn setColor_0(self , rsthis: & QMapNodeBase) -> RetType;
}
impl<'a> /*trait*/ QMapNodeBase_setColor_0<(/*void*/)> for (i32) {
  fn setColor_0(self , rsthis: & QMapNodeBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QMapNodeBase8setColorENS_5ColorE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmap.h:98
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QMapNodeBase * parent() const

/*

*/
impl /*struct*/ QMapNodeBase {
  pub fn parent_0<RetType, T: QMapNodeBase_parent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.parent_0(self);
    // return 1;
  }
}
pub trait QMapNodeBase_parent_0<RetType> {
  fn parent_0(self , rsthis: & QMapNodeBase) -> RetType;
}
impl<'a> /*trait*/ QMapNodeBase_parent_0<usize> for () {
  fn parent_0(self , rsthis: & QMapNodeBase) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QMapNodeBase6parentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQMapNodeBase(this :*mut QMapNodeBase) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN12QMapNodeBaseD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*


*/
pub type QMapNodeBase__Color = i32;
// 
pub const QMapNodeBase__Red :QMapNodeBase__Color = 0;
// 
pub const QMapNodeBase__Black :QMapNodeBase__Color = 1;
pub fn QMapNodeBase_ColorItemName(val: i32) ->String {
  match val {
     QMapNodeBase__Red => // 0
     {return String::from("Red");}
     QMapNodeBase__Black => // 1
     {return String::from("Black");}
  _ => {return format!("{}", val);}
}
}
pub fn QMapNodeBase_ColorItemName_s(val: i32) ->String {
  //var nilthis *QMapNodeBase
  //return nilthis.ColorItemName(val);
  return QMapNodeBase_ColorItemName(val);
}


/*


*/
pub type QMapNodeBase__ = i32;
// 
pub const QMapNodeBase__Mask :QMapNodeBase__ = 3;
pub fn QMapNodeBase_ItemName(val: i32) ->String {
  match val {
     QMapNodeBase__Mask => // 3
     {return String::from("Mask");}
  _ => {return format!("{}", val);}
}
}
pub fn QMapNodeBase_ItemName_s(val: i32) ->String {
  //var nilthis *QMapNodeBase
  //return nilthis.ItemName(val);
  return QMapNodeBase_ItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
