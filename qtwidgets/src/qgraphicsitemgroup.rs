

// mod ::widgets::QGraphicsItemGroup
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
// extern C begin: 17
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



/*

*/
#[derive(Default)] // class sizeof(QGraphicsItemGroup)=16
pub struct QGraphicsItemGroup {
  qbase: QGraphicsItem,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGraphicsItemGroup_ITF interface {
//    QGraphicsItem_ITF
//    QGraphicsItemGroup_PTR() *QGraphicsItemGroup
//}
//func (ptr *QGraphicsItemGroup) QGraphicsItemGroup_PTR() *QGraphicsItemGroup { return ptr }

impl /*struct*/ QGraphicsItemGroup {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGraphicsItemGroup {
    return QGraphicsItemGroup{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGraphicsItemGroup {
//  type Target = QGraphicsItemGroupBASE;
//
//  fn deref(&self) -> &QGraphicsItemGroupBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGraphicsItemGroupBASE> for QGraphicsItemGroup {
//  fn as_ref(& self) -> & QGraphicsItemGroupBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgraphicsitem.h:1004
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QGraphicsItemGroup(QGraphicsItem *)

/*

*/
// QGraphicsItemGroup(QGraphicsItem *) ctx.fn_proto_cpp
impl /*struct*/ QGraphicsItemGroup {
  pub fn QGraphicsItemGroup_0<T: QGraphicsItemGroup_QGraphicsItemGroup_0>(value: T) -> QGraphicsItemGroup {
    let rsthis = value.QGraphicsItemGroup_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsItemGroup_QGraphicsItemGroup_0 {
  fn QGraphicsItemGroup_0(self) -> QGraphicsItemGroup;
}
// QGraphicsItemGroup(QGraphicsItem *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGraphicsItemGroup_QGraphicsItemGroup_0 for (usize) {
  fn QGraphicsItemGroup_0(self) -> QGraphicsItemGroup {
    // unsafe{_ZN18QGraphicsItemGroupC2EP13QGraphicsItem()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QGraphicsItemGroupC2EP13QGraphicsItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGraphicsItemGroup{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:1005
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QGraphicsItemGroup()

/*

*/
pub fn DeleteQGraphicsItemGroup(this :*mut QGraphicsItemGroup) {
    // let rv = qtrt::InvokeQtFunc6("_ZN18QGraphicsItemGroupD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgraphicsitem.h:1007
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addToGroup(QGraphicsItem *)

/*

*/
impl /*struct*/ QGraphicsItemGroup {
  pub fn addToGroup_0<RetType, T: QGraphicsItemGroup_addToGroup_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addToGroup_0(self);
    // return 1;
  }
}
pub trait QGraphicsItemGroup_addToGroup_0<RetType> {
  fn addToGroup_0(self , rsthis: & QGraphicsItemGroup) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItemGroup_addToGroup_0<(/*void*/)> for (usize) {
  fn addToGroup_0(self , rsthis: & QGraphicsItemGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QGraphicsItemGroup10addToGroupEP13QGraphicsItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:1008
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeFromGroup(QGraphicsItem *)

/*

*/
impl /*struct*/ QGraphicsItemGroup {
  pub fn removeFromGroup_0<RetType, T: QGraphicsItemGroup_removeFromGroup_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeFromGroup_0(self);
    // return 1;
  }
}
pub trait QGraphicsItemGroup_removeFromGroup_0<RetType> {
  fn removeFromGroup_0(self , rsthis: & QGraphicsItemGroup) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItemGroup_removeFromGroup_0<(/*void*/)> for (usize) {
  fn removeFromGroup_0(self , rsthis: & QGraphicsItemGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QGraphicsItemGroup15removeFromGroupEP13QGraphicsItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:1010
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
impl /*struct*/ QGraphicsItemGroup {
  pub fn boundingRect_0<RetType, T: QGraphicsItemGroup_boundingRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.boundingRect_0(self);
    // return 1;
  }
}
pub trait QGraphicsItemGroup_boundingRect_0<RetType> {
  fn boundingRect_0(self , rsthis: & QGraphicsItemGroup) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItemGroup_boundingRect_0<usize> for () {
  fn boundingRect_0(self , rsthis: & QGraphicsItemGroup) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QGraphicsItemGroup12boundingRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:1011
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
impl /*struct*/ QGraphicsItemGroup {
  pub fn paint_0<RetType, T: QGraphicsItemGroup_paint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paint_0(self);
    // return 1;
  }
}
pub trait QGraphicsItemGroup_paint_0<RetType> {
  fn paint_0(self , rsthis: & QGraphicsItemGroup) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItemGroup_paint_0<(/*void*/)> for (usize,usize,usize) {
  fn paint_0(self , rsthis: & QGraphicsItemGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QGraphicsItemGroup5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:1013
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool isObscuredBy(const QGraphicsItem *) const

/*
Returns true if this item's bounding rect is completely obscured by the opaque shape of item.

The base implementation maps item's opaqueArea() to this item's coordinate system, and then checks if this item's boundingRect() is fully contained within the mapped shape.

You can reimplement this function to provide a custom algorithm for determining whether this item is obscured by item.

See also opaqueArea() and isObscured().
*/
impl /*struct*/ QGraphicsItemGroup {
  pub fn isObscuredBy_0<RetType, T: QGraphicsItemGroup_isObscuredBy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isObscuredBy_0(self);
    // return 1;
  }
}
pub trait QGraphicsItemGroup_isObscuredBy_0<RetType> {
  fn isObscuredBy_0(self , rsthis: & QGraphicsItemGroup) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItemGroup_isObscuredBy_0<bool> for (usize) {
  fn isObscuredBy_0(self , rsthis: & QGraphicsItemGroup) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QGraphicsItemGroup12isObscuredByEPK13QGraphicsItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:1014
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QPainterPath opaqueArea() const

/*
This virtual function returns a shape representing the area where this item is opaque. An area is opaque if it is filled using an opaque brush or color (i.e., not transparent).

This function is used by isObscuredBy(), which is called by underlying items to determine if they are obscured by this item.

The default implementation returns an empty QPainterPath, indicating that this item is completely transparent and does not obscure any other items.

See also isObscuredBy(), isObscured(), and shape().
*/
impl /*struct*/ QGraphicsItemGroup {
  pub fn opaqueArea_0<RetType, T: QGraphicsItemGroup_opaqueArea_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.opaqueArea_0(self);
    // return 1;
  }
}
pub trait QGraphicsItemGroup_opaqueArea_0<RetType> {
  fn opaqueArea_0(self , rsthis: & QGraphicsItemGroup) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItemGroup_opaqueArea_0<usize> for () {
  fn opaqueArea_0(self , rsthis: & QGraphicsItemGroup) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QGraphicsItemGroup10opaqueAreaEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitem.h:1017
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
impl /*struct*/ QGraphicsItemGroup {
  pub fn type__0<RetType, T: QGraphicsItemGroup_type__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.type__0(self);
    // return 1;
  }
}
pub trait QGraphicsItemGroup_type__0<RetType> {
  fn type__0(self , rsthis: & QGraphicsItemGroup) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItemGroup_type__0<i32> for () {
  fn type__0(self , rsthis: & QGraphicsItemGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QGraphicsItemGroup4typeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}


/*


*/
pub type QGraphicsItemGroup__ = i32;
// 
pub const QGraphicsItemGroup__Type :QGraphicsItemGroup__ = 10;
pub fn QGraphicsItemGroup_ItemName(val: i32) ->String {
  match val {
     QGraphicsItemGroup__Type => // 10
     {return String::from("Type");}
  _ => {return format!("{}", val);}
}
}
pub fn QGraphicsItemGroup_ItemName_s(val: i32) ->String {
  //var nilthis *QGraphicsItemGroup
  //return nilthis.ItemName(val);
  return QGraphicsItemGroup_ItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
