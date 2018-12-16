

// mod ::gui::QStandardItemModel
// package qtgui
// /usr/include/qt/QtGui/qstandarditemmodel.h
// #include <qstandarditemmodel.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 83
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
#[derive(Default)] // class sizeof(QStandardItemModel)=16
pub struct QStandardItemModel {
  qbase: QAbstractItemModel,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStandardItemModel_ITF interface {
//    qtcore.QAbstractItemModel_ITF
//    QStandardItemModel_PTR() *QStandardItemModel
//}
//func (ptr *QStandardItemModel) QStandardItemModel_PTR() *QStandardItemModel { return ptr }

impl /*struct*/ QStandardItemModel {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStandardItemModel {
    return QStandardItemModel{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStandardItemModel {
//  type Target = QStandardItemModelBASE;
//
//  fn deref(&self) -> &QStandardItemModelBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStandardItemModelBASE> for QStandardItemModel {
//  fn as_ref(& self) -> & QStandardItemModelBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qstandarditemmodel.h:326
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QStandardItemModel {
  pub fn metaObject_0<RetType, T: QStandardItemModel_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QStandardItemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QStandardItemModel10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:330
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStandardItemModel(QObject *)

/*
Constructs a new item model with the given parent.
*/
// QStandardItemModel(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QStandardItemModel {
  pub fn QStandardItemModel_0<T: QStandardItemModel_QStandardItemModel_0>(value: T) -> QStandardItemModel {
    let rsthis = value.QStandardItemModel_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStandardItemModel_QStandardItemModel_0 {
  fn QStandardItemModel_0(self) -> QStandardItemModel;
}
// QStandardItemModel(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStandardItemModel_QStandardItemModel_0 for (usize) {
  fn QStandardItemModel_0(self) -> QStandardItemModel {
    // unsafe{_ZN18QStandardItemModelC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QStandardItemModelC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStandardItemModel{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:331
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QStandardItemModel(int, int, QObject *)

/*
Constructs a new item model with the given parent.
*/
// QStandardItemModel(int, int, QObject *) ctx.fn_proto_cpp
impl /*struct*/ QStandardItemModel {
  pub fn QStandardItemModel_1<T: QStandardItemModel_QStandardItemModel_1>(value: T) -> QStandardItemModel {
    let rsthis = value.QStandardItemModel_1();
    return rsthis;
    // return 1;
  }
}

pub trait QStandardItemModel_QStandardItemModel_1 {
  fn QStandardItemModel_1(self) -> QStandardItemModel;
}
// QStandardItemModel(int, int, QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStandardItemModel_QStandardItemModel_1 for (i32,i32,usize) {
  fn QStandardItemModel_1(self) -> QStandardItemModel {
    // unsafe{_ZN18QStandardItemModelC2EiiP7QObject()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QStandardItemModelC2EiiP7QObject", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStandardItemModel{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:332
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QStandardItemModel()

/*

*/
pub fn DeleteQStandardItemModel(this :*mut QStandardItemModel) {
    // let rv = qtrt::InvokeQtFunc6("_ZN18QStandardItemModelD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qstandarditemmodel.h:336
// index:0
// Public virtual Visibility=Default Availability=Available
// [24] QModelIndex index(int, int, const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::index().
*/
impl /*struct*/ QStandardItemModel {
  pub fn index_0<RetType, T: QStandardItemModel_index_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.index_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_index_0<RetType> {
  fn index_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_index_0<usize> for (i32,i32,usize) {
  fn index_0(self , rsthis: & QStandardItemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QStandardItemModel5indexEiiRK11QModelIndex", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:337
// index:0
// Public virtual Visibility=Default Availability=Available
// [24] QModelIndex parent(const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::parent().
*/
impl /*struct*/ QStandardItemModel {
  pub fn parent_0<RetType, T: QStandardItemModel_parent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.parent_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_parent_0<RetType> {
  fn parent_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_parent_0<usize> for (usize) {
  fn parent_0(self , rsthis: & QStandardItemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QStandardItemModel6parentERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:339
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int rowCount(const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::rowCount().

See also setRowCount().
*/
impl /*struct*/ QStandardItemModel {
  pub fn rowCount_0<RetType, T: QStandardItemModel_rowCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowCount_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_rowCount_0<RetType> {
  fn rowCount_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_rowCount_0<i32> for (usize) {
  fn rowCount_0(self , rsthis: & QStandardItemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QStandardItemModel8rowCountERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:340
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int columnCount(const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::columnCount().

See also setColumnCount().
*/
impl /*struct*/ QStandardItemModel {
  pub fn columnCount_0<RetType, T: QStandardItemModel_columnCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnCount_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_columnCount_0<RetType> {
  fn columnCount_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_columnCount_0<i32> for (usize) {
  fn columnCount_0(self , rsthis: & QStandardItemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QStandardItemModel11columnCountERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:341
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool hasChildren(const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::hasChildren().
*/
impl /*struct*/ QStandardItemModel {
  pub fn hasChildren_0<RetType, T: QStandardItemModel_hasChildren_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasChildren_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_hasChildren_0<RetType> {
  fn hasChildren_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_hasChildren_0<bool> for (usize) {
  fn hasChildren_0(self , rsthis: & QStandardItemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QStandardItemModel11hasChildrenERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:343
// index:0
// Public virtual Visibility=Default Availability=Available
// [24] QModelIndex sibling(int, int, const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::sibling().
*/
impl /*struct*/ QStandardItemModel {
  pub fn sibling_0<RetType, T: QStandardItemModel_sibling_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sibling_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_sibling_0<RetType> {
  fn sibling_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_sibling_0<usize> for (i32,i32,usize) {
  fn sibling_0(self , rsthis: & QStandardItemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QStandardItemModel7siblingEiiRK11QModelIndex", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:345
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QVariant data(const QModelIndex &, int) const

/*
Reimplemented from QAbstractItemModel::data().

See also setData().
*/
impl /*struct*/ QStandardItemModel {
  pub fn data_0<RetType, T: QStandardItemModel_data_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.data_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_data_0<RetType> {
  fn data_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_data_0<usize> for (usize,i32) {
  fn data_0(self , rsthis: & QStandardItemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QStandardItemModel4dataERK11QModelIndexi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:346
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool setData(const QModelIndex &, const QVariant &, int)

/*
Reimplemented from QAbstractItemModel::setData().

See also data().
*/
impl /*struct*/ QStandardItemModel {
  pub fn setData_0<RetType, T: QStandardItemModel_setData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setData_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_setData_0<RetType> {
  fn setData_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_setData_0<bool> for (usize,usize,i32) {
  fn setData_0(self , rsthis: & QStandardItemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QStandardItemModel7setDataERK11QModelIndexRK8QVarianti", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:348
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QVariant headerData(int, Qt::Orientation, int) const

/*
Reimplemented from QAbstractItemModel::headerData().

See also setHeaderData().
*/
impl /*struct*/ QStandardItemModel {
  pub fn headerData_0<RetType, T: QStandardItemModel_headerData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.headerData_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_headerData_0<RetType> {
  fn headerData_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_headerData_0<usize> for (i32,i32,i32) {
  fn headerData_0(self , rsthis: & QStandardItemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QStandardItemModel10headerDataEiN2Qt11OrientationEi", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:350
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool setHeaderData(int, Qt::Orientation, const QVariant &, int)

/*
Reimplemented from QAbstractItemModel::setHeaderData().

See also headerData().
*/
impl /*struct*/ QStandardItemModel {
  pub fn setHeaderData_0<RetType, T: QStandardItemModel_setHeaderData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHeaderData_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_setHeaderData_0<RetType> {
  fn setHeaderData_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_setHeaderData_0<bool> for (i32,i32,usize,i32) {
  fn setHeaderData_0(self , rsthis: & QStandardItemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QStandardItemModel13setHeaderDataEiN2Qt11OrientationERK8QVarianti", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:353
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool insertRows(int, int, const QModelIndex &)

/*
Reimplemented from QAbstractItemModel::insertRows().
*/
impl /*struct*/ QStandardItemModel {
  pub fn insertRows_0<RetType, T: QStandardItemModel_insertRows_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertRows_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_insertRows_0<RetType> {
  fn insertRows_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_insertRows_0<bool> for (i32,i32,usize) {
  fn insertRows_0(self , rsthis: & QStandardItemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QStandardItemModel10insertRowsEiiRK11QModelIndex", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:354
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool insertColumns(int, int, const QModelIndex &)

/*
Reimplemented from QAbstractItemModel::insertColumns().
*/
impl /*struct*/ QStandardItemModel {
  pub fn insertColumns_0<RetType, T: QStandardItemModel_insertColumns_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertColumns_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_insertColumns_0<RetType> {
  fn insertColumns_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_insertColumns_0<bool> for (i32,i32,usize) {
  fn insertColumns_0(self , rsthis: & QStandardItemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QStandardItemModel13insertColumnsEiiRK11QModelIndex", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:355
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool removeRows(int, int, const QModelIndex &)

/*
Reimplemented from QAbstractItemModel::removeRows().
*/
impl /*struct*/ QStandardItemModel {
  pub fn removeRows_0<RetType, T: QStandardItemModel_removeRows_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeRows_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_removeRows_0<RetType> {
  fn removeRows_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_removeRows_0<bool> for (i32,i32,usize) {
  fn removeRows_0(self , rsthis: & QStandardItemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QStandardItemModel10removeRowsEiiRK11QModelIndex", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:356
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool removeColumns(int, int, const QModelIndex &)

/*
Reimplemented from QAbstractItemModel::removeColumns().
*/
impl /*struct*/ QStandardItemModel {
  pub fn removeColumns_0<RetType, T: QStandardItemModel_removeColumns_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeColumns_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_removeColumns_0<RetType> {
  fn removeColumns_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_removeColumns_0<bool> for (i32,i32,usize) {
  fn removeColumns_0(self , rsthis: & QStandardItemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QStandardItemModel13removeColumnsEiiRK11QModelIndex", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:358
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] Qt::ItemFlags flags(const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::flags().
*/
impl /*struct*/ QStandardItemModel {
  pub fn flags_0<RetType, T: QStandardItemModel_flags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.flags_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_flags_0<RetType> {
  fn flags_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_flags_0<i32> for (usize) {
  fn flags_0(self , rsthis: & QStandardItemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QStandardItemModel5flagsERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:359
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] Qt::DropActions supportedDropActions() const

/*
Reimplemented from QAbstractItemModel::supportedDropActions().

QStandardItemModel supports both copy and move.
*/
impl /*struct*/ QStandardItemModel {
  pub fn supportedDropActions_0<RetType, T: QStandardItemModel_supportedDropActions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.supportedDropActions_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_supportedDropActions_0<RetType> {
  fn supportedDropActions_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_supportedDropActions_0<i32> for () {
  fn supportedDropActions_0(self , rsthis: & QStandardItemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QStandardItemModel20supportedDropActionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:364
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clear()

/*
Removes all items (including header items) from the model and sets the number of rows and columns to zero.

See also removeColumns() and removeRows().
*/
impl /*struct*/ QStandardItemModel {
  pub fn clear_0<RetType, T: QStandardItemModel_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_clear_0<RetType> {
  fn clear_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_clear_0<(/*void*/)> for () {
  fn clear_0(self , rsthis: & QStandardItemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN18QStandardItemModel5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:368
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void sort(int, Qt::SortOrder)

/*
Reimplemented from QAbstractItemModel::sort().
*/
impl /*struct*/ QStandardItemModel {
  pub fn sort_0<RetType, T: QStandardItemModel_sort_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sort_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_sort_0<RetType> {
  fn sort_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_sort_0<(/*void*/)> for (i32,i32) {
  fn sort_0(self , rsthis: & QStandardItemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN18QStandardItemModel4sortEiN2Qt9SortOrderE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:370
// index:0
// Public Visibility=Default Availability=Available
// [8] QStandardItem * itemFromIndex(const QModelIndex &) const

/*
Returns a pointer to the QStandardItem associated with the given index.

Calling this function is typically the initial step when processing QModelIndex-based signals from a view, such as QAbstractItemView::activated(). In your slot, you call itemFromIndex(), with the QModelIndex carried by the signal as argument, to obtain a pointer to the corresponding QStandardItem.

Note that this function will lazily create an item for the index (using itemPrototype()), and set it in the parent item's child table, if no item already exists at that index.

If index is an invalid index, this function returns 0.

This function was introduced in  Qt 4.2.

See also indexFromItem().
*/
impl /*struct*/ QStandardItemModel {
  pub fn itemFromIndex_0<RetType, T: QStandardItemModel_itemFromIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemFromIndex_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_itemFromIndex_0<RetType> {
  fn itemFromIndex_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_itemFromIndex_0<usize> for (usize) {
  fn itemFromIndex_0(self , rsthis: & QStandardItemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QStandardItemModel13itemFromIndexERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:371
// index:0
// Public Visibility=Default Availability=Available
// [24] QModelIndex indexFromItem(const QStandardItem *) const

/*
Returns the QModelIndex associated with the given item.

Use this function when you want to perform an operation that requires the QModelIndex of the item, such as QAbstractItemView::scrollTo(). QStandardItem::index() is provided as convenience; it is equivalent to calling this function.

This function was introduced in  Qt 4.2.

See also itemFromIndex() and QStandardItem::index().
*/
impl /*struct*/ QStandardItemModel {
  pub fn indexFromItem_0<RetType, T: QStandardItemModel_indexFromItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexFromItem_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_indexFromItem_0<RetType> {
  fn indexFromItem_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_indexFromItem_0<usize> for (usize) {
  fn indexFromItem_0(self , rsthis: & QStandardItemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QStandardItemModel13indexFromItemEPK13QStandardItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:373
// index:0
// Public Visibility=Default Availability=Available
// [8] QStandardItem * item(int, int) const

/*
Returns the item for the given row and column if one has been set; otherwise returns 0.

This function was introduced in  Qt 4.2.

See also setItem(), takeItem(), and itemFromIndex().
*/
impl /*struct*/ QStandardItemModel {
  pub fn item_0<RetType, T: QStandardItemModel_item_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.item_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_item_0<RetType> {
  fn item_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_item_0<usize> for (i32,i32) {
  fn item_0(self , rsthis: & QStandardItemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QStandardItemModel4itemEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:374
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setItem(int, int, QStandardItem *)

/*
Sets the item for the given row and column to item. The model takes ownership of the item. If necessary, the row count and column count are increased to fit the item. The previous item at the given location (if there was one) is deleted.

This function was introduced in  Qt 4.2.

See also item().
*/
impl /*struct*/ QStandardItemModel {
  pub fn setItem_0<RetType, T: QStandardItemModel_setItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setItem_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_setItem_0<RetType> {
  fn setItem_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_setItem_0<(/*void*/)> for (i32,i32,usize) {
  fn setItem_0(self , rsthis: & QStandardItemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QStandardItemModel7setItemEiiP13QStandardItem", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:375
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void setItem(int, QStandardItem *)

/*
Sets the item for the given row and column to item. The model takes ownership of the item. If necessary, the row count and column count are increased to fit the item. The previous item at the given location (if there was one) is deleted.

This function was introduced in  Qt 4.2.

See also item().
*/
impl /*struct*/ QStandardItemModel {
  pub fn setItem_1<RetType, T: QStandardItemModel_setItem_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setItem_1(self);
    // return 1;
  }
}
pub trait QStandardItemModel_setItem_1<RetType> {
  fn setItem_1(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_setItem_1<(/*void*/)> for (i32,usize) {
  fn setItem_1(self , rsthis: & QStandardItemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QStandardItemModel7setItemEiP13QStandardItem", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:376
// index:0
// Public Visibility=Default Availability=Available
// [8] QStandardItem * invisibleRootItem() const

/*
Returns the model's invisible root item.

The invisible root item provides access to the model's top-level items through the QStandardItem API, making it possible to write functions that can treat top-level items and their children in a uniform way; for example, recursive functions involving a tree model.

Note: Calling index() on the QStandardItem object retrieved from this function is not valid.

This function was introduced in  Qt 4.2.
*/
impl /*struct*/ QStandardItemModel {
  pub fn invisibleRootItem_0<RetType, T: QStandardItemModel_invisibleRootItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.invisibleRootItem_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_invisibleRootItem_0<RetType> {
  fn invisibleRootItem_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_invisibleRootItem_0<usize> for () {
  fn invisibleRootItem_0(self , rsthis: & QStandardItemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QStandardItemModel17invisibleRootItemEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:378
// index:0
// Public Visibility=Default Availability=Available
// [8] QStandardItem * horizontalHeaderItem(int) const

/*
Returns the horizontal header item for column if one has been set; otherwise returns 0.

This function was introduced in  Qt 4.2.

See also setHorizontalHeaderItem() and verticalHeaderItem().
*/
impl /*struct*/ QStandardItemModel {
  pub fn horizontalHeaderItem_0<RetType, T: QStandardItemModel_horizontalHeaderItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.horizontalHeaderItem_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_horizontalHeaderItem_0<RetType> {
  fn horizontalHeaderItem_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_horizontalHeaderItem_0<usize> for (i32) {
  fn horizontalHeaderItem_0(self , rsthis: & QStandardItemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QStandardItemModel20horizontalHeaderItemEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:379
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setHorizontalHeaderItem(int, QStandardItem *)

/*
Sets the horizontal header item for column to item. The model takes ownership of the item. If necessary, the column count is increased to fit the item. The previous header item (if there was one) is deleted.

This function was introduced in  Qt 4.2.

See also horizontalHeaderItem(), setHorizontalHeaderLabels(), and setVerticalHeaderItem().
*/
impl /*struct*/ QStandardItemModel {
  pub fn setHorizontalHeaderItem_0<RetType, T: QStandardItemModel_setHorizontalHeaderItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHorizontalHeaderItem_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_setHorizontalHeaderItem_0<RetType> {
  fn setHorizontalHeaderItem_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_setHorizontalHeaderItem_0<(/*void*/)> for (i32,usize) {
  fn setHorizontalHeaderItem_0(self , rsthis: & QStandardItemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QStandardItemModel23setHorizontalHeaderItemEiP13QStandardItem", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:380
// index:0
// Public Visibility=Default Availability=Available
// [8] QStandardItem * verticalHeaderItem(int) const

/*
Returns the vertical header item for row row if one has been set; otherwise returns 0.

This function was introduced in  Qt 4.2.

See also setVerticalHeaderItem() and horizontalHeaderItem().
*/
impl /*struct*/ QStandardItemModel {
  pub fn verticalHeaderItem_0<RetType, T: QStandardItemModel_verticalHeaderItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.verticalHeaderItem_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_verticalHeaderItem_0<RetType> {
  fn verticalHeaderItem_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_verticalHeaderItem_0<usize> for (i32) {
  fn verticalHeaderItem_0(self , rsthis: & QStandardItemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QStandardItemModel18verticalHeaderItemEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:381
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setVerticalHeaderItem(int, QStandardItem *)

/*
Sets the vertical header item for row to item. The model takes ownership of the item. If necessary, the row count is increased to fit the item. The previous header item (if there was one) is deleted.

This function was introduced in  Qt 4.2.

See also verticalHeaderItem(), setVerticalHeaderLabels(), and setHorizontalHeaderItem().
*/
impl /*struct*/ QStandardItemModel {
  pub fn setVerticalHeaderItem_0<RetType, T: QStandardItemModel_setVerticalHeaderItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVerticalHeaderItem_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_setVerticalHeaderItem_0<RetType> {
  fn setVerticalHeaderItem_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_setVerticalHeaderItem_0<(/*void*/)> for (i32,usize) {
  fn setVerticalHeaderItem_0(self , rsthis: & QStandardItemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QStandardItemModel21setVerticalHeaderItemEiP13QStandardItem", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:383
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setHorizontalHeaderLabels(const QStringList &)

/*
Sets the horizontal header labels using labels. If necessary, the column count is increased to the size of labels.

This function was introduced in  Qt 4.2.

See also setHorizontalHeaderItem().
*/
impl /*struct*/ QStandardItemModel {
  pub fn setHorizontalHeaderLabels_0<RetType, T: QStandardItemModel_setHorizontalHeaderLabels_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHorizontalHeaderLabels_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_setHorizontalHeaderLabels_0<RetType> {
  fn setHorizontalHeaderLabels_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_setHorizontalHeaderLabels_0<(/*void*/)> for (usize) {
  fn setHorizontalHeaderLabels_0(self , rsthis: & QStandardItemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QStandardItemModel25setHorizontalHeaderLabelsERK11QStringList", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:384
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setVerticalHeaderLabels(const QStringList &)

/*
Sets the vertical header labels using labels. If necessary, the row count is increased to the size of labels.

This function was introduced in  Qt 4.2.

See also setVerticalHeaderItem().
*/
impl /*struct*/ QStandardItemModel {
  pub fn setVerticalHeaderLabels_0<RetType, T: QStandardItemModel_setVerticalHeaderLabels_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVerticalHeaderLabels_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_setVerticalHeaderLabels_0<RetType> {
  fn setVerticalHeaderLabels_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_setVerticalHeaderLabels_0<(/*void*/)> for (usize) {
  fn setVerticalHeaderLabels_0(self , rsthis: & QStandardItemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QStandardItemModel23setVerticalHeaderLabelsERK11QStringList", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:386
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRowCount(int)

/*
Sets the number of rows in this model to rows. If this is less than rowCount(), the data in the unwanted rows is discarded.

This function was introduced in  Qt 4.2.

See also rowCount() and setColumnCount().
*/
impl /*struct*/ QStandardItemModel {
  pub fn setRowCount_0<RetType, T: QStandardItemModel_setRowCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRowCount_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_setRowCount_0<RetType> {
  fn setRowCount_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_setRowCount_0<(/*void*/)> for (i32) {
  fn setRowCount_0(self , rsthis: & QStandardItemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN18QStandardItemModel11setRowCountEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:387
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setColumnCount(int)

/*
Sets the number of columns in this model to columns. If this is less than columnCount(), the data in the unwanted columns is discarded.

This function was introduced in  Qt 4.2.

See also columnCount() and setRowCount().
*/
impl /*struct*/ QStandardItemModel {
  pub fn setColumnCount_0<RetType, T: QStandardItemModel_setColumnCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setColumnCount_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_setColumnCount_0<RetType> {
  fn setColumnCount_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_setColumnCount_0<(/*void*/)> for (i32) {
  fn setColumnCount_0(self , rsthis: & QStandardItemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN18QStandardItemModel14setColumnCountEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:391
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void appendRow(QStandardItem *)

/*
Appends a row containing items. If necessary, the column count is increased to the size of items.

This function was introduced in  Qt 4.2.

See also insertRow() and appendColumn().
*/
impl /*struct*/ QStandardItemModel {
  pub fn appendRow_0<RetType, T: QStandardItemModel_appendRow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.appendRow_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_appendRow_0<RetType> {
  fn appendRow_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_appendRow_0<(/*void*/)> for (usize) {
  fn appendRow_0(self , rsthis: & QStandardItemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QStandardItemModel9appendRowEP13QStandardItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:395
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void insertRow(int, QStandardItem *)

/*
Inserts a row at row containing items. If necessary, the column count is increased to the size of items.

This function was introduced in  Qt 4.2.

See also takeRow(), appendRow(), and insertColumn().
*/
impl /*struct*/ QStandardItemModel {
  pub fn insertRow_0<RetType, T: QStandardItemModel_insertRow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertRow_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_insertRow_0<RetType> {
  fn insertRow_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_insertRow_0<(/*void*/)> for (i32,usize) {
  fn insertRow_0(self , rsthis: & QStandardItemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QStandardItemModel9insertRowEiP13QStandardItem", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:397
// index:1
// Public inline Visibility=Default Availability=Available
// [1] bool insertRow(int, const QModelIndex &)

/*
Inserts a row at row containing items. If necessary, the column count is increased to the size of items.

This function was introduced in  Qt 4.2.

See also takeRow(), appendRow(), and insertColumn().
*/
impl /*struct*/ QStandardItemModel {
  pub fn insertRow_1<RetType, T: QStandardItemModel_insertRow_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertRow_1(self);
    // return 1;
  }
}
pub trait QStandardItemModel_insertRow_1<RetType> {
  fn insertRow_1(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_insertRow_1<bool> for (i32,usize) {
  fn insertRow_1(self , rsthis: & QStandardItemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QStandardItemModel9insertRowEiRK11QModelIndex", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:398
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool insertColumn(int, const QModelIndex &)

/*
Inserts a column at column containing items. If necessary, the row count is increased to the size of items.

This function was introduced in  Qt 4.2.

See also takeColumn(), appendColumn(), and insertRow().
*/
impl /*struct*/ QStandardItemModel {
  pub fn insertColumn_0<RetType, T: QStandardItemModel_insertColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertColumn_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_insertColumn_0<RetType> {
  fn insertColumn_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_insertColumn_0<bool> for (i32,usize) {
  fn insertColumn_0(self , rsthis: & QStandardItemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QStandardItemModel12insertColumnEiRK11QModelIndex", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:400
// index:0
// Public Visibility=Default Availability=Available
// [8] QStandardItem * takeItem(int, int)

/*
Removes the item at (row, column) without deleting it. The model releases ownership of the item.

This function was introduced in  Qt 4.2.

See also item(), takeRow(), and takeColumn().
*/
impl /*struct*/ QStandardItemModel {
  pub fn takeItem_0<RetType, T: QStandardItemModel_takeItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.takeItem_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_takeItem_0<RetType> {
  fn takeItem_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_takeItem_0<usize> for (i32,i32) {
  fn takeItem_0(self , rsthis: & QStandardItemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QStandardItemModel8takeItemEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:404
// index:0
// Public Visibility=Default Availability=Available
// [8] QStandardItem * takeHorizontalHeaderItem(int)

/*
Removes the horizontal header item at column from the header without deleting it, and returns a pointer to the item. The model releases ownership of the item.

This function was introduced in  Qt 4.2.

See also horizontalHeaderItem() and takeVerticalHeaderItem().
*/
impl /*struct*/ QStandardItemModel {
  pub fn takeHorizontalHeaderItem_0<RetType, T: QStandardItemModel_takeHorizontalHeaderItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.takeHorizontalHeaderItem_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_takeHorizontalHeaderItem_0<RetType> {
  fn takeHorizontalHeaderItem_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_takeHorizontalHeaderItem_0<usize> for (i32) {
  fn takeHorizontalHeaderItem_0(self , rsthis: & QStandardItemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QStandardItemModel24takeHorizontalHeaderItemEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:405
// index:0
// Public Visibility=Default Availability=Available
// [8] QStandardItem * takeVerticalHeaderItem(int)

/*
Removes the vertical header item at row from the header without deleting it, and returns a pointer to the item. The model releases ownership of the item.

This function was introduced in  Qt 4.2.

See also verticalHeaderItem() and takeHorizontalHeaderItem().
*/
impl /*struct*/ QStandardItemModel {
  pub fn takeVerticalHeaderItem_0<RetType, T: QStandardItemModel_takeVerticalHeaderItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.takeVerticalHeaderItem_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_takeVerticalHeaderItem_0<RetType> {
  fn takeVerticalHeaderItem_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_takeVerticalHeaderItem_0<usize> for (i32) {
  fn takeVerticalHeaderItem_0(self , rsthis: & QStandardItemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QStandardItemModel22takeVerticalHeaderItemEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:407
// index:0
// Public Visibility=Default Availability=Available
// [8] const QStandardItem * itemPrototype() const

/*
Returns the item prototype used by the model. The model uses the item prototype as an item factory when it needs to construct new items on demand (for instance, when a view or item delegate calls setData()).

This function was introduced in  Qt 4.2.

See also setItemPrototype().
*/
impl /*struct*/ QStandardItemModel {
  pub fn itemPrototype_0<RetType, T: QStandardItemModel_itemPrototype_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemPrototype_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_itemPrototype_0<RetType> {
  fn itemPrototype_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_itemPrototype_0<usize> for () {
  fn itemPrototype_0(self , rsthis: & QStandardItemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QStandardItemModel13itemPrototypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:408
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setItemPrototype(const QStandardItem *)

/*
Sets the item prototype for the model to the specified item. The model takes ownership of the prototype.

The item prototype acts as a QStandardItem factory, by relying on the QStandardItem::clone() function. To provide your own prototype, subclass QStandardItem, reimplement QStandardItem::clone() and set the prototype to be an instance of your custom class. Whenever QStandardItemModel needs to create an item on demand (for instance, when a view or item delegate calls setData())), the new items will be instances of your custom class.

This function was introduced in  Qt 4.2.

See also itemPrototype() and QStandardItem::clone().
*/
impl /*struct*/ QStandardItemModel {
  pub fn setItemPrototype_0<RetType, T: QStandardItemModel_setItemPrototype_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setItemPrototype_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_setItemPrototype_0<RetType> {
  fn setItemPrototype_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_setItemPrototype_0<(/*void*/)> for (usize) {
  fn setItemPrototype_0(self , rsthis: & QStandardItemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QStandardItemModel16setItemPrototypeEPK13QStandardItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:414
// index:0
// Public Visibility=Default Availability=Available
// [4] int sortRole() const

/*

*/
impl /*struct*/ QStandardItemModel {
  pub fn sortRole_0<RetType, T: QStandardItemModel_sortRole_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sortRole_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_sortRole_0<RetType> {
  fn sortRole_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_sortRole_0<i32> for () {
  fn sortRole_0(self , rsthis: & QStandardItemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QStandardItemModel8sortRoleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:415
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSortRole(int)

/*

*/
impl /*struct*/ QStandardItemModel {
  pub fn setSortRole_0<RetType, T: QStandardItemModel_setSortRole_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSortRole_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_setSortRole_0<RetType> {
  fn setSortRole_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_setSortRole_0<(/*void*/)> for (i32) {
  fn setSortRole_0(self , rsthis: & QStandardItemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN18QStandardItemModel11setSortRoleEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:417
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QStringList mimeTypes() const

/*
Reimplemented from QAbstractItemModel::mimeTypes().
*/
impl /*struct*/ QStandardItemModel {
  pub fn mimeTypes_0<RetType, T: QStandardItemModel_mimeTypes_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mimeTypes_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_mimeTypes_0<RetType> {
  fn mimeTypes_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_mimeTypes_0<usize> for () {
  fn mimeTypes_0(self , rsthis: & QStandardItemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QStandardItemModel9mimeTypesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:419
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool dropMimeData(const QMimeData *, Qt::DropAction, int, int, const QModelIndex &)

/*
Reimplemented from QAbstractItemModel::dropMimeData().
*/
impl /*struct*/ QStandardItemModel {
  pub fn dropMimeData_0<RetType, T: QStandardItemModel_dropMimeData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dropMimeData_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_dropMimeData_0<RetType> {
  fn dropMimeData_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_dropMimeData_0<bool> for (usize,i32,i32,i32,usize) {
  fn dropMimeData_0(self , rsthis: & QStandardItemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QStandardItemModel12dropMimeDataEPK9QMimeDataN2Qt10DropActionEiiRK11QModelIndex", 5,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstandarditemmodel.h:422
// index:0
// Public Visibility=Default Availability=Available
// [-2] void itemChanged(QStandardItem *)

/*
This signal is emitted whenever the data of item has changed.

This function was introduced in  Qt 4.2.
*/
impl /*struct*/ QStandardItemModel {
  pub fn itemChanged_0<RetType, T: QStandardItemModel_itemChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemChanged_0(self);
    // return 1;
  }
}
pub trait QStandardItemModel_itemChanged_0<RetType> {
  fn itemChanged_0(self , rsthis: & QStandardItemModel) -> RetType;
}
impl<'a> /*trait*/ QStandardItemModel_itemChanged_0<(/*void*/)> for (usize) {
  fn itemChanged_0(self , rsthis: & QStandardItemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QStandardItemModel11itemChangedEP13QStandardItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
