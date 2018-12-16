

// mod ::core::QCommandLineOption
// package qtcore
// /usr/include/qt/QtCore/qcommandlineoption.h
// #include <qcommandlineoption.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 18
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
use std::default::Default;
use std::ops::Deref;
use super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QCommandLineOption)=8
pub struct QCommandLineOption {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QCommandLineOption_ITF interface {
//    QCommandLineOption_PTR() *QCommandLineOption
//}
//func (ptr *QCommandLineOption) QCommandLineOption_PTR() *QCommandLineOption { return ptr }

impl /*struct*/ QCommandLineOption {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QCommandLineOption {
    return QCommandLineOption{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QCommandLineOption {
//  type Target = QCommandLineOptionBASE;
//
//  fn deref(&self) -> &QCommandLineOptionBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QCommandLineOptionBASE> for QCommandLineOption {
//  fn as_ref(& self) -> & QCommandLineOptionBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qcommandlineoption.h:61
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QCommandLineOption(const QString &)

/*
Constructs a command line option object with the name name.

The name can be either short or long. If the name is one character in length, it is considered a short name. Option names must not be empty, must not start with a dash or a slash character, must not contain a = and cannot be repeated.

See also setDescription(), setValueName(), and setDefaultValues().
*/
// QCommandLineOption(const QString &) ctx.fn_proto_cpp
impl /*struct*/ QCommandLineOption {
  pub fn QCommandLineOption_0<T: QCommandLineOption_QCommandLineOption_0>(value: T) -> QCommandLineOption {
    let rsthis = value.QCommandLineOption_0();
    return rsthis;
    // return 1;
  }
}

pub trait QCommandLineOption_QCommandLineOption_0 {
  fn QCommandLineOption_0(self) -> QCommandLineOption;
}
// QCommandLineOption(const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QCommandLineOption_QCommandLineOption_0 for (usize) {
  fn QCommandLineOption_0(self) -> QCommandLineOption {
    // unsafe{_ZN18QCommandLineOptionC2ERK7QString()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QCommandLineOptionC2ERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QCommandLineOption{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcommandlineoption.h:62
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QCommandLineOption(const QStringList &)

/*
Constructs a command line option object with the name name.

The name can be either short or long. If the name is one character in length, it is considered a short name. Option names must not be empty, must not start with a dash or a slash character, must not contain a = and cannot be repeated.

See also setDescription(), setValueName(), and setDefaultValues().
*/
// QCommandLineOption(const QStringList &) ctx.fn_proto_cpp
impl /*struct*/ QCommandLineOption {
  pub fn QCommandLineOption_1<T: QCommandLineOption_QCommandLineOption_1>(value: T) -> QCommandLineOption {
    let rsthis = value.QCommandLineOption_1();
    return rsthis;
    // return 1;
  }
}

pub trait QCommandLineOption_QCommandLineOption_1 {
  fn QCommandLineOption_1(self) -> QCommandLineOption;
}
// QCommandLineOption(const QStringList &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QCommandLineOption_QCommandLineOption_1 for (usize) {
  fn QCommandLineOption_1(self) -> QCommandLineOption {
    // unsafe{_ZN18QCommandLineOptionC2ERK11QStringList()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QCommandLineOptionC2ERK11QStringList", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QCommandLineOption{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcommandlineoption.h:63
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QCommandLineOption(const QString &, const QString &, const QString &, const QString &)

/*
Constructs a command line option object with the name name.

The name can be either short or long. If the name is one character in length, it is considered a short name. Option names must not be empty, must not start with a dash or a slash character, must not contain a = and cannot be repeated.

See also setDescription(), setValueName(), and setDefaultValues().
*/
// QCommandLineOption(const QString &, const QString &, const QString &, const QString &) ctx.fn_proto_cpp
impl /*struct*/ QCommandLineOption {
  pub fn QCommandLineOption_2<T: QCommandLineOption_QCommandLineOption_2>(value: T) -> QCommandLineOption {
    let rsthis = value.QCommandLineOption_2();
    return rsthis;
    // return 1;
  }
}

pub trait QCommandLineOption_QCommandLineOption_2 {
  fn QCommandLineOption_2(self) -> QCommandLineOption;
}
// QCommandLineOption(const QString &, const QString &, const QString &, const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QCommandLineOption_QCommandLineOption_2 for (usize,usize,usize,usize) {
  fn QCommandLineOption_2(self) -> QCommandLineOption {
    // unsafe{_ZN18QCommandLineOptionC2ERK7QStringS2_S2_S2_()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QCommandLineOptionC2ERK7QStringS2_S2_S2_", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QCommandLineOption{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcommandlineoption.h:66
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QCommandLineOption(const QStringList &, const QString &, const QString &, const QString &)

/*
Constructs a command line option object with the name name.

The name can be either short or long. If the name is one character in length, it is considered a short name. Option names must not be empty, must not start with a dash or a slash character, must not contain a = and cannot be repeated.

See also setDescription(), setValueName(), and setDefaultValues().
*/
// QCommandLineOption(const QStringList &, const QString &, const QString &, const QString &) ctx.fn_proto_cpp
impl /*struct*/ QCommandLineOption {
  pub fn QCommandLineOption_3<T: QCommandLineOption_QCommandLineOption_3>(value: T) -> QCommandLineOption {
    let rsthis = value.QCommandLineOption_3();
    return rsthis;
    // return 1;
  }
}

pub trait QCommandLineOption_QCommandLineOption_3 {
  fn QCommandLineOption_3(self) -> QCommandLineOption;
}
// QCommandLineOption(const QStringList &, const QString &, const QString &, const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QCommandLineOption_QCommandLineOption_3 for (usize,usize,usize,usize) {
  fn QCommandLineOption_3(self) -> QCommandLineOption {
    // unsafe{_ZN18QCommandLineOptionC2ERK11QStringListRK7QStringS5_S5_()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QCommandLineOptionC2ERK11QStringListRK7QStringS5_S5_", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QCommandLineOption{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcommandlineoption.h:71
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QCommandLineOption()

/*

*/
pub fn DeleteQCommandLineOption(this :*mut QCommandLineOption) {
    // let rv = qtrt::InvokeQtFunc6("_ZN18QCommandLineOptionD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qcommandlineoption.h:73
// index:0
// Public Visibility=Default Availability=Available
// [8] QCommandLineOption & operator=(const QCommandLineOption &)

/*

*/
impl /*struct*/ QCommandLineOption {
  pub fn operator_equal_0<RetType, T: QCommandLineOption_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QCommandLineOption_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QCommandLineOption) -> RetType;
}
impl<'a> /*trait*/ QCommandLineOption_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QCommandLineOption) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QCommandLineOptionaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcommandlineoption.h:75
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QCommandLineOption & operator=(QCommandLineOption &&)

/*

*/
impl /*struct*/ QCommandLineOption {
  pub fn operator_equal_1<RetType, T: QCommandLineOption_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QCommandLineOption_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QCommandLineOption) -> RetType;
}
impl<'a> /*trait*/ QCommandLineOption_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QCommandLineOption) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QCommandLineOptionaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcommandlineoption.h:78
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QCommandLineOption &)

/*
Swaps option other with this option. This operation is very fast and never fails.
*/
impl /*struct*/ QCommandLineOption {
  pub fn swap_0<RetType, T: QCommandLineOption_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QCommandLineOption_swap_0<RetType> {
  fn swap_0(self , rsthis: & QCommandLineOption) -> RetType;
}
impl<'a> /*trait*/ QCommandLineOption_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QCommandLineOption) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QCommandLineOption4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcommandlineoption.h:81
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList names() const

/*
Returns the names set for this option.
*/
impl /*struct*/ QCommandLineOption {
  pub fn names_0<RetType, T: QCommandLineOption_names_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.names_0(self);
    // return 1;
  }
}
pub trait QCommandLineOption_names_0<RetType> {
  fn names_0(self , rsthis: & QCommandLineOption) -> RetType;
}
impl<'a> /*trait*/ QCommandLineOption_names_0<usize> for () {
  fn names_0(self , rsthis: & QCommandLineOption) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QCommandLineOption5namesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcommandlineoption.h:83
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setValueName(const QString &)

/*
Sets the name of the expected value, for the documentation, to valueName.

Options without a value assigned have a boolean-like behavior: either the user specifies --option or they don't.

Options with a value assigned need to set a name for the expected value, for the documentation of the option in the help output. An option with names o and output, and a value name of file will appear as -o, --output <file>.

Call QCommandLineParser::value() if you expect the option to be present only once, and QCommandLineParser::values() if you expect that option to be present multiple times.

See also valueName().
*/
impl /*struct*/ QCommandLineOption {
  pub fn setValueName_0<RetType, T: QCommandLineOption_setValueName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setValueName_0(self);
    // return 1;
  }
}
pub trait QCommandLineOption_setValueName_0<RetType> {
  fn setValueName_0(self , rsthis: & QCommandLineOption) -> RetType;
}
impl<'a> /*trait*/ QCommandLineOption_setValueName_0<(/*void*/)> for (usize) {
  fn setValueName_0(self , rsthis: & QCommandLineOption) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QCommandLineOption12setValueNameERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcommandlineoption.h:84
// index:0
// Public Visibility=Default Availability=Available
// [8] QString valueName() const

/*
Returns the name of the expected value.

If empty, the option doesn't take a value.

See also setValueName().
*/
impl /*struct*/ QCommandLineOption {
  pub fn valueName_0<RetType, T: QCommandLineOption_valueName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.valueName_0(self);
    // return 1;
  }
}
pub trait QCommandLineOption_valueName_0<RetType> {
  fn valueName_0(self , rsthis: & QCommandLineOption) -> RetType;
}
impl<'a> /*trait*/ QCommandLineOption_valueName_0<usize> for () {
  fn valueName_0(self , rsthis: & QCommandLineOption) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QCommandLineOption9valueNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcommandlineoption.h:86
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDescription(const QString &)

/*
Sets the description used for this option to description.

It is customary to add a "." at the end of the description.

The description is used by QCommandLineParser::showHelp().

See also description().
*/
impl /*struct*/ QCommandLineOption {
  pub fn setDescription_0<RetType, T: QCommandLineOption_setDescription_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDescription_0(self);
    // return 1;
  }
}
pub trait QCommandLineOption_setDescription_0<RetType> {
  fn setDescription_0(self , rsthis: & QCommandLineOption) -> RetType;
}
impl<'a> /*trait*/ QCommandLineOption_setDescription_0<(/*void*/)> for (usize) {
  fn setDescription_0(self , rsthis: & QCommandLineOption) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QCommandLineOption14setDescriptionERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcommandlineoption.h:87
// index:0
// Public Visibility=Default Availability=Available
// [8] QString description() const

/*
Returns the description set for this option.

See also setDescription().
*/
impl /*struct*/ QCommandLineOption {
  pub fn description_0<RetType, T: QCommandLineOption_description_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.description_0(self);
    // return 1;
  }
}
pub trait QCommandLineOption_description_0<RetType> {
  fn description_0(self , rsthis: & QCommandLineOption) -> RetType;
}
impl<'a> /*trait*/ QCommandLineOption_description_0<usize> for () {
  fn description_0(self , rsthis: & QCommandLineOption) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QCommandLineOption11descriptionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcommandlineoption.h:89
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDefaultValue(const QString &)

/*
Sets the default value used for this option to defaultValue.

The default value is used if the user of the application does not specify the option on the command line.

If defaultValue is empty, the option has no default values.

See also defaultValues() and setDefaultValues().
*/
impl /*struct*/ QCommandLineOption {
  pub fn setDefaultValue_0<RetType, T: QCommandLineOption_setDefaultValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDefaultValue_0(self);
    // return 1;
  }
}
pub trait QCommandLineOption_setDefaultValue_0<RetType> {
  fn setDefaultValue_0(self , rsthis: & QCommandLineOption) -> RetType;
}
impl<'a> /*trait*/ QCommandLineOption_setDefaultValue_0<(/*void*/)> for (usize) {
  fn setDefaultValue_0(self , rsthis: & QCommandLineOption) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QCommandLineOption15setDefaultValueERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcommandlineoption.h:90
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDefaultValues(const QStringList &)

/*
Sets the list of default values used for this option to defaultValues.

The default values are used if the user of the application does not specify the option on the command line.

See also defaultValues() and setDefaultValue().
*/
impl /*struct*/ QCommandLineOption {
  pub fn setDefaultValues_0<RetType, T: QCommandLineOption_setDefaultValues_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDefaultValues_0(self);
    // return 1;
  }
}
pub trait QCommandLineOption_setDefaultValues_0<RetType> {
  fn setDefaultValues_0(self , rsthis: & QCommandLineOption) -> RetType;
}
impl<'a> /*trait*/ QCommandLineOption_setDefaultValues_0<(/*void*/)> for (usize) {
  fn setDefaultValues_0(self , rsthis: & QCommandLineOption) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QCommandLineOption16setDefaultValuesERK11QStringList", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcommandlineoption.h:91
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList defaultValues() const

/*
Returns the default values set for this option.

See also setDefaultValues().
*/
impl /*struct*/ QCommandLineOption {
  pub fn defaultValues_0<RetType, T: QCommandLineOption_defaultValues_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.defaultValues_0(self);
    // return 1;
  }
}
pub trait QCommandLineOption_defaultValues_0<RetType> {
  fn defaultValues_0(self , rsthis: & QCommandLineOption) -> RetType;
}
impl<'a> /*trait*/ QCommandLineOption_defaultValues_0<usize> for () {
  fn defaultValues_0(self , rsthis: & QCommandLineOption) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QCommandLineOption13defaultValuesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcommandlineoption.h:93
// index:0
// Public Visibility=Default Availability=Available
// [4] QCommandLineOption::Flags flags() const

/*
Returns a set of flags that affect this command-line option.

This function was introduced in  Qt 5.8.

See also setFlags() and QCommandLineOption::Flags.
*/
impl /*struct*/ QCommandLineOption {
  pub fn flags_0<RetType, T: QCommandLineOption_flags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.flags_0(self);
    // return 1;
  }
}
pub trait QCommandLineOption_flags_0<RetType> {
  fn flags_0(self , rsthis: & QCommandLineOption) -> RetType;
}
impl<'a> /*trait*/ QCommandLineOption_flags_0<i32> for () {
  fn flags_0(self , rsthis: & QCommandLineOption) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QCommandLineOption5flagsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcommandlineoption.h:94
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFlags(QCommandLineOption::Flags)

/*
Set the set of flags that affect this command-line option to flags.

This function was introduced in  Qt 5.8.

See also flags() and QCommandLineOption::Flags.
*/
impl /*struct*/ QCommandLineOption {
  pub fn setFlags_0<RetType, T: QCommandLineOption_setFlags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFlags_0(self);
    // return 1;
  }
}
pub trait QCommandLineOption_setFlags_0<RetType> {
  fn setFlags_0(self , rsthis: & QCommandLineOption) -> RetType;
}
impl<'a> /*trait*/ QCommandLineOption_setFlags_0<(/*void*/)> for (i32) {
  fn setFlags_0(self , rsthis: & QCommandLineOption) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN18QCommandLineOption8setFlagsE6QFlagsINS_4FlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcommandlineoption.h:98
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setHidden(bool)

/*

*/
impl /*struct*/ QCommandLineOption {
  pub fn setHidden_0<RetType, T: QCommandLineOption_setHidden_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHidden_0(self);
    // return 1;
  }
}
pub trait QCommandLineOption_setHidden_0<RetType> {
  fn setHidden_0(self , rsthis: & QCommandLineOption) -> RetType;
}
impl<'a> /*trait*/ QCommandLineOption_setHidden_0<(/*void*/)> for (bool) {
  fn setHidden_0(self , rsthis: & QCommandLineOption) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN18QCommandLineOption9setHiddenEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcommandlineoption.h:100
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isHidden() const

/*

*/
impl /*struct*/ QCommandLineOption {
  pub fn isHidden_0<RetType, T: QCommandLineOption_isHidden_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isHidden_0(self);
    // return 1;
  }
}
pub trait QCommandLineOption_isHidden_0<RetType> {
  fn isHidden_0(self , rsthis: & QCommandLineOption) -> RetType;
}
impl<'a> /*trait*/ QCommandLineOption_isHidden_0<bool> for () {
  fn isHidden_0(self , rsthis: & QCommandLineOption) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QCommandLineOption8isHiddenEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


/*


*/
pub type QCommandLineOption__Flag = i32;
// 
pub const QCommandLineOption__HiddenFromHelp :QCommandLineOption__Flag = 1;
// 
pub const QCommandLineOption__ShortOptionStyle :QCommandLineOption__Flag = 2;
pub fn QCommandLineOption_FlagItemName(val: i32) ->String {
  match val {
     QCommandLineOption__HiddenFromHelp => // 1
     {return String::from("HiddenFromHelp");}
     QCommandLineOption__ShortOptionStyle => // 2
     {return String::from("ShortOptionStyle");}
  _ => {return format!("{}", val);}
}
}
pub fn QCommandLineOption_FlagItemName_s(val: i32) ->String {
  //var nilthis *QCommandLineOption
  //return nilthis.FlagItemName(val);
  return QCommandLineOption_FlagItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
