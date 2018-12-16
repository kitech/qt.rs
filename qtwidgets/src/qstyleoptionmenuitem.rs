

// mod ::widgets::QStyleOptionMenuItem
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
#[derive(Default)] // class sizeof(QStyleOptionMenuItem)=136
pub struct QStyleOptionMenuItem {
  qbase: QStyleOption,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStyleOptionMenuItem_ITF interface {
//    QStyleOption_ITF
//    QStyleOptionMenuItem_PTR() *QStyleOptionMenuItem
//}
//func (ptr *QStyleOptionMenuItem) QStyleOptionMenuItem_PTR() *QStyleOptionMenuItem { return ptr }

impl /*struct*/ QStyleOptionMenuItem {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStyleOptionMenuItem {
    return QStyleOptionMenuItem{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStyleOptionMenuItem {
//  type Target = QStyleOptionMenuItemBASE;
//
//  fn deref(&self) -> &QStyleOptionMenuItemBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStyleOptionMenuItemBASE> for QStyleOptionMenuItem {
//  fn as_ref(& self) -> & QStyleOptionMenuItemBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qstyleoption.h:372
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStyleOptionMenuItem()

/*

*/
// QStyleOptionMenuItem() ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionMenuItem {
  pub fn QStyleOptionMenuItem_0<T: QStyleOptionMenuItem_QStyleOptionMenuItem_0>(value: T) -> QStyleOptionMenuItem {
    let rsthis = value.QStyleOptionMenuItem_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionMenuItem_QStyleOptionMenuItem_0 {
  fn QStyleOptionMenuItem_0(self) -> QStyleOptionMenuItem;
}
// QStyleOptionMenuItem() ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionMenuItem_QStyleOptionMenuItem_0 for () {
  fn QStyleOptionMenuItem_0(self) -> QStyleOptionMenuItem {
    // unsafe{_ZN20QStyleOptionMenuItemC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN20QStyleOptionMenuItemC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionMenuItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyleoption.h:376
// index:1
// Protected Visibility=Default Availability=Available
// [-2] void QStyleOptionMenuItem(int)

/*

*/
// QStyleOptionMenuItem(int) ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionMenuItem {
  pub fn QStyleOptionMenuItem_1<T: QStyleOptionMenuItem_QStyleOptionMenuItem_1>(value: T) -> QStyleOptionMenuItem {
    let rsthis = value.QStyleOptionMenuItem_1();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionMenuItem_QStyleOptionMenuItem_1 {
  fn QStyleOptionMenuItem_1(self) -> QStyleOptionMenuItem;
}
// QStyleOptionMenuItem(int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionMenuItem_QStyleOptionMenuItem_1 for (i32) {
  fn QStyleOptionMenuItem_1(self) -> QStyleOptionMenuItem {
    // unsafe{_ZN20QStyleOptionMenuItemC2Ei()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN20QStyleOptionMenuItemC2Ei", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionMenuItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}


pub fn DeleteQStyleOptionMenuItem(this :*mut QStyleOptionMenuItem) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN20QStyleOptionMenuItemD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*
This enum is used to hold information about the type of the style option, and is defined for each QStyleOption subclass.

QStyleOption::TypeSO_DefaultThe type of style option provided (SO_Default for this class).


The type is used internally by QStyleOption, its subclasses, and qstyleoption_cast() to determine the type of style option. In general you do not need to worry about this unless you want to create your own QStyleOption subclass and your own styles.

See also StyleOptionVersion.

*/
pub type QStyleOptionMenuItem__StyleOptionType = i32;
// 
pub const QStyleOptionMenuItem__Type :QStyleOptionMenuItem__StyleOptionType = 4;
pub fn QStyleOptionMenuItem_StyleOptionTypeItemName(val: i32) ->String {
  match val {
     QStyleOptionMenuItem__Type => // 4
     {return String::from("Type");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionMenuItem_StyleOptionTypeItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionMenuItem
  //return nilthis.StyleOptionTypeItemName(val);
  return QStyleOptionMenuItem_StyleOptionTypeItemName(val);
}


/*
This enum is used to hold information about the version of the style option, and is defined for each QStyleOption subclass.



The version is used by QStyleOption subclasses to implement extensions without breaking compatibility. If you use qstyleoption_cast(), you normally do not need to check it.

See also StyleOptionType.

*/
pub type QStyleOptionMenuItem__StyleOptionVersion = i32;
// 1
pub const QStyleOptionMenuItem__Version :QStyleOptionMenuItem__StyleOptionVersion = 1;
pub fn QStyleOptionMenuItem_StyleOptionVersionItemName(val: i32) ->String {
  match val {
     QStyleOptionMenuItem__Version => // 1
     {return String::from("Version");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionMenuItem_StyleOptionVersionItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionMenuItem
  //return nilthis.StyleOptionVersionItemName(val);
  return QStyleOptionMenuItem_StyleOptionVersionItemName(val);
}


/*


*/
pub type QStyleOptionMenuItem__MenuItemType = i32;
// 
pub const QStyleOptionMenuItem__Normal :QStyleOptionMenuItem__MenuItemType = 0;
// 
pub const QStyleOptionMenuItem__DefaultItem :QStyleOptionMenuItem__MenuItemType = 1;
// 
pub const QStyleOptionMenuItem__Separator :QStyleOptionMenuItem__MenuItemType = 2;
// 
pub const QStyleOptionMenuItem__SubMenu :QStyleOptionMenuItem__MenuItemType = 3;
// 
pub const QStyleOptionMenuItem__Scroller :QStyleOptionMenuItem__MenuItemType = 4;
// 
pub const QStyleOptionMenuItem__TearOff :QStyleOptionMenuItem__MenuItemType = 5;
// 
pub const QStyleOptionMenuItem__Margin :QStyleOptionMenuItem__MenuItemType = 6;
// 
pub const QStyleOptionMenuItem__EmptyArea :QStyleOptionMenuItem__MenuItemType = 7;
pub fn QStyleOptionMenuItem_MenuItemTypeItemName(val: i32) ->String {
  match val {
     QStyleOptionMenuItem__Normal => // 0
     {return String::from("Normal");}
     QStyleOptionMenuItem__DefaultItem => // 1
     {return String::from("DefaultItem");}
     QStyleOptionMenuItem__Separator => // 2
     {return String::from("Separator");}
     QStyleOptionMenuItem__SubMenu => // 3
     {return String::from("SubMenu");}
     QStyleOptionMenuItem__Scroller => // 4
     {return String::from("Scroller");}
     QStyleOptionMenuItem__TearOff => // 5
     {return String::from("TearOff");}
     QStyleOptionMenuItem__Margin => // 6
     {return String::from("Margin");}
     QStyleOptionMenuItem__EmptyArea => // 7
     {return String::from("EmptyArea");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionMenuItem_MenuItemTypeItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionMenuItem
  //return nilthis.MenuItemTypeItemName(val);
  return QStyleOptionMenuItem_MenuItemTypeItemName(val);
}


/*


*/
pub type QStyleOptionMenuItem__CheckType = i32;
// 
pub const QStyleOptionMenuItem__NotCheckable :QStyleOptionMenuItem__CheckType = 0;
// 
pub const QStyleOptionMenuItem__Exclusive :QStyleOptionMenuItem__CheckType = 1;
// 
pub const QStyleOptionMenuItem__NonExclusive :QStyleOptionMenuItem__CheckType = 2;
pub fn QStyleOptionMenuItem_CheckTypeItemName(val: i32) ->String {
  match val {
     QStyleOptionMenuItem__NotCheckable => // 0
     {return String::from("NotCheckable");}
     QStyleOptionMenuItem__Exclusive => // 1
     {return String::from("Exclusive");}
     QStyleOptionMenuItem__NonExclusive => // 2
     {return String::from("NonExclusive");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionMenuItem_CheckTypeItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionMenuItem
  //return nilthis.CheckTypeItemName(val);
  return QStyleOptionMenuItem_CheckTypeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
