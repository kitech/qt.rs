

// mod ::widgets::QProgressBar
// package qtwidgets
// /usr/include/qt/QtWidgets/qprogressbar.h
// #include <qprogressbar.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 13
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
// func (this *QProgressBar) InheritEvent(f func(e *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// void paintEvent(QPaintEvent *)
// func (this *QProgressBar) InheritPaintEvent(f func(arg0 *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }

// void initStyleOption(QStyleOptionProgressBar *)
// func (this *QProgressBar) InheritInitStyleOption(f func(option *QStyleOptionProgressBar/*777 QStyleOptionProgressBar **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "initStyleOption", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QProgressBar)=48
pub struct QProgressBar {
  qbase: QWidget,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QProgressBar_ITF interface {
//    QWidget_ITF
//    QProgressBar_PTR() *QProgressBar
//}
//func (ptr *QProgressBar) QProgressBar_PTR() *QProgressBar { return ptr }

impl /*struct*/ QProgressBar {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QProgressBar {
    return QProgressBar{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QProgressBar {
//  type Target = QProgressBarBASE;
//
//  fn deref(&self) -> &QProgressBarBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QProgressBarBASE> for QProgressBar {
//  fn as_ref(& self) -> & QProgressBarBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qprogressbar.h:55
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QProgressBar {
  pub fn metaObject_0<RetType, T: QProgressBar_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QProgressBar_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QProgressBar) -> RetType;
}
impl<'a> /*trait*/ QProgressBar_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QProgressBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QProgressBar10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qprogressbar.h:71
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QProgressBar(QWidget *)

/*
Constructs a progress bar with the given parent.

By default, the minimum step value is set to 0, and the maximum to 100.

See also setRange().
*/
// QProgressBar(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QProgressBar {
  pub fn QProgressBar_0<T: QProgressBar_QProgressBar_0>(value: T) -> QProgressBar {
    let rsthis = value.QProgressBar_0();
    return rsthis;
    // return 1;
  }
}

pub trait QProgressBar_QProgressBar_0 {
  fn QProgressBar_0(self) -> QProgressBar;
}
// QProgressBar(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QProgressBar_QProgressBar_0 for (usize) {
  fn QProgressBar_0(self) -> QProgressBar {
    // unsafe{_ZN12QProgressBarC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QProgressBarC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QProgressBar{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qprogressbar.h:72
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QProgressBar()

/*

*/
pub fn DeleteQProgressBar(this :*mut QProgressBar) {
    // let rv = qtrt::InvokeQtFunc6("_ZN12QProgressBarD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qprogressbar.h:74
// index:0
// Public Visibility=Default Availability=Available
// [4] int minimum() const

/*

*/
impl /*struct*/ QProgressBar {
  pub fn minimum_0<RetType, T: QProgressBar_minimum_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimum_0(self);
    // return 1;
  }
}
pub trait QProgressBar_minimum_0<RetType> {
  fn minimum_0(self , rsthis: & QProgressBar) -> RetType;
}
impl<'a> /*trait*/ QProgressBar_minimum_0<i32> for () {
  fn minimum_0(self , rsthis: & QProgressBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QProgressBar7minimumEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qprogressbar.h:75
// index:0
// Public Visibility=Default Availability=Available
// [4] int maximum() const

/*

*/
impl /*struct*/ QProgressBar {
  pub fn maximum_0<RetType, T: QProgressBar_maximum_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maximum_0(self);
    // return 1;
  }
}
pub trait QProgressBar_maximum_0<RetType> {
  fn maximum_0(self , rsthis: & QProgressBar) -> RetType;
}
impl<'a> /*trait*/ QProgressBar_maximum_0<i32> for () {
  fn maximum_0(self , rsthis: & QProgressBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QProgressBar7maximumEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qprogressbar.h:77
// index:0
// Public Visibility=Default Availability=Available
// [4] int value() const

/*

*/
impl /*struct*/ QProgressBar {
  pub fn value_0<RetType, T: QProgressBar_value_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.value_0(self);
    // return 1;
  }
}
pub trait QProgressBar_value_0<RetType> {
  fn value_0(self , rsthis: & QProgressBar) -> RetType;
}
impl<'a> /*trait*/ QProgressBar_value_0<i32> for () {
  fn value_0(self , rsthis: & QProgressBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QProgressBar5valueEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qprogressbar.h:79
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QString text() const

/*

*/
impl /*struct*/ QProgressBar {
  pub fn text_0<RetType, T: QProgressBar_text_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.text_0(self);
    // return 1;
  }
}
pub trait QProgressBar_text_0<RetType> {
  fn text_0(self , rsthis: & QProgressBar) -> RetType;
}
impl<'a> /*trait*/ QProgressBar_text_0<usize> for () {
  fn text_0(self , rsthis: & QProgressBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QProgressBar4textEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qprogressbar.h:80
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTextVisible(bool)

/*

*/
impl /*struct*/ QProgressBar {
  pub fn setTextVisible_0<RetType, T: QProgressBar_setTextVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTextVisible_0(self);
    // return 1;
  }
}
pub trait QProgressBar_setTextVisible_0<RetType> {
  fn setTextVisible_0(self , rsthis: & QProgressBar) -> RetType;
}
impl<'a> /*trait*/ QProgressBar_setTextVisible_0<(/*void*/)> for (bool) {
  fn setTextVisible_0(self , rsthis: & QProgressBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN12QProgressBar14setTextVisibleEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qprogressbar.h:81
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isTextVisible() const

/*

*/
impl /*struct*/ QProgressBar {
  pub fn isTextVisible_0<RetType, T: QProgressBar_isTextVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isTextVisible_0(self);
    // return 1;
  }
}
pub trait QProgressBar_isTextVisible_0<RetType> {
  fn isTextVisible_0(self , rsthis: & QProgressBar) -> RetType;
}
impl<'a> /*trait*/ QProgressBar_isTextVisible_0<bool> for () {
  fn isTextVisible_0(self , rsthis: & QProgressBar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QProgressBar13isTextVisibleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qprogressbar.h:83
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::Alignment alignment() const

/*

*/
impl /*struct*/ QProgressBar {
  pub fn alignment_0<RetType, T: QProgressBar_alignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.alignment_0(self);
    // return 1;
  }
}
pub trait QProgressBar_alignment_0<RetType> {
  fn alignment_0(self , rsthis: & QProgressBar) -> RetType;
}
impl<'a> /*trait*/ QProgressBar_alignment_0<i32> for () {
  fn alignment_0(self , rsthis: & QProgressBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QProgressBar9alignmentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qprogressbar.h:84
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAlignment(Qt::Alignment)

/*

*/
impl /*struct*/ QProgressBar {
  pub fn setAlignment_0<RetType, T: QProgressBar_setAlignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAlignment_0(self);
    // return 1;
  }
}
pub trait QProgressBar_setAlignment_0<RetType> {
  fn setAlignment_0(self , rsthis: & QProgressBar) -> RetType;
}
impl<'a> /*trait*/ QProgressBar_setAlignment_0<(/*void*/)> for (i32) {
  fn setAlignment_0(self , rsthis: & QProgressBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QProgressBar12setAlignmentE6QFlagsIN2Qt13AlignmentFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qprogressbar.h:86
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Reimplemented from QWidget::sizeHint().
*/
impl /*struct*/ QProgressBar {
  pub fn sizeHint_0<RetType, T: QProgressBar_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QProgressBar_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QProgressBar) -> RetType;
}
impl<'a> /*trait*/ QProgressBar_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QProgressBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QProgressBar8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qprogressbar.h:87
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize minimumSizeHint() const

/*
Reimplemented from QWidget::minimumSizeHint().
*/
impl /*struct*/ QProgressBar {
  pub fn minimumSizeHint_0<RetType, T: QProgressBar_minimumSizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint_0(self);
    // return 1;
  }
}
pub trait QProgressBar_minimumSizeHint_0<RetType> {
  fn minimumSizeHint_0(self , rsthis: & QProgressBar) -> RetType;
}
impl<'a> /*trait*/ QProgressBar_minimumSizeHint_0<usize> for () {
  fn minimumSizeHint_0(self , rsthis: & QProgressBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QProgressBar15minimumSizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qprogressbar.h:89
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::Orientation orientation() const

/*

*/
impl /*struct*/ QProgressBar {
  pub fn orientation_0<RetType, T: QProgressBar_orientation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.orientation_0(self);
    // return 1;
  }
}
pub trait QProgressBar_orientation_0<RetType> {
  fn orientation_0(self , rsthis: & QProgressBar) -> RetType;
}
impl<'a> /*trait*/ QProgressBar_orientation_0<i32> for () {
  fn orientation_0(self , rsthis: & QProgressBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QProgressBar11orientationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qprogressbar.h:91
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setInvertedAppearance(bool)

/*

*/
impl /*struct*/ QProgressBar {
  pub fn setInvertedAppearance_0<RetType, T: QProgressBar_setInvertedAppearance_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setInvertedAppearance_0(self);
    // return 1;
  }
}
pub trait QProgressBar_setInvertedAppearance_0<RetType> {
  fn setInvertedAppearance_0(self , rsthis: & QProgressBar) -> RetType;
}
impl<'a> /*trait*/ QProgressBar_setInvertedAppearance_0<(/*void*/)> for (bool) {
  fn setInvertedAppearance_0(self , rsthis: & QProgressBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN12QProgressBar21setInvertedAppearanceEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qprogressbar.h:92
// index:0
// Public Visibility=Default Availability=Available
// [1] bool invertedAppearance() const

/*

*/
impl /*struct*/ QProgressBar {
  pub fn invertedAppearance_0<RetType, T: QProgressBar_invertedAppearance_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.invertedAppearance_0(self);
    // return 1;
  }
}
pub trait QProgressBar_invertedAppearance_0<RetType> {
  fn invertedAppearance_0(self , rsthis: & QProgressBar) -> RetType;
}
impl<'a> /*trait*/ QProgressBar_invertedAppearance_0<bool> for () {
  fn invertedAppearance_0(self , rsthis: & QProgressBar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QProgressBar18invertedAppearanceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qprogressbar.h:93
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTextDirection(QProgressBar::Direction)

/*

*/
impl /*struct*/ QProgressBar {
  pub fn setTextDirection_0<RetType, T: QProgressBar_setTextDirection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTextDirection_0(self);
    // return 1;
  }
}
pub trait QProgressBar_setTextDirection_0<RetType> {
  fn setTextDirection_0(self , rsthis: & QProgressBar) -> RetType;
}
impl<'a> /*trait*/ QProgressBar_setTextDirection_0<(/*void*/)> for (i32) {
  fn setTextDirection_0(self , rsthis: & QProgressBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QProgressBar16setTextDirectionENS_9DirectionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qprogressbar.h:94
// index:0
// Public Visibility=Default Availability=Available
// [4] QProgressBar::Direction textDirection() const

/*

*/
impl /*struct*/ QProgressBar {
  pub fn textDirection_0<RetType, T: QProgressBar_textDirection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textDirection_0(self);
    // return 1;
  }
}
pub trait QProgressBar_textDirection_0<RetType> {
  fn textDirection_0(self , rsthis: & QProgressBar) -> RetType;
}
impl<'a> /*trait*/ QProgressBar_textDirection_0<i32> for () {
  fn textDirection_0(self , rsthis: & QProgressBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QProgressBar13textDirectionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qprogressbar.h:96
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFormat(const QString &)

/*

*/
impl /*struct*/ QProgressBar {
  pub fn setFormat_0<RetType, T: QProgressBar_setFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFormat_0(self);
    // return 1;
  }
}
pub trait QProgressBar_setFormat_0<RetType> {
  fn setFormat_0(self , rsthis: & QProgressBar) -> RetType;
}
impl<'a> /*trait*/ QProgressBar_setFormat_0<(/*void*/)> for (usize) {
  fn setFormat_0(self , rsthis: & QProgressBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QProgressBar9setFormatERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qprogressbar.h:97
// index:0
// Public Visibility=Default Availability=Available
// [-2] void resetFormat()

/*

*/
impl /*struct*/ QProgressBar {
  pub fn resetFormat_0<RetType, T: QProgressBar_resetFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resetFormat_0(self);
    // return 1;
  }
}
pub trait QProgressBar_resetFormat_0<RetType> {
  fn resetFormat_0(self , rsthis: & QProgressBar) -> RetType;
}
impl<'a> /*trait*/ QProgressBar_resetFormat_0<(/*void*/)> for () {
  fn resetFormat_0(self , rsthis: & QProgressBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN12QProgressBar11resetFormatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qprogressbar.h:98
// index:0
// Public Visibility=Default Availability=Available
// [8] QString format() const

/*

*/
impl /*struct*/ QProgressBar {
  pub fn format_0<RetType, T: QProgressBar_format_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.format_0(self);
    // return 1;
  }
}
pub trait QProgressBar_format_0<RetType> {
  fn format_0(self , rsthis: & QProgressBar) -> RetType;
}
impl<'a> /*trait*/ QProgressBar_format_0<usize> for () {
  fn format_0(self , rsthis: & QProgressBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QProgressBar6formatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qprogressbar.h:101
// index:0
// Public Visibility=Default Availability=Available
// [-2] void reset()

/*
Reset the progress bar. The progress bar "rewinds" and shows no progress.
*/
impl /*struct*/ QProgressBar {
  pub fn reset_0<RetType, T: QProgressBar_reset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.reset_0(self);
    // return 1;
  }
}
pub trait QProgressBar_reset_0<RetType> {
  fn reset_0(self , rsthis: & QProgressBar) -> RetType;
}
impl<'a> /*trait*/ QProgressBar_reset_0<(/*void*/)> for () {
  fn reset_0(self , rsthis: & QProgressBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN12QProgressBar5resetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qprogressbar.h:102
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRange(int, int)

/*
Sets the progress bar's minimum and maximum values to minimum and maximum respectively.

If maximum is smaller than minimum, minimum becomes the only legal value.

If the current value falls outside the new range, the progress bar is reset with reset().

The QProgressBar can be set to undetermined state by using setRange(0, 0).

See also minimum and maximum.
*/
impl /*struct*/ QProgressBar {
  pub fn setRange_0<RetType, T: QProgressBar_setRange_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRange_0(self);
    // return 1;
  }
}
pub trait QProgressBar_setRange_0<RetType> {
  fn setRange_0(self , rsthis: & QProgressBar) -> RetType;
}
impl<'a> /*trait*/ QProgressBar_setRange_0<(/*void*/)> for (i32,i32) {
  fn setRange_0(self , rsthis: & QProgressBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QProgressBar8setRangeEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qprogressbar.h:103
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMinimum(int)

/*

*/
impl /*struct*/ QProgressBar {
  pub fn setMinimum_0<RetType, T: QProgressBar_setMinimum_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMinimum_0(self);
    // return 1;
  }
}
pub trait QProgressBar_setMinimum_0<RetType> {
  fn setMinimum_0(self , rsthis: & QProgressBar) -> RetType;
}
impl<'a> /*trait*/ QProgressBar_setMinimum_0<(/*void*/)> for (i32) {
  fn setMinimum_0(self , rsthis: & QProgressBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QProgressBar10setMinimumEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qprogressbar.h:104
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMaximum(int)

/*

*/
impl /*struct*/ QProgressBar {
  pub fn setMaximum_0<RetType, T: QProgressBar_setMaximum_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMaximum_0(self);
    // return 1;
  }
}
pub trait QProgressBar_setMaximum_0<RetType> {
  fn setMaximum_0(self , rsthis: & QProgressBar) -> RetType;
}
impl<'a> /*trait*/ QProgressBar_setMaximum_0<(/*void*/)> for (i32) {
  fn setMaximum_0(self , rsthis: & QProgressBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QProgressBar10setMaximumEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qprogressbar.h:105
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setValue(int)

/*

*/
impl /*struct*/ QProgressBar {
  pub fn setValue_0<RetType, T: QProgressBar_setValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setValue_0(self);
    // return 1;
  }
}
pub trait QProgressBar_setValue_0<RetType> {
  fn setValue_0(self , rsthis: & QProgressBar) -> RetType;
}
impl<'a> /*trait*/ QProgressBar_setValue_0<(/*void*/)> for (i32) {
  fn setValue_0(self , rsthis: & QProgressBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QProgressBar8setValueEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qprogressbar.h:106
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOrientation(Qt::Orientation)

/*

*/
impl /*struct*/ QProgressBar {
  pub fn setOrientation_0<RetType, T: QProgressBar_setOrientation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOrientation_0(self);
    // return 1;
  }
}
pub trait QProgressBar_setOrientation_0<RetType> {
  fn setOrientation_0(self , rsthis: & QProgressBar) -> RetType;
}
impl<'a> /*trait*/ QProgressBar_setOrientation_0<(/*void*/)> for (i32) {
  fn setOrientation_0(self , rsthis: & QProgressBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QProgressBar14setOrientationEN2Qt11OrientationE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qprogressbar.h:109
// index:0
// Public Visibility=Default Availability=Available
// [-2] void valueChanged(int)

/*
This signal is emitted when the value shown in the progress bar changes. value is the new value shown by the progress bar.

Note: Notifier signal for property value.
*/
impl /*struct*/ QProgressBar {
  pub fn valueChanged_0<RetType, T: QProgressBar_valueChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.valueChanged_0(self);
    // return 1;
  }
}
pub trait QProgressBar_valueChanged_0<RetType> {
  fn valueChanged_0(self , rsthis: & QProgressBar) -> RetType;
}
impl<'a> /*trait*/ QProgressBar_valueChanged_0<(/*void*/)> for (i32) {
  fn valueChanged_0(self , rsthis: & QProgressBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QProgressBar12valueChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qprogressbar.h:112
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QProgressBar {
  pub fn event_0<RetType, T: QProgressBar_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QProgressBar_event_0<RetType> {
  fn event_0(self , rsthis: & QProgressBar) -> RetType;
}
impl<'a> /*trait*/ QProgressBar_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QProgressBar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QProgressBar5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qprogressbar.h:113
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Reimplemented from QWidget::paintEvent().
*/
impl /*struct*/ QProgressBar {
  pub fn paintEvent_0<RetType, T: QProgressBar_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QProgressBar_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QProgressBar) -> RetType;
}
impl<'a> /*trait*/ QProgressBar_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QProgressBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QProgressBar10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qprogressbar.h:114
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void initStyleOption(QStyleOptionProgressBar *) const

/*
Initialize option with the values from this QProgressBar. This method is useful for subclasses when they need a QStyleOptionProgressBar, but don't want to fill in all the information themselves.

See also QStyleOption::initFrom().
*/
impl /*struct*/ QProgressBar {
  pub fn initStyleOption_0<RetType, T: QProgressBar_initStyleOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.initStyleOption_0(self);
    // return 1;
  }
}
pub trait QProgressBar_initStyleOption_0<RetType> {
  fn initStyleOption_0(self , rsthis: & QProgressBar) -> RetType;
}
impl<'a> /*trait*/ QProgressBar_initStyleOption_0<(/*void*/)> for (usize) {
  fn initStyleOption_0(self , rsthis: & QProgressBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK12QProgressBar15initStyleOptionEP23QStyleOptionProgressBar", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
Specifies the reading direction of the text for vertical progress bars.



Note that whether or not the text is drawn is dependent on the style. Currently CleanLooks and Plastique draw the text. Mac, Windows and WindowsVista style do not.

This enum was introduced or modified in  Qt 4.1.

See also textDirection.

*/
pub type QProgressBar__Direction = i32;
// 
pub const QProgressBar__TopToBottom :QProgressBar__Direction = 0;
// 
pub const QProgressBar__BottomToTop :QProgressBar__Direction = 1;
pub fn QProgressBar_DirectionItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QProgressBar", val);
}
pub fn QProgressBar_DirectionItemName_s(val: i32) ->String {
  //var nilthis *QProgressBar
  //return nilthis.DirectionItemName(val);
  return QProgressBar_DirectionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
