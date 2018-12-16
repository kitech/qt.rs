

// mod ::widgets::QFormLayout
// package qtwidgets
// /usr/include/qt/QtWidgets/qformlayout.h
// #include <qformlayout.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 20
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
#[derive(Default)] // class sizeof(QFormLayout)=32
pub struct QFormLayout {
  qbase: QLayout,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QFormLayout_ITF interface {
//    QLayout_ITF
//    QFormLayout_PTR() *QFormLayout
//}
//func (ptr *QFormLayout) QFormLayout_PTR() *QFormLayout { return ptr }

impl /*struct*/ QFormLayout {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QFormLayout {
    return QFormLayout{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QFormLayout {
//  type Target = QFormLayoutBASE;
//
//  fn deref(&self) -> &QFormLayoutBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QFormLayoutBASE> for QFormLayout {
//  fn as_ref(& self) -> & QFormLayoutBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qformlayout.h:55
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QFormLayout {
  pub fn metaObject_0<RetType, T: QFormLayout_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QFormLayout_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QFormLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFormLayout10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:91
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QFormLayout(QWidget *)

/*
Constructs a new form layout with the given parent widget.

See also QWidget::setLayout().
*/
// QFormLayout(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QFormLayout {
  pub fn QFormLayout_0<T: QFormLayout_QFormLayout_0>(value: T) -> QFormLayout {
    let rsthis = value.QFormLayout_0();
    return rsthis;
    // return 1;
  }
}

pub trait QFormLayout_QFormLayout_0 {
  fn QFormLayout_0(self) -> QFormLayout;
}
// QFormLayout(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QFormLayout_QFormLayout_0 for (usize) {
  fn QFormLayout_0(self) -> QFormLayout {
    // unsafe{_ZN11QFormLayoutC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QFormLayoutC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFormLayout{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:92
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QFormLayout()

/*

*/
pub fn DeleteQFormLayout(this :*mut QFormLayout) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QFormLayoutD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 32)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qformlayout.h:94
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFieldGrowthPolicy(QFormLayout::FieldGrowthPolicy)

/*

*/
impl /*struct*/ QFormLayout {
  pub fn setFieldGrowthPolicy_0<RetType, T: QFormLayout_setFieldGrowthPolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFieldGrowthPolicy_0(self);
    // return 1;
  }
}
pub trait QFormLayout_setFieldGrowthPolicy_0<RetType> {
  fn setFieldGrowthPolicy_0(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_setFieldGrowthPolicy_0<(/*void*/)> for (i32) {
  fn setFieldGrowthPolicy_0(self , rsthis: & QFormLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QFormLayout20setFieldGrowthPolicyENS_17FieldGrowthPolicyE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:95
// index:0
// Public Visibility=Default Availability=Available
// [4] QFormLayout::FieldGrowthPolicy fieldGrowthPolicy() const

/*

*/
impl /*struct*/ QFormLayout {
  pub fn fieldGrowthPolicy_0<RetType, T: QFormLayout_fieldGrowthPolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fieldGrowthPolicy_0(self);
    // return 1;
  }
}
pub trait QFormLayout_fieldGrowthPolicy_0<RetType> {
  fn fieldGrowthPolicy_0(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_fieldGrowthPolicy_0<i32> for () {
  fn fieldGrowthPolicy_0(self , rsthis: & QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFormLayout17fieldGrowthPolicyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:96
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRowWrapPolicy(QFormLayout::RowWrapPolicy)

/*

*/
impl /*struct*/ QFormLayout {
  pub fn setRowWrapPolicy_0<RetType, T: QFormLayout_setRowWrapPolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRowWrapPolicy_0(self);
    // return 1;
  }
}
pub trait QFormLayout_setRowWrapPolicy_0<RetType> {
  fn setRowWrapPolicy_0(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_setRowWrapPolicy_0<(/*void*/)> for (i32) {
  fn setRowWrapPolicy_0(self , rsthis: & QFormLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QFormLayout16setRowWrapPolicyENS_13RowWrapPolicyE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:97
// index:0
// Public Visibility=Default Availability=Available
// [4] QFormLayout::RowWrapPolicy rowWrapPolicy() const

/*

*/
impl /*struct*/ QFormLayout {
  pub fn rowWrapPolicy_0<RetType, T: QFormLayout_rowWrapPolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowWrapPolicy_0(self);
    // return 1;
  }
}
pub trait QFormLayout_rowWrapPolicy_0<RetType> {
  fn rowWrapPolicy_0(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_rowWrapPolicy_0<i32> for () {
  fn rowWrapPolicy_0(self , rsthis: & QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFormLayout13rowWrapPolicyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:98
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLabelAlignment(Qt::Alignment)

/*

*/
impl /*struct*/ QFormLayout {
  pub fn setLabelAlignment_0<RetType, T: QFormLayout_setLabelAlignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLabelAlignment_0(self);
    // return 1;
  }
}
pub trait QFormLayout_setLabelAlignment_0<RetType> {
  fn setLabelAlignment_0(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_setLabelAlignment_0<(/*void*/)> for (i32) {
  fn setLabelAlignment_0(self , rsthis: & QFormLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QFormLayout17setLabelAlignmentE6QFlagsIN2Qt13AlignmentFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:99
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::Alignment labelAlignment() const

/*

*/
impl /*struct*/ QFormLayout {
  pub fn labelAlignment_0<RetType, T: QFormLayout_labelAlignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.labelAlignment_0(self);
    // return 1;
  }
}
pub trait QFormLayout_labelAlignment_0<RetType> {
  fn labelAlignment_0(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_labelAlignment_0<i32> for () {
  fn labelAlignment_0(self , rsthis: & QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFormLayout14labelAlignmentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:100
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFormAlignment(Qt::Alignment)

/*

*/
impl /*struct*/ QFormLayout {
  pub fn setFormAlignment_0<RetType, T: QFormLayout_setFormAlignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFormAlignment_0(self);
    // return 1;
  }
}
pub trait QFormLayout_setFormAlignment_0<RetType> {
  fn setFormAlignment_0(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_setFormAlignment_0<(/*void*/)> for (i32) {
  fn setFormAlignment_0(self , rsthis: & QFormLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QFormLayout16setFormAlignmentE6QFlagsIN2Qt13AlignmentFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:101
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::Alignment formAlignment() const

/*

*/
impl /*struct*/ QFormLayout {
  pub fn formAlignment_0<RetType, T: QFormLayout_formAlignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.formAlignment_0(self);
    // return 1;
  }
}
pub trait QFormLayout_formAlignment_0<RetType> {
  fn formAlignment_0(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_formAlignment_0<i32> for () {
  fn formAlignment_0(self , rsthis: & QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFormLayout13formAlignmentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:103
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setHorizontalSpacing(int)

/*

*/
impl /*struct*/ QFormLayout {
  pub fn setHorizontalSpacing_0<RetType, T: QFormLayout_setHorizontalSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHorizontalSpacing_0(self);
    // return 1;
  }
}
pub trait QFormLayout_setHorizontalSpacing_0<RetType> {
  fn setHorizontalSpacing_0(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_setHorizontalSpacing_0<(/*void*/)> for (i32) {
  fn setHorizontalSpacing_0(self , rsthis: & QFormLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QFormLayout20setHorizontalSpacingEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:104
// index:0
// Public Visibility=Default Availability=Available
// [4] int horizontalSpacing() const

/*

*/
impl /*struct*/ QFormLayout {
  pub fn horizontalSpacing_0<RetType, T: QFormLayout_horizontalSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.horizontalSpacing_0(self);
    // return 1;
  }
}
pub trait QFormLayout_horizontalSpacing_0<RetType> {
  fn horizontalSpacing_0(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_horizontalSpacing_0<i32> for () {
  fn horizontalSpacing_0(self , rsthis: & QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFormLayout17horizontalSpacingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:105
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setVerticalSpacing(int)

/*

*/
impl /*struct*/ QFormLayout {
  pub fn setVerticalSpacing_0<RetType, T: QFormLayout_setVerticalSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVerticalSpacing_0(self);
    // return 1;
  }
}
pub trait QFormLayout_setVerticalSpacing_0<RetType> {
  fn setVerticalSpacing_0(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_setVerticalSpacing_0<(/*void*/)> for (i32) {
  fn setVerticalSpacing_0(self , rsthis: & QFormLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QFormLayout18setVerticalSpacingEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:106
// index:0
// Public Visibility=Default Availability=Available
// [4] int verticalSpacing() const

/*

*/
impl /*struct*/ QFormLayout {
  pub fn verticalSpacing_0<RetType, T: QFormLayout_verticalSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.verticalSpacing_0(self);
    // return 1;
  }
}
pub trait QFormLayout_verticalSpacing_0<RetType> {
  fn verticalSpacing_0(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_verticalSpacing_0<i32> for () {
  fn verticalSpacing_0(self , rsthis: & QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFormLayout15verticalSpacingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:108
// index:0
// Public Visibility=Default Availability=Available
// [4] int spacing() const

/*
If the vertical spacing is equal to the horizontal spacing, this function returns that value; otherwise it returns -1.

See also setSpacing(), verticalSpacing(), and horizontalSpacing().
*/
impl /*struct*/ QFormLayout {
  pub fn spacing_0<RetType, T: QFormLayout_spacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.spacing_0(self);
    // return 1;
  }
}
pub trait QFormLayout_spacing_0<RetType> {
  fn spacing_0(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_spacing_0<i32> for () {
  fn spacing_0(self , rsthis: & QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFormLayout7spacingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:109
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSpacing(int)

/*
This function sets both the vertical and horizontal spacing to spacing.

See also spacing(), setVerticalSpacing(), and setHorizontalSpacing().
*/
impl /*struct*/ QFormLayout {
  pub fn setSpacing_0<RetType, T: QFormLayout_setSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSpacing_0(self);
    // return 1;
  }
}
pub trait QFormLayout_setSpacing_0<RetType> {
  fn setSpacing_0(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_setSpacing_0<(/*void*/)> for (i32) {
  fn setSpacing_0(self , rsthis: & QFormLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QFormLayout10setSpacingEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:111
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addRow(QWidget *, QWidget *)

/*
Adds a new row to the bottom of this form layout, with the given label and field.

See also insertRow().
*/
impl /*struct*/ QFormLayout {
  pub fn addRow_0<RetType, T: QFormLayout_addRow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addRow_0(self);
    // return 1;
  }
}
pub trait QFormLayout_addRow_0<RetType> {
  fn addRow_0(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_addRow_0<(/*void*/)> for (usize,usize) {
  fn addRow_0(self , rsthis: & QFormLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFormLayout6addRowEP7QWidgetS1_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:112
// index:1
// Public Visibility=Default Availability=Available
// [-2] void addRow(QWidget *, QLayout *)

/*
Adds a new row to the bottom of this form layout, with the given label and field.

See also insertRow().
*/
impl /*struct*/ QFormLayout {
  pub fn addRow_1<RetType, T: QFormLayout_addRow_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addRow_1(self);
    // return 1;
  }
}
pub trait QFormLayout_addRow_1<RetType> {
  fn addRow_1(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_addRow_1<(/*void*/)> for (usize,usize) {
  fn addRow_1(self , rsthis: & QFormLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFormLayout6addRowEP7QWidgetP7QLayout", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:113
// index:2
// Public Visibility=Default Availability=Available
// [-2] void addRow(const QString &, QWidget *)

/*
Adds a new row to the bottom of this form layout, with the given label and field.

See also insertRow().
*/
impl /*struct*/ QFormLayout {
  pub fn addRow_2<RetType, T: QFormLayout_addRow_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addRow_2(self);
    // return 1;
  }
}
pub trait QFormLayout_addRow_2<RetType> {
  fn addRow_2(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_addRow_2<(/*void*/)> for (usize,usize) {
  fn addRow_2(self , rsthis: & QFormLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFormLayout6addRowERK7QStringP7QWidget", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:114
// index:3
// Public Visibility=Default Availability=Available
// [-2] void addRow(const QString &, QLayout *)

/*
Adds a new row to the bottom of this form layout, with the given label and field.

See also insertRow().
*/
impl /*struct*/ QFormLayout {
  pub fn addRow_3<RetType, T: QFormLayout_addRow_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addRow_3(self);
    // return 1;
  }
}
pub trait QFormLayout_addRow_3<RetType> {
  fn addRow_3(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_addRow_3<(/*void*/)> for (usize,usize) {
  fn addRow_3(self , rsthis: & QFormLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFormLayout6addRowERK7QStringP7QLayout", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:115
// index:4
// Public Visibility=Default Availability=Available
// [-2] void addRow(QWidget *)

/*
Adds a new row to the bottom of this form layout, with the given label and field.

See also insertRow().
*/
impl /*struct*/ QFormLayout {
  pub fn addRow_4<RetType, T: QFormLayout_addRow_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addRow_4(self);
    // return 1;
  }
}
pub trait QFormLayout_addRow_4<RetType> {
  fn addRow_4(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_addRow_4<(/*void*/)> for (usize) {
  fn addRow_4(self , rsthis: & QFormLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFormLayout6addRowEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:116
// index:5
// Public Visibility=Default Availability=Available
// [-2] void addRow(QLayout *)

/*
Adds a new row to the bottom of this form layout, with the given label and field.

See also insertRow().
*/
impl /*struct*/ QFormLayout {
  pub fn addRow_5<RetType, T: QFormLayout_addRow_5<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addRow_5(self);
    // return 1;
  }
}
pub trait QFormLayout_addRow_5<RetType> {
  fn addRow_5(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_addRow_5<(/*void*/)> for (usize) {
  fn addRow_5(self , rsthis: & QFormLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFormLayout6addRowEP7QLayout", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:118
// index:0
// Public Visibility=Default Availability=Available
// [-2] void insertRow(int, QWidget *, QWidget *)

/*
Inserts a new row at position row in this form layout, with the given label and field. If row is out of bounds, the new row is added at the end.

See also addRow().
*/
impl /*struct*/ QFormLayout {
  pub fn insertRow_0<RetType, T: QFormLayout_insertRow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertRow_0(self);
    // return 1;
  }
}
pub trait QFormLayout_insertRow_0<RetType> {
  fn insertRow_0(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_insertRow_0<(/*void*/)> for (i32,usize,usize) {
  fn insertRow_0(self , rsthis: & QFormLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFormLayout9insertRowEiP7QWidgetS1_", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:119
// index:1
// Public Visibility=Default Availability=Available
// [-2] void insertRow(int, QWidget *, QLayout *)

/*
Inserts a new row at position row in this form layout, with the given label and field. If row is out of bounds, the new row is added at the end.

See also addRow().
*/
impl /*struct*/ QFormLayout {
  pub fn insertRow_1<RetType, T: QFormLayout_insertRow_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertRow_1(self);
    // return 1;
  }
}
pub trait QFormLayout_insertRow_1<RetType> {
  fn insertRow_1(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_insertRow_1<(/*void*/)> for (i32,usize,usize) {
  fn insertRow_1(self , rsthis: & QFormLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFormLayout9insertRowEiP7QWidgetP7QLayout", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:120
// index:2
// Public Visibility=Default Availability=Available
// [-2] void insertRow(int, const QString &, QWidget *)

/*
Inserts a new row at position row in this form layout, with the given label and field. If row is out of bounds, the new row is added at the end.

See also addRow().
*/
impl /*struct*/ QFormLayout {
  pub fn insertRow_2<RetType, T: QFormLayout_insertRow_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertRow_2(self);
    // return 1;
  }
}
pub trait QFormLayout_insertRow_2<RetType> {
  fn insertRow_2(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_insertRow_2<(/*void*/)> for (i32,usize,usize) {
  fn insertRow_2(self , rsthis: & QFormLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFormLayout9insertRowEiRK7QStringP7QWidget", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:121
// index:3
// Public Visibility=Default Availability=Available
// [-2] void insertRow(int, const QString &, QLayout *)

/*
Inserts a new row at position row in this form layout, with the given label and field. If row is out of bounds, the new row is added at the end.

See also addRow().
*/
impl /*struct*/ QFormLayout {
  pub fn insertRow_3<RetType, T: QFormLayout_insertRow_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertRow_3(self);
    // return 1;
  }
}
pub trait QFormLayout_insertRow_3<RetType> {
  fn insertRow_3(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_insertRow_3<(/*void*/)> for (i32,usize,usize) {
  fn insertRow_3(self , rsthis: & QFormLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFormLayout9insertRowEiRK7QStringP7QLayout", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:122
// index:4
// Public Visibility=Default Availability=Available
// [-2] void insertRow(int, QWidget *)

/*
Inserts a new row at position row in this form layout, with the given label and field. If row is out of bounds, the new row is added at the end.

See also addRow().
*/
impl /*struct*/ QFormLayout {
  pub fn insertRow_4<RetType, T: QFormLayout_insertRow_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertRow_4(self);
    // return 1;
  }
}
pub trait QFormLayout_insertRow_4<RetType> {
  fn insertRow_4(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_insertRow_4<(/*void*/)> for (i32,usize) {
  fn insertRow_4(self , rsthis: & QFormLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFormLayout9insertRowEiP7QWidget", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:123
// index:5
// Public Visibility=Default Availability=Available
// [-2] void insertRow(int, QLayout *)

/*
Inserts a new row at position row in this form layout, with the given label and field. If row is out of bounds, the new row is added at the end.

See also addRow().
*/
impl /*struct*/ QFormLayout {
  pub fn insertRow_5<RetType, T: QFormLayout_insertRow_5<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertRow_5(self);
    // return 1;
  }
}
pub trait QFormLayout_insertRow_5<RetType> {
  fn insertRow_5(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_insertRow_5<(/*void*/)> for (i32,usize) {
  fn insertRow_5(self , rsthis: & QFormLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFormLayout9insertRowEiP7QLayout", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:125
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeRow(int)

/*
Deletes row row from this form layout.

row must be non-negative and less than rowCount().

After this call, rowCount() is decremented by one. All widgets and nested layouts that occupied this row are deleted. That includes both the field widget(s) and the label, if any. All following rows are shifted up one row and the freed vertical space is redistributed amongst the remaining rows.

You can use this function to undo a previous addRow() or insertRow():


  QFormLayout *flay = ...;
  QPointer<QLineEdit> le = new QLineEdit;
  flay->insertRow(2, "User:", le);
  // later:
  flay->removeRow(2); // le == nullptr at this point



If you want to remove the row from the layout without deleting the widgets, use takeRow() instead.

This function was introduced in  Qt 5.8.

See also takeRow().
*/
impl /*struct*/ QFormLayout {
  pub fn removeRow_0<RetType, T: QFormLayout_removeRow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeRow_0(self);
    // return 1;
  }
}
pub trait QFormLayout_removeRow_0<RetType> {
  fn removeRow_0(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_removeRow_0<(/*void*/)> for (i32) {
  fn removeRow_0(self , rsthis: & QFormLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QFormLayout9removeRowEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:126
// index:1
// Public Visibility=Default Availability=Available
// [-2] void removeRow(QWidget *)

/*
Deletes row row from this form layout.

row must be non-negative and less than rowCount().

After this call, rowCount() is decremented by one. All widgets and nested layouts that occupied this row are deleted. That includes both the field widget(s) and the label, if any. All following rows are shifted up one row and the freed vertical space is redistributed amongst the remaining rows.

You can use this function to undo a previous addRow() or insertRow():


  QFormLayout *flay = ...;
  QPointer<QLineEdit> le = new QLineEdit;
  flay->insertRow(2, "User:", le);
  // later:
  flay->removeRow(2); // le == nullptr at this point



If you want to remove the row from the layout without deleting the widgets, use takeRow() instead.

This function was introduced in  Qt 5.8.

See also takeRow().
*/
impl /*struct*/ QFormLayout {
  pub fn removeRow_1<RetType, T: QFormLayout_removeRow_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeRow_1(self);
    // return 1;
  }
}
pub trait QFormLayout_removeRow_1<RetType> {
  fn removeRow_1(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_removeRow_1<(/*void*/)> for (usize) {
  fn removeRow_1(self , rsthis: & QFormLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFormLayout9removeRowEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:127
// index:2
// Public Visibility=Default Availability=Available
// [-2] void removeRow(QLayout *)

/*
Deletes row row from this form layout.

row must be non-negative and less than rowCount().

After this call, rowCount() is decremented by one. All widgets and nested layouts that occupied this row are deleted. That includes both the field widget(s) and the label, if any. All following rows are shifted up one row and the freed vertical space is redistributed amongst the remaining rows.

You can use this function to undo a previous addRow() or insertRow():


  QFormLayout *flay = ...;
  QPointer<QLineEdit> le = new QLineEdit;
  flay->insertRow(2, "User:", le);
  // later:
  flay->removeRow(2); // le == nullptr at this point



If you want to remove the row from the layout without deleting the widgets, use takeRow() instead.

This function was introduced in  Qt 5.8.

See also takeRow().
*/
impl /*struct*/ QFormLayout {
  pub fn removeRow_2<RetType, T: QFormLayout_removeRow_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeRow_2(self);
    // return 1;
  }
}
pub trait QFormLayout_removeRow_2<RetType> {
  fn removeRow_2(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_removeRow_2<(/*void*/)> for (usize) {
  fn removeRow_2(self , rsthis: & QFormLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFormLayout9removeRowEP7QLayout", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:129
// index:0
// Public Visibility=Default Availability=Available
// [16] QFormLayout::TakeRowResult takeRow(int)

/*
Removes the specified row from this form layout.

row must be non-negative and less than rowCount().

Note: This function doesn't delete anything.

After this call, rowCount() is decremented by one. All following rows are shifted up one row and the freed vertical space is redistributed amongst the remaining rows.

You can use this function to undo a previous addRow() or insertRow():


  QFormLayout *flay = ...;
  QPointer<QLineEdit> le = new QLineEdit;
  flay->insertRow(2, "User:", le);
  // later:
  QFormLayout::TakeRowResult result = flay->takeRow(2);



If you want to remove the row from the layout and delete the widgets, use removeRow() instead.

Returns A structure containing both the widget and corresponding label layout items

This function was introduced in  Qt 5.8.

See also removeRow().
*/
impl /*struct*/ QFormLayout {
  pub fn takeRow_0<RetType, T: QFormLayout_takeRow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.takeRow_0(self);
    // return 1;
  }
}
pub trait QFormLayout_takeRow_0<RetType> {
  fn takeRow_0(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_takeRow_0<usize> for (i32) {
  fn takeRow_0(self , rsthis: & QFormLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QFormLayout7takeRowEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:130
// index:1
// Public Visibility=Default Availability=Available
// [16] QFormLayout::TakeRowResult takeRow(QWidget *)

/*
Removes the specified row from this form layout.

row must be non-negative and less than rowCount().

Note: This function doesn't delete anything.

After this call, rowCount() is decremented by one. All following rows are shifted up one row and the freed vertical space is redistributed amongst the remaining rows.

You can use this function to undo a previous addRow() or insertRow():


  QFormLayout *flay = ...;
  QPointer<QLineEdit> le = new QLineEdit;
  flay->insertRow(2, "User:", le);
  // later:
  QFormLayout::TakeRowResult result = flay->takeRow(2);



If you want to remove the row from the layout and delete the widgets, use removeRow() instead.

Returns A structure containing both the widget and corresponding label layout items

This function was introduced in  Qt 5.8.

See also removeRow().
*/
impl /*struct*/ QFormLayout {
  pub fn takeRow_1<RetType, T: QFormLayout_takeRow_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.takeRow_1(self);
    // return 1;
  }
}
pub trait QFormLayout_takeRow_1<RetType> {
  fn takeRow_1(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_takeRow_1<usize> for (usize) {
  fn takeRow_1(self , rsthis: & QFormLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QFormLayout7takeRowEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:131
// index:2
// Public Visibility=Default Availability=Available
// [16] QFormLayout::TakeRowResult takeRow(QLayout *)

/*
Removes the specified row from this form layout.

row must be non-negative and less than rowCount().

Note: This function doesn't delete anything.

After this call, rowCount() is decremented by one. All following rows are shifted up one row and the freed vertical space is redistributed amongst the remaining rows.

You can use this function to undo a previous addRow() or insertRow():


  QFormLayout *flay = ...;
  QPointer<QLineEdit> le = new QLineEdit;
  flay->insertRow(2, "User:", le);
  // later:
  QFormLayout::TakeRowResult result = flay->takeRow(2);



If you want to remove the row from the layout and delete the widgets, use removeRow() instead.

Returns A structure containing both the widget and corresponding label layout items

This function was introduced in  Qt 5.8.

See also removeRow().
*/
impl /*struct*/ QFormLayout {
  pub fn takeRow_2<RetType, T: QFormLayout_takeRow_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.takeRow_2(self);
    // return 1;
  }
}
pub trait QFormLayout_takeRow_2<RetType> {
  fn takeRow_2(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_takeRow_2<usize> for (usize) {
  fn takeRow_2(self , rsthis: & QFormLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QFormLayout7takeRowEP7QLayout", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:133
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setItem(int, QFormLayout::ItemRole, QLayoutItem *)

/*
Sets the item in the given row for the given role to item, extending the layout with empty rows if necessary.

If the cell is already occupied, the item is not inserted and an error message is sent to the console. The item spans both columns.

Warning: Do not use this function to add child layouts or child widget items. Use setLayout() or setWidget() instead.

See also setLayout().
*/
impl /*struct*/ QFormLayout {
  pub fn setItem_0<RetType, T: QFormLayout_setItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setItem_0(self);
    // return 1;
  }
}
pub trait QFormLayout_setItem_0<RetType> {
  fn setItem_0(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_setItem_0<(/*void*/)> for (i32,i32,usize) {
  fn setItem_0(self , rsthis: & QFormLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFormLayout7setItemEiNS_8ItemRoleEP11QLayoutItem", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:134
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWidget(int, QFormLayout::ItemRole, QWidget *)

/*
Sets the widget in the given row for the given role to widget, extending the layout with empty rows if necessary.

If the cell is already occupied, the widget is not inserted and an error message is sent to the console.

Note: For most applications, addRow() or insertRow() should be used instead of setWidget().

See also setLayout().
*/
impl /*struct*/ QFormLayout {
  pub fn setWidget_0<RetType, T: QFormLayout_setWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWidget_0(self);
    // return 1;
  }
}
pub trait QFormLayout_setWidget_0<RetType> {
  fn setWidget_0(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_setWidget_0<(/*void*/)> for (i32,i32,usize) {
  fn setWidget_0(self , rsthis: & QFormLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFormLayout9setWidgetEiNS_8ItemRoleEP7QWidget", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:135
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLayout(int, QFormLayout::ItemRole, QLayout *)

/*
Sets the sub-layout in the given row for the given role to layout, extending the form layout with empty rows if necessary.

If the cell is already occupied, the layout is not inserted and an error message is sent to the console.

Note: For most applications, addRow() or insertRow() should be used instead of setLayout().

See also setWidget().
*/
impl /*struct*/ QFormLayout {
  pub fn setLayout_0<RetType, T: QFormLayout_setLayout_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLayout_0(self);
    // return 1;
  }
}
pub trait QFormLayout_setLayout_0<RetType> {
  fn setLayout_0(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_setLayout_0<(/*void*/)> for (i32,i32,usize) {
  fn setLayout_0(self , rsthis: & QFormLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFormLayout9setLayoutEiNS_8ItemRoleEP7QLayout", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:137
// index:0
// Public Visibility=Default Availability=Available
// [8] QLayoutItem * itemAt(int, QFormLayout::ItemRole) const

/*
Returns the layout item in the given row with the specified role (column). Returns 0 if there is no such item.

See also QLayout::itemAt() and setItem().
*/
impl /*struct*/ QFormLayout {
  pub fn itemAt_0<RetType, T: QFormLayout_itemAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemAt_0(self);
    // return 1;
  }
}
pub trait QFormLayout_itemAt_0<RetType> {
  fn itemAt_0(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_itemAt_0<usize> for (i32,i32) {
  fn itemAt_0(self , rsthis: & QFormLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFormLayout6itemAtEiNS_8ItemRoleE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:146
// index:1
// Public virtual Visibility=Default Availability=Available
// [8] QLayoutItem * itemAt(int) const

/*
Returns the layout item in the given row with the specified role (column). Returns 0 if there is no such item.

See also QLayout::itemAt() and setItem().
*/
impl /*struct*/ QFormLayout {
  pub fn itemAt_1<RetType, T: QFormLayout_itemAt_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemAt_1(self);
    // return 1;
  }
}
pub trait QFormLayout_itemAt_1<RetType> {
  fn itemAt_1(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_itemAt_1<usize> for (i32) {
  fn itemAt_1(self , rsthis: & QFormLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFormLayout6itemAtEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:138
// index:0
// Public Visibility=Default Availability=Available
// [-2] void getItemPosition(int, int *, QFormLayout::ItemRole *) const

/*
Retrieves the row and role (column) of the item at the specified index. If index is out of bounds, *rowPtr is set to -1; otherwise the row is stored in *rowPtr and the role is stored in *rolePtr.

See also itemAt(), count(), getLayoutPosition(), and getWidgetPosition().
*/
impl /*struct*/ QFormLayout {
  pub fn getItemPosition_0<RetType, T: QFormLayout_getItemPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.getItemPosition_0(self);
    // return 1;
  }
}
pub trait QFormLayout_getItemPosition_0<RetType> {
  fn getItemPosition_0(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_getItemPosition_0<(/*void*/)> for (i32,usize,usize) {
  fn getItemPosition_0(self , rsthis: & QFormLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK11QFormLayout15getItemPositionEiPiPNS_8ItemRoleE", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:139
// index:0
// Public Visibility=Default Availability=Available
// [-2] void getWidgetPosition(QWidget *, int *, QFormLayout::ItemRole *) const

/*
Retrieves the row and role (column) of the specified widget in the layout. If widget is not in the layout, *rowPtr is set to -1; otherwise the row is stored in *rowPtr and the role is stored in *rolePtr.

See also getItemPosition() and itemAt().
*/
impl /*struct*/ QFormLayout {
  pub fn getWidgetPosition_0<RetType, T: QFormLayout_getWidgetPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.getWidgetPosition_0(self);
    // return 1;
  }
}
pub trait QFormLayout_getWidgetPosition_0<RetType> {
  fn getWidgetPosition_0(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_getWidgetPosition_0<(/*void*/)> for (usize,usize,usize) {
  fn getWidgetPosition_0(self , rsthis: & QFormLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK11QFormLayout17getWidgetPositionEP7QWidgetPiPNS_8ItemRoleE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:140
// index:0
// Public Visibility=Default Availability=Available
// [-2] void getLayoutPosition(QLayout *, int *, QFormLayout::ItemRole *) const

/*
Retrieves the row and role (column) of the specified child layout. If layout is not in the form layout, *rowPtr is set to -1; otherwise the row is stored in *rowPtr and the role is stored in *rolePtr.
*/
impl /*struct*/ QFormLayout {
  pub fn getLayoutPosition_0<RetType, T: QFormLayout_getLayoutPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.getLayoutPosition_0(self);
    // return 1;
  }
}
pub trait QFormLayout_getLayoutPosition_0<RetType> {
  fn getLayoutPosition_0(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_getLayoutPosition_0<(/*void*/)> for (usize,usize,usize) {
  fn getLayoutPosition_0(self , rsthis: & QFormLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK11QFormLayout17getLayoutPositionEP7QLayoutPiPNS_8ItemRoleE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:141
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * labelForField(QWidget *) const

/*
Returns the label associated with the given field.

See also itemAt().
*/
impl /*struct*/ QFormLayout {
  pub fn labelForField_0<RetType, T: QFormLayout_labelForField_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.labelForField_0(self);
    // return 1;
  }
}
pub trait QFormLayout_labelForField_0<RetType> {
  fn labelForField_0(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_labelForField_0<usize> for (usize) {
  fn labelForField_0(self , rsthis: & QFormLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFormLayout13labelForFieldEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:142
// index:1
// Public Visibility=Default Availability=Available
// [8] QWidget * labelForField(QLayout *) const

/*
Returns the label associated with the given field.

See also itemAt().
*/
impl /*struct*/ QFormLayout {
  pub fn labelForField_1<RetType, T: QFormLayout_labelForField_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.labelForField_1(self);
    // return 1;
  }
}
pub trait QFormLayout_labelForField_1<RetType> {
  fn labelForField_1(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_labelForField_1<usize> for (usize) {
  fn labelForField_1(self , rsthis: & QFormLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFormLayout13labelForFieldEP7QLayout", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:145
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void addItem(QLayoutItem *)

/*
Reimplemented from QLayout::addItem().
*/
impl /*struct*/ QFormLayout {
  pub fn addItem_0<RetType, T: QFormLayout_addItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addItem_0(self);
    // return 1;
  }
}
pub trait QFormLayout_addItem_0<RetType> {
  fn addItem_0(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_addItem_0<(/*void*/)> for (usize) {
  fn addItem_0(self , rsthis: & QFormLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFormLayout7addItemEP11QLayoutItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:147
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QLayoutItem * takeAt(int)

/*
Reimplemented from QLayout::takeAt().
*/
impl /*struct*/ QFormLayout {
  pub fn takeAt_0<RetType, T: QFormLayout_takeAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.takeAt_0(self);
    // return 1;
  }
}
pub trait QFormLayout_takeAt_0<RetType> {
  fn takeAt_0(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_takeAt_0<usize> for (i32) {
  fn takeAt_0(self , rsthis: & QFormLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QFormLayout6takeAtEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:149
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setGeometry(const QRect &)

/*
Reimplemented from QLayoutItem::setGeometry().
*/
impl /*struct*/ QFormLayout {
  pub fn setGeometry_0<RetType, T: QFormLayout_setGeometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setGeometry_0(self);
    // return 1;
  }
}
pub trait QFormLayout_setGeometry_0<RetType> {
  fn setGeometry_0(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_setGeometry_0<(/*void*/)> for (usize) {
  fn setGeometry_0(self , rsthis: & QFormLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFormLayout11setGeometryERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:150
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize minimumSize() const

/*
Reimplemented from QLayoutItem::minimumSize().
*/
impl /*struct*/ QFormLayout {
  pub fn minimumSize_0<RetType, T: QFormLayout_minimumSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumSize_0(self);
    // return 1;
  }
}
pub trait QFormLayout_minimumSize_0<RetType> {
  fn minimumSize_0(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_minimumSize_0<usize> for () {
  fn minimumSize_0(self , rsthis: & QFormLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFormLayout11minimumSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:151
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Reimplemented from QLayoutItem::sizeHint().
*/
impl /*struct*/ QFormLayout {
  pub fn sizeHint_0<RetType, T: QFormLayout_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QFormLayout_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QFormLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFormLayout8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:152
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void invalidate()

/*
Reimplemented from QLayoutItem::invalidate().
*/
impl /*struct*/ QFormLayout {
  pub fn invalidate_0<RetType, T: QFormLayout_invalidate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.invalidate_0(self);
    // return 1;
  }
}
pub trait QFormLayout_invalidate_0<RetType> {
  fn invalidate_0(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_invalidate_0<(/*void*/)> for () {
  fn invalidate_0(self , rsthis: & QFormLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QFormLayout10invalidateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:154
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool hasHeightForWidth() const

/*
Reimplemented from QLayoutItem::hasHeightForWidth().
*/
impl /*struct*/ QFormLayout {
  pub fn hasHeightForWidth_0<RetType, T: QFormLayout_hasHeightForWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasHeightForWidth_0(self);
    // return 1;
  }
}
pub trait QFormLayout_hasHeightForWidth_0<RetType> {
  fn hasHeightForWidth_0(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_hasHeightForWidth_0<bool> for () {
  fn hasHeightForWidth_0(self , rsthis: & QFormLayout) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFormLayout17hasHeightForWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:155
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int heightForWidth(int) const

/*
Reimplemented from QLayoutItem::heightForWidth().
*/
impl /*struct*/ QFormLayout {
  pub fn heightForWidth_0<RetType, T: QFormLayout_heightForWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.heightForWidth_0(self);
    // return 1;
  }
}
pub trait QFormLayout_heightForWidth_0<RetType> {
  fn heightForWidth_0(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_heightForWidth_0<i32> for (i32) {
  fn heightForWidth_0(self , rsthis: & QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFormLayout14heightForWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:156
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] Qt::Orientations expandingDirections() const

/*
Reimplemented from QLayoutItem::expandingDirections().
*/
impl /*struct*/ QFormLayout {
  pub fn expandingDirections_0<RetType, T: QFormLayout_expandingDirections_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.expandingDirections_0(self);
    // return 1;
  }
}
pub trait QFormLayout_expandingDirections_0<RetType> {
  fn expandingDirections_0(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_expandingDirections_0<i32> for () {
  fn expandingDirections_0(self , rsthis: & QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFormLayout19expandingDirectionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:157
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int count() const

/*
Reimplemented from QLayout::count().
*/
impl /*struct*/ QFormLayout {
  pub fn count_0<RetType, T: QFormLayout_count_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.count_0(self);
    // return 1;
  }
}
pub trait QFormLayout_count_0<RetType> {
  fn count_0(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_count_0<i32> for () {
  fn count_0(self , rsthis: & QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFormLayout5countEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qformlayout.h:159
// index:0
// Public Visibility=Default Availability=Available
// [4] int rowCount() const

/*
Returns the number of rows in the form.

See also QLayout::count().
*/
impl /*struct*/ QFormLayout {
  pub fn rowCount_0<RetType, T: QFormLayout_rowCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowCount_0(self);
    // return 1;
  }
}
pub trait QFormLayout_rowCount_0<RetType> {
  fn rowCount_0(self , rsthis: & QFormLayout) -> RetType;
}
impl<'a> /*trait*/ QFormLayout_rowCount_0<i32> for () {
  fn rowCount_0(self , rsthis: & QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFormLayout8rowCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}


/*
This enum specifies the different policies that can be used to control the way in which the form's fields grow.



See also fieldGrowthPolicy.

*/
pub type QFormLayout__FieldGrowthPolicy = i32;
// The fields never grow beyond their effective size hint. This is the default for QMacStyle.
pub const QFormLayout__FieldsStayAtSizeHint :QFormLayout__FieldGrowthPolicy = 0;
// Fields with an horizontal size policy of Expanding or MinimumExpanding will grow to fill the available space. The other fields will not grow beyond their effective size hint. This is the default policy for Plastique.
pub const QFormLayout__ExpandingFieldsGrow :QFormLayout__FieldGrowthPolicy = 1;
// All fields with a size policy that allows them to grow will grow to fill the available space. This is the default policy for most styles.
pub const QFormLayout__AllNonFixedFieldsGrow :QFormLayout__FieldGrowthPolicy = 2;
pub fn QFormLayout_FieldGrowthPolicyItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QFormLayout", val);
}
pub fn QFormLayout_FieldGrowthPolicyItemName_s(val: i32) ->String {
  //var nilthis *QFormLayout
  //return nilthis.FieldGrowthPolicyItemName(val);
  return QFormLayout_FieldGrowthPolicyItemName(val);
}


/*
This enum specifies the different policies that can be used to control the way in which the form's rows wrap.



See also rowWrapPolicy.

*/
pub type QFormLayout__RowWrapPolicy = i32;
// Fields are always laid out next to their label. This is the default policy for all styles except Qt Extended styles.
pub const QFormLayout__DontWrapRows :QFormLayout__RowWrapPolicy = 0;
// Labels are given enough horizontal space to fit the widest label, and the rest of the space is given to the fields. If the minimum size of a field pair is wider than the available space, the field is wrapped to the next line. This is the default policy for Qt Extended styles.
pub const QFormLayout__WrapLongRows :QFormLayout__RowWrapPolicy = 1;
// Fields are always laid out below their label.
pub const QFormLayout__WrapAllRows :QFormLayout__RowWrapPolicy = 2;
pub fn QFormLayout_RowWrapPolicyItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QFormLayout", val);
}
pub fn QFormLayout_RowWrapPolicyItemName_s(val: i32) ->String {
  //var nilthis *QFormLayout
  //return nilthis.RowWrapPolicyItemName(val);
  return QFormLayout_RowWrapPolicyItemName(val);
}


/*
This enum specifies the types of widgets (or other layout items) that may appear in a row.



See also itemAt() and getItemPosition().

*/
pub type QFormLayout__ItemRole = i32;
// A label widget.
pub const QFormLayout__LabelRole :QFormLayout__ItemRole = 0;
// A field widget.
pub const QFormLayout__FieldRole :QFormLayout__ItemRole = 1;
// A widget that spans label and field columns.
pub const QFormLayout__SpanningRole :QFormLayout__ItemRole = 2;
pub fn QFormLayout_ItemRoleItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QFormLayout", val);
}
pub fn QFormLayout_ItemRoleItemName_s(val: i32) ->String {
  //var nilthis *QFormLayout
  //return nilthis.ItemRoleItemName(val);
  return QFormLayout_ItemRoleItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
