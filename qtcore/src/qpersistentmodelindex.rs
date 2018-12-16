

// mod ::core::QPersistentModelIndex
// package qtcore
// /usr/include/qt/QtCore/qabstractitemmodel.h
// #include <qabstractitemmodel.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 15
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
#[derive(Default)] // class sizeof(QPersistentModelIndex)=8
pub struct QPersistentModelIndex {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QPersistentModelIndex_ITF interface {
//    QPersistentModelIndex_PTR() *QPersistentModelIndex
//}
//func (ptr *QPersistentModelIndex) QPersistentModelIndex_PTR() *QPersistentModelIndex { return ptr }

impl /*struct*/ QPersistentModelIndex {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QPersistentModelIndex {
    return QPersistentModelIndex{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QPersistentModelIndex {
//  type Target = QPersistentModelIndexBASE;
//
//  fn deref(&self) -> &QPersistentModelIndexBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QPersistentModelIndexBASE> for QPersistentModelIndex {
//  fn as_ref(& self) -> & QPersistentModelIndexBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qabstractitemmodel.h:107
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QPersistentModelIndex()

/*

*/
// QPersistentModelIndex() ctx.fn_proto_cpp
impl /*struct*/ QPersistentModelIndex {
  pub fn QPersistentModelIndex_0<T: QPersistentModelIndex_QPersistentModelIndex_0>(value: T) -> QPersistentModelIndex {
    let rsthis = value.QPersistentModelIndex_0();
    return rsthis;
    // return 1;
  }
}

pub trait QPersistentModelIndex_QPersistentModelIndex_0 {
  fn QPersistentModelIndex_0(self) -> QPersistentModelIndex;
}
// QPersistentModelIndex() ctx.fn_proto_cpp
impl<'a> /*trait*/ QPersistentModelIndex_QPersistentModelIndex_0 for () {
  fn QPersistentModelIndex_0(self) -> QPersistentModelIndex {
    // unsafe{_ZN21QPersistentModelIndexC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN21QPersistentModelIndexC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPersistentModelIndex{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:108
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QPersistentModelIndex(const QModelIndex &)

/*

*/
// QPersistentModelIndex(const QModelIndex &) ctx.fn_proto_cpp
impl /*struct*/ QPersistentModelIndex {
  pub fn QPersistentModelIndex_1<T: QPersistentModelIndex_QPersistentModelIndex_1>(value: T) -> QPersistentModelIndex {
    let rsthis = value.QPersistentModelIndex_1();
    return rsthis;
    // return 1;
  }
}

pub trait QPersistentModelIndex_QPersistentModelIndex_1 {
  fn QPersistentModelIndex_1(self) -> QPersistentModelIndex;
}
// QPersistentModelIndex(const QModelIndex &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPersistentModelIndex_QPersistentModelIndex_1 for (usize) {
  fn QPersistentModelIndex_1(self) -> QPersistentModelIndex {
    // unsafe{_ZN21QPersistentModelIndexC2ERK11QModelIndex()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN21QPersistentModelIndexC2ERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPersistentModelIndex{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:110
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QPersistentModelIndex()

/*

*/
pub fn DeleteQPersistentModelIndex(this :*mut QPersistentModelIndex) {
    // let rv = qtrt::InvokeQtFunc6("_ZN21QPersistentModelIndexD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qabstractitemmodel.h:111
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator<(const QPersistentModelIndex &) const

/*

*/
impl /*struct*/ QPersistentModelIndex {
  pub fn operator_less_than_0<RetType, T: QPersistentModelIndex_operator_less_than_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_0(self);
    // return 1;
  }
}
pub trait QPersistentModelIndex_operator_less_than_0<RetType> {
  fn operator_less_than_0(self , rsthis: & QPersistentModelIndex) -> RetType;
}
impl<'a> /*trait*/ QPersistentModelIndex_operator_less_than_0<bool> for (usize) {
  fn operator_less_than_0(self , rsthis: & QPersistentModelIndex) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QPersistentModelIndexltERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:112
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator==(const QPersistentModelIndex &) const

/*

*/
impl /*struct*/ QPersistentModelIndex {
  pub fn operator_equal_equal_0<RetType, T: QPersistentModelIndex_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QPersistentModelIndex_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QPersistentModelIndex) -> RetType;
}
impl<'a> /*trait*/ QPersistentModelIndex_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QPersistentModelIndex) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QPersistentModelIndexeqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:123
// index:1
// Public Visibility=Default Availability=Available
// [1] bool operator==(const QModelIndex &) const

/*

*/
impl /*struct*/ QPersistentModelIndex {
  pub fn operator_equal_equal_1<RetType, T: QPersistentModelIndex_operator_equal_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_1(self);
    // return 1;
  }
}
pub trait QPersistentModelIndex_operator_equal_equal_1<RetType> {
  fn operator_equal_equal_1(self , rsthis: & QPersistentModelIndex) -> RetType;
}
impl<'a> /*trait*/ QPersistentModelIndex_operator_equal_equal_1<bool> for (usize) {
  fn operator_equal_equal_1(self , rsthis: & QPersistentModelIndex) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QPersistentModelIndexeqERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:113
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QPersistentModelIndex &) const

/*

*/
impl /*struct*/ QPersistentModelIndex {
  pub fn operator_not_equal_0<RetType, T: QPersistentModelIndex_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QPersistentModelIndex_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QPersistentModelIndex) -> RetType;
}
impl<'a> /*trait*/ QPersistentModelIndex_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QPersistentModelIndex) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QPersistentModelIndexneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:124
// index:1
// Public Visibility=Default Availability=Available
// [1] bool operator!=(const QModelIndex &) const

/*

*/
impl /*struct*/ QPersistentModelIndex {
  pub fn operator_not_equal_1<RetType, T: QPersistentModelIndex_operator_not_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_1(self);
    // return 1;
  }
}
pub trait QPersistentModelIndex_operator_not_equal_1<RetType> {
  fn operator_not_equal_1(self , rsthis: & QPersistentModelIndex) -> RetType;
}
impl<'a> /*trait*/ QPersistentModelIndex_operator_not_equal_1<bool> for (usize) {
  fn operator_not_equal_1(self , rsthis: & QPersistentModelIndex) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QPersistentModelIndexneERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:115
// index:0
// Public Visibility=Default Availability=Available
// [8] QPersistentModelIndex & operator=(const QPersistentModelIndex &)

/*

*/
impl /*struct*/ QPersistentModelIndex {
  pub fn operator_equal_0<RetType, T: QPersistentModelIndex_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QPersistentModelIndex_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QPersistentModelIndex) -> RetType;
}
impl<'a> /*trait*/ QPersistentModelIndex_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QPersistentModelIndex) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN21QPersistentModelIndexaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:119
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QPersistentModelIndex & operator=(QPersistentModelIndex &&)

/*

*/
impl /*struct*/ QPersistentModelIndex {
  pub fn operator_equal_1<RetType, T: QPersistentModelIndex_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QPersistentModelIndex_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QPersistentModelIndex) -> RetType;
}
impl<'a> /*trait*/ QPersistentModelIndex_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QPersistentModelIndex) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN21QPersistentModelIndexaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:125
// index:2
// Public Visibility=Default Availability=Available
// [8] QPersistentModelIndex & operator=(const QModelIndex &)

/*

*/
impl /*struct*/ QPersistentModelIndex {
  pub fn operator_equal_2<RetType, T: QPersistentModelIndex_operator_equal_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_2(self);
    // return 1;
  }
}
pub trait QPersistentModelIndex_operator_equal_2<RetType> {
  fn operator_equal_2(self , rsthis: & QPersistentModelIndex) -> RetType;
}
impl<'a> /*trait*/ QPersistentModelIndex_operator_equal_2<usize> for (usize) {
  fn operator_equal_2(self , rsthis: & QPersistentModelIndex) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN21QPersistentModelIndexaSERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:122
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QPersistentModelIndex &)

/*

*/
impl /*struct*/ QPersistentModelIndex {
  pub fn swap_0<RetType, T: QPersistentModelIndex_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QPersistentModelIndex_swap_0<RetType> {
  fn swap_0(self , rsthis: & QPersistentModelIndex) -> RetType;
}
impl<'a> /*trait*/ QPersistentModelIndex_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QPersistentModelIndex) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN21QPersistentModelIndex4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:127
// index:0
// Public Visibility=Default Availability=Available
// [4] int row() const

/*

*/
impl /*struct*/ QPersistentModelIndex {
  pub fn row_0<RetType, T: QPersistentModelIndex_row_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.row_0(self);
    // return 1;
  }
}
pub trait QPersistentModelIndex_row_0<RetType> {
  fn row_0(self , rsthis: & QPersistentModelIndex) -> RetType;
}
impl<'a> /*trait*/ QPersistentModelIndex_row_0<i32> for () {
  fn row_0(self , rsthis: & QPersistentModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QPersistentModelIndex3rowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:128
// index:0
// Public Visibility=Default Availability=Available
// [4] int column() const

/*

*/
impl /*struct*/ QPersistentModelIndex {
  pub fn column_0<RetType, T: QPersistentModelIndex_column_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.column_0(self);
    // return 1;
  }
}
pub trait QPersistentModelIndex_column_0<RetType> {
  fn column_0(self , rsthis: & QPersistentModelIndex) -> RetType;
}
impl<'a> /*trait*/ QPersistentModelIndex_column_0<i32> for () {
  fn column_0(self , rsthis: & QPersistentModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QPersistentModelIndex6columnEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:129
// index:0
// Public Visibility=Default Availability=Available
// [8] void * internalPointer() const

/*

*/
impl /*struct*/ QPersistentModelIndex {
  pub fn internalPointer_0<RetType, T: QPersistentModelIndex_internalPointer_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.internalPointer_0(self);
    // return 1;
  }
}
pub trait QPersistentModelIndex_internalPointer_0<RetType> {
  fn internalPointer_0(self , rsthis: & QPersistentModelIndex) -> RetType;
}
impl<'a> /*trait*/ QPersistentModelIndex_internalPointer_0<usize> for () {
  fn internalPointer_0(self , rsthis: & QPersistentModelIndex) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QPersistentModelIndex15internalPointerEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:130
// index:0
// Public Visibility=Default Availability=Available
// [8] quintptr internalId() const

/*

*/
impl /*struct*/ QPersistentModelIndex {
  pub fn internalId_0<RetType, T: QPersistentModelIndex_internalId_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.internalId_0(self);
    // return 1;
  }
}
pub trait QPersistentModelIndex_internalId_0<RetType> {
  fn internalId_0(self , rsthis: & QPersistentModelIndex) -> RetType;
}
impl<'a> /*trait*/ QPersistentModelIndex_internalId_0<u64> for () {
  fn internalId_0(self , rsthis: & QPersistentModelIndex) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QPersistentModelIndex10internalIdEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:131
// index:0
// Public Visibility=Default Availability=Available
// [24] QModelIndex parent() const

/*
Returns the parent of the model item with the given index. If the item has no parent, an invalid QModelIndex is returned.

A common convention used in models that expose tree data structures is that only items in the first column have children. For that case, when reimplementing this function in a subclass the column of the returned QModelIndex would be 0.

When reimplementing this function in a subclass, be careful to avoid calling QModelIndex member functions, such as QModelIndex::parent(), since indexes belonging to your model will simply call your implementation, leading to infinite recursion.

See also createIndex().
*/
impl /*struct*/ QPersistentModelIndex {
  pub fn parent_0<RetType, T: QPersistentModelIndex_parent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.parent_0(self);
    // return 1;
  }
}
pub trait QPersistentModelIndex_parent_0<RetType> {
  fn parent_0(self , rsthis: & QPersistentModelIndex) -> RetType;
}
impl<'a> /*trait*/ QPersistentModelIndex_parent_0<usize> for () {
  fn parent_0(self , rsthis: & QPersistentModelIndex) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QPersistentModelIndex6parentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:132
// index:0
// Public Visibility=Default Availability=Available
// [24] QModelIndex sibling(int, int) const

/*
Returns the sibling at row and column for the item at index, or an invalid QModelIndex if there is no sibling at that location.

sibling() is just a convenience function that finds the item's parent, and uses it to retrieve the index of the child item in the specified row and column.

This method can optionally be overridden for implementation-specific optimization.

See also index(), QModelIndex::row(), and QModelIndex::column().
*/
impl /*struct*/ QPersistentModelIndex {
  pub fn sibling_0<RetType, T: QPersistentModelIndex_sibling_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sibling_0(self);
    // return 1;
  }
}
pub trait QPersistentModelIndex_sibling_0<RetType> {
  fn sibling_0(self , rsthis: & QPersistentModelIndex) -> RetType;
}
impl<'a> /*trait*/ QPersistentModelIndex_sibling_0<usize> for (i32,i32) {
  fn sibling_0(self , rsthis: & QPersistentModelIndex) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QPersistentModelIndex7siblingEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:134
// index:0
// Public Visibility=Default Availability=Available
// [24] QModelIndex child(int, int) const

/*

*/
impl /*struct*/ QPersistentModelIndex {
  pub fn child_0<RetType, T: QPersistentModelIndex_child_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.child_0(self);
    // return 1;
  }
}
pub trait QPersistentModelIndex_child_0<RetType> {
  fn child_0(self , rsthis: & QPersistentModelIndex) -> RetType;
}
impl<'a> /*trait*/ QPersistentModelIndex_child_0<usize> for (i32,i32) {
  fn child_0(self , rsthis: & QPersistentModelIndex) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QPersistentModelIndex5childEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:136
// index:0
// Public Visibility=Default Availability=Available
// [16] QVariant data(int) const

/*
Returns the data stored under the given role for the item referred to by the index.

Note: If you do not have a value to return, return an invalid QVariant instead of returning 0.

See also Qt::ItemDataRole, setData(), and headerData().
*/
impl /*struct*/ QPersistentModelIndex {
  pub fn data_0<RetType, T: QPersistentModelIndex_data_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.data_0(self);
    // return 1;
  }
}
pub trait QPersistentModelIndex_data_0<RetType> {
  fn data_0(self , rsthis: & QPersistentModelIndex) -> RetType;
}
impl<'a> /*trait*/ QPersistentModelIndex_data_0<usize> for (i32) {
  fn data_0(self , rsthis: & QPersistentModelIndex) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QPersistentModelIndex4dataEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:137
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::ItemFlags flags() const

/*
Returns the item flags for the given index.

The base class implementation returns a combination of flags that enables the item (ItemIsEnabled) and allows it to be selected (ItemIsSelectable).

See also Qt::ItemFlags.
*/
impl /*struct*/ QPersistentModelIndex {
  pub fn flags_0<RetType, T: QPersistentModelIndex_flags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.flags_0(self);
    // return 1;
  }
}
pub trait QPersistentModelIndex_flags_0<RetType> {
  fn flags_0(self , rsthis: & QPersistentModelIndex) -> RetType;
}
impl<'a> /*trait*/ QPersistentModelIndex_flags_0<i32> for () {
  fn flags_0(self , rsthis: & QPersistentModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QPersistentModelIndex5flagsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:138
// index:0
// Public Visibility=Default Availability=Available
// [8] const QAbstractItemModel * model() const

/*

*/
impl /*struct*/ QPersistentModelIndex {
  pub fn model_0<RetType, T: QPersistentModelIndex_model_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.model_0(self);
    // return 1;
  }
}
pub trait QPersistentModelIndex_model_0<RetType> {
  fn model_0(self , rsthis: & QPersistentModelIndex) -> RetType;
}
impl<'a> /*trait*/ QPersistentModelIndex_model_0<usize> for () {
  fn model_0(self , rsthis: & QPersistentModelIndex) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QPersistentModelIndex5modelEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:139
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isValid() const

/*

*/
impl /*struct*/ QPersistentModelIndex {
  pub fn isValid_0<RetType, T: QPersistentModelIndex_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QPersistentModelIndex_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QPersistentModelIndex) -> RetType;
}
impl<'a> /*trait*/ QPersistentModelIndex_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QPersistentModelIndex) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QPersistentModelIndex7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
