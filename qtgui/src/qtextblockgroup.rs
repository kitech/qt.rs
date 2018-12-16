

// mod ::gui::QTextBlockGroup
// package qtgui
// /usr/include/qt/QtGui/qtextobject.h
// #include <qtextobject.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 8
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin

// void blockInserted(const QTextBlock &)
// func (this *QTextBlockGroup) InheritBlockInserted(f func(block *QTextBlock) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "blockInserted", f)
// }

// void blockRemoved(const QTextBlock &)
// func (this *QTextBlockGroup) InheritBlockRemoved(f func(block *QTextBlock) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "blockRemoved", f)
// }

// void blockFormatChanged(const QTextBlock &)
// func (this *QTextBlockGroup) InheritBlockFormatChanged(f func(block *QTextBlock) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "blockFormatChanged", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QTextBlockGroup)=16
pub struct QTextBlockGroup {
  qbase: QTextObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTextBlockGroup_ITF interface {
//    QTextObject_ITF
//    QTextBlockGroup_PTR() *QTextBlockGroup
//}
//func (ptr *QTextBlockGroup) QTextBlockGroup_PTR() *QTextBlockGroup { return ptr }

impl /*struct*/ QTextBlockGroup {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTextBlockGroup {
    return QTextBlockGroup{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTextBlockGroup {
//  type Target = QTextBlockGroupBASE;
//
//  fn deref(&self) -> &QTextBlockGroupBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTextBlockGroupBASE> for QTextBlockGroup {
//  fn as_ref(& self) -> & QTextBlockGroupBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qtextobject.h:92
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QTextBlockGroup {
  pub fn metaObject_0<RetType, T: QTextBlockGroup_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QTextBlockGroup_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QTextBlockGroup) -> RetType;
}
impl<'a> /*trait*/ QTextBlockGroup_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QTextBlockGroup) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTextBlockGroup10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:95
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void QTextBlockGroup(QTextDocument *)

/*

*/
// QTextBlockGroup(QTextDocument *) ctx.fn_proto_cpp
impl /*struct*/ QTextBlockGroup {
  pub fn QTextBlockGroup_0<T: QTextBlockGroup_QTextBlockGroup_0>(value: T) -> QTextBlockGroup {
    let rsthis = value.QTextBlockGroup_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTextBlockGroup_QTextBlockGroup_0 {
  fn QTextBlockGroup_0(self) -> QTextBlockGroup;
}
// QTextBlockGroup(QTextDocument *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextBlockGroup_QTextBlockGroup_0 for (usize) {
  fn QTextBlockGroup_0(self) -> QTextBlockGroup {
    // unsafe{_ZN15QTextBlockGroupC2EP13QTextDocument()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QTextBlockGroupC2EP13QTextDocument", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextBlockGroup{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:96
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void ~QTextBlockGroup()

/*

*/
pub fn DeleteQTextBlockGroup(this :*mut QTextBlockGroup) {
    // let rv = qtrt::InvokeQtFunc6("_ZN15QTextBlockGroupD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qtextobject.h:98
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void blockInserted(const QTextBlock &)

/*

*/
impl /*struct*/ QTextBlockGroup {
  pub fn blockInserted_0<RetType, T: QTextBlockGroup_blockInserted_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.blockInserted_0(self);
    // return 1;
  }
}
pub trait QTextBlockGroup_blockInserted_0<RetType> {
  fn blockInserted_0(self , rsthis: & QTextBlockGroup) -> RetType;
}
impl<'a> /*trait*/ QTextBlockGroup_blockInserted_0<(/*void*/)> for (usize) {
  fn blockInserted_0(self , rsthis: & QTextBlockGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QTextBlockGroup13blockInsertedERK10QTextBlock", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:99
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void blockRemoved(const QTextBlock &)

/*

*/
impl /*struct*/ QTextBlockGroup {
  pub fn blockRemoved_0<RetType, T: QTextBlockGroup_blockRemoved_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.blockRemoved_0(self);
    // return 1;
  }
}
pub trait QTextBlockGroup_blockRemoved_0<RetType> {
  fn blockRemoved_0(self , rsthis: & QTextBlockGroup) -> RetType;
}
impl<'a> /*trait*/ QTextBlockGroup_blockRemoved_0<(/*void*/)> for (usize) {
  fn blockRemoved_0(self , rsthis: & QTextBlockGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QTextBlockGroup12blockRemovedERK10QTextBlock", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:100
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void blockFormatChanged(const QTextBlock &)

/*

*/
impl /*struct*/ QTextBlockGroup {
  pub fn blockFormatChanged_0<RetType, T: QTextBlockGroup_blockFormatChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.blockFormatChanged_0(self);
    // return 1;
  }
}
pub trait QTextBlockGroup_blockFormatChanged_0<RetType> {
  fn blockFormatChanged_0(self , rsthis: & QTextBlockGroup) -> RetType;
}
impl<'a> /*trait*/ QTextBlockGroup_blockFormatChanged_0<(/*void*/)> for (usize) {
  fn blockFormatChanged_0(self , rsthis: & QTextBlockGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QTextBlockGroup18blockFormatChangedERK10QTextBlock", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
