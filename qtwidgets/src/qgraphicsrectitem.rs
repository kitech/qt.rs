

// mod ::widgets::QGraphicsRectItem
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
// extern C begin: 15
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

// bool supportsExtension(QGraphicsItem::Extension)
// func (this *QGraphicsRectItem) InheritSupportsExtension(f func(extension int) bool) {
//  qtrt.SetAllInheritCallback(this, "supportsExtension", f)
// }

// void setExtension(QGraphicsItem::Extension, const QVariant &)
// func (this *QGraphicsRectItem) InheritSetExtension(f func(extension int, variant *qtcore.QVariant) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "setExtension", f)
// }

// QVariant extension(const QVariant &)
// func (this *QGraphicsRectItem) InheritExtension(f func(variant *qtcore.QVariant) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "extension", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QGraphicsRectItem)=16
pub struct QGraphicsRectItem {
  qbase: QAbstractGraphicsShapeItem,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGraphicsRectItem_ITF interface {
//    QAbstractGraphicsShapeItem_ITF
//    QGraphicsRectItem_PTR() *QGraphicsRectItem
//}
//func (ptr *QGraphicsRectItem) QGraphicsRectItem_PTR() *QGraphicsRectItem { return ptr }

impl /*struct*/ QGraphicsRectItem {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGraphicsRectItem {
    return QGraphicsRectItem{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGraphicsRectItem {
//  type Target = QGraphicsRectItemBASE;
//
//  fn deref(&self) -> &QGraphicsRectItemBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGraphicsRectItemBASE> for QGraphicsRectItem {
//  fn as_ref(& self) -> & QGraphicsRectItemBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgraphicsitem.h:661
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QGraphicsRectItem(QGraphicsItem *)

/*

*/
// QGraphicsRectItem(QGraphicsItem *) ctx.fn_proto_cpp
impl /*struct*/ QGraphicsRectItem {
  pub fn QGraphicsRectItem_0<T: QGraphicsRectItem_QGraphicsRectItem_0>(value: T) -> QGraphicsRectItem {
    let rsthis = value.QGraphicsRectItem_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsRectItem_QGraphicsRectItem_0 {
  fn QGraphicsRectItem_0(self) -> QGraphicsRectItem;
}
// QGraphicsRectItem(QGraphicsItem *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGraphicsRectItem_QGraphicsRectItem_0 for (usize) {
  fn QGraphicsRectItem_0(self) -> QGraphicsRectItem {
    // unsafe{_ZN17QGraphicsRectItemC2EP13QGraphicsItem()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN17QGraphicsRectItemC2EP13QGraphicsItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGraphicsRectItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:662
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QGraphicsRectItem(const QRectF &, QGraphicsItem *)

/*

*/
// QGraphicsRectItem(const QRectF &, QGraphicsItem *) ctx.fn_proto_cpp
impl /*struct*/ QGraphicsRectItem {
  pub fn QGraphicsRectItem_1<T: QGraphicsRectItem_QGraphicsRectItem_1>(value: T) -> QGraphicsRectItem {
    let rsthis = value.QGraphicsRectItem_1();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsRectItem_QGraphicsRectItem_1 {
  fn QGraphicsRectItem_1(self) -> QGraphicsRectItem;
}
// QGraphicsRectItem(const QRectF &, QGraphicsItem *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGraphicsRectItem_QGraphicsRectItem_1 for (usize,usize) {
  fn QGraphicsRectItem_1(self) -> QGraphicsRectItem {
    // unsafe{_ZN17QGraphicsRectItemC2ERK6QRectFP13QGraphicsItem()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN17QGraphicsRectItemC2ERK6QRectFP13QGraphicsItem", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGraphicsRectItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:663
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QGraphicsRectItem(qreal, qreal, qreal, qreal, QGraphicsItem *)

/*

*/
// QGraphicsRectItem(qreal, qreal, qreal, qreal, QGraphicsItem *) ctx.fn_proto_cpp
impl /*struct*/ QGraphicsRectItem {
  pub fn QGraphicsRectItem_2<T: QGraphicsRectItem_QGraphicsRectItem_2>(value: T) -> QGraphicsRectItem {
    let rsthis = value.QGraphicsRectItem_2();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsRectItem_QGraphicsRectItem_2 {
  fn QGraphicsRectItem_2(self) -> QGraphicsRectItem;
}
// QGraphicsRectItem(qreal, qreal, qreal, qreal, QGraphicsItem *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGraphicsRectItem_QGraphicsRectItem_2 for (f64,f64,f64,f64,usize) {
  fn QGraphicsRectItem_2(self) -> QGraphicsRectItem {
    // unsafe{_ZN17QGraphicsRectItemC2EddddP13QGraphicsItem()};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN17QGraphicsRectItemC2EddddP13QGraphicsItem", 5,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGraphicsRectItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:664
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QGraphicsRectItem()

/*

*/
pub fn DeleteQGraphicsRectItem(this :*mut QGraphicsRectItem) {
    // let rv = qtrt::InvokeQtFunc6("_ZN17QGraphicsRectItemD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgraphicsitem.h:666
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF rect() const

/*

*/
impl /*struct*/ QGraphicsRectItem {
  pub fn rect_0<RetType, T: QGraphicsRectItem_rect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rect_0(self);
    // return 1;
  }
}
pub trait QGraphicsRectItem_rect_0<RetType> {
  fn rect_0(self , rsthis: & QGraphicsRectItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsRectItem_rect_0<usize> for () {
  fn rect_0(self , rsthis: & QGraphicsRectItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QGraphicsRectItem4rectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:667
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRect(const QRectF &)

/*

*/
impl /*struct*/ QGraphicsRectItem {
  pub fn setRect_0<RetType, T: QGraphicsRectItem_setRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRect_0(self);
    // return 1;
  }
}
pub trait QGraphicsRectItem_setRect_0<RetType> {
  fn setRect_0(self , rsthis: & QGraphicsRectItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsRectItem_setRect_0<(/*void*/)> for (usize) {
  fn setRect_0(self , rsthis: & QGraphicsRectItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QGraphicsRectItem7setRectERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:668
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void setRect(qreal, qreal, qreal, qreal)

/*

*/
impl /*struct*/ QGraphicsRectItem {
  pub fn setRect_1<RetType, T: QGraphicsRectItem_setRect_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRect_1(self);
    // return 1;
  }
}
pub trait QGraphicsRectItem_setRect_1<RetType> {
  fn setRect_1(self , rsthis: & QGraphicsRectItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsRectItem_setRect_1<(/*void*/)> for (f64,f64,f64,f64) {
  fn setRect_1(self , rsthis: & QGraphicsRectItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN17QGraphicsRectItem7setRectEdddd", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:670
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
impl /*struct*/ QGraphicsRectItem {
  pub fn boundingRect_0<RetType, T: QGraphicsRectItem_boundingRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.boundingRect_0(self);
    // return 1;
  }
}
pub trait QGraphicsRectItem_boundingRect_0<RetType> {
  fn boundingRect_0(self , rsthis: & QGraphicsRectItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsRectItem_boundingRect_0<usize> for () {
  fn boundingRect_0(self , rsthis: & QGraphicsRectItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QGraphicsRectItem12boundingRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:671
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
impl /*struct*/ QGraphicsRectItem {
  pub fn shape_0<RetType, T: QGraphicsRectItem_shape_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.shape_0(self);
    // return 1;
  }
}
pub trait QGraphicsRectItem_shape_0<RetType> {
  fn shape_0(self , rsthis: & QGraphicsRectItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsRectItem_shape_0<usize> for () {
  fn shape_0(self , rsthis: & QGraphicsRectItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QGraphicsRectItem5shapeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:672
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool contains(const QPointF &) const

/*
Returns true if this item contains point, which is in local coordinates; otherwise, false is returned. It is most often called from QGraphicsView to determine what item is under the cursor, and for that reason, the implementation of this function should be as light-weight as possible.

By default, this function calls shape(), but you can reimplement it in a subclass to provide a (perhaps more efficient) implementation.

See also shape(), boundingRect(), and collidesWithPath().
*/
impl /*struct*/ QGraphicsRectItem {
  pub fn contains_0<RetType, T: QGraphicsRectItem_contains_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_0(self);
    // return 1;
  }
}
pub trait QGraphicsRectItem_contains_0<RetType> {
  fn contains_0(self , rsthis: & QGraphicsRectItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsRectItem_contains_0<bool> for (usize) {
  fn contains_0(self , rsthis: & QGraphicsRectItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QGraphicsRectItem8containsERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:674
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
impl /*struct*/ QGraphicsRectItem {
  pub fn paint_0<RetType, T: QGraphicsRectItem_paint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paint_0(self);
    // return 1;
  }
}
pub trait QGraphicsRectItem_paint_0<RetType> {
  fn paint_0(self , rsthis: & QGraphicsRectItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsRectItem_paint_0<(/*void*/)> for (usize,usize,usize) {
  fn paint_0(self , rsthis: & QGraphicsRectItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QGraphicsRectItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:676
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool isObscuredBy(const QGraphicsItem *) const

/*
Returns true if this item's bounding rect is completely obscured by the opaque shape of item.

The base implementation maps item's opaqueArea() to this item's coordinate system, and then checks if this item's boundingRect() is fully contained within the mapped shape.

You can reimplement this function to provide a custom algorithm for determining whether this item is obscured by item.

See also opaqueArea() and isObscured().
*/
impl /*struct*/ QGraphicsRectItem {
  pub fn isObscuredBy_0<RetType, T: QGraphicsRectItem_isObscuredBy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isObscuredBy_0(self);
    // return 1;
  }
}
pub trait QGraphicsRectItem_isObscuredBy_0<RetType> {
  fn isObscuredBy_0(self , rsthis: & QGraphicsRectItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsRectItem_isObscuredBy_0<bool> for (usize) {
  fn isObscuredBy_0(self , rsthis: & QGraphicsRectItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QGraphicsRectItem12isObscuredByEPK13QGraphicsItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:677
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QPainterPath opaqueArea() const

/*
This virtual function returns a shape representing the area where this item is opaque. An area is opaque if it is filled using an opaque brush or color (i.e., not transparent).

This function is used by isObscuredBy(), which is called by underlying items to determine if they are obscured by this item.

The default implementation returns an empty QPainterPath, indicating that this item is completely transparent and does not obscure any other items.

See also isObscuredBy(), isObscured(), and shape().
*/
impl /*struct*/ QGraphicsRectItem {
  pub fn opaqueArea_0<RetType, T: QGraphicsRectItem_opaqueArea_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.opaqueArea_0(self);
    // return 1;
  }
}
pub trait QGraphicsRectItem_opaqueArea_0<RetType> {
  fn opaqueArea_0(self , rsthis: & QGraphicsRectItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsRectItem_opaqueArea_0<usize> for () {
  fn opaqueArea_0(self , rsthis: & QGraphicsRectItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QGraphicsRectItem10opaqueAreaEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:680
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
impl /*struct*/ QGraphicsRectItem {
  pub fn type__0<RetType, T: QGraphicsRectItem_type__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.type__0(self);
    // return 1;
  }
}
pub trait QGraphicsRectItem_type__0<RetType> {
  fn type__0(self , rsthis: & QGraphicsRectItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsRectItem_type__0<i32> for () {
  fn type__0(self , rsthis: & QGraphicsRectItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QGraphicsRectItem4typeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:683
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool supportsExtension(QGraphicsItem::Extension) const

/*

*/
impl /*struct*/ QGraphicsRectItem {
  pub fn supportsExtension_0<RetType, T: QGraphicsRectItem_supportsExtension_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.supportsExtension_0(self);
    // return 1;
  }
}
pub trait QGraphicsRectItem_supportsExtension_0<RetType> {
  fn supportsExtension_0(self , rsthis: & QGraphicsRectItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsRectItem_supportsExtension_0<bool> for (i32) {
  fn supportsExtension_0(self , rsthis: & QGraphicsRectItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QGraphicsRectItem17supportsExtensionEN13QGraphicsItem9ExtensionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:684
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void setExtension(QGraphicsItem::Extension, const QVariant &)

/*

*/
impl /*struct*/ QGraphicsRectItem {
  pub fn setExtension_0<RetType, T: QGraphicsRectItem_setExtension_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setExtension_0(self);
    // return 1;
  }
}
pub trait QGraphicsRectItem_setExtension_0<RetType> {
  fn setExtension_0(self , rsthis: & QGraphicsRectItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsRectItem_setExtension_0<(/*void*/)> for (i32,usize) {
  fn setExtension_0(self , rsthis: & QGraphicsRectItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QGraphicsRectItem12setExtensionEN13QGraphicsItem9ExtensionERK8QVariant", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:685
// index:0
// Protected virtual Visibility=Default Availability=Available
// [16] QVariant extension(const QVariant &) const

/*

*/
impl /*struct*/ QGraphicsRectItem {
  pub fn extension_0<RetType, T: QGraphicsRectItem_extension_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.extension_0(self);
    // return 1;
  }
}
pub trait QGraphicsRectItem_extension_0<RetType> {
  fn extension_0(self , rsthis: & QGraphicsRectItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsRectItem_extension_0<usize> for (usize) {
  fn extension_0(self , rsthis: & QGraphicsRectItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QGraphicsRectItem9extensionERK8QVariant", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


/*


*/
pub type QGraphicsRectItem__ = i32;
// 
pub const QGraphicsRectItem__Type :QGraphicsRectItem__ = 3;
pub fn QGraphicsRectItem_ItemName(val: i32) ->String {
  match val {
     QGraphicsRectItem__Type => // 3
     {return String::from("Type");}
  _ => {return format!("{}", val);}
}
}
pub fn QGraphicsRectItem_ItemName_s(val: i32) ->String {
  //var nilthis *QGraphicsRectItem
  //return nilthis.ItemName(val);
  return QGraphicsRectItem_ItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
