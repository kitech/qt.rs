

// mod ::widgets::QSizePolicy
// package qtwidgets
// /usr/include/qt/QtWidgets/qsizepolicy.h
// #include <qsizepolicy.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 8
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
#[derive(Default)] // class sizeof(QSizePolicy)=4
pub struct QSizePolicy {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QSizePolicy_ITF interface {
//    QSizePolicy_PTR() *QSizePolicy
//}
//func (ptr *QSizePolicy) QSizePolicy_PTR() *QSizePolicy { return ptr }

impl /*struct*/ QSizePolicy {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QSizePolicy {
    return QSizePolicy{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QSizePolicy {
//  type Target = QSizePolicyBASE;
//
//  fn deref(&self) -> &QSizePolicyBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QSizePolicyBASE> for QSizePolicy {
//  fn as_ref(& self) -> & QSizePolicyBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qsizepolicy.h:113
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QSizePolicy()

/*
Constructs a QSizePolicy object with Fixed as its horizontal and vertical policies.

The policies can be altered using the setHorizontalPolicy() and setVerticalPolicy() functions. Use the setHeightForWidth() function if the preferred height of the widget is dependent on the width of the widget (for example, a QLabel with line wrapping).

See also setHorizontalStretch() and setVerticalStretch().
*/
// QSizePolicy() ctx.fn_proto_cpp
impl /*struct*/ QSizePolicy {
  pub fn QSizePolicy_0<T: QSizePolicy_QSizePolicy_0>(value: T) -> QSizePolicy {
    let rsthis = value.QSizePolicy_0();
    return rsthis;
    // return 1;
  }
}

pub trait QSizePolicy_QSizePolicy_0 {
  fn QSizePolicy_0(self) -> QSizePolicy;
}
// QSizePolicy() ctx.fn_proto_cpp
impl<'a> /*trait*/ QSizePolicy_QSizePolicy_0 for () {
  fn QSizePolicy_0(self) -> QSizePolicy {
    // unsafe{_ZN11QSizePolicyC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QSizePolicyC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSizePolicy{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsizepolicy.h:116
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QSizePolicy(QSizePolicy::Policy, QSizePolicy::Policy, QSizePolicy::ControlType)

/*
Constructs a QSizePolicy object with Fixed as its horizontal and vertical policies.

The policies can be altered using the setHorizontalPolicy() and setVerticalPolicy() functions. Use the setHeightForWidth() function if the preferred height of the widget is dependent on the width of the widget (for example, a QLabel with line wrapping).

See also setHorizontalStretch() and setVerticalStretch().
*/
// QSizePolicy(QSizePolicy::Policy, QSizePolicy::Policy, QSizePolicy::ControlType) ctx.fn_proto_cpp
impl /*struct*/ QSizePolicy {
  pub fn QSizePolicy_1<T: QSizePolicy_QSizePolicy_1>(value: T) -> QSizePolicy {
    let rsthis = value.QSizePolicy_1();
    return rsthis;
    // return 1;
  }
}

pub trait QSizePolicy_QSizePolicy_1 {
  fn QSizePolicy_1(self) -> QSizePolicy;
}
// QSizePolicy(QSizePolicy::Policy, QSizePolicy::Policy, QSizePolicy::ControlType) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSizePolicy_QSizePolicy_1 for (i32,i32,i32) {
  fn QSizePolicy_1(self) -> QSizePolicy {
    // unsafe{_ZN11QSizePolicyC2ENS_6PolicyES0_NS_11ControlTypeE()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QSizePolicyC2ENS_6PolicyES0_NS_11ControlTypeE", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSizePolicy{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsizepolicy.h:128
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QSizePolicy::Policy horizontalPolicy() const

/*
Returns the horizontal component of the size policy.

See also setHorizontalPolicy(), verticalPolicy(), and horizontalStretch().
*/
impl /*struct*/ QSizePolicy {
  pub fn horizontalPolicy_0<RetType, T: QSizePolicy_horizontalPolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.horizontalPolicy_0(self);
    // return 1;
  }
}
pub trait QSizePolicy_horizontalPolicy_0<RetType> {
  fn horizontalPolicy_0(self , rsthis: & QSizePolicy) -> RetType;
}
impl<'a> /*trait*/ QSizePolicy_horizontalPolicy_0<i32> for () {
  fn horizontalPolicy_0(self , rsthis: & QSizePolicy) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QSizePolicy16horizontalPolicyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsizepolicy.h:129
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QSizePolicy::Policy verticalPolicy() const

/*
Returns the vertical component of the size policy.

See also setVerticalPolicy(), horizontalPolicy(), and verticalStretch().
*/
impl /*struct*/ QSizePolicy {
  pub fn verticalPolicy_0<RetType, T: QSizePolicy_verticalPolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.verticalPolicy_0(self);
    // return 1;
  }
}
pub trait QSizePolicy_verticalPolicy_0<RetType> {
  fn verticalPolicy_0(self , rsthis: & QSizePolicy) -> RetType;
}
impl<'a> /*trait*/ QSizePolicy_verticalPolicy_0<i32> for () {
  fn verticalPolicy_0(self , rsthis: & QSizePolicy) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QSizePolicy14verticalPolicyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsizepolicy.h:130
// index:0
// Public Visibility=Default Availability=Available
// [4] QSizePolicy::ControlType controlType() const

/*
Returns the control type associated with the widget for which this size policy applies.

This function was introduced in  Qt 4.3.

See also setControlType().
*/
impl /*struct*/ QSizePolicy {
  pub fn controlType_0<RetType, T: QSizePolicy_controlType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.controlType_0(self);
    // return 1;
  }
}
pub trait QSizePolicy_controlType_0<RetType> {
  fn controlType_0(self , rsthis: & QSizePolicy) -> RetType;
}
impl<'a> /*trait*/ QSizePolicy_controlType_0<i32> for () {
  fn controlType_0(self , rsthis: & QSizePolicy) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QSizePolicy11controlTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsizepolicy.h:132
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setHorizontalPolicy(QSizePolicy::Policy)

/*
Sets the horizontal component to the given policy.

See also horizontalPolicy(), setVerticalPolicy(), and setHorizontalStretch().
*/
impl /*struct*/ QSizePolicy {
  pub fn setHorizontalPolicy_0<RetType, T: QSizePolicy_setHorizontalPolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHorizontalPolicy_0(self);
    // return 1;
  }
}
pub trait QSizePolicy_setHorizontalPolicy_0<RetType> {
  fn setHorizontalPolicy_0(self , rsthis: & QSizePolicy) -> RetType;
}
impl<'a> /*trait*/ QSizePolicy_setHorizontalPolicy_0<(/*void*/)> for (i32) {
  fn setHorizontalPolicy_0(self , rsthis: & QSizePolicy) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QSizePolicy19setHorizontalPolicyENS_6PolicyE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsizepolicy.h:133
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setVerticalPolicy(QSizePolicy::Policy)

/*
Sets the vertical component to the given policy.

See also verticalPolicy(), setHorizontalPolicy(), and setVerticalStretch().
*/
impl /*struct*/ QSizePolicy {
  pub fn setVerticalPolicy_0<RetType, T: QSizePolicy_setVerticalPolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVerticalPolicy_0(self);
    // return 1;
  }
}
pub trait QSizePolicy_setVerticalPolicy_0<RetType> {
  fn setVerticalPolicy_0(self , rsthis: & QSizePolicy) -> RetType;
}
impl<'a> /*trait*/ QSizePolicy_setVerticalPolicy_0<(/*void*/)> for (i32) {
  fn setVerticalPolicy_0(self , rsthis: & QSizePolicy) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QSizePolicy17setVerticalPolicyENS_6PolicyE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsizepolicy.h:134
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setControlType(QSizePolicy::ControlType)

/*
Sets the control type associated with the widget for which this size policy applies to type.

The control type specifies the type of the widget for which this size policy applies. It is used by some styles, notably QMacStyle, to insert proper spacing between widgets. For example, the macOS Aqua guidelines specify that push buttons should be separated by 12 pixels, whereas vertically stacked radio buttons only require 6 pixels.

This function was introduced in  Qt 4.3.

See also controlType() and QStyle::layoutSpacing().
*/
impl /*struct*/ QSizePolicy {
  pub fn setControlType_0<RetType, T: QSizePolicy_setControlType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setControlType_0(self);
    // return 1;
  }
}
pub trait QSizePolicy_setControlType_0<RetType> {
  fn setControlType_0(self , rsthis: & QSizePolicy) -> RetType;
}
impl<'a> /*trait*/ QSizePolicy_setControlType_0<(/*void*/)> for (i32) {
  fn setControlType_0(self , rsthis: & QSizePolicy) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QSizePolicy14setControlTypeENS_11ControlTypeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsizepolicy.h:136
// index:0
// Public inline Visibility=Default Availability=Available
// [4] Qt::Orientations expandingDirections() const

/*
Returns whether a widget can make use of more space than the QWidget::sizeHint() function indicates.

A value of Qt::Horizontal or Qt::Vertical means that the widget can grow horizontally or vertically (i.e., the horizontal or vertical policy is Expanding or MinimumExpanding), whereas Qt::Horizontal | Qt::Vertical means that it can grow in both dimensions.

See also horizontalPolicy() and verticalPolicy().
*/
impl /*struct*/ QSizePolicy {
  pub fn expandingDirections_0<RetType, T: QSizePolicy_expandingDirections_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.expandingDirections_0(self);
    // return 1;
  }
}
pub trait QSizePolicy_expandingDirections_0<RetType> {
  fn expandingDirections_0(self , rsthis: & QSizePolicy) -> RetType;
}
impl<'a> /*trait*/ QSizePolicy_expandingDirections_0<i32> for () {
  fn expandingDirections_0(self , rsthis: & QSizePolicy) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QSizePolicy19expandingDirectionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsizepolicy.h:141
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setHeightForWidth(bool)

/*
Sets the flag determining whether the widget's preferred height depends on its width, to dependent.

See also hasHeightForWidth() and setWidthForHeight().
*/
impl /*struct*/ QSizePolicy {
  pub fn setHeightForWidth_0<RetType, T: QSizePolicy_setHeightForWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHeightForWidth_0(self);
    // return 1;
  }
}
pub trait QSizePolicy_setHeightForWidth_0<RetType> {
  fn setHeightForWidth_0(self , rsthis: & QSizePolicy) -> RetType;
}
impl<'a> /*trait*/ QSizePolicy_setHeightForWidth_0<(/*void*/)> for (bool) {
  fn setHeightForWidth_0(self , rsthis: & QSizePolicy) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QSizePolicy17setHeightForWidthEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsizepolicy.h:142
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool hasHeightForWidth() const

/*
Returns true if the widget's preferred height depends on its width; otherwise returns false.

See also setHeightForWidth().
*/
impl /*struct*/ QSizePolicy {
  pub fn hasHeightForWidth_0<RetType, T: QSizePolicy_hasHeightForWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasHeightForWidth_0(self);
    // return 1;
  }
}
pub trait QSizePolicy_hasHeightForWidth_0<RetType> {
  fn hasHeightForWidth_0(self , rsthis: & QSizePolicy) -> RetType;
}
impl<'a> /*trait*/ QSizePolicy_hasHeightForWidth_0<bool> for () {
  fn hasHeightForWidth_0(self , rsthis: & QSizePolicy) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QSizePolicy17hasHeightForWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsizepolicy.h:143
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setWidthForHeight(bool)

/*
Sets the flag determining whether the widget's width depends on its height, to dependent.

This is only supported for QGraphicsLayout's subclasses. It is not possible to have a layout with both height-for-width and width-for-height constraints at the same time.

See also hasWidthForHeight() and setHeightForWidth().
*/
impl /*struct*/ QSizePolicy {
  pub fn setWidthForHeight_0<RetType, T: QSizePolicy_setWidthForHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWidthForHeight_0(self);
    // return 1;
  }
}
pub trait QSizePolicy_setWidthForHeight_0<RetType> {
  fn setWidthForHeight_0(self , rsthis: & QSizePolicy) -> RetType;
}
impl<'a> /*trait*/ QSizePolicy_setWidthForHeight_0<(/*void*/)> for (bool) {
  fn setWidthForHeight_0(self , rsthis: & QSizePolicy) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QSizePolicy17setWidthForHeightEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsizepolicy.h:144
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool hasWidthForHeight() const

/*
Returns true if the widget's width depends on its height; otherwise returns false.

See also setWidthForHeight().
*/
impl /*struct*/ QSizePolicy {
  pub fn hasWidthForHeight_0<RetType, T: QSizePolicy_hasWidthForHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasWidthForHeight_0(self);
    // return 1;
  }
}
pub trait QSizePolicy_hasWidthForHeight_0<RetType> {
  fn hasWidthForHeight_0(self , rsthis: & QSizePolicy) -> RetType;
}
impl<'a> /*trait*/ QSizePolicy_hasWidthForHeight_0<bool> for () {
  fn hasWidthForHeight_0(self , rsthis: & QSizePolicy) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QSizePolicy17hasWidthForHeightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsizepolicy.h:146
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator==(const QSizePolicy &) const

/*

*/
impl /*struct*/ QSizePolicy {
  pub fn operator_equal_equal_0<RetType, T: QSizePolicy_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QSizePolicy_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QSizePolicy) -> RetType;
}
impl<'a> /*trait*/ QSizePolicy_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QSizePolicy) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QSizePolicyeqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsizepolicy.h:147
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QSizePolicy &) const

/*

*/
impl /*struct*/ QSizePolicy {
  pub fn operator_not_equal_0<RetType, T: QSizePolicy_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QSizePolicy_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QSizePolicy) -> RetType;
}
impl<'a> /*trait*/ QSizePolicy_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QSizePolicy) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QSizePolicyneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsizepolicy.h:153
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int horizontalStretch() const

/*
Returns the horizontal stretch factor of the size policy.

See also setHorizontalStretch(), verticalStretch(), and horizontalPolicy().
*/
impl /*struct*/ QSizePolicy {
  pub fn horizontalStretch_0<RetType, T: QSizePolicy_horizontalStretch_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.horizontalStretch_0(self);
    // return 1;
  }
}
pub trait QSizePolicy_horizontalStretch_0<RetType> {
  fn horizontalStretch_0(self , rsthis: & QSizePolicy) -> RetType;
}
impl<'a> /*trait*/ QSizePolicy_horizontalStretch_0<i32> for () {
  fn horizontalStretch_0(self , rsthis: & QSizePolicy) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QSizePolicy17horizontalStretchEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsizepolicy.h:154
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int verticalStretch() const

/*
Returns the vertical stretch factor of the size policy.

See also setVerticalStretch(), horizontalStretch(), and verticalPolicy().
*/
impl /*struct*/ QSizePolicy {
  pub fn verticalStretch_0<RetType, T: QSizePolicy_verticalStretch_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.verticalStretch_0(self);
    // return 1;
  }
}
pub trait QSizePolicy_verticalStretch_0<RetType> {
  fn verticalStretch_0(self , rsthis: & QSizePolicy) -> RetType;
}
impl<'a> /*trait*/ QSizePolicy_verticalStretch_0<i32> for () {
  fn verticalStretch_0(self , rsthis: & QSizePolicy) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QSizePolicy15verticalStretchEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsizepolicy.h:155
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setHorizontalStretch(int)

/*
Sets the horizontal stretch factor of the size policy to the given stretchFactor. stretchFactor must be in the range [0,255].

When two widgets are adjacent to each other in a horizontal layout, setting the horizontal stretch factor of the widget on the left to 2 and the factor of widget on the right to 1 will ensure that the widget on the left will always be twice the size of the one on the right.

See also horizontalStretch(), setVerticalStretch(), and setHorizontalPolicy().
*/
impl /*struct*/ QSizePolicy {
  pub fn setHorizontalStretch_0<RetType, T: QSizePolicy_setHorizontalStretch_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHorizontalStretch_0(self);
    // return 1;
  }
}
pub trait QSizePolicy_setHorizontalStretch_0<RetType> {
  fn setHorizontalStretch_0(self , rsthis: & QSizePolicy) -> RetType;
}
impl<'a> /*trait*/ QSizePolicy_setHorizontalStretch_0<(/*void*/)> for (i32) {
  fn setHorizontalStretch_0(self , rsthis: & QSizePolicy) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QSizePolicy20setHorizontalStretchEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsizepolicy.h:156
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setVerticalStretch(int)

/*
Sets the vertical stretch factor of the size policy to the given stretchFactor. stretchFactor must be in the range [0,255].

When two widgets are adjacent to each other in a vertical layout, setting the vertical stretch factor of the widget on the top to 2 and the factor of widget on the bottom to 1 will ensure that the widget on the top will always be twice the size of the one on the bottom.

See also verticalStretch(), setHorizontalStretch(), and setVerticalPolicy().
*/
impl /*struct*/ QSizePolicy {
  pub fn setVerticalStretch_0<RetType, T: QSizePolicy_setVerticalStretch_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVerticalStretch_0(self);
    // return 1;
  }
}
pub trait QSizePolicy_setVerticalStretch_0<RetType> {
  fn setVerticalStretch_0(self , rsthis: & QSizePolicy) -> RetType;
}
impl<'a> /*trait*/ QSizePolicy_setVerticalStretch_0<(/*void*/)> for (i32) {
  fn setVerticalStretch_0(self , rsthis: & QSizePolicy) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QSizePolicy18setVerticalStretchEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsizepolicy.h:158
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool retainSizeWhenHidden() const

/*
Returns whether the layout should retain the widget's size when it is hidden. This is false by default.

This function was introduced in  Qt 5.2.

See also setRetainSizeWhenHidden().
*/
impl /*struct*/ QSizePolicy {
  pub fn retainSizeWhenHidden_0<RetType, T: QSizePolicy_retainSizeWhenHidden_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.retainSizeWhenHidden_0(self);
    // return 1;
  }
}
pub trait QSizePolicy_retainSizeWhenHidden_0<RetType> {
  fn retainSizeWhenHidden_0(self , rsthis: & QSizePolicy) -> RetType;
}
impl<'a> /*trait*/ QSizePolicy_retainSizeWhenHidden_0<bool> for () {
  fn retainSizeWhenHidden_0(self , rsthis: & QSizePolicy) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QSizePolicy20retainSizeWhenHiddenEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsizepolicy.h:159
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setRetainSizeWhenHidden(bool)

/*
Sets whether a layout should retain the widget's size when it is hidden. If retainSize is true, the layout will not be changed by hiding the widget.

This function was introduced in  Qt 5.2.

See also retainSizeWhenHidden().
*/
impl /*struct*/ QSizePolicy {
  pub fn setRetainSizeWhenHidden_0<RetType, T: QSizePolicy_setRetainSizeWhenHidden_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRetainSizeWhenHidden_0(self);
    // return 1;
  }
}
pub trait QSizePolicy_setRetainSizeWhenHidden_0<RetType> {
  fn setRetainSizeWhenHidden_0(self , rsthis: & QSizePolicy) -> RetType;
}
impl<'a> /*trait*/ QSizePolicy_setRetainSizeWhenHidden_0<(/*void*/)> for (bool) {
  fn setRetainSizeWhenHidden_0(self , rsthis: & QSizePolicy) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QSizePolicy23setRetainSizeWhenHiddenEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsizepolicy.h:161
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void transpose()

/*
Swaps the horizontal and vertical policies and stretches.

See also transposed().
*/
impl /*struct*/ QSizePolicy {
  pub fn transpose_0<RetType, T: QSizePolicy_transpose_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.transpose_0(self);
    // return 1;
  }
}
pub trait QSizePolicy_transpose_0<RetType> {
  fn transpose_0(self , rsthis: & QSizePolicy) -> RetType;
}
impl<'a> /*trait*/ QSizePolicy_transpose_0<(/*void*/)> for () {
  fn transpose_0(self , rsthis: & QSizePolicy) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QSizePolicy9transposeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsizepolicy.h:166
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QSizePolicy transposed() const

/*
Returns a size policy object with the horizontal and vertical policies and stretches swapped.

This function was introduced in  Qt 5.9.

See also transpose().
*/
impl /*struct*/ QSizePolicy {
  pub fn transposed_0<RetType, T: QSizePolicy_transposed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.transposed_0(self);
    // return 1;
  }
}
pub trait QSizePolicy_transposed_0<RetType> {
  fn transposed_0(self , rsthis: & QSizePolicy) -> RetType;
}
impl<'a> /*trait*/ QSizePolicy_transposed_0<usize> for () {
  fn transposed_0(self , rsthis: & QSizePolicy) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QSizePolicy10transposedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQSizePolicy(this :*mut QSizePolicy) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN11QSizePolicyD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*
These flags are combined together to form the various Policy values:



See also Policy.

*/
pub type QSizePolicy__PolicyFlag = i32;
// The widget can grow beyond its size hint if necessary.
pub const QSizePolicy__GrowFlag :QSizePolicy__PolicyFlag = 1;
// The widget should get as much space as possible.
pub const QSizePolicy__ExpandFlag :QSizePolicy__PolicyFlag = 2;
// The widget can shrink below its size hint if necessary.
pub const QSizePolicy__ShrinkFlag :QSizePolicy__PolicyFlag = 4;
// The widget's size hint is ignored. The widget will get as much space as possible.
pub const QSizePolicy__IgnoreFlag :QSizePolicy__PolicyFlag = 8;
pub fn QSizePolicy_PolicyFlagItemName(val: i32) ->String {
  match val {
     QSizePolicy__GrowFlag => // 1
     {return String::from("GrowFlag");}
     QSizePolicy__ExpandFlag => // 2
     {return String::from("ExpandFlag");}
     QSizePolicy__ShrinkFlag => // 4
     {return String::from("ShrinkFlag");}
     QSizePolicy__IgnoreFlag => // 8
     {return String::from("IgnoreFlag");}
  _ => {return format!("{}", val);}
}
}
pub fn QSizePolicy_PolicyFlagItemName_s(val: i32) ->String {
  //var nilthis *QSizePolicy
  //return nilthis.PolicyFlagItemName(val);
  return QSizePolicy_PolicyFlagItemName(val);
}


/*
This enum describes the various per-dimension sizing types used when constructing a QSizePolicy.

QSizePolicy::MinimumGrowFlagThe sizeHint() is minimal, and sufficient. The widget can be expanded, but there is no advantage to it being larger (e.g. the horizontal direction of a push button). It cannot be smaller than the size provided by sizeHint().
QSizePolicy::MaximumShrinkFlagThe sizeHint() is a maximum. The widget can be shrunk any amount without detriment if other widgets need the space (e.g. a separator line). It cannot be larger than the size provided by sizeHint().
QSizePolicy::PreferredGrowFlag | ShrinkFlagThe sizeHint() is best, but the widget can be shrunk and still be useful. The widget can be expanded, but there is no advantage to it being larger than sizeHint() (the default QWidget policy).
QSizePolicy::ExpandingGrowFlag | ShrinkFlag | ExpandFlagThe sizeHint() is a sensible size, but the widget can be shrunk and still be useful. The widget can make use of extra space, so it should get as much space as possible (e.g. the horizontal direction of a horizontal slider).
QSizePolicy::MinimumExpandingGrowFlag | ExpandFlagThe sizeHint() is minimal, and sufficient. The widget can make use of extra space, so it should get as much space as possible (e.g. the horizontal direction of a horizontal slider).
QSizePolicy::IgnoredShrinkFlag | GrowFlag | IgnoreFlagThe sizeHint() is ignored. The widget will get as much space as possible.


See also PolicyFlag, setHorizontalPolicy(), and setVerticalPolicy().

*/
pub type QSizePolicy__Policy = i32;
// The QWidget::sizeHint() is the only acceptable alternative, so the widget can never grow or shrink (e.g. the vertical direction of a push button).
pub const QSizePolicy__Fixed :QSizePolicy__Policy = 0;
// 
pub const QSizePolicy__Minimum :QSizePolicy__Policy = 1;
// 
pub const QSizePolicy__Maximum :QSizePolicy__Policy = 4;
// 
pub const QSizePolicy__Preferred :QSizePolicy__Policy = 5;
// 
pub const QSizePolicy__MinimumExpanding :QSizePolicy__Policy = 3;
// 
pub const QSizePolicy__Expanding :QSizePolicy__Policy = 7;
// 
pub const QSizePolicy__Ignored :QSizePolicy__Policy = 13;
pub fn QSizePolicy_PolicyItemName(val: i32) ->String {
  match val {
     QSizePolicy__Fixed => // 0
     {return String::from("Fixed");}
     QSizePolicy__Minimum => // 1
     {return String::from("Minimum");}
     QSizePolicy__Maximum => // 4
     {return String::from("Maximum");}
     QSizePolicy__Preferred => // 5
     {return String::from("Preferred");}
     QSizePolicy__MinimumExpanding => // 3
     {return String::from("MinimumExpanding");}
     QSizePolicy__Expanding => // 7
     {return String::from("Expanding");}
     QSizePolicy__Ignored => // 13
     {return String::from("Ignored");}
  _ => {return format!("{}", val);}
}
}
pub fn QSizePolicy_PolicyItemName_s(val: i32) ->String {
  //var nilthis *QSizePolicy
  //return nilthis.PolicyItemName(val);
  return QSizePolicy_PolicyItemName(val);
}


/*


*/
pub type QSizePolicy__ControlType = i32;
// 
pub const QSizePolicy__DefaultType :QSizePolicy__ControlType = 1;
// 
pub const QSizePolicy__ButtonBox :QSizePolicy__ControlType = 2;
// 
pub const QSizePolicy__CheckBox :QSizePolicy__ControlType = 4;
// 
pub const QSizePolicy__ComboBox :QSizePolicy__ControlType = 8;
// 
pub const QSizePolicy__Frame :QSizePolicy__ControlType = 16;
// 
pub const QSizePolicy__GroupBox :QSizePolicy__ControlType = 32;
// 
pub const QSizePolicy__Label :QSizePolicy__ControlType = 64;
// 
pub const QSizePolicy__Line :QSizePolicy__ControlType = 128;
// 
pub const QSizePolicy__LineEdit :QSizePolicy__ControlType = 256;
// 
pub const QSizePolicy__PushButton :QSizePolicy__ControlType = 512;
// 
pub const QSizePolicy__RadioButton :QSizePolicy__ControlType = 1024;
// 
pub const QSizePolicy__Slider :QSizePolicy__ControlType = 2048;
// 
pub const QSizePolicy__SpinBox :QSizePolicy__ControlType = 4096;
// 
pub const QSizePolicy__TabWidget :QSizePolicy__ControlType = 8192;
// 
pub const QSizePolicy__ToolButton :QSizePolicy__ControlType = 16384;
pub fn QSizePolicy_ControlTypeItemName(val: i32) ->String {
  match val {
     QSizePolicy__DefaultType => // 1
     {return String::from("DefaultType");}
     QSizePolicy__ButtonBox => // 2
     {return String::from("ButtonBox");}
     QSizePolicy__CheckBox => // 4
     {return String::from("CheckBox");}
     QSizePolicy__ComboBox => // 8
     {return String::from("ComboBox");}
     QSizePolicy__Frame => // 16
     {return String::from("Frame");}
     QSizePolicy__GroupBox => // 32
     {return String::from("GroupBox");}
     QSizePolicy__Label => // 64
     {return String::from("Label");}
     QSizePolicy__Line => // 128
     {return String::from("Line");}
     QSizePolicy__LineEdit => // 256
     {return String::from("LineEdit");}
     QSizePolicy__PushButton => // 512
     {return String::from("PushButton");}
     QSizePolicy__RadioButton => // 1024
     {return String::from("RadioButton");}
     QSizePolicy__Slider => // 2048
     {return String::from("Slider");}
     QSizePolicy__SpinBox => // 4096
     {return String::from("SpinBox");}
     QSizePolicy__TabWidget => // 8192
     {return String::from("TabWidget");}
     QSizePolicy__ToolButton => // 16384
     {return String::from("ToolButton");}
  _ => {return format!("{}", val);}
}
}
pub fn QSizePolicy_ControlTypeItemName_s(val: i32) ->String {
  //var nilthis *QSizePolicy
  //return nilthis.ControlTypeItemName(val);
  return QSizePolicy_ControlTypeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
