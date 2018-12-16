

// mod ::core::QSortFilterProxyModel
// package qtcore
// /usr/include/qt/QtCore/qsortfilterproxymodel.h
// #include <qsortfilterproxymodel.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 8
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

// bool filterAcceptsRow(int, const QModelIndex &)
// func (this *QSortFilterProxyModel) InheritFilterAcceptsRow(f func(source_row int, source_parent *QModelIndex) bool) {
//  qtrt.SetAllInheritCallback(this, "filterAcceptsRow", f)
// }

// bool filterAcceptsColumn(int, const QModelIndex &)
// func (this *QSortFilterProxyModel) InheritFilterAcceptsColumn(f func(source_column int, source_parent *QModelIndex) bool) {
//  qtrt.SetAllInheritCallback(this, "filterAcceptsColumn", f)
// }

// bool lessThan(const QModelIndex &, const QModelIndex &)
// func (this *QSortFilterProxyModel) InheritLessThan(f func(source_left *QModelIndex, source_right *QModelIndex) bool) {
//  qtrt.SetAllInheritCallback(this, "lessThan", f)
// }

// void filterChanged()
// func (this *QSortFilterProxyModel) InheritFilterChanged(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "filterChanged", f)
// }

// void invalidateFilter()
// func (this *QSortFilterProxyModel) InheritInvalidateFilter(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "invalidateFilter", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QSortFilterProxyModel)=16
pub struct QSortFilterProxyModel {
  qbase: QAbstractProxyModel,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QSortFilterProxyModel_ITF interface {
//    QAbstractProxyModel_ITF
//    QSortFilterProxyModel_PTR() *QSortFilterProxyModel
//}
//func (ptr *QSortFilterProxyModel) QSortFilterProxyModel_PTR() *QSortFilterProxyModel { return ptr }

impl /*struct*/ QSortFilterProxyModel {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QSortFilterProxyModel {
    return QSortFilterProxyModel{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QSortFilterProxyModel {
//  type Target = QSortFilterProxyModelBASE;
//
//  fn deref(&self) -> &QSortFilterProxyModelBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QSortFilterProxyModelBASE> for QSortFilterProxyModel {
//  fn as_ref(& self) -> & QSortFilterProxyModelBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qsortfilterproxymodel.h:61
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn metaObject_0<RetType, T: QSortFilterProxyModel_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QSortFilterProxyModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QSortFilterProxyModel10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:73
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QSortFilterProxyModel(QObject *)

/*
Constructs a sorting filter model with the given parent.
*/
// QSortFilterProxyModel(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QSortFilterProxyModel {
  pub fn QSortFilterProxyModel_0<T: QSortFilterProxyModel_QSortFilterProxyModel_0>(value: T) -> QSortFilterProxyModel {
    let rsthis = value.QSortFilterProxyModel_0();
    return rsthis;
    // return 1;
  }
}

pub trait QSortFilterProxyModel_QSortFilterProxyModel_0 {
  fn QSortFilterProxyModel_0(self) -> QSortFilterProxyModel;
}
// QSortFilterProxyModel(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSortFilterProxyModel_QSortFilterProxyModel_0 for (usize) {
  fn QSortFilterProxyModel_0(self) -> QSortFilterProxyModel {
    // unsafe{_ZN21QSortFilterProxyModelC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN21QSortFilterProxyModelC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSortFilterProxyModel{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:74
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QSortFilterProxyModel()

/*

*/
pub fn DeleteQSortFilterProxyModel(this :*mut QSortFilterProxyModel) {
    // let rv = qtrt::InvokeQtFunc6("_ZN21QSortFilterProxyModelD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qsortfilterproxymodel.h:76
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setSourceModel(QAbstractItemModel *)

/*
Reimplemented from QAbstractProxyModel::setSourceModel().
*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn setSourceModel_0<RetType, T: QSortFilterProxyModel_setSourceModel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSourceModel_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_setSourceModel_0<RetType> {
  fn setSourceModel_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_setSourceModel_0<(/*void*/)> for (usize) {
  fn setSourceModel_0(self , rsthis: & QSortFilterProxyModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN21QSortFilterProxyModel14setSourceModelEP18QAbstractItemModel", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:78
// index:0
// Public virtual Visibility=Default Availability=Available
// [24] QModelIndex mapToSource(const QModelIndex &) const

/*
Reimplemented from QAbstractProxyModel::mapToSource().

Returns the source model index corresponding to the given proxyIndex from the sorting filter model.

See also mapFromSource().
*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn mapToSource_0<RetType, T: QSortFilterProxyModel_mapToSource_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapToSource_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_mapToSource_0<RetType> {
  fn mapToSource_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_mapToSource_0<usize> for (usize) {
  fn mapToSource_0(self , rsthis: & QSortFilterProxyModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QSortFilterProxyModel11mapToSourceERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:79
// index:0
// Public virtual Visibility=Default Availability=Available
// [24] QModelIndex mapFromSource(const QModelIndex &) const

/*
Reimplemented from QAbstractProxyModel::mapFromSource().

Returns the model index in the QSortFilterProxyModel given the sourceIndex from the source model.

See also mapToSource().
*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn mapFromSource_0<RetType, T: QSortFilterProxyModel_mapFromSource_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapFromSource_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_mapFromSource_0<RetType> {
  fn mapFromSource_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_mapFromSource_0<usize> for (usize) {
  fn mapFromSource_0(self , rsthis: & QSortFilterProxyModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QSortFilterProxyModel13mapFromSourceERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:81
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QItemSelection mapSelectionToSource(const QItemSelection &) const

/*
Reimplemented from QAbstractProxyModel::mapSelectionToSource().
*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn mapSelectionToSource_0<RetType, T: QSortFilterProxyModel_mapSelectionToSource_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapSelectionToSource_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_mapSelectionToSource_0<RetType> {
  fn mapSelectionToSource_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_mapSelectionToSource_0<usize> for (usize) {
  fn mapSelectionToSource_0(self , rsthis: & QSortFilterProxyModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QSortFilterProxyModel20mapSelectionToSourceERK14QItemSelection", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:82
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QItemSelection mapSelectionFromSource(const QItemSelection &) const

/*
Reimplemented from QAbstractProxyModel::mapSelectionFromSource().
*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn mapSelectionFromSource_0<RetType, T: QSortFilterProxyModel_mapSelectionFromSource_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapSelectionFromSource_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_mapSelectionFromSource_0<RetType> {
  fn mapSelectionFromSource_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_mapSelectionFromSource_0<usize> for (usize) {
  fn mapSelectionFromSource_0(self , rsthis: & QSortFilterProxyModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QSortFilterProxyModel22mapSelectionFromSourceERK14QItemSelection", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:84
// index:0
// Public Visibility=Default Availability=Available
// [8] QRegExp filterRegExp() const

/*

*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn filterRegExp_0<RetType, T: QSortFilterProxyModel_filterRegExp_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.filterRegExp_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_filterRegExp_0<RetType> {
  fn filterRegExp_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_filterRegExp_0<usize> for () {
  fn filterRegExp_0(self , rsthis: & QSortFilterProxyModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QSortFilterProxyModel12filterRegExpEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:85
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFilterRegExp(const QRegExp &)

/*

*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn setFilterRegExp_0<RetType, T: QSortFilterProxyModel_setFilterRegExp_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFilterRegExp_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_setFilterRegExp_0<RetType> {
  fn setFilterRegExp_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_setFilterRegExp_0<(/*void*/)> for (usize) {
  fn setFilterRegExp_0(self , rsthis: & QSortFilterProxyModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN21QSortFilterProxyModel15setFilterRegExpERK7QRegExp", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:115
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setFilterRegExp(const QString &)

/*

*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn setFilterRegExp_1<RetType, T: QSortFilterProxyModel_setFilterRegExp_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFilterRegExp_1(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_setFilterRegExp_1<RetType> {
  fn setFilterRegExp_1(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_setFilterRegExp_1<(/*void*/)> for (usize) {
  fn setFilterRegExp_1(self , rsthis: & QSortFilterProxyModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN21QSortFilterProxyModel15setFilterRegExpERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:87
// index:0
// Public Visibility=Default Availability=Available
// [4] int filterKeyColumn() const

/*

*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn filterKeyColumn_0<RetType, T: QSortFilterProxyModel_filterKeyColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.filterKeyColumn_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_filterKeyColumn_0<RetType> {
  fn filterKeyColumn_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_filterKeyColumn_0<i32> for () {
  fn filterKeyColumn_0(self , rsthis: & QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QSortFilterProxyModel15filterKeyColumnEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:88
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFilterKeyColumn(int)

/*

*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn setFilterKeyColumn_0<RetType, T: QSortFilterProxyModel_setFilterKeyColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFilterKeyColumn_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_setFilterKeyColumn_0<RetType> {
  fn setFilterKeyColumn_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_setFilterKeyColumn_0<(/*void*/)> for (i32) {
  fn setFilterKeyColumn_0(self , rsthis: & QSortFilterProxyModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN21QSortFilterProxyModel18setFilterKeyColumnEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:90
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::CaseSensitivity filterCaseSensitivity() const

/*

*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn filterCaseSensitivity_0<RetType, T: QSortFilterProxyModel_filterCaseSensitivity_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.filterCaseSensitivity_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_filterCaseSensitivity_0<RetType> {
  fn filterCaseSensitivity_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_filterCaseSensitivity_0<i32> for () {
  fn filterCaseSensitivity_0(self , rsthis: & QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QSortFilterProxyModel21filterCaseSensitivityEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:91
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFilterCaseSensitivity(Qt::CaseSensitivity)

/*

*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn setFilterCaseSensitivity_0<RetType, T: QSortFilterProxyModel_setFilterCaseSensitivity_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFilterCaseSensitivity_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_setFilterCaseSensitivity_0<RetType> {
  fn setFilterCaseSensitivity_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_setFilterCaseSensitivity_0<(/*void*/)> for (i32) {
  fn setFilterCaseSensitivity_0(self , rsthis: & QSortFilterProxyModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN21QSortFilterProxyModel24setFilterCaseSensitivityEN2Qt15CaseSensitivityE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:93
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::CaseSensitivity sortCaseSensitivity() const

/*

*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn sortCaseSensitivity_0<RetType, T: QSortFilterProxyModel_sortCaseSensitivity_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sortCaseSensitivity_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_sortCaseSensitivity_0<RetType> {
  fn sortCaseSensitivity_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_sortCaseSensitivity_0<i32> for () {
  fn sortCaseSensitivity_0(self , rsthis: & QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QSortFilterProxyModel19sortCaseSensitivityEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:94
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSortCaseSensitivity(Qt::CaseSensitivity)

/*

*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn setSortCaseSensitivity_0<RetType, T: QSortFilterProxyModel_setSortCaseSensitivity_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSortCaseSensitivity_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_setSortCaseSensitivity_0<RetType> {
  fn setSortCaseSensitivity_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_setSortCaseSensitivity_0<(/*void*/)> for (i32) {
  fn setSortCaseSensitivity_0(self , rsthis: & QSortFilterProxyModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN21QSortFilterProxyModel22setSortCaseSensitivityEN2Qt15CaseSensitivityE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:96
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isSortLocaleAware() const

/*

*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn isSortLocaleAware_0<RetType, T: QSortFilterProxyModel_isSortLocaleAware_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isSortLocaleAware_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_isSortLocaleAware_0<RetType> {
  fn isSortLocaleAware_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_isSortLocaleAware_0<bool> for () {
  fn isSortLocaleAware_0(self , rsthis: & QSortFilterProxyModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QSortFilterProxyModel17isSortLocaleAwareEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:97
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSortLocaleAware(bool)

/*

*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn setSortLocaleAware_0<RetType, T: QSortFilterProxyModel_setSortLocaleAware_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSortLocaleAware_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_setSortLocaleAware_0<RetType> {
  fn setSortLocaleAware_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_setSortLocaleAware_0<(/*void*/)> for (bool) {
  fn setSortLocaleAware_0(self , rsthis: & QSortFilterProxyModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN21QSortFilterProxyModel18setSortLocaleAwareEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:99
// index:0
// Public Visibility=Default Availability=Available
// [4] int sortColumn() const

/*
the column currently used for sorting

This returns the most recently used sort column.

This function was introduced in  Qt 4.5.
*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn sortColumn_0<RetType, T: QSortFilterProxyModel_sortColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sortColumn_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_sortColumn_0<RetType> {
  fn sortColumn_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_sortColumn_0<i32> for () {
  fn sortColumn_0(self , rsthis: & QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QSortFilterProxyModel10sortColumnEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:100
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::SortOrder sortOrder() const

/*
the order currently used for sorting

This returns the most recently used sort order.

This function was introduced in  Qt 4.5.
*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn sortOrder_0<RetType, T: QSortFilterProxyModel_sortOrder_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sortOrder_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_sortOrder_0<RetType> {
  fn sortOrder_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_sortOrder_0<i32> for () {
  fn sortOrder_0(self , rsthis: & QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QSortFilterProxyModel9sortOrderEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:102
// index:0
// Public Visibility=Default Availability=Available
// [1] bool dynamicSortFilter() const

/*

*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn dynamicSortFilter_0<RetType, T: QSortFilterProxyModel_dynamicSortFilter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dynamicSortFilter_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_dynamicSortFilter_0<RetType> {
  fn dynamicSortFilter_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_dynamicSortFilter_0<bool> for () {
  fn dynamicSortFilter_0(self , rsthis: & QSortFilterProxyModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QSortFilterProxyModel17dynamicSortFilterEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:103
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDynamicSortFilter(bool)

/*

*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn setDynamicSortFilter_0<RetType, T: QSortFilterProxyModel_setDynamicSortFilter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDynamicSortFilter_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_setDynamicSortFilter_0<RetType> {
  fn setDynamicSortFilter_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_setDynamicSortFilter_0<(/*void*/)> for (bool) {
  fn setDynamicSortFilter_0(self , rsthis: & QSortFilterProxyModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN21QSortFilterProxyModel20setDynamicSortFilterEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:105
// index:0
// Public Visibility=Default Availability=Available
// [4] int sortRole() const

/*

*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn sortRole_0<RetType, T: QSortFilterProxyModel_sortRole_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sortRole_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_sortRole_0<RetType> {
  fn sortRole_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_sortRole_0<i32> for () {
  fn sortRole_0(self , rsthis: & QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QSortFilterProxyModel8sortRoleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:106
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSortRole(int)

/*

*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn setSortRole_0<RetType, T: QSortFilterProxyModel_setSortRole_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSortRole_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_setSortRole_0<RetType> {
  fn setSortRole_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_setSortRole_0<(/*void*/)> for (i32) {
  fn setSortRole_0(self , rsthis: & QSortFilterProxyModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN21QSortFilterProxyModel11setSortRoleEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:108
// index:0
// Public Visibility=Default Availability=Available
// [4] int filterRole() const

/*

*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn filterRole_0<RetType, T: QSortFilterProxyModel_filterRole_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.filterRole_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_filterRole_0<RetType> {
  fn filterRole_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_filterRole_0<i32> for () {
  fn filterRole_0(self , rsthis: & QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QSortFilterProxyModel10filterRoleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:109
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFilterRole(int)

/*

*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn setFilterRole_0<RetType, T: QSortFilterProxyModel_setFilterRole_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFilterRole_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_setFilterRole_0<RetType> {
  fn setFilterRole_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_setFilterRole_0<(/*void*/)> for (i32) {
  fn setFilterRole_0(self , rsthis: & QSortFilterProxyModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN21QSortFilterProxyModel13setFilterRoleEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:111
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isRecursiveFilteringEnabled() const

/*

*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn isRecursiveFilteringEnabled_0<RetType, T: QSortFilterProxyModel_isRecursiveFilteringEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isRecursiveFilteringEnabled_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_isRecursiveFilteringEnabled_0<RetType> {
  fn isRecursiveFilteringEnabled_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_isRecursiveFilteringEnabled_0<bool> for () {
  fn isRecursiveFilteringEnabled_0(self , rsthis: & QSortFilterProxyModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QSortFilterProxyModel27isRecursiveFilteringEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:112
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRecursiveFilteringEnabled(bool)

/*

*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn setRecursiveFilteringEnabled_0<RetType, T: QSortFilterProxyModel_setRecursiveFilteringEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRecursiveFilteringEnabled_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_setRecursiveFilteringEnabled_0<RetType> {
  fn setRecursiveFilteringEnabled_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_setRecursiveFilteringEnabled_0<(/*void*/)> for (bool) {
  fn setRecursiveFilteringEnabled_0(self , rsthis: & QSortFilterProxyModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN21QSortFilterProxyModel28setRecursiveFilteringEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:116
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFilterWildcard(const QString &)

/*
Sets the wildcard expression used to filter the contents of the source model to the given pattern.

See also setFilterCaseSensitivity(), setFilterRegExp(), setFilterFixedString(), and filterRegExp().
*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn setFilterWildcard_0<RetType, T: QSortFilterProxyModel_setFilterWildcard_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFilterWildcard_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_setFilterWildcard_0<RetType> {
  fn setFilterWildcard_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_setFilterWildcard_0<(/*void*/)> for (usize) {
  fn setFilterWildcard_0(self , rsthis: & QSortFilterProxyModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN21QSortFilterProxyModel17setFilterWildcardERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:117
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFilterFixedString(const QString &)

/*
Sets the fixed string used to filter the contents of the source model to the given pattern.

See also setFilterCaseSensitivity(), setFilterRegExp(), setFilterWildcard(), and filterRegExp().
*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn setFilterFixedString_0<RetType, T: QSortFilterProxyModel_setFilterFixedString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFilterFixedString_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_setFilterFixedString_0<RetType> {
  fn setFilterFixedString_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_setFilterFixedString_0<(/*void*/)> for (usize) {
  fn setFilterFixedString_0(self , rsthis: & QSortFilterProxyModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN21QSortFilterProxyModel20setFilterFixedStringERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:118
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clear()

/*

*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn clear_0<RetType, T: QSortFilterProxyModel_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_clear_0<RetType> {
  fn clear_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_clear_0<(/*void*/)> for () {
  fn clear_0(self , rsthis: & QSortFilterProxyModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN21QSortFilterProxyModel5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:119
// index:0
// Public Visibility=Default Availability=Available
// [-2] void invalidate()

/*
Invalidates the current sorting and filtering.

This function was introduced in  Qt 4.3.

See also invalidateFilter().
*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn invalidate_0<RetType, T: QSortFilterProxyModel_invalidate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.invalidate_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_invalidate_0<RetType> {
  fn invalidate_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_invalidate_0<(/*void*/)> for () {
  fn invalidate_0(self , rsthis: & QSortFilterProxyModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN21QSortFilterProxyModel10invalidateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:122
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool filterAcceptsRow(int, const QModelIndex &) const

/*
Returns true if the item in the row indicated by the given source_row and source_parent should be included in the model; otherwise returns false.

The default implementation returns true if the value held by the relevant item matches the filter string, wildcard string or regular expression.

Note: By default, the Qt::DisplayRole is used to determine if the row should be accepted or not. This can be changed by setting the filterRole property.

See also filterAcceptsColumn(), setFilterFixedString(), setFilterRegExp(), and setFilterWildcard().
*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn filterAcceptsRow_0<RetType, T: QSortFilterProxyModel_filterAcceptsRow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.filterAcceptsRow_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_filterAcceptsRow_0<RetType> {
  fn filterAcceptsRow_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_filterAcceptsRow_0<bool> for (i32,usize) {
  fn filterAcceptsRow_0(self , rsthis: & QSortFilterProxyModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QSortFilterProxyModel16filterAcceptsRowEiRK11QModelIndex", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:123
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool filterAcceptsColumn(int, const QModelIndex &) const

/*
Returns true if the item in the column indicated by the given source_column and source_parent should be included in the model; otherwise returns false.

The default implementation returns true if the value held by the relevant item matches the filter string, wildcard string or regular expression.

Note: By default, the Qt::DisplayRole is used to determine if the column should be accepted or not. This can be changed by setting the filterRole property.

See also filterAcceptsRow(), setFilterFixedString(), setFilterRegExp(), and setFilterWildcard().
*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn filterAcceptsColumn_0<RetType, T: QSortFilterProxyModel_filterAcceptsColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.filterAcceptsColumn_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_filterAcceptsColumn_0<RetType> {
  fn filterAcceptsColumn_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_filterAcceptsColumn_0<bool> for (i32,usize) {
  fn filterAcceptsColumn_0(self , rsthis: & QSortFilterProxyModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QSortFilterProxyModel19filterAcceptsColumnEiRK11QModelIndex", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:124
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool lessThan(const QModelIndex &, const QModelIndex &) const

/*
Returns true if the value of the item referred to by the given index source_left is less than the value of the item referred to by the given index source_right, otherwise returns false.

This function is used as the < operator when sorting, and handles the following QVariant types:


QMetaType::Int
QMetaType::UInt
QMetaType::LongLong
QMetaType::ULongLong
QMetaType::Float
QMetaType::Double
QMetaType::QChar
QMetaType::QDate
QMetaType::QTime
QMetaType::QDateTime
QMetaType::QString


Any other type will be converted to a QString using QVariant::toString().

Comparison of QStrings is case sensitive by default; this can be changed using the sortCaseSensitivity property.

By default, the Qt::DisplayRole associated with the QModelIndexes is used for comparisons. This can be changed by setting the sortRole property.

Note: The indices passed in correspond to the source model.

See also sortRole, sortCaseSensitivity, and dynamicSortFilter.
*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn lessThan_0<RetType, T: QSortFilterProxyModel_lessThan_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lessThan_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_lessThan_0<RetType> {
  fn lessThan_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_lessThan_0<bool> for (usize,usize) {
  fn lessThan_0(self , rsthis: & QSortFilterProxyModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QSortFilterProxyModel8lessThanERK11QModelIndexS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:126
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void filterChanged()

/*

*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn filterChanged_0<RetType, T: QSortFilterProxyModel_filterChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.filterChanged_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_filterChanged_0<RetType> {
  fn filterChanged_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_filterChanged_0<(/*void*/)> for () {
  fn filterChanged_0(self , rsthis: & QSortFilterProxyModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN21QSortFilterProxyModel13filterChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:127
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void invalidateFilter()

/*
Invalidates the current filtering.

This function should be called if you are implementing custom filtering (e.g. filterAcceptsRow()), and your filter parameters have changed.

This function was introduced in  Qt 4.3.

See also invalidate().
*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn invalidateFilter_0<RetType, T: QSortFilterProxyModel_invalidateFilter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.invalidateFilter_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_invalidateFilter_0<RetType> {
  fn invalidateFilter_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_invalidateFilter_0<(/*void*/)> for () {
  fn invalidateFilter_0(self , rsthis: & QSortFilterProxyModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN21QSortFilterProxyModel16invalidateFilterEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:132
// index:0
// Public virtual Visibility=Default Availability=Available
// [24] QModelIndex index(int, int, const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::index().
*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn index_0<RetType, T: QSortFilterProxyModel_index_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.index_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_index_0<RetType> {
  fn index_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_index_0<usize> for (i32,i32,usize) {
  fn index_0(self , rsthis: & QSortFilterProxyModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QSortFilterProxyModel5indexEiiRK11QModelIndex", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:133
// index:0
// Public virtual Visibility=Default Availability=Available
// [24] QModelIndex parent(const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::parent().
*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn parent_0<RetType, T: QSortFilterProxyModel_parent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.parent_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_parent_0<RetType> {
  fn parent_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_parent_0<usize> for (usize) {
  fn parent_0(self , rsthis: & QSortFilterProxyModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QSortFilterProxyModel6parentERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:134
// index:0
// Public virtual Visibility=Default Availability=Available
// [24] QModelIndex sibling(int, int, const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::sibling().
*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn sibling_0<RetType, T: QSortFilterProxyModel_sibling_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sibling_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_sibling_0<RetType> {
  fn sibling_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_sibling_0<usize> for (i32,i32,usize) {
  fn sibling_0(self , rsthis: & QSortFilterProxyModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QSortFilterProxyModel7siblingEiiRK11QModelIndex", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:136
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int rowCount(const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::rowCount().
*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn rowCount_0<RetType, T: QSortFilterProxyModel_rowCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowCount_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_rowCount_0<RetType> {
  fn rowCount_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_rowCount_0<i32> for (usize) {
  fn rowCount_0(self , rsthis: & QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QSortFilterProxyModel8rowCountERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:137
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int columnCount(const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::columnCount().
*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn columnCount_0<RetType, T: QSortFilterProxyModel_columnCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnCount_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_columnCount_0<RetType> {
  fn columnCount_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_columnCount_0<i32> for (usize) {
  fn columnCount_0(self , rsthis: & QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QSortFilterProxyModel11columnCountERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:138
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool hasChildren(const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::hasChildren().
*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn hasChildren_0<RetType, T: QSortFilterProxyModel_hasChildren_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasChildren_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_hasChildren_0<RetType> {
  fn hasChildren_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_hasChildren_0<bool> for (usize) {
  fn hasChildren_0(self , rsthis: & QSortFilterProxyModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QSortFilterProxyModel11hasChildrenERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:140
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QVariant data(const QModelIndex &, int) const

/*
Reimplemented from QAbstractItemModel::data().

See also setData().
*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn data_0<RetType, T: QSortFilterProxyModel_data_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.data_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_data_0<RetType> {
  fn data_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_data_0<usize> for (usize,i32) {
  fn data_0(self , rsthis: & QSortFilterProxyModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QSortFilterProxyModel4dataERK11QModelIndexi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:141
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool setData(const QModelIndex &, const QVariant &, int)

/*
Reimplemented from QAbstractItemModel::setData().

See also data().
*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn setData_0<RetType, T: QSortFilterProxyModel_setData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setData_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_setData_0<RetType> {
  fn setData_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_setData_0<bool> for (usize,usize,i32) {
  fn setData_0(self , rsthis: & QSortFilterProxyModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN21QSortFilterProxyModel7setDataERK11QModelIndexRK8QVarianti", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:143
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QVariant headerData(int, Qt::Orientation, int) const

/*
Reimplemented from QAbstractItemModel::headerData().

See also setHeaderData().
*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn headerData_0<RetType, T: QSortFilterProxyModel_headerData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.headerData_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_headerData_0<RetType> {
  fn headerData_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_headerData_0<usize> for (i32,i32,i32) {
  fn headerData_0(self , rsthis: & QSortFilterProxyModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QSortFilterProxyModel10headerDataEiN2Qt11OrientationEi", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:144
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool setHeaderData(int, Qt::Orientation, const QVariant &, int)

/*
Reimplemented from QAbstractItemModel::setHeaderData().

See also headerData().
*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn setHeaderData_0<RetType, T: QSortFilterProxyModel_setHeaderData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHeaderData_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_setHeaderData_0<RetType> {
  fn setHeaderData_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_setHeaderData_0<bool> for (i32,i32,usize,i32) {
  fn setHeaderData_0(self , rsthis: & QSortFilterProxyModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN21QSortFilterProxyModel13setHeaderDataEiN2Qt11OrientationERK8QVarianti", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:148
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool dropMimeData(const QMimeData *, Qt::DropAction, int, int, const QModelIndex &)

/*
Reimplemented from QAbstractItemModel::dropMimeData().
*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn dropMimeData_0<RetType, T: QSortFilterProxyModel_dropMimeData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dropMimeData_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_dropMimeData_0<RetType> {
  fn dropMimeData_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_dropMimeData_0<bool> for (usize,i32,i32,i32,usize) {
  fn dropMimeData_0(self , rsthis: & QSortFilterProxyModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN21QSortFilterProxyModel12dropMimeDataEPK9QMimeDataN2Qt10DropActionEiiRK11QModelIndex", 5,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:151
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool insertRows(int, int, const QModelIndex &)

/*
Reimplemented from QAbstractItemModel::insertRows().
*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn insertRows_0<RetType, T: QSortFilterProxyModel_insertRows_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertRows_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_insertRows_0<RetType> {
  fn insertRows_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_insertRows_0<bool> for (i32,i32,usize) {
  fn insertRows_0(self , rsthis: & QSortFilterProxyModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN21QSortFilterProxyModel10insertRowsEiiRK11QModelIndex", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:152
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool insertColumns(int, int, const QModelIndex &)

/*
Reimplemented from QAbstractItemModel::insertColumns().
*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn insertColumns_0<RetType, T: QSortFilterProxyModel_insertColumns_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertColumns_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_insertColumns_0<RetType> {
  fn insertColumns_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_insertColumns_0<bool> for (i32,i32,usize) {
  fn insertColumns_0(self , rsthis: & QSortFilterProxyModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN21QSortFilterProxyModel13insertColumnsEiiRK11QModelIndex", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:153
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool removeRows(int, int, const QModelIndex &)

/*
Reimplemented from QAbstractItemModel::removeRows().
*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn removeRows_0<RetType, T: QSortFilterProxyModel_removeRows_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeRows_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_removeRows_0<RetType> {
  fn removeRows_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_removeRows_0<bool> for (i32,i32,usize) {
  fn removeRows_0(self , rsthis: & QSortFilterProxyModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN21QSortFilterProxyModel10removeRowsEiiRK11QModelIndex", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:154
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool removeColumns(int, int, const QModelIndex &)

/*
Reimplemented from QAbstractItemModel::removeColumns().
*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn removeColumns_0<RetType, T: QSortFilterProxyModel_removeColumns_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeColumns_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_removeColumns_0<RetType> {
  fn removeColumns_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_removeColumns_0<bool> for (i32,i32,usize) {
  fn removeColumns_0(self , rsthis: & QSortFilterProxyModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN21QSortFilterProxyModel13removeColumnsEiiRK11QModelIndex", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:156
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void fetchMore(const QModelIndex &)

/*
Reimplemented from QAbstractItemModel::fetchMore().
*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn fetchMore_0<RetType, T: QSortFilterProxyModel_fetchMore_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fetchMore_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_fetchMore_0<RetType> {
  fn fetchMore_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_fetchMore_0<(/*void*/)> for (usize) {
  fn fetchMore_0(self , rsthis: & QSortFilterProxyModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN21QSortFilterProxyModel9fetchMoreERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:157
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool canFetchMore(const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::canFetchMore().
*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn canFetchMore_0<RetType, T: QSortFilterProxyModel_canFetchMore_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.canFetchMore_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_canFetchMore_0<RetType> {
  fn canFetchMore_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_canFetchMore_0<bool> for (usize) {
  fn canFetchMore_0(self , rsthis: & QSortFilterProxyModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QSortFilterProxyModel12canFetchMoreERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:158
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] Qt::ItemFlags flags(const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::flags().
*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn flags_0<RetType, T: QSortFilterProxyModel_flags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.flags_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_flags_0<RetType> {
  fn flags_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_flags_0<i32> for (usize) {
  fn flags_0(self , rsthis: & QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QSortFilterProxyModel5flagsERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:160
// index:0
// Public virtual Visibility=Default Availability=Available
// [24] QModelIndex buddy(const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::buddy().
*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn buddy_0<RetType, T: QSortFilterProxyModel_buddy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.buddy_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_buddy_0<RetType> {
  fn buddy_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_buddy_0<usize> for (usize) {
  fn buddy_0(self , rsthis: & QSortFilterProxyModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QSortFilterProxyModel5buddyERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:161
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QModelIndexList match(const QModelIndex &, int, const QVariant &, int, Qt::MatchFlags) const

/*
Reimplemented from QAbstractItemModel::match().
*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn match__0<RetType, T: QSortFilterProxyModel_match__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.match__0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_match__0<RetType> {
  fn match__0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_match__0<usize> for (usize,i32,usize,i32,i32) {
  fn match__0(self , rsthis: & QSortFilterProxyModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QSortFilterProxyModel5matchERK11QModelIndexiRK8QVarianti6QFlagsIN2Qt9MatchFlagEE", 5,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:165
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize span(const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::span().
*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn span_0<RetType, T: QSortFilterProxyModel_span_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.span_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_span_0<RetType> {
  fn span_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_span_0<usize> for (usize) {
  fn span_0(self , rsthis: & QSortFilterProxyModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QSortFilterProxyModel4spanERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:166
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void sort(int, Qt::SortOrder)

/*
Reimplemented from QAbstractItemModel::sort().
*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn sort_0<RetType, T: QSortFilterProxyModel_sort_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sort_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_sort_0<RetType> {
  fn sort_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_sort_0<(/*void*/)> for (i32,i32) {
  fn sort_0(self , rsthis: & QSortFilterProxyModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN21QSortFilterProxyModel4sortEiN2Qt9SortOrderE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:168
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QStringList mimeTypes() const

/*
Reimplemented from QAbstractItemModel::mimeTypes().
*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn mimeTypes_0<RetType, T: QSortFilterProxyModel_mimeTypes_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mimeTypes_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_mimeTypes_0<RetType> {
  fn mimeTypes_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_mimeTypes_0<usize> for () {
  fn mimeTypes_0(self , rsthis: & QSortFilterProxyModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QSortFilterProxyModel9mimeTypesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsortfilterproxymodel.h:169
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] Qt::DropActions supportedDropActions() const

/*
Reimplemented from QAbstractItemModel::supportedDropActions().
*/
impl /*struct*/ QSortFilterProxyModel {
  pub fn supportedDropActions_0<RetType, T: QSortFilterProxyModel_supportedDropActions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.supportedDropActions_0(self);
    // return 1;
  }
}
pub trait QSortFilterProxyModel_supportedDropActions_0<RetType> {
  fn supportedDropActions_0(self , rsthis: & QSortFilterProxyModel) -> RetType;
}
impl<'a> /*trait*/ QSortFilterProxyModel_supportedDropActions_0<i32> for () {
  fn supportedDropActions_0(self , rsthis: & QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QSortFilterProxyModel20supportedDropActionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
