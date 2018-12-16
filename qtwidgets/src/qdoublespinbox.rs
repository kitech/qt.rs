

// mod ::widgets::QDoubleSpinBox
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
// extern C begin: 26
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
#[derive(Default)] // class sizeof(QDoubleSpinBox)=48
pub struct QDoubleSpinBox {
  qbase: QAbstractSpinBox,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QDoubleSpinBox_ITF interface {
//    QAbstractSpinBox_ITF
//    QDoubleSpinBox_PTR() *QDoubleSpinBox
//}
//func (ptr *QDoubleSpinBox) QDoubleSpinBox_PTR() *QDoubleSpinBox { return ptr }

impl /*struct*/ QDoubleSpinBox {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QDoubleSpinBox {
    return QDoubleSpinBox{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QDoubleSpinBox {
//  type Target = QDoubleSpinBoxBASE;
//
//  fn deref(&self) -> &QDoubleSpinBoxBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QDoubleSpinBoxBASE> for QDoubleSpinBox {
//  fn as_ref(& self) -> & QDoubleSpinBoxBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qspinbox.h:115
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QDoubleSpinBox {
  pub fn metaObject_0<RetType, T: QDoubleSpinBox_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QDoubleSpinBox_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QDoubleSpinBox) -> RetType;
}
impl<'a> /*trait*/ QDoubleSpinBox_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QDoubleSpinBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QDoubleSpinBox10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:126
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QDoubleSpinBox(QWidget *)

/*

*/
// QDoubleSpinBox(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QDoubleSpinBox {
  pub fn QDoubleSpinBox_0<T: QDoubleSpinBox_QDoubleSpinBox_0>(value: T) -> QDoubleSpinBox {
    let rsthis = value.QDoubleSpinBox_0();
    return rsthis;
    // return 1;
  }
}

pub trait QDoubleSpinBox_QDoubleSpinBox_0 {
  fn QDoubleSpinBox_0(self) -> QDoubleSpinBox;
}
// QDoubleSpinBox(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDoubleSpinBox_QDoubleSpinBox_0 for (usize) {
  fn QDoubleSpinBox_0(self) -> QDoubleSpinBox {
    // unsafe{_ZN14QDoubleSpinBoxC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QDoubleSpinBoxC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDoubleSpinBox{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:127
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QDoubleSpinBox()

/*

*/
pub fn DeleteQDoubleSpinBox(this :*mut QDoubleSpinBox) {
    // let rv = qtrt::InvokeQtFunc6("_ZN14QDoubleSpinBoxD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qspinbox.h:129
// index:0
// Public Visibility=Default Availability=Available
// [8] double value() const

/*

*/
impl /*struct*/ QDoubleSpinBox {
  pub fn value_0<RetType, T: QDoubleSpinBox_value_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.value_0(self);
    // return 1;
  }
}
pub trait QDoubleSpinBox_value_0<RetType> {
  fn value_0(self , rsthis: & QDoubleSpinBox) -> RetType;
}
impl<'a> /*trait*/ QDoubleSpinBox_value_0<f64> for () {
  fn value_0(self , rsthis: & QDoubleSpinBox) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QDoubleSpinBox5valueEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:131
// index:0
// Public Visibility=Default Availability=Available
// [8] QString prefix() const

/*

*/
impl /*struct*/ QDoubleSpinBox {
  pub fn prefix_0<RetType, T: QDoubleSpinBox_prefix_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.prefix_0(self);
    // return 1;
  }
}
pub trait QDoubleSpinBox_prefix_0<RetType> {
  fn prefix_0(self , rsthis: & QDoubleSpinBox) -> RetType;
}
impl<'a> /*trait*/ QDoubleSpinBox_prefix_0<usize> for () {
  fn prefix_0(self , rsthis: & QDoubleSpinBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QDoubleSpinBox6prefixEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:132
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPrefix(const QString &)

/*

*/
impl /*struct*/ QDoubleSpinBox {
  pub fn setPrefix_0<RetType, T: QDoubleSpinBox_setPrefix_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPrefix_0(self);
    // return 1;
  }
}
pub trait QDoubleSpinBox_setPrefix_0<RetType> {
  fn setPrefix_0(self , rsthis: & QDoubleSpinBox) -> RetType;
}
impl<'a> /*trait*/ QDoubleSpinBox_setPrefix_0<(/*void*/)> for (usize) {
  fn setPrefix_0(self , rsthis: & QDoubleSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QDoubleSpinBox9setPrefixERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:134
// index:0
// Public Visibility=Default Availability=Available
// [8] QString suffix() const

/*

*/
impl /*struct*/ QDoubleSpinBox {
  pub fn suffix_0<RetType, T: QDoubleSpinBox_suffix_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.suffix_0(self);
    // return 1;
  }
}
pub trait QDoubleSpinBox_suffix_0<RetType> {
  fn suffix_0(self , rsthis: & QDoubleSpinBox) -> RetType;
}
impl<'a> /*trait*/ QDoubleSpinBox_suffix_0<usize> for () {
  fn suffix_0(self , rsthis: & QDoubleSpinBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QDoubleSpinBox6suffixEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:135
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSuffix(const QString &)

/*

*/
impl /*struct*/ QDoubleSpinBox {
  pub fn setSuffix_0<RetType, T: QDoubleSpinBox_setSuffix_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSuffix_0(self);
    // return 1;
  }
}
pub trait QDoubleSpinBox_setSuffix_0<RetType> {
  fn setSuffix_0(self , rsthis: & QDoubleSpinBox) -> RetType;
}
impl<'a> /*trait*/ QDoubleSpinBox_setSuffix_0<(/*void*/)> for (usize) {
  fn setSuffix_0(self , rsthis: & QDoubleSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QDoubleSpinBox9setSuffixERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:137
// index:0
// Public Visibility=Default Availability=Available
// [8] QString cleanText() const

/*

*/
impl /*struct*/ QDoubleSpinBox {
  pub fn cleanText_0<RetType, T: QDoubleSpinBox_cleanText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cleanText_0(self);
    // return 1;
  }
}
pub trait QDoubleSpinBox_cleanText_0<RetType> {
  fn cleanText_0(self , rsthis: & QDoubleSpinBox) -> RetType;
}
impl<'a> /*trait*/ QDoubleSpinBox_cleanText_0<usize> for () {
  fn cleanText_0(self , rsthis: & QDoubleSpinBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QDoubleSpinBox9cleanTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:139
// index:0
// Public Visibility=Default Availability=Available
// [8] double singleStep() const

/*

*/
impl /*struct*/ QDoubleSpinBox {
  pub fn singleStep_0<RetType, T: QDoubleSpinBox_singleStep_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.singleStep_0(self);
    // return 1;
  }
}
pub trait QDoubleSpinBox_singleStep_0<RetType> {
  fn singleStep_0(self , rsthis: & QDoubleSpinBox) -> RetType;
}
impl<'a> /*trait*/ QDoubleSpinBox_singleStep_0<f64> for () {
  fn singleStep_0(self , rsthis: & QDoubleSpinBox) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QDoubleSpinBox10singleStepEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:140
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSingleStep(double)

/*

*/
impl /*struct*/ QDoubleSpinBox {
  pub fn setSingleStep_0<RetType, T: QDoubleSpinBox_setSingleStep_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSingleStep_0(self);
    // return 1;
  }
}
pub trait QDoubleSpinBox_setSingleStep_0<RetType> {
  fn setSingleStep_0(self , rsthis: & QDoubleSpinBox) -> RetType;
}
impl<'a> /*trait*/ QDoubleSpinBox_setSingleStep_0<(/*void*/)> for (f64) {
  fn setSingleStep_0(self , rsthis: & QDoubleSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN14QDoubleSpinBox13setSingleStepEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:142
// index:0
// Public Visibility=Default Availability=Available
// [8] double minimum() const

/*

*/
impl /*struct*/ QDoubleSpinBox {
  pub fn minimum_0<RetType, T: QDoubleSpinBox_minimum_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimum_0(self);
    // return 1;
  }
}
pub trait QDoubleSpinBox_minimum_0<RetType> {
  fn minimum_0(self , rsthis: & QDoubleSpinBox) -> RetType;
}
impl<'a> /*trait*/ QDoubleSpinBox_minimum_0<f64> for () {
  fn minimum_0(self , rsthis: & QDoubleSpinBox) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QDoubleSpinBox7minimumEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:143
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMinimum(double)

/*

*/
impl /*struct*/ QDoubleSpinBox {
  pub fn setMinimum_0<RetType, T: QDoubleSpinBox_setMinimum_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMinimum_0(self);
    // return 1;
  }
}
pub trait QDoubleSpinBox_setMinimum_0<RetType> {
  fn setMinimum_0(self , rsthis: & QDoubleSpinBox) -> RetType;
}
impl<'a> /*trait*/ QDoubleSpinBox_setMinimum_0<(/*void*/)> for (f64) {
  fn setMinimum_0(self , rsthis: & QDoubleSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN14QDoubleSpinBox10setMinimumEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:145
// index:0
// Public Visibility=Default Availability=Available
// [8] double maximum() const

/*

*/
impl /*struct*/ QDoubleSpinBox {
  pub fn maximum_0<RetType, T: QDoubleSpinBox_maximum_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maximum_0(self);
    // return 1;
  }
}
pub trait QDoubleSpinBox_maximum_0<RetType> {
  fn maximum_0(self , rsthis: & QDoubleSpinBox) -> RetType;
}
impl<'a> /*trait*/ QDoubleSpinBox_maximum_0<f64> for () {
  fn maximum_0(self , rsthis: & QDoubleSpinBox) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QDoubleSpinBox7maximumEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:146
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMaximum(double)

/*

*/
impl /*struct*/ QDoubleSpinBox {
  pub fn setMaximum_0<RetType, T: QDoubleSpinBox_setMaximum_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMaximum_0(self);
    // return 1;
  }
}
pub trait QDoubleSpinBox_setMaximum_0<RetType> {
  fn setMaximum_0(self , rsthis: & QDoubleSpinBox) -> RetType;
}
impl<'a> /*trait*/ QDoubleSpinBox_setMaximum_0<(/*void*/)> for (f64) {
  fn setMaximum_0(self , rsthis: & QDoubleSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN14QDoubleSpinBox10setMaximumEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:148
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRange(double, double)

/*
Convenience function to set the minimum, and maximum values with a single function call.


  setRange(minimum, maximum);



is equivalent to:


  setMinimum(minimum);
  setMaximum(maximum);



See also minimum and maximum.
*/
impl /*struct*/ QDoubleSpinBox {
  pub fn setRange_0<RetType, T: QDoubleSpinBox_setRange_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRange_0(self);
    // return 1;
  }
}
pub trait QDoubleSpinBox_setRange_0<RetType> {
  fn setRange_0(self , rsthis: & QDoubleSpinBox) -> RetType;
}
impl<'a> /*trait*/ QDoubleSpinBox_setRange_0<(/*void*/)> for (f64,f64) {
  fn setRange_0(self , rsthis: & QDoubleSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN14QDoubleSpinBox8setRangeEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:150
// index:0
// Public Visibility=Default Availability=Available
// [4] int decimals() const

/*

*/
impl /*struct*/ QDoubleSpinBox {
  pub fn decimals_0<RetType, T: QDoubleSpinBox_decimals_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.decimals_0(self);
    // return 1;
  }
}
pub trait QDoubleSpinBox_decimals_0<RetType> {
  fn decimals_0(self , rsthis: & QDoubleSpinBox) -> RetType;
}
impl<'a> /*trait*/ QDoubleSpinBox_decimals_0<i32> for () {
  fn decimals_0(self , rsthis: & QDoubleSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QDoubleSpinBox8decimalsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:151
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDecimals(int)

/*

*/
impl /*struct*/ QDoubleSpinBox {
  pub fn setDecimals_0<RetType, T: QDoubleSpinBox_setDecimals_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDecimals_0(self);
    // return 1;
  }
}
pub trait QDoubleSpinBox_setDecimals_0<RetType> {
  fn setDecimals_0(self , rsthis: & QDoubleSpinBox) -> RetType;
}
impl<'a> /*trait*/ QDoubleSpinBox_setDecimals_0<(/*void*/)> for (i32) {
  fn setDecimals_0(self , rsthis: & QDoubleSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QDoubleSpinBox11setDecimalsEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:153
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] QValidator::State validate(QString &, int &) const

/*
Reimplemented from QAbstractSpinBox::validate().
*/
impl /*struct*/ QDoubleSpinBox {
  pub fn validate_0<RetType, T: QDoubleSpinBox_validate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.validate_0(self);
    // return 1;
  }
}
pub trait QDoubleSpinBox_validate_0<RetType> {
  fn validate_0(self , rsthis: & QDoubleSpinBox) -> RetType;
}
impl<'a> /*trait*/ QDoubleSpinBox_validate_0<i32> for (usize,usize) {
  fn validate_0(self , rsthis: & QDoubleSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QDoubleSpinBox8validateER7QStringRi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:154
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] double valueFromText(const QString &) const

/*
This virtual function is used by the spin box whenever it needs to interpret text entered by the user as a value.

Subclasses that need to display spin box values in a non-numeric way need to reimplement this function.

Note: QSpinBox handles specialValueText() separately; this function is only concerned with the other values.

See also textFromValue() and validate().
*/
impl /*struct*/ QDoubleSpinBox {
  pub fn valueFromText_0<RetType, T: QDoubleSpinBox_valueFromText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.valueFromText_0(self);
    // return 1;
  }
}
pub trait QDoubleSpinBox_valueFromText_0<RetType> {
  fn valueFromText_0(self , rsthis: & QDoubleSpinBox) -> RetType;
}
impl<'a> /*trait*/ QDoubleSpinBox_valueFromText_0<f64> for (usize) {
  fn valueFromText_0(self , rsthis: & QDoubleSpinBox) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QDoubleSpinBox13valueFromTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:155
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QString textFromValue(double) const

/*
This virtual function is used by the spin box whenever it needs to display the given value. The default implementation returns a string containing value printed in the standard way using QWidget::locale().toString(), but with the thousand separator removed unless setGroupSeparatorShown() is set. Reimplementations may return anything. (See the example in the detailed description.)

Note: QSpinBox does not call this function for specialValueText() and that neither prefix() nor suffix() should be included in the return value.

If you reimplement this, you may also need to reimplement valueFromText() and validate()

See also valueFromText(), validate(), and QLocale::groupSeparator().
*/
impl /*struct*/ QDoubleSpinBox {
  pub fn textFromValue_0<RetType, T: QDoubleSpinBox_textFromValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textFromValue_0(self);
    // return 1;
  }
}
pub trait QDoubleSpinBox_textFromValue_0<RetType> {
  fn textFromValue_0(self , rsthis: & QDoubleSpinBox) -> RetType;
}
impl<'a> /*trait*/ QDoubleSpinBox_textFromValue_0<usize> for (f64) {
  fn textFromValue_0(self , rsthis: & QDoubleSpinBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QDoubleSpinBox13textFromValueEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:156
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void fixup(QString &) const

/*
Reimplemented from QAbstractSpinBox::fixup().
*/
impl /*struct*/ QDoubleSpinBox {
  pub fn fixup_0<RetType, T: QDoubleSpinBox_fixup_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fixup_0(self);
    // return 1;
  }
}
pub trait QDoubleSpinBox_fixup_0<RetType> {
  fn fixup_0(self , rsthis: & QDoubleSpinBox) -> RetType;
}
impl<'a> /*trait*/ QDoubleSpinBox_fixup_0<(/*void*/)> for (usize) {
  fn fixup_0(self , rsthis: & QDoubleSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK14QDoubleSpinBox5fixupER7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:159
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setValue(double)

/*

*/
impl /*struct*/ QDoubleSpinBox {
  pub fn setValue_0<RetType, T: QDoubleSpinBox_setValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setValue_0(self);
    // return 1;
  }
}
pub trait QDoubleSpinBox_setValue_0<RetType> {
  fn setValue_0(self , rsthis: & QDoubleSpinBox) -> RetType;
}
impl<'a> /*trait*/ QDoubleSpinBox_setValue_0<(/*void*/)> for (f64) {
  fn setValue_0(self , rsthis: & QDoubleSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN14QDoubleSpinBox8setValueEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:162
// index:0
// Public Visibility=Default Availability=Available
// [-2] void valueChanged(double)

/*
This signal is emitted whenever the spin box's value is changed. The new value's integer value is passed in i.

Note: Signal valueChanged is overloaded in this class. To connect to this signal by using the function pointer syntax, Qt provides a convenient helper for obtaining the function pointer as shown in this example:


  connect(spinBox, QOverload<int>::of(&QSpinBox::valueChanged),
      [=](int i){ /-* ... *-/ });



Note: Notifier signal for property value.
*/
impl /*struct*/ QDoubleSpinBox {
  pub fn valueChanged_0<RetType, T: QDoubleSpinBox_valueChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.valueChanged_0(self);
    // return 1;
  }
}
pub trait QDoubleSpinBox_valueChanged_0<RetType> {
  fn valueChanged_0(self , rsthis: & QDoubleSpinBox) -> RetType;
}
impl<'a> /*trait*/ QDoubleSpinBox_valueChanged_0<(/*void*/)> for (f64) {
  fn valueChanged_0(self , rsthis: & QDoubleSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN14QDoubleSpinBox12valueChangedEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qspinbox.h:163
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
impl /*struct*/ QDoubleSpinBox {
  pub fn valueChanged_1<RetType, T: QDoubleSpinBox_valueChanged_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.valueChanged_1(self);
    // return 1;
  }
}
pub trait QDoubleSpinBox_valueChanged_1<RetType> {
  fn valueChanged_1(self , rsthis: & QDoubleSpinBox) -> RetType;
}
impl<'a> /*trait*/ QDoubleSpinBox_valueChanged_1<(/*void*/)> for (usize) {
  fn valueChanged_1(self , rsthis: & QDoubleSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QDoubleSpinBox12valueChangedERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
