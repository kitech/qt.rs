

// mod ::widgets::QStyleOptionToolButton
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
#[derive(Default)] // class sizeof(QStyleOptionToolButton)=136
pub struct QStyleOptionToolButton {
  qbase: QStyleOptionComplex,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStyleOptionToolButton_ITF interface {
//    QStyleOptionComplex_ITF
//    QStyleOptionToolButton_PTR() *QStyleOptionToolButton
//}
//func (ptr *QStyleOptionToolButton) QStyleOptionToolButton_PTR() *QStyleOptionToolButton { return ptr }

impl /*struct*/ QStyleOptionToolButton {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStyleOptionToolButton {
    return QStyleOptionToolButton{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStyleOptionToolButton {
//  type Target = QStyleOptionToolButtonBASE;
//
//  fn deref(&self) -> &QStyleOptionToolButtonBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStyleOptionToolButtonBASE> for QStyleOptionToolButton {
//  fn as_ref(& self) -> & QStyleOptionToolButtonBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qstyleoption.h:579
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStyleOptionToolButton()

/*

*/
// QStyleOptionToolButton() ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionToolButton {
  pub fn QStyleOptionToolButton_0<T: QStyleOptionToolButton_QStyleOptionToolButton_0>(value: T) -> QStyleOptionToolButton {
    let rsthis = value.QStyleOptionToolButton_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionToolButton_QStyleOptionToolButton_0 {
  fn QStyleOptionToolButton_0(self) -> QStyleOptionToolButton;
}
// QStyleOptionToolButton() ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionToolButton_QStyleOptionToolButton_0 for () {
  fn QStyleOptionToolButton_0(self) -> QStyleOptionToolButton {
    // unsafe{_ZN22QStyleOptionToolButtonC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN22QStyleOptionToolButtonC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionToolButton{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyleoption.h:583
// index:1
// Protected Visibility=Default Availability=Available
// [-2] void QStyleOptionToolButton(int)

/*

*/
// QStyleOptionToolButton(int) ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionToolButton {
  pub fn QStyleOptionToolButton_1<T: QStyleOptionToolButton_QStyleOptionToolButton_1>(value: T) -> QStyleOptionToolButton {
    let rsthis = value.QStyleOptionToolButton_1();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionToolButton_QStyleOptionToolButton_1 {
  fn QStyleOptionToolButton_1(self) -> QStyleOptionToolButton;
}
// QStyleOptionToolButton(int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionToolButton_QStyleOptionToolButton_1 for (i32) {
  fn QStyleOptionToolButton_1(self) -> QStyleOptionToolButton {
    // unsafe{_ZN22QStyleOptionToolButtonC2Ei()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN22QStyleOptionToolButtonC2Ei", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionToolButton{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}


pub fn DeleteQStyleOptionToolButton(this :*mut QStyleOptionToolButton) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN22QStyleOptionToolButtonD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*
This enum is used to hold information about the type of the style option, and is defined for each QStyleOption subclass.

QStyleOption::TypeSO_DefaultThe type of style option provided (SO_Default for this class).


The type is used internally by QStyleOption, its subclasses, and qstyleoption_cast() to determine the type of style option. In general you do not need to worry about this unless you want to create your own QStyleOption subclass and your own styles.

See also StyleOptionVersion.

*/
pub type QStyleOptionToolButton__StyleOptionType = i32;
// 
pub const QStyleOptionToolButton__Type :QStyleOptionToolButton__StyleOptionType = 983043;
pub fn QStyleOptionToolButton_StyleOptionTypeItemName(val: i32) ->String {
  match val {
     QStyleOptionToolButton__Type => // 983043
     {return String::from("Type");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionToolButton_StyleOptionTypeItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionToolButton
  //return nilthis.StyleOptionTypeItemName(val);
  return QStyleOptionToolButton_StyleOptionTypeItemName(val);
}


/*
This enum is used to hold information about the version of the style option, and is defined for each QStyleOption subclass.



The version is used by QStyleOption subclasses to implement extensions without breaking compatibility. If you use qstyleoption_cast(), you normally do not need to check it.

See also StyleOptionType.

*/
pub type QStyleOptionToolButton__StyleOptionVersion = i32;
// 1
pub const QStyleOptionToolButton__Version :QStyleOptionToolButton__StyleOptionVersion = 1;
pub fn QStyleOptionToolButton_StyleOptionVersionItemName(val: i32) ->String {
  match val {
     QStyleOptionToolButton__Version => // 1
     {return String::from("Version");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionToolButton_StyleOptionVersionItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionToolButton
  //return nilthis.StyleOptionVersionItemName(val);
  return QStyleOptionToolButton_StyleOptionVersionItemName(val);
}


/*


*/
pub type QStyleOptionToolButton__ToolButtonFeature = i32;
// 
pub const QStyleOptionToolButton__None :QStyleOptionToolButton__ToolButtonFeature = 0;
// 
pub const QStyleOptionToolButton__Arrow :QStyleOptionToolButton__ToolButtonFeature = 1;
// 
pub const QStyleOptionToolButton__Menu :QStyleOptionToolButton__ToolButtonFeature = 4;
// 
pub const QStyleOptionToolButton__MenuButtonPopup :QStyleOptionToolButton__ToolButtonFeature = 4;
// 
pub const QStyleOptionToolButton__PopupDelay :QStyleOptionToolButton__ToolButtonFeature = 8;
// 
pub const QStyleOptionToolButton__HasMenu :QStyleOptionToolButton__ToolButtonFeature = 16;
pub fn QStyleOptionToolButton_ToolButtonFeatureItemName(val: i32) ->String {
  match val {
     QStyleOptionToolButton__None => // 0
     {return String::from("None");}
     QStyleOptionToolButton__Arrow => // 1
     {return String::from("Arrow");}
     QStyleOptionToolButton__Menu => // 4
     {return String::from("Menu,MenuButtonPopup");}
    // QStyleOptionToolButton__MenuButtonPopup => // 4
    // {return String::from("");}
     QStyleOptionToolButton__PopupDelay => // 8
     {return String::from("PopupDelay");}
     QStyleOptionToolButton__HasMenu => // 16
     {return String::from("HasMenu");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionToolButton_ToolButtonFeatureItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionToolButton
  //return nilthis.ToolButtonFeatureItemName(val);
  return QStyleOptionToolButton_ToolButtonFeatureItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
