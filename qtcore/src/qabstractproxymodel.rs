

// mod ::core::QAbstractProxyModel
// package qtcore
// /usr/include/qt/QtCore/qabstractproxymodel.h
// #include <qabstractproxymodel.h>
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
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin

// void resetInternalData()
// func (this *QAbstractProxyModel) InheritResetInternalData(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "resetInternalData", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QAbstractProxyModel)=16
pub struct QAbstractProxyModel {
  qbase: QAbstractItemModel,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAbstractProxyModel_ITF interface {
//    QAbstractItemModel_ITF
//    QAbstractProxyModel_PTR() *QAbstractProxyModel
//}
//func (ptr *QAbstractProxyModel) QAbstractProxyModel_PTR() *QAbstractProxyModel { return ptr }

impl /*struct*/ QAbstractProxyModel {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAbstractProxyModel {
    return QAbstractProxyModel{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAbstractProxyModel {
//  type Target = QAbstractProxyModelBASE;
//
//  fn deref(&self) -> &QAbstractProxyModelBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAbstractProxyModelBASE> for QAbstractProxyModel {
//  fn as_ref(& self) -> & QAbstractProxyModelBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qabstractproxymodel.h:55
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QAbstractProxyModel {
  pub fn metaObject_0<RetType, T: QAbstractProxyModel_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QAbstractProxyModel_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QAbstractProxyModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractProxyModel_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QAbstractProxyModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QAbstractProxyModel10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractproxymodel.h:59
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QAbstractProxyModel(QObject *)

/*
Constructs a proxy model with the given parent.
*/
// QAbstractProxyModel(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QAbstractProxyModel {
  pub fn QAbstractProxyModel_0<T: QAbstractProxyModel_QAbstractProxyModel_0>(value: T) -> QAbstractProxyModel {
    let rsthis = value.QAbstractProxyModel_0();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractProxyModel_QAbstractProxyModel_0 {
  fn QAbstractProxyModel_0(self) -> QAbstractProxyModel;
}
// QAbstractProxyModel(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QAbstractProxyModel_QAbstractProxyModel_0 for (usize) {
  fn QAbstractProxyModel_0(self) -> QAbstractProxyModel {
    // unsafe{_ZN19QAbstractProxyModelC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QAbstractProxyModelC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAbstractProxyModel{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractproxymodel.h:60
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QAbstractProxyModel()

/*

*/
pub fn DeleteQAbstractProxyModel(this :*mut QAbstractProxyModel) {
    // let rv = qtrt::InvokeQtFunc6("_ZN19QAbstractProxyModelD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qabstractproxymodel.h:62
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setSourceModel(QAbstractItemModel *)

/*
Sets the given sourceModel to be processed by the proxy model.

Subclasses should call beginResetModel() at the beginning of the method, disconnect from the old model, call this method, connect to the new model, and call endResetModel().

Note: Setter function for property sourceModel. 

See also sourceModel().
*/
impl /*struct*/ QAbstractProxyModel {
  pub fn setSourceModel_0<RetType, T: QAbstractProxyModel_setSourceModel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSourceModel_0(self);
    // return 1;
  }
}
pub trait QAbstractProxyModel_setSourceModel_0<RetType> {
  fn setSourceModel_0(self , rsthis: & QAbstractProxyModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractProxyModel_setSourceModel_0<(/*void*/)> for (usize) {
  fn setSourceModel_0(self , rsthis: & QAbstractProxyModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QAbstractProxyModel14setSourceModelEP18QAbstractItemModel", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractproxymodel.h:63
// index:0
// Public Visibility=Default Availability=Available
// [8] QAbstractItemModel * sourceModel() const

/*
Returns the model that contains the data that is available through the proxy model.

Note: Getter function for property sourceModel. 

See also setSourceModel().
*/
impl /*struct*/ QAbstractProxyModel {
  pub fn sourceModel_0<RetType, T: QAbstractProxyModel_sourceModel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sourceModel_0(self);
    // return 1;
  }
}
pub trait QAbstractProxyModel_sourceModel_0<RetType> {
  fn sourceModel_0(self , rsthis: & QAbstractProxyModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractProxyModel_sourceModel_0<usize> for () {
  fn sourceModel_0(self , rsthis: & QAbstractProxyModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QAbstractProxyModel11sourceModelEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractproxymodel.h:65
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [24] QModelIndex mapToSource(const QModelIndex &) const

/*
Reimplement this function to return the model index in the source model that corresponds to the proxyIndex in the proxy model.

See also mapFromSource().
*/
impl /*struct*/ QAbstractProxyModel {
  pub fn mapToSource_0<RetType, T: QAbstractProxyModel_mapToSource_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapToSource_0(self);
    // return 1;
  }
}
pub trait QAbstractProxyModel_mapToSource_0<RetType> {
  fn mapToSource_0(self , rsthis: & QAbstractProxyModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractProxyModel_mapToSource_0<usize> for (usize) {
  fn mapToSource_0(self , rsthis: & QAbstractProxyModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QAbstractProxyModel11mapToSourceERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractproxymodel.h:66
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [24] QModelIndex mapFromSource(const QModelIndex &) const

/*
Reimplement this function to return the model index in the proxy model that corresponds to the sourceIndex from the source model.

See also mapToSource().
*/
impl /*struct*/ QAbstractProxyModel {
  pub fn mapFromSource_0<RetType, T: QAbstractProxyModel_mapFromSource_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapFromSource_0(self);
    // return 1;
  }
}
pub trait QAbstractProxyModel_mapFromSource_0<RetType> {
  fn mapFromSource_0(self , rsthis: & QAbstractProxyModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractProxyModel_mapFromSource_0<usize> for (usize) {
  fn mapFromSource_0(self , rsthis: & QAbstractProxyModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QAbstractProxyModel13mapFromSourceERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractproxymodel.h:68
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QItemSelection mapSelectionToSource(const QItemSelection &) const

/*
Returns a source selection mapped from the specified proxySelection.

Reimplement this method to map proxy selections to source selections.
*/
impl /*struct*/ QAbstractProxyModel {
  pub fn mapSelectionToSource_0<RetType, T: QAbstractProxyModel_mapSelectionToSource_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapSelectionToSource_0(self);
    // return 1;
  }
}
pub trait QAbstractProxyModel_mapSelectionToSource_0<RetType> {
  fn mapSelectionToSource_0(self , rsthis: & QAbstractProxyModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractProxyModel_mapSelectionToSource_0<usize> for (usize) {
  fn mapSelectionToSource_0(self , rsthis: & QAbstractProxyModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QAbstractProxyModel20mapSelectionToSourceERK14QItemSelection", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractproxymodel.h:69
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QItemSelection mapSelectionFromSource(const QItemSelection &) const

/*
Returns a proxy selection mapped from the specified sourceSelection.

Reimplement this method to map source selections to proxy selections.
*/
impl /*struct*/ QAbstractProxyModel {
  pub fn mapSelectionFromSource_0<RetType, T: QAbstractProxyModel_mapSelectionFromSource_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapSelectionFromSource_0(self);
    // return 1;
  }
}
pub trait QAbstractProxyModel_mapSelectionFromSource_0<RetType> {
  fn mapSelectionFromSource_0(self , rsthis: & QAbstractProxyModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractProxyModel_mapSelectionFromSource_0<usize> for (usize) {
  fn mapSelectionFromSource_0(self , rsthis: & QAbstractProxyModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QAbstractProxyModel22mapSelectionFromSourceERK14QItemSelection", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractproxymodel.h:71
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool submit()

/*
Reimplemented from QAbstractItemModel::submit().
*/
impl /*struct*/ QAbstractProxyModel {
  pub fn submit_0<RetType, T: QAbstractProxyModel_submit_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.submit_0(self);
    // return 1;
  }
}
pub trait QAbstractProxyModel_submit_0<RetType> {
  fn submit_0(self , rsthis: & QAbstractProxyModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractProxyModel_submit_0<bool> for () {
  fn submit_0(self , rsthis: & QAbstractProxyModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN19QAbstractProxyModel6submitEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractproxymodel.h:72
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void revert()

/*
Reimplemented from QAbstractItemModel::revert().
*/
impl /*struct*/ QAbstractProxyModel {
  pub fn revert_0<RetType, T: QAbstractProxyModel_revert_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.revert_0(self);
    // return 1;
  }
}
pub trait QAbstractProxyModel_revert_0<RetType> {
  fn revert_0(self , rsthis: & QAbstractProxyModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractProxyModel_revert_0<(/*void*/)> for () {
  fn revert_0(self , rsthis: & QAbstractProxyModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN19QAbstractProxyModel6revertEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractproxymodel.h:74
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QVariant data(const QModelIndex &, int) const

/*
Reimplemented from QAbstractItemModel::data().

See also setData().
*/
impl /*struct*/ QAbstractProxyModel {
  pub fn data_0<RetType, T: QAbstractProxyModel_data_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.data_0(self);
    // return 1;
  }
}
pub trait QAbstractProxyModel_data_0<RetType> {
  fn data_0(self , rsthis: & QAbstractProxyModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractProxyModel_data_0<usize> for (usize,i32) {
  fn data_0(self , rsthis: & QAbstractProxyModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QAbstractProxyModel4dataERK11QModelIndexi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractproxymodel.h:75
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QVariant headerData(int, Qt::Orientation, int) const

/*
Reimplemented from QAbstractItemModel::headerData().

See also setHeaderData().
*/
impl /*struct*/ QAbstractProxyModel {
  pub fn headerData_0<RetType, T: QAbstractProxyModel_headerData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.headerData_0(self);
    // return 1;
  }
}
pub trait QAbstractProxyModel_headerData_0<RetType> {
  fn headerData_0(self , rsthis: & QAbstractProxyModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractProxyModel_headerData_0<usize> for (i32,i32,i32) {
  fn headerData_0(self , rsthis: & QAbstractProxyModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QAbstractProxyModel10headerDataEiN2Qt11OrientationEi", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractproxymodel.h:77
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] Qt::ItemFlags flags(const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::flags().
*/
impl /*struct*/ QAbstractProxyModel {
  pub fn flags_0<RetType, T: QAbstractProxyModel_flags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.flags_0(self);
    // return 1;
  }
}
pub trait QAbstractProxyModel_flags_0<RetType> {
  fn flags_0(self , rsthis: & QAbstractProxyModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractProxyModel_flags_0<i32> for (usize) {
  fn flags_0(self , rsthis: & QAbstractProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QAbstractProxyModel5flagsERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractproxymodel.h:79
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool setData(const QModelIndex &, const QVariant &, int)

/*
Reimplemented from QAbstractItemModel::setData().

See also data().
*/
impl /*struct*/ QAbstractProxyModel {
  pub fn setData_0<RetType, T: QAbstractProxyModel_setData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setData_0(self);
    // return 1;
  }
}
pub trait QAbstractProxyModel_setData_0<RetType> {
  fn setData_0(self , rsthis: & QAbstractProxyModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractProxyModel_setData_0<bool> for (usize,usize,i32) {
  fn setData_0(self , rsthis: & QAbstractProxyModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN19QAbstractProxyModel7setDataERK11QModelIndexRK8QVarianti", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractproxymodel.h:81
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool setHeaderData(int, Qt::Orientation, const QVariant &, int)

/*
Reimplemented from QAbstractItemModel::setHeaderData().

See also headerData().
*/
impl /*struct*/ QAbstractProxyModel {
  pub fn setHeaderData_0<RetType, T: QAbstractProxyModel_setHeaderData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHeaderData_0(self);
    // return 1;
  }
}
pub trait QAbstractProxyModel_setHeaderData_0<RetType> {
  fn setHeaderData_0(self , rsthis: & QAbstractProxyModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractProxyModel_setHeaderData_0<bool> for (i32,i32,usize,i32) {
  fn setHeaderData_0(self , rsthis: & QAbstractProxyModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN19QAbstractProxyModel13setHeaderDataEiN2Qt11OrientationERK8QVarianti", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractproxymodel.h:83
// index:0
// Public virtual Visibility=Default Availability=Available
// [24] QModelIndex buddy(const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::buddy().
*/
impl /*struct*/ QAbstractProxyModel {
  pub fn buddy_0<RetType, T: QAbstractProxyModel_buddy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.buddy_0(self);
    // return 1;
  }
}
pub trait QAbstractProxyModel_buddy_0<RetType> {
  fn buddy_0(self , rsthis: & QAbstractProxyModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractProxyModel_buddy_0<usize> for (usize) {
  fn buddy_0(self , rsthis: & QAbstractProxyModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QAbstractProxyModel5buddyERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractproxymodel.h:84
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool canFetchMore(const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::canFetchMore().
*/
impl /*struct*/ QAbstractProxyModel {
  pub fn canFetchMore_0<RetType, T: QAbstractProxyModel_canFetchMore_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.canFetchMore_0(self);
    // return 1;
  }
}
pub trait QAbstractProxyModel_canFetchMore_0<RetType> {
  fn canFetchMore_0(self , rsthis: & QAbstractProxyModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractProxyModel_canFetchMore_0<bool> for (usize) {
  fn canFetchMore_0(self , rsthis: & QAbstractProxyModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QAbstractProxyModel12canFetchMoreERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractproxymodel.h:85
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void fetchMore(const QModelIndex &)

/*
Reimplemented from QAbstractItemModel::fetchMore().
*/
impl /*struct*/ QAbstractProxyModel {
  pub fn fetchMore_0<RetType, T: QAbstractProxyModel_fetchMore_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fetchMore_0(self);
    // return 1;
  }
}
pub trait QAbstractProxyModel_fetchMore_0<RetType> {
  fn fetchMore_0(self , rsthis: & QAbstractProxyModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractProxyModel_fetchMore_0<(/*void*/)> for (usize) {
  fn fetchMore_0(self , rsthis: & QAbstractProxyModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QAbstractProxyModel9fetchMoreERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractproxymodel.h:86
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void sort(int, Qt::SortOrder)

/*
Reimplemented from QAbstractItemModel::sort().
*/
impl /*struct*/ QAbstractProxyModel {
  pub fn sort_0<RetType, T: QAbstractProxyModel_sort_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sort_0(self);
    // return 1;
  }
}
pub trait QAbstractProxyModel_sort_0<RetType> {
  fn sort_0(self , rsthis: & QAbstractProxyModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractProxyModel_sort_0<(/*void*/)> for (i32,i32) {
  fn sort_0(self , rsthis: & QAbstractProxyModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN19QAbstractProxyModel4sortEiN2Qt9SortOrderE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractproxymodel.h:87
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize span(const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::span().
*/
impl /*struct*/ QAbstractProxyModel {
  pub fn span_0<RetType, T: QAbstractProxyModel_span_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.span_0(self);
    // return 1;
  }
}
pub trait QAbstractProxyModel_span_0<RetType> {
  fn span_0(self , rsthis: & QAbstractProxyModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractProxyModel_span_0<usize> for (usize) {
  fn span_0(self , rsthis: & QAbstractProxyModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QAbstractProxyModel4spanERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractproxymodel.h:88
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool hasChildren(const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::hasChildren().
*/
impl /*struct*/ QAbstractProxyModel {
  pub fn hasChildren_0<RetType, T: QAbstractProxyModel_hasChildren_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasChildren_0(self);
    // return 1;
  }
}
pub trait QAbstractProxyModel_hasChildren_0<RetType> {
  fn hasChildren_0(self , rsthis: & QAbstractProxyModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractProxyModel_hasChildren_0<bool> for (usize) {
  fn hasChildren_0(self , rsthis: & QAbstractProxyModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QAbstractProxyModel11hasChildrenERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractproxymodel.h:89
// index:0
// Public virtual Visibility=Default Availability=Available
// [24] QModelIndex sibling(int, int, const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::sibling().
*/
impl /*struct*/ QAbstractProxyModel {
  pub fn sibling_0<RetType, T: QAbstractProxyModel_sibling_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sibling_0(self);
    // return 1;
  }
}
pub trait QAbstractProxyModel_sibling_0<RetType> {
  fn sibling_0(self , rsthis: & QAbstractProxyModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractProxyModel_sibling_0<usize> for (i32,i32,usize) {
  fn sibling_0(self , rsthis: & QAbstractProxyModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QAbstractProxyModel7siblingEiiRK11QModelIndex", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractproxymodel.h:92
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool canDropMimeData(const QMimeData *, Qt::DropAction, int, int, const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::canDropMimeData().

This function was introduced in  Qt 5.4.
*/
impl /*struct*/ QAbstractProxyModel {
  pub fn canDropMimeData_0<RetType, T: QAbstractProxyModel_canDropMimeData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.canDropMimeData_0(self);
    // return 1;
  }
}
pub trait QAbstractProxyModel_canDropMimeData_0<RetType> {
  fn canDropMimeData_0(self , rsthis: & QAbstractProxyModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractProxyModel_canDropMimeData_0<bool> for (usize,i32,i32,i32,usize) {
  fn canDropMimeData_0(self , rsthis: & QAbstractProxyModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QAbstractProxyModel15canDropMimeDataEPK9QMimeDataN2Qt10DropActionEiiRK11QModelIndex", 5,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractproxymodel.h:94
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool dropMimeData(const QMimeData *, Qt::DropAction, int, int, const QModelIndex &)

/*
Reimplemented from QAbstractItemModel::dropMimeData().

This function was introduced in  Qt 5.4.
*/
impl /*struct*/ QAbstractProxyModel {
  pub fn dropMimeData_0<RetType, T: QAbstractProxyModel_dropMimeData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dropMimeData_0(self);
    // return 1;
  }
}
pub trait QAbstractProxyModel_dropMimeData_0<RetType> {
  fn dropMimeData_0(self , rsthis: & QAbstractProxyModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractProxyModel_dropMimeData_0<bool> for (usize,i32,i32,i32,usize) {
  fn dropMimeData_0(self , rsthis: & QAbstractProxyModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN19QAbstractProxyModel12dropMimeDataEPK9QMimeDataN2Qt10DropActionEiiRK11QModelIndex", 5,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractproxymodel.h:96
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QStringList mimeTypes() const

/*
Reimplemented from QAbstractItemModel::mimeTypes().
*/
impl /*struct*/ QAbstractProxyModel {
  pub fn mimeTypes_0<RetType, T: QAbstractProxyModel_mimeTypes_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mimeTypes_0(self);
    // return 1;
  }
}
pub trait QAbstractProxyModel_mimeTypes_0<RetType> {
  fn mimeTypes_0(self , rsthis: & QAbstractProxyModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractProxyModel_mimeTypes_0<usize> for () {
  fn mimeTypes_0(self , rsthis: & QAbstractProxyModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QAbstractProxyModel9mimeTypesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractproxymodel.h:97
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] Qt::DropActions supportedDragActions() const

/*
Reimplemented from QAbstractItemModel::supportedDragActions().
*/
impl /*struct*/ QAbstractProxyModel {
  pub fn supportedDragActions_0<RetType, T: QAbstractProxyModel_supportedDragActions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.supportedDragActions_0(self);
    // return 1;
  }
}
pub trait QAbstractProxyModel_supportedDragActions_0<RetType> {
  fn supportedDragActions_0(self , rsthis: & QAbstractProxyModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractProxyModel_supportedDragActions_0<i32> for () {
  fn supportedDragActions_0(self , rsthis: & QAbstractProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QAbstractProxyModel20supportedDragActionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractproxymodel.h:98
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] Qt::DropActions supportedDropActions() const

/*
Reimplemented from QAbstractItemModel::supportedDropActions().
*/
impl /*struct*/ QAbstractProxyModel {
  pub fn supportedDropActions_0<RetType, T: QAbstractProxyModel_supportedDropActions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.supportedDropActions_0(self);
    // return 1;
  }
}
pub trait QAbstractProxyModel_supportedDropActions_0<RetType> {
  fn supportedDropActions_0(self , rsthis: & QAbstractProxyModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractProxyModel_supportedDropActions_0<i32> for () {
  fn supportedDropActions_0(self , rsthis: & QAbstractProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QAbstractProxyModel20supportedDropActionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractproxymodel.h:104
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void resetInternalData()

/*
Clears the roleNames of this proxy model.
*/
impl /*struct*/ QAbstractProxyModel {
  pub fn resetInternalData_0<RetType, T: QAbstractProxyModel_resetInternalData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resetInternalData_0(self);
    // return 1;
  }
}
pub trait QAbstractProxyModel_resetInternalData_0<RetType> {
  fn resetInternalData_0(self , rsthis: & QAbstractProxyModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractProxyModel_resetInternalData_0<(/*void*/)> for () {
  fn resetInternalData_0(self , rsthis: & QAbstractProxyModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN19QAbstractProxyModel17resetInternalDataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
