

// mod ::widgets::QListView
// package qtwidgets
// /usr/include/qt/QtWidgets/qlistview.h
// #include <qlistview.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 27
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
// func (this *QListView) InheritEvent(f func(e *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// void scrollContentsBy(int, int)
// func (this *QListView) InheritScrollContentsBy(f func(dx int, dy int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "scrollContentsBy", f)
// }

// void resizeContents(int, int)
// func (this *QListView) InheritResizeContents(f func(width int, height int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "resizeContents", f)
// }

// QSize contentsSize()
// func (this *QListView) InheritContentsSize(f func() unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "contentsSize", f)
// }

// void rowsInserted(const QModelIndex &, int, int)
// func (this *QListView) InheritRowsInserted(f func(parent *qtcore.QModelIndex, start int, end_ int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "rowsInserted", f)
// }

// void rowsAboutToBeRemoved(const QModelIndex &, int, int)
// func (this *QListView) InheritRowsAboutToBeRemoved(f func(parent *qtcore.QModelIndex, start int, end_ int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "rowsAboutToBeRemoved", f)
// }

// void mouseMoveEvent(QMouseEvent *)
// func (this *QListView) InheritMouseMoveEvent(f func(e *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseMoveEvent", f)
// }

// void mouseReleaseEvent(QMouseEvent *)
// func (this *QListView) InheritMouseReleaseEvent(f func(e *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseReleaseEvent", f)
// }

// void wheelEvent(QWheelEvent *)
// func (this *QListView) InheritWheelEvent(f func(e *qtgui.QWheelEvent/*777 QWheelEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "wheelEvent", f)
// }

// void timerEvent(QTimerEvent *)
// func (this *QListView) InheritTimerEvent(f func(e *qtcore.QTimerEvent/*777 QTimerEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "timerEvent", f)
// }

// void resizeEvent(QResizeEvent *)
// func (this *QListView) InheritResizeEvent(f func(e *qtgui.QResizeEvent/*777 QResizeEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "resizeEvent", f)
// }

// void dragMoveEvent(QDragMoveEvent *)
// func (this *QListView) InheritDragMoveEvent(f func(e *qtgui.QDragMoveEvent/*777 QDragMoveEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dragMoveEvent", f)
// }

// void dragLeaveEvent(QDragLeaveEvent *)
// func (this *QListView) InheritDragLeaveEvent(f func(e *qtgui.QDragLeaveEvent/*777 QDragLeaveEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dragLeaveEvent", f)
// }

// void dropEvent(QDropEvent *)
// func (this *QListView) InheritDropEvent(f func(e *qtgui.QDropEvent/*777 QDropEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dropEvent", f)
// }

// void startDrag(Qt::DropActions)
// func (this *QListView) InheritStartDrag(f func(supportedActions int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "startDrag", f)
// }

// QStyleOptionViewItem viewOptions()
// func (this *QListView) InheritViewOptions(f func() unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "viewOptions", f)
// }

// void paintEvent(QPaintEvent *)
// func (this *QListView) InheritPaintEvent(f func(e *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }

// int horizontalOffset()
// func (this *QListView) InheritHorizontalOffset(f func() int) {
//  qtrt.SetAllInheritCallback(this, "horizontalOffset", f)
// }

// int verticalOffset()
// func (this *QListView) InheritVerticalOffset(f func() int) {
//  qtrt.SetAllInheritCallback(this, "verticalOffset", f)
// }

// QModelIndex moveCursor(QAbstractItemView::CursorAction, Qt::KeyboardModifiers)
// func (this *QListView) InheritMoveCursor(f func(cursorAction int, modifiers int) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "moveCursor", f)
// }

// QRect rectForIndex(const QModelIndex &)
// func (this *QListView) InheritRectForIndex(f func(index *qtcore.QModelIndex) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "rectForIndex", f)
// }

// void setPositionForIndex(const QPoint &, const QModelIndex &)
// func (this *QListView) InheritSetPositionForIndex(f func(position *qtcore.QPoint, index *qtcore.QModelIndex) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "setPositionForIndex", f)
// }

// void setSelection(const QRect &, QItemSelectionModel::SelectionFlags)
// func (this *QListView) InheritSetSelection(f func(rect *qtcore.QRect, command int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "setSelection", f)
// }

// QRegion visualRegionForSelection(const QItemSelection &)
// func (this *QListView) InheritVisualRegionForSelection(f func(selection *qtcore.QItemSelection) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "visualRegionForSelection", f)
// }

// QModelIndexList selectedIndexes()
// func (this *QListView) InheritSelectedIndexes(f func() *qtcore.QModelIndexList/*9999*/) {
//  qtrt.SetAllInheritCallback(this, "selectedIndexes", f)
// }

// void updateGeometries()
// func (this *QListView) InheritUpdateGeometries(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "updateGeometries", f)
// }

// bool isIndexHidden(const QModelIndex &)
// func (this *QListView) InheritIsIndexHidden(f func(index *qtcore.QModelIndex) bool) {
//  qtrt.SetAllInheritCallback(this, "isIndexHidden", f)
// }

// void selectionChanged(const QItemSelection &, const QItemSelection &)
// func (this *QListView) InheritSelectionChanged(f func(selected *qtcore.QItemSelection, deselected *qtcore.QItemSelection) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "selectionChanged", f)
// }

// void currentChanged(const QModelIndex &, const QModelIndex &)
// func (this *QListView) InheritCurrentChanged(f func(current *qtcore.QModelIndex, previous *qtcore.QModelIndex) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "currentChanged", f)
// }

// QSize viewportSizeHint()
// func (this *QListView) InheritViewportSizeHint(f func() unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "viewportSizeHint", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QListView)=48
pub struct QListView {
  qbase: QAbstractItemView,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QListView_ITF interface {
//    QAbstractItemView_ITF
//    QListView_PTR() *QListView
//}
//func (ptr *QListView) QListView_PTR() *QListView { return ptr }

impl /*struct*/ QListView {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QListView {
    return QListView{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QListView {
//  type Target = QListViewBASE;
//
//  fn deref(&self) -> &QListViewBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QListViewBASE> for QListView {
//  fn as_ref(& self) -> & QListViewBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qlistview.h:54
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QListView {
  pub fn metaObject_0<RetType, T: QListView_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QListView_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QListView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QListView10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:81
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QListView(QWidget *)

/*
Creates a new QListView with the given parent to view a model. Use setModel() to set the model.
*/
// QListView(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QListView {
  pub fn QListView_0<T: QListView_QListView_0>(value: T) -> QListView {
    let rsthis = value.QListView_0();
    return rsthis;
    // return 1;
  }
}

pub trait QListView_QListView_0 {
  fn QListView_0(self) -> QListView;
}
// QListView(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QListView_QListView_0 for (usize) {
  fn QListView_0(self) -> QListView {
    // unsafe{_ZN9QListViewC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QListViewC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QListView{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:82
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QListView()

/*

*/
pub fn DeleteQListView(this :*mut QListView) {
    // let rv = qtrt::InvokeQtFunc6("_ZN9QListViewD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qlistview.h:84
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMovement(QListView::Movement)

/*

*/
impl /*struct*/ QListView {
  pub fn setMovement_0<RetType, T: QListView_setMovement_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMovement_0(self);
    // return 1;
  }
}
pub trait QListView_setMovement_0<RetType> {
  fn setMovement_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_setMovement_0<(/*void*/)> for (i32) {
  fn setMovement_0(self , rsthis: & QListView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QListView11setMovementENS_8MovementE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:85
// index:0
// Public Visibility=Default Availability=Available
// [4] QListView::Movement movement() const

/*

*/
impl /*struct*/ QListView {
  pub fn movement_0<RetType, T: QListView_movement_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.movement_0(self);
    // return 1;
  }
}
pub trait QListView_movement_0<RetType> {
  fn movement_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_movement_0<i32> for () {
  fn movement_0(self , rsthis: & QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QListView8movementEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:87
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFlow(QListView::Flow)

/*

*/
impl /*struct*/ QListView {
  pub fn setFlow_0<RetType, T: QListView_setFlow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFlow_0(self);
    // return 1;
  }
}
pub trait QListView_setFlow_0<RetType> {
  fn setFlow_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_setFlow_0<(/*void*/)> for (i32) {
  fn setFlow_0(self , rsthis: & QListView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QListView7setFlowENS_4FlowE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:88
// index:0
// Public Visibility=Default Availability=Available
// [4] QListView::Flow flow() const

/*

*/
impl /*struct*/ QListView {
  pub fn flow_0<RetType, T: QListView_flow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.flow_0(self);
    // return 1;
  }
}
pub trait QListView_flow_0<RetType> {
  fn flow_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_flow_0<i32> for () {
  fn flow_0(self , rsthis: & QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QListView4flowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:90
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWrapping(bool)

/*

*/
impl /*struct*/ QListView {
  pub fn setWrapping_0<RetType, T: QListView_setWrapping_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWrapping_0(self);
    // return 1;
  }
}
pub trait QListView_setWrapping_0<RetType> {
  fn setWrapping_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_setWrapping_0<(/*void*/)> for (bool) {
  fn setWrapping_0(self , rsthis: & QListView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QListView11setWrappingEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:91
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isWrapping() const

/*

*/
impl /*struct*/ QListView {
  pub fn isWrapping_0<RetType, T: QListView_isWrapping_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isWrapping_0(self);
    // return 1;
  }
}
pub trait QListView_isWrapping_0<RetType> {
  fn isWrapping_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_isWrapping_0<bool> for () {
  fn isWrapping_0(self , rsthis: & QListView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QListView10isWrappingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:93
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setResizeMode(QListView::ResizeMode)

/*

*/
impl /*struct*/ QListView {
  pub fn setResizeMode_0<RetType, T: QListView_setResizeMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setResizeMode_0(self);
    // return 1;
  }
}
pub trait QListView_setResizeMode_0<RetType> {
  fn setResizeMode_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_setResizeMode_0<(/*void*/)> for (i32) {
  fn setResizeMode_0(self , rsthis: & QListView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QListView13setResizeModeENS_10ResizeModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:94
// index:0
// Public Visibility=Default Availability=Available
// [4] QListView::ResizeMode resizeMode() const

/*

*/
impl /*struct*/ QListView {
  pub fn resizeMode_0<RetType, T: QListView_resizeMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeMode_0(self);
    // return 1;
  }
}
pub trait QListView_resizeMode_0<RetType> {
  fn resizeMode_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_resizeMode_0<i32> for () {
  fn resizeMode_0(self , rsthis: & QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QListView10resizeModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:96
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLayoutMode(QListView::LayoutMode)

/*

*/
impl /*struct*/ QListView {
  pub fn setLayoutMode_0<RetType, T: QListView_setLayoutMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLayoutMode_0(self);
    // return 1;
  }
}
pub trait QListView_setLayoutMode_0<RetType> {
  fn setLayoutMode_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_setLayoutMode_0<(/*void*/)> for (i32) {
  fn setLayoutMode_0(self , rsthis: & QListView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QListView13setLayoutModeENS_10LayoutModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:97
// index:0
// Public Visibility=Default Availability=Available
// [4] QListView::LayoutMode layoutMode() const

/*

*/
impl /*struct*/ QListView {
  pub fn layoutMode_0<RetType, T: QListView_layoutMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.layoutMode_0(self);
    // return 1;
  }
}
pub trait QListView_layoutMode_0<RetType> {
  fn layoutMode_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_layoutMode_0<i32> for () {
  fn layoutMode_0(self , rsthis: & QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QListView10layoutModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:99
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSpacing(int)

/*

*/
impl /*struct*/ QListView {
  pub fn setSpacing_0<RetType, T: QListView_setSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSpacing_0(self);
    // return 1;
  }
}
pub trait QListView_setSpacing_0<RetType> {
  fn setSpacing_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_setSpacing_0<(/*void*/)> for (i32) {
  fn setSpacing_0(self , rsthis: & QListView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QListView10setSpacingEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:100
// index:0
// Public Visibility=Default Availability=Available
// [4] int spacing() const

/*

*/
impl /*struct*/ QListView {
  pub fn spacing_0<RetType, T: QListView_spacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.spacing_0(self);
    // return 1;
  }
}
pub trait QListView_spacing_0<RetType> {
  fn spacing_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_spacing_0<i32> for () {
  fn spacing_0(self , rsthis: & QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QListView7spacingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:102
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setBatchSize(int)

/*

*/
impl /*struct*/ QListView {
  pub fn setBatchSize_0<RetType, T: QListView_setBatchSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBatchSize_0(self);
    // return 1;
  }
}
pub trait QListView_setBatchSize_0<RetType> {
  fn setBatchSize_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_setBatchSize_0<(/*void*/)> for (i32) {
  fn setBatchSize_0(self , rsthis: & QListView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QListView12setBatchSizeEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:103
// index:0
// Public Visibility=Default Availability=Available
// [4] int batchSize() const

/*

*/
impl /*struct*/ QListView {
  pub fn batchSize_0<RetType, T: QListView_batchSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.batchSize_0(self);
    // return 1;
  }
}
pub trait QListView_batchSize_0<RetType> {
  fn batchSize_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_batchSize_0<i32> for () {
  fn batchSize_0(self , rsthis: & QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QListView9batchSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:105
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setGridSize(const QSize &)

/*

*/
impl /*struct*/ QListView {
  pub fn setGridSize_0<RetType, T: QListView_setGridSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setGridSize_0(self);
    // return 1;
  }
}
pub trait QListView_setGridSize_0<RetType> {
  fn setGridSize_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_setGridSize_0<(/*void*/)> for (usize) {
  fn setGridSize_0(self , rsthis: & QListView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QListView11setGridSizeERK5QSize", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:106
// index:0
// Public Visibility=Default Availability=Available
// [8] QSize gridSize() const

/*

*/
impl /*struct*/ QListView {
  pub fn gridSize_0<RetType, T: QListView_gridSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.gridSize_0(self);
    // return 1;
  }
}
pub trait QListView_gridSize_0<RetType> {
  fn gridSize_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_gridSize_0<usize> for () {
  fn gridSize_0(self , rsthis: & QListView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QListView8gridSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:108
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setViewMode(QListView::ViewMode)

/*

*/
impl /*struct*/ QListView {
  pub fn setViewMode_0<RetType, T: QListView_setViewMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setViewMode_0(self);
    // return 1;
  }
}
pub trait QListView_setViewMode_0<RetType> {
  fn setViewMode_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_setViewMode_0<(/*void*/)> for (i32) {
  fn setViewMode_0(self , rsthis: & QListView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QListView11setViewModeENS_8ViewModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:109
// index:0
// Public Visibility=Default Availability=Available
// [4] QListView::ViewMode viewMode() const

/*

*/
impl /*struct*/ QListView {
  pub fn viewMode_0<RetType, T: QListView_viewMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.viewMode_0(self);
    // return 1;
  }
}
pub trait QListView_viewMode_0<RetType> {
  fn viewMode_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_viewMode_0<i32> for () {
  fn viewMode_0(self , rsthis: & QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QListView8viewModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:111
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clearPropertyFlags()

/*
Clears the QListView-specific property flags. See viewMode.

Properties inherited from QAbstractItemView are not covered by the property flags. Specifically, dragEnabled and acceptsDrops are computed by QListView when calling setMovement() or setViewMode().
*/
impl /*struct*/ QListView {
  pub fn clearPropertyFlags_0<RetType, T: QListView_clearPropertyFlags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clearPropertyFlags_0(self);
    // return 1;
  }
}
pub trait QListView_clearPropertyFlags_0<RetType> {
  fn clearPropertyFlags_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_clearPropertyFlags_0<(/*void*/)> for () {
  fn clearPropertyFlags_0(self , rsthis: & QListView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QListView18clearPropertyFlagsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:113
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isRowHidden(int) const

/*
Returns true if the row is hidden; otherwise returns false.
*/
impl /*struct*/ QListView {
  pub fn isRowHidden_0<RetType, T: QListView_isRowHidden_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isRowHidden_0(self);
    // return 1;
  }
}
pub trait QListView_isRowHidden_0<RetType> {
  fn isRowHidden_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_isRowHidden_0<bool> for (i32) {
  fn isRowHidden_0(self , rsthis: & QListView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QListView11isRowHiddenEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:114
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRowHidden(int, bool)

/*
If hide is true, the given row will be hidden; otherwise the row will be shown.

See also isRowHidden().
*/
impl /*struct*/ QListView {
  pub fn setRowHidden_0<RetType, T: QListView_setRowHidden_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRowHidden_0(self);
    // return 1;
  }
}
pub trait QListView_setRowHidden_0<RetType> {
  fn setRowHidden_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_setRowHidden_0<(/*void*/)> for (i32,bool) {
  fn setRowHidden_0(self , rsthis: & QListView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QListView12setRowHiddenEib", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:116
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setModelColumn(int)

/*

*/
impl /*struct*/ QListView {
  pub fn setModelColumn_0<RetType, T: QListView_setModelColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setModelColumn_0(self);
    // return 1;
  }
}
pub trait QListView_setModelColumn_0<RetType> {
  fn setModelColumn_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_setModelColumn_0<(/*void*/)> for (i32) {
  fn setModelColumn_0(self , rsthis: & QListView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QListView14setModelColumnEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:117
// index:0
// Public Visibility=Default Availability=Available
// [4] int modelColumn() const

/*

*/
impl /*struct*/ QListView {
  pub fn modelColumn_0<RetType, T: QListView_modelColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.modelColumn_0(self);
    // return 1;
  }
}
pub trait QListView_modelColumn_0<RetType> {
  fn modelColumn_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_modelColumn_0<i32> for () {
  fn modelColumn_0(self , rsthis: & QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QListView11modelColumnEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:119
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setUniformItemSizes(bool)

/*

*/
impl /*struct*/ QListView {
  pub fn setUniformItemSizes_0<RetType, T: QListView_setUniformItemSizes_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setUniformItemSizes_0(self);
    // return 1;
  }
}
pub trait QListView_setUniformItemSizes_0<RetType> {
  fn setUniformItemSizes_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_setUniformItemSizes_0<(/*void*/)> for (bool) {
  fn setUniformItemSizes_0(self , rsthis: & QListView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QListView19setUniformItemSizesEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:120
// index:0
// Public Visibility=Default Availability=Available
// [1] bool uniformItemSizes() const

/*

*/
impl /*struct*/ QListView {
  pub fn uniformItemSizes_0<RetType, T: QListView_uniformItemSizes_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.uniformItemSizes_0(self);
    // return 1;
  }
}
pub trait QListView_uniformItemSizes_0<RetType> {
  fn uniformItemSizes_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_uniformItemSizes_0<bool> for () {
  fn uniformItemSizes_0(self , rsthis: & QListView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QListView16uniformItemSizesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:122
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWordWrap(bool)

/*

*/
impl /*struct*/ QListView {
  pub fn setWordWrap_0<RetType, T: QListView_setWordWrap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWordWrap_0(self);
    // return 1;
  }
}
pub trait QListView_setWordWrap_0<RetType> {
  fn setWordWrap_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_setWordWrap_0<(/*void*/)> for (bool) {
  fn setWordWrap_0(self , rsthis: & QListView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QListView11setWordWrapEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:123
// index:0
// Public Visibility=Default Availability=Available
// [1] bool wordWrap() const

/*

*/
impl /*struct*/ QListView {
  pub fn wordWrap_0<RetType, T: QListView_wordWrap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.wordWrap_0(self);
    // return 1;
  }
}
pub trait QListView_wordWrap_0<RetType> {
  fn wordWrap_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_wordWrap_0<bool> for () {
  fn wordWrap_0(self , rsthis: & QListView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QListView8wordWrapEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:125
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSelectionRectVisible(bool)

/*

*/
impl /*struct*/ QListView {
  pub fn setSelectionRectVisible_0<RetType, T: QListView_setSelectionRectVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSelectionRectVisible_0(self);
    // return 1;
  }
}
pub trait QListView_setSelectionRectVisible_0<RetType> {
  fn setSelectionRectVisible_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_setSelectionRectVisible_0<(/*void*/)> for (bool) {
  fn setSelectionRectVisible_0(self , rsthis: & QListView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QListView23setSelectionRectVisibleEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:126
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isSelectionRectVisible() const

/*

*/
impl /*struct*/ QListView {
  pub fn isSelectionRectVisible_0<RetType, T: QListView_isSelectionRectVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isSelectionRectVisible_0(self);
    // return 1;
  }
}
pub trait QListView_isSelectionRectVisible_0<RetType> {
  fn isSelectionRectVisible_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_isSelectionRectVisible_0<bool> for () {
  fn isSelectionRectVisible_0(self , rsthis: & QListView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QListView22isSelectionRectVisibleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:128
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QRect visualRect(const QModelIndex &) const

/*
Reimplemented from QAbstractItemView::visualRect().
*/
impl /*struct*/ QListView {
  pub fn visualRect_0<RetType, T: QListView_visualRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.visualRect_0(self);
    // return 1;
  }
}
pub trait QListView_visualRect_0<RetType> {
  fn visualRect_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_visualRect_0<usize> for (usize) {
  fn visualRect_0(self , rsthis: & QListView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QListView10visualRectERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:129
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void scrollTo(const QModelIndex &, QAbstractItemView::ScrollHint)

/*
Reimplemented from QAbstractItemView::scrollTo().
*/
impl /*struct*/ QListView {
  pub fn scrollTo_0<RetType, T: QListView_scrollTo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scrollTo_0(self);
    // return 1;
  }
}
pub trait QListView_scrollTo_0<RetType> {
  fn scrollTo_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_scrollTo_0<(/*void*/)> for (usize,i32) {
  fn scrollTo_0(self , rsthis: & QListView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QListView8scrollToERK11QModelIndexN17QAbstractItemView10ScrollHintE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:130
// index:0
// Public virtual Visibility=Default Availability=Available
// [24] QModelIndex indexAt(const QPoint &) const

/*
Reimplemented from QAbstractItemView::indexAt().
*/
impl /*struct*/ QListView {
  pub fn indexAt_0<RetType, T: QListView_indexAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexAt_0(self);
    // return 1;
  }
}
pub trait QListView_indexAt_0<RetType> {
  fn indexAt_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_indexAt_0<usize> for (usize) {
  fn indexAt_0(self , rsthis: & QListView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QListView7indexAtERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:132
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void doItemsLayout()

/*

*/
impl /*struct*/ QListView {
  pub fn doItemsLayout_0<RetType, T: QListView_doItemsLayout_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.doItemsLayout_0(self);
    // return 1;
  }
}
pub trait QListView_doItemsLayout_0<RetType> {
  fn doItemsLayout_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_doItemsLayout_0<(/*void*/)> for () {
  fn doItemsLayout_0(self , rsthis: & QListView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QListView13doItemsLayoutEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:133
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void reset()

/*

*/
impl /*struct*/ QListView {
  pub fn reset_0<RetType, T: QListView_reset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.reset_0(self);
    // return 1;
  }
}
pub trait QListView_reset_0<RetType> {
  fn reset_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_reset_0<(/*void*/)> for () {
  fn reset_0(self , rsthis: & QListView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QListView5resetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:134
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setRootIndex(const QModelIndex &)

/*

*/
impl /*struct*/ QListView {
  pub fn setRootIndex_0<RetType, T: QListView_setRootIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRootIndex_0(self);
    // return 1;
  }
}
pub trait QListView_setRootIndex_0<RetType> {
  fn setRootIndex_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_setRootIndex_0<(/*void*/)> for (usize) {
  fn setRootIndex_0(self , rsthis: & QListView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QListView12setRootIndexERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:142
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QListView {
  pub fn event_0<RetType, T: QListView_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QListView_event_0<RetType> {
  fn event_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QListView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QListView5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:144
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void scrollContentsBy(int, int)

/*

*/
impl /*struct*/ QListView {
  pub fn scrollContentsBy_0<RetType, T: QListView_scrollContentsBy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scrollContentsBy_0(self);
    // return 1;
  }
}
pub trait QListView_scrollContentsBy_0<RetType> {
  fn scrollContentsBy_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_scrollContentsBy_0<(/*void*/)> for (i32,i32) {
  fn scrollContentsBy_0(self , rsthis: & QListView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QListView16scrollContentsByEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:146
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void resizeContents(int, int)

/*

*/
impl /*struct*/ QListView {
  pub fn resizeContents_0<RetType, T: QListView_resizeContents_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeContents_0(self);
    // return 1;
  }
}
pub trait QListView_resizeContents_0<RetType> {
  fn resizeContents_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_resizeContents_0<(/*void*/)> for (i32,i32) {
  fn resizeContents_0(self , rsthis: & QListView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QListView14resizeContentsEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:147
// index:0
// Protected Visibility=Default Availability=Available
// [8] QSize contentsSize() const

/*

*/
impl /*struct*/ QListView {
  pub fn contentsSize_0<RetType, T: QListView_contentsSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contentsSize_0(self);
    // return 1;
  }
}
pub trait QListView_contentsSize_0<RetType> {
  fn contentsSize_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_contentsSize_0<usize> for () {
  fn contentsSize_0(self , rsthis: & QListView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QListView12contentsSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:150
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void rowsInserted(const QModelIndex &, int, int)

/*
Reimplemented from QAbstractItemView::rowsInserted().
*/
impl /*struct*/ QListView {
  pub fn rowsInserted_0<RetType, T: QListView_rowsInserted_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowsInserted_0(self);
    // return 1;
  }
}
pub trait QListView_rowsInserted_0<RetType> {
  fn rowsInserted_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_rowsInserted_0<(/*void*/)> for (usize,i32,i32) {
  fn rowsInserted_0(self , rsthis: & QListView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QListView12rowsInsertedERK11QModelIndexii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:151
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void rowsAboutToBeRemoved(const QModelIndex &, int, int)

/*
Reimplemented from QAbstractItemView::rowsAboutToBeRemoved().
*/
impl /*struct*/ QListView {
  pub fn rowsAboutToBeRemoved_0<RetType, T: QListView_rowsAboutToBeRemoved_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowsAboutToBeRemoved_0(self);
    // return 1;
  }
}
pub trait QListView_rowsAboutToBeRemoved_0<RetType> {
  fn rowsAboutToBeRemoved_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_rowsAboutToBeRemoved_0<(/*void*/)> for (usize,i32,i32) {
  fn rowsAboutToBeRemoved_0(self , rsthis: & QListView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QListView20rowsAboutToBeRemovedERK11QModelIndexii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:153
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseMoveEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseMoveEvent().
*/
impl /*struct*/ QListView {
  pub fn mouseMoveEvent_0<RetType, T: QListView_mouseMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseMoveEvent_0(self);
    // return 1;
  }
}
pub trait QListView_mouseMoveEvent_0<RetType> {
  fn mouseMoveEvent_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_mouseMoveEvent_0<(/*void*/)> for (usize) {
  fn mouseMoveEvent_0(self , rsthis: & QListView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QListView14mouseMoveEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:154
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseReleaseEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseReleaseEvent().
*/
impl /*struct*/ QListView {
  pub fn mouseReleaseEvent_0<RetType, T: QListView_mouseReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QListView_mouseReleaseEvent_0<RetType> {
  fn mouseReleaseEvent_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_mouseReleaseEvent_0<(/*void*/)> for (usize) {
  fn mouseReleaseEvent_0(self , rsthis: & QListView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QListView17mouseReleaseEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:156
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void wheelEvent(QWheelEvent *)

/*
Reimplemented from QWidget::wheelEvent().
*/
impl /*struct*/ QListView {
  pub fn wheelEvent_0<RetType, T: QListView_wheelEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.wheelEvent_0(self);
    // return 1;
  }
}
pub trait QListView_wheelEvent_0<RetType> {
  fn wheelEvent_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_wheelEvent_0<(/*void*/)> for (usize) {
  fn wheelEvent_0(self , rsthis: & QListView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QListView10wheelEventEP11QWheelEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:159
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void timerEvent(QTimerEvent *)

/*
Reimplemented from QObject::timerEvent().
*/
impl /*struct*/ QListView {
  pub fn timerEvent_0<RetType, T: QListView_timerEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.timerEvent_0(self);
    // return 1;
  }
}
pub trait QListView_timerEvent_0<RetType> {
  fn timerEvent_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_timerEvent_0<(/*void*/)> for (usize) {
  fn timerEvent_0(self , rsthis: & QListView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QListView10timerEventEP11QTimerEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:160
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void resizeEvent(QResizeEvent *)

/*
Reimplemented from QWidget::resizeEvent().
*/
impl /*struct*/ QListView {
  pub fn resizeEvent_0<RetType, T: QListView_resizeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeEvent_0(self);
    // return 1;
  }
}
pub trait QListView_resizeEvent_0<RetType> {
  fn resizeEvent_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_resizeEvent_0<(/*void*/)> for (usize) {
  fn resizeEvent_0(self , rsthis: & QListView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QListView11resizeEventEP12QResizeEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:162
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dragMoveEvent(QDragMoveEvent *)

/*
Reimplemented from QWidget::dragMoveEvent().
*/
impl /*struct*/ QListView {
  pub fn dragMoveEvent_0<RetType, T: QListView_dragMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragMoveEvent_0(self);
    // return 1;
  }
}
pub trait QListView_dragMoveEvent_0<RetType> {
  fn dragMoveEvent_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_dragMoveEvent_0<(/*void*/)> for (usize) {
  fn dragMoveEvent_0(self , rsthis: & QListView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QListView13dragMoveEventEP14QDragMoveEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:163
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dragLeaveEvent(QDragLeaveEvent *)

/*
Reimplemented from QWidget::dragLeaveEvent().
*/
impl /*struct*/ QListView {
  pub fn dragLeaveEvent_0<RetType, T: QListView_dragLeaveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragLeaveEvent_0(self);
    // return 1;
  }
}
pub trait QListView_dragLeaveEvent_0<RetType> {
  fn dragLeaveEvent_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_dragLeaveEvent_0<(/*void*/)> for (usize) {
  fn dragLeaveEvent_0(self , rsthis: & QListView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QListView14dragLeaveEventEP15QDragLeaveEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:164
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dropEvent(QDropEvent *)

/*
Reimplemented from QWidget::dropEvent().
*/
impl /*struct*/ QListView {
  pub fn dropEvent_0<RetType, T: QListView_dropEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dropEvent_0(self);
    // return 1;
  }
}
pub trait QListView_dropEvent_0<RetType> {
  fn dropEvent_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_dropEvent_0<(/*void*/)> for (usize) {
  fn dropEvent_0(self , rsthis: & QListView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QListView9dropEventEP10QDropEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:165
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void startDrag(Qt::DropActions)

/*
Reimplemented from QAbstractItemView::startDrag().
*/
impl /*struct*/ QListView {
  pub fn startDrag_0<RetType, T: QListView_startDrag_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.startDrag_0(self);
    // return 1;
  }
}
pub trait QListView_startDrag_0<RetType> {
  fn startDrag_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_startDrag_0<(/*void*/)> for (i32) {
  fn startDrag_0(self , rsthis: & QListView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QListView9startDragE6QFlagsIN2Qt10DropActionEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:168
// index:0
// Protected virtual Visibility=Default Availability=Available
// [192] QStyleOptionViewItem viewOptions() const

/*
Reimplemented from QAbstractItemView::viewOptions().
*/
impl /*struct*/ QListView {
  pub fn viewOptions_0<RetType, T: QListView_viewOptions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.viewOptions_0(self);
    // return 1;
  }
}
pub trait QListView_viewOptions_0<RetType> {
  fn viewOptions_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_viewOptions_0<usize> for () {
  fn viewOptions_0(self , rsthis: & QListView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QListView11viewOptionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:169
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Reimplemented from QWidget::paintEvent().
*/
impl /*struct*/ QListView {
  pub fn paintEvent_0<RetType, T: QListView_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QListView_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QListView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QListView10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:171
// index:0
// Protected virtual Visibility=Default Availability=Available
// [4] int horizontalOffset() const

/*
Reimplemented from QAbstractItemView::horizontalOffset().
*/
impl /*struct*/ QListView {
  pub fn horizontalOffset_0<RetType, T: QListView_horizontalOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.horizontalOffset_0(self);
    // return 1;
  }
}
pub trait QListView_horizontalOffset_0<RetType> {
  fn horizontalOffset_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_horizontalOffset_0<i32> for () {
  fn horizontalOffset_0(self , rsthis: & QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QListView16horizontalOffsetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:172
// index:0
// Protected virtual Visibility=Default Availability=Available
// [4] int verticalOffset() const

/*
Reimplemented from QAbstractItemView::verticalOffset().
*/
impl /*struct*/ QListView {
  pub fn verticalOffset_0<RetType, T: QListView_verticalOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.verticalOffset_0(self);
    // return 1;
  }
}
pub trait QListView_verticalOffset_0<RetType> {
  fn verticalOffset_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_verticalOffset_0<i32> for () {
  fn verticalOffset_0(self , rsthis: & QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QListView14verticalOffsetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:173
// index:0
// Protected virtual Visibility=Default Availability=Available
// [24] QModelIndex moveCursor(QAbstractItemView::CursorAction, Qt::KeyboardModifiers)

/*
Reimplemented from QAbstractItemView::moveCursor().
*/
impl /*struct*/ QListView {
  pub fn moveCursor_0<RetType, T: QListView_moveCursor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveCursor_0(self);
    // return 1;
  }
}
pub trait QListView_moveCursor_0<RetType> {
  fn moveCursor_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_moveCursor_0<usize> for (i32,i32) {
  fn moveCursor_0(self , rsthis: & QListView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QListView10moveCursorEN17QAbstractItemView12CursorActionE6QFlagsIN2Qt16KeyboardModifierEE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:174
// index:0
// Protected Visibility=Default Availability=Available
// [16] QRect rectForIndex(const QModelIndex &) const

/*
Returns the rectangle of the item at position index in the model. The rectangle is in contents coordinates.

See also visualRect().
*/
impl /*struct*/ QListView {
  pub fn rectForIndex_0<RetType, T: QListView_rectForIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rectForIndex_0(self);
    // return 1;
  }
}
pub trait QListView_rectForIndex_0<RetType> {
  fn rectForIndex_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_rectForIndex_0<usize> for (usize) {
  fn rectForIndex_0(self , rsthis: & QListView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QListView12rectForIndexERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:175
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void setPositionForIndex(const QPoint &, const QModelIndex &)

/*
Sets the contents position of the item at index in the model to the given position. If the list view's movement mode is Static or its view mode is ListView, this function will have no effect.

This function was introduced in  Qt 4.1.
*/
impl /*struct*/ QListView {
  pub fn setPositionForIndex_0<RetType, T: QListView_setPositionForIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPositionForIndex_0(self);
    // return 1;
  }
}
pub trait QListView_setPositionForIndex_0<RetType> {
  fn setPositionForIndex_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_setPositionForIndex_0<(/*void*/)> for (usize,usize) {
  fn setPositionForIndex_0(self , rsthis: & QListView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QListView19setPositionForIndexERK6QPointRK11QModelIndex", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:177
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void setSelection(const QRect &, QItemSelectionModel::SelectionFlags)

/*
Reimplemented from QAbstractItemView::setSelection().
*/
impl /*struct*/ QListView {
  pub fn setSelection_0<RetType, T: QListView_setSelection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSelection_0(self);
    // return 1;
  }
}
pub trait QListView_setSelection_0<RetType> {
  fn setSelection_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_setSelection_0<(/*void*/)> for (usize,i32) {
  fn setSelection_0(self , rsthis: & QListView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QListView12setSelectionERK5QRect6QFlagsIN19QItemSelectionModel13SelectionFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:178
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] QRegion visualRegionForSelection(const QItemSelection &) const

/*
Reimplemented from QAbstractItemView::visualRegionForSelection().

Since 4.7, the returned region only contains rectangles intersecting (or included in) the viewport.
*/
impl /*struct*/ QListView {
  pub fn visualRegionForSelection_0<RetType, T: QListView_visualRegionForSelection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.visualRegionForSelection_0(self);
    // return 1;
  }
}
pub trait QListView_visualRegionForSelection_0<RetType> {
  fn visualRegionForSelection_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_visualRegionForSelection_0<usize> for (usize) {
  fn visualRegionForSelection_0(self , rsthis: & QListView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QListView24visualRegionForSelectionERK14QItemSelection", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:179
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] QModelIndexList selectedIndexes() const

/*
Reimplemented from QAbstractItemView::selectedIndexes().
*/
impl /*struct*/ QListView {
  pub fn selectedIndexes_0<RetType, T: QListView_selectedIndexes_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectedIndexes_0(self);
    // return 1;
  }
}
pub trait QListView_selectedIndexes_0<RetType> {
  fn selectedIndexes_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_selectedIndexes_0<usize> for () {
  fn selectedIndexes_0(self , rsthis: & QListView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QListView15selectedIndexesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:181
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void updateGeometries()

/*
Reimplemented from QAbstractItemView::updateGeometries().
*/
impl /*struct*/ QListView {
  pub fn updateGeometries_0<RetType, T: QListView_updateGeometries_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updateGeometries_0(self);
    // return 1;
  }
}
pub trait QListView_updateGeometries_0<RetType> {
  fn updateGeometries_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_updateGeometries_0<(/*void*/)> for () {
  fn updateGeometries_0(self , rsthis: & QListView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QListView16updateGeometriesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:183
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool isIndexHidden(const QModelIndex &) const

/*
Reimplemented from QAbstractItemView::isIndexHidden().
*/
impl /*struct*/ QListView {
  pub fn isIndexHidden_0<RetType, T: QListView_isIndexHidden_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isIndexHidden_0(self);
    // return 1;
  }
}
pub trait QListView_isIndexHidden_0<RetType> {
  fn isIndexHidden_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_isIndexHidden_0<bool> for (usize) {
  fn isIndexHidden_0(self , rsthis: & QListView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QListView13isIndexHiddenERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:185
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void selectionChanged(const QItemSelection &, const QItemSelection &)

/*
Reimplemented from QAbstractItemView::selectionChanged().
*/
impl /*struct*/ QListView {
  pub fn selectionChanged_0<RetType, T: QListView_selectionChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectionChanged_0(self);
    // return 1;
  }
}
pub trait QListView_selectionChanged_0<RetType> {
  fn selectionChanged_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_selectionChanged_0<(/*void*/)> for (usize,usize) {
  fn selectionChanged_0(self , rsthis: & QListView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QListView16selectionChangedERK14QItemSelectionS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:186
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void currentChanged(const QModelIndex &, const QModelIndex &)

/*
Reimplemented from QAbstractItemView::currentChanged().
*/
impl /*struct*/ QListView {
  pub fn currentChanged_0<RetType, T: QListView_currentChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentChanged_0(self);
    // return 1;
  }
}
pub trait QListView_currentChanged_0<RetType> {
  fn currentChanged_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_currentChanged_0<(/*void*/)> for (usize,usize) {
  fn currentChanged_0(self , rsthis: & QListView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QListView14currentChangedERK11QModelIndexS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlistview.h:188
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] QSize viewportSizeHint() const

/*
Reimplemented from QAbstractScrollArea::viewportSizeHint().

This function was introduced in  Qt 5.2.
*/
impl /*struct*/ QListView {
  pub fn viewportSizeHint_0<RetType, T: QListView_viewportSizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.viewportSizeHint_0(self);
    // return 1;
  }
}
pub trait QListView_viewportSizeHint_0<RetType> {
  fn viewportSizeHint_0(self , rsthis: & QListView) -> RetType;
}
impl<'a> /*trait*/ QListView_viewportSizeHint_0<usize> for () {
  fn viewportSizeHint_0(self , rsthis: & QListView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QListView16viewportSizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


/*

*/
pub type QListView__Movement = i32;
// The items cannot be moved by the user.
pub const QListView__Static :QListView__Movement = 0;
// The items can be moved freely by the user.
pub const QListView__Free :QListView__Movement = 1;
// The items snap to the specified grid when moved; see setGridSize().
pub const QListView__Snap :QListView__Movement = 2;
pub fn QListView_MovementItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QListView", val);
}
pub fn QListView_MovementItemName_s(val: i32) ->String {
  //var nilthis *QListView
  //return nilthis.MovementItemName(val);
  return QListView_MovementItemName(val);
}


/*

*/
pub type QListView__Flow = i32;
// The items are laid out in the view from the left to the right.
pub const QListView__LeftToRight :QListView__Flow = 0;
// The items are laid out in the view from the top to the bottom.
pub const QListView__TopToBottom :QListView__Flow = 1;
pub fn QListView_FlowItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QListView", val);
}
pub fn QListView_FlowItemName_s(val: i32) ->String {
  //var nilthis *QListView
  //return nilthis.FlowItemName(val);
  return QListView_FlowItemName(val);
}


/*

*/
pub type QListView__ResizeMode = i32;
// The items will only be laid out the first time the view is shown.
pub const QListView__Fixed :QListView__ResizeMode = 0;
// The items will be laid out every time the view is resized.
pub const QListView__Adjust :QListView__ResizeMode = 1;
pub fn QListView_ResizeModeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QListView", val);
}
pub fn QListView_ResizeModeItemName_s(val: i32) ->String {
  //var nilthis *QListView
  //return nilthis.ResizeModeItemName(val);
  return QListView_ResizeModeItemName(val);
}


/*


See also batchSize.

*/
pub type QListView__LayoutMode = i32;
// The items are laid out all at once.
pub const QListView__SinglePass :QListView__LayoutMode = 0;
// The items are laid out in batches of batchSize items.
pub const QListView__Batched :QListView__LayoutMode = 1;
pub fn QListView_LayoutModeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QListView", val);
}
pub fn QListView_LayoutModeItemName_s(val: i32) ->String {
  //var nilthis *QListView
  //return nilthis.LayoutModeItemName(val);
  return QListView_LayoutModeItemName(val);
}


/*

*/
pub type QListView__ViewMode = i32;
// The items are laid out using TopToBottom flow, with Small size and Static movement
pub const QListView__ListMode :QListView__ViewMode = 0;
// The items are laid out using LeftToRight flow, with Large size and Free movement
pub const QListView__IconMode :QListView__ViewMode = 1;
pub fn QListView_ViewModeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QListView", val);
}
pub fn QListView_ViewModeItemName_s(val: i32) ->String {
  //var nilthis *QListView
  //return nilthis.ViewModeItemName(val);
  return QListView_ViewModeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
