

// mod ::widgets::QGraphicsAnchorLayout
// package qtwidgets
// /usr/include/qt/QtWidgets/qgraphicsanchorlayout.h
// #include <qgraphicsanchorlayout.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 7
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

// QSizeF sizeHint(Qt::SizeHint, const QSizeF &)
// func (this *QGraphicsAnchorLayout) InheritSizeHint(f func(which int, constraint *qtcore.QSizeF) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "sizeHint", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QGraphicsAnchorLayout)=16
pub struct QGraphicsAnchorLayout {
  qbase: QGraphicsLayout,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGraphicsAnchorLayout_ITF interface {
//    QGraphicsLayout_ITF
//    QGraphicsAnchorLayout_PTR() *QGraphicsAnchorLayout
//}
//func (ptr *QGraphicsAnchorLayout) QGraphicsAnchorLayout_PTR() *QGraphicsAnchorLayout { return ptr }

impl /*struct*/ QGraphicsAnchorLayout {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGraphicsAnchorLayout {
    return QGraphicsAnchorLayout{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGraphicsAnchorLayout {
//  type Target = QGraphicsAnchorLayoutBASE;
//
//  fn deref(&self) -> &QGraphicsAnchorLayoutBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGraphicsAnchorLayoutBASE> for QGraphicsAnchorLayout {
//  fn as_ref(& self) -> & QGraphicsAnchorLayoutBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgraphicsanchorlayout.h:79
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QGraphicsAnchorLayout(QGraphicsLayoutItem *)

/*
Constructs a QGraphicsAnchorLayout instance. parent is passed to QGraphicsLayout's constructor.
*/
// QGraphicsAnchorLayout(QGraphicsLayoutItem *) ctx.fn_proto_cpp
impl /*struct*/ QGraphicsAnchorLayout {
  pub fn QGraphicsAnchorLayout_0<T: QGraphicsAnchorLayout_QGraphicsAnchorLayout_0>(value: T) -> QGraphicsAnchorLayout {
    let rsthis = value.QGraphicsAnchorLayout_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsAnchorLayout_QGraphicsAnchorLayout_0 {
  fn QGraphicsAnchorLayout_0(self) -> QGraphicsAnchorLayout;
}
// QGraphicsAnchorLayout(QGraphicsLayoutItem *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGraphicsAnchorLayout_QGraphicsAnchorLayout_0 for (usize) {
  fn QGraphicsAnchorLayout_0(self) -> QGraphicsAnchorLayout {
    // unsafe{_ZN21QGraphicsAnchorLayoutC2EP19QGraphicsLayoutItem()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN21QGraphicsAnchorLayoutC2EP19QGraphicsLayoutItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGraphicsAnchorLayout{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsanchorlayout.h:80
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QGraphicsAnchorLayout()

/*

*/
pub fn DeleteQGraphicsAnchorLayout(this :*mut QGraphicsAnchorLayout) {
    // let rv = qtrt::InvokeQtFunc6("_ZN21QGraphicsAnchorLayoutD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgraphicsanchorlayout.h:82
// index:0
// Public Visibility=Default Availability=Available
// [8] QGraphicsAnchor * addAnchor(QGraphicsLayoutItem *, Qt::AnchorPoint, QGraphicsLayoutItem *, Qt::AnchorPoint)

/*
Creates an anchor between the edge firstEdge of item firstItem and the edge secondEdge of item secondItem. The spacing of the anchor is picked up from the style. Anchors between a layout edge and an item edge will have a size of 0. If there is already an anchor between the edges, the new anchor will replace the old one.

firstItem and secondItem are automatically added to the layout if they are not part of the layout. This means that count() can increase by up to 2.

The spacing an anchor will get depends on the type of anchor. For instance, anchors from the Right edge of one item to the Left edge of another (or vice versa) will use the default horizontal spacing. The same behaviour applies to Bottom to Top anchors, (but they will use the default vertical spacing). For all other anchor combinations, the spacing will be 0. All anchoring functions will follow this rule.

The spacing can also be set manually by using QGraphicsAnchor::setSpacing() method.

Calling this function where firstItem or secondItem are ancestors of the layout have undefined behaviour.

See also addAnchors() and addCornerAnchors().
*/
impl /*struct*/ QGraphicsAnchorLayout {
  pub fn addAnchor_0<RetType, T: QGraphicsAnchorLayout_addAnchor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addAnchor_0(self);
    // return 1;
  }
}
pub trait QGraphicsAnchorLayout_addAnchor_0<RetType> {
  fn addAnchor_0(self , rsthis: & QGraphicsAnchorLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsAnchorLayout_addAnchor_0<usize> for (usize,i32,usize,i32) {
  fn addAnchor_0(self , rsthis: & QGraphicsAnchorLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN21QGraphicsAnchorLayout9addAnchorEP19QGraphicsLayoutItemN2Qt11AnchorPointES1_S3_", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsanchorlayout.h:84
// index:0
// Public Visibility=Default Availability=Available
// [8] QGraphicsAnchor * anchor(QGraphicsLayoutItem *, Qt::AnchorPoint, QGraphicsLayoutItem *, Qt::AnchorPoint)

/*
Returns the anchor between the anchor points defined by firstItem and firstEdge and secondItem and secondEdge. If there is no such anchor, the function will return 0.
*/
impl /*struct*/ QGraphicsAnchorLayout {
  pub fn anchor_0<RetType, T: QGraphicsAnchorLayout_anchor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.anchor_0(self);
    // return 1;
  }
}
pub trait QGraphicsAnchorLayout_anchor_0<RetType> {
  fn anchor_0(self , rsthis: & QGraphicsAnchorLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsAnchorLayout_anchor_0<usize> for (usize,i32,usize,i32) {
  fn anchor_0(self , rsthis: & QGraphicsAnchorLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN21QGraphicsAnchorLayout6anchorEP19QGraphicsLayoutItemN2Qt11AnchorPointES1_S3_", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsanchorlayout.h:87
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addCornerAnchors(QGraphicsLayoutItem *, Qt::Corner, QGraphicsLayoutItem *, Qt::Corner)

/*
Creates two anchors between firstItem and secondItem specified by the corners, firstCorner and secondCorner, where one is for the horizontal edge and another one for the vertical edge.

This is a convenience function, since anchoring corners can be expressed as anchoring two edges. For instance:


  layout->addAnchor(a, Qt::AnchorTop, layout, Qt::AnchorTop);
  layout->addAnchor(a, Qt::AnchorLeft, layout, Qt::AnchorLeft);



This can also be achieved with the following line of code:


  layout->addCornerAnchors(a, Qt::TopLeftCorner, layout, Qt::TopLeftCorner);



If there is already an anchor between the edge pairs, it will be replaced by the anchors that this function specifies.

firstItem and secondItem are automatically added to the layout if they are not part of the layout. This means that count() can increase by up to 2.

See also addAnchor() and addAnchors().
*/
impl /*struct*/ QGraphicsAnchorLayout {
  pub fn addCornerAnchors_0<RetType, T: QGraphicsAnchorLayout_addCornerAnchors_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addCornerAnchors_0(self);
    // return 1;
  }
}
pub trait QGraphicsAnchorLayout_addCornerAnchors_0<RetType> {
  fn addCornerAnchors_0(self , rsthis: & QGraphicsAnchorLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsAnchorLayout_addCornerAnchors_0<(/*void*/)> for (usize,i32,usize,i32) {
  fn addCornerAnchors_0(self , rsthis: & QGraphicsAnchorLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN21QGraphicsAnchorLayout16addCornerAnchorsEP19QGraphicsLayoutItemN2Qt6CornerES1_S3_", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsanchorlayout.h:90
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addAnchors(QGraphicsLayoutItem *, QGraphicsLayoutItem *, Qt::Orientations)

/*
Anchors two or four edges of firstItem with the corresponding edges of secondItem, so that firstItem has the same size as secondItem in the dimensions specified by orientations.

For example, the following example anchors the left and right edges of two items to match their widths:


  layout->addAnchor(b, Qt::AnchorLeft, c, Qt::AnchorLeft);
  layout->addAnchor(b, Qt::AnchorRight, c, Qt::AnchorRight);



This can also be achieved using the following line of code:


  layout->addAnchors(b, c, Qt::Horizontal);



See also addAnchor() and addCornerAnchors().
*/
impl /*struct*/ QGraphicsAnchorLayout {
  pub fn addAnchors_0<RetType, T: QGraphicsAnchorLayout_addAnchors_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addAnchors_0(self);
    // return 1;
  }
}
pub trait QGraphicsAnchorLayout_addAnchors_0<RetType> {
  fn addAnchors_0(self , rsthis: & QGraphicsAnchorLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsAnchorLayout_addAnchors_0<(/*void*/)> for (usize,usize,i32) {
  fn addAnchors_0(self , rsthis: & QGraphicsAnchorLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN21QGraphicsAnchorLayout10addAnchorsEP19QGraphicsLayoutItemS1_6QFlagsIN2Qt11OrientationEE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsanchorlayout.h:94
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setHorizontalSpacing(qreal)

/*
Sets the default horizontal spacing for the anchor layout to spacing.

See also horizontalSpacing(), setVerticalSpacing(), and setSpacing().
*/
impl /*struct*/ QGraphicsAnchorLayout {
  pub fn setHorizontalSpacing_0<RetType, T: QGraphicsAnchorLayout_setHorizontalSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHorizontalSpacing_0(self);
    // return 1;
  }
}
pub trait QGraphicsAnchorLayout_setHorizontalSpacing_0<RetType> {
  fn setHorizontalSpacing_0(self , rsthis: & QGraphicsAnchorLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsAnchorLayout_setHorizontalSpacing_0<(/*void*/)> for (f64) {
  fn setHorizontalSpacing_0(self , rsthis: & QGraphicsAnchorLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN21QGraphicsAnchorLayout20setHorizontalSpacingEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsanchorlayout.h:95
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setVerticalSpacing(qreal)

/*
Sets the default vertical spacing for the anchor layout to spacing.

See also verticalSpacing(), setHorizontalSpacing(), and setSpacing().
*/
impl /*struct*/ QGraphicsAnchorLayout {
  pub fn setVerticalSpacing_0<RetType, T: QGraphicsAnchorLayout_setVerticalSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVerticalSpacing_0(self);
    // return 1;
  }
}
pub trait QGraphicsAnchorLayout_setVerticalSpacing_0<RetType> {
  fn setVerticalSpacing_0(self , rsthis: & QGraphicsAnchorLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsAnchorLayout_setVerticalSpacing_0<(/*void*/)> for (f64) {
  fn setVerticalSpacing_0(self , rsthis: & QGraphicsAnchorLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN21QGraphicsAnchorLayout18setVerticalSpacingEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsanchorlayout.h:96
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSpacing(qreal)

/*
Sets the default horizontal and the default vertical spacing for the anchor layout to spacing.

If an item is anchored with no spacing associated with the anchor, it will use the default spacing.

QGraphicsAnchorLayout does not support negative spacings. Setting a negative value will unset the previous spacing and make the layout use the spacing provided by the current widget style.

See also setHorizontalSpacing() and setVerticalSpacing().
*/
impl /*struct*/ QGraphicsAnchorLayout {
  pub fn setSpacing_0<RetType, T: QGraphicsAnchorLayout_setSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSpacing_0(self);
    // return 1;
  }
}
pub trait QGraphicsAnchorLayout_setSpacing_0<RetType> {
  fn setSpacing_0(self , rsthis: & QGraphicsAnchorLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsAnchorLayout_setSpacing_0<(/*void*/)> for (f64) {
  fn setSpacing_0(self , rsthis: & QGraphicsAnchorLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN21QGraphicsAnchorLayout10setSpacingEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsanchorlayout.h:97
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal horizontalSpacing() const

/*
Returns the default horizontal spacing for the anchor layout.

See also verticalSpacing() and setHorizontalSpacing().
*/
impl /*struct*/ QGraphicsAnchorLayout {
  pub fn horizontalSpacing_0<RetType, T: QGraphicsAnchorLayout_horizontalSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.horizontalSpacing_0(self);
    // return 1;
  }
}
pub trait QGraphicsAnchorLayout_horizontalSpacing_0<RetType> {
  fn horizontalSpacing_0(self , rsthis: & QGraphicsAnchorLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsAnchorLayout_horizontalSpacing_0<f64> for () {
  fn horizontalSpacing_0(self , rsthis: & QGraphicsAnchorLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QGraphicsAnchorLayout17horizontalSpacingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsanchorlayout.h:98
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal verticalSpacing() const

/*
Returns the default vertical spacing for the anchor layout.

See also horizontalSpacing() and setVerticalSpacing().
*/
impl /*struct*/ QGraphicsAnchorLayout {
  pub fn verticalSpacing_0<RetType, T: QGraphicsAnchorLayout_verticalSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.verticalSpacing_0(self);
    // return 1;
  }
}
pub trait QGraphicsAnchorLayout_verticalSpacing_0<RetType> {
  fn verticalSpacing_0(self , rsthis: & QGraphicsAnchorLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsAnchorLayout_verticalSpacing_0<f64> for () {
  fn verticalSpacing_0(self , rsthis: & QGraphicsAnchorLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QGraphicsAnchorLayout15verticalSpacingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsanchorlayout.h:100
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void removeAt(int)

/*
Reimplemented from QGraphicsLayout::removeAt().

Removes the layout item at index without destroying it. Ownership of the item is transferred to the caller.

Removing an item will also remove any of the anchors associated with it.

See also itemAt() and count().
*/
impl /*struct*/ QGraphicsAnchorLayout {
  pub fn removeAt_0<RetType, T: QGraphicsAnchorLayout_removeAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeAt_0(self);
    // return 1;
  }
}
pub trait QGraphicsAnchorLayout_removeAt_0<RetType> {
  fn removeAt_0(self , rsthis: & QGraphicsAnchorLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsAnchorLayout_removeAt_0<(/*void*/)> for (i32) {
  fn removeAt_0(self , rsthis: & QGraphicsAnchorLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN21QGraphicsAnchorLayout8removeAtEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsanchorlayout.h:101
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setGeometry(const QRectF &)

/*
Reimplemented from QGraphicsLayoutItem::setGeometry().
*/
impl /*struct*/ QGraphicsAnchorLayout {
  pub fn setGeometry_0<RetType, T: QGraphicsAnchorLayout_setGeometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setGeometry_0(self);
    // return 1;
  }
}
pub trait QGraphicsAnchorLayout_setGeometry_0<RetType> {
  fn setGeometry_0(self , rsthis: & QGraphicsAnchorLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsAnchorLayout_setGeometry_0<(/*void*/)> for (usize) {
  fn setGeometry_0(self , rsthis: & QGraphicsAnchorLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN21QGraphicsAnchorLayout11setGeometryERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsanchorlayout.h:102
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int count() const

/*
Reimplemented from QGraphicsLayout::count().
*/
impl /*struct*/ QGraphicsAnchorLayout {
  pub fn count_0<RetType, T: QGraphicsAnchorLayout_count_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.count_0(self);
    // return 1;
  }
}
pub trait QGraphicsAnchorLayout_count_0<RetType> {
  fn count_0(self , rsthis: & QGraphicsAnchorLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsAnchorLayout_count_0<i32> for () {
  fn count_0(self , rsthis: & QGraphicsAnchorLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QGraphicsAnchorLayout5countEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsanchorlayout.h:103
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QGraphicsLayoutItem * itemAt(int) const

/*
Reimplemented from QGraphicsLayout::itemAt().
*/
impl /*struct*/ QGraphicsAnchorLayout {
  pub fn itemAt_0<RetType, T: QGraphicsAnchorLayout_itemAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemAt_0(self);
    // return 1;
  }
}
pub trait QGraphicsAnchorLayout_itemAt_0<RetType> {
  fn itemAt_0(self , rsthis: & QGraphicsAnchorLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsAnchorLayout_itemAt_0<usize> for (i32) {
  fn itemAt_0(self , rsthis: & QGraphicsAnchorLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QGraphicsAnchorLayout6itemAtEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsanchorlayout.h:105
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void invalidate()

/*
Reimplemented from QGraphicsLayout::invalidate().
*/
impl /*struct*/ QGraphicsAnchorLayout {
  pub fn invalidate_0<RetType, T: QGraphicsAnchorLayout_invalidate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.invalidate_0(self);
    // return 1;
  }
}
pub trait QGraphicsAnchorLayout_invalidate_0<RetType> {
  fn invalidate_0(self , rsthis: & QGraphicsAnchorLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsAnchorLayout_invalidate_0<(/*void*/)> for () {
  fn invalidate_0(self , rsthis: & QGraphicsAnchorLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN21QGraphicsAnchorLayout10invalidateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsanchorlayout.h:107
// index:0
// Protected virtual Visibility=Default Availability=Available
// [16] QSizeF sizeHint(Qt::SizeHint, const QSizeF &) const

/*
Reimplemented from QGraphicsLayoutItem::sizeHint().
*/
impl /*struct*/ QGraphicsAnchorLayout {
  pub fn sizeHint_0<RetType, T: QGraphicsAnchorLayout_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QGraphicsAnchorLayout_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QGraphicsAnchorLayout) -> RetType;
}
impl<'a> /*trait*/ QGraphicsAnchorLayout_sizeHint_0<usize> for (i32,usize) {
  fn sizeHint_0(self , rsthis: & QGraphicsAnchorLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QGraphicsAnchorLayout8sizeHintEN2Qt8SizeHintERK6QSizeF", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
