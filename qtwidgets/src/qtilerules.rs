

// mod ::widgets::QTileRules
// package qtwidgets
// /usr/include/qt/QtWidgets/qdrawutil.h
// #include <qdrawutil.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 26
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
#[derive(Default)] // class sizeof(QTileRules)=8
pub struct QTileRules {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTileRules_ITF interface {
//    QTileRules_PTR() *QTileRules
//}
//func (ptr *QTileRules) QTileRules_PTR() *QTileRules { return ptr }

impl /*struct*/ QTileRules {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTileRules {
    return QTileRules{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTileRules {
//  type Target = QTileRulesBASE;
//
//  fn deref(&self) -> &QTileRulesBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTileRulesBASE> for QTileRules {
//  fn as_ref(& self) -> & QTileRulesBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qdrawutil.h:114
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QTileRules(Qt::TileRule, Qt::TileRule)

/*

*/
// QTileRules(Qt::TileRule, Qt::TileRule) ctx.fn_proto_cpp
impl /*struct*/ QTileRules {
  pub fn QTileRules_0<T: QTileRules_QTileRules_0>(value: T) -> QTileRules {
    let rsthis = value.QTileRules_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTileRules_QTileRules_0 {
  fn QTileRules_0(self) -> QTileRules;
}
// QTileRules(Qt::TileRule, Qt::TileRule) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTileRules_QTileRules_0 for (i32,i32) {
  fn QTileRules_0(self) -> QTileRules {
    // unsafe{_ZN10QTileRulesC2EN2Qt8TileRuleES1_()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QTileRulesC2EN2Qt8TileRuleES1_", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTileRules{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdrawutil.h:116
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QTileRules(Qt::TileRule)

/*

*/
// QTileRules(Qt::TileRule) ctx.fn_proto_cpp
impl /*struct*/ QTileRules {
  pub fn QTileRules_1<T: QTileRules_QTileRules_1>(value: T) -> QTileRules {
    let rsthis = value.QTileRules_1();
    return rsthis;
    // return 1;
  }
}

pub trait QTileRules_QTileRules_1 {
  fn QTileRules_1(self) -> QTileRules;
}
// QTileRules(Qt::TileRule) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTileRules_QTileRules_1 for (i32) {
  fn QTileRules_1(self) -> QTileRules {
    // unsafe{_ZN10QTileRulesC2EN2Qt8TileRuleE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QTileRulesC2EN2Qt8TileRuleE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTileRules{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}


pub fn DeleteQTileRules(this :*mut QTileRules) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN10QTileRulesD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
