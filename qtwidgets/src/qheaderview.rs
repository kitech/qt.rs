

// mod ::widgets::QHeaderView
// package qtwidgets
// /usr/include/qt/QtWidgets/qheaderview.h
// #include <qheaderview.h>
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

// void updateSection(int)
// func (this *QHeaderView) InheritUpdateSection(f func(logicalIndex int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "updateSection", f)
// }

// void resizeSections()
// func (this *QHeaderView) InheritResizeSections(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "resizeSections", f)
// }

// void sectionsInserted(const QModelIndex &, int, int)
// func (this *QHeaderView) InheritSectionsInserted(f func(parent *qtcore.QModelIndex, logicalFirst int, logicalLast int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "sectionsInserted", f)
// }

// void sectionsAboutToBeRemoved(const QModelIndex &, int, int)
// func (this *QHeaderView) InheritSectionsAboutToBeRemoved(f func(parent *qtcore.QModelIndex, logicalFirst int, logicalLast int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "sectionsAboutToBeRemoved", f)
// }

// void initialize()
// func (this *QHeaderView) InheritInitialize(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "initialize", f)
// }

// void initializeSections()
// func (this *QHeaderView) InheritInitializeSections(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "initializeSections", f)
// }

// void currentChanged(const QModelIndex &, const QModelIndex &)
// func (this *QHeaderView) InheritCurrentChanged(f func(current *qtcore.QModelIndex, old *qtcore.QModelIndex) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "currentChanged", f)
// }

// bool event(QEvent *)
// func (this *QHeaderView) InheritEvent(f func(e *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// void paintEvent(QPaintEvent *)
// func (this *QHeaderView) InheritPaintEvent(f func(e *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }

// void mousePressEvent(QMouseEvent *)
// func (this *QHeaderView) InheritMousePressEvent(f func(e *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mousePressEvent", f)
// }

// void mouseMoveEvent(QMouseEvent *)
// func (this *QHeaderView) InheritMouseMoveEvent(f func(e *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseMoveEvent", f)
// }

// void mouseReleaseEvent(QMouseEvent *)
// func (this *QHeaderView) InheritMouseReleaseEvent(f func(e *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseReleaseEvent", f)
// }

// void mouseDoubleClickEvent(QMouseEvent *)
// func (this *QHeaderView) InheritMouseDoubleClickEvent(f func(e *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseDoubleClickEvent", f)
// }

// bool viewportEvent(QEvent *)
// func (this *QHeaderView) InheritViewportEvent(f func(e *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "viewportEvent", f)
// }

// void paintSection(QPainter *, const QRect &, int)
// func (this *QHeaderView) InheritPaintSection(f func(painter *qtgui.QPainter/*777 QPainter **/, rect *qtcore.QRect, logicalIndex int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintSection", f)
// }

// QSize sectionSizeFromContents(int)
// func (this *QHeaderView) InheritSectionSizeFromContents(f func(logicalIndex int) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "sectionSizeFromContents", f)
// }

// int horizontalOffset()
// func (this *QHeaderView) InheritHorizontalOffset(f func() int) {
//  qtrt.SetAllInheritCallback(this, "horizontalOffset", f)
// }

// int verticalOffset()
// func (this *QHeaderView) InheritVerticalOffset(f func() int) {
//  qtrt.SetAllInheritCallback(this, "verticalOffset", f)
// }

// void updateGeometries()
// func (this *QHeaderView) InheritUpdateGeometries(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "updateGeometries", f)
// }

// void scrollContentsBy(int, int)
// func (this *QHeaderView) InheritScrollContentsBy(f func(dx int, dy int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "scrollContentsBy", f)
// }

// void rowsInserted(const QModelIndex &, int, int)
// func (this *QHeaderView) InheritRowsInserted(f func(parent *qtcore.QModelIndex, start int, end_ int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "rowsInserted", f)
// }

// QRect visualRect(const QModelIndex &)
// func (this *QHeaderView) InheritVisualRect(f func(index *qtcore.QModelIndex) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "visualRect", f)
// }

// void scrollTo(const QModelIndex &, QAbstractItemView::ScrollHint)
// func (this *QHeaderView) InheritScrollTo(f func(index *qtcore.QModelIndex, hint int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "scrollTo", f)
// }

// QModelIndex indexAt(const QPoint &)
// func (this *QHeaderView) InheritIndexAt(f func(p *qtcore.QPoint) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "indexAt", f)
// }

// bool isIndexHidden(const QModelIndex &)
// func (this *QHeaderView) InheritIsIndexHidden(f func(index *qtcore.QModelIndex) bool) {
//  qtrt.SetAllInheritCallback(this, "isIndexHidden", f)
// }

// QModelIndex moveCursor(QAbstractItemView::CursorAction, Qt::KeyboardModifiers)
// func (this *QHeaderView) InheritMoveCursor(f func(arg0 int, arg1 int) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "moveCursor", f)
// }

// void setSelection(const QRect &, QItemSelectionModel::SelectionFlags)
// func (this *QHeaderView) InheritSetSelection(f func(rect *qtcore.QRect, flags int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "setSelection", f)
// }

// QRegion visualRegionForSelection(const QItemSelection &)
// func (this *QHeaderView) InheritVisualRegionForSelection(f func(selection *qtcore.QItemSelection) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "visualRegionForSelection", f)
// }

// void initStyleOption(QStyleOptionHeader *)
// func (this *QHeaderView) InheritInitStyleOption(f func(option *QStyleOptionHeader/*777 QStyleOptionHeader **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "initStyleOption", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QHeaderView)=48
pub struct QHeaderView {
  qbase: QAbstractItemView,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QHeaderView_ITF interface {
//    QAbstractItemView_ITF
//    QHeaderView_PTR() *QHeaderView
//}
//func (ptr *QHeaderView) QHeaderView_PTR() *QHeaderView { return ptr }

impl /*struct*/ QHeaderView {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QHeaderView {
    return QHeaderView{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QHeaderView {
//  type Target = QHeaderViewBASE;
//
//  fn deref(&self) -> &QHeaderViewBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QHeaderViewBASE> for QHeaderView {
//  fn as_ref(& self) -> & QHeaderViewBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qheaderview.h:55
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QHeaderView {
  pub fn metaObject_0<RetType, T: QHeaderView_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QHeaderView_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QHeaderView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:77
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QHeaderView(Qt::Orientation, QWidget *)

/*
Creates a new generic header with the given orientation and parent.
*/
// QHeaderView(Qt::Orientation, QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QHeaderView {
  pub fn QHeaderView_0<T: QHeaderView_QHeaderView_0>(value: T) -> QHeaderView {
    let rsthis = value.QHeaderView_0();
    return rsthis;
    // return 1;
  }
}

pub trait QHeaderView_QHeaderView_0 {
  fn QHeaderView_0(self) -> QHeaderView;
}
// QHeaderView(Qt::Orientation, QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QHeaderView_QHeaderView_0 for (i32,usize) {
  fn QHeaderView_0(self) -> QHeaderView {
    // unsafe{_ZN11QHeaderViewC2EN2Qt11OrientationEP7QWidget()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QHeaderViewC2EN2Qt11OrientationEP7QWidget", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QHeaderView{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:78
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QHeaderView()

/*

*/
pub fn DeleteQHeaderView(this :*mut QHeaderView) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QHeaderViewD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qheaderview.h:80
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setModel(QAbstractItemModel *)

/*
Reimplemented from QAbstractItemView::setModel().
*/
impl /*struct*/ QHeaderView {
  pub fn setModel_0<RetType, T: QHeaderView_setModel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setModel_0(self);
    // return 1;
  }
}
pub trait QHeaderView_setModel_0<RetType> {
  fn setModel_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_setModel_0<(/*void*/)> for (usize) {
  fn setModel_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView8setModelEP18QAbstractItemModel", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:82
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::Orientation orientation() const

/*
Returns the orientation of the header.

See also Qt::Orientation.
*/
impl /*struct*/ QHeaderView {
  pub fn orientation_0<RetType, T: QHeaderView_orientation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.orientation_0(self);
    // return 1;
  }
}
pub trait QHeaderView_orientation_0<RetType> {
  fn orientation_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_orientation_0<i32> for () {
  fn orientation_0(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView11orientationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:83
// index:0
// Public Visibility=Default Availability=Available
// [4] int offset() const

/*
Returns the offset of the header: this is the header's left-most (or top-most for vertical headers) visible pixel.

See also setOffset().
*/
impl /*struct*/ QHeaderView {
  pub fn offset_0<RetType, T: QHeaderView_offset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.offset_0(self);
    // return 1;
  }
}
pub trait QHeaderView_offset_0<RetType> {
  fn offset_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_offset_0<i32> for () {
  fn offset_0(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView6offsetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:84
// index:0
// Public Visibility=Default Availability=Available
// [4] int length() const

/*
Returns the length along the orientation of the header.

See also sizeHint(), setSectionResizeMode(), and offset().
*/
impl /*struct*/ QHeaderView {
  pub fn length_0<RetType, T: QHeaderView_length_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.length_0(self);
    // return 1;
  }
}
pub trait QHeaderView_length_0<RetType> {
  fn length_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_length_0<i32> for () {
  fn length_0(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView6lengthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:85
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Reimplemented from QWidget::sizeHint().

Returns a suitable size hint for this header.

See also sectionSizeHint().
*/
impl /*struct*/ QHeaderView {
  pub fn sizeHint_0<RetType, T: QHeaderView_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QHeaderView_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QHeaderView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:86
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setVisible(bool)

/*
Reimplemented from QWidget::setVisible().
*/
impl /*struct*/ QHeaderView {
  pub fn setVisible_0<RetType, T: QHeaderView_setVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVisible_0(self);
    // return 1;
  }
}
pub trait QHeaderView_setVisible_0<RetType> {
  fn setVisible_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_setVisible_0<(/*void*/)> for (bool) {
  fn setVisible_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView10setVisibleEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:87
// index:0
// Public Visibility=Default Availability=Available
// [4] int sectionSizeHint(int) const

/*
Returns a suitable size hint for the section specified by logicalIndex.

Qt::SizeHintRole

See also sizeHint(), defaultSectionSize(), minimumSectionSize(), and maximumSectionSize().
*/
impl /*struct*/ QHeaderView {
  pub fn sectionSizeHint_0<RetType, T: QHeaderView_sectionSizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sectionSizeHint_0(self);
    // return 1;
  }
}
pub trait QHeaderView_sectionSizeHint_0<RetType> {
  fn sectionSizeHint_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_sectionSizeHint_0<i32> for (i32) {
  fn sectionSizeHint_0(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView15sectionSizeHintEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:89
// index:0
// Public Visibility=Default Availability=Available
// [4] int visualIndexAt(int) const

/*
Returns the visual index of the section that covers the given position in the viewport.

See also logicalIndexAt().
*/
impl /*struct*/ QHeaderView {
  pub fn visualIndexAt_0<RetType, T: QHeaderView_visualIndexAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.visualIndexAt_0(self);
    // return 1;
  }
}
pub trait QHeaderView_visualIndexAt_0<RetType> {
  fn visualIndexAt_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_visualIndexAt_0<i32> for (i32) {
  fn visualIndexAt_0(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView13visualIndexAtEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:90
// index:0
// Public Visibility=Default Availability=Available
// [4] int logicalIndexAt(int) const

/*
Returns the section that covers the given position in the viewport.

See also visualIndexAt() and isSectionHidden().
*/
impl /*struct*/ QHeaderView {
  pub fn logicalIndexAt_0<RetType, T: QHeaderView_logicalIndexAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.logicalIndexAt_0(self);
    // return 1;
  }
}
pub trait QHeaderView_logicalIndexAt_0<RetType> {
  fn logicalIndexAt_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_logicalIndexAt_0<i32> for (i32) {
  fn logicalIndexAt_0(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView14logicalIndexAtEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:92
// index:1
// Public inline Visibility=Default Availability=Available
// [4] int logicalIndexAt(int, int) const

/*
Returns the section that covers the given position in the viewport.

See also visualIndexAt() and isSectionHidden().
*/
impl /*struct*/ QHeaderView {
  pub fn logicalIndexAt_1<RetType, T: QHeaderView_logicalIndexAt_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.logicalIndexAt_1(self);
    // return 1;
  }
}
pub trait QHeaderView_logicalIndexAt_1<RetType> {
  fn logicalIndexAt_1(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_logicalIndexAt_1<i32> for (i32,i32) {
  fn logicalIndexAt_1(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView14logicalIndexAtEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:93
// index:2
// Public inline Visibility=Default Availability=Available
// [4] int logicalIndexAt(const QPoint &) const

/*
Returns the section that covers the given position in the viewport.

See also visualIndexAt() and isSectionHidden().
*/
impl /*struct*/ QHeaderView {
  pub fn logicalIndexAt_2<RetType, T: QHeaderView_logicalIndexAt_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.logicalIndexAt_2(self);
    // return 1;
  }
}
pub trait QHeaderView_logicalIndexAt_2<RetType> {
  fn logicalIndexAt_2(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_logicalIndexAt_2<i32> for (usize) {
  fn logicalIndexAt_2(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView14logicalIndexAtERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:95
// index:0
// Public Visibility=Default Availability=Available
// [4] int sectionSize(int) const

/*
Returns the width (or height for vertical headers) of the given logicalIndex.

See also length(), setSectionResizeMode(), and defaultSectionSize().
*/
impl /*struct*/ QHeaderView {
  pub fn sectionSize_0<RetType, T: QHeaderView_sectionSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sectionSize_0(self);
    // return 1;
  }
}
pub trait QHeaderView_sectionSize_0<RetType> {
  fn sectionSize_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_sectionSize_0<i32> for (i32) {
  fn sectionSize_0(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView11sectionSizeEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:96
// index:0
// Public Visibility=Default Availability=Available
// [4] int sectionPosition(int) const

/*
Returns the section position of the given logicalIndex, or -1 if the section is hidden. The position is measured in pixels from the first visible item's top-left corner to the top-left corner of the item with logicalIndex. The measurement is along the x-axis for horizontal headers and along the y-axis for vertical headers.

See also sectionViewportPosition().
*/
impl /*struct*/ QHeaderView {
  pub fn sectionPosition_0<RetType, T: QHeaderView_sectionPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sectionPosition_0(self);
    // return 1;
  }
}
pub trait QHeaderView_sectionPosition_0<RetType> {
  fn sectionPosition_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_sectionPosition_0<i32> for (i32) {
  fn sectionPosition_0(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView15sectionPositionEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:97
// index:0
// Public Visibility=Default Availability=Available
// [4] int sectionViewportPosition(int) const

/*
Returns the section viewport position of the given logicalIndex.

If the section is hidden, the return value is undefined.

See also sectionPosition() and isSectionHidden().
*/
impl /*struct*/ QHeaderView {
  pub fn sectionViewportPosition_0<RetType, T: QHeaderView_sectionViewportPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sectionViewportPosition_0(self);
    // return 1;
  }
}
pub trait QHeaderView_sectionViewportPosition_0<RetType> {
  fn sectionViewportPosition_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_sectionViewportPosition_0<i32> for (i32) {
  fn sectionViewportPosition_0(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView23sectionViewportPositionEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:99
// index:0
// Public Visibility=Default Availability=Available
// [-2] void moveSection(int, int)

/*
Moves the section at visual index from to occupy visual index to.

See also sectionsMoved().
*/
impl /*struct*/ QHeaderView {
  pub fn moveSection_0<RetType, T: QHeaderView_moveSection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveSection_0(self);
    // return 1;
  }
}
pub trait QHeaderView_moveSection_0<RetType> {
  fn moveSection_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_moveSection_0<(/*void*/)> for (i32,i32) {
  fn moveSection_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView11moveSectionEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:100
// index:0
// Public Visibility=Default Availability=Available
// [-2] void swapSections(int, int)

/*
Swaps the section at visual index first with the section at visual index second.

This function was introduced in  Qt 4.2.

See also moveSection().
*/
impl /*struct*/ QHeaderView {
  pub fn swapSections_0<RetType, T: QHeaderView_swapSections_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swapSections_0(self);
    // return 1;
  }
}
pub trait QHeaderView_swapSections_0<RetType> {
  fn swapSections_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_swapSections_0<(/*void*/)> for (i32,i32) {
  fn swapSections_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView12swapSectionsEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:101
// index:0
// Public Visibility=Default Availability=Available
// [-2] void resizeSection(int, int)

/*
Resizes the section specified by logicalIndex to size measured in pixels. The size parameter must be a value larger or equal to zero. A size equal to zero is however not recommended. In that situation hideSection should be used instead.

See also sectionResized(), resizeMode(), sectionSize(), and hideSection().
*/
impl /*struct*/ QHeaderView {
  pub fn resizeSection_0<RetType, T: QHeaderView_resizeSection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeSection_0(self);
    // return 1;
  }
}
pub trait QHeaderView_resizeSection_0<RetType> {
  fn resizeSection_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_resizeSection_0<(/*void*/)> for (i32,i32) {
  fn resizeSection_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView13resizeSectionEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:102
// index:0
// Public Visibility=Default Availability=Available
// [-2] void resizeSections(QHeaderView::ResizeMode)

/*
Resizes the sections according to the given mode, ignoring the current resize mode.

See also resizeMode() and sectionResized().
*/
impl /*struct*/ QHeaderView {
  pub fn resizeSections_0<RetType, T: QHeaderView_resizeSections_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeSections_0(self);
    // return 1;
  }
}
pub trait QHeaderView_resizeSections_0<RetType> {
  fn resizeSections_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_resizeSections_0<(/*void*/)> for (i32) {
  fn resizeSections_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView14resizeSectionsENS_10ResizeModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:206
// index:1
// Protected Visibility=Default Availability=Available
// [-2] void resizeSections()

/*
Resizes the sections according to the given mode, ignoring the current resize mode.

See also resizeMode() and sectionResized().
*/
impl /*struct*/ QHeaderView {
  pub fn resizeSections_1<RetType, T: QHeaderView_resizeSections_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeSections_1(self);
    // return 1;
  }
}
pub trait QHeaderView_resizeSections_1<RetType> {
  fn resizeSections_1(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_resizeSections_1<(/*void*/)> for () {
  fn resizeSections_1(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QHeaderView14resizeSectionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:104
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isSectionHidden(int) const

/*
Returns true if the section specified by logicalIndex is explicitly hidden from the user; otherwise returns false.

See also hideSection(), showSection(), setSectionHidden(), and hiddenSectionCount().
*/
impl /*struct*/ QHeaderView {
  pub fn isSectionHidden_0<RetType, T: QHeaderView_isSectionHidden_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isSectionHidden_0(self);
    // return 1;
  }
}
pub trait QHeaderView_isSectionHidden_0<RetType> {
  fn isSectionHidden_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_isSectionHidden_0<bool> for (i32) {
  fn isSectionHidden_0(self , rsthis: & QHeaderView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView15isSectionHiddenEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:105
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSectionHidden(int, bool)

/*
If hide is true the section specified by logicalIndex is hidden; otherwise the section is shown.

See also isSectionHidden() and hiddenSectionCount().
*/
impl /*struct*/ QHeaderView {
  pub fn setSectionHidden_0<RetType, T: QHeaderView_setSectionHidden_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSectionHidden_0(self);
    // return 1;
  }
}
pub trait QHeaderView_setSectionHidden_0<RetType> {
  fn setSectionHidden_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_setSectionHidden_0<(/*void*/)> for (i32,bool) {
  fn setSectionHidden_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView16setSectionHiddenEib", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:106
// index:0
// Public Visibility=Default Availability=Available
// [4] int hiddenSectionCount() const

/*
Returns the number of sections in the header that has been hidden.

This function was introduced in  Qt 4.1.

See also setSectionHidden() and isSectionHidden().
*/
impl /*struct*/ QHeaderView {
  pub fn hiddenSectionCount_0<RetType, T: QHeaderView_hiddenSectionCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hiddenSectionCount_0(self);
    // return 1;
  }
}
pub trait QHeaderView_hiddenSectionCount_0<RetType> {
  fn hiddenSectionCount_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_hiddenSectionCount_0<i32> for () {
  fn hiddenSectionCount_0(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView18hiddenSectionCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:108
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void hideSection(int)

/*
Hides the section specified by logicalIndex.

See also showSection(), isSectionHidden(), hiddenSectionCount(), and setSectionHidden().
*/
impl /*struct*/ QHeaderView {
  pub fn hideSection_0<RetType, T: QHeaderView_hideSection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hideSection_0(self);
    // return 1;
  }
}
pub trait QHeaderView_hideSection_0<RetType> {
  fn hideSection_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_hideSection_0<(/*void*/)> for (i32) {
  fn hideSection_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView11hideSectionEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:109
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void showSection(int)

/*
Shows the section specified by logicalIndex.

See also hideSection(), isSectionHidden(), hiddenSectionCount(), and setSectionHidden().
*/
impl /*struct*/ QHeaderView {
  pub fn showSection_0<RetType, T: QHeaderView_showSection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showSection_0(self);
    // return 1;
  }
}
pub trait QHeaderView_showSection_0<RetType> {
  fn showSection_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_showSection_0<(/*void*/)> for (i32) {
  fn showSection_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView11showSectionEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:111
// index:0
// Public Visibility=Default Availability=Available
// [4] int count() const

/*
Returns the number of sections in the header.

See also sectionCountChanged() and length().
*/
impl /*struct*/ QHeaderView {
  pub fn count_0<RetType, T: QHeaderView_count_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.count_0(self);
    // return 1;
  }
}
pub trait QHeaderView_count_0<RetType> {
  fn count_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_count_0<i32> for () {
  fn count_0(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView5countEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:112
// index:0
// Public Visibility=Default Availability=Available
// [4] int visualIndex(int) const

/*
Returns the visual index position of the section specified by the given logicalIndex, or -1 otherwise.

Hidden sections still have valid visual indexes.

See also logicalIndex().
*/
impl /*struct*/ QHeaderView {
  pub fn visualIndex_0<RetType, T: QHeaderView_visualIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.visualIndex_0(self);
    // return 1;
  }
}
pub trait QHeaderView_visualIndex_0<RetType> {
  fn visualIndex_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_visualIndex_0<i32> for (i32) {
  fn visualIndex_0(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView11visualIndexEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:113
// index:0
// Public Visibility=Default Availability=Available
// [4] int logicalIndex(int) const

/*
Returns the logicalIndex for the section at the given visualIndex position, or -1 if visualIndex < 0 or visualIndex >= QHeaderView::count().

Note that the visualIndex is not affected by hidden sections.

See also visualIndex() and sectionPosition().
*/
impl /*struct*/ QHeaderView {
  pub fn logicalIndex_0<RetType, T: QHeaderView_logicalIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.logicalIndex_0(self);
    // return 1;
  }
}
pub trait QHeaderView_logicalIndex_0<RetType> {
  fn logicalIndex_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_logicalIndex_0<i32> for (i32) {
  fn logicalIndex_0(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView12logicalIndexEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:115
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSectionsMovable(bool)

/*
If movable is true, the header may be moved by the user; otherwise it is fixed in place.

This function was introduced in  Qt 5.0.

See also sectionsMovable() and sectionMoved().
*/
impl /*struct*/ QHeaderView {
  pub fn setSectionsMovable_0<RetType, T: QHeaderView_setSectionsMovable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSectionsMovable_0(self);
    // return 1;
  }
}
pub trait QHeaderView_setSectionsMovable_0<RetType> {
  fn setSectionsMovable_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_setSectionsMovable_0<(/*void*/)> for (bool) {
  fn setSectionsMovable_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView18setSectionsMovableEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:116
// index:0
// Public Visibility=Default Availability=Available
// [1] bool sectionsMovable() const

/*
Returns true if the header can be moved by the user; otherwise returns false.

This function was introduced in  Qt 5.0.

See also setSectionsMovable().
*/
impl /*struct*/ QHeaderView {
  pub fn sectionsMovable_0<RetType, T: QHeaderView_sectionsMovable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sectionsMovable_0(self);
    // return 1;
  }
}
pub trait QHeaderView_sectionsMovable_0<RetType> {
  fn sectionsMovable_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_sectionsMovable_0<bool> for () {
  fn sectionsMovable_0(self , rsthis: & QHeaderView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView15sectionsMovableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:122
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSectionsClickable(bool)

/*
If clickable is true, the header will respond to single clicks.

This function was introduced in  Qt 5.0.

See also sectionsClickable(), sectionClicked(), sectionPressed(), and setSortIndicatorShown().
*/
impl /*struct*/ QHeaderView {
  pub fn setSectionsClickable_0<RetType, T: QHeaderView_setSectionsClickable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSectionsClickable_0(self);
    // return 1;
  }
}
pub trait QHeaderView_setSectionsClickable_0<RetType> {
  fn setSectionsClickable_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_setSectionsClickable_0<(/*void*/)> for (bool) {
  fn setSectionsClickable_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView20setSectionsClickableEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:123
// index:0
// Public Visibility=Default Availability=Available
// [1] bool sectionsClickable() const

/*
Returns true if the header is clickable; otherwise returns false. A clickable header could be set up to allow the user to change the representation of the data in the view related to the header.

This function was introduced in  Qt 5.0.

See also setSectionsClickable().
*/
impl /*struct*/ QHeaderView {
  pub fn sectionsClickable_0<RetType, T: QHeaderView_sectionsClickable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sectionsClickable_0(self);
    // return 1;
  }
}
pub trait QHeaderView_sectionsClickable_0<RetType> {
  fn sectionsClickable_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_sectionsClickable_0<bool> for () {
  fn sectionsClickable_0(self , rsthis: & QHeaderView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView17sectionsClickableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:129
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setHighlightSections(bool)

/*

*/
impl /*struct*/ QHeaderView {
  pub fn setHighlightSections_0<RetType, T: QHeaderView_setHighlightSections_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHighlightSections_0(self);
    // return 1;
  }
}
pub trait QHeaderView_setHighlightSections_0<RetType> {
  fn setHighlightSections_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_setHighlightSections_0<(/*void*/)> for (bool) {
  fn setHighlightSections_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView20setHighlightSectionsEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:130
// index:0
// Public Visibility=Default Availability=Available
// [1] bool highlightSections() const

/*

*/
impl /*struct*/ QHeaderView {
  pub fn highlightSections_0<RetType, T: QHeaderView_highlightSections_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.highlightSections_0(self);
    // return 1;
  }
}
pub trait QHeaderView_highlightSections_0<RetType> {
  fn highlightSections_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_highlightSections_0<bool> for () {
  fn highlightSections_0(self , rsthis: & QHeaderView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView17highlightSectionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:132
// index:0
// Public Visibility=Default Availability=Available
// [4] QHeaderView::ResizeMode sectionResizeMode(int) const

/*
Returns the resize mode that applies to the section specified by the given logicalIndex.

This function was introduced in  Qt 5.0.

See also setSectionResizeMode().
*/
impl /*struct*/ QHeaderView {
  pub fn sectionResizeMode_0<RetType, T: QHeaderView_sectionResizeMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sectionResizeMode_0(self);
    // return 1;
  }
}
pub trait QHeaderView_sectionResizeMode_0<RetType> {
  fn sectionResizeMode_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_sectionResizeMode_0<i32> for (i32) {
  fn sectionResizeMode_0(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView17sectionResizeModeEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:133
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSectionResizeMode(QHeaderView::ResizeMode)

/*
Sets the constraints on how the header can be resized to those described by the given mode.

This function was introduced in  Qt 5.0.

See also sectionResizeMode(), resizeMode(), length(), and sectionResized().
*/
impl /*struct*/ QHeaderView {
  pub fn setSectionResizeMode_0<RetType, T: QHeaderView_setSectionResizeMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSectionResizeMode_0(self);
    // return 1;
  }
}
pub trait QHeaderView_setSectionResizeMode_0<RetType> {
  fn setSectionResizeMode_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_setSectionResizeMode_0<(/*void*/)> for (i32) {
  fn setSectionResizeMode_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView20setSectionResizeModeENS_10ResizeModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:134
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setSectionResizeMode(int, QHeaderView::ResizeMode)

/*
Sets the constraints on how the header can be resized to those described by the given mode.

This function was introduced in  Qt 5.0.

See also sectionResizeMode(), resizeMode(), length(), and sectionResized().
*/
impl /*struct*/ QHeaderView {
  pub fn setSectionResizeMode_1<RetType, T: QHeaderView_setSectionResizeMode_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSectionResizeMode_1(self);
    // return 1;
  }
}
pub trait QHeaderView_setSectionResizeMode_1<RetType> {
  fn setSectionResizeMode_1(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_setSectionResizeMode_1<(/*void*/)> for (i32,i32) {
  fn setSectionResizeMode_1(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView20setSectionResizeModeEiNS_10ResizeModeE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:136
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setResizeContentsPrecision(int)

/*
Sets how precise QHeaderView should calculate the size when ResizeToContents is used. A low value will provide a less accurate but fast auto resize while a higher value will provide a more accurate resize that however can be slow.

The number precision specifies how many sections that should be consider when calculating the preferred size.

The default value is 1000 meaning that a horizontal column with auto-resize will look at maximum 1000 rows on calculating when doing an auto resize.

Special value 0 means that it will look at only the visible area. Special value -1 will imply looking at all elements.

This value is used in QTableView::sizeHintForColumn(), QTableView::sizeHintForRow() and QTreeView::sizeHintForColumn(). Reimplementing these functions can make this function not having an effect.

This function was introduced in  Qt 5.2.

See also resizeContentsPrecision(), setSectionResizeMode(), resizeSections(), QTableView::sizeHintForColumn(), QTableView::sizeHintForRow(), and QTreeView::sizeHintForColumn().
*/
impl /*struct*/ QHeaderView {
  pub fn setResizeContentsPrecision_0<RetType, T: QHeaderView_setResizeContentsPrecision_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setResizeContentsPrecision_0(self);
    // return 1;
  }
}
pub trait QHeaderView_setResizeContentsPrecision_0<RetType> {
  fn setResizeContentsPrecision_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_setResizeContentsPrecision_0<(/*void*/)> for (i32) {
  fn setResizeContentsPrecision_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView26setResizeContentsPrecisionEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:137
// index:0
// Public Visibility=Default Availability=Available
// [4] int resizeContentsPrecision() const

/*
Returns how precise QHeaderView will calculate on ResizeToContents.

This function was introduced in  Qt 5.2.

See also setResizeContentsPrecision() and setSectionResizeMode().
*/
impl /*struct*/ QHeaderView {
  pub fn resizeContentsPrecision_0<RetType, T: QHeaderView_resizeContentsPrecision_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeContentsPrecision_0(self);
    // return 1;
  }
}
pub trait QHeaderView_resizeContentsPrecision_0<RetType> {
  fn resizeContentsPrecision_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_resizeContentsPrecision_0<i32> for () {
  fn resizeContentsPrecision_0(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView23resizeContentsPrecisionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:148
// index:0
// Public Visibility=Default Availability=Available
// [4] int stretchSectionCount() const

/*
Returns the number of sections that are set to resize mode stretch. In views, this can be used to see if the headerview needs to resize the sections when the view's geometry changes.

This function was introduced in  Qt 4.1.

See also stretchLastSection and resizeMode().
*/
impl /*struct*/ QHeaderView {
  pub fn stretchSectionCount_0<RetType, T: QHeaderView_stretchSectionCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.stretchSectionCount_0(self);
    // return 1;
  }
}
pub trait QHeaderView_stretchSectionCount_0<RetType> {
  fn stretchSectionCount_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_stretchSectionCount_0<i32> for () {
  fn stretchSectionCount_0(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView19stretchSectionCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:150
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSortIndicatorShown(bool)

/*

*/
impl /*struct*/ QHeaderView {
  pub fn setSortIndicatorShown_0<RetType, T: QHeaderView_setSortIndicatorShown_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSortIndicatorShown_0(self);
    // return 1;
  }
}
pub trait QHeaderView_setSortIndicatorShown_0<RetType> {
  fn setSortIndicatorShown_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_setSortIndicatorShown_0<(/*void*/)> for (bool) {
  fn setSortIndicatorShown_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView21setSortIndicatorShownEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:151
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isSortIndicatorShown() const

/*

*/
impl /*struct*/ QHeaderView {
  pub fn isSortIndicatorShown_0<RetType, T: QHeaderView_isSortIndicatorShown_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isSortIndicatorShown_0(self);
    // return 1;
  }
}
pub trait QHeaderView_isSortIndicatorShown_0<RetType> {
  fn isSortIndicatorShown_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_isSortIndicatorShown_0<bool> for () {
  fn isSortIndicatorShown_0(self , rsthis: & QHeaderView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView20isSortIndicatorShownEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:153
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSortIndicator(int, Qt::SortOrder)

/*
Sets the sort indicator for the section specified by the given logicalIndex in the direction specified by order, and removes the sort indicator from any other section that was showing it.

logicalIndex may be -1, in which case no sort indicator will be shown and the model will return to its natural, unsorted order. Note that not all models support this and may even crash in this case.

See also sortIndicatorSection() and sortIndicatorOrder().
*/
impl /*struct*/ QHeaderView {
  pub fn setSortIndicator_0<RetType, T: QHeaderView_setSortIndicator_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSortIndicator_0(self);
    // return 1;
  }
}
pub trait QHeaderView_setSortIndicator_0<RetType> {
  fn setSortIndicator_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_setSortIndicator_0<(/*void*/)> for (i32,i32) {
  fn setSortIndicator_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView16setSortIndicatorEiN2Qt9SortOrderE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:154
// index:0
// Public Visibility=Default Availability=Available
// [4] int sortIndicatorSection() const

/*
Returns the logical index of the section that has a sort indicator. By default this is section 0.

See also setSortIndicator(), sortIndicatorOrder(), and setSortIndicatorShown().
*/
impl /*struct*/ QHeaderView {
  pub fn sortIndicatorSection_0<RetType, T: QHeaderView_sortIndicatorSection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sortIndicatorSection_0(self);
    // return 1;
  }
}
pub trait QHeaderView_sortIndicatorSection_0<RetType> {
  fn sortIndicatorSection_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_sortIndicatorSection_0<i32> for () {
  fn sortIndicatorSection_0(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView20sortIndicatorSectionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:155
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::SortOrder sortIndicatorOrder() const

/*
Returns the order for the sort indicator. If no section has a sort indicator the return value of this function is undefined.

See also setSortIndicator() and sortIndicatorSection().
*/
impl /*struct*/ QHeaderView {
  pub fn sortIndicatorOrder_0<RetType, T: QHeaderView_sortIndicatorOrder_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sortIndicatorOrder_0(self);
    // return 1;
  }
}
pub trait QHeaderView_sortIndicatorOrder_0<RetType> {
  fn sortIndicatorOrder_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_sortIndicatorOrder_0<i32> for () {
  fn sortIndicatorOrder_0(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView18sortIndicatorOrderEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:157
// index:0
// Public Visibility=Default Availability=Available
// [1] bool stretchLastSection() const

/*

*/
impl /*struct*/ QHeaderView {
  pub fn stretchLastSection_0<RetType, T: QHeaderView_stretchLastSection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.stretchLastSection_0(self);
    // return 1;
  }
}
pub trait QHeaderView_stretchLastSection_0<RetType> {
  fn stretchLastSection_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_stretchLastSection_0<bool> for () {
  fn stretchLastSection_0(self , rsthis: & QHeaderView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView18stretchLastSectionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:158
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStretchLastSection(bool)

/*

*/
impl /*struct*/ QHeaderView {
  pub fn setStretchLastSection_0<RetType, T: QHeaderView_setStretchLastSection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStretchLastSection_0(self);
    // return 1;
  }
}
pub trait QHeaderView_setStretchLastSection_0<RetType> {
  fn setStretchLastSection_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_setStretchLastSection_0<(/*void*/)> for (bool) {
  fn setStretchLastSection_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView21setStretchLastSectionEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:160
// index:0
// Public Visibility=Default Availability=Available
// [1] bool cascadingSectionResizes() const

/*

*/
impl /*struct*/ QHeaderView {
  pub fn cascadingSectionResizes_0<RetType, T: QHeaderView_cascadingSectionResizes_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cascadingSectionResizes_0(self);
    // return 1;
  }
}
pub trait QHeaderView_cascadingSectionResizes_0<RetType> {
  fn cascadingSectionResizes_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_cascadingSectionResizes_0<bool> for () {
  fn cascadingSectionResizes_0(self , rsthis: & QHeaderView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView23cascadingSectionResizesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:161
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCascadingSectionResizes(bool)

/*

*/
impl /*struct*/ QHeaderView {
  pub fn setCascadingSectionResizes_0<RetType, T: QHeaderView_setCascadingSectionResizes_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCascadingSectionResizes_0(self);
    // return 1;
  }
}
pub trait QHeaderView_setCascadingSectionResizes_0<RetType> {
  fn setCascadingSectionResizes_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_setCascadingSectionResizes_0<(/*void*/)> for (bool) {
  fn setCascadingSectionResizes_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView26setCascadingSectionResizesEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:163
// index:0
// Public Visibility=Default Availability=Available
// [4] int defaultSectionSize() const

/*

*/
impl /*struct*/ QHeaderView {
  pub fn defaultSectionSize_0<RetType, T: QHeaderView_defaultSectionSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.defaultSectionSize_0(self);
    // return 1;
  }
}
pub trait QHeaderView_defaultSectionSize_0<RetType> {
  fn defaultSectionSize_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_defaultSectionSize_0<i32> for () {
  fn defaultSectionSize_0(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView18defaultSectionSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:164
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDefaultSectionSize(int)

/*

*/
impl /*struct*/ QHeaderView {
  pub fn setDefaultSectionSize_0<RetType, T: QHeaderView_setDefaultSectionSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDefaultSectionSize_0(self);
    // return 1;
  }
}
pub trait QHeaderView_setDefaultSectionSize_0<RetType> {
  fn setDefaultSectionSize_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_setDefaultSectionSize_0<(/*void*/)> for (i32) {
  fn setDefaultSectionSize_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView21setDefaultSectionSizeEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:165
// index:0
// Public Visibility=Default Availability=Available
// [-2] void resetDefaultSectionSize()

/*

*/
impl /*struct*/ QHeaderView {
  pub fn resetDefaultSectionSize_0<RetType, T: QHeaderView_resetDefaultSectionSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resetDefaultSectionSize_0(self);
    // return 1;
  }
}
pub trait QHeaderView_resetDefaultSectionSize_0<RetType> {
  fn resetDefaultSectionSize_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_resetDefaultSectionSize_0<(/*void*/)> for () {
  fn resetDefaultSectionSize_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QHeaderView23resetDefaultSectionSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:167
// index:0
// Public Visibility=Default Availability=Available
// [4] int minimumSectionSize() const

/*

*/
impl /*struct*/ QHeaderView {
  pub fn minimumSectionSize_0<RetType, T: QHeaderView_minimumSectionSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumSectionSize_0(self);
    // return 1;
  }
}
pub trait QHeaderView_minimumSectionSize_0<RetType> {
  fn minimumSectionSize_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_minimumSectionSize_0<i32> for () {
  fn minimumSectionSize_0(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView18minimumSectionSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:168
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMinimumSectionSize(int)

/*

*/
impl /*struct*/ QHeaderView {
  pub fn setMinimumSectionSize_0<RetType, T: QHeaderView_setMinimumSectionSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMinimumSectionSize_0(self);
    // return 1;
  }
}
pub trait QHeaderView_setMinimumSectionSize_0<RetType> {
  fn setMinimumSectionSize_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_setMinimumSectionSize_0<(/*void*/)> for (i32) {
  fn setMinimumSectionSize_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView21setMinimumSectionSizeEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:169
// index:0
// Public Visibility=Default Availability=Available
// [4] int maximumSectionSize() const

/*

*/
impl /*struct*/ QHeaderView {
  pub fn maximumSectionSize_0<RetType, T: QHeaderView_maximumSectionSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maximumSectionSize_0(self);
    // return 1;
  }
}
pub trait QHeaderView_maximumSectionSize_0<RetType> {
  fn maximumSectionSize_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_maximumSectionSize_0<i32> for () {
  fn maximumSectionSize_0(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView18maximumSectionSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:170
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMaximumSectionSize(int)

/*

*/
impl /*struct*/ QHeaderView {
  pub fn setMaximumSectionSize_0<RetType, T: QHeaderView_setMaximumSectionSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMaximumSectionSize_0(self);
    // return 1;
  }
}
pub trait QHeaderView_setMaximumSectionSize_0<RetType> {
  fn setMaximumSectionSize_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_setMaximumSectionSize_0<(/*void*/)> for (i32) {
  fn setMaximumSectionSize_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView21setMaximumSectionSizeEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:172
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::Alignment defaultAlignment() const

/*

*/
impl /*struct*/ QHeaderView {
  pub fn defaultAlignment_0<RetType, T: QHeaderView_defaultAlignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.defaultAlignment_0(self);
    // return 1;
  }
}
pub trait QHeaderView_defaultAlignment_0<RetType> {
  fn defaultAlignment_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_defaultAlignment_0<i32> for () {
  fn defaultAlignment_0(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView16defaultAlignmentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:173
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDefaultAlignment(Qt::Alignment)

/*

*/
impl /*struct*/ QHeaderView {
  pub fn setDefaultAlignment_0<RetType, T: QHeaderView_setDefaultAlignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDefaultAlignment_0(self);
    // return 1;
  }
}
pub trait QHeaderView_setDefaultAlignment_0<RetType> {
  fn setDefaultAlignment_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_setDefaultAlignment_0<(/*void*/)> for (i32) {
  fn setDefaultAlignment_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView19setDefaultAlignmentE6QFlagsIN2Qt13AlignmentFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:175
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void doItemsLayout()

/*

*/
impl /*struct*/ QHeaderView {
  pub fn doItemsLayout_0<RetType, T: QHeaderView_doItemsLayout_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.doItemsLayout_0(self);
    // return 1;
  }
}
pub trait QHeaderView_doItemsLayout_0<RetType> {
  fn doItemsLayout_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_doItemsLayout_0<(/*void*/)> for () {
  fn doItemsLayout_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QHeaderView13doItemsLayoutEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:176
// index:0
// Public Visibility=Default Availability=Available
// [1] bool sectionsMoved() const

/*
Returns true if sections in the header has been moved; otherwise returns false;

See also moveSection().
*/
impl /*struct*/ QHeaderView {
  pub fn sectionsMoved_0<RetType, T: QHeaderView_sectionsMoved_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sectionsMoved_0(self);
    // return 1;
  }
}
pub trait QHeaderView_sectionsMoved_0<RetType> {
  fn sectionsMoved_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_sectionsMoved_0<bool> for () {
  fn sectionsMoved_0(self , rsthis: & QHeaderView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView13sectionsMovedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:177
// index:0
// Public Visibility=Default Availability=Available
// [1] bool sectionsHidden() const

/*
Returns true if sections in the header has been hidden; otherwise returns false;

This function was introduced in  Qt 4.1.

See also setSectionHidden().
*/
impl /*struct*/ QHeaderView {
  pub fn sectionsHidden_0<RetType, T: QHeaderView_sectionsHidden_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sectionsHidden_0(self);
    // return 1;
  }
}
pub trait QHeaderView_sectionsHidden_0<RetType> {
  fn sectionsHidden_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_sectionsHidden_0<bool> for () {
  fn sectionsHidden_0(self , rsthis: & QHeaderView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView14sectionsHiddenEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:180
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray saveState() const

/*
Saves the current state of this header view.

To restore the saved state, pass the return value to restoreState().

This function was introduced in  Qt 4.3.

See also restoreState().
*/
impl /*struct*/ QHeaderView {
  pub fn saveState_0<RetType, T: QHeaderView_saveState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.saveState_0(self);
    // return 1;
  }
}
pub trait QHeaderView_saveState_0<RetType> {
  fn saveState_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_saveState_0<usize> for () {
  fn saveState_0(self , rsthis: & QHeaderView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView9saveStateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:181
// index:0
// Public Visibility=Default Availability=Available
// [1] bool restoreState(const QByteArray &)

/*
Restores the state of this header view. This function returns true if the state was restored; otherwise returns false.

This function was introduced in  Qt 4.3.

See also saveState().
*/
impl /*struct*/ QHeaderView {
  pub fn restoreState_0<RetType, T: QHeaderView_restoreState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.restoreState_0(self);
    // return 1;
  }
}
pub trait QHeaderView_restoreState_0<RetType> {
  fn restoreState_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_restoreState_0<bool> for (usize) {
  fn restoreState_0(self , rsthis: & QHeaderView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QHeaderView12restoreStateERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:184
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void reset()

/*
Reimplemented from QAbstractItemView::reset().
*/
impl /*struct*/ QHeaderView {
  pub fn reset_0<RetType, T: QHeaderView_reset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.reset_0(self);
    // return 1;
  }
}
pub trait QHeaderView_reset_0<RetType> {
  fn reset_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_reset_0<(/*void*/)> for () {
  fn reset_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QHeaderView5resetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:187
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOffset(int)

/*
Sets the header's offset to offset.

See also offset() and length().
*/
impl /*struct*/ QHeaderView {
  pub fn setOffset_0<RetType, T: QHeaderView_setOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOffset_0(self);
    // return 1;
  }
}
pub trait QHeaderView_setOffset_0<RetType> {
  fn setOffset_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_setOffset_0<(/*void*/)> for (i32) {
  fn setOffset_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView9setOffsetEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:188
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOffsetToSectionPosition(int)

/*
Sets the offset to the start of the section at the given visualSectionNumber. visualSectionNumber is the actual visible section when hiddenSections are not considered. That is not always the same as visualIndex().

This function was introduced in  Qt 4.2.

See also setOffset() and sectionPosition().
*/
impl /*struct*/ QHeaderView {
  pub fn setOffsetToSectionPosition_0<RetType, T: QHeaderView_setOffsetToSectionPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOffsetToSectionPosition_0(self);
    // return 1;
  }
}
pub trait QHeaderView_setOffsetToSectionPosition_0<RetType> {
  fn setOffsetToSectionPosition_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_setOffsetToSectionPosition_0<(/*void*/)> for (i32) {
  fn setOffsetToSectionPosition_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView26setOffsetToSectionPositionEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:189
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOffsetToLastSection()

/*
Sets the offset to make the last section visible.

This function was introduced in  Qt 4.2.

See also setOffset(), sectionPosition(), and setOffsetToSectionPosition().
*/
impl /*struct*/ QHeaderView {
  pub fn setOffsetToLastSection_0<RetType, T: QHeaderView_setOffsetToLastSection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOffsetToLastSection_0(self);
    // return 1;
  }
}
pub trait QHeaderView_setOffsetToLastSection_0<RetType> {
  fn setOffsetToLastSection_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_setOffsetToLastSection_0<(/*void*/)> for () {
  fn setOffsetToLastSection_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QHeaderView22setOffsetToLastSectionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:190
// index:0
// Public Visibility=Default Availability=Available
// [-2] void headerDataChanged(Qt::Orientation, int, int)

/*
Updates the changed header sections with the given orientation, from logicalFirst to logicalLast inclusive.
*/
impl /*struct*/ QHeaderView {
  pub fn headerDataChanged_0<RetType, T: QHeaderView_headerDataChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.headerDataChanged_0(self);
    // return 1;
  }
}
pub trait QHeaderView_headerDataChanged_0<RetType> {
  fn headerDataChanged_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_headerDataChanged_0<(/*void*/)> for (i32,i32,i32) {
  fn headerDataChanged_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView17headerDataChangedEN2Qt11OrientationEii", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:193
// index:0
// Public Visibility=Default Availability=Available
// [-2] void sectionMoved(int, int, int)

/*
This signal is emitted when a section is moved. The section's logical index is specified by logicalIndex, the old index by oldVisualIndex, and the new index position by newVisualIndex.

See also moveSection().
*/
impl /*struct*/ QHeaderView {
  pub fn sectionMoved_0<RetType, T: QHeaderView_sectionMoved_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sectionMoved_0(self);
    // return 1;
  }
}
pub trait QHeaderView_sectionMoved_0<RetType> {
  fn sectionMoved_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_sectionMoved_0<(/*void*/)> for (i32,i32,i32) {
  fn sectionMoved_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView12sectionMovedEiii", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:194
// index:0
// Public Visibility=Default Availability=Available
// [-2] void sectionResized(int, int, int)

/*
This signal is emitted when a section is resized. The section's logical number is specified by logicalIndex, the old size by oldSize, and the new size by newSize.

See also resizeSection().
*/
impl /*struct*/ QHeaderView {
  pub fn sectionResized_0<RetType, T: QHeaderView_sectionResized_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sectionResized_0(self);
    // return 1;
  }
}
pub trait QHeaderView_sectionResized_0<RetType> {
  fn sectionResized_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_sectionResized_0<(/*void*/)> for (i32,i32,i32) {
  fn sectionResized_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView14sectionResizedEiii", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:195
// index:0
// Public Visibility=Default Availability=Available
// [-2] void sectionPressed(int)

/*
This signal is emitted when a section is pressed. The section's logical index is specified by logicalIndex.

See also setSectionsClickable().
*/
impl /*struct*/ QHeaderView {
  pub fn sectionPressed_0<RetType, T: QHeaderView_sectionPressed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sectionPressed_0(self);
    // return 1;
  }
}
pub trait QHeaderView_sectionPressed_0<RetType> {
  fn sectionPressed_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_sectionPressed_0<(/*void*/)> for (i32) {
  fn sectionPressed_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView14sectionPressedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:196
// index:0
// Public Visibility=Default Availability=Available
// [-2] void sectionClicked(int)

/*
This signal is emitted when a section is clicked. The section's logical index is specified by logicalIndex.

Note that the sectionPressed signal will also be emitted.

See also setSectionsClickable() and sectionPressed().
*/
impl /*struct*/ QHeaderView {
  pub fn sectionClicked_0<RetType, T: QHeaderView_sectionClicked_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sectionClicked_0(self);
    // return 1;
  }
}
pub trait QHeaderView_sectionClicked_0<RetType> {
  fn sectionClicked_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_sectionClicked_0<(/*void*/)> for (i32) {
  fn sectionClicked_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView14sectionClickedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:197
// index:0
// Public Visibility=Default Availability=Available
// [-2] void sectionEntered(int)

/*
This signal is emitted when the cursor moves over the section and the left mouse button is pressed. The section's logical index is specified by logicalIndex.

This function was introduced in  Qt 4.3.

See also setSectionsClickable() and sectionPressed().
*/
impl /*struct*/ QHeaderView {
  pub fn sectionEntered_0<RetType, T: QHeaderView_sectionEntered_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sectionEntered_0(self);
    // return 1;
  }
}
pub trait QHeaderView_sectionEntered_0<RetType> {
  fn sectionEntered_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_sectionEntered_0<(/*void*/)> for (i32) {
  fn sectionEntered_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView14sectionEnteredEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:198
// index:0
// Public Visibility=Default Availability=Available
// [-2] void sectionDoubleClicked(int)

/*
This signal is emitted when a section is double-clicked. The section's logical index is specified by logicalIndex.

See also setSectionsClickable().
*/
impl /*struct*/ QHeaderView {
  pub fn sectionDoubleClicked_0<RetType, T: QHeaderView_sectionDoubleClicked_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sectionDoubleClicked_0(self);
    // return 1;
  }
}
pub trait QHeaderView_sectionDoubleClicked_0<RetType> {
  fn sectionDoubleClicked_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_sectionDoubleClicked_0<(/*void*/)> for (i32) {
  fn sectionDoubleClicked_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView20sectionDoubleClickedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:199
// index:0
// Public Visibility=Default Availability=Available
// [-2] void sectionCountChanged(int, int)

/*
This signal is emitted when the number of sections changes, i.e., when sections are added or deleted. The original count is specified by oldCount, and the new count by newCount.

See also count(), length(), and headerDataChanged().
*/
impl /*struct*/ QHeaderView {
  pub fn sectionCountChanged_0<RetType, T: QHeaderView_sectionCountChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sectionCountChanged_0(self);
    // return 1;
  }
}
pub trait QHeaderView_sectionCountChanged_0<RetType> {
  fn sectionCountChanged_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_sectionCountChanged_0<(/*void*/)> for (i32,i32) {
  fn sectionCountChanged_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView19sectionCountChangedEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:200
// index:0
// Public Visibility=Default Availability=Available
// [-2] void sectionHandleDoubleClicked(int)

/*
This signal is emitted when a section is double-clicked. The section's logical index is specified by logicalIndex.

See also setSectionsClickable().
*/
impl /*struct*/ QHeaderView {
  pub fn sectionHandleDoubleClicked_0<RetType, T: QHeaderView_sectionHandleDoubleClicked_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sectionHandleDoubleClicked_0(self);
    // return 1;
  }
}
pub trait QHeaderView_sectionHandleDoubleClicked_0<RetType> {
  fn sectionHandleDoubleClicked_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_sectionHandleDoubleClicked_0<(/*void*/)> for (i32) {
  fn sectionHandleDoubleClicked_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView26sectionHandleDoubleClickedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:201
// index:0
// Public Visibility=Default Availability=Available
// [-2] void geometriesChanged()

/*
This signal is emitted when the header's geometries have changed.

This function was introduced in  Qt 4.2.
*/
impl /*struct*/ QHeaderView {
  pub fn geometriesChanged_0<RetType, T: QHeaderView_geometriesChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.geometriesChanged_0(self);
    // return 1;
  }
}
pub trait QHeaderView_geometriesChanged_0<RetType> {
  fn geometriesChanged_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_geometriesChanged_0<(/*void*/)> for () {
  fn geometriesChanged_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QHeaderView17geometriesChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:202
// index:0
// Public Visibility=Default Availability=Available
// [-2] void sortIndicatorChanged(int, Qt::SortOrder)

/*
This signal is emitted when the section containing the sort indicator or the order indicated is changed. The section's logical index is specified by logicalIndex and the sort order is specified by order.

This function was introduced in  Qt 4.3.

See also setSortIndicator().
*/
impl /*struct*/ QHeaderView {
  pub fn sortIndicatorChanged_0<RetType, T: QHeaderView_sortIndicatorChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sortIndicatorChanged_0(self);
    // return 1;
  }
}
pub trait QHeaderView_sortIndicatorChanged_0<RetType> {
  fn sortIndicatorChanged_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_sortIndicatorChanged_0<(/*void*/)> for (i32,i32) {
  fn sortIndicatorChanged_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView20sortIndicatorChangedEiN2Qt9SortOrderE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:205
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void updateSection(int)

/*

*/
impl /*struct*/ QHeaderView {
  pub fn updateSection_0<RetType, T: QHeaderView_updateSection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updateSection_0(self);
    // return 1;
  }
}
pub trait QHeaderView_updateSection_0<RetType> {
  fn updateSection_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_updateSection_0<(/*void*/)> for (i32) {
  fn updateSection_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView13updateSectionEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:207
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void sectionsInserted(const QModelIndex &, int, int)

/*
This slot is called when sections are inserted into the parent. logicalFirst and logicalLast indices signify where the new sections were inserted.

If only one section is inserted, logicalFirst and logicalLast will be the same.
*/
impl /*struct*/ QHeaderView {
  pub fn sectionsInserted_0<RetType, T: QHeaderView_sectionsInserted_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sectionsInserted_0(self);
    // return 1;
  }
}
pub trait QHeaderView_sectionsInserted_0<RetType> {
  fn sectionsInserted_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_sectionsInserted_0<(/*void*/)> for (usize,i32,i32) {
  fn sectionsInserted_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView16sectionsInsertedERK11QModelIndexii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:208
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void sectionsAboutToBeRemoved(const QModelIndex &, int, int)

/*
This slot is called when sections are removed from the parent. logicalFirst and logicalLast signify where the sections were removed.

If only one section is removed, logicalFirst and logicalLast will be the same.
*/
impl /*struct*/ QHeaderView {
  pub fn sectionsAboutToBeRemoved_0<RetType, T: QHeaderView_sectionsAboutToBeRemoved_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sectionsAboutToBeRemoved_0(self);
    // return 1;
  }
}
pub trait QHeaderView_sectionsAboutToBeRemoved_0<RetType> {
  fn sectionsAboutToBeRemoved_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_sectionsAboutToBeRemoved_0<(/*void*/)> for (usize,i32,i32) {
  fn sectionsAboutToBeRemoved_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView24sectionsAboutToBeRemovedERK11QModelIndexii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:212
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void initialize()

/*

*/
impl /*struct*/ QHeaderView {
  pub fn initialize_0<RetType, T: QHeaderView_initialize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.initialize_0(self);
    // return 1;
  }
}
pub trait QHeaderView_initialize_0<RetType> {
  fn initialize_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_initialize_0<(/*void*/)> for () {
  fn initialize_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QHeaderView10initializeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:214
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void initializeSections()

/*

*/
impl /*struct*/ QHeaderView {
  pub fn initializeSections_0<RetType, T: QHeaderView_initializeSections_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.initializeSections_0(self);
    // return 1;
  }
}
pub trait QHeaderView_initializeSections_0<RetType> {
  fn initializeSections_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_initializeSections_0<(/*void*/)> for () {
  fn initializeSections_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QHeaderView18initializeSectionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:215
// index:1
// Protected Visibility=Default Availability=Available
// [-2] void initializeSections(int, int)

/*

*/
impl /*struct*/ QHeaderView {
  pub fn initializeSections_1<RetType, T: QHeaderView_initializeSections_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.initializeSections_1(self);
    // return 1;
  }
}
pub trait QHeaderView_initializeSections_1<RetType> {
  fn initializeSections_1(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_initializeSections_1<(/*void*/)> for (i32,i32) {
  fn initializeSections_1(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView18initializeSectionsEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:216
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void currentChanged(const QModelIndex &, const QModelIndex &)

/*
Reimplemented from QAbstractItemView::currentChanged().
*/
impl /*struct*/ QHeaderView {
  pub fn currentChanged_0<RetType, T: QHeaderView_currentChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentChanged_0(self);
    // return 1;
  }
}
pub trait QHeaderView_currentChanged_0<RetType> {
  fn currentChanged_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_currentChanged_0<(/*void*/)> for (usize,usize) {
  fn currentChanged_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView14currentChangedERK11QModelIndexS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:218
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QHeaderView {
  pub fn event_0<RetType, T: QHeaderView_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QHeaderView_event_0<RetType> {
  fn event_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QHeaderView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QHeaderView5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:219
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Reimplemented from QWidget::paintEvent().
*/
impl /*struct*/ QHeaderView {
  pub fn paintEvent_0<RetType, T: QHeaderView_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QHeaderView_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:220
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mousePressEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mousePressEvent().
*/
impl /*struct*/ QHeaderView {
  pub fn mousePressEvent_0<RetType, T: QHeaderView_mousePressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mousePressEvent_0(self);
    // return 1;
  }
}
pub trait QHeaderView_mousePressEvent_0<RetType> {
  fn mousePressEvent_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_mousePressEvent_0<(/*void*/)> for (usize) {
  fn mousePressEvent_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView15mousePressEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:221
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseMoveEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseMoveEvent().
*/
impl /*struct*/ QHeaderView {
  pub fn mouseMoveEvent_0<RetType, T: QHeaderView_mouseMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseMoveEvent_0(self);
    // return 1;
  }
}
pub trait QHeaderView_mouseMoveEvent_0<RetType> {
  fn mouseMoveEvent_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_mouseMoveEvent_0<(/*void*/)> for (usize) {
  fn mouseMoveEvent_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView14mouseMoveEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:222
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseReleaseEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseReleaseEvent().
*/
impl /*struct*/ QHeaderView {
  pub fn mouseReleaseEvent_0<RetType, T: QHeaderView_mouseReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QHeaderView_mouseReleaseEvent_0<RetType> {
  fn mouseReleaseEvent_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_mouseReleaseEvent_0<(/*void*/)> for (usize) {
  fn mouseReleaseEvent_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView17mouseReleaseEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:223
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseDoubleClickEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseDoubleClickEvent().
*/
impl /*struct*/ QHeaderView {
  pub fn mouseDoubleClickEvent_0<RetType, T: QHeaderView_mouseDoubleClickEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseDoubleClickEvent_0(self);
    // return 1;
  }
}
pub trait QHeaderView_mouseDoubleClickEvent_0<RetType> {
  fn mouseDoubleClickEvent_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_mouseDoubleClickEvent_0<(/*void*/)> for (usize) {
  fn mouseDoubleClickEvent_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView21mouseDoubleClickEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:224
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool viewportEvent(QEvent *)

/*
Reimplemented from QAbstractScrollArea::viewportEvent().
*/
impl /*struct*/ QHeaderView {
  pub fn viewportEvent_0<RetType, T: QHeaderView_viewportEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.viewportEvent_0(self);
    // return 1;
  }
}
pub trait QHeaderView_viewportEvent_0<RetType> {
  fn viewportEvent_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_viewportEvent_0<bool> for (usize) {
  fn viewportEvent_0(self , rsthis: & QHeaderView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QHeaderView13viewportEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:226
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintSection(QPainter *, const QRect &, int) const

/*
Paints the section specified by the given logicalIndex, using the given painter and rect.

Normally, you do not have to call this function.
*/
impl /*struct*/ QHeaderView {
  pub fn paintSection_0<RetType, T: QHeaderView_paintSection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintSection_0(self);
    // return 1;
  }
}
pub trait QHeaderView_paintSection_0<RetType> {
  fn paintSection_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_paintSection_0<(/*void*/)> for (usize,usize,i32) {
  fn paintSection_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZNK11QHeaderView12paintSectionEP8QPainterRK5QRecti", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:227
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] QSize sectionSizeFromContents(int) const

/*
Returns the size of the contents of the section specified by the given logicalIndex.

See also defaultSectionSize().
*/
impl /*struct*/ QHeaderView {
  pub fn sectionSizeFromContents_0<RetType, T: QHeaderView_sectionSizeFromContents_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sectionSizeFromContents_0(self);
    // return 1;
  }
}
pub trait QHeaderView_sectionSizeFromContents_0<RetType> {
  fn sectionSizeFromContents_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_sectionSizeFromContents_0<usize> for (i32) {
  fn sectionSizeFromContents_0(self , rsthis: & QHeaderView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView23sectionSizeFromContentsEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:229
// index:0
// Protected virtual Visibility=Default Availability=Available
// [4] int horizontalOffset() const

/*
Reimplemented from QAbstractItemView::horizontalOffset().

Returns the horizontal offset of the header. This is 0 for vertical headers.

See also offset().
*/
impl /*struct*/ QHeaderView {
  pub fn horizontalOffset_0<RetType, T: QHeaderView_horizontalOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.horizontalOffset_0(self);
    // return 1;
  }
}
pub trait QHeaderView_horizontalOffset_0<RetType> {
  fn horizontalOffset_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_horizontalOffset_0<i32> for () {
  fn horizontalOffset_0(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView16horizontalOffsetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:230
// index:0
// Protected virtual Visibility=Default Availability=Available
// [4] int verticalOffset() const

/*
Reimplemented from QAbstractItemView::verticalOffset().

Returns the vertical offset of the header. This is 0 for horizontal headers.

See also offset().
*/
impl /*struct*/ QHeaderView {
  pub fn verticalOffset_0<RetType, T: QHeaderView_verticalOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.verticalOffset_0(self);
    // return 1;
  }
}
pub trait QHeaderView_verticalOffset_0<RetType> {
  fn verticalOffset_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_verticalOffset_0<i32> for () {
  fn verticalOffset_0(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView14verticalOffsetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:231
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void updateGeometries()

/*

*/
impl /*struct*/ QHeaderView {
  pub fn updateGeometries_0<RetType, T: QHeaderView_updateGeometries_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updateGeometries_0(self);
    // return 1;
  }
}
pub trait QHeaderView_updateGeometries_0<RetType> {
  fn updateGeometries_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_updateGeometries_0<(/*void*/)> for () {
  fn updateGeometries_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QHeaderView16updateGeometriesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:232
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void scrollContentsBy(int, int)

/*

*/
impl /*struct*/ QHeaderView {
  pub fn scrollContentsBy_0<RetType, T: QHeaderView_scrollContentsBy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scrollContentsBy_0(self);
    // return 1;
  }
}
pub trait QHeaderView_scrollContentsBy_0<RetType> {
  fn scrollContentsBy_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_scrollContentsBy_0<(/*void*/)> for (i32,i32) {
  fn scrollContentsBy_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView16scrollContentsByEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:235
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void rowsInserted(const QModelIndex &, int, int)

/*

*/
impl /*struct*/ QHeaderView {
  pub fn rowsInserted_0<RetType, T: QHeaderView_rowsInserted_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowsInserted_0(self);
    // return 1;
  }
}
pub trait QHeaderView_rowsInserted_0<RetType> {
  fn rowsInserted_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_rowsInserted_0<(/*void*/)> for (usize,i32,i32) {
  fn rowsInserted_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView12rowsInsertedERK11QModelIndexii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:237
// index:0
// Protected virtual Visibility=Default Availability=Available
// [16] QRect visualRect(const QModelIndex &) const

/*

*/
impl /*struct*/ QHeaderView {
  pub fn visualRect_0<RetType, T: QHeaderView_visualRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.visualRect_0(self);
    // return 1;
  }
}
pub trait QHeaderView_visualRect_0<RetType> {
  fn visualRect_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_visualRect_0<usize> for (usize) {
  fn visualRect_0(self , rsthis: & QHeaderView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView10visualRectERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:238
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void scrollTo(const QModelIndex &, QAbstractItemView::ScrollHint)

/*

*/
impl /*struct*/ QHeaderView {
  pub fn scrollTo_0<RetType, T: QHeaderView_scrollTo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scrollTo_0(self);
    // return 1;
  }
}
pub trait QHeaderView_scrollTo_0<RetType> {
  fn scrollTo_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_scrollTo_0<(/*void*/)> for (usize,i32) {
  fn scrollTo_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView8scrollToERK11QModelIndexN17QAbstractItemView10ScrollHintE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:240
// index:0
// Protected virtual Visibility=Default Availability=Available
// [24] QModelIndex indexAt(const QPoint &) const

/*

*/
impl /*struct*/ QHeaderView {
  pub fn indexAt_0<RetType, T: QHeaderView_indexAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexAt_0(self);
    // return 1;
  }
}
pub trait QHeaderView_indexAt_0<RetType> {
  fn indexAt_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_indexAt_0<usize> for (usize) {
  fn indexAt_0(self , rsthis: & QHeaderView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView7indexAtERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:241
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool isIndexHidden(const QModelIndex &) const

/*

*/
impl /*struct*/ QHeaderView {
  pub fn isIndexHidden_0<RetType, T: QHeaderView_isIndexHidden_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isIndexHidden_0(self);
    // return 1;
  }
}
pub trait QHeaderView_isIndexHidden_0<RetType> {
  fn isIndexHidden_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_isIndexHidden_0<bool> for (usize) {
  fn isIndexHidden_0(self , rsthis: & QHeaderView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView13isIndexHiddenERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:243
// index:0
// Protected virtual Visibility=Default Availability=Available
// [24] QModelIndex moveCursor(QAbstractItemView::CursorAction, Qt::KeyboardModifiers)

/*

*/
impl /*struct*/ QHeaderView {
  pub fn moveCursor_0<RetType, T: QHeaderView_moveCursor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveCursor_0(self);
    // return 1;
  }
}
pub trait QHeaderView_moveCursor_0<RetType> {
  fn moveCursor_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_moveCursor_0<usize> for (i32,i32) {
  fn moveCursor_0(self , rsthis: & QHeaderView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QHeaderView10moveCursorEN17QAbstractItemView12CursorActionE6QFlagsIN2Qt16KeyboardModifierEE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:244
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void setSelection(const QRect &, QItemSelectionModel::SelectionFlags)

/*
Reimplemented from QAbstractItemView::setSelection().

Selects the items in the given rect according to the specified flags.

The base class implementation does nothing.
*/
impl /*struct*/ QHeaderView {
  pub fn setSelection_0<RetType, T: QHeaderView_setSelection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSelection_0(self);
    // return 1;
  }
}
pub trait QHeaderView_setSelection_0<RetType> {
  fn setSelection_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_setSelection_0<(/*void*/)> for (usize,i32) {
  fn setSelection_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QHeaderView12setSelectionERK5QRect6QFlagsIN19QItemSelectionModel13SelectionFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:245
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] QRegion visualRegionForSelection(const QItemSelection &) const

/*

*/
impl /*struct*/ QHeaderView {
  pub fn visualRegionForSelection_0<RetType, T: QHeaderView_visualRegionForSelection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.visualRegionForSelection_0(self);
    // return 1;
  }
}
pub trait QHeaderView_visualRegionForSelection_0<RetType> {
  fn visualRegionForSelection_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_visualRegionForSelection_0<usize> for (usize) {
  fn visualRegionForSelection_0(self , rsthis: & QHeaderView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHeaderView24visualRegionForSelectionERK14QItemSelection", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qheaderview.h:246
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void initStyleOption(QStyleOptionHeader *) const

/*
Initialize option with the values from this QHeaderView. This method is useful for subclasses when they need a QStyleOptionHeader, but do not want to fill in all the information themselves.

See also QStyleOption::initFrom().
*/
impl /*struct*/ QHeaderView {
  pub fn initStyleOption_0<RetType, T: QHeaderView_initStyleOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.initStyleOption_0(self);
    // return 1;
  }
}
pub trait QHeaderView_initStyleOption_0<RetType> {
  fn initStyleOption_0(self , rsthis: & QHeaderView) -> RetType;
}
impl<'a> /*trait*/ QHeaderView_initStyleOption_0<(/*void*/)> for (usize) {
  fn initStyleOption_0(self , rsthis: & QHeaderView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK11QHeaderView15initStyleOptionEP18QStyleOptionHeader", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
The resize mode specifies the behavior of the header sections. It can be set on the entire header view or on individual sections using setSectionResizeMode().



The following values are obsolete:

QHeaderView::CustomFixedUse Fixed instead.


See also setResizeMode(), setSectionResizeMode(), stretchLastSection, and minimumSectionSize.

*/
pub type QHeaderView__ResizeMode = i32;
// The user can resize the section. The section can also be resized programmatically using resizeSection(). The section size defaults to defaultSectionSize. (See also cascadingSectionResizes.)
pub const QHeaderView__Interactive :QHeaderView__ResizeMode = 0;
// QHeaderView will automatically resize the section to fill the available space. The size cannot be changed by the user or programmatically.
pub const QHeaderView__Stretch :QHeaderView__ResizeMode = 1;
// The user cannot resize the section. The section can only be resized programmatically using resizeSection(). The section size defaults to defaultSectionSize.
pub const QHeaderView__Fixed :QHeaderView__ResizeMode = 2;
// 
pub const QHeaderView__ResizeToContents :QHeaderView__ResizeMode = 3;
// 
pub const QHeaderView__Custom :QHeaderView__ResizeMode = 2;
pub fn QHeaderView_ResizeModeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QHeaderView", val);
}
pub fn QHeaderView_ResizeModeItemName_s(val: i32) ->String {
  //var nilthis *QHeaderView
  //return nilthis.ResizeModeItemName(val);
  return QHeaderView_ResizeModeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
