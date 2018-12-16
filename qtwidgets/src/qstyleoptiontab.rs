

// mod ::widgets::QStyleOptionTab
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
#[derive(Default)] // class sizeof(QStyleOptionTab)=136
pub struct QStyleOptionTab {
  qbase: QStyleOption,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStyleOptionTab_ITF interface {
//    QStyleOption_ITF
//    QStyleOptionTab_PTR() *QStyleOptionTab
//}
//func (ptr *QStyleOptionTab) QStyleOptionTab_PTR() *QStyleOptionTab { return ptr }

impl /*struct*/ QStyleOptionTab {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStyleOptionTab {
    return QStyleOptionTab{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStyleOptionTab {
//  type Target = QStyleOptionTabBASE;
//
//  fn deref(&self) -> &QStyleOptionTabBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStyleOptionTabBASE> for QStyleOptionTab {
//  fn as_ref(& self) -> & QStyleOptionTabBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qstyleoption.h:285
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStyleOptionTab()

/*

*/
// QStyleOptionTab() ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionTab {
  pub fn QStyleOptionTab_0<T: QStyleOptionTab_QStyleOptionTab_0>(value: T) -> QStyleOptionTab {
    let rsthis = value.QStyleOptionTab_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionTab_QStyleOptionTab_0 {
  fn QStyleOptionTab_0(self) -> QStyleOptionTab;
}
// QStyleOptionTab() ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionTab_QStyleOptionTab_0 for () {
  fn QStyleOptionTab_0(self) -> QStyleOptionTab {
    // unsafe{_ZN15QStyleOptionTabC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QStyleOptionTabC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionTab{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyleoption.h:289
// index:1
// Protected Visibility=Default Availability=Available
// [-2] void QStyleOptionTab(int)

/*

*/
// QStyleOptionTab(int) ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionTab {
  pub fn QStyleOptionTab_1<T: QStyleOptionTab_QStyleOptionTab_1>(value: T) -> QStyleOptionTab {
    let rsthis = value.QStyleOptionTab_1();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionTab_QStyleOptionTab_1 {
  fn QStyleOptionTab_1(self) -> QStyleOptionTab;
}
// QStyleOptionTab(int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionTab_QStyleOptionTab_1 for (i32) {
  fn QStyleOptionTab_1(self) -> QStyleOptionTab {
    // unsafe{_ZN15QStyleOptionTabC2Ei()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QStyleOptionTabC2Ei", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionTab{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}


pub fn DeleteQStyleOptionTab(this :*mut QStyleOptionTab) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN15QStyleOptionTabD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*
This enum is used to hold information about the type of the style option, and is defined for each QStyleOption subclass.

QStyleOption::TypeSO_DefaultThe type of style option provided (SO_Default for this class).


The type is used internally by QStyleOption, its subclasses, and qstyleoption_cast() to determine the type of style option. In general you do not need to worry about this unless you want to create your own QStyleOption subclass and your own styles.

See also StyleOptionVersion.

*/
pub type QStyleOptionTab__StyleOptionType = i32;
// 
pub const QStyleOptionTab__Type :QStyleOptionTab__StyleOptionType = 3;
pub fn QStyleOptionTab_StyleOptionTypeItemName(val: i32) ->String {
  match val {
     QStyleOptionTab__Type => // 3
     {return String::from("Type");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionTab_StyleOptionTypeItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionTab
  //return nilthis.StyleOptionTypeItemName(val);
  return QStyleOptionTab_StyleOptionTypeItemName(val);
}


/*
This enum is used to hold information about the version of the style option, and is defined for each QStyleOption subclass.



The version is used by QStyleOption subclasses to implement extensions without breaking compatibility. If you use qstyleoption_cast(), you normally do not need to check it.

See also StyleOptionType.

*/
pub type QStyleOptionTab__StyleOptionVersion = i32;
// 1
pub const QStyleOptionTab__Version :QStyleOptionTab__StyleOptionVersion = 3;
pub fn QStyleOptionTab_StyleOptionVersionItemName(val: i32) ->String {
  match val {
     QStyleOptionTab__Version => // 3
     {return String::from("Version");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionTab_StyleOptionVersionItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionTab
  //return nilthis.StyleOptionVersionItemName(val);
  return QStyleOptionTab_StyleOptionVersionItemName(val);
}


/*


*/
pub type QStyleOptionTab__TabPosition = i32;
// 
pub const QStyleOptionTab__Beginning :QStyleOptionTab__TabPosition = 0;
// 
pub const QStyleOptionTab__Middle :QStyleOptionTab__TabPosition = 1;
// 
pub const QStyleOptionTab__End :QStyleOptionTab__TabPosition = 2;
// 
pub const QStyleOptionTab__OnlyOneTab :QStyleOptionTab__TabPosition = 3;
pub fn QStyleOptionTab_TabPositionItemName(val: i32) ->String {
  match val {
     QStyleOptionTab__Beginning => // 0
     {return String::from("Beginning");}
     QStyleOptionTab__Middle => // 1
     {return String::from("Middle");}
     QStyleOptionTab__End => // 2
     {return String::from("End");}
     QStyleOptionTab__OnlyOneTab => // 3
     {return String::from("OnlyOneTab");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionTab_TabPositionItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionTab
  //return nilthis.TabPositionItemName(val);
  return QStyleOptionTab_TabPositionItemName(val);
}


/*


*/
pub type QStyleOptionTab__SelectedPosition = i32;
// 
pub const QStyleOptionTab__NotAdjacent :QStyleOptionTab__SelectedPosition = 0;
// 
pub const QStyleOptionTab__NextIsSelected :QStyleOptionTab__SelectedPosition = 1;
// 
pub const QStyleOptionTab__PreviousIsSelected :QStyleOptionTab__SelectedPosition = 2;
pub fn QStyleOptionTab_SelectedPositionItemName(val: i32) ->String {
  match val {
     QStyleOptionTab__NotAdjacent => // 0
     {return String::from("NotAdjacent");}
     QStyleOptionTab__NextIsSelected => // 1
     {return String::from("NextIsSelected");}
     QStyleOptionTab__PreviousIsSelected => // 2
     {return String::from("PreviousIsSelected");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionTab_SelectedPositionItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionTab
  //return nilthis.SelectedPositionItemName(val);
  return QStyleOptionTab_SelectedPositionItemName(val);
}


/*


*/
pub type QStyleOptionTab__CornerWidget = i32;
// 
pub const QStyleOptionTab__NoCornerWidgets :QStyleOptionTab__CornerWidget = 0;
// 
pub const QStyleOptionTab__LeftCornerWidget :QStyleOptionTab__CornerWidget = 1;
// 
pub const QStyleOptionTab__RightCornerWidget :QStyleOptionTab__CornerWidget = 2;
pub fn QStyleOptionTab_CornerWidgetItemName(val: i32) ->String {
  match val {
     QStyleOptionTab__NoCornerWidgets => // 0
     {return String::from("NoCornerWidgets");}
     QStyleOptionTab__LeftCornerWidget => // 1
     {return String::from("LeftCornerWidget");}
     QStyleOptionTab__RightCornerWidget => // 2
     {return String::from("RightCornerWidget");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionTab_CornerWidgetItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionTab
  //return nilthis.CornerWidgetItemName(val);
  return QStyleOptionTab_CornerWidgetItemName(val);
}


/*


*/
pub type QStyleOptionTab__TabFeature = i32;
// 
pub const QStyleOptionTab__None :QStyleOptionTab__TabFeature = 0;
// 
pub const QStyleOptionTab__HasFrame :QStyleOptionTab__TabFeature = 1;
pub fn QStyleOptionTab_TabFeatureItemName(val: i32) ->String {
  match val {
     QStyleOptionTab__None => // 0
     {return String::from("None");}
     QStyleOptionTab__HasFrame => // 1
     {return String::from("HasFrame");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionTab_TabFeatureItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionTab
  //return nilthis.TabFeatureItemName(val);
  return QStyleOptionTab_TabFeatureItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
