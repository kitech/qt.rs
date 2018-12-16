

// mod ::widgets::QGraphicsSimpleTextItem
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
// extern C begin: 56
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
// func (this *QGraphicsSimpleTextItem) InheritSupportsExtension(f func(extension int) bool) {
//  qtrt.SetAllInheritCallback(this, "supportsExtension", f)
// }

// void setExtension(QGraphicsItem::Extension, const QVariant &)
// func (this *QGraphicsSimpleTextItem) InheritSetExtension(f func(extension int, variant *qtcore.QVariant) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "setExtension", f)
// }

// QVariant extension(const QVariant &)
// func (this *QGraphicsSimpleTextItem) InheritExtension(f func(variant *qtcore.QVariant) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "extension", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QGraphicsSimpleTextItem)=16
pub struct QGraphicsSimpleTextItem {
  qbase: QAbstractGraphicsShapeItem,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGraphicsSimpleTextItem_ITF interface {
//    QAbstractGraphicsShapeItem_ITF
//    QGraphicsSimpleTextItem_PTR() *QGraphicsSimpleTextItem
//}
//func (ptr *QGraphicsSimpleTextItem) QGraphicsSimpleTextItem_PTR() *QGraphicsSimpleTextItem { return ptr }

impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGraphicsSimpleTextItem {
    return QGraphicsSimpleTextItem{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGraphicsSimpleTextItem {
//  type Target = QGraphicsSimpleTextItemBASE;
//
//  fn deref(&self) -> &QGraphicsSimpleTextItemBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGraphicsSimpleTextItemBASE> for QGraphicsSimpleTextItem {
//  fn as_ref(& self) -> & QGraphicsSimpleTextItemBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgraphicsitem.h:968
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QGraphicsSimpleTextItem(QGraphicsItem *)

/*

*/
// QGraphicsSimpleTextItem(QGraphicsItem *) ctx.fn_proto_cpp
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn QGraphicsSimpleTextItem_0<T: QGraphicsSimpleTextItem_QGraphicsSimpleTextItem_0>(value: T) -> QGraphicsSimpleTextItem {
    let rsthis = value.QGraphicsSimpleTextItem_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsSimpleTextItem_QGraphicsSimpleTextItem_0 {
  fn QGraphicsSimpleTextItem_0(self) -> QGraphicsSimpleTextItem;
}
// QGraphicsSimpleTextItem(QGraphicsItem *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGraphicsSimpleTextItem_QGraphicsSimpleTextItem_0 for (usize) {
  fn QGraphicsSimpleTextItem_0(self) -> QGraphicsSimpleTextItem {
    // unsafe{_ZN23QGraphicsSimpleTextItemC2EP13QGraphicsItem()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN23QGraphicsSimpleTextItemC2EP13QGraphicsItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGraphicsSimpleTextItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:969
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QGraphicsSimpleTextItem(const QString &, QGraphicsItem *)

/*

*/
// QGraphicsSimpleTextItem(const QString &, QGraphicsItem *) ctx.fn_proto_cpp
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn QGraphicsSimpleTextItem_1<T: QGraphicsSimpleTextItem_QGraphicsSimpleTextItem_1>(value: T) -> QGraphicsSimpleTextItem {
    let rsthis = value.QGraphicsSimpleTextItem_1();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsSimpleTextItem_QGraphicsSimpleTextItem_1 {
  fn QGraphicsSimpleTextItem_1(self) -> QGraphicsSimpleTextItem;
}
// QGraphicsSimpleTextItem(const QString &, QGraphicsItem *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGraphicsSimpleTextItem_QGraphicsSimpleTextItem_1 for (usize,usize) {
  fn QGraphicsSimpleTextItem_1(self) -> QGraphicsSimpleTextItem {
    // unsafe{_ZN23QGraphicsSimpleTextItemC2ERK7QStringP13QGraphicsItem()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN23QGraphicsSimpleTextItemC2ERK7QStringP13QGraphicsItem", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGraphicsSimpleTextItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:970
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QGraphicsSimpleTextItem()

/*

*/
pub fn DeleteQGraphicsSimpleTextItem(this :*mut QGraphicsSimpleTextItem) {
    // let rv = qtrt::InvokeQtFunc6("_ZN23QGraphicsSimpleTextItemD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgraphicsitem.h:972
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setText(const QString &)

/*

*/
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn setText_0<RetType, T: QGraphicsSimpleTextItem_setText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setText_0(self);
    // return 1;
  }
}
pub trait QGraphicsSimpleTextItem_setText_0<RetType> {
  fn setText_0(self , rsthis: & QGraphicsSimpleTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSimpleTextItem_setText_0<(/*void*/)> for (usize) {
  fn setText_0(self , rsthis: & QGraphicsSimpleTextItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN23QGraphicsSimpleTextItem7setTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:973
// index:0
// Public Visibility=Default Availability=Available
// [8] QString text() const

/*

*/
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn text_0<RetType, T: QGraphicsSimpleTextItem_text_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.text_0(self);
    // return 1;
  }
}
pub trait QGraphicsSimpleTextItem_text_0<RetType> {
  fn text_0(self , rsthis: & QGraphicsSimpleTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSimpleTextItem_text_0<usize> for () {
  fn text_0(self , rsthis: & QGraphicsSimpleTextItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QGraphicsSimpleTextItem4textEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:975
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFont(const QFont &)

/*

*/
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn setFont_0<RetType, T: QGraphicsSimpleTextItem_setFont_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFont_0(self);
    // return 1;
  }
}
pub trait QGraphicsSimpleTextItem_setFont_0<RetType> {
  fn setFont_0(self , rsthis: & QGraphicsSimpleTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSimpleTextItem_setFont_0<(/*void*/)> for (usize) {
  fn setFont_0(self , rsthis: & QGraphicsSimpleTextItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN23QGraphicsSimpleTextItem7setFontERK5QFont", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:976
// index:0
// Public Visibility=Default Availability=Available
// [16] QFont font() const

/*

*/
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn font_0<RetType, T: QGraphicsSimpleTextItem_font_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.font_0(self);
    // return 1;
  }
}
pub trait QGraphicsSimpleTextItem_font_0<RetType> {
  fn font_0(self , rsthis: & QGraphicsSimpleTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSimpleTextItem_font_0<usize> for () {
  fn font_0(self , rsthis: & QGraphicsSimpleTextItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QGraphicsSimpleTextItem4fontEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:978
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
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn boundingRect_0<RetType, T: QGraphicsSimpleTextItem_boundingRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.boundingRect_0(self);
    // return 1;
  }
}
pub trait QGraphicsSimpleTextItem_boundingRect_0<RetType> {
  fn boundingRect_0(self , rsthis: & QGraphicsSimpleTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSimpleTextItem_boundingRect_0<usize> for () {
  fn boundingRect_0(self , rsthis: & QGraphicsSimpleTextItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QGraphicsSimpleTextItem12boundingRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:979
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
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn shape_0<RetType, T: QGraphicsSimpleTextItem_shape_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.shape_0(self);
    // return 1;
  }
}
pub trait QGraphicsSimpleTextItem_shape_0<RetType> {
  fn shape_0(self , rsthis: & QGraphicsSimpleTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSimpleTextItem_shape_0<usize> for () {
  fn shape_0(self , rsthis: & QGraphicsSimpleTextItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QGraphicsSimpleTextItem5shapeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:980
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool contains(const QPointF &) const

/*
Returns true if this item contains point, which is in local coordinates; otherwise, false is returned. It is most often called from QGraphicsView to determine what item is under the cursor, and for that reason, the implementation of this function should be as light-weight as possible.

By default, this function calls shape(), but you can reimplement it in a subclass to provide a (perhaps more efficient) implementation.

See also shape(), boundingRect(), and collidesWithPath().
*/
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn contains_0<RetType, T: QGraphicsSimpleTextItem_contains_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_0(self);
    // return 1;
  }
}
pub trait QGraphicsSimpleTextItem_contains_0<RetType> {
  fn contains_0(self , rsthis: & QGraphicsSimpleTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSimpleTextItem_contains_0<bool> for (usize) {
  fn contains_0(self , rsthis: & QGraphicsSimpleTextItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QGraphicsSimpleTextItem8containsERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:982
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
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn paint_0<RetType, T: QGraphicsSimpleTextItem_paint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paint_0(self);
    // return 1;
  }
}
pub trait QGraphicsSimpleTextItem_paint_0<RetType> {
  fn paint_0(self , rsthis: & QGraphicsSimpleTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSimpleTextItem_paint_0<(/*void*/)> for (usize,usize,usize) {
  fn paint_0(self , rsthis: & QGraphicsSimpleTextItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN23QGraphicsSimpleTextItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:984
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool isObscuredBy(const QGraphicsItem *) const

/*
Returns true if this item's bounding rect is completely obscured by the opaque shape of item.

The base implementation maps item's opaqueArea() to this item's coordinate system, and then checks if this item's boundingRect() is fully contained within the mapped shape.

You can reimplement this function to provide a custom algorithm for determining whether this item is obscured by item.

See also opaqueArea() and isObscured().
*/
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn isObscuredBy_0<RetType, T: QGraphicsSimpleTextItem_isObscuredBy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isObscuredBy_0(self);
    // return 1;
  }
}
pub trait QGraphicsSimpleTextItem_isObscuredBy_0<RetType> {
  fn isObscuredBy_0(self , rsthis: & QGraphicsSimpleTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSimpleTextItem_isObscuredBy_0<bool> for (usize) {
  fn isObscuredBy_0(self , rsthis: & QGraphicsSimpleTextItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QGraphicsSimpleTextItem12isObscuredByEPK13QGraphicsItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:985
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QPainterPath opaqueArea() const

/*
This virtual function returns a shape representing the area where this item is opaque. An area is opaque if it is filled using an opaque brush or color (i.e., not transparent).

This function is used by isObscuredBy(), which is called by underlying items to determine if they are obscured by this item.

The default implementation returns an empty QPainterPath, indicating that this item is completely transparent and does not obscure any other items.

See also isObscuredBy(), isObscured(), and shape().
*/
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn opaqueArea_0<RetType, T: QGraphicsSimpleTextItem_opaqueArea_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.opaqueArea_0(self);
    // return 1;
  }
}
pub trait QGraphicsSimpleTextItem_opaqueArea_0<RetType> {
  fn opaqueArea_0(self , rsthis: & QGraphicsSimpleTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSimpleTextItem_opaqueArea_0<usize> for () {
  fn opaqueArea_0(self , rsthis: & QGraphicsSimpleTextItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QGraphicsSimpleTextItem10opaqueAreaEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:988
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
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn type__0<RetType, T: QGraphicsSimpleTextItem_type__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.type__0(self);
    // return 1;
  }
}
pub trait QGraphicsSimpleTextItem_type__0<RetType> {
  fn type__0(self , rsthis: & QGraphicsSimpleTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSimpleTextItem_type__0<i32> for () {
  fn type__0(self , rsthis: & QGraphicsSimpleTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QGraphicsSimpleTextItem4typeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:991
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool supportsExtension(QGraphicsItem::Extension) const

/*

*/
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn supportsExtension_0<RetType, T: QGraphicsSimpleTextItem_supportsExtension_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.supportsExtension_0(self);
    // return 1;
  }
}
pub trait QGraphicsSimpleTextItem_supportsExtension_0<RetType> {
  fn supportsExtension_0(self , rsthis: & QGraphicsSimpleTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSimpleTextItem_supportsExtension_0<bool> for (i32) {
  fn supportsExtension_0(self , rsthis: & QGraphicsSimpleTextItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QGraphicsSimpleTextItem17supportsExtensionEN13QGraphicsItem9ExtensionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:992
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void setExtension(QGraphicsItem::Extension, const QVariant &)

/*

*/
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn setExtension_0<RetType, T: QGraphicsSimpleTextItem_setExtension_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setExtension_0(self);
    // return 1;
  }
}
pub trait QGraphicsSimpleTextItem_setExtension_0<RetType> {
  fn setExtension_0(self , rsthis: & QGraphicsSimpleTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSimpleTextItem_setExtension_0<(/*void*/)> for (i32,usize) {
  fn setExtension_0(self , rsthis: & QGraphicsSimpleTextItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN23QGraphicsSimpleTextItem12setExtensionEN13QGraphicsItem9ExtensionERK8QVariant", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:993
// index:0
// Protected virtual Visibility=Default Availability=Available
// [16] QVariant extension(const QVariant &) const

/*

*/
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn extension_0<RetType, T: QGraphicsSimpleTextItem_extension_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.extension_0(self);
    // return 1;
  }
}
pub trait QGraphicsSimpleTextItem_extension_0<RetType> {
  fn extension_0(self , rsthis: & QGraphicsSimpleTextItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSimpleTextItem_extension_0<usize> for (usize) {
  fn extension_0(self , rsthis: & QGraphicsSimpleTextItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QGraphicsSimpleTextItem9extensionERK8QVariant", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


/*


*/
pub type QGraphicsSimpleTextItem__ = i32;
// 
pub const QGraphicsSimpleTextItem__Type :QGraphicsSimpleTextItem__ = 9;
pub fn QGraphicsSimpleTextItem_ItemName(val: i32) ->String {
  match val {
     QGraphicsSimpleTextItem__Type => // 9
     {return String::from("Type");}
  _ => {return format!("{}", val);}
}
}
pub fn QGraphicsSimpleTextItem_ItemName_s(val: i32) ->String {
  //var nilthis *QGraphicsSimpleTextItem
  //return nilthis.ItemName(val);
  return QGraphicsSimpleTextItem_ItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
