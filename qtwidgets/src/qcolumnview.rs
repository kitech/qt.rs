

// mod ::widgets::QColumnView
// package qtwidgets
// /usr/include/qt/QtWidgets/qcolumnview.h
// #include <qcolumnview.h>
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

// bool isIndexHidden(const QModelIndex &)
// func (this *QColumnView) InheritIsIndexHidden(f func(index *qtcore.QModelIndex) bool) {
//  qtrt.SetAllInheritCallback(this, "isIndexHidden", f)
// }

// QModelIndex moveCursor(QAbstractItemView::CursorAction, Qt::KeyboardModifiers)
// func (this *QColumnView) InheritMoveCursor(f func(cursorAction int, modifiers int) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "moveCursor", f)
// }

// void resizeEvent(QResizeEvent *)
// func (this *QColumnView) InheritResizeEvent(f func(event *qtgui.QResizeEvent/*777 QResizeEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "resizeEvent", f)
// }

// void setSelection(const QRect &, QItemSelectionModel::SelectionFlags)
// func (this *QColumnView) InheritSetSelection(f func(rect *qtcore.QRect, command int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "setSelection", f)
// }

// QRegion visualRegionForSelection(const QItemSelection &)
// func (this *QColumnView) InheritVisualRegionForSelection(f func(selection *qtcore.QItemSelection) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "visualRegionForSelection", f)
// }

// int horizontalOffset()
// func (this *QColumnView) InheritHorizontalOffset(f func() int) {
//  qtrt.SetAllInheritCallback(this, "horizontalOffset", f)
// }

// int verticalOffset()
// func (this *QColumnView) InheritVerticalOffset(f func() int) {
//  qtrt.SetAllInheritCallback(this, "verticalOffset", f)
// }

// void rowsInserted(const QModelIndex &, int, int)
// func (this *QColumnView) InheritRowsInserted(f func(parent *qtcore.QModelIndex, start int, end_ int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "rowsInserted", f)
// }

// void currentChanged(const QModelIndex &, const QModelIndex &)
// func (this *QColumnView) InheritCurrentChanged(f func(current *qtcore.QModelIndex, previous *qtcore.QModelIndex) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "currentChanged", f)
// }

// void scrollContentsBy(int, int)
// func (this *QColumnView) InheritScrollContentsBy(f func(dx int, dy int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "scrollContentsBy", f)
// }

// QAbstractItemView * createColumn(const QModelIndex &)
// func (this *QColumnView) InheritCreateColumn(f func(rootIndex *qtcore.QModelIndex) unsafe.Pointer/*666*/) {
//  qtrt.SetAllInheritCallback(this, "createColumn", f)
// }

// void initializeColumn(QAbstractItemView *)
// func (this *QColumnView) InheritInitializeColumn(f func(column *QAbstractItemView/*777 QAbstractItemView **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "initializeColumn", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QColumnView)=48
pub struct QColumnView {
  qbase: QAbstractItemView,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QColumnView_ITF interface {
//    QAbstractItemView_ITF
//    QColumnView_PTR() *QColumnView
//}
//func (ptr *QColumnView) QColumnView_PTR() *QColumnView { return ptr }

impl /*struct*/ QColumnView {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QColumnView {
    return QColumnView{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QColumnView {
//  type Target = QColumnViewBASE;
//
//  fn deref(&self) -> &QColumnViewBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QColumnViewBASE> for QColumnView {
//  fn as_ref(& self) -> & QColumnViewBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qcolumnview.h:54
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QColumnView {
  pub fn metaObject_0<RetType, T: QColumnView_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QColumnView_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QColumnView) -> RetType;
}
impl<'a> /*trait*/ QColumnView_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QColumnView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QColumnView10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcolumnview.h:58
// index:0
// Public Visibility=Default Availability=Available
// [-2] void updatePreviewWidget(const QModelIndex &)

/*
This signal is emitted when the preview widget should be updated to provide rich information about index

See also previewWidget().
*/
impl /*struct*/ QColumnView {
  pub fn updatePreviewWidget_0<RetType, T: QColumnView_updatePreviewWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updatePreviewWidget_0(self);
    // return 1;
  }
}
pub trait QColumnView_updatePreviewWidget_0<RetType> {
  fn updatePreviewWidget_0(self , rsthis: & QColumnView) -> RetType;
}
impl<'a> /*trait*/ QColumnView_updatePreviewWidget_0<(/*void*/)> for (usize) {
  fn updatePreviewWidget_0(self , rsthis: & QColumnView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QColumnView19updatePreviewWidgetERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcolumnview.h:61
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QColumnView(QWidget *)

/*
Constructs a column view with a parent to represent a model's data. Use setModel() to set the model.

See also QAbstractItemModel.
*/
// QColumnView(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QColumnView {
  pub fn QColumnView_0<T: QColumnView_QColumnView_0>(value: T) -> QColumnView {
    let rsthis = value.QColumnView_0();
    return rsthis;
    // return 1;
  }
}

pub trait QColumnView_QColumnView_0 {
  fn QColumnView_0(self) -> QColumnView;
}
// QColumnView(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QColumnView_QColumnView_0 for (usize) {
  fn QColumnView_0(self) -> QColumnView {
    // unsafe{_ZN11QColumnViewC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QColumnViewC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QColumnView{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcolumnview.h:62
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QColumnView()

/*

*/
pub fn DeleteQColumnView(this :*mut QColumnView) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QColumnViewD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qcolumnview.h:65
// index:0
// Public virtual Visibility=Default Availability=Available
// [24] QModelIndex indexAt(const QPoint &) const

/*
Reimplemented from QAbstractItemView::indexAt().
*/
impl /*struct*/ QColumnView {
  pub fn indexAt_0<RetType, T: QColumnView_indexAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexAt_0(self);
    // return 1;
  }
}
pub trait QColumnView_indexAt_0<RetType> {
  fn indexAt_0(self , rsthis: & QColumnView) -> RetType;
}
impl<'a> /*trait*/ QColumnView_indexAt_0<usize> for (usize) {
  fn indexAt_0(self , rsthis: & QColumnView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QColumnView7indexAtERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcolumnview.h:66
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void scrollTo(const QModelIndex &, QAbstractItemView::ScrollHint)

/*
Reimplemented from QAbstractItemView::scrollTo().
*/
impl /*struct*/ QColumnView {
  pub fn scrollTo_0<RetType, T: QColumnView_scrollTo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scrollTo_0(self);
    // return 1;
  }
}
pub trait QColumnView_scrollTo_0<RetType> {
  fn scrollTo_0(self , rsthis: & QColumnView) -> RetType;
}
impl<'a> /*trait*/ QColumnView_scrollTo_0<(/*void*/)> for (usize,i32) {
  fn scrollTo_0(self , rsthis: & QColumnView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QColumnView8scrollToERK11QModelIndexN17QAbstractItemView10ScrollHintE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcolumnview.h:67
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Reimplemented from QWidget::sizeHint().
*/
impl /*struct*/ QColumnView {
  pub fn sizeHint_0<RetType, T: QColumnView_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QColumnView_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QColumnView) -> RetType;
}
impl<'a> /*trait*/ QColumnView_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QColumnView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QColumnView8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcolumnview.h:68
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QRect visualRect(const QModelIndex &) const

/*
Reimplemented from QAbstractItemView::visualRect().
*/
impl /*struct*/ QColumnView {
  pub fn visualRect_0<RetType, T: QColumnView_visualRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.visualRect_0(self);
    // return 1;
  }
}
pub trait QColumnView_visualRect_0<RetType> {
  fn visualRect_0(self , rsthis: & QColumnView) -> RetType;
}
impl<'a> /*trait*/ QColumnView_visualRect_0<usize> for (usize) {
  fn visualRect_0(self , rsthis: & QColumnView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QColumnView10visualRectERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcolumnview.h:69
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setModel(QAbstractItemModel *)

/*
Reimplemented from QAbstractItemView::setModel().
*/
impl /*struct*/ QColumnView {
  pub fn setModel_0<RetType, T: QColumnView_setModel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setModel_0(self);
    // return 1;
  }
}
pub trait QColumnView_setModel_0<RetType> {
  fn setModel_0(self , rsthis: & QColumnView) -> RetType;
}
impl<'a> /*trait*/ QColumnView_setModel_0<(/*void*/)> for (usize) {
  fn setModel_0(self , rsthis: & QColumnView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QColumnView8setModelEP18QAbstractItemModel", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcolumnview.h:70
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setSelectionModel(QItemSelectionModel *)

/*
Reimplemented from QAbstractItemView::setSelectionModel().
*/
impl /*struct*/ QColumnView {
  pub fn setSelectionModel_0<RetType, T: QColumnView_setSelectionModel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSelectionModel_0(self);
    // return 1;
  }
}
pub trait QColumnView_setSelectionModel_0<RetType> {
  fn setSelectionModel_0(self , rsthis: & QColumnView) -> RetType;
}
impl<'a> /*trait*/ QColumnView_setSelectionModel_0<(/*void*/)> for (usize) {
  fn setSelectionModel_0(self , rsthis: & QColumnView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QColumnView17setSelectionModelEP19QItemSelectionModel", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcolumnview.h:71
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setRootIndex(const QModelIndex &)

/*
Reimplemented from QAbstractItemView::setRootIndex().
*/
impl /*struct*/ QColumnView {
  pub fn setRootIndex_0<RetType, T: QColumnView_setRootIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRootIndex_0(self);
    // return 1;
  }
}
pub trait QColumnView_setRootIndex_0<RetType> {
  fn setRootIndex_0(self , rsthis: & QColumnView) -> RetType;
}
impl<'a> /*trait*/ QColumnView_setRootIndex_0<(/*void*/)> for (usize) {
  fn setRootIndex_0(self , rsthis: & QColumnView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QColumnView12setRootIndexERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcolumnview.h:72
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void selectAll()

/*
Reimplemented from QAbstractItemView::selectAll().
*/
impl /*struct*/ QColumnView {
  pub fn selectAll_0<RetType, T: QColumnView_selectAll_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectAll_0(self);
    // return 1;
  }
}
pub trait QColumnView_selectAll_0<RetType> {
  fn selectAll_0(self , rsthis: & QColumnView) -> RetType;
}
impl<'a> /*trait*/ QColumnView_selectAll_0<(/*void*/)> for () {
  fn selectAll_0(self , rsthis: & QColumnView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QColumnView9selectAllEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcolumnview.h:75
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setResizeGripsVisible(bool)

/*

*/
impl /*struct*/ QColumnView {
  pub fn setResizeGripsVisible_0<RetType, T: QColumnView_setResizeGripsVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setResizeGripsVisible_0(self);
    // return 1;
  }
}
pub trait QColumnView_setResizeGripsVisible_0<RetType> {
  fn setResizeGripsVisible_0(self , rsthis: & QColumnView) -> RetType;
}
impl<'a> /*trait*/ QColumnView_setResizeGripsVisible_0<(/*void*/)> for (bool) {
  fn setResizeGripsVisible_0(self , rsthis: & QColumnView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QColumnView21setResizeGripsVisibleEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcolumnview.h:76
// index:0
// Public Visibility=Default Availability=Available
// [1] bool resizeGripsVisible() const

/*

*/
impl /*struct*/ QColumnView {
  pub fn resizeGripsVisible_0<RetType, T: QColumnView_resizeGripsVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeGripsVisible_0(self);
    // return 1;
  }
}
pub trait QColumnView_resizeGripsVisible_0<RetType> {
  fn resizeGripsVisible_0(self , rsthis: & QColumnView) -> RetType;
}
impl<'a> /*trait*/ QColumnView_resizeGripsVisible_0<bool> for () {
  fn resizeGripsVisible_0(self , rsthis: & QColumnView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QColumnView18resizeGripsVisibleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcolumnview.h:78
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * previewWidget() const

/*
Returns the preview widget, or 0 if there is none.

See also setPreviewWidget() and updatePreviewWidget().
*/
impl /*struct*/ QColumnView {
  pub fn previewWidget_0<RetType, T: QColumnView_previewWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.previewWidget_0(self);
    // return 1;
  }
}
pub trait QColumnView_previewWidget_0<RetType> {
  fn previewWidget_0(self , rsthis: & QColumnView) -> RetType;
}
impl<'a> /*trait*/ QColumnView_previewWidget_0<usize> for () {
  fn previewWidget_0(self , rsthis: & QColumnView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QColumnView13previewWidgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcolumnview.h:79
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPreviewWidget(QWidget *)

/*
Sets the preview widget.

The widget becomes a child of the column view, and will be destroyed when the column area is deleted or when a new widget is set.

See also previewWidget() and updatePreviewWidget().
*/
impl /*struct*/ QColumnView {
  pub fn setPreviewWidget_0<RetType, T: QColumnView_setPreviewWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPreviewWidget_0(self);
    // return 1;
  }
}
pub trait QColumnView_setPreviewWidget_0<RetType> {
  fn setPreviewWidget_0(self , rsthis: & QColumnView) -> RetType;
}
impl<'a> /*trait*/ QColumnView_setPreviewWidget_0<(/*void*/)> for (usize) {
  fn setPreviewWidget_0(self , rsthis: & QColumnView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QColumnView16setPreviewWidgetEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcolumnview.h:88
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool isIndexHidden(const QModelIndex &) const

/*
Reimplemented from QAbstractItemView::isIndexHidden().
*/
impl /*struct*/ QColumnView {
  pub fn isIndexHidden_0<RetType, T: QColumnView_isIndexHidden_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isIndexHidden_0(self);
    // return 1;
  }
}
pub trait QColumnView_isIndexHidden_0<RetType> {
  fn isIndexHidden_0(self , rsthis: & QColumnView) -> RetType;
}
impl<'a> /*trait*/ QColumnView_isIndexHidden_0<bool> for (usize) {
  fn isIndexHidden_0(self , rsthis: & QColumnView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QColumnView13isIndexHiddenERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcolumnview.h:89
// index:0
// Protected virtual Visibility=Default Availability=Available
// [24] QModelIndex moveCursor(QAbstractItemView::CursorAction, Qt::KeyboardModifiers)

/*
Reimplemented from QAbstractItemView::moveCursor().

Move left should go to the parent index Move right should go to the child index or down if there is no child
*/
impl /*struct*/ QColumnView {
  pub fn moveCursor_0<RetType, T: QColumnView_moveCursor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveCursor_0(self);
    // return 1;
  }
}
pub trait QColumnView_moveCursor_0<RetType> {
  fn moveCursor_0(self , rsthis: & QColumnView) -> RetType;
}
impl<'a> /*trait*/ QColumnView_moveCursor_0<usize> for (i32,i32) {
  fn moveCursor_0(self , rsthis: & QColumnView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QColumnView10moveCursorEN17QAbstractItemView12CursorActionE6QFlagsIN2Qt16KeyboardModifierEE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcolumnview.h:90
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void resizeEvent(QResizeEvent *)

/*
Reimplemented from QWidget::resizeEvent().
*/
impl /*struct*/ QColumnView {
  pub fn resizeEvent_0<RetType, T: QColumnView_resizeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeEvent_0(self);
    // return 1;
  }
}
pub trait QColumnView_resizeEvent_0<RetType> {
  fn resizeEvent_0(self , rsthis: & QColumnView) -> RetType;
}
impl<'a> /*trait*/ QColumnView_resizeEvent_0<(/*void*/)> for (usize) {
  fn resizeEvent_0(self , rsthis: & QColumnView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QColumnView11resizeEventEP12QResizeEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcolumnview.h:91
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void setSelection(const QRect &, QItemSelectionModel::SelectionFlags)

/*
Reimplemented from QAbstractItemView::setSelection().
*/
impl /*struct*/ QColumnView {
  pub fn setSelection_0<RetType, T: QColumnView_setSelection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSelection_0(self);
    // return 1;
  }
}
pub trait QColumnView_setSelection_0<RetType> {
  fn setSelection_0(self , rsthis: & QColumnView) -> RetType;
}
impl<'a> /*trait*/ QColumnView_setSelection_0<(/*void*/)> for (usize,i32) {
  fn setSelection_0(self , rsthis: & QColumnView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QColumnView12setSelectionERK5QRect6QFlagsIN19QItemSelectionModel13SelectionFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcolumnview.h:92
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] QRegion visualRegionForSelection(const QItemSelection &) const

/*
Reimplemented from QAbstractItemView::visualRegionForSelection().
*/
impl /*struct*/ QColumnView {
  pub fn visualRegionForSelection_0<RetType, T: QColumnView_visualRegionForSelection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.visualRegionForSelection_0(self);
    // return 1;
  }
}
pub trait QColumnView_visualRegionForSelection_0<RetType> {
  fn visualRegionForSelection_0(self , rsthis: & QColumnView) -> RetType;
}
impl<'a> /*trait*/ QColumnView_visualRegionForSelection_0<usize> for (usize) {
  fn visualRegionForSelection_0(self , rsthis: & QColumnView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QColumnView24visualRegionForSelectionERK14QItemSelection", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcolumnview.h:93
// index:0
// Protected virtual Visibility=Default Availability=Available
// [4] int horizontalOffset() const

/*
Reimplemented from QAbstractItemView::horizontalOffset().
*/
impl /*struct*/ QColumnView {
  pub fn horizontalOffset_0<RetType, T: QColumnView_horizontalOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.horizontalOffset_0(self);
    // return 1;
  }
}
pub trait QColumnView_horizontalOffset_0<RetType> {
  fn horizontalOffset_0(self , rsthis: & QColumnView) -> RetType;
}
impl<'a> /*trait*/ QColumnView_horizontalOffset_0<i32> for () {
  fn horizontalOffset_0(self , rsthis: & QColumnView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QColumnView16horizontalOffsetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcolumnview.h:94
// index:0
// Protected virtual Visibility=Default Availability=Available
// [4] int verticalOffset() const

/*
Reimplemented from QAbstractItemView::verticalOffset().
*/
impl /*struct*/ QColumnView {
  pub fn verticalOffset_0<RetType, T: QColumnView_verticalOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.verticalOffset_0(self);
    // return 1;
  }
}
pub trait QColumnView_verticalOffset_0<RetType> {
  fn verticalOffset_0(self , rsthis: & QColumnView) -> RetType;
}
impl<'a> /*trait*/ QColumnView_verticalOffset_0<i32> for () {
  fn verticalOffset_0(self , rsthis: & QColumnView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QColumnView14verticalOffsetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcolumnview.h:95
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void rowsInserted(const QModelIndex &, int, int)

/*
Reimplemented from QAbstractItemView::rowsInserted().
*/
impl /*struct*/ QColumnView {
  pub fn rowsInserted_0<RetType, T: QColumnView_rowsInserted_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowsInserted_0(self);
    // return 1;
  }
}
pub trait QColumnView_rowsInserted_0<RetType> {
  fn rowsInserted_0(self , rsthis: & QColumnView) -> RetType;
}
impl<'a> /*trait*/ QColumnView_rowsInserted_0<(/*void*/)> for (usize,i32,i32) {
  fn rowsInserted_0(self , rsthis: & QColumnView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QColumnView12rowsInsertedERK11QModelIndexii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcolumnview.h:96
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void currentChanged(const QModelIndex &, const QModelIndex &)

/*
Reimplemented from QAbstractItemView::currentChanged().
*/
impl /*struct*/ QColumnView {
  pub fn currentChanged_0<RetType, T: QColumnView_currentChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentChanged_0(self);
    // return 1;
  }
}
pub trait QColumnView_currentChanged_0<RetType> {
  fn currentChanged_0(self , rsthis: & QColumnView) -> RetType;
}
impl<'a> /*trait*/ QColumnView_currentChanged_0<(/*void*/)> for (usize,usize) {
  fn currentChanged_0(self , rsthis: & QColumnView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QColumnView14currentChangedERK11QModelIndexS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcolumnview.h:99
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void scrollContentsBy(int, int)

/*
Reimplemented from QAbstractScrollArea::scrollContentsBy().
*/
impl /*struct*/ QColumnView {
  pub fn scrollContentsBy_0<RetType, T: QColumnView_scrollContentsBy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scrollContentsBy_0(self);
    // return 1;
  }
}
pub trait QColumnView_scrollContentsBy_0<RetType> {
  fn scrollContentsBy_0(self , rsthis: & QColumnView) -> RetType;
}
impl<'a> /*trait*/ QColumnView_scrollContentsBy_0<(/*void*/)> for (i32,i32) {
  fn scrollContentsBy_0(self , rsthis: & QColumnView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QColumnView16scrollContentsByEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcolumnview.h:100
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] QAbstractItemView * createColumn(const QModelIndex &)

/*
To use a custom widget for the final column when you select an item overload this function and return a widget. index is the root index that will be assigned to the view.

Return the new view. QColumnView will automatically take ownership of the widget.

See also setPreviewWidget().
*/
impl /*struct*/ QColumnView {
  pub fn createColumn_0<RetType, T: QColumnView_createColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.createColumn_0(self);
    // return 1;
  }
}
pub trait QColumnView_createColumn_0<RetType> {
  fn createColumn_0(self , rsthis: & QColumnView) -> RetType;
}
impl<'a> /*trait*/ QColumnView_createColumn_0<usize> for (usize) {
  fn createColumn_0(self , rsthis: & QColumnView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QColumnView12createColumnERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcolumnview.h:101
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void initializeColumn(QAbstractItemView *) const

/*
Copies the behavior and options of the column view and applies them to the column such as the iconSize(), textElideMode() and alternatingRowColors(). This can be useful when reimplementing createColumn().

This function was introduced in  Qt 4.4.

See also createColumn().
*/
impl /*struct*/ QColumnView {
  pub fn initializeColumn_0<RetType, T: QColumnView_initializeColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.initializeColumn_0(self);
    // return 1;
  }
}
pub trait QColumnView_initializeColumn_0<RetType> {
  fn initializeColumn_0(self , rsthis: & QColumnView) -> RetType;
}
impl<'a> /*trait*/ QColumnView_initializeColumn_0<(/*void*/)> for (usize) {
  fn initializeColumn_0(self , rsthis: & QColumnView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK11QColumnView16initializeColumnEP17QAbstractItemView", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
