

// mod ::widgets::QTableWidget
// package qtwidgets
// /usr/include/qt/QtWidgets/qtablewidget.h
// #include <qtablewidget.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 45
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

// bool event(QEvent *)
// func (this *QTableWidget) InheritEvent(f func(e *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// QStringList mimeTypes()
// func (this *QTableWidget) InheritMimeTypes(f func() unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "mimeTypes", f)
// }

// bool dropMimeData(int, int, const QMimeData *, Qt::DropAction)
// func (this *QTableWidget) InheritDropMimeData(f func(row int, column int, data *qtcore.QMimeData/*777 const QMimeData **/, action int) bool) {
//  qtrt.SetAllInheritCallback(this, "dropMimeData", f)
// }

// Qt::DropActions supportedDropActions()
// func (this *QTableWidget) InheritSupportedDropActions(f func() int) {
//  qtrt.SetAllInheritCallback(this, "supportedDropActions", f)
// }

// QModelIndex indexFromItem(QTableWidgetItem *)
// func (this *QTableWidget) InheritIndexFromItem(f func(item *QTableWidgetItem/*777 QTableWidgetItem **/) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "indexFromItem", f)
// }

// QTableWidgetItem * itemFromIndex(const QModelIndex &)
// func (this *QTableWidget) InheritItemFromIndex(f func(index *qtcore.QModelIndex) unsafe.Pointer/*666*/) {
//  qtrt.SetAllInheritCallback(this, "itemFromIndex", f)
// }

// void dropEvent(QDropEvent *)
// func (this *QTableWidget) InheritDropEvent(f func(event *qtgui.QDropEvent/*777 QDropEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dropEvent", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QTableWidget)=48
pub struct QTableWidget {
  qbase: QTableView,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTableWidget_ITF interface {
//    QTableView_ITF
//    QTableWidget_PTR() *QTableWidget
//}
//func (ptr *QTableWidget) QTableWidget_PTR() *QTableWidget { return ptr }

impl /*struct*/ QTableWidget {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTableWidget {
    return QTableWidget{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTableWidget {
//  type Target = QTableWidgetBASE;
//
//  fn deref(&self) -> &QTableWidgetBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTableWidgetBASE> for QTableWidget {
//  fn as_ref(& self) -> & QTableWidgetBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qtablewidget.h:216
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QTableWidget {
  pub fn metaObject_0<RetType, T: QTableWidget_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QTableWidget_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QTableWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTableWidget10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:222
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTableWidget(QWidget *)

/*
Creates a new table view with the given parent.
*/
// QTableWidget(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QTableWidget {
  pub fn QTableWidget_0<T: QTableWidget_QTableWidget_0>(value: T) -> QTableWidget {
    let rsthis = value.QTableWidget_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTableWidget_QTableWidget_0 {
  fn QTableWidget_0(self) -> QTableWidget;
}
// QTableWidget(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTableWidget_QTableWidget_0 for (usize) {
  fn QTableWidget_0(self) -> QTableWidget {
    // unsafe{_ZN12QTableWidgetC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QTableWidgetC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTableWidget{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:223
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QTableWidget(int, int, QWidget *)

/*
Creates a new table view with the given parent.
*/
// QTableWidget(int, int, QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QTableWidget {
  pub fn QTableWidget_1<T: QTableWidget_QTableWidget_1>(value: T) -> QTableWidget {
    let rsthis = value.QTableWidget_1();
    return rsthis;
    // return 1;
  }
}

pub trait QTableWidget_QTableWidget_1 {
  fn QTableWidget_1(self) -> QTableWidget;
}
// QTableWidget(int, int, QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTableWidget_QTableWidget_1 for (i32,i32,usize) {
  fn QTableWidget_1(self) -> QTableWidget {
    // unsafe{_ZN12QTableWidgetC2EiiP7QWidget()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QTableWidgetC2EiiP7QWidget", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTableWidget{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:224
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QTableWidget()

/*

*/
pub fn DeleteQTableWidget(this :*mut QTableWidget) {
    // let rv = qtrt::InvokeQtFunc6("_ZN12QTableWidgetD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qtablewidget.h:226
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRowCount(int)

/*
Sets the number of rows in this table's model to rows. If this is less than rowCount(), the data in the unwanted rows is discarded.

Note: Setter function for property rowCount. 

See also rowCount() and setColumnCount().
*/
impl /*struct*/ QTableWidget {
  pub fn setRowCount_0<RetType, T: QTableWidget_setRowCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRowCount_0(self);
    // return 1;
  }
}
pub trait QTableWidget_setRowCount_0<RetType> {
  fn setRowCount_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_setRowCount_0<(/*void*/)> for (i32) {
  fn setRowCount_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget11setRowCountEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:227
// index:0
// Public Visibility=Default Availability=Available
// [4] int rowCount() const

/*
Returns the number of rows.

Note: Getter function for property rowCount. 

See also setRowCount().
*/
impl /*struct*/ QTableWidget {
  pub fn rowCount_0<RetType, T: QTableWidget_rowCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowCount_0(self);
    // return 1;
  }
}
pub trait QTableWidget_rowCount_0<RetType> {
  fn rowCount_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_rowCount_0<i32> for () {
  fn rowCount_0(self , rsthis: & QTableWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTableWidget8rowCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:229
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setColumnCount(int)

/*
Sets the number of columns in this table's model to columns. If this is less than columnCount(), the data in the unwanted columns is discarded.

Note: Setter function for property columnCount. 

See also columnCount() and setRowCount().
*/
impl /*struct*/ QTableWidget {
  pub fn setColumnCount_0<RetType, T: QTableWidget_setColumnCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setColumnCount_0(self);
    // return 1;
  }
}
pub trait QTableWidget_setColumnCount_0<RetType> {
  fn setColumnCount_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_setColumnCount_0<(/*void*/)> for (i32) {
  fn setColumnCount_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget14setColumnCountEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:230
// index:0
// Public Visibility=Default Availability=Available
// [4] int columnCount() const

/*
Returns the number of columns.

Note: Getter function for property columnCount. 

See also setColumnCount().
*/
impl /*struct*/ QTableWidget {
  pub fn columnCount_0<RetType, T: QTableWidget_columnCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnCount_0(self);
    // return 1;
  }
}
pub trait QTableWidget_columnCount_0<RetType> {
  fn columnCount_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_columnCount_0<i32> for () {
  fn columnCount_0(self , rsthis: & QTableWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTableWidget11columnCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:232
// index:0
// Public Visibility=Default Availability=Available
// [4] int row(const QTableWidgetItem *) const

/*
Returns the row for the item.
*/
impl /*struct*/ QTableWidget {
  pub fn row_0<RetType, T: QTableWidget_row_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.row_0(self);
    // return 1;
  }
}
pub trait QTableWidget_row_0<RetType> {
  fn row_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_row_0<i32> for (usize) {
  fn row_0(self , rsthis: & QTableWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTableWidget3rowEPK16QTableWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:233
// index:0
// Public Visibility=Default Availability=Available
// [4] int column(const QTableWidgetItem *) const

/*
Returns the column for the item.
*/
impl /*struct*/ QTableWidget {
  pub fn column_0<RetType, T: QTableWidget_column_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.column_0(self);
    // return 1;
  }
}
pub trait QTableWidget_column_0<RetType> {
  fn column_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_column_0<i32> for (usize) {
  fn column_0(self , rsthis: & QTableWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTableWidget6columnEPK16QTableWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:235
// index:0
// Public Visibility=Default Availability=Available
// [8] QTableWidgetItem * item(int, int) const

/*
Returns the item for the given row and column if one has been set; otherwise returns 0.

See also setItem().
*/
impl /*struct*/ QTableWidget {
  pub fn item_0<RetType, T: QTableWidget_item_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.item_0(self);
    // return 1;
  }
}
pub trait QTableWidget_item_0<RetType> {
  fn item_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_item_0<usize> for (i32,i32) {
  fn item_0(self , rsthis: & QTableWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTableWidget4itemEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:236
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setItem(int, int, QTableWidgetItem *)

/*
Sets the item for the given row and column to item.

The table takes ownership of the item.

Note that if sorting is enabled (see sortingEnabled) and column is the current sort column, the row will be moved to the sorted position determined by item.

If you want to set several items of a particular row (say, by calling setItem() in a loop), you may want to turn off sorting before doing so, and turn it back on afterwards; this will allow you to use the same row argument for all items in the same row (i.e. setItem() will not move the row).

See also item() and takeItem().
*/
impl /*struct*/ QTableWidget {
  pub fn setItem_0<RetType, T: QTableWidget_setItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setItem_0(self);
    // return 1;
  }
}
pub trait QTableWidget_setItem_0<RetType> {
  fn setItem_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_setItem_0<(/*void*/)> for (i32,i32,usize) {
  fn setItem_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget7setItemEiiP16QTableWidgetItem", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:237
// index:0
// Public Visibility=Default Availability=Available
// [8] QTableWidgetItem * takeItem(int, int)

/*
Removes the item at row and column from the table without deleting it.
*/
impl /*struct*/ QTableWidget {
  pub fn takeItem_0<RetType, T: QTableWidget_takeItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.takeItem_0(self);
    // return 1;
  }
}
pub trait QTableWidget_takeItem_0<RetType> {
  fn takeItem_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_takeItem_0<usize> for (i32,i32) {
  fn takeItem_0(self , rsthis: & QTableWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QTableWidget8takeItemEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:239
// index:0
// Public Visibility=Default Availability=Available
// [8] QTableWidgetItem * verticalHeaderItem(int) const

/*
Returns the vertical header item for row row.

See also setVerticalHeaderItem().
*/
impl /*struct*/ QTableWidget {
  pub fn verticalHeaderItem_0<RetType, T: QTableWidget_verticalHeaderItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.verticalHeaderItem_0(self);
    // return 1;
  }
}
pub trait QTableWidget_verticalHeaderItem_0<RetType> {
  fn verticalHeaderItem_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_verticalHeaderItem_0<usize> for (i32) {
  fn verticalHeaderItem_0(self , rsthis: & QTableWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTableWidget18verticalHeaderItemEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:240
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setVerticalHeaderItem(int, QTableWidgetItem *)

/*
Sets the vertical header item for row row to item.

See also verticalHeaderItem().
*/
impl /*struct*/ QTableWidget {
  pub fn setVerticalHeaderItem_0<RetType, T: QTableWidget_setVerticalHeaderItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVerticalHeaderItem_0(self);
    // return 1;
  }
}
pub trait QTableWidget_setVerticalHeaderItem_0<RetType> {
  fn setVerticalHeaderItem_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_setVerticalHeaderItem_0<(/*void*/)> for (i32,usize) {
  fn setVerticalHeaderItem_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget21setVerticalHeaderItemEiP16QTableWidgetItem", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:241
// index:0
// Public Visibility=Default Availability=Available
// [8] QTableWidgetItem * takeVerticalHeaderItem(int)

/*
Removes the vertical header item at row from the header without deleting it.

This function was introduced in  Qt 4.1.
*/
impl /*struct*/ QTableWidget {
  pub fn takeVerticalHeaderItem_0<RetType, T: QTableWidget_takeVerticalHeaderItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.takeVerticalHeaderItem_0(self);
    // return 1;
  }
}
pub trait QTableWidget_takeVerticalHeaderItem_0<RetType> {
  fn takeVerticalHeaderItem_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_takeVerticalHeaderItem_0<usize> for (i32) {
  fn takeVerticalHeaderItem_0(self , rsthis: & QTableWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QTableWidget22takeVerticalHeaderItemEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:243
// index:0
// Public Visibility=Default Availability=Available
// [8] QTableWidgetItem * horizontalHeaderItem(int) const

/*
Returns the horizontal header item for column, column, if one has been set; otherwise returns 0.

See also setHorizontalHeaderItem().
*/
impl /*struct*/ QTableWidget {
  pub fn horizontalHeaderItem_0<RetType, T: QTableWidget_horizontalHeaderItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.horizontalHeaderItem_0(self);
    // return 1;
  }
}
pub trait QTableWidget_horizontalHeaderItem_0<RetType> {
  fn horizontalHeaderItem_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_horizontalHeaderItem_0<usize> for (i32) {
  fn horizontalHeaderItem_0(self , rsthis: & QTableWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTableWidget20horizontalHeaderItemEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:244
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setHorizontalHeaderItem(int, QTableWidgetItem *)

/*
Sets the horizontal header item for column column to item. If necessary, the column count is increased to fit the item. The previous header item (if there was one) is deleted.

See also horizontalHeaderItem().
*/
impl /*struct*/ QTableWidget {
  pub fn setHorizontalHeaderItem_0<RetType, T: QTableWidget_setHorizontalHeaderItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHorizontalHeaderItem_0(self);
    // return 1;
  }
}
pub trait QTableWidget_setHorizontalHeaderItem_0<RetType> {
  fn setHorizontalHeaderItem_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_setHorizontalHeaderItem_0<(/*void*/)> for (i32,usize) {
  fn setHorizontalHeaderItem_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget23setHorizontalHeaderItemEiP16QTableWidgetItem", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:245
// index:0
// Public Visibility=Default Availability=Available
// [8] QTableWidgetItem * takeHorizontalHeaderItem(int)

/*
Removes the horizontal header item at column from the header without deleting it.

This function was introduced in  Qt 4.1.
*/
impl /*struct*/ QTableWidget {
  pub fn takeHorizontalHeaderItem_0<RetType, T: QTableWidget_takeHorizontalHeaderItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.takeHorizontalHeaderItem_0(self);
    // return 1;
  }
}
pub trait QTableWidget_takeHorizontalHeaderItem_0<RetType> {
  fn takeHorizontalHeaderItem_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_takeHorizontalHeaderItem_0<usize> for (i32) {
  fn takeHorizontalHeaderItem_0(self , rsthis: & QTableWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QTableWidget24takeHorizontalHeaderItemEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:246
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setVerticalHeaderLabels(const QStringList &)

/*
Sets the vertical header labels using labels.
*/
impl /*struct*/ QTableWidget {
  pub fn setVerticalHeaderLabels_0<RetType, T: QTableWidget_setVerticalHeaderLabels_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVerticalHeaderLabels_0(self);
    // return 1;
  }
}
pub trait QTableWidget_setVerticalHeaderLabels_0<RetType> {
  fn setVerticalHeaderLabels_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_setVerticalHeaderLabels_0<(/*void*/)> for (usize) {
  fn setVerticalHeaderLabels_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget23setVerticalHeaderLabelsERK11QStringList", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:247
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setHorizontalHeaderLabels(const QStringList &)

/*
Sets the horizontal header labels using labels.
*/
impl /*struct*/ QTableWidget {
  pub fn setHorizontalHeaderLabels_0<RetType, T: QTableWidget_setHorizontalHeaderLabels_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHorizontalHeaderLabels_0(self);
    // return 1;
  }
}
pub trait QTableWidget_setHorizontalHeaderLabels_0<RetType> {
  fn setHorizontalHeaderLabels_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_setHorizontalHeaderLabels_0<(/*void*/)> for (usize) {
  fn setHorizontalHeaderLabels_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget25setHorizontalHeaderLabelsERK11QStringList", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:249
// index:0
// Public Visibility=Default Availability=Available
// [4] int currentRow() const

/*
Returns the row of the current item.

See also currentColumn() and setCurrentCell().
*/
impl /*struct*/ QTableWidget {
  pub fn currentRow_0<RetType, T: QTableWidget_currentRow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentRow_0(self);
    // return 1;
  }
}
pub trait QTableWidget_currentRow_0<RetType> {
  fn currentRow_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_currentRow_0<i32> for () {
  fn currentRow_0(self , rsthis: & QTableWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTableWidget10currentRowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:250
// index:0
// Public Visibility=Default Availability=Available
// [4] int currentColumn() const

/*
Returns the column of the current item.

See also currentRow() and setCurrentCell().
*/
impl /*struct*/ QTableWidget {
  pub fn currentColumn_0<RetType, T: QTableWidget_currentColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentColumn_0(self);
    // return 1;
  }
}
pub trait QTableWidget_currentColumn_0<RetType> {
  fn currentColumn_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_currentColumn_0<i32> for () {
  fn currentColumn_0(self , rsthis: & QTableWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTableWidget13currentColumnEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:251
// index:0
// Public Visibility=Default Availability=Available
// [8] QTableWidgetItem * currentItem() const

/*
Returns the current item.

See also setCurrentItem().
*/
impl /*struct*/ QTableWidget {
  pub fn currentItem_0<RetType, T: QTableWidget_currentItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentItem_0(self);
    // return 1;
  }
}
pub trait QTableWidget_currentItem_0<RetType> {
  fn currentItem_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_currentItem_0<usize> for () {
  fn currentItem_0(self , rsthis: & QTableWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTableWidget11currentItemEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:252
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCurrentItem(QTableWidgetItem *)

/*
Sets the current item to item.

Unless the selection mode is NoSelection, the item is also selected.

See also currentItem() and setCurrentCell().
*/
impl /*struct*/ QTableWidget {
  pub fn setCurrentItem_0<RetType, T: QTableWidget_setCurrentItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentItem_0(self);
    // return 1;
  }
}
pub trait QTableWidget_setCurrentItem_0<RetType> {
  fn setCurrentItem_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_setCurrentItem_0<(/*void*/)> for (usize) {
  fn setCurrentItem_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget14setCurrentItemEP16QTableWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:253
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setCurrentItem(QTableWidgetItem *, QItemSelectionModel::SelectionFlags)

/*
Sets the current item to item.

Unless the selection mode is NoSelection, the item is also selected.

See also currentItem() and setCurrentCell().
*/
impl /*struct*/ QTableWidget {
  pub fn setCurrentItem_1<RetType, T: QTableWidget_setCurrentItem_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentItem_1(self);
    // return 1;
  }
}
pub trait QTableWidget_setCurrentItem_1<RetType> {
  fn setCurrentItem_1(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_setCurrentItem_1<(/*void*/)> for (usize,i32) {
  fn setCurrentItem_1(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget14setCurrentItemEP16QTableWidgetItem6QFlagsIN19QItemSelectionModel13SelectionFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:254
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCurrentCell(int, int)

/*
Sets the current cell to be the cell at position (row, column).

Depending on the current selection mode, the cell may also be selected.

This function was introduced in  Qt 4.1.

See also setCurrentItem(), currentRow(), and currentColumn().
*/
impl /*struct*/ QTableWidget {
  pub fn setCurrentCell_0<RetType, T: QTableWidget_setCurrentCell_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentCell_0(self);
    // return 1;
  }
}
pub trait QTableWidget_setCurrentCell_0<RetType> {
  fn setCurrentCell_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_setCurrentCell_0<(/*void*/)> for (i32,i32) {
  fn setCurrentCell_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget14setCurrentCellEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:255
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setCurrentCell(int, int, QItemSelectionModel::SelectionFlags)

/*
Sets the current cell to be the cell at position (row, column).

Depending on the current selection mode, the cell may also be selected.

This function was introduced in  Qt 4.1.

See also setCurrentItem(), currentRow(), and currentColumn().
*/
impl /*struct*/ QTableWidget {
  pub fn setCurrentCell_1<RetType, T: QTableWidget_setCurrentCell_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentCell_1(self);
    // return 1;
  }
}
pub trait QTableWidget_setCurrentCell_1<RetType> {
  fn setCurrentCell_1(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_setCurrentCell_1<(/*void*/)> for (i32,i32,i32) {
  fn setCurrentCell_1(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget14setCurrentCellEii6QFlagsIN19QItemSelectionModel13SelectionFlagEE", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:257
// index:0
// Public Visibility=Default Availability=Available
// [-2] void sortItems(int, Qt::SortOrder)

/*
Sorts all the rows in the table widget based on column and order.
*/
impl /*struct*/ QTableWidget {
  pub fn sortItems_0<RetType, T: QTableWidget_sortItems_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sortItems_0(self);
    // return 1;
  }
}
pub trait QTableWidget_sortItems_0<RetType> {
  fn sortItems_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_sortItems_0<(/*void*/)> for (i32,i32) {
  fn sortItems_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget9sortItemsEiN2Qt9SortOrderE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:258
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSortingEnabled(bool)

/*

*/
impl /*struct*/ QTableWidget {
  pub fn setSortingEnabled_0<RetType, T: QTableWidget_setSortingEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSortingEnabled_0(self);
    // return 1;
  }
}
pub trait QTableWidget_setSortingEnabled_0<RetType> {
  fn setSortingEnabled_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_setSortingEnabled_0<(/*void*/)> for (bool) {
  fn setSortingEnabled_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget17setSortingEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:259
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isSortingEnabled() const

/*

*/
impl /*struct*/ QTableWidget {
  pub fn isSortingEnabled_0<RetType, T: QTableWidget_isSortingEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isSortingEnabled_0(self);
    // return 1;
  }
}
pub trait QTableWidget_isSortingEnabled_0<RetType> {
  fn isSortingEnabled_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_isSortingEnabled_0<bool> for () {
  fn isSortingEnabled_0(self , rsthis: & QTableWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTableWidget16isSortingEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:261
// index:0
// Public Visibility=Default Availability=Available
// [-2] void editItem(QTableWidgetItem *)

/*
Starts editing the item if it is editable.
*/
impl /*struct*/ QTableWidget {
  pub fn editItem_0<RetType, T: QTableWidget_editItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.editItem_0(self);
    // return 1;
  }
}
pub trait QTableWidget_editItem_0<RetType> {
  fn editItem_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_editItem_0<(/*void*/)> for (usize) {
  fn editItem_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget8editItemEP16QTableWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:262
// index:0
// Public Visibility=Default Availability=Available
// [-2] void openPersistentEditor(QTableWidgetItem *)

/*
Opens an editor for the give item. The editor remains open after editing.

See also closePersistentEditor() and isPersistentEditorOpen().
*/
impl /*struct*/ QTableWidget {
  pub fn openPersistentEditor_0<RetType, T: QTableWidget_openPersistentEditor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.openPersistentEditor_0(self);
    // return 1;
  }
}
pub trait QTableWidget_openPersistentEditor_0<RetType> {
  fn openPersistentEditor_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_openPersistentEditor_0<(/*void*/)> for (usize) {
  fn openPersistentEditor_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget20openPersistentEditorEP16QTableWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:263
// index:0
// Public Visibility=Default Availability=Available
// [-2] void closePersistentEditor(QTableWidgetItem *)

/*
Closes the persistent editor for item.

See also openPersistentEditor() and isPersistentEditorOpen().
*/
impl /*struct*/ QTableWidget {
  pub fn closePersistentEditor_0<RetType, T: QTableWidget_closePersistentEditor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.closePersistentEditor_0(self);
    // return 1;
  }
}
pub trait QTableWidget_closePersistentEditor_0<RetType> {
  fn closePersistentEditor_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_closePersistentEditor_0<(/*void*/)> for (usize) {
  fn closePersistentEditor_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget21closePersistentEditorEP16QTableWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:265
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isPersistentEditorOpen(QTableWidgetItem *) const

/*
Returns whether a persistent editor is open for item item.

This function was introduced in  Qt 5.10.

See also openPersistentEditor() and closePersistentEditor().
*/
impl /*struct*/ QTableWidget {
  pub fn isPersistentEditorOpen_0<RetType, T: QTableWidget_isPersistentEditorOpen_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isPersistentEditorOpen_0(self);
    // return 1;
  }
}
pub trait QTableWidget_isPersistentEditorOpen_0<RetType> {
  fn isPersistentEditorOpen_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_isPersistentEditorOpen_0<bool> for (usize) {
  fn isPersistentEditorOpen_0(self , rsthis: & QTableWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTableWidget22isPersistentEditorOpenEP16QTableWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:267
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * cellWidget(int, int) const

/*
Returns the widget displayed in the cell in the given row and column.

Note: The table takes ownership of the widget.

This function was introduced in  Qt 4.1.

See also setCellWidget().
*/
impl /*struct*/ QTableWidget {
  pub fn cellWidget_0<RetType, T: QTableWidget_cellWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cellWidget_0(self);
    // return 1;
  }
}
pub trait QTableWidget_cellWidget_0<RetType> {
  fn cellWidget_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_cellWidget_0<usize> for (i32,i32) {
  fn cellWidget_0(self , rsthis: & QTableWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTableWidget10cellWidgetEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:268
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCellWidget(int, int, QWidget *)

/*
Sets the given widget to be displayed in the cell in the given row and column, passing the ownership of the widget to the table.

If cell widget A is replaced with cell widget B, cell widget A will be deleted. For example, in the code snippet below, the QLineEdit object will be deleted.


  setCellWidget(row, column, new QLineEdit);
  ...
  setCellWidget(row, column, new QTextEdit);



This function was introduced in  Qt 4.1.

See also cellWidget().
*/
impl /*struct*/ QTableWidget {
  pub fn setCellWidget_0<RetType, T: QTableWidget_setCellWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCellWidget_0(self);
    // return 1;
  }
}
pub trait QTableWidget_setCellWidget_0<RetType> {
  fn setCellWidget_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_setCellWidget_0<(/*void*/)> for (i32,i32,usize) {
  fn setCellWidget_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget13setCellWidgetEiiP7QWidget", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:269
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void removeCellWidget(int, int)

/*
Removes the widget set on the cell indicated by row and column.

This function was introduced in  Qt 4.3.
*/
impl /*struct*/ QTableWidget {
  pub fn removeCellWidget_0<RetType, T: QTableWidget_removeCellWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeCellWidget_0(self);
    // return 1;
  }
}
pub trait QTableWidget_removeCellWidget_0<RetType> {
  fn removeCellWidget_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_removeCellWidget_0<(/*void*/)> for (i32,i32) {
  fn removeCellWidget_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget16removeCellWidgetEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:271
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isItemSelected(const QTableWidgetItem *) const

/*

*/
impl /*struct*/ QTableWidget {
  pub fn isItemSelected_0<RetType, T: QTableWidget_isItemSelected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isItemSelected_0(self);
    // return 1;
  }
}
pub trait QTableWidget_isItemSelected_0<RetType> {
  fn isItemSelected_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_isItemSelected_0<bool> for (usize) {
  fn isItemSelected_0(self , rsthis: & QTableWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTableWidget14isItemSelectedEPK16QTableWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:272
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setItemSelected(const QTableWidgetItem *, bool)

/*

*/
impl /*struct*/ QTableWidget {
  pub fn setItemSelected_0<RetType, T: QTableWidget_setItemSelected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setItemSelected_0(self);
    // return 1;
  }
}
pub trait QTableWidget_setItemSelected_0<RetType> {
  fn setItemSelected_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_setItemSelected_0<(/*void*/)> for (usize,bool) {
  fn setItemSelected_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget15setItemSelectedEPK16QTableWidgetItemb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:273
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRangeSelected(const QTableWidgetSelectionRange &, bool)

/*
Selects or deselects the range depending on select.
*/
impl /*struct*/ QTableWidget {
  pub fn setRangeSelected_0<RetType, T: QTableWidget_setRangeSelected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRangeSelected_0(self);
    // return 1;
  }
}
pub trait QTableWidget_setRangeSelected_0<RetType> {
  fn setRangeSelected_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_setRangeSelected_0<(/*void*/)> for (usize,bool) {
  fn setRangeSelected_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget16setRangeSelectedERK26QTableWidgetSelectionRangeb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:279
// index:0
// Public Visibility=Default Availability=Available
// [4] int visualRow(int) const

/*
Returns the visual row of the given logicalRow.
*/
impl /*struct*/ QTableWidget {
  pub fn visualRow_0<RetType, T: QTableWidget_visualRow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.visualRow_0(self);
    // return 1;
  }
}
pub trait QTableWidget_visualRow_0<RetType> {
  fn visualRow_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_visualRow_0<i32> for (i32) {
  fn visualRow_0(self , rsthis: & QTableWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTableWidget9visualRowEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:280
// index:0
// Public Visibility=Default Availability=Available
// [4] int visualColumn(int) const

/*
Returns the visual column of the given logicalColumn.
*/
impl /*struct*/ QTableWidget {
  pub fn visualColumn_0<RetType, T: QTableWidget_visualColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.visualColumn_0(self);
    // return 1;
  }
}
pub trait QTableWidget_visualColumn_0<RetType> {
  fn visualColumn_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_visualColumn_0<i32> for (i32) {
  fn visualColumn_0(self , rsthis: & QTableWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTableWidget12visualColumnEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:282
// index:0
// Public Visibility=Default Availability=Available
// [8] QTableWidgetItem * itemAt(const QPoint &) const

/*
Returns a pointer to the item at the given point, or returns 0 if point is not covered by an item in the table widget.

See also item().
*/
impl /*struct*/ QTableWidget {
  pub fn itemAt_0<RetType, T: QTableWidget_itemAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemAt_0(self);
    // return 1;
  }
}
pub trait QTableWidget_itemAt_0<RetType> {
  fn itemAt_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_itemAt_0<usize> for (usize) {
  fn itemAt_0(self , rsthis: & QTableWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTableWidget6itemAtERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:283
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QTableWidgetItem * itemAt(int, int) const

/*
Returns a pointer to the item at the given point, or returns 0 if point is not covered by an item in the table widget.

See also item().
*/
impl /*struct*/ QTableWidget {
  pub fn itemAt_1<RetType, T: QTableWidget_itemAt_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemAt_1(self);
    // return 1;
  }
}
pub trait QTableWidget_itemAt_1<RetType> {
  fn itemAt_1(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_itemAt_1<usize> for (i32,i32) {
  fn itemAt_1(self , rsthis: & QTableWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTableWidget6itemAtEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:284
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect visualItemRect(const QTableWidgetItem *) const

/*
Returns the rectangle on the viewport occupied by the item at item.
*/
impl /*struct*/ QTableWidget {
  pub fn visualItemRect_0<RetType, T: QTableWidget_visualItemRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.visualItemRect_0(self);
    // return 1;
  }
}
pub trait QTableWidget_visualItemRect_0<RetType> {
  fn visualItemRect_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_visualItemRect_0<usize> for (usize) {
  fn visualItemRect_0(self , rsthis: & QTableWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTableWidget14visualItemRectEPK16QTableWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:286
// index:0
// Public Visibility=Default Availability=Available
// [8] const QTableWidgetItem * itemPrototype() const

/*
Returns the item prototype used by the table.

See also setItemPrototype().
*/
impl /*struct*/ QTableWidget {
  pub fn itemPrototype_0<RetType, T: QTableWidget_itemPrototype_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemPrototype_0(self);
    // return 1;
  }
}
pub trait QTableWidget_itemPrototype_0<RetType> {
  fn itemPrototype_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_itemPrototype_0<usize> for () {
  fn itemPrototype_0(self , rsthis: & QTableWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTableWidget13itemPrototypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:287
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setItemPrototype(const QTableWidgetItem *)

/*
Sets the item prototype for the table to the specified item.

The table widget will use the item prototype clone function when it needs to create a new table item. For example when the user is editing in an empty cell. This is useful when you have a QTableWidgetItem subclass and want to make sure that QTableWidget creates instances of your subclass.

The table takes ownership of the prototype.

See also itemPrototype().
*/
impl /*struct*/ QTableWidget {
  pub fn setItemPrototype_0<RetType, T: QTableWidget_setItemPrototype_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setItemPrototype_0(self);
    // return 1;
  }
}
pub trait QTableWidget_setItemPrototype_0<RetType> {
  fn setItemPrototype_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_setItemPrototype_0<(/*void*/)> for (usize) {
  fn setItemPrototype_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget16setItemPrototypeEPK16QTableWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:290
// index:0
// Public Visibility=Default Availability=Available
// [-2] void scrollToItem(const QTableWidgetItem *, QAbstractItemView::ScrollHint)

/*
Scrolls the view if necessary to ensure that the item is visible. The hint parameter specifies more precisely where the item should be located after the operation.
*/
impl /*struct*/ QTableWidget {
  pub fn scrollToItem_0<RetType, T: QTableWidget_scrollToItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scrollToItem_0(self);
    // return 1;
  }
}
pub trait QTableWidget_scrollToItem_0<RetType> {
  fn scrollToItem_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_scrollToItem_0<(/*void*/)> for (usize,i32) {
  fn scrollToItem_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget12scrollToItemEPK16QTableWidgetItemN17QAbstractItemView10ScrollHintE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:291
// index:0
// Public Visibility=Default Availability=Available
// [-2] void insertRow(int)

/*
Inserts an empty row into the table at row.
*/
impl /*struct*/ QTableWidget {
  pub fn insertRow_0<RetType, T: QTableWidget_insertRow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertRow_0(self);
    // return 1;
  }
}
pub trait QTableWidget_insertRow_0<RetType> {
  fn insertRow_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_insertRow_0<(/*void*/)> for (i32) {
  fn insertRow_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget9insertRowEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:292
// index:0
// Public Visibility=Default Availability=Available
// [-2] void insertColumn(int)

/*
Inserts an empty column into the table at column.
*/
impl /*struct*/ QTableWidget {
  pub fn insertColumn_0<RetType, T: QTableWidget_insertColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertColumn_0(self);
    // return 1;
  }
}
pub trait QTableWidget_insertColumn_0<RetType> {
  fn insertColumn_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_insertColumn_0<(/*void*/)> for (i32) {
  fn insertColumn_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget12insertColumnEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:293
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeRow(int)

/*
Removes the row row and all its items from the table.
*/
impl /*struct*/ QTableWidget {
  pub fn removeRow_0<RetType, T: QTableWidget_removeRow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeRow_0(self);
    // return 1;
  }
}
pub trait QTableWidget_removeRow_0<RetType> {
  fn removeRow_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_removeRow_0<(/*void*/)> for (i32) {
  fn removeRow_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget9removeRowEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:294
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeColumn(int)

/*
Removes the column column and all its items from the table.
*/
impl /*struct*/ QTableWidget {
  pub fn removeColumn_0<RetType, T: QTableWidget_removeColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeColumn_0(self);
    // return 1;
  }
}
pub trait QTableWidget_removeColumn_0<RetType> {
  fn removeColumn_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_removeColumn_0<(/*void*/)> for (i32) {
  fn removeColumn_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget12removeColumnEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:295
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clear()

/*
Removes all items in the view. This will also remove all selections and headers. If you don't want to remove the headers, use QTableWidget::clearContents(). The table dimensions stay the same.
*/
impl /*struct*/ QTableWidget {
  pub fn clear_0<RetType, T: QTableWidget_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QTableWidget_clear_0<RetType> {
  fn clear_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_clear_0<(/*void*/)> for () {
  fn clear_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN12QTableWidget5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:296
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clearContents()

/*
Removes all items not in the headers from the view. This will also remove all selections. The table dimensions stay the same.

This function was introduced in  Qt 4.2.
*/
impl /*struct*/ QTableWidget {
  pub fn clearContents_0<RetType, T: QTableWidget_clearContents_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clearContents_0(self);
    // return 1;
  }
}
pub trait QTableWidget_clearContents_0<RetType> {
  fn clearContents_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_clearContents_0<(/*void*/)> for () {
  fn clearContents_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN12QTableWidget13clearContentsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:299
// index:0
// Public Visibility=Default Availability=Available
// [-2] void itemPressed(QTableWidgetItem *)

/*
This signal is emitted whenever an item in the table is pressed. The item specified is the item that was pressed.
*/
impl /*struct*/ QTableWidget {
  pub fn itemPressed_0<RetType, T: QTableWidget_itemPressed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemPressed_0(self);
    // return 1;
  }
}
pub trait QTableWidget_itemPressed_0<RetType> {
  fn itemPressed_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_itemPressed_0<(/*void*/)> for (usize) {
  fn itemPressed_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget11itemPressedEP16QTableWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:300
// index:0
// Public Visibility=Default Availability=Available
// [-2] void itemClicked(QTableWidgetItem *)

/*
This signal is emitted whenever an item in the table is clicked. The item specified is the item that was clicked.
*/
impl /*struct*/ QTableWidget {
  pub fn itemClicked_0<RetType, T: QTableWidget_itemClicked_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemClicked_0(self);
    // return 1;
  }
}
pub trait QTableWidget_itemClicked_0<RetType> {
  fn itemClicked_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_itemClicked_0<(/*void*/)> for (usize) {
  fn itemClicked_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget11itemClickedEP16QTableWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:301
// index:0
// Public Visibility=Default Availability=Available
// [-2] void itemDoubleClicked(QTableWidgetItem *)

/*
This signal is emitted whenever an item in the table is double clicked. The item specified is the item that was double clicked.
*/
impl /*struct*/ QTableWidget {
  pub fn itemDoubleClicked_0<RetType, T: QTableWidget_itemDoubleClicked_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemDoubleClicked_0(self);
    // return 1;
  }
}
pub trait QTableWidget_itemDoubleClicked_0<RetType> {
  fn itemDoubleClicked_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_itemDoubleClicked_0<(/*void*/)> for (usize) {
  fn itemDoubleClicked_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget17itemDoubleClickedEP16QTableWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:303
// index:0
// Public Visibility=Default Availability=Available
// [-2] void itemActivated(QTableWidgetItem *)

/*
This signal is emitted when the specified item has been activated
*/
impl /*struct*/ QTableWidget {
  pub fn itemActivated_0<RetType, T: QTableWidget_itemActivated_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemActivated_0(self);
    // return 1;
  }
}
pub trait QTableWidget_itemActivated_0<RetType> {
  fn itemActivated_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_itemActivated_0<(/*void*/)> for (usize) {
  fn itemActivated_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget13itemActivatedEP16QTableWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:304
// index:0
// Public Visibility=Default Availability=Available
// [-2] void itemEntered(QTableWidgetItem *)

/*
This signal is emitted when the mouse cursor enters an item. The item is the item entered.

This signal is only emitted when mouseTracking is turned on, or when a mouse button is pressed while moving into an item.
*/
impl /*struct*/ QTableWidget {
  pub fn itemEntered_0<RetType, T: QTableWidget_itemEntered_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemEntered_0(self);
    // return 1;
  }
}
pub trait QTableWidget_itemEntered_0<RetType> {
  fn itemEntered_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_itemEntered_0<(/*void*/)> for (usize) {
  fn itemEntered_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget11itemEnteredEP16QTableWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:305
// index:0
// Public Visibility=Default Availability=Available
// [-2] void itemChanged(QTableWidgetItem *)

/*
This signal is emitted whenever the data of item has changed.
*/
impl /*struct*/ QTableWidget {
  pub fn itemChanged_0<RetType, T: QTableWidget_itemChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemChanged_0(self);
    // return 1;
  }
}
pub trait QTableWidget_itemChanged_0<RetType> {
  fn itemChanged_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_itemChanged_0<(/*void*/)> for (usize) {
  fn itemChanged_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget11itemChangedEP16QTableWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:307
// index:0
// Public Visibility=Default Availability=Available
// [-2] void currentItemChanged(QTableWidgetItem *, QTableWidgetItem *)

/*
This signal is emitted whenever the current item changes. The previous item is the item that previously had the focus, current is the new current item.
*/
impl /*struct*/ QTableWidget {
  pub fn currentItemChanged_0<RetType, T: QTableWidget_currentItemChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentItemChanged_0(self);
    // return 1;
  }
}
pub trait QTableWidget_currentItemChanged_0<RetType> {
  fn currentItemChanged_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_currentItemChanged_0<(/*void*/)> for (usize,usize) {
  fn currentItemChanged_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget18currentItemChangedEP16QTableWidgetItemS1_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:308
// index:0
// Public Visibility=Default Availability=Available
// [-2] void itemSelectionChanged()

/*
This signal is emitted whenever the selection changes.

See also selectedItems() and QTableWidgetItem::isSelected().
*/
impl /*struct*/ QTableWidget {
  pub fn itemSelectionChanged_0<RetType, T: QTableWidget_itemSelectionChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemSelectionChanged_0(self);
    // return 1;
  }
}
pub trait QTableWidget_itemSelectionChanged_0<RetType> {
  fn itemSelectionChanged_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_itemSelectionChanged_0<(/*void*/)> for () {
  fn itemSelectionChanged_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN12QTableWidget20itemSelectionChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:310
// index:0
// Public Visibility=Default Availability=Available
// [-2] void cellPressed(int, int)

/*
This signal is emitted whenever a cell in the table is pressed. The row and column specified is the cell that was pressed.

This function was introduced in  Qt 4.1.
*/
impl /*struct*/ QTableWidget {
  pub fn cellPressed_0<RetType, T: QTableWidget_cellPressed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cellPressed_0(self);
    // return 1;
  }
}
pub trait QTableWidget_cellPressed_0<RetType> {
  fn cellPressed_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_cellPressed_0<(/*void*/)> for (i32,i32) {
  fn cellPressed_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget11cellPressedEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:311
// index:0
// Public Visibility=Default Availability=Available
// [-2] void cellClicked(int, int)

/*
This signal is emitted whenever a cell in the table is clicked. The row and column specified is the cell that was clicked.

This function was introduced in  Qt 4.1.
*/
impl /*struct*/ QTableWidget {
  pub fn cellClicked_0<RetType, T: QTableWidget_cellClicked_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cellClicked_0(self);
    // return 1;
  }
}
pub trait QTableWidget_cellClicked_0<RetType> {
  fn cellClicked_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_cellClicked_0<(/*void*/)> for (i32,i32) {
  fn cellClicked_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget11cellClickedEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:312
// index:0
// Public Visibility=Default Availability=Available
// [-2] void cellDoubleClicked(int, int)

/*
This signal is emitted whenever a cell in the table is double clicked. The row and column specified is the cell that was double clicked.

This function was introduced in  Qt 4.1.
*/
impl /*struct*/ QTableWidget {
  pub fn cellDoubleClicked_0<RetType, T: QTableWidget_cellDoubleClicked_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cellDoubleClicked_0(self);
    // return 1;
  }
}
pub trait QTableWidget_cellDoubleClicked_0<RetType> {
  fn cellDoubleClicked_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_cellDoubleClicked_0<(/*void*/)> for (i32,i32) {
  fn cellDoubleClicked_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget17cellDoubleClickedEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:314
// index:0
// Public Visibility=Default Availability=Available
// [-2] void cellActivated(int, int)

/*
This signal is emitted when the cell specified by row and column has been activated

This function was introduced in  Qt 4.1.
*/
impl /*struct*/ QTableWidget {
  pub fn cellActivated_0<RetType, T: QTableWidget_cellActivated_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cellActivated_0(self);
    // return 1;
  }
}
pub trait QTableWidget_cellActivated_0<RetType> {
  fn cellActivated_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_cellActivated_0<(/*void*/)> for (i32,i32) {
  fn cellActivated_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget13cellActivatedEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:315
// index:0
// Public Visibility=Default Availability=Available
// [-2] void cellEntered(int, int)

/*
This signal is emitted when the mouse cursor enters a cell. The cell is specified by row and column.

This signal is only emitted when mouseTracking is turned on, or when a mouse button is pressed while moving into an item.

This function was introduced in  Qt 4.1.
*/
impl /*struct*/ QTableWidget {
  pub fn cellEntered_0<RetType, T: QTableWidget_cellEntered_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cellEntered_0(self);
    // return 1;
  }
}
pub trait QTableWidget_cellEntered_0<RetType> {
  fn cellEntered_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_cellEntered_0<(/*void*/)> for (i32,i32) {
  fn cellEntered_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget11cellEnteredEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:316
// index:0
// Public Visibility=Default Availability=Available
// [-2] void cellChanged(int, int)

/*
This signal is emitted whenever the data of the item in the cell specified by row and column has changed.

This function was introduced in  Qt 4.1.
*/
impl /*struct*/ QTableWidget {
  pub fn cellChanged_0<RetType, T: QTableWidget_cellChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cellChanged_0(self);
    // return 1;
  }
}
pub trait QTableWidget_cellChanged_0<RetType> {
  fn cellChanged_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_cellChanged_0<(/*void*/)> for (i32,i32) {
  fn cellChanged_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget11cellChangedEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:318
// index:0
// Public Visibility=Default Availability=Available
// [-2] void currentCellChanged(int, int, int, int)

/*
This signal is emitted whenever the current cell changes. The cell specified by previousRow and previousColumn is the cell that previously had the focus, the cell specified by currentRow and currentColumn is the new current cell.

This function was introduced in  Qt 4.1.
*/
impl /*struct*/ QTableWidget {
  pub fn currentCellChanged_0<RetType, T: QTableWidget_currentCellChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentCellChanged_0(self);
    // return 1;
  }
}
pub trait QTableWidget_currentCellChanged_0<RetType> {
  fn currentCellChanged_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_currentCellChanged_0<(/*void*/)> for (i32,i32,i32,i32) {
  fn currentCellChanged_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget18currentCellChangedEiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:321
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QTableWidget {
  pub fn event_0<RetType, T: QTableWidget_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QTableWidget_event_0<RetType> {
  fn event_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QTableWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QTableWidget5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:322
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] QStringList mimeTypes() const

/*
Returns a list of MIME types that can be used to describe a list of tablewidget items.

See also mimeData().
*/
impl /*struct*/ QTableWidget {
  pub fn mimeTypes_0<RetType, T: QTableWidget_mimeTypes_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mimeTypes_0(self);
    // return 1;
  }
}
pub trait QTableWidget_mimeTypes_0<RetType> {
  fn mimeTypes_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_mimeTypes_0<usize> for () {
  fn mimeTypes_0(self , rsthis: & QTableWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTableWidget9mimeTypesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:328
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool dropMimeData(int, int, const QMimeData *, Qt::DropAction)

/*
Handles the data supplied by a drag and drop operation that ended with the given action in the given row and column. Returns true if the data and action can be handled by the model; otherwise returns false.

See also supportedDropActions().
*/
impl /*struct*/ QTableWidget {
  pub fn dropMimeData_0<RetType, T: QTableWidget_dropMimeData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dropMimeData_0(self);
    // return 1;
  }
}
pub trait QTableWidget_dropMimeData_0<RetType> {
  fn dropMimeData_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_dropMimeData_0<bool> for (i32,i32,usize,i32) {
  fn dropMimeData_0(self , rsthis: & QTableWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QTableWidget12dropMimeDataEiiPK9QMimeDataN2Qt10DropActionE", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:329
// index:0
// Protected virtual Visibility=Default Availability=Available
// [4] Qt::DropActions supportedDropActions() const

/*
Returns the drop actions supported by this view.

See also Qt::DropActions.
*/
impl /*struct*/ QTableWidget {
  pub fn supportedDropActions_0<RetType, T: QTableWidget_supportedDropActions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.supportedDropActions_0(self);
    // return 1;
  }
}
pub trait QTableWidget_supportedDropActions_0<RetType> {
  fn supportedDropActions_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_supportedDropActions_0<i32> for () {
  fn supportedDropActions_0(self , rsthis: & QTableWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTableWidget20supportedDropActionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:338
// index:0
// Protected Visibility=Default Availability=Available
// [24] QModelIndex indexFromItem(QTableWidgetItem *) const

/*
Returns the QModelIndex associated with the given item.
*/
impl /*struct*/ QTableWidget {
  pub fn indexFromItem_0<RetType, T: QTableWidget_indexFromItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexFromItem_0(self);
    // return 1;
  }
}
pub trait QTableWidget_indexFromItem_0<RetType> {
  fn indexFromItem_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_indexFromItem_0<usize> for (usize) {
  fn indexFromItem_0(self , rsthis: & QTableWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTableWidget13indexFromItemEP16QTableWidgetItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:339
// index:0
// Protected Visibility=Default Availability=Available
// [8] QTableWidgetItem * itemFromIndex(const QModelIndex &) const

/*
Returns a pointer to the QTableWidgetItem associated with the given index.
*/
impl /*struct*/ QTableWidget {
  pub fn itemFromIndex_0<RetType, T: QTableWidget_itemFromIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemFromIndex_0(self);
    // return 1;
  }
}
pub trait QTableWidget_itemFromIndex_0<RetType> {
  fn itemFromIndex_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_itemFromIndex_0<usize> for (usize) {
  fn itemFromIndex_0(self , rsthis: & QTableWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTableWidget13itemFromIndexERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtablewidget.h:343
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dropEvent(QDropEvent *)

/*
Reimplemented from QWidget::dropEvent().
*/
impl /*struct*/ QTableWidget {
  pub fn dropEvent_0<RetType, T: QTableWidget_dropEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dropEvent_0(self);
    // return 1;
  }
}
pub trait QTableWidget_dropEvent_0<RetType> {
  fn dropEvent_0(self , rsthis: & QTableWidget) -> RetType;
}
impl<'a> /*trait*/ QTableWidget_dropEvent_0<(/*void*/)> for (usize) {
  fn dropEvent_0(self , rsthis: & QTableWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QTableWidget9dropEventEP10QDropEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
