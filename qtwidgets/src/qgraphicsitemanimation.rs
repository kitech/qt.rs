

// mod ::widgets::QGraphicsItemAnimation
// package qtwidgets
// /usr/include/qt/QtWidgets/qgraphicsitemanimation.h
// #include <qgraphicsitemanimation.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 47
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

// void beforeAnimationStep(qreal)
// func (this *QGraphicsItemAnimation) InheritBeforeAnimationStep(f func(step float64) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "beforeAnimationStep", f)
// }

// void afterAnimationStep(qreal)
// func (this *QGraphicsItemAnimation) InheritAfterAnimationStep(f func(step float64) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "afterAnimationStep", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QGraphicsItemAnimation)=24
pub struct QGraphicsItemAnimation {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGraphicsItemAnimation_ITF interface {
//    qtcore.QObject_ITF
//    QGraphicsItemAnimation_PTR() *QGraphicsItemAnimation
//}
//func (ptr *QGraphicsItemAnimation) QGraphicsItemAnimation_PTR() *QGraphicsItemAnimation { return ptr }

impl /*struct*/ QGraphicsItemAnimation {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGraphicsItemAnimation {
    return QGraphicsItemAnimation{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGraphicsItemAnimation {
//  type Target = QGraphicsItemAnimationBASE;
//
//  fn deref(&self) -> &QGraphicsItemAnimationBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGraphicsItemAnimationBASE> for QGraphicsItemAnimation {
//  fn as_ref(& self) -> & QGraphicsItemAnimationBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgraphicsitemanimation.h:59
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QGraphicsItemAnimation {
  pub fn metaObject_0<RetType, T: QGraphicsItemAnimation_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QGraphicsItemAnimation_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItemAnimation_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QGraphicsItemAnimation) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK22QGraphicsItemAnimation10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitemanimation.h:61
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QGraphicsItemAnimation(QObject *)

/*
Constructs an animation object with the given parent.
*/
// QGraphicsItemAnimation(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QGraphicsItemAnimation {
  pub fn QGraphicsItemAnimation_0<T: QGraphicsItemAnimation_QGraphicsItemAnimation_0>(value: T) -> QGraphicsItemAnimation {
    let rsthis = value.QGraphicsItemAnimation_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_QGraphicsItemAnimation_0 {
  fn QGraphicsItemAnimation_0(self) -> QGraphicsItemAnimation;
}
// QGraphicsItemAnimation(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGraphicsItemAnimation_QGraphicsItemAnimation_0 for (usize) {
  fn QGraphicsItemAnimation_0(self) -> QGraphicsItemAnimation {
    // unsafe{_ZN22QGraphicsItemAnimationC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN22QGraphicsItemAnimationC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGraphicsItemAnimation{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitemanimation.h:62
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QGraphicsItemAnimation()

/*

*/
pub fn DeleteQGraphicsItemAnimation(this :*mut QGraphicsItemAnimation) {
    // let rv = qtrt::InvokeQtFunc6("_ZN22QGraphicsItemAnimationD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 24)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgraphicsitemanimation.h:64
// index:0
// Public Visibility=Default Availability=Available
// [8] QGraphicsItem * item() const

/*
Returns the item on which the animation object operates.

See also setItem().
*/
impl /*struct*/ QGraphicsItemAnimation {
  pub fn item_0<RetType, T: QGraphicsItemAnimation_item_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.item_0(self);
    // return 1;
  }
}
pub trait QGraphicsItemAnimation_item_0<RetType> {
  fn item_0(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItemAnimation_item_0<usize> for () {
  fn item_0(self , rsthis: & QGraphicsItemAnimation) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK22QGraphicsItemAnimation4itemEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitemanimation.h:65
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setItem(QGraphicsItem *)

/*
Sets the specified item to be used in the animation.

See also item().
*/
impl /*struct*/ QGraphicsItemAnimation {
  pub fn setItem_0<RetType, T: QGraphicsItemAnimation_setItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setItem_0(self);
    // return 1;
  }
}
pub trait QGraphicsItemAnimation_setItem_0<RetType> {
  fn setItem_0(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItemAnimation_setItem_0<(/*void*/)> for (usize) {
  fn setItem_0(self , rsthis: & QGraphicsItemAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN22QGraphicsItemAnimation7setItemEP13QGraphicsItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitemanimation.h:67
// index:0
// Public Visibility=Default Availability=Available
// [8] QTimeLine * timeLine() const

/*
Returns the timeline object used to control the rate at which the animation occurs.

See also setTimeLine().
*/
impl /*struct*/ QGraphicsItemAnimation {
  pub fn timeLine_0<RetType, T: QGraphicsItemAnimation_timeLine_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.timeLine_0(self);
    // return 1;
  }
}
pub trait QGraphicsItemAnimation_timeLine_0<RetType> {
  fn timeLine_0(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItemAnimation_timeLine_0<usize> for () {
  fn timeLine_0(self , rsthis: & QGraphicsItemAnimation) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK22QGraphicsItemAnimation8timeLineEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitemanimation.h:68
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTimeLine(QTimeLine *)

/*
Sets the timeline object used to control the rate of animation to the timeLine specified.

See also timeLine().
*/
impl /*struct*/ QGraphicsItemAnimation {
  pub fn setTimeLine_0<RetType, T: QGraphicsItemAnimation_setTimeLine_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTimeLine_0(self);
    // return 1;
  }
}
pub trait QGraphicsItemAnimation_setTimeLine_0<RetType> {
  fn setTimeLine_0(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItemAnimation_setTimeLine_0<(/*void*/)> for (usize) {
  fn setTimeLine_0(self , rsthis: & QGraphicsItemAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN22QGraphicsItemAnimation11setTimeLineEP9QTimeLine", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitemanimation.h:70
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF posAt(qreal) const

/*
Returns the position of the item at the given step value.

See also setPosAt().
*/
impl /*struct*/ QGraphicsItemAnimation {
  pub fn posAt_0<RetType, T: QGraphicsItemAnimation_posAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.posAt_0(self);
    // return 1;
  }
}
pub trait QGraphicsItemAnimation_posAt_0<RetType> {
  fn posAt_0(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItemAnimation_posAt_0<usize> for (f64) {
  fn posAt_0(self , rsthis: & QGraphicsItemAnimation) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK22QGraphicsItemAnimation5posAtEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitemanimation.h:72
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPosAt(qreal, const QPointF &)

/*
Sets the position of the item at the given step value to the point specified.

See also posAt().
*/
impl /*struct*/ QGraphicsItemAnimation {
  pub fn setPosAt_0<RetType, T: QGraphicsItemAnimation_setPosAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPosAt_0(self);
    // return 1;
  }
}
pub trait QGraphicsItemAnimation_setPosAt_0<RetType> {
  fn setPosAt_0(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItemAnimation_setPosAt_0<(/*void*/)> for (f64,usize) {
  fn setPosAt_0(self , rsthis: & QGraphicsItemAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN22QGraphicsItemAnimation8setPosAtEdRK7QPointF", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitemanimation.h:74
// index:0
// Public Visibility=Default Availability=Available
// [48] QMatrix matrixAt(qreal) const

/*
Returns the matrix used to transform the item at the specified step value.
*/
impl /*struct*/ QGraphicsItemAnimation {
  pub fn matrixAt_0<RetType, T: QGraphicsItemAnimation_matrixAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.matrixAt_0(self);
    // return 1;
  }
}
pub trait QGraphicsItemAnimation_matrixAt_0<RetType> {
  fn matrixAt_0(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItemAnimation_matrixAt_0<usize> for (f64) {
  fn matrixAt_0(self , rsthis: & QGraphicsItemAnimation) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK22QGraphicsItemAnimation8matrixAtEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitemanimation.h:76
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal rotationAt(qreal) const

/*
Returns the angle at which the item is rotated at the specified step value.

See also setRotationAt().
*/
impl /*struct*/ QGraphicsItemAnimation {
  pub fn rotationAt_0<RetType, T: QGraphicsItemAnimation_rotationAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rotationAt_0(self);
    // return 1;
  }
}
pub trait QGraphicsItemAnimation_rotationAt_0<RetType> {
  fn rotationAt_0(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItemAnimation_rotationAt_0<f64> for (f64) {
  fn rotationAt_0(self , rsthis: & QGraphicsItemAnimation) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK22QGraphicsItemAnimation10rotationAtEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitemanimation.h:78
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRotationAt(qreal, qreal)

/*
Sets the rotation of the item at the given step value to the angle specified.

See also rotationAt().
*/
impl /*struct*/ QGraphicsItemAnimation {
  pub fn setRotationAt_0<RetType, T: QGraphicsItemAnimation_setRotationAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRotationAt_0(self);
    // return 1;
  }
}
pub trait QGraphicsItemAnimation_setRotationAt_0<RetType> {
  fn setRotationAt_0(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItemAnimation_setRotationAt_0<(/*void*/)> for (f64,f64) {
  fn setRotationAt_0(self , rsthis: & QGraphicsItemAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN22QGraphicsItemAnimation13setRotationAtEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitemanimation.h:80
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal xTranslationAt(qreal) const

/*
Returns the horizontal translation of the item at the specified step value.

See also setTranslationAt().
*/
impl /*struct*/ QGraphicsItemAnimation {
  pub fn xTranslationAt_0<RetType, T: QGraphicsItemAnimation_xTranslationAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.xTranslationAt_0(self);
    // return 1;
  }
}
pub trait QGraphicsItemAnimation_xTranslationAt_0<RetType> {
  fn xTranslationAt_0(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItemAnimation_xTranslationAt_0<f64> for (f64) {
  fn xTranslationAt_0(self , rsthis: & QGraphicsItemAnimation) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK22QGraphicsItemAnimation14xTranslationAtEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitemanimation.h:81
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal yTranslationAt(qreal) const

/*
Returns the vertical translation of the item at the specified step value.

See also setTranslationAt().
*/
impl /*struct*/ QGraphicsItemAnimation {
  pub fn yTranslationAt_0<RetType, T: QGraphicsItemAnimation_yTranslationAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.yTranslationAt_0(self);
    // return 1;
  }
}
pub trait QGraphicsItemAnimation_yTranslationAt_0<RetType> {
  fn yTranslationAt_0(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItemAnimation_yTranslationAt_0<f64> for (f64) {
  fn yTranslationAt_0(self , rsthis: & QGraphicsItemAnimation) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK22QGraphicsItemAnimation14yTranslationAtEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitemanimation.h:83
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTranslationAt(qreal, qreal, qreal)

/*
Sets the translation of the item at the given step value using the horizontal and vertical coordinates specified by dx and dy.

See also xTranslationAt() and yTranslationAt().
*/
impl /*struct*/ QGraphicsItemAnimation {
  pub fn setTranslationAt_0<RetType, T: QGraphicsItemAnimation_setTranslationAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTranslationAt_0(self);
    // return 1;
  }
}
pub trait QGraphicsItemAnimation_setTranslationAt_0<RetType> {
  fn setTranslationAt_0(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItemAnimation_setTranslationAt_0<(/*void*/)> for (f64,f64,f64) {
  fn setTranslationAt_0(self , rsthis: & QGraphicsItemAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN22QGraphicsItemAnimation16setTranslationAtEddd", 3,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitemanimation.h:85
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal verticalScaleAt(qreal) const

/*
Returns the vertical scale for the item at the specified step value.

See also setScaleAt().
*/
impl /*struct*/ QGraphicsItemAnimation {
  pub fn verticalScaleAt_0<RetType, T: QGraphicsItemAnimation_verticalScaleAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.verticalScaleAt_0(self);
    // return 1;
  }
}
pub trait QGraphicsItemAnimation_verticalScaleAt_0<RetType> {
  fn verticalScaleAt_0(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItemAnimation_verticalScaleAt_0<f64> for (f64) {
  fn verticalScaleAt_0(self , rsthis: & QGraphicsItemAnimation) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK22QGraphicsItemAnimation15verticalScaleAtEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitemanimation.h:86
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal horizontalScaleAt(qreal) const

/*
Returns the horizontal scale for the item at the specified step value.

See also setScaleAt().
*/
impl /*struct*/ QGraphicsItemAnimation {
  pub fn horizontalScaleAt_0<RetType, T: QGraphicsItemAnimation_horizontalScaleAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.horizontalScaleAt_0(self);
    // return 1;
  }
}
pub trait QGraphicsItemAnimation_horizontalScaleAt_0<RetType> {
  fn horizontalScaleAt_0(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItemAnimation_horizontalScaleAt_0<f64> for (f64) {
  fn horizontalScaleAt_0(self , rsthis: & QGraphicsItemAnimation) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK22QGraphicsItemAnimation17horizontalScaleAtEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitemanimation.h:88
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setScaleAt(qreal, qreal, qreal)

/*
Sets the scale of the item at the given step value using the horizontal and vertical scale factors specified by sx and sy.

See also verticalScaleAt() and horizontalScaleAt().
*/
impl /*struct*/ QGraphicsItemAnimation {
  pub fn setScaleAt_0<RetType, T: QGraphicsItemAnimation_setScaleAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setScaleAt_0(self);
    // return 1;
  }
}
pub trait QGraphicsItemAnimation_setScaleAt_0<RetType> {
  fn setScaleAt_0(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItemAnimation_setScaleAt_0<(/*void*/)> for (f64,f64,f64) {
  fn setScaleAt_0(self , rsthis: & QGraphicsItemAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN22QGraphicsItemAnimation10setScaleAtEddd", 3,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitemanimation.h:90
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal verticalShearAt(qreal) const

/*
Returns the vertical shear for the item at the specified step value.

See also setShearAt().
*/
impl /*struct*/ QGraphicsItemAnimation {
  pub fn verticalShearAt_0<RetType, T: QGraphicsItemAnimation_verticalShearAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.verticalShearAt_0(self);
    // return 1;
  }
}
pub trait QGraphicsItemAnimation_verticalShearAt_0<RetType> {
  fn verticalShearAt_0(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItemAnimation_verticalShearAt_0<f64> for (f64) {
  fn verticalShearAt_0(self , rsthis: & QGraphicsItemAnimation) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK22QGraphicsItemAnimation15verticalShearAtEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitemanimation.h:91
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal horizontalShearAt(qreal) const

/*
Returns the horizontal shear for the item at the specified step value.

See also setShearAt().
*/
impl /*struct*/ QGraphicsItemAnimation {
  pub fn horizontalShearAt_0<RetType, T: QGraphicsItemAnimation_horizontalShearAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.horizontalShearAt_0(self);
    // return 1;
  }
}
pub trait QGraphicsItemAnimation_horizontalShearAt_0<RetType> {
  fn horizontalShearAt_0(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItemAnimation_horizontalShearAt_0<f64> for (f64) {
  fn horizontalShearAt_0(self , rsthis: & QGraphicsItemAnimation) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK22QGraphicsItemAnimation17horizontalShearAtEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitemanimation.h:93
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setShearAt(qreal, qreal, qreal)

/*
Sets the shear of the item at the given step value using the horizontal and vertical shear factors specified by sh and sv.

See also verticalShearAt() and horizontalShearAt().
*/
impl /*struct*/ QGraphicsItemAnimation {
  pub fn setShearAt_0<RetType, T: QGraphicsItemAnimation_setShearAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setShearAt_0(self);
    // return 1;
  }
}
pub trait QGraphicsItemAnimation_setShearAt_0<RetType> {
  fn setShearAt_0(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItemAnimation_setShearAt_0<(/*void*/)> for (f64,f64,f64) {
  fn setShearAt_0(self , rsthis: & QGraphicsItemAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN22QGraphicsItemAnimation10setShearAtEddd", 3,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitemanimation.h:95
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clear()

/*
Clears the scheduled transformations used for the animation, but retains the item and timeline.
*/
impl /*struct*/ QGraphicsItemAnimation {
  pub fn clear_0<RetType, T: QGraphicsItemAnimation_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QGraphicsItemAnimation_clear_0<RetType> {
  fn clear_0(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItemAnimation_clear_0<(/*void*/)> for () {
  fn clear_0(self , rsthis: & QGraphicsItemAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN22QGraphicsItemAnimation5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitemanimation.h:98
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStep(qreal)

/*
Sets the current step value for the animation, causing the transformations scheduled at this step to be performed.
*/
impl /*struct*/ QGraphicsItemAnimation {
  pub fn setStep_0<RetType, T: QGraphicsItemAnimation_setStep_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStep_0(self);
    // return 1;
  }
}
pub trait QGraphicsItemAnimation_setStep_0<RetType> {
  fn setStep_0(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItemAnimation_setStep_0<(/*void*/)> for (f64) {
  fn setStep_0(self , rsthis: & QGraphicsItemAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN22QGraphicsItemAnimation7setStepEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitemanimation.h:99
// index:0
// Public Visibility=Default Availability=Available
// [-2] void reset()

/*

*/
impl /*struct*/ QGraphicsItemAnimation {
  pub fn reset_0<RetType, T: QGraphicsItemAnimation_reset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.reset_0(self);
    // return 1;
  }
}
pub trait QGraphicsItemAnimation_reset_0<RetType> {
  fn reset_0(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItemAnimation_reset_0<(/*void*/)> for () {
  fn reset_0(self , rsthis: & QGraphicsItemAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN22QGraphicsItemAnimation5resetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitemanimation.h:102
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void beforeAnimationStep(qreal)

/*
This method is meant to be overridden by subclassed that needs to execute additional code before a new step takes place. The animation step is provided for use in cases where the action depends on its value.
*/
impl /*struct*/ QGraphicsItemAnimation {
  pub fn beforeAnimationStep_0<RetType, T: QGraphicsItemAnimation_beforeAnimationStep_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.beforeAnimationStep_0(self);
    // return 1;
  }
}
pub trait QGraphicsItemAnimation_beforeAnimationStep_0<RetType> {
  fn beforeAnimationStep_0(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItemAnimation_beforeAnimationStep_0<(/*void*/)> for (f64) {
  fn beforeAnimationStep_0(self , rsthis: & QGraphicsItemAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN22QGraphicsItemAnimation19beforeAnimationStepEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsitemanimation.h:103
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void afterAnimationStep(qreal)

/*
This method is meant to be overridden in subclasses that need to execute additional code after a new step has taken place. The animation step is provided for use in cases where the action depends on its value.
*/
impl /*struct*/ QGraphicsItemAnimation {
  pub fn afterAnimationStep_0<RetType, T: QGraphicsItemAnimation_afterAnimationStep_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.afterAnimationStep_0(self);
    // return 1;
  }
}
pub trait QGraphicsItemAnimation_afterAnimationStep_0<RetType> {
  fn afterAnimationStep_0(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}
impl<'a> /*trait*/ QGraphicsItemAnimation_afterAnimationStep_0<(/*void*/)> for (f64) {
  fn afterAnimationStep_0(self , rsthis: & QGraphicsItemAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN22QGraphicsItemAnimation18afterAnimationStepEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
