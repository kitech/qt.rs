

// mod ::widgets::QStyleOptionViewItem
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
#[derive(Default)] // class sizeof(QStyleOptionViewItem)=192
pub struct QStyleOptionViewItem {
  qbase: QStyleOption,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStyleOptionViewItem_ITF interface {
//    QStyleOption_ITF
//    QStyleOptionViewItem_PTR() *QStyleOptionViewItem
//}
//func (ptr *QStyleOptionViewItem) QStyleOptionViewItem_PTR() *QStyleOptionViewItem { return ptr }

impl /*struct*/ QStyleOptionViewItem {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStyleOptionViewItem {
    return QStyleOptionViewItem{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStyleOptionViewItem {
//  type Target = QStyleOptionViewItemBASE;
//
//  fn deref(&self) -> &QStyleOptionViewItemBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStyleOptionViewItemBASE> for QStyleOptionViewItem {
//  fn as_ref(& self) -> & QStyleOptionViewItemBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qstyleoption.h:442
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStyleOptionViewItem()

/*

*/
// QStyleOptionViewItem() ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionViewItem {
  pub fn QStyleOptionViewItem_0<T: QStyleOptionViewItem_QStyleOptionViewItem_0>(value: T) -> QStyleOptionViewItem {
    let rsthis = value.QStyleOptionViewItem_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionViewItem_QStyleOptionViewItem_0 {
  fn QStyleOptionViewItem_0(self) -> QStyleOptionViewItem;
}
// QStyleOptionViewItem() ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionViewItem_QStyleOptionViewItem_0 for () {
  fn QStyleOptionViewItem_0(self) -> QStyleOptionViewItem {
    // unsafe{_ZN20QStyleOptionViewItemC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN20QStyleOptionViewItemC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionViewItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyleoption.h:446
// index:1
// Protected Visibility=Default Availability=Available
// [-2] void QStyleOptionViewItem(int)

/*

*/
// QStyleOptionViewItem(int) ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionViewItem {
  pub fn QStyleOptionViewItem_1<T: QStyleOptionViewItem_QStyleOptionViewItem_1>(value: T) -> QStyleOptionViewItem {
    let rsthis = value.QStyleOptionViewItem_1();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionViewItem_QStyleOptionViewItem_1 {
  fn QStyleOptionViewItem_1(self) -> QStyleOptionViewItem;
}
// QStyleOptionViewItem(int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionViewItem_QStyleOptionViewItem_1 for (i32) {
  fn QStyleOptionViewItem_1(self) -> QStyleOptionViewItem {
    // unsafe{_ZN20QStyleOptionViewItemC2Ei()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN20QStyleOptionViewItemC2Ei", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionViewItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}


pub fn DeleteQStyleOptionViewItem(this :*mut QStyleOptionViewItem) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN20QStyleOptionViewItemD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*
This enum is used to hold information about the type of the style option, and is defined for each QStyleOption subclass.

QStyleOption::TypeSO_DefaultThe type of style option provided (SO_Default for this class).


The type is used internally by QStyleOption, its subclasses, and qstyleoption_cast() to determine the type of style option. In general you do not need to worry about this unless you want to create your own QStyleOption subclass and your own styles.

See also StyleOptionVersion.

*/
pub type QStyleOptionViewItem__StyleOptionType = i32;
// 
pub const QStyleOptionViewItem__Type :QStyleOptionViewItem__StyleOptionType = 10;
pub fn QStyleOptionViewItem_StyleOptionTypeItemName(val: i32) ->String {
  match val {
     QStyleOptionViewItem__Type => // 10
     {return String::from("Type");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionViewItem_StyleOptionTypeItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionViewItem
  //return nilthis.StyleOptionTypeItemName(val);
  return QStyleOptionViewItem_StyleOptionTypeItemName(val);
}


/*
This enum is used to hold information about the version of the style option, and is defined for each QStyleOption subclass.



The version is used by QStyleOption subclasses to implement extensions without breaking compatibility. If you use qstyleoption_cast(), you normally do not need to check it.

See also StyleOptionType.

*/
pub type QStyleOptionViewItem__StyleOptionVersion = i32;
// 1
pub const QStyleOptionViewItem__Version :QStyleOptionViewItem__StyleOptionVersion = 4;
pub fn QStyleOptionViewItem_StyleOptionVersionItemName(val: i32) ->String {
  match val {
     QStyleOptionViewItem__Version => // 4
     {return String::from("Version");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionViewItem_StyleOptionVersionItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionViewItem
  //return nilthis.StyleOptionVersionItemName(val);
  return QStyleOptionViewItem_StyleOptionVersionItemName(val);
}


/*


*/
pub type QStyleOptionViewItem__Position = i32;
// 
pub const QStyleOptionViewItem__Left :QStyleOptionViewItem__Position = 0;
// 
pub const QStyleOptionViewItem__Right :QStyleOptionViewItem__Position = 1;
// 
pub const QStyleOptionViewItem__Top :QStyleOptionViewItem__Position = 2;
// 
pub const QStyleOptionViewItem__Bottom :QStyleOptionViewItem__Position = 3;
pub fn QStyleOptionViewItem_PositionItemName(val: i32) ->String {
  match val {
     QStyleOptionViewItem__Left => // 0
     {return String::from("Left");}
     QStyleOptionViewItem__Right => // 1
     {return String::from("Right");}
     QStyleOptionViewItem__Top => // 2
     {return String::from("Top");}
     QStyleOptionViewItem__Bottom => // 3
     {return String::from("Bottom");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionViewItem_PositionItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionViewItem
  //return nilthis.PositionItemName(val);
  return QStyleOptionViewItem_PositionItemName(val);
}


/*


*/
pub type QStyleOptionViewItem__ViewItemFeature = i32;
// 
pub const QStyleOptionViewItem__None :QStyleOptionViewItem__ViewItemFeature = 0;
// 
pub const QStyleOptionViewItem__WrapText :QStyleOptionViewItem__ViewItemFeature = 1;
// 
pub const QStyleOptionViewItem__Alternate :QStyleOptionViewItem__ViewItemFeature = 2;
// 
pub const QStyleOptionViewItem__HasCheckIndicator :QStyleOptionViewItem__ViewItemFeature = 4;
// 
pub const QStyleOptionViewItem__HasDisplay :QStyleOptionViewItem__ViewItemFeature = 8;
// 
pub const QStyleOptionViewItem__HasDecoration :QStyleOptionViewItem__ViewItemFeature = 16;
pub fn QStyleOptionViewItem_ViewItemFeatureItemName(val: i32) ->String {
  match val {
     QStyleOptionViewItem__None => // 0
     {return String::from("None");}
     QStyleOptionViewItem__WrapText => // 1
     {return String::from("WrapText");}
     QStyleOptionViewItem__Alternate => // 2
     {return String::from("Alternate");}
     QStyleOptionViewItem__HasCheckIndicator => // 4
     {return String::from("HasCheckIndicator");}
     QStyleOptionViewItem__HasDisplay => // 8
     {return String::from("HasDisplay");}
     QStyleOptionViewItem__HasDecoration => // 16
     {return String::from("HasDecoration");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionViewItem_ViewItemFeatureItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionViewItem
  //return nilthis.ViewItemFeatureItemName(val);
  return QStyleOptionViewItem_ViewItemFeatureItemName(val);
}


/*


*/
pub type QStyleOptionViewItem__ViewItemPosition = i32;
// 
pub const QStyleOptionViewItem__Invalid :QStyleOptionViewItem__ViewItemPosition = 0;
// 
pub const QStyleOptionViewItem__Beginning :QStyleOptionViewItem__ViewItemPosition = 1;
// 
pub const QStyleOptionViewItem__Middle :QStyleOptionViewItem__ViewItemPosition = 2;
// 
pub const QStyleOptionViewItem__End :QStyleOptionViewItem__ViewItemPosition = 3;
// 
pub const QStyleOptionViewItem__OnlyOne :QStyleOptionViewItem__ViewItemPosition = 4;
pub fn QStyleOptionViewItem_ViewItemPositionItemName(val: i32) ->String {
  match val {
     QStyleOptionViewItem__Invalid => // 0
     {return String::from("Invalid");}
     QStyleOptionViewItem__Beginning => // 1
     {return String::from("Beginning");}
     QStyleOptionViewItem__Middle => // 2
     {return String::from("Middle");}
     QStyleOptionViewItem__End => // 3
     {return String::from("End");}
     QStyleOptionViewItem__OnlyOne => // 4
     {return String::from("OnlyOne");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionViewItem_ViewItemPositionItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionViewItem
  //return nilthis.ViewItemPositionItemName(val);
  return QStyleOptionViewItem_ViewItemPositionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
