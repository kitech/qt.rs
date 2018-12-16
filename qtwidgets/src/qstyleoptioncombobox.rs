

// mod ::widgets::QStyleOptionComboBox
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
#[derive(Default)] // class sizeof(QStyleOptionComboBox)=120
pub struct QStyleOptionComboBox {
  qbase: QStyleOptionComplex,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStyleOptionComboBox_ITF interface {
//    QStyleOptionComplex_ITF
//    QStyleOptionComboBox_PTR() *QStyleOptionComboBox
//}
//func (ptr *QStyleOptionComboBox) QStyleOptionComboBox_PTR() *QStyleOptionComboBox { return ptr }

impl /*struct*/ QStyleOptionComboBox {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStyleOptionComboBox {
    return QStyleOptionComboBox{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStyleOptionComboBox {
//  type Target = QStyleOptionComboBoxBASE;
//
//  fn deref(&self) -> &QStyleOptionComboBoxBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStyleOptionComboBoxBASE> for QStyleOptionComboBox {
//  fn as_ref(& self) -> & QStyleOptionComboBoxBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qstyleoption.h:601
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStyleOptionComboBox()

/*

*/
// QStyleOptionComboBox() ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionComboBox {
  pub fn QStyleOptionComboBox_0<T: QStyleOptionComboBox_QStyleOptionComboBox_0>(value: T) -> QStyleOptionComboBox {
    let rsthis = value.QStyleOptionComboBox_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionComboBox_QStyleOptionComboBox_0 {
  fn QStyleOptionComboBox_0(self) -> QStyleOptionComboBox;
}
// QStyleOptionComboBox() ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionComboBox_QStyleOptionComboBox_0 for () {
  fn QStyleOptionComboBox_0(self) -> QStyleOptionComboBox {
    // unsafe{_ZN20QStyleOptionComboBoxC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN20QStyleOptionComboBoxC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionComboBox{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyleoption.h:605
// index:1
// Protected Visibility=Default Availability=Available
// [-2] void QStyleOptionComboBox(int)

/*

*/
// QStyleOptionComboBox(int) ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionComboBox {
  pub fn QStyleOptionComboBox_1<T: QStyleOptionComboBox_QStyleOptionComboBox_1>(value: T) -> QStyleOptionComboBox {
    let rsthis = value.QStyleOptionComboBox_1();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionComboBox_QStyleOptionComboBox_1 {
  fn QStyleOptionComboBox_1(self) -> QStyleOptionComboBox;
}
// QStyleOptionComboBox(int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionComboBox_QStyleOptionComboBox_1 for (i32) {
  fn QStyleOptionComboBox_1(self) -> QStyleOptionComboBox {
    // unsafe{_ZN20QStyleOptionComboBoxC2Ei()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN20QStyleOptionComboBoxC2Ei", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionComboBox{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}


pub fn DeleteQStyleOptionComboBox(this :*mut QStyleOptionComboBox) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN20QStyleOptionComboBoxD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*
This enum is used to hold information about the type of the style option, and is defined for each QStyleOption subclass.

QStyleOption::TypeSO_DefaultThe type of style option provided (SO_Default for this class).


The type is used internally by QStyleOption, its subclasses, and qstyleoption_cast() to determine the type of style option. In general you do not need to worry about this unless you want to create your own QStyleOption subclass and your own styles.

See also StyleOptionVersion.

*/
pub type QStyleOptionComboBox__StyleOptionType = i32;
// 
pub const QStyleOptionComboBox__Type :QStyleOptionComboBox__StyleOptionType = 983044;
pub fn QStyleOptionComboBox_StyleOptionTypeItemName(val: i32) ->String {
  match val {
     QStyleOptionComboBox__Type => // 983044
     {return String::from("Type");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionComboBox_StyleOptionTypeItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionComboBox
  //return nilthis.StyleOptionTypeItemName(val);
  return QStyleOptionComboBox_StyleOptionTypeItemName(val);
}


/*
This enum is used to hold information about the version of the style option, and is defined for each QStyleOption subclass.



The version is used by QStyleOption subclasses to implement extensions without breaking compatibility. If you use qstyleoption_cast(), you normally do not need to check it.

See also StyleOptionType.

*/
pub type QStyleOptionComboBox__StyleOptionVersion = i32;
// 1
pub const QStyleOptionComboBox__Version :QStyleOptionComboBox__StyleOptionVersion = 1;
pub fn QStyleOptionComboBox_StyleOptionVersionItemName(val: i32) ->String {
  match val {
     QStyleOptionComboBox__Version => // 1
     {return String::from("Version");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionComboBox_StyleOptionVersionItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionComboBox
  //return nilthis.StyleOptionVersionItemName(val);
  return QStyleOptionComboBox_StyleOptionVersionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
