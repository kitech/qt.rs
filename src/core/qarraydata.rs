

// mod ::core::QArrayData
// package qtcore
// /usr/include/qt/QtCore/qarraydata.h
// #include <qarraydata.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 89
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
use std::default::Default;
use std::ops::Deref;
use super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QArrayData)=24
pub struct QArrayData {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QArrayData_ITF interface {
//    QArrayData_PTR() *QArrayData
//}
//func (ptr *QArrayData) QArrayData_PTR() *QArrayData { return ptr }

impl /*struct*/ QArrayData {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QArrayData {
    return QArrayData{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QArrayData {
//  type Target = QArrayDataBASE;
//
//  fn deref(&self) -> &QArrayDataBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QArrayDataBASE> for QArrayData {
//  fn as_ref(& self) -> & QArrayDataBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qarraydata.h:57
// index:0
// Public inline Visibility=Default Availability=Available
// [8] void * data()

/*

*/
impl /*struct*/ QArrayData {
  pub fn data_0<RetType, T: QArrayData_data_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.data_0(self);
    // return 1;
  }
}
pub trait QArrayData_data_0<RetType> {
  fn data_0(self , rsthis: & QArrayData) -> RetType;
}
impl<'a> /*trait*/ QArrayData_data_0<usize> for () {
  fn data_0(self , rsthis: & QArrayData) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QArrayData4dataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qarraydata.h:64
// index:1
// Public inline Visibility=Default Availability=Available
// [8] const void * data() const

/*

*/
impl /*struct*/ QArrayData {
  pub fn data_1<RetType, T: QArrayData_data_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.data_1(self);
    // return 1;
  }
}
pub trait QArrayData_data_1<RetType> {
  fn data_1(self , rsthis: & QArrayData) -> RetType;
}
impl<'a> /*trait*/ QArrayData_data_1<usize> for () {
  fn data_1(self , rsthis: & QArrayData) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QArrayData4dataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qarraydata.h:74
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isMutable() const

/*

*/
impl /*struct*/ QArrayData {
  pub fn isMutable_0<RetType, T: QArrayData_isMutable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isMutable_0(self);
    // return 1;
  }
}
pub trait QArrayData_isMutable_0<RetType> {
  fn isMutable_0(self , rsthis: & QArrayData) -> RetType;
}
impl<'a> /*trait*/ QArrayData_isMutable_0<bool> for () {
  fn isMutable_0(self , rsthis: & QArrayData) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QArrayData9isMutableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qarraydata.h:92
// index:0
// Public inline Visibility=Default Availability=Available
// [8] size_t detachCapacity(size_t) const

/*

*/
impl /*struct*/ QArrayData {
  pub fn detachCapacity_0<RetType, T: QArrayData_detachCapacity_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.detachCapacity_0(self);
    // return 1;
  }
}
pub trait QArrayData_detachCapacity_0<RetType> {
  fn detachCapacity_0(self , rsthis: & QArrayData) -> RetType;
}
impl<'a> /*trait*/ QArrayData_detachCapacity_0<u64> for (u64) {
  fn detachCapacity_0(self , rsthis: & QArrayData) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QArrayData14detachCapacityEm", 1,qtrt::FFITY_UINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qarraydata.h:99
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QArrayData::AllocationOptions detachFlags() const

/*

*/
impl /*struct*/ QArrayData {
  pub fn detachFlags_0<RetType, T: QArrayData_detachFlags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.detachFlags_0(self);
    // return 1;
  }
}
pub trait QArrayData_detachFlags_0<RetType> {
  fn detachFlags_0(self , rsthis: & QArrayData) -> RetType;
}
impl<'a> /*trait*/ QArrayData_detachFlags_0<i32> for () {
  fn detachFlags_0(self , rsthis: & QArrayData) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QArrayData11detachFlagsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qarraydata.h:107
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QArrayData::AllocationOptions cloneFlags() const

/*

*/
impl /*struct*/ QArrayData {
  pub fn cloneFlags_0<RetType, T: QArrayData_cloneFlags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cloneFlags_0(self);
    // return 1;
  }
}
pub trait QArrayData_cloneFlags_0<RetType> {
  fn cloneFlags_0(self , rsthis: & QArrayData) -> RetType;
}
impl<'a> /*trait*/ QArrayData_cloneFlags_0<i32> for () {
  fn cloneFlags_0(self , rsthis: & QArrayData) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QArrayData10cloneFlagsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qarraydata.h:115
// index:0
// Public static Visibility=Default Availability=Available
// [8] QArrayData * allocate(size_t, size_t, size_t, QArrayData::AllocationOptions)

/*

*/
impl /*struct*/ QArrayData {
  pub fn allocate_0<RetType, T: QArrayData_allocate_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.allocate_0();
    // return 1;
  }
}
pub trait QArrayData_allocate_0<RetType> {
  fn allocate_0(self ) -> RetType;
}
impl<'a> /*trait*/ QArrayData_allocate_0<usize> for (u64,u64,u64,i32) {
  fn allocate_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const u64 as usize;
    let arg1 = (&self.1) as *const u64 as usize;
    let arg2 = (&self.2) as *const u64 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QArrayData8allocateEmmm6QFlagsINS_16AllocationOptionEE", 4,qtrt::FFITY_UINT64,qtrt::FFITY_UINT64,qtrt::FFITY_UINT64,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qarraydata.h:123
// index:0
// Public static inline Visibility=Default Availability=Available
// [8] QArrayData * sharedNull()

/*

*/
impl /*struct*/ QArrayData {
  pub fn sharedNull_0<RetType, T: QArrayData_sharedNull_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.sharedNull_0();
    // return 1;
  }
}
pub trait QArrayData_sharedNull_0<RetType> {
  fn sharedNull_0(self ) -> RetType;
}
impl<'a> /*trait*/ QArrayData_sharedNull_0<usize> for () {
  fn sharedNull_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QArrayData10sharedNullEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQArrayData(this :*mut QArrayData) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN10QArrayDataD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*


*/
pub type QArrayData__AllocationOption = i32;
// 
pub const QArrayData__CapacityReserved :QArrayData__AllocationOption = 1;
// 
pub const QArrayData__Unsharable :QArrayData__AllocationOption = 2;
// 
pub const QArrayData__RawData :QArrayData__AllocationOption = 4;
// 
pub const QArrayData__Grow :QArrayData__AllocationOption = 8;
// 
pub const QArrayData__Default :QArrayData__AllocationOption = 0;
pub fn QArrayData_AllocationOptionItemName(val: i32) ->String {
  match val {
     QArrayData__CapacityReserved => // 1
     {return String::from("CapacityReserved");}
     QArrayData__Unsharable => // 2
     {return String::from("Unsharable");}
     QArrayData__RawData => // 4
     {return String::from("RawData");}
     QArrayData__Grow => // 8
     {return String::from("Grow");}
     QArrayData__Default => // 0
     {return String::from("Default");}
  _ => {return format!("{}", val);}
}
}
pub fn QArrayData_AllocationOptionItemName_s(val: i32) ->String {
  //var nilthis *QArrayData
  //return nilthis.AllocationOptionItemName(val);
  return QArrayData_AllocationOptionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
