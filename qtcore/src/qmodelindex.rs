

// mod ::core::QModelIndex
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
// extern C begin: 5
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
#[derive(Default)] // class sizeof(QModelIndex)=24
pub struct QModelIndex {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QModelIndex_ITF interface {
//    QModelIndex_PTR() *QModelIndex
//}
//func (ptr *QModelIndex) QModelIndex_PTR() *QModelIndex { return ptr }

impl /*struct*/ QModelIndex {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QModelIndex {
    return QModelIndex{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QModelIndex {
//  type Target = QModelIndexBASE;
//
//  fn deref(&self) -> &QModelIndexBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QModelIndexBASE> for QModelIndex {
//  fn as_ref(& self) -> & QModelIndexBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qabstractitemmodel.h:58
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QModelIndex()

/*

*/
// QModelIndex() ctx.fn_proto_cpp
impl /*struct*/ QModelIndex {
  pub fn QModelIndex_0<T: QModelIndex_QModelIndex_0>(value: T) -> QModelIndex {
    let rsthis = value.QModelIndex_0();
    return rsthis;
    // return 1;
  }
}

pub trait QModelIndex_QModelIndex_0 {
  fn QModelIndex_0(self) -> QModelIndex;
}
// QModelIndex() ctx.fn_proto_cpp
impl<'a> /*trait*/ QModelIndex_QModelIndex_0 for () {
  fn QModelIndex_0(self) -> QModelIndex {
    // unsafe{_ZN11QModelIndexC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QModelIndexC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QModelIndex{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:60
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int row() const

/*

*/
impl /*struct*/ QModelIndex {
  pub fn row_0<RetType, T: QModelIndex_row_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.row_0(self);
    // return 1;
  }
}
pub trait QModelIndex_row_0<RetType> {
  fn row_0(self , rsthis: & QModelIndex) -> RetType;
}
impl<'a> /*trait*/ QModelIndex_row_0<i32> for () {
  fn row_0(self , rsthis: & QModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QModelIndex3rowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:61
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int column() const

/*

*/
impl /*struct*/ QModelIndex {
  pub fn column_0<RetType, T: QModelIndex_column_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.column_0(self);
    // return 1;
  }
}
pub trait QModelIndex_column_0<RetType> {
  fn column_0(self , rsthis: & QModelIndex) -> RetType;
}
impl<'a> /*trait*/ QModelIndex_column_0<i32> for () {
  fn column_0(self , rsthis: & QModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QModelIndex6columnEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:62
// index:0
// Public inline Visibility=Default Availability=Available
// [8] quintptr internalId() const

/*

*/
impl /*struct*/ QModelIndex {
  pub fn internalId_0<RetType, T: QModelIndex_internalId_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.internalId_0(self);
    // return 1;
  }
}
pub trait QModelIndex_internalId_0<RetType> {
  fn internalId_0(self , rsthis: & QModelIndex) -> RetType;
}
impl<'a> /*trait*/ QModelIndex_internalId_0<u64> for () {
  fn internalId_0(self , rsthis: & QModelIndex) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QModelIndex10internalIdEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:63
// index:0
// Public inline Visibility=Default Availability=Available
// [8] void * internalPointer() const

/*

*/
impl /*struct*/ QModelIndex {
  pub fn internalPointer_0<RetType, T: QModelIndex_internalPointer_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.internalPointer_0(self);
    // return 1;
  }
}
pub trait QModelIndex_internalPointer_0<RetType> {
  fn internalPointer_0(self , rsthis: & QModelIndex) -> RetType;
}
impl<'a> /*trait*/ QModelIndex_internalPointer_0<usize> for () {
  fn internalPointer_0(self , rsthis: & QModelIndex) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QModelIndex15internalPointerEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:64
// index:0
// Public inline Visibility=Default Availability=Available
// [24] QModelIndex parent() const

/*
Returns the parent of the model item with the given index. If the item has no parent, an invalid QModelIndex is returned.

A common convention used in models that expose tree data structures is that only items in the first column have children. For that case, when reimplementing this function in a subclass the column of the returned QModelIndex would be 0.

When reimplementing this function in a subclass, be careful to avoid calling QModelIndex member functions, such as QModelIndex::parent(), since indexes belonging to your model will simply call your implementation, leading to infinite recursion.

See also createIndex().
*/
impl /*struct*/ QModelIndex {
  pub fn parent_0<RetType, T: QModelIndex_parent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.parent_0(self);
    // return 1;
  }
}
pub trait QModelIndex_parent_0<RetType> {
  fn parent_0(self , rsthis: & QModelIndex) -> RetType;
}
impl<'a> /*trait*/ QModelIndex_parent_0<usize> for () {
  fn parent_0(self , rsthis: & QModelIndex) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QModelIndex6parentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:65
// index:0
// Public inline Visibility=Default Availability=Available
// [24] QModelIndex sibling(int, int) const

/*
Returns the sibling at row and column for the item at index, or an invalid QModelIndex if there is no sibling at that location.

sibling() is just a convenience function that finds the item's parent, and uses it to retrieve the index of the child item in the specified row and column.

This method can optionally be overridden for implementation-specific optimization.

See also index(), QModelIndex::row(), and QModelIndex::column().
*/
impl /*struct*/ QModelIndex {
  pub fn sibling_0<RetType, T: QModelIndex_sibling_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sibling_0(self);
    // return 1;
  }
}
pub trait QModelIndex_sibling_0<RetType> {
  fn sibling_0(self , rsthis: & QModelIndex) -> RetType;
}
impl<'a> /*trait*/ QModelIndex_sibling_0<usize> for (i32,i32) {
  fn sibling_0(self , rsthis: & QModelIndex) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QModelIndex7siblingEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:67
// index:0
// Public inline Visibility=Default Availability=Available
// [24] QModelIndex child(int, int) const

/*

*/
impl /*struct*/ QModelIndex {
  pub fn child_0<RetType, T: QModelIndex_child_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.child_0(self);
    // return 1;
  }
}
pub trait QModelIndex_child_0<RetType> {
  fn child_0(self , rsthis: & QModelIndex) -> RetType;
}
impl<'a> /*trait*/ QModelIndex_child_0<usize> for (i32,i32) {
  fn child_0(self , rsthis: & QModelIndex) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QModelIndex5childEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:69
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QVariant data(int) const

/*
Returns the data stored under the given role for the item referred to by the index.

Note: If you do not have a value to return, return an invalid QVariant instead of returning 0.

See also Qt::ItemDataRole, setData(), and headerData().
*/
impl /*struct*/ QModelIndex {
  pub fn data_0<RetType, T: QModelIndex_data_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.data_0(self);
    // return 1;
  }
}
pub trait QModelIndex_data_0<RetType> {
  fn data_0(self , rsthis: & QModelIndex) -> RetType;
}
impl<'a> /*trait*/ QModelIndex_data_0<usize> for (i32) {
  fn data_0(self , rsthis: & QModelIndex) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QModelIndex4dataEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:70
// index:0
// Public inline Visibility=Default Availability=Available
// [4] Qt::ItemFlags flags() const

/*
Returns the item flags for the given index.

The base class implementation returns a combination of flags that enables the item (ItemIsEnabled) and allows it to be selected (ItemIsSelectable).

See also Qt::ItemFlags.
*/
impl /*struct*/ QModelIndex {
  pub fn flags_0<RetType, T: QModelIndex_flags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.flags_0(self);
    // return 1;
  }
}
pub trait QModelIndex_flags_0<RetType> {
  fn flags_0(self , rsthis: & QModelIndex) -> RetType;
}
impl<'a> /*trait*/ QModelIndex_flags_0<i32> for () {
  fn flags_0(self , rsthis: & QModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QModelIndex5flagsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:71
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QAbstractItemModel * model() const

/*

*/
impl /*struct*/ QModelIndex {
  pub fn model_0<RetType, T: QModelIndex_model_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.model_0(self);
    // return 1;
  }
}
pub trait QModelIndex_model_0<RetType> {
  fn model_0(self , rsthis: & QModelIndex) -> RetType;
}
impl<'a> /*trait*/ QModelIndex_model_0<usize> for () {
  fn model_0(self , rsthis: & QModelIndex) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QModelIndex5modelEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:72
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isValid() const

/*

*/
impl /*struct*/ QModelIndex {
  pub fn isValid_0<RetType, T: QModelIndex_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QModelIndex_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QModelIndex) -> RetType;
}
impl<'a> /*trait*/ QModelIndex_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QModelIndex) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QModelIndex7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:73
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator==(const QModelIndex &) const

/*

*/
impl /*struct*/ QModelIndex {
  pub fn operator_equal_equal_0<RetType, T: QModelIndex_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QModelIndex_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QModelIndex) -> RetType;
}
impl<'a> /*trait*/ QModelIndex_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QModelIndex) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QModelIndexeqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:75
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QModelIndex &) const

/*

*/
impl /*struct*/ QModelIndex {
  pub fn operator_not_equal_0<RetType, T: QModelIndex_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QModelIndex_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QModelIndex) -> RetType;
}
impl<'a> /*trait*/ QModelIndex_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QModelIndex) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QModelIndexneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:77
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator<(const QModelIndex &) const

/*

*/
impl /*struct*/ QModelIndex {
  pub fn operator_less_than_0<RetType, T: QModelIndex_operator_less_than_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_0(self);
    // return 1;
  }
}
pub trait QModelIndex_operator_less_than_0<RetType> {
  fn operator_less_than_0(self , rsthis: & QModelIndex) -> RetType;
}
impl<'a> /*trait*/ QModelIndex_operator_less_than_0<bool> for (usize) {
  fn operator_less_than_0(self , rsthis: & QModelIndex) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QModelIndexltERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


pub fn DeleteQModelIndex(this :*mut QModelIndex) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN11QModelIndexD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
