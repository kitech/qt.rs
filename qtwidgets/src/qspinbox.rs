

// mod ::widgets::QSpinBox
// package qtwidgets
// /usr/include/qt/QtWidgets/qspinbox.h
// #include <qspinbox.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 14
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
// func (this *QSpinBox) InheritEvent(f func(event *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// QValidator::State validate(QString &, int &)
// func (this *QSpinBox) InheritValidate(f func(input string, pos int) int) {
//  qtrt.SetAllInheritCallback(this, "validate", f)
// }

// int valueFromText(const QString &)
// func (this *QSpinBox) InheritValueFromText(f func(text string) int) {
//  qtrt.SetAllInheritCallback(this, "valueFromText", f)
// }

// QString textFromValue(int)
// func (this *QSpinBox) InheritTextFromValue(f func(val int) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "textFromValue", f)
// }

// void fixup(QString &)
// func (this *QSpinBox) InheritFixup(f func(str string) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "fixup", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QSpinBox)=48
pub struct QSpinBox {
  qbase: QAbstractSpinBox,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QSpinBox_ITF interface {
//    QAbstractSpinBox_ITF
//    QSpinBox_PTR() *QSpinBox
//}
//func (ptr *QSpinBox) QSpinBox_PTR() *QSpinBox { return ptr }

impl /*struct*/ QSpinBox {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QSpinBox {
    return QSpinBox{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QSpinBox {
//  type Target = QSpinBoxBASE;
//
//  fn deref(&self) -> &QSpinBoxBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QSpinBoxBASE> for QSpinBox {
//  fn as_ref(& self) -> & QSpinBoxBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qspinbox.h:53
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QSpinBox {
  pub fn metaObject_0<RetType, T: QSpinBox_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QSpinBox_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QSpinBox) -> RetType;
}
impl<'a> /*trait*/ QSpinBox_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QSpinBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QSpinBox10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:65
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QSpinBox(QWidget *)

/*
Constructs a spin box with 0 as minimum value and 99 as maximum value, a step value of 1. The value is initially set to 0. It is parented to parent.

See also setMinimum(), setMaximum(), and setSingleStep().
*/
// QSpinBox(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QSpinBox {
  pub fn QSpinBox_0<T: QSpinBox_QSpinBox_0>(value: T) -> QSpinBox {
    let rsthis = value.QSpinBox_0();
    return rsthis;
    // return 1;
  }
}

pub trait QSpinBox_QSpinBox_0 {
  fn QSpinBox_0(self) -> QSpinBox;
}
// QSpinBox(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSpinBox_QSpinBox_0 for (usize) {
  fn QSpinBox_0(self) -> QSpinBox {
    // unsafe{_ZN8QSpinBoxC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QSpinBoxC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSpinBox{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:66
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QSpinBox()

/*

*/
pub fn DeleteQSpinBox(this :*mut QSpinBox) {
    // let rv = qtrt::InvokeQtFunc6("_ZN8QSpinBoxD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qspinbox.h:68
// index:0
// Public Visibility=Default Availability=Available
// [4] int value() const

/*

*/
impl /*struct*/ QSpinBox {
  pub fn value_0<RetType, T: QSpinBox_value_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.value_0(self);
    // return 1;
  }
}
pub trait QSpinBox_value_0<RetType> {
  fn value_0(self , rsthis: & QSpinBox) -> RetType;
}
impl<'a> /*trait*/ QSpinBox_value_0<i32> for () {
  fn value_0(self , rsthis: & QSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QSpinBox5valueEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:70
// index:0
// Public Visibility=Default Availability=Available
// [8] QString prefix() const

/*

*/
impl /*struct*/ QSpinBox {
  pub fn prefix_0<RetType, T: QSpinBox_prefix_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.prefix_0(self);
    // return 1;
  }
}
pub trait QSpinBox_prefix_0<RetType> {
  fn prefix_0(self , rsthis: & QSpinBox) -> RetType;
}
impl<'a> /*trait*/ QSpinBox_prefix_0<usize> for () {
  fn prefix_0(self , rsthis: & QSpinBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QSpinBox6prefixEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:71
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPrefix(const QString &)

/*

*/
impl /*struct*/ QSpinBox {
  pub fn setPrefix_0<RetType, T: QSpinBox_setPrefix_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPrefix_0(self);
    // return 1;
  }
}
pub trait QSpinBox_setPrefix_0<RetType> {
  fn setPrefix_0(self , rsthis: & QSpinBox) -> RetType;
}
impl<'a> /*trait*/ QSpinBox_setPrefix_0<(/*void*/)> for (usize) {
  fn setPrefix_0(self , rsthis: & QSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QSpinBox9setPrefixERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:73
// index:0
// Public Visibility=Default Availability=Available
// [8] QString suffix() const

/*

*/
impl /*struct*/ QSpinBox {
  pub fn suffix_0<RetType, T: QSpinBox_suffix_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.suffix_0(self);
    // return 1;
  }
}
pub trait QSpinBox_suffix_0<RetType> {
  fn suffix_0(self , rsthis: & QSpinBox) -> RetType;
}
impl<'a> /*trait*/ QSpinBox_suffix_0<usize> for () {
  fn suffix_0(self , rsthis: & QSpinBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QSpinBox6suffixEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:74
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSuffix(const QString &)

/*

*/
impl /*struct*/ QSpinBox {
  pub fn setSuffix_0<RetType, T: QSpinBox_setSuffix_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSuffix_0(self);
    // return 1;
  }
}
pub trait QSpinBox_setSuffix_0<RetType> {
  fn setSuffix_0(self , rsthis: & QSpinBox) -> RetType;
}
impl<'a> /*trait*/ QSpinBox_setSuffix_0<(/*void*/)> for (usize) {
  fn setSuffix_0(self , rsthis: & QSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QSpinBox9setSuffixERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:76
// index:0
// Public Visibility=Default Availability=Available
// [8] QString cleanText() const

/*

*/
impl /*struct*/ QSpinBox {
  pub fn cleanText_0<RetType, T: QSpinBox_cleanText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cleanText_0(self);
    // return 1;
  }
}
pub trait QSpinBox_cleanText_0<RetType> {
  fn cleanText_0(self , rsthis: & QSpinBox) -> RetType;
}
impl<'a> /*trait*/ QSpinBox_cleanText_0<usize> for () {
  fn cleanText_0(self , rsthis: & QSpinBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QSpinBox9cleanTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:78
// index:0
// Public Visibility=Default Availability=Available
// [4] int singleStep() const

/*

*/
impl /*struct*/ QSpinBox {
  pub fn singleStep_0<RetType, T: QSpinBox_singleStep_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.singleStep_0(self);
    // return 1;
  }
}
pub trait QSpinBox_singleStep_0<RetType> {
  fn singleStep_0(self , rsthis: & QSpinBox) -> RetType;
}
impl<'a> /*trait*/ QSpinBox_singleStep_0<i32> for () {
  fn singleStep_0(self , rsthis: & QSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QSpinBox10singleStepEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:79
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSingleStep(int)

/*

*/
impl /*struct*/ QSpinBox {
  pub fn setSingleStep_0<RetType, T: QSpinBox_setSingleStep_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSingleStep_0(self);
    // return 1;
  }
}
pub trait QSpinBox_setSingleStep_0<RetType> {
  fn setSingleStep_0(self , rsthis: & QSpinBox) -> RetType;
}
impl<'a> /*trait*/ QSpinBox_setSingleStep_0<(/*void*/)> for (i32) {
  fn setSingleStep_0(self , rsthis: & QSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QSpinBox13setSingleStepEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:81
// index:0
// Public Visibility=Default Availability=Available
// [4] int minimum() const

/*

*/
impl /*struct*/ QSpinBox {
  pub fn minimum_0<RetType, T: QSpinBox_minimum_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimum_0(self);
    // return 1;
  }
}
pub trait QSpinBox_minimum_0<RetType> {
  fn minimum_0(self , rsthis: & QSpinBox) -> RetType;
}
impl<'a> /*trait*/ QSpinBox_minimum_0<i32> for () {
  fn minimum_0(self , rsthis: & QSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QSpinBox7minimumEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:82
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMinimum(int)

/*

*/
impl /*struct*/ QSpinBox {
  pub fn setMinimum_0<RetType, T: QSpinBox_setMinimum_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMinimum_0(self);
    // return 1;
  }
}
pub trait QSpinBox_setMinimum_0<RetType> {
  fn setMinimum_0(self , rsthis: & QSpinBox) -> RetType;
}
impl<'a> /*trait*/ QSpinBox_setMinimum_0<(/*void*/)> for (i32) {
  fn setMinimum_0(self , rsthis: & QSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QSpinBox10setMinimumEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:84
// index:0
// Public Visibility=Default Availability=Available
// [4] int maximum() const

/*

*/
impl /*struct*/ QSpinBox {
  pub fn maximum_0<RetType, T: QSpinBox_maximum_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maximum_0(self);
    // return 1;
  }
}
pub trait QSpinBox_maximum_0<RetType> {
  fn maximum_0(self , rsthis: & QSpinBox) -> RetType;
}
impl<'a> /*trait*/ QSpinBox_maximum_0<i32> for () {
  fn maximum_0(self , rsthis: & QSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QSpinBox7maximumEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:85
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMaximum(int)

/*

*/
impl /*struct*/ QSpinBox {
  pub fn setMaximum_0<RetType, T: QSpinBox_setMaximum_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMaximum_0(self);
    // return 1;
  }
}
pub trait QSpinBox_setMaximum_0<RetType> {
  fn setMaximum_0(self , rsthis: & QSpinBox) -> RetType;
}
impl<'a> /*trait*/ QSpinBox_setMaximum_0<(/*void*/)> for (i32) {
  fn setMaximum_0(self , rsthis: & QSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QSpinBox10setMaximumEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:87
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRange(int, int)

/*
Convenience function to set the minimum, and maximum values with a single function call.


  setRange(minimum, maximum);



is equivalent to:


  setMinimum(minimum);
  setMaximum(maximum);



See also minimum and maximum.
*/
impl /*struct*/ QSpinBox {
  pub fn setRange_0<RetType, T: QSpinBox_setRange_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRange_0(self);
    // return 1;
  }
}
pub trait QSpinBox_setRange_0<RetType> {
  fn setRange_0(self , rsthis: & QSpinBox) -> RetType;
}
impl<'a> /*trait*/ QSpinBox_setRange_0<(/*void*/)> for (i32,i32) {
  fn setRange_0(self , rsthis: & QSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QSpinBox8setRangeEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:89
// index:0
// Public Visibility=Default Availability=Available
// [4] int displayIntegerBase() const

/*

*/
impl /*struct*/ QSpinBox {
  pub fn displayIntegerBase_0<RetType, T: QSpinBox_displayIntegerBase_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.displayIntegerBase_0(self);
    // return 1;
  }
}
pub trait QSpinBox_displayIntegerBase_0<RetType> {
  fn displayIntegerBase_0(self , rsthis: & QSpinBox) -> RetType;
}
impl<'a> /*trait*/ QSpinBox_displayIntegerBase_0<i32> for () {
  fn displayIntegerBase_0(self , rsthis: & QSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QSpinBox18displayIntegerBaseEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:90
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDisplayIntegerBase(int)

/*

*/
impl /*struct*/ QSpinBox {
  pub fn setDisplayIntegerBase_0<RetType, T: QSpinBox_setDisplayIntegerBase_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDisplayIntegerBase_0(self);
    // return 1;
  }
}
pub trait QSpinBox_setDisplayIntegerBase_0<RetType> {
  fn setDisplayIntegerBase_0(self , rsthis: & QSpinBox) -> RetType;
}
impl<'a> /*trait*/ QSpinBox_setDisplayIntegerBase_0<(/*void*/)> for (i32) {
  fn setDisplayIntegerBase_0(self , rsthis: & QSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QSpinBox21setDisplayIntegerBaseEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:93
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QSpinBox {
  pub fn event_0<RetType, T: QSpinBox_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QSpinBox_event_0<RetType> {
  fn event_0(self , rsthis: & QSpinBox) -> RetType;
}
impl<'a> /*trait*/ QSpinBox_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QSpinBox) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QSpinBox5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:94
// index:0
// Protected virtual Visibility=Default Availability=Available
// [4] QValidator::State validate(QString &, int &) const

/*
Reimplemented from QAbstractSpinBox::validate().
*/
impl /*struct*/ QSpinBox {
  pub fn validate_0<RetType, T: QSpinBox_validate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.validate_0(self);
    // return 1;
  }
}
pub trait QSpinBox_validate_0<RetType> {
  fn validate_0(self , rsthis: & QSpinBox) -> RetType;
}
impl<'a> /*trait*/ QSpinBox_validate_0<i32> for (usize,usize) {
  fn validate_0(self , rsthis: & QSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QSpinBox8validateER7QStringRi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:95
// index:0
// Protected virtual Visibility=Default Availability=Available
// [4] int valueFromText(const QString &) const

/*
This virtual function is used by the spin box whenever it needs to interpret text entered by the user as a value.

Subclasses that need to display spin box values in a non-numeric way need to reimplement this function.

Note: QSpinBox handles specialValueText() separately; this function is only concerned with the other values.

See also textFromValue() and validate().
*/
impl /*struct*/ QSpinBox {
  pub fn valueFromText_0<RetType, T: QSpinBox_valueFromText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.valueFromText_0(self);
    // return 1;
  }
}
pub trait QSpinBox_valueFromText_0<RetType> {
  fn valueFromText_0(self , rsthis: & QSpinBox) -> RetType;
}
impl<'a> /*trait*/ QSpinBox_valueFromText_0<i32> for (usize) {
  fn valueFromText_0(self , rsthis: & QSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QSpinBox13valueFromTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:96
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] QString textFromValue(int) const

/*
This virtual function is used by the spin box whenever it needs to display the given value. The default implementation returns a string containing value printed in the standard way using QWidget::locale().toString(), but with the thousand separator removed unless setGroupSeparatorShown() is set. Reimplementations may return anything. (See the example in the detailed description.)

Note: QSpinBox does not call this function for specialValueText() and that neither prefix() nor suffix() should be included in the return value.

If you reimplement this, you may also need to reimplement valueFromText() and validate()

See also valueFromText(), validate(), and QLocale::groupSeparator().
*/
impl /*struct*/ QSpinBox {
  pub fn textFromValue_0<RetType, T: QSpinBox_textFromValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textFromValue_0(self);
    // return 1;
  }
}
pub trait QSpinBox_textFromValue_0<RetType> {
  fn textFromValue_0(self , rsthis: & QSpinBox) -> RetType;
}
impl<'a> /*trait*/ QSpinBox_textFromValue_0<usize> for (i32) {
  fn textFromValue_0(self , rsthis: & QSpinBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QSpinBox13textFromValueEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:97
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void fixup(QString &) const

/*
Reimplemented from QAbstractSpinBox::fixup().
*/
impl /*struct*/ QSpinBox {
  pub fn fixup_0<RetType, T: QSpinBox_fixup_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fixup_0(self);
    // return 1;
  }
}
pub trait QSpinBox_fixup_0<RetType> {
  fn fixup_0(self , rsthis: & QSpinBox) -> RetType;
}
impl<'a> /*trait*/ QSpinBox_fixup_0<(/*void*/)> for (usize) {
  fn fixup_0(self , rsthis: & QSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK8QSpinBox5fixupER7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:101
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setValue(int)

/*

*/
impl /*struct*/ QSpinBox {
  pub fn setValue_0<RetType, T: QSpinBox_setValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setValue_0(self);
    // return 1;
  }
}
pub trait QSpinBox_setValue_0<RetType> {
  fn setValue_0(self , rsthis: & QSpinBox) -> RetType;
}
impl<'a> /*trait*/ QSpinBox_setValue_0<(/*void*/)> for (i32) {
  fn setValue_0(self , rsthis: & QSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QSpinBox8setValueEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:104
// index:0
// Public Visibility=Default Availability=Available
// [-2] void valueChanged(int)

/*
This signal is emitted whenever the spin box's value is changed. The new value's integer value is passed in i.

Note: Signal valueChanged is overloaded in this class. To connect to this signal by using the function pointer syntax, Qt provides a convenient helper for obtaining the function pointer as shown in this example:


  connect(spinBox, QOverload<int>::of(&QSpinBox::valueChanged),
      [=](int i){ /-* ... *-/ });



Note: Notifier signal for property value.
*/
impl /*struct*/ QSpinBox {
  pub fn valueChanged_0<RetType, T: QSpinBox_valueChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.valueChanged_0(self);
    // return 1;
  }
}
pub trait QSpinBox_valueChanged_0<RetType> {
  fn valueChanged_0(self , rsthis: & QSpinBox) -> RetType;
}
impl<'a> /*trait*/ QSpinBox_valueChanged_0<(/*void*/)> for (i32) {
  fn valueChanged_0(self , rsthis: & QSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QSpinBox12valueChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:105
// index:1
// Public Visibility=Default Availability=Available
// [-2] void valueChanged(const QString &)

/*
This signal is emitted whenever the spin box's value is changed. The new value's integer value is passed in i.

Note: Signal valueChanged is overloaded in this class. To connect to this signal by using the function pointer syntax, Qt provides a convenient helper for obtaining the function pointer as shown in this example:


  connect(spinBox, QOverload<int>::of(&QSpinBox::valueChanged),
      [=](int i){ /-* ... *-/ });



Note: Notifier signal for property value.
*/
impl /*struct*/ QSpinBox {
  pub fn valueChanged_1<RetType, T: QSpinBox_valueChanged_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.valueChanged_1(self);
    // return 1;
  }
}
pub trait QSpinBox_valueChanged_1<RetType> {
  fn valueChanged_1(self , rsthis: & QSpinBox) -> RetType;
}
impl<'a> /*trait*/ QSpinBox_valueChanged_1<(/*void*/)> for (usize) {
  fn valueChanged_1(self , rsthis: & QSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QSpinBox12valueChangedERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
