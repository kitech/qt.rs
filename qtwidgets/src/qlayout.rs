

// mod ::widgets::QLayout
// package qtwidgets
// /usr/include/qt/QtWidgets/qlayout.h
// #include <qlayout.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 6
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

// void widgetEvent(QEvent *)
// func (this *QLayout) InheritWidgetEvent(f func(arg0 *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "widgetEvent", f)
// }

// void childEvent(QChildEvent *)
// func (this *QLayout) InheritChildEvent(f func(e *qtcore.QChildEvent/*777 QChildEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "childEvent", f)
// }

// void addChildLayout(QLayout *)
// func (this *QLayout) InheritAddChildLayout(f func(l *QLayout/*777 QLayout **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "addChildLayout", f)
// }

// void addChildWidget(QWidget *)
// func (this *QLayout) InheritAddChildWidget(f func(w *QWidget/*777 QWidget **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "addChildWidget", f)
// }

// bool adoptLayout(QLayout *)
// func (this *QLayout) InheritAdoptLayout(f func(layout *QLayout/*777 QLayout **/) bool) {
//  qtrt.SetAllInheritCallback(this, "adoptLayout", f)
// }

// QRect alignmentRect(const QRect &)
// func (this *QLayout) InheritAlignmentRect(f func(arg0 *qtcore.QRect) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "alignmentRect", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QLayout)=32
pub struct QLayout {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QLayout_ITF interface {
//    qtcore.QObject_ITF
//    QLayoutItem_ITF
//    QLayout_PTR() *QLayout
//}
//func (ptr *QLayout) QLayout_PTR() *QLayout { return ptr }

impl /*struct*/ QLayout {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QLayout {
    return QLayout{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QLayout {
//  type Target = QLayoutBASE;
//
//  fn deref(&self) -> &QLayoutBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QLayoutBASE> for QLayout {
//  fn as_ref(& self) -> & QLayoutBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qlayout.h:63
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QLayout {
  pub fn metaObject_0<RetType, T: QLayout_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QLayout_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLayout10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:80
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QLayout(QWidget *)

/*
Constructs a new top-level QLayout, with parent parent. parent may not be 0.

There can be only one top-level layout for a widget. It is returned by QWidget::layout().
*/
// QLayout(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QLayout {
  pub fn QLayout_0<T: QLayout_QLayout_0>(value: T) -> QLayout {
    let rsthis = value.QLayout_0();
    return rsthis;
    // return 1;
  }
}

pub trait QLayout_QLayout_0 {
  fn QLayout_0(self) -> QLayout;
}
// QLayout(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QLayout_QLayout_0 for (usize) {
  fn QLayout_0(self) -> QLayout {
    // unsafe{_ZN7QLayoutC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QLayoutC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QLayout{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:81
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QLayout()

/*
Constructs a new top-level QLayout, with parent parent. parent may not be 0.

There can be only one top-level layout for a widget. It is returned by QWidget::layout().
*/
// QLayout() ctx.fn_proto_cpp
impl /*struct*/ QLayout {
  pub fn QLayout_1<T: QLayout_QLayout_1>(value: T) -> QLayout {
    let rsthis = value.QLayout_1();
    return rsthis;
    // return 1;
  }
}

pub trait QLayout_QLayout_1 {
  fn QLayout_1(self) -> QLayout;
}
// QLayout() ctx.fn_proto_cpp
impl<'a> /*trait*/ QLayout_QLayout_1 for () {
  fn QLayout_1(self) -> QLayout {
    // unsafe{_ZN7QLayoutC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QLayoutC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QLayout{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:82
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QLayout()

/*

*/
pub fn DeleteQLayout(this :*mut QLayout) {
    // let rv = qtrt::InvokeQtFunc6("_ZN7QLayoutD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 32)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qlayout.h:84
// index:0
// Public Visibility=Default Availability=Available
// [4] int margin() const

/*

*/
impl /*struct*/ QLayout {
  pub fn margin_0<RetType, T: QLayout_margin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.margin_0(self);
    // return 1;
  }
}
pub trait QLayout_margin_0<RetType> {
  fn margin_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_margin_0<i32> for () {
  fn margin_0(self , rsthis: & QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLayout6marginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:85
// index:0
// Public Visibility=Default Availability=Available
// [4] int spacing() const

/*

*/
impl /*struct*/ QLayout {
  pub fn spacing_0<RetType, T: QLayout_spacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.spacing_0(self);
    // return 1;
  }
}
pub trait QLayout_spacing_0<RetType> {
  fn spacing_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_spacing_0<i32> for () {
  fn spacing_0(self , rsthis: & QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLayout7spacingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:87
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMargin(int)

/*

*/
impl /*struct*/ QLayout {
  pub fn setMargin_0<RetType, T: QLayout_setMargin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMargin_0(self);
    // return 1;
  }
}
pub trait QLayout_setMargin_0<RetType> {
  fn setMargin_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_setMargin_0<(/*void*/)> for (i32) {
  fn setMargin_0(self , rsthis: & QLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QLayout9setMarginEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:88
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSpacing(int)

/*

*/
impl /*struct*/ QLayout {
  pub fn setSpacing_0<RetType, T: QLayout_setSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSpacing_0(self);
    // return 1;
  }
}
pub trait QLayout_setSpacing_0<RetType> {
  fn setSpacing_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_setSpacing_0<(/*void*/)> for (i32) {
  fn setSpacing_0(self , rsthis: & QLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QLayout10setSpacingEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:90
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setContentsMargins(int, int, int, int)

/*
Sets the left, top, right, and bottom margins to use around the layout.

By default, QLayout uses the values provided by the style. On most platforms, the margin is 11 pixels in all directions.

This function was introduced in  Qt 4.3.

See also contentsMargins(), getContentsMargins(), QStyle::pixelMetric(), PM_LayoutLeftMargin, PM_LayoutTopMargin, PM_LayoutRightMargin, and PM_LayoutBottomMargin.
*/
impl /*struct*/ QLayout {
  pub fn setContentsMargins_0<RetType, T: QLayout_setContentsMargins_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setContentsMargins_0(self);
    // return 1;
  }
}
pub trait QLayout_setContentsMargins_0<RetType> {
  fn setContentsMargins_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_setContentsMargins_0<(/*void*/)> for (i32,i32,i32,i32) {
  fn setContentsMargins_0(self , rsthis: & QLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QLayout18setContentsMarginsEiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:91
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setContentsMargins(const QMargins &)

/*
Sets the left, top, right, and bottom margins to use around the layout.

By default, QLayout uses the values provided by the style. On most platforms, the margin is 11 pixels in all directions.

This function was introduced in  Qt 4.3.

See also contentsMargins(), getContentsMargins(), QStyle::pixelMetric(), PM_LayoutLeftMargin, PM_LayoutTopMargin, PM_LayoutRightMargin, and PM_LayoutBottomMargin.
*/
impl /*struct*/ QLayout {
  pub fn setContentsMargins_1<RetType, T: QLayout_setContentsMargins_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setContentsMargins_1(self);
    // return 1;
  }
}
pub trait QLayout_setContentsMargins_1<RetType> {
  fn setContentsMargins_1(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_setContentsMargins_1<(/*void*/)> for (usize) {
  fn setContentsMargins_1(self , rsthis: & QLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QLayout18setContentsMarginsERK8QMargins", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:92
// index:0
// Public Visibility=Default Availability=Available
// [-2] void getContentsMargins(int *, int *, int *, int *) const

/*
Extracts the left, top, right, and bottom margins used around the layout, and assigns them to *left, *top, *right, and *bottom (unless they are null pointers).

By default, QLayout uses the values provided by the style. On most platforms, the margin is 11 pixels in all directions.

This function was introduced in  Qt 4.3.

See also setContentsMargins(), QStyle::pixelMetric(), PM_LayoutLeftMargin, PM_LayoutTopMargin, PM_LayoutRightMargin, and PM_LayoutBottomMargin.
*/
impl /*struct*/ QLayout {
  pub fn getContentsMargins_0<RetType, T: QLayout_getContentsMargins_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.getContentsMargins_0(self);
    // return 1;
  }
}
pub trait QLayout_getContentsMargins_0<RetType> {
  fn getContentsMargins_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_getContentsMargins_0<(/*void*/)> for (usize,usize,usize,usize) {
  fn getContentsMargins_0(self , rsthis: & QLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let arg3 = (&self.3) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK7QLayout18getContentsMarginsEPiS0_S0_S0_", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:93
// index:0
// Public Visibility=Default Availability=Available
// [16] QMargins contentsMargins() const

/*
Returns the margins used around the layout.

By default, QLayout uses the values provided by the style. On most platforms, the margin is 11 pixels in all directions.

This function was introduced in  Qt 4.6.

See also setContentsMargins().
*/
impl /*struct*/ QLayout {
  pub fn contentsMargins_0<RetType, T: QLayout_contentsMargins_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contentsMargins_0(self);
    // return 1;
  }
}
pub trait QLayout_contentsMargins_0<RetType> {
  fn contentsMargins_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_contentsMargins_0<usize> for () {
  fn contentsMargins_0(self , rsthis: & QLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLayout15contentsMarginsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:94
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect contentsRect() const

/*
Returns the layout's geometry() rectangle, but taking into account the contents margins.

This function was introduced in  Qt 4.3.

See also setContentsMargins() and getContentsMargins().
*/
impl /*struct*/ QLayout {
  pub fn contentsRect_0<RetType, T: QLayout_contentsRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contentsRect_0(self);
    // return 1;
  }
}
pub trait QLayout_contentsRect_0<RetType> {
  fn contentsRect_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_contentsRect_0<usize> for () {
  fn contentsRect_0(self , rsthis: & QLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLayout12contentsRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:96
// index:0
// Public Visibility=Default Availability=Available
// [1] bool setAlignment(QWidget *, Qt::Alignment)

/*
Sets the alignment for widget w to alignment and returns true if w is found in this layout (not including child layouts); otherwise returns false.
*/
impl /*struct*/ QLayout {
  pub fn setAlignment_0<RetType, T: QLayout_setAlignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAlignment_0(self);
    // return 1;
  }
}
pub trait QLayout_setAlignment_0<RetType> {
  fn setAlignment_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_setAlignment_0<bool> for (usize,i32) {
  fn setAlignment_0(self , rsthis: & QLayout) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QLayout12setAlignmentEP7QWidget6QFlagsIN2Qt13AlignmentFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:97
// index:1
// Public Visibility=Default Availability=Available
// [1] bool setAlignment(QLayout *, Qt::Alignment)

/*
Sets the alignment for widget w to alignment and returns true if w is found in this layout (not including child layouts); otherwise returns false.
*/
impl /*struct*/ QLayout {
  pub fn setAlignment_1<RetType, T: QLayout_setAlignment_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAlignment_1(self);
    // return 1;
  }
}
pub trait QLayout_setAlignment_1<RetType> {
  fn setAlignment_1(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_setAlignment_1<bool> for (usize,i32) {
  fn setAlignment_1(self , rsthis: & QLayout) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QLayout12setAlignmentEPS_6QFlagsIN2Qt13AlignmentFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:100
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSizeConstraint(QLayout::SizeConstraint)

/*

*/
impl /*struct*/ QLayout {
  pub fn setSizeConstraint_0<RetType, T: QLayout_setSizeConstraint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSizeConstraint_0(self);
    // return 1;
  }
}
pub trait QLayout_setSizeConstraint_0<RetType> {
  fn setSizeConstraint_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_setSizeConstraint_0<(/*void*/)> for (i32) {
  fn setSizeConstraint_0(self , rsthis: & QLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QLayout17setSizeConstraintENS_14SizeConstraintE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:101
// index:0
// Public Visibility=Default Availability=Available
// [4] QLayout::SizeConstraint sizeConstraint() const

/*

*/
impl /*struct*/ QLayout {
  pub fn sizeConstraint_0<RetType, T: QLayout_sizeConstraint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeConstraint_0(self);
    // return 1;
  }
}
pub trait QLayout_sizeConstraint_0<RetType> {
  fn sizeConstraint_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_sizeConstraint_0<i32> for () {
  fn sizeConstraint_0(self , rsthis: & QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLayout14sizeConstraintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:102
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMenuBar(QWidget *)

/*
Tells the geometry manager to place the menu bar widget at the top of parentWidget(), outside QWidget::contentsMargins(). All child widgets are placed below the bottom edge of the menu bar.

See also menuBar().
*/
impl /*struct*/ QLayout {
  pub fn setMenuBar_0<RetType, T: QLayout_setMenuBar_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMenuBar_0(self);
    // return 1;
  }
}
pub trait QLayout_setMenuBar_0<RetType> {
  fn setMenuBar_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_setMenuBar_0<(/*void*/)> for (usize) {
  fn setMenuBar_0(self , rsthis: & QLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QLayout10setMenuBarEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:103
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * menuBar() const

/*
Returns the menu bar set for this layout, or 0 if no menu bar is set.

See also setMenuBar().
*/
impl /*struct*/ QLayout {
  pub fn menuBar_0<RetType, T: QLayout_menuBar_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.menuBar_0(self);
    // return 1;
  }
}
pub trait QLayout_menuBar_0<RetType> {
  fn menuBar_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_menuBar_0<usize> for () {
  fn menuBar_0(self , rsthis: & QLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLayout7menuBarEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:105
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * parentWidget() const

/*
Returns the parent widget of this layout, or 0 if this layout is not installed on any widget.

If the layout is a sub-layout, this function returns the parent widget of the parent layout.

See also parent().
*/
impl /*struct*/ QLayout {
  pub fn parentWidget_0<RetType, T: QLayout_parentWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.parentWidget_0(self);
    // return 1;
  }
}
pub trait QLayout_parentWidget_0<RetType> {
  fn parentWidget_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_parentWidget_0<usize> for () {
  fn parentWidget_0(self , rsthis: & QLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLayout12parentWidgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:107
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void invalidate()

/*
Reimplemented from QLayoutItem::invalidate().
*/
impl /*struct*/ QLayout {
  pub fn invalidate_0<RetType, T: QLayout_invalidate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.invalidate_0(self);
    // return 1;
  }
}
pub trait QLayout_invalidate_0<RetType> {
  fn invalidate_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_invalidate_0<(/*void*/)> for () {
  fn invalidate_0(self , rsthis: & QLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QLayout10invalidateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:108
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QRect geometry() const

/*
Reimplemented from QLayoutItem::geometry().

See also setGeometry().
*/
impl /*struct*/ QLayout {
  pub fn geometry_0<RetType, T: QLayout_geometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.geometry_0(self);
    // return 1;
  }
}
pub trait QLayout_geometry_0<RetType> {
  fn geometry_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_geometry_0<usize> for () {
  fn geometry_0(self , rsthis: & QLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLayout8geometryEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:109
// index:0
// Public Visibility=Default Availability=Available
// [1] bool activate()

/*
Redoes the layout for parentWidget() if necessary.

You should generally not need to call this because it is automatically called at the most appropriate times. It returns true if the layout was redone.

See also update() and QWidget::updateGeometry().
*/
impl /*struct*/ QLayout {
  pub fn activate_0<RetType, T: QLayout_activate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.activate_0(self);
    // return 1;
  }
}
pub trait QLayout_activate_0<RetType> {
  fn activate_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_activate_0<bool> for () {
  fn activate_0(self , rsthis: & QLayout) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QLayout8activateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:110
// index:0
// Public Visibility=Default Availability=Available
// [-2] void update()

/*
Updates the layout for parentWidget().

You should generally not need to call this because it is automatically called at the most appropriate times.

See also activate() and invalidate().
*/
impl /*struct*/ QLayout {
  pub fn update_0<RetType, T: QLayout_update_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.update_0(self);
    // return 1;
  }
}
pub trait QLayout_update_0<RetType> {
  fn update_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_update_0<(/*void*/)> for () {
  fn update_0(self , rsthis: & QLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QLayout6updateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:112
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addWidget(QWidget *)

/*
Adds widget w to this layout in a manner specific to the layout. This function uses addItem().
*/
impl /*struct*/ QLayout {
  pub fn addWidget_0<RetType, T: QLayout_addWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addWidget_0(self);
    // return 1;
  }
}
pub trait QLayout_addWidget_0<RetType> {
  fn addWidget_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_addWidget_0<(/*void*/)> for (usize) {
  fn addWidget_0(self , rsthis: & QLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QLayout9addWidgetEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:113
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [-2] void addItem(QLayoutItem *)

/*
Implemented in subclasses to add an item. How it is added is specific to each subclass.

This function is not usually called in application code. To add a widget to a layout, use the addWidget() function; to add a child layout, use the addLayout() function provided by the relevant QLayout subclass.

Note: The ownership of item is transferred to the layout, and it's the layout's responsibility to delete it.

See also addWidget(), QBoxLayout::addLayout(), and QGridLayout::addLayout().
*/
impl /*struct*/ QLayout {
  pub fn addItem_0<RetType, T: QLayout_addItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addItem_0(self);
    // return 1;
  }
}
pub trait QLayout_addItem_0<RetType> {
  fn addItem_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_addItem_0<(/*void*/)> for (usize) {
  fn addItem_0(self , rsthis: & QLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QLayout7addItemEP11QLayoutItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:115
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeWidget(QWidget *)

/*
Removes the widget widget from the layout. After this call, it is the caller's responsibility to give the widget a reasonable geometry or to put the widget back into a layout or to explicitly hide it if necessary.

Note: The ownership of widget remains the same as when it was added.

See also removeItem(), QWidget::setGeometry(), and addWidget().
*/
impl /*struct*/ QLayout {
  pub fn removeWidget_0<RetType, T: QLayout_removeWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeWidget_0(self);
    // return 1;
  }
}
pub trait QLayout_removeWidget_0<RetType> {
  fn removeWidget_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_removeWidget_0<(/*void*/)> for (usize) {
  fn removeWidget_0(self , rsthis: & QLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QLayout12removeWidgetEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:116
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeItem(QLayoutItem *)

/*
Removes the layout item item from the layout. It is the caller's responsibility to delete the item.

Notice that item can be a layout (since QLayout inherits QLayoutItem).

See also removeWidget() and addItem().
*/
impl /*struct*/ QLayout {
  pub fn removeItem_0<RetType, T: QLayout_removeItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeItem_0(self);
    // return 1;
  }
}
pub trait QLayout_removeItem_0<RetType> {
  fn removeItem_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_removeItem_0<(/*void*/)> for (usize) {
  fn removeItem_0(self , rsthis: & QLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QLayout10removeItemEP11QLayoutItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:118
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] Qt::Orientations expandingDirections() const

/*
Reimplemented from QLayoutItem::expandingDirections().

Returns whether this layout can make use of more space than sizeHint(). A value of Qt::Vertical or Qt::Horizontal means that it wants to grow in only one dimension, whereas Qt::Vertical | Qt::Horizontal means that it wants to grow in both dimensions.

The default implementation returns Qt::Horizontal | Qt::Vertical. Subclasses reimplement it to return a meaningful value based on their child widgets's size policies.

See also sizeHint().
*/
impl /*struct*/ QLayout {
  pub fn expandingDirections_0<RetType, T: QLayout_expandingDirections_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.expandingDirections_0(self);
    // return 1;
  }
}
pub trait QLayout_expandingDirections_0<RetType> {
  fn expandingDirections_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_expandingDirections_0<i32> for () {
  fn expandingDirections_0(self , rsthis: & QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLayout19expandingDirectionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:119
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize minimumSize() const

/*
Reimplemented from QLayoutItem::minimumSize().

Returns the minimum size of this layout. This is the smallest size that the layout can have while still respecting the specifications.

The returned value doesn't include the space required by QWidget::setContentsMargins() or menuBar().

The default implementation allows unlimited resizing.
*/
impl /*struct*/ QLayout {
  pub fn minimumSize_0<RetType, T: QLayout_minimumSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumSize_0(self);
    // return 1;
  }
}
pub trait QLayout_minimumSize_0<RetType> {
  fn minimumSize_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_minimumSize_0<usize> for () {
  fn minimumSize_0(self , rsthis: & QLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLayout11minimumSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:120
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize maximumSize() const

/*
Reimplemented from QLayoutItem::maximumSize().

Returns the maximum size of this layout. This is the largest size that the layout can have while still respecting the specifications.

The returned value doesn't include the space required by QWidget::setContentsMargins() or menuBar().

The default implementation allows unlimited resizing.
*/
impl /*struct*/ QLayout {
  pub fn maximumSize_0<RetType, T: QLayout_maximumSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maximumSize_0(self);
    // return 1;
  }
}
pub trait QLayout_maximumSize_0<RetType> {
  fn maximumSize_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_maximumSize_0<usize> for () {
  fn maximumSize_0(self , rsthis: & QLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLayout11maximumSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:121
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setGeometry(const QRect &)

/*
Reimplemented from QLayoutItem::setGeometry().

See also geometry().
*/
impl /*struct*/ QLayout {
  pub fn setGeometry_0<RetType, T: QLayout_setGeometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setGeometry_0(self);
    // return 1;
  }
}
pub trait QLayout_setGeometry_0<RetType> {
  fn setGeometry_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_setGeometry_0<(/*void*/)> for (usize) {
  fn setGeometry_0(self , rsthis: & QLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QLayout11setGeometryERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:122
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [8] QLayoutItem * itemAt(int) const

/*
Must be implemented in subclasses to return the layout item at index. If there is no such item, the function must return 0. Items are numbered consecutively from 0. If an item is deleted, other items will be renumbered.

This function can be used to iterate over a layout. The following code will draw a rectangle for each layout item in the layout structure of the widget.


  static void paintLayout(QPainter *painter, QLayoutItem *item)
  {
      QLayout *layout = item->layout();
      if (layout) {
          for (int i = 0; i < layout->count(); ++i)
              paintLayout(painter, layout->itemAt(i));
      }
      painter->drawRect(item->geometry());
  }

  void MyWidget::paintEvent(QPaintEvent *)
  {
      QPainter painter(this);
      if (layout())
          paintLayout(&painter, layout());
  }



See also count() and takeAt().
*/
impl /*struct*/ QLayout {
  pub fn itemAt_0<RetType, T: QLayout_itemAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemAt_0(self);
    // return 1;
  }
}
pub trait QLayout_itemAt_0<RetType> {
  fn itemAt_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_itemAt_0<usize> for (i32) {
  fn itemAt_0(self , rsthis: & QLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLayout6itemAtEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:123
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [8] QLayoutItem * takeAt(int)

/*
Must be implemented in subclasses to remove the layout item at index from the layout, and return the item. If there is no such item, the function must do nothing and return 0. Items are numbered consecutively from 0. If an item is removed, other items will be renumbered.

The following code fragment shows a safe way to remove all items from a layout:


  QLayoutItem *child;
  while ((child = layout->takeAt(0)) != 0) {
      ...
      delete child;
  }



See also itemAt() and count().
*/
impl /*struct*/ QLayout {
  pub fn takeAt_0<RetType, T: QLayout_takeAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.takeAt_0(self);
    // return 1;
  }
}
pub trait QLayout_takeAt_0<RetType> {
  fn takeAt_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_takeAt_0<usize> for (i32) {
  fn takeAt_0(self , rsthis: & QLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QLayout6takeAtEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:124
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int indexOf(QWidget *) const

/*
Searches for widget widget in this layout (not including child layouts).

Returns the index of widget, or -1 if widget is not found.

The default implementation iterates over all items using itemAt()
*/
impl /*struct*/ QLayout {
  pub fn indexOf_0<RetType, T: QLayout_indexOf_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexOf_0(self);
    // return 1;
  }
}
pub trait QLayout_indexOf_0<RetType> {
  fn indexOf_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_indexOf_0<i32> for (usize) {
  fn indexOf_0(self , rsthis: & QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLayout7indexOfEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:125
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [4] int count() const

/*
Must be implemented in subclasses to return the number of items in the layout.

See also itemAt().
*/
impl /*struct*/ QLayout {
  pub fn count_0<RetType, T: QLayout_count_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.count_0(self);
    // return 1;
  }
}
pub trait QLayout_count_0<RetType> {
  fn count_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_count_0<i32> for () {
  fn count_0(self , rsthis: & QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLayout5countEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:126
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool isEmpty() const

/*
Reimplemented from QLayoutItem::isEmpty().
*/
impl /*struct*/ QLayout {
  pub fn isEmpty_0<RetType, T: QLayout_isEmpty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEmpty_0(self);
    // return 1;
  }
}
pub trait QLayout_isEmpty_0<RetType> {
  fn isEmpty_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_isEmpty_0<bool> for () {
  fn isEmpty_0(self , rsthis: & QLayout) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLayout7isEmptyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:127
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] QSizePolicy::ControlTypes controlTypes() const

/*
Reimplemented from QLayoutItem::controlTypes().
*/
impl /*struct*/ QLayout {
  pub fn controlTypes_0<RetType, T: QLayout_controlTypes_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.controlTypes_0(self);
    // return 1;
  }
}
pub trait QLayout_controlTypes_0<RetType> {
  fn controlTypes_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_controlTypes_0<i32> for () {
  fn controlTypes_0(self , rsthis: & QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLayout12controlTypesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:130
// index:0
// Public Visibility=Default Availability=Available
// [8] QLayoutItem * replaceWidget(QWidget *, QWidget *, Qt::FindChildOptions)

/*
Searches for widget from and replaces it with widget to if found. Returns the layout item that contains the widget from on success. Otherwise 0 is returned. If options contains Qt::FindChildrenRecursively (the default), sub-layouts are searched for doing the replacement. Any other flag in options is ignored.

Notice that the returned item therefore might not belong to this layout, but to a sub-layout.

The returned layout item is no longer owned by the layout and should be either deleted or inserted to another layout. The widget from is no longer managed by the layout and may need to be deleted or hidden. The parent of widget from is left unchanged.

This function works for the built-in Qt layouts, but might not work for custom layouts.

This function was introduced in  Qt 5.2.

See also indexOf().
*/
impl /*struct*/ QLayout {
  pub fn replaceWidget_0<RetType, T: QLayout_replaceWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.replaceWidget_0(self);
    // return 1;
  }
}
pub trait QLayout_replaceWidget_0<RetType> {
  fn replaceWidget_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_replaceWidget_0<usize> for (usize,usize,i32) {
  fn replaceWidget_0(self , rsthis: & QLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QLayout13replaceWidgetEP7QWidgetS1_6QFlagsIN2Qt15FindChildOptionEE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:132
// index:0
// Public Visibility=Default Availability=Available
// [4] int totalHeightForWidth(int) const

/*

*/
impl /*struct*/ QLayout {
  pub fn totalHeightForWidth_0<RetType, T: QLayout_totalHeightForWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.totalHeightForWidth_0(self);
    // return 1;
  }
}
pub trait QLayout_totalHeightForWidth_0<RetType> {
  fn totalHeightForWidth_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_totalHeightForWidth_0<i32> for (i32) {
  fn totalHeightForWidth_0(self , rsthis: & QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLayout19totalHeightForWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:133
// index:0
// Public Visibility=Default Availability=Available
// [8] QSize totalMinimumSize() const

/*

*/
impl /*struct*/ QLayout {
  pub fn totalMinimumSize_0<RetType, T: QLayout_totalMinimumSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.totalMinimumSize_0(self);
    // return 1;
  }
}
pub trait QLayout_totalMinimumSize_0<RetType> {
  fn totalMinimumSize_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_totalMinimumSize_0<usize> for () {
  fn totalMinimumSize_0(self , rsthis: & QLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLayout16totalMinimumSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:134
// index:0
// Public Visibility=Default Availability=Available
// [8] QSize totalMaximumSize() const

/*

*/
impl /*struct*/ QLayout {
  pub fn totalMaximumSize_0<RetType, T: QLayout_totalMaximumSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.totalMaximumSize_0(self);
    // return 1;
  }
}
pub trait QLayout_totalMaximumSize_0<RetType> {
  fn totalMaximumSize_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_totalMaximumSize_0<usize> for () {
  fn totalMaximumSize_0(self , rsthis: & QLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLayout16totalMaximumSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:135
// index:0
// Public Visibility=Default Availability=Available
// [8] QSize totalSizeHint() const

/*

*/
impl /*struct*/ QLayout {
  pub fn totalSizeHint_0<RetType, T: QLayout_totalSizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.totalSizeHint_0(self);
    // return 1;
  }
}
pub trait QLayout_totalSizeHint_0<RetType> {
  fn totalSizeHint_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_totalSizeHint_0<usize> for () {
  fn totalSizeHint_0(self , rsthis: & QLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLayout13totalSizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:136
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QLayout * layout()

/*
Reimplemented from QLayoutItem::layout().
*/
impl /*struct*/ QLayout {
  pub fn layout_0<RetType, T: QLayout_layout_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.layout_0(self);
    // return 1;
  }
}
pub trait QLayout_layout_0<RetType> {
  fn layout_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_layout_0<usize> for () {
  fn layout_0(self , rsthis: & QLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QLayout6layoutEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:138
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setEnabled(bool)

/*
Enables this layout if enable is true, otherwise disables it.

An enabled layout adjusts dynamically to changes; a disabled layout acts as if it did not exist.

By default all layouts are enabled.

See also isEnabled().
*/
impl /*struct*/ QLayout {
  pub fn setEnabled_0<RetType, T: QLayout_setEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setEnabled_0(self);
    // return 1;
  }
}
pub trait QLayout_setEnabled_0<RetType> {
  fn setEnabled_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_setEnabled_0<(/*void*/)> for (bool) {
  fn setEnabled_0(self , rsthis: & QLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QLayout10setEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:139
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isEnabled() const

/*
Returns true if the layout is enabled; otherwise returns false.

See also setEnabled().
*/
impl /*struct*/ QLayout {
  pub fn isEnabled_0<RetType, T: QLayout_isEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEnabled_0(self);
    // return 1;
  }
}
pub trait QLayout_isEnabled_0<RetType> {
  fn isEnabled_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_isEnabled_0<bool> for () {
  fn isEnabled_0(self , rsthis: & QLayout) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLayout9isEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:142
// index:0
// Public static Visibility=Default Availability=Available
// [8] QSize closestAcceptableSize(const QWidget *, const QSize &)

/*
Returns a size that satisfies all size constraints on widget, including heightForWidth() and that is as close as possible to size.
*/
impl /*struct*/ QLayout {
  pub fn closestAcceptableSize_0<RetType, T: QLayout_closestAcceptableSize_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.closestAcceptableSize_0();
    // return 1;
  }
}
pub trait QLayout_closestAcceptableSize_0<RetType> {
  fn closestAcceptableSize_0(self ) -> RetType;
}
impl<'a> /*trait*/ QLayout_closestAcceptableSize_0<usize> for (usize,usize) {
  fn closestAcceptableSize_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QLayout21closestAcceptableSizeEPK7QWidgetRK5QSize", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:145
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void widgetEvent(QEvent *)

/*

*/
impl /*struct*/ QLayout {
  pub fn widgetEvent_0<RetType, T: QLayout_widgetEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.widgetEvent_0(self);
    // return 1;
  }
}
pub trait QLayout_widgetEvent_0<RetType> {
  fn widgetEvent_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_widgetEvent_0<(/*void*/)> for (usize) {
  fn widgetEvent_0(self , rsthis: & QLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QLayout11widgetEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:146
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void childEvent(QChildEvent *)

/*
Reimplemented from QObject::childEvent().
*/
impl /*struct*/ QLayout {
  pub fn childEvent_0<RetType, T: QLayout_childEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.childEvent_0(self);
    // return 1;
  }
}
pub trait QLayout_childEvent_0<RetType> {
  fn childEvent_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_childEvent_0<(/*void*/)> for (usize) {
  fn childEvent_0(self , rsthis: & QLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QLayout10childEventEP11QChildEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:147
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void addChildLayout(QLayout *)

/*
This function is called from addLayout() or insertLayout() functions in subclasses to add layout l as a sub-layout.

The only scenario in which you need to call it directly is if you implement a custom layout that supports nested layouts.

See also QBoxLayout::addLayout(), QBoxLayout::insertLayout(), and QGridLayout::addLayout().
*/
impl /*struct*/ QLayout {
  pub fn addChildLayout_0<RetType, T: QLayout_addChildLayout_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addChildLayout_0(self);
    // return 1;
  }
}
pub trait QLayout_addChildLayout_0<RetType> {
  fn addChildLayout_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_addChildLayout_0<(/*void*/)> for (usize) {
  fn addChildLayout_0(self , rsthis: & QLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QLayout14addChildLayoutEPS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:148
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void addChildWidget(QWidget *)

/*
This function is called from addWidget() functions in subclasses to add w as a managed widget of a layout.

If w is already managed by a layout, this function will give a warning and remove w from that layout. This function must therefore be called before adding w to the layout's data structure.
*/
impl /*struct*/ QLayout {
  pub fn addChildWidget_0<RetType, T: QLayout_addChildWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addChildWidget_0(self);
    // return 1;
  }
}
pub trait QLayout_addChildWidget_0<RetType> {
  fn addChildWidget_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_addChildWidget_0<(/*void*/)> for (usize) {
  fn addChildWidget_0(self , rsthis: & QLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QLayout14addChildWidgetEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:149
// index:0
// Protected Visibility=Default Availability=Available
// [1] bool adoptLayout(QLayout *)

/*

*/
impl /*struct*/ QLayout {
  pub fn adoptLayout_0<RetType, T: QLayout_adoptLayout_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.adoptLayout_0(self);
    // return 1;
  }
}
pub trait QLayout_adoptLayout_0<RetType> {
  fn adoptLayout_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_adoptLayout_0<bool> for (usize) {
  fn adoptLayout_0(self , rsthis: & QLayout) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QLayout11adoptLayoutEPS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlayout.h:151
// index:0
// Protected Visibility=Default Availability=Available
// [16] QRect alignmentRect(const QRect &) const

/*
Returns the rectangle that should be covered when the geometry of this layout is set to r, provided that this layout supports setAlignment().

The result is derived from sizeHint() and expanding(). It is never larger than r.
*/
impl /*struct*/ QLayout {
  pub fn alignmentRect_0<RetType, T: QLayout_alignmentRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.alignmentRect_0(self);
    // return 1;
  }
}
pub trait QLayout_alignmentRect_0<RetType> {
  fn alignmentRect_0(self , rsthis: & QLayout) -> RetType;
}
impl<'a> /*trait*/ QLayout_alignmentRect_0<usize> for (usize) {
  fn alignmentRect_0(self , rsthis: & QLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLayout13alignmentRectERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


/*
The possible values are:



See also setSizeConstraint().

*/
pub type QLayout__SizeConstraint = i32;
// The main widget's minimum size is set to minimumSize(), unless the widget already has a minimum size.
pub const QLayout__SetDefaultConstraint :QLayout__SizeConstraint = 0;
// The widget is not constrained.
pub const QLayout__SetNoConstraint :QLayout__SizeConstraint = 1;
// The main widget's minimum size is set to minimumSize(); it cannot be smaller.
pub const QLayout__SetMinimumSize :QLayout__SizeConstraint = 2;
// The main widget's size is set to sizeHint(); it cannot be resized at all.
pub const QLayout__SetFixedSize :QLayout__SizeConstraint = 3;
// The main widget's maximum size is set to maximumSize(); it cannot be larger.
pub const QLayout__SetMaximumSize :QLayout__SizeConstraint = 4;
// The main widget's minimum size is set to minimumSize() and its maximum size is set to maximumSize().
pub const QLayout__SetMinAndMaxSize :QLayout__SizeConstraint = 5;
pub fn QLayout_SizeConstraintItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QLayout", val);
}
pub fn QLayout_SizeConstraintItemName_s(val: i32) ->String {
  //var nilthis *QLayout
  //return nilthis.SizeConstraintItemName(val);
  return QLayout_SizeConstraintItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
