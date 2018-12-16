

// mod ::widgets::QSplitter
// package qtwidgets
// /usr/include/qt/QtWidgets/qsplitter.h
// #include <qsplitter.h>
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

// QSplitterHandle * createHandle()
// func (this *QSplitter) InheritCreateHandle(f func() unsafe.Pointer/*666*/) {
//  qtrt.SetAllInheritCallback(this, "createHandle", f)
// }

// void childEvent(QChildEvent *)
// func (this *QSplitter) InheritChildEvent(f func(arg0 *qtcore.QChildEvent/*777 QChildEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "childEvent", f)
// }

// bool event(QEvent *)
// func (this *QSplitter) InheritEvent(f func(arg0 *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// void resizeEvent(QResizeEvent *)
// func (this *QSplitter) InheritResizeEvent(f func(arg0 *qtgui.QResizeEvent/*777 QResizeEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "resizeEvent", f)
// }

// void changeEvent(QEvent *)
// func (this *QSplitter) InheritChangeEvent(f func(arg0 *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "changeEvent", f)
// }

// void moveSplitter(int, int)
// func (this *QSplitter) InheritMoveSplitter(f func(pos int, index int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "moveSplitter", f)
// }

// void setRubberBand(int)
// func (this *QSplitter) InheritSetRubberBand(f func(position int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "setRubberBand", f)
// }

// int closestLegalPosition(int, int)
// func (this *QSplitter) InheritClosestLegalPosition(f func(arg0 int, arg1 int) int) {
//  qtrt.SetAllInheritCallback(this, "closestLegalPosition", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QSplitter)=48
pub struct QSplitter {
  qbase: QFrame,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QSplitter_ITF interface {
//    QFrame_ITF
//    QSplitter_PTR() *QSplitter
//}
//func (ptr *QSplitter) QSplitter_PTR() *QSplitter { return ptr }

impl /*struct*/ QSplitter {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QSplitter {
    return QSplitter{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QSplitter {
//  type Target = QSplitterBASE;
//
//  fn deref(&self) -> &QSplitterBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QSplitterBASE> for QSplitter {
//  fn as_ref(& self) -> & QSplitterBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qsplitter.h:59
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QSplitter {
  pub fn metaObject_0<RetType, T: QSplitter_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QSplitter_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QSplitter) -> RetType;
}
impl<'a> /*trait*/ QSplitter_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QSplitter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QSplitter10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:67
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QSplitter(QWidget *)

/*
Constructs a horizontal splitter with the parent argument passed on to the QFrame constructor.

See also setOrientation().
*/
// QSplitter(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QSplitter {
  pub fn QSplitter_0<T: QSplitter_QSplitter_0>(value: T) -> QSplitter {
    let rsthis = value.QSplitter_0();
    return rsthis;
    // return 1;
  }
}

pub trait QSplitter_QSplitter_0 {
  fn QSplitter_0(self) -> QSplitter;
}
// QSplitter(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSplitter_QSplitter_0 for (usize) {
  fn QSplitter_0(self) -> QSplitter {
    // unsafe{_ZN9QSplitterC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QSplitterC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSplitter{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:68
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QSplitter(Qt::Orientation, QWidget *)

/*
Constructs a horizontal splitter with the parent argument passed on to the QFrame constructor.

See also setOrientation().
*/
// QSplitter(Qt::Orientation, QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QSplitter {
  pub fn QSplitter_1<T: QSplitter_QSplitter_1>(value: T) -> QSplitter {
    let rsthis = value.QSplitter_1();
    return rsthis;
    // return 1;
  }
}

pub trait QSplitter_QSplitter_1 {
  fn QSplitter_1(self) -> QSplitter;
}
// QSplitter(Qt::Orientation, QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSplitter_QSplitter_1 for (i32,usize) {
  fn QSplitter_1(self) -> QSplitter {
    // unsafe{_ZN9QSplitterC2EN2Qt11OrientationEP7QWidget()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QSplitterC2EN2Qt11OrientationEP7QWidget", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSplitter{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:69
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QSplitter()

/*

*/
pub fn DeleteQSplitter(this :*mut QSplitter) {
    // let rv = qtrt::InvokeQtFunc6("_ZN9QSplitterD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qsplitter.h:71
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addWidget(QWidget *)

/*
Adds the given widget to the splitter's layout after all the other items.

If widget is already in the splitter, it will be moved to the new position.

Note: The splitter takes ownership of the widget.

See also insertWidget(), widget(), and indexOf().
*/
impl /*struct*/ QSplitter {
  pub fn addWidget_0<RetType, T: QSplitter_addWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addWidget_0(self);
    // return 1;
  }
}
pub trait QSplitter_addWidget_0<RetType> {
  fn addWidget_0(self , rsthis: & QSplitter) -> RetType;
}
impl<'a> /*trait*/ QSplitter_addWidget_0<(/*void*/)> for (usize) {
  fn addWidget_0(self , rsthis: & QSplitter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QSplitter9addWidgetEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:72
// index:0
// Public Visibility=Default Availability=Available
// [-2] void insertWidget(int, QWidget *)

/*
Inserts the widget specified into the splitter's layout at the given index.

If widget is already in the splitter, it will be moved to the new position.

If index is an invalid index, then the widget will be inserted at the end.

Note: The splitter takes ownership of the widget.

See also addWidget(), indexOf(), and widget().
*/
impl /*struct*/ QSplitter {
  pub fn insertWidget_0<RetType, T: QSplitter_insertWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertWidget_0(self);
    // return 1;
  }
}
pub trait QSplitter_insertWidget_0<RetType> {
  fn insertWidget_0(self , rsthis: & QSplitter) -> RetType;
}
impl<'a> /*trait*/ QSplitter_insertWidget_0<(/*void*/)> for (i32,usize) {
  fn insertWidget_0(self , rsthis: & QSplitter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QSplitter12insertWidgetEiP7QWidget", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:73
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * replaceWidget(int, QWidget *)

/*
Replaces the widget in the splitter's layout at the given index by widget.

Returns the widget that has just been replaced if index is valid and widget is not already a child of the splitter. Otherwise, it returns null and no replacement or addition is made.

The geometry of the newly inserted widget will be the same as the widget it replaces. Its visible and collapsed states are also inherited.

Note: The splitter takes ownership of widget and sets the parent of the replaced widget to null.

Note: Because widget gets reparented into the splitter, its geometry may not be set right away, but only after widget will receive the appropriate events.

This function was introduced in  Qt 5.9.

See also insertWidget() and indexOf().
*/
impl /*struct*/ QSplitter {
  pub fn replaceWidget_0<RetType, T: QSplitter_replaceWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.replaceWidget_0(self);
    // return 1;
  }
}
pub trait QSplitter_replaceWidget_0<RetType> {
  fn replaceWidget_0(self , rsthis: & QSplitter) -> RetType;
}
impl<'a> /*trait*/ QSplitter_replaceWidget_0<usize> for (i32,usize) {
  fn replaceWidget_0(self , rsthis: & QSplitter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QSplitter13replaceWidgetEiP7QWidget", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:75
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOrientation(Qt::Orientation)

/*

*/
impl /*struct*/ QSplitter {
  pub fn setOrientation_0<RetType, T: QSplitter_setOrientation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOrientation_0(self);
    // return 1;
  }
}
pub trait QSplitter_setOrientation_0<RetType> {
  fn setOrientation_0(self , rsthis: & QSplitter) -> RetType;
}
impl<'a> /*trait*/ QSplitter_setOrientation_0<(/*void*/)> for (i32) {
  fn setOrientation_0(self , rsthis: & QSplitter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QSplitter14setOrientationEN2Qt11OrientationE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:76
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::Orientation orientation() const

/*

*/
impl /*struct*/ QSplitter {
  pub fn orientation_0<RetType, T: QSplitter_orientation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.orientation_0(self);
    // return 1;
  }
}
pub trait QSplitter_orientation_0<RetType> {
  fn orientation_0(self , rsthis: & QSplitter) -> RetType;
}
impl<'a> /*trait*/ QSplitter_orientation_0<i32> for () {
  fn orientation_0(self , rsthis: & QSplitter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QSplitter11orientationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:78
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setChildrenCollapsible(bool)

/*

*/
impl /*struct*/ QSplitter {
  pub fn setChildrenCollapsible_0<RetType, T: QSplitter_setChildrenCollapsible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setChildrenCollapsible_0(self);
    // return 1;
  }
}
pub trait QSplitter_setChildrenCollapsible_0<RetType> {
  fn setChildrenCollapsible_0(self , rsthis: & QSplitter) -> RetType;
}
impl<'a> /*trait*/ QSplitter_setChildrenCollapsible_0<(/*void*/)> for (bool) {
  fn setChildrenCollapsible_0(self , rsthis: & QSplitter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QSplitter22setChildrenCollapsibleEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:79
// index:0
// Public Visibility=Default Availability=Available
// [1] bool childrenCollapsible() const

/*

*/
impl /*struct*/ QSplitter {
  pub fn childrenCollapsible_0<RetType, T: QSplitter_childrenCollapsible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.childrenCollapsible_0(self);
    // return 1;
  }
}
pub trait QSplitter_childrenCollapsible_0<RetType> {
  fn childrenCollapsible_0(self , rsthis: & QSplitter) -> RetType;
}
impl<'a> /*trait*/ QSplitter_childrenCollapsible_0<bool> for () {
  fn childrenCollapsible_0(self , rsthis: & QSplitter) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QSplitter19childrenCollapsibleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:81
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCollapsible(int, bool)

/*
Sets whether the child widget at index is collapsible to collapse.

By default, children are collapsible, meaning that the user can resize them down to size 0, even if they have a non-zero minimumSize() or minimumSizeHint(). This behavior can be changed on a per-widget basis by calling this function, or globally for all the widgets in the splitter by setting the childrenCollapsible property.

See also isCollapsible() and childrenCollapsible.
*/
impl /*struct*/ QSplitter {
  pub fn setCollapsible_0<RetType, T: QSplitter_setCollapsible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCollapsible_0(self);
    // return 1;
  }
}
pub trait QSplitter_setCollapsible_0<RetType> {
  fn setCollapsible_0(self , rsthis: & QSplitter) -> RetType;
}
impl<'a> /*trait*/ QSplitter_setCollapsible_0<(/*void*/)> for (i32,bool) {
  fn setCollapsible_0(self , rsthis: & QSplitter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QSplitter14setCollapsibleEib", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:82
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isCollapsible(int) const

/*
Returns true if the widget at index is collapsible, otherwise returns false.
*/
impl /*struct*/ QSplitter {
  pub fn isCollapsible_0<RetType, T: QSplitter_isCollapsible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isCollapsible_0(self);
    // return 1;
  }
}
pub trait QSplitter_isCollapsible_0<RetType> {
  fn isCollapsible_0(self , rsthis: & QSplitter) -> RetType;
}
impl<'a> /*trait*/ QSplitter_isCollapsible_0<bool> for (i32) {
  fn isCollapsible_0(self , rsthis: & QSplitter) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QSplitter13isCollapsibleEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:83
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOpaqueResize(bool)

/*

*/
impl /*struct*/ QSplitter {
  pub fn setOpaqueResize_0<RetType, T: QSplitter_setOpaqueResize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOpaqueResize_0(self);
    // return 1;
  }
}
pub trait QSplitter_setOpaqueResize_0<RetType> {
  fn setOpaqueResize_0(self , rsthis: & QSplitter) -> RetType;
}
impl<'a> /*trait*/ QSplitter_setOpaqueResize_0<(/*void*/)> for (bool) {
  fn setOpaqueResize_0(self , rsthis: & QSplitter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QSplitter15setOpaqueResizeEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:84
// index:0
// Public Visibility=Default Availability=Available
// [1] bool opaqueResize() const

/*

*/
impl /*struct*/ QSplitter {
  pub fn opaqueResize_0<RetType, T: QSplitter_opaqueResize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.opaqueResize_0(self);
    // return 1;
  }
}
pub trait QSplitter_opaqueResize_0<RetType> {
  fn opaqueResize_0(self , rsthis: & QSplitter) -> RetType;
}
impl<'a> /*trait*/ QSplitter_opaqueResize_0<bool> for () {
  fn opaqueResize_0(self , rsthis: & QSplitter) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QSplitter12opaqueResizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:85
// index:0
// Public Visibility=Default Availability=Available
// [-2] void refresh()

/*
Updates the splitter's state. You should not need to call this function.
*/
impl /*struct*/ QSplitter {
  pub fn refresh_0<RetType, T: QSplitter_refresh_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.refresh_0(self);
    // return 1;
  }
}
pub trait QSplitter_refresh_0<RetType> {
  fn refresh_0(self , rsthis: & QSplitter) -> RetType;
}
impl<'a> /*trait*/ QSplitter_refresh_0<(/*void*/)> for () {
  fn refresh_0(self , rsthis: & QSplitter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QSplitter7refreshEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:87
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Reimplemented from QWidget::sizeHint().
*/
impl /*struct*/ QSplitter {
  pub fn sizeHint_0<RetType, T: QSplitter_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QSplitter_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QSplitter) -> RetType;
}
impl<'a> /*trait*/ QSplitter_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QSplitter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QSplitter8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:88
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize minimumSizeHint() const

/*
Reimplemented from QWidget::minimumSizeHint().
*/
impl /*struct*/ QSplitter {
  pub fn minimumSizeHint_0<RetType, T: QSplitter_minimumSizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint_0(self);
    // return 1;
  }
}
pub trait QSplitter_minimumSizeHint_0<RetType> {
  fn minimumSizeHint_0(self , rsthis: & QSplitter) -> RetType;
}
impl<'a> /*trait*/ QSplitter_minimumSizeHint_0<usize> for () {
  fn minimumSizeHint_0(self , rsthis: & QSplitter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QSplitter15minimumSizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:93
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray saveState() const

/*
Saves the state of the splitter's layout.

Typically this is used in conjunction with QSettings to remember the size for a future session. A version number is stored as part of the data. Here is an example:


      QSettings settings;
      settings.setValue("splitterSizes", splitter->saveState());



See also restoreState().
*/
impl /*struct*/ QSplitter {
  pub fn saveState_0<RetType, T: QSplitter_saveState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.saveState_0(self);
    // return 1;
  }
}
pub trait QSplitter_saveState_0<RetType> {
  fn saveState_0(self , rsthis: & QSplitter) -> RetType;
}
impl<'a> /*trait*/ QSplitter_saveState_0<usize> for () {
  fn saveState_0(self , rsthis: & QSplitter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QSplitter9saveStateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:94
// index:0
// Public Visibility=Default Availability=Available
// [1] bool restoreState(const QByteArray &)

/*
Restores the splitter's layout to the state specified. Returns true if the state is restored; otherwise returns false.

Typically this is used in conjunction with QSettings to restore the size from a past session. Here is an example:

Restore the splitter's state:


      QSettings settings;
      splitter->restoreState(settings.value("splitterSizes").toByteArray());



A failure to restore the splitter's layout may result from either invalid or out-of-date data in the supplied byte array.

See also saveState().
*/
impl /*struct*/ QSplitter {
  pub fn restoreState_0<RetType, T: QSplitter_restoreState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.restoreState_0(self);
    // return 1;
  }
}
pub trait QSplitter_restoreState_0<RetType> {
  fn restoreState_0(self , rsthis: & QSplitter) -> RetType;
}
impl<'a> /*trait*/ QSplitter_restoreState_0<bool> for (usize) {
  fn restoreState_0(self , rsthis: & QSplitter) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QSplitter12restoreStateERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:96
// index:0
// Public Visibility=Default Availability=Available
// [4] int handleWidth() const

/*

*/
impl /*struct*/ QSplitter {
  pub fn handleWidth_0<RetType, T: QSplitter_handleWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.handleWidth_0(self);
    // return 1;
  }
}
pub trait QSplitter_handleWidth_0<RetType> {
  fn handleWidth_0(self , rsthis: & QSplitter) -> RetType;
}
impl<'a> /*trait*/ QSplitter_handleWidth_0<i32> for () {
  fn handleWidth_0(self , rsthis: & QSplitter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QSplitter11handleWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:97
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setHandleWidth(int)

/*

*/
impl /*struct*/ QSplitter {
  pub fn setHandleWidth_0<RetType, T: QSplitter_setHandleWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHandleWidth_0(self);
    // return 1;
  }
}
pub trait QSplitter_setHandleWidth_0<RetType> {
  fn setHandleWidth_0(self , rsthis: & QSplitter) -> RetType;
}
impl<'a> /*trait*/ QSplitter_setHandleWidth_0<(/*void*/)> for (i32) {
  fn setHandleWidth_0(self , rsthis: & QSplitter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QSplitter14setHandleWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:99
// index:0
// Public Visibility=Default Availability=Available
// [4] int indexOf(QWidget *) const

/*
Returns the index in the splitter's layout of the specified widget. This also works for handles.

Handles are numbered from 0. There are as many handles as there are child widgets, but the handle at position 0 is always hidden.

See also count() and widget().
*/
impl /*struct*/ QSplitter {
  pub fn indexOf_0<RetType, T: QSplitter_indexOf_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexOf_0(self);
    // return 1;
  }
}
pub trait QSplitter_indexOf_0<RetType> {
  fn indexOf_0(self , rsthis: & QSplitter) -> RetType;
}
impl<'a> /*trait*/ QSplitter_indexOf_0<i32> for (usize) {
  fn indexOf_0(self , rsthis: & QSplitter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QSplitter7indexOfEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:100
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * widget(int) const

/*
Returns the widget at the given index in the splitter's layout.

See also count(), handle(), indexOf(), and insertWidget().
*/
impl /*struct*/ QSplitter {
  pub fn widget_0<RetType, T: QSplitter_widget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.widget_0(self);
    // return 1;
  }
}
pub trait QSplitter_widget_0<RetType> {
  fn widget_0(self , rsthis: & QSplitter) -> RetType;
}
impl<'a> /*trait*/ QSplitter_widget_0<usize> for (i32) {
  fn widget_0(self , rsthis: & QSplitter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QSplitter6widgetEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:101
// index:0
// Public Visibility=Default Availability=Available
// [4] int count() const

/*
Returns the number of widgets contained in the splitter's layout.

See also widget() and handle().
*/
impl /*struct*/ QSplitter {
  pub fn count_0<RetType, T: QSplitter_count_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.count_0(self);
    // return 1;
  }
}
pub trait QSplitter_count_0<RetType> {
  fn count_0(self , rsthis: & QSplitter) -> RetType;
}
impl<'a> /*trait*/ QSplitter_count_0<i32> for () {
  fn count_0(self , rsthis: & QSplitter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QSplitter5countEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:103
// index:0
// Public Visibility=Default Availability=Available
// [-2] void getRange(int, int *, int *) const

/*
Returns the valid range of the splitter at index in *min and *max if min and max are not 0.
*/
impl /*struct*/ QSplitter {
  pub fn getRange_0<RetType, T: QSplitter_getRange_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.getRange_0(self);
    // return 1;
  }
}
pub trait QSplitter_getRange_0<RetType> {
  fn getRange_0(self , rsthis: & QSplitter) -> RetType;
}
impl<'a> /*trait*/ QSplitter_getRange_0<(/*void*/)> for (i32,usize,usize) {
  fn getRange_0(self , rsthis: & QSplitter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK9QSplitter8getRangeEiPiS0_", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:104
// index:0
// Public Visibility=Default Availability=Available
// [8] QSplitterHandle * handle(int) const

/*
Returns the handle to the left (or above) for the item in the splitter's layout at the given index. The handle at index 0 is always hidden.

For right-to-left languages such as Arabic and Hebrew, the layout of horizontal splitters is reversed. The handle will be to the right of the widget at index.

See also count(), widget(), indexOf(), createHandle(), and setHandleWidth().
*/
impl /*struct*/ QSplitter {
  pub fn handle_0<RetType, T: QSplitter_handle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.handle_0(self);
    // return 1;
  }
}
pub trait QSplitter_handle_0<RetType> {
  fn handle_0(self , rsthis: & QSplitter) -> RetType;
}
impl<'a> /*trait*/ QSplitter_handle_0<usize> for (i32) {
  fn handle_0(self , rsthis: & QSplitter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QSplitter6handleEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:106
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStretchFactor(int, int)

/*
Updates the size policy of the widget at position index to have a stretch factor of stretch.

stretch is not the effective stretch factor; the effective stretch factor is calculated by taking the initial size of the widget and multiplying it with stretch.

This function is provided for convenience. It is equivalent to


  QWidget *widget = splitter->widget(index);
  QSizePolicy policy = widget->sizePolicy();
  policy.setHorizontalStretch(stretch);
  policy.setVerticalStretch(stretch);
  widget->setSizePolicy(policy);



See also setSizes() and widget().
*/
impl /*struct*/ QSplitter {
  pub fn setStretchFactor_0<RetType, T: QSplitter_setStretchFactor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStretchFactor_0(self);
    // return 1;
  }
}
pub trait QSplitter_setStretchFactor_0<RetType> {
  fn setStretchFactor_0(self , rsthis: & QSplitter) -> RetType;
}
impl<'a> /*trait*/ QSplitter_setStretchFactor_0<(/*void*/)> for (i32,i32) {
  fn setStretchFactor_0(self , rsthis: & QSplitter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QSplitter16setStretchFactorEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:109
// index:0
// Public Visibility=Default Availability=Available
// [-2] void splitterMoved(int, int)

/*
This signal is emitted when the splitter handle at a particular index has been moved to position pos.

For right-to-left languages such as Arabic and Hebrew, the layout of horizontal splitters is reversed. pos is then the distance from the right edge of the widget.

See also moveSplitter().
*/
impl /*struct*/ QSplitter {
  pub fn splitterMoved_0<RetType, T: QSplitter_splitterMoved_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.splitterMoved_0(self);
    // return 1;
  }
}
pub trait QSplitter_splitterMoved_0<RetType> {
  fn splitterMoved_0(self , rsthis: & QSplitter) -> RetType;
}
impl<'a> /*trait*/ QSplitter_splitterMoved_0<(/*void*/)> for (i32,i32) {
  fn splitterMoved_0(self , rsthis: & QSplitter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QSplitter13splitterMovedEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:112
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] QSplitterHandle * createHandle()

/*
Returns a new splitter handle as a child widget of this splitter. This function can be reimplemented in subclasses to provide support for custom handles.

See also handle() and indexOf().
*/
impl /*struct*/ QSplitter {
  pub fn createHandle_0<RetType, T: QSplitter_createHandle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.createHandle_0(self);
    // return 1;
  }
}
pub trait QSplitter_createHandle_0<RetType> {
  fn createHandle_0(self , rsthis: & QSplitter) -> RetType;
}
impl<'a> /*trait*/ QSplitter_createHandle_0<usize> for () {
  fn createHandle_0(self , rsthis: & QSplitter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QSplitter12createHandleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:114
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void childEvent(QChildEvent *)

/*
Reimplemented from QObject::childEvent().

Tells the splitter that the child widget described by c has been inserted or removed.

This method is also used to handle the situation where a widget is created with the splitter as a parent but not explicitly added with insertWidget() or addWidget(). This is for compatibility and not the recommended way of putting widgets into a splitter in new code. Please use insertWidget() or addWidget() in new code.

See also addWidget() and insertWidget().
*/
impl /*struct*/ QSplitter {
  pub fn childEvent_0<RetType, T: QSplitter_childEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.childEvent_0(self);
    // return 1;
  }
}
pub trait QSplitter_childEvent_0<RetType> {
  fn childEvent_0(self , rsthis: & QSplitter) -> RetType;
}
impl<'a> /*trait*/ QSplitter_childEvent_0<(/*void*/)> for (usize) {
  fn childEvent_0(self , rsthis: & QSplitter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QSplitter10childEventEP11QChildEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:116
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QSplitter {
  pub fn event_0<RetType, T: QSplitter_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QSplitter_event_0<RetType> {
  fn event_0(self , rsthis: & QSplitter) -> RetType;
}
impl<'a> /*trait*/ QSplitter_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QSplitter) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QSplitter5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:117
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void resizeEvent(QResizeEvent *)

/*
Reimplemented from QWidget::resizeEvent().
*/
impl /*struct*/ QSplitter {
  pub fn resizeEvent_0<RetType, T: QSplitter_resizeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeEvent_0(self);
    // return 1;
  }
}
pub trait QSplitter_resizeEvent_0<RetType> {
  fn resizeEvent_0(self , rsthis: & QSplitter) -> RetType;
}
impl<'a> /*trait*/ QSplitter_resizeEvent_0<(/*void*/)> for (usize) {
  fn resizeEvent_0(self , rsthis: & QSplitter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QSplitter11resizeEventEP12QResizeEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:119
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void changeEvent(QEvent *)

/*
Reimplemented from QWidget::changeEvent().
*/
impl /*struct*/ QSplitter {
  pub fn changeEvent_0<RetType, T: QSplitter_changeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changeEvent_0(self);
    // return 1;
  }
}
pub trait QSplitter_changeEvent_0<RetType> {
  fn changeEvent_0(self , rsthis: & QSplitter) -> RetType;
}
impl<'a> /*trait*/ QSplitter_changeEvent_0<(/*void*/)> for (usize) {
  fn changeEvent_0(self , rsthis: & QSplitter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QSplitter11changeEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:120
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void moveSplitter(int, int)

/*
Moves the left or top edge of the splitter handle at index as close as possible to position pos, which is the distance from the left or top edge of the widget.

For right-to-left languages such as Arabic and Hebrew, the layout of horizontal splitters is reversed. pos is then the distance from the right edge of the widget.

See also splitterMoved(), closestLegalPosition(), and getRange().
*/
impl /*struct*/ QSplitter {
  pub fn moveSplitter_0<RetType, T: QSplitter_moveSplitter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveSplitter_0(self);
    // return 1;
  }
}
pub trait QSplitter_moveSplitter_0<RetType> {
  fn moveSplitter_0(self , rsthis: & QSplitter) -> RetType;
}
impl<'a> /*trait*/ QSplitter_moveSplitter_0<(/*void*/)> for (i32,i32) {
  fn moveSplitter_0(self , rsthis: & QSplitter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QSplitter12moveSplitterEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:121
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void setRubberBand(int)

/*
Displays a rubber band at position pos. If pos is negative, the rubber band is removed.
*/
impl /*struct*/ QSplitter {
  pub fn setRubberBand_0<RetType, T: QSplitter_setRubberBand_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRubberBand_0(self);
    // return 1;
  }
}
pub trait QSplitter_setRubberBand_0<RetType> {
  fn setRubberBand_0(self , rsthis: & QSplitter) -> RetType;
}
impl<'a> /*trait*/ QSplitter_setRubberBand_0<(/*void*/)> for (i32) {
  fn setRubberBand_0(self , rsthis: & QSplitter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QSplitter13setRubberBandEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:122
// index:0
// Protected Visibility=Default Availability=Available
// [4] int closestLegalPosition(int, int)

/*
Returns the closest legal position to pos of the widget at index.

For right-to-left languages such as Arabic and Hebrew, the layout of horizontal splitters is reversed. Positions are then measured from the right edge of the widget.

See also getRange().
*/
impl /*struct*/ QSplitter {
  pub fn closestLegalPosition_0<RetType, T: QSplitter_closestLegalPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.closestLegalPosition_0(self);
    // return 1;
  }
}
pub trait QSplitter_closestLegalPosition_0<RetType> {
  fn closestLegalPosition_0(self , rsthis: & QSplitter) -> RetType;
}
impl<'a> /*trait*/ QSplitter_closestLegalPosition_0<i32> for (i32,i32) {
  fn closestLegalPosition_0(self , rsthis: & QSplitter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QSplitter20closestLegalPositionEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
