

// mod ::widgets::QGraphicsWidget
// package qtwidgets
// /usr/include/qt/QtWidgets/qgraphicswidget.h
// #include <qgraphicswidget.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 25
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

// void initStyleOption(QStyleOption *)
// func (this *QGraphicsWidget) InheritInitStyleOption(f func(option *QStyleOption/*777 QStyleOption **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "initStyleOption", f)
// }

// QSizeF sizeHint(Qt::SizeHint, const QSizeF &)
// func (this *QGraphicsWidget) InheritSizeHint(f func(which int, constraint *qtcore.QSizeF) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "sizeHint", f)
// }

// void updateGeometry()
// func (this *QGraphicsWidget) InheritUpdateGeometry(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "updateGeometry", f)
// }

// QVariant itemChange(QGraphicsItem::GraphicsItemChange, const QVariant &)
// func (this *QGraphicsWidget) InheritItemChange(f func(change int, value *qtcore.QVariant) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "itemChange", f)
// }

// QVariant propertyChange(const QString &, const QVariant &)
// func (this *QGraphicsWidget) InheritPropertyChange(f func(propertyName string, value *qtcore.QVariant) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "propertyChange", f)
// }

// bool sceneEvent(QEvent *)
// func (this *QGraphicsWidget) InheritSceneEvent(f func(event *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "sceneEvent", f)
// }

// bool windowFrameEvent(QEvent *)
// func (this *QGraphicsWidget) InheritWindowFrameEvent(f func(e *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "windowFrameEvent", f)
// }

// Qt::WindowFrameSection windowFrameSectionAt(const QPointF &)
// func (this *QGraphicsWidget) InheritWindowFrameSectionAt(f func(pos *qtcore.QPointF) int) {
//  qtrt.SetAllInheritCallback(this, "windowFrameSectionAt", f)
// }

// bool event(QEvent *)
// func (this *QGraphicsWidget) InheritEvent(f func(event *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// void changeEvent(QEvent *)
// func (this *QGraphicsWidget) InheritChangeEvent(f func(event *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "changeEvent", f)
// }

// void closeEvent(QCloseEvent *)
// func (this *QGraphicsWidget) InheritCloseEvent(f func(event *qtgui.QCloseEvent/*777 QCloseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "closeEvent", f)
// }

// void focusInEvent(QFocusEvent *)
// func (this *QGraphicsWidget) InheritFocusInEvent(f func(event *qtgui.QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusInEvent", f)
// }

// bool focusNextPrevChild(bool)
// func (this *QGraphicsWidget) InheritFocusNextPrevChild(f func(next bool) bool) {
//  qtrt.SetAllInheritCallback(this, "focusNextPrevChild", f)
// }

// void focusOutEvent(QFocusEvent *)
// func (this *QGraphicsWidget) InheritFocusOutEvent(f func(event *qtgui.QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusOutEvent", f)
// }

// void hideEvent(QHideEvent *)
// func (this *QGraphicsWidget) InheritHideEvent(f func(event *qtgui.QHideEvent/*777 QHideEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "hideEvent", f)
// }

// void moveEvent(QGraphicsSceneMoveEvent *)
// func (this *QGraphicsWidget) InheritMoveEvent(f func(event *QGraphicsSceneMoveEvent/*777 QGraphicsSceneMoveEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "moveEvent", f)
// }

// void polishEvent()
// func (this *QGraphicsWidget) InheritPolishEvent(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "polishEvent", f)
// }

// void resizeEvent(QGraphicsSceneResizeEvent *)
// func (this *QGraphicsWidget) InheritResizeEvent(f func(event *QGraphicsSceneResizeEvent/*777 QGraphicsSceneResizeEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "resizeEvent", f)
// }

// void showEvent(QShowEvent *)
// func (this *QGraphicsWidget) InheritShowEvent(f func(event *qtgui.QShowEvent/*777 QShowEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "showEvent", f)
// }

// void hoverMoveEvent(QGraphicsSceneHoverEvent *)
// func (this *QGraphicsWidget) InheritHoverMoveEvent(f func(event *QGraphicsSceneHoverEvent/*777 QGraphicsSceneHoverEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "hoverMoveEvent", f)
// }

// void hoverLeaveEvent(QGraphicsSceneHoverEvent *)
// func (this *QGraphicsWidget) InheritHoverLeaveEvent(f func(event *QGraphicsSceneHoverEvent/*777 QGraphicsSceneHoverEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "hoverLeaveEvent", f)
// }

// void grabMouseEvent(QEvent *)
// func (this *QGraphicsWidget) InheritGrabMouseEvent(f func(event *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "grabMouseEvent", f)
// }

// void ungrabMouseEvent(QEvent *)
// func (this *QGraphicsWidget) InheritUngrabMouseEvent(f func(event *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "ungrabMouseEvent", f)
// }

// void grabKeyboardEvent(QEvent *)
// func (this *QGraphicsWidget) InheritGrabKeyboardEvent(f func(event *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "grabKeyboardEvent", f)
// }

// void ungrabKeyboardEvent(QEvent *)
// func (this *QGraphicsWidget) InheritUngrabKeyboardEvent(f func(event *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "ungrabKeyboardEvent", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QGraphicsWidget)=48
pub struct QGraphicsWidget {
  qbase: QGraphicsLayoutItem,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGraphicsWidget_ITF interface {
//    QGraphicsLayoutItem_ITF
//    QGraphicsWidget_PTR() *QGraphicsWidget
//}
//func (ptr *QGraphicsWidget) QGraphicsWidget_PTR() *QGraphicsWidget { return ptr }

impl /*struct*/ QGraphicsWidget {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGraphicsWidget {
    return QGraphicsWidget{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGraphicsWidget {
//  type Target = QGraphicsWidgetBASE;
//
//  fn deref(&self) -> &QGraphicsWidgetBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGraphicsWidgetBASE> for QGraphicsWidget {
//  fn as_ref(& self) -> & QGraphicsWidgetBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgraphicswidget.h:66
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QGraphicsWidget {
  pub fn metaObject_0<RetType, T: QGraphicsWidget_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QGraphicsWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGraphicsWidget10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:83
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QGraphicsWidget(QGraphicsItem *, Qt::WindowFlags)

/*
Constructs a QGraphicsWidget instance. The optional parent argument is passed to QGraphicsItem's constructor. The optional wFlags argument specifies the widget's window flags (e.g., whether the widget should be a window, a tool, a popup, etc).
*/
// QGraphicsWidget(QGraphicsItem *, Qt::WindowFlags) ctx.fn_proto_cpp
impl /*struct*/ QGraphicsWidget {
  pub fn QGraphicsWidget_0<T: QGraphicsWidget_QGraphicsWidget_0>(value: T) -> QGraphicsWidget {
    let rsthis = value.QGraphicsWidget_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsWidget_QGraphicsWidget_0 {
  fn QGraphicsWidget_0(self) -> QGraphicsWidget;
}
// QGraphicsWidget(QGraphicsItem *, Qt::WindowFlags) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGraphicsWidget_QGraphicsWidget_0 for (usize,i32) {
  fn QGraphicsWidget_0(self) -> QGraphicsWidget {
    // unsafe{_ZN15QGraphicsWidgetC2EP13QGraphicsItem6QFlagsIN2Qt10WindowTypeEE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QGraphicsWidgetC2EP13QGraphicsItem6QFlagsIN2Qt10WindowTypeEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGraphicsWidget{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:84
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QGraphicsWidget()

/*

*/
pub fn DeleteQGraphicsWidget(this :*mut QGraphicsWidget) {
    // let rv = qtrt::InvokeQtFunc6("_ZN15QGraphicsWidgetD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgraphicswidget.h:85
// index:0
// Public Visibility=Default Availability=Available
// [8] QGraphicsLayout * layout() const

/*
Returns this widget's layout, or 0 if no layout is currently managing this widget.

Note: Getter function for property layout. 

See also setLayout().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn layout_0<RetType, T: QGraphicsWidget_layout_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.layout_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_layout_0<RetType> {
  fn layout_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_layout_0<usize> for () {
  fn layout_0(self , rsthis: & QGraphicsWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGraphicsWidget6layoutEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:86
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLayout(QGraphicsLayout *)

/*
Sets the layout for this widget to layout. Any existing layout manager is deleted before the new layout is assigned. If layout is 0, the widget is left without a layout. Existing subwidgets' geometries will remain unaffected.

All widgets that are currently managed by layout or all of its sublayouts, are automatically reparented to this item. The layout is then invalidated, and the child widget geometries are adjusted according to this item's geometry() and contentsMargins(). Children who are not explicitly managed by layout remain unaffected by the layout after it has been assigned to this widget.

QGraphicsWidget takes ownership of layout.

Note: Setter function for property layout. 

See also layout(), QGraphicsLinearLayout::addItem(), and QGraphicsLayout::invalidate().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn setLayout_0<RetType, T: QGraphicsWidget_setLayout_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLayout_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_setLayout_0<RetType> {
  fn setLayout_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_setLayout_0<(/*void*/)> for (usize) {
  fn setLayout_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget9setLayoutEP15QGraphicsLayout", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:87
// index:0
// Public Visibility=Default Availability=Available
// [-2] void adjustSize()

/*
Adjusts the size of the widget to its effective preferred size hint.

This function is called implicitly when the item is shown for the first time.

See also effectiveSizeHint() and Qt::MinimumSize.
*/
impl /*struct*/ QGraphicsWidget {
  pub fn adjustSize_0<RetType, T: QGraphicsWidget_adjustSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.adjustSize_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_adjustSize_0<RetType> {
  fn adjustSize_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_adjustSize_0<(/*void*/)> for () {
  fn adjustSize_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget10adjustSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:89
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::LayoutDirection layoutDirection() const

/*

*/
impl /*struct*/ QGraphicsWidget {
  pub fn layoutDirection_0<RetType, T: QGraphicsWidget_layoutDirection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.layoutDirection_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_layoutDirection_0<RetType> {
  fn layoutDirection_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_layoutDirection_0<i32> for () {
  fn layoutDirection_0(self , rsthis: & QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGraphicsWidget15layoutDirectionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:90
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLayoutDirection(Qt::LayoutDirection)

/*

*/
impl /*struct*/ QGraphicsWidget {
  pub fn setLayoutDirection_0<RetType, T: QGraphicsWidget_setLayoutDirection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLayoutDirection_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_setLayoutDirection_0<RetType> {
  fn setLayoutDirection_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_setLayoutDirection_0<(/*void*/)> for (i32) {
  fn setLayoutDirection_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget18setLayoutDirectionEN2Qt15LayoutDirectionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:91
// index:0
// Public Visibility=Default Availability=Available
// [-2] void unsetLayoutDirection()

/*

*/
impl /*struct*/ QGraphicsWidget {
  pub fn unsetLayoutDirection_0<RetType, T: QGraphicsWidget_unsetLayoutDirection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unsetLayoutDirection_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_unsetLayoutDirection_0<RetType> {
  fn unsetLayoutDirection_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_unsetLayoutDirection_0<(/*void*/)> for () {
  fn unsetLayoutDirection_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget20unsetLayoutDirectionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:93
// index:0
// Public Visibility=Default Availability=Available
// [8] QStyle * style() const

/*
Returns a pointer to the widget's style. If this widget does not have any explicitly assigned style, the scene's style is returned instead. In turn, if the scene does not have any assigned style, this function returns QApplication::style().

See also setStyle().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn style_0<RetType, T: QGraphicsWidget_style_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.style_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_style_0<RetType> {
  fn style_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_style_0<usize> for () {
  fn style_0(self , rsthis: & QGraphicsWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGraphicsWidget5styleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:94
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStyle(QStyle *)

/*
Sets the widget's style to style. QGraphicsWidget does not take ownership of style.

If no style is assigned, or style is 0, the widget will use QGraphicsScene::style() (if this has been set). Otherwise the widget will use QApplication::style().

This function sets the Qt::WA_SetStyle attribute if style is not 0; otherwise it clears the attribute.

See also style().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn setStyle_0<RetType, T: QGraphicsWidget_setStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStyle_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_setStyle_0<RetType> {
  fn setStyle_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_setStyle_0<(/*void*/)> for (usize) {
  fn setStyle_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget8setStyleEP6QStyle", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:96
// index:0
// Public Visibility=Default Availability=Available
// [16] QFont font() const

/*

*/
impl /*struct*/ QGraphicsWidget {
  pub fn font_0<RetType, T: QGraphicsWidget_font_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.font_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_font_0<RetType> {
  fn font_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_font_0<usize> for () {
  fn font_0(self , rsthis: & QGraphicsWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGraphicsWidget4fontEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:97
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFont(const QFont &)

/*

*/
impl /*struct*/ QGraphicsWidget {
  pub fn setFont_0<RetType, T: QGraphicsWidget_setFont_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFont_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_setFont_0<RetType> {
  fn setFont_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_setFont_0<(/*void*/)> for (usize) {
  fn setFont_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget7setFontERK5QFont", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:99
// index:0
// Public Visibility=Default Availability=Available
// [16] QPalette palette() const

/*

*/
impl /*struct*/ QGraphicsWidget {
  pub fn palette_0<RetType, T: QGraphicsWidget_palette_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.palette_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_palette_0<RetType> {
  fn palette_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_palette_0<usize> for () {
  fn palette_0(self , rsthis: & QGraphicsWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGraphicsWidget7paletteEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:100
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPalette(const QPalette &)

/*

*/
impl /*struct*/ QGraphicsWidget {
  pub fn setPalette_0<RetType, T: QGraphicsWidget_setPalette_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPalette_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_setPalette_0<RetType> {
  fn setPalette_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_setPalette_0<(/*void*/)> for (usize) {
  fn setPalette_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget10setPaletteERK8QPalette", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:102
// index:0
// Public Visibility=Default Availability=Available
// [1] bool autoFillBackground() const

/*

*/
impl /*struct*/ QGraphicsWidget {
  pub fn autoFillBackground_0<RetType, T: QGraphicsWidget_autoFillBackground_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.autoFillBackground_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_autoFillBackground_0<RetType> {
  fn autoFillBackground_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_autoFillBackground_0<bool> for () {
  fn autoFillBackground_0(self , rsthis: & QGraphicsWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGraphicsWidget18autoFillBackgroundEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:103
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAutoFillBackground(bool)

/*

*/
impl /*struct*/ QGraphicsWidget {
  pub fn setAutoFillBackground_0<RetType, T: QGraphicsWidget_setAutoFillBackground_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAutoFillBackground_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_setAutoFillBackground_0<RetType> {
  fn setAutoFillBackground_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_setAutoFillBackground_0<(/*void*/)> for (bool) {
  fn setAutoFillBackground_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget21setAutoFillBackgroundEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:105
// index:0
// Public Visibility=Default Availability=Available
// [-2] void resize(const QSizeF &)

/*

*/
impl /*struct*/ QGraphicsWidget {
  pub fn resize_0<RetType, T: QGraphicsWidget_resize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resize_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_resize_0<RetType> {
  fn resize_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_resize_0<(/*void*/)> for (usize) {
  fn resize_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget6resizeERK6QSizeF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:106
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void resize(qreal, qreal)

/*

*/
impl /*struct*/ QGraphicsWidget {
  pub fn resize_1<RetType, T: QGraphicsWidget_resize_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resize_1(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_resize_1<RetType> {
  fn resize_1(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_resize_1<(/*void*/)> for (f64,f64) {
  fn resize_1(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget6resizeEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:107
// index:0
// Public Visibility=Default Availability=Available
// [16] QSizeF size() const

/*

*/
impl /*struct*/ QGraphicsWidget {
  pub fn size_0<RetType, T: QGraphicsWidget_size_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.size_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_size_0<RetType> {
  fn size_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_size_0<usize> for () {
  fn size_0(self , rsthis: & QGraphicsWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGraphicsWidget4sizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:109
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setGeometry(const QRectF &)

/*

*/
impl /*struct*/ QGraphicsWidget {
  pub fn setGeometry_0<RetType, T: QGraphicsWidget_setGeometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setGeometry_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_setGeometry_0<RetType> {
  fn setGeometry_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_setGeometry_0<(/*void*/)> for (usize) {
  fn setGeometry_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget11setGeometryERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:110
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void setGeometry(qreal, qreal, qreal, qreal)

/*

*/
impl /*struct*/ QGraphicsWidget {
  pub fn setGeometry_1<RetType, T: QGraphicsWidget_setGeometry_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setGeometry_1(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_setGeometry_1<RetType> {
  fn setGeometry_1(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_setGeometry_1<(/*void*/)> for (f64,f64,f64,f64) {
  fn setGeometry_1(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget11setGeometryEdddd", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:111
// index:0
// Public inline Visibility=Default Availability=Available
// [32] QRectF rect() const

/*
Returns the item's local rect as a QRectF. This function is equivalent to QRectF(QPointF(), size()).

See also setGeometry() and resize().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn rect_0<RetType, T: QGraphicsWidget_rect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rect_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_rect_0<RetType> {
  fn rect_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_rect_0<usize> for () {
  fn rect_0(self , rsthis: & QGraphicsWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGraphicsWidget4rectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:113
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setContentsMargins(qreal, qreal, qreal, qreal)

/*
Sets the widget's contents margins to left, top, right and bottom.

Contents margins are used by the assigned layout to define the placement of subwidgets and layouts. Margins are particularly useful for widgets that constrain subwidgets to only a section of its own geometry. For example, a group box with a layout will place subwidgets inside its frame, but below the title.

Changing a widget's contents margins will always trigger an update(), and any assigned layout will be activated automatically. The widget will then receive a ContentsRectChange event.

See also getContentsMargins() and setGeometry().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn setContentsMargins_0<RetType, T: QGraphicsWidget_setContentsMargins_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setContentsMargins_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_setContentsMargins_0<RetType> {
  fn setContentsMargins_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_setContentsMargins_0<(/*void*/)> for (f64,f64,f64,f64) {
  fn setContentsMargins_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget18setContentsMarginsEdddd", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:114
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void getContentsMargins(qreal *, qreal *, qreal *, qreal *) const

/*
Reimplemented from QGraphicsLayoutItem::getContentsMargins().

Gets the widget's contents margins. The margins are stored in left, top, right and bottom, as pointers to qreals. Each argument can be omitted by passing 0.

See also setContentsMargins().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn getContentsMargins_0<RetType, T: QGraphicsWidget_getContentsMargins_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.getContentsMargins_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_getContentsMargins_0<RetType> {
  fn getContentsMargins_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_getContentsMargins_0<(/*void*/)> for (usize,usize,usize,usize) {
  fn getContentsMargins_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let arg3 = (&self.3) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK15QGraphicsWidget18getContentsMarginsEPdS0_S0_S0_", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:116
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWindowFrameMargins(qreal, qreal, qreal, qreal)

/*
Sets the widget's window frame margins to left, top, right and bottom. The default frame margins are provided by the style, and they depend on the current window flags.

If you would like to draw your own window decoration, you can set your own frame margins to override the default margins.

See also unsetWindowFrameMargins(), getWindowFrameMargins(), and windowFrameRect().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn setWindowFrameMargins_0<RetType, T: QGraphicsWidget_setWindowFrameMargins_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWindowFrameMargins_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_setWindowFrameMargins_0<RetType> {
  fn setWindowFrameMargins_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_setWindowFrameMargins_0<(/*void*/)> for (f64,f64,f64,f64) {
  fn setWindowFrameMargins_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget21setWindowFrameMarginsEdddd", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:117
// index:0
// Public Visibility=Default Availability=Available
// [-2] void getWindowFrameMargins(qreal *, qreal *, qreal *, qreal *) const

/*
Gets the widget's window frame margins. The margins are stored in left, top, right and bottom as pointers to qreals. Each argument can be omitted by passing 0.

See also setWindowFrameMargins() and windowFrameRect().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn getWindowFrameMargins_0<RetType, T: QGraphicsWidget_getWindowFrameMargins_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.getWindowFrameMargins_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_getWindowFrameMargins_0<RetType> {
  fn getWindowFrameMargins_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_getWindowFrameMargins_0<(/*void*/)> for (usize,usize,usize,usize) {
  fn getWindowFrameMargins_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let arg3 = (&self.3) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK15QGraphicsWidget21getWindowFrameMarginsEPdS0_S0_S0_", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:118
// index:0
// Public Visibility=Default Availability=Available
// [-2] void unsetWindowFrameMargins()

/*
Resets the window frame margins to the default value, provided by the style.

See also setWindowFrameMargins(), getWindowFrameMargins(), and windowFrameRect().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn unsetWindowFrameMargins_0<RetType, T: QGraphicsWidget_unsetWindowFrameMargins_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unsetWindowFrameMargins_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_unsetWindowFrameMargins_0<RetType> {
  fn unsetWindowFrameMargins_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_unsetWindowFrameMargins_0<(/*void*/)> for () {
  fn unsetWindowFrameMargins_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget23unsetWindowFrameMarginsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:119
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF windowFrameGeometry() const

/*
Returns the widget's geometry in parent coordinates including any window frame.

See also windowFrameRect(), getWindowFrameMargins(), and setWindowFrameMargins().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn windowFrameGeometry_0<RetType, T: QGraphicsWidget_windowFrameGeometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.windowFrameGeometry_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_windowFrameGeometry_0<RetType> {
  fn windowFrameGeometry_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_windowFrameGeometry_0<usize> for () {
  fn windowFrameGeometry_0(self , rsthis: & QGraphicsWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGraphicsWidget19windowFrameGeometryEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:120
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF windowFrameRect() const

/*
Returns the widget's local rect including any window frame.

See also windowFrameGeometry(), getWindowFrameMargins(), and setWindowFrameMargins().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn windowFrameRect_0<RetType, T: QGraphicsWidget_windowFrameRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.windowFrameRect_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_windowFrameRect_0<RetType> {
  fn windowFrameRect_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_windowFrameRect_0<usize> for () {
  fn windowFrameRect_0(self , rsthis: & QGraphicsWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGraphicsWidget15windowFrameRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:123
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::WindowFlags windowFlags() const

/*

*/
impl /*struct*/ QGraphicsWidget {
  pub fn windowFlags_0<RetType, T: QGraphicsWidget_windowFlags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.windowFlags_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_windowFlags_0<RetType> {
  fn windowFlags_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_windowFlags_0<i32> for () {
  fn windowFlags_0(self , rsthis: & QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGraphicsWidget11windowFlagsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:124
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::WindowType windowType() const

/*
Returns the widgets window type.

See also windowFlags(), isWindow(), and isPanel().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn windowType_0<RetType, T: QGraphicsWidget_windowType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.windowType_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_windowType_0<RetType> {
  fn windowType_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_windowType_0<i32> for () {
  fn windowType_0(self , rsthis: & QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGraphicsWidget10windowTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:125
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWindowFlags(Qt::WindowFlags)

/*

*/
impl /*struct*/ QGraphicsWidget {
  pub fn setWindowFlags_0<RetType, T: QGraphicsWidget_setWindowFlags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWindowFlags_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_setWindowFlags_0<RetType> {
  fn setWindowFlags_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_setWindowFlags_0<(/*void*/)> for (i32) {
  fn setWindowFlags_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget14setWindowFlagsE6QFlagsIN2Qt10WindowTypeEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:126
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isActiveWindow() const

/*
Returns true if this widget's window is in the active window, or if the widget does not have a window but is in an active scene (i.e., a scene that currently has focus).

The active window is the window that either contains a child widget that currently has input focus, or that itself has input focus.

See also QGraphicsScene::activeWindow(), QGraphicsScene::setActiveWindow(), and isActive().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn isActiveWindow_0<RetType, T: QGraphicsWidget_isActiveWindow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isActiveWindow_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_isActiveWindow_0<RetType> {
  fn isActiveWindow_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_isActiveWindow_0<bool> for () {
  fn isActiveWindow_0(self , rsthis: & QGraphicsWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGraphicsWidget14isActiveWindowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:127
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWindowTitle(const QString &)

/*

*/
impl /*struct*/ QGraphicsWidget {
  pub fn setWindowTitle_0<RetType, T: QGraphicsWidget_setWindowTitle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWindowTitle_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_setWindowTitle_0<RetType> {
  fn setWindowTitle_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_setWindowTitle_0<(/*void*/)> for (usize) {
  fn setWindowTitle_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget14setWindowTitleERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:128
// index:0
// Public Visibility=Default Availability=Available
// [8] QString windowTitle() const

/*

*/
impl /*struct*/ QGraphicsWidget {
  pub fn windowTitle_0<RetType, T: QGraphicsWidget_windowTitle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.windowTitle_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_windowTitle_0<RetType> {
  fn windowTitle_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_windowTitle_0<usize> for () {
  fn windowTitle_0(self , rsthis: & QGraphicsWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGraphicsWidget11windowTitleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:131
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::FocusPolicy focusPolicy() const

/*

*/
impl /*struct*/ QGraphicsWidget {
  pub fn focusPolicy_0<RetType, T: QGraphicsWidget_focusPolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusPolicy_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_focusPolicy_0<RetType> {
  fn focusPolicy_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_focusPolicy_0<i32> for () {
  fn focusPolicy_0(self , rsthis: & QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGraphicsWidget11focusPolicyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:132
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFocusPolicy(Qt::FocusPolicy)

/*

*/
impl /*struct*/ QGraphicsWidget {
  pub fn setFocusPolicy_0<RetType, T: QGraphicsWidget_setFocusPolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFocusPolicy_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_setFocusPolicy_0<RetType> {
  fn setFocusPolicy_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_setFocusPolicy_0<(/*void*/)> for (i32) {
  fn setFocusPolicy_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget14setFocusPolicyEN2Qt11FocusPolicyE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:133
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setTabOrder(QGraphicsWidget *, QGraphicsWidget *)

/*
Moves the second widget around the ring of focus widgets so that keyboard focus moves from the first widget to the second widget when the Tab key is pressed.

Note that since the tab order of the second widget is changed, you should order a chain like this:


  setTabOrder(a, b); // a to b
  setTabOrder(b, c); // a to b to c
  setTabOrder(c, d); // a to b to c to d



not like this:


  // WRONG
  setTabOrder(c, d); // c to d
  setTabOrder(a, b); // a to b AND c to d
  setTabOrder(b, c); // a to b to c, but not c to d



If first is 0, this indicates that second should be the first widget to receive input focus should the scene gain Tab focus (i.e., the user hits Tab so that focus passes into the scene). If second is 0, this indicates that first should be the first widget to gain focus if the scene gained BackTab focus.

By default, tab order is defined implicitly using widget creation order.

See also focusPolicy and Keyboard Focus in Widgets.
*/
impl /*struct*/ QGraphicsWidget {
  pub fn setTabOrder_0<RetType, T: QGraphicsWidget_setTabOrder_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setTabOrder_0();
    // return 1;
  }
}
pub trait QGraphicsWidget_setTabOrder_0<RetType> {
  fn setTabOrder_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_setTabOrder_0<(/*void*/)> for (usize,usize) {
  fn setTabOrder_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget11setTabOrderEPS_S0_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:134
// index:0
// Public Visibility=Default Availability=Available
// [8] QGraphicsWidget * focusWidget() const

/*
If this widget, a child or descendant of this widget currently has input focus, this function will return a pointer to that widget. If no descendant widget has input focus, 0 is returned.

See also QGraphicsItem::focusItem() and QWidget::focusWidget().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn focusWidget_0<RetType, T: QGraphicsWidget_focusWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusWidget_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_focusWidget_0<RetType> {
  fn focusWidget_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_focusWidget_0<usize> for () {
  fn focusWidget_0(self , rsthis: & QGraphicsWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGraphicsWidget11focusWidgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:137
// index:0
// Public Visibility=Default Availability=Available
// [4] int grabShortcut(const QKeySequence &, Qt::ShortcutContext)

/*
Adds a shortcut to Qt's shortcut system that watches for the given key sequence in the given context. If the context is Qt::ApplicationShortcut, the shortcut applies to the application as a whole. Otherwise, it is either local to this widget, Qt::WidgetShortcut, or to the window itself, Qt::WindowShortcut. For widgets that are not part of a window (i.e., top-level widgets and their children), Qt::WindowShortcut shortcuts apply to the scene.

If the same key sequence has been grabbed by several widgets, when the key sequence occurs a QEvent::Shortcut event is sent to all the widgets to which it applies in a non-deterministic order, but with the ``ambiguous'' flag set to true.

Warning: You should not normally need to use this function; instead create QActions with the shortcut key sequences you require (if you also want equivalent menu options and toolbar buttons), or create QShortcuts if you just need key sequences. Both QAction and QShortcut handle all the event filtering for you, and provide signals which are triggered when the user triggers the key sequence, so are much easier to use than this low-level function.

This function was introduced in  Qt 4.5.

See also releaseShortcut(), setShortcutEnabled(), and QWidget::grabShortcut().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn grabShortcut_0<RetType, T: QGraphicsWidget_grabShortcut_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.grabShortcut_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_grabShortcut_0<RetType> {
  fn grabShortcut_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_grabShortcut_0<i32> for (usize,i32) {
  fn grabShortcut_0(self , rsthis: & QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget12grabShortcutERK12QKeySequenceN2Qt15ShortcutContextE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:138
// index:0
// Public Visibility=Default Availability=Available
// [-2] void releaseShortcut(int)

/*
Removes the shortcut with the given id from Qt's shortcut system. The widget will no longer receive QEvent::Shortcut events for the shortcut's key sequence (unless it has other shortcuts with the same key sequence).

Warning: You should not normally need to use this function since Qt's shortcut system removes shortcuts automatically when their parent widget is destroyed. It is best to use QAction or QShortcut to handle shortcuts, since they are easier to use than this low-level function. Note also that this is an expensive operation.

This function was introduced in  Qt 4.5.

See also grabShortcut(), setShortcutEnabled(), and QWidget::releaseShortcut().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn releaseShortcut_0<RetType, T: QGraphicsWidget_releaseShortcut_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.releaseShortcut_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_releaseShortcut_0<RetType> {
  fn releaseShortcut_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_releaseShortcut_0<(/*void*/)> for (i32) {
  fn releaseShortcut_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget15releaseShortcutEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:139
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setShortcutEnabled(int, bool)

/*
If enabled is true, the shortcut with the given id is enabled; otherwise the shortcut is disabled.

Warning: You should not normally need to use this function since Qt's shortcut system enables/disables shortcuts automatically as widgets become hidden/visible and gain or lose focus. It is best to use QAction or QShortcut to handle shortcuts, since they are easier to use than this low-level function.

This function was introduced in  Qt 4.5.

See also grabShortcut(), releaseShortcut(), and QWidget::setShortcutEnabled().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn setShortcutEnabled_0<RetType, T: QGraphicsWidget_setShortcutEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setShortcutEnabled_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_setShortcutEnabled_0<RetType> {
  fn setShortcutEnabled_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_setShortcutEnabled_0<(/*void*/)> for (i32,bool) {
  fn setShortcutEnabled_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget18setShortcutEnabledEib", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:140
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setShortcutAutoRepeat(int, bool)

/*
If enabled is true, auto repeat of the shortcut with the given id is enabled; otherwise it is disabled.

This function was introduced in  Qt 4.5.

See also grabShortcut(), releaseShortcut(), and QWidget::setShortcutAutoRepeat().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn setShortcutAutoRepeat_0<RetType, T: QGraphicsWidget_setShortcutAutoRepeat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setShortcutAutoRepeat_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_setShortcutAutoRepeat_0<RetType> {
  fn setShortcutAutoRepeat_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_setShortcutAutoRepeat_0<(/*void*/)> for (i32,bool) {
  fn setShortcutAutoRepeat_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget21setShortcutAutoRepeatEib", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:145
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addAction(QAction *)

/*
Appends the action action to this widget's list of actions.

All QGraphicsWidgets have a list of QActions, however they can be represented graphically in many different ways. The default use of the QAction list (as returned by actions()) is to create a context QMenu.

A QGraphicsWidget should only have one of each action and adding an action it already has will not cause the same action to be in the widget twice.

This function was introduced in  Qt 4.5.

See also removeAction(), insertAction(), actions(), and QWidget::addAction().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn addAction_0<RetType, T: QGraphicsWidget_addAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addAction_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_addAction_0<RetType> {
  fn addAction_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_addAction_0<(/*void*/)> for (usize) {
  fn addAction_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget9addActionEP7QAction", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:153
// index:0
// Public Visibility=Default Availability=Available
// [-2] void insertAction(QAction *, QAction *)

/*
Inserts the action action to this widget's list of actions, before the action before. It appends the action if before is 0 or before is not a valid action for this widget.

A QGraphicsWidget should only have one of each action.

This function was introduced in  Qt 4.5.

See also removeAction(), addAction(), QMenu, actions(), and QWidget::insertActions().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn insertAction_0<RetType, T: QGraphicsWidget_insertAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertAction_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_insertAction_0<RetType> {
  fn insertAction_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_insertAction_0<(/*void*/)> for (usize,usize) {
  fn insertAction_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget12insertActionEP7QActionS1_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:154
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeAction(QAction *)

/*
Removes the action action from this widget's list of actions.

This function was introduced in  Qt 4.5.

See also insertAction(), actions(), insertAction(), and QWidget::removeAction().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn removeAction_0<RetType, T: QGraphicsWidget_removeAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeAction_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_removeAction_0<RetType> {
  fn removeAction_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_removeAction_0<(/*void*/)> for (usize) {
  fn removeAction_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget12removeActionEP7QAction", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:158
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAttribute(Qt::WidgetAttribute, bool)

/*
If on is true, this function enables attribute; otherwise attribute is disabled.

See the class documentation for QGraphicsWidget for a complete list of which attributes are supported, and what they are for.

See also testAttribute() and QWidget::setAttribute().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn setAttribute_0<RetType, T: QGraphicsWidget_setAttribute_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAttribute_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_setAttribute_0<RetType> {
  fn setAttribute_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_setAttribute_0<(/*void*/)> for (i32,bool) {
  fn setAttribute_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget12setAttributeEN2Qt15WidgetAttributeEb", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:159
// index:0
// Public Visibility=Default Availability=Available
// [1] bool testAttribute(Qt::WidgetAttribute) const

/*
Returns true if attribute is enabled for this widget; otherwise, returns false.

See also setAttribute().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn testAttribute_0<RetType, T: QGraphicsWidget_testAttribute_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.testAttribute_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_testAttribute_0<RetType> {
  fn testAttribute_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_testAttribute_0<bool> for (i32) {
  fn testAttribute_0(self , rsthis: & QGraphicsWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGraphicsWidget13testAttributeEN2Qt15WidgetAttributeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:164
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int type() const

/*
Reimplemented from QGraphicsItem::type().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn type__0<RetType, T: QGraphicsWidget_type__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.type__0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_type__0<RetType> {
  fn type__0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_type__0<i32> for () {
  fn type__0(self , rsthis: & QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGraphicsWidget4typeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:166
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void paint(QPainter *, const QStyleOptionGraphicsItem *, QWidget *)

/*
Reimplemented from QGraphicsItem::paint().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn paint_0<RetType, T: QGraphicsWidget_paint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paint_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_paint_0<RetType> {
  fn paint_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_paint_0<(/*void*/)> for (usize,usize,usize) {
  fn paint_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:167
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void paintWindowFrame(QPainter *, const QStyleOptionGraphicsItem *, QWidget *)

/*
This virtual function is called by QGraphicsScene to draw the window frame for windows using painter, option, and widget, in local coordinates. The base implementation uses the current style to render the frame and title bar.

You can reimplement this function in a subclass of QGraphicsWidget to provide custom rendering of the widget's window frame.

See also QGraphicsItem::paint().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn paintWindowFrame_0<RetType, T: QGraphicsWidget_paintWindowFrame_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintWindowFrame_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_paintWindowFrame_0<RetType> {
  fn paintWindowFrame_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_paintWindowFrame_0<(/*void*/)> for (usize,usize,usize) {
  fn paintWindowFrame_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget16paintWindowFrameEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:168
// index:0
// Public virtual Visibility=Default Availability=Available
// [32] QRectF boundingRect() const

/*
Reimplemented from QGraphicsItem::boundingRect().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn boundingRect_0<RetType, T: QGraphicsWidget_boundingRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.boundingRect_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_boundingRect_0<RetType> {
  fn boundingRect_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_boundingRect_0<usize> for () {
  fn boundingRect_0(self , rsthis: & QGraphicsWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGraphicsWidget12boundingRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:169
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QPainterPath shape() const

/*
Reimplemented from QGraphicsItem::shape().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn shape_0<RetType, T: QGraphicsWidget_shape_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.shape_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_shape_0<RetType> {
  fn shape_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_shape_0<usize> for () {
  fn shape_0(self , rsthis: & QGraphicsWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGraphicsWidget5shapeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:178
// index:0
// Public Visibility=Default Availability=Available
// [-2] void geometryChanged()

/*
This signal gets emitted whenever the geometry is changed in setGeometry().

Note: Notifier signal for property geometry. Notifier signal for property size.
*/
impl /*struct*/ QGraphicsWidget {
  pub fn geometryChanged_0<RetType, T: QGraphicsWidget_geometryChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.geometryChanged_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_geometryChanged_0<RetType> {
  fn geometryChanged_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_geometryChanged_0<(/*void*/)> for () {
  fn geometryChanged_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget15geometryChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:179
// index:0
// Public Visibility=Default Availability=Available
// [-2] void layoutChanged()

/*

*/
impl /*struct*/ QGraphicsWidget {
  pub fn layoutChanged_0<RetType, T: QGraphicsWidget_layoutChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.layoutChanged_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_layoutChanged_0<RetType> {
  fn layoutChanged_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_layoutChanged_0<(/*void*/)> for () {
  fn layoutChanged_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget13layoutChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:182
// index:0
// Public Visibility=Default Availability=Available
// [1] bool close()

/*
Call this function to close the widget.

Returns true if the widget was closed; otherwise returns false. This slot will first send a QCloseEvent to the widget, which may or may not accept the event. If the event was ignored, nothing happens. If the event was accepted, it will hide() the widget.

If the widget has the Qt::WA_DeleteOnClose attribute set it will be deleted.
*/
impl /*struct*/ QGraphicsWidget {
  pub fn close_0<RetType, T: QGraphicsWidget_close_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.close_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_close_0<RetType> {
  fn close_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_close_0<bool> for () {
  fn close_0(self , rsthis: & QGraphicsWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget5closeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:185
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void initStyleOption(QStyleOption *) const

/*
Populates a style option object for this widget based on its current state, and stores the output in option. The default implementation populates option with the following properties.


 Style Option PropertyValue
state & QStyle::State_EnabledCorresponds to QGraphicsItem::isEnabled().
state & QStyle::State_HasFocusCorresponds to QGraphicsItem::hasFocus().
state & QStyle::State_MouseOverCorresponds to QGraphicsItem::isUnderMouse().
directionCorresponds to QGraphicsWidget::layoutDirection().
rectCorresponds to QGraphicsWidget::rect().toRect().
paletteCorresponds to QGraphicsWidget::palette().
fontMetricsCorresponds to QFontMetrics(QGraphicsWidget::font()).


Subclasses of QGraphicsWidget should call the base implementation, and then test the type of option using qstyleoption_cast<>() or test QStyleOption::Type before storing widget-specific options.

For example:


  void MyGroupBoxWidget::initStyleOption(QStyleOption *option) const
  {
      QGraphicsWidget::initStyleOption(option);
      if (QStyleOptionGroupBox *box = qstyleoption_cast<QStyleOptionGroupBox *>(option)) {
          // Add group box specific state.
          box->flat = isFlat();
          ...
      }
  }



See also QStyleOption::initFrom().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn initStyleOption_0<RetType, T: QGraphicsWidget_initStyleOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.initStyleOption_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_initStyleOption_0<RetType> {
  fn initStyleOption_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_initStyleOption_0<(/*void*/)> for (usize) {
  fn initStyleOption_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK15QGraphicsWidget15initStyleOptionEP12QStyleOption", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:187
// index:0
// Protected virtual Visibility=Default Availability=Available
// [16] QSizeF sizeHint(Qt::SizeHint, const QSizeF &) const

/*
Reimplemented from QGraphicsLayoutItem::sizeHint().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn sizeHint_0<RetType, T: QGraphicsWidget_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_sizeHint_0<usize> for (i32,usize) {
  fn sizeHint_0(self , rsthis: & QGraphicsWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGraphicsWidget8sizeHintEN2Qt8SizeHintERK6QSizeF", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:188
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void updateGeometry()

/*
Reimplemented from QGraphicsLayoutItem::updateGeometry().

If this widget is currently managed by a layout, this function notifies the layout that the widget's size hints have changed and the layout may need to resize and reposition the widget accordingly.

Call this function if the widget's sizeHint() has changed.

See also QGraphicsLayout::invalidate().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn updateGeometry_0<RetType, T: QGraphicsWidget_updateGeometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updateGeometry_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_updateGeometry_0<RetType> {
  fn updateGeometry_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_updateGeometry_0<(/*void*/)> for () {
  fn updateGeometry_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget14updateGeometryEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:191
// index:0
// Protected virtual Visibility=Default Availability=Available
// [16] QVariant itemChange(QGraphicsItem::GraphicsItemChange, const QVariant &)

/*
Reimplemented from QGraphicsItem::itemChange().

QGraphicsWidget uses the base implementation of this function to catch and deliver events related to state changes in the item. Because of this, it is very important that subclasses call the base implementation.

change specifies the type of change, and value is the new value.

For example, QGraphicsWidget uses ItemVisibleChange to deliver Show and Hide events, ItemPositionHasChanged to deliver Move events, and ItemParentChange both to deliver ParentChange events, and for managing the focus chain.

QGraphicsWidget enables the ItemSendsGeometryChanges flag by default in order to track position changes.

See also QGraphicsItem::itemChange().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn itemChange_0<RetType, T: QGraphicsWidget_itemChange_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemChange_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_itemChange_0<RetType> {
  fn itemChange_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_itemChange_0<usize> for (i32,usize) {
  fn itemChange_0(self , rsthis: & QGraphicsWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget10itemChangeEN13QGraphicsItem18GraphicsItemChangeERK8QVariant", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:192
// index:0
// Protected virtual Visibility=Default Availability=Available
// [16] QVariant propertyChange(const QString &, const QVariant &)

/*

*/
impl /*struct*/ QGraphicsWidget {
  pub fn propertyChange_0<RetType, T: QGraphicsWidget_propertyChange_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.propertyChange_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_propertyChange_0<RetType> {
  fn propertyChange_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_propertyChange_0<usize> for (usize,usize) {
  fn propertyChange_0(self , rsthis: & QGraphicsWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget14propertyChangeERK7QStringRK8QVariant", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:195
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool sceneEvent(QEvent *)

/*
Reimplemented from QGraphicsItem::sceneEvent().

QGraphicsWidget's implementation of sceneEvent() simply passes event to QGraphicsWidget::event(). You can handle all events for your widget in event() or in any of the convenience functions; you should not have to reimplement this function in a subclass of QGraphicsWidget.

See also QGraphicsItem::sceneEvent().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn sceneEvent_0<RetType, T: QGraphicsWidget_sceneEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sceneEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_sceneEvent_0<RetType> {
  fn sceneEvent_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_sceneEvent_0<bool> for (usize) {
  fn sceneEvent_0(self , rsthis: & QGraphicsWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget10sceneEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:196
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool windowFrameEvent(QEvent *)

/*
This event handler, for event, receives events for the window frame if this widget is a window. Its base implementation provides support for default window frame interaction such as moving, resizing, etc.

You can reimplement this handler in a subclass of QGraphicsWidget to provide your own custom window frame interaction support.

Returns true if event has been recognized and processed; otherwise, returns false.

See also event().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn windowFrameEvent_0<RetType, T: QGraphicsWidget_windowFrameEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.windowFrameEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_windowFrameEvent_0<RetType> {
  fn windowFrameEvent_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_windowFrameEvent_0<bool> for (usize) {
  fn windowFrameEvent_0(self , rsthis: & QGraphicsWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget16windowFrameEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:197
// index:0
// Protected virtual Visibility=Default Availability=Available
// [4] Qt::WindowFrameSection windowFrameSectionAt(const QPointF &) const

/*
Returns the window frame section at position pos, or Qt::NoSection if there is no window frame section at this position.

This function is used in QGraphicsWidget's base implementation for window frame interaction.

You can reimplement this function if you want to customize how a window can be interactively moved or resized. For instance, if you only want to allow a window to be resized by the bottom right corner, you can reimplement this function to return Qt::NoSection for all sections except Qt::BottomRightSection.

This function was introduced in  Qt 4.4.

See also windowFrameEvent(), paintWindowFrame(), and windowFrameGeometry().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn windowFrameSectionAt_0<RetType, T: QGraphicsWidget_windowFrameSectionAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.windowFrameSectionAt_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_windowFrameSectionAt_0<RetType> {
  fn windowFrameSectionAt_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_windowFrameSectionAt_0<i32> for (usize) {
  fn windowFrameSectionAt_0(self , rsthis: & QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGraphicsWidget20windowFrameSectionAtERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:200
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().

Handles the event. QGraphicsWidget handles the following events:


 EventUsage
PolishDelivered to the widget some time after it has been shown.
GraphicsSceneMoveDelivered to the widget after its local position has changed.
GraphicsSceneResizeDelivered to the widget after its size has changed.
ShowDelivered to the widget before it has been shown.
HideDelivered to the widget after it has been hidden.
PaletteChangeDelivered to the widget after its palette has changed.
FontChangeDelivered to the widget after its font has changed.
EnabledChangeDelivered to the widget after its enabled state has changed.
StyleChangeDelivered to the widget after its style has changed.
LayoutDirectionChangeDelivered to the widget after its layout direction has changed.
ContentsRectChangeDelivered to the widget after its contents margins/ contents rect has changed.
*/
impl /*struct*/ QGraphicsWidget {
  pub fn event_0<RetType, T: QGraphicsWidget_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_event_0<RetType> {
  fn event_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QGraphicsWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:202
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void changeEvent(QEvent *)

/*
This event handler can be reimplemented to handle state changes.

The state being changed in this event can be retrieved through event.

Change events include: QEvent::ActivationChange, QEvent::EnabledChange, QEvent::FontChange, QEvent::StyleChange, QEvent::PaletteChange, QEvent::ParentChange, QEvent::LayoutDirectionChange, and QEvent::ContentsRectChange.
*/
impl /*struct*/ QGraphicsWidget {
  pub fn changeEvent_0<RetType, T: QGraphicsWidget_changeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changeEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_changeEvent_0<RetType> {
  fn changeEvent_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_changeEvent_0<(/*void*/)> for (usize) {
  fn changeEvent_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget11changeEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:203
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void closeEvent(QCloseEvent *)

/*
This event handler, for event, can be reimplemented in a subclass to receive widget close events. The default implementation accepts the event.

See also close() and QCloseEvent.
*/
impl /*struct*/ QGraphicsWidget {
  pub fn closeEvent_0<RetType, T: QGraphicsWidget_closeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.closeEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_closeEvent_0<RetType> {
  fn closeEvent_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_closeEvent_0<(/*void*/)> for (usize) {
  fn closeEvent_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget10closeEventEP11QCloseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:206
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusInEvent(QFocusEvent *)

/*
Reimplemented from QGraphicsItem::focusInEvent().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn focusInEvent_0<RetType, T: QGraphicsWidget_focusInEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusInEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_focusInEvent_0<RetType> {
  fn focusInEvent_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_focusInEvent_0<(/*void*/)> for (usize) {
  fn focusInEvent_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget12focusInEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:207
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool focusNextPrevChild(bool)

/*
Finds a new widget to give the keyboard focus to, as appropriate for Tab and Shift+Tab, and returns true if it can find a new widget; returns false otherwise. If next is true, this function searches forward; if next is false, it searches backward.

Sometimes, you will want to reimplement this function to provide special focus handling for your widget and its subwidgets. For example, a web browser might reimplement it to move its current active link forward or backward, and call the base implementation only when it reaches the last or first link on the page.

Child widgets call focusNextPrevChild() on their parent widgets, but only the window that contains the child widgets decides where to redirect focus. By reimplementing this function for an object, you gain control of focus traversal for all child widgets.

See also focusPolicy().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn focusNextPrevChild_0<RetType, T: QGraphicsWidget_focusNextPrevChild_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusNextPrevChild_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_focusNextPrevChild_0<RetType> {
  fn focusNextPrevChild_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_focusNextPrevChild_0<bool> for (bool) {
  fn focusNextPrevChild_0(self , rsthis: & QGraphicsWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget18focusNextPrevChildEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:208
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusOutEvent(QFocusEvent *)

/*
Reimplemented from QGraphicsItem::focusOutEvent().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn focusOutEvent_0<RetType, T: QGraphicsWidget_focusOutEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusOutEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_focusOutEvent_0<RetType> {
  fn focusOutEvent_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_focusOutEvent_0<(/*void*/)> for (usize) {
  fn focusOutEvent_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget13focusOutEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:209
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void hideEvent(QHideEvent *)

/*
This event handler, for Hide events, is delivered after the widget has been hidden, for example, setVisible(false) has been called for the widget or one of its ancestors when the widget was previously shown.

You can reimplement this event handler to detect when your widget is hidden. Calling QEvent::accept() or QEvent::ignore() on event has no effect.

See also showEvent(), QWidget::hideEvent(), and ItemVisibleChange.
*/
impl /*struct*/ QGraphicsWidget {
  pub fn hideEvent_0<RetType, T: QGraphicsWidget_hideEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hideEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_hideEvent_0<RetType> {
  fn hideEvent_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_hideEvent_0<(/*void*/)> for (usize) {
  fn hideEvent_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget9hideEventEP10QHideEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:211
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void moveEvent(QGraphicsSceneMoveEvent *)

/*
This event handler, for GraphicsSceneMove events, is delivered after the widget has moved (e.g., its local position has changed).

This event is only delivered when the item is moved locally. Calling setTransform() or moving any of the item's ancestors does not affect the item's local position.

You can reimplement this event handler to detect when your widget has moved. Calling QEvent::accept() or QEvent::ignore() on event has no effect.

See also ItemPositionChange and ItemPositionHasChanged.
*/
impl /*struct*/ QGraphicsWidget {
  pub fn moveEvent_0<RetType, T: QGraphicsWidget_moveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_moveEvent_0<RetType> {
  fn moveEvent_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_moveEvent_0<(/*void*/)> for (usize) {
  fn moveEvent_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget9moveEventEP23QGraphicsSceneMoveEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:212
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void polishEvent()

/*
This event is delivered to the item by the scene at some point after it has been constructed, but before it is shown or otherwise accessed through the scene. You can use this event handler to do last-minute initializations of the widget which require the item to be fully constructed.

The base implementation does nothing.
*/
impl /*struct*/ QGraphicsWidget {
  pub fn polishEvent_0<RetType, T: QGraphicsWidget_polishEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.polishEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_polishEvent_0<RetType> {
  fn polishEvent_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_polishEvent_0<(/*void*/)> for () {
  fn polishEvent_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget11polishEventEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:214
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void resizeEvent(QGraphicsSceneResizeEvent *)

/*
This event handler, for GraphicsSceneResize events, is delivered after the widget has been resized (i.e., its local size has changed). event contains both the old and the new size.

This event is only delivered when the widget is resized locally; calling setTransform() on the widget or any of its ancestors or view, does not affect the widget's local size.

You can reimplement this event handler to detect when your widget has been resized. Calling QEvent::accept() or QEvent::ignore() on event has no effect.

See also geometry() and setGeometry().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn resizeEvent_0<RetType, T: QGraphicsWidget_resizeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_resizeEvent_0<RetType> {
  fn resizeEvent_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_resizeEvent_0<(/*void*/)> for (usize) {
  fn resizeEvent_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget11resizeEventEP25QGraphicsSceneResizeEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:215
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void showEvent(QShowEvent *)

/*
This event handler, for Show events, is delivered before the widget has been shown, for example, setVisible(true) has been called for the widget or one of its ancestors when the widget was previously hidden.

You can reimplement this event handler to detect when your widget is shown. Calling QEvent::accept() or QEvent::ignore() on event has no effect.

See also hideEvent(), QWidget::showEvent(), and ItemVisibleChange.
*/
impl /*struct*/ QGraphicsWidget {
  pub fn showEvent_0<RetType, T: QGraphicsWidget_showEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_showEvent_0<RetType> {
  fn showEvent_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_showEvent_0<(/*void*/)> for (usize) {
  fn showEvent_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget9showEventEP10QShowEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:217
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void hoverMoveEvent(QGraphicsSceneHoverEvent *)

/*
Reimplemented from QGraphicsItem::hoverMoveEvent().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn hoverMoveEvent_0<RetType, T: QGraphicsWidget_hoverMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hoverMoveEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_hoverMoveEvent_0<RetType> {
  fn hoverMoveEvent_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_hoverMoveEvent_0<(/*void*/)> for (usize) {
  fn hoverMoveEvent_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget14hoverMoveEventEP24QGraphicsSceneHoverEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:218
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void hoverLeaveEvent(QGraphicsSceneHoverEvent *)

/*
Reimplemented from QGraphicsItem::hoverLeaveEvent().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn hoverLeaveEvent_0<RetType, T: QGraphicsWidget_hoverLeaveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hoverLeaveEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_hoverLeaveEvent_0<RetType> {
  fn hoverLeaveEvent_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_hoverLeaveEvent_0<(/*void*/)> for (usize) {
  fn hoverLeaveEvent_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget15hoverLeaveEventEP24QGraphicsSceneHoverEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:219
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void grabMouseEvent(QEvent *)

/*
This event handler, for event, can be reimplemented in a subclass to receive notifications for QEvent::GrabMouse events.

See also grabMouse() and grabKeyboard().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn grabMouseEvent_0<RetType, T: QGraphicsWidget_grabMouseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.grabMouseEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_grabMouseEvent_0<RetType> {
  fn grabMouseEvent_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_grabMouseEvent_0<(/*void*/)> for (usize) {
  fn grabMouseEvent_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget14grabMouseEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:220
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void ungrabMouseEvent(QEvent *)

/*
This event handler, for event, can be reimplemented in a subclass to receive notifications for QEvent::UngrabMouse events.

See also ungrabMouse() and ungrabKeyboard().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn ungrabMouseEvent_0<RetType, T: QGraphicsWidget_ungrabMouseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ungrabMouseEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_ungrabMouseEvent_0<RetType> {
  fn ungrabMouseEvent_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_ungrabMouseEvent_0<(/*void*/)> for (usize) {
  fn ungrabMouseEvent_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget16ungrabMouseEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:221
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void grabKeyboardEvent(QEvent *)

/*
This event handler, for event, can be reimplemented in a subclass to receive notifications for QEvent::GrabKeyboard events.

See also grabKeyboard() and grabMouse().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn grabKeyboardEvent_0<RetType, T: QGraphicsWidget_grabKeyboardEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.grabKeyboardEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_grabKeyboardEvent_0<RetType> {
  fn grabKeyboardEvent_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_grabKeyboardEvent_0<(/*void*/)> for (usize) {
  fn grabKeyboardEvent_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget17grabKeyboardEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicswidget.h:222
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void ungrabKeyboardEvent(QEvent *)

/*
This event handler, for event, can be reimplemented in a subclass to receive notifications for QEvent::UngrabKeyboard events.

See also ungrabKeyboard() and ungrabMouse().
*/
impl /*struct*/ QGraphicsWidget {
  pub fn ungrabKeyboardEvent_0<RetType, T: QGraphicsWidget_ungrabKeyboardEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ungrabKeyboardEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsWidget_ungrabKeyboardEvent_0<RetType> {
  fn ungrabKeyboardEvent_0(self , rsthis: & QGraphicsWidget) -> RetType;
}
impl<'a> /*trait*/ QGraphicsWidget_ungrabKeyboardEvent_0<(/*void*/)> for (usize) {
  fn ungrabKeyboardEvent_0(self , rsthis: & QGraphicsWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsWidget19ungrabKeyboardEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*


*/
pub type QGraphicsWidget__ = i32;
// 
pub const QGraphicsWidget__Type :QGraphicsWidget__ = 11;
pub fn QGraphicsWidget_ItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QGraphicsWidget", val);
}
pub fn QGraphicsWidget_ItemName_s(val: i32) ->String {
  //var nilthis *QGraphicsWidget
  //return nilthis.ItemName(val);
  return QGraphicsWidget_ItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
