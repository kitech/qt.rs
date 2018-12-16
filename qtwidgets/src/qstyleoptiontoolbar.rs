

// mod ::widgets::QStyleOptionToolBar
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
// extern C begin: 2
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
#[derive(Default)] // class sizeof(QStyleOptionToolBar)=88
pub struct QStyleOptionToolBar {
  qbase: QStyleOption,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStyleOptionToolBar_ITF interface {
//    QStyleOption_ITF
//    QStyleOptionToolBar_PTR() *QStyleOptionToolBar
//}
//func (ptr *QStyleOptionToolBar) QStyleOptionToolBar_PTR() *QStyleOptionToolBar { return ptr }

impl /*struct*/ QStyleOptionToolBar {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStyleOptionToolBar {
    return QStyleOptionToolBar{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStyleOptionToolBar {
//  type Target = QStyleOptionToolBarBASE;
//
//  fn deref(&self) -> &QStyleOptionToolBarBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStyleOptionToolBarBASE> for QStyleOptionToolBar {
//  fn as_ref(& self) -> & QStyleOptionToolBarBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qstyleoption.h:315
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStyleOptionToolBar()

/*

*/
// QStyleOptionToolBar() ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionToolBar {
  pub fn QStyleOptionToolBar_0<T: QStyleOptionToolBar_QStyleOptionToolBar_0>(value: T) -> QStyleOptionToolBar {
    let rsthis = value.QStyleOptionToolBar_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionToolBar_QStyleOptionToolBar_0 {
  fn QStyleOptionToolBar_0(self) -> QStyleOptionToolBar;
}
// QStyleOptionToolBar() ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionToolBar_QStyleOptionToolBar_0 for () {
  fn QStyleOptionToolBar_0(self) -> QStyleOptionToolBar {
    // unsafe{_ZN19QStyleOptionToolBarC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QStyleOptionToolBarC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionToolBar{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyleoption.h:319
// index:1
// Protected Visibility=Default Availability=Available
// [-2] void QStyleOptionToolBar(int)

/*

*/
// QStyleOptionToolBar(int) ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionToolBar {
  pub fn QStyleOptionToolBar_1<T: QStyleOptionToolBar_QStyleOptionToolBar_1>(value: T) -> QStyleOptionToolBar {
    let rsthis = value.QStyleOptionToolBar_1();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionToolBar_QStyleOptionToolBar_1 {
  fn QStyleOptionToolBar_1(self) -> QStyleOptionToolBar;
}
// QStyleOptionToolBar(int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionToolBar_QStyleOptionToolBar_1 for (i32) {
  fn QStyleOptionToolBar_1(self) -> QStyleOptionToolBar {
    // unsafe{_ZN19QStyleOptionToolBarC2Ei()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QStyleOptionToolBarC2Ei", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionToolBar{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}


pub fn DeleteQStyleOptionToolBar(this :*mut QStyleOptionToolBar) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN19QStyleOptionToolBarD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*
This enum is used to hold information about the type of the style option, and is defined for each QStyleOption subclass.

QStyleOption::TypeSO_DefaultThe type of style option provided (SO_Default for this class).


The type is used internally by QStyleOption, its subclasses, and qstyleoption_cast() to determine the type of style option. In general you do not need to worry about this unless you want to create your own QStyleOption subclass and your own styles.

See also StyleOptionVersion.

*/
pub type QStyleOptionToolBar__StyleOptionType = i32;
// 
pub const QStyleOptionToolBar__Type :QStyleOptionToolBar__StyleOptionType = 14;
pub fn QStyleOptionToolBar_StyleOptionTypeItemName(val: i32) ->String {
  match val {
     QStyleOptionToolBar__Type => // 14
     {return String::from("Type");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionToolBar_StyleOptionTypeItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionToolBar
  //return nilthis.StyleOptionTypeItemName(val);
  return QStyleOptionToolBar_StyleOptionTypeItemName(val);
}


/*
This enum is used to hold information about the version of the style option, and is defined for each QStyleOption subclass.



The version is used by QStyleOption subclasses to implement extensions without breaking compatibility. If you use qstyleoption_cast(), you normally do not need to check it.

See also StyleOptionType.

*/
pub type QStyleOptionToolBar__StyleOptionVersion = i32;
// 1
pub const QStyleOptionToolBar__Version :QStyleOptionToolBar__StyleOptionVersion = 1;
pub fn QStyleOptionToolBar_StyleOptionVersionItemName(val: i32) ->String {
  match val {
     QStyleOptionToolBar__Version => // 1
     {return String::from("Version");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionToolBar_StyleOptionVersionItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionToolBar
  //return nilthis.StyleOptionVersionItemName(val);
  return QStyleOptionToolBar_StyleOptionVersionItemName(val);
}


/*


*/
pub type QStyleOptionToolBar__ToolBarPosition = i32;
// 
pub const QStyleOptionToolBar__Beginning :QStyleOptionToolBar__ToolBarPosition = 0;
// 
pub const QStyleOptionToolBar__Middle :QStyleOptionToolBar__ToolBarPosition = 1;
// 
pub const QStyleOptionToolBar__End :QStyleOptionToolBar__ToolBarPosition = 2;
// 
pub const QStyleOptionToolBar__OnlyOne :QStyleOptionToolBar__ToolBarPosition = 3;
pub fn QStyleOptionToolBar_ToolBarPositionItemName(val: i32) ->String {
  match val {
     QStyleOptionToolBar__Beginning => // 0
     {return String::from("Beginning");}
     QStyleOptionToolBar__Middle => // 1
     {return String::from("Middle");}
     QStyleOptionToolBar__End => // 2
     {return String::from("End");}
     QStyleOptionToolBar__OnlyOne => // 3
     {return String::from("OnlyOne");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionToolBar_ToolBarPositionItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionToolBar
  //return nilthis.ToolBarPositionItemName(val);
  return QStyleOptionToolBar_ToolBarPositionItemName(val);
}


/*


*/
pub type QStyleOptionToolBar__ToolBarFeature = i32;
// 
pub const QStyleOptionToolBar__None :QStyleOptionToolBar__ToolBarFeature = 0;
// 
pub const QStyleOptionToolBar__Movable :QStyleOptionToolBar__ToolBarFeature = 1;
pub fn QStyleOptionToolBar_ToolBarFeatureItemName(val: i32) ->String {
  match val {
     QStyleOptionToolBar__None => // 0
     {return String::from("None");}
     QStyleOptionToolBar__Movable => // 1
     {return String::from("Movable");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionToolBar_ToolBarFeatureItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionToolBar
  //return nilthis.ToolBarFeatureItemName(val);
  return QStyleOptionToolBar_ToolBarFeatureItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
