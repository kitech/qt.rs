

// mod ::widgets::QGraphicsTextItem
// package qtwidgets
// /usr/include/qt/QtWidgets/qgraphicsitem.h
// #include <qgraphicsitem.h>
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

// bool sceneEvent(QEvent *)
// func (this *QGraphicsTextItem) InheritSceneEvent(f func(event *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "sceneEvent", f)
// }

// void mousePressEvent(QGraphicsSceneMouseEvent *)
// func (this *QGraphicsTextItem) InheritMousePressEvent(f func(event *QGraphicsSceneMouseEvent/*777 QGraphicsSceneMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mousePressEvent", f)
// }

// void mouseMoveEvent(QGraphicsSceneMouseEvent *)
// func (this *QGraphicsTextItem) InheritMouseMoveEvent(f func(event *QGraphicsSceneMouseEvent/*777 QGraphicsSceneMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseMoveEvent", f)
// }

// void mouseReleaseEvent(QGraphicsSceneMouseEvent *)
// func (this *QGraphicsTextItem) InheritMouseReleaseEvent(f func(event *QGraphicsSceneMouseEvent/*777 QGraphicsSceneMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseReleaseEvent", f)
// }

// void mouseDoubleClickEvent(QGraphicsSceneMouseEvent *)
// func (this *QGraphicsTextItem) InheritMouseDoubleClickEvent(f func(event *QGraphicsSceneMouseEvent/*777 QGraphicsSceneMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseDoubleClickEvent", f)
// }

// void contextMenuEvent(QGraphicsSceneContextMenuEvent *)
// func (this *QGraphicsTextItem) InheritContextMenuEvent(f func(event *QGraphicsSceneContextMenuEvent/*777 QGraphicsSceneContextMenuEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "contextMenuEvent", f)
// }

// void keyPressEvent(QKeyEvent *)
// func (this *QGraphicsTextItem) InheritKeyPressEvent(f func(event *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyPressEvent", f)
// }

// void keyReleaseEvent(QKeyEvent *)
// func (this *QGraphicsTextItem) InheritKeyReleaseEvent(f func(event *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyReleaseEvent", f)
// }

// void focusInEvent(QFocusEvent *)
// func (this *QGraphicsTextItem) InheritFocusInEvent(f func(event *qtgui.QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusInEvent", f)
// }

// void focusOutEvent(QFocusEvent *)
// func (this *QGraphicsTextItem) InheritFocusOutEvent(f func(event *qtgui.QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusOutEvent", f)
// }

// void dragEnterEvent(QGraphicsSceneDragDropEvent *)
// func (this *QGraphicsTextItem) InheritDragEnterEvent(f func(event *QGraphicsSceneDragDropEvent/*777 QGraphicsSceneDragDropEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dragEnterEvent", f)
// }

// void dragLeaveEvent(QGraphicsSceneDragDropEvent *)
// func (this *QGraphicsTextItem) InheritDragLeaveEvent(f func(event *QGraphicsSceneDragDropEvent/*777 QGraphicsSceneDragDropEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dragLeaveEvent", f)
// }

// void dragMoveEvent(QGraphicsSceneDragDropEvent *)
// func (this *QGraphicsTextItem) InheritDragMoveEvent(f func(event *QGraphicsSceneDragDropEvent/*777 QGraphicsSceneDragDropEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dragMoveEvent", f)
// }

// void dropEvent(QGraphicsSceneDragDropEvent *)
// func (this *QGraphicsTextItem) InheritDropEvent(f func(event *QGraphicsSceneDragDropEvent/*777 QGraphicsSceneDragDropEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dropEvent", f)
// }

// void inputMethodEvent(QInputMethodEvent *)
// func (this *QGraphicsTextItem) InheritInputMethodEvent(f func(event *qtgui.QInputMethodEvent/*777 QInputMethodEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "inputMethodEvent", f)
// }

// void hoverEnterEvent(QGraphicsSceneHoverEvent *)
// func (this *QGraphicsTextItem) InheritHoverEnterEvent(f func(event *QGraphicsSceneHoverEvent/*777 QGraphicsSceneHoverEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "hoverEnterEvent", f)
// }

// void hoverMoveEvent(QGraphicsSceneHoverEvent *)
// func (this *QGraphicsTextItem) InheritHoverMoveEvent(f func(event *QGraphicsSceneHoverEvent/*777 QGraphicsSceneHoverEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "hoverMoveEvent", f)
// }

// void hoverLeaveEvent(QGraphicsSceneHoverEvent *)
// func (this *QGraphicsTextItem) InheritHoverLeaveEvent(f func(event *QGraphicsSceneHoverEvent/*777 QGraphicsSceneHoverEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "hoverLeaveEvent", f)
// }

// QVariant inputMethodQuery(Qt::InputMethodQuery)
// func (this *QGraphicsTextItem) InheritInputMethodQuery(f func(query int) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "inputMethodQuery", f)
// }

// bool supportsExtension(QGraphicsItem::Extension)
// func (this *QGraphicsTextItem) InheritSupportsExtension(f func(extension int) bool) {
//  qtrt.SetAllInheritCallback(this, "supportsExtension", f)
// }

// void setExtension(QGraphicsItem::Extension, const QVariant &)
// func (this *QGraphicsTextItem) InheritSetExtension(f func(extension int, variant *qtcore.QVariant) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "setExtension", f)
// }

// QVariant extension(const QVariant &)
// func (this *QGraphicsTextItem) InheritExtension(f func(variant *qtcore.QVariant) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "extension", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QGraphicsTextItem)=40
pub struct QGraphicsTextItem {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGraphicsTextItem_ITF interface {
//    QGraphicsTextItem_PTR() *QGraphicsTextItem
//}
//func (ptr *QGraphicsTextItem) QGraphicsTextItem_PTR() *QGraphicsTextItem { return ptr }

impl /*struct*/ QGraphicsTextItem {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGraphicsTextItem {
    return QGraphicsTextItem{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGraphicsTextItem {
//  type Target = QGraphicsTextItemBASE;
//
//  fn deref(&self) -> &QGraphicsTextItemBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGraphicsTextItemBASE> for QGraphicsTextItem {
//  fn as_ref(& self) -> & QGraphicsTextItemBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgraphicsitem.h:872
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QGraphicsTextItem {
  pub fn metaObject_0<RetType, T: QGraphicsTextItem_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QGraphicsTextItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QGraphicsTextItem10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:877
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QGraphicsTextItem(QGraphicsItem *)

/*

*/
// QGraphicsTextItem(QGraphicsItem *) ctx.fn_proto_cpp
impl /*struct*/ QGraphicsTextItem {
  pub fn QGraphicsTextItem_0<T: QGraphicsTextItem_QGraphicsTextItem_0>(value: T) -> QGraphicsTextItem {
    let rsthis = value.QGraphicsTextItem_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsTextItem_QGraphicsTextItem_0 {
  fn QGraphicsTextItem_0(self) -> QGraphicsTextItem;
}
// QGraphicsTextItem(QGraphicsItem *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGraphicsTextItem_QGraphicsTextItem_0 for (usize) {
  fn QGraphicsTextItem_0(self) -> QGraphicsTextItem {
    // unsafe{_ZN17QGraphicsTextItemC2EP13QGraphicsItem()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN17QGraphicsTextItemC2EP13QGraphicsItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGraphicsTextItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:878
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QGraphicsTextItem(const QString &, QGraphicsItem *)

/*

*/
// QGraphicsTextItem(const QString &, QGraphicsItem *) ctx.fn_proto_cpp
impl /*struct*/ QGraphicsTextItem {
  pub fn QGraphicsTextItem_1<T: QGraphicsTextItem_QGraphicsTextItem_1>(value: T) -> QGraphicsTextItem {
    let rsthis = value.QGraphicsTextItem_1();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsTextItem_QGraphicsTextItem_1 {
  fn QGraphicsTextItem_1(self) -> QGraphicsTextItem;
}
// QGraphicsTextItem(const QString &, QGraphicsItem *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGraphicsTextItem_QGraphicsTextItem_1 for (usize,usize) {
  fn QGraphicsTextItem_1(self) -> QGraphicsTextItem {
    // unsafe{_ZN17QGraphicsTextItemC2ERK7QStringP13QGraphicsItem()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN17QGraphicsTextItemC2ERK7QStringP13QGraphicsItem", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGraphicsTextItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:879
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QGraphicsTextItem()

/*

*/
pub fn DeleteQGraphicsTextItem(this :*mut QGraphicsTextItem) {
    // let rv = qtrt::InvokeQtFunc6("_ZN17QGraphicsTextItemD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 40)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgraphicsitem.h:881
// index:0
// Public Visibility=Default Availability=Available
// [8] QString toHtml() const

/*

*/
impl /*struct*/ QGraphicsTextItem {
  pub fn toHtml_0<RetType, T: QGraphicsTextItem_toHtml_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toHtml_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_toHtml_0<RetType> {
  fn toHtml_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_toHtml_0<usize> for () {
  fn toHtml_0(self , rsthis: & QGraphicsTextItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QGraphicsTextItem6toHtmlEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:882
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setHtml(const QString &)

/*

*/
impl /*struct*/ QGraphicsTextItem {
  pub fn setHtml_0<RetType, T: QGraphicsTextItem_setHtml_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHtml_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_setHtml_0<RetType> {
  fn setHtml_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_setHtml_0<(/*void*/)> for (usize) {
  fn setHtml_0(self , rsthis: & QGraphicsTextItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QGraphicsTextItem7setHtmlERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:884
// index:0
// Public Visibility=Default Availability=Available
// [8] QString toPlainText() const

/*

*/
impl /*struct*/ QGraphicsTextItem {
  pub fn toPlainText_0<RetType, T: QGraphicsTextItem_toPlainText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toPlainText_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_toPlainText_0<RetType> {
  fn toPlainText_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_toPlainText_0<usize> for () {
  fn toPlainText_0(self , rsthis: & QGraphicsTextItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QGraphicsTextItem11toPlainTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:885
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPlainText(const QString &)

/*

*/
impl /*struct*/ QGraphicsTextItem {
  pub fn setPlainText_0<RetType, T: QGraphicsTextItem_setPlainText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPlainText_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_setPlainText_0<RetType> {
  fn setPlainText_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_setPlainText_0<(/*void*/)> for (usize) {
  fn setPlainText_0(self , rsthis: & QGraphicsTextItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QGraphicsTextItem12setPlainTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:887
// index:0
// Public Visibility=Default Availability=Available
// [16] QFont font() const

/*

*/
impl /*struct*/ QGraphicsTextItem {
  pub fn font_0<RetType, T: QGraphicsTextItem_font_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.font_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_font_0<RetType> {
  fn font_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_font_0<usize> for () {
  fn font_0(self , rsthis: & QGraphicsTextItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QGraphicsTextItem4fontEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:888
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFont(const QFont &)

/*

*/
impl /*struct*/ QGraphicsTextItem {
  pub fn setFont_0<RetType, T: QGraphicsTextItem_setFont_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFont_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_setFont_0<RetType> {
  fn setFont_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_setFont_0<(/*void*/)> for (usize) {
  fn setFont_0(self , rsthis: & QGraphicsTextItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QGraphicsTextItem7setFontERK5QFont", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:890
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDefaultTextColor(const QColor &)

/*

*/
impl /*struct*/ QGraphicsTextItem {
  pub fn setDefaultTextColor_0<RetType, T: QGraphicsTextItem_setDefaultTextColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDefaultTextColor_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_setDefaultTextColor_0<RetType> {
  fn setDefaultTextColor_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_setDefaultTextColor_0<(/*void*/)> for (usize) {
  fn setDefaultTextColor_0(self , rsthis: & QGraphicsTextItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QGraphicsTextItem19setDefaultTextColorERK6QColor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:891
// index:0
// Public Visibility=Default Availability=Available
// [16] QColor defaultTextColor() const

/*

*/
impl /*struct*/ QGraphicsTextItem {
  pub fn defaultTextColor_0<RetType, T: QGraphicsTextItem_defaultTextColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.defaultTextColor_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_defaultTextColor_0<RetType> {
  fn defaultTextColor_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_defaultTextColor_0<usize> for () {
  fn defaultTextColor_0(self , rsthis: & QGraphicsTextItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QGraphicsTextItem16defaultTextColorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:893
// index:0
// Public virtual Visibility=Default Availability=Available
// [32] QRectF boundingRect() const

/*
This pure virtual function defines the outer bounds of the item as a rectangle; all painting must be restricted to inside an item's bounding rect. QGraphicsView uses this to determine whether the item requires redrawing.

Although the item's shape can be arbitrary, the bounding rect is always rectangular, and it is unaffected by the items' transformation.

If you want to change the item's bounding rectangle, you must first call prepareGeometryChange(). This notifies the scene of the imminent change, so that it can update its item geometry index; otherwise, the scene will be unaware of the item's new geometry, and the results are undefined (typically, rendering artifacts are left within the view).

Reimplement this function to let QGraphicsView determine what parts of the widget, if any, need to be redrawn.

Note: For shapes that paint an outline / stroke, it is important to include half the pen width in the bounding rect. It is not necessary to compensate for antialiasing, though.

Example:


  QRectF CircleItem::boundingRect() const
  {
      qreal penWidth = 1;
      return QRectF(-radius - penWidth / 2, -radius - penWidth / 2,
                    diameter + penWidth, diameter + penWidth);
  }



See also boundingRegion(), shape(), contains(), The Graphics View Coordinate System, and prepareGeometryChange().
*/
impl /*struct*/ QGraphicsTextItem {
  pub fn boundingRect_0<RetType, T: QGraphicsTextItem_boundingRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.boundingRect_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_boundingRect_0<RetType> {
  fn boundingRect_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_boundingRect_0<usize> for () {
  fn boundingRect_0(self , rsthis: & QGraphicsTextItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QGraphicsTextItem12boundingRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:894
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QPainterPath shape() const

/*
Returns the shape of this item as a QPainterPath in local coordinates. The shape is used for many things, including collision detection, hit tests, and for the QGraphicsScene::items() functions.

The default implementation calls boundingRect() to return a simple rectangular shape, but subclasses can reimplement this function to return a more accurate shape for non-rectangular items. For example, a round item may choose to return an elliptic shape for better collision detection. For example:


  QPainterPath RoundItem::shape() const
  {
      QPainterPath path;
      path.addEllipse(boundingRect());
      return path;
  }



The outline of a shape can vary depending on the width and style of the pen used when drawing. If you want to include this outline in the item's shape, you can create a shape from the stroke using QPainterPathStroker.

This function is called by the default implementations of contains() and collidesWithPath().

See also boundingRect(), contains(), prepareGeometryChange(), and QPainterPathStroker.
*/
impl /*struct*/ QGraphicsTextItem {
  pub fn shape_0<RetType, T: QGraphicsTextItem_shape_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.shape_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_shape_0<RetType> {
  fn shape_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_shape_0<usize> for () {
  fn shape_0(self , rsthis: & QGraphicsTextItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QGraphicsTextItem5shapeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:895
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool contains(const QPointF &) const

/*
Returns true if this item contains point, which is in local coordinates; otherwise, false is returned. It is most often called from QGraphicsView to determine what item is under the cursor, and for that reason, the implementation of this function should be as light-weight as possible.

By default, this function calls shape(), but you can reimplement it in a subclass to provide a (perhaps more efficient) implementation.

See also shape(), boundingRect(), and collidesWithPath().
*/
impl /*struct*/ QGraphicsTextItem {
  pub fn contains_0<RetType, T: QGraphicsTextItem_contains_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_contains_0<RetType> {
  fn contains_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_contains_0<bool> for (usize) {
  fn contains_0(self , rsthis: & QGraphicsTextItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QGraphicsTextItem8containsERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:897
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void paint(QPainter *, const QStyleOptionGraphicsItem *, QWidget *)

/*
This function, which is usually called by QGraphicsView, paints the contents of an item in local coordinates.

Reimplement this function in a QGraphicsItem subclass to provide the item's painting implementation, using painter. The option parameter provides style options for the item, such as its state, exposed area and its level-of-detail hints. The widget argument is optional. If provided, it points to the widget that is being painted on; otherwise, it is 0. For cached painting, widget is always 0.


  void RoundRectItem::paint(QPainter *painter,
                            const QStyleOptionGraphicsItem *option,
                            QWidget *widget)
  {
      painter->drawRoundedRect(-10, -10, 20, 20, 5, 5);
  }



The painter's pen is 0-width by default, and its pen is initialized to the QPalette::Text brush from the paint device's palette. The brush is initialized to QPalette::Window.

Make sure to constrain all painting inside the boundaries of boundingRect() to avoid rendering artifacts (as QGraphicsView does not clip the painter for you). In particular, when QPainter renders the outline of a shape using an assigned QPen, half of the outline will be drawn outside, and half inside, the shape you're rendering (e.g., with a pen width of 2 units, you must draw outlines 1 unit inside boundingRect()). QGraphicsItem does not support use of cosmetic pens with a non-zero width.

All painting is done in local coordinates.

Note: It is mandatory that an item will always redraw itself in the exact same way, unless update() was called; otherwise visual artifacts may occur. In other words, two subsequent calls to paint() must always produce the same output, unless update() was called between them.

Note: Enabling caching for an item does not guarantee that paint() will be invoked only once by the Graphics View framework, even without any explicit call to update(). See the documentation of setCacheMode() for more details.

See also setCacheMode(), QPen::width(), Item Coordinates, and ItemUsesExtendedStyleOption.
*/
impl /*struct*/ QGraphicsTextItem {
  pub fn paint_0<RetType, T: QGraphicsTextItem_paint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paint_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_paint_0<RetType> {
  fn paint_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_paint_0<(/*void*/)> for (usize,usize,usize) {
  fn paint_0(self , rsthis: & QGraphicsTextItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QGraphicsTextItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:899
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool isObscuredBy(const QGraphicsItem *) const

/*
Returns true if this item's bounding rect is completely obscured by the opaque shape of item.

The base implementation maps item's opaqueArea() to this item's coordinate system, and then checks if this item's boundingRect() is fully contained within the mapped shape.

You can reimplement this function to provide a custom algorithm for determining whether this item is obscured by item.

See also opaqueArea() and isObscured().
*/
impl /*struct*/ QGraphicsTextItem {
  pub fn isObscuredBy_0<RetType, T: QGraphicsTextItem_isObscuredBy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isObscuredBy_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_isObscuredBy_0<RetType> {
  fn isObscuredBy_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_isObscuredBy_0<bool> for (usize) {
  fn isObscuredBy_0(self , rsthis: & QGraphicsTextItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QGraphicsTextItem12isObscuredByEPK13QGraphicsItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:900
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QPainterPath opaqueArea() const

/*
This virtual function returns a shape representing the area where this item is opaque. An area is opaque if it is filled using an opaque brush or color (i.e., not transparent).

This function is used by isObscuredBy(), which is called by underlying items to determine if they are obscured by this item.

The default implementation returns an empty QPainterPath, indicating that this item is completely transparent and does not obscure any other items.

See also isObscuredBy(), isObscured(), and shape().
*/
impl /*struct*/ QGraphicsTextItem {
  pub fn opaqueArea_0<RetType, T: QGraphicsTextItem_opaqueArea_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.opaqueArea_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_opaqueArea_0<RetType> {
  fn opaqueArea_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_opaqueArea_0<usize> for () {
  fn opaqueArea_0(self , rsthis: & QGraphicsTextItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QGraphicsTextItem10opaqueAreaEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:903
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int type() const

/*
Returns the type of an item as an int. All standard graphicsitem classes are associated with a unique value; see QGraphicsItem::Type. This type information is used by qgraphicsitem_cast() to distinguish between types.

The default implementation (in QGraphicsItem) returns UserType.

To enable use of qgraphicsitem_cast() with a custom item, reimplement this function and declare a Type enum value equal to your custom item's type. Custom items must return a value larger than or equal to UserType (65536).

For example:


  class CustomItem : public QGraphicsItem
  {
  public:
     enum { Type = UserType + 1 };

     int type() const
     {
         // Enable the use of qgraphicsitem_cast with this item.
         return Type;
     }
     ...
  };



See also UserType.
*/
impl /*struct*/ QGraphicsTextItem {
  pub fn type__0<RetType, T: QGraphicsTextItem_type__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.type__0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_type__0<RetType> {
  fn type__0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_type__0<i32> for () {
  fn type__0(self , rsthis: & QGraphicsTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QGraphicsTextItem4typeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:905
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTextWidth(qreal)

/*

*/
impl /*struct*/ QGraphicsTextItem {
  pub fn setTextWidth_0<RetType, T: QGraphicsTextItem_setTextWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTextWidth_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_setTextWidth_0<RetType> {
  fn setTextWidth_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_setTextWidth_0<(/*void*/)> for (f64) {
  fn setTextWidth_0(self , rsthis: & QGraphicsTextItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN17QGraphicsTextItem12setTextWidthEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:906
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal textWidth() const

/*

*/
impl /*struct*/ QGraphicsTextItem {
  pub fn textWidth_0<RetType, T: QGraphicsTextItem_textWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textWidth_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_textWidth_0<RetType> {
  fn textWidth_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_textWidth_0<f64> for () {
  fn textWidth_0(self , rsthis: & QGraphicsTextItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QGraphicsTextItem9textWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:908
// index:0
// Public Visibility=Default Availability=Available
// [-2] void adjustSize()

/*

*/
impl /*struct*/ QGraphicsTextItem {
  pub fn adjustSize_0<RetType, T: QGraphicsTextItem_adjustSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.adjustSize_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_adjustSize_0<RetType> {
  fn adjustSize_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_adjustSize_0<(/*void*/)> for () {
  fn adjustSize_0(self , rsthis: & QGraphicsTextItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN17QGraphicsTextItem10adjustSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:910
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDocument(QTextDocument *)

/*

*/
impl /*struct*/ QGraphicsTextItem {
  pub fn setDocument_0<RetType, T: QGraphicsTextItem_setDocument_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDocument_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_setDocument_0<RetType> {
  fn setDocument_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_setDocument_0<(/*void*/)> for (usize) {
  fn setDocument_0(self , rsthis: & QGraphicsTextItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QGraphicsTextItem11setDocumentEP13QTextDocument", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:911
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextDocument * document() const

/*

*/
impl /*struct*/ QGraphicsTextItem {
  pub fn document_0<RetType, T: QGraphicsTextItem_document_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.document_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_document_0<RetType> {
  fn document_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_document_0<usize> for () {
  fn document_0(self , rsthis: & QGraphicsTextItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QGraphicsTextItem8documentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:913
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTextInteractionFlags(Qt::TextInteractionFlags)

/*

*/
impl /*struct*/ QGraphicsTextItem {
  pub fn setTextInteractionFlags_0<RetType, T: QGraphicsTextItem_setTextInteractionFlags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTextInteractionFlags_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_setTextInteractionFlags_0<RetType> {
  fn setTextInteractionFlags_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_setTextInteractionFlags_0<(/*void*/)> for (i32) {
  fn setTextInteractionFlags_0(self , rsthis: & QGraphicsTextItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN17QGraphicsTextItem23setTextInteractionFlagsE6QFlagsIN2Qt19TextInteractionFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:914
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::TextInteractionFlags textInteractionFlags() const

/*

*/
impl /*struct*/ QGraphicsTextItem {
  pub fn textInteractionFlags_0<RetType, T: QGraphicsTextItem_textInteractionFlags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textInteractionFlags_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_textInteractionFlags_0<RetType> {
  fn textInteractionFlags_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_textInteractionFlags_0<i32> for () {
  fn textInteractionFlags_0(self , rsthis: & QGraphicsTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QGraphicsTextItem20textInteractionFlagsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:916
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTabChangesFocus(bool)

/*

*/
impl /*struct*/ QGraphicsTextItem {
  pub fn setTabChangesFocus_0<RetType, T: QGraphicsTextItem_setTabChangesFocus_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTabChangesFocus_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_setTabChangesFocus_0<RetType> {
  fn setTabChangesFocus_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_setTabChangesFocus_0<(/*void*/)> for (bool) {
  fn setTabChangesFocus_0(self , rsthis: & QGraphicsTextItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN17QGraphicsTextItem18setTabChangesFocusEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:917
// index:0
// Public Visibility=Default Availability=Available
// [1] bool tabChangesFocus() const

/*

*/
impl /*struct*/ QGraphicsTextItem {
  pub fn tabChangesFocus_0<RetType, T: QGraphicsTextItem_tabChangesFocus_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabChangesFocus_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_tabChangesFocus_0<RetType> {
  fn tabChangesFocus_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_tabChangesFocus_0<bool> for () {
  fn tabChangesFocus_0(self , rsthis: & QGraphicsTextItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QGraphicsTextItem15tabChangesFocusEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:919
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOpenExternalLinks(bool)

/*

*/
impl /*struct*/ QGraphicsTextItem {
  pub fn setOpenExternalLinks_0<RetType, T: QGraphicsTextItem_setOpenExternalLinks_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOpenExternalLinks_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_setOpenExternalLinks_0<RetType> {
  fn setOpenExternalLinks_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_setOpenExternalLinks_0<(/*void*/)> for (bool) {
  fn setOpenExternalLinks_0(self , rsthis: & QGraphicsTextItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN17QGraphicsTextItem20setOpenExternalLinksEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:920
// index:0
// Public Visibility=Default Availability=Available
// [1] bool openExternalLinks() const

/*

*/
impl /*struct*/ QGraphicsTextItem {
  pub fn openExternalLinks_0<RetType, T: QGraphicsTextItem_openExternalLinks_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.openExternalLinks_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_openExternalLinks_0<RetType> {
  fn openExternalLinks_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_openExternalLinks_0<bool> for () {
  fn openExternalLinks_0(self , rsthis: & QGraphicsTextItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QGraphicsTextItem17openExternalLinksEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:922
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTextCursor(const QTextCursor &)

/*

*/
impl /*struct*/ QGraphicsTextItem {
  pub fn setTextCursor_0<RetType, T: QGraphicsTextItem_setTextCursor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTextCursor_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_setTextCursor_0<RetType> {
  fn setTextCursor_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_setTextCursor_0<(/*void*/)> for (usize) {
  fn setTextCursor_0(self , rsthis: & QGraphicsTextItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QGraphicsTextItem13setTextCursorERK11QTextCursor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:923
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextCursor textCursor() const

/*

*/
impl /*struct*/ QGraphicsTextItem {
  pub fn textCursor_0<RetType, T: QGraphicsTextItem_textCursor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textCursor_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_textCursor_0<RetType> {
  fn textCursor_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_textCursor_0<usize> for () {
  fn textCursor_0(self , rsthis: & QGraphicsTextItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QGraphicsTextItem10textCursorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:926
// index:0
// Public Visibility=Default Availability=Available
// [-2] void linkActivated(const QString &)

/*

*/
impl /*struct*/ QGraphicsTextItem {
  pub fn linkActivated_0<RetType, T: QGraphicsTextItem_linkActivated_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.linkActivated_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_linkActivated_0<RetType> {
  fn linkActivated_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_linkActivated_0<(/*void*/)> for (usize) {
  fn linkActivated_0(self , rsthis: & QGraphicsTextItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QGraphicsTextItem13linkActivatedERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:927
// index:0
// Public Visibility=Default Availability=Available
// [-2] void linkHovered(const QString &)

/*

*/
impl /*struct*/ QGraphicsTextItem {
  pub fn linkHovered_0<RetType, T: QGraphicsTextItem_linkHovered_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.linkHovered_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_linkHovered_0<RetType> {
  fn linkHovered_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_linkHovered_0<(/*void*/)> for (usize) {
  fn linkHovered_0(self , rsthis: & QGraphicsTextItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QGraphicsTextItem11linkHoveredERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:930
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool sceneEvent(QEvent *)

/*
This virtual function receives events to this item. Reimplement this function to intercept events before they are dispatched to the specialized event handlers contextMenuEvent(), focusInEvent(), focusOutEvent(), hoverEnterEvent(), hoverMoveEvent(), hoverLeaveEvent(), keyPressEvent(), keyReleaseEvent(), mousePressEvent(), mouseReleaseEvent(), mouseMoveEvent(), and mouseDoubleClickEvent().

Returns true if the event was recognized and handled; otherwise, (e.g., if the event type was not recognized,) false is returned.

event is the intercepted event.
*/
impl /*struct*/ QGraphicsTextItem {
  pub fn sceneEvent_0<RetType, T: QGraphicsTextItem_sceneEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sceneEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_sceneEvent_0<RetType> {
  fn sceneEvent_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_sceneEvent_0<bool> for (usize) {
  fn sceneEvent_0(self , rsthis: & QGraphicsTextItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN17QGraphicsTextItem10sceneEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:931
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mousePressEvent(QGraphicsSceneMouseEvent *)

/*
This event handler, for event event, can be reimplemented to receive mouse press events for this item. Mouse press events are only delivered to items that accept the mouse button that is pressed. By default, an item accepts all mouse buttons, but you can change this by calling setAcceptedMouseButtons().

The mouse press event decides which item should become the mouse grabber (see QGraphicsScene::mouseGrabberItem()). If you do not reimplement this function, the press event will propagate to any topmost item beneath this item, and no other mouse events will be delivered to this item.

If you do reimplement this function, event will by default be accepted (see QEvent::accept()), and this item is then the mouse grabber. This allows the item to receive future move, release and doubleclick events. If you call QEvent::ignore() on event, this item will lose the mouse grab, and event will propagate to any topmost item beneath. No further mouse events will be delivered to this item unless a new mouse press event is received.

The default implementation handles basic item interaction, such as selection and moving. If you want to keep the base implementation when reimplementing this function, call QGraphicsItem::mousePressEvent() in your reimplementation.

The event is QEvent::ignore()d for items that are neither movable nor selectable.

See also mouseMoveEvent(), mouseReleaseEvent(), mouseDoubleClickEvent(), and sceneEvent().
*/
impl /*struct*/ QGraphicsTextItem {
  pub fn mousePressEvent_0<RetType, T: QGraphicsTextItem_mousePressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mousePressEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_mousePressEvent_0<RetType> {
  fn mousePressEvent_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_mousePressEvent_0<(/*void*/)> for (usize) {
  fn mousePressEvent_0(self , rsthis: & QGraphicsTextItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QGraphicsTextItem15mousePressEventEP24QGraphicsSceneMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:932
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseMoveEvent(QGraphicsSceneMouseEvent *)

/*
This event handler, for event event, can be reimplemented to receive mouse move events for this item. If you do receive this event, you can be certain that this item also received a mouse press event, and that this item is the current mouse grabber.

Calling QEvent::ignore() or QEvent::accept() on event has no effect.

The default implementation handles basic item interaction, such as selection and moving. If you want to keep the base implementation when reimplementing this function, call QGraphicsItem::mouseMoveEvent() in your reimplementation.

Please note that mousePressEvent() decides which graphics item it is that receives mouse events. See the mousePressEvent() description for details.

See also mousePressEvent(), mouseReleaseEvent(), mouseDoubleClickEvent(), and sceneEvent().
*/
impl /*struct*/ QGraphicsTextItem {
  pub fn mouseMoveEvent_0<RetType, T: QGraphicsTextItem_mouseMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseMoveEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_mouseMoveEvent_0<RetType> {
  fn mouseMoveEvent_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_mouseMoveEvent_0<(/*void*/)> for (usize) {
  fn mouseMoveEvent_0(self , rsthis: & QGraphicsTextItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QGraphicsTextItem14mouseMoveEventEP24QGraphicsSceneMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:933
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseReleaseEvent(QGraphicsSceneMouseEvent *)

/*
This event handler, for event event, can be reimplemented to receive mouse release events for this item.

Calling QEvent::ignore() or QEvent::accept() on event has no effect.

The default implementation handles basic item interaction, such as selection and moving. If you want to keep the base implementation when reimplementing this function, call QGraphicsItem::mouseReleaseEvent() in your reimplementation.

Please note that mousePressEvent() decides which graphics item it is that receives mouse events. See the mousePressEvent() description for details.

See also mousePressEvent(), mouseMoveEvent(), mouseDoubleClickEvent(), and sceneEvent().
*/
impl /*struct*/ QGraphicsTextItem {
  pub fn mouseReleaseEvent_0<RetType, T: QGraphicsTextItem_mouseReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_mouseReleaseEvent_0<RetType> {
  fn mouseReleaseEvent_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_mouseReleaseEvent_0<(/*void*/)> for (usize) {
  fn mouseReleaseEvent_0(self , rsthis: & QGraphicsTextItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QGraphicsTextItem17mouseReleaseEventEP24QGraphicsSceneMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:934
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseDoubleClickEvent(QGraphicsSceneMouseEvent *)

/*
This event handler, for event event, can be reimplemented to receive mouse doubleclick events for this item.

When doubleclicking an item, the item will first receive a mouse press event, followed by a release event (i.e., a click), then a doubleclick event, and finally a release event.

Calling QEvent::ignore() or QEvent::accept() on event has no effect.

The default implementation calls mousePressEvent(). If you want to keep the base implementation when reimplementing this function, call QGraphicsItem::mouseDoubleClickEvent() in your reimplementation.

Note that an item will not receive double click events if it is neither selectable nor movable (single mouse clicks are ignored in this case, and that stops the generation of double clicks).

See also mousePressEvent(), mouseMoveEvent(), mouseReleaseEvent(), and sceneEvent().
*/
impl /*struct*/ QGraphicsTextItem {
  pub fn mouseDoubleClickEvent_0<RetType, T: QGraphicsTextItem_mouseDoubleClickEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseDoubleClickEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_mouseDoubleClickEvent_0<RetType> {
  fn mouseDoubleClickEvent_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_mouseDoubleClickEvent_0<(/*void*/)> for (usize) {
  fn mouseDoubleClickEvent_0(self , rsthis: & QGraphicsTextItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QGraphicsTextItem21mouseDoubleClickEventEP24QGraphicsSceneMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:935
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void contextMenuEvent(QGraphicsSceneContextMenuEvent *)

/*
This event handler can be reimplemented in a subclass to process context menu events. The event parameter contains details about the event to be handled.

If you ignore the event (i.e., by calling QEvent::ignore()), event will propagate to any item beneath this item. If no items accept the event, it will be ignored by the scene and propagate to the view.

It's common to open a QMenu in response to receiving a context menu event. Example:


  void CustomItem::contextMenuEvent(QGraphicsSceneContextMenuEvent *event)
  {
      QMenu menu;
      QAction *removeAction = menu.addAction("Remove");
      QAction *markAction = menu.addAction("Mark");
      QAction *selectedAction = menu.exec(event->screenPos());
      // ...
  }



The default implementation ignores the event.

See also sceneEvent().
*/
impl /*struct*/ QGraphicsTextItem {
  pub fn contextMenuEvent_0<RetType, T: QGraphicsTextItem_contextMenuEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contextMenuEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_contextMenuEvent_0<RetType> {
  fn contextMenuEvent_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_contextMenuEvent_0<(/*void*/)> for (usize) {
  fn contextMenuEvent_0(self , rsthis: & QGraphicsTextItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QGraphicsTextItem16contextMenuEventEP30QGraphicsSceneContextMenuEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:936
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyPressEvent(QKeyEvent *)

/*
This event handler, for event event, can be reimplemented to receive key press events for this item. The default implementation ignores the event. If you reimplement this handler, the event will by default be accepted.

Note that key events are only received for items that set the ItemIsFocusable flag, and that have keyboard input focus.

See also keyReleaseEvent(), setFocus(), QGraphicsScene::setFocusItem(), and sceneEvent().
*/
impl /*struct*/ QGraphicsTextItem {
  pub fn keyPressEvent_0<RetType, T: QGraphicsTextItem_keyPressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyPressEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_keyPressEvent_0<RetType> {
  fn keyPressEvent_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_keyPressEvent_0<(/*void*/)> for (usize) {
  fn keyPressEvent_0(self , rsthis: & QGraphicsTextItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QGraphicsTextItem13keyPressEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:937
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyReleaseEvent(QKeyEvent *)

/*
This event handler, for event event, can be reimplemented to receive key release events for this item. The default implementation ignores the event. If you reimplement this handler, the event will by default be accepted.

Note that key events are only received for items that set the ItemIsFocusable flag, and that have keyboard input focus.

See also keyPressEvent(), setFocus(), QGraphicsScene::setFocusItem(), and sceneEvent().
*/
impl /*struct*/ QGraphicsTextItem {
  pub fn keyReleaseEvent_0<RetType, T: QGraphicsTextItem_keyReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_keyReleaseEvent_0<RetType> {
  fn keyReleaseEvent_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_keyReleaseEvent_0<(/*void*/)> for (usize) {
  fn keyReleaseEvent_0(self , rsthis: & QGraphicsTextItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QGraphicsTextItem15keyReleaseEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:938
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusInEvent(QFocusEvent *)

/*
This event handler, for event event, can be reimplemented to receive focus in events for this item. The default implementation calls ensureVisible().

See also focusOutEvent(), sceneEvent(), and setFocus().
*/
impl /*struct*/ QGraphicsTextItem {
  pub fn focusInEvent_0<RetType, T: QGraphicsTextItem_focusInEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusInEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_focusInEvent_0<RetType> {
  fn focusInEvent_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_focusInEvent_0<(/*void*/)> for (usize) {
  fn focusInEvent_0(self , rsthis: & QGraphicsTextItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QGraphicsTextItem12focusInEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:939
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusOutEvent(QFocusEvent *)

/*
This event handler, for event event, can be reimplemented to receive focus out events for this item. The default implementation does nothing.

See also focusInEvent(), sceneEvent(), and setFocus().
*/
impl /*struct*/ QGraphicsTextItem {
  pub fn focusOutEvent_0<RetType, T: QGraphicsTextItem_focusOutEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusOutEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_focusOutEvent_0<RetType> {
  fn focusOutEvent_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_focusOutEvent_0<(/*void*/)> for (usize) {
  fn focusOutEvent_0(self , rsthis: & QGraphicsTextItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QGraphicsTextItem13focusOutEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:940
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dragEnterEvent(QGraphicsSceneDragDropEvent *)

/*
This event handler, for event event, can be reimplemented to receive drag enter events for this item. Drag enter events are generated as the cursor enters the item's area.

By accepting the event (i.e., by calling QEvent::accept()), the item will accept drop events, in addition to receiving drag move and drag leave. Otherwise, the event will be ignored and propagate to the item beneath. If the event is accepted, the item will receive a drag move event before control goes back to the event loop.

A common implementation of dragEnterEvent accepts or ignores event depending on the associated mime data in event. Example:


  CustomItem::CustomItem()
  {
      setAcceptDrops(true);
      ...
  }

  void CustomItem::dragEnterEvent(QGraphicsSceneDragDropEvent *event)
  {
      event->setAccepted(event->mimeData()->hasFormat("text/plain"));
  }



Items do not receive drag and drop events by default; to enable this feature, call setAcceptDrops(true).

The default implementation does nothing.

See also dropEvent(), dragMoveEvent(), and dragLeaveEvent().
*/
impl /*struct*/ QGraphicsTextItem {
  pub fn dragEnterEvent_0<RetType, T: QGraphicsTextItem_dragEnterEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragEnterEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_dragEnterEvent_0<RetType> {
  fn dragEnterEvent_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_dragEnterEvent_0<(/*void*/)> for (usize) {
  fn dragEnterEvent_0(self , rsthis: & QGraphicsTextItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QGraphicsTextItem14dragEnterEventEP27QGraphicsSceneDragDropEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:941
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dragLeaveEvent(QGraphicsSceneDragDropEvent *)

/*
This event handler, for event event, can be reimplemented to receive drag leave events for this item. Drag leave events are generated as the cursor leaves the item's area. Most often you will not need to reimplement this function, but it can be useful for resetting state in your item (e.g., highlighting).

Calling QEvent::ignore() or QEvent::accept() on event has no effect.

Items do not receive drag and drop events by default; to enable this feature, call setAcceptDrops(true).

The default implementation does nothing.

See also dragEnterEvent(), dropEvent(), and dragMoveEvent().
*/
impl /*struct*/ QGraphicsTextItem {
  pub fn dragLeaveEvent_0<RetType, T: QGraphicsTextItem_dragLeaveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragLeaveEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_dragLeaveEvent_0<RetType> {
  fn dragLeaveEvent_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_dragLeaveEvent_0<(/*void*/)> for (usize) {
  fn dragLeaveEvent_0(self , rsthis: & QGraphicsTextItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QGraphicsTextItem14dragLeaveEventEP27QGraphicsSceneDragDropEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:942
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dragMoveEvent(QGraphicsSceneDragDropEvent *)

/*
This event handler, for event event, can be reimplemented to receive drag move events for this item. Drag move events are generated as the cursor moves around inside the item's area. Most often you will not need to reimplement this function; it is used to indicate that only parts of the item can accept drops.

Calling QEvent::ignore() or QEvent::accept() on event toggles whether or not the item will accept drops at the position from the event. By default, event is accepted, indicating that the item allows drops at the specified position.

Items do not receive drag and drop events by default; to enable this feature, call setAcceptDrops(true).

The default implementation does nothing.

See also dropEvent(), dragEnterEvent(), and dragLeaveEvent().
*/
impl /*struct*/ QGraphicsTextItem {
  pub fn dragMoveEvent_0<RetType, T: QGraphicsTextItem_dragMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragMoveEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_dragMoveEvent_0<RetType> {
  fn dragMoveEvent_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_dragMoveEvent_0<(/*void*/)> for (usize) {
  fn dragMoveEvent_0(self , rsthis: & QGraphicsTextItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QGraphicsTextItem13dragMoveEventEP27QGraphicsSceneDragDropEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:943
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dropEvent(QGraphicsSceneDragDropEvent *)

/*
This event handler, for event event, can be reimplemented to receive drop events for this item. Items can only receive drop events if the last drag move event was accepted.

Calling QEvent::ignore() or QEvent::accept() on event has no effect.

Items do not receive drag and drop events by default; to enable this feature, call setAcceptDrops(true).

The default implementation does nothing.

See also dragEnterEvent(), dragMoveEvent(), and dragLeaveEvent().
*/
impl /*struct*/ QGraphicsTextItem {
  pub fn dropEvent_0<RetType, T: QGraphicsTextItem_dropEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dropEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_dropEvent_0<RetType> {
  fn dropEvent_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_dropEvent_0<(/*void*/)> for (usize) {
  fn dropEvent_0(self , rsthis: & QGraphicsTextItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QGraphicsTextItem9dropEventEP27QGraphicsSceneDragDropEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:944
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void inputMethodEvent(QInputMethodEvent *)

/*
This event handler, for event event, can be reimplemented to receive input method events for this item. The default implementation ignores the event.

See also inputMethodQuery() and sceneEvent().
*/
impl /*struct*/ QGraphicsTextItem {
  pub fn inputMethodEvent_0<RetType, T: QGraphicsTextItem_inputMethodEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inputMethodEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_inputMethodEvent_0<RetType> {
  fn inputMethodEvent_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_inputMethodEvent_0<(/*void*/)> for (usize) {
  fn inputMethodEvent_0(self , rsthis: & QGraphicsTextItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QGraphicsTextItem16inputMethodEventEP17QInputMethodEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:945
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void hoverEnterEvent(QGraphicsSceneHoverEvent *)

/*
This event handler, for event event, can be reimplemented to receive hover enter events for this item. The default implementation calls update(); otherwise it does nothing.

Calling QEvent::ignore() or QEvent::accept() on event has no effect.

See also hoverMoveEvent(), hoverLeaveEvent(), sceneEvent(), and setAcceptHoverEvents().
*/
impl /*struct*/ QGraphicsTextItem {
  pub fn hoverEnterEvent_0<RetType, T: QGraphicsTextItem_hoverEnterEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hoverEnterEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_hoverEnterEvent_0<RetType> {
  fn hoverEnterEvent_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_hoverEnterEvent_0<(/*void*/)> for (usize) {
  fn hoverEnterEvent_0(self , rsthis: & QGraphicsTextItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QGraphicsTextItem15hoverEnterEventEP24QGraphicsSceneHoverEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:946
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void hoverMoveEvent(QGraphicsSceneHoverEvent *)

/*
This event handler, for event event, can be reimplemented to receive hover move events for this item. The default implementation does nothing.

Calling QEvent::ignore() or QEvent::accept() on event has no effect.

See also hoverEnterEvent(), hoverLeaveEvent(), sceneEvent(), and setAcceptHoverEvents().
*/
impl /*struct*/ QGraphicsTextItem {
  pub fn hoverMoveEvent_0<RetType, T: QGraphicsTextItem_hoverMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hoverMoveEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_hoverMoveEvent_0<RetType> {
  fn hoverMoveEvent_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_hoverMoveEvent_0<(/*void*/)> for (usize) {
  fn hoverMoveEvent_0(self , rsthis: & QGraphicsTextItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QGraphicsTextItem14hoverMoveEventEP24QGraphicsSceneHoverEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:947
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void hoverLeaveEvent(QGraphicsSceneHoverEvent *)

/*
This event handler, for event event, can be reimplemented to receive hover leave events for this item. The default implementation calls update(); otherwise it does nothing.

Calling QEvent::ignore() or QEvent::accept() on event has no effect.

See also hoverEnterEvent(), hoverMoveEvent(), sceneEvent(), and setAcceptHoverEvents().
*/
impl /*struct*/ QGraphicsTextItem {
  pub fn hoverLeaveEvent_0<RetType, T: QGraphicsTextItem_hoverLeaveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hoverLeaveEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_hoverLeaveEvent_0<RetType> {
  fn hoverLeaveEvent_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_hoverLeaveEvent_0<(/*void*/)> for (usize) {
  fn hoverLeaveEvent_0(self , rsthis: & QGraphicsTextItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QGraphicsTextItem15hoverLeaveEventEP24QGraphicsSceneHoverEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:949
// index:0
// Protected virtual Visibility=Default Availability=Available
// [16] QVariant inputMethodQuery(Qt::InputMethodQuery) const

/*
This method is only relevant for input items. It is used by the input method to query a set of properties of the item to be able to support complex input method operations, such as support for surrounding text and reconversions. query specifies which property is queried.

See also inputMethodEvent() and QInputMethodEvent.
*/
impl /*struct*/ QGraphicsTextItem {
  pub fn inputMethodQuery_0<RetType, T: QGraphicsTextItem_inputMethodQuery_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inputMethodQuery_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_inputMethodQuery_0<RetType> {
  fn inputMethodQuery_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_inputMethodQuery_0<usize> for (i32) {
  fn inputMethodQuery_0(self , rsthis: & QGraphicsTextItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QGraphicsTextItem16inputMethodQueryEN2Qt16InputMethodQueryE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:951
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool supportsExtension(QGraphicsItem::Extension) const

/*

*/
impl /*struct*/ QGraphicsTextItem {
  pub fn supportsExtension_0<RetType, T: QGraphicsTextItem_supportsExtension_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.supportsExtension_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_supportsExtension_0<RetType> {
  fn supportsExtension_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_supportsExtension_0<bool> for (i32) {
  fn supportsExtension_0(self , rsthis: & QGraphicsTextItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QGraphicsTextItem17supportsExtensionEN13QGraphicsItem9ExtensionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:952
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void setExtension(QGraphicsItem::Extension, const QVariant &)

/*

*/
impl /*struct*/ QGraphicsTextItem {
  pub fn setExtension_0<RetType, T: QGraphicsTextItem_setExtension_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setExtension_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_setExtension_0<RetType> {
  fn setExtension_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_setExtension_0<(/*void*/)> for (i32,usize) {
  fn setExtension_0(self , rsthis: & QGraphicsTextItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QGraphicsTextItem12setExtensionEN13QGraphicsItem9ExtensionERK8QVariant", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:953
// index:0
// Protected virtual Visibility=Default Availability=Available
// [16] QVariant extension(const QVariant &) const

/*

*/
impl /*struct*/ QGraphicsTextItem {
  pub fn extension_0<RetType, T: QGraphicsTextItem_extension_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.extension_0(self);
    // return 1;
  }
}
pub trait QGraphicsTextItem_extension_0<RetType> {
  fn extension_0(self , rsthis: & QGraphicsTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTextItem_extension_0<usize> for (usize) {
  fn extension_0(self , rsthis: & QGraphicsTextItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QGraphicsTextItem9extensionERK8QVariant", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


/*


*/
pub type QGraphicsTextItem__ = i32;
// 
pub const QGraphicsTextItem__Type :QGraphicsTextItem__ = 8;
pub fn QGraphicsTextItem_ItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QGraphicsTextItem", val);
}
pub fn QGraphicsTextItem_ItemName_s(val: i32) ->String {
  //var nilthis *QGraphicsTextItem
  //return nilthis.ItemName(val);
  return QGraphicsTextItem_ItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
