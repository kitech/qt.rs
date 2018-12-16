

// mod ::core::QCommandLineParser
// package qtcore
// /usr/include/qt/QtCore/qcommandlineparser.h
// #include <qcommandlineparser.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 52
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
#[derive(Default)] // class sizeof(QCommandLineParser)=8
pub struct QCommandLineParser {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QCommandLineParser_ITF interface {
//    QCommandLineParser_PTR() *QCommandLineParser
//}
//func (ptr *QCommandLineParser) QCommandLineParser_PTR() *QCommandLineParser { return ptr }

impl /*struct*/ QCommandLineParser {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QCommandLineParser {
    return QCommandLineParser{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QCommandLineParser {
//  type Target = QCommandLineParserBASE;
//
//  fn deref(&self) -> &QCommandLineParserBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QCommandLineParserBASE> for QCommandLineParser {
//  fn as_ref(& self) -> & QCommandLineParserBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qcommandlineparser.h:59
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QCommandLineParser()

/*
Constructs a command line parser object.
*/
// QCommandLineParser() ctx.fn_proto_cpp
impl /*struct*/ QCommandLineParser {
  pub fn QCommandLineParser_0<T: QCommandLineParser_QCommandLineParser_0>(value: T) -> QCommandLineParser {
    let rsthis = value.QCommandLineParser_0();
    return rsthis;
    // return 1;
  }
}

pub trait QCommandLineParser_QCommandLineParser_0 {
  fn QCommandLineParser_0(self) -> QCommandLineParser;
}
// QCommandLineParser() ctx.fn_proto_cpp
impl<'a> /*trait*/ QCommandLineParser_QCommandLineParser_0 for () {
  fn QCommandLineParser_0(self) -> QCommandLineParser {
    // unsafe{_ZN18QCommandLineParserC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QCommandLineParserC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QCommandLineParser{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcommandlineparser.h:60
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QCommandLineParser()

/*

*/
pub fn DeleteQCommandLineParser(this :*mut QCommandLineParser) {
    // let rv = qtrt::InvokeQtFunc6("_ZN18QCommandLineParserD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qcommandlineparser.h:66
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSingleDashWordOptionMode(QCommandLineParser::SingleDashWordOptionMode)

/*
Sets the parsing mode to singleDashWordOptionMode. This must be called before process() or parse().
*/
impl /*struct*/ QCommandLineParser {
  pub fn setSingleDashWordOptionMode_0<RetType, T: QCommandLineParser_setSingleDashWordOptionMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSingleDashWordOptionMode_0(self);
    // return 1;
  }
}
pub trait QCommandLineParser_setSingleDashWordOptionMode_0<RetType> {
  fn setSingleDashWordOptionMode_0(self , rsthis: & QCommandLineParser) -> RetType;
}
impl<'a> /*trait*/ QCommandLineParser_setSingleDashWordOptionMode_0<(/*void*/)> for (i32) {
  fn setSingleDashWordOptionMode_0(self , rsthis: & QCommandLineParser) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN18QCommandLineParser27setSingleDashWordOptionModeENS_24SingleDashWordOptionModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcommandlineparser.h:72
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOptionsAfterPositionalArgumentsMode(QCommandLineParser::OptionsAfterPositionalArgumentsMode)

/*
Sets the parsing mode to parsingMode. This must be called before process() or parse().

This function was introduced in  Qt 5.6.
*/
impl /*struct*/ QCommandLineParser {
  pub fn setOptionsAfterPositionalArgumentsMode_0<RetType, T: QCommandLineParser_setOptionsAfterPositionalArgumentsMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOptionsAfterPositionalArgumentsMode_0(self);
    // return 1;
  }
}
pub trait QCommandLineParser_setOptionsAfterPositionalArgumentsMode_0<RetType> {
  fn setOptionsAfterPositionalArgumentsMode_0(self , rsthis: & QCommandLineParser) -> RetType;
}
impl<'a> /*trait*/ QCommandLineParser_setOptionsAfterPositionalArgumentsMode_0<(/*void*/)> for (i32) {
  fn setOptionsAfterPositionalArgumentsMode_0(self , rsthis: & QCommandLineParser) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN18QCommandLineParser38setOptionsAfterPositionalArgumentsModeENS_35OptionsAfterPositionalArgumentsModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcommandlineparser.h:74
// index:0
// Public Visibility=Default Availability=Available
// [1] bool addOption(const QCommandLineOption &)

/*
Adds the option option to look for while parsing.

Returns true if adding the option was successful; otherwise returns false.

Adding the option fails if there is no name attached to the option, or the option has a name that clashes with an option name added before.
*/
impl /*struct*/ QCommandLineParser {
  pub fn addOption_0<RetType, T: QCommandLineParser_addOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addOption_0(self);
    // return 1;
  }
}
pub trait QCommandLineParser_addOption_0<RetType> {
  fn addOption_0(self , rsthis: & QCommandLineParser) -> RetType;
}
impl<'a> /*trait*/ QCommandLineParser_addOption_0<bool> for (usize) {
  fn addOption_0(self , rsthis: & QCommandLineParser) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QCommandLineParser9addOptionERK18QCommandLineOption", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcommandlineparser.h:77
// index:0
// Public Visibility=Default Availability=Available
// [8] QCommandLineOption addVersionOption()

/*
Adds the -v / --version option, which displays the version string of the application.

This option is handled automatically by QCommandLineParser.

You can set the actual version string by using QCoreApplication::setApplicationVersion().

Returns the option instance, which can be used to call isSet().
*/
impl /*struct*/ QCommandLineParser {
  pub fn addVersionOption_0<RetType, T: QCommandLineParser_addVersionOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addVersionOption_0(self);
    // return 1;
  }
}
pub trait QCommandLineParser_addVersionOption_0<RetType> {
  fn addVersionOption_0(self , rsthis: & QCommandLineParser) -> RetType;
}
impl<'a> /*trait*/ QCommandLineParser_addVersionOption_0<usize> for () {
  fn addVersionOption_0(self , rsthis: & QCommandLineParser) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QCommandLineParser16addVersionOptionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcommandlineparser.h:78
// index:0
// Public Visibility=Default Availability=Available
// [8] QCommandLineOption addHelpOption()

/*
Adds the help option (-h, --help and -? on Windows) This option is handled automatically by QCommandLineParser.

Remember to use setApplicationDescription to set the application description, which will be displayed when this option is used.

Example:


  int main(int argc, char *argv[])
  {
      QCoreApplication app(argc, argv);
      QCoreApplication::setApplicationName("my-copy-program");
      QCoreApplication::setApplicationVersion("1.0");

      QCommandLineParser parser;
      parser.setApplicationDescription("Test helper");
      parser.addHelpOption();
      parser.addVersionOption();
      parser.addPositionalArgument("source", QCoreApplication::translate("main", "Source file to copy."));
      parser.addPositionalArgument("destination", QCoreApplication::translate("main", "Destination directory."));

      // A boolean option with a single name (-p)
      QCommandLineOption showProgressOption("p", QCoreApplication::translate("main", "Show progress during copy"));
      parser.addOption(showProgressOption);

      // A boolean option with multiple names (-f, --force)
      QCommandLineOption forceOption(QStringList() << "f" << "force",
              QCoreApplication::translate("main", "Overwrite existing files."));
      parser.addOption(forceOption);

      // An option with a value
      QCommandLineOption targetDirectoryOption(QStringList() << "t" << "target-directory",
              QCoreApplication::translate("main", "Copy all source files into <directory>."),
              QCoreApplication::translate("main", "directory"));
      parser.addOption(targetDirectoryOption);

      // Process the actual command line arguments given by the user
      parser.process(app);

      const QStringList args = parser.positionalArguments();
      // source is args.at(0), destination is args.at(1)

      bool showProgress = parser.isSet(showProgressOption);
      bool force = parser.isSet(forceOption);
      QString targetDir = parser.value(targetDirectoryOption);
      // ...
  }



Returns the option instance, which can be used to call isSet().
*/
impl /*struct*/ QCommandLineParser {
  pub fn addHelpOption_0<RetType, T: QCommandLineParser_addHelpOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addHelpOption_0(self);
    // return 1;
  }
}
pub trait QCommandLineParser_addHelpOption_0<RetType> {
  fn addHelpOption_0(self , rsthis: & QCommandLineParser) -> RetType;
}
impl<'a> /*trait*/ QCommandLineParser_addHelpOption_0<usize> for () {
  fn addHelpOption_0(self , rsthis: & QCommandLineParser) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QCommandLineParser13addHelpOptionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcommandlineparser.h:79
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setApplicationDescription(const QString &)

/*
Sets the application description shown by helpText().

See also applicationDescription().
*/
impl /*struct*/ QCommandLineParser {
  pub fn setApplicationDescription_0<RetType, T: QCommandLineParser_setApplicationDescription_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setApplicationDescription_0(self);
    // return 1;
  }
}
pub trait QCommandLineParser_setApplicationDescription_0<RetType> {
  fn setApplicationDescription_0(self , rsthis: & QCommandLineParser) -> RetType;
}
impl<'a> /*trait*/ QCommandLineParser_setApplicationDescription_0<(/*void*/)> for (usize) {
  fn setApplicationDescription_0(self , rsthis: & QCommandLineParser) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QCommandLineParser25setApplicationDescriptionERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcommandlineparser.h:80
// index:0
// Public Visibility=Default Availability=Available
// [8] QString applicationDescription() const

/*
Returns the application description set in setApplicationDescription().

See also setApplicationDescription().
*/
impl /*struct*/ QCommandLineParser {
  pub fn applicationDescription_0<RetType, T: QCommandLineParser_applicationDescription_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.applicationDescription_0(self);
    // return 1;
  }
}
pub trait QCommandLineParser_applicationDescription_0<RetType> {
  fn applicationDescription_0(self , rsthis: & QCommandLineParser) -> RetType;
}
impl<'a> /*trait*/ QCommandLineParser_applicationDescription_0<usize> for () {
  fn applicationDescription_0(self , rsthis: & QCommandLineParser) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QCommandLineParser22applicationDescriptionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcommandlineparser.h:81
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addPositionalArgument(const QString &, const QString &, const QString &)

/*
Defines an additional argument to the application, for the benefit of the help text.

The argument name and description will appear under the Arguments: section of the help. If syntax is specified, it will be appended to the Usage line, otherwise the name will be appended.

Example:


  // Usage: image-editor file
  //
  // Arguments:
  //   file                  The file to open.
  parser.addPositionalArgument("file", QCoreApplication::translate("main", "The file to open."));

  // Usage: web-browser [urls...]
  //
  // Arguments:
  //   urls                URLs to open, optionally.
  parser.addPositionalArgument("urls", QCoreApplication::translate("main", "URLs to open, optionally."), "[urls...]");

  // Usage: cp source destination
  //
  // Arguments:
  //   source                Source file to copy.
  //   destination           Destination directory.
  parser.addPositionalArgument("source", QCoreApplication::translate("main", "Source file to copy."));
  parser.addPositionalArgument("destination", QCoreApplication::translate("main", "Destination directory."));



See also addHelpOption() and helpText().
*/
impl /*struct*/ QCommandLineParser {
  pub fn addPositionalArgument_0<RetType, T: QCommandLineParser_addPositionalArgument_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addPositionalArgument_0(self);
    // return 1;
  }
}
pub trait QCommandLineParser_addPositionalArgument_0<RetType> {
  fn addPositionalArgument_0(self , rsthis: & QCommandLineParser) -> RetType;
}
impl<'a> /*trait*/ QCommandLineParser_addPositionalArgument_0<(/*void*/)> for (usize,usize,usize) {
  fn addPositionalArgument_0(self , rsthis: & QCommandLineParser) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QCommandLineParser21addPositionalArgumentERK7QStringS2_S2_", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcommandlineparser.h:82
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clearPositionalArguments()

/*
Clears the definitions of additional arguments from the help text.

This is only needed for the special case of tools which support multiple commands with different options. Once the actual command has been identified, the options for this command can be defined, and the help text for the command can be adjusted accordingly.

Example:


  QCoreApplication app(argc, argv);
  QCommandLineParser parser;

  parser.addPositionalArgument("command", "The command to execute.");

  // Call parse() to find out the positional arguments.
  parser.parse(QCoreApplication::arguments());

  const QStringList args = parser.positionalArguments();
  const QString command = args.isEmpty() ? QString() : args.first();
  if (command == "resize") {
      parser.clearPositionalArguments();
      parser.addPositionalArgument("resize", "Resize the object to a new size.", "resize [resize_options]");
      parser.addOption(QCommandLineOption("size", "New size.", "new_size"));
      parser.process(app);
      // ...
  }

  /-*
  This code results in context-dependent help:

  $ tool --help
  Usage: tool command

  Arguments:
    command  The command to execute.

  $ tool resize --help
  Usage: tool resize [resize_options]

  Options:
    --size <size>  New size.

  Arguments:
    resize         Resize the object to a new size.
  *-/
*/
impl /*struct*/ QCommandLineParser {
  pub fn clearPositionalArguments_0<RetType, T: QCommandLineParser_clearPositionalArguments_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clearPositionalArguments_0(self);
    // return 1;
  }
}
pub trait QCommandLineParser_clearPositionalArguments_0<RetType> {
  fn clearPositionalArguments_0(self , rsthis: & QCommandLineParser) -> RetType;
}
impl<'a> /*trait*/ QCommandLineParser_clearPositionalArguments_0<(/*void*/)> for () {
  fn clearPositionalArguments_0(self , rsthis: & QCommandLineParser) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN18QCommandLineParser24clearPositionalArgumentsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcommandlineparser.h:84
// index:0
// Public Visibility=Default Availability=Available
// [-2] void process(const QStringList &)

/*
Processes the command line arguments.

In addition to parsing the options (like parse()), this function also handles the builtin options and handles errors.

The builtin options are --version if addVersionOption was called and --help if addHelpOption was called.

When invoking one of these options, or when an error happens (for instance an unknown option was passed), the current process will then stop, using the exit() function.

See also QCoreApplication::arguments() and parse().
*/
impl /*struct*/ QCommandLineParser {
  pub fn process_0<RetType, T: QCommandLineParser_process_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.process_0(self);
    // return 1;
  }
}
pub trait QCommandLineParser_process_0<RetType> {
  fn process_0(self , rsthis: & QCommandLineParser) -> RetType;
}
impl<'a> /*trait*/ QCommandLineParser_process_0<(/*void*/)> for (usize) {
  fn process_0(self , rsthis: & QCommandLineParser) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QCommandLineParser7processERK11QStringList", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcommandlineparser.h:85
// index:1
// Public Visibility=Default Availability=Available
// [-2] void process(const QCoreApplication &)

/*
Processes the command line arguments.

In addition to parsing the options (like parse()), this function also handles the builtin options and handles errors.

The builtin options are --version if addVersionOption was called and --help if addHelpOption was called.

When invoking one of these options, or when an error happens (for instance an unknown option was passed), the current process will then stop, using the exit() function.

See also QCoreApplication::arguments() and parse().
*/
impl /*struct*/ QCommandLineParser {
  pub fn process_1<RetType, T: QCommandLineParser_process_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.process_1(self);
    // return 1;
  }
}
pub trait QCommandLineParser_process_1<RetType> {
  fn process_1(self , rsthis: & QCommandLineParser) -> RetType;
}
impl<'a> /*trait*/ QCommandLineParser_process_1<(/*void*/)> for (usize) {
  fn process_1(self , rsthis: & QCommandLineParser) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QCommandLineParser7processERK16QCoreApplication", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcommandlineparser.h:87
// index:0
// Public Visibility=Default Availability=Available
// [1] bool parse(const QStringList &)

/*
Parses the command line arguments.

Most programs don't need to call this, a simple call to process() is enough.

parse() is more low-level, and only does the parsing. The application will have to take care of the error handling, using errorText() if parse() returns false. This can be useful for instance to show a graphical error message in graphical programs.

Calling parse() instead of process() can also be useful in order to ignore unknown options temporarily, because more option definitions will be provided later on (depending on one of the arguments), before calling process().

Don't forget that arguments must start with the name of the executable (ignored, though).

Returns false in case of a parse error (unknown option or missing value); returns true otherwise.

See also process().
*/
impl /*struct*/ QCommandLineParser {
  pub fn parse_0<RetType, T: QCommandLineParser_parse_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.parse_0(self);
    // return 1;
  }
}
pub trait QCommandLineParser_parse_0<RetType> {
  fn parse_0(self , rsthis: & QCommandLineParser) -> RetType;
}
impl<'a> /*trait*/ QCommandLineParser_parse_0<bool> for (usize) {
  fn parse_0(self , rsthis: & QCommandLineParser) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QCommandLineParser5parseERK11QStringList", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcommandlineparser.h:88
// index:0
// Public Visibility=Default Availability=Available
// [8] QString errorText() const

/*
Returns a translated error text for the user. This should only be called when parse() returns false.
*/
impl /*struct*/ QCommandLineParser {
  pub fn errorText_0<RetType, T: QCommandLineParser_errorText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.errorText_0(self);
    // return 1;
  }
}
pub trait QCommandLineParser_errorText_0<RetType> {
  fn errorText_0(self , rsthis: & QCommandLineParser) -> RetType;
}
impl<'a> /*trait*/ QCommandLineParser_errorText_0<usize> for () {
  fn errorText_0(self , rsthis: & QCommandLineParser) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QCommandLineParser9errorTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcommandlineparser.h:90
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isSet(const QString &) const

/*
Checks whether the option name was passed to the application.

Returns true if the option name was set, false otherwise.

The name provided can be any long or short name of any option that was added with addOption(). All the options names are treated as being equivalent. If the name is not recognized or that option was not present, false is returned.

Example:


  bool verbose = parser.isSet("verbose");
*/
impl /*struct*/ QCommandLineParser {
  pub fn isSet_0<RetType, T: QCommandLineParser_isSet_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isSet_0(self);
    // return 1;
  }
}
pub trait QCommandLineParser_isSet_0<RetType> {
  fn isSet_0(self , rsthis: & QCommandLineParser) -> RetType;
}
impl<'a> /*trait*/ QCommandLineParser_isSet_0<bool> for (usize) {
  fn isSet_0(self , rsthis: & QCommandLineParser) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QCommandLineParser5isSetERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcommandlineparser.h:94
// index:1
// Public Visibility=Default Availability=Available
// [1] bool isSet(const QCommandLineOption &) const

/*
Checks whether the option name was passed to the application.

Returns true if the option name was set, false otherwise.

The name provided can be any long or short name of any option that was added with addOption(). All the options names are treated as being equivalent. If the name is not recognized or that option was not present, false is returned.

Example:


  bool verbose = parser.isSet("verbose");
*/
impl /*struct*/ QCommandLineParser {
  pub fn isSet_1<RetType, T: QCommandLineParser_isSet_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isSet_1(self);
    // return 1;
  }
}
pub trait QCommandLineParser_isSet_1<RetType> {
  fn isSet_1(self , rsthis: & QCommandLineParser) -> RetType;
}
impl<'a> /*trait*/ QCommandLineParser_isSet_1<bool> for (usize) {
  fn isSet_1(self , rsthis: & QCommandLineParser) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QCommandLineParser5isSetERK18QCommandLineOption", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcommandlineparser.h:91
// index:0
// Public Visibility=Default Availability=Available
// [8] QString value(const QString &) const

/*
Returns the option value found for the given option name optionName, or an empty string if not found.

The name provided can be any long or short name of any option that was added with addOption(). All the option names are treated as being equivalent. If the name is not recognized or that option was not present, an empty string is returned.

For options found by the parser, the last value found for that option is returned. If the option wasn't specified on the command line, the default value is returned.

An empty string is returned if the option does not take a value.

See also values(), QCommandLineOption::setDefaultValue(), and QCommandLineOption::setDefaultValues().
*/
impl /*struct*/ QCommandLineParser {
  pub fn value_0<RetType, T: QCommandLineParser_value_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.value_0(self);
    // return 1;
  }
}
pub trait QCommandLineParser_value_0<RetType> {
  fn value_0(self , rsthis: & QCommandLineParser) -> RetType;
}
impl<'a> /*trait*/ QCommandLineParser_value_0<usize> for (usize) {
  fn value_0(self , rsthis: & QCommandLineParser) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QCommandLineParser5valueERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcommandlineparser.h:95
// index:1
// Public Visibility=Default Availability=Available
// [8] QString value(const QCommandLineOption &) const

/*
Returns the option value found for the given option name optionName, or an empty string if not found.

The name provided can be any long or short name of any option that was added with addOption(). All the option names are treated as being equivalent. If the name is not recognized or that option was not present, an empty string is returned.

For options found by the parser, the last value found for that option is returned. If the option wasn't specified on the command line, the default value is returned.

An empty string is returned if the option does not take a value.

See also values(), QCommandLineOption::setDefaultValue(), and QCommandLineOption::setDefaultValues().
*/
impl /*struct*/ QCommandLineParser {
  pub fn value_1<RetType, T: QCommandLineParser_value_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.value_1(self);
    // return 1;
  }
}
pub trait QCommandLineParser_value_1<RetType> {
  fn value_1(self , rsthis: & QCommandLineParser) -> RetType;
}
impl<'a> /*trait*/ QCommandLineParser_value_1<usize> for (usize) {
  fn value_1(self , rsthis: & QCommandLineParser) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QCommandLineParser5valueERK18QCommandLineOption", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcommandlineparser.h:92
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList values(const QString &) const

/*
Returns a list of option values found for the given option name optionName, or an empty list if not found.

The name provided can be any long or short name of any option that was added with addOption(). All the options names are treated as being equivalent. If the name is not recognized or that option was not present, an empty list is returned.

For options found by the parser, the list will contain an entry for each time the option was encountered by the parser. If the option wasn't specified on the command line, the default values are returned.

An empty list is returned if the option does not take a value.

See also value(), QCommandLineOption::setDefaultValue(), and QCommandLineOption::setDefaultValues().
*/
impl /*struct*/ QCommandLineParser {
  pub fn values_0<RetType, T: QCommandLineParser_values_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.values_0(self);
    // return 1;
  }
}
pub trait QCommandLineParser_values_0<RetType> {
  fn values_0(self , rsthis: & QCommandLineParser) -> RetType;
}
impl<'a> /*trait*/ QCommandLineParser_values_0<usize> for (usize) {
  fn values_0(self , rsthis: & QCommandLineParser) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QCommandLineParser6valuesERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcommandlineparser.h:96
// index:1
// Public Visibility=Default Availability=Available
// [8] QStringList values(const QCommandLineOption &) const

/*
Returns a list of option values found for the given option name optionName, or an empty list if not found.

The name provided can be any long or short name of any option that was added with addOption(). All the options names are treated as being equivalent. If the name is not recognized or that option was not present, an empty list is returned.

For options found by the parser, the list will contain an entry for each time the option was encountered by the parser. If the option wasn't specified on the command line, the default values are returned.

An empty list is returned if the option does not take a value.

See also value(), QCommandLineOption::setDefaultValue(), and QCommandLineOption::setDefaultValues().
*/
impl /*struct*/ QCommandLineParser {
  pub fn values_1<RetType, T: QCommandLineParser_values_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.values_1(self);
    // return 1;
  }
}
pub trait QCommandLineParser_values_1<RetType> {
  fn values_1(self , rsthis: & QCommandLineParser) -> RetType;
}
impl<'a> /*trait*/ QCommandLineParser_values_1<usize> for (usize) {
  fn values_1(self , rsthis: & QCommandLineParser) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QCommandLineParser6valuesERK18QCommandLineOption", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcommandlineparser.h:98
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList positionalArguments() const

/*
Returns a list of positional arguments.

These are all of the arguments that were not recognized as part of an option.
*/
impl /*struct*/ QCommandLineParser {
  pub fn positionalArguments_0<RetType, T: QCommandLineParser_positionalArguments_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.positionalArguments_0(self);
    // return 1;
  }
}
pub trait QCommandLineParser_positionalArguments_0<RetType> {
  fn positionalArguments_0(self , rsthis: & QCommandLineParser) -> RetType;
}
impl<'a> /*trait*/ QCommandLineParser_positionalArguments_0<usize> for () {
  fn positionalArguments_0(self , rsthis: & QCommandLineParser) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QCommandLineParser19positionalArgumentsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcommandlineparser.h:99
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList optionNames() const

/*
Returns a list of option names that were found.

This returns a list of all the recognized option names found by the parser, in the order in which they were found. For any long options that were in the form {--option=value}, the value part will have been dropped.

The names in this list do not include the preceding dash characters. Names may appear more than once in this list if they were encountered more than once by the parser.

Any entry in the list can be used with value() or with values() to get any relevant option values.
*/
impl /*struct*/ QCommandLineParser {
  pub fn optionNames_0<RetType, T: QCommandLineParser_optionNames_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.optionNames_0(self);
    // return 1;
  }
}
pub trait QCommandLineParser_optionNames_0<RetType> {
  fn optionNames_0(self , rsthis: & QCommandLineParser) -> RetType;
}
impl<'a> /*trait*/ QCommandLineParser_optionNames_0<usize> for () {
  fn optionNames_0(self , rsthis: & QCommandLineParser) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QCommandLineParser11optionNamesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcommandlineparser.h:100
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList unknownOptionNames() const

/*
Returns a list of unknown option names.

This list will include both long an short name options that were not recognized. For any long options that were in the form {--option=value}, the value part will have been dropped and only the long name is added.

The names in this list do not include the preceding dash characters. Names may appear more than once in this list if they were encountered more than once by the parser.

See also optionNames().
*/
impl /*struct*/ QCommandLineParser {
  pub fn unknownOptionNames_0<RetType, T: QCommandLineParser_unknownOptionNames_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unknownOptionNames_0(self);
    // return 1;
  }
}
pub trait QCommandLineParser_unknownOptionNames_0<RetType> {
  fn unknownOptionNames_0(self , rsthis: & QCommandLineParser) -> RetType;
}
impl<'a> /*trait*/ QCommandLineParser_unknownOptionNames_0<usize> for () {
  fn unknownOptionNames_0(self , rsthis: & QCommandLineParser) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QCommandLineParser18unknownOptionNamesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcommandlineparser.h:102
// index:0
// Public Visibility=Default Availability=Available
// [-2] void showVersion()

/*
Displays the version information from QCoreApplication::applicationVersion(), and exits the application. This is automatically triggered by the --version option, but can also be used to display the version when not using process(). The exit code is set to EXIT_SUCCESS (0).

This function was introduced in  Qt 5.4.

See also addVersionOption().
*/
impl /*struct*/ QCommandLineParser {
  pub fn showVersion_0<RetType, T: QCommandLineParser_showVersion_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showVersion_0(self);
    // return 1;
  }
}
pub trait QCommandLineParser_showVersion_0<RetType> {
  fn showVersion_0(self , rsthis: & QCommandLineParser) -> RetType;
}
impl<'a> /*trait*/ QCommandLineParser_showVersion_0<(/*void*/)> for () {
  fn showVersion_0(self , rsthis: & QCommandLineParser) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN18QCommandLineParser11showVersionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcommandlineparser.h:103
// index:0
// Public Visibility=Default Availability=Available
// [-2] void showHelp(int)

/*
Displays the help information, and exits the application. This is automatically triggered by the --help option, but can also be used to display the help when the user is not invoking the application correctly. The exit code is set to exitCode. It should be set to 0 if the user requested to see the help, and to any other value in case of an error.

See also helpText().
*/
impl /*struct*/ QCommandLineParser {
  pub fn showHelp_0<RetType, T: QCommandLineParser_showHelp_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showHelp_0(self);
    // return 1;
  }
}
pub trait QCommandLineParser_showHelp_0<RetType> {
  fn showHelp_0(self , rsthis: & QCommandLineParser) -> RetType;
}
impl<'a> /*trait*/ QCommandLineParser_showHelp_0<(/*void*/)> for (i32) {
  fn showHelp_0(self , rsthis: & QCommandLineParser) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN18QCommandLineParser8showHelpEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcommandlineparser.h:104
// index:0
// Public Visibility=Default Availability=Available
// [8] QString helpText() const

/*
Returns a string containing the complete help information.

See also showHelp().
*/
impl /*struct*/ QCommandLineParser {
  pub fn helpText_0<RetType, T: QCommandLineParser_helpText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.helpText_0(self);
    // return 1;
  }
}
pub trait QCommandLineParser_helpText_0<RetType> {
  fn helpText_0(self , rsthis: & QCommandLineParser) -> RetType;
}
impl<'a> /*trait*/ QCommandLineParser_helpText_0<usize> for () {
  fn helpText_0(self , rsthis: & QCommandLineParser) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QCommandLineParser8helpTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


/*
This enum describes the way the parser interprets command-line options that use a single dash followed by multiple letters, as as -abc.



See also setSingleDashWordOptionMode().

*/
pub type QCommandLineParser__SingleDashWordOptionMode = i32;
// -abc is interpreted as -a -b -c, i.e. as three short options that have been compacted on the command-line, if none of the options take a value. If a takes a value, then it is interpreted as -a bc, i.e. the short option a followed by the value bc. This is typically used in tools that behave like compilers, in order to handle options such as -DDEFINE=VALUE or -I/include/path. This is the default parsing mode. New applications are recommended to use this mode.
pub const QCommandLineParser__ParseAsCompactedShortOptions :QCommandLineParser__SingleDashWordOptionMode = 0;
// -abc is interpreted as --abc, i.e. as the long option named abc. This is how Qt's own tools (uic, rcc...) have always been parsing arguments. This mode should be used for preserving compatibility in applications that were parsing arguments in such a way. There is an exception if the a option has the QCommandLineOption::ShortOptionStyle flag set, in which case it is still interpreted as -a bc.
pub const QCommandLineParser__ParseAsLongOptions :QCommandLineParser__SingleDashWordOptionMode = 1;
pub fn QCommandLineParser_SingleDashWordOptionModeItemName(val: i32) ->String {
  match val {
     QCommandLineParser__ParseAsCompactedShortOptions => // 0
     {return String::from("ParseAsCompactedShortOptions");}
     QCommandLineParser__ParseAsLongOptions => // 1
     {return String::from("ParseAsLongOptions");}
  _ => {return format!("{}", val);}
}
}
pub fn QCommandLineParser_SingleDashWordOptionModeItemName_s(val: i32) ->String {
  //var nilthis *QCommandLineParser
  //return nilthis.SingleDashWordOptionModeItemName(val);
  return QCommandLineParser_SingleDashWordOptionModeItemName(val);
}


/*
This enum describes the way the parser interprets options that occur after positional arguments.



This enum was introduced or modified in  Qt 5.6.

See also setOptionsAfterPositionalArgumentsMode().

*/
pub type QCommandLineParser__OptionsAfterPositionalArgumentsMode = i32;
// application argument --opt -t is interpreted as setting the options opt and t, just like application --opt -t argument would do. This is the default parsing mode. In order to specify that --opt and -t are positional arguments instead, the user can use --, as in application argument -- --opt -t.
pub const QCommandLineParser__ParseAsOptions :QCommandLineParser__OptionsAfterPositionalArgumentsMode = 0;
// application argument --opt is interpreted as having two positional arguments, argument and --opt. This mode is useful for executables that aim to launch other executables (e.g. wrappers, debugging tools, etc.) or that support internal commands followed by options for the command. argument is the name of the command, and all options occurring after it can be collected and parsed by another command line parser, possibly in another executable.
pub const QCommandLineParser__ParseAsPositionalArguments :QCommandLineParser__OptionsAfterPositionalArgumentsMode = 1;
pub fn QCommandLineParser_OptionsAfterPositionalArgumentsModeItemName(val: i32) ->String {
  match val {
     QCommandLineParser__ParseAsOptions => // 0
     {return String::from("ParseAsOptions");}
     QCommandLineParser__ParseAsPositionalArguments => // 1
     {return String::from("ParseAsPositionalArguments");}
  _ => {return format!("{}", val);}
}
}
pub fn QCommandLineParser_OptionsAfterPositionalArgumentsModeItemName_s(val: i32) ->String {
  //var nilthis *QCommandLineParser
  //return nilthis.OptionsAfterPositionalArgumentsModeItemName(val);
  return QCommandLineParser_OptionsAfterPositionalArgumentsModeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
