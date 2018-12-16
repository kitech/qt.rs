

// mod ::widgets::QWhatsThis
// package qtwidgets
// /usr/include/qt/QtWidgets/qwhatsthis.h
// #include <qwhatsthis.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 13
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
#[derive(Default)] // class sizeof(QWhatsThis)=1
pub struct QWhatsThis {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QWhatsThis_ITF interface {
//    QWhatsThis_PTR() *QWhatsThis
//}
//func (ptr *QWhatsThis) QWhatsThis_PTR() *QWhatsThis { return ptr }

impl /*struct*/ QWhatsThis {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QWhatsThis {
    return QWhatsThis{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QWhatsThis {
//  type Target = QWhatsThisBASE;
//
//  fn deref(&self) -> &QWhatsThisBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QWhatsThisBASE> for QWhatsThis {
//  fn as_ref(& self) -> & QWhatsThisBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qwhatsthis.h:58
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void enterWhatsThisMode()

/*
This function switches the user interface into "What's This?" mode. The user interface can be switched back into normal mode by the user (e.g. by them clicking or pressing Esc), or programmatically by calling leaveWhatsThisMode().

When entering "What's This?" mode, a QEvent of type Qt::EnterWhatsThisMode is sent to all toplevel widgets.

See also inWhatsThisMode() and leaveWhatsThisMode().
*/
impl /*struct*/ QWhatsThis {
  pub fn enterWhatsThisMode_0<RetType, T: QWhatsThis_enterWhatsThisMode_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.enterWhatsThisMode_0();
    // return 1;
  }
}
pub trait QWhatsThis_enterWhatsThisMode_0<RetType> {
  fn enterWhatsThisMode_0(self ) -> RetType;
}
impl<'a> /*trait*/ QWhatsThis_enterWhatsThisMode_0<(/*void*/)> for () {
  fn enterWhatsThisMode_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QWhatsThis18enterWhatsThisModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwhatsthis.h:59
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool inWhatsThisMode()

/*
Returns true if the user interface is in "What's This?" mode; otherwise returns false.

See also enterWhatsThisMode().
*/
impl /*struct*/ QWhatsThis {
  pub fn inWhatsThisMode_0<RetType, T: QWhatsThis_inWhatsThisMode_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.inWhatsThisMode_0();
    // return 1;
  }
}
pub trait QWhatsThis_inWhatsThisMode_0<RetType> {
  fn inWhatsThisMode_0(self ) -> RetType;
}
impl<'a> /*trait*/ QWhatsThis_inWhatsThisMode_0<bool> for () {
  fn inWhatsThisMode_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QWhatsThis15inWhatsThisModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwhatsthis.h:60
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void leaveWhatsThisMode()

/*
If the user interface is in "What's This?" mode, this function switches back to normal mode; otherwise it does nothing.

When leaving "What's This?" mode, a QEvent of type Qt::LeaveWhatsThisMode is sent to all toplevel widgets.

See also enterWhatsThisMode() and inWhatsThisMode().
*/
impl /*struct*/ QWhatsThis {
  pub fn leaveWhatsThisMode_0<RetType, T: QWhatsThis_leaveWhatsThisMode_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.leaveWhatsThisMode_0();
    // return 1;
  }
}
pub trait QWhatsThis_leaveWhatsThisMode_0<RetType> {
  fn leaveWhatsThisMode_0(self ) -> RetType;
}
impl<'a> /*trait*/ QWhatsThis_leaveWhatsThisMode_0<(/*void*/)> for () {
  fn leaveWhatsThisMode_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QWhatsThis18leaveWhatsThisModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwhatsthis.h:62
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void showText(const QPoint &, const QString &, QWidget *)

/*
Shows text as a "What's This?" window, at global position pos. The optional widget argument, w, is used to determine the appropriate screen on multi-head systems.

See also hideText().
*/
impl /*struct*/ QWhatsThis {
  pub fn showText_0<RetType, T: QWhatsThis_showText_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.showText_0();
    // return 1;
  }
}
pub trait QWhatsThis_showText_0<RetType> {
  fn showText_0(self ) -> RetType;
}
impl<'a> /*trait*/ QWhatsThis_showText_0<(/*void*/)> for (usize,usize,usize) {
  fn showText_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QWhatsThis8showTextERK6QPointRK7QStringP7QWidget", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwhatsthis.h:63
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void hideText()

/*
If a "What's This?" window is showing, this destroys it.

See also showText().
*/
impl /*struct*/ QWhatsThis {
  pub fn hideText_0<RetType, T: QWhatsThis_hideText_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.hideText_0();
    // return 1;
  }
}
pub trait QWhatsThis_hideText_0<RetType> {
  fn hideText_0(self ) -> RetType;
}
impl<'a> /*trait*/ QWhatsThis_hideText_0<(/*void*/)> for () {
  fn hideText_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QWhatsThis8hideTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwhatsthis.h:65
// index:0
// Public static Visibility=Default Availability=Available
// [8] QAction * createAction(QObject *)

/*
Returns a ready-made QAction, used to invoke "What's This?" context help, with the given parent.

The returned QAction provides a convenient way to let users enter "What's This?" mode.
*/
impl /*struct*/ QWhatsThis {
  pub fn createAction_0<RetType, T: QWhatsThis_createAction_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.createAction_0();
    // return 1;
  }
}
pub trait QWhatsThis_createAction_0<RetType> {
  fn createAction_0(self ) -> RetType;
}
impl<'a> /*trait*/ QWhatsThis_createAction_0<usize> for (usize) {
  fn createAction_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QWhatsThis12createActionEP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQWhatsThis(this :*mut QWhatsThis) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN10QWhatsThisD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
