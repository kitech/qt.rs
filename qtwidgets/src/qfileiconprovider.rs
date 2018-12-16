

// mod ::widgets::QFileIconProvider
// package qtwidgets
// /usr/include/qt/QtWidgets/qfileiconprovider.h
// #include <qfileiconprovider.h>
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
#[derive(Default)] // class sizeof(QFileIconProvider)=16
pub struct QFileIconProvider {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QFileIconProvider_ITF interface {
//    QFileIconProvider_PTR() *QFileIconProvider
//}
//func (ptr *QFileIconProvider) QFileIconProvider_PTR() *QFileIconProvider { return ptr }

impl /*struct*/ QFileIconProvider {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QFileIconProvider {
    return QFileIconProvider{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QFileIconProvider {
//  type Target = QFileIconProviderBASE;
//
//  fn deref(&self) -> &QFileIconProviderBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QFileIconProviderBASE> for QFileIconProvider {
//  fn as_ref(& self) -> & QFileIconProviderBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qfileiconprovider.h:56
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QFileIconProvider()

/*
Constructs a file icon provider.
*/
// QFileIconProvider() ctx.fn_proto_cpp
impl /*struct*/ QFileIconProvider {
  pub fn QFileIconProvider_0<T: QFileIconProvider_QFileIconProvider_0>(value: T) -> QFileIconProvider {
    let rsthis = value.QFileIconProvider_0();
    return rsthis;
    // return 1;
  }
}

pub trait QFileIconProvider_QFileIconProvider_0 {
  fn QFileIconProvider_0(self) -> QFileIconProvider;
}
// QFileIconProvider() ctx.fn_proto_cpp
impl<'a> /*trait*/ QFileIconProvider_QFileIconProvider_0 for () {
  fn QFileIconProvider_0(self) -> QFileIconProvider {
    // unsafe{_ZN17QFileIconProviderC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN17QFileIconProviderC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFileIconProvider{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfileiconprovider.h:57
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QFileIconProvider()

/*

*/
pub fn DeleteQFileIconProvider(this :*mut QFileIconProvider) {
    // let rv = qtrt::InvokeQtFunc6("_ZN17QFileIconProviderD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qfileiconprovider.h:65
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QIcon icon(QFileIconProvider::IconType) const

/*
Returns an icon set for the given type.
*/
impl /*struct*/ QFileIconProvider {
  pub fn icon_0<RetType, T: QFileIconProvider_icon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.icon_0(self);
    // return 1;
  }
}
pub trait QFileIconProvider_icon_0<RetType> {
  fn icon_0(self , rsthis: & QFileIconProvider) -> RetType;
}
impl<'a> /*trait*/ QFileIconProvider_icon_0<usize> for (i32) {
  fn icon_0(self , rsthis: & QFileIconProvider) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QFileIconProvider4iconENS_8IconTypeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfileiconprovider.h:66
// index:1
// Public virtual Visibility=Default Availability=Available
// [8] QIcon icon(const QFileInfo &) const

/*
Returns an icon set for the given type.
*/
impl /*struct*/ QFileIconProvider {
  pub fn icon_1<RetType, T: QFileIconProvider_icon_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.icon_1(self);
    // return 1;
  }
}
pub trait QFileIconProvider_icon_1<RetType> {
  fn icon_1(self , rsthis: & QFileIconProvider) -> RetType;
}
impl<'a> /*trait*/ QFileIconProvider_icon_1<usize> for (usize) {
  fn icon_1(self , rsthis: & QFileIconProvider) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QFileIconProvider4iconERK9QFileInfo", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfileiconprovider.h:67
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QString type(const QFileInfo &) const

/*
Returns the type of the file described by info.
*/
impl /*struct*/ QFileIconProvider {
  pub fn type__0<RetType, T: QFileIconProvider_type__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.type__0(self);
    // return 1;
  }
}
pub trait QFileIconProvider_type__0<RetType> {
  fn type__0(self , rsthis: & QFileIconProvider) -> RetType;
}
impl<'a> /*trait*/ QFileIconProvider_type__0<usize> for (usize) {
  fn type__0(self , rsthis: & QFileIconProvider) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QFileIconProvider4typeERK9QFileInfo", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfileiconprovider.h:69
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOptions(QFileIconProvider::Options)

/*
Sets options that affect the icon provider.

This function was introduced in  Qt 5.2.

See also options().
*/
impl /*struct*/ QFileIconProvider {
  pub fn setOptions_0<RetType, T: QFileIconProvider_setOptions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOptions_0(self);
    // return 1;
  }
}
pub trait QFileIconProvider_setOptions_0<RetType> {
  fn setOptions_0(self , rsthis: & QFileIconProvider) -> RetType;
}
impl<'a> /*trait*/ QFileIconProvider_setOptions_0<(/*void*/)> for (i32) {
  fn setOptions_0(self , rsthis: & QFileIconProvider) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN17QFileIconProvider10setOptionsE6QFlagsINS_6OptionEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfileiconprovider.h:70
// index:0
// Public Visibility=Default Availability=Available
// [4] QFileIconProvider::Options options() const

/*
Returns all the options that affect the icon provider. By default, all options are disabled.

This function was introduced in  Qt 5.2.

See also setOptions().
*/
impl /*struct*/ QFileIconProvider {
  pub fn options_0<RetType, T: QFileIconProvider_options_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.options_0(self);
    // return 1;
  }
}
pub trait QFileIconProvider_options_0<RetType> {
  fn options_0(self , rsthis: & QFileIconProvider) -> RetType;
}
impl<'a> /*trait*/ QFileIconProvider_options_0<i32> for () {
  fn options_0(self , rsthis: & QFileIconProvider) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QFileIconProvider7optionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}


/*
ConstantValue
QFileIconProvider::Computer0
QFileIconProvider::Desktop1
QFileIconProvider::Trashcan2
QFileIconProvider::Network3
QFileIconProvider::Drive4
QFileIconProvider::Folder5
QFileIconProvider::File6

*/
pub type QFileIconProvider__IconType = i32;
// 
pub const QFileIconProvider__Computer :QFileIconProvider__IconType = 0;
// 
pub const QFileIconProvider__Desktop :QFileIconProvider__IconType = 1;
// 
pub const QFileIconProvider__Trashcan :QFileIconProvider__IconType = 2;
// 
pub const QFileIconProvider__Network :QFileIconProvider__IconType = 3;
// 
pub const QFileIconProvider__Drive :QFileIconProvider__IconType = 4;
// 
pub const QFileIconProvider__Folder :QFileIconProvider__IconType = 5;
// 
pub const QFileIconProvider__File :QFileIconProvider__IconType = 6;
pub fn QFileIconProvider_IconTypeItemName(val: i32) ->String {
  match val {
     QFileIconProvider__Computer => // 0
     {return String::from("Computer");}
     QFileIconProvider__Desktop => // 1
     {return String::from("Desktop");}
     QFileIconProvider__Trashcan => // 2
     {return String::from("Trashcan");}
     QFileIconProvider__Network => // 3
     {return String::from("Network");}
     QFileIconProvider__Drive => // 4
     {return String::from("Drive");}
     QFileIconProvider__Folder => // 5
     {return String::from("Folder");}
     QFileIconProvider__File => // 6
     {return String::from("File");}
  _ => {return format!("{}", val);}
}
}
pub fn QFileIconProvider_IconTypeItemName_s(val: i32) ->String {
  //var nilthis *QFileIconProvider
  //return nilthis.IconTypeItemName(val);
  return QFileIconProvider_IconTypeItemName(val);
}


/*


*/
pub type QFileIconProvider__Option = i32;
// 
pub const QFileIconProvider__DontUseCustomDirectoryIcons :QFileIconProvider__Option = 1;
pub fn QFileIconProvider_OptionItemName(val: i32) ->String {
  match val {
     QFileIconProvider__DontUseCustomDirectoryIcons => // 1
     {return String::from("DontUseCustomDirectoryIcons");}
  _ => {return format!("{}", val);}
}
}
pub fn QFileIconProvider_OptionItemName_s(val: i32) ->String {
  //var nilthis *QFileIconProvider
  //return nilthis.OptionItemName(val);
  return QFileIconProvider_OptionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
