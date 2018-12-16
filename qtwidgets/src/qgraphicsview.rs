

// mod ::widgets::QGraphicsView
// package qtwidgets
// /usr/include/qt/QtWidgets/qgraphicsview.h
// #include <qgraphicsview.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 19
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

// void setupViewport(QWidget *)
// func (this *QGraphicsView) InheritSetupViewport(f func(widget *QWidget/*777 QWidget **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "setupViewport", f)
// }

// bool event(QEvent *)
// func (this *QGraphicsView) InheritEvent(f func(event *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// bool viewportEvent(QEvent *)
// func (this *QGraphicsView) InheritViewportEvent(f func(event *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "viewportEvent", f)
// }

// void contextMenuEvent(QContextMenuEvent *)
// func (this *QGraphicsView) InheritContextMenuEvent(f func(event *qtgui.QContextMenuEvent/*777 QContextMenuEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "contextMenuEvent", f)
// }

// void dragEnterEvent(QDragEnterEvent *)
// func (this *QGraphicsView) InheritDragEnterEvent(f func(event *qtgui.QDragEnterEvent/*777 QDragEnterEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dragEnterEvent", f)
// }

// void dragLeaveEvent(QDragLeaveEvent *)
// func (this *QGraphicsView) InheritDragLeaveEvent(f func(event *qtgui.QDragLeaveEvent/*777 QDragLeaveEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dragLeaveEvent", f)
// }

// void dragMoveEvent(QDragMoveEvent *)
// func (this *QGraphicsView) InheritDragMoveEvent(f func(event *qtgui.QDragMoveEvent/*777 QDragMoveEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dragMoveEvent", f)
// }

// void dropEvent(QDropEvent *)
// func (this *QGraphicsView) InheritDropEvent(f func(event *qtgui.QDropEvent/*777 QDropEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dropEvent", f)
// }

// void focusInEvent(QFocusEvent *)
// func (this *QGraphicsView) InheritFocusInEvent(f func(event *qtgui.QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusInEvent", f)
// }

// bool focusNextPrevChild(bool)
// func (this *QGraphicsView) InheritFocusNextPrevChild(f func(next bool) bool) {
//  qtrt.SetAllInheritCallback(this, "focusNextPrevChild", f)
// }

// void focusOutEvent(QFocusEvent *)
// func (this *QGraphicsView) InheritFocusOutEvent(f func(event *qtgui.QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusOutEvent", f)
// }

// void keyPressEvent(QKeyEvent *)
// func (this *QGraphicsView) InheritKeyPressEvent(f func(event *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyPressEvent", f)
// }

// void keyReleaseEvent(QKeyEvent *)
// func (this *QGraphicsView) InheritKeyReleaseEvent(f func(event *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyReleaseEvent", f)
// }

// void mouseDoubleClickEvent(QMouseEvent *)
// func (this *QGraphicsView) InheritMouseDoubleClickEvent(f func(event *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseDoubleClickEvent", f)
// }

// void mousePressEvent(QMouseEvent *)
// func (this *QGraphicsView) InheritMousePressEvent(f func(event *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mousePressEvent", f)
// }

// void mouseMoveEvent(QMouseEvent *)
// func (this *QGraphicsView) InheritMouseMoveEvent(f func(event *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseMoveEvent", f)
// }

// void mouseReleaseEvent(QMouseEvent *)
// func (this *QGraphicsView) InheritMouseReleaseEvent(f func(event *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseReleaseEvent", f)
// }

// void wheelEvent(QWheelEvent *)
// func (this *QGraphicsView) InheritWheelEvent(f func(event *qtgui.QWheelEvent/*777 QWheelEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "wheelEvent", f)
// }

// void paintEvent(QPaintEvent *)
// func (this *QGraphicsView) InheritPaintEvent(f func(event *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }

// void resizeEvent(QResizeEvent *)
// func (this *QGraphicsView) InheritResizeEvent(f func(event *qtgui.QResizeEvent/*777 QResizeEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "resizeEvent", f)
// }

// void scrollContentsBy(int, int)
// func (this *QGraphicsView) InheritScrollContentsBy(f func(dx int, dy int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "scrollContentsBy", f)
// }

// void showEvent(QShowEvent *)
// func (this *QGraphicsView) InheritShowEvent(f func(event *qtgui.QShowEvent/*777 QShowEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "showEvent", f)
// }

// void inputMethodEvent(QInputMethodEvent *)
// func (this *QGraphicsView) InheritInputMethodEvent(f func(event *qtgui.QInputMethodEvent/*777 QInputMethodEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "inputMethodEvent", f)
// }

// void drawBackground(QPainter *, const QRectF &)
// func (this *QGraphicsView) InheritDrawBackground(f func(painter *qtgui.QPainter/*777 QPainter **/, rect *qtcore.QRectF) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "drawBackground", f)
// }

// void drawForeground(QPainter *, const QRectF &)
// func (this *QGraphicsView) InheritDrawForeground(f func(painter *qtgui.QPainter/*777 QPainter **/, rect *qtcore.QRectF) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "drawForeground", f)
// }

// void drawItems(QPainter *, int, QGraphicsItem **, const QStyleOptionGraphicsItem *)
// func (this *QGraphicsView) InheritDrawItems(f func(painter *qtgui.QPainter/*777 QPainter **/, numItems int, items []interface{}, options []interface{}) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "drawItems", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QGraphicsView)=48
pub struct QGraphicsView {
  qbase: QAbstractScrollArea,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGraphicsView_ITF interface {
//    QAbstractScrollArea_ITF
//    QGraphicsView_PTR() *QGraphicsView
//}
//func (ptr *QGraphicsView) QGraphicsView_PTR() *QGraphicsView { return ptr }

impl /*struct*/ QGraphicsView {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGraphicsView {
    return QGraphicsView{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGraphicsView {
//  type Target = QGraphicsViewBASE;
//
//  fn deref(&self) -> &QGraphicsViewBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGraphicsViewBASE> for QGraphicsView {
//  fn as_ref(& self) -> & QGraphicsViewBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgraphicsview.h:61
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QGraphicsView {
  pub fn metaObject_0<RetType, T: QGraphicsView_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QGraphicsView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:117
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QGraphicsView(QWidget *)

/*
Constructs a QGraphicsView. parent is passed to QWidget's constructor.
*/
// QGraphicsView(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QGraphicsView {
  pub fn QGraphicsView_0<T: QGraphicsView_QGraphicsView_0>(value: T) -> QGraphicsView {
    let rsthis = value.QGraphicsView_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsView_QGraphicsView_0 {
  fn QGraphicsView_0(self) -> QGraphicsView;
}
// QGraphicsView(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGraphicsView_QGraphicsView_0 for (usize) {
  fn QGraphicsView_0(self) -> QGraphicsView {
    // unsafe{_ZN13QGraphicsViewC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QGraphicsViewC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGraphicsView{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:118
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QGraphicsView(QGraphicsScene *, QWidget *)

/*
Constructs a QGraphicsView. parent is passed to QWidget's constructor.
*/
// QGraphicsView(QGraphicsScene *, QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QGraphicsView {
  pub fn QGraphicsView_1<T: QGraphicsView_QGraphicsView_1>(value: T) -> QGraphicsView {
    let rsthis = value.QGraphicsView_1();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsView_QGraphicsView_1 {
  fn QGraphicsView_1(self) -> QGraphicsView;
}
// QGraphicsView(QGraphicsScene *, QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGraphicsView_QGraphicsView_1 for (usize,usize) {
  fn QGraphicsView_1(self) -> QGraphicsView {
    // unsafe{_ZN13QGraphicsViewC2EP14QGraphicsSceneP7QWidget()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QGraphicsViewC2EP14QGraphicsSceneP7QWidget", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGraphicsView{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:119
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QGraphicsView()

/*

*/
pub fn DeleteQGraphicsView(this :*mut QGraphicsView) {
    // let rv = qtrt::InvokeQtFunc6("_ZN13QGraphicsViewD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgraphicsview.h:121
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Reimplemented from QWidget::sizeHint().
*/
impl /*struct*/ QGraphicsView {
  pub fn sizeHint_0<RetType, T: QGraphicsView_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QGraphicsView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:123
// index:0
// Public Visibility=Default Availability=Available
// [4] QPainter::RenderHints renderHints() const

/*

*/
impl /*struct*/ QGraphicsView {
  pub fn renderHints_0<RetType, T: QGraphicsView_renderHints_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.renderHints_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_renderHints_0<RetType> {
  fn renderHints_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_renderHints_0<i32> for () {
  fn renderHints_0(self , rsthis: & QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView11renderHintsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:124
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRenderHint(QPainter::RenderHint, bool)

/*
If enabled is true, the render hint hint is enabled; otherwise it is disabled.

See also renderHints.
*/
impl /*struct*/ QGraphicsView {
  pub fn setRenderHint_0<RetType, T: QGraphicsView_setRenderHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRenderHint_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_setRenderHint_0<RetType> {
  fn setRenderHint_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_setRenderHint_0<(/*void*/)> for (i32,bool) {
  fn setRenderHint_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView13setRenderHintEN8QPainter10RenderHintEb", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:125
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRenderHints(QPainter::RenderHints)

/*

*/
impl /*struct*/ QGraphicsView {
  pub fn setRenderHints_0<RetType, T: QGraphicsView_setRenderHints_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRenderHints_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_setRenderHints_0<RetType> {
  fn setRenderHints_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_setRenderHints_0<(/*void*/)> for (i32) {
  fn setRenderHints_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView14setRenderHintsE6QFlagsIN8QPainter10RenderHintEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:127
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::Alignment alignment() const

/*

*/
impl /*struct*/ QGraphicsView {
  pub fn alignment_0<RetType, T: QGraphicsView_alignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.alignment_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_alignment_0<RetType> {
  fn alignment_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_alignment_0<i32> for () {
  fn alignment_0(self , rsthis: & QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView9alignmentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:128
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAlignment(Qt::Alignment)

/*

*/
impl /*struct*/ QGraphicsView {
  pub fn setAlignment_0<RetType, T: QGraphicsView_setAlignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAlignment_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_setAlignment_0<RetType> {
  fn setAlignment_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_setAlignment_0<(/*void*/)> for (i32) {
  fn setAlignment_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView12setAlignmentE6QFlagsIN2Qt13AlignmentFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:130
// index:0
// Public Visibility=Default Availability=Available
// [4] QGraphicsView::ViewportAnchor transformationAnchor() const

/*

*/
impl /*struct*/ QGraphicsView {
  pub fn transformationAnchor_0<RetType, T: QGraphicsView_transformationAnchor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.transformationAnchor_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_transformationAnchor_0<RetType> {
  fn transformationAnchor_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_transformationAnchor_0<i32> for () {
  fn transformationAnchor_0(self , rsthis: & QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView20transformationAnchorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:131
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTransformationAnchor(QGraphicsView::ViewportAnchor)

/*

*/
impl /*struct*/ QGraphicsView {
  pub fn setTransformationAnchor_0<RetType, T: QGraphicsView_setTransformationAnchor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTransformationAnchor_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_setTransformationAnchor_0<RetType> {
  fn setTransformationAnchor_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_setTransformationAnchor_0<(/*void*/)> for (i32) {
  fn setTransformationAnchor_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView23setTransformationAnchorENS_14ViewportAnchorE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:133
// index:0
// Public Visibility=Default Availability=Available
// [4] QGraphicsView::ViewportAnchor resizeAnchor() const

/*

*/
impl /*struct*/ QGraphicsView {
  pub fn resizeAnchor_0<RetType, T: QGraphicsView_resizeAnchor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeAnchor_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_resizeAnchor_0<RetType> {
  fn resizeAnchor_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_resizeAnchor_0<i32> for () {
  fn resizeAnchor_0(self , rsthis: & QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView12resizeAnchorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:134
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setResizeAnchor(QGraphicsView::ViewportAnchor)

/*

*/
impl /*struct*/ QGraphicsView {
  pub fn setResizeAnchor_0<RetType, T: QGraphicsView_setResizeAnchor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setResizeAnchor_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_setResizeAnchor_0<RetType> {
  fn setResizeAnchor_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_setResizeAnchor_0<(/*void*/)> for (i32) {
  fn setResizeAnchor_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView15setResizeAnchorENS_14ViewportAnchorE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:136
// index:0
// Public Visibility=Default Availability=Available
// [4] QGraphicsView::ViewportUpdateMode viewportUpdateMode() const

/*

*/
impl /*struct*/ QGraphicsView {
  pub fn viewportUpdateMode_0<RetType, T: QGraphicsView_viewportUpdateMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.viewportUpdateMode_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_viewportUpdateMode_0<RetType> {
  fn viewportUpdateMode_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_viewportUpdateMode_0<i32> for () {
  fn viewportUpdateMode_0(self , rsthis: & QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView18viewportUpdateModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:137
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setViewportUpdateMode(QGraphicsView::ViewportUpdateMode)

/*

*/
impl /*struct*/ QGraphicsView {
  pub fn setViewportUpdateMode_0<RetType, T: QGraphicsView_setViewportUpdateMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setViewportUpdateMode_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_setViewportUpdateMode_0<RetType> {
  fn setViewportUpdateMode_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_setViewportUpdateMode_0<(/*void*/)> for (i32) {
  fn setViewportUpdateMode_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView21setViewportUpdateModeENS_18ViewportUpdateModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:139
// index:0
// Public Visibility=Default Availability=Available
// [4] QGraphicsView::OptimizationFlags optimizationFlags() const

/*

*/
impl /*struct*/ QGraphicsView {
  pub fn optimizationFlags_0<RetType, T: QGraphicsView_optimizationFlags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.optimizationFlags_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_optimizationFlags_0<RetType> {
  fn optimizationFlags_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_optimizationFlags_0<i32> for () {
  fn optimizationFlags_0(self , rsthis: & QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView17optimizationFlagsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:140
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOptimizationFlag(QGraphicsView::OptimizationFlag, bool)

/*
Enables flag if enabled is true; otherwise disables flag.

See also optimizationFlags.
*/
impl /*struct*/ QGraphicsView {
  pub fn setOptimizationFlag_0<RetType, T: QGraphicsView_setOptimizationFlag_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOptimizationFlag_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_setOptimizationFlag_0<RetType> {
  fn setOptimizationFlag_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_setOptimizationFlag_0<(/*void*/)> for (i32,bool) {
  fn setOptimizationFlag_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView19setOptimizationFlagENS_16OptimizationFlagEb", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:141
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOptimizationFlags(QGraphicsView::OptimizationFlags)

/*

*/
impl /*struct*/ QGraphicsView {
  pub fn setOptimizationFlags_0<RetType, T: QGraphicsView_setOptimizationFlags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOptimizationFlags_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_setOptimizationFlags_0<RetType> {
  fn setOptimizationFlags_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_setOptimizationFlags_0<(/*void*/)> for (i32) {
  fn setOptimizationFlags_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView20setOptimizationFlagsE6QFlagsINS_16OptimizationFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:143
// index:0
// Public Visibility=Default Availability=Available
// [4] QGraphicsView::DragMode dragMode() const

/*

*/
impl /*struct*/ QGraphicsView {
  pub fn dragMode_0<RetType, T: QGraphicsView_dragMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragMode_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_dragMode_0<RetType> {
  fn dragMode_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_dragMode_0<i32> for () {
  fn dragMode_0(self , rsthis: & QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView8dragModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:144
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDragMode(QGraphicsView::DragMode)

/*

*/
impl /*struct*/ QGraphicsView {
  pub fn setDragMode_0<RetType, T: QGraphicsView_setDragMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDragMode_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_setDragMode_0<RetType> {
  fn setDragMode_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_setDragMode_0<(/*void*/)> for (i32) {
  fn setDragMode_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView11setDragModeENS_8DragModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:147
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::ItemSelectionMode rubberBandSelectionMode() const

/*

*/
impl /*struct*/ QGraphicsView {
  pub fn rubberBandSelectionMode_0<RetType, T: QGraphicsView_rubberBandSelectionMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rubberBandSelectionMode_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_rubberBandSelectionMode_0<RetType> {
  fn rubberBandSelectionMode_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_rubberBandSelectionMode_0<i32> for () {
  fn rubberBandSelectionMode_0(self , rsthis: & QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView23rubberBandSelectionModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:148
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRubberBandSelectionMode(Qt::ItemSelectionMode)

/*

*/
impl /*struct*/ QGraphicsView {
  pub fn setRubberBandSelectionMode_0<RetType, T: QGraphicsView_setRubberBandSelectionMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRubberBandSelectionMode_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_setRubberBandSelectionMode_0<RetType> {
  fn setRubberBandSelectionMode_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_setRubberBandSelectionMode_0<(/*void*/)> for (i32) {
  fn setRubberBandSelectionMode_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView26setRubberBandSelectionModeEN2Qt17ItemSelectionModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:149
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect rubberBandRect() const

/*
This functions returns the current rubber band area (in viewport coordinates) if the user is currently doing an itemselection with rubber band. When the user is not using the rubber band this functions returns (a null) QRectF().

Notice that part of this QRect can be outise the visual viewport. It can e.g contain negative values.

This function was introduced in  Qt 5.1.

See also rubberBandSelectionMode and rubberBandChanged().
*/
impl /*struct*/ QGraphicsView {
  pub fn rubberBandRect_0<RetType, T: QGraphicsView_rubberBandRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rubberBandRect_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_rubberBandRect_0<RetType> {
  fn rubberBandRect_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_rubberBandRect_0<usize> for () {
  fn rubberBandRect_0(self , rsthis: & QGraphicsView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView14rubberBandRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:152
// index:0
// Public Visibility=Default Availability=Available
// [4] QGraphicsView::CacheMode cacheMode() const

/*

*/
impl /*struct*/ QGraphicsView {
  pub fn cacheMode_0<RetType, T: QGraphicsView_cacheMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cacheMode_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_cacheMode_0<RetType> {
  fn cacheMode_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_cacheMode_0<i32> for () {
  fn cacheMode_0(self , rsthis: & QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView9cacheModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:153
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCacheMode(QGraphicsView::CacheMode)

/*

*/
impl /*struct*/ QGraphicsView {
  pub fn setCacheMode_0<RetType, T: QGraphicsView_setCacheMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCacheMode_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_setCacheMode_0<RetType> {
  fn setCacheMode_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_setCacheMode_0<(/*void*/)> for (i32) {
  fn setCacheMode_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView12setCacheModeE6QFlagsINS_13CacheModeFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:154
// index:0
// Public Visibility=Default Availability=Available
// [-2] void resetCachedContent()

/*
Resets any cached content. Calling this function will clear QGraphicsView's cache. If the current cache mode is CacheNone, this function does nothing.

This function is called automatically for you when the backgroundBrush or QGraphicsScene::backgroundBrush properties change; you only need to call this function if you have reimplemented QGraphicsScene::drawBackground() or QGraphicsView::drawBackground() to draw a custom background, and need to trigger a full redraw.

See also cacheMode().
*/
impl /*struct*/ QGraphicsView {
  pub fn resetCachedContent_0<RetType, T: QGraphicsView_resetCachedContent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resetCachedContent_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_resetCachedContent_0<RetType> {
  fn resetCachedContent_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_resetCachedContent_0<(/*void*/)> for () {
  fn resetCachedContent_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView18resetCachedContentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:156
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isInteractive() const

/*

*/
impl /*struct*/ QGraphicsView {
  pub fn isInteractive_0<RetType, T: QGraphicsView_isInteractive_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isInteractive_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_isInteractive_0<RetType> {
  fn isInteractive_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_isInteractive_0<bool> for () {
  fn isInteractive_0(self , rsthis: & QGraphicsView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView13isInteractiveEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:157
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setInteractive(bool)

/*

*/
impl /*struct*/ QGraphicsView {
  pub fn setInteractive_0<RetType, T: QGraphicsView_setInteractive_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setInteractive_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_setInteractive_0<RetType> {
  fn setInteractive_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_setInteractive_0<(/*void*/)> for (bool) {
  fn setInteractive_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView14setInteractiveEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:159
// index:0
// Public Visibility=Default Availability=Available
// [8] QGraphicsScene * scene() const

/*
Returns a pointer to the scene that is currently visualized in the view. If no scene is currently visualized, 0 is returned.

See also setScene().
*/
impl /*struct*/ QGraphicsView {
  pub fn scene_0<RetType, T: QGraphicsView_scene_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scene_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_scene_0<RetType> {
  fn scene_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_scene_0<usize> for () {
  fn scene_0(self , rsthis: & QGraphicsView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView5sceneEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:160
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setScene(QGraphicsScene *)

/*
Sets the current scene to scene. If scene is already being viewed, this function does nothing.

When a scene is set on a view, the QGraphicsScene::changed() signal is automatically connected to this view's updateScene() slot, and the view's scroll bars are adjusted to fit the size of the scene.

The view does not take ownership of scene.

See also scene().
*/
impl /*struct*/ QGraphicsView {
  pub fn setScene_0<RetType, T: QGraphicsView_setScene_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setScene_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_setScene_0<RetType> {
  fn setScene_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_setScene_0<(/*void*/)> for (usize) {
  fn setScene_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView8setSceneEP14QGraphicsScene", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:162
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF sceneRect() const

/*

*/
impl /*struct*/ QGraphicsView {
  pub fn sceneRect_0<RetType, T: QGraphicsView_sceneRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sceneRect_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_sceneRect_0<RetType> {
  fn sceneRect_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_sceneRect_0<usize> for () {
  fn sceneRect_0(self , rsthis: & QGraphicsView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView9sceneRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:163
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSceneRect(const QRectF &)

/*

*/
impl /*struct*/ QGraphicsView {
  pub fn setSceneRect_0<RetType, T: QGraphicsView_setSceneRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSceneRect_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_setSceneRect_0<RetType> {
  fn setSceneRect_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_setSceneRect_0<(/*void*/)> for (usize) {
  fn setSceneRect_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView12setSceneRectERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:164
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void setSceneRect(qreal, qreal, qreal, qreal)

/*

*/
impl /*struct*/ QGraphicsView {
  pub fn setSceneRect_1<RetType, T: QGraphicsView_setSceneRect_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSceneRect_1(self);
    // return 1;
  }
}
pub trait QGraphicsView_setSceneRect_1<RetType> {
  fn setSceneRect_1(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_setSceneRect_1<(/*void*/)> for (f64,f64,f64,f64) {
  fn setSceneRect_1(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView12setSceneRectEdddd", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:166
// index:0
// Public Visibility=Default Availability=Available
// [48] QMatrix matrix() const

/*
Returns the current transformation matrix for the view. If no current transformation is set, the identity matrix is returned.

See also setMatrix(), transform(), rotate(), scale(), shear(), and translate().
*/
impl /*struct*/ QGraphicsView {
  pub fn matrix_0<RetType, T: QGraphicsView_matrix_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.matrix_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_matrix_0<RetType> {
  fn matrix_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_matrix_0<usize> for () {
  fn matrix_0(self , rsthis: & QGraphicsView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView6matrixEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:167
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMatrix(const QMatrix &, bool)

/*
Sets the view's current transformation matrix to matrix.

If combine is true, then matrix is combined with the current matrix; otherwise, matrix replaces the current matrix. combine is false by default.

The transformation matrix tranforms the scene into view coordinates. Using the default transformation, provided by the identity matrix, one pixel in the view represents one unit in the scene (e.g., a 10x10 rectangular item is drawn using 10x10 pixels in the view). If a 2x2 scaling matrix is applied, the scene will be drawn in 1:2 (e.g., a 10x10 rectangular item is then drawn using 20x20 pixels in the view).

Example:


  QGraphicsScene scene;
  scene.addText("GraphicsView rotated clockwise");

  QGraphicsView view(&scene);
  view.rotate(90); // the text is rendered with a 90 degree clockwise rotation
  view.show();



To simplify interation with items using a transformed view, QGraphicsView provides mapTo... and mapFrom... functions that can translate between scene and view coordinates. For example, you can call mapToScene() to map a view coordinate to a floating point scene coordinate, or mapFromScene() to map from floating point scene coordinates to view coordinates.

See also matrix(), setTransform(), rotate(), scale(), shear(), and translate().
*/
impl /*struct*/ QGraphicsView {
  pub fn setMatrix_0<RetType, T: QGraphicsView_setMatrix_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMatrix_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_setMatrix_0<RetType> {
  fn setMatrix_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_setMatrix_0<(/*void*/)> for (usize,bool) {
  fn setMatrix_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView9setMatrixERK7QMatrixb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:168
// index:0
// Public Visibility=Default Availability=Available
// [-2] void resetMatrix()

/*
Resets the view transformation matrix to the identity matrix.

See also resetTransform().
*/
impl /*struct*/ QGraphicsView {
  pub fn resetMatrix_0<RetType, T: QGraphicsView_resetMatrix_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resetMatrix_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_resetMatrix_0<RetType> {
  fn resetMatrix_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_resetMatrix_0<(/*void*/)> for () {
  fn resetMatrix_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView11resetMatrixEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:169
// index:0
// Public Visibility=Default Availability=Available
// [88] QTransform transform() const

/*
Returns the current transformation matrix for the view. If no current transformation is set, the identity matrix is returned.

See also setTransform(), rotate(), scale(), shear(), and translate().
*/
impl /*struct*/ QGraphicsView {
  pub fn transform_0<RetType, T: QGraphicsView_transform_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.transform_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_transform_0<RetType> {
  fn transform_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_transform_0<usize> for () {
  fn transform_0(self , rsthis: & QGraphicsView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView9transformEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:170
// index:0
// Public Visibility=Default Availability=Available
// [88] QTransform viewportTransform() const

/*
Returns a matrix that maps scene coordinates to viewport coordinates.

See also mapToScene() and mapFromScene().
*/
impl /*struct*/ QGraphicsView {
  pub fn viewportTransform_0<RetType, T: QGraphicsView_viewportTransform_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.viewportTransform_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_viewportTransform_0<RetType> {
  fn viewportTransform_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_viewportTransform_0<usize> for () {
  fn viewportTransform_0(self , rsthis: & QGraphicsView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView17viewportTransformEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:171
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isTransformed() const

/*
Returns true if the view is transformed (i.e., a non-identity transform has been assigned, or the scrollbars are adjusted).

This function was introduced in  Qt 4.6.

See also setTransform(), horizontalScrollBar(), and verticalScrollBar().
*/
impl /*struct*/ QGraphicsView {
  pub fn isTransformed_0<RetType, T: QGraphicsView_isTransformed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isTransformed_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_isTransformed_0<RetType> {
  fn isTransformed_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_isTransformed_0<bool> for () {
  fn isTransformed_0(self , rsthis: & QGraphicsView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView13isTransformedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:172
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTransform(const QTransform &, bool)

/*
Sets the view's current transformation matrix to matrix.

If combine is true, then matrix is combined with the current matrix; otherwise, matrix replaces the current matrix. combine is false by default.

The transformation matrix tranforms the scene into view coordinates. Using the default transformation, provided by the identity matrix, one pixel in the view represents one unit in the scene (e.g., a 10x10 rectangular item is drawn using 10x10 pixels in the view). If a 2x2 scaling matrix is applied, the scene will be drawn in 1:2 (e.g., a 10x10 rectangular item is then drawn using 20x20 pixels in the view).

Example:


  QGraphicsScene scene;
  scene.addText("GraphicsView rotated clockwise");

  QGraphicsView view(&scene);
  view.rotate(90); // the text is rendered with a 90 degree clockwise rotation
  view.show();



To simplify interation with items using a transformed view, QGraphicsView provides mapTo... and mapFrom... functions that can translate between scene and view coordinates. For example, you can call mapToScene() to map a view coordiate to a floating point scene coordinate, or mapFromScene() to map from floating point scene coordinates to view coordinates.

See also transform(), rotate(), scale(), shear(), and translate().
*/
impl /*struct*/ QGraphicsView {
  pub fn setTransform_0<RetType, T: QGraphicsView_setTransform_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTransform_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_setTransform_0<RetType> {
  fn setTransform_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_setTransform_0<(/*void*/)> for (usize,bool) {
  fn setTransform_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView12setTransformERK10QTransformb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:173
// index:0
// Public Visibility=Default Availability=Available
// [-2] void resetTransform()

/*
Resets the view transformation to the identity matrix.

See also transform() and setTransform().
*/
impl /*struct*/ QGraphicsView {
  pub fn resetTransform_0<RetType, T: QGraphicsView_resetTransform_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resetTransform_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_resetTransform_0<RetType> {
  fn resetTransform_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_resetTransform_0<(/*void*/)> for () {
  fn resetTransform_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView14resetTransformEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:174
// index:0
// Public Visibility=Default Availability=Available
// [-2] void rotate(qreal)

/*
Rotates the current view transformation angle degrees clockwise.

See also setTransform(), transform(), scale(), shear(), and translate().
*/
impl /*struct*/ QGraphicsView {
  pub fn rotate_0<RetType, T: QGraphicsView_rotate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rotate_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_rotate_0<RetType> {
  fn rotate_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_rotate_0<(/*void*/)> for (f64) {
  fn rotate_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView6rotateEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:175
// index:0
// Public Visibility=Default Availability=Available
// [-2] void scale(qreal, qreal)

/*
Scales the current view transformation by (sx, sy).

See also setTransform(), transform(), rotate(), shear(), and translate().
*/
impl /*struct*/ QGraphicsView {
  pub fn scale_0<RetType, T: QGraphicsView_scale_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scale_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_scale_0<RetType> {
  fn scale_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_scale_0<(/*void*/)> for (f64,f64) {
  fn scale_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView5scaleEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:176
// index:0
// Public Visibility=Default Availability=Available
// [-2] void shear(qreal, qreal)

/*
Shears the current view transformation by (sh, sv).

See also setTransform(), transform(), rotate(), scale(), and translate().
*/
impl /*struct*/ QGraphicsView {
  pub fn shear_0<RetType, T: QGraphicsView_shear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.shear_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_shear_0<RetType> {
  fn shear_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_shear_0<(/*void*/)> for (f64,f64) {
  fn shear_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView5shearEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:177
// index:0
// Public Visibility=Default Availability=Available
// [-2] void translate(qreal, qreal)

/*
Translates the current view transformation by (dx, dy).

See also setTransform(), transform(), rotate(), and shear().
*/
impl /*struct*/ QGraphicsView {
  pub fn translate_0<RetType, T: QGraphicsView_translate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translate_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_translate_0<RetType> {
  fn translate_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_translate_0<(/*void*/)> for (f64,f64) {
  fn translate_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView9translateEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:179
// index:0
// Public Visibility=Default Availability=Available
// [-2] void centerOn(const QPointF &)

/*
Scrolls the contents of the viewport to ensure that the scene coordinate pos, is centered in the view.

Because pos is a floating point coordinate, and the scroll bars operate on integer coordinates, the centering is only an approximation.

Note: If the item is close to or outside the border, it will be visible in the view, but not centered.

See also ensureVisible().
*/
impl /*struct*/ QGraphicsView {
  pub fn centerOn_0<RetType, T: QGraphicsView_centerOn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.centerOn_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_centerOn_0<RetType> {
  fn centerOn_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_centerOn_0<(/*void*/)> for (usize) {
  fn centerOn_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView8centerOnERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:180
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void centerOn(qreal, qreal)

/*
Scrolls the contents of the viewport to ensure that the scene coordinate pos, is centered in the view.

Because pos is a floating point coordinate, and the scroll bars operate on integer coordinates, the centering is only an approximation.

Note: If the item is close to or outside the border, it will be visible in the view, but not centered.

See also ensureVisible().
*/
impl /*struct*/ QGraphicsView {
  pub fn centerOn_1<RetType, T: QGraphicsView_centerOn_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.centerOn_1(self);
    // return 1;
  }
}
pub trait QGraphicsView_centerOn_1<RetType> {
  fn centerOn_1(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_centerOn_1<(/*void*/)> for (f64,f64) {
  fn centerOn_1(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView8centerOnEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:181
// index:2
// Public Visibility=Default Availability=Available
// [-2] void centerOn(const QGraphicsItem *)

/*
Scrolls the contents of the viewport to ensure that the scene coordinate pos, is centered in the view.

Because pos is a floating point coordinate, and the scroll bars operate on integer coordinates, the centering is only an approximation.

Note: If the item is close to or outside the border, it will be visible in the view, but not centered.

See also ensureVisible().
*/
impl /*struct*/ QGraphicsView {
  pub fn centerOn_2<RetType, T: QGraphicsView_centerOn_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.centerOn_2(self);
    // return 1;
  }
}
pub trait QGraphicsView_centerOn_2<RetType> {
  fn centerOn_2(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_centerOn_2<(/*void*/)> for (usize) {
  fn centerOn_2(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView8centerOnEPK13QGraphicsItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:182
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ensureVisible(const QRectF &, int, int)

/*
Scrolls the contents of the viewport so that the scene rectangle rect is visible, with margins specified in pixels by xmargin and ymargin. If the specified rect cannot be reached, the contents are scrolled to the nearest valid position. The default value for both margins is 50 pixels.

See also centerOn().
*/
impl /*struct*/ QGraphicsView {
  pub fn ensureVisible_0<RetType, T: QGraphicsView_ensureVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ensureVisible_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_ensureVisible_0<RetType> {
  fn ensureVisible_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_ensureVisible_0<(/*void*/)> for (usize,i32,i32) {
  fn ensureVisible_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView13ensureVisibleERK6QRectFii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:183
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void ensureVisible(qreal, qreal, qreal, qreal, int, int)

/*
Scrolls the contents of the viewport so that the scene rectangle rect is visible, with margins specified in pixels by xmargin and ymargin. If the specified rect cannot be reached, the contents are scrolled to the nearest valid position. The default value for both margins is 50 pixels.

See also centerOn().
*/
impl /*struct*/ QGraphicsView {
  pub fn ensureVisible_1<RetType, T: QGraphicsView_ensureVisible_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ensureVisible_1(self);
    // return 1;
  }
}
pub trait QGraphicsView_ensureVisible_1<RetType> {
  fn ensureVisible_1(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_ensureVisible_1<(/*void*/)> for (f64,f64,f64,f64,i32,i32) {
  fn ensureVisible_1(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView13ensureVisibleEddddii", 6,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:184
// index:2
// Public Visibility=Default Availability=Available
// [-2] void ensureVisible(const QGraphicsItem *, int, int)

/*
Scrolls the contents of the viewport so that the scene rectangle rect is visible, with margins specified in pixels by xmargin and ymargin. If the specified rect cannot be reached, the contents are scrolled to the nearest valid position. The default value for both margins is 50 pixels.

See also centerOn().
*/
impl /*struct*/ QGraphicsView {
  pub fn ensureVisible_2<RetType, T: QGraphicsView_ensureVisible_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ensureVisible_2(self);
    // return 1;
  }
}
pub trait QGraphicsView_ensureVisible_2<RetType> {
  fn ensureVisible_2(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_ensureVisible_2<(/*void*/)> for (usize,i32,i32) {
  fn ensureVisible_2(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView13ensureVisibleEPK13QGraphicsItemii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:185
// index:0
// Public Visibility=Default Availability=Available
// [-2] void fitInView(const QRectF &, Qt::AspectRatioMode)

/*
Scales the view matrix and scrolls the scroll bars to ensure that the scene rectangle rect fits inside the viewport. rect must be inside the scene rect; otherwise, fitInView() cannot guarantee that the whole rect is visible.

This function keeps the view's rotation, translation, or shear. The view is scaled according to aspectRatioMode. rect will be centered in the view if it does not fit tightly.

It's common to call fitInView() from inside a reimplementation of resizeEvent(), to ensure that the whole scene, or parts of the scene, scales automatically to fit the new size of the viewport as the view is resized. Note though, that calling fitInView() from inside resizeEvent() can lead to unwanted resize recursion, if the new transformation toggles the automatic state of the scrollbars. You can toggle the scrollbar policies to always on or always off to prevent this (see horizontalScrollBarPolicy() and verticalScrollBarPolicy()).

If rect is empty, or if the viewport is too small, this function will do nothing.

See also setTransform(), ensureVisible(), and centerOn().
*/
impl /*struct*/ QGraphicsView {
  pub fn fitInView_0<RetType, T: QGraphicsView_fitInView_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fitInView_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_fitInView_0<RetType> {
  fn fitInView_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_fitInView_0<(/*void*/)> for (usize,i32) {
  fn fitInView_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView9fitInViewERK6QRectFN2Qt15AspectRatioModeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:186
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void fitInView(qreal, qreal, qreal, qreal, Qt::AspectRatioMode)

/*
Scales the view matrix and scrolls the scroll bars to ensure that the scene rectangle rect fits inside the viewport. rect must be inside the scene rect; otherwise, fitInView() cannot guarantee that the whole rect is visible.

This function keeps the view's rotation, translation, or shear. The view is scaled according to aspectRatioMode. rect will be centered in the view if it does not fit tightly.

It's common to call fitInView() from inside a reimplementation of resizeEvent(), to ensure that the whole scene, or parts of the scene, scales automatically to fit the new size of the viewport as the view is resized. Note though, that calling fitInView() from inside resizeEvent() can lead to unwanted resize recursion, if the new transformation toggles the automatic state of the scrollbars. You can toggle the scrollbar policies to always on or always off to prevent this (see horizontalScrollBarPolicy() and verticalScrollBarPolicy()).

If rect is empty, or if the viewport is too small, this function will do nothing.

See also setTransform(), ensureVisible(), and centerOn().
*/
impl /*struct*/ QGraphicsView {
  pub fn fitInView_1<RetType, T: QGraphicsView_fitInView_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fitInView_1(self);
    // return 1;
  }
}
pub trait QGraphicsView_fitInView_1<RetType> {
  fn fitInView_1(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_fitInView_1<(/*void*/)> for (f64,f64,f64,f64,i32) {
  fn fitInView_1(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView9fitInViewEddddN2Qt15AspectRatioModeE", 5,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:188
// index:2
// Public Visibility=Default Availability=Available
// [-2] void fitInView(const QGraphicsItem *, Qt::AspectRatioMode)

/*
Scales the view matrix and scrolls the scroll bars to ensure that the scene rectangle rect fits inside the viewport. rect must be inside the scene rect; otherwise, fitInView() cannot guarantee that the whole rect is visible.

This function keeps the view's rotation, translation, or shear. The view is scaled according to aspectRatioMode. rect will be centered in the view if it does not fit tightly.

It's common to call fitInView() from inside a reimplementation of resizeEvent(), to ensure that the whole scene, or parts of the scene, scales automatically to fit the new size of the viewport as the view is resized. Note though, that calling fitInView() from inside resizeEvent() can lead to unwanted resize recursion, if the new transformation toggles the automatic state of the scrollbars. You can toggle the scrollbar policies to always on or always off to prevent this (see horizontalScrollBarPolicy() and verticalScrollBarPolicy()).

If rect is empty, or if the viewport is too small, this function will do nothing.

See also setTransform(), ensureVisible(), and centerOn().
*/
impl /*struct*/ QGraphicsView {
  pub fn fitInView_2<RetType, T: QGraphicsView_fitInView_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fitInView_2(self);
    // return 1;
  }
}
pub trait QGraphicsView_fitInView_2<RetType> {
  fn fitInView_2(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_fitInView_2<(/*void*/)> for (usize,i32) {
  fn fitInView_2(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView9fitInViewEPK13QGraphicsItemN2Qt15AspectRatioModeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:191
// index:0
// Public Visibility=Default Availability=Available
// [-2] void render(QPainter *, const QRectF &, const QRect &, Qt::AspectRatioMode)

/*
Renders the source rect, which is in view coordinates, from the scene into target, which is in paint device coordinates, using painter. This function is useful for capturing the contents of the view onto a paint device, such as a QImage (e.g., to take a screenshot), or for printing to QPrinter. For example:


  QGraphicsScene scene;
  scene.addItem(...
  ...

  QGraphicsView view(&scene);
  view.show();
  ...

  QPrinter printer(QPrinter::HighResolution);
  printer.setPageSize(QPrinter::A4);
  QPainter painter(&printer);

  // print, fitting the viewport contents into a full page
  view.render(&painter);

  // print the upper half of the viewport into the lower.
  // half of the page.
  QRect viewport = view.viewport()->rect();
  view.render(&painter,
              QRectF(0, printer.height() / 2,
                     printer.width(), printer.height() / 2),
              viewport.adjusted(0, 0, 0, -viewport.height() / 2));



If source is a null rect, this function will use viewport()->rect() to determine what to draw. If target is a null rect, the full dimensions of painter's paint device (e.g., for a QPrinter, the page size) will be used.

The source rect contents will be transformed according to aspectRatioMode to fit into the target rect. By default, the aspect ratio is kept, and source is scaled to fit in target.

See also QGraphicsScene::render().
*/
impl /*struct*/ QGraphicsView {
  pub fn render_0<RetType, T: QGraphicsView_render_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.render_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_render_0<RetType> {
  fn render_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_render_0<(/*void*/)> for (usize,usize,usize,i32) {
  fn render_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView6renderEP8QPainterRK6QRectFRK5QRectN2Qt15AspectRatioModeE", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:194
// index:0
// Public Visibility=Default Availability=Available
// [8] QList<QGraphicsItem *> items() const

/*
Returns a list of all the items in the associated scene, in descending stacking order (i.e., the first item in the returned list is the uppermost item).

See also QGraphicsScene::items() and Sorting.
*/
impl /*struct*/ QGraphicsView {
  pub fn items_0<RetType, T: QGraphicsView_items_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.items_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_items_0<RetType> {
  fn items_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_items_0<usize> for () {
  fn items_0(self , rsthis: & QGraphicsView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView5itemsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:195
// index:1
// Public Visibility=Default Availability=Available
// [8] QList<QGraphicsItem *> items(const QPoint &) const

/*
Returns a list of all the items in the associated scene, in descending stacking order (i.e., the first item in the returned list is the uppermost item).

See also QGraphicsScene::items() and Sorting.
*/
impl /*struct*/ QGraphicsView {
  pub fn items_1<RetType, T: QGraphicsView_items_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.items_1(self);
    // return 1;
  }
}
pub trait QGraphicsView_items_1<RetType> {
  fn items_1(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_items_1<usize> for (usize) {
  fn items_1(self , rsthis: & QGraphicsView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView5itemsERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:196
// index:2
// Public inline Visibility=Default Availability=Available
// [8] QList<QGraphicsItem *> items(int, int) const

/*
Returns a list of all the items in the associated scene, in descending stacking order (i.e., the first item in the returned list is the uppermost item).

See also QGraphicsScene::items() and Sorting.
*/
impl /*struct*/ QGraphicsView {
  pub fn items_2<RetType, T: QGraphicsView_items_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.items_2(self);
    // return 1;
  }
}
pub trait QGraphicsView_items_2<RetType> {
  fn items_2(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_items_2<usize> for (i32,i32) {
  fn items_2(self , rsthis: & QGraphicsView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView5itemsEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:197
// index:3
// Public Visibility=Default Availability=Available
// [8] QList<QGraphicsItem *> items(const QRect &, Qt::ItemSelectionMode) const

/*
Returns a list of all the items in the associated scene, in descending stacking order (i.e., the first item in the returned list is the uppermost item).

See also QGraphicsScene::items() and Sorting.
*/
impl /*struct*/ QGraphicsView {
  pub fn items_3<RetType, T: QGraphicsView_items_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.items_3(self);
    // return 1;
  }
}
pub trait QGraphicsView_items_3<RetType> {
  fn items_3(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_items_3<usize> for (usize,i32) {
  fn items_3(self , rsthis: & QGraphicsView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView5itemsERK5QRectN2Qt17ItemSelectionModeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:198
// index:4
// Public inline Visibility=Default Availability=Available
// [8] QList<QGraphicsItem *> items(int, int, int, int, Qt::ItemSelectionMode) const

/*
Returns a list of all the items in the associated scene, in descending stacking order (i.e., the first item in the returned list is the uppermost item).

See also QGraphicsScene::items() and Sorting.
*/
impl /*struct*/ QGraphicsView {
  pub fn items_4<RetType, T: QGraphicsView_items_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.items_4(self);
    // return 1;
  }
}
pub trait QGraphicsView_items_4<RetType> {
  fn items_4(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_items_4<usize> for (i32,i32,i32,i32,i32) {
  fn items_4(self , rsthis: & QGraphicsView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView5itemsEiiiiN2Qt17ItemSelectionModeE", 5,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:199
// index:5
// Public Visibility=Default Availability=Available
// [8] QList<QGraphicsItem *> items(const QPolygon &, Qt::ItemSelectionMode) const

/*
Returns a list of all the items in the associated scene, in descending stacking order (i.e., the first item in the returned list is the uppermost item).

See also QGraphicsScene::items() and Sorting.
*/
impl /*struct*/ QGraphicsView {
  pub fn items_5<RetType, T: QGraphicsView_items_5<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.items_5(self);
    // return 1;
  }
}
pub trait QGraphicsView_items_5<RetType> {
  fn items_5(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_items_5<usize> for (usize,i32) {
  fn items_5(self , rsthis: & QGraphicsView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView5itemsERK8QPolygonN2Qt17ItemSelectionModeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:200
// index:6
// Public Visibility=Default Availability=Available
// [8] QList<QGraphicsItem *> items(const QPainterPath &, Qt::ItemSelectionMode) const

/*
Returns a list of all the items in the associated scene, in descending stacking order (i.e., the first item in the returned list is the uppermost item).

See also QGraphicsScene::items() and Sorting.
*/
impl /*struct*/ QGraphicsView {
  pub fn items_6<RetType, T: QGraphicsView_items_6<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.items_6(self);
    // return 1;
  }
}
pub trait QGraphicsView_items_6<RetType> {
  fn items_6(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_items_6<usize> for (usize,i32) {
  fn items_6(self , rsthis: & QGraphicsView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView5itemsERK12QPainterPathN2Qt17ItemSelectionModeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:201
// index:0
// Public Visibility=Default Availability=Available
// [8] QGraphicsItem * itemAt(const QPoint &) const

/*
Returns the item at position pos, which is in viewport coordinates. If there are several items at this position, this function returns the topmost item.

Example:


  void CustomView::mousePressEvent(QMouseEvent *event)
  {
      if (QGraphicsItem *item = itemAt(event->pos())) {
          qDebug() << "You clicked on item" << item;
      } else {
          qDebug("You didn't click on an item.");
      }
  }



See also items() and Sorting.
*/
impl /*struct*/ QGraphicsView {
  pub fn itemAt_0<RetType, T: QGraphicsView_itemAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemAt_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_itemAt_0<RetType> {
  fn itemAt_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_itemAt_0<usize> for (usize) {
  fn itemAt_0(self , rsthis: & QGraphicsView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView6itemAtERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:202
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QGraphicsItem * itemAt(int, int) const

/*
Returns the item at position pos, which is in viewport coordinates. If there are several items at this position, this function returns the topmost item.

Example:


  void CustomView::mousePressEvent(QMouseEvent *event)
  {
      if (QGraphicsItem *item = itemAt(event->pos())) {
          qDebug() << "You clicked on item" << item;
      } else {
          qDebug("You didn't click on an item.");
      }
  }



See also items() and Sorting.
*/
impl /*struct*/ QGraphicsView {
  pub fn itemAt_1<RetType, T: QGraphicsView_itemAt_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemAt_1(self);
    // return 1;
  }
}
pub trait QGraphicsView_itemAt_1<RetType> {
  fn itemAt_1(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_itemAt_1<usize> for (i32,i32) {
  fn itemAt_1(self , rsthis: & QGraphicsView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView6itemAtEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:204
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF mapToScene(const QPoint &) const

/*
Returns the viewport coordinate point mapped to scene coordinates.

Note: It can be useful to map the whole rectangle covered by the pixel at point instead of the point itself. To do this, you can call mapToScene(QRect(point, QSize(2, 2))).

See also mapFromScene().
*/
impl /*struct*/ QGraphicsView {
  pub fn mapToScene_0<RetType, T: QGraphicsView_mapToScene_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapToScene_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_mapToScene_0<RetType> {
  fn mapToScene_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_mapToScene_0<usize> for (usize) {
  fn mapToScene_0(self , rsthis: & QGraphicsView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView10mapToSceneERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:205
// index:1
// Public Visibility=Default Availability=Available
// [8] QPolygonF mapToScene(const QRect &) const

/*
Returns the viewport coordinate point mapped to scene coordinates.

Note: It can be useful to map the whole rectangle covered by the pixel at point instead of the point itself. To do this, you can call mapToScene(QRect(point, QSize(2, 2))).

See also mapFromScene().
*/
impl /*struct*/ QGraphicsView {
  pub fn mapToScene_1<RetType, T: QGraphicsView_mapToScene_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapToScene_1(self);
    // return 1;
  }
}
pub trait QGraphicsView_mapToScene_1<RetType> {
  fn mapToScene_1(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_mapToScene_1<usize> for (usize) {
  fn mapToScene_1(self , rsthis: & QGraphicsView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView10mapToSceneERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:206
// index:2
// Public Visibility=Default Availability=Available
// [8] QPolygonF mapToScene(const QPolygon &) const

/*
Returns the viewport coordinate point mapped to scene coordinates.

Note: It can be useful to map the whole rectangle covered by the pixel at point instead of the point itself. To do this, you can call mapToScene(QRect(point, QSize(2, 2))).

See also mapFromScene().
*/
impl /*struct*/ QGraphicsView {
  pub fn mapToScene_2<RetType, T: QGraphicsView_mapToScene_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapToScene_2(self);
    // return 1;
  }
}
pub trait QGraphicsView_mapToScene_2<RetType> {
  fn mapToScene_2(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_mapToScene_2<usize> for (usize) {
  fn mapToScene_2(self , rsthis: & QGraphicsView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView10mapToSceneERK8QPolygon", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:207
// index:3
// Public Visibility=Default Availability=Available
// [8] QPainterPath mapToScene(const QPainterPath &) const

/*
Returns the viewport coordinate point mapped to scene coordinates.

Note: It can be useful to map the whole rectangle covered by the pixel at point instead of the point itself. To do this, you can call mapToScene(QRect(point, QSize(2, 2))).

See also mapFromScene().
*/
impl /*struct*/ QGraphicsView {
  pub fn mapToScene_3<RetType, T: QGraphicsView_mapToScene_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapToScene_3(self);
    // return 1;
  }
}
pub trait QGraphicsView_mapToScene_3<RetType> {
  fn mapToScene_3(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_mapToScene_3<usize> for (usize) {
  fn mapToScene_3(self , rsthis: & QGraphicsView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView10mapToSceneERK12QPainterPath", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:212
// index:4
// Public inline Visibility=Default Availability=Available
// [16] QPointF mapToScene(int, int) const

/*
Returns the viewport coordinate point mapped to scene coordinates.

Note: It can be useful to map the whole rectangle covered by the pixel at point instead of the point itself. To do this, you can call mapToScene(QRect(point, QSize(2, 2))).

See also mapFromScene().
*/
impl /*struct*/ QGraphicsView {
  pub fn mapToScene_4<RetType, T: QGraphicsView_mapToScene_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapToScene_4(self);
    // return 1;
  }
}
pub trait QGraphicsView_mapToScene_4<RetType> {
  fn mapToScene_4(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_mapToScene_4<usize> for (i32,i32) {
  fn mapToScene_4(self , rsthis: & QGraphicsView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView10mapToSceneEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:213
// index:5
// Public inline Visibility=Default Availability=Available
// [8] QPolygonF mapToScene(int, int, int, int) const

/*
Returns the viewport coordinate point mapped to scene coordinates.

Note: It can be useful to map the whole rectangle covered by the pixel at point instead of the point itself. To do this, you can call mapToScene(QRect(point, QSize(2, 2))).

See also mapFromScene().
*/
impl /*struct*/ QGraphicsView {
  pub fn mapToScene_5<RetType, T: QGraphicsView_mapToScene_5<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapToScene_5(self);
    // return 1;
  }
}
pub trait QGraphicsView_mapToScene_5<RetType> {
  fn mapToScene_5(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_mapToScene_5<usize> for (i32,i32,i32,i32) {
  fn mapToScene_5(self , rsthis: & QGraphicsView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView10mapToSceneEiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:208
// index:0
// Public Visibility=Default Availability=Available
// [8] QPoint mapFromScene(const QPointF &) const

/*
Returns the scene coordinate point to viewport coordinates.

See also mapToScene().
*/
impl /*struct*/ QGraphicsView {
  pub fn mapFromScene_0<RetType, T: QGraphicsView_mapFromScene_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapFromScene_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_mapFromScene_0<RetType> {
  fn mapFromScene_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_mapFromScene_0<usize> for (usize) {
  fn mapFromScene_0(self , rsthis: & QGraphicsView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView12mapFromSceneERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:209
// index:1
// Public Visibility=Default Availability=Available
// [8] QPolygon mapFromScene(const QRectF &) const

/*
Returns the scene coordinate point to viewport coordinates.

See also mapToScene().
*/
impl /*struct*/ QGraphicsView {
  pub fn mapFromScene_1<RetType, T: QGraphicsView_mapFromScene_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapFromScene_1(self);
    // return 1;
  }
}
pub trait QGraphicsView_mapFromScene_1<RetType> {
  fn mapFromScene_1(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_mapFromScene_1<usize> for (usize) {
  fn mapFromScene_1(self , rsthis: & QGraphicsView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView12mapFromSceneERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:210
// index:2
// Public Visibility=Default Availability=Available
// [8] QPolygon mapFromScene(const QPolygonF &) const

/*
Returns the scene coordinate point to viewport coordinates.

See also mapToScene().
*/
impl /*struct*/ QGraphicsView {
  pub fn mapFromScene_2<RetType, T: QGraphicsView_mapFromScene_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapFromScene_2(self);
    // return 1;
  }
}
pub trait QGraphicsView_mapFromScene_2<RetType> {
  fn mapFromScene_2(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_mapFromScene_2<usize> for (usize) {
  fn mapFromScene_2(self , rsthis: & QGraphicsView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView12mapFromSceneERK9QPolygonF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:211
// index:3
// Public Visibility=Default Availability=Available
// [8] QPainterPath mapFromScene(const QPainterPath &) const

/*
Returns the scene coordinate point to viewport coordinates.

See also mapToScene().
*/
impl /*struct*/ QGraphicsView {
  pub fn mapFromScene_3<RetType, T: QGraphicsView_mapFromScene_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapFromScene_3(self);
    // return 1;
  }
}
pub trait QGraphicsView_mapFromScene_3<RetType> {
  fn mapFromScene_3(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_mapFromScene_3<usize> for (usize) {
  fn mapFromScene_3(self , rsthis: & QGraphicsView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView12mapFromSceneERK12QPainterPath", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:214
// index:4
// Public inline Visibility=Default Availability=Available
// [8] QPoint mapFromScene(qreal, qreal) const

/*
Returns the scene coordinate point to viewport coordinates.

See also mapToScene().
*/
impl /*struct*/ QGraphicsView {
  pub fn mapFromScene_4<RetType, T: QGraphicsView_mapFromScene_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapFromScene_4(self);
    // return 1;
  }
}
pub trait QGraphicsView_mapFromScene_4<RetType> {
  fn mapFromScene_4(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_mapFromScene_4<usize> for (f64,f64) {
  fn mapFromScene_4(self , rsthis: & QGraphicsView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView12mapFromSceneEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:215
// index:5
// Public inline Visibility=Default Availability=Available
// [8] QPolygon mapFromScene(qreal, qreal, qreal, qreal) const

/*
Returns the scene coordinate point to viewport coordinates.

See also mapToScene().
*/
impl /*struct*/ QGraphicsView {
  pub fn mapFromScene_5<RetType, T: QGraphicsView_mapFromScene_5<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapFromScene_5(self);
    // return 1;
  }
}
pub trait QGraphicsView_mapFromScene_5<RetType> {
  fn mapFromScene_5(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_mapFromScene_5<usize> for (f64,f64,f64,f64) {
  fn mapFromScene_5(self , rsthis: & QGraphicsView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView12mapFromSceneEdddd", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:217
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QVariant inputMethodQuery(Qt::InputMethodQuery) const

/*
Reimplemented from QWidget::inputMethodQuery().
*/
impl /*struct*/ QGraphicsView {
  pub fn inputMethodQuery_0<RetType, T: QGraphicsView_inputMethodQuery_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inputMethodQuery_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_inputMethodQuery_0<RetType> {
  fn inputMethodQuery_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_inputMethodQuery_0<usize> for (i32) {
  fn inputMethodQuery_0(self , rsthis: & QGraphicsView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView16inputMethodQueryEN2Qt16InputMethodQueryE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:219
// index:0
// Public Visibility=Default Availability=Available
// [8] QBrush backgroundBrush() const

/*

*/
impl /*struct*/ QGraphicsView {
  pub fn backgroundBrush_0<RetType, T: QGraphicsView_backgroundBrush_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.backgroundBrush_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_backgroundBrush_0<RetType> {
  fn backgroundBrush_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_backgroundBrush_0<usize> for () {
  fn backgroundBrush_0(self , rsthis: & QGraphicsView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView15backgroundBrushEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:220
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setBackgroundBrush(const QBrush &)

/*

*/
impl /*struct*/ QGraphicsView {
  pub fn setBackgroundBrush_0<RetType, T: QGraphicsView_setBackgroundBrush_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBackgroundBrush_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_setBackgroundBrush_0<RetType> {
  fn setBackgroundBrush_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_setBackgroundBrush_0<(/*void*/)> for (usize) {
  fn setBackgroundBrush_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView18setBackgroundBrushERK6QBrush", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:222
// index:0
// Public Visibility=Default Availability=Available
// [8] QBrush foregroundBrush() const

/*

*/
impl /*struct*/ QGraphicsView {
  pub fn foregroundBrush_0<RetType, T: QGraphicsView_foregroundBrush_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.foregroundBrush_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_foregroundBrush_0<RetType> {
  fn foregroundBrush_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_foregroundBrush_0<usize> for () {
  fn foregroundBrush_0(self , rsthis: & QGraphicsView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGraphicsView15foregroundBrushEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:223
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setForegroundBrush(const QBrush &)

/*

*/
impl /*struct*/ QGraphicsView {
  pub fn setForegroundBrush_0<RetType, T: QGraphicsView_setForegroundBrush_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setForegroundBrush_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_setForegroundBrush_0<RetType> {
  fn setForegroundBrush_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_setForegroundBrush_0<(/*void*/)> for (usize) {
  fn setForegroundBrush_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView18setForegroundBrushERK6QBrush", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:227
// index:0
// Public Visibility=Default Availability=Available
// [-2] void invalidateScene(const QRectF &, QGraphicsScene::SceneLayers)

/*
Invalidates and schedules a redraw of layers inside rect. rect is in scene coordinates. Any cached content for layers inside rect is unconditionally invalidated and redrawn.

You can call this function to notify QGraphicsView of changes to the background or the foreground of the scene. It is commonly used for scenes with tile-based backgrounds to notify changes when QGraphicsView has enabled background caching.

Note that QGraphicsView currently supports background caching only (see QGraphicsView::CacheBackground). This function is equivalent to calling update() if any layer but QGraphicsScene::BackgroundLayer is passed.

See also QGraphicsScene::invalidate() and update().
*/
impl /*struct*/ QGraphicsView {
  pub fn invalidateScene_0<RetType, T: QGraphicsView_invalidateScene_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.invalidateScene_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_invalidateScene_0<RetType> {
  fn invalidateScene_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_invalidateScene_0<(/*void*/)> for (usize,i32) {
  fn invalidateScene_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView15invalidateSceneERK6QRectF6QFlagsIN14QGraphicsScene10SceneLayerEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:228
// index:0
// Public Visibility=Default Availability=Available
// [-2] void updateSceneRect(const QRectF &)

/*
Notifies QGraphicsView that the scene's scene rect has changed. rect is the new scene rect. If the view already has an explicitly set scene rect, this function does nothing.

See also sceneRect and QGraphicsScene::sceneRectChanged().
*/
impl /*struct*/ QGraphicsView {
  pub fn updateSceneRect_0<RetType, T: QGraphicsView_updateSceneRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updateSceneRect_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_updateSceneRect_0<RetType> {
  fn updateSceneRect_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_updateSceneRect_0<(/*void*/)> for (usize) {
  fn updateSceneRect_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView15updateSceneRectERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:232
// index:0
// Public Visibility=Default Availability=Available
// [-2] void rubberBandChanged(QRect, QPointF, QPointF)

/*
This signal is emitted when the rubber band rect is changed. The viewport Rect is specified by rubberBandRect. The drag start position and drag end position are provided in scene points with fromScenePoint and toScenePoint.

When rubberband selection ends this signal will be emitted with null vales.

This function was introduced in  Qt 5.1.

See also rubberBandRect().
*/
impl /*struct*/ QGraphicsView {
  pub fn rubberBandChanged_0<RetType, T: QGraphicsView_rubberBandChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rubberBandChanged_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_rubberBandChanged_0<RetType> {
  fn rubberBandChanged_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_rubberBandChanged_0<(/*void*/)> for (usize,usize,usize) {
  fn rubberBandChanged_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView17rubberBandChangedE5QRect7QPointFS1_", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:236
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void setupViewport(QWidget *)

/*
Reimplemented from QAbstractScrollArea::setupViewport().

This slot is called by QAbstractScrollArea after setViewport() has been called. Reimplement this function in a subclass of QGraphicsView to initialize the new viewport widget before it is used.

See also setViewport().
*/
impl /*struct*/ QGraphicsView {
  pub fn setupViewport_0<RetType, T: QGraphicsView_setupViewport_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setupViewport_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_setupViewport_0<RetType> {
  fn setupViewport_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_setupViewport_0<(/*void*/)> for (usize) {
  fn setupViewport_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView13setupViewportEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:240
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QGraphicsView {
  pub fn event_0<RetType, T: QGraphicsView_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_event_0<RetType> {
  fn event_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QGraphicsView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QGraphicsView5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:241
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool viewportEvent(QEvent *)

/*
Reimplemented from QAbstractScrollArea::viewportEvent().
*/
impl /*struct*/ QGraphicsView {
  pub fn viewportEvent_0<RetType, T: QGraphicsView_viewportEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.viewportEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_viewportEvent_0<RetType> {
  fn viewportEvent_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_viewportEvent_0<bool> for (usize) {
  fn viewportEvent_0(self , rsthis: & QGraphicsView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QGraphicsView13viewportEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:244
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void contextMenuEvent(QContextMenuEvent *)

/*
Reimplemented from QWidget::contextMenuEvent().
*/
impl /*struct*/ QGraphicsView {
  pub fn contextMenuEvent_0<RetType, T: QGraphicsView_contextMenuEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contextMenuEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_contextMenuEvent_0<RetType> {
  fn contextMenuEvent_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_contextMenuEvent_0<(/*void*/)> for (usize) {
  fn contextMenuEvent_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView16contextMenuEventEP17QContextMenuEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:247
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dragEnterEvent(QDragEnterEvent *)

/*
Reimplemented from QWidget::dragEnterEvent().
*/
impl /*struct*/ QGraphicsView {
  pub fn dragEnterEvent_0<RetType, T: QGraphicsView_dragEnterEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragEnterEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_dragEnterEvent_0<RetType> {
  fn dragEnterEvent_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_dragEnterEvent_0<(/*void*/)> for (usize) {
  fn dragEnterEvent_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView14dragEnterEventEP15QDragEnterEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:248
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dragLeaveEvent(QDragLeaveEvent *)

/*
Reimplemented from QWidget::dragLeaveEvent().
*/
impl /*struct*/ QGraphicsView {
  pub fn dragLeaveEvent_0<RetType, T: QGraphicsView_dragLeaveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragLeaveEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_dragLeaveEvent_0<RetType> {
  fn dragLeaveEvent_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_dragLeaveEvent_0<(/*void*/)> for (usize) {
  fn dragLeaveEvent_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView14dragLeaveEventEP15QDragLeaveEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:249
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dragMoveEvent(QDragMoveEvent *)

/*
Reimplemented from QWidget::dragMoveEvent().
*/
impl /*struct*/ QGraphicsView {
  pub fn dragMoveEvent_0<RetType, T: QGraphicsView_dragMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragMoveEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_dragMoveEvent_0<RetType> {
  fn dragMoveEvent_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_dragMoveEvent_0<(/*void*/)> for (usize) {
  fn dragMoveEvent_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView13dragMoveEventEP14QDragMoveEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:250
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dropEvent(QDropEvent *)

/*
Reimplemented from QWidget::dropEvent().
*/
impl /*struct*/ QGraphicsView {
  pub fn dropEvent_0<RetType, T: QGraphicsView_dropEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dropEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_dropEvent_0<RetType> {
  fn dropEvent_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_dropEvent_0<(/*void*/)> for (usize) {
  fn dropEvent_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView9dropEventEP10QDropEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:252
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusInEvent(QFocusEvent *)

/*
Reimplemented from QWidget::focusInEvent().
*/
impl /*struct*/ QGraphicsView {
  pub fn focusInEvent_0<RetType, T: QGraphicsView_focusInEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusInEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_focusInEvent_0<RetType> {
  fn focusInEvent_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_focusInEvent_0<(/*void*/)> for (usize) {
  fn focusInEvent_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView12focusInEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:253
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool focusNextPrevChild(bool)

/*
Reimplemented from QWidget::focusNextPrevChild().
*/
impl /*struct*/ QGraphicsView {
  pub fn focusNextPrevChild_0<RetType, T: QGraphicsView_focusNextPrevChild_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusNextPrevChild_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_focusNextPrevChild_0<RetType> {
  fn focusNextPrevChild_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_focusNextPrevChild_0<bool> for (bool) {
  fn focusNextPrevChild_0(self , rsthis: & QGraphicsView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QGraphicsView18focusNextPrevChildEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:254
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusOutEvent(QFocusEvent *)

/*
Reimplemented from QWidget::focusOutEvent().
*/
impl /*struct*/ QGraphicsView {
  pub fn focusOutEvent_0<RetType, T: QGraphicsView_focusOutEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusOutEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_focusOutEvent_0<RetType> {
  fn focusOutEvent_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_focusOutEvent_0<(/*void*/)> for (usize) {
  fn focusOutEvent_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView13focusOutEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:255
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyPressEvent(QKeyEvent *)

/*
Reimplemented from QWidget::keyPressEvent().
*/
impl /*struct*/ QGraphicsView {
  pub fn keyPressEvent_0<RetType, T: QGraphicsView_keyPressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyPressEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_keyPressEvent_0<RetType> {
  fn keyPressEvent_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_keyPressEvent_0<(/*void*/)> for (usize) {
  fn keyPressEvent_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView13keyPressEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:256
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyReleaseEvent(QKeyEvent *)

/*
Reimplemented from QWidget::keyReleaseEvent().
*/
impl /*struct*/ QGraphicsView {
  pub fn keyReleaseEvent_0<RetType, T: QGraphicsView_keyReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_keyReleaseEvent_0<RetType> {
  fn keyReleaseEvent_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_keyReleaseEvent_0<(/*void*/)> for (usize) {
  fn keyReleaseEvent_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView15keyReleaseEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:257
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseDoubleClickEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseDoubleClickEvent().
*/
impl /*struct*/ QGraphicsView {
  pub fn mouseDoubleClickEvent_0<RetType, T: QGraphicsView_mouseDoubleClickEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseDoubleClickEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_mouseDoubleClickEvent_0<RetType> {
  fn mouseDoubleClickEvent_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_mouseDoubleClickEvent_0<(/*void*/)> for (usize) {
  fn mouseDoubleClickEvent_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView21mouseDoubleClickEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:258
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mousePressEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mousePressEvent().
*/
impl /*struct*/ QGraphicsView {
  pub fn mousePressEvent_0<RetType, T: QGraphicsView_mousePressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mousePressEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_mousePressEvent_0<RetType> {
  fn mousePressEvent_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_mousePressEvent_0<(/*void*/)> for (usize) {
  fn mousePressEvent_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView15mousePressEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:259
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseMoveEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseMoveEvent().
*/
impl /*struct*/ QGraphicsView {
  pub fn mouseMoveEvent_0<RetType, T: QGraphicsView_mouseMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseMoveEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_mouseMoveEvent_0<RetType> {
  fn mouseMoveEvent_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_mouseMoveEvent_0<(/*void*/)> for (usize) {
  fn mouseMoveEvent_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView14mouseMoveEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:260
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseReleaseEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseReleaseEvent().
*/
impl /*struct*/ QGraphicsView {
  pub fn mouseReleaseEvent_0<RetType, T: QGraphicsView_mouseReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_mouseReleaseEvent_0<RetType> {
  fn mouseReleaseEvent_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_mouseReleaseEvent_0<(/*void*/)> for (usize) {
  fn mouseReleaseEvent_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView17mouseReleaseEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:262
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void wheelEvent(QWheelEvent *)

/*
Reimplemented from QWidget::wheelEvent().
*/
impl /*struct*/ QGraphicsView {
  pub fn wheelEvent_0<RetType, T: QGraphicsView_wheelEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.wheelEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_wheelEvent_0<RetType> {
  fn wheelEvent_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_wheelEvent_0<(/*void*/)> for (usize) {
  fn wheelEvent_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView10wheelEventEP11QWheelEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:264
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Reimplemented from QWidget::paintEvent().
*/
impl /*struct*/ QGraphicsView {
  pub fn paintEvent_0<RetType, T: QGraphicsView_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:265
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void resizeEvent(QResizeEvent *)

/*
Reimplemented from QWidget::resizeEvent().
*/
impl /*struct*/ QGraphicsView {
  pub fn resizeEvent_0<RetType, T: QGraphicsView_resizeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_resizeEvent_0<RetType> {
  fn resizeEvent_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_resizeEvent_0<(/*void*/)> for (usize) {
  fn resizeEvent_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView11resizeEventEP12QResizeEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:266
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void scrollContentsBy(int, int)

/*
Reimplemented from QAbstractScrollArea::scrollContentsBy().
*/
impl /*struct*/ QGraphicsView {
  pub fn scrollContentsBy_0<RetType, T: QGraphicsView_scrollContentsBy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scrollContentsBy_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_scrollContentsBy_0<RetType> {
  fn scrollContentsBy_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_scrollContentsBy_0<(/*void*/)> for (i32,i32) {
  fn scrollContentsBy_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView16scrollContentsByEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:267
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void showEvent(QShowEvent *)

/*
Reimplemented from QWidget::showEvent().
*/
impl /*struct*/ QGraphicsView {
  pub fn showEvent_0<RetType, T: QGraphicsView_showEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_showEvent_0<RetType> {
  fn showEvent_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_showEvent_0<(/*void*/)> for (usize) {
  fn showEvent_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView9showEventEP10QShowEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:268
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void inputMethodEvent(QInputMethodEvent *)

/*
Reimplemented from QWidget::inputMethodEvent().
*/
impl /*struct*/ QGraphicsView {
  pub fn inputMethodEvent_0<RetType, T: QGraphicsView_inputMethodEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inputMethodEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_inputMethodEvent_0<RetType> {
  fn inputMethodEvent_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_inputMethodEvent_0<(/*void*/)> for (usize) {
  fn inputMethodEvent_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView16inputMethodEventEP17QInputMethodEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:270
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void drawBackground(QPainter *, const QRectF &)

/*
Draws the background of the scene using painter, before any items and the foreground are drawn. Reimplement this function to provide a custom background for this view.

If all you want is to define a color, texture or gradient for the background, you can call setBackgroundBrush() instead.

All painting is done in scene coordinates. rect is the exposed rectangle.

The default implementation fills rect using the view's backgroundBrush. If no such brush is defined (the default), the scene's drawBackground() function is called instead.

See also drawForeground() and QGraphicsScene::drawBackground().
*/
impl /*struct*/ QGraphicsView {
  pub fn drawBackground_0<RetType, T: QGraphicsView_drawBackground_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawBackground_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_drawBackground_0<RetType> {
  fn drawBackground_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_drawBackground_0<(/*void*/)> for (usize,usize) {
  fn drawBackground_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView14drawBackgroundEP8QPainterRK6QRectF", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:271
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void drawForeground(QPainter *, const QRectF &)

/*
Draws the foreground of the scene using painter, after the background and all items are drawn. Reimplement this function to provide a custom foreground for this view.

If all you want is to define a color, texture or gradient for the foreground, you can call setForegroundBrush() instead.

All painting is done in scene coordinates. rect is the exposed rectangle.

The default implementation fills rect using the view's foregroundBrush. If no such brush is defined (the default), the scene's drawForeground() function is called instead.

See also drawBackground() and QGraphicsScene::drawForeground().
*/
impl /*struct*/ QGraphicsView {
  pub fn drawForeground_0<RetType, T: QGraphicsView_drawForeground_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawForeground_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_drawForeground_0<RetType> {
  fn drawForeground_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_drawForeground_0<(/*void*/)> for (usize,usize) {
  fn drawForeground_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView14drawForegroundEP8QPainterRK6QRectF", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsview.h:272
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void drawItems(QPainter *, int, QGraphicsItem **, const QStyleOptionGraphicsItem *)

/*

*/
impl /*struct*/ QGraphicsView {
  pub fn drawItems_0<RetType, T: QGraphicsView_drawItems_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawItems_0(self);
    // return 1;
  }
}
pub trait QGraphicsView_drawItems_0<RetType> {
  fn drawItems_0(self , rsthis: & QGraphicsView) -> RetType;
}
impl<'a> /*trait*/ QGraphicsView_drawItems_0<(/*void*/)> for (usize,i32,usize,usize) {
  fn drawItems_0(self , rsthis: & QGraphicsView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let arg3 = (&self.3) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGraphicsView9drawItemsEP8QPainteriPP13QGraphicsItemPK24QStyleOptionGraphicsItem", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
This enums describe the possible anchors that QGraphicsView can use when the user resizes the view or when the view is transformed.



See also resizeAnchor and transformationAnchor.

*/
pub type QGraphicsView__ViewportAnchor = i32;
// No anchor, i.e. the view leaves the scene's position unchanged.
pub const QGraphicsView__NoAnchor :QGraphicsView__ViewportAnchor = 0;
// The scene point at the center of the view is used as the anchor.
pub const QGraphicsView__AnchorViewCenter :QGraphicsView__ViewportAnchor = 1;
// The point under the mouse is used as the anchor.
pub const QGraphicsView__AnchorUnderMouse :QGraphicsView__ViewportAnchor = 2;
pub fn QGraphicsView_ViewportAnchorItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QGraphicsView", val);
}
pub fn QGraphicsView_ViewportAnchorItemName_s(val: i32) ->String {
  //var nilthis *QGraphicsView
  //return nilthis.ViewportAnchorItemName(val);
  return QGraphicsView_ViewportAnchorItemName(val);
}


/*


*/
pub type QGraphicsView__CacheModeFlag = i32;
// 
pub const QGraphicsView__CacheNone :QGraphicsView__CacheModeFlag = 0;
// 
pub const QGraphicsView__CacheBackground :QGraphicsView__CacheModeFlag = 1;
pub fn QGraphicsView_CacheModeFlagItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QGraphicsView", val);
}
pub fn QGraphicsView_CacheModeFlagItemName_s(val: i32) ->String {
  //var nilthis *QGraphicsView
  //return nilthis.CacheModeFlagItemName(val);
  return QGraphicsView_CacheModeFlagItemName(val);
}


/*
This enum describes the default action for the view when pressing and dragging the mouse over the viewport.



See also dragMode and QGraphicsScene::setSelectionArea().

*/
pub type QGraphicsView__DragMode = i32;
// Nothing happens; the mouse event is ignored.
pub const QGraphicsView__NoDrag :QGraphicsView__DragMode = 0;
// The cursor changes into a pointing hand, and dragging the mouse around will scroll the scrolbars. This mode works both in interactive and non-interactive mode.
pub const QGraphicsView__ScrollHandDrag :QGraphicsView__DragMode = 1;
// A rubber band will appear. Dragging the mouse will set the rubber band geometry, and all items covered by the rubber band are selected. This mode is disabled for non-interactive views.
pub const QGraphicsView__RubberBandDrag :QGraphicsView__DragMode = 2;
pub fn QGraphicsView_DragModeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QGraphicsView", val);
}
pub fn QGraphicsView_DragModeItemName_s(val: i32) ->String {
  //var nilthis *QGraphicsView
  //return nilthis.DragModeItemName(val);
  return QGraphicsView_DragModeItemName(val);
}


/*
This enum describes how QGraphicsView updates its viewport when the scene contents change or are exposed.



This enum was introduced or modified in  Qt 4.3.

See also viewportUpdateMode.

*/
pub type QGraphicsView__ViewportUpdateMode = i32;
// When any visible part of the scene changes or is reexposed, QGraphicsView will update the entire viewport. This approach is fastest when QGraphicsView spends more time figuring out what to draw than it would spend drawing (e.g., when very many small items are repeatedly updated). This is the preferred update mode for viewports that do not support partial updates, such as QGLWidget, and for viewports that need to disable scroll optimization.
pub const QGraphicsView__FullViewportUpdate :QGraphicsView__ViewportUpdateMode = 0;
// QGraphicsView will determine the minimal viewport region that requires a redraw, minimizing the time spent drawing by avoiding a redraw of areas that have not changed. This is QGraphicsView's default mode. Although this approach provides the best performance in general, if there are many small visible changes on the scene, QGraphicsView might end up spending more time finding the minimal approach than it will spend drawing.
pub const QGraphicsView__MinimalViewportUpdate :QGraphicsView__ViewportUpdateMode = 1;
// QGraphicsView will attempt to find an optimal update mode by analyzing the areas that require a redraw.
pub const QGraphicsView__SmartViewportUpdate :QGraphicsView__ViewportUpdateMode = 2;
// QGraphicsView will never update its viewport when the scene changes; the user is expected to control all updates. This mode disables all (potentially slow) item visibility testing in QGraphicsView, and is suitable for scenes that either require a fixed frame rate, or where the viewport is otherwise updated externally.
pub const QGraphicsView__NoViewportUpdate :QGraphicsView__ViewportUpdateMode = 3;
// The bounding rectangle of all changes in the viewport will be redrawn. This mode has the advantage that QGraphicsView searches only one region for changes, minimizing time spent determining what needs redrawing. The disadvantage is that areas that have not changed also need to be redrawn.
pub const QGraphicsView__BoundingRectViewportUpdate :QGraphicsView__ViewportUpdateMode = 4;
pub fn QGraphicsView_ViewportUpdateModeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QGraphicsView", val);
}
pub fn QGraphicsView_ViewportUpdateModeItemName_s(val: i32) ->String {
  //var nilthis *QGraphicsView
  //return nilthis.ViewportUpdateModeItemName(val);
  return QGraphicsView_ViewportUpdateModeItemName(val);
}


/*


*/
pub type QGraphicsView__OptimizationFlag = i32;
// 
pub const QGraphicsView__DontClipPainter :QGraphicsView__OptimizationFlag = 1;
// 
pub const QGraphicsView__DontSavePainterState :QGraphicsView__OptimizationFlag = 2;
// 
pub const QGraphicsView__DontAdjustForAntialiasing :QGraphicsView__OptimizationFlag = 4;
// 
pub const QGraphicsView__IndirectPainting :QGraphicsView__OptimizationFlag = 8;
pub fn QGraphicsView_OptimizationFlagItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QGraphicsView", val);
}
pub fn QGraphicsView_OptimizationFlagItemName_s(val: i32) ->String {
  //var nilthis *QGraphicsView
  //return nilthis.OptimizationFlagItemName(val);
  return QGraphicsView_OptimizationFlagItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
