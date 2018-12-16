

// mod ::widgets::QGraphicsProxyWidget
// package qtwidgets
// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h
// #include <qgraphicsproxywidget.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 82
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

// QVariant itemChange(QGraphicsItem::GraphicsItemChange, const QVariant &)
// func (this *QGraphicsProxyWidget) InheritItemChange(f func(change int, value *qtcore.QVariant) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "itemChange", f)
// }

// bool event(QEvent *)
// func (this *QGraphicsProxyWidget) InheritEvent(f func(event *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// bool eventFilter(QObject *, QEvent *)
// func (this *QGraphicsProxyWidget) InheritEventFilter(f func(object *qtcore.QObject/*777 QObject **/, event *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "eventFilter", f)
// }

// void showEvent(QShowEvent *)
// func (this *QGraphicsProxyWidget) InheritShowEvent(f func(event *qtgui.QShowEvent/*777 QShowEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "showEvent", f)
// }

// void hideEvent(QHideEvent *)
// func (this *QGraphicsProxyWidget) InheritHideEvent(f func(event *qtgui.QHideEvent/*777 QHideEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "hideEvent", f)
// }

// void contextMenuEvent(QGraphicsSceneContextMenuEvent *)
// func (this *QGraphicsProxyWidget) InheritContextMenuEvent(f func(event *QGraphicsSceneContextMenuEvent/*777 QGraphicsSceneContextMenuEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "contextMenuEvent", f)
// }

// void dragEnterEvent(QGraphicsSceneDragDropEvent *)
// func (this *QGraphicsProxyWidget) InheritDragEnterEvent(f func(event *QGraphicsSceneDragDropEvent/*777 QGraphicsSceneDragDropEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dragEnterEvent", f)
// }

// void dragLeaveEvent(QGraphicsSceneDragDropEvent *)
// func (this *QGraphicsProxyWidget) InheritDragLeaveEvent(f func(event *QGraphicsSceneDragDropEvent/*777 QGraphicsSceneDragDropEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dragLeaveEvent", f)
// }

// void dragMoveEvent(QGraphicsSceneDragDropEvent *)
// func (this *QGraphicsProxyWidget) InheritDragMoveEvent(f func(event *QGraphicsSceneDragDropEvent/*777 QGraphicsSceneDragDropEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dragMoveEvent", f)
// }

// void dropEvent(QGraphicsSceneDragDropEvent *)
// func (this *QGraphicsProxyWidget) InheritDropEvent(f func(event *QGraphicsSceneDragDropEvent/*777 QGraphicsSceneDragDropEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dropEvent", f)
// }

// void hoverEnterEvent(QGraphicsSceneHoverEvent *)
// func (this *QGraphicsProxyWidget) InheritHoverEnterEvent(f func(event *QGraphicsSceneHoverEvent/*777 QGraphicsSceneHoverEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "hoverEnterEvent", f)
// }

// void hoverLeaveEvent(QGraphicsSceneHoverEvent *)
// func (this *QGraphicsProxyWidget) InheritHoverLeaveEvent(f func(event *QGraphicsSceneHoverEvent/*777 QGraphicsSceneHoverEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "hoverLeaveEvent", f)
// }

// void hoverMoveEvent(QGraphicsSceneHoverEvent *)
// func (this *QGraphicsProxyWidget) InheritHoverMoveEvent(f func(event *QGraphicsSceneHoverEvent/*777 QGraphicsSceneHoverEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "hoverMoveEvent", f)
// }

// void grabMouseEvent(QEvent *)
// func (this *QGraphicsProxyWidget) InheritGrabMouseEvent(f func(event *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "grabMouseEvent", f)
// }

// void ungrabMouseEvent(QEvent *)
// func (this *QGraphicsProxyWidget) InheritUngrabMouseEvent(f func(event *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "ungrabMouseEvent", f)
// }

// void mouseMoveEvent(QGraphicsSceneMouseEvent *)
// func (this *QGraphicsProxyWidget) InheritMouseMoveEvent(f func(event *QGraphicsSceneMouseEvent/*777 QGraphicsSceneMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseMoveEvent", f)
// }

// void mousePressEvent(QGraphicsSceneMouseEvent *)
// func (this *QGraphicsProxyWidget) InheritMousePressEvent(f func(event *QGraphicsSceneMouseEvent/*777 QGraphicsSceneMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mousePressEvent", f)
// }

// void mouseReleaseEvent(QGraphicsSceneMouseEvent *)
// func (this *QGraphicsProxyWidget) InheritMouseReleaseEvent(f func(event *QGraphicsSceneMouseEvent/*777 QGraphicsSceneMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseReleaseEvent", f)
// }

// void mouseDoubleClickEvent(QGraphicsSceneMouseEvent *)
// func (this *QGraphicsProxyWidget) InheritMouseDoubleClickEvent(f func(event *QGraphicsSceneMouseEvent/*777 QGraphicsSceneMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseDoubleClickEvent", f)
// }

// void wheelEvent(QGraphicsSceneWheelEvent *)
// func (this *QGraphicsProxyWidget) InheritWheelEvent(f func(event *QGraphicsSceneWheelEvent/*777 QGraphicsSceneWheelEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "wheelEvent", f)
// }

// void keyPressEvent(QKeyEvent *)
// func (this *QGraphicsProxyWidget) InheritKeyPressEvent(f func(event *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyPressEvent", f)
// }

// void keyReleaseEvent(QKeyEvent *)
// func (this *QGraphicsProxyWidget) InheritKeyReleaseEvent(f func(event *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyReleaseEvent", f)
// }

// void focusInEvent(QFocusEvent *)
// func (this *QGraphicsProxyWidget) InheritFocusInEvent(f func(event *qtgui.QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusInEvent", f)
// }

// void focusOutEvent(QFocusEvent *)
// func (this *QGraphicsProxyWidget) InheritFocusOutEvent(f func(event *qtgui.QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusOutEvent", f)
// }

// bool focusNextPrevChild(bool)
// func (this *QGraphicsProxyWidget) InheritFocusNextPrevChild(f func(next bool) bool) {
//  qtrt.SetAllInheritCallback(this, "focusNextPrevChild", f)
// }

// QVariant inputMethodQuery(Qt::InputMethodQuery)
// func (this *QGraphicsProxyWidget) InheritInputMethodQuery(f func(query int) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "inputMethodQuery", f)
// }

// void inputMethodEvent(QInputMethodEvent *)
// func (this *QGraphicsProxyWidget) InheritInputMethodEvent(f func(event *qtgui.QInputMethodEvent/*777 QInputMethodEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "inputMethodEvent", f)
// }

// QSizeF sizeHint(Qt::SizeHint, const QSizeF &)
// func (this *QGraphicsProxyWidget) InheritSizeHint(f func(which int, constraint *qtcore.QSizeF) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "sizeHint", f)
// }

// void resizeEvent(QGraphicsSceneResizeEvent *)
// func (this *QGraphicsProxyWidget) InheritResizeEvent(f func(event *QGraphicsSceneResizeEvent/*777 QGraphicsSceneResizeEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "resizeEvent", f)
// }

// QGraphicsProxyWidget * newProxyWidget(const QWidget *)
// func (this *QGraphicsProxyWidget) InheritNewProxyWidget(f func(arg0 *QWidget/*777 const QWidget **/) unsafe.Pointer/*666*/) {
//  qtrt.SetAllInheritCallback(this, "newProxyWidget", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QGraphicsProxyWidget)=48
pub struct QGraphicsProxyWidget {
  qbase: QGraphicsWidget,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGraphicsProxyWidget_ITF interface {
//    QGraphicsWidget_ITF
//    QGraphicsProxyWidget_PTR() *QGraphicsProxyWidget
//}
//func (ptr *QGraphicsProxyWidget) QGraphicsProxyWidget_PTR() *QGraphicsProxyWidget { return ptr }

impl /*struct*/ QGraphicsProxyWidget {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGraphicsProxyWidget {
    return QGraphicsProxyWidget{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGraphicsProxyWidget {
//  type Target = QGraphicsProxyWidgetBASE;
//
//  fn deref(&self) -> &QGraphicsProxyWidgetBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGraphicsProxyWidgetBASE> for QGraphicsProxyWidget {
//  fn as_ref(& self) -> & QGraphicsProxyWidgetBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h:54
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QGraphicsProxyWidget {
  pub fn metaObject_0<RetType, T: QGraphicsProxyWidget_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QGraphicsProxyWidget_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsProxyWidget_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QGraphicsProxyWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QGraphicsProxyWidget10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h:56
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QGraphicsProxyWidget(QGraphicsItem *, Qt::WindowFlags)

/*
Constructs a new QGraphicsProxy widget. parent and wFlags are passed to QGraphicsItem's constructor.
*/
// QGraphicsProxyWidget(QGraphicsItem *, Qt::WindowFlags) ctx.fn_proto_cpp
impl /*struct*/ QGraphicsProxyWidget {
  pub fn QGraphicsProxyWidget_0<T: QGraphicsProxyWidget_QGraphicsProxyWidget_0>(value: T) -> QGraphicsProxyWidget {
    let rsthis = value.QGraphicsProxyWidget_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsProxyWidget_QGraphicsProxyWidget_0 {
  fn QGraphicsProxyWidget_0(self) -> QGraphicsProxyWidget;
}
// QGraphicsProxyWidget(QGraphicsItem *, Qt::WindowFlags) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGraphicsProxyWidget_QGraphicsProxyWidget_0 for (usize,i32) {
  fn QGraphicsProxyWidget_0(self) -> QGraphicsProxyWidget {
    // unsafe{_ZN20QGraphicsProxyWidgetC2EP13QGraphicsItem6QFlagsIN2Qt10WindowTypeEE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN20QGraphicsProxyWidgetC2EP13QGraphicsItem6QFlagsIN2Qt10WindowTypeEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGraphicsProxyWidget{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h:57
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QGraphicsProxyWidget()

/*

*/
pub fn DeleteQGraphicsProxyWidget(this :*mut QGraphicsProxyWidget) {
    // let rv = qtrt::InvokeQtFunc6("_ZN20QGraphicsProxyWidgetD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h:59
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWidget(QWidget *)

/*
Embeds widget into this proxy widget. The embedded widget must reside exclusively either inside or outside of Graphics View. You cannot embed a widget as long as it is is visible elsewhere in the UI, at the same time.

widget must be a top-level widget whose parent is 0.

When the widget is embedded, its state (e.g., visible, enabled, geometry, size hints) is copied into the proxy widget. If the embedded widget is explicitly hidden or disabled, the proxy widget will become explicitly hidden or disabled after embedding is complete. The class documentation has a full overview over the shared state.

QGraphicsProxyWidget's window flags determine whether the widget, after embedding, will be given window decorations or not.

After this function returns, QGraphicsProxyWidget will keep its state synchronized with that of widget whenever possible.

If a widget is already embedded by this proxy when this function is called, that widget will first be automatically unembedded. Passing 0 for the widget argument will only unembed the widget, and the ownership of the currently embedded widget will be passed on to the caller. Every child widget that are embedded will also be embedded and their proxy widget destroyed.

Note that widgets with the Qt::WA_PaintOnScreen widget attribute set and widgets that wrap an external application or controller cannot be embedded. Examples are QGLWidget and QAxWidget.

See also widget().
*/
impl /*struct*/ QGraphicsProxyWidget {
  pub fn setWidget_0<RetType, T: QGraphicsProxyWidget_setWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWidget_0(self);
    // return 1;
  }
}
pub trait QGraphicsProxyWidget_setWidget_0<RetType> {
  fn setWidget_0(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsProxyWidget_setWidget_0<(/*void*/)> for (usize) {
  fn setWidget_0(self , rsthis: & QGraphicsProxyWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN20QGraphicsProxyWidget9setWidgetEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h:60
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * widget() const

/*
Returns a pointer to the embedded widget.

See also setWidget().
*/
impl /*struct*/ QGraphicsProxyWidget {
  pub fn widget_0<RetType, T: QGraphicsProxyWidget_widget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.widget_0(self);
    // return 1;
  }
}
pub trait QGraphicsProxyWidget_widget_0<RetType> {
  fn widget_0(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsProxyWidget_widget_0<usize> for () {
  fn widget_0(self , rsthis: & QGraphicsProxyWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QGraphicsProxyWidget6widgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h:62
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF subWidgetRect(const QWidget *) const

/*
Returns the rectangle for widget, which must be a descendant of widget(), or widget() itself, in this proxy item's local coordinates.

If no widget is embedded, widget is 0, or widget is not a descendant of the embedded widget, this function returns an empty QRectF.

See also widget().
*/
impl /*struct*/ QGraphicsProxyWidget {
  pub fn subWidgetRect_0<RetType, T: QGraphicsProxyWidget_subWidgetRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.subWidgetRect_0(self);
    // return 1;
  }
}
pub trait QGraphicsProxyWidget_subWidgetRect_0<RetType> {
  fn subWidgetRect_0(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsProxyWidget_subWidgetRect_0<usize> for (usize) {
  fn subWidgetRect_0(self , rsthis: & QGraphicsProxyWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QGraphicsProxyWidget13subWidgetRectEPK7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h:64
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setGeometry(const QRectF &)

/*
Reimplemented from QGraphicsLayoutItem::setGeometry().
*/
impl /*struct*/ QGraphicsProxyWidget {
  pub fn setGeometry_0<RetType, T: QGraphicsProxyWidget_setGeometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setGeometry_0(self);
    // return 1;
  }
}
pub trait QGraphicsProxyWidget_setGeometry_0<RetType> {
  fn setGeometry_0(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsProxyWidget_setGeometry_0<(/*void*/)> for (usize) {
  fn setGeometry_0(self , rsthis: & QGraphicsProxyWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN20QGraphicsProxyWidget11setGeometryERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h:66
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void paint(QPainter *, const QStyleOptionGraphicsItem *, QWidget *)

/*
Reimplemented from QGraphicsItem::paint().
*/
impl /*struct*/ QGraphicsProxyWidget {
  pub fn paint_0<RetType, T: QGraphicsProxyWidget_paint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paint_0(self);
    // return 1;
  }
}
pub trait QGraphicsProxyWidget_paint_0<RetType> {
  fn paint_0(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsProxyWidget_paint_0<(/*void*/)> for (usize,usize,usize) {
  fn paint_0(self , rsthis: & QGraphicsProxyWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN20QGraphicsProxyWidget5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h:71
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int type() const

/*
Reimplemented from QGraphicsItem::type().
*/
impl /*struct*/ QGraphicsProxyWidget {
  pub fn type__0<RetType, T: QGraphicsProxyWidget_type__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.type__0(self);
    // return 1;
  }
}
pub trait QGraphicsProxyWidget_type__0<RetType> {
  fn type__0(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsProxyWidget_type__0<i32> for () {
  fn type__0(self , rsthis: & QGraphicsProxyWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QGraphicsProxyWidget4typeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h:73
// index:0
// Public Visibility=Default Availability=Available
// [8] QGraphicsProxyWidget * createProxyForChildWidget(QWidget *)

/*
Creates a proxy widget for the given child of the widget contained in this proxy.

This function makes it possible to acquire proxies for non top-level widgets. For instance, you can embed a dialog, and then transform only one of its widgets.

If the widget is already embedded, return the existing proxy widget.

This function was introduced in  Qt 4.5.

See also newProxyWidget() and QGraphicsScene::addWidget().
*/
impl /*struct*/ QGraphicsProxyWidget {
  pub fn createProxyForChildWidget_0<RetType, T: QGraphicsProxyWidget_createProxyForChildWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.createProxyForChildWidget_0(self);
    // return 1;
  }
}
pub trait QGraphicsProxyWidget_createProxyForChildWidget_0<RetType> {
  fn createProxyForChildWidget_0(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsProxyWidget_createProxyForChildWidget_0<usize> for (usize) {
  fn createProxyForChildWidget_0(self , rsthis: & QGraphicsProxyWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN20QGraphicsProxyWidget25createProxyForChildWidgetEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h:76
// index:0
// Protected virtual Visibility=Default Availability=Available
// [16] QVariant itemChange(QGraphicsItem::GraphicsItemChange, const QVariant &)

/*
Reimplemented from QGraphicsItem::itemChange().
*/
impl /*struct*/ QGraphicsProxyWidget {
  pub fn itemChange_0<RetType, T: QGraphicsProxyWidget_itemChange_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemChange_0(self);
    // return 1;
  }
}
pub trait QGraphicsProxyWidget_itemChange_0<RetType> {
  fn itemChange_0(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsProxyWidget_itemChange_0<usize> for (i32,usize) {
  fn itemChange_0(self , rsthis: & QGraphicsProxyWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN20QGraphicsProxyWidget10itemChangeEN13QGraphicsItem18GraphicsItemChangeERK8QVariant", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h:78
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QGraphicsProxyWidget {
  pub fn event_0<RetType, T: QGraphicsProxyWidget_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QGraphicsProxyWidget_event_0<RetType> {
  fn event_0(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsProxyWidget_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QGraphicsProxyWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN20QGraphicsProxyWidget5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h:79
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool eventFilter(QObject *, QEvent *)

/*
Reimplemented from QObject::eventFilter().
*/
impl /*struct*/ QGraphicsProxyWidget {
  pub fn eventFilter_0<RetType, T: QGraphicsProxyWidget_eventFilter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.eventFilter_0(self);
    // return 1;
  }
}
pub trait QGraphicsProxyWidget_eventFilter_0<RetType> {
  fn eventFilter_0(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsProxyWidget_eventFilter_0<bool> for (usize,usize) {
  fn eventFilter_0(self , rsthis: & QGraphicsProxyWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN20QGraphicsProxyWidget11eventFilterEP7QObjectP6QEvent", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h:81
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void showEvent(QShowEvent *)

/*
Reimplemented from QGraphicsWidget::showEvent().
*/
impl /*struct*/ QGraphicsProxyWidget {
  pub fn showEvent_0<RetType, T: QGraphicsProxyWidget_showEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsProxyWidget_showEvent_0<RetType> {
  fn showEvent_0(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsProxyWidget_showEvent_0<(/*void*/)> for (usize) {
  fn showEvent_0(self , rsthis: & QGraphicsProxyWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN20QGraphicsProxyWidget9showEventEP10QShowEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h:82
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void hideEvent(QHideEvent *)

/*
Reimplemented from QGraphicsWidget::hideEvent().
*/
impl /*struct*/ QGraphicsProxyWidget {
  pub fn hideEvent_0<RetType, T: QGraphicsProxyWidget_hideEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hideEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsProxyWidget_hideEvent_0<RetType> {
  fn hideEvent_0(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsProxyWidget_hideEvent_0<(/*void*/)> for (usize) {
  fn hideEvent_0(self , rsthis: & QGraphicsProxyWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN20QGraphicsProxyWidget9hideEventEP10QHideEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h:85
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void contextMenuEvent(QGraphicsSceneContextMenuEvent *)

/*
Reimplemented from QGraphicsItem::contextMenuEvent().
*/
impl /*struct*/ QGraphicsProxyWidget {
  pub fn contextMenuEvent_0<RetType, T: QGraphicsProxyWidget_contextMenuEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contextMenuEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsProxyWidget_contextMenuEvent_0<RetType> {
  fn contextMenuEvent_0(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsProxyWidget_contextMenuEvent_0<(/*void*/)> for (usize) {
  fn contextMenuEvent_0(self , rsthis: & QGraphicsProxyWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN20QGraphicsProxyWidget16contextMenuEventEP30QGraphicsSceneContextMenuEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h:89
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dragEnterEvent(QGraphicsSceneDragDropEvent *)

/*
Reimplemented from QGraphicsItem::dragEnterEvent().
*/
impl /*struct*/ QGraphicsProxyWidget {
  pub fn dragEnterEvent_0<RetType, T: QGraphicsProxyWidget_dragEnterEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragEnterEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsProxyWidget_dragEnterEvent_0<RetType> {
  fn dragEnterEvent_0(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsProxyWidget_dragEnterEvent_0<(/*void*/)> for (usize) {
  fn dragEnterEvent_0(self , rsthis: & QGraphicsProxyWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN20QGraphicsProxyWidget14dragEnterEventEP27QGraphicsSceneDragDropEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h:90
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dragLeaveEvent(QGraphicsSceneDragDropEvent *)

/*
Reimplemented from QGraphicsItem::dragLeaveEvent().
*/
impl /*struct*/ QGraphicsProxyWidget {
  pub fn dragLeaveEvent_0<RetType, T: QGraphicsProxyWidget_dragLeaveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragLeaveEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsProxyWidget_dragLeaveEvent_0<RetType> {
  fn dragLeaveEvent_0(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsProxyWidget_dragLeaveEvent_0<(/*void*/)> for (usize) {
  fn dragLeaveEvent_0(self , rsthis: & QGraphicsProxyWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN20QGraphicsProxyWidget14dragLeaveEventEP27QGraphicsSceneDragDropEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h:91
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dragMoveEvent(QGraphicsSceneDragDropEvent *)

/*
Reimplemented from QGraphicsItem::dragMoveEvent().
*/
impl /*struct*/ QGraphicsProxyWidget {
  pub fn dragMoveEvent_0<RetType, T: QGraphicsProxyWidget_dragMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragMoveEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsProxyWidget_dragMoveEvent_0<RetType> {
  fn dragMoveEvent_0(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsProxyWidget_dragMoveEvent_0<(/*void*/)> for (usize) {
  fn dragMoveEvent_0(self , rsthis: & QGraphicsProxyWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN20QGraphicsProxyWidget13dragMoveEventEP27QGraphicsSceneDragDropEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h:92
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dropEvent(QGraphicsSceneDragDropEvent *)

/*
Reimplemented from QGraphicsItem::dropEvent().
*/
impl /*struct*/ QGraphicsProxyWidget {
  pub fn dropEvent_0<RetType, T: QGraphicsProxyWidget_dropEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dropEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsProxyWidget_dropEvent_0<RetType> {
  fn dropEvent_0(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsProxyWidget_dropEvent_0<(/*void*/)> for (usize) {
  fn dropEvent_0(self , rsthis: & QGraphicsProxyWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN20QGraphicsProxyWidget9dropEventEP27QGraphicsSceneDragDropEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h:95
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void hoverEnterEvent(QGraphicsSceneHoverEvent *)

/*
Reimplemented from QGraphicsItem::hoverEnterEvent().
*/
impl /*struct*/ QGraphicsProxyWidget {
  pub fn hoverEnterEvent_0<RetType, T: QGraphicsProxyWidget_hoverEnterEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hoverEnterEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsProxyWidget_hoverEnterEvent_0<RetType> {
  fn hoverEnterEvent_0(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsProxyWidget_hoverEnterEvent_0<(/*void*/)> for (usize) {
  fn hoverEnterEvent_0(self , rsthis: & QGraphicsProxyWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN20QGraphicsProxyWidget15hoverEnterEventEP24QGraphicsSceneHoverEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h:96
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void hoverLeaveEvent(QGraphicsSceneHoverEvent *)

/*
Reimplemented from QGraphicsItem::hoverLeaveEvent().
*/
impl /*struct*/ QGraphicsProxyWidget {
  pub fn hoverLeaveEvent_0<RetType, T: QGraphicsProxyWidget_hoverLeaveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hoverLeaveEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsProxyWidget_hoverLeaveEvent_0<RetType> {
  fn hoverLeaveEvent_0(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsProxyWidget_hoverLeaveEvent_0<(/*void*/)> for (usize) {
  fn hoverLeaveEvent_0(self , rsthis: & QGraphicsProxyWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN20QGraphicsProxyWidget15hoverLeaveEventEP24QGraphicsSceneHoverEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h:97
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void hoverMoveEvent(QGraphicsSceneHoverEvent *)

/*
Reimplemented from QGraphicsItem::hoverMoveEvent().
*/
impl /*struct*/ QGraphicsProxyWidget {
  pub fn hoverMoveEvent_0<RetType, T: QGraphicsProxyWidget_hoverMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hoverMoveEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsProxyWidget_hoverMoveEvent_0<RetType> {
  fn hoverMoveEvent_0(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsProxyWidget_hoverMoveEvent_0<(/*void*/)> for (usize) {
  fn hoverMoveEvent_0(self , rsthis: & QGraphicsProxyWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN20QGraphicsProxyWidget14hoverMoveEventEP24QGraphicsSceneHoverEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h:98
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void grabMouseEvent(QEvent *)

/*
Reimplemented from QGraphicsWidget::grabMouseEvent().
*/
impl /*struct*/ QGraphicsProxyWidget {
  pub fn grabMouseEvent_0<RetType, T: QGraphicsProxyWidget_grabMouseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.grabMouseEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsProxyWidget_grabMouseEvent_0<RetType> {
  fn grabMouseEvent_0(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsProxyWidget_grabMouseEvent_0<(/*void*/)> for (usize) {
  fn grabMouseEvent_0(self , rsthis: & QGraphicsProxyWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN20QGraphicsProxyWidget14grabMouseEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h:99
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void ungrabMouseEvent(QEvent *)

/*
Reimplemented from QGraphicsWidget::ungrabMouseEvent().
*/
impl /*struct*/ QGraphicsProxyWidget {
  pub fn ungrabMouseEvent_0<RetType, T: QGraphicsProxyWidget_ungrabMouseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ungrabMouseEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsProxyWidget_ungrabMouseEvent_0<RetType> {
  fn ungrabMouseEvent_0(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsProxyWidget_ungrabMouseEvent_0<(/*void*/)> for (usize) {
  fn ungrabMouseEvent_0(self , rsthis: & QGraphicsProxyWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN20QGraphicsProxyWidget16ungrabMouseEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h:101
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseMoveEvent(QGraphicsSceneMouseEvent *)

/*
Reimplemented from QGraphicsItem::mouseMoveEvent().
*/
impl /*struct*/ QGraphicsProxyWidget {
  pub fn mouseMoveEvent_0<RetType, T: QGraphicsProxyWidget_mouseMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseMoveEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsProxyWidget_mouseMoveEvent_0<RetType> {
  fn mouseMoveEvent_0(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsProxyWidget_mouseMoveEvent_0<(/*void*/)> for (usize) {
  fn mouseMoveEvent_0(self , rsthis: & QGraphicsProxyWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN20QGraphicsProxyWidget14mouseMoveEventEP24QGraphicsSceneMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h:102
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mousePressEvent(QGraphicsSceneMouseEvent *)

/*
Reimplemented from QGraphicsItem::mousePressEvent().
*/
impl /*struct*/ QGraphicsProxyWidget {
  pub fn mousePressEvent_0<RetType, T: QGraphicsProxyWidget_mousePressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mousePressEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsProxyWidget_mousePressEvent_0<RetType> {
  fn mousePressEvent_0(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsProxyWidget_mousePressEvent_0<(/*void*/)> for (usize) {
  fn mousePressEvent_0(self , rsthis: & QGraphicsProxyWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN20QGraphicsProxyWidget15mousePressEventEP24QGraphicsSceneMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h:103
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseReleaseEvent(QGraphicsSceneMouseEvent *)

/*
Reimplemented from QGraphicsItem::mouseReleaseEvent().
*/
impl /*struct*/ QGraphicsProxyWidget {
  pub fn mouseReleaseEvent_0<RetType, T: QGraphicsProxyWidget_mouseReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsProxyWidget_mouseReleaseEvent_0<RetType> {
  fn mouseReleaseEvent_0(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsProxyWidget_mouseReleaseEvent_0<(/*void*/)> for (usize) {
  fn mouseReleaseEvent_0(self , rsthis: & QGraphicsProxyWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN20QGraphicsProxyWidget17mouseReleaseEventEP24QGraphicsSceneMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h:104
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseDoubleClickEvent(QGraphicsSceneMouseEvent *)

/*
Reimplemented from QGraphicsItem::mouseDoubleClickEvent().
*/
impl /*struct*/ QGraphicsProxyWidget {
  pub fn mouseDoubleClickEvent_0<RetType, T: QGraphicsProxyWidget_mouseDoubleClickEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseDoubleClickEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsProxyWidget_mouseDoubleClickEvent_0<RetType> {
  fn mouseDoubleClickEvent_0(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsProxyWidget_mouseDoubleClickEvent_0<(/*void*/)> for (usize) {
  fn mouseDoubleClickEvent_0(self , rsthis: & QGraphicsProxyWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN20QGraphicsProxyWidget21mouseDoubleClickEventEP24QGraphicsSceneMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h:106
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void wheelEvent(QGraphicsSceneWheelEvent *)

/*
Reimplemented from QGraphicsItem::wheelEvent().
*/
impl /*struct*/ QGraphicsProxyWidget {
  pub fn wheelEvent_0<RetType, T: QGraphicsProxyWidget_wheelEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.wheelEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsProxyWidget_wheelEvent_0<RetType> {
  fn wheelEvent_0(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsProxyWidget_wheelEvent_0<(/*void*/)> for (usize) {
  fn wheelEvent_0(self , rsthis: & QGraphicsProxyWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN20QGraphicsProxyWidget10wheelEventEP24QGraphicsSceneWheelEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h:109
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyPressEvent(QKeyEvent *)

/*
Reimplemented from QGraphicsItem::keyPressEvent().
*/
impl /*struct*/ QGraphicsProxyWidget {
  pub fn keyPressEvent_0<RetType, T: QGraphicsProxyWidget_keyPressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyPressEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsProxyWidget_keyPressEvent_0<RetType> {
  fn keyPressEvent_0(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsProxyWidget_keyPressEvent_0<(/*void*/)> for (usize) {
  fn keyPressEvent_0(self , rsthis: & QGraphicsProxyWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN20QGraphicsProxyWidget13keyPressEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h:110
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyReleaseEvent(QKeyEvent *)

/*
Reimplemented from QGraphicsItem::keyReleaseEvent().
*/
impl /*struct*/ QGraphicsProxyWidget {
  pub fn keyReleaseEvent_0<RetType, T: QGraphicsProxyWidget_keyReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsProxyWidget_keyReleaseEvent_0<RetType> {
  fn keyReleaseEvent_0(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsProxyWidget_keyReleaseEvent_0<(/*void*/)> for (usize) {
  fn keyReleaseEvent_0(self , rsthis: & QGraphicsProxyWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN20QGraphicsProxyWidget15keyReleaseEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h:112
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusInEvent(QFocusEvent *)

/*
Reimplemented from QGraphicsItem::focusInEvent().
*/
impl /*struct*/ QGraphicsProxyWidget {
  pub fn focusInEvent_0<RetType, T: QGraphicsProxyWidget_focusInEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusInEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsProxyWidget_focusInEvent_0<RetType> {
  fn focusInEvent_0(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsProxyWidget_focusInEvent_0<(/*void*/)> for (usize) {
  fn focusInEvent_0(self , rsthis: & QGraphicsProxyWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN20QGraphicsProxyWidget12focusInEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h:113
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusOutEvent(QFocusEvent *)

/*
Reimplemented from QGraphicsItem::focusOutEvent().
*/
impl /*struct*/ QGraphicsProxyWidget {
  pub fn focusOutEvent_0<RetType, T: QGraphicsProxyWidget_focusOutEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusOutEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsProxyWidget_focusOutEvent_0<RetType> {
  fn focusOutEvent_0(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsProxyWidget_focusOutEvent_0<(/*void*/)> for (usize) {
  fn focusOutEvent_0(self , rsthis: & QGraphicsProxyWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN20QGraphicsProxyWidget13focusOutEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h:114
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool focusNextPrevChild(bool)

/*
Reimplemented from QGraphicsWidget::focusNextPrevChild().
*/
impl /*struct*/ QGraphicsProxyWidget {
  pub fn focusNextPrevChild_0<RetType, T: QGraphicsProxyWidget_focusNextPrevChild_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusNextPrevChild_0(self);
    // return 1;
  }
}
pub trait QGraphicsProxyWidget_focusNextPrevChild_0<RetType> {
  fn focusNextPrevChild_0(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsProxyWidget_focusNextPrevChild_0<bool> for (bool) {
  fn focusNextPrevChild_0(self , rsthis: & QGraphicsProxyWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN20QGraphicsProxyWidget18focusNextPrevChildEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h:116
// index:0
// Protected virtual Visibility=Default Availability=Available
// [16] QVariant inputMethodQuery(Qt::InputMethodQuery) const

/*
Reimplemented from QGraphicsItem::inputMethodQuery().
*/
impl /*struct*/ QGraphicsProxyWidget {
  pub fn inputMethodQuery_0<RetType, T: QGraphicsProxyWidget_inputMethodQuery_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inputMethodQuery_0(self);
    // return 1;
  }
}
pub trait QGraphicsProxyWidget_inputMethodQuery_0<RetType> {
  fn inputMethodQuery_0(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsProxyWidget_inputMethodQuery_0<usize> for (i32) {
  fn inputMethodQuery_0(self , rsthis: & QGraphicsProxyWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QGraphicsProxyWidget16inputMethodQueryEN2Qt16InputMethodQueryE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h:117
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void inputMethodEvent(QInputMethodEvent *)

/*
Reimplemented from QGraphicsItem::inputMethodEvent().
*/
impl /*struct*/ QGraphicsProxyWidget {
  pub fn inputMethodEvent_0<RetType, T: QGraphicsProxyWidget_inputMethodEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inputMethodEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsProxyWidget_inputMethodEvent_0<RetType> {
  fn inputMethodEvent_0(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsProxyWidget_inputMethodEvent_0<(/*void*/)> for (usize) {
  fn inputMethodEvent_0(self , rsthis: & QGraphicsProxyWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN20QGraphicsProxyWidget16inputMethodEventEP17QInputMethodEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h:119
// index:0
// Protected virtual Visibility=Default Availability=Available
// [16] QSizeF sizeHint(Qt::SizeHint, const QSizeF &) const

/*
Reimplemented from QGraphicsLayoutItem::sizeHint().
*/
impl /*struct*/ QGraphicsProxyWidget {
  pub fn sizeHint_0<RetType, T: QGraphicsProxyWidget_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QGraphicsProxyWidget_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsProxyWidget_sizeHint_0<usize> for (i32,usize) {
  fn sizeHint_0(self , rsthis: & QGraphicsProxyWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QGraphicsProxyWidget8sizeHintEN2Qt8SizeHintERK6QSizeF", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h:120
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void resizeEvent(QGraphicsSceneResizeEvent *)

/*
Reimplemented from QGraphicsWidget::resizeEvent().
*/
impl /*struct*/ QGraphicsProxyWidget {
  pub fn resizeEvent_0<RetType, T: QGraphicsProxyWidget_resizeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsProxyWidget_resizeEvent_0<RetType> {
  fn resizeEvent_0(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsProxyWidget_resizeEvent_0<(/*void*/)> for (usize) {
  fn resizeEvent_0(self , rsthis: & QGraphicsProxyWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN20QGraphicsProxyWidget11resizeEventEP25QGraphicsSceneResizeEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsproxywidget.h:123
// index:0
// Protected Visibility=Default Availability=Available
// [8] QGraphicsProxyWidget * newProxyWidget(const QWidget *)

/*
Creates a proxy widget for the given child of the widget contained in this proxy.

You should not call this function directly; use QGraphicsProxyWidget::createProxyForChildWidget() instead.

This function is a fake virtual slot that you can reimplement in your subclass in order to control how new proxy widgets are created. The default implementation returns a proxy created with the QGraphicsProxyWidget() constructor with this proxy widget as the parent.

This function was introduced in  Qt 4.5.

See also createProxyForChildWidget().
*/
impl /*struct*/ QGraphicsProxyWidget {
  pub fn newProxyWidget_0<RetType, T: QGraphicsProxyWidget_newProxyWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.newProxyWidget_0(self);
    // return 1;
  }
}
pub trait QGraphicsProxyWidget_newProxyWidget_0<RetType> {
  fn newProxyWidget_0(self , rsthis: & QGraphicsProxyWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsProxyWidget_newProxyWidget_0<usize> for (usize) {
  fn newProxyWidget_0(self , rsthis: & QGraphicsProxyWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN20QGraphicsProxyWidget14newProxyWidgetEPK7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


/*


*/
pub type QGraphicsProxyWidget__ = i32;
// 
pub const QGraphicsProxyWidget__Type :QGraphicsProxyWidget__ = 12;
pub fn QGraphicsProxyWidget_ItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QGraphicsProxyWidget", val);
}
pub fn QGraphicsProxyWidget_ItemName_s(val: i32) ->String {
  //var nilthis *QGraphicsProxyWidget
  //return nilthis.ItemName(val);
  return QGraphicsProxyWidget_ItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
