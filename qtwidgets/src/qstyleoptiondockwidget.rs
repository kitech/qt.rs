

// mod ::widgets::QStyleOptionDockWidget
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
#[derive(Default)] // class sizeof(QStyleOptionDockWidget)=80
pub struct QStyleOptionDockWidget {
  qbase: QStyleOption,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStyleOptionDockWidget_ITF interface {
//    QStyleOption_ITF
//    QStyleOptionDockWidget_PTR() *QStyleOptionDockWidget
//}
//func (ptr *QStyleOptionDockWidget) QStyleOptionDockWidget_PTR() *QStyleOptionDockWidget { return ptr }

impl /*struct*/ QStyleOptionDockWidget {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStyleOptionDockWidget {
    return QStyleOptionDockWidget{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStyleOptionDockWidget {
//  type Target = QStyleOptionDockWidgetBASE;
//
//  fn deref(&self) -> &QStyleOptionDockWidgetBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStyleOptionDockWidgetBASE> for QStyleOptionDockWidget {
//  fn as_ref(& self) -> & QStyleOptionDockWidgetBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qstyleoption.h:391
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStyleOptionDockWidget()

/*

*/
// QStyleOptionDockWidget() ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionDockWidget {
  pub fn QStyleOptionDockWidget_0<T: QStyleOptionDockWidget_QStyleOptionDockWidget_0>(value: T) -> QStyleOptionDockWidget {
    let rsthis = value.QStyleOptionDockWidget_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionDockWidget_QStyleOptionDockWidget_0 {
  fn QStyleOptionDockWidget_0(self) -> QStyleOptionDockWidget;
}
// QStyleOptionDockWidget() ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionDockWidget_QStyleOptionDockWidget_0 for () {
  fn QStyleOptionDockWidget_0(self) -> QStyleOptionDockWidget {
    // unsafe{_ZN22QStyleOptionDockWidgetC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN22QStyleOptionDockWidgetC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionDockWidget{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyleoption.h:395
// index:1
// Protected Visibility=Default Availability=Available
// [-2] void QStyleOptionDockWidget(int)

/*

*/
// QStyleOptionDockWidget(int) ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionDockWidget {
  pub fn QStyleOptionDockWidget_1<T: QStyleOptionDockWidget_QStyleOptionDockWidget_1>(value: T) -> QStyleOptionDockWidget {
    let rsthis = value.QStyleOptionDockWidget_1();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionDockWidget_QStyleOptionDockWidget_1 {
  fn QStyleOptionDockWidget_1(self) -> QStyleOptionDockWidget;
}
// QStyleOptionDockWidget(int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionDockWidget_QStyleOptionDockWidget_1 for (i32) {
  fn QStyleOptionDockWidget_1(self) -> QStyleOptionDockWidget {
    // unsafe{_ZN22QStyleOptionDockWidgetC2Ei()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN22QStyleOptionDockWidgetC2Ei", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionDockWidget{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}


pub fn DeleteQStyleOptionDockWidget(this :*mut QStyleOptionDockWidget) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN22QStyleOptionDockWidgetD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*
This enum is used to hold information about the type of the style option, and is defined for each QStyleOption subclass.

QStyleOption::TypeSO_DefaultThe type of style option provided (SO_Default for this class).


The type is used internally by QStyleOption, its subclasses, and qstyleoption_cast() to determine the type of style option. In general you do not need to worry about this unless you want to create your own QStyleOption subclass and your own styles.

See also StyleOptionVersion.

*/
pub type QStyleOptionDockWidget__StyleOptionType = i32;
// 
pub const QStyleOptionDockWidget__Type :QStyleOptionDockWidget__StyleOptionType = 9;
pub fn QStyleOptionDockWidget_StyleOptionTypeItemName(val: i32) ->String {
  match val {
     QStyleOptionDockWidget__Type => // 9
     {return String::from("Type");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionDockWidget_StyleOptionTypeItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionDockWidget
  //return nilthis.StyleOptionTypeItemName(val);
  return QStyleOptionDockWidget_StyleOptionTypeItemName(val);
}


/*
This enum is used to hold information about the version of the style option, and is defined for each QStyleOption subclass.



The version is used by QStyleOption subclasses to implement extensions without breaking compatibility. If you use qstyleoption_cast(), you normally do not need to check it.

See also StyleOptionType.

*/
pub type QStyleOptionDockWidget__StyleOptionVersion = i32;
// 1
pub const QStyleOptionDockWidget__Version :QStyleOptionDockWidget__StyleOptionVersion = 2;
pub fn QStyleOptionDockWidget_StyleOptionVersionItemName(val: i32) ->String {
  match val {
     QStyleOptionDockWidget__Version => // 2
     {return String::from("Version");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionDockWidget_StyleOptionVersionItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionDockWidget
  //return nilthis.StyleOptionVersionItemName(val);
  return QStyleOptionDockWidget_StyleOptionVersionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
