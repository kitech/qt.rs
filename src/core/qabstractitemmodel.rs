

// mod ::core::QAbstractItemModel
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
use super::super::qtrt;
use super::*;
//  ext block end

//  body block begin

// void resetInternalData()
// func (this *QAbstractItemModel) InheritResetInternalData(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "resetInternalData", f)
// }

// QModelIndex createIndex(int, int, void *)
// func (this *QAbstractItemModel) InheritCreateIndex(f func(row int, column int, data unsafe.Pointer /*666*/) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "createIndex", f)
// }

// bool decodeData(int, int, const QModelIndex &, QDataStream &)
// func (this *QAbstractItemModel) InheritDecodeData(f func(row int, column int, parent *QModelIndex, stream *QDataStream) bool) {
//  qtrt.SetAllInheritCallback(this, "decodeData", f)
// }

// void beginInsertRows(const QModelIndex &, int, int)
// func (this *QAbstractItemModel) InheritBeginInsertRows(f func(parent *QModelIndex, first int, last int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "beginInsertRows", f)
// }

// void endInsertRows()
// func (this *QAbstractItemModel) InheritEndInsertRows(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "endInsertRows", f)
// }

// void beginRemoveRows(const QModelIndex &, int, int)
// func (this *QAbstractItemModel) InheritBeginRemoveRows(f func(parent *QModelIndex, first int, last int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "beginRemoveRows", f)
// }

// void endRemoveRows()
// func (this *QAbstractItemModel) InheritEndRemoveRows(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "endRemoveRows", f)
// }

// bool beginMoveRows(const QModelIndex &, int, int, const QModelIndex &, int)
// func (this *QAbstractItemModel) InheritBeginMoveRows(f func(sourceParent *QModelIndex, sourceFirst int, sourceLast int, destinationParent *QModelIndex, destinationRow int) bool) {
//  qtrt.SetAllInheritCallback(this, "beginMoveRows", f)
// }

// void endMoveRows()
// func (this *QAbstractItemModel) InheritEndMoveRows(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "endMoveRows", f)
// }

// void beginInsertColumns(const QModelIndex &, int, int)
// func (this *QAbstractItemModel) InheritBeginInsertColumns(f func(parent *QModelIndex, first int, last int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "beginInsertColumns", f)
// }

// void endInsertColumns()
// func (this *QAbstractItemModel) InheritEndInsertColumns(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "endInsertColumns", f)
// }

// void beginRemoveColumns(const QModelIndex &, int, int)
// func (this *QAbstractItemModel) InheritBeginRemoveColumns(f func(parent *QModelIndex, first int, last int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "beginRemoveColumns", f)
// }

// void endRemoveColumns()
// func (this *QAbstractItemModel) InheritEndRemoveColumns(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "endRemoveColumns", f)
// }

// bool beginMoveColumns(const QModelIndex &, int, int, const QModelIndex &, int)
// func (this *QAbstractItemModel) InheritBeginMoveColumns(f func(sourceParent *QModelIndex, sourceFirst int, sourceLast int, destinationParent *QModelIndex, destinationColumn int) bool) {
//  qtrt.SetAllInheritCallback(this, "beginMoveColumns", f)
// }

// void endMoveColumns()
// func (this *QAbstractItemModel) InheritEndMoveColumns(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "endMoveColumns", f)
// }

// void beginResetModel()
// func (this *QAbstractItemModel) InheritBeginResetModel(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "beginResetModel", f)
// }

// void endResetModel()
// func (this *QAbstractItemModel) InheritEndResetModel(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "endResetModel", f)
// }

// void changePersistentIndex(const QModelIndex &, const QModelIndex &)
// func (this *QAbstractItemModel) InheritChangePersistentIndex(f func(from *QModelIndex, to *QModelIndex) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "changePersistentIndex", f)
// }

// QModelIndexList persistentIndexList()
// func (this *QAbstractItemModel) InheritPersistentIndexList(f func() *QModelIndexList/*9999*/) {
//  qtrt.SetAllInheritCallback(this, "persistentIndexList", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QAbstractItemModel)=16
pub struct QAbstractItemModel {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAbstractItemModel_ITF interface {
//    QObject_ITF
//    QAbstractItemModel_PTR() *QAbstractItemModel
//}
//func (ptr *QAbstractItemModel) QAbstractItemModel_PTR() *QAbstractItemModel { return ptr }

impl /*struct*/ QAbstractItemModel {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAbstractItemModel {
    return QAbstractItemModel{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAbstractItemModel {
//  type Target = QAbstractItemModelBASE;
//
//  fn deref(&self) -> &QAbstractItemModelBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAbstractItemModelBASE> for QAbstractItemModel {
//  fn as_ref(& self) -> & QAbstractItemModelBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qabstractitemmodel.h:167
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QAbstractItemModel {
  pub fn metaObject_0<RetType, T: QAbstractItemModel_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QAbstractItemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QAbstractItemModel10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:174
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QAbstractItemModel(QObject *)

/*
Constructs an abstract item model with the given parent.
*/
// QAbstractItemModel(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QAbstractItemModel {
  pub fn QAbstractItemModel_0<T: QAbstractItemModel_QAbstractItemModel_0>(value: T) -> QAbstractItemModel {
    let rsthis = value.QAbstractItemModel_0();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractItemModel_QAbstractItemModel_0 {
  fn QAbstractItemModel_0(self) -> QAbstractItemModel;
}
// QAbstractItemModel(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QAbstractItemModel_QAbstractItemModel_0 for (usize) {
  fn QAbstractItemModel_0(self) -> QAbstractItemModel {
    // unsafe{_ZN18QAbstractItemModelC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QAbstractItemModelC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAbstractItemModel{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:175
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QAbstractItemModel()

/*

*/
pub fn DeleteQAbstractItemModel(this :*mut QAbstractItemModel) {
    // let rv = qtrt::InvokeQtFunc6("_ZN18QAbstractItemModelD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qabstractitemmodel.h:177
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasIndex(int, int, const QModelIndex &) const

/*
Returns true if the model returns a valid QModelIndex for row and column with parent, otherwise returns false.
*/
impl /*struct*/ QAbstractItemModel {
  pub fn hasIndex_0<RetType, T: QAbstractItemModel_hasIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasIndex_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_hasIndex_0<RetType> {
  fn hasIndex_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_hasIndex_0<bool> for (i32,i32,usize) {
  fn hasIndex_0(self , rsthis: & QAbstractItemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QAbstractItemModel8hasIndexEiiRK11QModelIndex", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:178
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [24] QModelIndex index(int, int, const QModelIndex &) const

/*
Returns the index of the item in the model specified by the given row, column and parent index.

When reimplementing this function in a subclass, call createIndex() to generate model indexes that other components can use to refer to items in your model.

See also createIndex().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn index_0<RetType, T: QAbstractItemModel_index_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.index_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_index_0<RetType> {
  fn index_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_index_0<usize> for (i32,i32,usize) {
  fn index_0(self , rsthis: & QAbstractItemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QAbstractItemModel5indexEiiRK11QModelIndex", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:180
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [24] QModelIndex parent(const QModelIndex &) const

/*
Returns the parent of the model item with the given index. If the item has no parent, an invalid QModelIndex is returned.

A common convention used in models that expose tree data structures is that only items in the first column have children. For that case, when reimplementing this function in a subclass the column of the returned QModelIndex would be 0.

When reimplementing this function in a subclass, be careful to avoid calling QModelIndex member functions, such as QModelIndex::parent(), since indexes belonging to your model will simply call your implementation, leading to infinite recursion.

See also createIndex().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn parent_0<RetType, T: QAbstractItemModel_parent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.parent_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_parent_0<RetType> {
  fn parent_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_parent_0<usize> for (usize) {
  fn parent_0(self , rsthis: & QAbstractItemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QAbstractItemModel6parentERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:182
// index:0
// Public virtual Visibility=Default Availability=Available
// [24] QModelIndex sibling(int, int, const QModelIndex &) const

/*
Returns the sibling at row and column for the item at index, or an invalid QModelIndex if there is no sibling at that location.

sibling() is just a convenience function that finds the item's parent, and uses it to retrieve the index of the child item in the specified row and column.

This method can optionally be overridden for implementation-specific optimization.

See also index(), QModelIndex::row(), and QModelIndex::column().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn sibling_0<RetType, T: QAbstractItemModel_sibling_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sibling_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_sibling_0<RetType> {
  fn sibling_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_sibling_0<usize> for (i32,i32,usize) {
  fn sibling_0(self , rsthis: & QAbstractItemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QAbstractItemModel7siblingEiiRK11QModelIndex", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:183
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [4] int rowCount(const QModelIndex &) const

/*
Returns the number of rows under the given parent. When the parent is valid it means that rowCount is returning the number of children of parent.

Note: When implementing a table based model, rowCount() should return 0 when the parent is valid.

See also columnCount().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn rowCount_0<RetType, T: QAbstractItemModel_rowCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowCount_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_rowCount_0<RetType> {
  fn rowCount_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_rowCount_0<i32> for (usize) {
  fn rowCount_0(self , rsthis: & QAbstractItemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QAbstractItemModel8rowCountERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:184
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [4] int columnCount(const QModelIndex &) const

/*
Returns the number of columns for the children of the given parent.

In most subclasses, the number of columns is independent of the parent.

For example:


  int DomModel::columnCount(const QModelIndex &/-*parent*-/) const
  {
      return 3;
  }



Note: When implementing a table based model, columnCount() should return 0 when the parent is valid.

See also rowCount().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn columnCount_0<RetType, T: QAbstractItemModel_columnCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnCount_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_columnCount_0<RetType> {
  fn columnCount_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_columnCount_0<i32> for (usize) {
  fn columnCount_0(self , rsthis: & QAbstractItemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QAbstractItemModel11columnCountERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:185
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool hasChildren(const QModelIndex &) const

/*
Returns true if parent has any children; otherwise returns false.

Use rowCount() on the parent to find out the number of children.

Note that it is undefined behavior to report that a particular index hasChildren with this method if the same index has the flag Qt::ItemNeverHasChildren set.

See also parent() and index().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn hasChildren_0<RetType, T: QAbstractItemModel_hasChildren_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasChildren_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_hasChildren_0<RetType> {
  fn hasChildren_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_hasChildren_0<bool> for (usize) {
  fn hasChildren_0(self , rsthis: & QAbstractItemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QAbstractItemModel11hasChildrenERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:187
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [16] QVariant data(const QModelIndex &, int) const

/*
Returns the data stored under the given role for the item referred to by the index.

Note: If you do not have a value to return, return an invalid QVariant instead of returning 0.

See also Qt::ItemDataRole, setData(), and headerData().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn data_0<RetType, T: QAbstractItemModel_data_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.data_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_data_0<RetType> {
  fn data_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_data_0<usize> for (usize,i32) {
  fn data_0(self , rsthis: & QAbstractItemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QAbstractItemModel4dataERK11QModelIndexi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:188
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool setData(const QModelIndex &, const QVariant &, int)

/*
Sets the role data for the item at index to value.

Returns true if successful; otherwise returns false.

The dataChanged() signal should be emitted if the data was successfully set.

The base class implementation returns false. This function and data() must be reimplemented for editable models.

See also Qt::ItemDataRole, data(), and itemData().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn setData_0<RetType, T: QAbstractItemModel_setData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setData_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_setData_0<RetType> {
  fn setData_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_setData_0<bool> for (usize,usize,i32) {
  fn setData_0(self , rsthis: & QAbstractItemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QAbstractItemModel7setDataERK11QModelIndexRK8QVarianti", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:190
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QVariant headerData(int, Qt::Orientation, int) const

/*
Returns the data for the given role and section in the header with the specified orientation.

For horizontal headers, the section number corresponds to the column number. Similarly, for vertical headers, the section number corresponds to the row number.

See also Qt::ItemDataRole, setHeaderData(), and QHeaderView.
*/
impl /*struct*/ QAbstractItemModel {
  pub fn headerData_0<RetType, T: QAbstractItemModel_headerData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.headerData_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_headerData_0<RetType> {
  fn headerData_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_headerData_0<usize> for (i32,i32,i32) {
  fn headerData_0(self , rsthis: & QAbstractItemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QAbstractItemModel10headerDataEiN2Qt11OrientationEi", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:192
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool setHeaderData(int, Qt::Orientation, const QVariant &, int)

/*
Sets the data for the given role and section in the header with the specified orientation to the value supplied.

Returns true if the header's data was updated; otherwise returns false.

When reimplementing this function, the headerDataChanged() signal must be emitted explicitly.

See also Qt::ItemDataRole and headerData().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn setHeaderData_0<RetType, T: QAbstractItemModel_setHeaderData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHeaderData_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_setHeaderData_0<RetType> {
  fn setHeaderData_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_setHeaderData_0<bool> for (i32,i32,usize,i32) {
  fn setHeaderData_0(self , rsthis: & QAbstractItemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QAbstractItemModel13setHeaderDataEiN2Qt11OrientationERK8QVarianti", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:198
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QStringList mimeTypes() const

/*
Returns the list of allowed MIME types. By default, the built-in models and views use an internal MIME type: application/x-qabstractitemmodeldatalist.

When implementing drag and drop support in a custom model, if you will return data in formats other than the default internal MIME type, reimplement this function to return your list of MIME types.

If you reimplement this function in your custom model, you must also reimplement the member functions that call it: mimeData() and dropMimeData().

See also mimeData() and dropMimeData().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn mimeTypes_0<RetType, T: QAbstractItemModel_mimeTypes_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mimeTypes_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_mimeTypes_0<RetType> {
  fn mimeTypes_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_mimeTypes_0<usize> for () {
  fn mimeTypes_0(self , rsthis: & QAbstractItemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QAbstractItemModel9mimeTypesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:200
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool canDropMimeData(const QMimeData *, Qt::DropAction, int, int, const QModelIndex &) const

/*
Returns true if a model can accept a drop of the data. This default implementation only checks if data has at least one format in the list of mimeTypes() and if action is among the model's supportedDropActions().

Reimplement this function in your custom model, if you want to test whether the data can be dropped at row, column, parent with action. If you don't need that test, it is not necessary to reimplement this function.

See also dropMimeData() and Using drag and drop with item views.
*/
impl /*struct*/ QAbstractItemModel {
  pub fn canDropMimeData_0<RetType, T: QAbstractItemModel_canDropMimeData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.canDropMimeData_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_canDropMimeData_0<RetType> {
  fn canDropMimeData_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_canDropMimeData_0<bool> for (usize,i32,i32,i32,usize) {
  fn canDropMimeData_0(self , rsthis: & QAbstractItemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QAbstractItemModel15canDropMimeDataEPK9QMimeDataN2Qt10DropActionEiiRK11QModelIndex", 5,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:202
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool dropMimeData(const QMimeData *, Qt::DropAction, int, int, const QModelIndex &)

/*
Handles the data supplied by a drag and drop operation that ended with the given action.

Returns true if the data and action were handled by the model; otherwise returns false.

The specified row, column and parent indicate the location of an item in the model where the operation ended. It is the responsibility of the model to complete the action at the correct location.

For instance, a drop action on an item in a QTreeView can result in new items either being inserted as children of the item specified by row, column, and parent, or as siblings of the item.

When row and column are -1 it means that the dropped data should be considered as dropped directly on parent. Usually this will mean appending the data as child items of parent. If row and column are greater than or equal zero, it means that the drop occurred just before the specified row and column in the specified parent.

The mimeTypes() member is called to get the list of acceptable MIME types. This default implementation assumes the default implementation of mimeTypes(), which returns a single default MIME type. If you reimplement mimeTypes() in your custom model to return multiple MIME types, you must reimplement this function to make use of them.

See also supportedDropActions(), canDropMimeData(), and Using drag and drop with item views.
*/
impl /*struct*/ QAbstractItemModel {
  pub fn dropMimeData_0<RetType, T: QAbstractItemModel_dropMimeData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dropMimeData_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_dropMimeData_0<RetType> {
  fn dropMimeData_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_dropMimeData_0<bool> for (usize,i32,i32,i32,usize) {
  fn dropMimeData_0(self , rsthis: & QAbstractItemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QAbstractItemModel12dropMimeDataEPK9QMimeDataN2Qt10DropActionEiiRK11QModelIndex", 5,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:204
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] Qt::DropActions supportedDropActions() const

/*
Returns the drop actions supported by this model.

The default implementation returns Qt::CopyAction. Reimplement this function if you wish to support additional actions. You must also reimplement the dropMimeData() function to handle the additional operations.

This function was introduced in  Qt 4.2.

See also dropMimeData(), Qt::DropActions, and Using drag and drop with item views.
*/
impl /*struct*/ QAbstractItemModel {
  pub fn supportedDropActions_0<RetType, T: QAbstractItemModel_supportedDropActions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.supportedDropActions_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_supportedDropActions_0<RetType> {
  fn supportedDropActions_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_supportedDropActions_0<i32> for () {
  fn supportedDropActions_0(self , rsthis: & QAbstractItemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QAbstractItemModel20supportedDropActionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:206
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] Qt::DropActions supportedDragActions() const

/*
Returns the actions supported by the data in this model.

The default implementation returns supportedDropActions(). Reimplement this function if you wish to support additional actions.

supportedDragActions() is used by QAbstractItemView::startDrag() as the default values when a drag occurs.

See also setSupportedDragActions(), Qt::DropActions, and Using drag and drop with item views.
*/
impl /*struct*/ QAbstractItemModel {
  pub fn supportedDragActions_0<RetType, T: QAbstractItemModel_supportedDragActions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.supportedDragActions_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_supportedDragActions_0<RetType> {
  fn supportedDragActions_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_supportedDragActions_0<i32> for () {
  fn supportedDragActions_0(self , rsthis: & QAbstractItemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QAbstractItemModel20supportedDragActionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:212
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool insertRows(int, int, const QModelIndex &)

/*
Note: The base class implementation of this function does nothing and returns false.

On models that support this, inserts count rows into the model before the given row. Items in the new row will be children of the item represented by the parent model index.

If row is 0, the rows are prepended to any existing rows in the parent.

If row is rowCount(), the rows are appended to any existing rows in the parent.

If parent has no children, a single column with count rows is inserted.

Returns true if the rows were successfully inserted; otherwise returns false.

If you implement your own model, you can reimplement this function if you want to support insertions. Alternatively, you can provide your own API for altering the data. In either case, you will need to call beginInsertRows() and endInsertRows() to notify other components that the model has changed.

See also insertColumns(), removeRows(), beginInsertRows(), and endInsertRows().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn insertRows_0<RetType, T: QAbstractItemModel_insertRows_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertRows_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_insertRows_0<RetType> {
  fn insertRows_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_insertRows_0<bool> for (i32,i32,usize) {
  fn insertRows_0(self , rsthis: & QAbstractItemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QAbstractItemModel10insertRowsEiiRK11QModelIndex", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:213
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool insertColumns(int, int, const QModelIndex &)

/*
On models that support this, inserts count new columns into the model before the given column. The items in each new column will be children of the item represented by the parent model index.

If column is 0, the columns are prepended to any existing columns.

If column is columnCount(), the columns are appended to any existing columns.

If parent has no children, a single row with count columns is inserted.

Returns true if the columns were successfully inserted; otherwise returns false.

The base class implementation does nothing and returns false.

If you implement your own model, you can reimplement this function if you want to support insertions. Alternatively, you can provide your own API for altering the data.

See also insertRows(), removeColumns(), beginInsertColumns(), and endInsertColumns().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn insertColumns_0<RetType, T: QAbstractItemModel_insertColumns_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertColumns_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_insertColumns_0<RetType> {
  fn insertColumns_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_insertColumns_0<bool> for (i32,i32,usize) {
  fn insertColumns_0(self , rsthis: & QAbstractItemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QAbstractItemModel13insertColumnsEiiRK11QModelIndex", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:214
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool removeRows(int, int, const QModelIndex &)

/*
On models that support this, removes count rows starting with the given row under parent parent from the model.

Returns true if the rows were successfully removed; otherwise returns false.

The base class implementation does nothing and returns false.

If you implement your own model, you can reimplement this function if you want to support removing. Alternatively, you can provide your own API for altering the data.

See also removeRow(), removeColumns(), insertColumns(), beginRemoveRows(), and endRemoveRows().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn removeRows_0<RetType, T: QAbstractItemModel_removeRows_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeRows_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_removeRows_0<RetType> {
  fn removeRows_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_removeRows_0<bool> for (i32,i32,usize) {
  fn removeRows_0(self , rsthis: & QAbstractItemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QAbstractItemModel10removeRowsEiiRK11QModelIndex", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:215
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool removeColumns(int, int, const QModelIndex &)

/*
On models that support this, removes count columns starting with the given column under parent parent from the model.

Returns true if the columns were successfully removed; otherwise returns false.

The base class implementation does nothing and returns false.

If you implement your own model, you can reimplement this function if you want to support removing. Alternatively, you can provide your own API for altering the data.

See also removeColumn(), removeRows(), insertColumns(), beginRemoveColumns(), and endRemoveColumns().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn removeColumns_0<RetType, T: QAbstractItemModel_removeColumns_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeColumns_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_removeColumns_0<RetType> {
  fn removeColumns_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_removeColumns_0<bool> for (i32,i32,usize) {
  fn removeColumns_0(self , rsthis: & QAbstractItemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QAbstractItemModel13removeColumnsEiiRK11QModelIndex", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:216
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool moveRows(const QModelIndex &, int, int, const QModelIndex &, int)

/*
On models that support this, moves count rows starting with the given sourceRow under parent sourceParent to row destinationChild under parent destinationParent.

Returns true if the rows were successfully moved; otherwise returns false.

The base class implementation does nothing and returns false.

If you implement your own model, you can reimplement this function if you want to support moving. Alternatively, you can provide your own API for altering the data.

See also beginMoveRows() and endMoveRows().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn moveRows_0<RetType, T: QAbstractItemModel_moveRows_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveRows_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_moveRows_0<RetType> {
  fn moveRows_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_moveRows_0<bool> for (usize,i32,i32,usize,i32) {
  fn moveRows_0(self , rsthis: & QAbstractItemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QAbstractItemModel8moveRowsERK11QModelIndexiiS2_i", 5,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:218
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool moveColumns(const QModelIndex &, int, int, const QModelIndex &, int)

/*
On models that support this, moves count columns starting with the given sourceColumn under parent sourceParent to column destinationChild under parent destinationParent.

Returns true if the columns were successfully moved; otherwise returns false.

The base class implementation does nothing and returns false.

If you implement your own model, you can reimplement this function if you want to support moving. Alternatively, you can provide your own API for altering the data.

See also beginMoveColumns() and endMoveColumns().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn moveColumns_0<RetType, T: QAbstractItemModel_moveColumns_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveColumns_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_moveColumns_0<RetType> {
  fn moveColumns_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_moveColumns_0<bool> for (usize,i32,i32,usize,i32) {
  fn moveColumns_0(self , rsthis: & QAbstractItemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QAbstractItemModel11moveColumnsERK11QModelIndexiiS2_i", 5,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:221
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool insertRow(int, const QModelIndex &)

/*
Inserts a single row before the given row in the child items of the parent specified.

Note: This function calls the virtual method insertRows.

Returns true if the row is inserted; otherwise returns false.

See also insertRows(), insertColumn(), and removeRow().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn insertRow_0<RetType, T: QAbstractItemModel_insertRow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertRow_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_insertRow_0<RetType> {
  fn insertRow_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_insertRow_0<bool> for (i32,usize) {
  fn insertRow_0(self , rsthis: & QAbstractItemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QAbstractItemModel9insertRowEiRK11QModelIndex", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:222
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool insertColumn(int, const QModelIndex &)

/*
Inserts a single column before the given column in the child items of the parent specified.

Returns true if the column is inserted; otherwise returns false.

See also insertColumns(), insertRow(), and removeColumn().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn insertColumn_0<RetType, T: QAbstractItemModel_insertColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertColumn_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_insertColumn_0<RetType> {
  fn insertColumn_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_insertColumn_0<bool> for (i32,usize) {
  fn insertColumn_0(self , rsthis: & QAbstractItemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QAbstractItemModel12insertColumnEiRK11QModelIndex", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:223
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool removeRow(int, const QModelIndex &)

/*
Removes the given row from the child items of the parent specified.

Returns true if the row is removed; otherwise returns false.

This is a convenience function that calls removeRows(). The QAbstractItemModel implementation of removeRows() does nothing.

See also removeRows(), removeColumn(), and insertRow().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn removeRow_0<RetType, T: QAbstractItemModel_removeRow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeRow_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_removeRow_0<RetType> {
  fn removeRow_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_removeRow_0<bool> for (i32,usize) {
  fn removeRow_0(self , rsthis: & QAbstractItemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QAbstractItemModel9removeRowEiRK11QModelIndex", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:224
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool removeColumn(int, const QModelIndex &)

/*
Removes the given column from the child items of the parent specified.

Returns true if the column is removed; otherwise returns false.

See also removeColumns(), removeRow(), and insertColumn().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn removeColumn_0<RetType, T: QAbstractItemModel_removeColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeColumn_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_removeColumn_0<RetType> {
  fn removeColumn_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_removeColumn_0<bool> for (i32,usize) {
  fn removeColumn_0(self , rsthis: & QAbstractItemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QAbstractItemModel12removeColumnEiRK11QModelIndex", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:225
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool moveRow(const QModelIndex &, int, const QModelIndex &, int)

/*
On models that support this, moves sourceRow from sourceParent to destinationChild under destinationParent.

Returns true if the rows were successfully moved; otherwise returns false.

See also moveRows() and moveColumn().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn moveRow_0<RetType, T: QAbstractItemModel_moveRow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveRow_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_moveRow_0<RetType> {
  fn moveRow_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_moveRow_0<bool> for (usize,i32,usize,i32) {
  fn moveRow_0(self , rsthis: & QAbstractItemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QAbstractItemModel7moveRowERK11QModelIndexiS2_i", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:227
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool moveColumn(const QModelIndex &, int, const QModelIndex &, int)

/*
On models that support this, moves sourceColumn from sourceParent to destinationChild under destinationParent.

Returns true if the columns were successfully moved; otherwise returns false.

See also moveColumns() and moveRow().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn moveColumn_0<RetType, T: QAbstractItemModel_moveColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveColumn_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_moveColumn_0<RetType> {
  fn moveColumn_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_moveColumn_0<bool> for (usize,i32,usize,i32) {
  fn moveColumn_0(self , rsthis: & QAbstractItemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QAbstractItemModel10moveColumnERK11QModelIndexiS2_i", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:230
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void fetchMore(const QModelIndex &)

/*
Fetches any available data for the items with the parent specified by the parent index.

Reimplement this if you are populating your model incrementally.

The default implementation does nothing.

See also canFetchMore().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn fetchMore_0<RetType, T: QAbstractItemModel_fetchMore_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fetchMore_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_fetchMore_0<RetType> {
  fn fetchMore_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_fetchMore_0<(/*void*/)> for (usize) {
  fn fetchMore_0(self , rsthis: & QAbstractItemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QAbstractItemModel9fetchMoreERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:231
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool canFetchMore(const QModelIndex &) const

/*
Returns true if there is more data available for parent; otherwise returns false.

The default implementation always returns false.

If canFetchMore() returns true, the fetchMore() function should be called. This is the behavior of QAbstractItemView, for example.

See also fetchMore().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn canFetchMore_0<RetType, T: QAbstractItemModel_canFetchMore_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.canFetchMore_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_canFetchMore_0<RetType> {
  fn canFetchMore_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_canFetchMore_0<bool> for (usize) {
  fn canFetchMore_0(self , rsthis: & QAbstractItemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QAbstractItemModel12canFetchMoreERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:232
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] Qt::ItemFlags flags(const QModelIndex &) const

/*
Returns the item flags for the given index.

The base class implementation returns a combination of flags that enables the item (ItemIsEnabled) and allows it to be selected (ItemIsSelectable).

See also Qt::ItemFlags.
*/
impl /*struct*/ QAbstractItemModel {
  pub fn flags_0<RetType, T: QAbstractItemModel_flags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.flags_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_flags_0<RetType> {
  fn flags_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_flags_0<i32> for (usize) {
  fn flags_0(self , rsthis: & QAbstractItemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QAbstractItemModel5flagsERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:233
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void sort(int, Qt::SortOrder)

/*
Sorts the model by column in the given order.

The base class implementation does nothing.
*/
impl /*struct*/ QAbstractItemModel {
  pub fn sort_0<RetType, T: QAbstractItemModel_sort_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sort_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_sort_0<RetType> {
  fn sort_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_sort_0<(/*void*/)> for (i32,i32) {
  fn sort_0(self , rsthis: & QAbstractItemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN18QAbstractItemModel4sortEiN2Qt9SortOrderE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:234
// index:0
// Public virtual Visibility=Default Availability=Available
// [24] QModelIndex buddy(const QModelIndex &) const

/*
Returns a model index for the buddy of the item represented by index. When the user wants to edit an item, the view will call this function to check whether another item in the model should be edited instead. Then, the view will construct a delegate using the model index returned by the buddy item.

The default implementation of this function has each item as its own buddy.
*/
impl /*struct*/ QAbstractItemModel {
  pub fn buddy_0<RetType, T: QAbstractItemModel_buddy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.buddy_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_buddy_0<RetType> {
  fn buddy_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_buddy_0<usize> for (usize) {
  fn buddy_0(self , rsthis: & QAbstractItemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QAbstractItemModel5buddyERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:235
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QModelIndexList match(const QModelIndex &, int, const QVariant &, int, Qt::MatchFlags) const

/*
Returns a list of indexes for the items in the column of the start index where data stored under the given role matches the specified value. The way the search is performed is defined by the flags given. The list that is returned may be empty. Note also that the order of results in the list may not correspond to the order in the model, if for example a proxy model is used. The order of the results can not be relied upon.

The search begins from the start index, and continues until the number of matching data items equals hits, the search reaches the last row, or the search reaches start again - depending on whether MatchWrap is specified in flags. If you want to search for all matching items, use hits = -1.

By default, this function will perform a wrapping, string-based comparison on all items, searching for items that begin with the search term specified by value.

Note: The default implementation of this function only searches columns. Reimplement this function to include a different search behavior.
*/
impl /*struct*/ QAbstractItemModel {
  pub fn match__0<RetType, T: QAbstractItemModel_match__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.match__0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_match__0<RetType> {
  fn match__0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_match__0<usize> for (usize,i32,usize,i32,i32) {
  fn match__0(self , rsthis: & QAbstractItemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QAbstractItemModel5matchERK11QModelIndexiRK8QVarianti6QFlagsIN2Qt9MatchFlagEE", 5,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:239
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize span(const QModelIndex &) const

/*
Returns the row and column span of the item represented by index.

Note: Currently, span is not used.
*/
impl /*struct*/ QAbstractItemModel {
  pub fn span_0<RetType, T: QAbstractItemModel_span_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.span_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_span_0<RetType> {
  fn span_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_span_0<usize> for (usize) {
  fn span_0(self , rsthis: & QAbstractItemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QAbstractItemModel4spanERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:255
// index:0
// Public Visibility=Default Availability=Available
// [-2] void headerDataChanged(Qt::Orientation, int, int)

/*
This signal is emitted whenever a header is changed. The orientation indicates whether the horizontal or vertical header has changed. The sections in the header from the first to the last need to be updated.

When reimplementing the setHeaderData() function, this signal must be emitted explicitly.

If you are changing the number of columns or rows you do not need to emit this signal, but use the begin/end functions (refer to the section on subclassing in the QAbstractItemModel class description for details).

See also headerData(), setHeaderData(), and dataChanged().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn headerDataChanged_0<RetType, T: QAbstractItemModel_headerDataChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.headerDataChanged_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_headerDataChanged_0<RetType> {
  fn headerDataChanged_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_headerDataChanged_0<(/*void*/)> for (i32,i32,i32) {
  fn headerDataChanged_0(self , rsthis: & QAbstractItemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN18QAbstractItemModel17headerDataChangedEN2Qt11OrientationEii", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:281
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool submit()

/*
Lets the model know that it should submit cached information to permanent storage. This function is typically used for row editing.

Returns true if there is no error; otherwise returns false.

See also revert().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn submit_0<RetType, T: QAbstractItemModel_submit_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.submit_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_submit_0<RetType> {
  fn submit_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_submit_0<bool> for () {
  fn submit_0(self , rsthis: & QAbstractItemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QAbstractItemModel6submitEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:282
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void revert()

/*
Lets the model know that it should discard cached information. This function is typically used for row editing.

See also submit().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn revert_0<RetType, T: QAbstractItemModel_revert_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.revert_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_revert_0<RetType> {
  fn revert_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_revert_0<(/*void*/)> for () {
  fn revert_0(self , rsthis: & QAbstractItemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN18QAbstractItemModel6revertEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:286
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void resetInternalData()

/*
This slot is called just after the internal data of a model is cleared while it is being reset.

This slot is provided the convenience of subclasses of concrete proxy models, such as subclasses of QSortFilterProxyModel which maintain extra data.


  class CustomDataProxy : public QSortFilterProxyModel
  {
      Q_OBJECT
  public:
      CustomDataProxy(QObject *parent)
        : QSortFilterProxyModel(parent)
      {
      }

      ...

      QVariant data(const QModelIndex &index, int role)
      {
          if (role != Qt::BackgroundRole)
              return QSortFilterProxyModel::data(index, role);

          if (m_customData.contains(index.row()))
              return m_customData.value(index.row());
          return QSortFilterProxyModel::data(index, role);
      }

  private slots:
      void resetInternalData()
      {
          m_customData.clear();
      }

  private:
    QHash<int, QVariant> m_customData;
  };



Note: Due to a mistake, this slot is missing in Qt 5.0.

This function was introduced in  Qt 4.8.

See also modelAboutToBeReset() and modelReset().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn resetInternalData_0<RetType, T: QAbstractItemModel_resetInternalData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resetInternalData_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_resetInternalData_0<RetType> {
  fn resetInternalData_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_resetInternalData_0<(/*void*/)> for () {
  fn resetInternalData_0(self , rsthis: & QAbstractItemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN18QAbstractItemModel17resetInternalDataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:291
// index:0
// Protected inline Visibility=Default Availability=Available
// [24] QModelIndex createIndex(int, int, void *) const

/*
Creates a model index for the given row and column with the internal pointer ptr.

When using a QSortFilterProxyModel, its indexes have their own internal pointer. It is not advisable to access this internal pointer outside of the model. Use the data() function instead.

This function provides a consistent interface that model subclasses must use to create model indexes.
*/
impl /*struct*/ QAbstractItemModel {
  pub fn createIndex_0<RetType, T: QAbstractItemModel_createIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.createIndex_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_createIndex_0<RetType> {
  fn createIndex_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_createIndex_0<usize> for (i32,i32,usize) {
  fn createIndex_0(self , rsthis: & QAbstractItemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QAbstractItemModel11createIndexEiiPv", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:292
// index:1
// Protected inline Visibility=Default Availability=Available
// [24] QModelIndex createIndex(int, int, quintptr) const

/*
Creates a model index for the given row and column with the internal pointer ptr.

When using a QSortFilterProxyModel, its indexes have their own internal pointer. It is not advisable to access this internal pointer outside of the model. Use the data() function instead.

This function provides a consistent interface that model subclasses must use to create model indexes.
*/
impl /*struct*/ QAbstractItemModel {
  pub fn createIndex_1<RetType, T: QAbstractItemModel_createIndex_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.createIndex_1(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_createIndex_1<RetType> {
  fn createIndex_1(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_createIndex_1<usize> for (i32,i32,u64) {
  fn createIndex_1(self , rsthis: & QAbstractItemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const u64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QAbstractItemModel11createIndexEiiy", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_UINT64,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:295
// index:0
// Protected Visibility=Default Availability=Available
// [1] bool decodeData(int, int, const QModelIndex &, QDataStream &)

/*

*/
impl /*struct*/ QAbstractItemModel {
  pub fn decodeData_0<RetType, T: QAbstractItemModel_decodeData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.decodeData_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_decodeData_0<RetType> {
  fn decodeData_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_decodeData_0<bool> for (i32,i32,usize,usize) {
  fn decodeData_0(self , rsthis: & QAbstractItemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QAbstractItemModel10decodeDataEiiRK11QModelIndexR11QDataStream", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:297
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void beginInsertRows(const QModelIndex &, int, int)

/*
Begins a row insertion operation.

When reimplementing insertRows() in a subclass, you must call this function before inserting data into the model's underlying data store.

The parent index corresponds to the parent into which the new rows are inserted; first and last are the row numbers that the new rows will have after they have been inserted.


 Specify the first and last row numbers for the span of rows you want to insert into an item in a model.For example, as shown in the diagram, we insert three rows before row 2, so first is 2 and last is 4:

  beginInsertRows(parent, 2, 4);


This inserts the three new rows as rows 2, 3, and 4.

To append rows, insert them after the last row.For example, as shown in the diagram, we append two rows to a collection of 4 existing rows (ending in row 3), so first is 4 and last is 5:

  beginInsertRows(parent, 4, 5);


This appends the two new rows as rows 4 and 5.



Note: This function emits the rowsAboutToBeInserted() signal which connected views (or proxies) must handle before the data is inserted. Otherwise, the views may end up in an invalid state.

See also endInsertRows().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn beginInsertRows_0<RetType, T: QAbstractItemModel_beginInsertRows_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.beginInsertRows_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_beginInsertRows_0<RetType> {
  fn beginInsertRows_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_beginInsertRows_0<(/*void*/)> for (usize,i32,i32) {
  fn beginInsertRows_0(self , rsthis: & QAbstractItemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN18QAbstractItemModel15beginInsertRowsERK11QModelIndexii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:298
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void endInsertRows()

/*
Ends a row insertion operation.

When reimplementing insertRows() in a subclass, you must call this function after inserting data into the model's underlying data store.

See also beginInsertRows().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn endInsertRows_0<RetType, T: QAbstractItemModel_endInsertRows_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.endInsertRows_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_endInsertRows_0<RetType> {
  fn endInsertRows_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_endInsertRows_0<(/*void*/)> for () {
  fn endInsertRows_0(self , rsthis: & QAbstractItemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN18QAbstractItemModel13endInsertRowsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:300
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void beginRemoveRows(const QModelIndex &, int, int)

/*
Begins a row removal operation.

When reimplementing removeRows() in a subclass, you must call this function before removing data from the model's underlying data store.

The parent index corresponds to the parent from which the new rows are removed; first and last are the row numbers of the rows to be removed.


 Specify the first and last row numbers for the span of rows you want to remove from an item in a model.For example, as shown in the diagram, we remove the two rows from row 2 to row 3, so first is 2 and last is 3:

  beginRemoveRows(parent, 2, 3);





Note: This function emits the rowsAboutToBeRemoved() signal which connected views (or proxies) must handle before the data is removed. Otherwise, the views may end up in an invalid state.

See also endRemoveRows().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn beginRemoveRows_0<RetType, T: QAbstractItemModel_beginRemoveRows_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.beginRemoveRows_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_beginRemoveRows_0<RetType> {
  fn beginRemoveRows_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_beginRemoveRows_0<(/*void*/)> for (usize,i32,i32) {
  fn beginRemoveRows_0(self , rsthis: & QAbstractItemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN18QAbstractItemModel15beginRemoveRowsERK11QModelIndexii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:301
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void endRemoveRows()

/*
Ends a row removal operation.

When reimplementing removeRows() in a subclass, you must call this function after removing data from the model's underlying data store.

See also beginRemoveRows().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn endRemoveRows_0<RetType, T: QAbstractItemModel_endRemoveRows_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.endRemoveRows_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_endRemoveRows_0<RetType> {
  fn endRemoveRows_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_endRemoveRows_0<(/*void*/)> for () {
  fn endRemoveRows_0(self , rsthis: & QAbstractItemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN18QAbstractItemModel13endRemoveRowsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:303
// index:0
// Protected Visibility=Default Availability=Available
// [1] bool beginMoveRows(const QModelIndex &, int, int, const QModelIndex &, int)

/*
Begins a row move operation.

When reimplementing a subclass, this method simplifies moving entities in your model. This method is responsible for moving persistent indexes in the model, which you would otherwise be required to do yourself. Using beginMoveRows and endMoveRows is an alternative to emitting layoutAboutToBeChanged and layoutChanged directly along with changePersistentIndex.

The sourceParent index corresponds to the parent from which the rows are moved; sourceFirst and sourceLast are the first and last row numbers of the rows to be moved. The destinationParent index corresponds to the parent into which those rows are moved. The destinationChild is the row to which the rows will be moved. That is, the index at row sourceFirst in sourceParent will become row destinationChild in destinationParent, followed by all other rows up to sourceLast.

However, when moving rows down in the same parent (sourceParent and destinationParent are equal), the rows will be placed before the destinationChild index. That is, if you wish to move rows 0 and 1 so they will become rows 1 and 2, destinationChild should be 3. In this case, the new index for the source row i (which is between sourceFirst and sourceLast) is equal to (destinationChild-sourceLast-1+i).

Note that if sourceParent and destinationParent are the same, you must ensure that the destinationChild is not within the range of sourceFirst and sourceLast + 1. You must also ensure that you do not attempt to move a row to one of its own children or ancestors. This method returns false if either condition is true, in which case you should abort your move operation.


 Specify the first and last row numbers for the span of rows in the source parent you want to move in the model. Also specify the row in the destination parent to move the span to.For example, as shown in the diagram, we move three rows from row 2 to 4 in the source, so sourceFirst is 2 and sourceLast is 4. We move those items to above row 2 in the destination, so destinationChild is 2.

  beginMoveRows(sourceParent, 2, 4, destinationParent, 2);


This moves the three rows rows 2, 3, and 4 in the source to become 2, 3 and 4 in the destination. Other affected siblings are displaced accordingly.

To append rows to another parent, move them to after the last row.For example, as shown in the diagram, we move three rows to a collection of 6 existing rows (ending in row 5), so destinationChild is 6:

  beginMoveRows(sourceParent, 2, 4, destinationParent, 6);


This moves the target rows to the end of the target parent as 6, 7 and 8.

To move rows within the same parent, specify the row to move them to.For example, as shown in the diagram, we move one item from row 2 to row 0, so sourceFirst and sourceLast are 2 and destinationChild is 0.

  beginMoveRows(parent, 2, 2, parent, 0);


Note that other rows may be displaced accordingly. Note also that when moving items within the same parent you should not attempt invalid or no-op moves. In the above example, item 2 is at row 2 before the move, so it can not be moved to row 2 (where it is already) or row 3 (no-op as row 3 means above row 3, where it is already)

To move rows within the same parent, specify the row to move them to.For example, as shown in the diagram, we move one item from row 2 to row 4, so sourceFirst and sourceLast are 2 and destinationChild is 4.

  beginMoveRows(parent, 2, 2, parent, 4);


Note that other rows may be displaced accordingly.



This function was introduced in  Qt 4.6.

See also endMoveRows().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn beginMoveRows_0<RetType, T: QAbstractItemModel_beginMoveRows_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.beginMoveRows_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_beginMoveRows_0<RetType> {
  fn beginMoveRows_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_beginMoveRows_0<bool> for (usize,i32,i32,usize,i32) {
  fn beginMoveRows_0(self , rsthis: & QAbstractItemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QAbstractItemModel13beginMoveRowsERK11QModelIndexiiS2_i", 5,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:304
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void endMoveRows()

/*
Ends a row move operation.

When implementing a subclass, you must call this function after moving data within the model's underlying data store.

This function was introduced in  Qt 4.6.

See also beginMoveRows().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn endMoveRows_0<RetType, T: QAbstractItemModel_endMoveRows_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.endMoveRows_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_endMoveRows_0<RetType> {
  fn endMoveRows_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_endMoveRows_0<(/*void*/)> for () {
  fn endMoveRows_0(self , rsthis: & QAbstractItemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN18QAbstractItemModel11endMoveRowsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:306
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void beginInsertColumns(const QModelIndex &, int, int)

/*
Begins a column insertion operation.

When reimplementing insertColumns() in a subclass, you must call this function before inserting data into the model's underlying data store.

The parent index corresponds to the parent into which the new columns are inserted; first and last are the column numbers of the new columns will have after they have been inserted.


 Specify the first and last column numbers for the span of columns you want to insert into an item in a model.For example, as shown in the diagram, we insert three columns before column 4, so first is 4 and last is 6:

  beginInsertColumns(parent, 4, 6);


This inserts the three new columns as columns 4, 5, and 6.

To append columns, insert them after the last column.For example, as shown in the diagram, we append three columns to a collection of six existing columns (ending in column 5), so first is 6 and last is 8:

  beginInsertColumns(parent, 6, 8);


This appends the two new columns as columns 6, 7, and 8.



Note: This function emits the columnsAboutToBeInserted() signal which connected views (or proxies) must handle before the data is inserted. Otherwise, the views may end up in an invalid state.

See also endInsertColumns().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn beginInsertColumns_0<RetType, T: QAbstractItemModel_beginInsertColumns_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.beginInsertColumns_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_beginInsertColumns_0<RetType> {
  fn beginInsertColumns_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_beginInsertColumns_0<(/*void*/)> for (usize,i32,i32) {
  fn beginInsertColumns_0(self , rsthis: & QAbstractItemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN18QAbstractItemModel18beginInsertColumnsERK11QModelIndexii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:307
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void endInsertColumns()

/*
Ends a column insertion operation.

When reimplementing insertColumns() in a subclass, you must call this function after inserting data into the model's underlying data store.

See also beginInsertColumns().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn endInsertColumns_0<RetType, T: QAbstractItemModel_endInsertColumns_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.endInsertColumns_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_endInsertColumns_0<RetType> {
  fn endInsertColumns_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_endInsertColumns_0<(/*void*/)> for () {
  fn endInsertColumns_0(self , rsthis: & QAbstractItemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN18QAbstractItemModel16endInsertColumnsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:309
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void beginRemoveColumns(const QModelIndex &, int, int)

/*
Begins a column removal operation.

When reimplementing removeColumns() in a subclass, you must call this function before removing data from the model's underlying data store.

The parent index corresponds to the parent from which the new columns are removed; first and last are the column numbers of the first and last columns to be removed.


 Specify the first and last column numbers for the span of columns you want to remove from an item in a model.For example, as shown in the diagram, we remove the three columns from column 4 to column 6, so first is 4 and last is 6:

  beginRemoveColumns(parent, 4, 6);





Note: This function emits the columnsAboutToBeRemoved() signal which connected views (or proxies) must handle before the data is removed. Otherwise, the views may end up in an invalid state.

See also endRemoveColumns().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn beginRemoveColumns_0<RetType, T: QAbstractItemModel_beginRemoveColumns_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.beginRemoveColumns_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_beginRemoveColumns_0<RetType> {
  fn beginRemoveColumns_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_beginRemoveColumns_0<(/*void*/)> for (usize,i32,i32) {
  fn beginRemoveColumns_0(self , rsthis: & QAbstractItemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN18QAbstractItemModel18beginRemoveColumnsERK11QModelIndexii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:310
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void endRemoveColumns()

/*
Ends a column removal operation.

When reimplementing removeColumns() in a subclass, you must call this function after removing data from the model's underlying data store.

See also beginRemoveColumns().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn endRemoveColumns_0<RetType, T: QAbstractItemModel_endRemoveColumns_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.endRemoveColumns_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_endRemoveColumns_0<RetType> {
  fn endRemoveColumns_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_endRemoveColumns_0<(/*void*/)> for () {
  fn endRemoveColumns_0(self , rsthis: & QAbstractItemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN18QAbstractItemModel16endRemoveColumnsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:312
// index:0
// Protected Visibility=Default Availability=Available
// [1] bool beginMoveColumns(const QModelIndex &, int, int, const QModelIndex &, int)

/*
Begins a column move operation.

When reimplementing a subclass, this method simplifies moving entities in your model. This method is responsible for moving persistent indexes in the model, which you would otherwise be required to do yourself. Using beginMoveColumns and endMoveColumns is an alternative to emitting layoutAboutToBeChanged and layoutChanged directly along with changePersistentIndex.

The sourceParent index corresponds to the parent from which the columns are moved; sourceFirst and sourceLast are the first and last column numbers of the columns to be moved. The destinationParent index corresponds to the parent into which those columns are moved. The destinationChild is the column to which the columns will be moved. That is, the index at column sourceFirst in sourceParent will become column destinationChild in destinationParent, followed by all other columns up to sourceLast.

However, when moving columns down in the same parent (sourceParent and destinationParent are equal), the columns will be placed before the destinationChild index. That is, if you wish to move columns 0 and 1 so they will become columns 1 and 2, destinationChild should be 3. In this case, the new index for the source column i (which is between sourceFirst and sourceLast) is equal to (destinationChild-sourceLast-1+i).

Note that if sourceParent and destinationParent are the same, you must ensure that the destinationChild is not within the range of sourceFirst and sourceLast + 1. You must also ensure that you do not attempt to move a column to one of its own children or ancestors. This method returns false if either condition is true, in which case you should abort your move operation.

This function was introduced in  Qt 4.6.

See also endMoveColumns().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn beginMoveColumns_0<RetType, T: QAbstractItemModel_beginMoveColumns_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.beginMoveColumns_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_beginMoveColumns_0<RetType> {
  fn beginMoveColumns_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_beginMoveColumns_0<bool> for (usize,i32,i32,usize,i32) {
  fn beginMoveColumns_0(self , rsthis: & QAbstractItemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QAbstractItemModel16beginMoveColumnsERK11QModelIndexiiS2_i", 5,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:313
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void endMoveColumns()

/*
Ends a column move operation.

When implementing a subclass, you must call this function after moving data within the model's underlying data store.

This function was introduced in  Qt 4.6.

See also beginMoveColumns().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn endMoveColumns_0<RetType, T: QAbstractItemModel_endMoveColumns_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.endMoveColumns_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_endMoveColumns_0<RetType> {
  fn endMoveColumns_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_endMoveColumns_0<(/*void*/)> for () {
  fn endMoveColumns_0(self , rsthis: & QAbstractItemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN18QAbstractItemModel14endMoveColumnsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:324
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void beginResetModel()

/*
Begins a model reset operation.

A reset operation resets the model to its current state in any attached views.

Note: Any views attached to this model will be reset as well.

When a model is reset it means that any previous data reported from the model is now invalid and has to be queried for again. This also means that the current item and any selected items will become invalid.

When a model radically changes its data it can sometimes be easier to just call this function rather than emit dataChanged() to inform other components when the underlying data source, or its structure, has changed.

You must call this function before resetting any internal data structures in your model or proxy model.

This function emits the signal modelAboutToBeReset().

This function was introduced in  Qt 4.6.

See also modelAboutToBeReset(), modelReset(), and endResetModel().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn beginResetModel_0<RetType, T: QAbstractItemModel_beginResetModel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.beginResetModel_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_beginResetModel_0<RetType> {
  fn beginResetModel_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_beginResetModel_0<(/*void*/)> for () {
  fn beginResetModel_0(self , rsthis: & QAbstractItemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN18QAbstractItemModel15beginResetModelEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:325
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void endResetModel()

/*
Completes a model reset operation.

You must call this function after resetting any internal data structure in your model or proxy model.

This function emits the signal modelReset().

This function was introduced in  Qt 4.6.

See also beginResetModel().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn endResetModel_0<RetType, T: QAbstractItemModel_endResetModel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.endResetModel_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_endResetModel_0<RetType> {
  fn endResetModel_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_endResetModel_0<(/*void*/)> for () {
  fn endResetModel_0(self , rsthis: & QAbstractItemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN18QAbstractItemModel13endResetModelEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:327
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void changePersistentIndex(const QModelIndex &, const QModelIndex &)

/*
Changes the QPersistentModelIndex that is equal to the given from model index to the given to model index.

If no persistent model index equal to the given from model index was found, nothing is changed.

See also persistentIndexList() and changePersistentIndexList().
*/
impl /*struct*/ QAbstractItemModel {
  pub fn changePersistentIndex_0<RetType, T: QAbstractItemModel_changePersistentIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changePersistentIndex_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_changePersistentIndex_0<RetType> {
  fn changePersistentIndex_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_changePersistentIndex_0<(/*void*/)> for (usize,usize) {
  fn changePersistentIndex_0(self , rsthis: & QAbstractItemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QAbstractItemModel21changePersistentIndexERK11QModelIndexS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:329
// index:0
// Protected Visibility=Default Availability=Available
// [8] QModelIndexList persistentIndexList() const

/*
Returns the list of indexes stored as persistent indexes in the model.

This function was introduced in  Qt 4.2.
*/
impl /*struct*/ QAbstractItemModel {
  pub fn persistentIndexList_0<RetType, T: QAbstractItemModel_persistentIndexList_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.persistentIndexList_0(self);
    // return 1;
  }
}
pub trait QAbstractItemModel_persistentIndexList_0<RetType> {
  fn persistentIndexList_0(self , rsthis: & QAbstractItemModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemModel_persistentIndexList_0<usize> for () {
  fn persistentIndexList_0(self , rsthis: & QAbstractItemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QAbstractItemModel19persistentIndexListEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


/*
This enum describes the way the model changes layout.



Note that VerticalSortHint and HorizontalSortHint carry the meaning that items are being moved within the same parent, not moved to a different parent in the model, and not filtered out or in.

*/
pub type QAbstractItemModel__LayoutChangeHint = i32;
// No hint is available.
pub const QAbstractItemModel__NoLayoutChangeHint :QAbstractItemModel__LayoutChangeHint = 0;
// Rows are being sorted.
pub const QAbstractItemModel__VerticalSortHint :QAbstractItemModel__LayoutChangeHint = 1;
// Columns are being sorted.
pub const QAbstractItemModel__HorizontalSortHint :QAbstractItemModel__LayoutChangeHint = 2;
pub fn QAbstractItemModel_LayoutChangeHintItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QAbstractItemModel", val);
}
pub fn QAbstractItemModel_LayoutChangeHintItemName_s(val: i32) ->String {
  //var nilthis *QAbstractItemModel
  //return nilthis.LayoutChangeHintItemName(val);
  return QAbstractItemModel_LayoutChangeHintItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
