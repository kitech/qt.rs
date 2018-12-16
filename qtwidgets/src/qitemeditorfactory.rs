

// mod ::widgets::QItemEditorFactory
// package qtwidgets
// /usr/include/qt/QtWidgets/qitemeditorfactory.h
// #include <qitemeditorfactory.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 3
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
#[derive(Default)] // class sizeof(QItemEditorFactory)=16
pub struct QItemEditorFactory {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QItemEditorFactory_ITF interface {
//    QItemEditorFactory_PTR() *QItemEditorFactory
//}
//func (ptr *QItemEditorFactory) QItemEditorFactory_PTR() *QItemEditorFactory { return ptr }

impl /*struct*/ QItemEditorFactory {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QItemEditorFactory {
    return QItemEditorFactory{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QItemEditorFactory {
//  type Target = QItemEditorFactoryBASE;
//
//  fn deref(&self) -> &QItemEditorFactoryBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QItemEditorFactoryBASE> for QItemEditorFactory {
//  fn as_ref(& self) -> & QItemEditorFactoryBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qitemeditorfactory.h:98
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QItemEditorFactory()

/*
Constructs a new item editor factory.
*/
// QItemEditorFactory() ctx.fn_proto_cpp
impl /*struct*/ QItemEditorFactory {
  pub fn QItemEditorFactory_0<T: QItemEditorFactory_QItemEditorFactory_0>(value: T) -> QItemEditorFactory {
    let rsthis = value.QItemEditorFactory_0();
    return rsthis;
    // return 1;
  }
}

pub trait QItemEditorFactory_QItemEditorFactory_0 {
  fn QItemEditorFactory_0(self) -> QItemEditorFactory;
}
// QItemEditorFactory() ctx.fn_proto_cpp
impl<'a> /*trait*/ QItemEditorFactory_QItemEditorFactory_0 for () {
  fn QItemEditorFactory_0(self) -> QItemEditorFactory {
    // unsafe{_ZN18QItemEditorFactoryC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QItemEditorFactoryC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QItemEditorFactory{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qitemeditorfactory.h:99
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QItemEditorFactory()

/*

*/
pub fn DeleteQItemEditorFactory(this :*mut QItemEditorFactory) {
    // let rv = qtrt::InvokeQtFunc6("_ZN18QItemEditorFactoryD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qitemeditorfactory.h:101
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QWidget * createEditor(int, QWidget *) const

/*
Creates an editor widget with the given parent for the specified userType of data, and returns it as a QWidget.

See also registerEditor().
*/
impl /*struct*/ QItemEditorFactory {
  pub fn createEditor_0<RetType, T: QItemEditorFactory_createEditor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.createEditor_0(self);
    // return 1;
  }
}
pub trait QItemEditorFactory_createEditor_0<RetType> {
  fn createEditor_0(self , rsthis: & QItemEditorFactory) -> RetType;
}
impl<'a> /*trait*/ QItemEditorFactory_createEditor_0<usize> for (i32,usize) {
  fn createEditor_0(self , rsthis: & QItemEditorFactory) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QItemEditorFactory12createEditorEiP7QWidget", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qitemeditorfactory.h:102
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QByteArray valuePropertyName(int) const

/*
Returns the property name used to access data for the given userType of data.
*/
impl /*struct*/ QItemEditorFactory {
  pub fn valuePropertyName_0<RetType, T: QItemEditorFactory_valuePropertyName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.valuePropertyName_0(self);
    // return 1;
  }
}
pub trait QItemEditorFactory_valuePropertyName_0<RetType> {
  fn valuePropertyName_0(self , rsthis: & QItemEditorFactory) -> RetType;
}
impl<'a> /*trait*/ QItemEditorFactory_valuePropertyName_0<usize> for (i32) {
  fn valuePropertyName_0(self , rsthis: & QItemEditorFactory) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QItemEditorFactory17valuePropertyNameEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qitemeditorfactory.h:104
// index:0
// Public Visibility=Default Availability=Available
// [-2] void registerEditor(int, QItemEditorCreatorBase *)

/*
Registers an item editor creator specified by creator for the given userType of data.

Note: The factory takes ownership of the item editor creator and will destroy it if a new creator for the same type is registered later.

See also createEditor().
*/
impl /*struct*/ QItemEditorFactory {
  pub fn registerEditor_0<RetType, T: QItemEditorFactory_registerEditor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.registerEditor_0(self);
    // return 1;
  }
}
pub trait QItemEditorFactory_registerEditor_0<RetType> {
  fn registerEditor_0(self , rsthis: & QItemEditorFactory) -> RetType;
}
impl<'a> /*trait*/ QItemEditorFactory_registerEditor_0<(/*void*/)> for (i32,usize) {
  fn registerEditor_0(self , rsthis: & QItemEditorFactory) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QItemEditorFactory14registerEditorEiP22QItemEditorCreatorBase", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qitemeditorfactory.h:106
// index:0
// Public static Visibility=Default Availability=Available
// [8] const QItemEditorFactory * defaultFactory()

/*
Returns the default item editor factory.

See also setDefaultFactory().
*/
impl /*struct*/ QItemEditorFactory {
  pub fn defaultFactory_0<RetType, T: QItemEditorFactory_defaultFactory_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.defaultFactory_0();
    // return 1;
  }
}
pub trait QItemEditorFactory_defaultFactory_0<RetType> {
  fn defaultFactory_0(self ) -> RetType;
}
impl<'a> /*trait*/ QItemEditorFactory_defaultFactory_0<usize> for () {
  fn defaultFactory_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QItemEditorFactory14defaultFactoryEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qitemeditorfactory.h:107
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setDefaultFactory(QItemEditorFactory *)

/*
Sets the default item editor factory to the given factory. Both new and existing delegates will use the new factory.

See also defaultFactory().
*/
impl /*struct*/ QItemEditorFactory {
  pub fn setDefaultFactory_0<RetType, T: QItemEditorFactory_setDefaultFactory_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setDefaultFactory_0();
    // return 1;
  }
}
pub trait QItemEditorFactory_setDefaultFactory_0<RetType> {
  fn setDefaultFactory_0(self ) -> RetType;
}
impl<'a> /*trait*/ QItemEditorFactory_setDefaultFactory_0<(/*void*/)> for (usize) {
  fn setDefaultFactory_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QItemEditorFactory17setDefaultFactoryEPS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
