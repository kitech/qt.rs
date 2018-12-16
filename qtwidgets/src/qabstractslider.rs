

// mod ::widgets::QAbstractSlider
// package qtwidgets
// /usr/include/qt/QtWidgets/qabstractslider.h
// #include <qabstractslider.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 58
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
// func (this *QAbstractSlider) InheritEvent(f func(e *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// void setRepeatAction(QAbstractSlider::SliderAction, int, int)
// func (this *QAbstractSlider) InheritSetRepeatAction(f func(action int, thresholdTime int, repeatTime int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "setRepeatAction", f)
// }

// QAbstractSlider::SliderAction repeatAction()
// func (this *QAbstractSlider) InheritRepeatAction(f func() int) {
//  qtrt.SetAllInheritCallback(this, "repeatAction", f)
// }

// void sliderChange(QAbstractSlider::SliderChange)
// func (this *QAbstractSlider) InheritSliderChange(f func(change int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "sliderChange", f)
// }

// void keyPressEvent(QKeyEvent *)
// func (this *QAbstractSlider) InheritKeyPressEvent(f func(ev *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyPressEvent", f)
// }

// void timerEvent(QTimerEvent *)
// func (this *QAbstractSlider) InheritTimerEvent(f func(arg0 *qtcore.QTimerEvent/*777 QTimerEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "timerEvent", f)
// }

// void wheelEvent(QWheelEvent *)
// func (this *QAbstractSlider) InheritWheelEvent(f func(e *qtgui.QWheelEvent/*777 QWheelEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "wheelEvent", f)
// }

// void changeEvent(QEvent *)
// func (this *QAbstractSlider) InheritChangeEvent(f func(e *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "changeEvent", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QAbstractSlider)=48
pub struct QAbstractSlider {
  qbase: QWidget,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAbstractSlider_ITF interface {
//    QWidget_ITF
//    QAbstractSlider_PTR() *QAbstractSlider
//}
//func (ptr *QAbstractSlider) QAbstractSlider_PTR() *QAbstractSlider { return ptr }

impl /*struct*/ QAbstractSlider {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAbstractSlider {
    return QAbstractSlider{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAbstractSlider {
//  type Target = QAbstractSliderBASE;
//
//  fn deref(&self) -> &QAbstractSliderBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAbstractSliderBASE> for QAbstractSlider {
//  fn as_ref(& self) -> & QAbstractSliderBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qabstractslider.h:55
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QAbstractSlider {
  pub fn metaObject_0<RetType, T: QAbstractSlider_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QAbstractSlider_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QAbstractSlider) -> RetType;
}
impl<'a> /*trait*/ QAbstractSlider_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QAbstractSlider) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QAbstractSlider10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractslider.h:70
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QAbstractSlider(QWidget *)

/*
Constructs an abstract slider.

The parent argument is sent to the QWidget constructor.

The minimum defaults to 0, the maximum to 99, with a singleStep size of 1 and a pageStep size of 10, and an initial value of 0.
*/
// QAbstractSlider(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QAbstractSlider {
  pub fn QAbstractSlider_0<T: QAbstractSlider_QAbstractSlider_0>(value: T) -> QAbstractSlider {
    let rsthis = value.QAbstractSlider_0();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractSlider_QAbstractSlider_0 {
  fn QAbstractSlider_0(self) -> QAbstractSlider;
}
// QAbstractSlider(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QAbstractSlider_QAbstractSlider_0 for (usize) {
  fn QAbstractSlider_0(self) -> QAbstractSlider {
    // unsafe{_ZN15QAbstractSliderC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QAbstractSliderC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAbstractSlider{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractslider.h:71
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QAbstractSlider()

/*

*/
pub fn DeleteQAbstractSlider(this :*mut QAbstractSlider) {
    // let rv = qtrt::InvokeQtFunc6("_ZN15QAbstractSliderD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qabstractslider.h:73
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::Orientation orientation() const

/*

*/
impl /*struct*/ QAbstractSlider {
  pub fn orientation_0<RetType, T: QAbstractSlider_orientation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.orientation_0(self);
    // return 1;
  }
}
pub trait QAbstractSlider_orientation_0<RetType> {
  fn orientation_0(self , rsthis: & QAbstractSlider) -> RetType;
}
impl<'a> /*trait*/ QAbstractSlider_orientation_0<i32> for () {
  fn orientation_0(self , rsthis: & QAbstractSlider) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QAbstractSlider11orientationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractslider.h:75
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMinimum(int)

/*

*/
impl /*struct*/ QAbstractSlider {
  pub fn setMinimum_0<RetType, T: QAbstractSlider_setMinimum_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMinimum_0(self);
    // return 1;
  }
}
pub trait QAbstractSlider_setMinimum_0<RetType> {
  fn setMinimum_0(self , rsthis: & QAbstractSlider) -> RetType;
}
impl<'a> /*trait*/ QAbstractSlider_setMinimum_0<(/*void*/)> for (i32) {
  fn setMinimum_0(self , rsthis: & QAbstractSlider) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractSlider10setMinimumEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractslider.h:76
// index:0
// Public Visibility=Default Availability=Available
// [4] int minimum() const

/*

*/
impl /*struct*/ QAbstractSlider {
  pub fn minimum_0<RetType, T: QAbstractSlider_minimum_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimum_0(self);
    // return 1;
  }
}
pub trait QAbstractSlider_minimum_0<RetType> {
  fn minimum_0(self , rsthis: & QAbstractSlider) -> RetType;
}
impl<'a> /*trait*/ QAbstractSlider_minimum_0<i32> for () {
  fn minimum_0(self , rsthis: & QAbstractSlider) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QAbstractSlider7minimumEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractslider.h:78
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMaximum(int)

/*

*/
impl /*struct*/ QAbstractSlider {
  pub fn setMaximum_0<RetType, T: QAbstractSlider_setMaximum_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMaximum_0(self);
    // return 1;
  }
}
pub trait QAbstractSlider_setMaximum_0<RetType> {
  fn setMaximum_0(self , rsthis: & QAbstractSlider) -> RetType;
}
impl<'a> /*trait*/ QAbstractSlider_setMaximum_0<(/*void*/)> for (i32) {
  fn setMaximum_0(self , rsthis: & QAbstractSlider) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractSlider10setMaximumEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractslider.h:79
// index:0
// Public Visibility=Default Availability=Available
// [4] int maximum() const

/*

*/
impl /*struct*/ QAbstractSlider {
  pub fn maximum_0<RetType, T: QAbstractSlider_maximum_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maximum_0(self);
    // return 1;
  }
}
pub trait QAbstractSlider_maximum_0<RetType> {
  fn maximum_0(self , rsthis: & QAbstractSlider) -> RetType;
}
impl<'a> /*trait*/ QAbstractSlider_maximum_0<i32> for () {
  fn maximum_0(self , rsthis: & QAbstractSlider) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QAbstractSlider7maximumEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractslider.h:81
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSingleStep(int)

/*

*/
impl /*struct*/ QAbstractSlider {
  pub fn setSingleStep_0<RetType, T: QAbstractSlider_setSingleStep_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSingleStep_0(self);
    // return 1;
  }
}
pub trait QAbstractSlider_setSingleStep_0<RetType> {
  fn setSingleStep_0(self , rsthis: & QAbstractSlider) -> RetType;
}
impl<'a> /*trait*/ QAbstractSlider_setSingleStep_0<(/*void*/)> for (i32) {
  fn setSingleStep_0(self , rsthis: & QAbstractSlider) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractSlider13setSingleStepEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractslider.h:82
// index:0
// Public Visibility=Default Availability=Available
// [4] int singleStep() const

/*

*/
impl /*struct*/ QAbstractSlider {
  pub fn singleStep_0<RetType, T: QAbstractSlider_singleStep_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.singleStep_0(self);
    // return 1;
  }
}
pub trait QAbstractSlider_singleStep_0<RetType> {
  fn singleStep_0(self , rsthis: & QAbstractSlider) -> RetType;
}
impl<'a> /*trait*/ QAbstractSlider_singleStep_0<i32> for () {
  fn singleStep_0(self , rsthis: & QAbstractSlider) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QAbstractSlider10singleStepEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractslider.h:84
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPageStep(int)

/*

*/
impl /*struct*/ QAbstractSlider {
  pub fn setPageStep_0<RetType, T: QAbstractSlider_setPageStep_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPageStep_0(self);
    // return 1;
  }
}
pub trait QAbstractSlider_setPageStep_0<RetType> {
  fn setPageStep_0(self , rsthis: & QAbstractSlider) -> RetType;
}
impl<'a> /*trait*/ QAbstractSlider_setPageStep_0<(/*void*/)> for (i32) {
  fn setPageStep_0(self , rsthis: & QAbstractSlider) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractSlider11setPageStepEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractslider.h:85
// index:0
// Public Visibility=Default Availability=Available
// [4] int pageStep() const

/*

*/
impl /*struct*/ QAbstractSlider {
  pub fn pageStep_0<RetType, T: QAbstractSlider_pageStep_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pageStep_0(self);
    // return 1;
  }
}
pub trait QAbstractSlider_pageStep_0<RetType> {
  fn pageStep_0(self , rsthis: & QAbstractSlider) -> RetType;
}
impl<'a> /*trait*/ QAbstractSlider_pageStep_0<i32> for () {
  fn pageStep_0(self , rsthis: & QAbstractSlider) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QAbstractSlider8pageStepEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractslider.h:87
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTracking(bool)

/*

*/
impl /*struct*/ QAbstractSlider {
  pub fn setTracking_0<RetType, T: QAbstractSlider_setTracking_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTracking_0(self);
    // return 1;
  }
}
pub trait QAbstractSlider_setTracking_0<RetType> {
  fn setTracking_0(self , rsthis: & QAbstractSlider) -> RetType;
}
impl<'a> /*trait*/ QAbstractSlider_setTracking_0<(/*void*/)> for (bool) {
  fn setTracking_0(self , rsthis: & QAbstractSlider) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractSlider11setTrackingEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractslider.h:88
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasTracking() const

/*

*/
impl /*struct*/ QAbstractSlider {
  pub fn hasTracking_0<RetType, T: QAbstractSlider_hasTracking_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasTracking_0(self);
    // return 1;
  }
}
pub trait QAbstractSlider_hasTracking_0<RetType> {
  fn hasTracking_0(self , rsthis: & QAbstractSlider) -> RetType;
}
impl<'a> /*trait*/ QAbstractSlider_hasTracking_0<bool> for () {
  fn hasTracking_0(self , rsthis: & QAbstractSlider) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QAbstractSlider11hasTrackingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractslider.h:90
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSliderDown(bool)

/*

*/
impl /*struct*/ QAbstractSlider {
  pub fn setSliderDown_0<RetType, T: QAbstractSlider_setSliderDown_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSliderDown_0(self);
    // return 1;
  }
}
pub trait QAbstractSlider_setSliderDown_0<RetType> {
  fn setSliderDown_0(self , rsthis: & QAbstractSlider) -> RetType;
}
impl<'a> /*trait*/ QAbstractSlider_setSliderDown_0<(/*void*/)> for (bool) {
  fn setSliderDown_0(self , rsthis: & QAbstractSlider) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractSlider13setSliderDownEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractslider.h:91
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isSliderDown() const

/*

*/
impl /*struct*/ QAbstractSlider {
  pub fn isSliderDown_0<RetType, T: QAbstractSlider_isSliderDown_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isSliderDown_0(self);
    // return 1;
  }
}
pub trait QAbstractSlider_isSliderDown_0<RetType> {
  fn isSliderDown_0(self , rsthis: & QAbstractSlider) -> RetType;
}
impl<'a> /*trait*/ QAbstractSlider_isSliderDown_0<bool> for () {
  fn isSliderDown_0(self , rsthis: & QAbstractSlider) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QAbstractSlider12isSliderDownEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractslider.h:93
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSliderPosition(int)

/*

*/
impl /*struct*/ QAbstractSlider {
  pub fn setSliderPosition_0<RetType, T: QAbstractSlider_setSliderPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSliderPosition_0(self);
    // return 1;
  }
}
pub trait QAbstractSlider_setSliderPosition_0<RetType> {
  fn setSliderPosition_0(self , rsthis: & QAbstractSlider) -> RetType;
}
impl<'a> /*trait*/ QAbstractSlider_setSliderPosition_0<(/*void*/)> for (i32) {
  fn setSliderPosition_0(self , rsthis: & QAbstractSlider) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractSlider17setSliderPositionEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractslider.h:94
// index:0
// Public Visibility=Default Availability=Available
// [4] int sliderPosition() const

/*

*/
impl /*struct*/ QAbstractSlider {
  pub fn sliderPosition_0<RetType, T: QAbstractSlider_sliderPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sliderPosition_0(self);
    // return 1;
  }
}
pub trait QAbstractSlider_sliderPosition_0<RetType> {
  fn sliderPosition_0(self , rsthis: & QAbstractSlider) -> RetType;
}
impl<'a> /*trait*/ QAbstractSlider_sliderPosition_0<i32> for () {
  fn sliderPosition_0(self , rsthis: & QAbstractSlider) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QAbstractSlider14sliderPositionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractslider.h:96
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setInvertedAppearance(bool)

/*

*/
impl /*struct*/ QAbstractSlider {
  pub fn setInvertedAppearance_0<RetType, T: QAbstractSlider_setInvertedAppearance_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setInvertedAppearance_0(self);
    // return 1;
  }
}
pub trait QAbstractSlider_setInvertedAppearance_0<RetType> {
  fn setInvertedAppearance_0(self , rsthis: & QAbstractSlider) -> RetType;
}
impl<'a> /*trait*/ QAbstractSlider_setInvertedAppearance_0<(/*void*/)> for (bool) {
  fn setInvertedAppearance_0(self , rsthis: & QAbstractSlider) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractSlider21setInvertedAppearanceEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractslider.h:97
// index:0
// Public Visibility=Default Availability=Available
// [1] bool invertedAppearance() const

/*

*/
impl /*struct*/ QAbstractSlider {
  pub fn invertedAppearance_0<RetType, T: QAbstractSlider_invertedAppearance_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.invertedAppearance_0(self);
    // return 1;
  }
}
pub trait QAbstractSlider_invertedAppearance_0<RetType> {
  fn invertedAppearance_0(self , rsthis: & QAbstractSlider) -> RetType;
}
impl<'a> /*trait*/ QAbstractSlider_invertedAppearance_0<bool> for () {
  fn invertedAppearance_0(self , rsthis: & QAbstractSlider) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QAbstractSlider18invertedAppearanceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractslider.h:99
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setInvertedControls(bool)

/*

*/
impl /*struct*/ QAbstractSlider {
  pub fn setInvertedControls_0<RetType, T: QAbstractSlider_setInvertedControls_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setInvertedControls_0(self);
    // return 1;
  }
}
pub trait QAbstractSlider_setInvertedControls_0<RetType> {
  fn setInvertedControls_0(self , rsthis: & QAbstractSlider) -> RetType;
}
impl<'a> /*trait*/ QAbstractSlider_setInvertedControls_0<(/*void*/)> for (bool) {
  fn setInvertedControls_0(self , rsthis: & QAbstractSlider) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractSlider19setInvertedControlsEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractslider.h:100
// index:0
// Public Visibility=Default Availability=Available
// [1] bool invertedControls() const

/*

*/
impl /*struct*/ QAbstractSlider {
  pub fn invertedControls_0<RetType, T: QAbstractSlider_invertedControls_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.invertedControls_0(self);
    // return 1;
  }
}
pub trait QAbstractSlider_invertedControls_0<RetType> {
  fn invertedControls_0(self , rsthis: & QAbstractSlider) -> RetType;
}
impl<'a> /*trait*/ QAbstractSlider_invertedControls_0<bool> for () {
  fn invertedControls_0(self , rsthis: & QAbstractSlider) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QAbstractSlider16invertedControlsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractslider.h:113
// index:0
// Public Visibility=Default Availability=Available
// [4] int value() const

/*

*/
impl /*struct*/ QAbstractSlider {
  pub fn value_0<RetType, T: QAbstractSlider_value_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.value_0(self);
    // return 1;
  }
}
pub trait QAbstractSlider_value_0<RetType> {
  fn value_0(self , rsthis: & QAbstractSlider) -> RetType;
}
impl<'a> /*trait*/ QAbstractSlider_value_0<i32> for () {
  fn value_0(self , rsthis: & QAbstractSlider) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QAbstractSlider5valueEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractslider.h:115
// index:0
// Public Visibility=Default Availability=Available
// [-2] void triggerAction(QAbstractSlider::SliderAction)

/*
Triggers a slider action. Possible actions are SliderSingleStepAdd, SliderSingleStepSub, SliderPageStepAdd, SliderPageStepSub, SliderToMinimum, SliderToMaximum, and SliderMove.

See also actionTriggered().
*/
impl /*struct*/ QAbstractSlider {
  pub fn triggerAction_0<RetType, T: QAbstractSlider_triggerAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.triggerAction_0(self);
    // return 1;
  }
}
pub trait QAbstractSlider_triggerAction_0<RetType> {
  fn triggerAction_0(self , rsthis: & QAbstractSlider) -> RetType;
}
impl<'a> /*trait*/ QAbstractSlider_triggerAction_0<(/*void*/)> for (i32) {
  fn triggerAction_0(self , rsthis: & QAbstractSlider) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractSlider13triggerActionENS_12SliderActionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractslider.h:118
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setValue(int)

/*

*/
impl /*struct*/ QAbstractSlider {
  pub fn setValue_0<RetType, T: QAbstractSlider_setValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setValue_0(self);
    // return 1;
  }
}
pub trait QAbstractSlider_setValue_0<RetType> {
  fn setValue_0(self , rsthis: & QAbstractSlider) -> RetType;
}
impl<'a> /*trait*/ QAbstractSlider_setValue_0<(/*void*/)> for (i32) {
  fn setValue_0(self , rsthis: & QAbstractSlider) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractSlider8setValueEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractslider.h:119
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOrientation(Qt::Orientation)

/*

*/
impl /*struct*/ QAbstractSlider {
  pub fn setOrientation_0<RetType, T: QAbstractSlider_setOrientation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOrientation_0(self);
    // return 1;
  }
}
pub trait QAbstractSlider_setOrientation_0<RetType> {
  fn setOrientation_0(self , rsthis: & QAbstractSlider) -> RetType;
}
impl<'a> /*trait*/ QAbstractSlider_setOrientation_0<(/*void*/)> for (i32) {
  fn setOrientation_0(self , rsthis: & QAbstractSlider) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractSlider14setOrientationEN2Qt11OrientationE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractslider.h:120
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRange(int, int)

/*
Sets the slider's minimum to min and its maximum to max.

If max is smaller than min, min becomes the only legal value.

See also minimum and maximum.
*/
impl /*struct*/ QAbstractSlider {
  pub fn setRange_0<RetType, T: QAbstractSlider_setRange_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRange_0(self);
    // return 1;
  }
}
pub trait QAbstractSlider_setRange_0<RetType> {
  fn setRange_0(self , rsthis: & QAbstractSlider) -> RetType;
}
impl<'a> /*trait*/ QAbstractSlider_setRange_0<(/*void*/)> for (i32,i32) {
  fn setRange_0(self , rsthis: & QAbstractSlider) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractSlider8setRangeEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractslider.h:123
// index:0
// Public Visibility=Default Availability=Available
// [-2] void valueChanged(int)

/*
This signal is emitted when the slider value has changed, with the new slider value as argument.

Note: Notifier signal for property value.
*/
impl /*struct*/ QAbstractSlider {
  pub fn valueChanged_0<RetType, T: QAbstractSlider_valueChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.valueChanged_0(self);
    // return 1;
  }
}
pub trait QAbstractSlider_valueChanged_0<RetType> {
  fn valueChanged_0(self , rsthis: & QAbstractSlider) -> RetType;
}
impl<'a> /*trait*/ QAbstractSlider_valueChanged_0<(/*void*/)> for (i32) {
  fn valueChanged_0(self , rsthis: & QAbstractSlider) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractSlider12valueChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractslider.h:125
// index:0
// Public Visibility=Default Availability=Available
// [-2] void sliderPressed()

/*
This signal is emitted when the user presses the slider with the mouse, or programmatically when setSliderDown(true) is called.

See also sliderReleased(), sliderMoved(), and isSliderDown().
*/
impl /*struct*/ QAbstractSlider {
  pub fn sliderPressed_0<RetType, T: QAbstractSlider_sliderPressed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sliderPressed_0(self);
    // return 1;
  }
}
pub trait QAbstractSlider_sliderPressed_0<RetType> {
  fn sliderPressed_0(self , rsthis: & QAbstractSlider) -> RetType;
}
impl<'a> /*trait*/ QAbstractSlider_sliderPressed_0<(/*void*/)> for () {
  fn sliderPressed_0(self , rsthis: & QAbstractSlider) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QAbstractSlider13sliderPressedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractslider.h:126
// index:0
// Public Visibility=Default Availability=Available
// [-2] void sliderMoved(int)

/*
This signal is emitted when sliderDown is true and the slider moves. This usually happens when the user is dragging the slider. The value is the new slider position.

This signal is emitted even when tracking is turned off.

Note: Notifier signal for property sliderPosition. 

See also setTracking(), valueChanged(), isSliderDown(), sliderPressed(), and sliderReleased().
*/
impl /*struct*/ QAbstractSlider {
  pub fn sliderMoved_0<RetType, T: QAbstractSlider_sliderMoved_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sliderMoved_0(self);
    // return 1;
  }
}
pub trait QAbstractSlider_sliderMoved_0<RetType> {
  fn sliderMoved_0(self , rsthis: & QAbstractSlider) -> RetType;
}
impl<'a> /*trait*/ QAbstractSlider_sliderMoved_0<(/*void*/)> for (i32) {
  fn sliderMoved_0(self , rsthis: & QAbstractSlider) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractSlider11sliderMovedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractslider.h:127
// index:0
// Public Visibility=Default Availability=Available
// [-2] void sliderReleased()

/*
This signal is emitted when the user releases the slider with the mouse, or programmatically when setSliderDown(false) is called.

See also sliderPressed(), sliderMoved(), and sliderDown.
*/
impl /*struct*/ QAbstractSlider {
  pub fn sliderReleased_0<RetType, T: QAbstractSlider_sliderReleased_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sliderReleased_0(self);
    // return 1;
  }
}
pub trait QAbstractSlider_sliderReleased_0<RetType> {
  fn sliderReleased_0(self , rsthis: & QAbstractSlider) -> RetType;
}
impl<'a> /*trait*/ QAbstractSlider_sliderReleased_0<(/*void*/)> for () {
  fn sliderReleased_0(self , rsthis: & QAbstractSlider) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QAbstractSlider14sliderReleasedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractslider.h:129
// index:0
// Public Visibility=Default Availability=Available
// [-2] void rangeChanged(int, int)

/*
This signal is emitted when the slider range has changed, with min being the new minimum, and max being the new maximum.

See also minimum and maximum.
*/
impl /*struct*/ QAbstractSlider {
  pub fn rangeChanged_0<RetType, T: QAbstractSlider_rangeChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rangeChanged_0(self);
    // return 1;
  }
}
pub trait QAbstractSlider_rangeChanged_0<RetType> {
  fn rangeChanged_0(self , rsthis: & QAbstractSlider) -> RetType;
}
impl<'a> /*trait*/ QAbstractSlider_rangeChanged_0<(/*void*/)> for (i32,i32) {
  fn rangeChanged_0(self , rsthis: & QAbstractSlider) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractSlider12rangeChangedEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractslider.h:131
// index:0
// Public Visibility=Default Availability=Available
// [-2] void actionTriggered(int)

/*
This signal is emitted when the slider action action is triggered. Actions are SliderSingleStepAdd, SliderSingleStepSub, SliderPageStepAdd, SliderPageStepSub, SliderToMinimum, SliderToMaximum, and SliderMove.

When the signal is emitted, the sliderPosition has been adjusted according to the action, but the value has not yet been propagated (meaning the valueChanged() signal was not yet emitted), and the visual display has not been updated. In slots connected to this signal you can thus safely adjust any action by calling setSliderPosition() yourself, based on both the action and the slider's value.

See also triggerAction().
*/
impl /*struct*/ QAbstractSlider {
  pub fn actionTriggered_0<RetType, T: QAbstractSlider_actionTriggered_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.actionTriggered_0(self);
    // return 1;
  }
}
pub trait QAbstractSlider_actionTriggered_0<RetType> {
  fn actionTriggered_0(self , rsthis: & QAbstractSlider) -> RetType;
}
impl<'a> /*trait*/ QAbstractSlider_actionTriggered_0<(/*void*/)> for (i32) {
  fn actionTriggered_0(self , rsthis: & QAbstractSlider) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractSlider15actionTriggeredEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractslider.h:134
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QAbstractSlider {
  pub fn event_0<RetType, T: QAbstractSlider_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QAbstractSlider_event_0<RetType> {
  fn event_0(self , rsthis: & QAbstractSlider) -> RetType;
}
impl<'a> /*trait*/ QAbstractSlider_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QAbstractSlider) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QAbstractSlider5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractslider.h:136
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void setRepeatAction(QAbstractSlider::SliderAction, int, int)

/*
Sets action action to be triggered repetitively in intervals of repeatTime, after an initial delay of thresholdTime.

See also triggerAction() and repeatAction().
*/
impl /*struct*/ QAbstractSlider {
  pub fn setRepeatAction_0<RetType, T: QAbstractSlider_setRepeatAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRepeatAction_0(self);
    // return 1;
  }
}
pub trait QAbstractSlider_setRepeatAction_0<RetType> {
  fn setRepeatAction_0(self , rsthis: & QAbstractSlider) -> RetType;
}
impl<'a> /*trait*/ QAbstractSlider_setRepeatAction_0<(/*void*/)> for (i32,i32,i32) {
  fn setRepeatAction_0(self , rsthis: & QAbstractSlider) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractSlider15setRepeatActionENS_12SliderActionEii", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractslider.h:137
// index:0
// Protected Visibility=Default Availability=Available
// [4] QAbstractSlider::SliderAction repeatAction() const

/*
Returns the current repeat action.

See also setRepeatAction().
*/
impl /*struct*/ QAbstractSlider {
  pub fn repeatAction_0<RetType, T: QAbstractSlider_repeatAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.repeatAction_0(self);
    // return 1;
  }
}
pub trait QAbstractSlider_repeatAction_0<RetType> {
  fn repeatAction_0(self , rsthis: & QAbstractSlider) -> RetType;
}
impl<'a> /*trait*/ QAbstractSlider_repeatAction_0<i32> for () {
  fn repeatAction_0(self , rsthis: & QAbstractSlider) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QAbstractSlider12repeatActionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractslider.h:145
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void sliderChange(QAbstractSlider::SliderChange)

/*
Reimplement this virtual function to track slider changes such as SliderRangeChange, SliderOrientationChange, SliderStepsChange, or SliderValueChange. The default implementation only updates the display and ignores the change parameter.
*/
impl /*struct*/ QAbstractSlider {
  pub fn sliderChange_0<RetType, T: QAbstractSlider_sliderChange_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sliderChange_0(self);
    // return 1;
  }
}
pub trait QAbstractSlider_sliderChange_0<RetType> {
  fn sliderChange_0(self , rsthis: & QAbstractSlider) -> RetType;
}
impl<'a> /*trait*/ QAbstractSlider_sliderChange_0<(/*void*/)> for (i32) {
  fn sliderChange_0(self , rsthis: & QAbstractSlider) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractSlider12sliderChangeENS_12SliderChangeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractslider.h:147
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyPressEvent(QKeyEvent *)

/*
Reimplemented from QWidget::keyPressEvent().
*/
impl /*struct*/ QAbstractSlider {
  pub fn keyPressEvent_0<RetType, T: QAbstractSlider_keyPressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyPressEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractSlider_keyPressEvent_0<RetType> {
  fn keyPressEvent_0(self , rsthis: & QAbstractSlider) -> RetType;
}
impl<'a> /*trait*/ QAbstractSlider_keyPressEvent_0<(/*void*/)> for (usize) {
  fn keyPressEvent_0(self , rsthis: & QAbstractSlider) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractSlider13keyPressEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractslider.h:148
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void timerEvent(QTimerEvent *)

/*
Reimplemented from QObject::timerEvent().
*/
impl /*struct*/ QAbstractSlider {
  pub fn timerEvent_0<RetType, T: QAbstractSlider_timerEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.timerEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractSlider_timerEvent_0<RetType> {
  fn timerEvent_0(self , rsthis: & QAbstractSlider) -> RetType;
}
impl<'a> /*trait*/ QAbstractSlider_timerEvent_0<(/*void*/)> for (usize) {
  fn timerEvent_0(self , rsthis: & QAbstractSlider) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractSlider10timerEventEP11QTimerEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractslider.h:150
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void wheelEvent(QWheelEvent *)

/*
Reimplemented from QWidget::wheelEvent().
*/
impl /*struct*/ QAbstractSlider {
  pub fn wheelEvent_0<RetType, T: QAbstractSlider_wheelEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.wheelEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractSlider_wheelEvent_0<RetType> {
  fn wheelEvent_0(self , rsthis: & QAbstractSlider) -> RetType;
}
impl<'a> /*trait*/ QAbstractSlider_wheelEvent_0<(/*void*/)> for (usize) {
  fn wheelEvent_0(self , rsthis: & QAbstractSlider) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractSlider10wheelEventEP11QWheelEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractslider.h:152
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void changeEvent(QEvent *)

/*
Reimplemented from QWidget::changeEvent().
*/
impl /*struct*/ QAbstractSlider {
  pub fn changeEvent_0<RetType, T: QAbstractSlider_changeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changeEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractSlider_changeEvent_0<RetType> {
  fn changeEvent_0(self , rsthis: & QAbstractSlider) -> RetType;
}
impl<'a> /*trait*/ QAbstractSlider_changeEvent_0<(/*void*/)> for (usize) {
  fn changeEvent_0(self , rsthis: & QAbstractSlider) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractSlider11changeEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
ConstantValue
QAbstractSlider::SliderNoAction0
QAbstractSlider::SliderSingleStepAdd1
QAbstractSlider::SliderSingleStepSub2
QAbstractSlider::SliderPageStepAdd3
QAbstractSlider::SliderPageStepSub4
QAbstractSlider::SliderToMinimum5
QAbstractSlider::SliderToMaximum6
QAbstractSlider::SliderMove7

*/
pub type QAbstractSlider__SliderAction = i32;
// 
pub const QAbstractSlider__SliderNoAction :QAbstractSlider__SliderAction = 0;
// 
pub const QAbstractSlider__SliderSingleStepAdd :QAbstractSlider__SliderAction = 1;
// 
pub const QAbstractSlider__SliderSingleStepSub :QAbstractSlider__SliderAction = 2;
// 
pub const QAbstractSlider__SliderPageStepAdd :QAbstractSlider__SliderAction = 3;
// 
pub const QAbstractSlider__SliderPageStepSub :QAbstractSlider__SliderAction = 4;
// 
pub const QAbstractSlider__SliderToMinimum :QAbstractSlider__SliderAction = 5;
// 
pub const QAbstractSlider__SliderToMaximum :QAbstractSlider__SliderAction = 6;
// 
pub const QAbstractSlider__SliderMove :QAbstractSlider__SliderAction = 7;
pub fn QAbstractSlider_SliderActionItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QAbstractSlider", val);
}
pub fn QAbstractSlider_SliderActionItemName_s(val: i32) ->String {
  //var nilthis *QAbstractSlider
  //return nilthis.SliderActionItemName(val);
  return QAbstractSlider_SliderActionItemName(val);
}


/*
ConstantValue
QAbstractSlider::SliderRangeChange0
QAbstractSlider::SliderOrientationChange1
QAbstractSlider::SliderStepsChange2
QAbstractSlider::SliderValueChange3

*/
pub type QAbstractSlider__SliderChange = i32;
// 
pub const QAbstractSlider__SliderRangeChange :QAbstractSlider__SliderChange = 0;
// 
pub const QAbstractSlider__SliderOrientationChange :QAbstractSlider__SliderChange = 1;
// 
pub const QAbstractSlider__SliderStepsChange :QAbstractSlider__SliderChange = 2;
// 
pub const QAbstractSlider__SliderValueChange :QAbstractSlider__SliderChange = 3;
pub fn QAbstractSlider_SliderChangeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QAbstractSlider", val);
}
pub fn QAbstractSlider_SliderChangeItemName_s(val: i32) ->String {
  //var nilthis *QAbstractSlider
  //return nilthis.SliderChangeItemName(val);
  return QAbstractSlider_SliderChangeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
