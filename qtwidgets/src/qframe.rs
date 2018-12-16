

// mod ::widgets::QFrame
// package qtwidgets
// /usr/include/qt/QtWidgets/qframe.h
// #include <qframe.h>
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

// bool event(QEvent *)
// func (this *QFrame) InheritEvent(f func(e *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// void paintEvent(QPaintEvent *)
// func (this *QFrame) InheritPaintEvent(f func(arg0 *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }

// void changeEvent(QEvent *)
// func (this *QFrame) InheritChangeEvent(f func(arg0 *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "changeEvent", f)
// }

// void drawFrame(QPainter *)
// func (this *QFrame) InheritDrawFrame(f func(arg0 *qtgui.QPainter/*777 QPainter **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "drawFrame", f)
// }

// void initStyleOption(QStyleOptionFrame *)
// func (this *QFrame) InheritInitStyleOption(f func(option *QStyleOptionFrame/*777 QStyleOptionFrame **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "initStyleOption", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QFrame)=48
pub struct QFrame {
  qbase: QWidget,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QFrame_ITF interface {
//    QWidget_ITF
//    QFrame_PTR() *QFrame
//}
//func (ptr *QFrame) QFrame_PTR() *QFrame { return ptr }

impl /*struct*/ QFrame {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QFrame {
    return QFrame{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QFrame {
//  type Target = QFrameBASE;
//
//  fn deref(&self) -> &QFrameBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QFrameBASE> for QFrame {
//  fn as_ref(& self) -> & QFrameBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qframe.h:54
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QFrame {
  pub fn metaObject_0<RetType, T: QFrame_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QFrame_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QFrame) -> RetType;
}
impl<'a> /*trait*/ QFrame_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QFrame) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QFrame10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qframe.h:64
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QFrame(QWidget *, Qt::WindowFlags)

/*
Constructs a frame widget with frame style NoFrame and a 1-pixel frame width.

The parent and f arguments are passed to the QWidget constructor.
*/
// QFrame(QWidget *, Qt::WindowFlags) ctx.fn_proto_cpp
impl /*struct*/ QFrame {
  pub fn QFrame_0<T: QFrame_QFrame_0>(value: T) -> QFrame {
    let rsthis = value.QFrame_0();
    return rsthis;
    // return 1;
  }
}

pub trait QFrame_QFrame_0 {
  fn QFrame_0(self) -> QFrame;
}
// QFrame(QWidget *, Qt::WindowFlags) ctx.fn_proto_cpp
impl<'a> /*trait*/ QFrame_QFrame_0 for (usize,i32) {
  fn QFrame_0(self) -> QFrame {
    // unsafe{_ZN6QFrameC2EP7QWidget6QFlagsIN2Qt10WindowTypeEE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QFrameC2EP7QWidget6QFlagsIN2Qt10WindowTypeEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFrame{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qframe.h:65
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QFrame()

/*

*/
pub fn DeleteQFrame(this :*mut QFrame) {
    // let rv = qtrt::InvokeQtFunc6("_ZN6QFrameD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qframe.h:67
// index:0
// Public Visibility=Default Availability=Available
// [4] int frameStyle() const

/*
Returns the frame style.

The default value is QFrame::Plain.

See also setFrameStyle(), frameShape(), and frameShadow().
*/
impl /*struct*/ QFrame {
  pub fn frameStyle_0<RetType, T: QFrame_frameStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.frameStyle_0(self);
    // return 1;
  }
}
pub trait QFrame_frameStyle_0<RetType> {
  fn frameStyle_0(self , rsthis: & QFrame) -> RetType;
}
impl<'a> /*trait*/ QFrame_frameStyle_0<i32> for () {
  fn frameStyle_0(self , rsthis: & QFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QFrame10frameStyleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qframe.h:68
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFrameStyle(int)

/*
Sets the frame style to style.

The style is the bitwise OR between a frame shape and a frame shadow style. See the picture of the frames in the main class documentation.

The frame shapes are given in QFrame::Shape and the shadow styles in QFrame::Shadow.

If a mid-line width greater than 0 is specified, an additional line is drawn for Raised or Sunken Box, HLine, and VLine frames. The mid-color of the current color group is used for drawing middle lines.

See also frameStyle().
*/
impl /*struct*/ QFrame {
  pub fn setFrameStyle_0<RetType, T: QFrame_setFrameStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFrameStyle_0(self);
    // return 1;
  }
}
pub trait QFrame_setFrameStyle_0<RetType> {
  fn setFrameStyle_0(self , rsthis: & QFrame) -> RetType;
}
impl<'a> /*trait*/ QFrame_setFrameStyle_0<(/*void*/)> for (i32) {
  fn setFrameStyle_0(self , rsthis: & QFrame) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QFrame13setFrameStyleEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qframe.h:70
// index:0
// Public Visibility=Default Availability=Available
// [4] int frameWidth() const

/*

*/
impl /*struct*/ QFrame {
  pub fn frameWidth_0<RetType, T: QFrame_frameWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.frameWidth_0(self);
    // return 1;
  }
}
pub trait QFrame_frameWidth_0<RetType> {
  fn frameWidth_0(self , rsthis: & QFrame) -> RetType;
}
impl<'a> /*trait*/ QFrame_frameWidth_0<i32> for () {
  fn frameWidth_0(self , rsthis: & QFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QFrame10frameWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qframe.h:72
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Reimplemented from QWidget::sizeHint().
*/
impl /*struct*/ QFrame {
  pub fn sizeHint_0<RetType, T: QFrame_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QFrame_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QFrame) -> RetType;
}
impl<'a> /*trait*/ QFrame_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QFrame) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QFrame8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qframe.h:96
// index:0
// Public Visibility=Default Availability=Available
// [4] QFrame::Shape frameShape() const

/*

*/
impl /*struct*/ QFrame {
  pub fn frameShape_0<RetType, T: QFrame_frameShape_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.frameShape_0(self);
    // return 1;
  }
}
pub trait QFrame_frameShape_0<RetType> {
  fn frameShape_0(self , rsthis: & QFrame) -> RetType;
}
impl<'a> /*trait*/ QFrame_frameShape_0<i32> for () {
  fn frameShape_0(self , rsthis: & QFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QFrame10frameShapeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qframe.h:97
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFrameShape(QFrame::Shape)

/*

*/
impl /*struct*/ QFrame {
  pub fn setFrameShape_0<RetType, T: QFrame_setFrameShape_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFrameShape_0(self);
    // return 1;
  }
}
pub trait QFrame_setFrameShape_0<RetType> {
  fn setFrameShape_0(self , rsthis: & QFrame) -> RetType;
}
impl<'a> /*trait*/ QFrame_setFrameShape_0<(/*void*/)> for (i32) {
  fn setFrameShape_0(self , rsthis: & QFrame) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QFrame13setFrameShapeENS_5ShapeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qframe.h:98
// index:0
// Public Visibility=Default Availability=Available
// [4] QFrame::Shadow frameShadow() const

/*

*/
impl /*struct*/ QFrame {
  pub fn frameShadow_0<RetType, T: QFrame_frameShadow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.frameShadow_0(self);
    // return 1;
  }
}
pub trait QFrame_frameShadow_0<RetType> {
  fn frameShadow_0(self , rsthis: & QFrame) -> RetType;
}
impl<'a> /*trait*/ QFrame_frameShadow_0<i32> for () {
  fn frameShadow_0(self , rsthis: & QFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QFrame11frameShadowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qframe.h:99
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFrameShadow(QFrame::Shadow)

/*

*/
impl /*struct*/ QFrame {
  pub fn setFrameShadow_0<RetType, T: QFrame_setFrameShadow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFrameShadow_0(self);
    // return 1;
  }
}
pub trait QFrame_setFrameShadow_0<RetType> {
  fn setFrameShadow_0(self , rsthis: & QFrame) -> RetType;
}
impl<'a> /*trait*/ QFrame_setFrameShadow_0<(/*void*/)> for (i32) {
  fn setFrameShadow_0(self , rsthis: & QFrame) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QFrame14setFrameShadowENS_6ShadowE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qframe.h:101
// index:0
// Public Visibility=Default Availability=Available
// [4] int lineWidth() const

/*

*/
impl /*struct*/ QFrame {
  pub fn lineWidth_0<RetType, T: QFrame_lineWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lineWidth_0(self);
    // return 1;
  }
}
pub trait QFrame_lineWidth_0<RetType> {
  fn lineWidth_0(self , rsthis: & QFrame) -> RetType;
}
impl<'a> /*trait*/ QFrame_lineWidth_0<i32> for () {
  fn lineWidth_0(self , rsthis: & QFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QFrame9lineWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qframe.h:102
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLineWidth(int)

/*

*/
impl /*struct*/ QFrame {
  pub fn setLineWidth_0<RetType, T: QFrame_setLineWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLineWidth_0(self);
    // return 1;
  }
}
pub trait QFrame_setLineWidth_0<RetType> {
  fn setLineWidth_0(self , rsthis: & QFrame) -> RetType;
}
impl<'a> /*trait*/ QFrame_setLineWidth_0<(/*void*/)> for (i32) {
  fn setLineWidth_0(self , rsthis: & QFrame) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QFrame12setLineWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qframe.h:104
// index:0
// Public Visibility=Default Availability=Available
// [4] int midLineWidth() const

/*

*/
impl /*struct*/ QFrame {
  pub fn midLineWidth_0<RetType, T: QFrame_midLineWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.midLineWidth_0(self);
    // return 1;
  }
}
pub trait QFrame_midLineWidth_0<RetType> {
  fn midLineWidth_0(self , rsthis: & QFrame) -> RetType;
}
impl<'a> /*trait*/ QFrame_midLineWidth_0<i32> for () {
  fn midLineWidth_0(self , rsthis: & QFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QFrame12midLineWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qframe.h:105
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMidLineWidth(int)

/*

*/
impl /*struct*/ QFrame {
  pub fn setMidLineWidth_0<RetType, T: QFrame_setMidLineWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMidLineWidth_0(self);
    // return 1;
  }
}
pub trait QFrame_setMidLineWidth_0<RetType> {
  fn setMidLineWidth_0(self , rsthis: & QFrame) -> RetType;
}
impl<'a> /*trait*/ QFrame_setMidLineWidth_0<(/*void*/)> for (i32) {
  fn setMidLineWidth_0(self , rsthis: & QFrame) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QFrame15setMidLineWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qframe.h:107
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect frameRect() const

/*

*/
impl /*struct*/ QFrame {
  pub fn frameRect_0<RetType, T: QFrame_frameRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.frameRect_0(self);
    // return 1;
  }
}
pub trait QFrame_frameRect_0<RetType> {
  fn frameRect_0(self , rsthis: & QFrame) -> RetType;
}
impl<'a> /*trait*/ QFrame_frameRect_0<usize> for () {
  fn frameRect_0(self , rsthis: & QFrame) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QFrame9frameRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qframe.h:108
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFrameRect(const QRect &)

/*

*/
impl /*struct*/ QFrame {
  pub fn setFrameRect_0<RetType, T: QFrame_setFrameRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFrameRect_0(self);
    // return 1;
  }
}
pub trait QFrame_setFrameRect_0<RetType> {
  fn setFrameRect_0(self , rsthis: & QFrame) -> RetType;
}
impl<'a> /*trait*/ QFrame_setFrameRect_0<(/*void*/)> for (usize) {
  fn setFrameRect_0(self , rsthis: & QFrame) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QFrame12setFrameRectERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qframe.h:111
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QFrame {
  pub fn event_0<RetType, T: QFrame_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QFrame_event_0<RetType> {
  fn event_0(self , rsthis: & QFrame) -> RetType;
}
impl<'a> /*trait*/ QFrame_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QFrame) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QFrame5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qframe.h:112
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Reimplemented from QWidget::paintEvent().
*/
impl /*struct*/ QFrame {
  pub fn paintEvent_0<RetType, T: QFrame_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QFrame_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QFrame) -> RetType;
}
impl<'a> /*trait*/ QFrame_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QFrame) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QFrame10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qframe.h:113
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void changeEvent(QEvent *)

/*
Reimplemented from QWidget::changeEvent().
*/
impl /*struct*/ QFrame {
  pub fn changeEvent_0<RetType, T: QFrame_changeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changeEvent_0(self);
    // return 1;
  }
}
pub trait QFrame_changeEvent_0<RetType> {
  fn changeEvent_0(self , rsthis: & QFrame) -> RetType;
}
impl<'a> /*trait*/ QFrame_changeEvent_0<(/*void*/)> for (usize) {
  fn changeEvent_0(self , rsthis: & QFrame) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QFrame11changeEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qframe.h:114
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void drawFrame(QPainter *)

/*

*/
impl /*struct*/ QFrame {
  pub fn drawFrame_0<RetType, T: QFrame_drawFrame_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawFrame_0(self);
    // return 1;
  }
}
pub trait QFrame_drawFrame_0<RetType> {
  fn drawFrame_0(self , rsthis: & QFrame) -> RetType;
}
impl<'a> /*trait*/ QFrame_drawFrame_0<(/*void*/)> for (usize) {
  fn drawFrame_0(self , rsthis: & QFrame) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QFrame9drawFrameEP8QPainter", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qframe.h:119
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void initStyleOption(QStyleOptionFrame *) const

/*
Initializes option with the values from this QFrame. This method is useful for subclasses when they need a QStyleOptionFrame but don't want to fill in all the information themselves.

This function was introduced in  Qt 5.5.

See also QStyleOption::initFrom().
*/
impl /*struct*/ QFrame {
  pub fn initStyleOption_0<RetType, T: QFrame_initStyleOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.initStyleOption_0(self);
    // return 1;
  }
}
pub trait QFrame_initStyleOption_0<RetType> {
  fn initStyleOption_0(self , rsthis: & QFrame) -> RetType;
}
impl<'a> /*trait*/ QFrame_initStyleOption_0<(/*void*/)> for (usize) {
  fn initStyleOption_0(self , rsthis: & QFrame) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK6QFrame15initStyleOptionEP17QStyleOptionFrame", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
This enum type defines the shapes of frame available.



When it does not call QStyle, Shape interacts with QFrame::Shadow, the lineWidth() and the midLineWidth() to create the total result. See the picture of the frames in the main class documentation.

See also QFrame::Shadow, QFrame::style(), and QStyle::drawPrimitive().

*/
pub type QFrame__Shape = i32;
// QFrame draws nothing
pub const QFrame__NoFrame :QFrame__Shape = 0;
// 
pub const QFrame__Box :QFrame__Shape = 1;
// 
pub const QFrame__Panel :QFrame__Shape = 2;
// 
pub const QFrame__WinPanel :QFrame__Shape = 3;
// 
pub const QFrame__HLine :QFrame__Shape = 4;
// 
pub const QFrame__VLine :QFrame__Shape = 5;
// 
pub const QFrame__StyledPanel :QFrame__Shape = 6;
pub fn QFrame_ShapeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QFrame", val);
}
pub fn QFrame_ShapeItemName_s(val: i32) ->String {
  //var nilthis *QFrame
  //return nilthis.ShapeItemName(val);
  return QFrame_ShapeItemName(val);
}


/*
This enum type defines the types of shadow that are used to give a 3D effect to frames.



Shadow interacts with QFrame::Shape, the lineWidth() and the midLineWidth(). See the picture of the frames in the main class documentation.

See also QFrame::Shape, lineWidth(), and midLineWidth().

*/
pub type QFrame__Shadow = i32;
// 
pub const QFrame__Plain :QFrame__Shadow = 16;
// 
pub const QFrame__Raised :QFrame__Shadow = 32;
// 
pub const QFrame__Sunken :QFrame__Shadow = 48;
pub fn QFrame_ShadowItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QFrame", val);
}
pub fn QFrame_ShadowItemName_s(val: i32) ->String {
  //var nilthis *QFrame
  //return nilthis.ShadowItemName(val);
  return QFrame_ShadowItemName(val);
}


/*
This enum defines two constants that can be used to extract the two components of frameStyle():



Normally, you don't need to use these, since frameShadow() and frameShape() already extract the Shadow and the Shape parts of frameStyle().

See also frameStyle() and setFrameStyle().

*/
pub type QFrame__StyleMask = i32;
// 
pub const QFrame__Shadow_Mask :QFrame__StyleMask = 240;
// 
pub const QFrame__Shape_Mask :QFrame__StyleMask = 15;
pub fn QFrame_StyleMaskItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QFrame", val);
}
pub fn QFrame_StyleMaskItemName_s(val: i32) ->String {
  //var nilthis *QFrame
  //return nilthis.StyleMaskItemName(val);
  return QFrame_StyleMaskItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
