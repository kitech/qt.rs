

// mod ::core::QIdentityProxyModel
// package qtcore
// /usr/include/qt/QtCore/qidentityproxymodel.h
// #include <qidentityproxymodel.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 13
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
#[derive(Default)] // class sizeof(QIdentityProxyModel)=16
pub struct QIdentityProxyModel {
  qbase: QAbstractProxyModel,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QIdentityProxyModel_ITF interface {
//    QAbstractProxyModel_ITF
//    QIdentityProxyModel_PTR() *QIdentityProxyModel
//}
//func (ptr *QIdentityProxyModel) QIdentityProxyModel_PTR() *QIdentityProxyModel { return ptr }

impl /*struct*/ QIdentityProxyModel {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QIdentityProxyModel {
    return QIdentityProxyModel{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QIdentityProxyModel {
//  type Target = QIdentityProxyModelBASE;
//
//  fn deref(&self) -> &QIdentityProxyModelBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QIdentityProxyModelBASE> for QIdentityProxyModel {
//  fn as_ref(& self) -> & QIdentityProxyModelBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qidentityproxymodel.h:55
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QIdentityProxyModel {
  pub fn metaObject_0<RetType, T: QIdentityProxyModel_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QIdentityProxyModel_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QIdentityProxyModel) -> RetType;
}
impl<'a> /*trait*/ QIdentityProxyModel_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QIdentityProxyModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QIdentityProxyModel10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qidentityproxymodel.h:57
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QIdentityProxyModel(QObject *)

/*
Constructs an identity model with the given parent.
*/
// QIdentityProxyModel(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QIdentityProxyModel {
  pub fn QIdentityProxyModel_0<T: QIdentityProxyModel_QIdentityProxyModel_0>(value: T) -> QIdentityProxyModel {
    let rsthis = value.QIdentityProxyModel_0();
    return rsthis;
    // return 1;
  }
}

pub trait QIdentityProxyModel_QIdentityProxyModel_0 {
  fn QIdentityProxyModel_0(self) -> QIdentityProxyModel;
}
// QIdentityProxyModel(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QIdentityProxyModel_QIdentityProxyModel_0 for (usize) {
  fn QIdentityProxyModel_0(self) -> QIdentityProxyModel {
    // unsafe{_ZN19QIdentityProxyModelC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QIdentityProxyModelC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QIdentityProxyModel{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qidentityproxymodel.h:58
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QIdentityProxyModel()

/*

*/
pub fn DeleteQIdentityProxyModel(this :*mut QIdentityProxyModel) {
    // let rv = qtrt::InvokeQtFunc6("_ZN19QIdentityProxyModelD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qidentityproxymodel.h:60
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int columnCount(const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::columnCount().
*/
impl /*struct*/ QIdentityProxyModel {
  pub fn columnCount_0<RetType, T: QIdentityProxyModel_columnCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnCount_0(self);
    // return 1;
  }
}
pub trait QIdentityProxyModel_columnCount_0<RetType> {
  fn columnCount_0(self , rsthis: & QIdentityProxyModel) -> RetType;
}
impl<'a> /*trait*/ QIdentityProxyModel_columnCount_0<i32> for (usize) {
  fn columnCount_0(self , rsthis: & QIdentityProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QIdentityProxyModel11columnCountERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qidentityproxymodel.h:61
// index:0
// Public virtual Visibility=Default Availability=Available
// [24] QModelIndex index(int, int, const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::index().
*/
impl /*struct*/ QIdentityProxyModel {
  pub fn index_0<RetType, T: QIdentityProxyModel_index_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.index_0(self);
    // return 1;
  }
}
pub trait QIdentityProxyModel_index_0<RetType> {
  fn index_0(self , rsthis: & QIdentityProxyModel) -> RetType;
}
impl<'a> /*trait*/ QIdentityProxyModel_index_0<usize> for (i32,i32,usize) {
  fn index_0(self , rsthis: & QIdentityProxyModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QIdentityProxyModel5indexEiiRK11QModelIndex", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qidentityproxymodel.h:62
// index:0
// Public virtual Visibility=Default Availability=Available
// [24] QModelIndex mapFromSource(const QModelIndex &) const

/*
Reimplemented from QAbstractProxyModel::mapFromSource().
*/
impl /*struct*/ QIdentityProxyModel {
  pub fn mapFromSource_0<RetType, T: QIdentityProxyModel_mapFromSource_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapFromSource_0(self);
    // return 1;
  }
}
pub trait QIdentityProxyModel_mapFromSource_0<RetType> {
  fn mapFromSource_0(self , rsthis: & QIdentityProxyModel) -> RetType;
}
impl<'a> /*trait*/ QIdentityProxyModel_mapFromSource_0<usize> for (usize) {
  fn mapFromSource_0(self , rsthis: & QIdentityProxyModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QIdentityProxyModel13mapFromSourceERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qidentityproxymodel.h:63
// index:0
// Public virtual Visibility=Default Availability=Available
// [24] QModelIndex mapToSource(const QModelIndex &) const

/*
Reimplemented from QAbstractProxyModel::mapToSource().
*/
impl /*struct*/ QIdentityProxyModel {
  pub fn mapToSource_0<RetType, T: QIdentityProxyModel_mapToSource_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapToSource_0(self);
    // return 1;
  }
}
pub trait QIdentityProxyModel_mapToSource_0<RetType> {
  fn mapToSource_0(self , rsthis: & QIdentityProxyModel) -> RetType;
}
impl<'a> /*trait*/ QIdentityProxyModel_mapToSource_0<usize> for (usize) {
  fn mapToSource_0(self , rsthis: & QIdentityProxyModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QIdentityProxyModel11mapToSourceERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qidentityproxymodel.h:64
// index:0
// Public virtual Visibility=Default Availability=Available
// [24] QModelIndex parent(const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::parent().
*/
impl /*struct*/ QIdentityProxyModel {
  pub fn parent_0<RetType, T: QIdentityProxyModel_parent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.parent_0(self);
    // return 1;
  }
}
pub trait QIdentityProxyModel_parent_0<RetType> {
  fn parent_0(self , rsthis: & QIdentityProxyModel) -> RetType;
}
impl<'a> /*trait*/ QIdentityProxyModel_parent_0<usize> for (usize) {
  fn parent_0(self , rsthis: & QIdentityProxyModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QIdentityProxyModel6parentERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qidentityproxymodel.h:66
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int rowCount(const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::rowCount().
*/
impl /*struct*/ QIdentityProxyModel {
  pub fn rowCount_0<RetType, T: QIdentityProxyModel_rowCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowCount_0(self);
    // return 1;
  }
}
pub trait QIdentityProxyModel_rowCount_0<RetType> {
  fn rowCount_0(self , rsthis: & QIdentityProxyModel) -> RetType;
}
impl<'a> /*trait*/ QIdentityProxyModel_rowCount_0<i32> for (usize) {
  fn rowCount_0(self , rsthis: & QIdentityProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QIdentityProxyModel8rowCountERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qidentityproxymodel.h:67
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QVariant headerData(int, Qt::Orientation, int) const

/*
Reimplemented from QAbstractItemModel::headerData().
*/
impl /*struct*/ QIdentityProxyModel {
  pub fn headerData_0<RetType, T: QIdentityProxyModel_headerData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.headerData_0(self);
    // return 1;
  }
}
pub trait QIdentityProxyModel_headerData_0<RetType> {
  fn headerData_0(self , rsthis: & QIdentityProxyModel) -> RetType;
}
impl<'a> /*trait*/ QIdentityProxyModel_headerData_0<usize> for (i32,i32,i32) {
  fn headerData_0(self , rsthis: & QIdentityProxyModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QIdentityProxyModel10headerDataEiN2Qt11OrientationEi", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qidentityproxymodel.h:68
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool dropMimeData(const QMimeData *, Qt::DropAction, int, int, const QModelIndex &)

/*
Reimplemented from QAbstractItemModel::dropMimeData().
*/
impl /*struct*/ QIdentityProxyModel {
  pub fn dropMimeData_0<RetType, T: QIdentityProxyModel_dropMimeData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dropMimeData_0(self);
    // return 1;
  }
}
pub trait QIdentityProxyModel_dropMimeData_0<RetType> {
  fn dropMimeData_0(self , rsthis: & QIdentityProxyModel) -> RetType;
}
impl<'a> /*trait*/ QIdentityProxyModel_dropMimeData_0<bool> for (usize,i32,i32,i32,usize) {
  fn dropMimeData_0(self , rsthis: & QIdentityProxyModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN19QIdentityProxyModel12dropMimeDataEPK9QMimeDataN2Qt10DropActionEiiRK11QModelIndex", 5,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qidentityproxymodel.h:69
// index:0
// Public virtual Visibility=Default Availability=Available
// [24] QModelIndex sibling(int, int, const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::sibling().
*/
impl /*struct*/ QIdentityProxyModel {
  pub fn sibling_0<RetType, T: QIdentityProxyModel_sibling_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sibling_0(self);
    // return 1;
  }
}
pub trait QIdentityProxyModel_sibling_0<RetType> {
  fn sibling_0(self , rsthis: & QIdentityProxyModel) -> RetType;
}
impl<'a> /*trait*/ QIdentityProxyModel_sibling_0<usize> for (i32,i32,usize) {
  fn sibling_0(self , rsthis: & QIdentityProxyModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QIdentityProxyModel7siblingEiiRK11QModelIndex", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qidentityproxymodel.h:71
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QItemSelection mapSelectionFromSource(const QItemSelection &) const

/*
Reimplemented from QAbstractProxyModel::mapSelectionFromSource().
*/
impl /*struct*/ QIdentityProxyModel {
  pub fn mapSelectionFromSource_0<RetType, T: QIdentityProxyModel_mapSelectionFromSource_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapSelectionFromSource_0(self);
    // return 1;
  }
}
pub trait QIdentityProxyModel_mapSelectionFromSource_0<RetType> {
  fn mapSelectionFromSource_0(self , rsthis: & QIdentityProxyModel) -> RetType;
}
impl<'a> /*trait*/ QIdentityProxyModel_mapSelectionFromSource_0<usize> for (usize) {
  fn mapSelectionFromSource_0(self , rsthis: & QIdentityProxyModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QIdentityProxyModel22mapSelectionFromSourceERK14QItemSelection", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qidentityproxymodel.h:72
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QItemSelection mapSelectionToSource(const QItemSelection &) const

/*
Reimplemented from QAbstractProxyModel::mapSelectionToSource().
*/
impl /*struct*/ QIdentityProxyModel {
  pub fn mapSelectionToSource_0<RetType, T: QIdentityProxyModel_mapSelectionToSource_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapSelectionToSource_0(self);
    // return 1;
  }
}
pub trait QIdentityProxyModel_mapSelectionToSource_0<RetType> {
  fn mapSelectionToSource_0(self , rsthis: & QIdentityProxyModel) -> RetType;
}
impl<'a> /*trait*/ QIdentityProxyModel_mapSelectionToSource_0<usize> for (usize) {
  fn mapSelectionToSource_0(self , rsthis: & QIdentityProxyModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QIdentityProxyModel20mapSelectionToSourceERK14QItemSelection", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qidentityproxymodel.h:73
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QModelIndexList match(const QModelIndex &, int, const QVariant &, int, Qt::MatchFlags) const

/*
Reimplemented from QAbstractItemModel::match().
*/
impl /*struct*/ QIdentityProxyModel {
  pub fn match__0<RetType, T: QIdentityProxyModel_match__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.match__0(self);
    // return 1;
  }
}
pub trait QIdentityProxyModel_match__0<RetType> {
  fn match__0(self , rsthis: & QIdentityProxyModel) -> RetType;
}
impl<'a> /*trait*/ QIdentityProxyModel_match__0<usize> for (usize,i32,usize,i32,i32) {
  fn match__0(self , rsthis: & QIdentityProxyModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QIdentityProxyModel5matchERK11QModelIndexiRK8QVarianti6QFlagsIN2Qt9MatchFlagEE", 5,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qidentityproxymodel.h:74
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setSourceModel(QAbstractItemModel *)

/*
Reimplemented from QAbstractProxyModel::setSourceModel().
*/
impl /*struct*/ QIdentityProxyModel {
  pub fn setSourceModel_0<RetType, T: QIdentityProxyModel_setSourceModel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSourceModel_0(self);
    // return 1;
  }
}
pub trait QIdentityProxyModel_setSourceModel_0<RetType> {
  fn setSourceModel_0(self , rsthis: & QIdentityProxyModel) -> RetType;
}
impl<'a> /*trait*/ QIdentityProxyModel_setSourceModel_0<(/*void*/)> for (usize) {
  fn setSourceModel_0(self , rsthis: & QIdentityProxyModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QIdentityProxyModel14setSourceModelEP18QAbstractItemModel", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qidentityproxymodel.h:76
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool insertColumns(int, int, const QModelIndex &)

/*
Reimplemented from QAbstractItemModel::insertColumns().
*/
impl /*struct*/ QIdentityProxyModel {
  pub fn insertColumns_0<RetType, T: QIdentityProxyModel_insertColumns_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertColumns_0(self);
    // return 1;
  }
}
pub trait QIdentityProxyModel_insertColumns_0<RetType> {
  fn insertColumns_0(self , rsthis: & QIdentityProxyModel) -> RetType;
}
impl<'a> /*trait*/ QIdentityProxyModel_insertColumns_0<bool> for (i32,i32,usize) {
  fn insertColumns_0(self , rsthis: & QIdentityProxyModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN19QIdentityProxyModel13insertColumnsEiiRK11QModelIndex", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qidentityproxymodel.h:77
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool insertRows(int, int, const QModelIndex &)

/*
Reimplemented from QAbstractItemModel::insertRows().
*/
impl /*struct*/ QIdentityProxyModel {
  pub fn insertRows_0<RetType, T: QIdentityProxyModel_insertRows_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertRows_0(self);
    // return 1;
  }
}
pub trait QIdentityProxyModel_insertRows_0<RetType> {
  fn insertRows_0(self , rsthis: & QIdentityProxyModel) -> RetType;
}
impl<'a> /*trait*/ QIdentityProxyModel_insertRows_0<bool> for (i32,i32,usize) {
  fn insertRows_0(self , rsthis: & QIdentityProxyModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN19QIdentityProxyModel10insertRowsEiiRK11QModelIndex", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qidentityproxymodel.h:78
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool removeColumns(int, int, const QModelIndex &)

/*
Reimplemented from QAbstractItemModel::removeColumns().
*/
impl /*struct*/ QIdentityProxyModel {
  pub fn removeColumns_0<RetType, T: QIdentityProxyModel_removeColumns_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeColumns_0(self);
    // return 1;
  }
}
pub trait QIdentityProxyModel_removeColumns_0<RetType> {
  fn removeColumns_0(self , rsthis: & QIdentityProxyModel) -> RetType;
}
impl<'a> /*trait*/ QIdentityProxyModel_removeColumns_0<bool> for (i32,i32,usize) {
  fn removeColumns_0(self , rsthis: & QIdentityProxyModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN19QIdentityProxyModel13removeColumnsEiiRK11QModelIndex", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qidentityproxymodel.h:79
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool removeRows(int, int, const QModelIndex &)

/*
Reimplemented from QAbstractItemModel::removeRows().
*/
impl /*struct*/ QIdentityProxyModel {
  pub fn removeRows_0<RetType, T: QIdentityProxyModel_removeRows_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeRows_0(self);
    // return 1;
  }
}
pub trait QIdentityProxyModel_removeRows_0<RetType> {
  fn removeRows_0(self , rsthis: & QIdentityProxyModel) -> RetType;
}
impl<'a> /*trait*/ QIdentityProxyModel_removeRows_0<bool> for (i32,i32,usize) {
  fn removeRows_0(self , rsthis: & QIdentityProxyModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN19QIdentityProxyModel10removeRowsEiiRK11QModelIndex", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
