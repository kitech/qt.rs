

// mod ::core::QAbstractTableModel
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
// extern C begin: 61
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
#[derive(Default)] // class sizeof(QAbstractTableModel)=16
pub struct QAbstractTableModel {
  qbase: QAbstractItemModel,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAbstractTableModel_ITF interface {
//    QAbstractItemModel_ITF
//    QAbstractTableModel_PTR() *QAbstractTableModel
//}
//func (ptr *QAbstractTableModel) QAbstractTableModel_PTR() *QAbstractTableModel { return ptr }

impl /*struct*/ QAbstractTableModel {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAbstractTableModel {
    return QAbstractTableModel{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAbstractTableModel {
//  type Target = QAbstractTableModelBASE;
//
//  fn deref(&self) -> &QAbstractTableModelBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAbstractTableModelBASE> for QAbstractTableModel {
//  fn as_ref(& self) -> & QAbstractTableModelBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qabstractitemmodel.h:367
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QAbstractTableModel {
  pub fn metaObject_0<RetType, T: QAbstractTableModel_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QAbstractTableModel_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QAbstractTableModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractTableModel_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QAbstractTableModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QAbstractTableModel10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:370
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QAbstractTableModel(QObject *)

/*

*/
// QAbstractTableModel(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QAbstractTableModel {
  pub fn QAbstractTableModel_0<T: QAbstractTableModel_QAbstractTableModel_0>(value: T) -> QAbstractTableModel {
    let rsthis = value.QAbstractTableModel_0();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractTableModel_QAbstractTableModel_0 {
  fn QAbstractTableModel_0(self) -> QAbstractTableModel;
}
// QAbstractTableModel(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QAbstractTableModel_QAbstractTableModel_0 for (usize) {
  fn QAbstractTableModel_0(self) -> QAbstractTableModel {
    // unsafe{_ZN19QAbstractTableModelC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QAbstractTableModelC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAbstractTableModel{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:371
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QAbstractTableModel()

/*

*/
pub fn DeleteQAbstractTableModel(this :*mut QAbstractTableModel) {
    // let rv = qtrt::InvokeQtFunc6("_ZN19QAbstractTableModelD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qabstractitemmodel.h:373
// index:0
// Public virtual Visibility=Default Availability=Available
// [24] QModelIndex index(int, int, const QModelIndex &) const

/*
Returns the index of the item in the model specified by the given row, column and parent index.

When reimplementing this function in a subclass, call createIndex() to generate model indexes that other components can use to refer to items in your model.

See also createIndex().
*/
impl /*struct*/ QAbstractTableModel {
  pub fn index_0<RetType, T: QAbstractTableModel_index_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.index_0(self);
    // return 1;
  }
}
pub trait QAbstractTableModel_index_0<RetType> {
  fn index_0(self , rsthis: & QAbstractTableModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractTableModel_index_0<usize> for (i32,i32,usize) {
  fn index_0(self , rsthis: & QAbstractTableModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QAbstractTableModel5indexEiiRK11QModelIndex", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:374
// index:0
// Public virtual Visibility=Default Availability=Available
// [24] QModelIndex sibling(int, int, const QModelIndex &) const

/*
Returns the sibling at row and column for the item at index, or an invalid QModelIndex if there is no sibling at that location.

sibling() is just a convenience function that finds the item's parent, and uses it to retrieve the index of the child item in the specified row and column.

This method can optionally be overridden for implementation-specific optimization.

See also index(), QModelIndex::row(), and QModelIndex::column().
*/
impl /*struct*/ QAbstractTableModel {
  pub fn sibling_0<RetType, T: QAbstractTableModel_sibling_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sibling_0(self);
    // return 1;
  }
}
pub trait QAbstractTableModel_sibling_0<RetType> {
  fn sibling_0(self , rsthis: & QAbstractTableModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractTableModel_sibling_0<usize> for (i32,i32,usize) {
  fn sibling_0(self , rsthis: & QAbstractTableModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QAbstractTableModel7siblingEiiRK11QModelIndex", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:375
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
impl /*struct*/ QAbstractTableModel {
  pub fn dropMimeData_0<RetType, T: QAbstractTableModel_dropMimeData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dropMimeData_0(self);
    // return 1;
  }
}
pub trait QAbstractTableModel_dropMimeData_0<RetType> {
  fn dropMimeData_0(self , rsthis: & QAbstractTableModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractTableModel_dropMimeData_0<bool> for (usize,i32,i32,i32,usize) {
  fn dropMimeData_0(self , rsthis: & QAbstractTableModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN19QAbstractTableModel12dropMimeDataEPK9QMimeDataN2Qt10DropActionEiiRK11QModelIndex", 5,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:378
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] Qt::ItemFlags flags(const QModelIndex &) const

/*
Returns the item flags for the given index.

The base class implementation returns a combination of flags that enables the item (ItemIsEnabled) and allows it to be selected (ItemIsSelectable).

See also Qt::ItemFlags.
*/
impl /*struct*/ QAbstractTableModel {
  pub fn flags_0<RetType, T: QAbstractTableModel_flags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.flags_0(self);
    // return 1;
  }
}
pub trait QAbstractTableModel_flags_0<RetType> {
  fn flags_0(self , rsthis: & QAbstractTableModel) -> RetType;
}
impl<'a> /*trait*/ QAbstractTableModel_flags_0<i32> for (usize) {
  fn flags_0(self , rsthis: & QAbstractTableModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QAbstractTableModel5flagsERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
