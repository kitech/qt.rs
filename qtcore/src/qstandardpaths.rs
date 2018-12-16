

// mod ::core::QStandardPaths
// package qtcore
// /usr/include/qt/QtCore/qstandardpaths.h
// #include <qstandardpaths.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 62
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QStandardPaths)=1
pub struct QStandardPaths {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStandardPaths_ITF interface {
//    QStandardPaths_PTR() *QStandardPaths
//}
//func (ptr *QStandardPaths) QStandardPaths_PTR() *QStandardPaths { return ptr }

impl /*struct*/ QStandardPaths {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStandardPaths {
    return QStandardPaths{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStandardPaths {
//  type Target = QStandardPathsBASE;
//
//  fn deref(&self) -> &QStandardPathsBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStandardPathsBASE> for QStandardPaths {
//  fn as_ref(& self) -> & QStandardPathsBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qstandardpaths.h:81
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString writableLocation(QStandardPaths::StandardLocation)

/*
Returns the directory where files of type should be written to, or an empty string if the location cannot be determined.

Note: The storage location returned can be a directory that does not exist; i.e., it may need to be created by the system or the user.
*/
impl /*struct*/ QStandardPaths {
  pub fn writableLocation_0<RetType, T: QStandardPaths_writableLocation_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.writableLocation_0();
    // return 1;
  }
}
pub trait QStandardPaths_writableLocation_0<RetType> {
  fn writableLocation_0(self ) -> RetType;
}
impl<'a> /*trait*/ QStandardPaths_writableLocation_0<usize> for (i32) {
  fn writableLocation_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QStandardPaths16writableLocationENS_16StandardLocationE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstandardpaths.h:82
// index:0
// Public static Visibility=Default Availability=Available
// [8] QStringList standardLocations(QStandardPaths::StandardLocation)

/*
Returns all the directories where files of type belong.

The list of directories is sorted from high to low priority, starting with writableLocation() if it can be determined. This list is empty if no locations for type are defined.

See also writableLocation().
*/
impl /*struct*/ QStandardPaths {
  pub fn standardLocations_0<RetType, T: QStandardPaths_standardLocations_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.standardLocations_0();
    // return 1;
  }
}
pub trait QStandardPaths_standardLocations_0<RetType> {
  fn standardLocations_0(self ) -> RetType;
}
impl<'a> /*trait*/ QStandardPaths_standardLocations_0<usize> for (i32) {
  fn standardLocations_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QStandardPaths17standardLocationsENS_16StandardLocationE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstandardpaths.h:91
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString locate(QStandardPaths::StandardLocation, const QString &, QStandardPaths::LocateOptions)

/*
Tries to find a file or directory called fileName in the standard locations for type.

The full path to the first file or directory (depending on options) found is returned. If no such file or directory can be found, an empty string is returned.
*/
impl /*struct*/ QStandardPaths {
  pub fn locate_0<RetType, T: QStandardPaths_locate_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.locate_0();
    // return 1;
  }
}
pub trait QStandardPaths_locate_0<RetType> {
  fn locate_0(self ) -> RetType;
}
impl<'a> /*trait*/ QStandardPaths_locate_0<usize> for (i32,usize,i32) {
  fn locate_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QStandardPaths6locateENS_16StandardLocationERK7QString6QFlagsINS_12LocateOptionEE", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstandardpaths.h:92
// index:0
// Public static Visibility=Default Availability=Available
// [8] QStringList locateAll(QStandardPaths::StandardLocation, const QString &, QStandardPaths::LocateOptions)

/*
Tries to find all files or directories called fileName in the standard locations for type.

The options flag allows to specify whether to look for files or directories.

Returns the list of all the files that were found.
*/
impl /*struct*/ QStandardPaths {
  pub fn locateAll_0<RetType, T: QStandardPaths_locateAll_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.locateAll_0();
    // return 1;
  }
}
pub trait QStandardPaths_locateAll_0<RetType> {
  fn locateAll_0(self ) -> RetType;
}
impl<'a> /*trait*/ QStandardPaths_locateAll_0<usize> for (i32,usize,i32) {
  fn locateAll_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QStandardPaths9locateAllENS_16StandardLocationERK7QString6QFlagsINS_12LocateOptionEE", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstandardpaths.h:94
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString displayName(QStandardPaths::StandardLocation)

/*
Returns a localized display name for the given location type or an empty QString if no relevant location can be found.
*/
impl /*struct*/ QStandardPaths {
  pub fn displayName_0<RetType, T: QStandardPaths_displayName_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.displayName_0();
    // return 1;
  }
}
pub trait QStandardPaths_displayName_0<RetType> {
  fn displayName_0(self ) -> RetType;
}
impl<'a> /*trait*/ QStandardPaths_displayName_0<usize> for (i32) {
  fn displayName_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QStandardPaths11displayNameENS_16StandardLocationE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstandardpaths.h:97
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString findExecutable(const QString &, const QStringList &)

/*
Finds the executable named executableName in the paths specified by paths, or the system paths if paths is empty.

On most operating systems the system path is determined by the PATH environment variable.

The directories where to search for the executable can be set in the paths argument. To search in both your own paths and the system paths, call findExecutable twice, once with paths set and once with paths empty.

Symlinks are not resolved, in order to preserve behavior for the case of executables whose behavior depends on the name they are invoked with.

Note: On Windows, the usual executable extensions (from the PATHEXT environment variable) are automatically appended, so that for instance findExecutable("foo") will find foo.exe or foo.bat if present.

Returns the absolute file path to the executable, or an empty string if not found.
*/
impl /*struct*/ QStandardPaths {
  pub fn findExecutable_0<RetType, T: QStandardPaths_findExecutable_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.findExecutable_0();
    // return 1;
  }
}
pub trait QStandardPaths_findExecutable_0<RetType> {
  fn findExecutable_0(self ) -> RetType;
}
impl<'a> /*trait*/ QStandardPaths_findExecutable_0<usize> for (usize,usize) {
  fn findExecutable_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QStandardPaths14findExecutableERK7QStringRK11QStringList", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstandardpaths.h:100
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void enableTestMode(bool)

/*

*/
impl /*struct*/ QStandardPaths {
  pub fn enableTestMode_0<RetType, T: QStandardPaths_enableTestMode_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.enableTestMode_0();
    // return 1;
  }
}
pub trait QStandardPaths_enableTestMode_0<RetType> {
  fn enableTestMode_0(self ) -> RetType;
}
impl<'a> /*trait*/ QStandardPaths_enableTestMode_0<(/*void*/)> for (bool) {
  fn enableTestMode_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN14QStandardPaths14enableTestModeEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstandardpaths.h:102
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setTestModeEnabled(bool)

/*
If testMode is true, this enables a special "test mode" in QStandardPaths, which changes writable locations to point to test directories, in order to prevent auto tests from reading from or writing to the current user's configuration.

This affects the locations into which test programs might write files: GenericDataLocation, DataLocation, ConfigLocation, GenericConfigLocation, AppConfigLocation, GenericCacheLocation, CacheLocation. Other locations are not affected.

On Unix, XDG_DATA_HOME is set to ~/.qttest/share, XDG_CONFIG_HOME is set to ~/.qttest/config, and XDG_CACHE_HOME is set to ~/.qttest/cache.

On macOS, data goes to ~/.qttest/Application Support, cache goes to ~/.qttest/Cache, and config goes to ~/.qttest/Preferences.

On Windows, everything goes to a "qttest" directory under Application Data.
*/
impl /*struct*/ QStandardPaths {
  pub fn setTestModeEnabled_0<RetType, T: QStandardPaths_setTestModeEnabled_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setTestModeEnabled_0();
    // return 1;
  }
}
pub trait QStandardPaths_setTestModeEnabled_0<RetType> {
  fn setTestModeEnabled_0(self ) -> RetType;
}
impl<'a> /*trait*/ QStandardPaths_setTestModeEnabled_0<(/*void*/)> for (bool) {
  fn setTestModeEnabled_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN14QStandardPaths18setTestModeEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstandardpaths.h:103
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool isTestModeEnabled()

/*

*/
impl /*struct*/ QStandardPaths {
  pub fn isTestModeEnabled_0<RetType, T: QStandardPaths_isTestModeEnabled_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.isTestModeEnabled_0();
    // return 1;
  }
}
pub trait QStandardPaths_isTestModeEnabled_0<RetType> {
  fn isTestModeEnabled_0(self ) -> RetType;
}
impl<'a> /*trait*/ QStandardPaths_isTestModeEnabled_0<bool> for () {
  fn isTestModeEnabled_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QStandardPaths17isTestModeEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


pub fn DeleteQStandardPaths(this :*mut QStandardPaths) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN14QStandardPathsD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*
This enum describes the different locations that can be queried using methods such as QStandardPaths::writableLocation, QStandardPaths::standardLocations, and QStandardPaths::displayName.

Some of the values in this enum represent a user configuration. Such enum values will return the same paths in different applications, so they could be used to share data with other applications. Other values are specific to this application. Each enum value in the table below describes whether it's application-specific or generic.

Application-specific directories should be assumed to be unreachable by other applications. Therefore, files placed there might not be readable by other applications, even if run by the same user. On the other hand, generic directories should be assumed to be accessible by all applications run by this user, but should still be assumed to be unreachable by applications by other users.

Data interchange with other users is out of the scope of QStandardPaths.



The following table gives examples of paths on different operating systems. The first path is the writable path (unless noted). Other, additional paths, if any, represent non-writable locations.


 Path typemacOSWindows
DesktopLocation"~/Desktop""C:/Users/<USER>/Desktop"
DocumentsLocation"~/Documents""C:/Users/<USER>/Documents"
FontsLocation"/System/Library/Fonts" (not writable)"C:/Windows/Fonts" (not writable)
ApplicationsLocation"/Applications" (not writable)"C:/Users/<USER>/AppData/Roaming/Microsoft/Windows/Start Menu/Programs"
MusicLocation"~/Music""C:/Users/<USER>/Music"
MoviesLocation"~/Movies""C:/Users/<USER>/Videos"
PicturesLocation"~/Pictures""C:/Users/<USER>/Pictures"
TempLocationrandomly generated by the OS"C:/Users/<USER>/AppData/Local/Temp"
HomeLocation"~""C:/Users/<USER>"
DataLocation"~/Library/Application Support/<APPNAME>", "/Library/Application Support/<APPNAME>". "<APPDIR>/../Resources""C:/Users/<USER>/AppData/Local/<APPNAME>", "C:/ProgramData/<APPNAME>", "<APPDIR>", "<APPDIR>/data", "<APPDIR>/data/<APPNAME>"
CacheLocation"~/Library/Caches/<APPNAME>", "/Library/Caches/<APPNAME>""C:/Users/<USER>/AppData/Local/<APPNAME>/cache"
GenericDataLocation"~/Library/Application Support", "/Library/Application Support""C:/Users/<USER>/AppData/Local", "C:/ProgramData", "<APPDIR>", "<APPDIR>/data"
RuntimeLocation"~/Library/Application Support""C:/Users/<USER>"
ConfigLocation"~/Library/Preferences""C:/Users/<USER>/AppData/Local/<APPNAME>", "C:/ProgramData/<APPNAME>"
GenericConfigLocation"~/Library/Preferences""C:/Users/<USER>/AppData/Local", "C:/ProgramData"
DownloadLocation"~/Downloads""C:/Users/<USER>/Documents"
GenericCacheLocation"~/Library/Caches", "/Library/Caches""C:/Users/<USER>/AppData/Local/cache"
AppDataLocation"~/Library/Application Support/<APPNAME>", "/Library/Application Support/<APPNAME>". "<APPDIR>/../Resources""C:/Users/<USER>/AppData/Roaming/<APPNAME>", "C:/ProgramData/<APPNAME>", "<APPDIR>", "<APPDIR>/data", "<APPDIR>/data/<APPNAME>"
AppLocalDataLocation"~/Library/Application Support/<APPNAME>", "/Library/Application Support/<APPNAME>". "<APPDIR>/../Resources""C:/Users/<USER>/AppData/Local/<APPNAME>", "C:/ProgramData/<APPNAME>", "<APPDIR>", "<APPDIR>/data", "<APPDIR>/data/<APPNAME>"
AppConfigLocation"~/Library/Preferences/<APPNAME>""C:/Users/<USER>/AppData/Local/<APPNAME>", "C:/ProgramData/<APPNAME>"



 Path typeLinux
DesktopLocation"~/Desktop"
DocumentsLocation"~/Documents"
FontsLocation"~/.fonts"
ApplicationsLocation"~/.local/share/applications", "/usr/local/share/applications", "/usr/share/applications"
MusicLocation"~/Music"
MoviesLocation"~/Videos"
PicturesLocation"~/Pictures"
TempLocation"/tmp"
HomeLocation"~"
DataLocation"~/.local/share/<APPNAME>", "/usr/local/share/<APPNAME>", "/usr/share/<APPNAME>"
CacheLocation"~/.cache/<APPNAME>"
GenericDataLocation"~/.local/share", "/usr/local/share", "/usr/share"
RuntimeLocation"/run/user/<USER>"
ConfigLocation"~/.config", "/etc/xdg"
GenericConfigLocation"~/.config", "/etc/xdg"
DownloadLocation"~/Downloads"
GenericCacheLocation"~/.cache"
AppDataLocation"~/.local/share/<APPNAME>", "/usr/local/share/<APPNAME>", "/usr/share/<APPNAME>"
AppLocalDataLocation"~/.local/share/<APPNAME>", "/usr/local/share/<APPNAME>", "/usr/share/<APPNAME>"
AppConfigLocation"~/.config/<APPNAME>", "/etc/xdg/<APPNAME>"



 Path typeAndroidiOS
DesktopLocation"<APPROOT>/files""<APPROOT>/Documents/Desktop"
DocumentsLocation"<USER>/Documents", "<USER>/<APPNAME>/Documents""<APPROOT>/Documents"
FontsLocation"/system/fonts" (not writable)"<APPROOT>/Library/Fonts"
ApplicationsLocationnot supported (directory not readable)not supported
MusicLocation"<USER>/Music", "<USER>/<APPNAME>/Music""<APPROOT>/Documents/Music"
MoviesLocation"<USER>/Movies", "<USER>/<APPNAME>/Movies""<APPROOT>/Documents/Movies"
PicturesLocation"<USER>/Pictures", "<USER>/<APPNAME>/Pictures""<APPROOT>/Documents/Pictures", "assets-library://"
TempLocation"<APPROOT>/cache""<APPROOT>/tmp"
HomeLocation"<APPROOT>/files""<APPROOT>" (not writable)
DataLocation"<APPROOT>/files", "<USER>/<APPNAME>/files""<APPROOT>/Library/Application Support"
CacheLocation"<APPROOT>/cache", "<USER>/<APPNAME>/cache""<APPROOT>/Library/Caches"
GenericDataLocation"<USER>""<APPROOT>/Documents"
RuntimeLocation"<APPROOT>/cache"not supported
ConfigLocation"<APPROOT>/files/settings""<APPROOT>/Library/Preferences"
GenericConfigLocation"<APPROOT>/files/settings" (there is no shared settings)"<APPROOT>/Library/Preferences"
DownloadLocation"<USER>/Downloads", "<USER>/<APPNAME>/Downloads""<APPROOT>/Documents/Downloads"
GenericCacheLocation"<APPROOT>/cache" (there is no shared cache)"<APPROOT>/Library/Caches"
AppDataLocation"<APPROOT>/files", "<USER>/<APPNAME>/files""<APPROOT>/Library/Application Support"
AppConfigLocation"<APPROOT>/files/settings""<APPROOT>/Library/Preferences/<APPNAME>"
AppLocalDataLocation"<APPROOT>/files", "<USER>/<APPNAME>/files""<APPROOT>/Library/Application Support"


In the table above, <APPNAME> is usually the organization name, the application name, or both, or a unique name generated at packaging. Similarly, <APPROOT> is the location where this application is installed (often a sandbox). <APPDIR> is the directory containing the application executable.

The paths above should not be relied upon, as they may change according to OS configuration, locale, or they may change in future Qt versions.

Note: On Android, applications with open files on the external storage (<USER> locations), will be killed if the external storage is unmounted.


See also writableLocation(), standardLocations(), displayName(), locate(), and locateAll().

*/
pub type QStandardPaths__StandardLocation = i32;
// Returns the user's desktop directory. This is a generic value. On systems with no concept of a desktop.
pub const QStandardPaths__DesktopLocation :QStandardPaths__StandardLocation = 0;
// Returns the directory containing user document files. This is a generic value. The returned path is never empty.
pub const QStandardPaths__DocumentsLocation :QStandardPaths__StandardLocation = 1;
// Returns the directory containing user's fonts. This is a generic value. Note that installing fonts may require additional, platform-specific operations.
pub const QStandardPaths__FontsLocation :QStandardPaths__StandardLocation = 2;
// Returns the directory containing the user applications (either executables, application bundles, or shortcuts to them). This is a generic value. Note that installing applications may require additional, platform-specific operations. Files, folders or shortcuts in this directory are platform-specific.
pub const QStandardPaths__ApplicationsLocation :QStandardPaths__StandardLocation = 3;
// Returns the directory containing the user's music or other audio files. This is a generic value. If no directory specific for music files exists, a sensible fallback for storing user documents is returned.
pub const QStandardPaths__MusicLocation :QStandardPaths__StandardLocation = 4;
// Returns the directory containing the user's movies and videos. This is a generic value. If no directory specific for movie files exists, a sensible fallback for storing user documents is returned.
pub const QStandardPaths__MoviesLocation :QStandardPaths__StandardLocation = 5;
// Returns the directory containing the user's pictures or photos. This is a generic value. If no directory specific for picture files exists, a sensible fallback for storing user documents is returned.
pub const QStandardPaths__PicturesLocation :QStandardPaths__StandardLocation = 6;
// Returns a directory where temporary files can be stored. The returned value might be application-specific, shared among other applications for this user, or even system-wide. The returned path is never empty.
pub const QStandardPaths__TempLocation :QStandardPaths__StandardLocation = 7;
// Returns the user's home directory (the same as QDir::homePath()). On Unix systems, this is equal to the HOME environment variable. This value might be generic or application-specific, but the returned path is never empty.
pub const QStandardPaths__HomeLocation :QStandardPaths__StandardLocation = 8;
// Returns the same value as AppLocalDataLocation. This enumeration value is deprecated. Using AppDataLocation is preferable since on Windows, the roaming path is recommended.
pub const QStandardPaths__DataLocation :QStandardPaths__StandardLocation = 9;
// 
pub const QStandardPaths__CacheLocation :QStandardPaths__StandardLocation = 10;
// 
pub const QStandardPaths__GenericDataLocation :QStandardPaths__StandardLocation = 11;
// 
pub const QStandardPaths__RuntimeLocation :QStandardPaths__StandardLocation = 12;
// 
pub const QStandardPaths__ConfigLocation :QStandardPaths__StandardLocation = 13;
// 
pub const QStandardPaths__DownloadLocation :QStandardPaths__StandardLocation = 14;
// 
pub const QStandardPaths__GenericCacheLocation :QStandardPaths__StandardLocation = 15;
// 
pub const QStandardPaths__GenericConfigLocation :QStandardPaths__StandardLocation = 16;
// 
pub const QStandardPaths__AppDataLocation :QStandardPaths__StandardLocation = 17;
// 
pub const QStandardPaths__AppConfigLocation :QStandardPaths__StandardLocation = 18;
// 
pub const QStandardPaths__AppLocalDataLocation :QStandardPaths__StandardLocation = 9;
pub fn QStandardPaths_StandardLocationItemName(val: i32) ->String {
  match val {
     QStandardPaths__DesktopLocation => // 0
     {return String::from("DesktopLocation");}
     QStandardPaths__DocumentsLocation => // 1
     {return String::from("DocumentsLocation");}
     QStandardPaths__FontsLocation => // 2
     {return String::from("FontsLocation");}
     QStandardPaths__ApplicationsLocation => // 3
     {return String::from("ApplicationsLocation");}
     QStandardPaths__MusicLocation => // 4
     {return String::from("MusicLocation");}
     QStandardPaths__MoviesLocation => // 5
     {return String::from("MoviesLocation");}
     QStandardPaths__PicturesLocation => // 6
     {return String::from("PicturesLocation");}
     QStandardPaths__TempLocation => // 7
     {return String::from("TempLocation");}
     QStandardPaths__HomeLocation => // 8
     {return String::from("HomeLocation");}
     QStandardPaths__DataLocation => // 9
     {return String::from("DataLocation,AppLocalDataLocation");}
     QStandardPaths__CacheLocation => // 10
     {return String::from("CacheLocation");}
     QStandardPaths__GenericDataLocation => // 11
     {return String::from("GenericDataLocation");}
     QStandardPaths__RuntimeLocation => // 12
     {return String::from("RuntimeLocation");}
     QStandardPaths__ConfigLocation => // 13
     {return String::from("ConfigLocation");}
     QStandardPaths__DownloadLocation => // 14
     {return String::from("DownloadLocation");}
     QStandardPaths__GenericCacheLocation => // 15
     {return String::from("GenericCacheLocation");}
     QStandardPaths__GenericConfigLocation => // 16
     {return String::from("GenericConfigLocation");}
     QStandardPaths__AppDataLocation => // 17
     {return String::from("AppDataLocation");}
     QStandardPaths__AppConfigLocation => // 18
     {return String::from("AppConfigLocation");}
    // QStandardPaths__AppLocalDataLocation => // 9
    // {return String::from("");}
  _ => {return format!("{}", val);}
}
}
pub fn QStandardPaths_StandardLocationItemName_s(val: i32) ->String {
  //var nilthis *QStandardPaths
  //return nilthis.StandardLocationItemName(val);
  return QStandardPaths_StandardLocationItemName(val);
}


/*


*/
pub type QStandardPaths__LocateOption = i32;
// 
pub const QStandardPaths__LocateFile :QStandardPaths__LocateOption = 0;
// 
pub const QStandardPaths__LocateDirectory :QStandardPaths__LocateOption = 1;
pub fn QStandardPaths_LocateOptionItemName(val: i32) ->String {
  match val {
     QStandardPaths__LocateFile => // 0
     {return String::from("LocateFile");}
     QStandardPaths__LocateDirectory => // 1
     {return String::from("LocateDirectory");}
  _ => {return format!("{}", val);}
}
}
pub fn QStandardPaths_LocateOptionItemName_s(val: i32) ->String {
  //var nilthis *QStandardPaths
  //return nilthis.LocateOptionItemName(val);
  return QStandardPaths_LocateOptionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
