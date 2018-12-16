

// mod ::widgets::QGraphicsAnchor
// package qtwidgets
// /usr/include/qt/QtWidgets/qgraphicsanchorlayout.h
// #include <qgraphicsanchorlayout.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 15
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
#[derive(Default)] // class sizeof(QGraphicsAnchor)=16
pub struct QGraphicsAnchor {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGraphicsAnchor_ITF interface {
//    qtcore.QObject_ITF
//    QGraphicsAnchor_PTR() *QGraphicsAnchor
//}
//func (ptr *QGraphicsAnchor) QGraphicsAnchor_PTR() *QGraphicsAnchor { return ptr }

impl /*struct*/ QGraphicsAnchor {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGraphicsAnchor {
    return QGraphicsAnchor{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGraphicsAnchor {
//  type Target = QGraphicsAnchorBASE;
//
//  fn deref(&self) -> &QGraphicsAnchorBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGraphicsAnchorBASE> for QGraphicsAnchor {
//  fn as_ref(& self) -> & QGraphicsAnchorBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgraphicsanchorlayout.h:57
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QGraphicsAnchor {
  pub fn metaObject_0<RetType, T: QGraphicsAnchor_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QGraphicsAnchor_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QGraphicsAnchor) -> RetType;
}
impl<'a> /*trait*/ QGraphicsAnchor_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QGraphicsAnchor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGraphicsAnchor10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsanchorlayout.h:61
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSpacing(qreal)

/*
Sets the default horizontal and the default vertical spacing for the anchor layout to spacing.

If an item is anchored with no spacing associated with the anchor, it will use the default spacing.

QGraphicsAnchorLayout does not support negative spacings. Setting a negative value will unset the previous spacing and make the layout use the spacing provided by the current widget style.

See also setHorizontalSpacing() and setVerticalSpacing().
*/
impl /*struct*/ QGraphicsAnchor {
  pub fn setSpacing_0<RetType, T: QGraphicsAnchor_setSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSpacing_0(self);
    // return 1;
  }
}
pub trait QGraphicsAnchor_setSpacing_0<RetType> {
  fn setSpacing_0(self , rsthis: & QGraphicsAnchor) -> RetType;
}
impl<'a> /*trait*/ QGraphicsAnchor_setSpacing_0<(/*void*/)> for (f64) {
  fn setSpacing_0(self , rsthis: & QGraphicsAnchor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsAnchor10setSpacingEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsanchorlayout.h:62
// index:0
// Public Visibility=Default Availability=Available
// [-2] void unsetSpacing()

/*

*/
impl /*struct*/ QGraphicsAnchor {
  pub fn unsetSpacing_0<RetType, T: QGraphicsAnchor_unsetSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unsetSpacing_0(self);
    // return 1;
  }
}
pub trait QGraphicsAnchor_unsetSpacing_0<RetType> {
  fn unsetSpacing_0(self , rsthis: & QGraphicsAnchor) -> RetType;
}
impl<'a> /*trait*/ QGraphicsAnchor_unsetSpacing_0<(/*void*/)> for () {
  fn unsetSpacing_0(self , rsthis: & QGraphicsAnchor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QGraphicsAnchor12unsetSpacingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsanchorlayout.h:63
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal spacing() const

/*

*/
impl /*struct*/ QGraphicsAnchor {
  pub fn spacing_0<RetType, T: QGraphicsAnchor_spacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.spacing_0(self);
    // return 1;
  }
}
pub trait QGraphicsAnchor_spacing_0<RetType> {
  fn spacing_0(self , rsthis: & QGraphicsAnchor) -> RetType;
}
impl<'a> /*trait*/ QGraphicsAnchor_spacing_0<f64> for () {
  fn spacing_0(self , rsthis: & QGraphicsAnchor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGraphicsAnchor7spacingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsanchorlayout.h:64
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSizePolicy(QSizePolicy::Policy)

/*

*/
impl /*struct*/ QGraphicsAnchor {
  pub fn setSizePolicy_0<RetType, T: QGraphicsAnchor_setSizePolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSizePolicy_0(self);
    // return 1;
  }
}
pub trait QGraphicsAnchor_setSizePolicy_0<RetType> {
  fn setSizePolicy_0(self , rsthis: & QGraphicsAnchor) -> RetType;
}
impl<'a> /*trait*/ QGraphicsAnchor_setSizePolicy_0<(/*void*/)> for (i32) {
  fn setSizePolicy_0(self , rsthis: & QGraphicsAnchor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsAnchor13setSizePolicyEN11QSizePolicy6PolicyE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsanchorlayout.h:65
// index:0
// Public Visibility=Default Availability=Available
// [4] QSizePolicy::Policy sizePolicy() const

/*

*/
impl /*struct*/ QGraphicsAnchor {
  pub fn sizePolicy_0<RetType, T: QGraphicsAnchor_sizePolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizePolicy_0(self);
    // return 1;
  }
}
pub trait QGraphicsAnchor_sizePolicy_0<RetType> {
  fn sizePolicy_0(self , rsthis: & QGraphicsAnchor) -> RetType;
}
impl<'a> /*trait*/ QGraphicsAnchor_sizePolicy_0<i32> for () {
  fn sizePolicy_0(self , rsthis: & QGraphicsAnchor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGraphicsAnchor10sizePolicyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsanchorlayout.h:66
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QGraphicsAnchor()

/*

*/
pub fn DeleteQGraphicsAnchor(this :*mut QGraphicsAnchor) {
    // let rv = qtrt::InvokeQtFunc6("_ZN15QGraphicsAnchorD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
