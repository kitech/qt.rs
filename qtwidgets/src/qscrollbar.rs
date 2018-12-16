

// mod ::widgets::QScrollBar
// package qtwidgets
// /usr/include/qt/QtWidgets/qscrollbar.h
// #include <qscrollbar.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 11
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

// void wheelEvent(QWheelEvent *)
// func (this *QScrollBar) InheritWheelEvent(f func(arg0 *qtgui.QWheelEvent/*777 QWheelEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "wheelEvent", f)
// }

// void paintEvent(QPaintEvent *)
// func (this *QScrollBar) InheritPaintEvent(f func(arg0 *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }

// void mousePressEvent(QMouseEvent *)
// func (this *QScrollBar) InheritMousePressEvent(f func(arg0 *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mousePressEvent", f)
// }

// void mouseReleaseEvent(QMouseEvent *)
// func (this *QScrollBar) InheritMouseReleaseEvent(f func(arg0 *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseReleaseEvent", f)
// }

// void mouseMoveEvent(QMouseEvent *)
// func (this *QScrollBar) InheritMouseMoveEvent(f func(arg0 *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseMoveEvent", f)
// }

// void hideEvent(QHideEvent *)
// func (this *QScrollBar) InheritHideEvent(f func(arg0 *qtgui.QHideEvent/*777 QHideEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "hideEvent", f)
// }

// void sliderChange(QAbstractSlider::SliderChange)
// func (this *QScrollBar) InheritSliderChange(f func(change int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "sliderChange", f)
// }

// void contextMenuEvent(QContextMenuEvent *)
// func (this *QScrollBar) InheritContextMenuEvent(f func(arg0 *qtgui.QContextMenuEvent/*777 QContextMenuEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "contextMenuEvent", f)
// }

// void initStyleOption(QStyleOptionSlider *)
// func (this *QScrollBar) InheritInitStyleOption(f func(option *QStyleOptionSlider/*777 QStyleOptionSlider **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "initStyleOption", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QScrollBar)=48
pub struct QScrollBar {
  qbase: QAbstractSlider,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QScrollBar_ITF interface {
//    QAbstractSlider_ITF
//    QScrollBar_PTR() *QScrollBar
//}
//func (ptr *QScrollBar) QScrollBar_PTR() *QScrollBar { return ptr }

impl /*struct*/ QScrollBar {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QScrollBar {
    return QScrollBar{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QScrollBar {
//  type Target = QScrollBarBASE;
//
//  fn deref(&self) -> &QScrollBarBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QScrollBarBASE> for QScrollBar {
//  fn as_ref(& self) -> & QScrollBarBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qscrollbar.h:57
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QScrollBar {
  pub fn metaObject_0<RetType, T: QScrollBar_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QScrollBar_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QScrollBar) -> RetType;
}
impl<'a> /*trait*/ QScrollBar_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QScrollBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QScrollBar10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qscrollbar.h:59
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QScrollBar(QWidget *)

/*
Constructs a vertical scroll bar.

The parent argument is sent to the QWidget constructor.

The minimum defaults to 0, the maximum to 99, with a singleStep size of 1 and a pageStep size of 10, and an initial value of 0.
*/
// QScrollBar(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QScrollBar {
  pub fn QScrollBar_0<T: QScrollBar_QScrollBar_0>(value: T) -> QScrollBar {
    let rsthis = value.QScrollBar_0();
    return rsthis;
    // return 1;
  }
}

pub trait QScrollBar_QScrollBar_0 {
  fn QScrollBar_0(self) -> QScrollBar;
}
// QScrollBar(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QScrollBar_QScrollBar_0 for (usize) {
  fn QScrollBar_0(self) -> QScrollBar {
    // unsafe{_ZN10QScrollBarC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QScrollBarC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QScrollBar{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qscrollbar.h:60
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QScrollBar(Qt::Orientation, QWidget *)

/*
Constructs a vertical scroll bar.

The parent argument is sent to the QWidget constructor.

The minimum defaults to 0, the maximum to 99, with a singleStep size of 1 and a pageStep size of 10, and an initial value of 0.
*/
// QScrollBar(Qt::Orientation, QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QScrollBar {
  pub fn QScrollBar_1<T: QScrollBar_QScrollBar_1>(value: T) -> QScrollBar {
    let rsthis = value.QScrollBar_1();
    return rsthis;
    // return 1;
  }
}

pub trait QScrollBar_QScrollBar_1 {
  fn QScrollBar_1(self) -> QScrollBar;
}
// QScrollBar(Qt::Orientation, QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QScrollBar_QScrollBar_1 for (i32,usize) {
  fn QScrollBar_1(self) -> QScrollBar {
    // unsafe{_ZN10QScrollBarC2EN2Qt11OrientationEP7QWidget()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QScrollBarC2EN2Qt11OrientationEP7QWidget", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QScrollBar{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qscrollbar.h:61
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QScrollBar()

/*

*/
pub fn DeleteQScrollBar(this :*mut QScrollBar) {
    // let rv = qtrt::InvokeQtFunc6("_ZN10QScrollBarD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qscrollbar.h:63
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Reimplemented from QWidget::sizeHint().
*/
impl /*struct*/ QScrollBar {
  pub fn sizeHint_0<RetType, T: QScrollBar_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QScrollBar_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QScrollBar) -> RetType;
}
impl<'a> /*trait*/ QScrollBar_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QScrollBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QScrollBar8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qscrollbar.h:64
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QScrollBar {
  pub fn event_0<RetType, T: QScrollBar_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QScrollBar_event_0<RetType> {
  fn event_0(self , rsthis: & QScrollBar) -> RetType;
}
impl<'a> /*trait*/ QScrollBar_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QScrollBar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QScrollBar5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qscrollbar.h:68
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void wheelEvent(QWheelEvent *)

/*
Reimplemented from QWidget::wheelEvent().
*/
impl /*struct*/ QScrollBar {
  pub fn wheelEvent_0<RetType, T: QScrollBar_wheelEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.wheelEvent_0(self);
    // return 1;
  }
}
pub trait QScrollBar_wheelEvent_0<RetType> {
  fn wheelEvent_0(self , rsthis: & QScrollBar) -> RetType;
}
impl<'a> /*trait*/ QScrollBar_wheelEvent_0<(/*void*/)> for (usize) {
  fn wheelEvent_0(self , rsthis: & QScrollBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QScrollBar10wheelEventEP11QWheelEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qscrollbar.h:70
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Reimplemented from QWidget::paintEvent().
*/
impl /*struct*/ QScrollBar {
  pub fn paintEvent_0<RetType, T: QScrollBar_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QScrollBar_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QScrollBar) -> RetType;
}
impl<'a> /*trait*/ QScrollBar_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QScrollBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QScrollBar10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qscrollbar.h:71
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mousePressEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mousePressEvent().
*/
impl /*struct*/ QScrollBar {
  pub fn mousePressEvent_0<RetType, T: QScrollBar_mousePressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mousePressEvent_0(self);
    // return 1;
  }
}
pub trait QScrollBar_mousePressEvent_0<RetType> {
  fn mousePressEvent_0(self , rsthis: & QScrollBar) -> RetType;
}
impl<'a> /*trait*/ QScrollBar_mousePressEvent_0<(/*void*/)> for (usize) {
  fn mousePressEvent_0(self , rsthis: & QScrollBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QScrollBar15mousePressEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qscrollbar.h:72
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseReleaseEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseReleaseEvent().
*/
impl /*struct*/ QScrollBar {
  pub fn mouseReleaseEvent_0<RetType, T: QScrollBar_mouseReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QScrollBar_mouseReleaseEvent_0<RetType> {
  fn mouseReleaseEvent_0(self , rsthis: & QScrollBar) -> RetType;
}
impl<'a> /*trait*/ QScrollBar_mouseReleaseEvent_0<(/*void*/)> for (usize) {
  fn mouseReleaseEvent_0(self , rsthis: & QScrollBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QScrollBar17mouseReleaseEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qscrollbar.h:73
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseMoveEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseMoveEvent().
*/
impl /*struct*/ QScrollBar {
  pub fn mouseMoveEvent_0<RetType, T: QScrollBar_mouseMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseMoveEvent_0(self);
    // return 1;
  }
}
pub trait QScrollBar_mouseMoveEvent_0<RetType> {
  fn mouseMoveEvent_0(self , rsthis: & QScrollBar) -> RetType;
}
impl<'a> /*trait*/ QScrollBar_mouseMoveEvent_0<(/*void*/)> for (usize) {
  fn mouseMoveEvent_0(self , rsthis: & QScrollBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QScrollBar14mouseMoveEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qscrollbar.h:74
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void hideEvent(QHideEvent *)

/*
Reimplemented from QWidget::hideEvent().
*/
impl /*struct*/ QScrollBar {
  pub fn hideEvent_0<RetType, T: QScrollBar_hideEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hideEvent_0(self);
    // return 1;
  }
}
pub trait QScrollBar_hideEvent_0<RetType> {
  fn hideEvent_0(self , rsthis: & QScrollBar) -> RetType;
}
impl<'a> /*trait*/ QScrollBar_hideEvent_0<(/*void*/)> for (usize) {
  fn hideEvent_0(self , rsthis: & QScrollBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QScrollBar9hideEventEP10QHideEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qscrollbar.h:75
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void sliderChange(QAbstractSlider::SliderChange)

/*
Reimplemented from QAbstractSlider::sliderChange().
*/
impl /*struct*/ QScrollBar {
  pub fn sliderChange_0<RetType, T: QScrollBar_sliderChange_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sliderChange_0(self);
    // return 1;
  }
}
pub trait QScrollBar_sliderChange_0<RetType> {
  fn sliderChange_0(self , rsthis: & QScrollBar) -> RetType;
}
impl<'a> /*trait*/ QScrollBar_sliderChange_0<(/*void*/)> for (i32) {
  fn sliderChange_0(self , rsthis: & QScrollBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QScrollBar12sliderChangeEN15QAbstractSlider12SliderChangeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qscrollbar.h:77
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void contextMenuEvent(QContextMenuEvent *)

/*
Reimplemented from QWidget::contextMenuEvent().
*/
impl /*struct*/ QScrollBar {
  pub fn contextMenuEvent_0<RetType, T: QScrollBar_contextMenuEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contextMenuEvent_0(self);
    // return 1;
  }
}
pub trait QScrollBar_contextMenuEvent_0<RetType> {
  fn contextMenuEvent_0(self , rsthis: & QScrollBar) -> RetType;
}
impl<'a> /*trait*/ QScrollBar_contextMenuEvent_0<(/*void*/)> for (usize) {
  fn contextMenuEvent_0(self , rsthis: & QScrollBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QScrollBar16contextMenuEventEP17QContextMenuEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qscrollbar.h:79
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void initStyleOption(QStyleOptionSlider *) const

/*
Initialize option with the values from this QScrollBar. This method is useful for subclasses when they need a QStyleOptionSlider, but don't want to fill in all the information themselves.

See also QStyleOption::initFrom().
*/
impl /*struct*/ QScrollBar {
  pub fn initStyleOption_0<RetType, T: QScrollBar_initStyleOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.initStyleOption_0(self);
    // return 1;
  }
}
pub trait QScrollBar_initStyleOption_0<RetType> {
  fn initStyleOption_0(self , rsthis: & QScrollBar) -> RetType;
}
impl<'a> /*trait*/ QScrollBar_initStyleOption_0<(/*void*/)> for (usize) {
  fn initStyleOption_0(self , rsthis: & QScrollBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK10QScrollBar15initStyleOptionEP18QStyleOptionSlider", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
