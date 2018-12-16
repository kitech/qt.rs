

// mod ::widgets::QAbstractSpinBox
// package qtwidgets
// /usr/include/qt/QtWidgets/qabstractspinbox.h
// #include <qabstractspinbox.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 47
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

// void resizeEvent(QResizeEvent *)
// func (this *QAbstractSpinBox) InheritResizeEvent(f func(event *qtgui.QResizeEvent/*777 QResizeEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "resizeEvent", f)
// }

// void keyPressEvent(QKeyEvent *)
// func (this *QAbstractSpinBox) InheritKeyPressEvent(f func(event *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyPressEvent", f)
// }

// void keyReleaseEvent(QKeyEvent *)
// func (this *QAbstractSpinBox) InheritKeyReleaseEvent(f func(event *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyReleaseEvent", f)
// }

// void wheelEvent(QWheelEvent *)
// func (this *QAbstractSpinBox) InheritWheelEvent(f func(event *qtgui.QWheelEvent/*777 QWheelEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "wheelEvent", f)
// }

// void focusInEvent(QFocusEvent *)
// func (this *QAbstractSpinBox) InheritFocusInEvent(f func(event *qtgui.QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusInEvent", f)
// }

// void focusOutEvent(QFocusEvent *)
// func (this *QAbstractSpinBox) InheritFocusOutEvent(f func(event *qtgui.QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusOutEvent", f)
// }

// void contextMenuEvent(QContextMenuEvent *)
// func (this *QAbstractSpinBox) InheritContextMenuEvent(f func(event *qtgui.QContextMenuEvent/*777 QContextMenuEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "contextMenuEvent", f)
// }

// void changeEvent(QEvent *)
// func (this *QAbstractSpinBox) InheritChangeEvent(f func(event *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "changeEvent", f)
// }

// void closeEvent(QCloseEvent *)
// func (this *QAbstractSpinBox) InheritCloseEvent(f func(event *qtgui.QCloseEvent/*777 QCloseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "closeEvent", f)
// }

// void hideEvent(QHideEvent *)
// func (this *QAbstractSpinBox) InheritHideEvent(f func(event *qtgui.QHideEvent/*777 QHideEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "hideEvent", f)
// }

// void mousePressEvent(QMouseEvent *)
// func (this *QAbstractSpinBox) InheritMousePressEvent(f func(event *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mousePressEvent", f)
// }

// void mouseReleaseEvent(QMouseEvent *)
// func (this *QAbstractSpinBox) InheritMouseReleaseEvent(f func(event *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseReleaseEvent", f)
// }

// void mouseMoveEvent(QMouseEvent *)
// func (this *QAbstractSpinBox) InheritMouseMoveEvent(f func(event *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseMoveEvent", f)
// }

// void timerEvent(QTimerEvent *)
// func (this *QAbstractSpinBox) InheritTimerEvent(f func(event *qtcore.QTimerEvent/*777 QTimerEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "timerEvent", f)
// }

// void paintEvent(QPaintEvent *)
// func (this *QAbstractSpinBox) InheritPaintEvent(f func(event *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }

// void showEvent(QShowEvent *)
// func (this *QAbstractSpinBox) InheritShowEvent(f func(event *qtgui.QShowEvent/*777 QShowEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "showEvent", f)
// }

// void initStyleOption(QStyleOptionSpinBox *)
// func (this *QAbstractSpinBox) InheritInitStyleOption(f func(option *QStyleOptionSpinBox/*777 QStyleOptionSpinBox **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "initStyleOption", f)
// }

// QLineEdit * lineEdit()
// func (this *QAbstractSpinBox) InheritLineEdit(f func() unsafe.Pointer/*666*/) {
//  qtrt.SetAllInheritCallback(this, "lineEdit", f)
// }

// void setLineEdit(QLineEdit *)
// func (this *QAbstractSpinBox) InheritSetLineEdit(f func(edit *QLineEdit/*777 QLineEdit **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "setLineEdit", f)
// }

// QAbstractSpinBox::StepEnabled stepEnabled()
// func (this *QAbstractSpinBox) InheritStepEnabled(f func() int) {
//  qtrt.SetAllInheritCallback(this, "stepEnabled", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QAbstractSpinBox)=48
pub struct QAbstractSpinBox {
  qbase: QWidget,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAbstractSpinBox_ITF interface {
//    QWidget_ITF
//    QAbstractSpinBox_PTR() *QAbstractSpinBox
//}
//func (ptr *QAbstractSpinBox) QAbstractSpinBox_PTR() *QAbstractSpinBox { return ptr }

impl /*struct*/ QAbstractSpinBox {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAbstractSpinBox {
    return QAbstractSpinBox{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAbstractSpinBox {
//  type Target = QAbstractSpinBoxBASE;
//
//  fn deref(&self) -> &QAbstractSpinBoxBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAbstractSpinBoxBASE> for QAbstractSpinBox {
//  fn as_ref(& self) -> & QAbstractSpinBoxBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qabstractspinbox.h:58
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QAbstractSpinBox {
  pub fn metaObject_0<RetType, T: QAbstractSpinBox_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QAbstractSpinBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QAbstractSpinBox10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:73
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QAbstractSpinBox(QWidget *)

/*
Constructs an abstract spinbox with the given parent with default wrapping, and alignment properties.
*/
// QAbstractSpinBox(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QAbstractSpinBox {
  pub fn QAbstractSpinBox_0<T: QAbstractSpinBox_QAbstractSpinBox_0>(value: T) -> QAbstractSpinBox {
    let rsthis = value.QAbstractSpinBox_0();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractSpinBox_QAbstractSpinBox_0 {
  fn QAbstractSpinBox_0(self) -> QAbstractSpinBox;
}
// QAbstractSpinBox(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QAbstractSpinBox_QAbstractSpinBox_0 for (usize) {
  fn QAbstractSpinBox_0(self) -> QAbstractSpinBox {
    // unsafe{_ZN16QAbstractSpinBoxC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QAbstractSpinBoxC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAbstractSpinBox{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:74
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QAbstractSpinBox()

/*

*/
pub fn DeleteQAbstractSpinBox(this :*mut QAbstractSpinBox) {
    // let rv = qtrt::InvokeQtFunc6("_ZN16QAbstractSpinBoxD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qabstractspinbox.h:83
// index:0
// Public Visibility=Default Availability=Available
// [4] QAbstractSpinBox::ButtonSymbols buttonSymbols() const

/*

*/
impl /*struct*/ QAbstractSpinBox {
  pub fn buttonSymbols_0<RetType, T: QAbstractSpinBox_buttonSymbols_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.buttonSymbols_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_buttonSymbols_0<RetType> {
  fn buttonSymbols_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_buttonSymbols_0<i32> for () {
  fn buttonSymbols_0(self , rsthis: & QAbstractSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QAbstractSpinBox13buttonSymbolsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:84
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setButtonSymbols(QAbstractSpinBox::ButtonSymbols)

/*

*/
impl /*struct*/ QAbstractSpinBox {
  pub fn setButtonSymbols_0<RetType, T: QAbstractSpinBox_setButtonSymbols_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setButtonSymbols_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_setButtonSymbols_0<RetType> {
  fn setButtonSymbols_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_setButtonSymbols_0<(/*void*/)> for (i32) {
  fn setButtonSymbols_0(self , rsthis: & QAbstractSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN16QAbstractSpinBox16setButtonSymbolsENS_13ButtonSymbolsE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:89
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCorrectionMode(QAbstractSpinBox::CorrectionMode)

/*

*/
impl /*struct*/ QAbstractSpinBox {
  pub fn setCorrectionMode_0<RetType, T: QAbstractSpinBox_setCorrectionMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCorrectionMode_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_setCorrectionMode_0<RetType> {
  fn setCorrectionMode_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_setCorrectionMode_0<(/*void*/)> for (i32) {
  fn setCorrectionMode_0(self , rsthis: & QAbstractSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN16QAbstractSpinBox17setCorrectionModeENS_14CorrectionModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:90
// index:0
// Public Visibility=Default Availability=Available
// [4] QAbstractSpinBox::CorrectionMode correctionMode() const

/*

*/
impl /*struct*/ QAbstractSpinBox {
  pub fn correctionMode_0<RetType, T: QAbstractSpinBox_correctionMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.correctionMode_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_correctionMode_0<RetType> {
  fn correctionMode_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_correctionMode_0<i32> for () {
  fn correctionMode_0(self , rsthis: & QAbstractSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QAbstractSpinBox14correctionModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:92
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasAcceptableInput() const

/*

*/
impl /*struct*/ QAbstractSpinBox {
  pub fn hasAcceptableInput_0<RetType, T: QAbstractSpinBox_hasAcceptableInput_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasAcceptableInput_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_hasAcceptableInput_0<RetType> {
  fn hasAcceptableInput_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_hasAcceptableInput_0<bool> for () {
  fn hasAcceptableInput_0(self , rsthis: & QAbstractSpinBox) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QAbstractSpinBox18hasAcceptableInputEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:93
// index:0
// Public Visibility=Default Availability=Available
// [8] QString text() const

/*

*/
impl /*struct*/ QAbstractSpinBox {
  pub fn text_0<RetType, T: QAbstractSpinBox_text_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.text_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_text_0<RetType> {
  fn text_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_text_0<usize> for () {
  fn text_0(self , rsthis: & QAbstractSpinBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QAbstractSpinBox4textEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:95
// index:0
// Public Visibility=Default Availability=Available
// [8] QString specialValueText() const

/*

*/
impl /*struct*/ QAbstractSpinBox {
  pub fn specialValueText_0<RetType, T: QAbstractSpinBox_specialValueText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.specialValueText_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_specialValueText_0<RetType> {
  fn specialValueText_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_specialValueText_0<usize> for () {
  fn specialValueText_0(self , rsthis: & QAbstractSpinBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QAbstractSpinBox16specialValueTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:96
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSpecialValueText(const QString &)

/*

*/
impl /*struct*/ QAbstractSpinBox {
  pub fn setSpecialValueText_0<RetType, T: QAbstractSpinBox_setSpecialValueText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSpecialValueText_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_setSpecialValueText_0<RetType> {
  fn setSpecialValueText_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_setSpecialValueText_0<(/*void*/)> for (usize) {
  fn setSpecialValueText_0(self , rsthis: & QAbstractSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QAbstractSpinBox19setSpecialValueTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:98
// index:0
// Public Visibility=Default Availability=Available
// [1] bool wrapping() const

/*

*/
impl /*struct*/ QAbstractSpinBox {
  pub fn wrapping_0<RetType, T: QAbstractSpinBox_wrapping_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.wrapping_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_wrapping_0<RetType> {
  fn wrapping_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_wrapping_0<bool> for () {
  fn wrapping_0(self , rsthis: & QAbstractSpinBox) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QAbstractSpinBox8wrappingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:99
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWrapping(bool)

/*

*/
impl /*struct*/ QAbstractSpinBox {
  pub fn setWrapping_0<RetType, T: QAbstractSpinBox_setWrapping_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWrapping_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_setWrapping_0<RetType> {
  fn setWrapping_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_setWrapping_0<(/*void*/)> for (bool) {
  fn setWrapping_0(self , rsthis: & QAbstractSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN16QAbstractSpinBox11setWrappingEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:101
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setReadOnly(bool)

/*

*/
impl /*struct*/ QAbstractSpinBox {
  pub fn setReadOnly_0<RetType, T: QAbstractSpinBox_setReadOnly_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setReadOnly_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_setReadOnly_0<RetType> {
  fn setReadOnly_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_setReadOnly_0<(/*void*/)> for (bool) {
  fn setReadOnly_0(self , rsthis: & QAbstractSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN16QAbstractSpinBox11setReadOnlyEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:102
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isReadOnly() const

/*

*/
impl /*struct*/ QAbstractSpinBox {
  pub fn isReadOnly_0<RetType, T: QAbstractSpinBox_isReadOnly_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isReadOnly_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_isReadOnly_0<RetType> {
  fn isReadOnly_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_isReadOnly_0<bool> for () {
  fn isReadOnly_0(self , rsthis: & QAbstractSpinBox) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QAbstractSpinBox10isReadOnlyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:104
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setKeyboardTracking(bool)

/*

*/
impl /*struct*/ QAbstractSpinBox {
  pub fn setKeyboardTracking_0<RetType, T: QAbstractSpinBox_setKeyboardTracking_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setKeyboardTracking_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_setKeyboardTracking_0<RetType> {
  fn setKeyboardTracking_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_setKeyboardTracking_0<(/*void*/)> for (bool) {
  fn setKeyboardTracking_0(self , rsthis: & QAbstractSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN16QAbstractSpinBox19setKeyboardTrackingEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:105
// index:0
// Public Visibility=Default Availability=Available
// [1] bool keyboardTracking() const

/*

*/
impl /*struct*/ QAbstractSpinBox {
  pub fn keyboardTracking_0<RetType, T: QAbstractSpinBox_keyboardTracking_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyboardTracking_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_keyboardTracking_0<RetType> {
  fn keyboardTracking_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_keyboardTracking_0<bool> for () {
  fn keyboardTracking_0(self , rsthis: & QAbstractSpinBox) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QAbstractSpinBox16keyboardTrackingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:107
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAlignment(Qt::Alignment)

/*

*/
impl /*struct*/ QAbstractSpinBox {
  pub fn setAlignment_0<RetType, T: QAbstractSpinBox_setAlignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAlignment_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_setAlignment_0<RetType> {
  fn setAlignment_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_setAlignment_0<(/*void*/)> for (i32) {
  fn setAlignment_0(self , rsthis: & QAbstractSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN16QAbstractSpinBox12setAlignmentE6QFlagsIN2Qt13AlignmentFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:108
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::Alignment alignment() const

/*

*/
impl /*struct*/ QAbstractSpinBox {
  pub fn alignment_0<RetType, T: QAbstractSpinBox_alignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.alignment_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_alignment_0<RetType> {
  fn alignment_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_alignment_0<i32> for () {
  fn alignment_0(self , rsthis: & QAbstractSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QAbstractSpinBox9alignmentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:110
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFrame(bool)

/*

*/
impl /*struct*/ QAbstractSpinBox {
  pub fn setFrame_0<RetType, T: QAbstractSpinBox_setFrame_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFrame_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_setFrame_0<RetType> {
  fn setFrame_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_setFrame_0<(/*void*/)> for (bool) {
  fn setFrame_0(self , rsthis: & QAbstractSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN16QAbstractSpinBox8setFrameEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:111
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasFrame() const

/*

*/
impl /*struct*/ QAbstractSpinBox {
  pub fn hasFrame_0<RetType, T: QAbstractSpinBox_hasFrame_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasFrame_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_hasFrame_0<RetType> {
  fn hasFrame_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_hasFrame_0<bool> for () {
  fn hasFrame_0(self , rsthis: & QAbstractSpinBox) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QAbstractSpinBox8hasFrameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:113
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAccelerated(bool)

/*

*/
impl /*struct*/ QAbstractSpinBox {
  pub fn setAccelerated_0<RetType, T: QAbstractSpinBox_setAccelerated_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAccelerated_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_setAccelerated_0<RetType> {
  fn setAccelerated_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_setAccelerated_0<(/*void*/)> for (bool) {
  fn setAccelerated_0(self , rsthis: & QAbstractSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN16QAbstractSpinBox14setAcceleratedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:114
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isAccelerated() const

/*

*/
impl /*struct*/ QAbstractSpinBox {
  pub fn isAccelerated_0<RetType, T: QAbstractSpinBox_isAccelerated_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isAccelerated_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_isAccelerated_0<RetType> {
  fn isAccelerated_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_isAccelerated_0<bool> for () {
  fn isAccelerated_0(self , rsthis: & QAbstractSpinBox) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QAbstractSpinBox13isAcceleratedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:116
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setGroupSeparatorShown(bool)

/*

*/
impl /*struct*/ QAbstractSpinBox {
  pub fn setGroupSeparatorShown_0<RetType, T: QAbstractSpinBox_setGroupSeparatorShown_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setGroupSeparatorShown_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_setGroupSeparatorShown_0<RetType> {
  fn setGroupSeparatorShown_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_setGroupSeparatorShown_0<(/*void*/)> for (bool) {
  fn setGroupSeparatorShown_0(self , rsthis: & QAbstractSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN16QAbstractSpinBox22setGroupSeparatorShownEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:117
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isGroupSeparatorShown() const

/*

*/
impl /*struct*/ QAbstractSpinBox {
  pub fn isGroupSeparatorShown_0<RetType, T: QAbstractSpinBox_isGroupSeparatorShown_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isGroupSeparatorShown_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_isGroupSeparatorShown_0<RetType> {
  fn isGroupSeparatorShown_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_isGroupSeparatorShown_0<bool> for () {
  fn isGroupSeparatorShown_0(self , rsthis: & QAbstractSpinBox) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QAbstractSpinBox21isGroupSeparatorShownEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:119
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Reimplemented from QWidget::sizeHint().
*/
impl /*struct*/ QAbstractSpinBox {
  pub fn sizeHint_0<RetType, T: QAbstractSpinBox_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QAbstractSpinBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QAbstractSpinBox8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:120
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize minimumSizeHint() const

/*
Reimplemented from QWidget::minimumSizeHint().
*/
impl /*struct*/ QAbstractSpinBox {
  pub fn minimumSizeHint_0<RetType, T: QAbstractSpinBox_minimumSizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_minimumSizeHint_0<RetType> {
  fn minimumSizeHint_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_minimumSizeHint_0<usize> for () {
  fn minimumSizeHint_0(self , rsthis: & QAbstractSpinBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QAbstractSpinBox15minimumSizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:121
// index:0
// Public Visibility=Default Availability=Available
// [-2] void interpretText()

/*
This function interprets the text of the spin box. If the value has changed since last interpretation it will emit signals.
*/
impl /*struct*/ QAbstractSpinBox {
  pub fn interpretText_0<RetType, T: QAbstractSpinBox_interpretText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.interpretText_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_interpretText_0<RetType> {
  fn interpretText_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_interpretText_0<(/*void*/)> for () {
  fn interpretText_0(self , rsthis: & QAbstractSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN16QAbstractSpinBox13interpretTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:122
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QAbstractSpinBox {
  pub fn event_0<RetType, T: QAbstractSpinBox_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_event_0<RetType> {
  fn event_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QAbstractSpinBox) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QAbstractSpinBox5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:124
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QVariant inputMethodQuery(Qt::InputMethodQuery) const

/*
Reimplemented from QWidget::inputMethodQuery().
*/
impl /*struct*/ QAbstractSpinBox {
  pub fn inputMethodQuery_0<RetType, T: QAbstractSpinBox_inputMethodQuery_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inputMethodQuery_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_inputMethodQuery_0<RetType> {
  fn inputMethodQuery_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_inputMethodQuery_0<usize> for (i32) {
  fn inputMethodQuery_0(self , rsthis: & QAbstractSpinBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QAbstractSpinBox16inputMethodQueryEN2Qt16InputMethodQueryE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:126
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] QValidator::State validate(QString &, int &) const

/*
This virtual function is called by the QAbstractSpinBox to determine whether input is valid. The pos parameter indicates the position in the string. Reimplemented in the various subclasses.
*/
impl /*struct*/ QAbstractSpinBox {
  pub fn validate_0<RetType, T: QAbstractSpinBox_validate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.validate_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_validate_0<RetType> {
  fn validate_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_validate_0<i32> for (usize,usize) {
  fn validate_0(self , rsthis: & QAbstractSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QAbstractSpinBox8validateER7QStringRi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:127
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void fixup(QString &) const

/*
This virtual function is called by the QAbstractSpinBox if the input is not validated to QValidator::Acceptable when Return is pressed or interpretText() is called. It will try to change the text so it is valid. Reimplemented in the various subclasses.
*/
impl /*struct*/ QAbstractSpinBox {
  pub fn fixup_0<RetType, T: QAbstractSpinBox_fixup_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fixup_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_fixup_0<RetType> {
  fn fixup_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_fixup_0<(/*void*/)> for (usize) {
  fn fixup_0(self , rsthis: & QAbstractSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK16QAbstractSpinBox5fixupER7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:129
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void stepBy(int)

/*
Virtual function that is called whenever the user triggers a step. The steps parameter indicates how many steps were taken, e.g. Pressing Qt::Key_Down will trigger a call to stepBy(-1), whereas pressing Qt::Key_Prior will trigger a call to stepBy(10).

If you subclass QAbstractSpinBox you must reimplement this function. Note that this function is called even if the resulting value will be outside the bounds of minimum and maximum. It's this function's job to handle these situations.
*/
impl /*struct*/ QAbstractSpinBox {
  pub fn stepBy_0<RetType, T: QAbstractSpinBox_stepBy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.stepBy_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_stepBy_0<RetType> {
  fn stepBy_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_stepBy_0<(/*void*/)> for (i32) {
  fn stepBy_0(self , rsthis: & QAbstractSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN16QAbstractSpinBox6stepByEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:131
// index:0
// Public Visibility=Default Availability=Available
// [-2] void stepUp()

/*
Steps up by one linestep Calling this slot is analogous to calling stepBy(1);

See also stepBy() and stepDown().
*/
impl /*struct*/ QAbstractSpinBox {
  pub fn stepUp_0<RetType, T: QAbstractSpinBox_stepUp_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.stepUp_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_stepUp_0<RetType> {
  fn stepUp_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_stepUp_0<(/*void*/)> for () {
  fn stepUp_0(self , rsthis: & QAbstractSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN16QAbstractSpinBox6stepUpEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:132
// index:0
// Public Visibility=Default Availability=Available
// [-2] void stepDown()

/*
Steps down by one linestep Calling this slot is analogous to calling stepBy(-1);

See also stepBy() and stepUp().
*/
impl /*struct*/ QAbstractSpinBox {
  pub fn stepDown_0<RetType, T: QAbstractSpinBox_stepDown_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.stepDown_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_stepDown_0<RetType> {
  fn stepDown_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_stepDown_0<(/*void*/)> for () {
  fn stepDown_0(self , rsthis: & QAbstractSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN16QAbstractSpinBox8stepDownEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:133
// index:0
// Public Visibility=Default Availability=Available
// [-2] void selectAll()

/*
Selects all the text in the spinbox except the prefix and suffix.
*/
impl /*struct*/ QAbstractSpinBox {
  pub fn selectAll_0<RetType, T: QAbstractSpinBox_selectAll_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectAll_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_selectAll_0<RetType> {
  fn selectAll_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_selectAll_0<(/*void*/)> for () {
  fn selectAll_0(self , rsthis: & QAbstractSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN16QAbstractSpinBox9selectAllEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:134
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void clear()

/*
Clears the lineedit of all text but prefix and suffix.
*/
impl /*struct*/ QAbstractSpinBox {
  pub fn clear_0<RetType, T: QAbstractSpinBox_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_clear_0<RetType> {
  fn clear_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_clear_0<(/*void*/)> for () {
  fn clear_0(self , rsthis: & QAbstractSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN16QAbstractSpinBox5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:136
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void resizeEvent(QResizeEvent *)

/*
Reimplemented from QWidget::resizeEvent().
*/
impl /*struct*/ QAbstractSpinBox {
  pub fn resizeEvent_0<RetType, T: QAbstractSpinBox_resizeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_resizeEvent_0<RetType> {
  fn resizeEvent_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_resizeEvent_0<(/*void*/)> for (usize) {
  fn resizeEvent_0(self , rsthis: & QAbstractSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QAbstractSpinBox11resizeEventEP12QResizeEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:137
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyPressEvent(QKeyEvent *)

/*
Reimplemented from QWidget::keyPressEvent().

This function handles keyboard input.

The following keys are handled specifically:


 Enter/ReturnThis will reinterpret the text and emit a signal even if the value has not changed since last time a signal was emitted.
UpThis will invoke stepBy(1)
DownThis will invoke stepBy(-1)
Page upThis will invoke stepBy(10)
Page downThis will invoke stepBy(-10)
*/
impl /*struct*/ QAbstractSpinBox {
  pub fn keyPressEvent_0<RetType, T: QAbstractSpinBox_keyPressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyPressEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_keyPressEvent_0<RetType> {
  fn keyPressEvent_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_keyPressEvent_0<(/*void*/)> for (usize) {
  fn keyPressEvent_0(self , rsthis: & QAbstractSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QAbstractSpinBox13keyPressEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:138
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyReleaseEvent(QKeyEvent *)

/*
Reimplemented from QWidget::keyReleaseEvent().
*/
impl /*struct*/ QAbstractSpinBox {
  pub fn keyReleaseEvent_0<RetType, T: QAbstractSpinBox_keyReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_keyReleaseEvent_0<RetType> {
  fn keyReleaseEvent_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_keyReleaseEvent_0<(/*void*/)> for (usize) {
  fn keyReleaseEvent_0(self , rsthis: & QAbstractSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QAbstractSpinBox15keyReleaseEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:140
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void wheelEvent(QWheelEvent *)

/*
Reimplemented from QWidget::wheelEvent().
*/
impl /*struct*/ QAbstractSpinBox {
  pub fn wheelEvent_0<RetType, T: QAbstractSpinBox_wheelEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.wheelEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_wheelEvent_0<RetType> {
  fn wheelEvent_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_wheelEvent_0<(/*void*/)> for (usize) {
  fn wheelEvent_0(self , rsthis: & QAbstractSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QAbstractSpinBox10wheelEventEP11QWheelEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:142
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusInEvent(QFocusEvent *)

/*
Reimplemented from QWidget::focusInEvent().
*/
impl /*struct*/ QAbstractSpinBox {
  pub fn focusInEvent_0<RetType, T: QAbstractSpinBox_focusInEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusInEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_focusInEvent_0<RetType> {
  fn focusInEvent_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_focusInEvent_0<(/*void*/)> for (usize) {
  fn focusInEvent_0(self , rsthis: & QAbstractSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QAbstractSpinBox12focusInEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:143
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusOutEvent(QFocusEvent *)

/*
Reimplemented from QWidget::focusOutEvent().
*/
impl /*struct*/ QAbstractSpinBox {
  pub fn focusOutEvent_0<RetType, T: QAbstractSpinBox_focusOutEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusOutEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_focusOutEvent_0<RetType> {
  fn focusOutEvent_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_focusOutEvent_0<(/*void*/)> for (usize) {
  fn focusOutEvent_0(self , rsthis: & QAbstractSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QAbstractSpinBox13focusOutEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:145
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void contextMenuEvent(QContextMenuEvent *)

/*
Reimplemented from QWidget::contextMenuEvent().
*/
impl /*struct*/ QAbstractSpinBox {
  pub fn contextMenuEvent_0<RetType, T: QAbstractSpinBox_contextMenuEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contextMenuEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_contextMenuEvent_0<RetType> {
  fn contextMenuEvent_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_contextMenuEvent_0<(/*void*/)> for (usize) {
  fn contextMenuEvent_0(self , rsthis: & QAbstractSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QAbstractSpinBox16contextMenuEventEP17QContextMenuEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:147
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void changeEvent(QEvent *)

/*
Reimplemented from QWidget::changeEvent().
*/
impl /*struct*/ QAbstractSpinBox {
  pub fn changeEvent_0<RetType, T: QAbstractSpinBox_changeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changeEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_changeEvent_0<RetType> {
  fn changeEvent_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_changeEvent_0<(/*void*/)> for (usize) {
  fn changeEvent_0(self , rsthis: & QAbstractSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QAbstractSpinBox11changeEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:148
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void closeEvent(QCloseEvent *)

/*
Reimplemented from QWidget::closeEvent().
*/
impl /*struct*/ QAbstractSpinBox {
  pub fn closeEvent_0<RetType, T: QAbstractSpinBox_closeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.closeEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_closeEvent_0<RetType> {
  fn closeEvent_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_closeEvent_0<(/*void*/)> for (usize) {
  fn closeEvent_0(self , rsthis: & QAbstractSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QAbstractSpinBox10closeEventEP11QCloseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:149
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void hideEvent(QHideEvent *)

/*
Reimplemented from QWidget::hideEvent().
*/
impl /*struct*/ QAbstractSpinBox {
  pub fn hideEvent_0<RetType, T: QAbstractSpinBox_hideEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hideEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_hideEvent_0<RetType> {
  fn hideEvent_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_hideEvent_0<(/*void*/)> for (usize) {
  fn hideEvent_0(self , rsthis: & QAbstractSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QAbstractSpinBox9hideEventEP10QHideEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:150
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mousePressEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mousePressEvent().
*/
impl /*struct*/ QAbstractSpinBox {
  pub fn mousePressEvent_0<RetType, T: QAbstractSpinBox_mousePressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mousePressEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_mousePressEvent_0<RetType> {
  fn mousePressEvent_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_mousePressEvent_0<(/*void*/)> for (usize) {
  fn mousePressEvent_0(self , rsthis: & QAbstractSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QAbstractSpinBox15mousePressEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:151
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseReleaseEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseReleaseEvent().
*/
impl /*struct*/ QAbstractSpinBox {
  pub fn mouseReleaseEvent_0<RetType, T: QAbstractSpinBox_mouseReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_mouseReleaseEvent_0<RetType> {
  fn mouseReleaseEvent_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_mouseReleaseEvent_0<(/*void*/)> for (usize) {
  fn mouseReleaseEvent_0(self , rsthis: & QAbstractSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QAbstractSpinBox17mouseReleaseEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:152
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseMoveEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseMoveEvent().
*/
impl /*struct*/ QAbstractSpinBox {
  pub fn mouseMoveEvent_0<RetType, T: QAbstractSpinBox_mouseMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseMoveEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_mouseMoveEvent_0<RetType> {
  fn mouseMoveEvent_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_mouseMoveEvent_0<(/*void*/)> for (usize) {
  fn mouseMoveEvent_0(self , rsthis: & QAbstractSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QAbstractSpinBox14mouseMoveEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:153
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void timerEvent(QTimerEvent *)

/*
Reimplemented from QObject::timerEvent().
*/
impl /*struct*/ QAbstractSpinBox {
  pub fn timerEvent_0<RetType, T: QAbstractSpinBox_timerEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.timerEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_timerEvent_0<RetType> {
  fn timerEvent_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_timerEvent_0<(/*void*/)> for (usize) {
  fn timerEvent_0(self , rsthis: & QAbstractSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QAbstractSpinBox10timerEventEP11QTimerEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:154
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Reimplemented from QWidget::paintEvent().
*/
impl /*struct*/ QAbstractSpinBox {
  pub fn paintEvent_0<RetType, T: QAbstractSpinBox_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QAbstractSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QAbstractSpinBox10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:155
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void showEvent(QShowEvent *)

/*
Reimplemented from QWidget::showEvent().
*/
impl /*struct*/ QAbstractSpinBox {
  pub fn showEvent_0<RetType, T: QAbstractSpinBox_showEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_showEvent_0<RetType> {
  fn showEvent_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_showEvent_0<(/*void*/)> for (usize) {
  fn showEvent_0(self , rsthis: & QAbstractSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QAbstractSpinBox9showEventEP10QShowEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:156
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void initStyleOption(QStyleOptionSpinBox *) const

/*
Initialize option with the values from this QSpinBox. This method is useful for subclasses when they need a QStyleOptionSpinBox, but don't want to fill in all the information themselves.

See also QStyleOption::initFrom().
*/
impl /*struct*/ QAbstractSpinBox {
  pub fn initStyleOption_0<RetType, T: QAbstractSpinBox_initStyleOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.initStyleOption_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_initStyleOption_0<RetType> {
  fn initStyleOption_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_initStyleOption_0<(/*void*/)> for (usize) {
  fn initStyleOption_0(self , rsthis: & QAbstractSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK16QAbstractSpinBox15initStyleOptionEP19QStyleOptionSpinBox", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:158
// index:0
// Protected Visibility=Default Availability=Available
// [8] QLineEdit * lineEdit() const

/*
This function returns a pointer to the line edit of the spin box.

See also setLineEdit().
*/
impl /*struct*/ QAbstractSpinBox {
  pub fn lineEdit_0<RetType, T: QAbstractSpinBox_lineEdit_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lineEdit_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_lineEdit_0<RetType> {
  fn lineEdit_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_lineEdit_0<usize> for () {
  fn lineEdit_0(self , rsthis: & QAbstractSpinBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QAbstractSpinBox8lineEditEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:159
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void setLineEdit(QLineEdit *)

/*
Sets the line edit of the spinbox to be lineEdit instead of the current line edit widget. lineEdit can not be 0.

QAbstractSpinBox takes ownership of the new lineEdit

If QLineEdit::validator() for the lineEdit returns 0, the internal validator of the spinbox will be set on the line edit.

See also lineEdit().
*/
impl /*struct*/ QAbstractSpinBox {
  pub fn setLineEdit_0<RetType, T: QAbstractSpinBox_setLineEdit_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLineEdit_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_setLineEdit_0<RetType> {
  fn setLineEdit_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_setLineEdit_0<(/*void*/)> for (usize) {
  fn setLineEdit_0(self , rsthis: & QAbstractSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QAbstractSpinBox11setLineEditEP9QLineEdit", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:161
// index:0
// Protected virtual Visibility=Default Availability=Available
// [4] QAbstractSpinBox::StepEnabled stepEnabled() const

/*
Virtual function that determines whether stepping up and down is legal at any given time.

The up arrow will be painted as disabled unless (stepEnabled() & StepUpEnabled) != 0.

The default implementation will return (StepUpEnabled| StepDownEnabled) if wrapping is turned on. Else it will return StepDownEnabled if value is > minimum() or'ed with StepUpEnabled if value < maximum().

If you subclass QAbstractSpinBox you will need to reimplement this function.

See also QSpinBox::minimum(), QSpinBox::maximum(), and wrapping().
*/
impl /*struct*/ QAbstractSpinBox {
  pub fn stepEnabled_0<RetType, T: QAbstractSpinBox_stepEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.stepEnabled_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_stepEnabled_0<RetType> {
  fn stepEnabled_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_stepEnabled_0<i32> for () {
  fn stepEnabled_0(self , rsthis: & QAbstractSpinBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QAbstractSpinBox11stepEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractspinbox.h:163
// index:0
// Public Visibility=Default Availability=Available
// [-2] void editingFinished()

/*
This signal is emitted editing is finished. This happens when the spinbox loses focus and when enter is pressed.
*/
impl /*struct*/ QAbstractSpinBox {
  pub fn editingFinished_0<RetType, T: QAbstractSpinBox_editingFinished_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.editingFinished_0(self);
    // return 1;
  }
}
pub trait QAbstractSpinBox_editingFinished_0<RetType> {
  fn editingFinished_0(self , rsthis: & QAbstractSpinBox) -> RetType;
}
impl<'a> /*trait*/ QAbstractSpinBox_editingFinished_0<(/*void*/)> for () {
  fn editingFinished_0(self , rsthis: & QAbstractSpinBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN16QAbstractSpinBox15editingFinishedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*


*/
pub type QAbstractSpinBox__StepEnabledFlag = i32;
// 
pub const QAbstractSpinBox__StepNone :QAbstractSpinBox__StepEnabledFlag = 0;
// 
pub const QAbstractSpinBox__StepUpEnabled :QAbstractSpinBox__StepEnabledFlag = 1;
// 
pub const QAbstractSpinBox__StepDownEnabled :QAbstractSpinBox__StepEnabledFlag = 2;
pub fn QAbstractSpinBox_StepEnabledFlagItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QAbstractSpinBox", val);
}
pub fn QAbstractSpinBox_StepEnabledFlagItemName_s(val: i32) ->String {
  //var nilthis *QAbstractSpinBox
  //return nilthis.StepEnabledFlagItemName(val);
  return QAbstractSpinBox_StepEnabledFlagItemName(val);
}


/*
This enum type describes the symbols that can be displayed on the buttons in a spin box.

 



See also QAbstractSpinBox::buttonSymbols.

*/
pub type QAbstractSpinBox__ButtonSymbols = i32;
// Little arrows in the classic style.
pub const QAbstractSpinBox__UpDownArrows :QAbstractSpinBox__ButtonSymbols = 0;
// + and - symbols.
pub const QAbstractSpinBox__PlusMinus :QAbstractSpinBox__ButtonSymbols = 1;
// Don't display buttons.
pub const QAbstractSpinBox__NoButtons :QAbstractSpinBox__ButtonSymbols = 2;
pub fn QAbstractSpinBox_ButtonSymbolsItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QAbstractSpinBox", val);
}
pub fn QAbstractSpinBox_ButtonSymbolsItemName_s(val: i32) ->String {
  //var nilthis *QAbstractSpinBox
  //return nilthis.ButtonSymbolsItemName(val);
  return QAbstractSpinBox_ButtonSymbolsItemName(val);
}


/*
This enum type describes the mode the spinbox will use to correct an Intermediate value if editing finishes.



See also correctionMode.

*/
pub type QAbstractSpinBox__CorrectionMode = i32;
// The spinbox will revert to the last valid value.
pub const QAbstractSpinBox__CorrectToPreviousValue :QAbstractSpinBox__CorrectionMode = 0;
// The spinbox will revert to the nearest valid value.
pub const QAbstractSpinBox__CorrectToNearestValue :QAbstractSpinBox__CorrectionMode = 1;
pub fn QAbstractSpinBox_CorrectionModeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QAbstractSpinBox", val);
}
pub fn QAbstractSpinBox_CorrectionModeItemName_s(val: i32) ->String {
  //var nilthis *QAbstractSpinBox
  //return nilthis.CorrectionModeItemName(val);
  return QAbstractSpinBox_CorrectionModeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
