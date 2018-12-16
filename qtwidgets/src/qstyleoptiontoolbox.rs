

// mod ::widgets::QStyleOptionToolBox
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
#[derive(Default)] // class sizeof(QStyleOptionToolBox)=88
pub struct QStyleOptionToolBox {
  qbase: QStyleOption,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStyleOptionToolBox_ITF interface {
//    QStyleOption_ITF
//    QStyleOptionToolBox_PTR() *QStyleOptionToolBox
//}
//func (ptr *QStyleOptionToolBox) QStyleOptionToolBox_PTR() *QStyleOptionToolBox { return ptr }

impl /*struct*/ QStyleOptionToolBox {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStyleOptionToolBox {
    return QStyleOptionToolBox{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStyleOptionToolBox {
//  type Target = QStyleOptionToolBoxBASE;
//
//  fn deref(&self) -> &QStyleOptionToolBoxBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStyleOptionToolBoxBASE> for QStyleOptionToolBox {
//  fn as_ref(& self) -> & QStyleOptionToolBoxBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qstyleoption.h:472
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStyleOptionToolBox()

/*

*/
// QStyleOptionToolBox() ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionToolBox {
  pub fn QStyleOptionToolBox_0<T: QStyleOptionToolBox_QStyleOptionToolBox_0>(value: T) -> QStyleOptionToolBox {
    let rsthis = value.QStyleOptionToolBox_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionToolBox_QStyleOptionToolBox_0 {
  fn QStyleOptionToolBox_0(self) -> QStyleOptionToolBox;
}
// QStyleOptionToolBox() ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionToolBox_QStyleOptionToolBox_0 for () {
  fn QStyleOptionToolBox_0(self) -> QStyleOptionToolBox {
    // unsafe{_ZN19QStyleOptionToolBoxC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QStyleOptionToolBoxC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionToolBox{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyleoption.h:476
// index:1
// Protected Visibility=Default Availability=Available
// [-2] void QStyleOptionToolBox(int)

/*

*/
// QStyleOptionToolBox(int) ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionToolBox {
  pub fn QStyleOptionToolBox_1<T: QStyleOptionToolBox_QStyleOptionToolBox_1>(value: T) -> QStyleOptionToolBox {
    let rsthis = value.QStyleOptionToolBox_1();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionToolBox_QStyleOptionToolBox_1 {
  fn QStyleOptionToolBox_1(self) -> QStyleOptionToolBox;
}
// QStyleOptionToolBox(int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionToolBox_QStyleOptionToolBox_1 for (i32) {
  fn QStyleOptionToolBox_1(self) -> QStyleOptionToolBox {
    // unsafe{_ZN19QStyleOptionToolBoxC2Ei()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QStyleOptionToolBoxC2Ei", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionToolBox{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}


pub fn DeleteQStyleOptionToolBox(this :*mut QStyleOptionToolBox) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN19QStyleOptionToolBoxD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*
This enum is used to hold information about the type of the style option, and is defined for each QStyleOption subclass.

QStyleOption::TypeSO_DefaultThe type of style option provided (SO_Default for this class).


The type is used internally by QStyleOption, its subclasses, and qstyleoption_cast() to determine the type of style option. In general you do not need to worry about this unless you want to create your own QStyleOption subclass and your own styles.

See also StyleOptionVersion.

*/
pub type QStyleOptionToolBox__StyleOptionType = i32;
// 
pub const QStyleOptionToolBox__Type :QStyleOptionToolBox__StyleOptionType = 7;
pub fn QStyleOptionToolBox_StyleOptionTypeItemName(val: i32) ->String {
  match val {
     QStyleOptionToolBox__Type => // 7
     {return String::from("Type");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionToolBox_StyleOptionTypeItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionToolBox
  //return nilthis.StyleOptionTypeItemName(val);
  return QStyleOptionToolBox_StyleOptionTypeItemName(val);
}


/*
This enum is used to hold information about the version of the style option, and is defined for each QStyleOption subclass.



The version is used by QStyleOption subclasses to implement extensions without breaking compatibility. If you use qstyleoption_cast(), you normally do not need to check it.

See also StyleOptionType.

*/
pub type QStyleOptionToolBox__StyleOptionVersion = i32;
// 1
pub const QStyleOptionToolBox__Version :QStyleOptionToolBox__StyleOptionVersion = 2;
pub fn QStyleOptionToolBox_StyleOptionVersionItemName(val: i32) ->String {
  match val {
     QStyleOptionToolBox__Version => // 2
     {return String::from("Version");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionToolBox_StyleOptionVersionItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionToolBox
  //return nilthis.StyleOptionVersionItemName(val);
  return QStyleOptionToolBox_StyleOptionVersionItemName(val);
}


/*


*/
pub type QStyleOptionToolBox__TabPosition = i32;
// 
pub const QStyleOptionToolBox__Beginning :QStyleOptionToolBox__TabPosition = 0;
// 
pub const QStyleOptionToolBox__Middle :QStyleOptionToolBox__TabPosition = 1;
// 
pub const QStyleOptionToolBox__End :QStyleOptionToolBox__TabPosition = 2;
// 
pub const QStyleOptionToolBox__OnlyOneTab :QStyleOptionToolBox__TabPosition = 3;
pub fn QStyleOptionToolBox_TabPositionItemName(val: i32) ->String {
  match val {
     QStyleOptionToolBox__Beginning => // 0
     {return String::from("Beginning");}
     QStyleOptionToolBox__Middle => // 1
     {return String::from("Middle");}
     QStyleOptionToolBox__End => // 2
     {return String::from("End");}
     QStyleOptionToolBox__OnlyOneTab => // 3
     {return String::from("OnlyOneTab");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionToolBox_TabPositionItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionToolBox
  //return nilthis.TabPositionItemName(val);
  return QStyleOptionToolBox_TabPositionItemName(val);
}


/*


*/
pub type QStyleOptionToolBox__SelectedPosition = i32;
// 
pub const QStyleOptionToolBox__NotAdjacent :QStyleOptionToolBox__SelectedPosition = 0;
// 
pub const QStyleOptionToolBox__NextIsSelected :QStyleOptionToolBox__SelectedPosition = 1;
// 
pub const QStyleOptionToolBox__PreviousIsSelected :QStyleOptionToolBox__SelectedPosition = 2;
pub fn QStyleOptionToolBox_SelectedPositionItemName(val: i32) ->String {
  match val {
     QStyleOptionToolBox__NotAdjacent => // 0
     {return String::from("NotAdjacent");}
     QStyleOptionToolBox__NextIsSelected => // 1
     {return String::from("NextIsSelected");}
     QStyleOptionToolBox__PreviousIsSelected => // 2
     {return String::from("PreviousIsSelected");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionToolBox_SelectedPositionItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionToolBox
  //return nilthis.SelectedPositionItemName(val);
  return QStyleOptionToolBox_SelectedPositionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
