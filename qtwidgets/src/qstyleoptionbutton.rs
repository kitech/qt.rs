

// mod ::widgets::QStyleOptionButton
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
#[derive(Default)] // class sizeof(QStyleOptionButton)=96
pub struct QStyleOptionButton {
  qbase: QStyleOption,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStyleOptionButton_ITF interface {
//    QStyleOption_ITF
//    QStyleOptionButton_PTR() *QStyleOptionButton
//}
//func (ptr *QStyleOptionButton) QStyleOptionButton_PTR() *QStyleOptionButton { return ptr }

impl /*struct*/ QStyleOptionButton {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStyleOptionButton {
    return QStyleOptionButton{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStyleOptionButton {
//  type Target = QStyleOptionButtonBASE;
//
//  fn deref(&self) -> &QStyleOptionButtonBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStyleOptionButtonBASE> for QStyleOptionButton {
//  fn as_ref(& self) -> & QStyleOptionButtonBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qstyleoption.h:248
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStyleOptionButton()

/*

*/
// QStyleOptionButton() ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionButton {
  pub fn QStyleOptionButton_0<T: QStyleOptionButton_QStyleOptionButton_0>(value: T) -> QStyleOptionButton {
    let rsthis = value.QStyleOptionButton_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionButton_QStyleOptionButton_0 {
  fn QStyleOptionButton_0(self) -> QStyleOptionButton;
}
// QStyleOptionButton() ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionButton_QStyleOptionButton_0 for () {
  fn QStyleOptionButton_0(self) -> QStyleOptionButton {
    // unsafe{_ZN18QStyleOptionButtonC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QStyleOptionButtonC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionButton{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyleoption.h:252
// index:1
// Protected Visibility=Default Availability=Available
// [-2] void QStyleOptionButton(int)

/*

*/
// QStyleOptionButton(int) ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionButton {
  pub fn QStyleOptionButton_1<T: QStyleOptionButton_QStyleOptionButton_1>(value: T) -> QStyleOptionButton {
    let rsthis = value.QStyleOptionButton_1();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionButton_QStyleOptionButton_1 {
  fn QStyleOptionButton_1(self) -> QStyleOptionButton;
}
// QStyleOptionButton(int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionButton_QStyleOptionButton_1 for (i32) {
  fn QStyleOptionButton_1(self) -> QStyleOptionButton {
    // unsafe{_ZN18QStyleOptionButtonC2Ei()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QStyleOptionButtonC2Ei", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionButton{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}


pub fn DeleteQStyleOptionButton(this :*mut QStyleOptionButton) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN18QStyleOptionButtonD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*
This enum is used to hold information about the type of the style option, and is defined for each QStyleOption subclass.

QStyleOption::TypeSO_DefaultThe type of style option provided (SO_Default for this class).


The type is used internally by QStyleOption, its subclasses, and qstyleoption_cast() to determine the type of style option. In general you do not need to worry about this unless you want to create your own QStyleOption subclass and your own styles.

See also StyleOptionVersion.

*/
pub type QStyleOptionButton__StyleOptionType = i32;
// 
pub const QStyleOptionButton__Type :QStyleOptionButton__StyleOptionType = 2;
pub fn QStyleOptionButton_StyleOptionTypeItemName(val: i32) ->String {
  match val {
     QStyleOptionButton__Type => // 2
     {return String::from("Type");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionButton_StyleOptionTypeItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionButton
  //return nilthis.StyleOptionTypeItemName(val);
  return QStyleOptionButton_StyleOptionTypeItemName(val);
}


/*
This enum is used to hold information about the version of the style option, and is defined for each QStyleOption subclass.



The version is used by QStyleOption subclasses to implement extensions without breaking compatibility. If you use qstyleoption_cast(), you normally do not need to check it.

See also StyleOptionType.

*/
pub type QStyleOptionButton__StyleOptionVersion = i32;
// 1
pub const QStyleOptionButton__Version :QStyleOptionButton__StyleOptionVersion = 1;
pub fn QStyleOptionButton_StyleOptionVersionItemName(val: i32) ->String {
  match val {
     QStyleOptionButton__Version => // 1
     {return String::from("Version");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionButton_StyleOptionVersionItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionButton
  //return nilthis.StyleOptionVersionItemName(val);
  return QStyleOptionButton_StyleOptionVersionItemName(val);
}


/*


*/
pub type QStyleOptionButton__ButtonFeature = i32;
// 
pub const QStyleOptionButton__None :QStyleOptionButton__ButtonFeature = 0;
// 
pub const QStyleOptionButton__Flat :QStyleOptionButton__ButtonFeature = 1;
// 
pub const QStyleOptionButton__HasMenu :QStyleOptionButton__ButtonFeature = 2;
// 
pub const QStyleOptionButton__DefaultButton :QStyleOptionButton__ButtonFeature = 4;
// 
pub const QStyleOptionButton__AutoDefaultButton :QStyleOptionButton__ButtonFeature = 8;
// 
pub const QStyleOptionButton__CommandLinkButton :QStyleOptionButton__ButtonFeature = 16;
pub fn QStyleOptionButton_ButtonFeatureItemName(val: i32) ->String {
  match val {
     QStyleOptionButton__None => // 0
     {return String::from("None");}
     QStyleOptionButton__Flat => // 1
     {return String::from("Flat");}
     QStyleOptionButton__HasMenu => // 2
     {return String::from("HasMenu");}
     QStyleOptionButton__DefaultButton => // 4
     {return String::from("DefaultButton");}
     QStyleOptionButton__AutoDefaultButton => // 8
     {return String::from("AutoDefaultButton");}
     QStyleOptionButton__CommandLinkButton => // 16
     {return String::from("CommandLinkButton");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionButton_ButtonFeatureItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionButton
  //return nilthis.ButtonFeatureItemName(val);
  return QStyleOptionButton_ButtonFeatureItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
