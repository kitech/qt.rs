

// mod ::widgets::QBoxLayout
// package qtwidgets
// /usr/include/qt/QtWidgets/qboxlayout.h
// #include <qboxlayout.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 45
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
#[derive(Default)] // class sizeof(QBoxLayout)=32
pub struct QBoxLayout {
  qbase: QLayout,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QBoxLayout_ITF interface {
//    QLayout_ITF
//    QBoxLayout_PTR() *QBoxLayout
//}
//func (ptr *QBoxLayout) QBoxLayout_PTR() *QBoxLayout { return ptr }

impl /*struct*/ QBoxLayout {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QBoxLayout {
    return QBoxLayout{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QBoxLayout {
//  type Target = QBoxLayoutBASE;
//
//  fn deref(&self) -> &QBoxLayoutBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QBoxLayoutBASE> for QBoxLayout {
//  fn as_ref(& self) -> & QBoxLayoutBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qboxlayout.h:58
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QBoxLayout {
  pub fn metaObject_0<RetType, T: QBoxLayout_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QBoxLayout_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QBoxLayout) -> RetType;
}
impl<'a> /*trait*/ QBoxLayout_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QBoxLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QBoxLayout10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qboxlayout.h:64
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QBoxLayout(QBoxLayout::Direction, QWidget *)

/*
Constructs a new QBoxLayout with direction dir and parent widget parent.

See also direction().
*/
// QBoxLayout(QBoxLayout::Direction, QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QBoxLayout {
  pub fn QBoxLayout_0<T: QBoxLayout_QBoxLayout_0>(value: T) -> QBoxLayout {
    let rsthis = value.QBoxLayout_0();
    return rsthis;
    // return 1;
  }
}

pub trait QBoxLayout_QBoxLayout_0 {
  fn QBoxLayout_0(self) -> QBoxLayout;
}
// QBoxLayout(QBoxLayout::Direction, QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QBoxLayout_QBoxLayout_0 for (i32,usize) {
  fn QBoxLayout_0(self) -> QBoxLayout {
    // unsafe{_ZN10QBoxLayoutC2ENS_9DirectionEP7QWidget()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QBoxLayoutC2ENS_9DirectionEP7QWidget", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QBoxLayout{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qboxlayout.h:66
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QBoxLayout()

/*

*/
pub fn DeleteQBoxLayout(this :*mut QBoxLayout) {
    // let rv = qtrt::InvokeQtFunc6("_ZN10QBoxLayoutD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 32)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qboxlayout.h:68
// index:0
// Public Visibility=Default Availability=Available
// [4] QBoxLayout::Direction direction() const

/*
Returns the direction of the box. addWidget() and addSpacing() work in this direction; the stretch stretches in this direction.

See also setDirection(), QBoxLayout::Direction, addWidget(), and addSpacing().
*/
impl /*struct*/ QBoxLayout {
  pub fn direction_0<RetType, T: QBoxLayout_direction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.direction_0(self);
    // return 1;
  }
}
pub trait QBoxLayout_direction_0<RetType> {
  fn direction_0(self , rsthis: & QBoxLayout) -> RetType;
}
impl<'a> /*trait*/ QBoxLayout_direction_0<i32> for () {
  fn direction_0(self , rsthis: & QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QBoxLayout9directionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qboxlayout.h:69
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDirection(QBoxLayout::Direction)

/*
Sets the direction of this layout to direction.

See also direction().
*/
impl /*struct*/ QBoxLayout {
  pub fn setDirection_0<RetType, T: QBoxLayout_setDirection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDirection_0(self);
    // return 1;
  }
}
pub trait QBoxLayout_setDirection_0<RetType> {
  fn setDirection_0(self , rsthis: & QBoxLayout) -> RetType;
}
impl<'a> /*trait*/ QBoxLayout_setDirection_0<(/*void*/)> for (i32) {
  fn setDirection_0(self , rsthis: & QBoxLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QBoxLayout12setDirectionENS_9DirectionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qboxlayout.h:71
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addSpacing(int)

/*
Adds a non-stretchable space (a QSpacerItem) with size size to the end of this box layout. QBoxLayout provides default margin and spacing. This function adds additional space.

See also insertSpacing(), addItem(), and QSpacerItem.
*/
impl /*struct*/ QBoxLayout {
  pub fn addSpacing_0<RetType, T: QBoxLayout_addSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addSpacing_0(self);
    // return 1;
  }
}
pub trait QBoxLayout_addSpacing_0<RetType> {
  fn addSpacing_0(self , rsthis: & QBoxLayout) -> RetType;
}
impl<'a> /*trait*/ QBoxLayout_addSpacing_0<(/*void*/)> for (i32) {
  fn addSpacing_0(self , rsthis: & QBoxLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QBoxLayout10addSpacingEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qboxlayout.h:72
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addStretch(int)

/*
Adds a stretchable space (a QSpacerItem) with zero minimum size and stretch factor stretch to the end of this box layout.

See also insertStretch(), addItem(), and QSpacerItem.
*/
impl /*struct*/ QBoxLayout {
  pub fn addStretch_0<RetType, T: QBoxLayout_addStretch_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addStretch_0(self);
    // return 1;
  }
}
pub trait QBoxLayout_addStretch_0<RetType> {
  fn addStretch_0(self , rsthis: & QBoxLayout) -> RetType;
}
impl<'a> /*trait*/ QBoxLayout_addStretch_0<(/*void*/)> for (i32) {
  fn addStretch_0(self , rsthis: & QBoxLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QBoxLayout10addStretchEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qboxlayout.h:73
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addSpacerItem(QSpacerItem *)

/*
Adds spacerItem to the end of this box layout.

This function was introduced in  Qt 4.4.

See also addSpacing() and addStretch().
*/
impl /*struct*/ QBoxLayout {
  pub fn addSpacerItem_0<RetType, T: QBoxLayout_addSpacerItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addSpacerItem_0(self);
    // return 1;
  }
}
pub trait QBoxLayout_addSpacerItem_0<RetType> {
  fn addSpacerItem_0(self , rsthis: & QBoxLayout) -> RetType;
}
impl<'a> /*trait*/ QBoxLayout_addSpacerItem_0<(/*void*/)> for (usize) {
  fn addSpacerItem_0(self , rsthis: & QBoxLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QBoxLayout13addSpacerItemEP11QSpacerItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qboxlayout.h:74
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addWidget(QWidget *, int, Qt::Alignment)

/*
Adds widget to the end of this box layout, with a stretch factor of stretch and alignment alignment.

The stretch factor applies only in the direction of the QBoxLayout, and is relative to the other boxes and widgets in this QBoxLayout. Widgets and boxes with higher stretch factors grow more.

If the stretch factor is 0 and nothing else in the QBoxLayout has a stretch factor greater than zero, the space is distributed according to the QWidget:sizePolicy() of each widget that's involved.

The alignment is specified by alignment. The default alignment is 0, which means that the widget fills the entire cell.

See also insertWidget(), addItem(), addLayout(), addStretch(), addSpacing(), and addStrut().
*/
impl /*struct*/ QBoxLayout {
  pub fn addWidget_0<RetType, T: QBoxLayout_addWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addWidget_0(self);
    // return 1;
  }
}
pub trait QBoxLayout_addWidget_0<RetType> {
  fn addWidget_0(self , rsthis: & QBoxLayout) -> RetType;
}
impl<'a> /*trait*/ QBoxLayout_addWidget_0<(/*void*/)> for (usize,i32,i32) {
  fn addWidget_0(self , rsthis: & QBoxLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QBoxLayout9addWidgetEP7QWidgeti6QFlagsIN2Qt13AlignmentFlagEE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qboxlayout.h:75
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addLayout(QLayout *, int)

/*
Adds layout to the end of the box, with serial stretch factor stretch.

See also insertLayout(), addItem(), and addWidget().
*/
impl /*struct*/ QBoxLayout {
  pub fn addLayout_0<RetType, T: QBoxLayout_addLayout_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addLayout_0(self);
    // return 1;
  }
}
pub trait QBoxLayout_addLayout_0<RetType> {
  fn addLayout_0(self , rsthis: & QBoxLayout) -> RetType;
}
impl<'a> /*trait*/ QBoxLayout_addLayout_0<(/*void*/)> for (usize,i32) {
  fn addLayout_0(self , rsthis: & QBoxLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QBoxLayout9addLayoutEP7QLayouti", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qboxlayout.h:76
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addStrut(int)

/*
Limits the perpendicular dimension of the box (e.g. height if the box is LeftToRight) to a minimum of size. Other constraints may increase the limit.

See also addItem().
*/
impl /*struct*/ QBoxLayout {
  pub fn addStrut_0<RetType, T: QBoxLayout_addStrut_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addStrut_0(self);
    // return 1;
  }
}
pub trait QBoxLayout_addStrut_0<RetType> {
  fn addStrut_0(self , rsthis: & QBoxLayout) -> RetType;
}
impl<'a> /*trait*/ QBoxLayout_addStrut_0<(/*void*/)> for (i32) {
  fn addStrut_0(self , rsthis: & QBoxLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QBoxLayout8addStrutEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qboxlayout.h:77
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void addItem(QLayoutItem *)

/*
Reimplemented from QLayout::addItem().
*/
impl /*struct*/ QBoxLayout {
  pub fn addItem_0<RetType, T: QBoxLayout_addItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addItem_0(self);
    // return 1;
  }
}
pub trait QBoxLayout_addItem_0<RetType> {
  fn addItem_0(self , rsthis: & QBoxLayout) -> RetType;
}
impl<'a> /*trait*/ QBoxLayout_addItem_0<(/*void*/)> for (usize) {
  fn addItem_0(self , rsthis: & QBoxLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QBoxLayout7addItemEP11QLayoutItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qboxlayout.h:79
// index:0
// Public Visibility=Default Availability=Available
// [-2] void insertSpacing(int, int)

/*
Inserts a non-stretchable space (a QSpacerItem) at position index, with size size. If index is negative the space is added at the end.

The box layout has default margin and spacing. This function adds additional space.

See also addSpacing(), insertItem(), and QSpacerItem.
*/
impl /*struct*/ QBoxLayout {
  pub fn insertSpacing_0<RetType, T: QBoxLayout_insertSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertSpacing_0(self);
    // return 1;
  }
}
pub trait QBoxLayout_insertSpacing_0<RetType> {
  fn insertSpacing_0(self , rsthis: & QBoxLayout) -> RetType;
}
impl<'a> /*trait*/ QBoxLayout_insertSpacing_0<(/*void*/)> for (i32,i32) {
  fn insertSpacing_0(self , rsthis: & QBoxLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QBoxLayout13insertSpacingEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qboxlayout.h:80
// index:0
// Public Visibility=Default Availability=Available
// [-2] void insertStretch(int, int)

/*
Inserts a stretchable space (a QSpacerItem) at position index, with zero minimum size and stretch factor stretch. If index is negative the space is added at the end.

See also addStretch(), insertItem(), and QSpacerItem.
*/
impl /*struct*/ QBoxLayout {
  pub fn insertStretch_0<RetType, T: QBoxLayout_insertStretch_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertStretch_0(self);
    // return 1;
  }
}
pub trait QBoxLayout_insertStretch_0<RetType> {
  fn insertStretch_0(self , rsthis: & QBoxLayout) -> RetType;
}
impl<'a> /*trait*/ QBoxLayout_insertStretch_0<(/*void*/)> for (i32,i32) {
  fn insertStretch_0(self , rsthis: & QBoxLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QBoxLayout13insertStretchEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qboxlayout.h:81
// index:0
// Public Visibility=Default Availability=Available
// [-2] void insertSpacerItem(int, QSpacerItem *)

/*
Inserts spacerItem at position index, with zero minimum size and stretch factor. If index is negative the space is added at the end.

This function was introduced in  Qt 4.4.

See also addSpacerItem(), insertStretch(), and insertSpacing().
*/
impl /*struct*/ QBoxLayout {
  pub fn insertSpacerItem_0<RetType, T: QBoxLayout_insertSpacerItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertSpacerItem_0(self);
    // return 1;
  }
}
pub trait QBoxLayout_insertSpacerItem_0<RetType> {
  fn insertSpacerItem_0(self , rsthis: & QBoxLayout) -> RetType;
}
impl<'a> /*trait*/ QBoxLayout_insertSpacerItem_0<(/*void*/)> for (i32,usize) {
  fn insertSpacerItem_0(self , rsthis: & QBoxLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QBoxLayout16insertSpacerItemEiP11QSpacerItem", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qboxlayout.h:82
// index:0
// Public Visibility=Default Availability=Available
// [-2] void insertWidget(int, QWidget *, int, Qt::Alignment)

/*
Inserts widget at position index, with stretch factor stretch and alignment alignment. If index is negative, the widget is added at the end.

The stretch factor applies only in the direction of the QBoxLayout, and is relative to the other boxes and widgets in this QBoxLayout. Widgets and boxes with higher stretch factors grow more.

If the stretch factor is 0 and nothing else in the QBoxLayout has a stretch factor greater than zero, the space is distributed according to the QWidget:sizePolicy() of each widget that's involved.

The alignment is specified by alignment. The default alignment is 0, which means that the widget fills the entire cell.

See also addWidget() and insertItem().
*/
impl /*struct*/ QBoxLayout {
  pub fn insertWidget_0<RetType, T: QBoxLayout_insertWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertWidget_0(self);
    // return 1;
  }
}
pub trait QBoxLayout_insertWidget_0<RetType> {
  fn insertWidget_0(self , rsthis: & QBoxLayout) -> RetType;
}
impl<'a> /*trait*/ QBoxLayout_insertWidget_0<(/*void*/)> for (i32,usize,i32,i32) {
  fn insertWidget_0(self , rsthis: & QBoxLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QBoxLayout12insertWidgetEiP7QWidgeti6QFlagsIN2Qt13AlignmentFlagEE", 4,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qboxlayout.h:83
// index:0
// Public Visibility=Default Availability=Available
// [-2] void insertLayout(int, QLayout *, int)

/*
Inserts layout at position index, with stretch factor stretch. If index is negative, the layout is added at the end.

layout becomes a child of the box layout.

See also addLayout() and insertItem().
*/
impl /*struct*/ QBoxLayout {
  pub fn insertLayout_0<RetType, T: QBoxLayout_insertLayout_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertLayout_0(self);
    // return 1;
  }
}
pub trait QBoxLayout_insertLayout_0<RetType> {
  fn insertLayout_0(self , rsthis: & QBoxLayout) -> RetType;
}
impl<'a> /*trait*/ QBoxLayout_insertLayout_0<(/*void*/)> for (i32,usize,i32) {
  fn insertLayout_0(self , rsthis: & QBoxLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QBoxLayout12insertLayoutEiP7QLayouti", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qboxlayout.h:84
// index:0
// Public Visibility=Default Availability=Available
// [-2] void insertItem(int, QLayoutItem *)

/*
Inserts item into this box layout at position index. If index is negative, the item is added at the end.

See also addItem(), insertWidget(), insertLayout(), insertStretch(), and insertSpacing().
*/
impl /*struct*/ QBoxLayout {
  pub fn insertItem_0<RetType, T: QBoxLayout_insertItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertItem_0(self);
    // return 1;
  }
}
pub trait QBoxLayout_insertItem_0<RetType> {
  fn insertItem_0(self , rsthis: & QBoxLayout) -> RetType;
}
impl<'a> /*trait*/ QBoxLayout_insertItem_0<(/*void*/)> for (i32,usize) {
  fn insertItem_0(self , rsthis: & QBoxLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QBoxLayout10insertItemEiP11QLayoutItem", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qboxlayout.h:86
// index:0
// Public Visibility=Default Availability=Available
// [4] int spacing() const

/*
Reimplements QLayout::spacing(). If the spacing property is valid, that value is returned. Otherwise, a value for the spacing property is computed and returned. Since layout spacing in a widget is style dependent, if the parent is a widget, it queries the style for the (horizontal or vertical) spacing of the layout. Otherwise, the parent is a layout, and it queries the parent layout for the spacing().

See also QLayout::spacing() and setSpacing().
*/
impl /*struct*/ QBoxLayout {
  pub fn spacing_0<RetType, T: QBoxLayout_spacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.spacing_0(self);
    // return 1;
  }
}
pub trait QBoxLayout_spacing_0<RetType> {
  fn spacing_0(self , rsthis: & QBoxLayout) -> RetType;
}
impl<'a> /*trait*/ QBoxLayout_spacing_0<i32> for () {
  fn spacing_0(self , rsthis: & QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QBoxLayout7spacingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qboxlayout.h:87
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSpacing(int)

/*
Reimplements QLayout::setSpacing(). Sets the spacing property to spacing.

See also QLayout::setSpacing() and spacing().
*/
impl /*struct*/ QBoxLayout {
  pub fn setSpacing_0<RetType, T: QBoxLayout_setSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSpacing_0(self);
    // return 1;
  }
}
pub trait QBoxLayout_setSpacing_0<RetType> {
  fn setSpacing_0(self , rsthis: & QBoxLayout) -> RetType;
}
impl<'a> /*trait*/ QBoxLayout_setSpacing_0<(/*void*/)> for (i32) {
  fn setSpacing_0(self , rsthis: & QBoxLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QBoxLayout10setSpacingEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qboxlayout.h:89
// index:0
// Public Visibility=Default Availability=Available
// [1] bool setStretchFactor(QWidget *, int)

/*
Sets the stretch factor for widget to stretch and returns true if widget is found in this layout (not including child layouts); otherwise returns false.

See also setAlignment().
*/
impl /*struct*/ QBoxLayout {
  pub fn setStretchFactor_0<RetType, T: QBoxLayout_setStretchFactor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStretchFactor_0(self);
    // return 1;
  }
}
pub trait QBoxLayout_setStretchFactor_0<RetType> {
  fn setStretchFactor_0(self , rsthis: & QBoxLayout) -> RetType;
}
impl<'a> /*trait*/ QBoxLayout_setStretchFactor_0<bool> for (usize,i32) {
  fn setStretchFactor_0(self , rsthis: & QBoxLayout) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QBoxLayout16setStretchFactorEP7QWidgeti", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qboxlayout.h:90
// index:1
// Public Visibility=Default Availability=Available
// [1] bool setStretchFactor(QLayout *, int)

/*
Sets the stretch factor for widget to stretch and returns true if widget is found in this layout (not including child layouts); otherwise returns false.

See also setAlignment().
*/
impl /*struct*/ QBoxLayout {
  pub fn setStretchFactor_1<RetType, T: QBoxLayout_setStretchFactor_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStretchFactor_1(self);
    // return 1;
  }
}
pub trait QBoxLayout_setStretchFactor_1<RetType> {
  fn setStretchFactor_1(self , rsthis: & QBoxLayout) -> RetType;
}
impl<'a> /*trait*/ QBoxLayout_setStretchFactor_1<bool> for (usize,i32) {
  fn setStretchFactor_1(self , rsthis: & QBoxLayout) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QBoxLayout16setStretchFactorEP7QLayouti", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qboxlayout.h:91
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStretch(int, int)

/*
Sets the stretch factor at position index. to stretch.

This function was introduced in  Qt 4.5.

See also stretch().
*/
impl /*struct*/ QBoxLayout {
  pub fn setStretch_0<RetType, T: QBoxLayout_setStretch_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStretch_0(self);
    // return 1;
  }
}
pub trait QBoxLayout_setStretch_0<RetType> {
  fn setStretch_0(self , rsthis: & QBoxLayout) -> RetType;
}
impl<'a> /*trait*/ QBoxLayout_setStretch_0<(/*void*/)> for (i32,i32) {
  fn setStretch_0(self , rsthis: & QBoxLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QBoxLayout10setStretchEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qboxlayout.h:92
// index:0
// Public Visibility=Default Availability=Available
// [4] int stretch(int) const

/*
Returns the stretch factor at position index.

This function was introduced in  Qt 4.5.

See also setStretch().
*/
impl /*struct*/ QBoxLayout {
  pub fn stretch_0<RetType, T: QBoxLayout_stretch_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.stretch_0(self);
    // return 1;
  }
}
pub trait QBoxLayout_stretch_0<RetType> {
  fn stretch_0(self , rsthis: & QBoxLayout) -> RetType;
}
impl<'a> /*trait*/ QBoxLayout_stretch_0<i32> for (i32) {
  fn stretch_0(self , rsthis: & QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QBoxLayout7stretchEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qboxlayout.h:94
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Reimplemented from QLayoutItem::sizeHint().
*/
impl /*struct*/ QBoxLayout {
  pub fn sizeHint_0<RetType, T: QBoxLayout_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QBoxLayout_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QBoxLayout) -> RetType;
}
impl<'a> /*trait*/ QBoxLayout_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QBoxLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QBoxLayout8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qboxlayout.h:95
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize minimumSize() const

/*
Reimplemented from QLayoutItem::minimumSize().
*/
impl /*struct*/ QBoxLayout {
  pub fn minimumSize_0<RetType, T: QBoxLayout_minimumSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumSize_0(self);
    // return 1;
  }
}
pub trait QBoxLayout_minimumSize_0<RetType> {
  fn minimumSize_0(self , rsthis: & QBoxLayout) -> RetType;
}
impl<'a> /*trait*/ QBoxLayout_minimumSize_0<usize> for () {
  fn minimumSize_0(self , rsthis: & QBoxLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QBoxLayout11minimumSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qboxlayout.h:96
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize maximumSize() const

/*
Reimplemented from QLayoutItem::maximumSize().
*/
impl /*struct*/ QBoxLayout {
  pub fn maximumSize_0<RetType, T: QBoxLayout_maximumSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maximumSize_0(self);
    // return 1;
  }
}
pub trait QBoxLayout_maximumSize_0<RetType> {
  fn maximumSize_0(self , rsthis: & QBoxLayout) -> RetType;
}
impl<'a> /*trait*/ QBoxLayout_maximumSize_0<usize> for () {
  fn maximumSize_0(self , rsthis: & QBoxLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QBoxLayout11maximumSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qboxlayout.h:98
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool hasHeightForWidth() const

/*
Reimplemented from QLayoutItem::hasHeightForWidth().
*/
impl /*struct*/ QBoxLayout {
  pub fn hasHeightForWidth_0<RetType, T: QBoxLayout_hasHeightForWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasHeightForWidth_0(self);
    // return 1;
  }
}
pub trait QBoxLayout_hasHeightForWidth_0<RetType> {
  fn hasHeightForWidth_0(self , rsthis: & QBoxLayout) -> RetType;
}
impl<'a> /*trait*/ QBoxLayout_hasHeightForWidth_0<bool> for () {
  fn hasHeightForWidth_0(self , rsthis: & QBoxLayout) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QBoxLayout17hasHeightForWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qboxlayout.h:99
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int heightForWidth(int) const

/*
Reimplemented from QLayoutItem::heightForWidth().
*/
impl /*struct*/ QBoxLayout {
  pub fn heightForWidth_0<RetType, T: QBoxLayout_heightForWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.heightForWidth_0(self);
    // return 1;
  }
}
pub trait QBoxLayout_heightForWidth_0<RetType> {
  fn heightForWidth_0(self , rsthis: & QBoxLayout) -> RetType;
}
impl<'a> /*trait*/ QBoxLayout_heightForWidth_0<i32> for (i32) {
  fn heightForWidth_0(self , rsthis: & QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QBoxLayout14heightForWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qboxlayout.h:100
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int minimumHeightForWidth(int) const

/*
Reimplemented from QLayoutItem::minimumHeightForWidth().
*/
impl /*struct*/ QBoxLayout {
  pub fn minimumHeightForWidth_0<RetType, T: QBoxLayout_minimumHeightForWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumHeightForWidth_0(self);
    // return 1;
  }
}
pub trait QBoxLayout_minimumHeightForWidth_0<RetType> {
  fn minimumHeightForWidth_0(self , rsthis: & QBoxLayout) -> RetType;
}
impl<'a> /*trait*/ QBoxLayout_minimumHeightForWidth_0<i32> for (i32) {
  fn minimumHeightForWidth_0(self , rsthis: & QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QBoxLayout21minimumHeightForWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qboxlayout.h:102
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] Qt::Orientations expandingDirections() const

/*
Reimplemented from QLayoutItem::expandingDirections().
*/
impl /*struct*/ QBoxLayout {
  pub fn expandingDirections_0<RetType, T: QBoxLayout_expandingDirections_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.expandingDirections_0(self);
    // return 1;
  }
}
pub trait QBoxLayout_expandingDirections_0<RetType> {
  fn expandingDirections_0(self , rsthis: & QBoxLayout) -> RetType;
}
impl<'a> /*trait*/ QBoxLayout_expandingDirections_0<i32> for () {
  fn expandingDirections_0(self , rsthis: & QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QBoxLayout19expandingDirectionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qboxlayout.h:103
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void invalidate()

/*
Reimplemented from QLayoutItem::invalidate().

Resets cached information.
*/
impl /*struct*/ QBoxLayout {
  pub fn invalidate_0<RetType, T: QBoxLayout_invalidate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.invalidate_0(self);
    // return 1;
  }
}
pub trait QBoxLayout_invalidate_0<RetType> {
  fn invalidate_0(self , rsthis: & QBoxLayout) -> RetType;
}
impl<'a> /*trait*/ QBoxLayout_invalidate_0<(/*void*/)> for () {
  fn invalidate_0(self , rsthis: & QBoxLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QBoxLayout10invalidateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qboxlayout.h:104
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QLayoutItem * itemAt(int) const

/*
Reimplemented from QLayout::itemAt().
*/
impl /*struct*/ QBoxLayout {
  pub fn itemAt_0<RetType, T: QBoxLayout_itemAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemAt_0(self);
    // return 1;
  }
}
pub trait QBoxLayout_itemAt_0<RetType> {
  fn itemAt_0(self , rsthis: & QBoxLayout) -> RetType;
}
impl<'a> /*trait*/ QBoxLayout_itemAt_0<usize> for (i32) {
  fn itemAt_0(self , rsthis: & QBoxLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QBoxLayout6itemAtEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qboxlayout.h:105
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QLayoutItem * takeAt(int)

/*
Reimplemented from QLayout::takeAt().
*/
impl /*struct*/ QBoxLayout {
  pub fn takeAt_0<RetType, T: QBoxLayout_takeAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.takeAt_0(self);
    // return 1;
  }
}
pub trait QBoxLayout_takeAt_0<RetType> {
  fn takeAt_0(self , rsthis: & QBoxLayout) -> RetType;
}
impl<'a> /*trait*/ QBoxLayout_takeAt_0<usize> for (i32) {
  fn takeAt_0(self , rsthis: & QBoxLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QBoxLayout6takeAtEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qboxlayout.h:106
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int count() const

/*
Reimplemented from QLayout::count().
*/
impl /*struct*/ QBoxLayout {
  pub fn count_0<RetType, T: QBoxLayout_count_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.count_0(self);
    // return 1;
  }
}
pub trait QBoxLayout_count_0<RetType> {
  fn count_0(self , rsthis: & QBoxLayout) -> RetType;
}
impl<'a> /*trait*/ QBoxLayout_count_0<i32> for () {
  fn count_0(self , rsthis: & QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QBoxLayout5countEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qboxlayout.h:107
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setGeometry(const QRect &)

/*
Reimplemented from QLayoutItem::setGeometry().
*/
impl /*struct*/ QBoxLayout {
  pub fn setGeometry_0<RetType, T: QBoxLayout_setGeometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setGeometry_0(self);
    // return 1;
  }
}
pub trait QBoxLayout_setGeometry_0<RetType> {
  fn setGeometry_0(self , rsthis: & QBoxLayout) -> RetType;
}
impl<'a> /*trait*/ QBoxLayout_setGeometry_0<(/*void*/)> for (usize) {
  fn setGeometry_0(self , rsthis: & QBoxLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QBoxLayout11setGeometryERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
This type is used to determine the direction of a box layout.


*/
pub type QBoxLayout__Direction = i32;
// Horizontal from left to right.
pub const QBoxLayout__LeftToRight :QBoxLayout__Direction = 0;
// Horizontal from right to left.
pub const QBoxLayout__RightToLeft :QBoxLayout__Direction = 1;
// Vertical from top to bottom.
pub const QBoxLayout__TopToBottom :QBoxLayout__Direction = 2;
// Vertical from bottom to top.
pub const QBoxLayout__BottomToTop :QBoxLayout__Direction = 3;
// 
pub const QBoxLayout__Down :QBoxLayout__Direction = 2;
// 
pub const QBoxLayout__Up :QBoxLayout__Direction = 3;
pub fn QBoxLayout_DirectionItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QBoxLayout", val);
}
pub fn QBoxLayout_DirectionItemName_s(val: i32) ->String {
  //var nilthis *QBoxLayout
  //return nilthis.DirectionItemName(val);
  return QBoxLayout_DirectionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
