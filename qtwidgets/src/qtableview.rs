

// mod ::widgets::QTableView
// package qtwidgets
// /usr/include/qt/QtWidgets/qtableview.h
// #include <qtableview.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 22
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

// void rowMoved(int, int, int)
// func (this *QTableView) InheritRowMoved(f func(row int, oldIndex int, newIndex int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "rowMoved", f)
// }

// void columnMoved(int, int, int)
// func (this *QTableView) InheritColumnMoved(f func(column int, oldIndex int, newIndex int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "columnMoved", f)
// }

// void rowResized(int, int, int)
// func (this *QTableView) InheritRowResized(f func(row int, oldHeight int, newHeight int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "rowResized", f)
// }

// void columnResized(int, int, int)
// func (this *QTableView) InheritColumnResized(f func(column int, oldWidth int, newWidth int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "columnResized", f)
// }

// void rowCountChanged(int, int)
// func (this *QTableView) InheritRowCountChanged(f func(oldCount int, newCount int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "rowCountChanged", f)
// }

// void columnCountChanged(int, int)
// func (this *QTableView) InheritColumnCountChanged(f func(oldCount int, newCount int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "columnCountChanged", f)
// }

// void scrollContentsBy(int, int)
// func (this *QTableView) InheritScrollContentsBy(f func(dx int, dy int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "scrollContentsBy", f)
// }

// QStyleOptionViewItem viewOptions()
// func (this *QTableView) InheritViewOptions(f func() unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "viewOptions", f)
// }

// void paintEvent(QPaintEvent *)
// func (this *QTableView) InheritPaintEvent(f func(e *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }

// void timerEvent(QTimerEvent *)
// func (this *QTableView) InheritTimerEvent(f func(event *qtcore.QTimerEvent/*777 QTimerEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "timerEvent", f)
// }

// int horizontalOffset()
// func (this *QTableView) InheritHorizontalOffset(f func() int) {
//  qtrt.SetAllInheritCallback(this, "horizontalOffset", f)
// }

// int verticalOffset()
// func (this *QTableView) InheritVerticalOffset(f func() int) {
//  qtrt.SetAllInheritCallback(this, "verticalOffset", f)
// }

// QModelIndex moveCursor(QAbstractItemView::CursorAction, Qt::KeyboardModifiers)
// func (this *QTableView) InheritMoveCursor(f func(cursorAction int, modifiers int) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "moveCursor", f)
// }

// void setSelection(const QRect &, QItemSelectionModel::SelectionFlags)
// func (this *QTableView) InheritSetSelection(f func(rect *qtcore.QRect, command int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "setSelection", f)
// }

// QRegion visualRegionForSelection(const QItemSelection &)
// func (this *QTableView) InheritVisualRegionForSelection(f func(selection *qtcore.QItemSelection) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "visualRegionForSelection", f)
// }

// QModelIndexList selectedIndexes()
// func (this *QTableView) InheritSelectedIndexes(f func() *qtcore.QModelIndexList/*9999*/) {
//  qtrt.SetAllInheritCallback(this, "selectedIndexes", f)
// }

// void updateGeometries()
// func (this *QTableView) InheritUpdateGeometries(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "updateGeometries", f)
// }

// QSize viewportSizeHint()
// func (this *QTableView) InheritViewportSizeHint(f func() unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "viewportSizeHint", f)
// }

// int sizeHintForRow(int)
// func (this *QTableView) InheritSizeHintForRow(f func(row int) int) {
//  qtrt.SetAllInheritCallback(this, "sizeHintForRow", f)
// }

// int sizeHintForColumn(int)
// func (this *QTableView) InheritSizeHintForColumn(f func(column int) int) {
//  qtrt.SetAllInheritCallback(this, "sizeHintForColumn", f)
// }

// void verticalScrollbarAction(int)
// func (this *QTableView) InheritVerticalScrollbarAction(f func(action int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "verticalScrollbarAction", f)
// }

// void horizontalScrollbarAction(int)
// func (this *QTableView) InheritHorizontalScrollbarAction(f func(action int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "horizontalScrollbarAction", f)
// }

// bool isIndexHidden(const QModelIndex &)
// func (this *QTableView) InheritIsIndexHidden(f func(index *qtcore.QModelIndex) bool) {
//  qtrt.SetAllInheritCallback(this, "isIndexHidden", f)
// }

// void selectionChanged(const QItemSelection &, const QItemSelection &)
// func (this *QTableView) InheritSelectionChanged(f func(selected *qtcore.QItemSelection, deselected *qtcore.QItemSelection) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "selectionChanged", f)
// }

// void currentChanged(const QModelIndex &, const QModelIndex &)
// func (this *QTableView) InheritCurrentChanged(f func(current *qtcore.QModelIndex, previous *qtcore.QModelIndex) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "currentChanged", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QTableView)=48
pub struct QTableView {
  qbase: QAbstractItemView,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTableView_ITF interface {
//    QAbstractItemView_ITF
//    QTableView_PTR() *QTableView
//}
//func (ptr *QTableView) QTableView_PTR() *QTableView { return ptr }

impl /*struct*/ QTableView {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTableView {
    return QTableView{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTableView {
//  type Target = QTableViewBASE;
//
//  fn deref(&self) -> &QTableViewBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTableViewBASE> for QTableView {
//  fn as_ref(& self) -> & QTableViewBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qtableview.h:55
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QTableView {
  pub fn metaObject_0<RetType, T: QTableView_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QTableView_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QTableView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTableView10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:65
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTableView(QWidget *)

/*
Constructs a table view with a parent to represent the data.

See also QAbstractItemModel.
*/
// QTableView(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QTableView {
  pub fn QTableView_0<T: QTableView_QTableView_0>(value: T) -> QTableView {
    let rsthis = value.QTableView_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTableView_QTableView_0 {
  fn QTableView_0(self) -> QTableView;
}
// QTableView(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTableView_QTableView_0 for (usize) {
  fn QTableView_0(self) -> QTableView {
    // unsafe{_ZN10QTableViewC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QTableViewC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTableView{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:66
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QTableView()

/*

*/
pub fn DeleteQTableView(this :*mut QTableView) {
    // let rv = qtrt::InvokeQtFunc6("_ZN10QTableViewD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qtableview.h:68
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setModel(QAbstractItemModel *)

/*
Reimplemented from QAbstractItemView::setModel().
*/
impl /*struct*/ QTableView {
  pub fn setModel_0<RetType, T: QTableView_setModel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setModel_0(self);
    // return 1;
  }
}
pub trait QTableView_setModel_0<RetType> {
  fn setModel_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_setModel_0<(/*void*/)> for (usize) {
  fn setModel_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QTableView8setModelEP18QAbstractItemModel", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:69
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setRootIndex(const QModelIndex &)

/*
Reimplemented from QAbstractItemView::setRootIndex().
*/
impl /*struct*/ QTableView {
  pub fn setRootIndex_0<RetType, T: QTableView_setRootIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRootIndex_0(self);
    // return 1;
  }
}
pub trait QTableView_setRootIndex_0<RetType> {
  fn setRootIndex_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_setRootIndex_0<(/*void*/)> for (usize) {
  fn setRootIndex_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QTableView12setRootIndexERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:70
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setSelectionModel(QItemSelectionModel *)

/*
Reimplemented from QAbstractItemView::setSelectionModel().
*/
impl /*struct*/ QTableView {
  pub fn setSelectionModel_0<RetType, T: QTableView_setSelectionModel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSelectionModel_0(self);
    // return 1;
  }
}
pub trait QTableView_setSelectionModel_0<RetType> {
  fn setSelectionModel_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_setSelectionModel_0<(/*void*/)> for (usize) {
  fn setSelectionModel_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QTableView17setSelectionModelEP19QItemSelectionModel", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:71
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void doItemsLayout()

/*

*/
impl /*struct*/ QTableView {
  pub fn doItemsLayout_0<RetType, T: QTableView_doItemsLayout_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.doItemsLayout_0(self);
    // return 1;
  }
}
pub trait QTableView_doItemsLayout_0<RetType> {
  fn doItemsLayout_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_doItemsLayout_0<(/*void*/)> for () {
  fn doItemsLayout_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QTableView13doItemsLayoutEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:73
// index:0
// Public Visibility=Default Availability=Available
// [8] QHeaderView * horizontalHeader() const

/*
Returns the table view's horizontal header.

See also setHorizontalHeader(), verticalHeader(), and QAbstractItemModel::headerData().
*/
impl /*struct*/ QTableView {
  pub fn horizontalHeader_0<RetType, T: QTableView_horizontalHeader_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.horizontalHeader_0(self);
    // return 1;
  }
}
pub trait QTableView_horizontalHeader_0<RetType> {
  fn horizontalHeader_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_horizontalHeader_0<usize> for () {
  fn horizontalHeader_0(self , rsthis: & QTableView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTableView16horizontalHeaderEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:74
// index:0
// Public Visibility=Default Availability=Available
// [8] QHeaderView * verticalHeader() const

/*
Returns the table view's vertical header.

See also setVerticalHeader(), horizontalHeader(), and QAbstractItemModel::headerData().
*/
impl /*struct*/ QTableView {
  pub fn verticalHeader_0<RetType, T: QTableView_verticalHeader_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.verticalHeader_0(self);
    // return 1;
  }
}
pub trait QTableView_verticalHeader_0<RetType> {
  fn verticalHeader_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_verticalHeader_0<usize> for () {
  fn verticalHeader_0(self , rsthis: & QTableView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTableView14verticalHeaderEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:75
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setHorizontalHeader(QHeaderView *)

/*
Sets the widget to use for the horizontal header to header.

See also horizontalHeader() and setVerticalHeader().
*/
impl /*struct*/ QTableView {
  pub fn setHorizontalHeader_0<RetType, T: QTableView_setHorizontalHeader_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHorizontalHeader_0(self);
    // return 1;
  }
}
pub trait QTableView_setHorizontalHeader_0<RetType> {
  fn setHorizontalHeader_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_setHorizontalHeader_0<(/*void*/)> for (usize) {
  fn setHorizontalHeader_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QTableView19setHorizontalHeaderEP11QHeaderView", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:76
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setVerticalHeader(QHeaderView *)

/*
Sets the widget to use for the vertical header to header.

See also verticalHeader() and setHorizontalHeader().
*/
impl /*struct*/ QTableView {
  pub fn setVerticalHeader_0<RetType, T: QTableView_setVerticalHeader_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVerticalHeader_0(self);
    // return 1;
  }
}
pub trait QTableView_setVerticalHeader_0<RetType> {
  fn setVerticalHeader_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_setVerticalHeader_0<(/*void*/)> for (usize) {
  fn setVerticalHeader_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QTableView17setVerticalHeaderEP11QHeaderView", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:78
// index:0
// Public Visibility=Default Availability=Available
// [4] int rowViewportPosition(int) const

/*
Returns the y-coordinate in contents coordinates of the given row.
*/
impl /*struct*/ QTableView {
  pub fn rowViewportPosition_0<RetType, T: QTableView_rowViewportPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowViewportPosition_0(self);
    // return 1;
  }
}
pub trait QTableView_rowViewportPosition_0<RetType> {
  fn rowViewportPosition_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_rowViewportPosition_0<i32> for (i32) {
  fn rowViewportPosition_0(self , rsthis: & QTableView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTableView19rowViewportPositionEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:79
// index:0
// Public Visibility=Default Availability=Available
// [4] int rowAt(int) const

/*
Returns the row in which the given y-coordinate, y, in contents coordinates is located.

Note: This function returns -1 if the given coordinate is not valid (has no row).

See also columnAt().
*/
impl /*struct*/ QTableView {
  pub fn rowAt_0<RetType, T: QTableView_rowAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowAt_0(self);
    // return 1;
  }
}
pub trait QTableView_rowAt_0<RetType> {
  fn rowAt_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_rowAt_0<i32> for (i32) {
  fn rowAt_0(self , rsthis: & QTableView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTableView5rowAtEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:81
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRowHeight(int, int)

/*
Sets the height of the given row to be height.

This function was introduced in  Qt 4.1.

See also rowHeight().
*/
impl /*struct*/ QTableView {
  pub fn setRowHeight_0<RetType, T: QTableView_setRowHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRowHeight_0(self);
    // return 1;
  }
}
pub trait QTableView_setRowHeight_0<RetType> {
  fn setRowHeight_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_setRowHeight_0<(/*void*/)> for (i32,i32) {
  fn setRowHeight_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTableView12setRowHeightEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:82
// index:0
// Public Visibility=Default Availability=Available
// [4] int rowHeight(int) const

/*
Returns the height of the given row.

See also setRowHeight(), resizeRowToContents(), and columnWidth().
*/
impl /*struct*/ QTableView {
  pub fn rowHeight_0<RetType, T: QTableView_rowHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowHeight_0(self);
    // return 1;
  }
}
pub trait QTableView_rowHeight_0<RetType> {
  fn rowHeight_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_rowHeight_0<i32> for (i32) {
  fn rowHeight_0(self , rsthis: & QTableView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTableView9rowHeightEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:84
// index:0
// Public Visibility=Default Availability=Available
// [4] int columnViewportPosition(int) const

/*
Returns the x-coordinate in contents coordinates of the given column.
*/
impl /*struct*/ QTableView {
  pub fn columnViewportPosition_0<RetType, T: QTableView_columnViewportPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnViewportPosition_0(self);
    // return 1;
  }
}
pub trait QTableView_columnViewportPosition_0<RetType> {
  fn columnViewportPosition_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_columnViewportPosition_0<i32> for (i32) {
  fn columnViewportPosition_0(self , rsthis: & QTableView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTableView22columnViewportPositionEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:85
// index:0
// Public Visibility=Default Availability=Available
// [4] int columnAt(int) const

/*
Returns the column in which the given x-coordinate, x, in contents coordinates is located.

Note: This function returns -1 if the given coordinate is not valid (has no column).

See also rowAt().
*/
impl /*struct*/ QTableView {
  pub fn columnAt_0<RetType, T: QTableView_columnAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnAt_0(self);
    // return 1;
  }
}
pub trait QTableView_columnAt_0<RetType> {
  fn columnAt_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_columnAt_0<i32> for (i32) {
  fn columnAt_0(self , rsthis: & QTableView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTableView8columnAtEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:87
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setColumnWidth(int, int)

/*
Sets the width of the given column to be width.

This function was introduced in  Qt 4.1.

See also columnWidth().
*/
impl /*struct*/ QTableView {
  pub fn setColumnWidth_0<RetType, T: QTableView_setColumnWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setColumnWidth_0(self);
    // return 1;
  }
}
pub trait QTableView_setColumnWidth_0<RetType> {
  fn setColumnWidth_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_setColumnWidth_0<(/*void*/)> for (i32,i32) {
  fn setColumnWidth_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTableView14setColumnWidthEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:88
// index:0
// Public Visibility=Default Availability=Available
// [4] int columnWidth(int) const

/*
Returns the width of the given column.

See also setColumnWidth(), resizeColumnToContents(), and rowHeight().
*/
impl /*struct*/ QTableView {
  pub fn columnWidth_0<RetType, T: QTableView_columnWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnWidth_0(self);
    // return 1;
  }
}
pub trait QTableView_columnWidth_0<RetType> {
  fn columnWidth_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_columnWidth_0<i32> for (i32) {
  fn columnWidth_0(self , rsthis: & QTableView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTableView11columnWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:90
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isRowHidden(int) const

/*
Returns true if the given row is hidden; otherwise returns false.

See also isColumnHidden().
*/
impl /*struct*/ QTableView {
  pub fn isRowHidden_0<RetType, T: QTableView_isRowHidden_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isRowHidden_0(self);
    // return 1;
  }
}
pub trait QTableView_isRowHidden_0<RetType> {
  fn isRowHidden_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_isRowHidden_0<bool> for (i32) {
  fn isRowHidden_0(self , rsthis: & QTableView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTableView11isRowHiddenEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:91
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRowHidden(int, bool)

/*
If hide is true row will be hidden, otherwise it will be shown.

See also isRowHidden() and setColumnHidden().
*/
impl /*struct*/ QTableView {
  pub fn setRowHidden_0<RetType, T: QTableView_setRowHidden_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRowHidden_0(self);
    // return 1;
  }
}
pub trait QTableView_setRowHidden_0<RetType> {
  fn setRowHidden_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_setRowHidden_0<(/*void*/)> for (i32,bool) {
  fn setRowHidden_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN10QTableView12setRowHiddenEib", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:93
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isColumnHidden(int) const

/*
Returns true if the given column is hidden; otherwise returns false.

See also isRowHidden().
*/
impl /*struct*/ QTableView {
  pub fn isColumnHidden_0<RetType, T: QTableView_isColumnHidden_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isColumnHidden_0(self);
    // return 1;
  }
}
pub trait QTableView_isColumnHidden_0<RetType> {
  fn isColumnHidden_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_isColumnHidden_0<bool> for (i32) {
  fn isColumnHidden_0(self , rsthis: & QTableView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTableView14isColumnHiddenEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:94
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setColumnHidden(int, bool)

/*
If hide is true the given column will be hidden; otherwise it will be shown.

See also isColumnHidden() and setRowHidden().
*/
impl /*struct*/ QTableView {
  pub fn setColumnHidden_0<RetType, T: QTableView_setColumnHidden_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setColumnHidden_0(self);
    // return 1;
  }
}
pub trait QTableView_setColumnHidden_0<RetType> {
  fn setColumnHidden_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_setColumnHidden_0<(/*void*/)> for (i32,bool) {
  fn setColumnHidden_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN10QTableView15setColumnHiddenEib", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:96
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSortingEnabled(bool)

/*
If enable is true, enables sorting for the table and immediately trigger a call to sortByColumn() with the current sort section and order

Note: Setter function for property sortingEnabled. 

See also isSortingEnabled().
*/
impl /*struct*/ QTableView {
  pub fn setSortingEnabled_0<RetType, T: QTableView_setSortingEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSortingEnabled_0(self);
    // return 1;
  }
}
pub trait QTableView_setSortingEnabled_0<RetType> {
  fn setSortingEnabled_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_setSortingEnabled_0<(/*void*/)> for (bool) {
  fn setSortingEnabled_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN10QTableView17setSortingEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:97
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isSortingEnabled() const

/*

*/
impl /*struct*/ QTableView {
  pub fn isSortingEnabled_0<RetType, T: QTableView_isSortingEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isSortingEnabled_0(self);
    // return 1;
  }
}
pub trait QTableView_isSortingEnabled_0<RetType> {
  fn isSortingEnabled_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_isSortingEnabled_0<bool> for () {
  fn isSortingEnabled_0(self , rsthis: & QTableView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTableView16isSortingEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:99
// index:0
// Public Visibility=Default Availability=Available
// [1] bool showGrid() const

/*

*/
impl /*struct*/ QTableView {
  pub fn showGrid_0<RetType, T: QTableView_showGrid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showGrid_0(self);
    // return 1;
  }
}
pub trait QTableView_showGrid_0<RetType> {
  fn showGrid_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_showGrid_0<bool> for () {
  fn showGrid_0(self , rsthis: & QTableView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTableView8showGridEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:101
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::PenStyle gridStyle() const

/*

*/
impl /*struct*/ QTableView {
  pub fn gridStyle_0<RetType, T: QTableView_gridStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.gridStyle_0(self);
    // return 1;
  }
}
pub trait QTableView_gridStyle_0<RetType> {
  fn gridStyle_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_gridStyle_0<i32> for () {
  fn gridStyle_0(self , rsthis: & QTableView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTableView9gridStyleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:102
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setGridStyle(Qt::PenStyle)

/*

*/
impl /*struct*/ QTableView {
  pub fn setGridStyle_0<RetType, T: QTableView_setGridStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setGridStyle_0(self);
    // return 1;
  }
}
pub trait QTableView_setGridStyle_0<RetType> {
  fn setGridStyle_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_setGridStyle_0<(/*void*/)> for (i32) {
  fn setGridStyle_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTableView12setGridStyleEN2Qt8PenStyleE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:104
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWordWrap(bool)

/*

*/
impl /*struct*/ QTableView {
  pub fn setWordWrap_0<RetType, T: QTableView_setWordWrap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWordWrap_0(self);
    // return 1;
  }
}
pub trait QTableView_setWordWrap_0<RetType> {
  fn setWordWrap_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_setWordWrap_0<(/*void*/)> for (bool) {
  fn setWordWrap_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN10QTableView11setWordWrapEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:105
// index:0
// Public Visibility=Default Availability=Available
// [1] bool wordWrap() const

/*

*/
impl /*struct*/ QTableView {
  pub fn wordWrap_0<RetType, T: QTableView_wordWrap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.wordWrap_0(self);
    // return 1;
  }
}
pub trait QTableView_wordWrap_0<RetType> {
  fn wordWrap_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_wordWrap_0<bool> for () {
  fn wordWrap_0(self , rsthis: & QTableView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTableView8wordWrapEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:108
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCornerButtonEnabled(bool)

/*

*/
impl /*struct*/ QTableView {
  pub fn setCornerButtonEnabled_0<RetType, T: QTableView_setCornerButtonEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCornerButtonEnabled_0(self);
    // return 1;
  }
}
pub trait QTableView_setCornerButtonEnabled_0<RetType> {
  fn setCornerButtonEnabled_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_setCornerButtonEnabled_0<(/*void*/)> for (bool) {
  fn setCornerButtonEnabled_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN10QTableView22setCornerButtonEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:109
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isCornerButtonEnabled() const

/*

*/
impl /*struct*/ QTableView {
  pub fn isCornerButtonEnabled_0<RetType, T: QTableView_isCornerButtonEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isCornerButtonEnabled_0(self);
    // return 1;
  }
}
pub trait QTableView_isCornerButtonEnabled_0<RetType> {
  fn isCornerButtonEnabled_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_isCornerButtonEnabled_0<bool> for () {
  fn isCornerButtonEnabled_0(self , rsthis: & QTableView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTableView21isCornerButtonEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:112
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QRect visualRect(const QModelIndex &) const

/*

*/
impl /*struct*/ QTableView {
  pub fn visualRect_0<RetType, T: QTableView_visualRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.visualRect_0(self);
    // return 1;
  }
}
pub trait QTableView_visualRect_0<RetType> {
  fn visualRect_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_visualRect_0<usize> for (usize) {
  fn visualRect_0(self , rsthis: & QTableView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTableView10visualRectERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:113
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void scrollTo(const QModelIndex &, QAbstractItemView::ScrollHint)

/*

*/
impl /*struct*/ QTableView {
  pub fn scrollTo_0<RetType, T: QTableView_scrollTo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scrollTo_0(self);
    // return 1;
  }
}
pub trait QTableView_scrollTo_0<RetType> {
  fn scrollTo_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_scrollTo_0<(/*void*/)> for (usize,i32) {
  fn scrollTo_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTableView8scrollToERK11QModelIndexN17QAbstractItemView10ScrollHintE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:114
// index:0
// Public virtual Visibility=Default Availability=Available
// [24] QModelIndex indexAt(const QPoint &) const

/*
Reimplemented from QAbstractItemView::indexAt().

Returns the index position of the model item corresponding to the table item at position pos in contents coordinates.
*/
impl /*struct*/ QTableView {
  pub fn indexAt_0<RetType, T: QTableView_indexAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexAt_0(self);
    // return 1;
  }
}
pub trait QTableView_indexAt_0<RetType> {
  fn indexAt_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_indexAt_0<usize> for (usize) {
  fn indexAt_0(self , rsthis: & QTableView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTableView7indexAtERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:116
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSpan(int, int, int, int)

/*
Sets the span of the table element at (row, column) to the number of rows and columns specified by (rowSpanCount, columnSpanCount).

This function was introduced in  Qt 4.2.

See also rowSpan() and columnSpan().
*/
impl /*struct*/ QTableView {
  pub fn setSpan_0<RetType, T: QTableView_setSpan_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSpan_0(self);
    // return 1;
  }
}
pub trait QTableView_setSpan_0<RetType> {
  fn setSpan_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_setSpan_0<(/*void*/)> for (i32,i32,i32,i32) {
  fn setSpan_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTableView7setSpanEiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:117
// index:0
// Public Visibility=Default Availability=Available
// [4] int rowSpan(int, int) const

/*
Returns the row span of the table element at (row, column). The default is 1.

This function was introduced in  Qt 4.2.

See also setSpan() and columnSpan().
*/
impl /*struct*/ QTableView {
  pub fn rowSpan_0<RetType, T: QTableView_rowSpan_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowSpan_0(self);
    // return 1;
  }
}
pub trait QTableView_rowSpan_0<RetType> {
  fn rowSpan_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_rowSpan_0<i32> for (i32,i32) {
  fn rowSpan_0(self , rsthis: & QTableView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTableView7rowSpanEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:118
// index:0
// Public Visibility=Default Availability=Available
// [4] int columnSpan(int, int) const

/*
Returns the column span of the table element at (row, column). The default is 1.

This function was introduced in  Qt 4.2.

See also setSpan() and rowSpan().
*/
impl /*struct*/ QTableView {
  pub fn columnSpan_0<RetType, T: QTableView_columnSpan_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnSpan_0(self);
    // return 1;
  }
}
pub trait QTableView_columnSpan_0<RetType> {
  fn columnSpan_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_columnSpan_0<i32> for (i32,i32) {
  fn columnSpan_0(self , rsthis: & QTableView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTableView10columnSpanEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:119
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clearSpans()

/*
Removes all row and column spans in the table view.

This function was introduced in  Qt 4.4.

See also setSpan().
*/
impl /*struct*/ QTableView {
  pub fn clearSpans_0<RetType, T: QTableView_clearSpans_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clearSpans_0(self);
    // return 1;
  }
}
pub trait QTableView_clearSpans_0<RetType> {
  fn clearSpans_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_clearSpans_0<(/*void*/)> for () {
  fn clearSpans_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QTableView10clearSpansEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:121
// index:0
// Public Visibility=Default Availability=Available
// [-2] void sortByColumn(int, Qt::SortOrder)

/*
Sorts the model by the values in the given column in the given order.

This function was introduced in  Qt 4.2.

See also sortingEnabled.
*/
impl /*struct*/ QTableView {
  pub fn sortByColumn_0<RetType, T: QTableView_sortByColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sortByColumn_0(self);
    // return 1;
  }
}
pub trait QTableView_sortByColumn_0<RetType> {
  fn sortByColumn_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_sortByColumn_0<(/*void*/)> for (i32,i32) {
  fn sortByColumn_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTableView12sortByColumnEiN2Qt9SortOrderE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:134
// index:1
// Public Visibility=Default Availability=Available
// [-2] void sortByColumn(int)

/*
Sorts the model by the values in the given column in the given order.

This function was introduced in  Qt 4.2.

See also sortingEnabled.
*/
impl /*struct*/ QTableView {
  pub fn sortByColumn_1<RetType, T: QTableView_sortByColumn_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sortByColumn_1(self);
    // return 1;
  }
}
pub trait QTableView_sortByColumn_1<RetType> {
  fn sortByColumn_1(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_sortByColumn_1<(/*void*/)> for (i32) {
  fn sortByColumn_1(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTableView12sortByColumnEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:124
// index:0
// Public Visibility=Default Availability=Available
// [-2] void selectRow(int)

/*
Selects the given row in the table view if the current SelectionMode and SelectionBehavior allows rows to be selected.

See also selectColumn().
*/
impl /*struct*/ QTableView {
  pub fn selectRow_0<RetType, T: QTableView_selectRow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectRow_0(self);
    // return 1;
  }
}
pub trait QTableView_selectRow_0<RetType> {
  fn selectRow_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_selectRow_0<(/*void*/)> for (i32) {
  fn selectRow_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTableView9selectRowEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:125
// index:0
// Public Visibility=Default Availability=Available
// [-2] void selectColumn(int)

/*
Selects the given column in the table view if the current SelectionMode and SelectionBehavior allows columns to be selected.

See also selectRow().
*/
impl /*struct*/ QTableView {
  pub fn selectColumn_0<RetType, T: QTableView_selectColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectColumn_0(self);
    // return 1;
  }
}
pub trait QTableView_selectColumn_0<RetType> {
  fn selectColumn_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_selectColumn_0<(/*void*/)> for (i32) {
  fn selectColumn_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTableView12selectColumnEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:126
// index:0
// Public Visibility=Default Availability=Available
// [-2] void hideRow(int)

/*
Hide the given row.

See also showRow() and hideColumn().
*/
impl /*struct*/ QTableView {
  pub fn hideRow_0<RetType, T: QTableView_hideRow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hideRow_0(self);
    // return 1;
  }
}
pub trait QTableView_hideRow_0<RetType> {
  fn hideRow_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_hideRow_0<(/*void*/)> for (i32) {
  fn hideRow_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTableView7hideRowEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:127
// index:0
// Public Visibility=Default Availability=Available
// [-2] void hideColumn(int)

/*
Hide the given column.

See also showColumn() and hideRow().
*/
impl /*struct*/ QTableView {
  pub fn hideColumn_0<RetType, T: QTableView_hideColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hideColumn_0(self);
    // return 1;
  }
}
pub trait QTableView_hideColumn_0<RetType> {
  fn hideColumn_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_hideColumn_0<(/*void*/)> for (i32) {
  fn hideColumn_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTableView10hideColumnEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:128
// index:0
// Public Visibility=Default Availability=Available
// [-2] void showRow(int)

/*
Show the given row.

See also hideRow() and showColumn().
*/
impl /*struct*/ QTableView {
  pub fn showRow_0<RetType, T: QTableView_showRow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showRow_0(self);
    // return 1;
  }
}
pub trait QTableView_showRow_0<RetType> {
  fn showRow_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_showRow_0<(/*void*/)> for (i32) {
  fn showRow_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTableView7showRowEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:129
// index:0
// Public Visibility=Default Availability=Available
// [-2] void showColumn(int)

/*
Show the given column.

See also hideColumn() and showRow().
*/
impl /*struct*/ QTableView {
  pub fn showColumn_0<RetType, T: QTableView_showColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showColumn_0(self);
    // return 1;
  }
}
pub trait QTableView_showColumn_0<RetType> {
  fn showColumn_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_showColumn_0<(/*void*/)> for (i32) {
  fn showColumn_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTableView10showColumnEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:130
// index:0
// Public Visibility=Default Availability=Available
// [-2] void resizeRowToContents(int)

/*
Resizes the given row based on the size hints of the delegate used to render each item in the row.

See also resizeRowsToContents(), sizeHintForRow(), and QHeaderView::resizeContentsPrecision().
*/
impl /*struct*/ QTableView {
  pub fn resizeRowToContents_0<RetType, T: QTableView_resizeRowToContents_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeRowToContents_0(self);
    // return 1;
  }
}
pub trait QTableView_resizeRowToContents_0<RetType> {
  fn resizeRowToContents_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_resizeRowToContents_0<(/*void*/)> for (i32) {
  fn resizeRowToContents_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTableView19resizeRowToContentsEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:131
// index:0
// Public Visibility=Default Availability=Available
// [-2] void resizeRowsToContents()

/*
Resizes all rows based on the size hints of the delegate used to render each item in the rows.

See also resizeRowToContents(), sizeHintForRow(), and QHeaderView::resizeContentsPrecision().
*/
impl /*struct*/ QTableView {
  pub fn resizeRowsToContents_0<RetType, T: QTableView_resizeRowsToContents_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeRowsToContents_0(self);
    // return 1;
  }
}
pub trait QTableView_resizeRowsToContents_0<RetType> {
  fn resizeRowsToContents_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_resizeRowsToContents_0<(/*void*/)> for () {
  fn resizeRowsToContents_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QTableView20resizeRowsToContentsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:132
// index:0
// Public Visibility=Default Availability=Available
// [-2] void resizeColumnToContents(int)

/*
Resizes the given column based on the size hints of the delegate used to render each item in the column.

Note: Only visible columns will be resized. Reimplement sizeHintForColumn() to resize hidden columns as well.

See also resizeColumnsToContents(), sizeHintForColumn(), and QHeaderView::resizeContentsPrecision().
*/
impl /*struct*/ QTableView {
  pub fn resizeColumnToContents_0<RetType, T: QTableView_resizeColumnToContents_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeColumnToContents_0(self);
    // return 1;
  }
}
pub trait QTableView_resizeColumnToContents_0<RetType> {
  fn resizeColumnToContents_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_resizeColumnToContents_0<(/*void*/)> for (i32) {
  fn resizeColumnToContents_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTableView22resizeColumnToContentsEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:133
// index:0
// Public Visibility=Default Availability=Available
// [-2] void resizeColumnsToContents()

/*
Resizes all columns based on the size hints of the delegate used to render each item in the columns.

See also resizeColumnToContents(), sizeHintForColumn(), and QHeaderView::resizeContentsPrecision().
*/
impl /*struct*/ QTableView {
  pub fn resizeColumnsToContents_0<RetType, T: QTableView_resizeColumnsToContents_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeColumnsToContents_0(self);
    // return 1;
  }
}
pub trait QTableView_resizeColumnsToContents_0<RetType> {
  fn resizeColumnsToContents_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_resizeColumnsToContents_0<(/*void*/)> for () {
  fn resizeColumnsToContents_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QTableView23resizeColumnsToContentsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:135
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setShowGrid(bool)

/*

*/
impl /*struct*/ QTableView {
  pub fn setShowGrid_0<RetType, T: QTableView_setShowGrid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setShowGrid_0(self);
    // return 1;
  }
}
pub trait QTableView_setShowGrid_0<RetType> {
  fn setShowGrid_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_setShowGrid_0<(/*void*/)> for (bool) {
  fn setShowGrid_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN10QTableView11setShowGridEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:138
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void rowMoved(int, int, int)

/*
This slot is called to change the index of the given row in the table view. The old index is specified by oldIndex, and the new index by newIndex.

See also columnMoved().
*/
impl /*struct*/ QTableView {
  pub fn rowMoved_0<RetType, T: QTableView_rowMoved_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowMoved_0(self);
    // return 1;
  }
}
pub trait QTableView_rowMoved_0<RetType> {
  fn rowMoved_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_rowMoved_0<(/*void*/)> for (i32,i32,i32) {
  fn rowMoved_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTableView8rowMovedEiii", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:139
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void columnMoved(int, int, int)

/*
This slot is called to change the index of the given column in the table view. The old index is specified by oldIndex, and the new index by newIndex.

See also rowMoved().
*/
impl /*struct*/ QTableView {
  pub fn columnMoved_0<RetType, T: QTableView_columnMoved_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnMoved_0(self);
    // return 1;
  }
}
pub trait QTableView_columnMoved_0<RetType> {
  fn columnMoved_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_columnMoved_0<(/*void*/)> for (i32,i32,i32) {
  fn columnMoved_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTableView11columnMovedEiii", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:140
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void rowResized(int, int, int)

/*
This slot is called to change the height of the given row. The old height is specified by oldHeight, and the new height by newHeight.

See also columnResized().
*/
impl /*struct*/ QTableView {
  pub fn rowResized_0<RetType, T: QTableView_rowResized_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowResized_0(self);
    // return 1;
  }
}
pub trait QTableView_rowResized_0<RetType> {
  fn rowResized_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_rowResized_0<(/*void*/)> for (i32,i32,i32) {
  fn rowResized_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTableView10rowResizedEiii", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:141
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void columnResized(int, int, int)

/*
This slot is called to change the width of the given column. The old width is specified by oldWidth, and the new width by newWidth.

See also rowResized().
*/
impl /*struct*/ QTableView {
  pub fn columnResized_0<RetType, T: QTableView_columnResized_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnResized_0(self);
    // return 1;
  }
}
pub trait QTableView_columnResized_0<RetType> {
  fn columnResized_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_columnResized_0<(/*void*/)> for (i32,i32,i32) {
  fn columnResized_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTableView13columnResizedEiii", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:142
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void rowCountChanged(int, int)

/*
This slot is called whenever rows are added or deleted. The previous number of rows is specified by oldCount, and the new number of rows is specified by newCount.
*/
impl /*struct*/ QTableView {
  pub fn rowCountChanged_0<RetType, T: QTableView_rowCountChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowCountChanged_0(self);
    // return 1;
  }
}
pub trait QTableView_rowCountChanged_0<RetType> {
  fn rowCountChanged_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_rowCountChanged_0<(/*void*/)> for (i32,i32) {
  fn rowCountChanged_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTableView15rowCountChangedEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:143
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void columnCountChanged(int, int)

/*
This slot is called whenever columns are added or deleted. The previous number of columns is specified by oldCount, and the new number of columns is specified by newCount.
*/
impl /*struct*/ QTableView {
  pub fn columnCountChanged_0<RetType, T: QTableView_columnCountChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnCountChanged_0(self);
    // return 1;
  }
}
pub trait QTableView_columnCountChanged_0<RetType> {
  fn columnCountChanged_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_columnCountChanged_0<(/*void*/)> for (i32,i32) {
  fn columnCountChanged_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTableView18columnCountChangedEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:147
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void scrollContentsBy(int, int)

/*

*/
impl /*struct*/ QTableView {
  pub fn scrollContentsBy_0<RetType, T: QTableView_scrollContentsBy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scrollContentsBy_0(self);
    // return 1;
  }
}
pub trait QTableView_scrollContentsBy_0<RetType> {
  fn scrollContentsBy_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_scrollContentsBy_0<(/*void*/)> for (i32,i32) {
  fn scrollContentsBy_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTableView16scrollContentsByEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:149
// index:0
// Protected virtual Visibility=Default Availability=Available
// [192] QStyleOptionViewItem viewOptions() const

/*
Reimplemented from QAbstractItemView::viewOptions().
*/
impl /*struct*/ QTableView {
  pub fn viewOptions_0<RetType, T: QTableView_viewOptions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.viewOptions_0(self);
    // return 1;
  }
}
pub trait QTableView_viewOptions_0<RetType> {
  fn viewOptions_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_viewOptions_0<usize> for () {
  fn viewOptions_0(self , rsthis: & QTableView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTableView11viewOptionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:150
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Reimplemented from QWidget::paintEvent().

Paints the table on receipt of the given paint event event.
*/
impl /*struct*/ QTableView {
  pub fn paintEvent_0<RetType, T: QTableView_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QTableView_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QTableView10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:152
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void timerEvent(QTimerEvent *)

/*
Reimplemented from QObject::timerEvent().
*/
impl /*struct*/ QTableView {
  pub fn timerEvent_0<RetType, T: QTableView_timerEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.timerEvent_0(self);
    // return 1;
  }
}
pub trait QTableView_timerEvent_0<RetType> {
  fn timerEvent_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_timerEvent_0<(/*void*/)> for (usize) {
  fn timerEvent_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QTableView10timerEventEP11QTimerEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:154
// index:0
// Protected virtual Visibility=Default Availability=Available
// [4] int horizontalOffset() const

/*
Reimplemented from QAbstractItemView::horizontalOffset().

Returns the horizontal offset of the items in the table view.

Note that the table view uses the horizontal header section positions to determine the positions of columns in the view.

See also verticalOffset().
*/
impl /*struct*/ QTableView {
  pub fn horizontalOffset_0<RetType, T: QTableView_horizontalOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.horizontalOffset_0(self);
    // return 1;
  }
}
pub trait QTableView_horizontalOffset_0<RetType> {
  fn horizontalOffset_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_horizontalOffset_0<i32> for () {
  fn horizontalOffset_0(self , rsthis: & QTableView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTableView16horizontalOffsetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:155
// index:0
// Protected virtual Visibility=Default Availability=Available
// [4] int verticalOffset() const

/*
Reimplemented from QAbstractItemView::verticalOffset().

Returns the vertical offset of the items in the table view.

Note that the table view uses the vertical header section positions to determine the positions of rows in the view.

See also horizontalOffset().
*/
impl /*struct*/ QTableView {
  pub fn verticalOffset_0<RetType, T: QTableView_verticalOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.verticalOffset_0(self);
    // return 1;
  }
}
pub trait QTableView_verticalOffset_0<RetType> {
  fn verticalOffset_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_verticalOffset_0<i32> for () {
  fn verticalOffset_0(self , rsthis: & QTableView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTableView14verticalOffsetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:156
// index:0
// Protected virtual Visibility=Default Availability=Available
// [24] QModelIndex moveCursor(QAbstractItemView::CursorAction, Qt::KeyboardModifiers)

/*
Reimplemented from QAbstractItemView::moveCursor().

Moves the cursor in accordance with the given cursorAction, using the information provided by the modifiers.

See also QAbstractItemView::CursorAction.
*/
impl /*struct*/ QTableView {
  pub fn moveCursor_0<RetType, T: QTableView_moveCursor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveCursor_0(self);
    // return 1;
  }
}
pub trait QTableView_moveCursor_0<RetType> {
  fn moveCursor_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_moveCursor_0<usize> for (i32,i32) {
  fn moveCursor_0(self , rsthis: & QTableView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QTableView10moveCursorEN17QAbstractItemView12CursorActionE6QFlagsIN2Qt16KeyboardModifierEE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:158
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void setSelection(const QRect &, QItemSelectionModel::SelectionFlags)

/*
Reimplemented from QAbstractItemView::setSelection().

Selects the items within the given rect and in accordance with the specified selection flags.
*/
impl /*struct*/ QTableView {
  pub fn setSelection_0<RetType, T: QTableView_setSelection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSelection_0(self);
    // return 1;
  }
}
pub trait QTableView_setSelection_0<RetType> {
  fn setSelection_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_setSelection_0<(/*void*/)> for (usize,i32) {
  fn setSelection_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTableView12setSelectionERK5QRect6QFlagsIN19QItemSelectionModel13SelectionFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:159
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] QRegion visualRegionForSelection(const QItemSelection &) const

/*

*/
impl /*struct*/ QTableView {
  pub fn visualRegionForSelection_0<RetType, T: QTableView_visualRegionForSelection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.visualRegionForSelection_0(self);
    // return 1;
  }
}
pub trait QTableView_visualRegionForSelection_0<RetType> {
  fn visualRegionForSelection_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_visualRegionForSelection_0<usize> for (usize) {
  fn visualRegionForSelection_0(self , rsthis: & QTableView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTableView24visualRegionForSelectionERK14QItemSelection", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:160
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] QModelIndexList selectedIndexes() const

/*
Reimplemented from QAbstractItemView::selectedIndexes().
*/
impl /*struct*/ QTableView {
  pub fn selectedIndexes_0<RetType, T: QTableView_selectedIndexes_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectedIndexes_0(self);
    // return 1;
  }
}
pub trait QTableView_selectedIndexes_0<RetType> {
  fn selectedIndexes_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_selectedIndexes_0<usize> for () {
  fn selectedIndexes_0(self , rsthis: & QTableView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTableView15selectedIndexesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:162
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void updateGeometries()

/*
Reimplemented from QAbstractItemView::updateGeometries().
*/
impl /*struct*/ QTableView {
  pub fn updateGeometries_0<RetType, T: QTableView_updateGeometries_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updateGeometries_0(self);
    // return 1;
  }
}
pub trait QTableView_updateGeometries_0<RetType> {
  fn updateGeometries_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_updateGeometries_0<(/*void*/)> for () {
  fn updateGeometries_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QTableView16updateGeometriesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:164
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] QSize viewportSizeHint() const

/*
Reimplemented from QAbstractScrollArea::viewportSizeHint().
*/
impl /*struct*/ QTableView {
  pub fn viewportSizeHint_0<RetType, T: QTableView_viewportSizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.viewportSizeHint_0(self);
    // return 1;
  }
}
pub trait QTableView_viewportSizeHint_0<RetType> {
  fn viewportSizeHint_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_viewportSizeHint_0<usize> for () {
  fn viewportSizeHint_0(self , rsthis: & QTableView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTableView16viewportSizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:166
// index:0
// Protected virtual Visibility=Default Availability=Available
// [4] int sizeHintForRow(int) const

/*
Reimplemented from QAbstractItemView::sizeHintForRow().

Returns the size hint for the given row's height or -1 if there is no model.

If you need to set the height of a given row to a fixed value, call QHeaderView::resizeSection() on the table's vertical header.

If you reimplement this function in a subclass, note that the value you return is only used when resizeRowToContents() is called. In that case, if a larger row height is required by either the vertical header or the item delegate, that width will be used instead.

See also QWidget::sizeHint, verticalHeader(), and QHeaderView::resizeContentsPrecision().
*/
impl /*struct*/ QTableView {
  pub fn sizeHintForRow_0<RetType, T: QTableView_sizeHintForRow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHintForRow_0(self);
    // return 1;
  }
}
pub trait QTableView_sizeHintForRow_0<RetType> {
  fn sizeHintForRow_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_sizeHintForRow_0<i32> for (i32) {
  fn sizeHintForRow_0(self , rsthis: & QTableView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTableView14sizeHintForRowEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:167
// index:0
// Protected virtual Visibility=Default Availability=Available
// [4] int sizeHintForColumn(int) const

/*
Reimplemented from QAbstractItemView::sizeHintForColumn().

Returns the size hint for the given column's width or -1 if there is no model.

If you need to set the width of a given column to a fixed value, call QHeaderView::resizeSection() on the table's horizontal header.

If you reimplement this function in a subclass, note that the value you return will be used when resizeColumnToContents() or QHeaderView::resizeSections() is called. If a larger column width is required by either the horizontal header or the item delegate, the larger width will be used instead.

See also QWidget::sizeHint, horizontalHeader(), and QHeaderView::resizeContentsPrecision().
*/
impl /*struct*/ QTableView {
  pub fn sizeHintForColumn_0<RetType, T: QTableView_sizeHintForColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHintForColumn_0(self);
    // return 1;
  }
}
pub trait QTableView_sizeHintForColumn_0<RetType> {
  fn sizeHintForColumn_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_sizeHintForColumn_0<i32> for (i32) {
  fn sizeHintForColumn_0(self , rsthis: & QTableView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTableView17sizeHintForColumnEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:169
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void verticalScrollbarAction(int)

/*

*/
impl /*struct*/ QTableView {
  pub fn verticalScrollbarAction_0<RetType, T: QTableView_verticalScrollbarAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.verticalScrollbarAction_0(self);
    // return 1;
  }
}
pub trait QTableView_verticalScrollbarAction_0<RetType> {
  fn verticalScrollbarAction_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_verticalScrollbarAction_0<(/*void*/)> for (i32) {
  fn verticalScrollbarAction_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTableView23verticalScrollbarActionEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:170
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void horizontalScrollbarAction(int)

/*

*/
impl /*struct*/ QTableView {
  pub fn horizontalScrollbarAction_0<RetType, T: QTableView_horizontalScrollbarAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.horizontalScrollbarAction_0(self);
    // return 1;
  }
}
pub trait QTableView_horizontalScrollbarAction_0<RetType> {
  fn horizontalScrollbarAction_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_horizontalScrollbarAction_0<(/*void*/)> for (i32) {
  fn horizontalScrollbarAction_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTableView25horizontalScrollbarActionEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:172
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool isIndexHidden(const QModelIndex &) const

/*
Reimplemented from QAbstractItemView::isIndexHidden().
*/
impl /*struct*/ QTableView {
  pub fn isIndexHidden_0<RetType, T: QTableView_isIndexHidden_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isIndexHidden_0(self);
    // return 1;
  }
}
pub trait QTableView_isIndexHidden_0<RetType> {
  fn isIndexHidden_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_isIndexHidden_0<bool> for (usize) {
  fn isIndexHidden_0(self , rsthis: & QTableView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTableView13isIndexHiddenERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:174
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void selectionChanged(const QItemSelection &, const QItemSelection &)

/*
Reimplemented from QAbstractItemView::selectionChanged().
*/
impl /*struct*/ QTableView {
  pub fn selectionChanged_0<RetType, T: QTableView_selectionChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectionChanged_0(self);
    // return 1;
  }
}
pub trait QTableView_selectionChanged_0<RetType> {
  fn selectionChanged_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_selectionChanged_0<(/*void*/)> for (usize,usize) {
  fn selectionChanged_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QTableView16selectionChangedERK14QItemSelectionS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtableview.h:176
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void currentChanged(const QModelIndex &, const QModelIndex &)

/*
Reimplemented from QAbstractItemView::currentChanged().
*/
impl /*struct*/ QTableView {
  pub fn currentChanged_0<RetType, T: QTableView_currentChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentChanged_0(self);
    // return 1;
  }
}
pub trait QTableView_currentChanged_0<RetType> {
  fn currentChanged_0(self , rsthis: & QTableView) -> RetType;
}
impl<'a> /*trait*/ QTableView_currentChanged_0<(/*void*/)> for (usize,usize) {
  fn currentChanged_0(self , rsthis: & QTableView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QTableView14currentChangedERK11QModelIndexS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
