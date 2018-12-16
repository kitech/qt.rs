

// mod ::widgets::QStyleOption
// package qtwidgets
// /usr/include/qt/QtWidgets/qstyleoption.h
// #include <qstyleoption.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 22
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
#[derive(Default)] // class sizeof(QStyleOption)=64
pub struct QStyleOption {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStyleOption_ITF interface {
//    QStyleOption_PTR() *QStyleOption
//}
//func (ptr *QStyleOption) QStyleOption_PTR() *QStyleOption { return ptr }

impl /*struct*/ QStyleOption {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStyleOption {
    return QStyleOption{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStyleOption {
//  type Target = QStyleOptionBASE;
//
//  fn deref(&self) -> &QStyleOptionBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStyleOptionBASE> for QStyleOption {
//  fn as_ref(& self) -> & QStyleOptionBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qstyleoption.h:102
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStyleOption(int, int)

/*
Constructs a QStyleOption with the specified version and type.

The version has no special meaning for QStyleOption; it can be used by subclasses to distinguish between different version of the same option type.

The state member variable is initialized to QStyle::State_None.

See also version and type.
*/
// QStyleOption(int, int) ctx.fn_proto_cpp
impl /*struct*/ QStyleOption {
  pub fn QStyleOption_0<T: QStyleOption_QStyleOption_0>(value: T) -> QStyleOption {
    let rsthis = value.QStyleOption_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOption_QStyleOption_0 {
  fn QStyleOption_0(self) -> QStyleOption;
}
// QStyleOption(int, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOption_QStyleOption_0 for (i32,i32) {
  fn QStyleOption_0(self) -> QStyleOption {
    // unsafe{_ZN12QStyleOptionC2Eii()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QStyleOptionC2Eii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOption{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyleoption.h:104
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QStyleOption()

/*

*/
pub fn DeleteQStyleOption(this :*mut QStyleOption) {
    // let rv = qtrt::InvokeQtFunc6("_ZN12QStyleOptionD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 64)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qstyleoption.h:106
// index:0
// Public Visibility=Default Availability=Available
// [-2] void init(const QWidget *)

/*

*/
impl /*struct*/ QStyleOption {
  pub fn init_0<RetType, T: QStyleOption_init_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.init_0(self);
    // return 1;
  }
}
pub trait QStyleOption_init_0<RetType> {
  fn init_0(self , rsthis: & QStyleOption) -> RetType;
}
impl<'a> /*trait*/ QStyleOption_init_0<(/*void*/)> for (usize) {
  fn init_0(self , rsthis: & QStyleOption) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QStyleOption4initEPK7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyleoption.h:107
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void initFrom(const QWidget *)

/*
Initializes the state, direction, rect, palette, fontMetrics and styleObject member variables based on the specified widget.

This is a convenience function; the member variables can also be initialized manually.

This function was introduced in  Qt 4.1.

See also QWidget::layoutDirection(), QWidget::rect(), QWidget::palette(), and QWidget::fontMetrics().
*/
impl /*struct*/ QStyleOption {
  pub fn initFrom_0<RetType, T: QStyleOption_initFrom_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.initFrom_0(self);
    // return 1;
  }
}
pub trait QStyleOption_initFrom_0<RetType> {
  fn initFrom_0(self , rsthis: & QStyleOption) -> RetType;
}
impl<'a> /*trait*/ QStyleOption_initFrom_0<(/*void*/)> for (usize) {
  fn initFrom_0(self , rsthis: & QStyleOption) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QStyleOption8initFromEPK7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyleoption.h:108
// index:0
// Public Visibility=Default Availability=Available
// [64] QStyleOption & operator=(const QStyleOption &)

/*

*/
impl /*struct*/ QStyleOption {
  pub fn operator_equal_0<RetType, T: QStyleOption_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QStyleOption_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QStyleOption) -> RetType;
}
impl<'a> /*trait*/ QStyleOption_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QStyleOption) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QStyleOptionaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


/*
This enum is used internally by QStyleOption, its subclasses, and qstyleoption_cast() to determine the type of style option. In general you do not need to worry about this unless you want to create your own QStyleOption subclass and your own styles.

QStyleOption::SO_ComboBox?QStyleOptionComboBox
QStyleOption::SO_GroupBox?QStyleOptionGroupBox
QStyleOption::SO_SizeGrip?QStyleOptionSizeGrip
QStyleOption::SO_Slider?QStyleOptionSlider
QStyleOption::SO_SpinBox?QStyleOptionSpinBox
QStyleOption::SO_TitleBar?QStyleOptionTitleBar
QStyleOption::SO_ToolButton?QStyleOptionToolButton


The following values are used for custom controls:



See also type.

*/
pub type QStyleOption__OptionType = i32;
// QStyleOption
pub const QStyleOption__SO_Default :QStyleOption__OptionType = 0;
// QStyleOptionFocusRect
pub const QStyleOption__SO_FocusRect :QStyleOption__OptionType = 1;
// QStyleOptionButton
pub const QStyleOption__SO_Button :QStyleOption__OptionType = 2;
// QStyleOptionTab
pub const QStyleOption__SO_Tab :QStyleOption__OptionType = 3;
// QStyleOptionMenuItem
pub const QStyleOption__SO_MenuItem :QStyleOption__OptionType = 4;
// QStyleOptionFrame
pub const QStyleOption__SO_Frame :QStyleOption__OptionType = 5;
// QStyleOptionProgressBar
pub const QStyleOption__SO_ProgressBar :QStyleOption__OptionType = 6;
// QStyleOptionToolBox
pub const QStyleOption__SO_ToolBox :QStyleOption__OptionType = 7;
// QStyleOptionHeader
pub const QStyleOption__SO_Header :QStyleOption__OptionType = 8;
// QStyleOptionDockWidget
pub const QStyleOption__SO_DockWidget :QStyleOption__OptionType = 9;
// 
pub const QStyleOption__SO_ViewItem :QStyleOption__OptionType = 10;
// 
pub const QStyleOption__SO_TabWidgetFrame :QStyleOption__OptionType = 11;
// 
pub const QStyleOption__SO_TabBarBase :QStyleOption__OptionType = 12;
// 
pub const QStyleOption__SO_RubberBand :QStyleOption__OptionType = 13;
// 
pub const QStyleOption__SO_ToolBar :QStyleOption__OptionType = 14;
// 
pub const QStyleOption__SO_GraphicsItem :QStyleOption__OptionType = 15;
// 
pub const QStyleOption__SO_Complex :QStyleOption__OptionType = 983040;
// 
pub const QStyleOption__SO_Slider :QStyleOption__OptionType = 983041;
// 
pub const QStyleOption__SO_SpinBox :QStyleOption__OptionType = 983042;
// 
pub const QStyleOption__SO_ToolButton :QStyleOption__OptionType = 983043;
// 
pub const QStyleOption__SO_ComboBox :QStyleOption__OptionType = 983044;
// 
pub const QStyleOption__SO_TitleBar :QStyleOption__OptionType = 983045;
// 
pub const QStyleOption__SO_GroupBox :QStyleOption__OptionType = 983046;
// 
pub const QStyleOption__SO_SizeGrip :QStyleOption__OptionType = 983047;
// 
pub const QStyleOption__SO_CustomBase :QStyleOption__OptionType = 3840;
// 
pub const QStyleOption__SO_ComplexCustomBase :QStyleOption__OptionType = 251658240;
pub fn QStyleOption_OptionTypeItemName(val: i32) ->String {
  match val {
     QStyleOption__SO_Default => // 0
     {return String::from("SO_Default");}
     QStyleOption__SO_FocusRect => // 1
     {return String::from("SO_FocusRect");}
     QStyleOption__SO_Button => // 2
     {return String::from("SO_Button");}
     QStyleOption__SO_Tab => // 3
     {return String::from("SO_Tab");}
     QStyleOption__SO_MenuItem => // 4
     {return String::from("SO_MenuItem");}
     QStyleOption__SO_Frame => // 5
     {return String::from("SO_Frame");}
     QStyleOption__SO_ProgressBar => // 6
     {return String::from("SO_ProgressBar");}
     QStyleOption__SO_ToolBox => // 7
     {return String::from("SO_ToolBox");}
     QStyleOption__SO_Header => // 8
     {return String::from("SO_Header");}
     QStyleOption__SO_DockWidget => // 9
     {return String::from("SO_DockWidget");}
     QStyleOption__SO_ViewItem => // 10
     {return String::from("SO_ViewItem");}
     QStyleOption__SO_TabWidgetFrame => // 11
     {return String::from("SO_TabWidgetFrame");}
     QStyleOption__SO_TabBarBase => // 12
     {return String::from("SO_TabBarBase");}
     QStyleOption__SO_RubberBand => // 13
     {return String::from("SO_RubberBand");}
     QStyleOption__SO_ToolBar => // 14
     {return String::from("SO_ToolBar");}
     QStyleOption__SO_GraphicsItem => // 15
     {return String::from("SO_GraphicsItem");}
     QStyleOption__SO_Complex => // 983040
     {return String::from("SO_Complex");}
     QStyleOption__SO_Slider => // 983041
     {return String::from("SO_Slider");}
     QStyleOption__SO_SpinBox => // 983042
     {return String::from("SO_SpinBox");}
     QStyleOption__SO_ToolButton => // 983043
     {return String::from("SO_ToolButton");}
     QStyleOption__SO_ComboBox => // 983044
     {return String::from("SO_ComboBox");}
     QStyleOption__SO_TitleBar => // 983045
     {return String::from("SO_TitleBar");}
     QStyleOption__SO_GroupBox => // 983046
     {return String::from("SO_GroupBox");}
     QStyleOption__SO_SizeGrip => // 983047
     {return String::from("SO_SizeGrip");}
     QStyleOption__SO_CustomBase => // 3840
     {return String::from("SO_CustomBase");}
     QStyleOption__SO_ComplexCustomBase => // 251658240
     {return String::from("SO_ComplexCustomBase");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOption_OptionTypeItemName_s(val: i32) ->String {
  //var nilthis *QStyleOption
  //return nilthis.OptionTypeItemName(val);
  return QStyleOption_OptionTypeItemName(val);
}


/*
This enum is used to hold information about the type of the style option, and is defined for each QStyleOption subclass.

QStyleOption::TypeSO_DefaultThe type of style option provided (SO_Default for this class).


The type is used internally by QStyleOption, its subclasses, and qstyleoption_cast() to determine the type of style option. In general you do not need to worry about this unless you want to create your own QStyleOption subclass and your own styles.

See also StyleOptionVersion.

*/
pub type QStyleOption__StyleOptionType = i32;
// 
pub const QStyleOption__Type :QStyleOption__StyleOptionType = 0;
pub fn QStyleOption_StyleOptionTypeItemName(val: i32) ->String {
  match val {
     QStyleOption__Type => // 0
     {return String::from("Type");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOption_StyleOptionTypeItemName_s(val: i32) ->String {
  //var nilthis *QStyleOption
  //return nilthis.StyleOptionTypeItemName(val);
  return QStyleOption_StyleOptionTypeItemName(val);
}


/*
This enum is used to hold information about the version of the style option, and is defined for each QStyleOption subclass.



The version is used by QStyleOption subclasses to implement extensions without breaking compatibility. If you use qstyleoption_cast(), you normally do not need to check it.

See also StyleOptionType.

*/
pub type QStyleOption__StyleOptionVersion = i32;
// 1
pub const QStyleOption__Version :QStyleOption__StyleOptionVersion = 1;
pub fn QStyleOption_StyleOptionVersionItemName(val: i32) ->String {
  match val {
     QStyleOption__Version => // 1
     {return String::from("Version");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOption_StyleOptionVersionItemName_s(val: i32) ->String {
  //var nilthis *QStyleOption
  //return nilthis.StyleOptionVersionItemName(val);
  return QStyleOption_StyleOptionVersionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
