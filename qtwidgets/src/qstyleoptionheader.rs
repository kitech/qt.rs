

// mod ::widgets::QStyleOptionHeader
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
#[derive(Default)] // class sizeof(QStyleOptionHeader)=120
pub struct QStyleOptionHeader {
  qbase: QStyleOption,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStyleOptionHeader_ITF interface {
//    QStyleOption_ITF
//    QStyleOptionHeader_PTR() *QStyleOptionHeader
//}
//func (ptr *QStyleOptionHeader) QStyleOptionHeader_PTR() *QStyleOptionHeader { return ptr }

impl /*struct*/ QStyleOptionHeader {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStyleOptionHeader {
    return QStyleOptionHeader{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStyleOptionHeader {
//  type Target = QStyleOptionHeaderBASE;
//
//  fn deref(&self) -> &QStyleOptionHeaderBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStyleOptionHeaderBASE> for QStyleOptionHeader {
//  fn as_ref(& self) -> & QStyleOptionHeaderBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qstyleoption.h:226
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStyleOptionHeader()

/*

*/
// QStyleOptionHeader() ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionHeader {
  pub fn QStyleOptionHeader_0<T: QStyleOptionHeader_QStyleOptionHeader_0>(value: T) -> QStyleOptionHeader {
    let rsthis = value.QStyleOptionHeader_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionHeader_QStyleOptionHeader_0 {
  fn QStyleOptionHeader_0(self) -> QStyleOptionHeader;
}
// QStyleOptionHeader() ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionHeader_QStyleOptionHeader_0 for () {
  fn QStyleOptionHeader_0(self) -> QStyleOptionHeader {
    // unsafe{_ZN18QStyleOptionHeaderC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QStyleOptionHeaderC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionHeader{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyleoption.h:230
// index:1
// Protected Visibility=Default Availability=Available
// [-2] void QStyleOptionHeader(int)

/*

*/
// QStyleOptionHeader(int) ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionHeader {
  pub fn QStyleOptionHeader_1<T: QStyleOptionHeader_QStyleOptionHeader_1>(value: T) -> QStyleOptionHeader {
    let rsthis = value.QStyleOptionHeader_1();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionHeader_QStyleOptionHeader_1 {
  fn QStyleOptionHeader_1(self) -> QStyleOptionHeader;
}
// QStyleOptionHeader(int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionHeader_QStyleOptionHeader_1 for (i32) {
  fn QStyleOptionHeader_1(self) -> QStyleOptionHeader {
    // unsafe{_ZN18QStyleOptionHeaderC2Ei()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QStyleOptionHeaderC2Ei", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionHeader{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}


pub fn DeleteQStyleOptionHeader(this :*mut QStyleOptionHeader) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN18QStyleOptionHeaderD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*
This enum is used to hold information about the type of the style option, and is defined for each QStyleOption subclass.

QStyleOption::TypeSO_DefaultThe type of style option provided (SO_Default for this class).


The type is used internally by QStyleOption, its subclasses, and qstyleoption_cast() to determine the type of style option. In general you do not need to worry about this unless you want to create your own QStyleOption subclass and your own styles.

See also StyleOptionVersion.

*/
pub type QStyleOptionHeader__StyleOptionType = i32;
// 
pub const QStyleOptionHeader__Type :QStyleOptionHeader__StyleOptionType = 8;
pub fn QStyleOptionHeader_StyleOptionTypeItemName(val: i32) ->String {
  match val {
     QStyleOptionHeader__Type => // 8
     {return String::from("Type");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionHeader_StyleOptionTypeItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionHeader
  //return nilthis.StyleOptionTypeItemName(val);
  return QStyleOptionHeader_StyleOptionTypeItemName(val);
}


/*
This enum is used to hold information about the version of the style option, and is defined for each QStyleOption subclass.



The version is used by QStyleOption subclasses to implement extensions without breaking compatibility. If you use qstyleoption_cast(), you normally do not need to check it.

See also StyleOptionType.

*/
pub type QStyleOptionHeader__StyleOptionVersion = i32;
// 1
pub const QStyleOptionHeader__Version :QStyleOptionHeader__StyleOptionVersion = 1;
pub fn QStyleOptionHeader_StyleOptionVersionItemName(val: i32) ->String {
  match val {
     QStyleOptionHeader__Version => // 1
     {return String::from("Version");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionHeader_StyleOptionVersionItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionHeader
  //return nilthis.StyleOptionVersionItemName(val);
  return QStyleOptionHeader_StyleOptionVersionItemName(val);
}


/*


*/
pub type QStyleOptionHeader__SectionPosition = i32;
// 
pub const QStyleOptionHeader__Beginning :QStyleOptionHeader__SectionPosition = 0;
// 
pub const QStyleOptionHeader__Middle :QStyleOptionHeader__SectionPosition = 1;
// 
pub const QStyleOptionHeader__End :QStyleOptionHeader__SectionPosition = 2;
// 
pub const QStyleOptionHeader__OnlyOneSection :QStyleOptionHeader__SectionPosition = 3;
pub fn QStyleOptionHeader_SectionPositionItemName(val: i32) ->String {
  match val {
     QStyleOptionHeader__Beginning => // 0
     {return String::from("Beginning");}
     QStyleOptionHeader__Middle => // 1
     {return String::from("Middle");}
     QStyleOptionHeader__End => // 2
     {return String::from("End");}
     QStyleOptionHeader__OnlyOneSection => // 3
     {return String::from("OnlyOneSection");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionHeader_SectionPositionItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionHeader
  //return nilthis.SectionPositionItemName(val);
  return QStyleOptionHeader_SectionPositionItemName(val);
}


/*


*/
pub type QStyleOptionHeader__SelectedPosition = i32;
// 
pub const QStyleOptionHeader__NotAdjacent :QStyleOptionHeader__SelectedPosition = 0;
// 
pub const QStyleOptionHeader__NextIsSelected :QStyleOptionHeader__SelectedPosition = 1;
// 
pub const QStyleOptionHeader__PreviousIsSelected :QStyleOptionHeader__SelectedPosition = 2;
// 
pub const QStyleOptionHeader__NextAndPreviousAreSelected :QStyleOptionHeader__SelectedPosition = 3;
pub fn QStyleOptionHeader_SelectedPositionItemName(val: i32) ->String {
  match val {
     QStyleOptionHeader__NotAdjacent => // 0
     {return String::from("NotAdjacent");}
     QStyleOptionHeader__NextIsSelected => // 1
     {return String::from("NextIsSelected");}
     QStyleOptionHeader__PreviousIsSelected => // 2
     {return String::from("PreviousIsSelected");}
     QStyleOptionHeader__NextAndPreviousAreSelected => // 3
     {return String::from("NextAndPreviousAreSelected");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionHeader_SelectedPositionItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionHeader
  //return nilthis.SelectedPositionItemName(val);
  return QStyleOptionHeader_SelectedPositionItemName(val);
}


/*


*/
pub type QStyleOptionHeader__SortIndicator = i32;
// 
pub const QStyleOptionHeader__None :QStyleOptionHeader__SortIndicator = 0;
// 
pub const QStyleOptionHeader__SortUp :QStyleOptionHeader__SortIndicator = 1;
// 
pub const QStyleOptionHeader__SortDown :QStyleOptionHeader__SortIndicator = 2;
pub fn QStyleOptionHeader_SortIndicatorItemName(val: i32) ->String {
  match val {
     QStyleOptionHeader__None => // 0
     {return String::from("None");}
     QStyleOptionHeader__SortUp => // 1
     {return String::from("SortUp");}
     QStyleOptionHeader__SortDown => // 2
     {return String::from("SortDown");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionHeader_SortIndicatorItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionHeader
  //return nilthis.SortIndicatorItemName(val);
  return QStyleOptionHeader_SortIndicatorItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
