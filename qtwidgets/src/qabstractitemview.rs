

// mod ::widgets::QAbstractItemView
// package qtwidgets
// /usr/include/qt/QtWidgets/qabstractitemview.h
// #include <qabstractitemview.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 44
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

// void rowsInserted(const QModelIndex &, int, int)
// func (this *QAbstractItemView) InheritRowsInserted(f func(parent *qtcore.QModelIndex, start int, end_ int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "rowsInserted", f)
// }

// void rowsAboutToBeRemoved(const QModelIndex &, int, int)
// func (this *QAbstractItemView) InheritRowsAboutToBeRemoved(f func(parent *qtcore.QModelIndex, start int, end_ int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "rowsAboutToBeRemoved", f)
// }

// void selectionChanged(const QItemSelection &, const QItemSelection &)
// func (this *QAbstractItemView) InheritSelectionChanged(f func(selected *qtcore.QItemSelection, deselected *qtcore.QItemSelection) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "selectionChanged", f)
// }

// void currentChanged(const QModelIndex &, const QModelIndex &)
// func (this *QAbstractItemView) InheritCurrentChanged(f func(current *qtcore.QModelIndex, previous *qtcore.QModelIndex) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "currentChanged", f)
// }

// void updateEditorData()
// func (this *QAbstractItemView) InheritUpdateEditorData(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "updateEditorData", f)
// }

// void updateEditorGeometries()
// func (this *QAbstractItemView) InheritUpdateEditorGeometries(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "updateEditorGeometries", f)
// }

// void updateGeometries()
// func (this *QAbstractItemView) InheritUpdateGeometries(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "updateGeometries", f)
// }

// void verticalScrollbarAction(int)
// func (this *QAbstractItemView) InheritVerticalScrollbarAction(f func(action int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "verticalScrollbarAction", f)
// }

// void horizontalScrollbarAction(int)
// func (this *QAbstractItemView) InheritHorizontalScrollbarAction(f func(action int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "horizontalScrollbarAction", f)
// }

// void verticalScrollbarValueChanged(int)
// func (this *QAbstractItemView) InheritVerticalScrollbarValueChanged(f func(value int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "verticalScrollbarValueChanged", f)
// }

// void horizontalScrollbarValueChanged(int)
// func (this *QAbstractItemView) InheritHorizontalScrollbarValueChanged(f func(value int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "horizontalScrollbarValueChanged", f)
// }

// void closeEditor(QWidget *, QAbstractItemDelegate::EndEditHint)
// func (this *QAbstractItemView) InheritCloseEditor(f func(editor *QWidget/*777 QWidget **/, hint int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "closeEditor", f)
// }

// void commitData(QWidget *)
// func (this *QAbstractItemView) InheritCommitData(f func(editor *QWidget/*777 QWidget **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "commitData", f)
// }

// void editorDestroyed(QObject *)
// func (this *QAbstractItemView) InheritEditorDestroyed(f func(editor *qtcore.QObject/*777 QObject **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "editorDestroyed", f)
// }

// void setHorizontalStepsPerItem(int)
// func (this *QAbstractItemView) InheritSetHorizontalStepsPerItem(f func(steps int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "setHorizontalStepsPerItem", f)
// }

// int horizontalStepsPerItem()
// func (this *QAbstractItemView) InheritHorizontalStepsPerItem(f func() int) {
//  qtrt.SetAllInheritCallback(this, "horizontalStepsPerItem", f)
// }

// void setVerticalStepsPerItem(int)
// func (this *QAbstractItemView) InheritSetVerticalStepsPerItem(f func(steps int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "setVerticalStepsPerItem", f)
// }

// int verticalStepsPerItem()
// func (this *QAbstractItemView) InheritVerticalStepsPerItem(f func() int) {
//  qtrt.SetAllInheritCallback(this, "verticalStepsPerItem", f)
// }

// QModelIndex moveCursor(QAbstractItemView::CursorAction, Qt::KeyboardModifiers)
// func (this *QAbstractItemView) InheritMoveCursor(f func(cursorAction int, modifiers int) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "moveCursor", f)
// }

// int horizontalOffset()
// func (this *QAbstractItemView) InheritHorizontalOffset(f func() int) {
//  qtrt.SetAllInheritCallback(this, "horizontalOffset", f)
// }

// int verticalOffset()
// func (this *QAbstractItemView) InheritVerticalOffset(f func() int) {
//  qtrt.SetAllInheritCallback(this, "verticalOffset", f)
// }

// bool isIndexHidden(const QModelIndex &)
// func (this *QAbstractItemView) InheritIsIndexHidden(f func(index *qtcore.QModelIndex) bool) {
//  qtrt.SetAllInheritCallback(this, "isIndexHidden", f)
// }

// void setSelection(const QRect &, QItemSelectionModel::SelectionFlags)
// func (this *QAbstractItemView) InheritSetSelection(f func(rect *qtcore.QRect, command int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "setSelection", f)
// }

// QRegion visualRegionForSelection(const QItemSelection &)
// func (this *QAbstractItemView) InheritVisualRegionForSelection(f func(selection *qtcore.QItemSelection) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "visualRegionForSelection", f)
// }

// QModelIndexList selectedIndexes()
// func (this *QAbstractItemView) InheritSelectedIndexes(f func() *qtcore.QModelIndexList/*9999*/) {
//  qtrt.SetAllInheritCallback(this, "selectedIndexes", f)
// }

// bool edit(const QModelIndex &, QAbstractItemView::EditTrigger, QEvent *)
// func (this *QAbstractItemView) InheritEdit(f func(index *qtcore.QModelIndex, trigger int, event *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "edit", f)
// }

// QItemSelectionModel::SelectionFlags selectionCommand(const QModelIndex &, const QEvent *)
// func (this *QAbstractItemView) InheritSelectionCommand(f func(index *qtcore.QModelIndex, event *qtcore.QEvent/*777 const QEvent **/) int) {
//  qtrt.SetAllInheritCallback(this, "selectionCommand", f)
// }

// void startDrag(Qt::DropActions)
// func (this *QAbstractItemView) InheritStartDrag(f func(supportedActions int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "startDrag", f)
// }

// QStyleOptionViewItem viewOptions()
// func (this *QAbstractItemView) InheritViewOptions(f func() unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "viewOptions", f)
// }

// QAbstractItemView::State state()
// func (this *QAbstractItemView) InheritState(f func() int) {
//  qtrt.SetAllInheritCallback(this, "state", f)
// }

// void setState(QAbstractItemView::State)
// func (this *QAbstractItemView) InheritSetState(f func(state int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "setState", f)
// }

// void scheduleDelayedItemsLayout()
// func (this *QAbstractItemView) InheritScheduleDelayedItemsLayout(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "scheduleDelayedItemsLayout", f)
// }

// void executeDelayedItemsLayout()
// func (this *QAbstractItemView) InheritExecuteDelayedItemsLayout(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "executeDelayedItemsLayout", f)
// }

// void setDirtyRegion(const QRegion &)
// func (this *QAbstractItemView) InheritSetDirtyRegion(f func(region *qtgui.QRegion) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "setDirtyRegion", f)
// }

// void scrollDirtyRegion(int, int)
// func (this *QAbstractItemView) InheritScrollDirtyRegion(f func(dx int, dy int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "scrollDirtyRegion", f)
// }

// QPoint dirtyRegionOffset()
// func (this *QAbstractItemView) InheritDirtyRegionOffset(f func() unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "dirtyRegionOffset", f)
// }

// void startAutoScroll()
// func (this *QAbstractItemView) InheritStartAutoScroll(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "startAutoScroll", f)
// }

// void stopAutoScroll()
// func (this *QAbstractItemView) InheritStopAutoScroll(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "stopAutoScroll", f)
// }

// void doAutoScroll()
// func (this *QAbstractItemView) InheritDoAutoScroll(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "doAutoScroll", f)
// }

// bool focusNextPrevChild(bool)
// func (this *QAbstractItemView) InheritFocusNextPrevChild(f func(next bool) bool) {
//  qtrt.SetAllInheritCallback(this, "focusNextPrevChild", f)
// }

// bool event(QEvent *)
// func (this *QAbstractItemView) InheritEvent(f func(event *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// bool viewportEvent(QEvent *)
// func (this *QAbstractItemView) InheritViewportEvent(f func(event *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "viewportEvent", f)
// }

// void mousePressEvent(QMouseEvent *)
// func (this *QAbstractItemView) InheritMousePressEvent(f func(event *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mousePressEvent", f)
// }

// void mouseMoveEvent(QMouseEvent *)
// func (this *QAbstractItemView) InheritMouseMoveEvent(f func(event *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseMoveEvent", f)
// }

// void mouseReleaseEvent(QMouseEvent *)
// func (this *QAbstractItemView) InheritMouseReleaseEvent(f func(event *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseReleaseEvent", f)
// }

// void mouseDoubleClickEvent(QMouseEvent *)
// func (this *QAbstractItemView) InheritMouseDoubleClickEvent(f func(event *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseDoubleClickEvent", f)
// }

// void dragEnterEvent(QDragEnterEvent *)
// func (this *QAbstractItemView) InheritDragEnterEvent(f func(event *qtgui.QDragEnterEvent/*777 QDragEnterEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dragEnterEvent", f)
// }

// void dragMoveEvent(QDragMoveEvent *)
// func (this *QAbstractItemView) InheritDragMoveEvent(f func(event *qtgui.QDragMoveEvent/*777 QDragMoveEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dragMoveEvent", f)
// }

// void dragLeaveEvent(QDragLeaveEvent *)
// func (this *QAbstractItemView) InheritDragLeaveEvent(f func(event *qtgui.QDragLeaveEvent/*777 QDragLeaveEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dragLeaveEvent", f)
// }

// void dropEvent(QDropEvent *)
// func (this *QAbstractItemView) InheritDropEvent(f func(event *qtgui.QDropEvent/*777 QDropEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dropEvent", f)
// }

// void focusInEvent(QFocusEvent *)
// func (this *QAbstractItemView) InheritFocusInEvent(f func(event *qtgui.QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusInEvent", f)
// }

// void focusOutEvent(QFocusEvent *)
// func (this *QAbstractItemView) InheritFocusOutEvent(f func(event *qtgui.QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusOutEvent", f)
// }

// void keyPressEvent(QKeyEvent *)
// func (this *QAbstractItemView) InheritKeyPressEvent(f func(event *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyPressEvent", f)
// }

// void resizeEvent(QResizeEvent *)
// func (this *QAbstractItemView) InheritResizeEvent(f func(event *qtgui.QResizeEvent/*777 QResizeEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "resizeEvent", f)
// }

// void timerEvent(QTimerEvent *)
// func (this *QAbstractItemView) InheritTimerEvent(f func(event *qtcore.QTimerEvent/*777 QTimerEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "timerEvent", f)
// }

// void inputMethodEvent(QInputMethodEvent *)
// func (this *QAbstractItemView) InheritInputMethodEvent(f func(event *qtgui.QInputMethodEvent/*777 QInputMethodEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "inputMethodEvent", f)
// }

// QAbstractItemView::DropIndicatorPosition dropIndicatorPosition()
// func (this *QAbstractItemView) InheritDropIndicatorPosition(f func() int) {
//  qtrt.SetAllInheritCallback(this, "dropIndicatorPosition", f)
// }

// QSize viewportSizeHint()
// func (this *QAbstractItemView) InheritViewportSizeHint(f func() unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "viewportSizeHint", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QAbstractItemView)=48
pub struct QAbstractItemView {
  qbase: QAbstractScrollArea,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAbstractItemView_ITF interface {
//    QAbstractScrollArea_ITF
//    QAbstractItemView_PTR() *QAbstractItemView
//}
//func (ptr *QAbstractItemView) QAbstractItemView_PTR() *QAbstractItemView { return ptr }

impl /*struct*/ QAbstractItemView {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAbstractItemView {
    return QAbstractItemView{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAbstractItemView {
//  type Target = QAbstractItemViewBASE;
//
//  fn deref(&self) -> &QAbstractItemViewBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAbstractItemViewBASE> for QAbstractItemView {
//  fn as_ref(& self) -> & QAbstractItemViewBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qabstractitemview.h:63
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn metaObject_0<RetType, T: QAbstractItemView_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QAbstractItemView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:127
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QAbstractItemView(QWidget *)

/*
Constructs an abstract item view with the given parent.
*/
// QAbstractItemView(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QAbstractItemView {
  pub fn QAbstractItemView_0<T: QAbstractItemView_QAbstractItemView_0>(value: T) -> QAbstractItemView {
    let rsthis = value.QAbstractItemView_0();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractItemView_QAbstractItemView_0 {
  fn QAbstractItemView_0(self) -> QAbstractItemView;
}
// QAbstractItemView(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QAbstractItemView_QAbstractItemView_0 for (usize) {
  fn QAbstractItemView_0(self) -> QAbstractItemView {
    // unsafe{_ZN17QAbstractItemViewC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN17QAbstractItemViewC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAbstractItemView{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:128
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QAbstractItemView()

/*

*/
pub fn DeleteQAbstractItemView(this :*mut QAbstractItemView) {
    // let rv = qtrt::InvokeQtFunc6("_ZN17QAbstractItemViewD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qabstractitemview.h:130
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setModel(QAbstractItemModel *)

/*
Sets the model for the view to present.

This function will create and set a new selection model, replacing any model that was previously set with setSelectionModel(). However, the old selection model will not be deleted as it may be shared between several views. We recommend that you delete the old selection model if it is no longer required. This is done with the following code:


  QItemSelectionModel *m = view->selectionModel();
  view->setModel(new model);
  delete m;



If both the old model and the old selection model do not have parents, or if their parents are long-lived objects, it may be preferable to call their deleteLater() functions to explicitly delete them.

The view does not take ownership of the model unless it is the model's parent object because the model may be shared between many different views.

See also model(), selectionModel(), and setSelectionModel().
*/
impl /*struct*/ QAbstractItemView {
  pub fn setModel_0<RetType, T: QAbstractItemView_setModel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setModel_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_setModel_0<RetType> {
  fn setModel_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_setModel_0<(/*void*/)> for (usize) {
  fn setModel_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView8setModelEP18QAbstractItemModel", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:131
// index:0
// Public Visibility=Default Availability=Available
// [8] QAbstractItemModel * model() const

/*
Returns the model that this view is presenting.

See also setModel().
*/
impl /*struct*/ QAbstractItemView {
  pub fn model_0<RetType, T: QAbstractItemView_model_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.model_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_model_0<RetType> {
  fn model_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_model_0<usize> for () {
  fn model_0(self , rsthis: & QAbstractItemView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView5modelEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:133
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setSelectionModel(QItemSelectionModel *)

/*
Sets the current selection model to the given selectionModel.

Note that, if you call setModel() after this function, the given selectionModel will be replaced by one created by the view.

Note: It is up to the application to delete the old selection model if it is no longer needed; i.e., if it is not being used by other views. This will happen automatically when its parent object is deleted. However, if it does not have a parent, or if the parent is a long-lived object, it may be preferable to call its deleteLater() function to explicitly delete it.

See also selectionModel(), setModel(), and clearSelection().
*/
impl /*struct*/ QAbstractItemView {
  pub fn setSelectionModel_0<RetType, T: QAbstractItemView_setSelectionModel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSelectionModel_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_setSelectionModel_0<RetType> {
  fn setSelectionModel_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_setSelectionModel_0<(/*void*/)> for (usize) {
  fn setSelectionModel_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView17setSelectionModelEP19QItemSelectionModel", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:134
// index:0
// Public Visibility=Default Availability=Available
// [8] QItemSelectionModel * selectionModel() const

/*
Returns the current selection model.

See also setSelectionModel() and selectedIndexes().
*/
impl /*struct*/ QAbstractItemView {
  pub fn selectionModel_0<RetType, T: QAbstractItemView_selectionModel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectionModel_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_selectionModel_0<RetType> {
  fn selectionModel_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_selectionModel_0<usize> for () {
  fn selectionModel_0(self , rsthis: & QAbstractItemView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView14selectionModelEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:136
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setItemDelegate(QAbstractItemDelegate *)

/*
Sets the item delegate for this view and its model to delegate. This is useful if you want complete control over the editing and display of items.

Any existing delegate will be removed, but not deleted. QAbstractItemView does not take ownership of delegate.

Warning: You should not share the same instance of a delegate between views. Doing so can cause incorrect or unintuitive editing behavior since each view connected to a given delegate may receive the closeEditor() signal, and attempt to access, modify or close an editor that has already been closed.

See also itemDelegate().
*/
impl /*struct*/ QAbstractItemView {
  pub fn setItemDelegate_0<RetType, T: QAbstractItemView_setItemDelegate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setItemDelegate_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_setItemDelegate_0<RetType> {
  fn setItemDelegate_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_setItemDelegate_0<(/*void*/)> for (usize) {
  fn setItemDelegate_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView15setItemDelegateEP21QAbstractItemDelegate", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:137
// index:0
// Public Visibility=Default Availability=Available
// [8] QAbstractItemDelegate * itemDelegate() const

/*
Returns the item delegate used by this view and model. This is either one set with setItemDelegate(), or the default one.

See also setItemDelegate().
*/
impl /*struct*/ QAbstractItemView {
  pub fn itemDelegate_0<RetType, T: QAbstractItemView_itemDelegate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemDelegate_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_itemDelegate_0<RetType> {
  fn itemDelegate_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_itemDelegate_0<usize> for () {
  fn itemDelegate_0(self , rsthis: & QAbstractItemView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView12itemDelegateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:226
// index:1
// Public Visibility=Default Availability=Available
// [8] QAbstractItemDelegate * itemDelegate(const QModelIndex &) const

/*
Returns the item delegate used by this view and model. This is either one set with setItemDelegate(), or the default one.

See also setItemDelegate().
*/
impl /*struct*/ QAbstractItemView {
  pub fn itemDelegate_1<RetType, T: QAbstractItemView_itemDelegate_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemDelegate_1(self);
    // return 1;
  }
}
pub trait QAbstractItemView_itemDelegate_1<RetType> {
  fn itemDelegate_1(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_itemDelegate_1<usize> for (usize) {
  fn itemDelegate_1(self , rsthis: & QAbstractItemView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView12itemDelegateERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:139
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSelectionMode(QAbstractItemView::SelectionMode)

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn setSelectionMode_0<RetType, T: QAbstractItemView_setSelectionMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSelectionMode_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_setSelectionMode_0<RetType> {
  fn setSelectionMode_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_setSelectionMode_0<(/*void*/)> for (i32) {
  fn setSelectionMode_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView16setSelectionModeENS_13SelectionModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:140
// index:0
// Public Visibility=Default Availability=Available
// [4] QAbstractItemView::SelectionMode selectionMode() const

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn selectionMode_0<RetType, T: QAbstractItemView_selectionMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectionMode_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_selectionMode_0<RetType> {
  fn selectionMode_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_selectionMode_0<i32> for () {
  fn selectionMode_0(self , rsthis: & QAbstractItemView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView13selectionModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:142
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSelectionBehavior(QAbstractItemView::SelectionBehavior)

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn setSelectionBehavior_0<RetType, T: QAbstractItemView_setSelectionBehavior_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSelectionBehavior_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_setSelectionBehavior_0<RetType> {
  fn setSelectionBehavior_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_setSelectionBehavior_0<(/*void*/)> for (i32) {
  fn setSelectionBehavior_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView20setSelectionBehaviorENS_17SelectionBehaviorE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:143
// index:0
// Public Visibility=Default Availability=Available
// [4] QAbstractItemView::SelectionBehavior selectionBehavior() const

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn selectionBehavior_0<RetType, T: QAbstractItemView_selectionBehavior_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectionBehavior_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_selectionBehavior_0<RetType> {
  fn selectionBehavior_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_selectionBehavior_0<i32> for () {
  fn selectionBehavior_0(self , rsthis: & QAbstractItemView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView17selectionBehaviorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:145
// index:0
// Public Visibility=Default Availability=Available
// [24] QModelIndex currentIndex() const

/*
Returns the model index of the current item.

See also setCurrentIndex().
*/
impl /*struct*/ QAbstractItemView {
  pub fn currentIndex_0<RetType, T: QAbstractItemView_currentIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentIndex_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_currentIndex_0<RetType> {
  fn currentIndex_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_currentIndex_0<usize> for () {
  fn currentIndex_0(self , rsthis: & QAbstractItemView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView12currentIndexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:146
// index:0
// Public Visibility=Default Availability=Available
// [24] QModelIndex rootIndex() const

/*
Returns the model index of the model's root item. The root item is the parent item to the view's toplevel items. The root can be invalid.

See also setRootIndex().
*/
impl /*struct*/ QAbstractItemView {
  pub fn rootIndex_0<RetType, T: QAbstractItemView_rootIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rootIndex_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_rootIndex_0<RetType> {
  fn rootIndex_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_rootIndex_0<usize> for () {
  fn rootIndex_0(self , rsthis: & QAbstractItemView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView9rootIndexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:148
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setEditTriggers(QAbstractItemView::EditTriggers)

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn setEditTriggers_0<RetType, T: QAbstractItemView_setEditTriggers_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setEditTriggers_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_setEditTriggers_0<RetType> {
  fn setEditTriggers_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_setEditTriggers_0<(/*void*/)> for (i32) {
  fn setEditTriggers_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView15setEditTriggersE6QFlagsINS_11EditTriggerEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:149
// index:0
// Public Visibility=Default Availability=Available
// [4] QAbstractItemView::EditTriggers editTriggers() const

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn editTriggers_0<RetType, T: QAbstractItemView_editTriggers_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.editTriggers_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_editTriggers_0<RetType> {
  fn editTriggers_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_editTriggers_0<i32> for () {
  fn editTriggers_0(self , rsthis: & QAbstractItemView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView12editTriggersEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:151
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setVerticalScrollMode(QAbstractItemView::ScrollMode)

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn setVerticalScrollMode_0<RetType, T: QAbstractItemView_setVerticalScrollMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVerticalScrollMode_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_setVerticalScrollMode_0<RetType> {
  fn setVerticalScrollMode_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_setVerticalScrollMode_0<(/*void*/)> for (i32) {
  fn setVerticalScrollMode_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView21setVerticalScrollModeENS_10ScrollModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:152
// index:0
// Public Visibility=Default Availability=Available
// [4] QAbstractItemView::ScrollMode verticalScrollMode() const

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn verticalScrollMode_0<RetType, T: QAbstractItemView_verticalScrollMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.verticalScrollMode_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_verticalScrollMode_0<RetType> {
  fn verticalScrollMode_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_verticalScrollMode_0<i32> for () {
  fn verticalScrollMode_0(self , rsthis: & QAbstractItemView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView18verticalScrollModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:153
// index:0
// Public Visibility=Default Availability=Available
// [-2] void resetVerticalScrollMode()

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn resetVerticalScrollMode_0<RetType, T: QAbstractItemView_resetVerticalScrollMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resetVerticalScrollMode_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_resetVerticalScrollMode_0<RetType> {
  fn resetVerticalScrollMode_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_resetVerticalScrollMode_0<(/*void*/)> for () {
  fn resetVerticalScrollMode_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView23resetVerticalScrollModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:155
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setHorizontalScrollMode(QAbstractItemView::ScrollMode)

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn setHorizontalScrollMode_0<RetType, T: QAbstractItemView_setHorizontalScrollMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHorizontalScrollMode_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_setHorizontalScrollMode_0<RetType> {
  fn setHorizontalScrollMode_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_setHorizontalScrollMode_0<(/*void*/)> for (i32) {
  fn setHorizontalScrollMode_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView23setHorizontalScrollModeENS_10ScrollModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:156
// index:0
// Public Visibility=Default Availability=Available
// [4] QAbstractItemView::ScrollMode horizontalScrollMode() const

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn horizontalScrollMode_0<RetType, T: QAbstractItemView_horizontalScrollMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.horizontalScrollMode_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_horizontalScrollMode_0<RetType> {
  fn horizontalScrollMode_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_horizontalScrollMode_0<i32> for () {
  fn horizontalScrollMode_0(self , rsthis: & QAbstractItemView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView20horizontalScrollModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:157
// index:0
// Public Visibility=Default Availability=Available
// [-2] void resetHorizontalScrollMode()

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn resetHorizontalScrollMode_0<RetType, T: QAbstractItemView_resetHorizontalScrollMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resetHorizontalScrollMode_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_resetHorizontalScrollMode_0<RetType> {
  fn resetHorizontalScrollMode_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_resetHorizontalScrollMode_0<(/*void*/)> for () {
  fn resetHorizontalScrollMode_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView25resetHorizontalScrollModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:159
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAutoScroll(bool)

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn setAutoScroll_0<RetType, T: QAbstractItemView_setAutoScroll_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAutoScroll_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_setAutoScroll_0<RetType> {
  fn setAutoScroll_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_setAutoScroll_0<(/*void*/)> for (bool) {
  fn setAutoScroll_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView13setAutoScrollEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:160
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasAutoScroll() const

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn hasAutoScroll_0<RetType, T: QAbstractItemView_hasAutoScroll_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasAutoScroll_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_hasAutoScroll_0<RetType> {
  fn hasAutoScroll_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_hasAutoScroll_0<bool> for () {
  fn hasAutoScroll_0(self , rsthis: & QAbstractItemView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView13hasAutoScrollEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:162
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAutoScrollMargin(int)

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn setAutoScrollMargin_0<RetType, T: QAbstractItemView_setAutoScrollMargin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAutoScrollMargin_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_setAutoScrollMargin_0<RetType> {
  fn setAutoScrollMargin_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_setAutoScrollMargin_0<(/*void*/)> for (i32) {
  fn setAutoScrollMargin_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView19setAutoScrollMarginEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:163
// index:0
// Public Visibility=Default Availability=Available
// [4] int autoScrollMargin() const

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn autoScrollMargin_0<RetType, T: QAbstractItemView_autoScrollMargin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.autoScrollMargin_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_autoScrollMargin_0<RetType> {
  fn autoScrollMargin_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_autoScrollMargin_0<i32> for () {
  fn autoScrollMargin_0(self , rsthis: & QAbstractItemView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView16autoScrollMarginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:165
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTabKeyNavigation(bool)

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn setTabKeyNavigation_0<RetType, T: QAbstractItemView_setTabKeyNavigation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTabKeyNavigation_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_setTabKeyNavigation_0<RetType> {
  fn setTabKeyNavigation_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_setTabKeyNavigation_0<(/*void*/)> for (bool) {
  fn setTabKeyNavigation_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView19setTabKeyNavigationEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:166
// index:0
// Public Visibility=Default Availability=Available
// [1] bool tabKeyNavigation() const

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn tabKeyNavigation_0<RetType, T: QAbstractItemView_tabKeyNavigation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabKeyNavigation_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_tabKeyNavigation_0<RetType> {
  fn tabKeyNavigation_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_tabKeyNavigation_0<bool> for () {
  fn tabKeyNavigation_0(self , rsthis: & QAbstractItemView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView16tabKeyNavigationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:169
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDropIndicatorShown(bool)

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn setDropIndicatorShown_0<RetType, T: QAbstractItemView_setDropIndicatorShown_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDropIndicatorShown_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_setDropIndicatorShown_0<RetType> {
  fn setDropIndicatorShown_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_setDropIndicatorShown_0<(/*void*/)> for (bool) {
  fn setDropIndicatorShown_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView21setDropIndicatorShownEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:170
// index:0
// Public Visibility=Default Availability=Available
// [1] bool showDropIndicator() const

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn showDropIndicator_0<RetType, T: QAbstractItemView_showDropIndicator_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showDropIndicator_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_showDropIndicator_0<RetType> {
  fn showDropIndicator_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_showDropIndicator_0<bool> for () {
  fn showDropIndicator_0(self , rsthis: & QAbstractItemView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView17showDropIndicatorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:172
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDragEnabled(bool)

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn setDragEnabled_0<RetType, T: QAbstractItemView_setDragEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDragEnabled_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_setDragEnabled_0<RetType> {
  fn setDragEnabled_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_setDragEnabled_0<(/*void*/)> for (bool) {
  fn setDragEnabled_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView14setDragEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:173
// index:0
// Public Visibility=Default Availability=Available
// [1] bool dragEnabled() const

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn dragEnabled_0<RetType, T: QAbstractItemView_dragEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragEnabled_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_dragEnabled_0<RetType> {
  fn dragEnabled_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_dragEnabled_0<bool> for () {
  fn dragEnabled_0(self , rsthis: & QAbstractItemView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView11dragEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:175
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDragDropOverwriteMode(bool)

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn setDragDropOverwriteMode_0<RetType, T: QAbstractItemView_setDragDropOverwriteMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDragDropOverwriteMode_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_setDragDropOverwriteMode_0<RetType> {
  fn setDragDropOverwriteMode_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_setDragDropOverwriteMode_0<(/*void*/)> for (bool) {
  fn setDragDropOverwriteMode_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView24setDragDropOverwriteModeEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:176
// index:0
// Public Visibility=Default Availability=Available
// [1] bool dragDropOverwriteMode() const

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn dragDropOverwriteMode_0<RetType, T: QAbstractItemView_dragDropOverwriteMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragDropOverwriteMode_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_dragDropOverwriteMode_0<RetType> {
  fn dragDropOverwriteMode_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_dragDropOverwriteMode_0<bool> for () {
  fn dragDropOverwriteMode_0(self , rsthis: & QAbstractItemView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView21dragDropOverwriteModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:187
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDragDropMode(QAbstractItemView::DragDropMode)

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn setDragDropMode_0<RetType, T: QAbstractItemView_setDragDropMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDragDropMode_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_setDragDropMode_0<RetType> {
  fn setDragDropMode_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_setDragDropMode_0<(/*void*/)> for (i32) {
  fn setDragDropMode_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView15setDragDropModeENS_12DragDropModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:188
// index:0
// Public Visibility=Default Availability=Available
// [4] QAbstractItemView::DragDropMode dragDropMode() const

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn dragDropMode_0<RetType, T: QAbstractItemView_dragDropMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragDropMode_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_dragDropMode_0<RetType> {
  fn dragDropMode_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_dragDropMode_0<i32> for () {
  fn dragDropMode_0(self , rsthis: & QAbstractItemView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView12dragDropModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:190
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDefaultDropAction(Qt::DropAction)

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn setDefaultDropAction_0<RetType, T: QAbstractItemView_setDefaultDropAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDefaultDropAction_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_setDefaultDropAction_0<RetType> {
  fn setDefaultDropAction_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_setDefaultDropAction_0<(/*void*/)> for (i32) {
  fn setDefaultDropAction_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView20setDefaultDropActionEN2Qt10DropActionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:191
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::DropAction defaultDropAction() const

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn defaultDropAction_0<RetType, T: QAbstractItemView_defaultDropAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.defaultDropAction_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_defaultDropAction_0<RetType> {
  fn defaultDropAction_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_defaultDropAction_0<i32> for () {
  fn defaultDropAction_0(self , rsthis: & QAbstractItemView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView17defaultDropActionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:194
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAlternatingRowColors(bool)

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn setAlternatingRowColors_0<RetType, T: QAbstractItemView_setAlternatingRowColors_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAlternatingRowColors_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_setAlternatingRowColors_0<RetType> {
  fn setAlternatingRowColors_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_setAlternatingRowColors_0<(/*void*/)> for (bool) {
  fn setAlternatingRowColors_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView23setAlternatingRowColorsEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:195
// index:0
// Public Visibility=Default Availability=Available
// [1] bool alternatingRowColors() const

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn alternatingRowColors_0<RetType, T: QAbstractItemView_alternatingRowColors_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.alternatingRowColors_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_alternatingRowColors_0<RetType> {
  fn alternatingRowColors_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_alternatingRowColors_0<bool> for () {
  fn alternatingRowColors_0(self , rsthis: & QAbstractItemView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView20alternatingRowColorsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:197
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setIconSize(const QSize &)

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn setIconSize_0<RetType, T: QAbstractItemView_setIconSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIconSize_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_setIconSize_0<RetType> {
  fn setIconSize_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_setIconSize_0<(/*void*/)> for (usize) {
  fn setIconSize_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView11setIconSizeERK5QSize", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:198
// index:0
// Public Visibility=Default Availability=Available
// [8] QSize iconSize() const

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn iconSize_0<RetType, T: QAbstractItemView_iconSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.iconSize_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_iconSize_0<RetType> {
  fn iconSize_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_iconSize_0<usize> for () {
  fn iconSize_0(self , rsthis: & QAbstractItemView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView8iconSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:200
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTextElideMode(Qt::TextElideMode)

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn setTextElideMode_0<RetType, T: QAbstractItemView_setTextElideMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTextElideMode_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_setTextElideMode_0<RetType> {
  fn setTextElideMode_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_setTextElideMode_0<(/*void*/)> for (i32) {
  fn setTextElideMode_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView16setTextElideModeEN2Qt13TextElideModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:201
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::TextElideMode textElideMode() const

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn textElideMode_0<RetType, T: QAbstractItemView_textElideMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textElideMode_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_textElideMode_0<RetType> {
  fn textElideMode_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_textElideMode_0<i32> for () {
  fn textElideMode_0(self , rsthis: & QAbstractItemView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView13textElideModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:203
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void keyboardSearch(const QString &)

/*
Moves to and selects the item best matching the string search. If no item is found nothing happens.

In the default implementation, the search is reset if search is empty, or the time interval since the last search has exceeded QApplication::keyboardInputInterval().
*/
impl /*struct*/ QAbstractItemView {
  pub fn keyboardSearch_0<RetType, T: QAbstractItemView_keyboardSearch_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyboardSearch_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_keyboardSearch_0<RetType> {
  fn keyboardSearch_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_keyboardSearch_0<(/*void*/)> for (usize) {
  fn keyboardSearch_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView14keyboardSearchERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:205
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [16] QRect visualRect(const QModelIndex &) const

/*
Returns the rectangle on the viewport occupied by the item at index.

If your item is displayed in several areas then visualRect should return the primary area that contains index and not the complete area that index might encompasses, touch or cause drawing.

In the base class this is a pure virtual function.

See also indexAt() and visualRegionForSelection().
*/
impl /*struct*/ QAbstractItemView {
  pub fn visualRect_0<RetType, T: QAbstractItemView_visualRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.visualRect_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_visualRect_0<RetType> {
  fn visualRect_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_visualRect_0<usize> for (usize) {
  fn visualRect_0(self , rsthis: & QAbstractItemView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView10visualRectERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:206
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [-2] void scrollTo(const QModelIndex &, QAbstractItemView::ScrollHint)

/*
Scrolls the view if necessary to ensure that the item at index is visible. The view will try to position the item according to the given hint.

In the base class this is a pure virtual function.
*/
impl /*struct*/ QAbstractItemView {
  pub fn scrollTo_0<RetType, T: QAbstractItemView_scrollTo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scrollTo_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_scrollTo_0<RetType> {
  fn scrollTo_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_scrollTo_0<(/*void*/)> for (usize,i32) {
  fn scrollTo_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView8scrollToERK11QModelIndexNS_10ScrollHintE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:207
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [24] QModelIndex indexAt(const QPoint &) const

/*
Returns the model index of the item at the viewport coordinates point.

In the base class this is a pure virtual function.

See also visualRect().
*/
impl /*struct*/ QAbstractItemView {
  pub fn indexAt_0<RetType, T: QAbstractItemView_indexAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexAt_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_indexAt_0<RetType> {
  fn indexAt_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_indexAt_0<usize> for (usize) {
  fn indexAt_0(self , rsthis: & QAbstractItemView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView7indexAtERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:209
// index:0
// Public Visibility=Default Availability=Available
// [8] QSize sizeHintForIndex(const QModelIndex &) const

/*
Returns the size hint for the item with the specified index or an invalid size for invalid indexes.

See also sizeHintForRow() and sizeHintForColumn().
*/
impl /*struct*/ QAbstractItemView {
  pub fn sizeHintForIndex_0<RetType, T: QAbstractItemView_sizeHintForIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHintForIndex_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_sizeHintForIndex_0<RetType> {
  fn sizeHintForIndex_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_sizeHintForIndex_0<usize> for (usize) {
  fn sizeHintForIndex_0(self , rsthis: & QAbstractItemView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView16sizeHintForIndexERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:210
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int sizeHintForRow(int) const

/*
Returns the height size hint for the specified row or -1 if there is no model.

The returned height is calculated using the size hints of the given row's items, i.e. the returned value is the maximum height among the items. Note that to control the height of a row, you must reimplement the QAbstractItemDelegate::sizeHint() function.

This function is used in views with a vertical header to find the size hint for a header section based on the contents of the given row.

See also sizeHintForColumn().
*/
impl /*struct*/ QAbstractItemView {
  pub fn sizeHintForRow_0<RetType, T: QAbstractItemView_sizeHintForRow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHintForRow_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_sizeHintForRow_0<RetType> {
  fn sizeHintForRow_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_sizeHintForRow_0<i32> for (i32) {
  fn sizeHintForRow_0(self , rsthis: & QAbstractItemView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView14sizeHintForRowEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:211
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int sizeHintForColumn(int) const

/*
Returns the width size hint for the specified column or -1 if there is no model.

This function is used in views with a horizontal header to find the size hint for a header section based on the contents of the given column.

See also sizeHintForRow().
*/
impl /*struct*/ QAbstractItemView {
  pub fn sizeHintForColumn_0<RetType, T: QAbstractItemView_sizeHintForColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHintForColumn_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_sizeHintForColumn_0<RetType> {
  fn sizeHintForColumn_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_sizeHintForColumn_0<i32> for (i32) {
  fn sizeHintForColumn_0(self , rsthis: & QAbstractItemView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView17sizeHintForColumnEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:213
// index:0
// Public Visibility=Default Availability=Available
// [-2] void openPersistentEditor(const QModelIndex &)

/*
Opens a persistent editor on the item at the given index. If no editor exists, the delegate will create a new editor.

See also closePersistentEditor() and isPersistentEditorOpen().
*/
impl /*struct*/ QAbstractItemView {
  pub fn openPersistentEditor_0<RetType, T: QAbstractItemView_openPersistentEditor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.openPersistentEditor_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_openPersistentEditor_0<RetType> {
  fn openPersistentEditor_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_openPersistentEditor_0<(/*void*/)> for (usize) {
  fn openPersistentEditor_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView20openPersistentEditorERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:214
// index:0
// Public Visibility=Default Availability=Available
// [-2] void closePersistentEditor(const QModelIndex &)

/*
Closes the persistent editor for the item at the given index.

See also openPersistentEditor() and isPersistentEditorOpen().
*/
impl /*struct*/ QAbstractItemView {
  pub fn closePersistentEditor_0<RetType, T: QAbstractItemView_closePersistentEditor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.closePersistentEditor_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_closePersistentEditor_0<RetType> {
  fn closePersistentEditor_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_closePersistentEditor_0<(/*void*/)> for (usize) {
  fn closePersistentEditor_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView21closePersistentEditorERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:215
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isPersistentEditorOpen(const QModelIndex &) const

/*
Returns whether a persistent editor is open for the item at index index.

This function was introduced in  Qt 5.10.

See also openPersistentEditor() and closePersistentEditor().
*/
impl /*struct*/ QAbstractItemView {
  pub fn isPersistentEditorOpen_0<RetType, T: QAbstractItemView_isPersistentEditorOpen_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isPersistentEditorOpen_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_isPersistentEditorOpen_0<RetType> {
  fn isPersistentEditorOpen_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_isPersistentEditorOpen_0<bool> for (usize) {
  fn isPersistentEditorOpen_0(self , rsthis: & QAbstractItemView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView22isPersistentEditorOpenERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:217
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setIndexWidget(const QModelIndex &, QWidget *)

/*
Sets the given widget on the item at the given index, passing the ownership of the widget to the viewport.

If index is invalid (e.g., if you pass the root index), this function will do nothing.

The given widget's autoFillBackground property must be set to true, otherwise the widget's background will be transparent, showing both the model data and the item at the given index.

If index widget A is replaced with index widget B, index widget A will be deleted. For example, in the code snippet below, the QLineEdit object will be deleted.


  setIndexWidget(index, new QLineEdit);
  ...
  setIndexWidget(index, new QTextEdit);



This function should only be used to display static content within the visible area corresponding to an item of data. If you want to display custom dynamic content or implement a custom editor widget, subclass QItemDelegate instead.

This function was introduced in  Qt 4.1.

See also indexWidget() and Delegate Classes.
*/
impl /*struct*/ QAbstractItemView {
  pub fn setIndexWidget_0<RetType, T: QAbstractItemView_setIndexWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIndexWidget_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_setIndexWidget_0<RetType> {
  fn setIndexWidget_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_setIndexWidget_0<(/*void*/)> for (usize,usize) {
  fn setIndexWidget_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView14setIndexWidgetERK11QModelIndexP7QWidget", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:218
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * indexWidget(const QModelIndex &) const

/*
Returns the widget for the item at the given index.

This function was introduced in  Qt 4.1.

See also setIndexWidget().
*/
impl /*struct*/ QAbstractItemView {
  pub fn indexWidget_0<RetType, T: QAbstractItemView_indexWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexWidget_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_indexWidget_0<RetType> {
  fn indexWidget_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_indexWidget_0<usize> for (usize) {
  fn indexWidget_0(self , rsthis: & QAbstractItemView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView11indexWidgetERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:220
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setItemDelegateForRow(int, QAbstractItemDelegate *)

/*
Sets the given item delegate used by this view and model for the given row. All items on row will be drawn and managed by delegate instead of using the default delegate (i.e., itemDelegate()).

Any existing row delegate for row will be removed, but not deleted. QAbstractItemView does not take ownership of delegate.

Note: If a delegate has been assigned to both a row and a column, the row delegate (i.e., this delegate) will take precedence and manage the intersecting cell index.

Warning: You should not share the same instance of a delegate between views. Doing so can cause incorrect or unintuitive editing behavior since each view connected to a given delegate may receive the closeEditor() signal, and attempt to access, modify or close an editor that has already been closed.

This function was introduced in  Qt 4.2.

See also itemDelegateForRow(), setItemDelegateForColumn(), and itemDelegate().
*/
impl /*struct*/ QAbstractItemView {
  pub fn setItemDelegateForRow_0<RetType, T: QAbstractItemView_setItemDelegateForRow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setItemDelegateForRow_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_setItemDelegateForRow_0<RetType> {
  fn setItemDelegateForRow_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_setItemDelegateForRow_0<(/*void*/)> for (i32,usize) {
  fn setItemDelegateForRow_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView21setItemDelegateForRowEiP21QAbstractItemDelegate", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:221
// index:0
// Public Visibility=Default Availability=Available
// [8] QAbstractItemDelegate * itemDelegateForRow(int) const

/*
Returns the item delegate used by this view and model for the given row, or 0 if no delegate has been assigned. You can call itemDelegate() to get a pointer to the current delegate for a given index.

This function was introduced in  Qt 4.2.

See also setItemDelegateForRow(), itemDelegateForColumn(), and setItemDelegate().
*/
impl /*struct*/ QAbstractItemView {
  pub fn itemDelegateForRow_0<RetType, T: QAbstractItemView_itemDelegateForRow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemDelegateForRow_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_itemDelegateForRow_0<RetType> {
  fn itemDelegateForRow_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_itemDelegateForRow_0<usize> for (i32) {
  fn itemDelegateForRow_0(self , rsthis: & QAbstractItemView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView18itemDelegateForRowEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:223
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setItemDelegateForColumn(int, QAbstractItemDelegate *)

/*
Sets the given item delegate used by this view and model for the given column. All items on column will be drawn and managed by delegate instead of using the default delegate (i.e., itemDelegate()).

Any existing column delegate for column will be removed, but not deleted. QAbstractItemView does not take ownership of delegate.

Note: If a delegate has been assigned to both a row and a column, the row delegate will take precedence and manage the intersecting cell index.

Warning: You should not share the same instance of a delegate between views. Doing so can cause incorrect or unintuitive editing behavior since each view connected to a given delegate may receive the closeEditor() signal, and attempt to access, modify or close an editor that has already been closed.

This function was introduced in  Qt 4.2.

See also itemDelegateForColumn(), setItemDelegateForRow(), and itemDelegate().
*/
impl /*struct*/ QAbstractItemView {
  pub fn setItemDelegateForColumn_0<RetType, T: QAbstractItemView_setItemDelegateForColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setItemDelegateForColumn_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_setItemDelegateForColumn_0<RetType> {
  fn setItemDelegateForColumn_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_setItemDelegateForColumn_0<(/*void*/)> for (i32,usize) {
  fn setItemDelegateForColumn_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView24setItemDelegateForColumnEiP21QAbstractItemDelegate", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:224
// index:0
// Public Visibility=Default Availability=Available
// [8] QAbstractItemDelegate * itemDelegateForColumn(int) const

/*
Returns the item delegate used by this view and model for the given column. You can call itemDelegate() to get a pointer to the current delegate for a given index.

This function was introduced in  Qt 4.2.

See also setItemDelegateForColumn(), itemDelegateForRow(), and itemDelegate().
*/
impl /*struct*/ QAbstractItemView {
  pub fn itemDelegateForColumn_0<RetType, T: QAbstractItemView_itemDelegateForColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemDelegateForColumn_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_itemDelegateForColumn_0<RetType> {
  fn itemDelegateForColumn_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_itemDelegateForColumn_0<usize> for (i32) {
  fn itemDelegateForColumn_0(self , rsthis: & QAbstractItemView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView21itemDelegateForColumnEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:228
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QVariant inputMethodQuery(Qt::InputMethodQuery) const

/*
Reimplemented from QWidget::inputMethodQuery().
*/
impl /*struct*/ QAbstractItemView {
  pub fn inputMethodQuery_0<RetType, T: QAbstractItemView_inputMethodQuery_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inputMethodQuery_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_inputMethodQuery_0<RetType> {
  fn inputMethodQuery_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_inputMethodQuery_0<usize> for (i32) {
  fn inputMethodQuery_0(self , rsthis: & QAbstractItemView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView16inputMethodQueryEN2Qt16InputMethodQueryE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:233
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void reset()

/*
Reset the internal state of the view.

Warning: This function will reset open editors, scroll bar positions, selections, etc. Existing changes will not be committed. If you would like to save your changes when resetting the view, you can reimplement this function, commit your changes, and then call the superclass' implementation.
*/
impl /*struct*/ QAbstractItemView {
  pub fn reset_0<RetType, T: QAbstractItemView_reset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.reset_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_reset_0<RetType> {
  fn reset_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_reset_0<(/*void*/)> for () {
  fn reset_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView5resetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:234
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setRootIndex(const QModelIndex &)

/*
Sets the root item to the item at the given index.

See also rootIndex().
*/
impl /*struct*/ QAbstractItemView {
  pub fn setRootIndex_0<RetType, T: QAbstractItemView_setRootIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRootIndex_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_setRootIndex_0<RetType> {
  fn setRootIndex_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_setRootIndex_0<(/*void*/)> for (usize) {
  fn setRootIndex_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView12setRootIndexERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:235
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void doItemsLayout()

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn doItemsLayout_0<RetType, T: QAbstractItemView_doItemsLayout_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.doItemsLayout_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_doItemsLayout_0<RetType> {
  fn doItemsLayout_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_doItemsLayout_0<(/*void*/)> for () {
  fn doItemsLayout_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView13doItemsLayoutEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:236
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void selectAll()

/*
Selects all items in the view. This function will use the selection behavior set on the view when selecting.

See also setSelection(), selectedIndexes(), and clearSelection().
*/
impl /*struct*/ QAbstractItemView {
  pub fn selectAll_0<RetType, T: QAbstractItemView_selectAll_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectAll_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_selectAll_0<RetType> {
  fn selectAll_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_selectAll_0<(/*void*/)> for () {
  fn selectAll_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView9selectAllEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:237
// index:0
// Public Visibility=Default Availability=Available
// [-2] void edit(const QModelIndex &)

/*
Starts editing the item corresponding to the given index if it is editable.

Note that this function does not change the current index. Since the current index defines the next and previous items to edit, users may find that keyboard navigation does not work as expected. To provide consistent navigation behavior, call setCurrentIndex() before this function with the same model index.

See also QModelIndex::flags().
*/
impl /*struct*/ QAbstractItemView {
  pub fn edit_0<RetType, T: QAbstractItemView_edit_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.edit_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_edit_0<RetType> {
  fn edit_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_edit_0<(/*void*/)> for (usize) {
  fn edit_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView4editERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:295
// index:1
// Protected virtual Visibility=Default Availability=Available
// [1] bool edit(const QModelIndex &, QAbstractItemView::EditTrigger, QEvent *)

/*
Starts editing the item corresponding to the given index if it is editable.

Note that this function does not change the current index. Since the current index defines the next and previous items to edit, users may find that keyboard navigation does not work as expected. To provide consistent navigation behavior, call setCurrentIndex() before this function with the same model index.

See also QModelIndex::flags().
*/
impl /*struct*/ QAbstractItemView {
  pub fn edit_1<RetType, T: QAbstractItemView_edit_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.edit_1(self);
    // return 1;
  }
}
pub trait QAbstractItemView_edit_1<RetType> {
  fn edit_1(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_edit_1<bool> for (usize,i32,usize) {
  fn edit_1(self , rsthis: & QAbstractItemView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN17QAbstractItemView4editERK11QModelIndexNS_11EditTriggerEP6QEvent", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:238
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clearSelection()

/*
Deselects all selected items. The current index will not be changed.

See also setSelection() and selectAll().
*/
impl /*struct*/ QAbstractItemView {
  pub fn clearSelection_0<RetType, T: QAbstractItemView_clearSelection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clearSelection_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_clearSelection_0<RetType> {
  fn clearSelection_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_clearSelection_0<(/*void*/)> for () {
  fn clearSelection_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView14clearSelectionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:239
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCurrentIndex(const QModelIndex &)

/*
Sets the current item to be the item at index.

Unless the current selection mode is NoSelection, the item is also selected. Note that this function also updates the starting position for any new selections the user performs.

To set an item as the current item without selecting it, call

selectionModel()->setCurrentIndex(index, QItemSelectionModel::NoUpdate);

See also currentIndex(), currentChanged(), and selectionMode.
*/
impl /*struct*/ QAbstractItemView {
  pub fn setCurrentIndex_0<RetType, T: QAbstractItemView_setCurrentIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentIndex_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_setCurrentIndex_0<RetType> {
  fn setCurrentIndex_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_setCurrentIndex_0<(/*void*/)> for (usize) {
  fn setCurrentIndex_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView15setCurrentIndexERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:240
// index:0
// Public Visibility=Default Availability=Available
// [-2] void scrollToTop()

/*
Scrolls the view to the top.

This function was introduced in  Qt 4.1.

See also scrollTo() and scrollToBottom().
*/
impl /*struct*/ QAbstractItemView {
  pub fn scrollToTop_0<RetType, T: QAbstractItemView_scrollToTop_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scrollToTop_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_scrollToTop_0<RetType> {
  fn scrollToTop_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_scrollToTop_0<(/*void*/)> for () {
  fn scrollToTop_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView11scrollToTopEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:241
// index:0
// Public Visibility=Default Availability=Available
// [-2] void scrollToBottom()

/*
Scrolls the view to the bottom.

This function was introduced in  Qt 4.1.

See also scrollTo() and scrollToTop().
*/
impl /*struct*/ QAbstractItemView {
  pub fn scrollToBottom_0<RetType, T: QAbstractItemView_scrollToBottom_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scrollToBottom_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_scrollToBottom_0<RetType> {
  fn scrollToBottom_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_scrollToBottom_0<(/*void*/)> for () {
  fn scrollToBottom_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView14scrollToBottomEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:242
// index:0
// Public Visibility=Default Availability=Available
// [-2] void update(const QModelIndex &)

/*
Updates the area occupied by the given index.

This function was introduced in  Qt 4.3.
*/
impl /*struct*/ QAbstractItemView {
  pub fn update_0<RetType, T: QAbstractItemView_update_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.update_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_update_0<RetType> {
  fn update_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_update_0<(/*void*/)> for (usize) {
  fn update_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView6updateERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:246
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void rowsInserted(const QModelIndex &, int, int)

/*
This slot is called when rows are inserted. The new rows are those under the given parent from start to end inclusive. The base class implementation calls fetchMore() on the model to check for more data.

See also rowsAboutToBeRemoved().
*/
impl /*struct*/ QAbstractItemView {
  pub fn rowsInserted_0<RetType, T: QAbstractItemView_rowsInserted_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowsInserted_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_rowsInserted_0<RetType> {
  fn rowsInserted_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_rowsInserted_0<(/*void*/)> for (usize,i32,i32) {
  fn rowsInserted_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView12rowsInsertedERK11QModelIndexii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:247
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void rowsAboutToBeRemoved(const QModelIndex &, int, int)

/*
This slot is called when rows are about to be removed. The deleted rows are those under the given parent from start to end inclusive.

See also rowsInserted().
*/
impl /*struct*/ QAbstractItemView {
  pub fn rowsAboutToBeRemoved_0<RetType, T: QAbstractItemView_rowsAboutToBeRemoved_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowsAboutToBeRemoved_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_rowsAboutToBeRemoved_0<RetType> {
  fn rowsAboutToBeRemoved_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_rowsAboutToBeRemoved_0<(/*void*/)> for (usize,i32,i32) {
  fn rowsAboutToBeRemoved_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView20rowsAboutToBeRemovedERK11QModelIndexii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:248
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void selectionChanged(const QItemSelection &, const QItemSelection &)

/*
This slot is called when the selection is changed. The previous selection (which may be empty), is specified by deselected, and the new selection by selected.

See also setSelection().
*/
impl /*struct*/ QAbstractItemView {
  pub fn selectionChanged_0<RetType, T: QAbstractItemView_selectionChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectionChanged_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_selectionChanged_0<RetType> {
  fn selectionChanged_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_selectionChanged_0<(/*void*/)> for (usize,usize) {
  fn selectionChanged_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView16selectionChangedERK14QItemSelectionS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:249
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void currentChanged(const QModelIndex &, const QModelIndex &)

/*
This slot is called when a new item becomes the current item. The previous current item is specified by the previous index, and the new item by the current index.

If you want to know about changes to items see the dataChanged() signal.
*/
impl /*struct*/ QAbstractItemView {
  pub fn currentChanged_0<RetType, T: QAbstractItemView_currentChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentChanged_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_currentChanged_0<RetType> {
  fn currentChanged_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_currentChanged_0<(/*void*/)> for (usize,usize) {
  fn currentChanged_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView14currentChangedERK11QModelIndexS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:250
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void updateEditorData()

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn updateEditorData_0<RetType, T: QAbstractItemView_updateEditorData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updateEditorData_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_updateEditorData_0<RetType> {
  fn updateEditorData_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_updateEditorData_0<(/*void*/)> for () {
  fn updateEditorData_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView16updateEditorDataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:251
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void updateEditorGeometries()

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn updateEditorGeometries_0<RetType, T: QAbstractItemView_updateEditorGeometries_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updateEditorGeometries_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_updateEditorGeometries_0<RetType> {
  fn updateEditorGeometries_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_updateEditorGeometries_0<(/*void*/)> for () {
  fn updateEditorGeometries_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView22updateEditorGeometriesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:252
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void updateGeometries()

/*
Updates the geometry of the child widgets of the view.

This function was introduced in  Qt 4.4.
*/
impl /*struct*/ QAbstractItemView {
  pub fn updateGeometries_0<RetType, T: QAbstractItemView_updateGeometries_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updateGeometries_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_updateGeometries_0<RetType> {
  fn updateGeometries_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_updateGeometries_0<(/*void*/)> for () {
  fn updateGeometries_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView16updateGeometriesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:253
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void verticalScrollbarAction(int)

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn verticalScrollbarAction_0<RetType, T: QAbstractItemView_verticalScrollbarAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.verticalScrollbarAction_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_verticalScrollbarAction_0<RetType> {
  fn verticalScrollbarAction_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_verticalScrollbarAction_0<(/*void*/)> for (i32) {
  fn verticalScrollbarAction_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView23verticalScrollbarActionEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:254
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void horizontalScrollbarAction(int)

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn horizontalScrollbarAction_0<RetType, T: QAbstractItemView_horizontalScrollbarAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.horizontalScrollbarAction_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_horizontalScrollbarAction_0<RetType> {
  fn horizontalScrollbarAction_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_horizontalScrollbarAction_0<(/*void*/)> for (i32) {
  fn horizontalScrollbarAction_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView25horizontalScrollbarActionEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:255
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void verticalScrollbarValueChanged(int)

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn verticalScrollbarValueChanged_0<RetType, T: QAbstractItemView_verticalScrollbarValueChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.verticalScrollbarValueChanged_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_verticalScrollbarValueChanged_0<RetType> {
  fn verticalScrollbarValueChanged_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_verticalScrollbarValueChanged_0<(/*void*/)> for (i32) {
  fn verticalScrollbarValueChanged_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView29verticalScrollbarValueChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:256
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void horizontalScrollbarValueChanged(int)

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn horizontalScrollbarValueChanged_0<RetType, T: QAbstractItemView_horizontalScrollbarValueChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.horizontalScrollbarValueChanged_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_horizontalScrollbarValueChanged_0<RetType> {
  fn horizontalScrollbarValueChanged_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_horizontalScrollbarValueChanged_0<(/*void*/)> for (i32) {
  fn horizontalScrollbarValueChanged_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView31horizontalScrollbarValueChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:257
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void closeEditor(QWidget *, QAbstractItemDelegate::EndEditHint)

/*
Closes the given editor, and releases it. The hint is used to specify how the view should respond to the end of the editing operation. For example, the hint may indicate that the next item in the view should be opened for editing.

See also edit() and commitData().
*/
impl /*struct*/ QAbstractItemView {
  pub fn closeEditor_0<RetType, T: QAbstractItemView_closeEditor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.closeEditor_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_closeEditor_0<RetType> {
  fn closeEditor_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_closeEditor_0<(/*void*/)> for (usize,i32) {
  fn closeEditor_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView11closeEditorEP7QWidgetN21QAbstractItemDelegate11EndEditHintE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:258
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void commitData(QWidget *)

/*
Commit the data in the editor to the model.

See also closeEditor().
*/
impl /*struct*/ QAbstractItemView {
  pub fn commitData_0<RetType, T: QAbstractItemView_commitData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.commitData_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_commitData_0<RetType> {
  fn commitData_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_commitData_0<(/*void*/)> for (usize) {
  fn commitData_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView10commitDataEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:259
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void editorDestroyed(QObject *)

/*
This function is called when the given editor has been destroyed.

See also closeEditor().
*/
impl /*struct*/ QAbstractItemView {
  pub fn editorDestroyed_0<RetType, T: QAbstractItemView_editorDestroyed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.editorDestroyed_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_editorDestroyed_0<RetType> {
  fn editorDestroyed_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_editorDestroyed_0<(/*void*/)> for (usize) {
  fn editorDestroyed_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView15editorDestroyedEP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:262
// index:0
// Public Visibility=Default Availability=Available
// [-2] void pressed(const QModelIndex &)

/*
This signal is emitted when a mouse button is pressed. The item the mouse was pressed on is specified by index. The signal is only emitted when the index is valid.

Use the QApplication::mouseButtons() function to get the state of the mouse buttons.

See also activated(), clicked(), doubleClicked(), and entered().
*/
impl /*struct*/ QAbstractItemView {
  pub fn pressed_0<RetType, T: QAbstractItemView_pressed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pressed_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_pressed_0<RetType> {
  fn pressed_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_pressed_0<(/*void*/)> for (usize) {
  fn pressed_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView7pressedERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:263
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clicked(const QModelIndex &)

/*
This signal is emitted when a mouse button is left-clicked. The item the mouse was clicked on is specified by index. The signal is only emitted when the index is valid.

See also activated(), doubleClicked(), entered(), and pressed().
*/
impl /*struct*/ QAbstractItemView {
  pub fn clicked_0<RetType, T: QAbstractItemView_clicked_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clicked_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_clicked_0<RetType> {
  fn clicked_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_clicked_0<(/*void*/)> for (usize) {
  fn clicked_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView7clickedERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:264
// index:0
// Public Visibility=Default Availability=Available
// [-2] void doubleClicked(const QModelIndex &)

/*
This signal is emitted when a mouse button is double-clicked. The item the mouse was double-clicked on is specified by index. The signal is only emitted when the index is valid.

See also clicked() and activated().
*/
impl /*struct*/ QAbstractItemView {
  pub fn doubleClicked_0<RetType, T: QAbstractItemView_doubleClicked_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.doubleClicked_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_doubleClicked_0<RetType> {
  fn doubleClicked_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_doubleClicked_0<(/*void*/)> for (usize) {
  fn doubleClicked_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView13doubleClickedERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:266
// index:0
// Public Visibility=Default Availability=Available
// [-2] void activated(const QModelIndex &)

/*
This signal is emitted when the item specified by index is activated by the user. How to activate items depends on the platform; e.g., by single- or double-clicking the item, or by pressing the Return or Enter key when the item is current.

See also clicked(), doubleClicked(), entered(), and pressed().
*/
impl /*struct*/ QAbstractItemView {
  pub fn activated_0<RetType, T: QAbstractItemView_activated_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.activated_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_activated_0<RetType> {
  fn activated_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_activated_0<(/*void*/)> for (usize) {
  fn activated_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView9activatedERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:267
// index:0
// Public Visibility=Default Availability=Available
// [-2] void entered(const QModelIndex &)

/*
This signal is emitted when the mouse cursor enters the item specified by index. Mouse tracking needs to be enabled for this feature to work.

See also viewportEntered(), activated(), clicked(), doubleClicked(), and pressed().
*/
impl /*struct*/ QAbstractItemView {
  pub fn entered_0<RetType, T: QAbstractItemView_entered_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.entered_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_entered_0<RetType> {
  fn entered_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_entered_0<(/*void*/)> for (usize) {
  fn entered_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView7enteredERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:268
// index:0
// Public Visibility=Default Availability=Available
// [-2] void viewportEntered()

/*
This signal is emitted when the mouse cursor enters the viewport. Mouse tracking needs to be enabled for this feature to work.

See also entered().
*/
impl /*struct*/ QAbstractItemView {
  pub fn viewportEntered_0<RetType, T: QAbstractItemView_viewportEntered_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.viewportEntered_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_viewportEntered_0<RetType> {
  fn viewportEntered_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_viewportEntered_0<(/*void*/)> for () {
  fn viewportEntered_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView15viewportEnteredEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:270
// index:0
// Public Visibility=Default Availability=Available
// [-2] void iconSizeChanged(const QSize &)

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn iconSizeChanged_0<RetType, T: QAbstractItemView_iconSizeChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.iconSizeChanged_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_iconSizeChanged_0<RetType> {
  fn iconSizeChanged_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_iconSizeChanged_0<(/*void*/)> for (usize) {
  fn iconSizeChanged_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView15iconSizeChangedERK5QSize", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:275
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void setHorizontalStepsPerItem(int)

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn setHorizontalStepsPerItem_0<RetType, T: QAbstractItemView_setHorizontalStepsPerItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHorizontalStepsPerItem_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_setHorizontalStepsPerItem_0<RetType> {
  fn setHorizontalStepsPerItem_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_setHorizontalStepsPerItem_0<(/*void*/)> for (i32) {
  fn setHorizontalStepsPerItem_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView25setHorizontalStepsPerItemEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:276
// index:0
// Protected Visibility=Default Availability=Available
// [4] int horizontalStepsPerItem() const

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn horizontalStepsPerItem_0<RetType, T: QAbstractItemView_horizontalStepsPerItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.horizontalStepsPerItem_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_horizontalStepsPerItem_0<RetType> {
  fn horizontalStepsPerItem_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_horizontalStepsPerItem_0<i32> for () {
  fn horizontalStepsPerItem_0(self , rsthis: & QAbstractItemView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView22horizontalStepsPerItemEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:277
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void setVerticalStepsPerItem(int)

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn setVerticalStepsPerItem_0<RetType, T: QAbstractItemView_setVerticalStepsPerItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVerticalStepsPerItem_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_setVerticalStepsPerItem_0<RetType> {
  fn setVerticalStepsPerItem_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_setVerticalStepsPerItem_0<(/*void*/)> for (i32) {
  fn setVerticalStepsPerItem_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView23setVerticalStepsPerItemEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:278
// index:0
// Protected Visibility=Default Availability=Available
// [4] int verticalStepsPerItem() const

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn verticalStepsPerItem_0<RetType, T: QAbstractItemView_verticalStepsPerItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.verticalStepsPerItem_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_verticalStepsPerItem_0<RetType> {
  fn verticalStepsPerItem_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_verticalStepsPerItem_0<i32> for () {
  fn verticalStepsPerItem_0(self , rsthis: & QAbstractItemView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView20verticalStepsPerItemEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:283
// index:0
// Protected purevirtual virtual Visibility=Default Availability=Available
// [24] QModelIndex moveCursor(QAbstractItemView::CursorAction, Qt::KeyboardModifiers)

/*
Returns a QModelIndex object pointing to the next object in the view, based on the given cursorAction and keyboard modifiers specified by modifiers.

In the base class this is a pure virtual function.
*/
impl /*struct*/ QAbstractItemView {
  pub fn moveCursor_0<RetType, T: QAbstractItemView_moveCursor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveCursor_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_moveCursor_0<RetType> {
  fn moveCursor_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_moveCursor_0<usize> for (i32,i32) {
  fn moveCursor_0(self , rsthis: & QAbstractItemView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN17QAbstractItemView10moveCursorENS_12CursorActionE6QFlagsIN2Qt16KeyboardModifierEE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:286
// index:0
// Protected purevirtual virtual Visibility=Default Availability=Available
// [4] int horizontalOffset() const

/*
Returns the horizontal offset of the view.

In the base class this is a pure virtual function.

See also verticalOffset().
*/
impl /*struct*/ QAbstractItemView {
  pub fn horizontalOffset_0<RetType, T: QAbstractItemView_horizontalOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.horizontalOffset_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_horizontalOffset_0<RetType> {
  fn horizontalOffset_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_horizontalOffset_0<i32> for () {
  fn horizontalOffset_0(self , rsthis: & QAbstractItemView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView16horizontalOffsetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:287
// index:0
// Protected purevirtual virtual Visibility=Default Availability=Available
// [4] int verticalOffset() const

/*
Returns the vertical offset of the view.

In the base class this is a pure virtual function.

See also horizontalOffset().
*/
impl /*struct*/ QAbstractItemView {
  pub fn verticalOffset_0<RetType, T: QAbstractItemView_verticalOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.verticalOffset_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_verticalOffset_0<RetType> {
  fn verticalOffset_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_verticalOffset_0<i32> for () {
  fn verticalOffset_0(self , rsthis: & QAbstractItemView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView14verticalOffsetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:289
// index:0
// Protected purevirtual virtual Visibility=Default Availability=Available
// [1] bool isIndexHidden(const QModelIndex &) const

/*
Returns true if the item referred to by the given index is hidden in the view, otherwise returns false.

Hiding is a view specific feature. For example in TableView a column can be marked as hidden or a row in the TreeView.

In the base class this is a pure virtual function.
*/
impl /*struct*/ QAbstractItemView {
  pub fn isIndexHidden_0<RetType, T: QAbstractItemView_isIndexHidden_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isIndexHidden_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_isIndexHidden_0<RetType> {
  fn isIndexHidden_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_isIndexHidden_0<bool> for (usize) {
  fn isIndexHidden_0(self , rsthis: & QAbstractItemView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView13isIndexHiddenERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:291
// index:0
// Protected purevirtual virtual Visibility=Default Availability=Available
// [-2] void setSelection(const QRect &, QItemSelectionModel::SelectionFlags)

/*
Applies the selection flags to the items in or touched by the rectangle, rect.

When implementing your own itemview setSelection should call selectionModel()->select(selection, flags) where selection is either an empty QModelIndex or a QItemSelection that contains all items that are contained in rect.

See also selectionCommand() and selectedIndexes().
*/
impl /*struct*/ QAbstractItemView {
  pub fn setSelection_0<RetType, T: QAbstractItemView_setSelection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSelection_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_setSelection_0<RetType> {
  fn setSelection_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_setSelection_0<(/*void*/)> for (usize,i32) {
  fn setSelection_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView12setSelectionERK5QRect6QFlagsIN19QItemSelectionModel13SelectionFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:292
// index:0
// Protected purevirtual virtual Visibility=Default Availability=Available
// [8] QRegion visualRegionForSelection(const QItemSelection &) const

/*
Returns the region from the viewport of the items in the given selection.

In the base class this is a pure virtual function.

See also visualRect() and selectedIndexes().
*/
impl /*struct*/ QAbstractItemView {
  pub fn visualRegionForSelection_0<RetType, T: QAbstractItemView_visualRegionForSelection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.visualRegionForSelection_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_visualRegionForSelection_0<RetType> {
  fn visualRegionForSelection_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_visualRegionForSelection_0<usize> for (usize) {
  fn visualRegionForSelection_0(self , rsthis: & QAbstractItemView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView24visualRegionForSelectionERK14QItemSelection", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:293
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] QModelIndexList selectedIndexes() const

/*
This convenience function returns a list of all selected and non-hidden item indexes in the view. The list contains no duplicates, and is not sorted.

See also QItemSelectionModel::selectedIndexes().
*/
impl /*struct*/ QAbstractItemView {
  pub fn selectedIndexes_0<RetType, T: QAbstractItemView_selectedIndexes_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectedIndexes_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_selectedIndexes_0<RetType> {
  fn selectedIndexes_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_selectedIndexes_0<usize> for () {
  fn selectedIndexes_0(self , rsthis: & QAbstractItemView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView15selectedIndexesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:297
// index:0
// Protected virtual Visibility=Default Availability=Available
// [4] QItemSelectionModel::SelectionFlags selectionCommand(const QModelIndex &, const QEvent *) const

/*
Returns the SelectionFlags to be used when updating a selection with to include the index specified. The event is a user input event, such as a mouse or keyboard event.

Reimplement this function to define your own selection behavior.

See also setSelection().
*/
impl /*struct*/ QAbstractItemView {
  pub fn selectionCommand_0<RetType, T: QAbstractItemView_selectionCommand_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectionCommand_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_selectionCommand_0<RetType> {
  fn selectionCommand_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_selectionCommand_0<i32> for (usize,usize) {
  fn selectionCommand_0(self , rsthis: & QAbstractItemView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView16selectionCommandERK11QModelIndexPK6QEvent", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:301
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void startDrag(Qt::DropActions)

/*
Starts a drag by calling drag->exec() using the given supportedActions.
*/
impl /*struct*/ QAbstractItemView {
  pub fn startDrag_0<RetType, T: QAbstractItemView_startDrag_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.startDrag_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_startDrag_0<RetType> {
  fn startDrag_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_startDrag_0<(/*void*/)> for (i32) {
  fn startDrag_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView9startDragE6QFlagsIN2Qt10DropActionEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:304
// index:0
// Protected virtual Visibility=Default Availability=Available
// [192] QStyleOptionViewItem viewOptions() const

/*
Returns a QStyleOptionViewItem structure populated with the view's palette, font, state, alignments etc.
*/
impl /*struct*/ QAbstractItemView {
  pub fn viewOptions_0<RetType, T: QAbstractItemView_viewOptions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.viewOptions_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_viewOptions_0<RetType> {
  fn viewOptions_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_viewOptions_0<usize> for () {
  fn viewOptions_0(self , rsthis: & QAbstractItemView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView11viewOptionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:316
// index:0
// Protected Visibility=Default Availability=Available
// [4] QAbstractItemView::State state() const

/*
Returns the item view's state.

See also setState().
*/
impl /*struct*/ QAbstractItemView {
  pub fn state_0<RetType, T: QAbstractItemView_state_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.state_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_state_0<RetType> {
  fn state_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_state_0<i32> for () {
  fn state_0(self , rsthis: & QAbstractItemView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView5stateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:317
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void setState(QAbstractItemView::State)

/*
Sets the item view's state to the given state.

See also state().
*/
impl /*struct*/ QAbstractItemView {
  pub fn setState_0<RetType, T: QAbstractItemView_setState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setState_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_setState_0<RetType> {
  fn setState_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_setState_0<(/*void*/)> for (i32) {
  fn setState_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView8setStateENS_5StateE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:319
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void scheduleDelayedItemsLayout()

/*
Schedules a layout of the items in the view to be executed when the event processing starts.

Even if scheduleDelayedItemsLayout() is called multiple times before events are processed, the view will only do the layout once.

See also executeDelayedItemsLayout().
*/
impl /*struct*/ QAbstractItemView {
  pub fn scheduleDelayedItemsLayout_0<RetType, T: QAbstractItemView_scheduleDelayedItemsLayout_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scheduleDelayedItemsLayout_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_scheduleDelayedItemsLayout_0<RetType> {
  fn scheduleDelayedItemsLayout_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_scheduleDelayedItemsLayout_0<(/*void*/)> for () {
  fn scheduleDelayedItemsLayout_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView26scheduleDelayedItemsLayoutEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:320
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void executeDelayedItemsLayout()

/*
Executes the scheduled layouts without waiting for the event processing to begin.

See also scheduleDelayedItemsLayout().
*/
impl /*struct*/ QAbstractItemView {
  pub fn executeDelayedItemsLayout_0<RetType, T: QAbstractItemView_executeDelayedItemsLayout_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.executeDelayedItemsLayout_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_executeDelayedItemsLayout_0<RetType> {
  fn executeDelayedItemsLayout_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_executeDelayedItemsLayout_0<(/*void*/)> for () {
  fn executeDelayedItemsLayout_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView25executeDelayedItemsLayoutEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:322
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void setDirtyRegion(const QRegion &)

/*
Marks the given region as dirty and schedules it to be updated. You only need to call this function if you are implementing your own view subclass.

This function was introduced in  Qt 4.1.

See also scrollDirtyRegion() and dirtyRegionOffset().
*/
impl /*struct*/ QAbstractItemView {
  pub fn setDirtyRegion_0<RetType, T: QAbstractItemView_setDirtyRegion_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDirtyRegion_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_setDirtyRegion_0<RetType> {
  fn setDirtyRegion_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_setDirtyRegion_0<(/*void*/)> for (usize) {
  fn setDirtyRegion_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView14setDirtyRegionERK7QRegion", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:323
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void scrollDirtyRegion(int, int)

/*
Prepares the view for scrolling by (dx,dy) pixels by moving the dirty regions in the opposite direction. You only need to call this function if you are implementing a scrolling viewport in your view subclass.

If you implement scrollContentsBy() in a subclass of QAbstractItemView, call this function before you call QWidget::scroll() on the viewport. Alternatively, just call update().

See also scrollContentsBy(), dirtyRegionOffset(), and setDirtyRegion().
*/
impl /*struct*/ QAbstractItemView {
  pub fn scrollDirtyRegion_0<RetType, T: QAbstractItemView_scrollDirtyRegion_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scrollDirtyRegion_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_scrollDirtyRegion_0<RetType> {
  fn scrollDirtyRegion_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_scrollDirtyRegion_0<(/*void*/)> for (i32,i32) {
  fn scrollDirtyRegion_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView17scrollDirtyRegionEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:324
// index:0
// Protected Visibility=Default Availability=Available
// [8] QPoint dirtyRegionOffset() const

/*
Returns the offset of the dirty regions in the view.

If you use scrollDirtyRegion() and implement a paintEvent() in a subclass of QAbstractItemView, you should translate the area given by the paint event with the offset returned from this function.

See also scrollDirtyRegion() and setDirtyRegion().
*/
impl /*struct*/ QAbstractItemView {
  pub fn dirtyRegionOffset_0<RetType, T: QAbstractItemView_dirtyRegionOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dirtyRegionOffset_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_dirtyRegionOffset_0<RetType> {
  fn dirtyRegionOffset_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_dirtyRegionOffset_0<usize> for () {
  fn dirtyRegionOffset_0(self , rsthis: & QAbstractItemView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView17dirtyRegionOffsetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:326
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void startAutoScroll()

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn startAutoScroll_0<RetType, T: QAbstractItemView_startAutoScroll_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.startAutoScroll_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_startAutoScroll_0<RetType> {
  fn startAutoScroll_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_startAutoScroll_0<(/*void*/)> for () {
  fn startAutoScroll_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView15startAutoScrollEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:327
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void stopAutoScroll()

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn stopAutoScroll_0<RetType, T: QAbstractItemView_stopAutoScroll_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.stopAutoScroll_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_stopAutoScroll_0<RetType> {
  fn stopAutoScroll_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_stopAutoScroll_0<(/*void*/)> for () {
  fn stopAutoScroll_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView14stopAutoScrollEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:328
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void doAutoScroll()

/*

*/
impl /*struct*/ QAbstractItemView {
  pub fn doAutoScroll_0<RetType, T: QAbstractItemView_doAutoScroll_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.doAutoScroll_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_doAutoScroll_0<RetType> {
  fn doAutoScroll_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_doAutoScroll_0<(/*void*/)> for () {
  fn doAutoScroll_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView12doAutoScrollEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:330
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool focusNextPrevChild(bool)

/*
Reimplemented from QWidget::focusNextPrevChild().
*/
impl /*struct*/ QAbstractItemView {
  pub fn focusNextPrevChild_0<RetType, T: QAbstractItemView_focusNextPrevChild_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusNextPrevChild_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_focusNextPrevChild_0<RetType> {
  fn focusNextPrevChild_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_focusNextPrevChild_0<bool> for (bool) {
  fn focusNextPrevChild_0(self , rsthis: & QAbstractItemView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN17QAbstractItemView18focusNextPrevChildEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:331
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QAbstractItemView {
  pub fn event_0<RetType, T: QAbstractItemView_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_event_0<RetType> {
  fn event_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QAbstractItemView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN17QAbstractItemView5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:332
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool viewportEvent(QEvent *)

/*
Reimplemented from QAbstractScrollArea::viewportEvent().

This function is used to handle tool tips, and What's This? mode, if the given event is a QEvent::ToolTip,or a QEvent::WhatsThis. It passes all other events on to its base class viewportEvent() handler.
*/
impl /*struct*/ QAbstractItemView {
  pub fn viewportEvent_0<RetType, T: QAbstractItemView_viewportEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.viewportEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_viewportEvent_0<RetType> {
  fn viewportEvent_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_viewportEvent_0<bool> for (usize) {
  fn viewportEvent_0(self , rsthis: & QAbstractItemView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN17QAbstractItemView13viewportEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:333
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mousePressEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mousePressEvent().

This function is called with the given event when a mouse button is pressed while the cursor is inside the widget. If a valid item is pressed on it is made into the current item. This function emits the pressed() signal.
*/
impl /*struct*/ QAbstractItemView {
  pub fn mousePressEvent_0<RetType, T: QAbstractItemView_mousePressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mousePressEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_mousePressEvent_0<RetType> {
  fn mousePressEvent_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_mousePressEvent_0<(/*void*/)> for (usize) {
  fn mousePressEvent_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView15mousePressEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:334
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseMoveEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseMoveEvent().

This function is called with the given event when a mouse move event is sent to the widget. If a selection is in progress and new items are moved over the selection is extended; if a drag is in progress it is continued.
*/
impl /*struct*/ QAbstractItemView {
  pub fn mouseMoveEvent_0<RetType, T: QAbstractItemView_mouseMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseMoveEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_mouseMoveEvent_0<RetType> {
  fn mouseMoveEvent_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_mouseMoveEvent_0<(/*void*/)> for (usize) {
  fn mouseMoveEvent_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView14mouseMoveEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:335
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseReleaseEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseReleaseEvent().

This function is called with the given event when a mouse button is released, after a mouse press event on the widget. If a user presses the mouse inside your widget and then drags the mouse to another location before releasing the mouse button, your widget receives the release event. The function will emit the clicked() signal if an item was being pressed.
*/
impl /*struct*/ QAbstractItemView {
  pub fn mouseReleaseEvent_0<RetType, T: QAbstractItemView_mouseReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_mouseReleaseEvent_0<RetType> {
  fn mouseReleaseEvent_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_mouseReleaseEvent_0<(/*void*/)> for (usize) {
  fn mouseReleaseEvent_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView17mouseReleaseEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:336
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseDoubleClickEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseDoubleClickEvent().

This function is called with the given event when a mouse button is double clicked inside the widget. If the double-click is on a valid item it emits the doubleClicked() signal and calls edit() on the item.
*/
impl /*struct*/ QAbstractItemView {
  pub fn mouseDoubleClickEvent_0<RetType, T: QAbstractItemView_mouseDoubleClickEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseDoubleClickEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_mouseDoubleClickEvent_0<RetType> {
  fn mouseDoubleClickEvent_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_mouseDoubleClickEvent_0<(/*void*/)> for (usize) {
  fn mouseDoubleClickEvent_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView21mouseDoubleClickEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:338
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dragEnterEvent(QDragEnterEvent *)

/*
Reimplemented from QWidget::dragEnterEvent().

This function is called with the given event when a drag and drop operation enters the widget. If the drag is over a valid dropping place (e.g. over an item that accepts drops), the event is accepted; otherwise it is ignored.

See also dropEvent() and startDrag().
*/
impl /*struct*/ QAbstractItemView {
  pub fn dragEnterEvent_0<RetType, T: QAbstractItemView_dragEnterEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragEnterEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_dragEnterEvent_0<RetType> {
  fn dragEnterEvent_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_dragEnterEvent_0<(/*void*/)> for (usize) {
  fn dragEnterEvent_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView14dragEnterEventEP15QDragEnterEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:339
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dragMoveEvent(QDragMoveEvent *)

/*
Reimplemented from QWidget::dragMoveEvent().

This function is called continuously with the given event during a drag and drop operation over the widget. It can cause the view to scroll if, for example, the user drags a selection to view's right or bottom edge. In this case, the event will be accepted; otherwise it will be ignored.

See also dropEvent() and startDrag().
*/
impl /*struct*/ QAbstractItemView {
  pub fn dragMoveEvent_0<RetType, T: QAbstractItemView_dragMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragMoveEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_dragMoveEvent_0<RetType> {
  fn dragMoveEvent_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_dragMoveEvent_0<(/*void*/)> for (usize) {
  fn dragMoveEvent_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView13dragMoveEventEP14QDragMoveEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:340
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dragLeaveEvent(QDragLeaveEvent *)

/*
Reimplemented from QWidget::dragLeaveEvent().

This function is called when the item being dragged leaves the view. The event describes the state of the drag and drop operation.
*/
impl /*struct*/ QAbstractItemView {
  pub fn dragLeaveEvent_0<RetType, T: QAbstractItemView_dragLeaveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragLeaveEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_dragLeaveEvent_0<RetType> {
  fn dragLeaveEvent_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_dragLeaveEvent_0<(/*void*/)> for (usize) {
  fn dragLeaveEvent_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView14dragLeaveEventEP15QDragLeaveEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:341
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dropEvent(QDropEvent *)

/*
Reimplemented from QWidget::dropEvent().

This function is called with the given event when a drop event occurs over the widget. If the model accepts the even position the drop event is accepted; otherwise it is ignored.

See also startDrag().
*/
impl /*struct*/ QAbstractItemView {
  pub fn dropEvent_0<RetType, T: QAbstractItemView_dropEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dropEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_dropEvent_0<RetType> {
  fn dropEvent_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_dropEvent_0<(/*void*/)> for (usize) {
  fn dropEvent_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView9dropEventEP10QDropEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:343
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusInEvent(QFocusEvent *)

/*
Reimplemented from QWidget::focusInEvent().

This function is called with the given event when the widget obtains the focus. By default, the event is ignored.

See also setFocus() and focusOutEvent().
*/
impl /*struct*/ QAbstractItemView {
  pub fn focusInEvent_0<RetType, T: QAbstractItemView_focusInEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusInEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_focusInEvent_0<RetType> {
  fn focusInEvent_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_focusInEvent_0<(/*void*/)> for (usize) {
  fn focusInEvent_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView12focusInEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:344
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusOutEvent(QFocusEvent *)

/*
Reimplemented from QWidget::focusOutEvent().

This function is called with the given event when the widget loses the focus. By default, the event is ignored.

See also clearFocus() and focusInEvent().
*/
impl /*struct*/ QAbstractItemView {
  pub fn focusOutEvent_0<RetType, T: QAbstractItemView_focusOutEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusOutEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_focusOutEvent_0<RetType> {
  fn focusOutEvent_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_focusOutEvent_0<(/*void*/)> for (usize) {
  fn focusOutEvent_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView13focusOutEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:345
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyPressEvent(QKeyEvent *)

/*
Reimplemented from QWidget::keyPressEvent().

This function is called with the given event when a key event is sent to the widget. The default implementation handles basic cursor movement, e.g. Up, Down, Left, Right, Home, PageUp, and PageDown; the activated() signal is emitted if the current index is valid and the activation key is pressed (e.g. Enter or Return, depending on the platform). This function is where editing is initiated by key press, e.g. if F2 is pressed.

See also edit(), moveCursor(), keyboardSearch(), and tabKeyNavigation.
*/
impl /*struct*/ QAbstractItemView {
  pub fn keyPressEvent_0<RetType, T: QAbstractItemView_keyPressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyPressEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_keyPressEvent_0<RetType> {
  fn keyPressEvent_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_keyPressEvent_0<(/*void*/)> for (usize) {
  fn keyPressEvent_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView13keyPressEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:346
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void resizeEvent(QResizeEvent *)

/*
Reimplemented from QWidget::resizeEvent().

This function is called with the given event when a resize event is sent to the widget.

See also QWidget::resizeEvent().
*/
impl /*struct*/ QAbstractItemView {
  pub fn resizeEvent_0<RetType, T: QAbstractItemView_resizeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_resizeEvent_0<RetType> {
  fn resizeEvent_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_resizeEvent_0<(/*void*/)> for (usize) {
  fn resizeEvent_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView11resizeEventEP12QResizeEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:347
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void timerEvent(QTimerEvent *)

/*
Reimplemented from QObject::timerEvent().

This function is called with the given event when a timer event is sent to the widget.

See also QObject::timerEvent().
*/
impl /*struct*/ QAbstractItemView {
  pub fn timerEvent_0<RetType, T: QAbstractItemView_timerEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.timerEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_timerEvent_0<RetType> {
  fn timerEvent_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_timerEvent_0<(/*void*/)> for (usize) {
  fn timerEvent_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView10timerEventEP11QTimerEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:348
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void inputMethodEvent(QInputMethodEvent *)

/*
Reimplemented from QWidget::inputMethodEvent().
*/
impl /*struct*/ QAbstractItemView {
  pub fn inputMethodEvent_0<RetType, T: QAbstractItemView_inputMethodEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inputMethodEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_inputMethodEvent_0<RetType> {
  fn inputMethodEvent_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_inputMethodEvent_0<(/*void*/)> for (usize) {
  fn inputMethodEvent_0(self , rsthis: & QAbstractItemView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAbstractItemView16inputMethodEventEP17QInputMethodEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:352
// index:0
// Protected Visibility=Default Availability=Available
// [4] QAbstractItemView::DropIndicatorPosition dropIndicatorPosition() const

/*
Returns the position of the drop indicator in relation to the closest item.

This function was introduced in  Qt 4.1.
*/
impl /*struct*/ QAbstractItemView {
  pub fn dropIndicatorPosition_0<RetType, T: QAbstractItemView_dropIndicatorPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dropIndicatorPosition_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_dropIndicatorPosition_0<RetType> {
  fn dropIndicatorPosition_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_dropIndicatorPosition_0<i32> for () {
  fn dropIndicatorPosition_0(self , rsthis: & QAbstractItemView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView21dropIndicatorPositionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractitemview.h:355
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] QSize viewportSizeHint() const

/*
Reimplemented from QAbstractScrollArea::viewportSizeHint().

This function was introduced in  Qt 5.2.
*/
impl /*struct*/ QAbstractItemView {
  pub fn viewportSizeHint_0<RetType, T: QAbstractItemView_viewportSizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.viewportSizeHint_0(self);
    // return 1;
  }
}
pub trait QAbstractItemView_viewportSizeHint_0<RetType> {
  fn viewportSizeHint_0(self , rsthis: & QAbstractItemView) -> RetType;
}
impl<'a> /*trait*/ QAbstractItemView_viewportSizeHint_0<usize> for () {
  fn viewportSizeHint_0(self , rsthis: & QAbstractItemView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAbstractItemView16viewportSizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


/*
This enum indicates how the view responds to user selections:



The most commonly used modes are SingleSelection and ExtendedSelection.

*/
pub type QAbstractItemView__SelectionMode = i32;
// Items cannot be selected.
pub const QAbstractItemView__NoSelection :QAbstractItemView__SelectionMode = 0;
// When the user selects an item, any already-selected item becomes unselected. It is possible for the user to deselect the selected item.
pub const QAbstractItemView__SingleSelection :QAbstractItemView__SelectionMode = 1;
// When the user selects an item in the usual way, the selection status of that item is toggled and the other items are left alone. Multiple items can be toggled by dragging the mouse over them.
pub const QAbstractItemView__MultiSelection :QAbstractItemView__SelectionMode = 2;
// When the user selects an item in the usual way, the selection is cleared and the new item selected. However, if the user presses the Ctrl key when clicking on an item, the clicked item gets toggled and all other items are left untouched. If the user presses the Shift key while clicking on an item, all items between the current item and the clicked item are selected or unselected, depending on the state of the clicked item. Multiple items can be selected by dragging the mouse over them.
pub const QAbstractItemView__ExtendedSelection :QAbstractItemView__SelectionMode = 3;
// When the user selects an item in the usual way, the selection is cleared and the new item selected. However, if the user presses the Shift key while clicking on an item, all items between the current item and the clicked item are selected or unselected, depending on the state of the clicked item.
pub const QAbstractItemView__ContiguousSelection :QAbstractItemView__SelectionMode = 4;
pub fn QAbstractItemView_SelectionModeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QAbstractItemView", val);
}
pub fn QAbstractItemView_SelectionModeItemName_s(val: i32) ->String {
  //var nilthis *QAbstractItemView
  //return nilthis.SelectionModeItemName(val);
  return QAbstractItemView_SelectionModeItemName(val);
}


/*

*/
pub type QAbstractItemView__SelectionBehavior = i32;
// Selecting single items.
pub const QAbstractItemView__SelectItems :QAbstractItemView__SelectionBehavior = 0;
// Selecting only rows.
pub const QAbstractItemView__SelectRows :QAbstractItemView__SelectionBehavior = 1;
// Selecting only columns.
pub const QAbstractItemView__SelectColumns :QAbstractItemView__SelectionBehavior = 2;
pub fn QAbstractItemView_SelectionBehaviorItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QAbstractItemView", val);
}
pub fn QAbstractItemView_SelectionBehaviorItemName_s(val: i32) ->String {
  //var nilthis *QAbstractItemView
  //return nilthis.SelectionBehaviorItemName(val);
  return QAbstractItemView_SelectionBehaviorItemName(val);
}


/*

*/
pub type QAbstractItemView__ScrollHint = i32;
// Scroll to ensure that the item is visible.
pub const QAbstractItemView__EnsureVisible :QAbstractItemView__ScrollHint = 0;
// Scroll to position the item at the top of the viewport.
pub const QAbstractItemView__PositionAtTop :QAbstractItemView__ScrollHint = 1;
// Scroll to position the item at the bottom of the viewport.
pub const QAbstractItemView__PositionAtBottom :QAbstractItemView__ScrollHint = 2;
// Scroll to position the item at the center of the viewport.
pub const QAbstractItemView__PositionAtCenter :QAbstractItemView__ScrollHint = 3;
pub fn QAbstractItemView_ScrollHintItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QAbstractItemView", val);
}
pub fn QAbstractItemView_ScrollHintItemName_s(val: i32) ->String {
  //var nilthis *QAbstractItemView
  //return nilthis.ScrollHintItemName(val);
  return QAbstractItemView_ScrollHintItemName(val);
}


/*


*/
pub type QAbstractItemView__EditTrigger = i32;
// 
pub const QAbstractItemView__NoEditTriggers :QAbstractItemView__EditTrigger = 0;
// 
pub const QAbstractItemView__CurrentChanged :QAbstractItemView__EditTrigger = 1;
// 
pub const QAbstractItemView__DoubleClicked :QAbstractItemView__EditTrigger = 2;
// 
pub const QAbstractItemView__SelectedClicked :QAbstractItemView__EditTrigger = 4;
// 
pub const QAbstractItemView__EditKeyPressed :QAbstractItemView__EditTrigger = 8;
// 
pub const QAbstractItemView__AnyKeyPressed :QAbstractItemView__EditTrigger = 16;
// 
pub const QAbstractItemView__AllEditTriggers :QAbstractItemView__EditTrigger = 31;
pub fn QAbstractItemView_EditTriggerItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QAbstractItemView", val);
}
pub fn QAbstractItemView_EditTriggerItemName_s(val: i32) ->String {
  //var nilthis *QAbstractItemView
  //return nilthis.EditTriggerItemName(val);
  return QAbstractItemView_EditTriggerItemName(val);
}


/*
Describes how the scrollbar should behave. When setting the scroll mode to ScrollPerPixel the single step size will adjust automatically unless it was set explicitly using setSingleStep(). The automatic adjustment can be restored by setting the single step size to -1.



This enum was introduced or modified in  Qt 4.2.

*/
pub type QAbstractItemView__ScrollMode = i32;
// The view will scroll the contents one item at a time.
pub const QAbstractItemView__ScrollPerItem :QAbstractItemView__ScrollMode = 0;
// The view will scroll the contents one pixel at a time.
pub const QAbstractItemView__ScrollPerPixel :QAbstractItemView__ScrollMode = 1;
pub fn QAbstractItemView_ScrollModeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QAbstractItemView", val);
}
pub fn QAbstractItemView_ScrollModeItemName_s(val: i32) ->String {
  //var nilthis *QAbstractItemView
  //return nilthis.ScrollModeItemName(val);
  return QAbstractItemView_ScrollModeItemName(val);
}


/*
Describes the various drag and drop events the view can act upon. By default the view does not support dragging or dropping (NoDragDrop).



Note that the model used needs to provide support for drag and drop operations.

This enum was introduced or modified in  Qt 4.2.

See also setDragDropMode() and Using drag and drop with item views.

*/
pub type QAbstractItemView__DragDropMode = i32;
// Does not support dragging or dropping.
pub const QAbstractItemView__NoDragDrop :QAbstractItemView__DragDropMode = 0;
// The view supports dragging of its own items
pub const QAbstractItemView__DragOnly :QAbstractItemView__DragDropMode = 1;
// The view accepts drops
pub const QAbstractItemView__DropOnly :QAbstractItemView__DragDropMode = 2;
// The view supports both dragging and dropping
pub const QAbstractItemView__DragDrop :QAbstractItemView__DragDropMode = 3;
// The view accepts move (not copy) operations only from itself.
pub const QAbstractItemView__InternalMove :QAbstractItemView__DragDropMode = 4;
pub fn QAbstractItemView_DragDropModeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QAbstractItemView", val);
}
pub fn QAbstractItemView_DragDropModeItemName_s(val: i32) ->String {
  //var nilthis *QAbstractItemView
  //return nilthis.DragDropModeItemName(val);
  return QAbstractItemView_DragDropModeItemName(val);
}


/*
This enum describes the different ways to navigate between items,



See also moveCursor().

*/
pub type QAbstractItemView__CursorAction = i32;
// Move to the item above the current item.
pub const QAbstractItemView__MoveUp :QAbstractItemView__CursorAction = 0;
// Move to the item below the current item.
pub const QAbstractItemView__MoveDown :QAbstractItemView__CursorAction = 1;
// Move to the item left of the current item.
pub const QAbstractItemView__MoveLeft :QAbstractItemView__CursorAction = 2;
// Move to the item right of the current item.
pub const QAbstractItemView__MoveRight :QAbstractItemView__CursorAction = 3;
// Move to the top-left corner item.
pub const QAbstractItemView__MoveHome :QAbstractItemView__CursorAction = 4;
// Move to the bottom-right corner item.
pub const QAbstractItemView__MoveEnd :QAbstractItemView__CursorAction = 5;
// Move one page up above the current item.
pub const QAbstractItemView__MovePageUp :QAbstractItemView__CursorAction = 6;
// Move one page down below the current item.
pub const QAbstractItemView__MovePageDown :QAbstractItemView__CursorAction = 7;
// Move to the item after the current item.
pub const QAbstractItemView__MoveNext :QAbstractItemView__CursorAction = 8;
// Move to the item before the current item.
pub const QAbstractItemView__MovePrevious :QAbstractItemView__CursorAction = 9;
pub fn QAbstractItemView_CursorActionItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QAbstractItemView", val);
}
pub fn QAbstractItemView_CursorActionItemName_s(val: i32) ->String {
  //var nilthis *QAbstractItemView
  //return nilthis.CursorActionItemName(val);
  return QAbstractItemView_CursorActionItemName(val);
}


/*
Describes the different states the view can be in. This is usually only interesting when reimplementing your own view.


*/
pub type QAbstractItemView__State = i32;
// The is the default state.
pub const QAbstractItemView__NoState :QAbstractItemView__State = 0;
// The user is dragging items.
pub const QAbstractItemView__DraggingState :QAbstractItemView__State = 1;
// The user is selecting items.
pub const QAbstractItemView__DragSelectingState :QAbstractItemView__State = 2;
// The user is editing an item in a widget editor.
pub const QAbstractItemView__EditingState :QAbstractItemView__State = 3;
// The user is opening a branch of items.
pub const QAbstractItemView__ExpandingState :QAbstractItemView__State = 4;
// The user is closing a branch of items.
pub const QAbstractItemView__CollapsingState :QAbstractItemView__State = 5;
// The item view is performing an animation.
pub const QAbstractItemView__AnimatingState :QAbstractItemView__State = 6;
pub fn QAbstractItemView_StateItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QAbstractItemView", val);
}
pub fn QAbstractItemView_StateItemName_s(val: i32) ->String {
  //var nilthis *QAbstractItemView
  //return nilthis.StateItemName(val);
  return QAbstractItemView_StateItemName(val);
}


/*
This enum indicates the position of the drop indicator in relation to the index at the current mouse position:


*/
pub type QAbstractItemView__DropIndicatorPosition = i32;
// The item will be dropped on the index.
pub const QAbstractItemView__OnItem :QAbstractItemView__DropIndicatorPosition = 0;
// The item will be dropped above the index.
pub const QAbstractItemView__AboveItem :QAbstractItemView__DropIndicatorPosition = 1;
// The item will be dropped below the index.
pub const QAbstractItemView__BelowItem :QAbstractItemView__DropIndicatorPosition = 2;
// The item will be dropped onto a region of the viewport with no items. The way each view handles items dropped onto the viewport depends on the behavior of the underlying model in use.
pub const QAbstractItemView__OnViewport :QAbstractItemView__DropIndicatorPosition = 3;
pub fn QAbstractItemView_DropIndicatorPositionItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QAbstractItemView", val);
}
pub fn QAbstractItemView_DropIndicatorPositionItemName_s(val: i32) ->String {
  //var nilthis *QAbstractItemView
  //return nilthis.DropIndicatorPositionItemName(val);
  return QAbstractItemView_DropIndicatorPositionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
