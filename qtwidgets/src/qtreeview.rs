

// mod ::widgets::QTreeView
// package qtwidgets
// /usr/include/qt/QtWidgets/qtreeview.h
// #include <qtreeview.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 10
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

// void columnResized(int, int, int)
// func (this *QTreeView) InheritColumnResized(f func(column int, oldSize int, newSize int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "columnResized", f)
// }

// void columnCountChanged(int, int)
// func (this *QTreeView) InheritColumnCountChanged(f func(oldCount int, newCount int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "columnCountChanged", f)
// }

// void columnMoved()
// func (this *QTreeView) InheritColumnMoved(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "columnMoved", f)
// }

// void reexpand()
// func (this *QTreeView) InheritReexpand(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "reexpand", f)
// }

// void rowsRemoved(const QModelIndex &, int, int)
// func (this *QTreeView) InheritRowsRemoved(f func(parent *qtcore.QModelIndex, first int, last int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "rowsRemoved", f)
// }

// void scrollContentsBy(int, int)
// func (this *QTreeView) InheritScrollContentsBy(f func(dx int, dy int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "scrollContentsBy", f)
// }

// void rowsInserted(const QModelIndex &, int, int)
// func (this *QTreeView) InheritRowsInserted(f func(parent *qtcore.QModelIndex, start int, end_ int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "rowsInserted", f)
// }

// void rowsAboutToBeRemoved(const QModelIndex &, int, int)
// func (this *QTreeView) InheritRowsAboutToBeRemoved(f func(parent *qtcore.QModelIndex, start int, end_ int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "rowsAboutToBeRemoved", f)
// }

// QModelIndex moveCursor(QAbstractItemView::CursorAction, Qt::KeyboardModifiers)
// func (this *QTreeView) InheritMoveCursor(f func(cursorAction int, modifiers int) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "moveCursor", f)
// }

// int horizontalOffset()
// func (this *QTreeView) InheritHorizontalOffset(f func() int) {
//  qtrt.SetAllInheritCallback(this, "horizontalOffset", f)
// }

// int verticalOffset()
// func (this *QTreeView) InheritVerticalOffset(f func() int) {
//  qtrt.SetAllInheritCallback(this, "verticalOffset", f)
// }

// void setSelection(const QRect &, QItemSelectionModel::SelectionFlags)
// func (this *QTreeView) InheritSetSelection(f func(rect *qtcore.QRect, command int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "setSelection", f)
// }

// QRegion visualRegionForSelection(const QItemSelection &)
// func (this *QTreeView) InheritVisualRegionForSelection(f func(selection *qtcore.QItemSelection) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "visualRegionForSelection", f)
// }

// QModelIndexList selectedIndexes()
// func (this *QTreeView) InheritSelectedIndexes(f func() *qtcore.QModelIndexList/*9999*/) {
//  qtrt.SetAllInheritCallback(this, "selectedIndexes", f)
// }

// void timerEvent(QTimerEvent *)
// func (this *QTreeView) InheritTimerEvent(f func(event *qtcore.QTimerEvent/*777 QTimerEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "timerEvent", f)
// }

// void paintEvent(QPaintEvent *)
// func (this *QTreeView) InheritPaintEvent(f func(event *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }

// void drawTree(QPainter *, const QRegion &)
// func (this *QTreeView) InheritDrawTree(f func(painter *qtgui.QPainter/*777 QPainter **/, region *qtgui.QRegion) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "drawTree", f)
// }

// void drawRow(QPainter *, const QStyleOptionViewItem &, const QModelIndex &)
// func (this *QTreeView) InheritDrawRow(f func(painter *qtgui.QPainter/*777 QPainter **/, options *QStyleOptionViewItem, index *qtcore.QModelIndex) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "drawRow", f)
// }

// void drawBranches(QPainter *, const QRect &, const QModelIndex &)
// func (this *QTreeView) InheritDrawBranches(f func(painter *qtgui.QPainter/*777 QPainter **/, rect *qtcore.QRect, index *qtcore.QModelIndex) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "drawBranches", f)
// }

// void mousePressEvent(QMouseEvent *)
// func (this *QTreeView) InheritMousePressEvent(f func(event *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mousePressEvent", f)
// }

// void mouseReleaseEvent(QMouseEvent *)
// func (this *QTreeView) InheritMouseReleaseEvent(f func(event *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseReleaseEvent", f)
// }

// void mouseDoubleClickEvent(QMouseEvent *)
// func (this *QTreeView) InheritMouseDoubleClickEvent(f func(event *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseDoubleClickEvent", f)
// }

// void mouseMoveEvent(QMouseEvent *)
// func (this *QTreeView) InheritMouseMoveEvent(f func(event *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseMoveEvent", f)
// }

// void keyPressEvent(QKeyEvent *)
// func (this *QTreeView) InheritKeyPressEvent(f func(event *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyPressEvent", f)
// }

// void dragMoveEvent(QDragMoveEvent *)
// func (this *QTreeView) InheritDragMoveEvent(f func(event *qtgui.QDragMoveEvent/*777 QDragMoveEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dragMoveEvent", f)
// }

// bool viewportEvent(QEvent *)
// func (this *QTreeView) InheritViewportEvent(f func(event *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "viewportEvent", f)
// }

// void updateGeometries()
// func (this *QTreeView) InheritUpdateGeometries(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "updateGeometries", f)
// }

// QSize viewportSizeHint()
// func (this *QTreeView) InheritViewportSizeHint(f func() unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "viewportSizeHint", f)
// }

// int sizeHintForColumn(int)
// func (this *QTreeView) InheritSizeHintForColumn(f func(column int) int) {
//  qtrt.SetAllInheritCallback(this, "sizeHintForColumn", f)
// }

// int indexRowSizeHint(const QModelIndex &)
// func (this *QTreeView) InheritIndexRowSizeHint(f func(index *qtcore.QModelIndex) int) {
//  qtrt.SetAllInheritCallback(this, "indexRowSizeHint", f)
// }

// int rowHeight(const QModelIndex &)
// func (this *QTreeView) InheritRowHeight(f func(index *qtcore.QModelIndex) int) {
//  qtrt.SetAllInheritCallback(this, "rowHeight", f)
// }

// void horizontalScrollbarAction(int)
// func (this *QTreeView) InheritHorizontalScrollbarAction(f func(action int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "horizontalScrollbarAction", f)
// }

// bool isIndexHidden(const QModelIndex &)
// func (this *QTreeView) InheritIsIndexHidden(f func(index *qtcore.QModelIndex) bool) {
//  qtrt.SetAllInheritCallback(this, "isIndexHidden", f)
// }

// void selectionChanged(const QItemSelection &, const QItemSelection &)
// func (this *QTreeView) InheritSelectionChanged(f func(selected *qtcore.QItemSelection, deselected *qtcore.QItemSelection) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "selectionChanged", f)
// }

// void currentChanged(const QModelIndex &, const QModelIndex &)
// func (this *QTreeView) InheritCurrentChanged(f func(current *qtcore.QModelIndex, previous *qtcore.QModelIndex) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "currentChanged", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QTreeView)=48
pub struct QTreeView {
  qbase: QAbstractItemView,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTreeView_ITF interface {
//    QAbstractItemView_ITF
//    QTreeView_PTR() *QTreeView
//}
//func (ptr *QTreeView) QTreeView_PTR() *QTreeView { return ptr }

impl /*struct*/ QTreeView {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTreeView {
    return QTreeView{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTreeView {
//  type Target = QTreeViewBASE;
//
//  fn deref(&self) -> &QTreeViewBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTreeViewBASE> for QTreeView {
//  fn as_ref(& self) -> & QTreeViewBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qtreeview.h:57
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QTreeView {
  pub fn metaObject_0<RetType, T: QTreeView_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QTreeView_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QTreeView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTreeView10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:71
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTreeView(QWidget *)

/*
Constructs a tree view with a parent to represent a model's data. Use setModel() to set the model.

See also QAbstractItemModel.
*/
// QTreeView(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QTreeView {
  pub fn QTreeView_0<T: QTreeView_QTreeView_0>(value: T) -> QTreeView {
    let rsthis = value.QTreeView_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTreeView_QTreeView_0 {
  fn QTreeView_0(self) -> QTreeView;
}
// QTreeView(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTreeView_QTreeView_0 for (usize) {
  fn QTreeView_0(self) -> QTreeView {
    // unsafe{_ZN9QTreeViewC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QTreeViewC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTreeView{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:72
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QTreeView()

/*

*/
pub fn DeleteQTreeView(this :*mut QTreeView) {
    // let rv = qtrt::InvokeQtFunc6("_ZN9QTreeViewD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qtreeview.h:74
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setModel(QAbstractItemModel *)

/*
Reimplemented from QAbstractItemView::setModel().
*/
impl /*struct*/ QTreeView {
  pub fn setModel_0<RetType, T: QTreeView_setModel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setModel_0(self);
    // return 1;
  }
}
pub trait QTreeView_setModel_0<RetType> {
  fn setModel_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_setModel_0<(/*void*/)> for (usize) {
  fn setModel_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView8setModelEP18QAbstractItemModel", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:75
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setRootIndex(const QModelIndex &)

/*
Reimplemented from QAbstractItemView::setRootIndex().
*/
impl /*struct*/ QTreeView {
  pub fn setRootIndex_0<RetType, T: QTreeView_setRootIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRootIndex_0(self);
    // return 1;
  }
}
pub trait QTreeView_setRootIndex_0<RetType> {
  fn setRootIndex_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_setRootIndex_0<(/*void*/)> for (usize) {
  fn setRootIndex_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView12setRootIndexERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:76
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setSelectionModel(QItemSelectionModel *)

/*
Reimplemented from QAbstractItemView::setSelectionModel().
*/
impl /*struct*/ QTreeView {
  pub fn setSelectionModel_0<RetType, T: QTreeView_setSelectionModel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSelectionModel_0(self);
    // return 1;
  }
}
pub trait QTreeView_setSelectionModel_0<RetType> {
  fn setSelectionModel_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_setSelectionModel_0<(/*void*/)> for (usize) {
  fn setSelectionModel_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView17setSelectionModelEP19QItemSelectionModel", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:78
// index:0
// Public Visibility=Default Availability=Available
// [8] QHeaderView * header() const

/*
Returns the header for the tree view.

See also setHeader() and QAbstractItemModel::headerData().
*/
impl /*struct*/ QTreeView {
  pub fn header_0<RetType, T: QTreeView_header_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.header_0(self);
    // return 1;
  }
}
pub trait QTreeView_header_0<RetType> {
  fn header_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_header_0<usize> for () {
  fn header_0(self , rsthis: & QTreeView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTreeView6headerEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:79
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setHeader(QHeaderView *)

/*
Sets the header for the tree view, to the given header.

The view takes ownership over the given header and deletes it when a new header is set.

See also QAbstractItemModel::headerData().
*/
impl /*struct*/ QTreeView {
  pub fn setHeader_0<RetType, T: QTreeView_setHeader_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHeader_0(self);
    // return 1;
  }
}
pub trait QTreeView_setHeader_0<RetType> {
  fn setHeader_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_setHeader_0<(/*void*/)> for (usize) {
  fn setHeader_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView9setHeaderEP11QHeaderView", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:81
// index:0
// Public Visibility=Default Availability=Available
// [4] int autoExpandDelay() const

/*

*/
impl /*struct*/ QTreeView {
  pub fn autoExpandDelay_0<RetType, T: QTreeView_autoExpandDelay_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.autoExpandDelay_0(self);
    // return 1;
  }
}
pub trait QTreeView_autoExpandDelay_0<RetType> {
  fn autoExpandDelay_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_autoExpandDelay_0<i32> for () {
  fn autoExpandDelay_0(self , rsthis: & QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTreeView15autoExpandDelayEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:82
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAutoExpandDelay(int)

/*

*/
impl /*struct*/ QTreeView {
  pub fn setAutoExpandDelay_0<RetType, T: QTreeView_setAutoExpandDelay_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAutoExpandDelay_0(self);
    // return 1;
  }
}
pub trait QTreeView_setAutoExpandDelay_0<RetType> {
  fn setAutoExpandDelay_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_setAutoExpandDelay_0<(/*void*/)> for (i32) {
  fn setAutoExpandDelay_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView18setAutoExpandDelayEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:84
// index:0
// Public Visibility=Default Availability=Available
// [4] int indentation() const

/*

*/
impl /*struct*/ QTreeView {
  pub fn indentation_0<RetType, T: QTreeView_indentation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indentation_0(self);
    // return 1;
  }
}
pub trait QTreeView_indentation_0<RetType> {
  fn indentation_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_indentation_0<i32> for () {
  fn indentation_0(self , rsthis: & QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTreeView11indentationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:85
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setIndentation(int)

/*

*/
impl /*struct*/ QTreeView {
  pub fn setIndentation_0<RetType, T: QTreeView_setIndentation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIndentation_0(self);
    // return 1;
  }
}
pub trait QTreeView_setIndentation_0<RetType> {
  fn setIndentation_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_setIndentation_0<(/*void*/)> for (i32) {
  fn setIndentation_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView14setIndentationEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:86
// index:0
// Public Visibility=Default Availability=Available
// [-2] void resetIndentation()

/*

*/
impl /*struct*/ QTreeView {
  pub fn resetIndentation_0<RetType, T: QTreeView_resetIndentation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resetIndentation_0(self);
    // return 1;
  }
}
pub trait QTreeView_resetIndentation_0<RetType> {
  fn resetIndentation_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_resetIndentation_0<(/*void*/)> for () {
  fn resetIndentation_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QTreeView16resetIndentationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:88
// index:0
// Public Visibility=Default Availability=Available
// [1] bool rootIsDecorated() const

/*

*/
impl /*struct*/ QTreeView {
  pub fn rootIsDecorated_0<RetType, T: QTreeView_rootIsDecorated_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rootIsDecorated_0(self);
    // return 1;
  }
}
pub trait QTreeView_rootIsDecorated_0<RetType> {
  fn rootIsDecorated_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_rootIsDecorated_0<bool> for () {
  fn rootIsDecorated_0(self , rsthis: & QTreeView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTreeView15rootIsDecoratedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:89
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRootIsDecorated(bool)

/*

*/
impl /*struct*/ QTreeView {
  pub fn setRootIsDecorated_0<RetType, T: QTreeView_setRootIsDecorated_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRootIsDecorated_0(self);
    // return 1;
  }
}
pub trait QTreeView_setRootIsDecorated_0<RetType> {
  fn setRootIsDecorated_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_setRootIsDecorated_0<(/*void*/)> for (bool) {
  fn setRootIsDecorated_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView18setRootIsDecoratedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:91
// index:0
// Public Visibility=Default Availability=Available
// [1] bool uniformRowHeights() const

/*

*/
impl /*struct*/ QTreeView {
  pub fn uniformRowHeights_0<RetType, T: QTreeView_uniformRowHeights_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.uniformRowHeights_0(self);
    // return 1;
  }
}
pub trait QTreeView_uniformRowHeights_0<RetType> {
  fn uniformRowHeights_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_uniformRowHeights_0<bool> for () {
  fn uniformRowHeights_0(self , rsthis: & QTreeView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTreeView17uniformRowHeightsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:92
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setUniformRowHeights(bool)

/*

*/
impl /*struct*/ QTreeView {
  pub fn setUniformRowHeights_0<RetType, T: QTreeView_setUniformRowHeights_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setUniformRowHeights_0(self);
    // return 1;
  }
}
pub trait QTreeView_setUniformRowHeights_0<RetType> {
  fn setUniformRowHeights_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_setUniformRowHeights_0<(/*void*/)> for (bool) {
  fn setUniformRowHeights_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView20setUniformRowHeightsEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:94
// index:0
// Public Visibility=Default Availability=Available
// [1] bool itemsExpandable() const

/*

*/
impl /*struct*/ QTreeView {
  pub fn itemsExpandable_0<RetType, T: QTreeView_itemsExpandable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemsExpandable_0(self);
    // return 1;
  }
}
pub trait QTreeView_itemsExpandable_0<RetType> {
  fn itemsExpandable_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_itemsExpandable_0<bool> for () {
  fn itemsExpandable_0(self , rsthis: & QTreeView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTreeView15itemsExpandableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:95
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setItemsExpandable(bool)

/*

*/
impl /*struct*/ QTreeView {
  pub fn setItemsExpandable_0<RetType, T: QTreeView_setItemsExpandable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setItemsExpandable_0(self);
    // return 1;
  }
}
pub trait QTreeView_setItemsExpandable_0<RetType> {
  fn setItemsExpandable_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_setItemsExpandable_0<(/*void*/)> for (bool) {
  fn setItemsExpandable_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView18setItemsExpandableEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:97
// index:0
// Public Visibility=Default Availability=Available
// [1] bool expandsOnDoubleClick() const

/*

*/
impl /*struct*/ QTreeView {
  pub fn expandsOnDoubleClick_0<RetType, T: QTreeView_expandsOnDoubleClick_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.expandsOnDoubleClick_0(self);
    // return 1;
  }
}
pub trait QTreeView_expandsOnDoubleClick_0<RetType> {
  fn expandsOnDoubleClick_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_expandsOnDoubleClick_0<bool> for () {
  fn expandsOnDoubleClick_0(self , rsthis: & QTreeView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTreeView20expandsOnDoubleClickEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:98
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setExpandsOnDoubleClick(bool)

/*

*/
impl /*struct*/ QTreeView {
  pub fn setExpandsOnDoubleClick_0<RetType, T: QTreeView_setExpandsOnDoubleClick_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setExpandsOnDoubleClick_0(self);
    // return 1;
  }
}
pub trait QTreeView_setExpandsOnDoubleClick_0<RetType> {
  fn setExpandsOnDoubleClick_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_setExpandsOnDoubleClick_0<(/*void*/)> for (bool) {
  fn setExpandsOnDoubleClick_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView23setExpandsOnDoubleClickEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:100
// index:0
// Public Visibility=Default Availability=Available
// [4] int columnViewportPosition(int) const

/*
Returns the horizontal position of the column in the viewport.
*/
impl /*struct*/ QTreeView {
  pub fn columnViewportPosition_0<RetType, T: QTreeView_columnViewportPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnViewportPosition_0(self);
    // return 1;
  }
}
pub trait QTreeView_columnViewportPosition_0<RetType> {
  fn columnViewportPosition_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_columnViewportPosition_0<i32> for (i32) {
  fn columnViewportPosition_0(self , rsthis: & QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTreeView22columnViewportPositionEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:101
// index:0
// Public Visibility=Default Availability=Available
// [4] int columnWidth(int) const

/*
Returns the width of the column.

See also resizeColumnToContents() and setColumnWidth().
*/
impl /*struct*/ QTreeView {
  pub fn columnWidth_0<RetType, T: QTreeView_columnWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnWidth_0(self);
    // return 1;
  }
}
pub trait QTreeView_columnWidth_0<RetType> {
  fn columnWidth_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_columnWidth_0<i32> for (i32) {
  fn columnWidth_0(self , rsthis: & QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTreeView11columnWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:102
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setColumnWidth(int, int)

/*
Sets the width of the given column to the width specified.

This function was introduced in  Qt 4.2.

See also columnWidth() and resizeColumnToContents().
*/
impl /*struct*/ QTreeView {
  pub fn setColumnWidth_0<RetType, T: QTreeView_setColumnWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setColumnWidth_0(self);
    // return 1;
  }
}
pub trait QTreeView_setColumnWidth_0<RetType> {
  fn setColumnWidth_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_setColumnWidth_0<(/*void*/)> for (i32,i32) {
  fn setColumnWidth_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView14setColumnWidthEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:103
// index:0
// Public Visibility=Default Availability=Available
// [4] int columnAt(int) const

/*
Returns the column in the tree view whose header covers the x coordinate given.
*/
impl /*struct*/ QTreeView {
  pub fn columnAt_0<RetType, T: QTreeView_columnAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnAt_0(self);
    // return 1;
  }
}
pub trait QTreeView_columnAt_0<RetType> {
  fn columnAt_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_columnAt_0<i32> for (i32) {
  fn columnAt_0(self , rsthis: & QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTreeView8columnAtEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:105
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isColumnHidden(int) const

/*
Returns true if the column is hidden; otherwise returns false.

See also hideColumn() and isRowHidden().
*/
impl /*struct*/ QTreeView {
  pub fn isColumnHidden_0<RetType, T: QTreeView_isColumnHidden_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isColumnHidden_0(self);
    // return 1;
  }
}
pub trait QTreeView_isColumnHidden_0<RetType> {
  fn isColumnHidden_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_isColumnHidden_0<bool> for (i32) {
  fn isColumnHidden_0(self , rsthis: & QTreeView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTreeView14isColumnHiddenEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:106
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setColumnHidden(int, bool)

/*
If hide is true the column is hidden, otherwise the column is shown.

See also isColumnHidden(), hideColumn(), and setRowHidden().
*/
impl /*struct*/ QTreeView {
  pub fn setColumnHidden_0<RetType, T: QTreeView_setColumnHidden_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setColumnHidden_0(self);
    // return 1;
  }
}
pub trait QTreeView_setColumnHidden_0<RetType> {
  fn setColumnHidden_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_setColumnHidden_0<(/*void*/)> for (i32,bool) {
  fn setColumnHidden_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView15setColumnHiddenEib", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:108
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isHeaderHidden() const

/*

*/
impl /*struct*/ QTreeView {
  pub fn isHeaderHidden_0<RetType, T: QTreeView_isHeaderHidden_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isHeaderHidden_0(self);
    // return 1;
  }
}
pub trait QTreeView_isHeaderHidden_0<RetType> {
  fn isHeaderHidden_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_isHeaderHidden_0<bool> for () {
  fn isHeaderHidden_0(self , rsthis: & QTreeView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTreeView14isHeaderHiddenEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:109
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setHeaderHidden(bool)

/*

*/
impl /*struct*/ QTreeView {
  pub fn setHeaderHidden_0<RetType, T: QTreeView_setHeaderHidden_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHeaderHidden_0(self);
    // return 1;
  }
}
pub trait QTreeView_setHeaderHidden_0<RetType> {
  fn setHeaderHidden_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_setHeaderHidden_0<(/*void*/)> for (bool) {
  fn setHeaderHidden_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView15setHeaderHiddenEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:111
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isRowHidden(int, const QModelIndex &) const

/*
Returns true if the item in the given row of the parent is hidden; otherwise returns false.

See also setRowHidden() and isColumnHidden().
*/
impl /*struct*/ QTreeView {
  pub fn isRowHidden_0<RetType, T: QTreeView_isRowHidden_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isRowHidden_0(self);
    // return 1;
  }
}
pub trait QTreeView_isRowHidden_0<RetType> {
  fn isRowHidden_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_isRowHidden_0<bool> for (i32,usize) {
  fn isRowHidden_0(self , rsthis: & QTreeView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTreeView11isRowHiddenEiRK11QModelIndex", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:112
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRowHidden(int, const QModelIndex &, bool)

/*
If hide is true the row with the given parent is hidden, otherwise the row is shown.

See also isRowHidden() and setColumnHidden().
*/
impl /*struct*/ QTreeView {
  pub fn setRowHidden_0<RetType, T: QTreeView_setRowHidden_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRowHidden_0(self);
    // return 1;
  }
}
pub trait QTreeView_setRowHidden_0<RetType> {
  fn setRowHidden_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_setRowHidden_0<(/*void*/)> for (i32,usize,bool) {
  fn setRowHidden_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView12setRowHiddenEiRK11QModelIndexb", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:114
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isFirstColumnSpanned(int, const QModelIndex &) const

/*
Returns true if the item in first column in the given row of the parent is spanning all the columns; otherwise returns false.

This function was introduced in  Qt 4.3.

See also setFirstColumnSpanned().
*/
impl /*struct*/ QTreeView {
  pub fn isFirstColumnSpanned_0<RetType, T: QTreeView_isFirstColumnSpanned_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isFirstColumnSpanned_0(self);
    // return 1;
  }
}
pub trait QTreeView_isFirstColumnSpanned_0<RetType> {
  fn isFirstColumnSpanned_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_isFirstColumnSpanned_0<bool> for (i32,usize) {
  fn isFirstColumnSpanned_0(self , rsthis: & QTreeView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTreeView20isFirstColumnSpannedEiRK11QModelIndex", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:115
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFirstColumnSpanned(int, const QModelIndex &, bool)

/*
If span is true the item in the first column in the row with the given parent is set to span all columns, otherwise all items on the row are shown.

This function was introduced in  Qt 4.3.

See also isFirstColumnSpanned().
*/
impl /*struct*/ QTreeView {
  pub fn setFirstColumnSpanned_0<RetType, T: QTreeView_setFirstColumnSpanned_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFirstColumnSpanned_0(self);
    // return 1;
  }
}
pub trait QTreeView_setFirstColumnSpanned_0<RetType> {
  fn setFirstColumnSpanned_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_setFirstColumnSpanned_0<(/*void*/)> for (i32,usize,bool) {
  fn setFirstColumnSpanned_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView21setFirstColumnSpannedEiRK11QModelIndexb", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:117
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isExpanded(const QModelIndex &) const

/*
Returns true if the model item index is expanded; otherwise returns false.

See also expand(), expanded(), and setExpanded().
*/
impl /*struct*/ QTreeView {
  pub fn isExpanded_0<RetType, T: QTreeView_isExpanded_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isExpanded_0(self);
    // return 1;
  }
}
pub trait QTreeView_isExpanded_0<RetType> {
  fn isExpanded_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_isExpanded_0<bool> for (usize) {
  fn isExpanded_0(self , rsthis: & QTreeView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTreeView10isExpandedERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:118
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setExpanded(const QModelIndex &, bool)

/*
Sets the item referred to by index to either collapse or expanded, depending on the value of expanded.

See also expanded(), expand(), and isExpanded().
*/
impl /*struct*/ QTreeView {
  pub fn setExpanded_0<RetType, T: QTreeView_setExpanded_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setExpanded_0(self);
    // return 1;
  }
}
pub trait QTreeView_setExpanded_0<RetType> {
  fn setExpanded_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_setExpanded_0<(/*void*/)> for (usize,bool) {
  fn setExpanded_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView11setExpandedERK11QModelIndexb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:120
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSortingEnabled(bool)

/*

*/
impl /*struct*/ QTreeView {
  pub fn setSortingEnabled_0<RetType, T: QTreeView_setSortingEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSortingEnabled_0(self);
    // return 1;
  }
}
pub trait QTreeView_setSortingEnabled_0<RetType> {
  fn setSortingEnabled_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_setSortingEnabled_0<(/*void*/)> for (bool) {
  fn setSortingEnabled_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView17setSortingEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:121
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isSortingEnabled() const

/*

*/
impl /*struct*/ QTreeView {
  pub fn isSortingEnabled_0<RetType, T: QTreeView_isSortingEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isSortingEnabled_0(self);
    // return 1;
  }
}
pub trait QTreeView_isSortingEnabled_0<RetType> {
  fn isSortingEnabled_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_isSortingEnabled_0<bool> for () {
  fn isSortingEnabled_0(self , rsthis: & QTreeView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTreeView16isSortingEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:123
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAnimated(bool)

/*

*/
impl /*struct*/ QTreeView {
  pub fn setAnimated_0<RetType, T: QTreeView_setAnimated_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAnimated_0(self);
    // return 1;
  }
}
pub trait QTreeView_setAnimated_0<RetType> {
  fn setAnimated_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_setAnimated_0<(/*void*/)> for (bool) {
  fn setAnimated_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView11setAnimatedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:124
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isAnimated() const

/*

*/
impl /*struct*/ QTreeView {
  pub fn isAnimated_0<RetType, T: QTreeView_isAnimated_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isAnimated_0(self);
    // return 1;
  }
}
pub trait QTreeView_isAnimated_0<RetType> {
  fn isAnimated_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_isAnimated_0<bool> for () {
  fn isAnimated_0(self , rsthis: & QTreeView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTreeView10isAnimatedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:126
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAllColumnsShowFocus(bool)

/*

*/
impl /*struct*/ QTreeView {
  pub fn setAllColumnsShowFocus_0<RetType, T: QTreeView_setAllColumnsShowFocus_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAllColumnsShowFocus_0(self);
    // return 1;
  }
}
pub trait QTreeView_setAllColumnsShowFocus_0<RetType> {
  fn setAllColumnsShowFocus_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_setAllColumnsShowFocus_0<(/*void*/)> for (bool) {
  fn setAllColumnsShowFocus_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView22setAllColumnsShowFocusEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:127
// index:0
// Public Visibility=Default Availability=Available
// [1] bool allColumnsShowFocus() const

/*

*/
impl /*struct*/ QTreeView {
  pub fn allColumnsShowFocus_0<RetType, T: QTreeView_allColumnsShowFocus_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.allColumnsShowFocus_0(self);
    // return 1;
  }
}
pub trait QTreeView_allColumnsShowFocus_0<RetType> {
  fn allColumnsShowFocus_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_allColumnsShowFocus_0<bool> for () {
  fn allColumnsShowFocus_0(self , rsthis: & QTreeView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTreeView19allColumnsShowFocusEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:129
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWordWrap(bool)

/*

*/
impl /*struct*/ QTreeView {
  pub fn setWordWrap_0<RetType, T: QTreeView_setWordWrap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWordWrap_0(self);
    // return 1;
  }
}
pub trait QTreeView_setWordWrap_0<RetType> {
  fn setWordWrap_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_setWordWrap_0<(/*void*/)> for (bool) {
  fn setWordWrap_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView11setWordWrapEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:130
// index:0
// Public Visibility=Default Availability=Available
// [1] bool wordWrap() const

/*

*/
impl /*struct*/ QTreeView {
  pub fn wordWrap_0<RetType, T: QTreeView_wordWrap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.wordWrap_0(self);
    // return 1;
  }
}
pub trait QTreeView_wordWrap_0<RetType> {
  fn wordWrap_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_wordWrap_0<bool> for () {
  fn wordWrap_0(self , rsthis: & QTreeView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTreeView8wordWrapEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:132
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTreePosition(int)

/*
This specifies that the tree structure should be placed at logical index index. If  set to -1 then the tree will always follow visual index 0.

This function was introduced in  Qt 5.2.

See also treePosition(), QHeaderView::swapSections(), and QHeaderView::moveSection().
*/
impl /*struct*/ QTreeView {
  pub fn setTreePosition_0<RetType, T: QTreeView_setTreePosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTreePosition_0(self);
    // return 1;
  }
}
pub trait QTreeView_setTreePosition_0<RetType> {
  fn setTreePosition_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_setTreePosition_0<(/*void*/)> for (i32) {
  fn setTreePosition_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView15setTreePositionEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:133
// index:0
// Public Visibility=Default Availability=Available
// [4] int treePosition() const

/*
Return the logical index the tree is set on. If the return value is -1 then the tree is placed on the visual index 0.

This function was introduced in  Qt 5.2.

See also setTreePosition().
*/
impl /*struct*/ QTreeView {
  pub fn treePosition_0<RetType, T: QTreeView_treePosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.treePosition_0(self);
    // return 1;
  }
}
pub trait QTreeView_treePosition_0<RetType> {
  fn treePosition_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_treePosition_0<i32> for () {
  fn treePosition_0(self , rsthis: & QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTreeView12treePositionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:135
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void keyboardSearch(const QString &)

/*
Reimplemented from QAbstractItemView::keyboardSearch().
*/
impl /*struct*/ QTreeView {
  pub fn keyboardSearch_0<RetType, T: QTreeView_keyboardSearch_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyboardSearch_0(self);
    // return 1;
  }
}
pub trait QTreeView_keyboardSearch_0<RetType> {
  fn keyboardSearch_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_keyboardSearch_0<(/*void*/)> for (usize) {
  fn keyboardSearch_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView14keyboardSearchERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:137
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QRect visualRect(const QModelIndex &) const

/*
Reimplemented from QAbstractItemView::visualRect().

Returns the rectangle on the viewport occupied by the item at index. If the index is not visible or explicitly hidden, the returned rectangle is invalid.
*/
impl /*struct*/ QTreeView {
  pub fn visualRect_0<RetType, T: QTreeView_visualRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.visualRect_0(self);
    // return 1;
  }
}
pub trait QTreeView_visualRect_0<RetType> {
  fn visualRect_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_visualRect_0<usize> for (usize) {
  fn visualRect_0(self , rsthis: & QTreeView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTreeView10visualRectERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:138
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void scrollTo(const QModelIndex &, QAbstractItemView::ScrollHint)

/*
Reimplemented from QAbstractItemView::scrollTo().

Scroll the contents of the tree view until the given model item index is visible. The hint parameter specifies more precisely where the item should be located after the operation. If any of the parents of the model item are collapsed, they will be expanded to ensure that the model item is visible.
*/
impl /*struct*/ QTreeView {
  pub fn scrollTo_0<RetType, T: QTreeView_scrollTo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scrollTo_0(self);
    // return 1;
  }
}
pub trait QTreeView_scrollTo_0<RetType> {
  fn scrollTo_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_scrollTo_0<(/*void*/)> for (usize,i32) {
  fn scrollTo_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView8scrollToERK11QModelIndexN17QAbstractItemView10ScrollHintE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:139
// index:0
// Public virtual Visibility=Default Availability=Available
// [24] QModelIndex indexAt(const QPoint &) const

/*
Reimplemented from QAbstractItemView::indexAt().
*/
impl /*struct*/ QTreeView {
  pub fn indexAt_0<RetType, T: QTreeView_indexAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexAt_0(self);
    // return 1;
  }
}
pub trait QTreeView_indexAt_0<RetType> {
  fn indexAt_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_indexAt_0<usize> for (usize) {
  fn indexAt_0(self , rsthis: & QTreeView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTreeView7indexAtERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:140
// index:0
// Public Visibility=Default Availability=Available
// [24] QModelIndex indexAbove(const QModelIndex &) const

/*
Returns the model index of the item above index.
*/
impl /*struct*/ QTreeView {
  pub fn indexAbove_0<RetType, T: QTreeView_indexAbove_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexAbove_0(self);
    // return 1;
  }
}
pub trait QTreeView_indexAbove_0<RetType> {
  fn indexAbove_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_indexAbove_0<usize> for (usize) {
  fn indexAbove_0(self , rsthis: & QTreeView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTreeView10indexAboveERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:141
// index:0
// Public Visibility=Default Availability=Available
// [24] QModelIndex indexBelow(const QModelIndex &) const

/*
Returns the model index of the item below index.
*/
impl /*struct*/ QTreeView {
  pub fn indexBelow_0<RetType, T: QTreeView_indexBelow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexBelow_0(self);
    // return 1;
  }
}
pub trait QTreeView_indexBelow_0<RetType> {
  fn indexBelow_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_indexBelow_0<usize> for (usize) {
  fn indexBelow_0(self , rsthis: & QTreeView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTreeView10indexBelowERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:143
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void doItemsLayout()

/*

*/
impl /*struct*/ QTreeView {
  pub fn doItemsLayout_0<RetType, T: QTreeView_doItemsLayout_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.doItemsLayout_0(self);
    // return 1;
  }
}
pub trait QTreeView_doItemsLayout_0<RetType> {
  fn doItemsLayout_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_doItemsLayout_0<(/*void*/)> for () {
  fn doItemsLayout_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QTreeView13doItemsLayoutEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:144
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void reset()

/*
Reimplemented from QAbstractItemView::reset().
*/
impl /*struct*/ QTreeView {
  pub fn reset_0<RetType, T: QTreeView_reset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.reset_0(self);
    // return 1;
  }
}
pub trait QTreeView_reset_0<RetType> {
  fn reset_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_reset_0<(/*void*/)> for () {
  fn reset_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QTreeView5resetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:146
// index:0
// Public Visibility=Default Availability=Available
// [-2] void sortByColumn(int, Qt::SortOrder)

/*
Sets the model up for sorting by the values in the given column and order.

column may be -1, in which case no sort indicator will be shown and the model will return to its natural, unsorted order. Note that not all models support this and may even crash in this case.

This function was introduced in  Qt 4.2.

See also sortingEnabled.
*/
impl /*struct*/ QTreeView {
  pub fn sortByColumn_0<RetType, T: QTreeView_sortByColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sortByColumn_0(self);
    // return 1;
  }
}
pub trait QTreeView_sortByColumn_0<RetType> {
  fn sortByColumn_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_sortByColumn_0<(/*void*/)> for (i32,i32) {
  fn sortByColumn_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView12sortByColumnEiN2Qt9SortOrderE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:161
// index:1
// Public Visibility=Default Availability=Available
// [-2] void sortByColumn(int)

/*
Sets the model up for sorting by the values in the given column and order.

column may be -1, in which case no sort indicator will be shown and the model will return to its natural, unsorted order. Note that not all models support this and may even crash in this case.

This function was introduced in  Qt 4.2.

See also sortingEnabled.
*/
impl /*struct*/ QTreeView {
  pub fn sortByColumn_1<RetType, T: QTreeView_sortByColumn_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sortByColumn_1(self);
    // return 1;
  }
}
pub trait QTreeView_sortByColumn_1<RetType> {
  fn sortByColumn_1(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_sortByColumn_1<(/*void*/)> for (i32) {
  fn sortByColumn_1(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView12sortByColumnEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:149
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void selectAll()

/*
Reimplemented from QAbstractItemView::selectAll().
*/
impl /*struct*/ QTreeView {
  pub fn selectAll_0<RetType, T: QTreeView_selectAll_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectAll_0(self);
    // return 1;
  }
}
pub trait QTreeView_selectAll_0<RetType> {
  fn selectAll_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_selectAll_0<(/*void*/)> for () {
  fn selectAll_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QTreeView9selectAllEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:152
// index:0
// Public Visibility=Default Availability=Available
// [-2] void expanded(const QModelIndex &)

/*
This signal is emitted when the item specified by index is expanded.

See also setExpanded().
*/
impl /*struct*/ QTreeView {
  pub fn expanded_0<RetType, T: QTreeView_expanded_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.expanded_0(self);
    // return 1;
  }
}
pub trait QTreeView_expanded_0<RetType> {
  fn expanded_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_expanded_0<(/*void*/)> for (usize) {
  fn expanded_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView8expandedERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:153
// index:0
// Public Visibility=Default Availability=Available
// [-2] void collapsed(const QModelIndex &)

/*
This signal is emitted when the item specified by index is collapsed.
*/
impl /*struct*/ QTreeView {
  pub fn collapsed_0<RetType, T: QTreeView_collapsed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.collapsed_0(self);
    // return 1;
  }
}
pub trait QTreeView_collapsed_0<RetType> {
  fn collapsed_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_collapsed_0<(/*void*/)> for (usize) {
  fn collapsed_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView9collapsedERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:156
// index:0
// Public Visibility=Default Availability=Available
// [-2] void hideColumn(int)

/*
Hides the column given.

Note: This function should only be called after the model has been initialized, as the view needs to know the number of columns in order to hide column.

See also showColumn() and setColumnHidden().
*/
impl /*struct*/ QTreeView {
  pub fn hideColumn_0<RetType, T: QTreeView_hideColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hideColumn_0(self);
    // return 1;
  }
}
pub trait QTreeView_hideColumn_0<RetType> {
  fn hideColumn_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_hideColumn_0<(/*void*/)> for (i32) {
  fn hideColumn_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView10hideColumnEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:157
// index:0
// Public Visibility=Default Availability=Available
// [-2] void showColumn(int)

/*
Shows the given column in the tree view.

See also hideColumn() and setColumnHidden().
*/
impl /*struct*/ QTreeView {
  pub fn showColumn_0<RetType, T: QTreeView_showColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showColumn_0(self);
    // return 1;
  }
}
pub trait QTreeView_showColumn_0<RetType> {
  fn showColumn_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_showColumn_0<(/*void*/)> for (i32) {
  fn showColumn_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView10showColumnEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:158
// index:0
// Public Visibility=Default Availability=Available
// [-2] void expand(const QModelIndex &)

/*
Expands the model item specified by the index.

See also expanded().
*/
impl /*struct*/ QTreeView {
  pub fn expand_0<RetType, T: QTreeView_expand_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.expand_0(self);
    // return 1;
  }
}
pub trait QTreeView_expand_0<RetType> {
  fn expand_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_expand_0<(/*void*/)> for (usize) {
  fn expand_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView6expandERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:159
// index:0
// Public Visibility=Default Availability=Available
// [-2] void collapse(const QModelIndex &)

/*
Collapses the model item specified by the index.

See also collapsed().
*/
impl /*struct*/ QTreeView {
  pub fn collapse_0<RetType, T: QTreeView_collapse_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.collapse_0(self);
    // return 1;
  }
}
pub trait QTreeView_collapse_0<RetType> {
  fn collapse_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_collapse_0<(/*void*/)> for (usize) {
  fn collapse_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView8collapseERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:160
// index:0
// Public Visibility=Default Availability=Available
// [-2] void resizeColumnToContents(int)

/*
Resizes the column given to the size of its contents.

See also columnWidth(), setColumnWidth(), sizeHintForColumn(), and QHeaderView::resizeContentsPrecision().
*/
impl /*struct*/ QTreeView {
  pub fn resizeColumnToContents_0<RetType, T: QTreeView_resizeColumnToContents_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeColumnToContents_0(self);
    // return 1;
  }
}
pub trait QTreeView_resizeColumnToContents_0<RetType> {
  fn resizeColumnToContents_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_resizeColumnToContents_0<(/*void*/)> for (i32) {
  fn resizeColumnToContents_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView22resizeColumnToContentsEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:162
// index:0
// Public Visibility=Default Availability=Available
// [-2] void expandAll()

/*
Expands all expandable items.

Warning: if the model contains a large number of items, this function will take some time to execute.

This function was introduced in  Qt 4.2.

See also collapseAll(), expand(), collapse(), and setExpanded().
*/
impl /*struct*/ QTreeView {
  pub fn expandAll_0<RetType, T: QTreeView_expandAll_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.expandAll_0(self);
    // return 1;
  }
}
pub trait QTreeView_expandAll_0<RetType> {
  fn expandAll_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_expandAll_0<(/*void*/)> for () {
  fn expandAll_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QTreeView9expandAllEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:163
// index:0
// Public Visibility=Default Availability=Available
// [-2] void collapseAll()

/*
Collapses all expanded items.

This function was introduced in  Qt 4.2.

See also expandAll(), expand(), collapse(), and setExpanded().
*/
impl /*struct*/ QTreeView {
  pub fn collapseAll_0<RetType, T: QTreeView_collapseAll_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.collapseAll_0(self);
    // return 1;
  }
}
pub trait QTreeView_collapseAll_0<RetType> {
  fn collapseAll_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_collapseAll_0<(/*void*/)> for () {
  fn collapseAll_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QTreeView11collapseAllEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:164
// index:0
// Public Visibility=Default Availability=Available
// [-2] void expandToDepth(int)

/*
Expands all expandable items to the given depth.

This function was introduced in  Qt 4.3.

See also expandAll(), collapseAll(), expand(), collapse(), and setExpanded().
*/
impl /*struct*/ QTreeView {
  pub fn expandToDepth_0<RetType, T: QTreeView_expandToDepth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.expandToDepth_0(self);
    // return 1;
  }
}
pub trait QTreeView_expandToDepth_0<RetType> {
  fn expandToDepth_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_expandToDepth_0<(/*void*/)> for (i32) {
  fn expandToDepth_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView13expandToDepthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:167
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void columnResized(int, int, int)

/*
This function is called whenever column's size is changed in the header. oldSize and newSize give the previous size and the new size in pixels.

See also setColumnWidth().
*/
impl /*struct*/ QTreeView {
  pub fn columnResized_0<RetType, T: QTreeView_columnResized_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnResized_0(self);
    // return 1;
  }
}
pub trait QTreeView_columnResized_0<RetType> {
  fn columnResized_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_columnResized_0<(/*void*/)> for (i32,i32,i32) {
  fn columnResized_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView13columnResizedEiii", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:168
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void columnCountChanged(int, int)

/*
Informs the tree view that the number of columns in the tree view has changed from oldCount to newCount.
*/
impl /*struct*/ QTreeView {
  pub fn columnCountChanged_0<RetType, T: QTreeView_columnCountChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnCountChanged_0(self);
    // return 1;
  }
}
pub trait QTreeView_columnCountChanged_0<RetType> {
  fn columnCountChanged_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_columnCountChanged_0<(/*void*/)> for (i32,i32) {
  fn columnCountChanged_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView18columnCountChangedEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:169
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void columnMoved()

/*
This slot is called whenever a column has been moved.
*/
impl /*struct*/ QTreeView {
  pub fn columnMoved_0<RetType, T: QTreeView_columnMoved_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnMoved_0(self);
    // return 1;
  }
}
pub trait QTreeView_columnMoved_0<RetType> {
  fn columnMoved_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_columnMoved_0<(/*void*/)> for () {
  fn columnMoved_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QTreeView11columnMovedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:170
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void reexpand()

/*

*/
impl /*struct*/ QTreeView {
  pub fn reexpand_0<RetType, T: QTreeView_reexpand_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.reexpand_0(self);
    // return 1;
  }
}
pub trait QTreeView_reexpand_0<RetType> {
  fn reexpand_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_reexpand_0<(/*void*/)> for () {
  fn reexpand_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QTreeView8reexpandEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:171
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void rowsRemoved(const QModelIndex &, int, int)

/*
Informs the view that the rows from the start row to the end row inclusive have been removed from the given parent model item.

This function was introduced in  Qt 4.1.
*/
impl /*struct*/ QTreeView {
  pub fn rowsRemoved_0<RetType, T: QTreeView_rowsRemoved_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowsRemoved_0(self);
    // return 1;
  }
}
pub trait QTreeView_rowsRemoved_0<RetType> {
  fn rowsRemoved_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_rowsRemoved_0<(/*void*/)> for (usize,i32,i32) {
  fn rowsRemoved_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView11rowsRemovedERK11QModelIndexii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:175
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void scrollContentsBy(int, int)

/*
Reimplemented from QAbstractScrollArea::scrollContentsBy().

Scrolls the contents of the tree view by (dx, dy).
*/
impl /*struct*/ QTreeView {
  pub fn scrollContentsBy_0<RetType, T: QTreeView_scrollContentsBy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scrollContentsBy_0(self);
    // return 1;
  }
}
pub trait QTreeView_scrollContentsBy_0<RetType> {
  fn scrollContentsBy_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_scrollContentsBy_0<(/*void*/)> for (i32,i32) {
  fn scrollContentsBy_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView16scrollContentsByEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:176
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void rowsInserted(const QModelIndex &, int, int)

/*
Reimplemented from QAbstractItemView::rowsInserted().

Informs the view that the rows from the start row to the end row inclusive have been inserted into the parent model item.
*/
impl /*struct*/ QTreeView {
  pub fn rowsInserted_0<RetType, T: QTreeView_rowsInserted_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowsInserted_0(self);
    // return 1;
  }
}
pub trait QTreeView_rowsInserted_0<RetType> {
  fn rowsInserted_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_rowsInserted_0<(/*void*/)> for (usize,i32,i32) {
  fn rowsInserted_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView12rowsInsertedERK11QModelIndexii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:177
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void rowsAboutToBeRemoved(const QModelIndex &, int, int)

/*
Reimplemented from QAbstractItemView::rowsAboutToBeRemoved().

Informs the view that the rows from the start row to the end row inclusive are about to removed from the given parent model item.
*/
impl /*struct*/ QTreeView {
  pub fn rowsAboutToBeRemoved_0<RetType, T: QTreeView_rowsAboutToBeRemoved_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowsAboutToBeRemoved_0(self);
    // return 1;
  }
}
pub trait QTreeView_rowsAboutToBeRemoved_0<RetType> {
  fn rowsAboutToBeRemoved_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_rowsAboutToBeRemoved_0<(/*void*/)> for (usize,i32,i32) {
  fn rowsAboutToBeRemoved_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView20rowsAboutToBeRemovedERK11QModelIndexii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:179
// index:0
// Protected virtual Visibility=Default Availability=Available
// [24] QModelIndex moveCursor(QAbstractItemView::CursorAction, Qt::KeyboardModifiers)

/*
Reimplemented from QAbstractItemView::moveCursor().

Move the cursor in the way described by cursorAction, using the information provided by the button modifiers.
*/
impl /*struct*/ QTreeView {
  pub fn moveCursor_0<RetType, T: QTreeView_moveCursor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveCursor_0(self);
    // return 1;
  }
}
pub trait QTreeView_moveCursor_0<RetType> {
  fn moveCursor_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_moveCursor_0<usize> for (i32,i32) {
  fn moveCursor_0(self , rsthis: & QTreeView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QTreeView10moveCursorEN17QAbstractItemView12CursorActionE6QFlagsIN2Qt16KeyboardModifierEE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:180
// index:0
// Protected virtual Visibility=Default Availability=Available
// [4] int horizontalOffset() const

/*
Reimplemented from QAbstractItemView::horizontalOffset().

Returns the horizontal offset of the items in the treeview.

Note that the tree view uses the horizontal header section positions to determine the positions of columns in the view.

See also verticalOffset().
*/
impl /*struct*/ QTreeView {
  pub fn horizontalOffset_0<RetType, T: QTreeView_horizontalOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.horizontalOffset_0(self);
    // return 1;
  }
}
pub trait QTreeView_horizontalOffset_0<RetType> {
  fn horizontalOffset_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_horizontalOffset_0<i32> for () {
  fn horizontalOffset_0(self , rsthis: & QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTreeView16horizontalOffsetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:181
// index:0
// Protected virtual Visibility=Default Availability=Available
// [4] int verticalOffset() const

/*
Reimplemented from QAbstractItemView::verticalOffset().

Returns the vertical offset of the items in the tree view.

See also horizontalOffset().
*/
impl /*struct*/ QTreeView {
  pub fn verticalOffset_0<RetType, T: QTreeView_verticalOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.verticalOffset_0(self);
    // return 1;
  }
}
pub trait QTreeView_verticalOffset_0<RetType> {
  fn verticalOffset_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_verticalOffset_0<i32> for () {
  fn verticalOffset_0(self , rsthis: & QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTreeView14verticalOffsetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:183
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void setSelection(const QRect &, QItemSelectionModel::SelectionFlags)

/*
Reimplemented from QAbstractItemView::setSelection().

Applies the selection command to the items in or touched by the rectangle, rect.

See also selectionCommand().
*/
impl /*struct*/ QTreeView {
  pub fn setSelection_0<RetType, T: QTreeView_setSelection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSelection_0(self);
    // return 1;
  }
}
pub trait QTreeView_setSelection_0<RetType> {
  fn setSelection_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_setSelection_0<(/*void*/)> for (usize,i32) {
  fn setSelection_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView12setSelectionERK5QRect6QFlagsIN19QItemSelectionModel13SelectionFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:184
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] QRegion visualRegionForSelection(const QItemSelection &) const

/*
Reimplemented from QAbstractItemView::visualRegionForSelection().

Returns the rectangle from the viewport of the items in the given selection.

Since 4.7, the returned region only contains rectangles intersecting (or included in) the viewport.
*/
impl /*struct*/ QTreeView {
  pub fn visualRegionForSelection_0<RetType, T: QTreeView_visualRegionForSelection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.visualRegionForSelection_0(self);
    // return 1;
  }
}
pub trait QTreeView_visualRegionForSelection_0<RetType> {
  fn visualRegionForSelection_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_visualRegionForSelection_0<usize> for (usize) {
  fn visualRegionForSelection_0(self , rsthis: & QTreeView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTreeView24visualRegionForSelectionERK14QItemSelection", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:185
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] QModelIndexList selectedIndexes() const

/*
Reimplemented from QAbstractItemView::selectedIndexes().
*/
impl /*struct*/ QTreeView {
  pub fn selectedIndexes_0<RetType, T: QTreeView_selectedIndexes_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectedIndexes_0(self);
    // return 1;
  }
}
pub trait QTreeView_selectedIndexes_0<RetType> {
  fn selectedIndexes_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_selectedIndexes_0<usize> for () {
  fn selectedIndexes_0(self , rsthis: & QTreeView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTreeView15selectedIndexesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:187
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void timerEvent(QTimerEvent *)

/*
Reimplemented from QObject::timerEvent().
*/
impl /*struct*/ QTreeView {
  pub fn timerEvent_0<RetType, T: QTreeView_timerEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.timerEvent_0(self);
    // return 1;
  }
}
pub trait QTreeView_timerEvent_0<RetType> {
  fn timerEvent_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_timerEvent_0<(/*void*/)> for (usize) {
  fn timerEvent_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView10timerEventEP11QTimerEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:188
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Reimplemented from QWidget::paintEvent().
*/
impl /*struct*/ QTreeView {
  pub fn paintEvent_0<RetType, T: QTreeView_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QTreeView_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:190
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void drawTree(QPainter *, const QRegion &) const

/*
Draws the part of the tree intersecting the given region using the specified painter.

This function was introduced in  Qt 4.2.

See also paintEvent().
*/
impl /*struct*/ QTreeView {
  pub fn drawTree_0<RetType, T: QTreeView_drawTree_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawTree_0(self);
    // return 1;
  }
}
pub trait QTreeView_drawTree_0<RetType> {
  fn drawTree_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_drawTree_0<(/*void*/)> for (usize,usize) {
  fn drawTree_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK9QTreeView8drawTreeEP8QPainterRK7QRegion", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:191
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void drawRow(QPainter *, const QStyleOptionViewItem &, const QModelIndex &) const

/*
Draws the row in the tree view that contains the model item index, using the painter given. The option controls how the item is displayed.

See also setAlternatingRowColors().
*/
impl /*struct*/ QTreeView {
  pub fn drawRow_0<RetType, T: QTreeView_drawRow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawRow_0(self);
    // return 1;
  }
}
pub trait QTreeView_drawRow_0<RetType> {
  fn drawRow_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_drawRow_0<(/*void*/)> for (usize,usize,usize) {
  fn drawRow_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK9QTreeView7drawRowEP8QPainterRK20QStyleOptionViewItemRK11QModelIndex", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:194
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void drawBranches(QPainter *, const QRect &, const QModelIndex &) const

/*
Draws the branches in the tree view on the same row as the model item index, using the painter given. The branches are drawn in the rectangle specified by rect.
*/
impl /*struct*/ QTreeView {
  pub fn drawBranches_0<RetType, T: QTreeView_drawBranches_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawBranches_0(self);
    // return 1;
  }
}
pub trait QTreeView_drawBranches_0<RetType> {
  fn drawBranches_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_drawBranches_0<(/*void*/)> for (usize,usize,usize) {
  fn drawBranches_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK9QTreeView12drawBranchesEP8QPainterRK5QRectRK11QModelIndex", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:198
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mousePressEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mousePressEvent().
*/
impl /*struct*/ QTreeView {
  pub fn mousePressEvent_0<RetType, T: QTreeView_mousePressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mousePressEvent_0(self);
    // return 1;
  }
}
pub trait QTreeView_mousePressEvent_0<RetType> {
  fn mousePressEvent_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_mousePressEvent_0<(/*void*/)> for (usize) {
  fn mousePressEvent_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView15mousePressEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:199
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseReleaseEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseReleaseEvent().
*/
impl /*struct*/ QTreeView {
  pub fn mouseReleaseEvent_0<RetType, T: QTreeView_mouseReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QTreeView_mouseReleaseEvent_0<RetType> {
  fn mouseReleaseEvent_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_mouseReleaseEvent_0<(/*void*/)> for (usize) {
  fn mouseReleaseEvent_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView17mouseReleaseEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:200
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseDoubleClickEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseDoubleClickEvent().
*/
impl /*struct*/ QTreeView {
  pub fn mouseDoubleClickEvent_0<RetType, T: QTreeView_mouseDoubleClickEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseDoubleClickEvent_0(self);
    // return 1;
  }
}
pub trait QTreeView_mouseDoubleClickEvent_0<RetType> {
  fn mouseDoubleClickEvent_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_mouseDoubleClickEvent_0<(/*void*/)> for (usize) {
  fn mouseDoubleClickEvent_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView21mouseDoubleClickEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:201
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseMoveEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseMoveEvent().
*/
impl /*struct*/ QTreeView {
  pub fn mouseMoveEvent_0<RetType, T: QTreeView_mouseMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseMoveEvent_0(self);
    // return 1;
  }
}
pub trait QTreeView_mouseMoveEvent_0<RetType> {
  fn mouseMoveEvent_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_mouseMoveEvent_0<(/*void*/)> for (usize) {
  fn mouseMoveEvent_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView14mouseMoveEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:202
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyPressEvent(QKeyEvent *)

/*
Reimplemented from QWidget::keyPressEvent().
*/
impl /*struct*/ QTreeView {
  pub fn keyPressEvent_0<RetType, T: QTreeView_keyPressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyPressEvent_0(self);
    // return 1;
  }
}
pub trait QTreeView_keyPressEvent_0<RetType> {
  fn keyPressEvent_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_keyPressEvent_0<(/*void*/)> for (usize) {
  fn keyPressEvent_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView13keyPressEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:204
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dragMoveEvent(QDragMoveEvent *)

/*
Reimplemented from QWidget::dragMoveEvent().
*/
impl /*struct*/ QTreeView {
  pub fn dragMoveEvent_0<RetType, T: QTreeView_dragMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragMoveEvent_0(self);
    // return 1;
  }
}
pub trait QTreeView_dragMoveEvent_0<RetType> {
  fn dragMoveEvent_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_dragMoveEvent_0<(/*void*/)> for (usize) {
  fn dragMoveEvent_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView13dragMoveEventEP14QDragMoveEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:206
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool viewportEvent(QEvent *)

/*
Reimplemented from QAbstractScrollArea::viewportEvent().
*/
impl /*struct*/ QTreeView {
  pub fn viewportEvent_0<RetType, T: QTreeView_viewportEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.viewportEvent_0(self);
    // return 1;
  }
}
pub trait QTreeView_viewportEvent_0<RetType> {
  fn viewportEvent_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_viewportEvent_0<bool> for (usize) {
  fn viewportEvent_0(self , rsthis: & QTreeView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QTreeView13viewportEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:208
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void updateGeometries()

/*
Reimplemented from QAbstractItemView::updateGeometries().
*/
impl /*struct*/ QTreeView {
  pub fn updateGeometries_0<RetType, T: QTreeView_updateGeometries_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updateGeometries_0(self);
    // return 1;
  }
}
pub trait QTreeView_updateGeometries_0<RetType> {
  fn updateGeometries_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_updateGeometries_0<(/*void*/)> for () {
  fn updateGeometries_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QTreeView16updateGeometriesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:210
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] QSize viewportSizeHint() const

/*
Reimplemented from QAbstractScrollArea::viewportSizeHint().
*/
impl /*struct*/ QTreeView {
  pub fn viewportSizeHint_0<RetType, T: QTreeView_viewportSizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.viewportSizeHint_0(self);
    // return 1;
  }
}
pub trait QTreeView_viewportSizeHint_0<RetType> {
  fn viewportSizeHint_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_viewportSizeHint_0<usize> for () {
  fn viewportSizeHint_0(self , rsthis: & QTreeView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTreeView16viewportSizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:212
// index:0
// Protected virtual Visibility=Default Availability=Available
// [4] int sizeHintForColumn(int) const

/*
Reimplemented from QAbstractItemView::sizeHintForColumn().

Returns the size hint for the column's width or -1 if there is no model.

If you need to set the width of a given column to a fixed value, call QHeaderView::resizeSection() on the view's header.

If you reimplement this function in a subclass, note that the value you return is only used when resizeColumnToContents() is called. In that case, if a larger column width is required by either the view's header or the item delegate, that width will be used instead.

See also QWidget::sizeHint, header(), and QHeaderView::resizeContentsPrecision().
*/
impl /*struct*/ QTreeView {
  pub fn sizeHintForColumn_0<RetType, T: QTreeView_sizeHintForColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHintForColumn_0(self);
    // return 1;
  }
}
pub trait QTreeView_sizeHintForColumn_0<RetType> {
  fn sizeHintForColumn_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_sizeHintForColumn_0<i32> for (i32) {
  fn sizeHintForColumn_0(self , rsthis: & QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTreeView17sizeHintForColumnEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:213
// index:0
// Protected Visibility=Default Availability=Available
// [4] int indexRowSizeHint(const QModelIndex &) const

/*
Returns the size hint for the row indicated by index.

See also sizeHintForColumn() and uniformRowHeights().
*/
impl /*struct*/ QTreeView {
  pub fn indexRowSizeHint_0<RetType, T: QTreeView_indexRowSizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexRowSizeHint_0(self);
    // return 1;
  }
}
pub trait QTreeView_indexRowSizeHint_0<RetType> {
  fn indexRowSizeHint_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_indexRowSizeHint_0<i32> for (usize) {
  fn indexRowSizeHint_0(self , rsthis: & QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTreeView16indexRowSizeHintERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:214
// index:0
// Protected Visibility=Default Availability=Available
// [4] int rowHeight(const QModelIndex &) const

/*
Returns the height of the row indicated by the given index.

This function was introduced in  Qt 4.3.

See also indexRowSizeHint().
*/
impl /*struct*/ QTreeView {
  pub fn rowHeight_0<RetType, T: QTreeView_rowHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowHeight_0(self);
    // return 1;
  }
}
pub trait QTreeView_rowHeight_0<RetType> {
  fn rowHeight_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_rowHeight_0<i32> for (usize) {
  fn rowHeight_0(self , rsthis: & QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTreeView9rowHeightERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:216
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void horizontalScrollbarAction(int)

/*

*/
impl /*struct*/ QTreeView {
  pub fn horizontalScrollbarAction_0<RetType, T: QTreeView_horizontalScrollbarAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.horizontalScrollbarAction_0(self);
    // return 1;
  }
}
pub trait QTreeView_horizontalScrollbarAction_0<RetType> {
  fn horizontalScrollbarAction_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_horizontalScrollbarAction_0<(/*void*/)> for (i32) {
  fn horizontalScrollbarAction_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView25horizontalScrollbarActionEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:218
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool isIndexHidden(const QModelIndex &) const

/*
Reimplemented from QAbstractItemView::isIndexHidden().
*/
impl /*struct*/ QTreeView {
  pub fn isIndexHidden_0<RetType, T: QTreeView_isIndexHidden_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isIndexHidden_0(self);
    // return 1;
  }
}
pub trait QTreeView_isIndexHidden_0<RetType> {
  fn isIndexHidden_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_isIndexHidden_0<bool> for (usize) {
  fn isIndexHidden_0(self , rsthis: & QTreeView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTreeView13isIndexHiddenERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:219
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void selectionChanged(const QItemSelection &, const QItemSelection &)

/*
Reimplemented from QAbstractItemView::selectionChanged().
*/
impl /*struct*/ QTreeView {
  pub fn selectionChanged_0<RetType, T: QTreeView_selectionChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectionChanged_0(self);
    // return 1;
  }
}
pub trait QTreeView_selectionChanged_0<RetType> {
  fn selectionChanged_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_selectionChanged_0<(/*void*/)> for (usize,usize) {
  fn selectionChanged_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView16selectionChangedERK14QItemSelectionS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtreeview.h:221
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void currentChanged(const QModelIndex &, const QModelIndex &)

/*
Reimplemented from QAbstractItemView::currentChanged().
*/
impl /*struct*/ QTreeView {
  pub fn currentChanged_0<RetType, T: QTreeView_currentChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentChanged_0(self);
    // return 1;
  }
}
pub trait QTreeView_currentChanged_0<RetType> {
  fn currentChanged_0(self , rsthis: & QTreeView) -> RetType;
}
impl<'a> /*trait*/ QTreeView_currentChanged_0<(/*void*/)> for (usize,usize) {
  fn currentChanged_0(self , rsthis: & QTreeView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTreeView14currentChangedERK11QModelIndexS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
