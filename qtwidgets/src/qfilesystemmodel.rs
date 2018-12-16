

// mod ::widgets::QFileSystemModel
// package qtwidgets
// /usr/include/qt/QtWidgets/qfilesystemmodel.h
// #include <qfilesystemmodel.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 80
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

// void timerEvent(QTimerEvent *)
// func (this *QFileSystemModel) InheritTimerEvent(f func(event *qtcore.QTimerEvent/*777 QTimerEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "timerEvent", f)
// }

// bool event(QEvent *)
// func (this *QFileSystemModel) InheritEvent(f func(event *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QFileSystemModel)=16
pub struct QFileSystemModel {
  qbase: QAbstractItemModel,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QFileSystemModel_ITF interface {
//    qtcore.QAbstractItemModel_ITF
//    QFileSystemModel_PTR() *QFileSystemModel
//}
//func (ptr *QFileSystemModel) QFileSystemModel_PTR() *QFileSystemModel { return ptr }

impl /*struct*/ QFileSystemModel {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QFileSystemModel {
    return QFileSystemModel{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QFileSystemModel {
//  type Target = QFileSystemModelBASE;
//
//  fn deref(&self) -> &QFileSystemModelBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QFileSystemModelBASE> for QFileSystemModel {
//  fn as_ref(& self) -> & QFileSystemModelBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qfilesystemmodel.h:60
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QFileSystemModel {
  pub fn metaObject_0<RetType, T: QFileSystemModel_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QFileSystemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QFileSystemModel10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:66
// index:0
// Public Visibility=Default Availability=Available
// [-2] void rootPathChanged(const QString &)

/*
This signal is emitted whenever the root path has been changed to a newPath.
*/
impl /*struct*/ QFileSystemModel {
  pub fn rootPathChanged_0<RetType, T: QFileSystemModel_rootPathChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rootPathChanged_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_rootPathChanged_0<RetType> {
  fn rootPathChanged_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_rootPathChanged_0<(/*void*/)> for (usize) {
  fn rootPathChanged_0(self , rsthis: & QFileSystemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QFileSystemModel15rootPathChangedERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:67
// index:0
// Public Visibility=Default Availability=Available
// [-2] void fileRenamed(const QString &, const QString &, const QString &)

/*
This signal is emitted whenever a file with the oldName is successfully renamed to newName. The file is located in in the directory path.
*/
impl /*struct*/ QFileSystemModel {
  pub fn fileRenamed_0<RetType, T: QFileSystemModel_fileRenamed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fileRenamed_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_fileRenamed_0<RetType> {
  fn fileRenamed_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_fileRenamed_0<(/*void*/)> for (usize,usize,usize) {
  fn fileRenamed_0(self , rsthis: & QFileSystemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QFileSystemModel11fileRenamedERK7QStringS2_S2_", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:68
// index:0
// Public Visibility=Default Availability=Available
// [-2] void directoryLoaded(const QString &)

/*
This signal is emitted when the gatherer thread has finished to load the path.

This function was introduced in  Qt 4.7.
*/
impl /*struct*/ QFileSystemModel {
  pub fn directoryLoaded_0<RetType, T: QFileSystemModel_directoryLoaded_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.directoryLoaded_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_directoryLoaded_0<RetType> {
  fn directoryLoaded_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_directoryLoaded_0<(/*void*/)> for (usize) {
  fn directoryLoaded_0(self , rsthis: & QFileSystemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QFileSystemModel15directoryLoadedERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:78
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QFileSystemModel(QObject *)

/*
Constructs a file system model with the given parent.
*/
// QFileSystemModel(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QFileSystemModel {
  pub fn QFileSystemModel_0<T: QFileSystemModel_QFileSystemModel_0>(value: T) -> QFileSystemModel {
    let rsthis = value.QFileSystemModel_0();
    return rsthis;
    // return 1;
  }
}

pub trait QFileSystemModel_QFileSystemModel_0 {
  fn QFileSystemModel_0(self) -> QFileSystemModel;
}
// QFileSystemModel(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QFileSystemModel_QFileSystemModel_0 for (usize) {
  fn QFileSystemModel_0(self) -> QFileSystemModel {
    // unsafe{_ZN16QFileSystemModelC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QFileSystemModelC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFileSystemModel{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:79
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QFileSystemModel()

/*

*/
pub fn DeleteQFileSystemModel(this :*mut QFileSystemModel) {
    // let rv = qtrt::InvokeQtFunc6("_ZN16QFileSystemModelD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qfilesystemmodel.h:81
// index:0
// Public virtual Visibility=Default Availability=Available
// [24] QModelIndex index(int, int, const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::index().
*/
impl /*struct*/ QFileSystemModel {
  pub fn index_0<RetType, T: QFileSystemModel_index_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.index_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_index_0<RetType> {
  fn index_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_index_0<usize> for (i32,i32,usize) {
  fn index_0(self , rsthis: & QFileSystemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QFileSystemModel5indexEiiRK11QModelIndex", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:82
// index:1
// Public Visibility=Default Availability=Available
// [24] QModelIndex index(const QString &, int) const

/*
Reimplemented from QAbstractItemModel::index().
*/
impl /*struct*/ QFileSystemModel {
  pub fn index_1<RetType, T: QFileSystemModel_index_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.index_1(self);
    // return 1;
  }
}
pub trait QFileSystemModel_index_1<RetType> {
  fn index_1(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_index_1<usize> for (usize,i32) {
  fn index_1(self , rsthis: & QFileSystemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QFileSystemModel5indexERK7QStringi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:83
// index:0
// Public virtual Visibility=Default Availability=Available
// [24] QModelIndex parent(const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::parent().
*/
impl /*struct*/ QFileSystemModel {
  pub fn parent_0<RetType, T: QFileSystemModel_parent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.parent_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_parent_0<RetType> {
  fn parent_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_parent_0<usize> for (usize) {
  fn parent_0(self , rsthis: & QFileSystemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QFileSystemModel6parentERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:85
// index:0
// Public virtual Visibility=Default Availability=Available
// [24] QModelIndex sibling(int, int, const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::sibling().
*/
impl /*struct*/ QFileSystemModel {
  pub fn sibling_0<RetType, T: QFileSystemModel_sibling_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sibling_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_sibling_0<RetType> {
  fn sibling_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_sibling_0<usize> for (i32,i32,usize) {
  fn sibling_0(self , rsthis: & QFileSystemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QFileSystemModel7siblingEiiRK11QModelIndex", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:86
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool hasChildren(const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::hasChildren().
*/
impl /*struct*/ QFileSystemModel {
  pub fn hasChildren_0<RetType, T: QFileSystemModel_hasChildren_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasChildren_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_hasChildren_0<RetType> {
  fn hasChildren_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_hasChildren_0<bool> for (usize) {
  fn hasChildren_0(self , rsthis: & QFileSystemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QFileSystemModel11hasChildrenERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:87
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool canFetchMore(const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::canFetchMore().
*/
impl /*struct*/ QFileSystemModel {
  pub fn canFetchMore_0<RetType, T: QFileSystemModel_canFetchMore_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.canFetchMore_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_canFetchMore_0<RetType> {
  fn canFetchMore_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_canFetchMore_0<bool> for (usize) {
  fn canFetchMore_0(self , rsthis: & QFileSystemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QFileSystemModel12canFetchMoreERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:88
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void fetchMore(const QModelIndex &)

/*
Reimplemented from QAbstractItemModel::fetchMore().
*/
impl /*struct*/ QFileSystemModel {
  pub fn fetchMore_0<RetType, T: QFileSystemModel_fetchMore_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fetchMore_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_fetchMore_0<RetType> {
  fn fetchMore_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_fetchMore_0<(/*void*/)> for (usize) {
  fn fetchMore_0(self , rsthis: & QFileSystemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QFileSystemModel9fetchMoreERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:90
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int rowCount(const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::rowCount().
*/
impl /*struct*/ QFileSystemModel {
  pub fn rowCount_0<RetType, T: QFileSystemModel_rowCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowCount_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_rowCount_0<RetType> {
  fn rowCount_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_rowCount_0<i32> for (usize) {
  fn rowCount_0(self , rsthis: & QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QFileSystemModel8rowCountERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:91
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int columnCount(const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::columnCount().
*/
impl /*struct*/ QFileSystemModel {
  pub fn columnCount_0<RetType, T: QFileSystemModel_columnCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnCount_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_columnCount_0<RetType> {
  fn columnCount_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_columnCount_0<i32> for (usize) {
  fn columnCount_0(self , rsthis: & QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QFileSystemModel11columnCountERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:93
// index:0
// Public Visibility=Default Availability=Available
// [16] QVariant myComputer(int) const

/*
Returns the data stored under the given role for the item "My Computer".

See also Qt::ItemDataRole.
*/
impl /*struct*/ QFileSystemModel {
  pub fn myComputer_0<RetType, T: QFileSystemModel_myComputer_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.myComputer_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_myComputer_0<RetType> {
  fn myComputer_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_myComputer_0<usize> for (i32) {
  fn myComputer_0(self , rsthis: & QFileSystemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QFileSystemModel10myComputerEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:94
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QVariant data(const QModelIndex &, int) const

/*
Reimplemented from QAbstractItemModel::data().

See also setData().
*/
impl /*struct*/ QFileSystemModel {
  pub fn data_0<RetType, T: QFileSystemModel_data_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.data_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_data_0<RetType> {
  fn data_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_data_0<usize> for (usize,i32) {
  fn data_0(self , rsthis: & QFileSystemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QFileSystemModel4dataERK11QModelIndexi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:95
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool setData(const QModelIndex &, const QVariant &, int)

/*
Reimplemented from QAbstractItemModel::setData().

See also data().
*/
impl /*struct*/ QFileSystemModel {
  pub fn setData_0<RetType, T: QFileSystemModel_setData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setData_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_setData_0<RetType> {
  fn setData_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_setData_0<bool> for (usize,usize,i32) {
  fn setData_0(self , rsthis: & QFileSystemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QFileSystemModel7setDataERK11QModelIndexRK8QVarianti", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:97
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QVariant headerData(int, Qt::Orientation, int) const

/*
Reimplemented from QAbstractItemModel::headerData().
*/
impl /*struct*/ QFileSystemModel {
  pub fn headerData_0<RetType, T: QFileSystemModel_headerData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.headerData_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_headerData_0<RetType> {
  fn headerData_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_headerData_0<usize> for (i32,i32,i32) {
  fn headerData_0(self , rsthis: & QFileSystemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QFileSystemModel10headerDataEiN2Qt11OrientationEi", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:99
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] Qt::ItemFlags flags(const QModelIndex &) const

/*
Reimplemented from QAbstractItemModel::flags().
*/
impl /*struct*/ QFileSystemModel {
  pub fn flags_0<RetType, T: QFileSystemModel_flags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.flags_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_flags_0<RetType> {
  fn flags_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_flags_0<i32> for (usize) {
  fn flags_0(self , rsthis: & QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QFileSystemModel5flagsERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:101
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void sort(int, Qt::SortOrder)

/*
Reimplemented from QAbstractItemModel::sort().
*/
impl /*struct*/ QFileSystemModel {
  pub fn sort_0<RetType, T: QFileSystemModel_sort_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sort_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_sort_0<RetType> {
  fn sort_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_sort_0<(/*void*/)> for (i32,i32) {
  fn sort_0(self , rsthis: & QFileSystemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN16QFileSystemModel4sortEiN2Qt9SortOrderE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:103
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QStringList mimeTypes() const

/*
Reimplemented from QAbstractItemModel::mimeTypes().

Returns a list of MIME types that can be used to describe a list of items in the model.
*/
impl /*struct*/ QFileSystemModel {
  pub fn mimeTypes_0<RetType, T: QFileSystemModel_mimeTypes_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mimeTypes_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_mimeTypes_0<RetType> {
  fn mimeTypes_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_mimeTypes_0<usize> for () {
  fn mimeTypes_0(self , rsthis: & QFileSystemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QFileSystemModel9mimeTypesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:105
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool dropMimeData(const QMimeData *, Qt::DropAction, int, int, const QModelIndex &)

/*
Reimplemented from QAbstractItemModel::dropMimeData().

Handles the data supplied by a drag and drop operation that ended with the given action over the row in the model specified by the row and column and by the parent index.

See also supportedDropActions().
*/
impl /*struct*/ QFileSystemModel {
  pub fn dropMimeData_0<RetType, T: QFileSystemModel_dropMimeData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dropMimeData_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_dropMimeData_0<RetType> {
  fn dropMimeData_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_dropMimeData_0<bool> for (usize,i32,i32,i32,usize) {
  fn dropMimeData_0(self , rsthis: & QFileSystemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QFileSystemModel12dropMimeDataEPK9QMimeDataN2Qt10DropActionEiiRK11QModelIndex", 5,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:107
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] Qt::DropActions supportedDropActions() const

/*
Reimplemented from QAbstractItemModel::supportedDropActions().
*/
impl /*struct*/ QFileSystemModel {
  pub fn supportedDropActions_0<RetType, T: QFileSystemModel_supportedDropActions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.supportedDropActions_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_supportedDropActions_0<RetType> {
  fn supportedDropActions_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_supportedDropActions_0<i32> for () {
  fn supportedDropActions_0(self , rsthis: & QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QFileSystemModel20supportedDropActionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:110
// index:0
// Public Visibility=Default Availability=Available
// [24] QModelIndex setRootPath(const QString &)

/*
Sets the directory that is being watched by the model to newPath by installing a file system watcher on it. Any changes to files and directories within this directory will be reflected in the model.

If the path is changed, the rootPathChanged() signal will be emitted.

Note: This function does not change the structure of the model or modify the data available to views. In other words, the "root" of the model is not changed to include only files and directories within the directory specified by newPath in the file system.

See also rootPath().
*/
impl /*struct*/ QFileSystemModel {
  pub fn setRootPath_0<RetType, T: QFileSystemModel_setRootPath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRootPath_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_setRootPath_0<RetType> {
  fn setRootPath_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_setRootPath_0<usize> for (usize) {
  fn setRootPath_0(self , rsthis: & QFileSystemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QFileSystemModel11setRootPathERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:111
// index:0
// Public Visibility=Default Availability=Available
// [8] QString rootPath() const

/*
The currently set root path

See also setRootPath() and rootDirectory().
*/
impl /*struct*/ QFileSystemModel {
  pub fn rootPath_0<RetType, T: QFileSystemModel_rootPath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rootPath_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_rootPath_0<RetType> {
  fn rootPath_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_rootPath_0<usize> for () {
  fn rootPath_0(self , rsthis: & QFileSystemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QFileSystemModel8rootPathEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:112
// index:0
// Public Visibility=Default Availability=Available
// [8] QDir rootDirectory() const

/*
The currently set directory

See also rootPath().
*/
impl /*struct*/ QFileSystemModel {
  pub fn rootDirectory_0<RetType, T: QFileSystemModel_rootDirectory_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rootDirectory_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_rootDirectory_0<RetType> {
  fn rootDirectory_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_rootDirectory_0<usize> for () {
  fn rootDirectory_0(self , rsthis: & QFileSystemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QFileSystemModel13rootDirectoryEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:114
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setIconProvider(QFileIconProvider *)

/*
Sets the provider of file icons for the directory model.

See also iconProvider().
*/
impl /*struct*/ QFileSystemModel {
  pub fn setIconProvider_0<RetType, T: QFileSystemModel_setIconProvider_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIconProvider_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_setIconProvider_0<RetType> {
  fn setIconProvider_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_setIconProvider_0<(/*void*/)> for (usize) {
  fn setIconProvider_0(self , rsthis: & QFileSystemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QFileSystemModel15setIconProviderEP17QFileIconProvider", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:115
// index:0
// Public Visibility=Default Availability=Available
// [8] QFileIconProvider * iconProvider() const

/*
Returns the file icon provider for this directory model.

See also setIconProvider().
*/
impl /*struct*/ QFileSystemModel {
  pub fn iconProvider_0<RetType, T: QFileSystemModel_iconProvider_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.iconProvider_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_iconProvider_0<RetType> {
  fn iconProvider_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_iconProvider_0<usize> for () {
  fn iconProvider_0(self , rsthis: & QFileSystemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QFileSystemModel12iconProviderEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:117
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFilter(QDir::Filters)

/*
Sets the directory model's filter to that specified by filters.

Note that the filter you set should always include the QDir::AllDirs enum value, otherwise QFileSystemModel won't be able to read the directory structure.

See also filter() and QDir::Filters.
*/
impl /*struct*/ QFileSystemModel {
  pub fn setFilter_0<RetType, T: QFileSystemModel_setFilter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFilter_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_setFilter_0<RetType> {
  fn setFilter_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_setFilter_0<(/*void*/)> for (i32) {
  fn setFilter_0(self , rsthis: & QFileSystemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN16QFileSystemModel9setFilterE6QFlagsIN4QDir6FilterEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:118
// index:0
// Public Visibility=Default Availability=Available
// [4] QDir::Filters filter() const

/*
Returns the filter specified for the directory model.

If a filter has not been set, the default filter is QDir::AllEntries | QDir::NoDotAndDotDot | QDir::AllDirs.

See also setFilter() and QDir::Filters.
*/
impl /*struct*/ QFileSystemModel {
  pub fn filter_0<RetType, T: QFileSystemModel_filter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.filter_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_filter_0<RetType> {
  fn filter_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_filter_0<i32> for () {
  fn filter_0(self , rsthis: & QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QFileSystemModel6filterEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:120
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setResolveSymlinks(bool)

/*

*/
impl /*struct*/ QFileSystemModel {
  pub fn setResolveSymlinks_0<RetType, T: QFileSystemModel_setResolveSymlinks_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setResolveSymlinks_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_setResolveSymlinks_0<RetType> {
  fn setResolveSymlinks_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_setResolveSymlinks_0<(/*void*/)> for (bool) {
  fn setResolveSymlinks_0(self , rsthis: & QFileSystemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN16QFileSystemModel18setResolveSymlinksEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:121
// index:0
// Public Visibility=Default Availability=Available
// [1] bool resolveSymlinks() const

/*

*/
impl /*struct*/ QFileSystemModel {
  pub fn resolveSymlinks_0<RetType, T: QFileSystemModel_resolveSymlinks_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resolveSymlinks_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_resolveSymlinks_0<RetType> {
  fn resolveSymlinks_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_resolveSymlinks_0<bool> for () {
  fn resolveSymlinks_0(self , rsthis: & QFileSystemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QFileSystemModel15resolveSymlinksEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:123
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setReadOnly(bool)

/*

*/
impl /*struct*/ QFileSystemModel {
  pub fn setReadOnly_0<RetType, T: QFileSystemModel_setReadOnly_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setReadOnly_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_setReadOnly_0<RetType> {
  fn setReadOnly_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_setReadOnly_0<(/*void*/)> for (bool) {
  fn setReadOnly_0(self , rsthis: & QFileSystemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN16QFileSystemModel11setReadOnlyEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:124
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isReadOnly() const

/*

*/
impl /*struct*/ QFileSystemModel {
  pub fn isReadOnly_0<RetType, T: QFileSystemModel_isReadOnly_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isReadOnly_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_isReadOnly_0<RetType> {
  fn isReadOnly_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_isReadOnly_0<bool> for () {
  fn isReadOnly_0(self , rsthis: & QFileSystemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QFileSystemModel10isReadOnlyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:126
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setNameFilterDisables(bool)

/*

*/
impl /*struct*/ QFileSystemModel {
  pub fn setNameFilterDisables_0<RetType, T: QFileSystemModel_setNameFilterDisables_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNameFilterDisables_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_setNameFilterDisables_0<RetType> {
  fn setNameFilterDisables_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_setNameFilterDisables_0<(/*void*/)> for (bool) {
  fn setNameFilterDisables_0(self , rsthis: & QFileSystemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN16QFileSystemModel21setNameFilterDisablesEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:127
// index:0
// Public Visibility=Default Availability=Available
// [1] bool nameFilterDisables() const

/*

*/
impl /*struct*/ QFileSystemModel {
  pub fn nameFilterDisables_0<RetType, T: QFileSystemModel_nameFilterDisables_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.nameFilterDisables_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_nameFilterDisables_0<RetType> {
  fn nameFilterDisables_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_nameFilterDisables_0<bool> for () {
  fn nameFilterDisables_0(self , rsthis: & QFileSystemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QFileSystemModel18nameFilterDisablesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:129
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setNameFilters(const QStringList &)

/*
Sets the name filters to apply against the existing files.

See also nameFilters().
*/
impl /*struct*/ QFileSystemModel {
  pub fn setNameFilters_0<RetType, T: QFileSystemModel_setNameFilters_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNameFilters_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_setNameFilters_0<RetType> {
  fn setNameFilters_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_setNameFilters_0<(/*void*/)> for (usize) {
  fn setNameFilters_0(self , rsthis: & QFileSystemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QFileSystemModel14setNameFiltersERK11QStringList", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:130
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList nameFilters() const

/*
Returns a list of filters applied to the names in the model.

See also setNameFilters().
*/
impl /*struct*/ QFileSystemModel {
  pub fn nameFilters_0<RetType, T: QFileSystemModel_nameFilters_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.nameFilters_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_nameFilters_0<RetType> {
  fn nameFilters_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_nameFilters_0<usize> for () {
  fn nameFilters_0(self , rsthis: & QFileSystemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QFileSystemModel11nameFiltersEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:132
// index:0
// Public Visibility=Default Availability=Available
// [8] QString filePath(const QModelIndex &) const

/*
Returns the path of the item stored in the model under the index given.
*/
impl /*struct*/ QFileSystemModel {
  pub fn filePath_0<RetType, T: QFileSystemModel_filePath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.filePath_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_filePath_0<RetType> {
  fn filePath_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_filePath_0<usize> for (usize) {
  fn filePath_0(self , rsthis: & QFileSystemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QFileSystemModel8filePathERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:133
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isDir(const QModelIndex &) const

/*
Returns true if the model item index represents a directory; otherwise returns false.
*/
impl /*struct*/ QFileSystemModel {
  pub fn isDir_0<RetType, T: QFileSystemModel_isDir_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isDir_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_isDir_0<RetType> {
  fn isDir_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_isDir_0<bool> for (usize) {
  fn isDir_0(self , rsthis: & QFileSystemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QFileSystemModel5isDirERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:134
// index:0
// Public Visibility=Default Availability=Available
// [8] qint64 size(const QModelIndex &) const

/*
Returns the size in bytes of index. If the file does not exist, 0 is returned.
*/
impl /*struct*/ QFileSystemModel {
  pub fn size_0<RetType, T: QFileSystemModel_size_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.size_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_size_0<RetType> {
  fn size_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_size_0<i64> for (usize) {
  fn size_0(self , rsthis: & QFileSystemModel) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QFileSystemModel4sizeERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:135
// index:0
// Public Visibility=Default Availability=Available
// [8] QString type(const QModelIndex &) const

/*
Returns the type of file index such as "Directory" or "JPEG file".
*/
impl /*struct*/ QFileSystemModel {
  pub fn type__0<RetType, T: QFileSystemModel_type__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.type__0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_type__0<RetType> {
  fn type__0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_type__0<usize> for (usize) {
  fn type__0(self , rsthis: & QFileSystemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QFileSystemModel4typeERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:136
// index:0
// Public Visibility=Default Availability=Available
// [8] QDateTime lastModified(const QModelIndex &) const

/*
Returns the date and time when index was last modified.
*/
impl /*struct*/ QFileSystemModel {
  pub fn lastModified_0<RetType, T: QFileSystemModel_lastModified_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastModified_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_lastModified_0<RetType> {
  fn lastModified_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_lastModified_0<usize> for (usize) {
  fn lastModified_0(self , rsthis: & QFileSystemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QFileSystemModel12lastModifiedERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:138
// index:0
// Public Visibility=Default Availability=Available
// [24] QModelIndex mkdir(const QModelIndex &, const QString &)

/*
Create a directory with the name in the parent model index.
*/
impl /*struct*/ QFileSystemModel {
  pub fn mkdir_0<RetType, T: QFileSystemModel_mkdir_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mkdir_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_mkdir_0<RetType> {
  fn mkdir_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_mkdir_0<usize> for (usize,usize) {
  fn mkdir_0(self , rsthis: & QFileSystemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QFileSystemModel5mkdirERK11QModelIndexRK7QString", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:139
// index:0
// Public Visibility=Default Availability=Available
// [1] bool rmdir(const QModelIndex &)

/*
Removes the directory corresponding to the model item index in the file system model and deletes the corresponding directory from the file system, returning true if successful. If the directory cannot be removed, false is returned.

Warning: This function deletes directories from the file system; it does not move them to a location where they can be recovered.

See also remove().
*/
impl /*struct*/ QFileSystemModel {
  pub fn rmdir_0<RetType, T: QFileSystemModel_rmdir_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rmdir_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_rmdir_0<RetType> {
  fn rmdir_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_rmdir_0<bool> for (usize) {
  fn rmdir_0(self , rsthis: & QFileSystemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QFileSystemModel5rmdirERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:140
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString fileName(const QModelIndex &) const

/*
Returns the file name for the item stored in the model under the given index.
*/
impl /*struct*/ QFileSystemModel {
  pub fn fileName_0<RetType, T: QFileSystemModel_fileName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fileName_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_fileName_0<RetType> {
  fn fileName_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_fileName_0<usize> for (usize) {
  fn fileName_0(self , rsthis: & QFileSystemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QFileSystemModel8fileNameERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:141
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QIcon fileIcon(const QModelIndex &) const

/*
Returns the icon for the item stored in the model under the given index.
*/
impl /*struct*/ QFileSystemModel {
  pub fn fileIcon_0<RetType, T: QFileSystemModel_fileIcon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fileIcon_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_fileIcon_0<RetType> {
  fn fileIcon_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_fileIcon_0<usize> for (usize) {
  fn fileIcon_0(self , rsthis: & QFileSystemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QFileSystemModel8fileIconERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:142
// index:0
// Public Visibility=Default Availability=Available
// [4] QFile::Permissions permissions(const QModelIndex &) const

/*
Returns the complete OR-ed together combination of QFile::Permission for the index.
*/
impl /*struct*/ QFileSystemModel {
  pub fn permissions_0<RetType, T: QFileSystemModel_permissions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.permissions_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_permissions_0<RetType> {
  fn permissions_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_permissions_0<i32> for (usize) {
  fn permissions_0(self , rsthis: & QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QFileSystemModel11permissionsERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:143
// index:0
// Public Visibility=Default Availability=Available
// [8] QFileInfo fileInfo(const QModelIndex &) const

/*
Returns the QFileInfo for the item stored in the model under the given index.
*/
impl /*struct*/ QFileSystemModel {
  pub fn fileInfo_0<RetType, T: QFileSystemModel_fileInfo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fileInfo_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_fileInfo_0<RetType> {
  fn fileInfo_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_fileInfo_0<usize> for (usize) {
  fn fileInfo_0(self , rsthis: & QFileSystemModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QFileSystemModel8fileInfoERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:144
// index:0
// Public Visibility=Default Availability=Available
// [1] bool remove(const QModelIndex &)

/*
Removes the model item index from the file system model and deletes the corresponding file from the file system, returning true if successful. If the item cannot be removed, false is returned.

Warning: This function deletes files from the file system; it does not move them to a location where they can be recovered.

See also rmdir().
*/
impl /*struct*/ QFileSystemModel {
  pub fn remove_0<RetType, T: QFileSystemModel_remove_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.remove_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_remove_0<RetType> {
  fn remove_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_remove_0<bool> for (usize) {
  fn remove_0(self , rsthis: & QFileSystemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QFileSystemModel6removeERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:148
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void timerEvent(QTimerEvent *)

/*
Reimplemented from QObject::timerEvent().
*/
impl /*struct*/ QFileSystemModel {
  pub fn timerEvent_0<RetType, T: QFileSystemModel_timerEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.timerEvent_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_timerEvent_0<RetType> {
  fn timerEvent_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_timerEvent_0<(/*void*/)> for (usize) {
  fn timerEvent_0(self , rsthis: & QFileSystemModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QFileSystemModel10timerEventEP11QTimerEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfilesystemmodel.h:149
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QFileSystemModel {
  pub fn event_0<RetType, T: QFileSystemModel_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QFileSystemModel_event_0<RetType> {
  fn event_0(self , rsthis: & QFileSystemModel) -> RetType;
}
impl<'a> /*trait*/ QFileSystemModel_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QFileSystemModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QFileSystemModel5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


/*
ConstantValue
QFileSystemModel::FileIconRoleQt::DecorationRole
QFileSystemModel::FilePathRoleQt::UserRole + 1
QFileSystemModel::FileNameRoleQt::UserRole + 2
QFileSystemModel::FilePermissionsQt::UserRole + 3

*/
pub type QFileSystemModel__Roles = i32;
// 
pub const QFileSystemModel__FileIconRole :QFileSystemModel__Roles = 1;
// 
pub const QFileSystemModel__FilePathRole :QFileSystemModel__Roles = 257;
// 
pub const QFileSystemModel__FileNameRole :QFileSystemModel__Roles = 258;
// 
pub const QFileSystemModel__FilePermissions :QFileSystemModel__Roles = 259;
pub fn QFileSystemModel_RolesItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QFileSystemModel", val);
}
pub fn QFileSystemModel_RolesItemName_s(val: i32) ->String {
  //var nilthis *QFileSystemModel
  //return nilthis.RolesItemName(val);
  return QFileSystemModel_RolesItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
