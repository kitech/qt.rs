

// mod ::core::QItemSelectionModel
// package qtcore
// /usr/include/qt/QtCore/qitemselectionmodel.h
// #include <qitemselectionmodel.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 26
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

// void emitSelectionChanged(const QItemSelection &, const QItemSelection &)
// func (this *QItemSelectionModel) InheritEmitSelectionChanged(f func(newSelection *QItemSelection, oldSelection *QItemSelection) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "emitSelectionChanged", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QItemSelectionModel)=16
pub struct QItemSelectionModel {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QItemSelectionModel_ITF interface {
//    QObject_ITF
//    QItemSelectionModel_PTR() *QItemSelectionModel
//}
//func (ptr *QItemSelectionModel) QItemSelectionModel_PTR() *QItemSelectionModel { return ptr }

impl /*struct*/ QItemSelectionModel {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QItemSelectionModel {
    return QItemSelectionModel{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QItemSelectionModel {
//  type Target = QItemSelectionModelBASE;
//
//  fn deref(&self) -> &QItemSelectionModelBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QItemSelectionModelBASE> for QItemSelectionModel {
//  fn as_ref(& self) -> & QItemSelectionModelBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qitemselectionmodel.h:139
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QItemSelectionModel {
  pub fn metaObject_0<RetType, T: QItemSelectionModel_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QItemSelectionModel_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QItemSelectionModel) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionModel_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QItemSelectionModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QItemSelectionModel10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:167
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QItemSelectionModel(QAbstractItemModel *)

/*
Constructs a selection model that operates on the specified item model.
*/
// QItemSelectionModel(QAbstractItemModel *) ctx.fn_proto_cpp
impl /*struct*/ QItemSelectionModel {
  pub fn QItemSelectionModel_0<T: QItemSelectionModel_QItemSelectionModel_0>(value: T) -> QItemSelectionModel {
    let rsthis = value.QItemSelectionModel_0();
    return rsthis;
    // return 1;
  }
}

pub trait QItemSelectionModel_QItemSelectionModel_0 {
  fn QItemSelectionModel_0(self) -> QItemSelectionModel;
}
// QItemSelectionModel(QAbstractItemModel *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QItemSelectionModel_QItemSelectionModel_0 for (usize) {
  fn QItemSelectionModel_0(self) -> QItemSelectionModel {
    // unsafe{_ZN19QItemSelectionModelC2EP18QAbstractItemModel()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QItemSelectionModelC2EP18QAbstractItemModel", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QItemSelectionModel{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:168
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QItemSelectionModel(QAbstractItemModel *, QObject *)

/*
Constructs a selection model that operates on the specified item model.
*/
// QItemSelectionModel(QAbstractItemModel *, QObject *) ctx.fn_proto_cpp
impl /*struct*/ QItemSelectionModel {
  pub fn QItemSelectionModel_1<T: QItemSelectionModel_QItemSelectionModel_1>(value: T) -> QItemSelectionModel {
    let rsthis = value.QItemSelectionModel_1();
    return rsthis;
    // return 1;
  }
}

pub trait QItemSelectionModel_QItemSelectionModel_1 {
  fn QItemSelectionModel_1(self) -> QItemSelectionModel;
}
// QItemSelectionModel(QAbstractItemModel *, QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QItemSelectionModel_QItemSelectionModel_1 for (usize,usize) {
  fn QItemSelectionModel_1(self) -> QItemSelectionModel {
    // unsafe{_ZN19QItemSelectionModelC2EP18QAbstractItemModelP7QObject()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QItemSelectionModelC2EP18QAbstractItemModelP7QObject", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QItemSelectionModel{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:169
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QItemSelectionModel()

/*

*/
pub fn DeleteQItemSelectionModel(this :*mut QItemSelectionModel) {
    // let rv = qtrt::InvokeQtFunc6("_ZN19QItemSelectionModelD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qitemselectionmodel.h:171
// index:0
// Public Visibility=Default Availability=Available
// [24] QModelIndex currentIndex() const

/*
Returns the model item index for the current item, or an invalid index if there is no current item.

See also setCurrentIndex().
*/
impl /*struct*/ QItemSelectionModel {
  pub fn currentIndex_0<RetType, T: QItemSelectionModel_currentIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentIndex_0(self);
    // return 1;
  }
}
pub trait QItemSelectionModel_currentIndex_0<RetType> {
  fn currentIndex_0(self , rsthis: & QItemSelectionModel) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionModel_currentIndex_0<usize> for () {
  fn currentIndex_0(self , rsthis: & QItemSelectionModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QItemSelectionModel12currentIndexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:173
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isSelected(const QModelIndex &) const

/*
Returns true if the given model item index is selected.
*/
impl /*struct*/ QItemSelectionModel {
  pub fn isSelected_0<RetType, T: QItemSelectionModel_isSelected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isSelected_0(self);
    // return 1;
  }
}
pub trait QItemSelectionModel_isSelected_0<RetType> {
  fn isSelected_0(self , rsthis: & QItemSelectionModel) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionModel_isSelected_0<bool> for (usize) {
  fn isSelected_0(self , rsthis: & QItemSelectionModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QItemSelectionModel10isSelectedERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:174
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isRowSelected(int, const QModelIndex &) const

/*
Returns true if all items are selected in the row with the given parent.

Note that this function is usually faster than calling isSelected() on all items in the same row and that unselectable items are ignored.
*/
impl /*struct*/ QItemSelectionModel {
  pub fn isRowSelected_0<RetType, T: QItemSelectionModel_isRowSelected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isRowSelected_0(self);
    // return 1;
  }
}
pub trait QItemSelectionModel_isRowSelected_0<RetType> {
  fn isRowSelected_0(self , rsthis: & QItemSelectionModel) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionModel_isRowSelected_0<bool> for (i32,usize) {
  fn isRowSelected_0(self , rsthis: & QItemSelectionModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QItemSelectionModel13isRowSelectedEiRK11QModelIndex", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:175
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isColumnSelected(int, const QModelIndex &) const

/*
Returns true if all items are selected in the column with the given parent.

Note that this function is usually faster than calling isSelected() on all items in the same column and that unselectable items are ignored.
*/
impl /*struct*/ QItemSelectionModel {
  pub fn isColumnSelected_0<RetType, T: QItemSelectionModel_isColumnSelected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isColumnSelected_0(self);
    // return 1;
  }
}
pub trait QItemSelectionModel_isColumnSelected_0<RetType> {
  fn isColumnSelected_0(self , rsthis: & QItemSelectionModel) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionModel_isColumnSelected_0<bool> for (i32,usize) {
  fn isColumnSelected_0(self , rsthis: & QItemSelectionModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QItemSelectionModel16isColumnSelectedEiRK11QModelIndex", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:177
// index:0
// Public Visibility=Default Availability=Available
// [1] bool rowIntersectsSelection(int, const QModelIndex &) const

/*
Returns true if there are any items selected in the row with the given parent.
*/
impl /*struct*/ QItemSelectionModel {
  pub fn rowIntersectsSelection_0<RetType, T: QItemSelectionModel_rowIntersectsSelection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowIntersectsSelection_0(self);
    // return 1;
  }
}
pub trait QItemSelectionModel_rowIntersectsSelection_0<RetType> {
  fn rowIntersectsSelection_0(self , rsthis: & QItemSelectionModel) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionModel_rowIntersectsSelection_0<bool> for (i32,usize) {
  fn rowIntersectsSelection_0(self , rsthis: & QItemSelectionModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QItemSelectionModel22rowIntersectsSelectionEiRK11QModelIndex", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:178
// index:0
// Public Visibility=Default Availability=Available
// [1] bool columnIntersectsSelection(int, const QModelIndex &) const

/*
Returns true if there are any items selected in the column with the given parent.
*/
impl /*struct*/ QItemSelectionModel {
  pub fn columnIntersectsSelection_0<RetType, T: QItemSelectionModel_columnIntersectsSelection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnIntersectsSelection_0(self);
    // return 1;
  }
}
pub trait QItemSelectionModel_columnIntersectsSelection_0<RetType> {
  fn columnIntersectsSelection_0(self , rsthis: & QItemSelectionModel) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionModel_columnIntersectsSelection_0<bool> for (i32,usize) {
  fn columnIntersectsSelection_0(self , rsthis: & QItemSelectionModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QItemSelectionModel25columnIntersectsSelectionEiRK11QModelIndex", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:180
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasSelection() const

/*
Returns true if the selection model contains any selection ranges; otherwise returns false.

This function was introduced in  Qt 4.2.
*/
impl /*struct*/ QItemSelectionModel {
  pub fn hasSelection_0<RetType, T: QItemSelectionModel_hasSelection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasSelection_0(self);
    // return 1;
  }
}
pub trait QItemSelectionModel_hasSelection_0<RetType> {
  fn hasSelection_0(self , rsthis: & QItemSelectionModel) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionModel_hasSelection_0<bool> for () {
  fn hasSelection_0(self , rsthis: & QItemSelectionModel) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QItemSelectionModel12hasSelectionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:182
// index:0
// Public Visibility=Default Availability=Available
// [8] QModelIndexList selectedIndexes() const

/*
Returns a list of all selected model item indexes. The list contains no duplicates, and is not sorted.

Note: Getter function for property selectedIndexes.
*/
impl /*struct*/ QItemSelectionModel {
  pub fn selectedIndexes_0<RetType, T: QItemSelectionModel_selectedIndexes_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectedIndexes_0(self);
    // return 1;
  }
}
pub trait QItemSelectionModel_selectedIndexes_0<RetType> {
  fn selectedIndexes_0(self , rsthis: & QItemSelectionModel) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionModel_selectedIndexes_0<usize> for () {
  fn selectedIndexes_0(self , rsthis: & QItemSelectionModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QItemSelectionModel15selectedIndexesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:183
// index:0
// Public Visibility=Default Availability=Available
// [8] QModelIndexList selectedRows(int) const

/*
Returns the indexes in the given column for the rows where all columns are selected.

This function was introduced in  Qt 4.2.

See also selectedIndexes() and selectedColumns().
*/
impl /*struct*/ QItemSelectionModel {
  pub fn selectedRows_0<RetType, T: QItemSelectionModel_selectedRows_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectedRows_0(self);
    // return 1;
  }
}
pub trait QItemSelectionModel_selectedRows_0<RetType> {
  fn selectedRows_0(self , rsthis: & QItemSelectionModel) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionModel_selectedRows_0<usize> for (i32) {
  fn selectedRows_0(self , rsthis: & QItemSelectionModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QItemSelectionModel12selectedRowsEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:184
// index:0
// Public Visibility=Default Availability=Available
// [8] QModelIndexList selectedColumns(int) const

/*
Returns the indexes in the given row for columns where all rows are selected.

This function was introduced in  Qt 4.2.

See also selectedIndexes() and selectedRows().
*/
impl /*struct*/ QItemSelectionModel {
  pub fn selectedColumns_0<RetType, T: QItemSelectionModel_selectedColumns_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectedColumns_0(self);
    // return 1;
  }
}
pub trait QItemSelectionModel_selectedColumns_0<RetType> {
  fn selectedColumns_0(self , rsthis: & QItemSelectionModel) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionModel_selectedColumns_0<usize> for (i32) {
  fn selectedColumns_0(self , rsthis: & QItemSelectionModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QItemSelectionModel15selectedColumnsEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:185
// index:0
// Public Visibility=Default Availability=Available
// [8] const QItemSelection selection() const

/*
Returns the selection ranges stored in the selection model.
*/
impl /*struct*/ QItemSelectionModel {
  pub fn selection_0<RetType, T: QItemSelectionModel_selection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selection_0(self);
    // return 1;
  }
}
pub trait QItemSelectionModel_selection_0<RetType> {
  fn selection_0(self , rsthis: & QItemSelectionModel) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionModel_selection_0<usize> for () {
  fn selection_0(self , rsthis: & QItemSelectionModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QItemSelectionModel9selectionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:188
// index:0
// Public Visibility=Default Availability=Available
// [8] const QAbstractItemModel * model() const

/*
Returns the item model operated on by the selection model.

See also setModel().
*/
impl /*struct*/ QItemSelectionModel {
  pub fn model_0<RetType, T: QItemSelectionModel_model_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.model_0(self);
    // return 1;
  }
}
pub trait QItemSelectionModel_model_0<RetType> {
  fn model_0(self , rsthis: & QItemSelectionModel) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionModel_model_0<usize> for () {
  fn model_0(self , rsthis: & QItemSelectionModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QItemSelectionModel5modelEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:189
// index:1
// Public Visibility=Default Availability=Available
// [8] QAbstractItemModel * model()

/*
Returns the item model operated on by the selection model.

See also setModel().
*/
impl /*struct*/ QItemSelectionModel {
  pub fn model_1<RetType, T: QItemSelectionModel_model_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.model_1(self);
    // return 1;
  }
}
pub trait QItemSelectionModel_model_1<RetType> {
  fn model_1(self , rsthis: & QItemSelectionModel) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionModel_model_1<usize> for () {
  fn model_1(self , rsthis: & QItemSelectionModel) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN19QItemSelectionModel5modelEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:191
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setModel(QAbstractItemModel *)

/*
Sets the model to model. The modelChanged() signal will be emitted.

This function was introduced in  Qt 5.5.

See also model() and modelChanged().
*/
impl /*struct*/ QItemSelectionModel {
  pub fn setModel_0<RetType, T: QItemSelectionModel_setModel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setModel_0(self);
    // return 1;
  }
}
pub trait QItemSelectionModel_setModel_0<RetType> {
  fn setModel_0(self , rsthis: & QItemSelectionModel) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionModel_setModel_0<(/*void*/)> for (usize) {
  fn setModel_0(self , rsthis: & QItemSelectionModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QItemSelectionModel8setModelEP18QAbstractItemModel", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:194
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setCurrentIndex(const QModelIndex &, QItemSelectionModel::SelectionFlags)

/*
Sets the model item index to be the current item, and emits currentChanged(). The current item is used for keyboard navigation and focus indication; it is independent of any selected items, although a selected item can also be the current item.

Depending on the specified command, the index can also become part of the current selection.

See also currentIndex() and select().
*/
impl /*struct*/ QItemSelectionModel {
  pub fn setCurrentIndex_0<RetType, T: QItemSelectionModel_setCurrentIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentIndex_0(self);
    // return 1;
  }
}
pub trait QItemSelectionModel_setCurrentIndex_0<RetType> {
  fn setCurrentIndex_0(self , rsthis: & QItemSelectionModel) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionModel_setCurrentIndex_0<(/*void*/)> for (usize,i32) {
  fn setCurrentIndex_0(self , rsthis: & QItemSelectionModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN19QItemSelectionModel15setCurrentIndexERK11QModelIndex6QFlagsINS_13SelectionFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:195
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void select(const QModelIndex &, QItemSelectionModel::SelectionFlags)

/*
Selects the model item index using the specified command, and emits selectionChanged().

See also QItemSelectionModel::SelectionFlags.
*/
impl /*struct*/ QItemSelectionModel {
  pub fn select__0<RetType, T: QItemSelectionModel_select__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.select__0(self);
    // return 1;
  }
}
pub trait QItemSelectionModel_select__0<RetType> {
  fn select__0(self , rsthis: & QItemSelectionModel) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionModel_select__0<(/*void*/)> for (usize,i32) {
  fn select__0(self , rsthis: & QItemSelectionModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN19QItemSelectionModel6selectERK11QModelIndex6QFlagsINS_13SelectionFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:196
// index:1
// Public virtual Visibility=Default Availability=Available
// [-2] void select(const QItemSelection &, QItemSelectionModel::SelectionFlags)

/*
Selects the model item index using the specified command, and emits selectionChanged().

See also QItemSelectionModel::SelectionFlags.
*/
impl /*struct*/ QItemSelectionModel {
  pub fn select__1<RetType, T: QItemSelectionModel_select__1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.select__1(self);
    // return 1;
  }
}
pub trait QItemSelectionModel_select__1<RetType> {
  fn select__1(self , rsthis: & QItemSelectionModel) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionModel_select__1<(/*void*/)> for (usize,i32) {
  fn select__1(self , rsthis: & QItemSelectionModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN19QItemSelectionModel6selectERK14QItemSelection6QFlagsINS_13SelectionFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:197
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void clear()

/*
Clears the selection model. Emits selectionChanged() and currentChanged().
*/
impl /*struct*/ QItemSelectionModel {
  pub fn clear_0<RetType, T: QItemSelectionModel_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QItemSelectionModel_clear_0<RetType> {
  fn clear_0(self , rsthis: & QItemSelectionModel) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionModel_clear_0<(/*void*/)> for () {
  fn clear_0(self , rsthis: & QItemSelectionModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN19QItemSelectionModel5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:198
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void reset()

/*
Clears the selection model. Does not emit any signals.
*/
impl /*struct*/ QItemSelectionModel {
  pub fn reset_0<RetType, T: QItemSelectionModel_reset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.reset_0(self);
    // return 1;
  }
}
pub trait QItemSelectionModel_reset_0<RetType> {
  fn reset_0(self , rsthis: & QItemSelectionModel) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionModel_reset_0<(/*void*/)> for () {
  fn reset_0(self , rsthis: & QItemSelectionModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN19QItemSelectionModel5resetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:200
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clearSelection()

/*
Clears the selection in the selection model. Emits selectionChanged().

This function was introduced in  Qt 4.2.
*/
impl /*struct*/ QItemSelectionModel {
  pub fn clearSelection_0<RetType, T: QItemSelectionModel_clearSelection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clearSelection_0(self);
    // return 1;
  }
}
pub trait QItemSelectionModel_clearSelection_0<RetType> {
  fn clearSelection_0(self , rsthis: & QItemSelectionModel) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionModel_clearSelection_0<(/*void*/)> for () {
  fn clearSelection_0(self , rsthis: & QItemSelectionModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN19QItemSelectionModel14clearSelectionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:201
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void clearCurrentIndex()

/*
Clears the current index. Emits currentChanged().
*/
impl /*struct*/ QItemSelectionModel {
  pub fn clearCurrentIndex_0<RetType, T: QItemSelectionModel_clearCurrentIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clearCurrentIndex_0(self);
    // return 1;
  }
}
pub trait QItemSelectionModel_clearCurrentIndex_0<RetType> {
  fn clearCurrentIndex_0(self , rsthis: & QItemSelectionModel) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionModel_clearCurrentIndex_0<(/*void*/)> for () {
  fn clearCurrentIndex_0(self , rsthis: & QItemSelectionModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN19QItemSelectionModel17clearCurrentIndexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:204
// index:0
// Public Visibility=Default Availability=Available
// [-2] void selectionChanged(const QItemSelection &, const QItemSelection &)

/*
This signal is emitted whenever the selection changes. The change in the selection is represented as an item selection of deselected items and an item selection of selected items.

Note the that the current index changes independently from the selection. Also note that this signal will not be emitted when the item model is reset.

Note: Notifier signal for property selectedIndexes. 

See also select() and currentChanged().
*/
impl /*struct*/ QItemSelectionModel {
  pub fn selectionChanged_0<RetType, T: QItemSelectionModel_selectionChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectionChanged_0(self);
    // return 1;
  }
}
pub trait QItemSelectionModel_selectionChanged_0<RetType> {
  fn selectionChanged_0(self , rsthis: & QItemSelectionModel) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionModel_selectionChanged_0<(/*void*/)> for (usize,usize) {
  fn selectionChanged_0(self , rsthis: & QItemSelectionModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QItemSelectionModel16selectionChangedERK14QItemSelectionS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:205
// index:0
// Public Visibility=Default Availability=Available
// [-2] void currentChanged(const QModelIndex &, const QModelIndex &)

/*
This signal is emitted whenever the current item changes. The previous model item index is replaced by the current index as the selection's current item.

Note that this signal will not be emitted when the item model is reset.

See also currentIndex(), setCurrentIndex(), and selectionChanged().
*/
impl /*struct*/ QItemSelectionModel {
  pub fn currentChanged_0<RetType, T: QItemSelectionModel_currentChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentChanged_0(self);
    // return 1;
  }
}
pub trait QItemSelectionModel_currentChanged_0<RetType> {
  fn currentChanged_0(self , rsthis: & QItemSelectionModel) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionModel_currentChanged_0<(/*void*/)> for (usize,usize) {
  fn currentChanged_0(self , rsthis: & QItemSelectionModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QItemSelectionModel14currentChangedERK11QModelIndexS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:206
// index:0
// Public Visibility=Default Availability=Available
// [-2] void currentRowChanged(const QModelIndex &, const QModelIndex &)

/*
This signal is emitted if the current item changes and its row is different to the row of the previous current item.

Note that this signal will not be emitted when the item model is reset.

See also currentChanged(), currentColumnChanged(), currentIndex(), and setCurrentIndex().
*/
impl /*struct*/ QItemSelectionModel {
  pub fn currentRowChanged_0<RetType, T: QItemSelectionModel_currentRowChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentRowChanged_0(self);
    // return 1;
  }
}
pub trait QItemSelectionModel_currentRowChanged_0<RetType> {
  fn currentRowChanged_0(self , rsthis: & QItemSelectionModel) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionModel_currentRowChanged_0<(/*void*/)> for (usize,usize) {
  fn currentRowChanged_0(self , rsthis: & QItemSelectionModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QItemSelectionModel17currentRowChangedERK11QModelIndexS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:207
// index:0
// Public Visibility=Default Availability=Available
// [-2] void currentColumnChanged(const QModelIndex &, const QModelIndex &)

/*
This signal is emitted if the current item changes and its column is different to the column of the previous current item.

Note that this signal will not be emitted when the item model is reset.

See also currentChanged(), currentRowChanged(), currentIndex(), and setCurrentIndex().
*/
impl /*struct*/ QItemSelectionModel {
  pub fn currentColumnChanged_0<RetType, T: QItemSelectionModel_currentColumnChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentColumnChanged_0(self);
    // return 1;
  }
}
pub trait QItemSelectionModel_currentColumnChanged_0<RetType> {
  fn currentColumnChanged_0(self , rsthis: & QItemSelectionModel) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionModel_currentColumnChanged_0<(/*void*/)> for (usize,usize) {
  fn currentColumnChanged_0(self , rsthis: & QItemSelectionModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QItemSelectionModel20currentColumnChangedERK11QModelIndexS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:208
// index:0
// Public Visibility=Default Availability=Available
// [-2] void modelChanged(QAbstractItemModel *)

/*
This signal is emitted when the model is successfully set with setModel().

This function was introduced in  Qt 5.5.

See also model() and setModel().
*/
impl /*struct*/ QItemSelectionModel {
  pub fn modelChanged_0<RetType, T: QItemSelectionModel_modelChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.modelChanged_0(self);
    // return 1;
  }
}
pub trait QItemSelectionModel_modelChanged_0<RetType> {
  fn modelChanged_0(self , rsthis: & QItemSelectionModel) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionModel_modelChanged_0<(/*void*/)> for (usize) {
  fn modelChanged_0(self , rsthis: & QItemSelectionModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QItemSelectionModel12modelChangedEP18QAbstractItemModel", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:212
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void emitSelectionChanged(const QItemSelection &, const QItemSelection &)

/*
Compares the two selections newSelection and oldSelection and emits selectionChanged() with the deselected and selected items.
*/
impl /*struct*/ QItemSelectionModel {
  pub fn emitSelectionChanged_0<RetType, T: QItemSelectionModel_emitSelectionChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.emitSelectionChanged_0(self);
    // return 1;
  }
}
pub trait QItemSelectionModel_emitSelectionChanged_0<RetType> {
  fn emitSelectionChanged_0(self , rsthis: & QItemSelectionModel) -> RetType;
}
impl<'a> /*trait*/ QItemSelectionModel_emitSelectionChanged_0<(/*void*/)> for (usize,usize) {
  fn emitSelectionChanged_0(self , rsthis: & QItemSelectionModel) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QItemSelectionModel20emitSelectionChangedERK14QItemSelectionS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*


*/
pub type QItemSelectionModel__SelectionFlag = i32;
// 
pub const QItemSelectionModel__NoUpdate :QItemSelectionModel__SelectionFlag = 0;
// 
pub const QItemSelectionModel__Clear :QItemSelectionModel__SelectionFlag = 1;
// 
pub const QItemSelectionModel__Select :QItemSelectionModel__SelectionFlag = 2;
// 
pub const QItemSelectionModel__Deselect :QItemSelectionModel__SelectionFlag = 4;
// 
pub const QItemSelectionModel__Toggle :QItemSelectionModel__SelectionFlag = 8;
// 
pub const QItemSelectionModel__Current :QItemSelectionModel__SelectionFlag = 16;
// 
pub const QItemSelectionModel__Rows :QItemSelectionModel__SelectionFlag = 32;
// 
pub const QItemSelectionModel__Columns :QItemSelectionModel__SelectionFlag = 64;
// 
pub const QItemSelectionModel__SelectCurrent :QItemSelectionModel__SelectionFlag = 18;
// 
pub const QItemSelectionModel__ToggleCurrent :QItemSelectionModel__SelectionFlag = 24;
// 
pub const QItemSelectionModel__ClearAndSelect :QItemSelectionModel__SelectionFlag = 3;
pub fn QItemSelectionModel_SelectionFlagItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QItemSelectionModel", val);
}
pub fn QItemSelectionModel_SelectionFlagItemName_s(val: i32) ->String {
  //var nilthis *QItemSelectionModel
  //return nilthis.SelectionFlagItemName(val);
  return QItemSelectionModel_SelectionFlagItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
