

// mod ::widgets::QGraphicsTransform
// package qtwidgets
// /usr/include/qt/QtWidgets/qgraphicstransform.h
// #include <qgraphicstransform.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 6
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

// void update()
// func (this *QGraphicsTransform) InheritUpdate(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "update", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QGraphicsTransform)=16
pub struct QGraphicsTransform {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGraphicsTransform_ITF interface {
//    qtcore.QObject_ITF
//    QGraphicsTransform_PTR() *QGraphicsTransform
//}
//func (ptr *QGraphicsTransform) QGraphicsTransform_PTR() *QGraphicsTransform { return ptr }

impl /*struct*/ QGraphicsTransform {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGraphicsTransform {
    return QGraphicsTransform{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGraphicsTransform {
//  type Target = QGraphicsTransformBASE;
//
//  fn deref(&self) -> &QGraphicsTransformBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGraphicsTransformBASE> for QGraphicsTransform {
//  fn as_ref(& self) -> & QGraphicsTransformBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgraphicstransform.h:58
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QGraphicsTransform {
  pub fn metaObject_0<RetType, T: QGraphicsTransform_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QGraphicsTransform_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QGraphicsTransform) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTransform_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QGraphicsTransform) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QGraphicsTransform10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicstransform.h:60
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QGraphicsTransform(QObject *)

/*
Constructs a new QGraphicsTransform with the given parent.
*/
// QGraphicsTransform(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QGraphicsTransform {
  pub fn QGraphicsTransform_0<T: QGraphicsTransform_QGraphicsTransform_0>(value: T) -> QGraphicsTransform {
    let rsthis = value.QGraphicsTransform_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsTransform_QGraphicsTransform_0 {
  fn QGraphicsTransform_0(self) -> QGraphicsTransform;
}
// QGraphicsTransform(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGraphicsTransform_QGraphicsTransform_0 for (usize) {
  fn QGraphicsTransform_0(self) -> QGraphicsTransform {
    // unsafe{_ZN18QGraphicsTransformC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QGraphicsTransformC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGraphicsTransform{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicstransform.h:61
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QGraphicsTransform()

/*

*/
pub fn DeleteQGraphicsTransform(this :*mut QGraphicsTransform) {
    // let rv = qtrt::InvokeQtFunc6("_ZN18QGraphicsTransformD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgraphicstransform.h:63
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [-2] void applyTo(QMatrix4x4 *) const

/*
This pure virtual method has to be reimplemented in derived classes.

It applies this transformation to matrix.

See also QGraphicsItem::transform() and QMatrix4x4::toTransform().
*/
impl /*struct*/ QGraphicsTransform {
  pub fn applyTo_0<RetType, T: QGraphicsTransform_applyTo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.applyTo_0(self);
    // return 1;
  }
}
pub trait QGraphicsTransform_applyTo_0<RetType> {
  fn applyTo_0(self , rsthis: & QGraphicsTransform) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTransform_applyTo_0<(/*void*/)> for (usize) {
  fn applyTo_0(self , rsthis: & QGraphicsTransform) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK18QGraphicsTransform7applyToEP10QMatrix4x4", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicstransform.h:66
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void update()

/*
Notifies that this transform operation has changed its parameters in such a way that applyTo() will return a different result than before.

When implementing you own custom graphics transform, you must call this function every time you change a parameter, to let QGraphicsItem know that its transformation needs to be updated.

See also applyTo().
*/
impl /*struct*/ QGraphicsTransform {
  pub fn update_0<RetType, T: QGraphicsTransform_update_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.update_0(self);
    // return 1;
  }
}
pub trait QGraphicsTransform_update_0<RetType> {
  fn update_0(self , rsthis: & QGraphicsTransform) -> RetType;
}
impl<'a> /*trait*/ QGraphicsTransform_update_0<(/*void*/)> for () {
  fn update_0(self , rsthis: & QGraphicsTransform) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN18QGraphicsTransform6updateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
