

// mod ::core::QStringListModel
// package qtcore
// /usr/include/qt/QtCore/qstringlistmodel.h
// #include <qstringlistmodel.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 3
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
#[derive(Default)] // class sizeof(QStringListModel)=24
pub struct QStringListModel {
  // qbase: QAbstractListModel,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStringListModel_ITF interface {
//    QAbstractListModel_ITF
//    QStringListModel_PTR() *QStringListModel
//}
//func (ptr *QStringListModel) QStringListModel_PTR() *QStringListModel { return ptr }

impl /*struct*/ QStringListModel {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStringListModel {
    return QStringListModel{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStringListModel {
//  type Target = QStringListModelBASE;
//
//  fn deref(&self) -> &QStringListModelBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStringListModelBASE> for QStringListModel {
//  fn as_ref(& self) -> & QStringListModelBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qstringlistmodel.h:53
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QStringListModel {
  pub fn metaObject_0<RetType, T: QStringListModel_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QStringListModel_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QStringListModel) -> RetType;
}
impl<'a> /*trait*/ QStringListModel_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QStringListModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QStringListModel10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringlistmodel.h:55
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStringListModel(QObject *)

/*
Constructs a string list model with the given parent.
*/
// QStringListModel(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QStringListModel {
  pub fn QStringListModel_0<T: QStringListModel_QStringListModel_0>(value: T) -> QStringListModel {
    let rsthis = value.QStringListModel_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStringListModel_QStringListModel_0 {
  fn QStringListModel_0(self) -> QStringListModel;
}
// QStringListModel(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStringListModel_QStringListModel_0 for (usize) {
  fn QStringListModel_0(self) -> QStringListModel {
    // unsafe{_ZN16QStringListModelC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QStringListModelC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStringListModel{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstringlistmodel.h:56
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QStringListModel(const QStringList &, QObject *)

/*
Constructs a string list model with the given parent.
*/
// QStringListModel(const QStringList &, QObject *) ctx.fn_proto_cpp
impl /*struct*/ QStringListModel {
  pub fn QStringListModel_1<T: QStringListModel_QStringListModel_1>(value: T) -> QStringListModel {
    let rsthis = value.QStringListModel_1();
    return rsthis;
    // return 1;
  }
}

pub trait QStringListModel_QStringListModel_1 {
  fn QStringListModel_1(self) -> QStringListModel;
}
// QStringListModel(const QStringList &, QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStringListModel_QStringListModel_1 for (usize,usize) {
  fn QStringListModel_1(self) -> QStringListModel {
    // unsafe{_ZN16QStringListModelC2ERK11QStringListP7QObject()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QStringListModelC2ERK11QStringListP7QObject", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStringListModel{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstringlistmodel.h:58
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int rowCount(const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::rowCount().

Returns the number of rows in the model. This value corresponds to the number of items in the model's internal string list.

The optional parent argument is in most models used to specify the parent of the rows to be counted. Because this is a list if a valid parent is specified, the result will always be 0.

See also insertRows(), removeRows(), and QAbstractItemModel::rowCount().
*/
impl /*struct*/ QStringListModel {
  pub fn rowCount_0<RetType, T: QStringListModel_rowCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowCount_0(self);
    // return 1;
  }
}
pub trait QStringListModel_rowCount_0<RetType> {
  fn rowCount_0(self , rsthis: & QStringListModel) -> RetType;
}
impl<'a> /*trait*/ QStringListModel_rowCount_0<i32> for (usize) {
  fn rowCount_0(self , rsthis: & QStringListModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QStringListModel8rowCountERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringlistmodel.h:59
// index:0
// Public virtual Visibility=Default Availability=Available
// [24] QModelIndex sibling(int, int, const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::sibling().
*/
impl /*struct*/ QStringListModel {
  pub fn sibling_0<RetType, T: QStringListModel_sibling_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sibling_0(self);
    // return 1;
  }
}
pub trait QStringListModel_sibling_0<RetType> {
  fn sibling_0(self , rsthis: & QStringListModel) -> RetType;
}
impl<'a> /*trait*/ QStringListModel_sibling_0<usize> for (i32,i32,usize) {
  fn sibling_0(self , rsthis: & QStringListModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QStringListModel7siblingEiiRK11QModelIndex", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringlistmodel.h:61
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QVariant data(const QModelIndex &, int) const

/*
Reimplemented from QAbstractItemModel::data().

Returns data for the specified role, from the item with the given index.

If the view requests an invalid index, an invalid variant is returned.

See also setData().
*/
impl /*struct*/ QStringListModel {
  pub fn data_0<RetType, T: QStringListModel_data_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.data_0(self);
    // return 1;
  }
}
pub trait QStringListModel_data_0<RetType> {
  fn data_0(self , rsthis: & QStringListModel) -> RetType;
}
impl<'a> /*trait*/ QStringListModel_data_0<usize> for (usize,i32) {
  fn data_0(self , rsthis: & QStringListModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QStringListModel4dataERK11QModelIndexi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringlistmodel.h:62
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool setData(const QModelIndex &, const QVariant &, int)

/*
Reimplemented from QAbstractItemModel::setData().

Sets the data for the specified role in the item with the given index in the model, to the provided value.

The dataChanged() signal is emitted if the item is changed.

See also Qt::ItemDataRole and data().
*/
impl /*struct*/ QStringListModel {
  pub fn setData_0<RetType, T: QStringListModel_setData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setData_0(self);
    // return 1;
  }
}
pub trait QStringListModel_setData_0<RetType> {
  fn setData_0(self , rsthis: & QStringListModel) -> RetType;
}
impl<'a> /*trait*/ QStringListModel_setData_0<bool> for (usize,usize,i32) {
  fn setData_0(self , rsthis: & QStringListModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QStringListModel7setDataERK11QModelIndexRK8QVarianti", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringlistmodel.h:64
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] Qt::ItemFlags flags(const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::flags().

Returns the flags for the item with the given index.

Valid items are enabled, selectable, editable, drag enabled and drop enabled.

See also QAbstractItemModel::flags().
*/
impl /*struct*/ QStringListModel {
  pub fn flags_0<RetType, T: QStringListModel_flags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.flags_0(self);
    // return 1;
  }
}
pub trait QStringListModel_flags_0<RetType> {
  fn flags_0(self , rsthis: & QStringListModel) -> RetType;
}
impl<'a> /*trait*/ QStringListModel_flags_0<i32> for (usize) {
  fn flags_0(self , rsthis: & QStringListModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QStringListModel5flagsERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringlistmodel.h:66
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool insertRows(int, int, const QModelIndex &)

/*
Reimplemented from QAbstractItemModel::insertRows().

Inserts count rows into the model, beginning at the given row.

The parent index of the rows is optional and is only used for consistency with QAbstractItemModel. By default, a null index is specified, indicating that the rows are inserted in the top level of the model.

See also QAbstractItemModel::insertRows().
*/
impl /*struct*/ QStringListModel {
  pub fn insertRows_0<RetType, T: QStringListModel_insertRows_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertRows_0(self);
    // return 1;
  }
}
pub trait QStringListModel_insertRows_0<RetType> {
  fn insertRows_0(self , rsthis: & QStringListModel) -> RetType;
}
impl<'a> /*trait*/ QStringListModel_insertRows_0<bool> for (i32,i32,usize) {
  fn insertRows_0(self , rsthis: & QStringListModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QStringListModel10insertRowsEiiRK11QModelIndex", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringlistmodel.h:67
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool removeRows(int, int, const QModelIndex &)

/*
Reimplemented from QAbstractItemModel::removeRows().

Removes count rows from the model, beginning at the given row.

The parent index of the rows is optional and is only used for consistency with QAbstractItemModel. By default, a null index is specified, indicating that the rows are removed in the top level of the model.

See also QAbstractItemModel::removeRows().
*/
impl /*struct*/ QStringListModel {
  pub fn removeRows_0<RetType, T: QStringListModel_removeRows_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeRows_0(self);
    // return 1;
  }
}
pub trait QStringListModel_removeRows_0<RetType> {
  fn removeRows_0(self , rsthis: & QStringListModel) -> RetType;
}
impl<'a> /*trait*/ QStringListModel_removeRows_0<bool> for (i32,i32,usize) {
  fn removeRows_0(self , rsthis: & QStringListModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QStringListModel10removeRowsEiiRK11QModelIndex", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringlistmodel.h:69
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void sort(int, Qt::SortOrder)

/*
Reimplemented from QAbstractItemModel::sort().
*/
impl /*struct*/ QStringListModel {
  pub fn sort_0<RetType, T: QStringListModel_sort_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sort_0(self);
    // return 1;
  }
}
pub trait QStringListModel_sort_0<RetType> {
  fn sort_0(self , rsthis: & QStringListModel) -> RetType;
}
impl<'a> /*trait*/ QStringListModel_sort_0<(/*void*/)> for (i32,i32) {
  fn sort_0(self , rsthis: & QStringListModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN16QStringListModel4sortEiN2Qt9SortOrderE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstringlistmodel.h:71
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList stringList() const

/*
Returns the string list used by the model to store data.

See also setStringList().
*/
impl /*struct*/ QStringListModel {
  pub fn stringList_0<RetType, T: QStringListModel_stringList_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.stringList_0(self);
    // return 1;
  }
}
pub trait QStringListModel_stringList_0<RetType> {
  fn stringList_0(self , rsthis: & QStringListModel) -> RetType;
}
impl<'a> /*trait*/ QStringListModel_stringList_0<usize> for () {
  fn stringList_0(self , rsthis: & QStringListModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QStringListModel10stringListEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringlistmodel.h:72
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStringList(const QStringList &)

/*
Sets the model's internal string list to strings. The model will notify any attached views that its underlying data has changed.

See also stringList() and dataChanged().
*/
impl /*struct*/ QStringListModel {
  pub fn setStringList_0<RetType, T: QStringListModel_setStringList_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStringList_0(self);
    // return 1;
  }
}
pub trait QStringListModel_setStringList_0<RetType> {
  fn setStringList_0(self , rsthis: & QStringListModel) -> RetType;
}
impl<'a> /*trait*/ QStringListModel_setStringList_0<(/*void*/)> for (usize) {
  fn setStringList_0(self , rsthis: & QStringListModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QStringListModel13setStringListERK11QStringList", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstringlistmodel.h:74
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] Qt::DropActions supportedDropActions() const

/*
Reimplemented from QAbstractItemModel::supportedDropActions().
*/
impl /*struct*/ QStringListModel {
  pub fn supportedDropActions_0<RetType, T: QStringListModel_supportedDropActions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.supportedDropActions_0(self);
    // return 1;
  }
}
pub trait QStringListModel_supportedDropActions_0<RetType> {
  fn supportedDropActions_0(self , rsthis: & QStringListModel) -> RetType;
}
impl<'a> /*trait*/ QStringListModel_supportedDropActions_0<i32> for () {
  fn supportedDropActions_0(self , rsthis: & QStringListModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QStringListModel20supportedDropActionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}


pub fn DeleteQStringListModel(this :*mut QStringListModel) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN16QStringListModelD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
