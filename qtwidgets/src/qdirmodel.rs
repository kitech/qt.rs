

// mod ::widgets::QDirModel
// package qtwidgets
// /usr/include/qt/QtWidgets/qdirmodel.h
// #include <qdirmodel.h>
// #include <QtWidgets>

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
// import "github.com/kitech/qt.go/qtgui"
use qtgui::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QDirModel)=16
pub struct QDirModel {
  qbase: QAbstractItemModel,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QDirModel_ITF interface {
//    qtcore.QAbstractItemModel_ITF
//    QDirModel_PTR() *QDirModel
//}
//func (ptr *QDirModel) QDirModel_PTR() *QDirModel { return ptr }

impl /*struct*/ QDirModel {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QDirModel {
    return QDirModel{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QDirModel {
//  type Target = QDirModelBASE;
//
//  fn deref(&self) -> &QDirModelBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QDirModelBASE> for QDirModel {
//  fn as_ref(& self) -> & QDirModelBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qdirmodel.h:56
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QDirModel {
  pub fn metaObject_0<RetType, T: QDirModel_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QDirModel_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QDirModel) -> RetType;
}
impl<'a> /*trait*/ QDirModel_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QDirModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDirModel10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdirmodel.h:68
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QDirModel(const QStringList &, QDir::Filters, QDir::SortFlags, QObject *)

/*
Constructs a new directory model with the given parent. Only those files matching the nameFilters and the filters are included in the model. The sort order is given by the sort flags.
*/
// QDirModel(const QStringList &, QDir::Filters, QDir::SortFlags, QObject *) ctx.fn_proto_cpp
impl /*struct*/ QDirModel {
  pub fn QDirModel_0<T: QDirModel_QDirModel_0>(value: T) -> QDirModel {
    let rsthis = value.QDirModel_0();
    return rsthis;
    // return 1;
  }
}

pub trait QDirModel_QDirModel_0 {
  fn QDirModel_0(self) -> QDirModel;
}
// QDirModel(const QStringList &, QDir::Filters, QDir::SortFlags, QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDirModel_QDirModel_0 for (usize,i32,i32,usize) {
  fn QDirModel_0(self) -> QDirModel {
    // unsafe{_ZN9QDirModelC2ERK11QStringList6QFlagsIN4QDir6FilterEES3_INS4_8SortFlagEEP7QObject()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QDirModelC2ERK11QStringList6QFlagsIN4QDir6FilterEES3_INS4_8SortFlagEEP7QObject", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDirModel{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdirmodel.h:70
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QDirModel(QObject *)

/*
Constructs a new directory model with the given parent. Only those files matching the nameFilters and the filters are included in the model. The sort order is given by the sort flags.
*/
// QDirModel(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QDirModel {
  pub fn QDirModel_1<T: QDirModel_QDirModel_1>(value: T) -> QDirModel {
    let rsthis = value.QDirModel_1();
    return rsthis;
    // return 1;
  }
}

pub trait QDirModel_QDirModel_1 {
  fn QDirModel_1(self) -> QDirModel;
}
// QDirModel(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDirModel_QDirModel_1 for (usize) {
  fn QDirModel_1(self) -> QDirModel {
    // unsafe{_ZN9QDirModelC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QDirModelC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDirModel{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdirmodel.h:71
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QDirModel()

/*

*/
pub fn DeleteQDirModel(this :*mut QDirModel) {
    // let rv = qtrt::InvokeQtFunc6("_ZN9QDirModelD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qdirmodel.h:73
// index:0
// Public virtual Visibility=Default Availability=Available
// [24] QModelIndex index(int, int, const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::index().

Returns the model item index for the item in the parent with the given row and column.
*/
impl /*struct*/ QDirModel {
  pub fn index_0<RetType, T: QDirModel_index_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.index_0(self);
    // return 1;
  }
}
pub trait QDirModel_index_0<RetType> {
  fn index_0(self , rsthis: & QDirModel) -> RetType;
}
impl<'a> /*trait*/ QDirModel_index_0<usize> for (i32,i32,usize) {
  fn index_0(self , rsthis: & QDirModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDirModel5indexEiiRK11QModelIndex", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdirmodel.h:118
// index:1
// Public Visibility=Default Availability=Available
// [24] QModelIndex index(const QString &, int) const

/*
Reimplemented from QAbstractItemModel::index().

Returns the model item index for the item in the parent with the given row and column.
*/
impl /*struct*/ QDirModel {
  pub fn index_1<RetType, T: QDirModel_index_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.index_1(self);
    // return 1;
  }
}
pub trait QDirModel_index_1<RetType> {
  fn index_1(self , rsthis: & QDirModel) -> RetType;
}
impl<'a> /*trait*/ QDirModel_index_1<usize> for (usize,i32) {
  fn index_1(self , rsthis: & QDirModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDirModel5indexERK7QStringi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdirmodel.h:74
// index:0
// Public virtual Visibility=Default Availability=Available
// [24] QModelIndex parent(const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::parent().

Return the parent of the given child model item.
*/
impl /*struct*/ QDirModel {
  pub fn parent_0<RetType, T: QDirModel_parent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.parent_0(self);
    // return 1;
  }
}
pub trait QDirModel_parent_0<RetType> {
  fn parent_0(self , rsthis: & QDirModel) -> RetType;
}
impl<'a> /*trait*/ QDirModel_parent_0<usize> for (usize) {
  fn parent_0(self , rsthis: & QDirModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDirModel6parentERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdirmodel.h:76
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int rowCount(const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::rowCount().

Returns the number of rows in the parent model item.
*/
impl /*struct*/ QDirModel {
  pub fn rowCount_0<RetType, T: QDirModel_rowCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowCount_0(self);
    // return 1;
  }
}
pub trait QDirModel_rowCount_0<RetType> {
  fn rowCount_0(self , rsthis: & QDirModel) -> RetType;
}
impl<'a> /*trait*/ QDirModel_rowCount_0<i32> for (usize) {
  fn rowCount_0(self , rsthis: & QDirModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDirModel8rowCountERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdirmodel.h:77
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int columnCount(const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::columnCount().

Returns the number of columns in the parent model item.
*/
impl /*struct*/ QDirModel {
  pub fn columnCount_0<RetType, T: QDirModel_columnCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnCount_0(self);
    // return 1;
  }
}
pub trait QDirModel_columnCount_0<RetType> {
  fn columnCount_0(self , rsthis: & QDirModel) -> RetType;
}
impl<'a> /*trait*/ QDirModel_columnCount_0<i32> for (usize) {
  fn columnCount_0(self , rsthis: & QDirModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDirModel11columnCountERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdirmodel.h:79
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QVariant data(const QModelIndex &, int) const

/*
Reimplemented from QAbstractItemModel::data().

Returns the data for the model item index with the given role.

See also setData().
*/
impl /*struct*/ QDirModel {
  pub fn data_0<RetType, T: QDirModel_data_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.data_0(self);
    // return 1;
  }
}
pub trait QDirModel_data_0<RetType> {
  fn data_0(self , rsthis: & QDirModel) -> RetType;
}
impl<'a> /*trait*/ QDirModel_data_0<usize> for (usize,i32) {
  fn data_0(self , rsthis: & QDirModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDirModel4dataERK11QModelIndexi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdirmodel.h:80
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool setData(const QModelIndex &, const QVariant &, int)

/*
Reimplemented from QAbstractItemModel::setData().

Sets the data for the model item index with the given role to the data referenced by the value. Returns true if successful; otherwise returns false.

See also data() and Qt::ItemDataRole.
*/
impl /*struct*/ QDirModel {
  pub fn setData_0<RetType, T: QDirModel_setData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setData_0(self);
    // return 1;
  }
}
pub trait QDirModel_setData_0<RetType> {
  fn setData_0(self , rsthis: & QDirModel) -> RetType;
}
impl<'a> /*trait*/ QDirModel_setData_0<bool> for (usize,usize,i32) {
  fn setData_0(self , rsthis: & QDirModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QDirModel7setDataERK11QModelIndexRK8QVarianti", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdirmodel.h:82
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QVariant headerData(int, Qt::Orientation, int) const

/*
Reimplemented from QAbstractItemModel::headerData().

Returns the data stored under the given role for the specified section of the header with the given orientation.
*/
impl /*struct*/ QDirModel {
  pub fn headerData_0<RetType, T: QDirModel_headerData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.headerData_0(self);
    // return 1;
  }
}
pub trait QDirModel_headerData_0<RetType> {
  fn headerData_0(self , rsthis: & QDirModel) -> RetType;
}
impl<'a> /*trait*/ QDirModel_headerData_0<usize> for (i32,i32,i32) {
  fn headerData_0(self , rsthis: & QDirModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDirModel10headerDataEiN2Qt11OrientationEi", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdirmodel.h:84
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool hasChildren(const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::hasChildren().

Returns true if the parent model item has children; otherwise returns false.
*/
impl /*struct*/ QDirModel {
  pub fn hasChildren_0<RetType, T: QDirModel_hasChildren_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasChildren_0(self);
    // return 1;
  }
}
pub trait QDirModel_hasChildren_0<RetType> {
  fn hasChildren_0(self , rsthis: & QDirModel) -> RetType;
}
impl<'a> /*trait*/ QDirModel_hasChildren_0<bool> for (usize) {
  fn hasChildren_0(self , rsthis: & QDirModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDirModel11hasChildrenERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdirmodel.h:85
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] Qt::ItemFlags flags(const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::flags().

Returns the item flags for the given index in the model.

See also Qt::ItemFlags.
*/
impl /*struct*/ QDirModel {
  pub fn flags_0<RetType, T: QDirModel_flags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.flags_0(self);
    // return 1;
  }
}
pub trait QDirModel_flags_0<RetType> {
  fn flags_0(self , rsthis: & QDirModel) -> RetType;
}
impl<'a> /*trait*/ QDirModel_flags_0<i32> for (usize) {
  fn flags_0(self , rsthis: & QDirModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDirModel5flagsERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdirmodel.h:87
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void sort(int, Qt::SortOrder)

/*
Reimplemented from QAbstractItemModel::sort().

Sort the model items in the column using the order given. The order is a value defined in Qt::SortOrder.
*/
impl /*struct*/ QDirModel {
  pub fn sort_0<RetType, T: QDirModel_sort_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sort_0(self);
    // return 1;
  }
}
pub trait QDirModel_sort_0<RetType> {
  fn sort_0(self , rsthis: & QDirModel) -> RetType;
}
impl<'a> /*trait*/ QDirModel_sort_0<(/*void*/)> for (i32,i32) {
  fn sort_0(self , rsthis: & QDirModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QDirModel4sortEiN2Qt9SortOrderE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdirmodel.h:89
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QStringList mimeTypes() const

/*
Reimplemented from QAbstractItemModel::mimeTypes().

Returns a list of MIME types that can be used to describe a list of items in the model.
*/
impl /*struct*/ QDirModel {
  pub fn mimeTypes_0<RetType, T: QDirModel_mimeTypes_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mimeTypes_0(self);
    // return 1;
  }
}
pub trait QDirModel_mimeTypes_0<RetType> {
  fn mimeTypes_0(self , rsthis: & QDirModel) -> RetType;
}
impl<'a> /*trait*/ QDirModel_mimeTypes_0<usize> for () {
  fn mimeTypes_0(self , rsthis: & QDirModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDirModel9mimeTypesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdirmodel.h:91
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool dropMimeData(const QMimeData *, Qt::DropAction, int, int, const QModelIndex &)

/*
Reimplemented from QAbstractItemModel::dropMimeData().

Handles the data supplied by a drag and drop operation that ended with the given action over the row in the model specified by the row and column and by the parent index.

Returns true if the drop was successful, and false otherwise.

See also supportedDropActions().
*/
impl /*struct*/ QDirModel {
  pub fn dropMimeData_0<RetType, T: QDirModel_dropMimeData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dropMimeData_0(self);
    // return 1;
  }
}
pub trait QDirModel_dropMimeData_0<RetType> {
  fn dropMimeData_0(self , rsthis: & QDirModel) -> RetType;
}
impl<'a> /*trait*/ QDirModel_dropMimeData_0<bool> for (usize,i32,i32,i32,usize) {
  fn dropMimeData_0(self , rsthis: & QDirModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QDirModel12dropMimeDataEPK9QMimeDataN2Qt10DropActionEiiRK11QModelIndex", 5,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdirmodel.h:93
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] Qt::DropActions supportedDropActions() const

/*
Reimplemented from QAbstractItemModel::supportedDropActions().

Returns the drop actions supported by this model.

See also Qt::DropActions.
*/
impl /*struct*/ QDirModel {
  pub fn supportedDropActions_0<RetType, T: QDirModel_supportedDropActions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.supportedDropActions_0(self);
    // return 1;
  }
}
pub trait QDirModel_supportedDropActions_0<RetType> {
  fn supportedDropActions_0(self , rsthis: & QDirModel) -> RetType;
}
impl<'a> /*trait*/ QDirModel_supportedDropActions_0<i32> for () {
  fn supportedDropActions_0(self , rsthis: & QDirModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDirModel20supportedDropActionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdirmodel.h:97
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setIconProvider(QFileIconProvider *)

/*
Sets the provider of file icons for the directory model.

See also iconProvider().
*/
impl /*struct*/ QDirModel {
  pub fn setIconProvider_0<RetType, T: QDirModel_setIconProvider_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIconProvider_0(self);
    // return 1;
  }
}
pub trait QDirModel_setIconProvider_0<RetType> {
  fn setIconProvider_0(self , rsthis: & QDirModel) -> RetType;
}
impl<'a> /*trait*/ QDirModel_setIconProvider_0<(/*void*/)> for (usize) {
  fn setIconProvider_0(self , rsthis: & QDirModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QDirModel15setIconProviderEP17QFileIconProvider", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdirmodel.h:98
// index:0
// Public Visibility=Default Availability=Available
// [8] QFileIconProvider * iconProvider() const

/*
Returns the file icon provider for this directory model.

See also setIconProvider().
*/
impl /*struct*/ QDirModel {
  pub fn iconProvider_0<RetType, T: QDirModel_iconProvider_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.iconProvider_0(self);
    // return 1;
  }
}
pub trait QDirModel_iconProvider_0<RetType> {
  fn iconProvider_0(self , rsthis: & QDirModel) -> RetType;
}
impl<'a> /*trait*/ QDirModel_iconProvider_0<usize> for () {
  fn iconProvider_0(self , rsthis: & QDirModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDirModel12iconProviderEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdirmodel.h:100
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setNameFilters(const QStringList &)

/*
Sets the name filters for the directory model.

See also nameFilters().
*/
impl /*struct*/ QDirModel {
  pub fn setNameFilters_0<RetType, T: QDirModel_setNameFilters_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNameFilters_0(self);
    // return 1;
  }
}
pub trait QDirModel_setNameFilters_0<RetType> {
  fn setNameFilters_0(self , rsthis: & QDirModel) -> RetType;
}
impl<'a> /*trait*/ QDirModel_setNameFilters_0<(/*void*/)> for (usize) {
  fn setNameFilters_0(self , rsthis: & QDirModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QDirModel14setNameFiltersERK11QStringList", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdirmodel.h:101
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList nameFilters() const

/*
Returns a list of filters applied to the names in the model.

See also setNameFilters().
*/
impl /*struct*/ QDirModel {
  pub fn nameFilters_0<RetType, T: QDirModel_nameFilters_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.nameFilters_0(self);
    // return 1;
  }
}
pub trait QDirModel_nameFilters_0<RetType> {
  fn nameFilters_0(self , rsthis: & QDirModel) -> RetType;
}
impl<'a> /*trait*/ QDirModel_nameFilters_0<usize> for () {
  fn nameFilters_0(self , rsthis: & QDirModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDirModel11nameFiltersEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdirmodel.h:103
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFilter(QDir::Filters)

/*
Sets the directory model's filter to that specified by filters.

Note that the filter you set should always include the QDir::AllDirs enum value, otherwise QDirModel won't be able to read the directory structure.

See also filter() and QDir::Filters.
*/
impl /*struct*/ QDirModel {
  pub fn setFilter_0<RetType, T: QDirModel_setFilter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFilter_0(self);
    // return 1;
  }
}
pub trait QDirModel_setFilter_0<RetType> {
  fn setFilter_0(self , rsthis: & QDirModel) -> RetType;
}
impl<'a> /*trait*/ QDirModel_setFilter_0<(/*void*/)> for (i32) {
  fn setFilter_0(self , rsthis: & QDirModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QDirModel9setFilterE6QFlagsIN4QDir6FilterEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdirmodel.h:104
// index:0
// Public Visibility=Default Availability=Available
// [4] QDir::Filters filter() const

/*
Returns the filter specification for the directory model.

See also setFilter() and QDir::Filters.
*/
impl /*struct*/ QDirModel {
  pub fn filter_0<RetType, T: QDirModel_filter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.filter_0(self);
    // return 1;
  }
}
pub trait QDirModel_filter_0<RetType> {
  fn filter_0(self , rsthis: & QDirModel) -> RetType;
}
impl<'a> /*trait*/ QDirModel_filter_0<i32> for () {
  fn filter_0(self , rsthis: & QDirModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDirModel6filterEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdirmodel.h:106
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSorting(QDir::SortFlags)

/*
Sets the directory model's sorting order to that specified by sort.

See also sorting() and QDir::SortFlags.
*/
impl /*struct*/ QDirModel {
  pub fn setSorting_0<RetType, T: QDirModel_setSorting_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSorting_0(self);
    // return 1;
  }
}
pub trait QDirModel_setSorting_0<RetType> {
  fn setSorting_0(self , rsthis: & QDirModel) -> RetType;
}
impl<'a> /*trait*/ QDirModel_setSorting_0<(/*void*/)> for (i32) {
  fn setSorting_0(self , rsthis: & QDirModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QDirModel10setSortingE6QFlagsIN4QDir8SortFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdirmodel.h:107
// index:0
// Public Visibility=Default Availability=Available
// [4] QDir::SortFlags sorting() const

/*
Returns the sorting method used for the directory model.

See also setSorting() and QDir::SortFlags.
*/
impl /*struct*/ QDirModel {
  pub fn sorting_0<RetType, T: QDirModel_sorting_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sorting_0(self);
    // return 1;
  }
}
pub trait QDirModel_sorting_0<RetType> {
  fn sorting_0(self , rsthis: & QDirModel) -> RetType;
}
impl<'a> /*trait*/ QDirModel_sorting_0<i32> for () {
  fn sorting_0(self , rsthis: & QDirModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDirModel7sortingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdirmodel.h:109
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setResolveSymlinks(bool)

/*

*/
impl /*struct*/ QDirModel {
  pub fn setResolveSymlinks_0<RetType, T: QDirModel_setResolveSymlinks_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setResolveSymlinks_0(self);
    // return 1;
  }
}
pub trait QDirModel_setResolveSymlinks_0<RetType> {
  fn setResolveSymlinks_0(self , rsthis: & QDirModel) -> RetType;
}
impl<'a> /*trait*/ QDirModel_setResolveSymlinks_0<(/*void*/)> for (bool) {
  fn setResolveSymlinks_0(self , rsthis: & QDirModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QDirModel18setResolveSymlinksEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdirmodel.h:110
// index:0
// Public Visibility=Default Availability=Available
// [1] bool resolveSymlinks() const

/*

*/
impl /*struct*/ QDirModel {
  pub fn resolveSymlinks_0<RetType, T: QDirModel_resolveSymlinks_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resolveSymlinks_0(self);
    // return 1;
  }
}
pub trait QDirModel_resolveSymlinks_0<RetType> {
  fn resolveSymlinks_0(self , rsthis: & QDirModel) -> RetType;
}
impl<'a> /*trait*/ QDirModel_resolveSymlinks_0<bool> for () {
  fn resolveSymlinks_0(self , rsthis: & QDirModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDirModel15resolveSymlinksEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdirmodel.h:112
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setReadOnly(bool)

/*

*/
impl /*struct*/ QDirModel {
  pub fn setReadOnly_0<RetType, T: QDirModel_setReadOnly_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setReadOnly_0(self);
    // return 1;
  }
}
pub trait QDirModel_setReadOnly_0<RetType> {
  fn setReadOnly_0(self , rsthis: & QDirModel) -> RetType;
}
impl<'a> /*trait*/ QDirModel_setReadOnly_0<(/*void*/)> for (bool) {
  fn setReadOnly_0(self , rsthis: & QDirModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QDirModel11setReadOnlyEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdirmodel.h:113
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isReadOnly() const

/*

*/
impl /*struct*/ QDirModel {
  pub fn isReadOnly_0<RetType, T: QDirModel_isReadOnly_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isReadOnly_0(self);
    // return 1;
  }
}
pub trait QDirModel_isReadOnly_0<RetType> {
  fn isReadOnly_0(self , rsthis: & QDirModel) -> RetType;
}
impl<'a> /*trait*/ QDirModel_isReadOnly_0<bool> for () {
  fn isReadOnly_0(self , rsthis: & QDirModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDirModel10isReadOnlyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdirmodel.h:115
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLazyChildCount(bool)

/*

*/
impl /*struct*/ QDirModel {
  pub fn setLazyChildCount_0<RetType, T: QDirModel_setLazyChildCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLazyChildCount_0(self);
    // return 1;
  }
}
pub trait QDirModel_setLazyChildCount_0<RetType> {
  fn setLazyChildCount_0(self , rsthis: & QDirModel) -> RetType;
}
impl<'a> /*trait*/ QDirModel_setLazyChildCount_0<(/*void*/)> for (bool) {
  fn setLazyChildCount_0(self , rsthis: & QDirModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QDirModel17setLazyChildCountEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdirmodel.h:116
// index:0
// Public Visibility=Default Availability=Available
// [1] bool lazyChildCount() const

/*

*/
impl /*struct*/ QDirModel {
  pub fn lazyChildCount_0<RetType, T: QDirModel_lazyChildCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lazyChildCount_0(self);
    // return 1;
  }
}
pub trait QDirModel_lazyChildCount_0<RetType> {
  fn lazyChildCount_0(self , rsthis: & QDirModel) -> RetType;
}
impl<'a> /*trait*/ QDirModel_lazyChildCount_0<bool> for () {
  fn lazyChildCount_0(self , rsthis: & QDirModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDirModel14lazyChildCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdirmodel.h:120
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isDir(const QModelIndex &) const

/*
Returns true if the model item index represents a directory; otherwise returns false.
*/
impl /*struct*/ QDirModel {
  pub fn isDir_0<RetType, T: QDirModel_isDir_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isDir_0(self);
    // return 1;
  }
}
pub trait QDirModel_isDir_0<RetType> {
  fn isDir_0(self , rsthis: & QDirModel) -> RetType;
}
impl<'a> /*trait*/ QDirModel_isDir_0<bool> for (usize) {
  fn isDir_0(self , rsthis: & QDirModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDirModel5isDirERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdirmodel.h:121
// index:0
// Public Visibility=Default Availability=Available
// [24] QModelIndex mkdir(const QModelIndex &, const QString &)

/*
Create a directory with the name in the parent model item.
*/
impl /*struct*/ QDirModel {
  pub fn mkdir_0<RetType, T: QDirModel_mkdir_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mkdir_0(self);
    // return 1;
  }
}
pub trait QDirModel_mkdir_0<RetType> {
  fn mkdir_0(self , rsthis: & QDirModel) -> RetType;
}
impl<'a> /*trait*/ QDirModel_mkdir_0<usize> for (usize,usize) {
  fn mkdir_0(self , rsthis: & QDirModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QDirModel5mkdirERK11QModelIndexRK7QString", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdirmodel.h:122
// index:0
// Public Visibility=Default Availability=Available
// [1] bool rmdir(const QModelIndex &)

/*
Removes the directory corresponding to the model item index in the directory model and deletes the corresponding directory from the file system, returning true if successful. If the directory cannot be removed, false is returned.

Warning: This function deletes directories from the file system; it does not move them to a location where they can be recovered.

See also remove().
*/
impl /*struct*/ QDirModel {
  pub fn rmdir_0<RetType, T: QDirModel_rmdir_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rmdir_0(self);
    // return 1;
  }
}
pub trait QDirModel_rmdir_0<RetType> {
  fn rmdir_0(self , rsthis: & QDirModel) -> RetType;
}
impl<'a> /*trait*/ QDirModel_rmdir_0<bool> for (usize) {
  fn rmdir_0(self , rsthis: & QDirModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QDirModel5rmdirERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdirmodel.h:123
// index:0
// Public Visibility=Default Availability=Available
// [1] bool remove(const QModelIndex &)

/*
Removes the model item index from the directory model and deletes the corresponding file from the file system, returning true if successful. If the item cannot be removed, false is returned.

Warning: This function deletes files from the file system; it does not move them to a location where they can be recovered.

See also rmdir().
*/
impl /*struct*/ QDirModel {
  pub fn remove_0<RetType, T: QDirModel_remove_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.remove_0(self);
    // return 1;
  }
}
pub trait QDirModel_remove_0<RetType> {
  fn remove_0(self , rsthis: & QDirModel) -> RetType;
}
impl<'a> /*trait*/ QDirModel_remove_0<bool> for (usize) {
  fn remove_0(self , rsthis: & QDirModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QDirModel6removeERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdirmodel.h:125
// index:0
// Public Visibility=Default Availability=Available
// [8] QString filePath(const QModelIndex &) const

/*
Returns the path of the item stored in the model under the index given.
*/
impl /*struct*/ QDirModel {
  pub fn filePath_0<RetType, T: QDirModel_filePath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.filePath_0(self);
    // return 1;
  }
}
pub trait QDirModel_filePath_0<RetType> {
  fn filePath_0(self , rsthis: & QDirModel) -> RetType;
}
impl<'a> /*trait*/ QDirModel_filePath_0<usize> for (usize) {
  fn filePath_0(self , rsthis: & QDirModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDirModel8filePathERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdirmodel.h:126
// index:0
// Public Visibility=Default Availability=Available
// [8] QString fileName(const QModelIndex &) const

/*
Returns the name of the item stored in the model under the index given.
*/
impl /*struct*/ QDirModel {
  pub fn fileName_0<RetType, T: QDirModel_fileName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fileName_0(self);
    // return 1;
  }
}
pub trait QDirModel_fileName_0<RetType> {
  fn fileName_0(self , rsthis: & QDirModel) -> RetType;
}
impl<'a> /*trait*/ QDirModel_fileName_0<usize> for (usize) {
  fn fileName_0(self , rsthis: & QDirModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDirModel8fileNameERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdirmodel.h:127
// index:0
// Public Visibility=Default Availability=Available
// [8] QIcon fileIcon(const QModelIndex &) const

/*
Returns the icons for the item stored in the model under the given index.
*/
impl /*struct*/ QDirModel {
  pub fn fileIcon_0<RetType, T: QDirModel_fileIcon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fileIcon_0(self);
    // return 1;
  }
}
pub trait QDirModel_fileIcon_0<RetType> {
  fn fileIcon_0(self , rsthis: & QDirModel) -> RetType;
}
impl<'a> /*trait*/ QDirModel_fileIcon_0<usize> for (usize) {
  fn fileIcon_0(self , rsthis: & QDirModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDirModel8fileIconERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdirmodel.h:128
// index:0
// Public Visibility=Default Availability=Available
// [8] QFileInfo fileInfo(const QModelIndex &) const

/*
Returns the file information for the specified model index.

Note: If the model index represents a symbolic link in the underlying filing system, the file information returned will contain information about the symbolic link itself, regardless of whether resolveSymlinks is enabled or not.

See also QFileInfo::symLinkTarget().
*/
impl /*struct*/ QDirModel {
  pub fn fileInfo_0<RetType, T: QDirModel_fileInfo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fileInfo_0(self);
    // return 1;
  }
}
pub trait QDirModel_fileInfo_0<RetType> {
  fn fileInfo_0(self , rsthis: & QDirModel) -> RetType;
}
impl<'a> /*trait*/ QDirModel_fileInfo_0<usize> for (usize) {
  fn fileInfo_0(self , rsthis: & QDirModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDirModel8fileInfoERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdirmodel.h:133
// index:0
// Public Visibility=Default Availability=Available
// [-2] void refresh(const QModelIndex &)

/*
QDirModel caches file information. This function updates the cache. The parent parameter is the directory from which the model is updated; the default value will update the model from root directory of the file system (the entire model).
*/
impl /*struct*/ QDirModel {
  pub fn refresh_0<RetType, T: QDirModel_refresh_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.refresh_0(self);
    // return 1;
  }
}
pub trait QDirModel_refresh_0<RetType> {
  fn refresh_0(self , rsthis: & QDirModel) -> RetType;
}
impl<'a> /*trait*/ QDirModel_refresh_0<(/*void*/)> for (usize) {
  fn refresh_0(self , rsthis: & QDirModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QDirModel7refreshERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
ConstantValue
QDirModel::FileIconRoleQt::DecorationRole
QDirModel::FilePathRoleQt::UserRole + 1
QDirModel::FileNameRole?

*/
pub type QDirModel__Roles = i32;
// 
pub const QDirModel__FileIconRole :QDirModel__Roles = 1;
// 
pub const QDirModel__FilePathRole :QDirModel__Roles = 257;
// 
pub const QDirModel__FileNameRole :QDirModel__Roles = 258;
pub fn QDirModel_RolesItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QDirModel", val);
}
pub fn QDirModel_RolesItemName_s(val: i32) ->String {
  //var nilthis *QDirModel
  //return nilthis.RolesItemName(val);
  return QDirModel_RolesItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
