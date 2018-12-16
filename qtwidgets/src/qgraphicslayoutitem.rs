

// mod ::widgets::QGraphicsLayoutItem
// package qtwidgets
// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h
// #include <qgraphicslayoutitem.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 9
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

// void setGraphicsItem(QGraphicsItem *)
// func (this *QGraphicsLayoutItem) InheritSetGraphicsItem(f func(item *QGraphicsItem/*777 QGraphicsItem **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "setGraphicsItem", f)
// }

// void setOwnedByLayout(bool)
// func (this *QGraphicsLayoutItem) InheritSetOwnedByLayout(f func(ownedByLayout bool) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "setOwnedByLayout", f)
// }

// QSizeF sizeHint(Qt::SizeHint, const QSizeF &)
// func (this *QGraphicsLayoutItem) InheritSizeHint(f func(which int, constraint *qtcore.QSizeF) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "sizeHint", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QGraphicsLayoutItem)=16
pub struct QGraphicsLayoutItem {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGraphicsLayoutItem_ITF interface {
//    QGraphicsLayoutItem_PTR() *QGraphicsLayoutItem
//}
//func (ptr *QGraphicsLayoutItem) QGraphicsLayoutItem_PTR() *QGraphicsLayoutItem { return ptr }

impl /*struct*/ QGraphicsLayoutItem {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGraphicsLayoutItem {
    return QGraphicsLayoutItem{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGraphicsLayoutItem {
//  type Target = QGraphicsLayoutItemBASE;
//
//  fn deref(&self) -> &QGraphicsLayoutItemBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGraphicsLayoutItemBASE> for QGraphicsLayoutItem {
//  fn as_ref(& self) -> & QGraphicsLayoutItemBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h:57
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QGraphicsLayoutItem(QGraphicsLayoutItem *, bool)

/*
Constructs the QGraphicsLayoutItem object. parent becomes the object's parent. If isLayout is true the item is a layout, otherwise isLayout is false.
*/
// QGraphicsLayoutItem(QGraphicsLayoutItem *, bool) ctx.fn_proto_cpp
impl /*struct*/ QGraphicsLayoutItem {
  pub fn QGraphicsLayoutItem_0<T: QGraphicsLayoutItem_QGraphicsLayoutItem_0>(value: T) -> QGraphicsLayoutItem {
    let rsthis = value.QGraphicsLayoutItem_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_QGraphicsLayoutItem_0 {
  fn QGraphicsLayoutItem_0(self) -> QGraphicsLayoutItem;
}
// QGraphicsLayoutItem(QGraphicsLayoutItem *, bool) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGraphicsLayoutItem_QGraphicsLayoutItem_0 for (usize,bool) {
  fn QGraphicsLayoutItem_0(self) -> QGraphicsLayoutItem {
    // unsafe{_ZN19QGraphicsLayoutItemC2EPS_b()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const bool as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QGraphicsLayoutItemC2EPS_b", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGraphicsLayoutItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h:58
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QGraphicsLayoutItem()

/*

*/
pub fn DeleteQGraphicsLayoutItem(this :*mut QGraphicsLayoutItem) {
    // let rv = qtrt::InvokeQtFunc6("_ZN19QGraphicsLayoutItemD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h:60
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSizePolicy(const QSizePolicy &)

/*
Sets the size policy to policy. The size policy describes how the item should grow horizontally and vertically when arranged in a layout.

QGraphicsLayoutItem's default size policy is (QSizePolicy::Fixed, QSizePolicy::Fixed, QSizePolicy::DefaultType), but it is common for subclasses to change the default. For example, QGraphicsWidget defaults to (QSizePolicy::Preferred, QSizePolicy::Preferred, QSizePolicy::DefaultType).

See also sizePolicy() and QWidget::sizePolicy().
*/
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setSizePolicy_0<RetType, T: QGraphicsLayoutItem_setSizePolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSizePolicy_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayoutItem_setSizePolicy_0<RetType> {
  fn setSizePolicy_0(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayoutItem_setSizePolicy_0<(/*void*/)> for (usize) {
  fn setSizePolicy_0(self , rsthis: & QGraphicsLayoutItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsLayoutItem13setSizePolicyERK11QSizePolicy", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h:61
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setSizePolicy(QSizePolicy::Policy, QSizePolicy::Policy, QSizePolicy::ControlType)

/*
Sets the size policy to policy. The size policy describes how the item should grow horizontally and vertically when arranged in a layout.

QGraphicsLayoutItem's default size policy is (QSizePolicy::Fixed, QSizePolicy::Fixed, QSizePolicy::DefaultType), but it is common for subclasses to change the default. For example, QGraphicsWidget defaults to (QSizePolicy::Preferred, QSizePolicy::Preferred, QSizePolicy::DefaultType).

See also sizePolicy() and QWidget::sizePolicy().
*/
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setSizePolicy_1<RetType, T: QGraphicsLayoutItem_setSizePolicy_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSizePolicy_1(self);
    // return 1;
  }
}
pub trait QGraphicsLayoutItem_setSizePolicy_1<RetType> {
  fn setSizePolicy_1(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayoutItem_setSizePolicy_1<(/*void*/)> for (i32,i32,i32) {
  fn setSizePolicy_1(self , rsthis: & QGraphicsLayoutItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsLayoutItem13setSizePolicyEN11QSizePolicy6PolicyES1_NS0_11ControlTypeE", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h:62
// index:0
// Public Visibility=Default Availability=Available
// [4] QSizePolicy sizePolicy() const

/*
Returns the current size policy.

See also setSizePolicy() and QWidget::sizePolicy().
*/
impl /*struct*/ QGraphicsLayoutItem {
  pub fn sizePolicy_0<RetType, T: QGraphicsLayoutItem_sizePolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizePolicy_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayoutItem_sizePolicy_0<RetType> {
  fn sizePolicy_0(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayoutItem_sizePolicy_0<usize> for () {
  fn sizePolicy_0(self , rsthis: & QGraphicsLayoutItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsLayoutItem10sizePolicyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h:64
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMinimumSize(const QSizeF &)

/*
Sets the minimum size to size. This property overrides sizeHint() for Qt::MinimumSize and ensures that effectiveSizeHint() will never return a size smaller than size. In order to unset the minimum size, use an invalid size.

See also minimumSize(), maximumSize(), preferredSize(), Qt::MinimumSize, sizeHint(), setMinimumWidth(), and setMinimumHeight().
*/
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setMinimumSize_0<RetType, T: QGraphicsLayoutItem_setMinimumSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMinimumSize_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayoutItem_setMinimumSize_0<RetType> {
  fn setMinimumSize_0(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayoutItem_setMinimumSize_0<(/*void*/)> for (usize) {
  fn setMinimumSize_0(self , rsthis: & QGraphicsLayoutItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsLayoutItem14setMinimumSizeERK6QSizeF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h:65
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void setMinimumSize(qreal, qreal)

/*
Sets the minimum size to size. This property overrides sizeHint() for Qt::MinimumSize and ensures that effectiveSizeHint() will never return a size smaller than size. In order to unset the minimum size, use an invalid size.

See also minimumSize(), maximumSize(), preferredSize(), Qt::MinimumSize, sizeHint(), setMinimumWidth(), and setMinimumHeight().
*/
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setMinimumSize_1<RetType, T: QGraphicsLayoutItem_setMinimumSize_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMinimumSize_1(self);
    // return 1;
  }
}
pub trait QGraphicsLayoutItem_setMinimumSize_1<RetType> {
  fn setMinimumSize_1(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayoutItem_setMinimumSize_1<(/*void*/)> for (f64,f64) {
  fn setMinimumSize_1(self , rsthis: & QGraphicsLayoutItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsLayoutItem14setMinimumSizeEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h:66
// index:0
// Public Visibility=Default Availability=Available
// [16] QSizeF minimumSize() const

/*
Returns the minimum size.

See also setMinimumSize(), preferredSize(), maximumSize(), Qt::MinimumSize, and sizeHint().
*/
impl /*struct*/ QGraphicsLayoutItem {
  pub fn minimumSize_0<RetType, T: QGraphicsLayoutItem_minimumSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumSize_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayoutItem_minimumSize_0<RetType> {
  fn minimumSize_0(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayoutItem_minimumSize_0<usize> for () {
  fn minimumSize_0(self , rsthis: & QGraphicsLayoutItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsLayoutItem11minimumSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h:67
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMinimumWidth(qreal)

/*
Sets the minimum width to width.

See also minimumWidth(), setMinimumSize(), and minimumSize().
*/
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setMinimumWidth_0<RetType, T: QGraphicsLayoutItem_setMinimumWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMinimumWidth_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayoutItem_setMinimumWidth_0<RetType> {
  fn setMinimumWidth_0(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayoutItem_setMinimumWidth_0<(/*void*/)> for (f64) {
  fn setMinimumWidth_0(self , rsthis: & QGraphicsLayoutItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsLayoutItem15setMinimumWidthEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h:68
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal minimumWidth() const

/*
Returns the minimum width.

See also setMinimumWidth(), setMinimumSize(), and minimumSize().
*/
impl /*struct*/ QGraphicsLayoutItem {
  pub fn minimumWidth_0<RetType, T: QGraphicsLayoutItem_minimumWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumWidth_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayoutItem_minimumWidth_0<RetType> {
  fn minimumWidth_0(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayoutItem_minimumWidth_0<f64> for () {
  fn minimumWidth_0(self , rsthis: & QGraphicsLayoutItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsLayoutItem12minimumWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h:69
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMinimumHeight(qreal)

/*
Sets the minimum height to height.

See also minimumHeight(), setMinimumSize(), and minimumSize().
*/
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setMinimumHeight_0<RetType, T: QGraphicsLayoutItem_setMinimumHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMinimumHeight_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayoutItem_setMinimumHeight_0<RetType> {
  fn setMinimumHeight_0(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayoutItem_setMinimumHeight_0<(/*void*/)> for (f64) {
  fn setMinimumHeight_0(self , rsthis: & QGraphicsLayoutItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsLayoutItem16setMinimumHeightEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h:70
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal minimumHeight() const

/*
Returns the minimum height.

See also setMinimumHeight(), setMinimumSize(), and minimumSize().
*/
impl /*struct*/ QGraphicsLayoutItem {
  pub fn minimumHeight_0<RetType, T: QGraphicsLayoutItem_minimumHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumHeight_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayoutItem_minimumHeight_0<RetType> {
  fn minimumHeight_0(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayoutItem_minimumHeight_0<f64> for () {
  fn minimumHeight_0(self , rsthis: & QGraphicsLayoutItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsLayoutItem13minimumHeightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h:72
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPreferredSize(const QSizeF &)

/*
Sets the preferred size to size. This property overrides sizeHint() for Qt::PreferredSize and provides the default value for effectiveSizeHint(). In order to unset the preferred size, use an invalid size.

See also preferredSize(), minimumSize(), maximumSize(), Qt::PreferredSize, and sizeHint().
*/
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setPreferredSize_0<RetType, T: QGraphicsLayoutItem_setPreferredSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPreferredSize_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayoutItem_setPreferredSize_0<RetType> {
  fn setPreferredSize_0(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayoutItem_setPreferredSize_0<(/*void*/)> for (usize) {
  fn setPreferredSize_0(self , rsthis: & QGraphicsLayoutItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsLayoutItem16setPreferredSizeERK6QSizeF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h:73
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void setPreferredSize(qreal, qreal)

/*
Sets the preferred size to size. This property overrides sizeHint() for Qt::PreferredSize and provides the default value for effectiveSizeHint(). In order to unset the preferred size, use an invalid size.

See also preferredSize(), minimumSize(), maximumSize(), Qt::PreferredSize, and sizeHint().
*/
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setPreferredSize_1<RetType, T: QGraphicsLayoutItem_setPreferredSize_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPreferredSize_1(self);
    // return 1;
  }
}
pub trait QGraphicsLayoutItem_setPreferredSize_1<RetType> {
  fn setPreferredSize_1(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayoutItem_setPreferredSize_1<(/*void*/)> for (f64,f64) {
  fn setPreferredSize_1(self , rsthis: & QGraphicsLayoutItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsLayoutItem16setPreferredSizeEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h:74
// index:0
// Public Visibility=Default Availability=Available
// [16] QSizeF preferredSize() const

/*
Returns the preferred size.

See also setPreferredSize(), minimumSize(), maximumSize(), Qt::PreferredSize, and sizeHint().
*/
impl /*struct*/ QGraphicsLayoutItem {
  pub fn preferredSize_0<RetType, T: QGraphicsLayoutItem_preferredSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.preferredSize_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayoutItem_preferredSize_0<RetType> {
  fn preferredSize_0(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayoutItem_preferredSize_0<usize> for () {
  fn preferredSize_0(self , rsthis: & QGraphicsLayoutItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsLayoutItem13preferredSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h:75
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPreferredWidth(qreal)

/*
Sets the preferred width to width.

See also preferredWidth(), preferredHeight(), setPreferredSize(), and preferredSize().
*/
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setPreferredWidth_0<RetType, T: QGraphicsLayoutItem_setPreferredWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPreferredWidth_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayoutItem_setPreferredWidth_0<RetType> {
  fn setPreferredWidth_0(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayoutItem_setPreferredWidth_0<(/*void*/)> for (f64) {
  fn setPreferredWidth_0(self , rsthis: & QGraphicsLayoutItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsLayoutItem17setPreferredWidthEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h:76
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal preferredWidth() const

/*
Returns the preferred width.

See also setPreferredWidth(), setPreferredSize(), and preferredSize().
*/
impl /*struct*/ QGraphicsLayoutItem {
  pub fn preferredWidth_0<RetType, T: QGraphicsLayoutItem_preferredWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.preferredWidth_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayoutItem_preferredWidth_0<RetType> {
  fn preferredWidth_0(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayoutItem_preferredWidth_0<f64> for () {
  fn preferredWidth_0(self , rsthis: & QGraphicsLayoutItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsLayoutItem14preferredWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h:77
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPreferredHeight(qreal)

/*
Sets the preferred height to height.

See also preferredHeight(), preferredWidth(), setPreferredSize(), and preferredSize().
*/
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setPreferredHeight_0<RetType, T: QGraphicsLayoutItem_setPreferredHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPreferredHeight_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayoutItem_setPreferredHeight_0<RetType> {
  fn setPreferredHeight_0(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayoutItem_setPreferredHeight_0<(/*void*/)> for (f64) {
  fn setPreferredHeight_0(self , rsthis: & QGraphicsLayoutItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsLayoutItem18setPreferredHeightEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h:78
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal preferredHeight() const

/*
Returns the preferred height.

See also setPreferredHeight(), setPreferredSize(), and preferredSize().
*/
impl /*struct*/ QGraphicsLayoutItem {
  pub fn preferredHeight_0<RetType, T: QGraphicsLayoutItem_preferredHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.preferredHeight_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayoutItem_preferredHeight_0<RetType> {
  fn preferredHeight_0(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayoutItem_preferredHeight_0<f64> for () {
  fn preferredHeight_0(self , rsthis: & QGraphicsLayoutItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsLayoutItem15preferredHeightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h:80
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMaximumSize(const QSizeF &)

/*
Sets the maximum size to size. This property overrides sizeHint() for Qt::MaximumSize and ensures that effectiveSizeHint() will never return a size larger than size. In order to unset the maximum size, use an invalid size.

See also maximumSize(), minimumSize(), preferredSize(), Qt::MaximumSize, and sizeHint().
*/
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setMaximumSize_0<RetType, T: QGraphicsLayoutItem_setMaximumSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMaximumSize_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayoutItem_setMaximumSize_0<RetType> {
  fn setMaximumSize_0(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayoutItem_setMaximumSize_0<(/*void*/)> for (usize) {
  fn setMaximumSize_0(self , rsthis: & QGraphicsLayoutItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsLayoutItem14setMaximumSizeERK6QSizeF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h:81
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void setMaximumSize(qreal, qreal)

/*
Sets the maximum size to size. This property overrides sizeHint() for Qt::MaximumSize and ensures that effectiveSizeHint() will never return a size larger than size. In order to unset the maximum size, use an invalid size.

See also maximumSize(), minimumSize(), preferredSize(), Qt::MaximumSize, and sizeHint().
*/
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setMaximumSize_1<RetType, T: QGraphicsLayoutItem_setMaximumSize_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMaximumSize_1(self);
    // return 1;
  }
}
pub trait QGraphicsLayoutItem_setMaximumSize_1<RetType> {
  fn setMaximumSize_1(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayoutItem_setMaximumSize_1<(/*void*/)> for (f64,f64) {
  fn setMaximumSize_1(self , rsthis: & QGraphicsLayoutItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsLayoutItem14setMaximumSizeEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h:82
// index:0
// Public Visibility=Default Availability=Available
// [16] QSizeF maximumSize() const

/*
Returns the maximum size.

See also setMaximumSize(), minimumSize(), preferredSize(), Qt::MaximumSize, and sizeHint().
*/
impl /*struct*/ QGraphicsLayoutItem {
  pub fn maximumSize_0<RetType, T: QGraphicsLayoutItem_maximumSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maximumSize_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayoutItem_maximumSize_0<RetType> {
  fn maximumSize_0(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayoutItem_maximumSize_0<usize> for () {
  fn maximumSize_0(self , rsthis: & QGraphicsLayoutItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsLayoutItem11maximumSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h:83
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMaximumWidth(qreal)

/*
Sets the maximum width to width.

See also maximumWidth(), setMaximumSize(), and maximumSize().
*/
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setMaximumWidth_0<RetType, T: QGraphicsLayoutItem_setMaximumWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMaximumWidth_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayoutItem_setMaximumWidth_0<RetType> {
  fn setMaximumWidth_0(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayoutItem_setMaximumWidth_0<(/*void*/)> for (f64) {
  fn setMaximumWidth_0(self , rsthis: & QGraphicsLayoutItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsLayoutItem15setMaximumWidthEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h:84
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal maximumWidth() const

/*
Returns the maximum width.

See also setMaximumWidth(), setMaximumSize(), and maximumSize().
*/
impl /*struct*/ QGraphicsLayoutItem {
  pub fn maximumWidth_0<RetType, T: QGraphicsLayoutItem_maximumWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maximumWidth_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayoutItem_maximumWidth_0<RetType> {
  fn maximumWidth_0(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayoutItem_maximumWidth_0<f64> for () {
  fn maximumWidth_0(self , rsthis: & QGraphicsLayoutItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsLayoutItem12maximumWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h:85
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMaximumHeight(qreal)

/*
Sets the maximum height to height.

See also maximumHeight(), setMaximumSize(), and maximumSize().
*/
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setMaximumHeight_0<RetType, T: QGraphicsLayoutItem_setMaximumHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMaximumHeight_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayoutItem_setMaximumHeight_0<RetType> {
  fn setMaximumHeight_0(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayoutItem_setMaximumHeight_0<(/*void*/)> for (f64) {
  fn setMaximumHeight_0(self , rsthis: & QGraphicsLayoutItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsLayoutItem16setMaximumHeightEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h:86
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal maximumHeight() const

/*
Returns the maximum height.

See also setMaximumHeight(), setMaximumSize(), and maximumSize().
*/
impl /*struct*/ QGraphicsLayoutItem {
  pub fn maximumHeight_0<RetType, T: QGraphicsLayoutItem_maximumHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maximumHeight_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayoutItem_maximumHeight_0<RetType> {
  fn maximumHeight_0(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayoutItem_maximumHeight_0<f64> for () {
  fn maximumHeight_0(self , rsthis: & QGraphicsLayoutItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsLayoutItem13maximumHeightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h:88
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setGeometry(const QRectF &)

/*
This virtual function sets the geometry of the QGraphicsLayoutItem to rect, which is in parent coordinates (e.g., the top-left corner of rect is equivalent to the item's position in parent coordinates).

You must reimplement this function in a subclass of QGraphicsLayoutItem to receive geometry updates. The layout will call this function when it does a rearrangement.

If rect is outside of the bounds of minimumSize and maximumSize, it will be adjusted to its closest size so that it is within the legal bounds.

See also geometry().
*/
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setGeometry_0<RetType, T: QGraphicsLayoutItem_setGeometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setGeometry_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayoutItem_setGeometry_0<RetType> {
  fn setGeometry_0(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayoutItem_setGeometry_0<(/*void*/)> for (usize) {
  fn setGeometry_0(self , rsthis: & QGraphicsLayoutItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsLayoutItem11setGeometryERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h:89
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF geometry() const

/*
Returns the item's geometry (e.g., position and size) as a QRectF. This function is equivalent to QRectF(pos(), size()).

See also setGeometry().
*/
impl /*struct*/ QGraphicsLayoutItem {
  pub fn geometry_0<RetType, T: QGraphicsLayoutItem_geometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.geometry_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayoutItem_geometry_0<RetType> {
  fn geometry_0(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayoutItem_geometry_0<usize> for () {
  fn geometry_0(self , rsthis: & QGraphicsLayoutItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsLayoutItem8geometryEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h:90
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void getContentsMargins(qreal *, qreal *, qreal *, qreal *) const

/*
This virtual function provides the left, top, right and bottom contents margins for this QGraphicsLayoutItem. The default implementation assumes all contents margins are 0. The parameters point to values stored in qreals. If any of the pointers is 0, that value will not be updated.

See also QGraphicsWidget::setContentsMargins().
*/
impl /*struct*/ QGraphicsLayoutItem {
  pub fn getContentsMargins_0<RetType, T: QGraphicsLayoutItem_getContentsMargins_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.getContentsMargins_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayoutItem_getContentsMargins_0<RetType> {
  fn getContentsMargins_0(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayoutItem_getContentsMargins_0<(/*void*/)> for (usize,usize,usize,usize) {
  fn getContentsMargins_0(self , rsthis: & QGraphicsLayoutItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let arg3 = (&self.3) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK19QGraphicsLayoutItem18getContentsMarginsEPdS0_S0_S0_", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h:91
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF contentsRect() const

/*
Returns the contents rect in local coordinates.

The contents rect defines the subrectangle used by an associated layout when arranging subitems. This function is a convenience function that adjusts the item's geometry() by its contents margins. Note that getContentsMargins() is a virtual function that you can reimplement to return the item's contents margins.

See also getContentsMargins() and geometry().
*/
impl /*struct*/ QGraphicsLayoutItem {
  pub fn contentsRect_0<RetType, T: QGraphicsLayoutItem_contentsRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contentsRect_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayoutItem_contentsRect_0<RetType> {
  fn contentsRect_0(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayoutItem_contentsRect_0<usize> for () {
  fn contentsRect_0(self , rsthis: & QGraphicsLayoutItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsLayoutItem12contentsRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h:93
// index:0
// Public Visibility=Default Availability=Available
// [16] QSizeF effectiveSizeHint(Qt::SizeHint, const QSizeF &) const

/*
Returns the effective size hint for this QGraphicsLayoutItem.

which is the size hint in question. constraint is an optional argument that defines a special constrain when calculating the effective size hint. By default, constraint is QSizeF(-1, -1), which means there is no constraint to the size hint.

If you want to specify the widget's size hint for a given width or height, you can provide the fixed dimension in constraint. This is useful for widgets that can grow only either vertically or horizontally, and need to set either their width or their height to a special value.

For example, a text paragraph item fit into a column width of 200 may grow vertically. You can pass QSizeF(200, -1) as a constraint to get a suitable minimum, preferred and maximum height).

You can adjust the effective size hint by reimplementing sizeHint() in a QGraphicsLayoutItem subclass, or by calling one of the following functions: setMinimumSize(), setPreferredSize, or setMaximumSize() (or a combination of both).

This function caches each of the size hints and guarantees that sizeHint() will be called only once for each value of which - unless constraint is not specified and updateGeometry() has been called.

See also sizeHint().
*/
impl /*struct*/ QGraphicsLayoutItem {
  pub fn effectiveSizeHint_0<RetType, T: QGraphicsLayoutItem_effectiveSizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.effectiveSizeHint_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayoutItem_effectiveSizeHint_0<RetType> {
  fn effectiveSizeHint_0(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayoutItem_effectiveSizeHint_0<usize> for (i32,usize) {
  fn effectiveSizeHint_0(self , rsthis: & QGraphicsLayoutItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsLayoutItem17effectiveSizeHintEN2Qt8SizeHintERK6QSizeF", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h:95
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void updateGeometry()

/*
This virtual function discards any cached size hint information. You should always call this function if you change the return value of the sizeHint() function. Subclasses must always call the base implementation when reimplementing this function.

See also effectiveSizeHint().
*/
impl /*struct*/ QGraphicsLayoutItem {
  pub fn updateGeometry_0<RetType, T: QGraphicsLayoutItem_updateGeometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updateGeometry_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayoutItem_updateGeometry_0<RetType> {
  fn updateGeometry_0(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayoutItem_updateGeometry_0<(/*void*/)> for () {
  fn updateGeometry_0(self , rsthis: & QGraphicsLayoutItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN19QGraphicsLayoutItem14updateGeometryEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h:97
// index:0
// Public Visibility=Default Availability=Available
// [8] QGraphicsLayoutItem * parentLayoutItem() const

/*
Returns the parent of this QGraphicsLayoutItem, or 0 if there is no parent, or if the parent does not inherit from QGraphicsLayoutItem (QGraphicsLayoutItem is often used through multiple inheritance with QObject-derived classes).

See also setParentLayoutItem().
*/
impl /*struct*/ QGraphicsLayoutItem {
  pub fn parentLayoutItem_0<RetType, T: QGraphicsLayoutItem_parentLayoutItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.parentLayoutItem_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayoutItem_parentLayoutItem_0<RetType> {
  fn parentLayoutItem_0(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayoutItem_parentLayoutItem_0<usize> for () {
  fn parentLayoutItem_0(self , rsthis: & QGraphicsLayoutItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsLayoutItem16parentLayoutItemEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h:98
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setParentLayoutItem(QGraphicsLayoutItem *)

/*
Sets the parent of this QGraphicsLayoutItem to parent.

See also parentLayoutItem().
*/
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setParentLayoutItem_0<RetType, T: QGraphicsLayoutItem_setParentLayoutItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setParentLayoutItem_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayoutItem_setParentLayoutItem_0<RetType> {
  fn setParentLayoutItem_0(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayoutItem_setParentLayoutItem_0<(/*void*/)> for (usize) {
  fn setParentLayoutItem_0(self , rsthis: & QGraphicsLayoutItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsLayoutItem19setParentLayoutItemEPS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h:100
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isLayout() const

/*
Returns true if this QGraphicsLayoutItem is a layout (e.g., is inherited by an object that arranges other QGraphicsLayoutItem objects); otherwise returns false.

See also QGraphicsLayout.
*/
impl /*struct*/ QGraphicsLayoutItem {
  pub fn isLayout_0<RetType, T: QGraphicsLayoutItem_isLayout_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isLayout_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayoutItem_isLayout_0<RetType> {
  fn isLayout_0(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayoutItem_isLayout_0<bool> for () {
  fn isLayout_0(self , rsthis: & QGraphicsLayoutItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsLayoutItem8isLayoutEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h:101
// index:0
// Public Visibility=Default Availability=Available
// [8] QGraphicsItem * graphicsItem() const

/*
Returns the QGraphicsItem that this layout item represents. For QGraphicsWidget it will return itself. For custom items it can return an aggregated value.

See also setGraphicsItem().
*/
impl /*struct*/ QGraphicsLayoutItem {
  pub fn graphicsItem_0<RetType, T: QGraphicsLayoutItem_graphicsItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.graphicsItem_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayoutItem_graphicsItem_0<RetType> {
  fn graphicsItem_0(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayoutItem_graphicsItem_0<usize> for () {
  fn graphicsItem_0(self , rsthis: & QGraphicsLayoutItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsLayoutItem12graphicsItemEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h:102
// index:0
// Public Visibility=Default Availability=Available
// [1] bool ownedByLayout() const

/*
Returns whether a layout should delete this item in its destructor. If its true, then the layout will delete it. If its false, then it is assumed that another object has the ownership of it, and the layout won't delete this item.

If the item inherits both QGraphicsItem and QGraphicsLayoutItem (such as QGraphicsWidget does) the item is really part of two ownership hierarchies. This property informs what the layout should do with its child items when it is destructed. In the case of QGraphicsWidget, it is preferred that when the layout is deleted it won't delete its children (since they are also part of the graphics item hierarchy).

By default this value is initialized to false in QGraphicsLayoutItem, but it is overridden by QGraphicsLayout to return true. This is because QGraphicsLayout is not normally part of the QGraphicsItem hierarchy, so the parent layout should delete it. Subclasses might override this default behaviour by calling setOwnedByLayout(true).

This function was introduced in  Qt 4.6.

See also setOwnedByLayout().
*/
impl /*struct*/ QGraphicsLayoutItem {
  pub fn ownedByLayout_0<RetType, T: QGraphicsLayoutItem_ownedByLayout_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ownedByLayout_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayoutItem_ownedByLayout_0<RetType> {
  fn ownedByLayout_0(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayoutItem_ownedByLayout_0<bool> for () {
  fn ownedByLayout_0(self , rsthis: & QGraphicsLayoutItem) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsLayoutItem13ownedByLayoutEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h:105
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void setGraphicsItem(QGraphicsItem *)

/*
If the QGraphicsLayoutItem represents a QGraphicsItem, and it wants to take advantage of the automatic reparenting capabilities of QGraphicsLayout it should set this value. Note that if you delete item and not delete the layout item, you are responsible of calling setGraphicsItem(0) in order to avoid having a dangling pointer.

See also graphicsItem().
*/
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setGraphicsItem_0<RetType, T: QGraphicsLayoutItem_setGraphicsItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setGraphicsItem_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayoutItem_setGraphicsItem_0<RetType> {
  fn setGraphicsItem_0(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayoutItem_setGraphicsItem_0<(/*void*/)> for (usize) {
  fn setGraphicsItem_0(self , rsthis: & QGraphicsLayoutItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsLayoutItem15setGraphicsItemEP13QGraphicsItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h:106
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void setOwnedByLayout(bool)

/*
Sets whether a layout should delete this item in its destructor or not. ownership must be true to in order for the layout to delete it.

This function was introduced in  Qt 4.6.

See also ownedByLayout().
*/
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setOwnedByLayout_0<RetType, T: QGraphicsLayoutItem_setOwnedByLayout_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOwnedByLayout_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayoutItem_setOwnedByLayout_0<RetType> {
  fn setOwnedByLayout_0(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayoutItem_setOwnedByLayout_0<(/*void*/)> for (bool) {
  fn setOwnedByLayout_0(self , rsthis: & QGraphicsLayoutItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsLayoutItem16setOwnedByLayoutEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicslayoutitem.h:109
// index:0
// Protected purevirtual virtual Visibility=Default Availability=Available
// [16] QSizeF sizeHint(Qt::SizeHint, const QSizeF &) const

/*
This pure virtual function returns the size hint for which of the QGraphicsLayoutItem, using the width or height of constraint to constrain the output.

Reimplement this function in a subclass of QGraphicsLayoutItem to provide the necessary size hints for your items.

See also effectiveSizeHint().
*/
impl /*struct*/ QGraphicsLayoutItem {
  pub fn sizeHint_0<RetType, T: QGraphicsLayoutItem_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QGraphicsLayoutItem_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}
impl<'a> /*trait*/ QGraphicsLayoutItem_sizeHint_0<usize> for (i32,usize) {
  fn sizeHint_0(self , rsthis: & QGraphicsLayoutItem) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsLayoutItem8sizeHintEN2Qt8SizeHintERK6QSizeF", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
